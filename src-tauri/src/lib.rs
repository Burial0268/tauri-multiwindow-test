use tauri::{Emitter, Manager};
use tauri::Url;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str, handle: tauri::AppHandle) -> Result<String, ()> {
    let url = "https://login.live.com/oauth20_authorize.srf?client_id=aaaf3f8c-8b99-4e7b-8265-b637bd89317e&response_type=code&redirect_uri=http://localhost:8080&scope=XboxLive.signin%20offline_access&state=NOT_NEEDED";
    let url = Url::parse(url).unwrap();
    let webview_window = tauri::webview::WebviewWindowBuilder::new(&handle.clone(), "ms", tauri::WebviewUrl::External(url))
        .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
        .on_navigation(move |url| {
        if url.to_string().contains("code=") {
            let _ = handle.emit("oauth:ms:callback_url", url.to_string());
            if let Some(w) = handle.webview_windows().get("ms") {
                let _ = w.close();
            }
        }
        true
    })
    .build()
    .unwrap();
    Ok(format!("Hello {name}, You have been greeted from Rust!"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
