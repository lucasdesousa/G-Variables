#[path = "extension.rs"]
mod extension;

#[tauri::command]
async fn hello_world_command(_app: tauri::AppHandle) -> Result<String, String> {
    println!("I was invoked from JS!");
    Ok("Hello world from Tauri!".into())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn greet2() -> String {
    // extension::minha_funcao();
    format!("Greet2!")
}

pub fn show() {
    println!("G-Variables -> ui.rs -> showed!");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, hello_world_command, greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
