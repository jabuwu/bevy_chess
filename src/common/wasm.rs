use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        _app.add_startup_system(wasm_title)
            .add_system(wasm_fullscreen);
    }
}

#[cfg(target_arch = "wasm32")]
fn wasm_title() {
    let web_window = web_sys::window().unwrap();
    let document = web_window.document().unwrap();
    document.set_title(&format!("Bevy Chess v{}", VERSION));
}

#[cfg(target_arch = "wasm32")]
fn wasm_fullscreen(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        let web_window = web_sys::window().unwrap();
        let document_element = web_window.document().unwrap().document_element().unwrap();
        window.set_resolution(
            document_element.client_width() as f32,
            document_element.client_height() as f32,
        );
    }
}
