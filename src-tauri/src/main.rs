// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod cryptography;
mod database;

fn main() {
    cypher_app_lib::run()
}
