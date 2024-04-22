# bevy_python_ffi 🧩
[![test](https://github.com/mosure/bevy_python_ffi/workflows/test/badge.svg)](https://github.com/Mosure/bevy_python_ffi/actions?query=workflow%3Atest)
[![GitHub License](https://img.shields.io/github/license/mosure/bevy_python_ffi)](https://raw.githubusercontent.com/mosure/bevy_python_ffi/main/LICENSE)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/mosure/bevy_python_ffi)](https://github.com/mosure/bevy_python_ffi)
[![GitHub Releases](https://img.shields.io/github/v/release/mosure/bevy_python_ffi?include_prereleases&sort=semver)](https://github.com/mosure/bevy_python_ffi/releases)
[![GitHub Issues](https://img.shields.io/github/issues/mosure/bevy_python_ffi)](https://github.com/mosure/bevy_python_ffi/issues)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/mosure/bevy_python_ffi.svg)](http://isitmaintained.com/project/mosure/bevy_python_ffi)
[![crates.io](https://img.shields.io/crates/v/bevy_python_ffi.svg)](https://crates.io/crates/bevy_python_ffi)

run and interact with bevy apps from python


## run the example app (from python)

- `pip install examples/minimal`

```python
import bevy_python_minimal

bevy_python_minimal.main(False)  # new_thread=False
```

- `python ./examples/minimal/python/main.py`


## development

> depending on environment, you may need to open shell from a python virtual environment to build
- `python -m venv .venv`
- `source .venv/Scripts/activate`


## compatible bevy versions

| `bevy_python_ffi` | `bevy` |
| :--         | :--    |
| `0.1`       | `0.13` |
