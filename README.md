[![Build Status](https://travis-ci.org/arakhmat/pyrpds.svg?branch=master)](https://travis-ci.org/arakhmat/pyrpds)
[![Crates.io](https://img.shields.io/crates/v/pyrpds.svg?style=flat-square)](https://crates.io/crates/pyrpds)
[![PyPI](https://img.shields.io/pypi/v/pyrpds.svg?style=flat-square)](https://pypi.org/project/pyrpds/)

# Python Wrapper for Rust Persistent Data Structures

`pyrpds` is a library which provides CPython bindings to Rust's [rpds](https://github.com/orium/rpds) library.

The project has 2 goals:
- Allow packages that are built on top of [rpds](https://github.com/orium/rpds) to expose Python bindings easily.
- Provide faster drop-in replacements for [pyrsistent](https://github.com/tobgu/pyrsistent) data structures.

Python API will be exactly the same as [pyrsistent API](https://pyrsistent.readthedocs.io/en/latest/api.html).


# Installation Instructions

## For Users

### Python
```bash
pip install pyrpds
```

### Rust
Add following lines to Cargo.toml:
```toml
[dependencies.pyrpds]
version = "<version>"
```

## For Contributors

### Installing Dependencies

Install [Rust](https://www.rust-lang.org/tools/install) and [Conda](https://docs.conda.io/projects/conda/en/latest/user-guide/install/download.html).

### Installing nightly Rust
```bash
rustup install nightly
rustup override set nightly
```

### Installing Conda environment
```bash
conda env create -f environment.yaml
```

### Installing pyrpds
```bash
conda activate pyrpds
maturin develop --release -m lib/pyrpds/Cargo.toml
```


### Testing pyrpds
```bash
conda activate pyrpds
pytest
```
