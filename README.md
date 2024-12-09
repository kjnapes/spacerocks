![Alt text](assets/logo.png)

![spacerocks](https://github.com/kjnapes/spacerocks/workflows/spacerocks/badge.svg?branch=master)
[![Python 3.6+](https://img.shields.io/badge/python-3.6+-blue.svg)](https://www.python.org/downloads/release/python-360/)
[![PyPI version shields.io](https://img.shields.io/pypi/v/spacerocks.svg)](https://pypi.python.org/pypi/spacerocks/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Documentation Status](https://readthedocs.org/projects/spacerocks/badge/?version=latest)](https://spacerocks.readthedocs.io/en/latest/?badge=latest)
[![codecov](https://codecov.io/gh/kjnapier/spacerocks/branch/master/graph/badge.svg?token=1WO1H5WNYV)](https://codecov.io/gh/kjnapier/spacerocks)

`spacerocks` is a software package that puts the solar system at your fingertips. 

The package is written in `Rust` and is exposed to `Python` using [`PyO3`](https://github.com/PyO3/pyo3).


## Python Installation

<!-- To install `spacerocks` from `PyPI`, run the following command:
```bash
pip install spacerocks
``` -->

To install from source, you will need to have `Rust` and `maturin` installed.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin

git clone https://github.com/kjnapier/spacerocks
cd spacerocks
git checkout v3
cd py-spacerocks
maturin develop --release
```

You will also need to have [`cspice`](https://naif.jpl.nasa.gov/naif/toolkit_C.html) installed on your system, and add its location to your path as
```bash
export CSPICE_DIR="/path/to/cspice"
```
You might also need to rename `cspice/lib/cspice.a` to `cspice/lib/libcspice.a` if you are on a Unix system.

We actually use very few of the `cspice` functions, so it should be possible to remove this dependency in the future. 
If you would like to help with this, please send a message!

Testing! Tom ruch 12/9
Need to rename c.toml to cargo.toml when we want to reintroduce py-spacerocks

