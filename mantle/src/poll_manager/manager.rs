use crate::poll_manager::poll::Poll;
use crate::poll_manager::PollConfig;
use crate::threadpool::num_worker_threads;
use log::{debug, error};
use std::collections::HashMap;
use std::sync::atomic::Ordering::{Acquire, Release, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct PollManager<T> {
    state: Arc<State<T>>,
}

#[derive(Debug)]
struct State<T> {
    poll_map: Mutex<HashMap<u32, Poll<T>>>,
    polling: AtomicBool,
    next_poll_id: AtomicU32,
    thread_pool: Mutex<ThreadPool>,
}

impl<T: 'static> PollManager<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(State {
                poll_map: Mutex::new(HashMap::new()),
                polling: AtomicBool::from(false),
                next_poll_id: AtomicU32::from(0),
                thread_pool: Mutex::new(ThreadPool::new(num_worker_threads())),
            }),
        }
    }

    pub fn start_polling(&self) {
        if self
            .state
            .polling
            .compare_exchange(false, true, SeqCst, Acquire)
            .is_ok()
        {
            for (_, poll) in self.lock_poll_map().iter_mut() {
                self.start_poll(poll);
            }
        }
    }

    pub fn stop_polling(&self) {
        self.state.polling.store(false, Release);
        for (_, poll) in self.lock_poll_map().iter_mut() {
            poll.set_on(false);
        }
    }

    pub fn add_poll(&self, config: PollConfig<T>) -> u32 {
        let poll_id = self.next_poll_id();
        let poll = Poll::new(poll_id, config);

        let mut poll_map = self.lock_poll_map();
        poll_map.insert(poll_id, poll);
        let count = poll_map.len();
        self.set_pool_count(count);
        if self.state.polling.load(Acquire) {
            if let Some(poll) = poll_map.get_mut(&poll_id) {
                self.start_poll(poll);
            }
        }
        poll_id
    }

    /// Replaces the `Poll` config with `config` values that are Some.
    pub fn update_poll(&self, poll_id: u32, new_config: PollConfig<T>) {
        if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
            poll.update(new_config);
        }
    }

    /// Replaces config of the `Poll`. Uses default values in place of None in `config`.
    /// Default values are:
    /// on - true
    /// sleep_time - 5000 milliseconds
    /// callback - None
    /// respondent - None
    pub fn replace_poll_config(&self, poll_id: u32, new_config: PollConfig<T>) {
        if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
            poll.replace(new_config);
        }
    }

    pub fn set_poll_callback(&self, poll_id: u32, callback: Option<Box<dyn Fn(T) + Send>>) {
        if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
            poll.set_callback(callback);
        }
    }

    pub fn set_poll_respondent(&self, poll_id: u32, respondent: Option<Box<dyn Fn() -> T + Send>>) {
        if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
            poll.set_respondent(respondent);
        }
    }

    pub fn clear_poll(&self, poll_id: u32) {
        let mut poll_map = self.lock_poll_map();
        if poll_map.remove(&poll_id).is_some() {
            debug!("Removed poll ID: {}", poll_id);
            self.set_pool_count(poll_map.len());
        }
    }

    pub fn remove_poll(&self, poll_id: u32) {
        if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
            poll.set_on(false);
            poll.set_remove(true);
        }
    }

    fn next_poll_id(&self) -> u32 {
        self.state.next_poll_id.fetch_add(1, Release)
    }

    fn start_poll(&self, poll: &mut Poll<T>) {
        poll.set_on(true);
        let poll_id = poll.id;
        let manager = self.clone();

        self.execute_job(move || {
            let mut on = false;
            if let Some(poll) = manager.lock_poll_map().get_mut(&poll_id) {
                if !poll.running {
                    on = poll.on;
                }
            }

            if on {
                manager.poll_loop(poll_id);
            }

            let remove = if let Some(poll) = manager.lock_poll_map().get_mut(&poll_id) {
                poll.running = false;
                poll.remove
            } else {
                false
            };
            if remove {
                manager.clear_poll(poll_id);
            }
        });
    }

    fn lock_poll_map(&self) -> MutexGuard<HashMap<u32, Poll<T>>> {
        self.state.poll_map.lock().unwrap()
    }

    fn poll_loop(&self, poll_id: u32) {
        loop {
            let (sleep_time, callback, respondent) =
                if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
                    if !poll.on {
                        break;
                    }
                    poll.running = true;
                    poll.should_store_callback_after_sleep = true;
                    poll.should_store_respondent_after_sleep = true;
                    (
                        poll.sleep_time,
                        poll.callback.take(),
                        poll.respondent.take(),
                    )
                } else {
                    break;
                };

            if let Some(respondent) = respondent.as_ref() {
                let result = respondent();

                if let Some(callback) = callback.as_ref() {
                    callback(result);
                }
            };

            sleep(Duration::from_millis(sleep_time));

            if let Some(poll) = self.lock_poll_map().get_mut(&poll_id) {
                if poll.should_store_callback_after_sleep {
                    poll.callback = callback;
                }
                if poll.should_store_respondent_after_sleep {
                    poll.respondent = respondent;
                }

                if !poll.on {
                    poll.running = false;
                    break;
                }
            }
        }
    }

    fn set_pool_count(&self, count: usize) {
        if count < num_worker_threads() {
            return;
        }
        match self.state.thread_pool.lock() {
            Ok(mut pool) => {
                pool.set_num_threads(count);
                debug!("poll pool active count is: {}", pool.active_count());
                debug!("poll pool queued count is: {}", pool.queued_count());
            }
            Err(err) => error!("Error getting lock on thread-pool: {err}"),
        }
    }

    pub fn execute_job<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        match self.state.thread_pool.lock() {
            Ok(pool) => {
                pool.execute(job);
            }
            Err(err) => error!("Error getting lock on thread-pool: {err}"),
        }
    }
}

impl<T> Clone for PollManager<T> {
    fn clone(&self) -> Self {
        PollManager {
            state: Arc::clone(&self.state),
        }
    }
}

impl<T: 'static> Default for PollManager<T> {
    fn default() -> Self {
        PollManager::new()
    }
}
