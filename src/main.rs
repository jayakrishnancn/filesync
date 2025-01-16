use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::{
    env, fs, io,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

struct SyncState {
    conn: Mutex<Connection>,
}

impl SyncState {
    fn new(state_file: &str) -> Self {
        let conn = Connection::open(state_file).expect("Failed to open or create database file.");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sync_state (
                path TEXT PRIMARY KEY,
                status TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create sync_state table.");
        Self {
            conn: Mutex::new(conn),
        }
    }

    fn mark_completed(&self, path: &Path) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sync_state (path, status) VALUES (?1, 'completed')",
            params![path.to_string_lossy()],
        )
        .expect("Failed to mark file as completed.");
    }

    fn mark_skipped(&self, path: &Path) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sync_state (path, status) VALUES (?1, 'skipped')",
            params![path.to_string_lossy()],
        )
        .expect("Failed to mark file as skipped.");
    }

    fn is_completed(&self, path: &Path) -> bool {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT 1 FROM sync_state WHERE path = ?1 AND status = 'completed'")
            .expect("Failed to prepare statement.");
        stmt.exists(params![path.to_string_lossy()])
            .expect("Failed to check completion status.")
    }
}

fn sync_directory(source: &Path, destination: &Path, state: Arc<SyncState>) -> io::Result<()> {
    if !source.exists() {
        eprintln!("Source directory does not exist: {:?}", source);
        exit(1);
    }

    let entries: Vec<_> = fs::read_dir(source)?.collect::<Result<_, _>>()?;

    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        let relative_path = path.strip_prefix(source).unwrap();
        let destination_path = destination.join(relative_path);

        if state.is_completed(&path) {
            println!("Skipping already completed: {:?}", path);
            return;
        }
        // Ensure the destination directory exists or create it
        if !destination.exists() {
            println!(
                "Destination directory {:?} does not exist. Creating it...",
                destination
            );
            if let Err(e) = fs::create_dir_all(destination) {
                eprintln!(
                    "Failed to create destination directory {:?}: {:?}",
                    destination, e
                );
                exit(1);
            }
        }

        if path.is_dir() {
            println!("Entering directory: {:?}", path);
            if let Err(e) = fs::create_dir_all(&destination_path) {
                eprintln!("Failed to create directory {:?}: {:?}", destination_path, e);
            } else {
                sync_directory(&path, &destination_path, Arc::clone(&state)).unwrap_or_else(|e| {
                    eprintln!("Failed to sync directory {:?}: {:?}", path, e);
                });
            }
        } else if path.is_file() {
            println!("Copying file: {:?} to {:?}", path, destination_path);
            if let Err(e) = fs::copy(&path, &destination_path) {
                eprintln!("Failed to copy {:?}: {:?}", path, e);
                state.mark_skipped(&path);
            } else {
                state.mark_completed(&path);
            }
        }
    });

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <source_directory> <destination_directory>",
            args[0]
        );
        exit(1);
    }

    let source_dir = Path::new(&args[1])
        .canonicalize()
        .expect("Failed to get absolute path");
    let destination = Path::new(&args[2]);
    if !destination.exists() {
        println!(
            "Destination directory {:?} does not exist. Creating it...",
            destination
        );
        if let Err(e) = fs::create_dir_all(destination) {
            eprintln!(
                "Failed to create destination directory {:?}: {:?}",
                destination, e
            );
            exit(1);
        }
    }
    let destination_dir = destination
        .canonicalize()
        .expect("Failed to get absolute path");
    let state_file = "sync_state-1.db";

    println!("Initializing synchronization state...");
    let state = Arc::new(SyncState::new(state_file));

    println!(
        "Starting synchronization from {:?} to {:?}...",
        source_dir.as_path(),
        destination_dir
    );

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()
        .expect("Failed to build thread pool");

    sync_directory(
        source_dir.as_path(),
        destination_dir.as_path(),
        Arc::clone(&state),
    )?;

    println!("Synchronization completed.");
    Ok(())
}
