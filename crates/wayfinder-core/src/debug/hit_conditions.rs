//! Hit condition evaluation for breakpoints
//!
//! This module provides functionality to evaluate breakpoint hit conditions
//! such as "> 5", "== 3", or "% 2".

/// Evaluates a hit condition against a hit count
pub fn evaluate_hit_condition(condition: &str, hit_count: usize) -> Result<bool, String> {
    let trimmed = condition.trim();
    if trimmed.is_empty() {
        return Ok(true);
    }

    // Parse the condition
    // Supported formats:
    // - "> N" - hit count greater than N
    // - ">= N" - hit count greater than or equal to N
    // - "< N" - hit count less than N
    // - "<= N" - hit count less than or equal to N
    // - "== N" - hit count equal to N
    // - "!= N" - hit count not equal to N
    // - "% N" - hit count modulo N equals 0

    if let Some(rest) = trimmed.strip_prefix(">=") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count >= threshold)
    } else if let Some(rest) = trimmed.strip_prefix(">") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count > threshold)
    } else if let Some(rest) = trimmed.strip_prefix("<=") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count <= threshold)
    } else if let Some(rest) = trimmed.strip_prefix("<") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count < threshold)
    } else if let Some(rest) = trimmed.strip_prefix("==") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count == threshold)
    } else if let Some(rest) = trimmed.strip_prefix("!=") {
        let threshold: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        Ok(hit_count != threshold)
    } else if let Some(rest) = trimmed.strip_prefix("%") {
        let divisor: usize = rest
            .trim()
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", rest))?;
        if divisor == 0 {
            return Err("Division by zero in hit condition".to_string());
        }
        Ok(hit_count % divisor == 0)
    } else {
        // Default to equality check if no operator specified
        let threshold: usize = trimmed
            .parse()
            .map_err(|_| format!("Invalid number in hit condition: {}", trimmed))?;
        Ok(hit_count == threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_than() {
        assert_eq!(evaluate_hit_condition("> 5", 6).unwrap(), true);
        assert_eq!(evaluate_hit_condition("> 5", 5).unwrap(), false);
        assert_eq!(evaluate_hit_condition("> 5", 4).unwrap(), false);
    }

    #[test]
    fn test_greater_than_or_equal() {
        assert_eq!(evaluate_hit_condition(">= 5", 6).unwrap(), true);
        assert_eq!(evaluate_hit_condition(">= 5", 5).unwrap(), true);
        assert_eq!(evaluate_hit_condition(">= 5", 4).unwrap(), false);
    }

    #[test]
    fn test_less_than() {
        assert_eq!(evaluate_hit_condition("< 5", 4).unwrap(), true);
        assert_eq!(evaluate_hit_condition("< 5", 5).unwrap(), false);
        assert_eq!(evaluate_hit_condition("< 5", 6).unwrap(), false);
    }

    #[test]
    fn test_less_than_or_equal() {
        assert_eq!(evaluate_hit_condition("<= 5", 4).unwrap(), true);
        assert_eq!(evaluate_hit_condition("<= 5", 5).unwrap(), true);
        assert_eq!(evaluate_hit_condition("<= 5", 6).unwrap(), false);
    }

    #[test]
    fn test_equal() {
        assert_eq!(evaluate_hit_condition("== 5", 5).unwrap(), true);
        assert_eq!(evaluate_hit_condition("== 5", 4).unwrap(), false);
        assert_eq!(evaluate_hit_condition("== 5", 6).unwrap(), false);

        // Default to equality
        assert_eq!(evaluate_hit_condition("5", 5).unwrap(), true);
        assert_eq!(evaluate_hit_condition("5", 4).unwrap(), false);
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(evaluate_hit_condition("!= 5", 4).unwrap(), true);
        assert_eq!(evaluate_hit_condition("!= 5", 6).unwrap(), true);
        assert_eq!(evaluate_hit_condition("!= 5", 5).unwrap(), false);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(evaluate_hit_condition("% 3", 3).unwrap(), true);
        assert_eq!(evaluate_hit_condition("% 3", 6).unwrap(), true);
        assert_eq!(evaluate_hit_condition("% 3", 9).unwrap(), true);
        assert_eq!(evaluate_hit_condition("% 3", 4).unwrap(), false);
        assert_eq!(evaluate_hit_condition("% 3", 5).unwrap(), false);
    }

    #[test]
    fn test_invalid_conditions() {
        assert!(evaluate_hit_condition("> abc", 5).is_err());
        assert!(evaluate_hit_condition("% 0", 5).is_err());
    }

    #[test]
    fn test_empty_condition() {
        assert_eq!(evaluate_hit_condition("", 5).unwrap(), true);
        assert_eq!(evaluate_hit_condition("   ", 5).unwrap(), true);
    }
}
