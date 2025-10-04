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
├── Cargo.toml      # Workspace definition  
├── imgconv-core/   # Pure Rust library crate with the core algorithm.  
├── imgconv-cli/    # Command-line executable  
├── imgconv-ffi/    # C-compatible shared library  
└── imgconv-py/     # Python extension module  
```

## 🚀 Getting Started (terminal)

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

### 2. Build crates and integration targets
```bash
cargo build --release --workspace
```

### 3. Run tests
```bash
cargo test --package imgconv-core --lib -- tests --show-output
```

### 4. Run the CLI
```bash
cargo run -p imgconv-cli -- data/pug.jpg data/pug_converted_cli.png
```

### 5. Use shared library (from Python via ctypes)
```python
import ctypes

lib = ctypes.CDLL("target/release/libimgconv_ffi.so")  # or .dll / .dylib
res = lib.convert_image_c(b"data/pug.jpg", b"data/pug_converted_ffi.png")
if res == 0:
    print("Success!")
else:
    print("Error:", res)
```

### 6. Use Python extension module (PyO3 + Maturin)
Build and install locally:
```bash
cd imgconv-py
maturin develop
```
Then in Python:
```python
import imgconv_py
imgconv_py.convert_image_py("data/pug.jpg", "data/pug_converted_py.png")
```

## 🚀 Getting Started (VSCode)

### 1. Setup Python environment (required only for imgconv-py)
* From the command palette (<kbd>Ctrl+Shift+P</kbd>), select `Python: Create Environment...` 
* Select a `venv` environment, and name it `.venv`
* Open the integrated terminal (<kbd>Ctrl+`</kbd>)
* Activate the environment and install Maturin   using the commands above

### 2. Build crates and integration targets
* **Terminal** > **Run Build Task...** (<kbd>Ctrl+Shift+B</kbd>) 
* Select the crate to build from the menu that appears
* To install the PyO3 crate in the virtual environment, select one of the Maturin build options

### 3. Test crates and integration targets
* Select a target under **Run and Debug** (<kbd>Ctrl+Shift+D</kbd>)
* Run the target (<kbd>F5</kbd>)
* To test the crate directly, open [imgconv-core/src/lib.rs](./imgconv-core/src/lib.rs) and click on the **Run Tests** CodeLens above the `Tests` module

## 🛠️ Dependencies
* [image](https://crates.io/crates/image) – Rust image loading/encoding
* [clap](https://crates.io/crates/clap) - CLI argument parsing
* [pyo3](https://crates.io/crates/pyo3) - Python bindings
* [maturin](https://github.com/PyO3/maturin) - Build tool for PyO3

## 📚 Use Cases
This template can be reused for projects where:
* Performance-critical algorithms are implemented in Rust.
* Multiple integration paths are required: CLI, C API, Python bindings.
* You want a clean separation of core logic and integration code.

## 🧩 Contributing
* The imgconv-core crate should remain pure Rust with no integration dependencies.
* Wrappers (`imgconv-cli`, `imgconv-ffi`, `imgconv-py`) are kept minimal and only handle integration concerns.
* Pull requests adding new integration layers or extending the example are welcome.

## 👏 Acknowledgements
Sample image by [Bruce Galpin](https://unsplash.com/@star2dev?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash) on [Unsplash](https://unsplash.com/photos/fawn-pug-jumping-on-water-h7oZAHnS9_E?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash)

## 📜 License
This repository is released under the MIT license. See [LICENSE](https://github.com/d112358/rust-integration-template-imgconv/blob/main/LICENSE) for additional details.

