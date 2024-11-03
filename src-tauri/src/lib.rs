use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

#[tauri::command]
fn read_csv() -> String {
    let file_path = "./timetable.csv";
    let default_value = "start,end,星期一,星期二,星期三,星期四,星期五,星期六,星期日\n6:00,6:30,语文,数学,英语,语文,数学,英语,语文\n7:00,7:30,语文,数学,英语,语文,数学,英语,语文\n8:00,8:30,语文,数学,英语,语文,数学,英语,语文\n9:00,9:30,语文,数学,英语,语文,数学,英语,语文\n9:30,10:00,语文,数学,英语,语文,数学,英语,语文\n10:00,10:30,-,-,-,-,-,-,-\n11:00,11:30,语文,数学,英语,语文,数学,英语,语文\n12:00,12:30,语文,数学,英语,语文,数学,英语,语文\n13:00,13:30,语文,数学,英语,语文,数学,英语,语文\n14:00,14:30,语文,数学,英语,语文,数学,英语,语文\n15:00,15:30,语文,数学,英语,语文,数学,英语,语文\n16:00,16:30,-,-,-,-,-,-,-\n17:00,17:30,语文,数学,英语,语文,数学,英语,语文\n18:00,18:30,语文,数学,英语,语文,数学,英语,语文\n19:00,19:30,语文,数学,英语,语文,数学,英语,语文".to_string();
    if Path::new(file_path).exists() {
        let contents = fs::read_to_string(file_path).expect("");
        return contents;
    } else {
        if let Ok(mut file) = File::create(file_path) {
            if file.write_all(default_value.as_bytes()).is_err() {
                return default_value.to_string();
            }
        }
        return default_value.to_string()
    }
}
#[tauri::command]
fn open_csv() {
    let file_path = "./timetable.csv";
    Command::new("cmd")
        .args(&["/C", "start", "", file_path])
        .spawn()
        .expect("Failed to open file");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None,))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![read_csv, open_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
