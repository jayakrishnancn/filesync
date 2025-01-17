use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::compare::State;

pub struct SyncState {
    conn: Mutex<Connection>,
}

impl SyncState {
    pub fn new(state_file: &str) -> Self {
        let conn = Connection::open(state_file).expect("Failed to open or create database file.");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sync_state (
                path TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                action TEXT NOT NULL,
                checksum TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create sync_state table.");
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn mark_multiple(&self, items: Vec<State>) {
        let mut conn = self.conn.lock().unwrap(); // Declare as mutable
                                                  // Use a transaction for efficiency when inserting multiple rows
        let transaction = conn.transaction().expect("Failed to start transaction.");

        for state in items {
            transaction
                .execute(
                    "INSERT OR REPLACE INTO sync_state (path, status, action, checksum) VALUES (?1, ?2, ?3, ?4)",
                    params![state.path, "marked", state.action.to_string(), state.hash ],
                )
                .expect("Failed to update sync state.");
        }
        transaction.commit().expect("Failed to commit transaction.");
        println!("marked all.")
    }
}
