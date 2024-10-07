use notify::{ watcher, RecommendedWatcher, DebouncedEvent, RecursiveMode, Watcher };
use std::sync::mpsc::{ channel, Sender };
use std::time::Duration;
use std::path::PathBuf;

pub fn watch_directory(path: PathBuf, tx: Sender<DebouncedEvent>) -> RecommendedWatcher {
    let (watch_tx, watch_rx) = channel();
    let mut watcher = watcher(watch_tx, Duration::from_secs(2)).unwrap();

    println!("Watching directory: {:?}", path);

    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    // Spawn a thread to send events through the channel
    std::thread::spawn(move || {
        loop {
            match watch_rx.recv() {
                Ok(event) => {
                    println!("Event received: {:?}", event);
                    tx.send(event).unwrap();
                }
                Err(e) => {
                    eprintln!("watch error: {:?}", e);
                    break;
                }
            }
        }
    });

    watcher
}
