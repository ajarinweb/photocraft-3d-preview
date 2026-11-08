use std::cell::RefCell;
use std::sync::Mutex;

struct PreviewerState {
    last_texture_size: usize,
}

static STATE: Mutex<RefCell<Option<PreviewerState>>> = Mutex::new(RefCell::new(None));

const HTML_UI: &str = include_str!("../ui/index.html");
const THREE_JS: &str = include_str!("../ui/libs/three.min.js");
const ORBIT_JS: &str = include_str!("../ui/libs/OrbitControls.js");
const OBJ_JS: &str = include_str!("../ui/libs/OBJLoader.js");
const GLTF_JS: &str = include_str!("../ui/libs/GLTFLoader.js");
const CONTROLLER_JS: &str = include_str!("../ui/view_controller.js");

#[no_mangle]
pub extern "C" fn allocate_memory(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Mencegah deallokasi otomatis oleh Rust garbage collector
    ptr
}

fn return_string_to_host(s: String) -> *mut u8 {
    let mut bytes = s.into_bytes();
    bytes.push(0); // Tambahkan null-terminator di akhir string
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    ptr
}

#[no_mangle]
pub extern "C" fn initialize_plugin() {
    let mut state = STATE.lock().unwrap();
    *state.borrow_mut() = Some(PreviewerState {
        last_texture_size: 0,
    });
}

#[no_mangle]
pub extern "C" fn get_plugin_ui(asset_type_ptr: *const u8) -> *mut u8 {
    let asset_type = unsafe {
        std::fs::read_to_string(std::ffi::CStr::from_ptr(asset_type_ptr as *const i8).to_string_lossy().into_owned())
            .unwrap_or_else(|_| String::from("html")) 
    };

    let response = match asset_type.as_str() {
        "html" => HTML_UI.to_string(),
        "three" => THREE_JS.to_string(),
        "orbit" => ORBIT_JS.to_string(),
        "obj" => OBJ_JS.to_string(),
        "gltf" => GLTF_JS.to_string(),
        "controller" => CONTROLLER_JS.to_string(),
        _ => String::from(""),
    };

    return_string_to_host(response)
}

#[no_mangle]
pub extern "C" fn process_canvas_stream(base64_data_ptr: *const u8) -> *mut u8 {
    let raw_str = unsafe {
        std::ffi::CStr::from_ptr(base64_data_ptr as *const i8).to_string_lossy().into_owned()
    };

    let state_lock = STATE.lock().unwrap();
    let mut state_borrow = state_lock.borrow_mut();
    
    if let Some(ref mut state) = *state_borrow {
        state.last_texture_size = raw_str.len();
        let success_json = format!(
            "{{\"status\": \"success\", \"action\": \"update_mesh_texture\", \"data\": \"{}\"}}",
            raw_str
        );
        return return_string_to_host(success_json);
    }
    
    return_string_to_host(String::from("{\"status\": \"error\"}"))
}

#[no_mangle]
pub extern "C" fn on_plugin_tick() -> *mut u8 {
    return_string_to_host(String::from(
        "{\"command\": \"request_active_canvas_layer\", \"format\": \"base64\"}"
    ))
}
