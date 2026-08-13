# NOVA Language Specification

NOVA is a domain-specific programming language for building 3D-first web applications.

---

## 1. Primary Language Constructs

### 1.1 `scene`
A container block defining 3D objects, lighting, cameras, materials, and environment settings.

```nova
scene ProductShowcase {
    camera main {
        position: (0, 2, 8)
        fov: 45
    }

    environment {
        background: "#080808"
        lighting: "studio"
    }

    model phone {
        source: "./assets/phone.glb"
        position: (0, 0, 0)
        rotation: (0, 45deg, 0)
        scale: 1.0

        animate {
            rotation.y += 0.5deg
        }

        interaction {
            hover {
                scale = 1.05
            }
            click {
                open "/product"
            }
        }
    }
}
```

### 1.2 `camera`
Defines 3D cameras (Perspective and Orthographic).

```nova
camera main {
    type: perspective
    position: (0, 1.5, 6)
    target: (0, 0, 0)
    fov: 45
    near: 0.1
    far: 1000
}
```

### 1.3 `model` & `mesh`
Import external 3D models (GLTF, GLB, OBJ, FBX) or declare procedural geometries (box, sphere, cylinder, plane).

```nova
model car {
    source: "./car.glb"
    position: (0, 0, 0)
}

mesh heroCube {
    geometry: box(2, 2, 2)
    material: glass
}
```

### 1.4 `material`
Physically-based materials (PBR) and physical transmission shaders.

```nova
material glass {
    type: physical
    color: "#ffffff"
    roughness: 0.1
    metalness: 0.0
    transmission: 1.0
    ior: 1.5
}
```

### 1.5 `light`
Direct lighting, ambient lighting, point lights, and spot lights.

```nova
light sun {
    type: directional
    color: "#fff5ea"
    intensity: 2.0
    position: (5, 10, 5)
    shadows: true
}

light ambient {
    type: ambient
    color: "#404050"
    intensity: 0.5
}
```

---

## 2. Animation & Timelines

### 2.1 Keyframe Animations & Interpolations
```nova
animate heroPulse {
    target: model.heroCube
    property: position.y
    from: 0
    to: 2.5
    duration: 1.5s
    easing: easeInOutQuad
    repeat: infinite
    yoyo: true
}
```

### 2.2 Scroll Animations
Driven by page scroll position:

```nova
scroll animation {
    0% {
        camera.position.z = 10
        model.phone.rotation.y = 0deg
    }
    100% {
        camera.position.z = 4
        model.phone.rotation.y = 360deg
    }
}
```

---

## 3. Web UI Components & Layout

```nova
component Navbar {
    container {
        logo "NOVA 3D"
        nav {
            link "Home" -> "/"
            link "Showcase" -> "/showcase"
            link "Docs" -> "/docs"
        }
    }
}

page Home {
    Navbar

    section hero {
        heading "Declarative 3D for the Web"
        text "Build high-performance 3D experiences with minimal code."
        button "Get Started" {
            click => navigate("/docs")
        }
    }

    scene ProductShowcase
}
```

---

## 4. Reactive State Management

Declarative state variables update 3D properties and DOM elements automatically:

```nova
state rotationSpeed = 1.0deg
state activeColor = "#3b82f6"
state isHovered = false

model heroObject {
    rotation.y += rotationSpeed
    material.color = activeColor
}

button "Speed Up" {
    click {
        rotationSpeed += 1.0deg
    }
}
```

---

## 5. Modern Data Types & Units

- **Numbers**: `42`, `3.14159`
- **Vectors**: `(x, y, z)` e.g., `(0, 1.5, -5)`
- **Angles**: `90deg`, `1.5rad`
- **Time/Duration**: `500ms`, `2s`
- **Colors**: `"#ff0000"`, `rgb(255, 0, 0)`, `hsl(210, 100%, 50%)`
- **Assets**: `asset("./model.glb")`

---

## 6. Escape Hatch: GPU Shaders (`gpu {}`)

For power users who need custom WebGPU WGSL or WebGL GLSL shaders:

```nova
shader hologram {
    vertex {
        "void main() { gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); }"
    }
    fragment {
        "void main() { gl_FragColor = vec4(0.0, 0.8, 1.0, 0.6); }"
    }
}
```
