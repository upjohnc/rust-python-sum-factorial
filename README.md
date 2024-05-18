# Description

Code to test maturin+pyo3 to port portions of code from Python to Rust.

## Scenario

You receive files in a directory everyday.  The business needs to have
the first line parsed.  The individual digits of the integer in the first
line are parsed to calculate the factorial of each digit and then
the sum of those values is calculated.

The team decided to move the reading of the contents of the file and the calculation
into a Rust function.  The solution is to have a python script read a specified
directory, call the Rust function, and prints the file with the related value.

Maturin and pyo3 are used to translate the Rust function into a python function.

## Local Dev

To create the local environment, you can create the python virtual env and then
have the Rust ffi build into that virtual env.

- create vintual env and source it
    - `python -m venv .venv`
    - `source .venv/bin/activate`
    - `pip install -r requirements.txt`
- build the rust function
    - `maturin develop`
- run python code
    - `python python/src/main.py`

## Precommit

Precommit has a hook to format the rust code.  Cargo did not have the `fmt` command
in my local installation, so I needed to install `rustfmt` through homebrew.
