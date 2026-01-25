// web/app.js
import init, { WasmCompiler } from './pkg/compiler_wasm.js';

// DSL Examples
const EXAMPLES = {
    cube: `scene {
  name: "Simple Cube"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

library_imports {
  math: "core_mechanics"
  geometry: "basic_solids"
}

entity cube1 {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: cube
    }
    physical {
      mass: 1.0
      rigid: true
    }
  }
}`,

    spinning: `scene {
  name: "Spinning Cube"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

library_imports {
  math: "core_mechanics"
  geometry: "basic_solids"
}

entity cube1 {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: cube
    }
    physical {
      mass: 1.0
      rigid: true
    }
  }
}

motion spin {
  target: cube1
  type: rotation
  axis: [0, 1, 0]
  speed: 1.5708
}

timeline main {
  event {
    motion: spin
    start: 0.0
    duration: 10.0
  }
}`,

    multi: `scene {
  name: "Multiple Shapes"
  version: 1
  ir_version: "0.1.0"
  unit_system: "SI"
}

library_imports {
  math: "core_mechanics"
  geometry: "basic_solids"
}

entity cube1 {
  kind: solid
  components {
    transform {
      position: [-2, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: cube
    }
    physical {
      mass: 1.0
      rigid: true
    }
  }
}

entity sphere1 {
  kind: solid
  components {
    transform {
      position: [0, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: sphere
    }
    physical {
      mass: 1.0
      rigid: true
    }
  }
}

entity cylinder1 {
  kind: solid
  components {
    transform {
      position: [2, 0, 0]
      rotation: [0, 0, 0]
      scale: [1, 1, 1]
    }
    geometry {
      primitive: cylinder
    }
    physical {
      mass: 1.0
      rigid: true
    }
  }
}`
};

// Three.js setup
let scene, camera, renderer, compiler;
let renderObjects = new Map();
let isPlaying = true;
let frameCount = 0;
let lastFpsUpdate = 0;

async function main() {
    // Initialize WASM
    await init();
    console.log('WASM initialized');
    
    // Create compiler instance
    compiler = new WasmCompiler();
    
    // Setup Three.js
    setupThreeJS();
    
    // Setup UI
    setupUI();
    
    // Load default example
    loadExample('cube');
    
    // Start render loop
    animate();
}

function setupThreeJS() {
    const canvas = document.getElementById('renderCanvas');
    
    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1a2e);
    
    // Camera
    camera = new THREE.PerspectiveCamera(
        75,
        canvas.clientWidth / canvas.clientHeight,
        0.1,
        1000
    );
    camera.position.set(3, 3, 5);
    camera.lookAt(0, 0, 0);
    
    // Renderer
    renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    renderer.shadowMap.enabled = true;
    
    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 5);
    directionalLight.castShadow = true;
    scene.add(directionalLight);
    
    // Grid
    const gridHelper = new THREE.GridHelper(10, 10);
    scene.add(gridHelper);
    
    // Orbit controls (optional, requires OrbitControls.js)
    // const controls = new THREE.OrbitControls(camera, canvas);
}

function setupUI() {
    const compileBtn = document.getElementById('compileBtn');
    const playBtn = document.getElementById('playBtn');
    const resetBtn = document.getElementById('resetBtn');
    const exampleSelect = document.getElementById('exampleSelect');
    
    compileBtn.addEventListener('click', compileAndRender);
    playBtn.addEventListener('click', togglePlay);
    resetBtn.addEventListener('click', reset);
    exampleSelect.addEventListener('change', (e) => loadExample(e.target.value));
}

function loadExample(name) {
    const editor = document.getElementById('dslEditor');
    editor.value = EXAMPLES[name];
}

async function compileAndRender() {
    const dslSource = document.getElementById('dslEditor').value;
    const errorLog = document.getElementById('errorLog');
    const status = document.getElementById('status');
    
    try {
        // Clear previous errors
        errorLog.textContent = '';
        errorLog.style.display = 'none';
        
        // Clear existing objects
        clearScene();
        
        // Compile DSL
        status.textContent = 'Compiling...';
        const result = compiler.compile(dslSource);
        
        status.textContent = 'Compiled successfully! Rendering...';
        
        // Get initial snapshot
        updateScene();
        
        // Enable play button
        document.getElementById('playBtn').disabled = false;
        
        status.textContent = 'Ready - Scene loaded';
    } catch (error) {
        errorLog.textContent = `Error: ${error}`;
        errorLog.style.display = 'block';
        status.textContent = 'Compilation failed';
        console.error('Compilation error:', error);
    }
}

function clearScene() {
    // Remove all render objects
    renderObjects.forEach(mesh => {
        scene.remove(mesh);
        mesh.geometry.dispose();
        mesh.material.dispose();
    });
    renderObjects.clear();
}

function updateScene() {
    try {
        const snapshot = compiler.get_snapshot();
        
        // Update info
        document.getElementById('sceneInfo').textContent = 
            `Objects: ${snapshot.objects.length} | Tick: ${snapshot.tick}`;
        
        // Update or create objects
        snapshot.objects.forEach(obj => {
            const geom = JSON.parse(obj.geometry);
            const transform = JSON.parse(obj.transform);
            
            let mesh = renderObjects.get(obj.id);
            
            if (!mesh) {
                // Create new mesh
                mesh = createMesh(geom);
                scene.add(mesh);
                renderObjects.set(obj.id, mesh);
            }
            
            // Update transform
            mesh.position.set(
                transform.position[0],
                transform.position[1],
                transform.position[2]
            );
            
            mesh.quaternion.set(
                transform.rotation[0],
                transform.rotation[1],
                transform.rotation[2],
                transform.rotation[3]
            );
            
            mesh.scale.set(
                transform.scale[0],
                transform.scale[1],
                transform.scale[2]
            );
            
            mesh.visible = obj.visible;
        });
    } catch (error) {
        console.error('Scene update error:', error);
    }
}

function createMesh(geomData) {
    let geometry;
    
    switch (geomData.type) {
        case 'sphere':
            geometry = new THREE.SphereGeometry(geomData.radius, 32, 32);
            break;
        case 'box':
            geometry = new THREE.BoxGeometry(
                geomData.width,
                geomData.height,
                geomData.depth
            );
            break;
        case 'cylinder':
            geometry = new THREE.CylinderGeometry(
                geomData.radius,
                geomData.radius,
                geomData.height,
                32
            );
            break;
        default:
            geometry = new THREE.BoxGeometry(1, 1, 1);
    }
    
    const material = new THREE.MeshStandardMaterial({
        color: 0x4488ff,
        metalness: 0.3,
        roughness: 0.7,
    });
    
    const mesh = new THREE.Mesh(geometry, material);
    mesh.castShadow = true;
    mesh.receiveShadow = true;
    
    return mesh;
}

function animate() {
    requestAnimationFrame(animate);
    
    // Step simulation
    if (isPlaying && compiler) {
        try {
            compiler.step();
            updateScene();
        } catch (error) {
            console.error('Step error:', error);
        }
    }
    
    // Render
    renderer.render(scene, camera);
    
    // Update FPS
    frameCount++;
    const now = performance.now();
    if (now - lastFpsUpdate > 1000) {
        document.getElementById('fps').textContent = `FPS: ${frameCount}`;
        frameCount = 0;
        lastFpsUpdate = now;
    }
}

function togglePlay() {
    isPlaying = !isPlaying;
    const btn = document.getElementById('playBtn');
    btn.textContent = isPlaying ? '⏸ Pause' : '▶ Play';
}

function reset() {
    const exampleName = document.getElementById('exampleSelect').value;
    loadExample(exampleName);
    compileAndRender();
}

// Handle window resize
window.addEventListener('resize', () => {
    const canvas = document.getElementById('renderCanvas');
    camera.aspect = canvas.clientWidth / canvas.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
});

// Start the app
main().catch(console.error);