use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::sync::Mutex;

struct PreviewerState {
    last_texture_hash: String,
}

static STATE: Mutex<RefCell<Option<PreviewerState>>> = Mutex::new(RefCell::new(None));

const HTML_UI: &str = include_str!("../ui/index.html");
const THREE_JS: &str = include_str!("../ui/libs/three.min.js");
const ORBIT_JS: &str = include_str!("../ui/libs/OrbitControls.js");
const OBJ_JS: &str = include_str!("../ui/libs/OBJLoader.js");
const GLTF_JS: &str = include_str!("../ui/libs/GLTFLoader.js");
const CONTROLLER_JS: &str = include_str!("../ui/view_controller.js");

#[wasm_bindgen]
pub fn initialize_plugin() {
    let mut state = STATE.lock().unwrap();
    *state.borrow_mut() = Some(PreviewerState {
        last_texture_hash: String::new(),
    });
}

#[wasm_bindgen]
pub fn get_plugin_ui(asset_type: &str) -> String {
    match asset_type {
        "html" => HTML_UI.to_string(),
        "three" => THREE_JS.to_string(),
        "orbit" => ORBIT_JS.to_string(),
        "obj" => OBJ_JS.to_string(),
        "gltf" => GLTF_JS.to_string(),
        "controller" => CONTROLLER_JS.to_string(),
        _ => String::from(""),
    }
}

#[wasm_bindgen]
pub fn process_canvas_stream(base64_pixel_data: &str) -> String {
    let state_lock = STATE.lock().unwrap();
    let mut state_borrow = state_lock.borrow_mut();
    
    if let Some(ref mut state) = *state_borrow {
        state.last_texture_hash = format!("hash_{}", base64_pixel_data.len());
        return format!(
            "{{\"status\": \"success\", \"action\": \"update_mesh_texture\", \"data\": \"{}\"}}",
            base64_pixel_data
        );
    }
    String::from("{\"status\": \"error\"}")
}
