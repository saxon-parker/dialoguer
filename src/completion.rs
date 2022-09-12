pub struct CompletionResult {
    pub completion: String,
    pub redraw_prompt: bool,
}

/// Trait for completion handling.
pub trait Completion {
    fn get(&self, input: &str) -> Option<CompletionResult>;
}
