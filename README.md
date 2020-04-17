[![Build Status](https://travis-ci.org/arakhmat/pyrpds.svg?branch=master)](https://travis-ci.org/arakhmat/pyrpds)

# Python Wrapper for Rust Persistent Data Structures

[rpds](https://github.com/orium/rpds) is a Rust package that provides [fully persistent data structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
with structural sharing. `pyrdps` is a python wrapper around it.


# Installation Instructions

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
maturin build
```


### Testing pyrpds
```bash
conda activate pyrpds
pytest
```
