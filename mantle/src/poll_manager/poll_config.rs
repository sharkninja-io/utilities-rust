pub struct PollConfig<T> {
    pub on: Option<bool>,
    /// In milliseconds
    pub sleep_time: Option<u64>,
    pub callback: Option<Box<dyn Fn(T) + Send>>,
    pub respondent: Option<Box<dyn Fn() -> T + Send>>,
}

// Separate impl block to remove the T: Default bound
impl<T> Default for PollConfig<T> {
    fn default() -> Self {
        PollConfig {
            on: None,
            sleep_time: None,
            callback: None,
            respondent: None,
        }
    }
}
