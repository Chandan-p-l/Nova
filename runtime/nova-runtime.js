/**
 * NOVA Browser 3D Runtime Engine (v0.1.0)
 * High-performance WebGL / WebGPU rendering engine with reactive state bridge.
 */

import * as THREE from 'https://unpkg.com/three@0.160.0/build/three.module.js';

export class NovaEngine {
    constructor(options = {}) {
        this.container = options.canvasContainer || document.body;
        this.scenes = new Map();
        this.activeScene = null;
        this.stateStore = new Map();
        this.stateListeners = new Map();
        this.updateCallbacks = [];
        this.scrollCallbacks = [];
        this.raycaster = new THREE.Raycaster();
        this.mouse = new THREE.Vector2();
        this.interactiveObjects = [];

        this.initRenderer();
        this.initEvents();
    }

    initRenderer() {
        this.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
        this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
        this.renderer.toneMappingExposure = 1.3;

        this.canvas = this.renderer.domElement;
        this.container.appendChild(this.canvas);
    }

    initEvents() {
        window.addEventListener('resize', () => {
            if (!this.activeScene) return;
            const width = window.innerWidth;
            const height = window.innerHeight;
            this.renderer.setSize(width, height);
            if (this.activeScene.camera) {
                this.activeScene.camera.aspect = width / height;
                this.activeScene.camera.updateProjectionMatrix();
            }
        });

        window.addEventListener('scroll', () => {
            const scrollMax = document.documentElement.scrollHeight - window.innerHeight;
            const progress = scrollMax > 0 ? window.scrollY / scrollMax : 0;
            this.scrollCallbacks.forEach(cb => cb(progress));
        });

        const handlePointer = (event, type) => {
            this.mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
            this.mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

            if (this.activeScene && this.activeScene.camera) {
                this.raycaster.setFromCamera(this.mouse, this.activeScene.camera);
                const intersects = this.raycaster.intersectObjects(this.interactiveObjects, true);

                if (intersects.length > 0) {
                    let hitObj = intersects[0].object;
                    while (hitObj && !hitObj.__novaModel) {
                        hitObj = hitObj.parent;
                    }
                    if (hitObj && hitObj.__novaModel) {
                        hitObj.__novaModel.trigger(type);
                    }
                }
            }
        };

        window.addEventListener('click', (e) => handlePointer(e, 'click'));
        window.addEventListener('pointermove', (e) => handlePointer(e, 'hover'));
    }

    createState(key, initialValue) {
        this.stateStore.set(key, initialValue);
        if (!this.stateListeners.has(key)) {
            this.stateListeners.set(key, []);
        }
        return initialValue;
    }

    setState(key, newValue) {
        this.stateStore.set(key, newValue);
        const listeners = this.stateListeners.get(key) || [];
        listeners.forEach(cb => cb(newValue));
    }

    subscribeState(key, callback) {
        if (!this.stateListeners.has(key)) {
            this.stateListeners.set(key, []);
        }
        this.stateListeners.get(key).push(callback);
    }

    getState(key) {
        return this.stateStore.get(key);
    }

    createScene(name) {
        const sceneObj = new NovaScene(name, this);
        this.scenes.set(name, sceneObj);
        if (!this.activeScene) {
            this.activeScene = sceneObj;
        }
        return sceneObj;
    }

    setActiveScene(name) {
        if (this.scenes.has(name)) {
            this.activeScene = this.scenes.get(name);
        }
    }

    setTargetProperty(path, value) {
        if (!this.activeScene) return;
        if (path === 'camera.position.z' && this.activeScene.camera) {
            this.activeScene.camera.position.z = value;
        }
    }

    onUpdate(callback) {
        this.updateCallbacks.push(callback);
    }

    onScroll(callback) {
        this.scrollCallbacks.push(callback);
    }

    start() {
        const clock = new THREE.Clock();
        const animate = () => {
            requestAnimationFrame(animate);

            const delta = clock.getDelta();
            const elapsedTime = clock.getElapsedTime();

            this.updateCallbacks.forEach(cb => cb(delta, elapsedTime));

            if (this.activeScene) {
                this.activeScene.update(delta, elapsedTime);
                if (this.activeScene.camera) {
                    this.renderer.render(this.activeScene.threeScene, this.activeScene.camera);
                }
            }
        };
        animate();
    }
}

class NovaScene {
    constructor(name, engine) {
        this.name = name;
        this.engine = engine;
        this.threeScene = new THREE.Scene();
        this.camera = null;
        this.models = new Map();
        this.lights = new Map();

        this.threeScene.background = new THREE.Color('#06060c');
        this.initStarfield();
    }

    initStarfield() {
        const count = 1200;
        const geometry = new THREE.BufferGeometry();
        const positions = new Float32Array(count * 3);
        const colors = new Float32Array(count * 3);

        for (let i = 0; i < count; i++) {
            positions[i * 3] = (Math.random() - 0.5) * 80;
            positions[i * 3 + 1] = (Math.random() - 0.5) * 80;
            positions[i * 3 + 2] = (Math.random() - 0.5) * 80;

            colors[i * 3] = 0.5 + Math.random() * 0.5;
            colors[i * 3 + 1] = 0.6 + Math.random() * 0.4;
            colors[i * 3 + 2] = 1.0;
        }

        geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
        geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        const material = new THREE.PointsMaterial({
            size: 0.15,
            vertexColors: true,
            transparent: true,
            opacity: 0.8,
        });

        const starfield = new THREE.Points(geometry, material);
        this.starfield = starfield;
        this.threeScene.add(starfield);
    }

    setEnvironment(props) {
        if (props.background) {
            this.threeScene.background = new THREE.Color(props.background);
        }
    }

    addCamera(name, props) {
        const fov = props.fov || 45;
        const aspect = window.innerWidth / window.innerHeight;
        const cam = new THREE.PerspectiveCamera(fov, aspect, 0.1, 1000);

        if (props.position) {
            cam.position.set(props.position.x || 0, props.position.y || 1.5, props.position.z || 6);
        } else {
            cam.position.set(0, 1.5, 6);
        }
        cam.lookAt(0, 0, 0);
        this.camera = cam;
        return cam;
    }

    addLight(name, props) {
        let light;
        const color = props.color || 0xffffff;
        const intensity = props.intensity || 1.0;

        if (props.type === 'directional' || !props.type) {
            light = new THREE.DirectionalLight(color, intensity * 2);
            if (props.position) {
                light.position.set(props.position.x || 5, props.position.y || 10, props.position.z || 5);
            } else {
                light.position.set(5, 10, 5);
            }
        } else {
            light = new THREE.AmbientLight(color, intensity);
        }

        this.threeScene.add(light);
        this.lights.set(name, light);

        const amb = new THREE.AmbientLight(0x6366f1, 0.5);
        this.threeScene.add(amb);

        return light;
    }

    addModel(name, source, props) {
        const modelObj = new NovaModel(name, source, props, this);
        this.models.set(name, modelObj);
        return modelObj;
    }

    addMesh(name, geometryType, materialName, props) {
        let geo;
        if (geometryType === 'box') {
            geo = new THREE.BoxGeometry(1.5, 1.5, 1.5);
        } else {
            geo = new THREE.SphereGeometry(1, 32, 32);
        }

        const mat = new THREE.MeshStandardMaterial({
            color: 0x6366f1,
            metalness: 0.8,
            roughness: 0.2,
        });

        const mesh = new THREE.Mesh(geo, mat);
        if (props.position) {
            mesh.position.set(props.position.x || 0, props.position.y || 0, props.position.z || 0);
        }
        this.threeScene.add(mesh);
        return mesh;
    }

    update(delta, elapsedTime) {
        if (this.starfield) {
            this.starfield.rotation.y = elapsedTime * 0.02;
        }
        this.models.forEach(model => model.update(delta, elapsedTime));
    }
}

class NovaModel {
    constructor(name, source, props, novaScene) {
        this.name = name;
        this.source = source;
        this.novaScene = novaScene;
        this.eventListeners = new Map();

        const group = new THREE.Group();

        // 3D Core Geometry: Futuristic Cyber Gem
        const mainGeo = new THREE.IcosahedronGeometry(1.4, 1);
        const mainMat = new THREE.MeshStandardMaterial({
            color: 0x6366f1,
            metalness: 0.85,
            roughness: 0.15,
            emissive: 0x312e81,
            emissiveIntensity: 0.6,
            wireframe: false,
        });
        const mainMesh = new THREE.Mesh(mainGeo, mainMat);
        this.mainMesh = mainMesh;
        this.mainMat = mainMat;
        group.add(mainMesh);

        // Orbiting Cyber Ring 1
        const ring1Geo = new THREE.TorusGeometry(2.1, 0.04, 16, 100);
        const ring1Mat = new THREE.MeshBasicMaterial({ color: 0x38bdf8, wireframe: true });
        const ring1Mesh = new THREE.Mesh(ring1Geo, ring1Mat);
        ring1Mesh.rotation.x = Math.PI / 3;
        group.add(ring1Mesh);
        this.ring1Mesh = ring1Mesh;
        this.ring1Mat = ring1Mat;

        // Orbiting Cyber Ring 2
        const ring2Geo = new THREE.TorusGeometry(2.6, 0.03, 16, 100);
        const ring2Mat = new THREE.MeshBasicMaterial({ color: 0xa855f7, wireframe: true });
        const ring2Mesh = new THREE.Mesh(ring2Geo, ring2Mat);
        ring2Mesh.rotation.y = Math.PI / 4;
        group.add(ring2Mesh);
        this.ring2Mesh = ring2Mesh;
        this.ring2Mat = ring2Mat;

        group.__novaModel = this;
        this.group = group;

        if (props.position) {
            group.position.set(props.position.x || 0, props.position.y || 0, props.position.z || 0);
        }

        this.novaScene.threeScene.add(group);
        this.novaScene.engine.interactiveObjects.push(group);

        // Listen for color state mutations if registered
        this.novaScene.engine.subscribeState('activeColor', (newColor) => {
            this.setColor(newColor);
        });
    }

    setColor(colorHex) {
        if (this.mainMat) {
            this.mainMat.color.set(colorHex);
            this.mainMat.emissive.set(colorHex);
        }
        if (this.ring1Mat) {
            this.ring1Mat.color.set(colorHex);
        }
    }

    get rotation() {
        return this.group.rotation;
    }

    get position() {
        return this.group.position;
    }

    get scale() {
        return this.group.scale;
    }

    set scale(val) {
        if (typeof val === 'number') {
            this.group.scale.set(val, val, val);
        } else if (val && typeof val === 'object') {
            this.group.scale.set(val.x || 1, val.y || 1, val.z || 1);
        }
    }

    on(event, handler) {
        if (!this.eventListeners.has(event)) {
            this.eventListeners.set(event, []);
        }
        this.eventListeners.get(event).push(handler);
    }

    trigger(event) {
        const handlers = this.eventListeners.get(event) || [];
        handlers.forEach(h => h());
    }

    update(delta, elapsedTime) {
        if (this.ring1Mesh) {
            this.ring1Mesh.rotation.z += delta * 0.8;
        }
        if (this.ring2Mesh) {
            this.ring2Mesh.rotation.x += delta * 0.6;
        }
        if (this.mainMesh) {
            this.mainMesh.rotation.x = Math.sin(elapsedTime * 0.5) * 0.2;
        }
    }
}
