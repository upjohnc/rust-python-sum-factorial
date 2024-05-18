default:
    just --list

create-venv:
     python -m venv .venv

python-install:
    pip install -r requirements.txt

python:
    python python/src/main.py

maturin-develop:
    maturin develop

rust-test:
    cargo test
