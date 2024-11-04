use log::error;
use once_cell::sync::Lazy;
use std::string::ToString;
use std::sync::{mpsc, Arc, Mutex};
use threadpool::ThreadPool;

static TPOOL: Lazy<Arc<Mutex<ThreadPool>>> = Lazy::new(|| {
    Arc::new(Mutex::new(ThreadPool::with_name(
        "mantle-thread".to_string(),
        num_worker_threads(),
    )))
});

pub fn execute_job<F>(job: F)
where
    F: FnOnce() + Send + 'static,
{
    match TPOOL.clone().lock() {
        Ok(mut pool) => {
            let active = pool.active_count();
            let worker_threads = num_worker_threads();
            if active >= worker_threads {
                pool.set_num_threads(active + 1);
            } else {
                pool.set_num_threads(worker_threads)
            }

            pool.execute(job);
        }
        Err(err) => error!("Error getting lock on TPOOL clone: {}", err.to_string()),
    }
}

/// Execute 'jobs' on the tread pool and wait for the 'jobs' results.
/// 'jobs' - collection of closures.
pub fn execute_and_join_jobs<I, F, R>(jobs: I) -> Vec<R>
where
    I: IntoIterator<Item = F>,
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let mut jobs_count = 0;
    let (tx, rx) = mpsc::channel();

    for job in jobs {
        let tx = tx.clone();
        execute_job(move || {
            let result = job();
            tx.send(result).unwrap();
        });

        jobs_count += 1;
    }

    let mut results = Vec::with_capacity(jobs_count);
    for _ in 0..jobs_count {
        let result = rx.recv().unwrap();
        results.push(result);
    }
    results
}

// FIXME: execute_job rewrites this limit.
pub fn set_max_threads(max: usize) {
    match TPOOL.clone().lock() {
        Ok(mut pool) => {
            pool.set_num_threads(max);
            drop(pool);
        }
        Err(err) => error!("Error getting lock on TPOOL clone: {}", err.to_string()),
    }
}

pub fn num_available_cpus() -> usize {
    num_cpus::get()
}

pub fn num_worker_threads() -> usize {
    num_available_cpus()
}
