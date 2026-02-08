use luanext_sourcemap::SourceMap;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TranslatedSource {
    pub source_file: String,
    pub source_content: Option<String>,
    pub translated_line: u32,
    pub translated_column: u32,
    pub original_span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug)]
pub struct SourceMapTranslator {
    source_map: SourceMap,
    decoded_mappings: Vec<Vec<DecodedMapping>>,
    source_index_cache: HashMap<usize, String>,
}

#[derive(Debug, Clone)]
struct DecodedMapping {
    generated_column: usize,
    source_line: usize,
    source_column: usize,
}

impl SourceMapTranslator {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Self::from_json(&content)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let source_map: SourceMap = serde_json::from_str(json)?;
        Self::new(source_map)
    }

    pub fn new(source_map: SourceMap) -> Result<Self, Box<dyn std::error::Error>> {
        let decoded_mappings = Self::decode_mappings(&source_map.mappings);
        let mut source_index_cache = HashMap::new();
        for (idx, source) in source_map.sources.iter().enumerate() {
            source_index_cache.insert(idx, source.clone());
        }
        Ok(Self {
            source_map,
            decoded_mappings,
            source_index_cache,
        })
    }

    fn decode_mappings(mappings: &str) -> Vec<Vec<DecodedMapping>> {
        let mut result: Vec<Vec<DecodedMapping>> = Vec::new();
        if mappings.is_empty() {
            return result;
        }

        let mut prev_gen_col = 0usize;
        let mut prev_src_idx = 0usize;
        let mut prev_src_line = 0usize;
        let mut prev_src_col = 0usize;
        let mut prev_name_idx = 0usize;

        let segments: Vec<&str> = mappings.split(';').collect();

        for _segment in &segments {
            let mut mappings_for_line: Vec<DecodedMapping> = Vec::new();
            let fields: Vec<&str> = segments[0].split(',').filter(|s| !s.is_empty()).collect();

            let mut gen_col = 0usize;
            let mut src_idx = 0usize;
            let mut src_line = 0usize;
            let mut src_col = 0usize;
            let mut name_idx = 0usize;

            for field in &fields {
                let values = Self::decode_vlq(field);
                if !values.is_empty() {
                    gen_col = (prev_gen_col as i64 + values[0]) as usize;
                    if values.len() > 1 {
                        src_idx = (prev_src_idx as i64 + values[1]) as usize;
                    }
                    if values.len() > 2 {
                        src_line = (prev_src_line as i64 + values[2]) as usize;
                    }
                    if values.len() > 3 {
                        src_col = (prev_src_col as i64 + values[3]) as usize;
                    }
                    if values.len() > 4 {
                        name_idx = (prev_name_idx as i64 + values[4]) as usize;
                    }
                }

                let mapping = DecodedMapping {
                    generated_column: gen_col,
                    source_line: src_line,
                    source_column: src_col,
                };
                mappings_for_line.push(mapping);

                prev_gen_col = gen_col;
                prev_src_idx = src_idx;
                prev_src_line = src_line;
                prev_src_col = src_col;
                prev_name_idx = name_idx;
            }

            if !mappings_for_line.is_empty() {
                result.push(mappings_for_line);
            }
        }

        result
    }

    fn decode_vlq(segment: &str) -> Vec<i64> {
        let mut result = Vec::new();
        let mut current = 0i64;
        let mut shift = 0i64;

        for ch in segment.chars() {
            let value = Self::base64_value(ch);
            current |= (value & 0x1F) << shift;
            shift += 5;

            if (value & 0x20) == 0 {
                if current >= 0x10 {
                    current -= 0x20;
                }
                result.push(current);
                current = 0;
                shift = 0;
            }
        }

        result
    }

    fn base64_value(ch: char) -> i64 {
        match ch {
            'A'..='Z' => (ch as u8 - b'A') as i64,
            'a'..='z' => (ch as u8 - b'a' + 26) as i64,
            '0'..='9' => (ch as u8 - b'0' + 52) as i64,
            '+' => 62,
            '/' => 63,
            _ => 0,
        }
    }

    pub fn translate(
        &self,
        generated_line: u32,
        generated_column: u32,
    ) -> Option<TranslatedSource> {
        let line_idx = generated_line as usize;
        let col = generated_column as usize;

        if line_idx >= self.decoded_mappings.len() {
            return None;
        }

        let mappings = &self.decoded_mappings[line_idx];

        let mut closest_mapping: Option<&DecodedMapping> = None;

        for mapping in mappings {
            if mapping.generated_column <= col {
                closest_mapping = Some(mapping);
            } else {
                break;
            }
        }

        if let Some(mapping) = closest_mapping {
            let source_file = self
                .source_index_cache
                .get(&mapping.source_line)
                .cloned()
                .unwrap_or_else(|| String::from("unknown"));

            let source_content = if mapping.source_line < self.source_map.sources_content.len() {
                self.source_map.sources_content[mapping.source_line].clone()
            } else {
                None
            };

            let original_span = Some(SourceSpan {
                start_line: mapping.source_line as u32,
                start_column: mapping.source_column as u32,
                end_line: mapping.source_line as u32,
                end_column: mapping.source_column as u32,
            });

            return Some(TranslatedSource {
                source_file,
                source_content,
                translated_line: mapping.source_line as u32,
                translated_column: mapping.source_column as u32,
                original_span,
            });
        }

        None
    }

    pub fn get_source_content(&self, source_file: &str) -> Option<&str> {
        if let Some(idx) = self
            .source_map
            .sources
            .iter()
            .position(|s| s == source_file)
        {
            if idx < self.source_map.sources_content.len() {
                return self.source_map.sources_content[idx].as_deref();
            }
        }
        None
    }

    pub fn sources(&self) -> &[String] {
        &self.source_map.sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use luanext_sourcemap::SourceMapBuilder;

    fn create_test_source_map() -> SourceMap {
        let mut builder = SourceMapBuilder::new("input.luax".to_string());
        builder.set_file("output.lua".to_string());
        builder.add_source_content("local x = 1\nprint(x)".to_string());

        builder.add_mapping(luanext_sourcemap::Span::new(0, 5, 0, 0), None);
        builder.advance("local");
        builder.advance(" ");

        builder.add_mapping(luanext_sourcemap::Span::new(6, 9, 0, 7), None);
        builder.advance("x");
        builder.advance(" ");
        builder.advance("=");
        builder.advance(" ");
        builder.advance("1");
        builder.advance(";");

        builder.add_mapping(luanext_sourcemap::Span::new(10, 13, 1, 0), None);
        builder.advance("print");
        builder.advance("(");
        builder.advance("x");
        builder.advance(")");

        builder.build()
    }

    #[test]
    fn test_translator_from_source_map() {
        let source_map = create_test_source_map();
        let translator = SourceMapTranslator::new(source_map).unwrap();

        assert!(!translator.sources().is_empty());
    }

    #[test]
    fn test_translate_position() {
        let source_map = create_test_source_map();
        let translator = SourceMapTranslator::new(source_map).unwrap();

        let result = translator.translate(0, 0);
        assert!(result.is_some());

        let result = translator.translate(0, 5);
        assert!(result.is_some());
    }

    #[test]
    fn test_translate_beyond_range() {
        let source_map = create_test_source_map();
        let translator = SourceMapTranslator::new(source_map).unwrap();

        let result = translator.translate(100, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_source_content() {
        let source_map = create_test_source_map();
        let translator = SourceMapTranslator::new(source_map).unwrap();

        let content = translator.get_source_content("input.luax");
        assert!(content.is_some());
    }
}
