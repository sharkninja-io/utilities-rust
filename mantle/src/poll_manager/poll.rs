use crate::poll_manager::PollConfig;
use std::fmt::{Debug, Formatter};

const DEFAULT_SLEEP_TIME_MILLIS: u64 = 5000;

pub struct Poll<T> {
    pub id: u32,
    pub on: bool,
    pub running: bool,
    pub sleep_time: u64,
    pub remove: bool,
    pub callback: Option<Box<dyn Fn(T) + Send>>,
    pub should_store_callback_after_sleep: bool,
    pub respondent: Option<Box<dyn Fn() -> T + Send>>,
    pub should_store_respondent_after_sleep: bool,
}

impl<T> Poll<T> {
    pub fn new(new_id: u32, config: PollConfig<T>) -> Self {
        let mut poll = Self {
            id: new_id,
            on: true,
            running: false,
            remove: false,
            sleep_time: DEFAULT_SLEEP_TIME_MILLIS,
            callback: None,
            // Indicates whether the poll callback has changed between the map locks in the poll loop. So we won't rewrite the new callback.
            should_store_callback_after_sleep: true,
            respondent: None,
            // Indicates whether the poll respondent has changed between the map locks in the poll loop.
            should_store_respondent_after_sleep: true,
        };
        poll.update(config);
        poll
    }

    /// Replaces the `Poll` config with `config` values that are Some.
    pub fn update(&mut self, config: PollConfig<T>) {
        if let Some(on) = config.on {
            self.set_on(on)
        }
        if let Some(sleep_time) = config.sleep_time {
            self.sleep_time = sleep_time
        }
        if let Some(callback) = config.callback {
            self.should_store_callback_after_sleep = false;
            self.callback = Some(callback);
        }
        if let Some(respondent) = config.respondent {
            self.should_store_respondent_after_sleep = false;
            self.respondent = Some(respondent);
        }
    }

    /// Replaces config of the `Poll`. Uses default values in place of None in `config`.
    /// Default values are:
    /// on - true
    /// sleep_time - 5000 milliseconds
    /// callback - None
    /// respondent - None
    pub fn replace(&mut self, config: PollConfig<T>) {
        self.set_on(config.on.unwrap_or(true));
        self.sleep_time = config.sleep_time.unwrap_or(DEFAULT_SLEEP_TIME_MILLIS);
        self.callback = config.callback;
        self.should_store_callback_after_sleep = false;
        self.respondent = config.respondent;
        self.should_store_respondent_after_sleep = false;
    }

    pub fn set_on(&mut self, on: bool) {
        self.on = on;
        if !on {
            self.running = false;
        }
    }

    pub fn set_remove(&mut self, remove: bool) {
        self.remove = remove;
    }

    pub fn set_callback(&mut self, callback: Option<Box<dyn Fn(T) + Send>>) {
        self.callback = callback;
        self.should_store_callback_after_sleep = false;
    }

    pub fn set_respondent(&mut self, respondent: Option<Box<dyn Fn() -> T + Send>>) {
        self.respondent = respondent;
        self.should_store_respondent_after_sleep = false;
    }
}

impl<T> Debug for Poll<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Poll")
            .field("id", &self.id)
            .field("on", &self.on)
            .field("running", &self.running)
            .field("sleep_time", &self.sleep_time)
            .field("remove", &self.remove)
            .field("has callback", &self.callback.is_some())
            .field(
                "should_store_callback_after_sleep",
                &self.should_store_callback_after_sleep,
            )
            .field("has respondent", &self.respondent.is_some())
            .field(
                "should_store_respondent_after_sleep",
                &self.should_store_respondent_after_sleep,
            )
            .finish()
    }
}
