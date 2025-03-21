@default:
    just --list

alias env := setup-env
alias install := setup-env
alias local := setup-env
alias dev := setup-env
alias py := python
alias doit := all

# Run a quick cargo check build
cargo-check:
    cargo check

# Run Rust tests
cargo-test:
    cargo test

# Make a debug build
debug-build:
    maturin develop

# Run all Rust recipes
rust: cargo-check cargo-test debug-build

# Set up the Python environment.
setup-env:
    uv venv
    uv pip install pip maturin
    source .venv/bin/activate
    maturin develop --release
    uv sync

# Run all Ruff Python lints
py-lints:
    uv sync
    source .venv/bin/activate
    ruff check . --exit-zero --fix --unsafe-fixes

# Format all Python scripts in the current working directory.
py-format:
    uv sync
    source .venv/bin/activate
    ruff format .

# Sort Python imports in all Python scripts in the project.
py-sort-imports:
    uv sync
    source .venv/bin/activate
    ruff check . -n --select=I --fix

# Test on the locally available version of the python interpreter
pytest:
    uv sync
    source .venv/bin/activate
    pytest -s

# Run tests across supported interpreter versions with tox and uv
py-tox:
    uv sync
    source .venv/bin/activate
    tox

# Run all Python recipes in sequence.
python: setup-env py-lints py-format py-sort-imports pytest

# Run all recipes in sequence with one another.
all: setup-env python
