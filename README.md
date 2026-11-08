# 🎨 PhotoCraft 3D Preview Plugin

[![MIT License](https://shields.io)](LICENSE)
[![PRs Welcome](https://shields.io)](https://github.com)
[![PhotoCraft Compatibility](https://shields.io)](https://github.com/storytold/photocraft)
![PhotoCraft Plugin](https://shields.io)
![Rust Wasm](https://shields.io)
![License MIT](https://shields.io)
![Status Offline](https://shields.io)

An open-source internal plugin for **PhotoCraft** that allows designers to load custom 3D models directly into the editor’s UI and view fabric/pattern texture projections in *real time* whilst editing the canvas. Inspired by the workflow of the iconic `Photoshop-3D-Preview` repository by tylerpelaez.

---

## 🚀 Key Features
* **Wasm Binary Sandbox Isolation:** Runs within a secure `.wasm` binary sandbox using the PhotoCraft Wasmi runtime.
* **Real-Time Canvas Synchronisation:** Monitors active manipulations on PhotoCraft layers and updates texture maps instantly without lag.
* **Auto-Scaling & Centring:** Automatically calculates the bounding box size of custom models (OBJ/GLTF/GLB) to centre them within the viewport.
* **Memory Allocation Optimisation:** A managed memory system within WebAssembly to handle texture resolutions up to 4K.

---

## 🛠️ System Architecture

This plugin uses a **Hybrid Wasm Architecture** approach:
1. **Core Sandbox (Rust):** The `.wasm` binary file manages texture state, event tick synchronisation, secure inter-process communication, and binary data mesh manipulation.
2. **WebView UI Renderer (WebGL/Three.js):** The interface side receives an encrypted binary payload from the sandbox to display the Orbit 3D camera projection.

---

### 🔄 How Does the PhotoCraft Host Load This Plugin?

When a user clicks the **‘Open’** button in the PhotoCraft `.wasm` binary installation menu, the internal Wasmi runtime executes the following lifecycle:

1. **Instantiation:** The host reads the `.wasm` file and calls the `initialize_plugin()` function.
2. **UI Intercept:** The host calls `get_plugin_ui(‘html’)`, `get_plugin_ui(‘three’)`, etc. All JavaScript text strings are merged into the application’s internal WebView DOM binary without making any HTTP requests (100% offline and local).
3. **Data Piping:** Every time the user moves the brush on the PhotoCraft canvas, the host application calls the `process_canvas_stream(data)` function within the sandboxed memory, passing the data instantly to the Three.js render function.

---

## 📦 Quick Installation Guide

### For End Users
As this plugin is a *Full-Offline Single Bundle*, you only need a single binary file – no fuss!
1. Open the **Releases** tab on this GitHub repository, then download the latest binary release file named `photocraft_3d_preview.wasm`.
2. Open your **PhotoCraft** application.
3. Go to the **Plug-in Panel** menu or click the **‘Install Plug-in’** button available in the application interface.
4. Select the `photocraft_3d_preview.wasm` file you have just downloaded.
5. The **3D Preview** panel will appear immediately within the application and is 100% ready for offline use!

### For Developers & Contributors (Local Build)
If you wish to customise the code and compile it yourself locally:

```bash
# 1. Clone the repository
git clone https://github.com
cd photocraft-3d-preview

# 2. Register the WebAssembly compilation target with your Rust compiler
rustup target add wasm32-unknown-unknown

# 3. Compile the binary with size-optimisation
cargo build --target wasm32-unknown-unknown --release
```
The single `.wasm` binary, which already bundles all internal UI assets, can be found in the directory:
`target/wasm32-unknown-unknown/release/photocraft_3d_preview.wasm`

---

## 🤝 Contributions
Your contributions are greatly appreciated! Please open an *Issue* to report a bug or create a *Pull Request* directly.

## 📄 Licence
This project is licensed under the **MIT Licence** – see the [LICENSE](LICENSE) file for full details.
