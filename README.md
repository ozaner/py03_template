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

## Notes
### Python version management
Had problems (on Windows) using `pyenv-win`. So used `scoop` to manage multiple python versions instead. Seems to works flawlessly with this project.

### Environment variables
Make sure that that the `poetry-dotenv-plugin` plugin is installed for `poetry`. You can install it like so:

```sh
poetry self add poetry-dotenv-plugin
```

When running a command with `poetry run`, the environment variables in the ".env" file will be automatically included.