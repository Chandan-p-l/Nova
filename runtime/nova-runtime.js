/**
 * NOVA Browser 3D Runtime Engine (v0.1.0)
 * Modern WebGL / WebGPU 3D rendering backend with reactive state bridge.
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
        this.renderer.toneMappingExposure = 1.2;

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
        this.stateListeners.set(key, []);
        return initialValue;
    }

    setState(key, newValue) {
        this.stateStore.set(key, newValue);
        const listeners = this.stateListeners.get(key) || [];
        listeners.forEach(cb => cb(newValue));
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

        this.threeScene.background = new THREE.Color('#080810');
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

        // Add soft secondary ambient light
        const amb = new THREE.AmbientLight(0xffffff, 0.6);
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
            metalness: 0.3,
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
        this.models.forEach(model => model.update(delta, elapsedTime));
    }
}

class NovaModel {
    constructor(name, source, props, novaScene) {
        this.name = name;
        this.source = source;
        this.novaScene = novaScene;
        this.eventListeners = new Map();

        // Create fallback procedural geometry immediately (3D Glowing Product Mesh)
        const group = new THREE.Group();

        const mainGeo = new THREE.IcosahedronGeometry(1.2, 2);
        const mainMat = new THREE.MeshStandardMaterial({
            color: 0x6366f1,
            metalness: 0.8,
            roughness: 0.15,
            emissive: 0x312e81,
            emissiveIntensity: 0.5,
        });
        const mainMesh = new THREE.Mesh(mainGeo, mainMat);
        group.add(mainMesh);

        // Outer Wireframe Ring
        const ringGeo = new THREE.TorusGeometry(1.8, 0.04, 16, 100);
        const ringMat = new THREE.MeshBasicMaterial({ color: 0x38bdf8, wireframe: true });
        const ringMesh = new THREE.Mesh(ringGeo, ringMat);
        ringMesh.rotation.x = Math.PI / 3;
        group.add(ringMesh);
        this.ringMesh = ringMesh;

        group.__novaModel = this;
        this.group = group;

        if (props.position) {
            group.position.set(props.position.x || 0, props.position.y || 0, props.position.z || 0);
        }

        this.novaScene.threeScene.add(group);
        this.novaScene.engine.interactiveObjects.push(group);
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
        if (this.ringMesh) {
            this.ringMesh.rotation.z += delta * 0.5;
        }
    }
}
