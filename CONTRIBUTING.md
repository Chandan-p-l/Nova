# Contributing to NOVA

Thank you for contributing to **NOVA**! This guide explains how to set up the compiler development environment, run tests, and follow code style conventions.

---

## 1. Development Requirements

- **Rust**: 1.75 or newer (`rustc`, `cargo`)
- **Node.js**: v18+ and `npm`

---

## 2. Setting Up Workspace

```bash
git clone https://github.com/nova-lang/nova.git
cd nova
export PATH="$HOME/.cargo/bin:$PATH"
cargo build
```

---

## 3. Running Tests

### Compiler Test Suite
```bash
cargo test --workspace
```

### Runtime Engine Tests
```bash
node runtime/test-runtime.js
```

---

## 4. Code Formatting & Linting

### Rust Compiler Code
```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

---

## 5. Adding Compiler Features

1. **AST**: Declare new AST nodes in `crates/nova-ast/src/lib.rs`.
2. **Lexer**: Add new token types in `crates/nova-lexer/src/lib.rs`.
3. **Parser**: Add parsing logic in `crates/nova-parser/src/lib.rs`.
4. **Analyzer**: Add validation logic in `crates/nova-analyzer/src/lib.rs`.
5. **Codegen**: Update HTML/CSS/JS generator in `crates/nova-codegen/src/lib.rs`.
6. **Tests**: Add unit test coverage for every new feature!
