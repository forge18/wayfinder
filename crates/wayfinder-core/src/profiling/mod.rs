use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Instant, Duration};

/// Profiling modes with different overhead/detail tradeoffs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfilingMode {
    /// No profiling active
    Disabled,
    /// Low overhead sampling at specified interval in milliseconds
    Sampling { interval_ms: u32 },
    /// Medium overhead call/return tracing
    CallTrace,
    /// High overhead line-level profiling
    LineLevel,
}

impl Default for ProfilingMode {
    fn default() -> Self {
        ProfilingMode::Disabled
    }
}

/// Profile data for a single function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionProfile {
    /// Function name
    pub name: String,
    /// Source file or chunk where function is defined
    pub source: Option<String>,
    /// Line number where function is defined
    pub line_defined: u32,
    /// Number of times this function was called
    pub call_count: u64,
    /// Total time spent in this function including callees (milliseconds)
    pub total_time_ms: f64,
    /// Time spent in this function excluding callees (milliseconds)
    pub self_time_ms: f64,
    /// Count of calls to child functions
    pub children: HashMap<String, u64>,
}

/// Complete profiling data for a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    /// Which profiling mode was used
    pub mode: ProfilingMode,
    /// Total duration of profiling (milliseconds)
    pub duration_ms: f64,
    /// Profile data for each function, keyed by function name
    pub functions: HashMap<String, FunctionProfile>,
    /// Total number of samples (for sampling mode)
    pub total_samples: u64,
}

/// Runtime profiler that tracks function calls and timing
pub struct Profiler {
    mode: ProfilingMode,
    start_time: Instant,
    /// Stack of currently executing functions with start times
    current_stack: Vec<(String, Instant)>,
    /// Accumulated profile data
    functions: HashMap<String, FunctionProfile>,
    /// Sample counter (incremented on each hook event for sampling mode)
    sample_count: u64,
}

impl Profiler {
    /// Create a new profiler with the given mode
    pub fn new(mode: ProfilingMode) -> Self {
        Self {
            mode,
            start_time: Instant::now(),
            current_stack: Vec::new(),
            functions: HashMap::new(),
            sample_count: 0,
        }
    }

    /// Record a function call
    pub fn on_call(&mut self, name: String, source: Option<String>, line: u32) {
        self.current_stack.push((name.clone(), Instant::now()));

        let profile = self.functions.entry(name.clone()).or_insert(FunctionProfile {
            name,
            source,
            line_defined: line,
            call_count: 0,
            total_time_ms: 0.0,
            self_time_ms: 0.0,
            children: HashMap::new(),
        });
        profile.call_count += 1;
    }

    /// Record a function return
    pub fn on_return(&mut self) {
        if let Some((name, start)) = self.current_stack.pop() {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;

            if let Some(profile) = self.functions.get_mut(&name) {
                profile.total_time_ms += elapsed;
            }

            // Track parent-child relationship
            if let Some((parent_name, _)) = self.current_stack.last() {
                if let Some(parent) = self.functions.get_mut(parent_name) {
                    *parent.children.entry(name).or_insert(0) += 1;
                }
            }
        }
    }

    /// Record a sample (for sampling mode)
    pub fn on_sample(&mut self) {
        self.sample_count += 1;

        // Record current stack for sampling mode
        if let Some((name, _)) = self.current_stack.last() {
            if let Some(profile) = self.functions.get_mut(name) {
                profile.self_time_ms += 1.0; // Sample weight
            }
        }
    }

    /// Finish profiling and return the collected data (consumes self)
    pub fn finish(self) -> ProfileData {
        ProfileData {
            mode: self.mode,
            duration_ms: self.start_time.elapsed().as_secs_f64() * 1000.0,
            functions: self.functions,
            total_samples: self.sample_count,
        }
    }

    /// Get profile data without consuming self (clones the data)
    pub fn to_profile_data(&self) -> ProfileData {
        ProfileData {
            mode: self.mode,
            duration_ms: self.start_time.elapsed().as_secs_f64() * 1000.0,
            functions: self.functions.clone(),
            total_samples: self.sample_count,
        }
    }

    /// Get the current profiling mode
    pub fn mode(&self) -> ProfilingMode {
        self.mode
    }

    /// Get the elapsed time since profiling started
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the current call stack depth
    pub fn stack_depth(&self) -> usize {
        self.current_stack.len()
    }

    /// Get sample count so far
    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }

    /// Get reference to the functions map
    pub fn functions(&self) -> &HashMap<String, FunctionProfile> {
        &self.functions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new(ProfilingMode::Sampling { interval_ms: 10 });
        assert_eq!(profiler.mode(), ProfilingMode::Sampling { interval_ms: 10 });
        assert_eq!(profiler.stack_depth(), 0);
    }

    #[test]
    fn test_function_call_tracking() {
        let mut profiler = Profiler::new(ProfilingMode::CallTrace);

        profiler.on_call("foo".to_string(), None, 10);
        assert_eq!(profiler.stack_depth(), 1);
        assert!(profiler.functions.contains_key("foo"));

        let foo_profile = &profiler.functions["foo"];
        assert_eq!(foo_profile.call_count, 1);
    }

    #[test]
    fn test_call_return_pairing() {
        let mut profiler = Profiler::new(ProfilingMode::CallTrace);

        profiler.on_call("foo".to_string(), None, 10);
        profiler.on_call("bar".to_string(), None, 20);
        profiler.on_return();

        assert_eq!(profiler.stack_depth(), 1);
        assert!(profiler.functions["foo"].children.contains_key("bar"));
    }

    #[test]
    fn test_profile_finish() {
        let mut profiler = Profiler::new(ProfilingMode::CallTrace);

        profiler.on_call("foo".to_string(), Some("test.lua".to_string()), 10);
        profiler.on_return();

        let data = profiler.finish();
        assert_eq!(data.mode, ProfilingMode::CallTrace);
        assert!(data.duration_ms >= 0.0);
        assert!(data.functions.contains_key("foo"));
    }
}
