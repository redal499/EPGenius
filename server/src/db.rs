//db.rs

use rusqlite::{Connection, params};
use std::time::SystemTime;

pub fn get_db_conn() -> Connection {
    Connection::open("iptv.db").expect("failed to open sqlite connection")
}

pub fn initialize_db(){
    let conn = get_db_conn();
    conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();

    // Users
    conn.execute("
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            email TEXT UNIQUE,
            password TEXT,
            date_of_birth TEXT
        )
    ", []).unwrap();

    // Interests
    conn.execute("
        CREATE TABLE IF NOT EXISTS interests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            interest TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        )
    ", []).unwrap();

    // Simplified EPG
    conn.execute("
        CREATE TABLE IF NOT EXISTS epg (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_name TEXT,
            program_name TEXT,
            start_time TEXT,
            end_time TEXT,
            description TEXT
        )
    ", []).unwrap();
    println!("DB initialized");
}