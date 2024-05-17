default:
    just --list

create-venv:
     python -m venv .venv

source-venv:
    source .venv/bin/activate


python-install:
    pip install -r requirements.txt

python:
    python python/src/basic.py

maturin-develop:
    maturin develop

rust-test:
    cargo test
