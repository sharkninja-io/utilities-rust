use mantle_utilities::poll_manager::{PollConfig, PollManager};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn send_value() {
    let manager = PollManager::new();
    let (tx, rx) = mpsc::channel();

    manager.add_poll(PollConfig {
        on: Some(true),
        sleep_time: Some(1),
        callback: Some(Box::new(move |value| {
            let _ = tx.send(value);
        })),
        respondent: Some(Box::new(|| 42)),
    });
    manager.start_polling();

    assert_eq!(42, rx.recv().unwrap());
    assert_eq!(42, rx.recv().unwrap());
}

#[test]
fn remove_poll() {
    let manager = PollManager::new();
    let (tx, rx) = mpsc::channel();

    let id = manager.add_poll(PollConfig {
        sleep_time: Some(20),
        callback: Some(Box::new(move |_| tx.send(()).unwrap())),
        respondent: Some(Box::new(|| ())),
        ..Default::default()
    });
    manager.start_polling();
    sleep(Duration::from_millis(10));

    manager.remove_poll(id);
    assert!(rx.recv().is_ok());
    assert!(rx.recv().is_err());
}

#[test]
fn update_poll() {
    let manager = PollManager::new();
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let id = manager.add_poll(PollConfig {
        on: Some(true),
        sleep_time: Some(50),
        callback: Some(Box::new(move |_| tx.send(()).unwrap())),
        respondent: Some(Box::new(|| ())),
    });
    manager.start_polling();

    assert!(rx.recv().is_ok());

    manager.update_poll(
        id,
        PollConfig {
            callback: Some(Box::new(move |_| {
                let _ = tx2.send(());
            })),
            ..Default::default()
        },
    );
    assert!(rx2.recv().is_ok());
    assert!(rx.recv().is_err());
}

#[test]
fn stop_polling() {
    let manager = PollManager::new();
    let (tx, rx) = mpsc::channel();

    manager.add_poll(PollConfig {
        on: Some(true),
        sleep_time: Some(10),
        callback: Some(Box::new(move |_| tx.send(()).unwrap())),
        respondent: Some(Box::new(|| ())),
    });
    manager.start_polling();

    assert!(rx.recv().is_ok());

    manager.stop_polling();
    assert!(rx.recv_timeout(Duration::from_millis(20)).is_err());
}

#[test]
fn no_deadlock_when_manager_called_from_callback_or_respondent() {
    let manager = PollManager::new();
    let (tx, rx) = mpsc::channel();
    let poll_manager_callback = manager.clone();
    let poll_manager_responder = manager.clone();

    manager.add_poll(PollConfig {
        on: Some(true),
        sleep_time: Some(10),
        callback: Some(Box::new(move |_| {
            poll_manager_callback.stop_polling();
            tx.send(()).unwrap();
        })),
        respondent: Some(Box::new(move || {
            poll_manager_responder.add_poll(PollConfig::default());
        })),
    });
    manager.start_polling();

    assert!(rx.recv_timeout(Duration::from_millis(20)).is_ok());
}
