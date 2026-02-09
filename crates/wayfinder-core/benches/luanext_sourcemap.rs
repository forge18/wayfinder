//! Benchmarks for LuaNext source map translation performance
//!
//! Measures:
//! - Source map loading from files, inline, and data URIs
//! - Position translation (forward and reverse lookups)
//! - Bundle mode with multiple sources
//! - Cache performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use luanext_sourcemap::{SourceMap, SourceMapSource, PositionTranslator};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Convert source map to JSON with all required fields
fn source_map_to_json_with_all_fields(map: &SourceMap) -> serde_json::Value {
    // Manually construct JSON with all fields to avoid skip_serializing_if issues
    // Use snake_case for field names since that's what the SourceMap struct expects
    let mut json = serde_json::json!({
        "version": map.version,
        "sources": map.sources,
        "mappings": map.mappings,
        "sources_content": map.sources_content,
        "names": map.names,
    });

    if let Some(ref file) = map.file {
        json["file"] = serde_json::json!(file);
    }

    if let Some(ref root) = map.source_root {
        json["source_root"] = serde_json::json!(root);
    }

    json
}

/// Write a source map to a file, including all required fields
fn write_source_map(map: &SourceMap, path: &PathBuf) -> std::io::Result<()> {
    let json = source_map_to_json_with_all_fields(map);
    fs::write(path, serde_json::to_string(&json)?)
}

/// Create a test source map
fn create_source_map(num_sources: usize, mappings_complexity: usize) -> SourceMap {
    let sources: Vec<String> = (0..num_sources)
        .map(|i| format!("module{}.luax", i))
        .collect();

    // Create sources_content with same length as sources to avoid empty array issues
    let sources_content: Vec<Option<String>> = (0..num_sources)
        .map(|_| None)
        .collect();

    let mappings = "AAAA,CAAC,CAAC,CAAC,EAAE,EAAE,GAAG,GAAG,IAAI,IAAI,KAAK".repeat(mappings_complexity);

    SourceMap {
        version: 3,
        file: Some("bundle.lua".to_string()),
        source_root: None,
        sources,
        sources_content,
        names: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        mappings,
    }
}

/// Benchmark source map loading from file
fn bench_load_from_file(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let map_path = temp_dir.path().join("test.lua.map");

    let source_map = create_source_map(1, 10);
    write_source_map(&source_map, &map_path).unwrap();

    c.bench_function("source_map_load_file", |b| {
        b.iter(|| {
            let mut translator = PositionTranslator::new();
            translator.load_source_map(
                black_box(PathBuf::from("test.lua")),
                black_box(&SourceMapSource::File(map_path.clone()))
            ).unwrap();
        });
    });
}

/// Benchmark source map loading from inline JSON
fn bench_load_from_inline(c: &mut Criterion) {
    let source_map = create_source_map(1, 10);
    let json = serde_json::to_string(&source_map).unwrap();

    c.bench_function("source_map_load_inline", |b| {
        b.iter(|| {
            let mut translator = PositionTranslator::new();
            translator.load_source_map(
                black_box(PathBuf::from("test.lua")),
                black_box(&SourceMapSource::Inline(json.clone()))
            ).unwrap();
        });
    });
}

/// Benchmark source map loading from data URI
fn bench_load_from_data_uri(c: &mut Criterion) {
    use base64::Engine;

    let source_map = create_source_map(1, 10);

    // Create data URI in format the loader expects (without charset=utf-8)
    let json_value = source_map_to_json_with_all_fields(&source_map);
    let json = serde_json::to_string(&json_value).unwrap();
    let encoded = base64::engine::general_purpose::STANDARD.encode(json.as_bytes());
    let data_uri = format!("data:application/json;base64,{}", encoded);

    c.bench_function("source_map_load_data_uri", |b| {
        b.iter(|| {
            let mut translator = PositionTranslator::new();
            translator.load_source_map(
                black_box(PathBuf::from("test.lua")),
                black_box(&SourceMapSource::DataUri(data_uri.clone()))
            ).unwrap();
        });
    });
}

/// Benchmark forward lookup (compiled → original)
fn bench_forward_lookup(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let map_path = temp_dir.path().join("test.lua.map");

    let source_map = create_source_map(1, 10);
    write_source_map(&source_map, &map_path).unwrap();

    let mut translator = PositionTranslator::new();
    translator.load_source_map(
        PathBuf::from("test.lua"),
        &SourceMapSource::File(map_path)
    ).unwrap();

    c.bench_function("source_map_forward_lookup", |b| {
        b.iter(|| {
            translator.forward_lookup(
                black_box(&PathBuf::from("test.lua")),
                black_box(10),
                black_box(5)
            )
        });
    });
}

/// Benchmark reverse lookup (original → compiled)
fn bench_reverse_lookup(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let map_path = temp_dir.path().join("test.lua.map");

    let source_map = create_source_map(1, 10);
    write_source_map(&source_map, &map_path).unwrap();

    let mut translator = PositionTranslator::new();
    translator.load_source_map(
        PathBuf::from("test.lua"),
        &SourceMapSource::File(map_path)
    ).unwrap();

    c.bench_function("source_map_reverse_lookup", |b| {
        b.iter(|| {
            translator.reverse_lookup(
                black_box(&PathBuf::from("module0.luax")),
                black_box(10),
                black_box(5)
            )
        });
    });
}

/// Benchmark bundle mode with varying number of sources
fn bench_bundle_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("source_map_bundle_mode");

    for num_sources in [1, 5, 10, 50, 100].iter() {
        let temp_dir = TempDir::new().unwrap();
        let map_path = temp_dir.path().join("bundle.lua.map");

        let source_map = create_source_map(*num_sources, 10);
        write_source_map(&source_map, &map_path).unwrap();

        let mut translator = PositionTranslator::new();
        translator.load_source_map(
            PathBuf::from("bundle.lua"),
            &SourceMapSource::File(map_path)
        ).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(num_sources),
            num_sources,
            |b, _| {
                b.iter(|| {
                    translator.handle_bundle_mode(black_box(&PathBuf::from("bundle.lua")))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark source map with varying mappings complexity
fn bench_mappings_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("source_map_mappings_complexity");

    for complexity in [10, 50, 100, 500, 1000].iter() {
        let temp_dir = TempDir::new().unwrap();
        let map_path = temp_dir.path().join("test.lua.map");

        let source_map = create_source_map(1, *complexity);
        write_source_map(&source_map, &map_path).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(complexity),
            complexity,
            |b, _| {
                b.iter(|| {
                    let mut translator = PositionTranslator::new();
                    translator.load_source_map(
                        black_box(PathBuf::from("test.lua")),
                        black_box(&SourceMapSource::File(map_path.clone()))
                    ).unwrap();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multiple source maps loading
fn bench_multiple_source_maps(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    c.bench_function("source_map_load_10_files", |b| {
        b.iter(|| {
            let mut translator = PositionTranslator::new();

            for i in 0..10 {
                let map_path = temp_dir.path().join(format!("test{}.lua.map", i));
                let source_map = create_source_map(1, 10);
                write_source_map(&source_map, &map_path).unwrap();

                translator.load_source_map(
                    black_box(PathBuf::from(format!("test{}.lua", i))),
                    black_box(&SourceMapSource::File(map_path))
                ).unwrap();
            }
        });
    });
}

/// Benchmark data URI generation
fn bench_data_uri_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("source_map_data_uri_generation");

    for size in [10, 50, 100, 500].iter() {
        let source_map = create_source_map(1, *size);

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(source_map.to_data_uri().unwrap());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark JSON serialization/deserialization
fn bench_json_serialization(c: &mut Criterion) {
    let source_map = create_source_map(10, 100);

    c.bench_function("source_map_to_json", |b| {
        b.iter(|| {
            black_box(source_map.to_json().unwrap());
        });
    });

    let json = source_map.to_json().unwrap();

    c.bench_function("source_map_from_json", |b| {
        b.iter(|| {
            let _: SourceMap = black_box(serde_json::from_str(&json).unwrap());
        });
    });
}

/// Benchmark lookup fallback
fn bench_lookup_fallback(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    // Load 10 source maps
    let mut translator = PositionTranslator::new();
    for i in 0..10 {
        let map_path = temp_dir.path().join(format!("test{}.lua.map", i));
        let source_map = create_source_map(1, 10);
        write_source_map(&source_map, &map_path).unwrap();
        translator.load_source_map(
            PathBuf::from(format!("test{}.lua", i)),
            &SourceMapSource::File(map_path)
        ).unwrap();
    }

    c.bench_function("source_map_lookup_fallback", |b| {
        b.iter(|| {
            // Lookup existing file (hit)
            black_box(translator.lookup_with_fallback(&PathBuf::from("test5.lua")));
            // Lookup non-existing file (miss)
            black_box(translator.lookup_with_fallback(&PathBuf::from("nonexistent.lua")));
        });
    });
}

/// Benchmark sequential lookups (simulating debugging session)
fn bench_debugging_session(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let map_path = temp_dir.path().join("app.lua.map");

    let source_map = create_source_map(5, 100);
    write_source_map(&source_map, &map_path).unwrap();

    let mut translator = PositionTranslator::new();
    translator.load_source_map(
        PathBuf::from("app.lua"),
        &SourceMapSource::File(map_path)
    ).unwrap();

    c.bench_function("source_map_debugging_session", |b| {
        b.iter(|| {
            // Simulate debugging session with multiple operations
            for line in 1..20 {
                // Forward lookup (stack trace)
                let _ = translator.forward_lookup(
                    black_box(&PathBuf::from("app.lua")),
                    black_box(line),
                    black_box(1)
                );

                // Reverse lookup (breakpoint)
                let _ = translator.reverse_lookup(
                    black_box(&PathBuf::from("module0.luax")),
                    black_box(line),
                    black_box(1)
                );
            }
        });
    });
}

criterion_group!(
    benches,
    bench_load_from_file,
    bench_load_from_inline,
    bench_load_from_data_uri,
    bench_forward_lookup,
    bench_reverse_lookup,
    bench_bundle_mode,
    bench_mappings_complexity,
    bench_multiple_source_maps,
    bench_data_uri_generation,
    bench_json_serialization,
    bench_lookup_fallback,
    bench_debugging_session,
);

criterion_main!(benches);
