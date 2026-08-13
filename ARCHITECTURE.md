# NOVA Compiler & Runtime Architecture

This document describes the compilation pipeline, language infrastructure, runtime engine, and development server for **NOVA**.

---

## 1. Compiler Pipeline Architecture

```text
Source Code (.nova)
        │
        ▼
  [ Lexer / Tokenizer ]   ──> Tokens + Spans (Line, Col)
        │
        ▼
  [ Syntax Parser ]       ──> Abstract Syntax Tree (AST)
        │
        ▼
 [ Semantic Analyzer ]    ──> Type Checking, Scope Validation, Symbol Table
        │
        ▼
 [ Code Generator ]       ──> Output (HTML + CSS + ES Modules JS)
        │
        ▼
 [ Browser Runtime ]      ──> WebGL/WebGPU Rendering Engine
```

---

## 2. Compiler Crates (Rust Workspace)

- **`nova-ast`**: Data structures defining all AST nodes (e.g. `SceneDecl`, `ModelDecl`, `CameraDecl`, `PageDecl`, `StateDecl`, `Expr`, `Stmt`).
- **`nova-lexer`**: High performance tokenizer converting source text into token streams while preserving line/column spans for error diagnostics.
- **`nova-parser`**: Recursive descent / Pratt parser with structured error recovery and human-friendly diagnostics (Error N-series).
- **`nova-analyzer`**: Symbol resolution table, reactive state dependency graph, type checking, and asset validation.
- **`nova-codegen`**: Generates production-ready JavaScript ES modules, HTML containers, modern CSS styles, and WebGL/WebGPU scene initializations.
- **`nova-cli`**: CLI executable (`nova`) powering `nova dev`, `nova build`, `nova preview`, `nova create`, `nova format`, `nova lint`, `nova check`, `nova doctor`.

---

## 3. Web 3D Runtime Architecture (`nova-runtime.js`)

The browser runtime module handles rendering, asset loading, reactivity, and physics:

1. **Render Manager**:
   - WebGL renderer fallback + WebGPU rendering support.
   - Scene graph lifecycle management.
   - Dynamic window resize & aspect ratio handling.

2. **Asset Loader Engine**:
   - Async loading & caching for GLTF, GLB, OBJ models, HDR environment maps, and textures.
   - Automatic geometry centering and bounding box calculation.

3. **Reactive State Bridge**:
   - Signal state subscriber store connecting state mutations to 3D object attributes and DOM elements in real-time.

4. **Event & Raycasting Engine**:
   - Mouse & touch raycasting into 3D scene graph for `click` and `hover` detection on models and meshes.
   - Native DOM event listeners (`click`, `scroll`, `hover`).

5. **Scroll & Time Animation Runtime**:
   - Smooth requestAnimationFrame render loop.
   - Scroll position percentage tracking for scroll keyframe animations.

---

## 4. Development Server Architecture (`nova dev`)

- Built-in lightweight HTTP web server.
- WebSocket server delivering hot-reload updates to browser client.
- File system watcher watching `src/` directory for `.nova` and asset changes.
- Incremental compilation pipeline rebuilding changed files in milliseconds.
