mod watcher;
mod sync;

use notify::DebouncedEvent;
use std::sync::mpsc::channel;
use std::path::PathBuf;
// use std::sync::mpsc::Sender;

fn main() {
    let source = PathBuf::from("./source_dir");
    let destination = PathBuf::from("./dest_dir");

    if !source.exists() {
        panic!("Source directory does not exist: {:?}", source);
    }

    // Ensure destination directory exists
    if !destination.exists() {
        panic!("Destination directory does not exist: {:?}", destination);
    }

    let (tx, rx) = channel();

    let _watcher = watcher::watch_directory(source.clone(), tx.clone()); // NOTE: Hold watcher in main thread.

    // Listen for file changes and synchronize directories
    loop {
        match rx.recv() {
            Ok(event) =>
                match event {
                    DebouncedEvent::Write(path) => {
                        println!("File changed: {:?}", path);
                        sync::sync_files(&source, &destination);
                    }
                    DebouncedEvent::Remove(path) => {
                        println!("File removed: {:?}", path);
                    }
                    _ => (),
                }
            Err(e) => eprintln!("Error receiving events: {:?}", e),
        }
    }
}
