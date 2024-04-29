// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem
};
// use tauri::api::process::Command;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn hello() {
    println!("hello, ")
}

#[tauri::command]
fn open_exe(exe_path: String) {
    // 使用操作系统的默认程序打开 .exe 文件
    Command::new(exe_path)
        .output()
        .expect("failed to open exe");
}

// #[tauri::command]
// fn run_script(script_path: String) {
//     // 使用 cmd 执行脚本文件
//     Command::new(script_path)
//             .output()
//             .expect("failed to run script");
// }

#[tauri::command]
fn run_script(script_path: String) {
    // 使用 cmd 执行脚本文件
    let status = Command::new("cmd")
        .args(&["/C", &script_path])
        .status()
        .expect("failed to execute process");

    if status.success() {
        println!("Successfully ran script");
    } else {
        println!("Failed to run script");
    }
}

fn main() {
    // add new menu item
    let quit = CustomMenuItem::new("quit".to_string(), "关闭窗口");
//     let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let tray_menu = SystemTrayMenu::new()
      .add_item(quit);
//       .add_native_item(SystemTrayMenuItem::Separator)
//       .add_item(hide);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
        // 暴露上面定义好的 greet 方法
        .invoke_handler(tauri::generate_handler![greet, open_exe, run_script])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// 托盘点击事件
fn menu_handle(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::LeftClick {
      position: _,
      size: _,
      ..
    } => {
      println!("鼠标-左击");
    }
    SystemTrayEvent::RightClick {
      position: _,
      size: _,
      ..
    } => {
      println!("鼠标-右击");
    }
    SystemTrayEvent::DoubleClick {
      position: _,
      size: _,
      ..
    } => {
      println!("鼠标-双击");
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "quit" => {
        std::process::exit(0);
      }
      "hide" => {
        let item_handle = app_handle.tray_handle().get_item(&id);
        let window = app_handle.get_window("main").unwrap();
        if window.is_visible().unwrap() {
          window.hide().unwrap();
          item_handle.set_title("显示窗口").unwrap();
        } else {
          window.show().unwrap();
          item_handle.set_title("隐藏窗口").unwrap();
        }
      }
      _ => {}
    },
    _ => {}
  }
}
