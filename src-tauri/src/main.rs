// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn app_menu() -> Menu {
    let m_new = CustomMenuItem ::new("new".to_string(), "New" );
    let m_open = CustomMenuItem ::new("open".to_string(), "Open" );
    let m_quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let submenu = MenuEntry::Submenu(
        Submenu::new("File",
                     Menu::with_items([m_new.into(), m_open.into() , m_quit.into()])));

    let menu = Menu::with_items([submenu]);
    menu
}

fn main() {
    let menu = app_menu();
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => { std::process::exit(0); }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
