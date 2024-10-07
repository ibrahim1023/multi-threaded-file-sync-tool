# Multithreaded File Synchronization Tool

This is a simple, multithreaded file synchronization tool written in Rust. It watches a source directory for changes and synchronizes the updated files to a destination directory. It uses the `notify` crate for watching file changes and `rayon` for parallel file synchronization.

## Features

- **File Watching**: Monitors a source directory for changes (new, modified, or deleted files).
- **Multithreaded Synchronization**: Efficiently copies modified files from the source to the destination directory using multiple threads.
- **Debouncing**: Groups rapid file system events (e.g., multiple saves) into a single event.
- **Recursive Watching**: Watches all subdirectories under the source directory.

## Usage

Once you have cloned the repository, make sure to create source and destination directories

```
mkdir source_dir destination_dir
```

Run the tool

```
cargo run
```

## Project Structure

```
file_sync_tool/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # Entry point, initializes the watcher and sync logic
│   ├── watcher.rs      # File watcher using `notify` crate
│   ├── sync.rs         # File synchronization logic using `rayon`
├── source_dir/         # Source directory to be watched (manually created)
└── destination_dir/    # Destination directory where files are synchronized (manually created)
```

## Modules

- watcher.rs: This module sets up a watcher that monitors the source directory for changes. It spawns a thread that listens for file events and sends them to the main thread through a channel.
- sync.rs: This module handles the actual file synchronization. It uses rayon to parallelize the file copying process, speeding up the sync operation.

## How it works

1. The tool creates a channel for communication between the file watcher and the main loop.
2. The watcher listens for any changes in the source directory and sends events through the channel.
3. The main loop receives these events and triggers the synchronization process if a file is added, modified, or removed.
4. The sync_files function uses the rayon crate to copy files from the source to the destination in parallel, ensuring that synchronization happens as quickly as possible.
5. Files are only copied if they don’t already exist in the destination or if they have been modified.

## Dependencies

- [`notify`](https://crates.io/crates/notify): For watching file system changes.
- [`rayon`](https://crates.io/crates/rayon): For parallel file processing and synchronization.
