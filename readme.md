# Rust Integration Template

This repository is a **template for Rust projects with multiple integration targets**. It demonstrates how to structure a Rust workspace with a reusable **core algorithm crate** and multiple wrappers for different integration needs:  
* A command-line executable for testing and scripting
* A C-compatible shared library (`.so`/`.dll`/`.dylib`)
* A Python extension module using [PyO3](https://pyo3.rs/) and [Maturin](https://github.com/PyO3/maturin)
  
For demonstration purposes, this template performs a trivial **image format conversion** between PNG and JPEG 
in either direction, based on the extensions of paths received as arguments. Users of this template can simply 
substitute `imgconv` everywhere for a suitable project-specific prefix.

## 📦 Workspace Layout
```yaml
rust-integration-template/  
├── Cargo.toml # Workspace definition  
├── imgconv-core/ # Pure Rust library crate with the core algorithm.  
├── imgconv-cli/ # Command-line executable  
├── imgconv-ffi/ # C-compatible shared library  
└── imgconv-py/ # Python extension module  
```

## 🚀 Getting Started

### 1. Setup Python environment (required only for imgconv-py)
#### Windows
```bash
python3 -m venv .venv
.venv\Scripts\activate.bat
pip install -U pip maturin
```
#### Linux
```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
```

### 2. Build everything
```bash
cargo build --release --workspace
```

### 3. Run the CLI
```bash
cargo run -p imgconv-cli -- input.jpg output.png
```

### 4. Use Python extension module (PyO3 + Maturin)
Build and install locally:
```bash
cd imgconv-py
maturin develop
```
Then in Python:
```python
import imgconv_py
imgconv_py.convert_image_py("input.jpg", "output.png")
```

### 5. Use shared library (from Python via ctypes)
```python
import ctypes

lib = ctypes.CDLL("target/release/libimgconv-ffi.so")  # or .dll / .dylib
res = lib.convert_image_c(b"input.jpg", b"output.png")
if res == 0:
    print("Success!")
else:
    print("Error:", res)
```

## 🛠️ Dependencies
* [image](https://crates.io/crates/image) – Rust image loading/encoding
* [clap](https://crates.io/crates/clap) - CLI argument parsing
* [pyo3](https://crates.io/crates/pyo3) - Python bindings
* [maturin](https://github.com/PyO3/maturin) - Build tool for PyO3

## 📚 Use Cases
This template can be reused for projects where:
* Performance-critical algorithms are implemented in Rust.
* You need multiple integration paths: CLI, C API, Python bindings.
* You want a clean separation of core logic and integration code.

## 🧩 Contributing
* The imgconv-core crate should remain pure Rust with no integration dependencies.
* Wrappers (`imgconv-cli`, `imgconv-ffi`, `imgconv-py`) are kept minimal and only handle integration concerns.
* Pull requests adding new integration layers or extending the example are welcome.

## 📜 License
MIT

