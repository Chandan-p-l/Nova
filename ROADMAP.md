# NOVA Development Roadmap

This document outlines the milestones and future capabilities planned for the **NOVA** language and engine.

---

## Phase 1: MVP Compiler & Runtime (Current Milestone)
- [x] Language Grammar & EBNF Specification
- [x] Rust Workspace Compiler Infrastructure (`nova-lexer`, `nova-parser`, `nova-ast`, `nova-analyzer`, `nova-codegen`, `nova-cli`)
- [x] Declarative constructs: `scene`, `camera`, `light`, `model`, `material`, `animate`, `scroll`, `page`, `heading`, `button`, `state`
- [x] Browser 3D Runtime Engine (`nova-runtime.js` powered by WebGL/WebGPU adapter)
- [x] Development server with live reload (`nova dev`)
- [x] CLI commands (`nova create`, `nova dev`, `nova build`, `nova preview`, `nova format`, `nova lint`, `nova check`, `nova doctor`)
- [x] Interactive 3D Showcase Demo Project

---

## Phase 2: Advanced Rendering & Shader Systems (v0.2 - v0.4)
- [ ] WebGPU Compute Shaders & WGSL Integration
- [ ] GPU-accelerated particle system engine (`particles stars { ... }`)
- [ ] Node-based material graph compiler
- [ ] Post-processing pipeline (bloom, depth of field, screen-space reflections, ambient occlusion)
- [ ] Automatic texture compression (Basis Universal, KTX2, WebP, AVIF)
- [ ] Level of Detail (LOD) and frustum culling compiler optimizations

---

## Phase 3: Physics, Audio & XR Capabilities (v0.5 - v0.8)
- [ ] Rigid-body physics engine integration (`physics world { gravity: (0, -9.8, 0) }`)
- [ ] Spatial 3D Audio runtime bindings
- [ ] WebXR (VR/AR) immersive viewing support
- [ ] Multiplayer real-time state synchronization via WebSockets

---

## Phase 4: Full-Stack & SSR Infrastructure (v1.0)
- [ ] Server-Side Rendering (SSR) for SEO pre-rendering of 3D canvas snapshots
- [ ] Backend API router (`server api { get "/products" { ... } }`)
- [ ] Package ecosystem registry (`nova install @nova/physics`)
- [ ] VS Code Extension & Language Server Protocol (LSP) for syntax highlighting and auto-completion
