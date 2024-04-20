# bevy_python 🧩
[![test](https://github.com/mosure/bevy_python/workflows/test/badge.svg)](https://github.com/Mosure/bevy_python/actions?query=workflow%3Atest)
[![GitHub License](https://img.shields.io/github/license/mosure/bevy_python)](https://raw.githubusercontent.com/mosure/bevy_python/main/LICENSE)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/mosure/bevy_python)](https://github.com/mosure/bevy_python)
[![GitHub Releases](https://img.shields.io/github/v/release/mosure/bevy_python?include_prereleases&sort=semver)](https://github.com/mosure/bevy_python/releases)
[![GitHub Issues](https://img.shields.io/github/issues/mosure/bevy_python)](https://github.com/mosure/bevy_python/issues)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/mosure/bevy_python.svg)](http://isitmaintained.com/project/mosure/bevy_python)
[![crates.io](https://img.shields.io/crates/v/bevy_python.svg)](https://crates.io/crates/bevy_python)

run and interact with bevy apps from python


## run the example app (from python)

- `pip install .`

```python
import bevy_python_minimal

bevy_python_minimal.main()
```


## development

- `pip install .`
- `python ./python/main.py`

---or---

- `pip install maturin`
- `python -m venv .venv`
- `source .venv/Scripts/activate`
- `maturin develop`
- `python ./python/main.py`


## compatible bevy versions

| `bevy_python` | `bevy` |
| :--         | :--    |
| `0.1`       | `0.13` |
