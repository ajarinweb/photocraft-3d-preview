let scene, camera, renderer, controls;
let modelGroup, currentTexture;
let axesScene, axesCamera, axesRenderer;

function init() {
    const container = document.getElementById('canvas3d');
    
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(45, container.clientWidth / container.clientHeight, 0.1, 1000);
    camera.position.set(2.5, 2.5, 4);

    renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setSize(container.clientWidth, container.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    controls = new THREE.OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;

    scene.add(new THREE.AmbientLight(0xffffff, 0.6));
    const shadowLight = new THREE.DirectionalLight(0xffffff, 0.7);
    shadowLight.position.set(4, 10, 4);
    scene.add(shadowLight);

    const grid = new THREE.GridHelper(12, 24, 0x00adb5, 0x393e46);
    grid.material.transparent = true;
    grid.material.opacity = 0.2;
    scene.add(grid);

    modelGroup = new THREE.Group();
    scene.add(modelGroup);

    setupDefaultBox();
    initAxesIndicator();
    
    window.addEventListener('resize', onResize, false);
    renderLoop();
    
    document.getElementById('statusDot').classList.add('active');
    document.getElementById('statusLabel').textContent = "Sandboxed Engine Ready";
}

function setupDefaultBox() {
    const geo = new THREE.BoxGeometry(1, 1, 1);
    const mat = new THREE.MeshStandardMaterial({ color: 0x90a4ae, roughness: 0.4 });
    const mesh = new THREE.Mesh(geo, mat);
    mesh.position.y = 0.5;
    modelGroup.add(mesh);
}

function initAxesIndicator() {
    const axesContainer = document.getElementById('axesIndicator');
    axesScene = new THREE.Scene();
    axesCamera = new THREE.PerspectiveCamera(50, 1, 0.1, 10);
    axesCamera.position.z = 2.5;

    axesRenderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    axesRenderer.setSize(80, 100);
    axesContainer.appendChild(axesRenderer.domElement);

    const helper = new THREE.AxesHelper(1.2);
    axesScene.add(helper);
}

function renderLoop() {
    requestAnimationFrame(renderLoop);
    controls.update();
    renderer.render(scene, camera);

    axesCamera.position.copy(camera.position);
    axesCamera.position.sub(controls.target).setLength(2.5);
    axesCamera.lookAt(axesScene.position);
    axesRenderer.render(axesScene, axesCamera);
}

function applyIncomingTexture(base64String) {
    const img = new Image();
    img.onload = function() {
        if (currentTexture) currentTexture.dispose();
        currentTexture = new THREE.Texture(img);
        currentTexture.needsUpdate = true;
        currentTexture.colorSpace = THREE.SRGBColorSpace;

        modelGroup.traverse(child => {
            if (child.isMesh) {
                child.material.map = currentTexture;
                child.material.needsUpdate = true;
            }
        });
    };
    img.src = `data:image/png;base64,${base64String}`;
}

document.getElementById('fileMesh').addEventListener('change', function(e) {
    const file = e.target.files[0];
    if (!file) return;

    const ext = file.name.split('.').pop().toLowerCase();
    const reader = new FileReader();

    reader.onload = function(evt) {
        const data = evt.target.result;
        
        while(modelGroup.children.length > 0) { 
            modelGroup.remove(modelGroup.children[0]); 
        }

        if (ext === 'obj') {
            const obj = new THREE.OBJLoader().parse(data);
            adjustAndNormalizeMesh(obj);
        } else {
            new THREE.GLTFLoader().parse(data, '', function(gltf) {
                adjustAndNormalizeMesh(gltf.scene);
            });
        }
    };

    if (ext === 'obj') reader.readAsText(file);
    else reader.readAsArrayBuffer(file);
});

function adjustAndNormalizeMesh(importedObject) {
    importedObject.traverse(child => {
        if (child.isMesh) {
            child.material = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.5 });
            if (currentTexture) child.material.map = currentTexture;
        }
    });

    const box = new THREE.Box3().setFromObject(importedObject);
    const size = box.getSize(new THREE.Vector3());
    const center = box.getCenter(new THREE.Vector3());

    importedObject.position.x = -center.x;
    importedObject.position.y = -box.min.y;
    importedObject.position.z = -center.z;

    controls.target.set(0, size.y / 2, 0);
    const boundingDimension = Math.max(size.x, size.y, size.z);
    camera.position.set(boundingDimension * 1.6, boundingDimension * 1.6, boundingDimension * 1.6);
    
    modelGroup.add(importedObject);
}

function onResize() {
    const container = document.getElementById('canvas3d');
    camera.aspect = container.clientWidth / container.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(container.clientWidth, container.clientHeight);
}

document.addEventListener('DOMContentLoaded', init);