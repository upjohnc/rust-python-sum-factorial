# Description

Example code of leveraging rust function in python code

## Scenario

Have files with integer in the first line.
Want to calculate the factorial of each digit of the integer and then
sum those values.
Rust can do it faster.  Faster to read the files and process the text.

This creates the function to take a file path, read the file content,
take the first line, parse the integer into individual digits,
calculate the factorial of the digit, then sum the individual factorials.

## Local Dev

Need to install `rustfmt`: `brew install rustfmt`

- create vintual env and source it
- maturin develop then run python
