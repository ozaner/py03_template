# py03_template

## Usage
```sh
poetry run python ./src-python/py03_template/main.py
```

## Dependency Management
- Use `poetry` to deal with python dependencies (e.g. `poetry add foo`)
- Use `cargo` to deal with rust dependencies (e.g. `cargo add bar`)

## Build
```sh
#installs python deps
poetry install

#compiles+installs rust crate as python module
poetry run maturin develop
```

## Testing
```sh
#runs all tests in "python/py03_template/tests" dir
poetry run pytest

#runs all rust tests
poetry run cargo test
```

### Notes
Running `poetry run cargo test` on windows returns an error `STATUS_DLL_NOT_FOUND` if the `python3x.dll` file is nowhere in the path (where `x` is the python version being used by the project).

`pyenv-win` seems to include shims for the `python3x` file but not the `python3x.dll` file. As a result, `cargo test` can't run.

To fix this, either:
- Add the directory containing that dll to the `PATH` var (e.g. for a pyenv installation it should be in "~/.pyenv/pyenv-win/shims")
- Copy and paste that dll into "target/debug/deps".
