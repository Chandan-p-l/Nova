# NOVA Programming Language

**NOVA** is a modern, high-performance domain-specific programming language designed specifically for building 3D-first web applications, interactive landing pages, product configurators, virtual showrooms, digital twins, and web-based 3D experiences.

Instead of writing verbose JavaScript + Three.js / WebGL / WebGPU + HTML + CSS boilerplate, developers can write clean, declarative NOVA code that compiles directly to optimized WebGL/WebGPU web targets.

---

## Key Features

- **3D-First Declarative Syntax**: Native constructs for `scene`, `camera`, `light`, `model`, `mesh`, `material`, `environment`, `animation`, `timeline`, `scroll`, and `physics`.
- **Declarative Web UI**: Integrated UI system with `page`, `section`, `heading`, `text`, `button`, `card`, `container`, and responsive layout syntax.
- **Reactive State System**: Built-in signal-like state (`state angle = 0deg`) that automatically syncs 3D scene properties and DOM elements.
- **High-Performance Rust Compiler**: Modular lexer, parser, semantic analyzer, and transpiler written in Rust.
- **Modern CLI & Dev Server**: Instant live reloading, asset watching, fast incremental compilation, and rich diagnostic error overlays via `nova dev`.
- **Progressive Architecture**: Simple declarative abstractions for beginners with GPU escape hatches (`gpu {}`) for low-level WebGPU / WebGL shaders.

---

## Quick Start

### 1. Build Compiler

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release
```

### 2. Create a New NOVA Project

```bash
nova create my-3d-site
cd my-3d-site
```

### 3. Start Development Server

```bash
nova dev
```

Open `http://localhost:3000` in your browser to view your live 3D application.

---

## Example NOVA Program

```nova
scene ProductShowcase {
    camera main {
        position: (0, 1.5, 5)
        fov: 45
    }

    environment {
        background: "#0a0a0f"
        lighting: "studio"
    }

    model phone {
        source: "./assets/phone.glb"
        position: (0, 0, 0)

        animate {
            rotation.y += 0.5deg
        }

        interaction {
            hover {
                scale = 1.05
            }
            click {
                open "/details"
            }
        }
    }
}

page Home {
    section hero {
        heading "The Future in 3D"
        text "Experience products like never before."
        button "Explore Now" {
            click => navigate("/explore")
        }
    }

    scene ProductShowcase
}
```

---

## Project Structure

```text
nova/
├── Cargo.toml               # Cargo Workspace root
├── crates/
│   ├── nova-ast/            # AST definitions
│   ├── nova-lexer/          # Tokenizer & lexer
│   ├── nova-parser/         # Syntax parser & error reporter
│   ├── nova-analyzer/       # Semantic analyzer & type checker
│   ├── nova-codegen/        # HTML/CSS/JS + WebGL code generator
│   └── nova-cli/            # CLI binary & dev server
├── runtime/
│   └── nova-runtime.js      # Browser 3D rendering runtime library
├── docs/                    # Specification & architecture docs
└── examples/
    └── demo_site/           # Showcase 3D demo application
```

---

## CLI Reference

| Command | Description |
|---|---|
| `nova dev` | Start development server with hot reload |
| `nova build` | Compile project for production |
| `nova preview` | Serve production build locally |
| `nova create <name>` | Scaffold a new NOVA project |
| `nova format` | Format `.nova` source files |
| `nova lint` | Check code for warnings and anti-patterns |
| `nova check` | Perform semantic & type analysis |
| `nova doctor` | Verify system requirements & dependencies |

---

## Documentation

- [Language Specification](file:///home/chandan/Documents/code/LANGUAGE.md)
- [Architecture Guide](file:///home/chandan/Documents/code/ARCHITECTURE.md)
- [EBNF Grammar](file:///home/chandan/Documents/code/GRAMMAR.md)
- [Development Roadmap](file:///home/chandan/Documents/code/ROADMAP.md)
- [Contributing Guide](file:///home/chandan/Documents/code/CONTRIBUTING.md)

---

MIT License. Designed by the NOVA Language Architecture Team.

