# Haochong-week8-mini-repo 
[![PYTHONCI](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/pytest.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/pytest.yml)[![Clippy](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/lint.yml)[![Tests](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/tests.yml)
This is a repo template for course 706_Data_Engineering Week 8 Mini Project. First of all, I write my code in python to read a csv and get the median value of to numeric columns. Then, create a `Cargo.toml`and transform my code into rust and create `lib.rs` and `main.rs` . After that, I create `test_main.rs` to test the functions. Finally, I add content for both rust and python in `Makefile`, and use Action to run `Makefile` and got a 100% pass. 

Important file:
* `Makefile`
* `lib.py`
* `main.py`
* `test_main.py`
* `lib.rs`
* `main.rs`
* `test_main.rs`
* `25ktopomapseriesindex.csv`
* `Cargo.toml`

## Purpose
- Take an existing Python script for data processing
- Rewrite it in Rust
- Highlight improvements in speed and resource usage


## Rust Implementation:
The Rust version reads data from my CSV file in main function by using external crates `csv`, then calculates the medians of two numeric columns  of `shape_leng` and `shape_area` using my function `calculate_median`, which use a reference to a vector of f64 (64-bit floating-point numbers) as its argument. Also, as required, I use `Instant` to collect start time and end time and then calculate the usage of time.


### Preparation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build`
4. run: `cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt` or use your own string

### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

## Python Implementation:
The Python version features a similar CLI, utilizing the argparse module for argument parsing. It also offers options for mode, input text, and optional key and IV in base64 format. The CLI measures execution time, logs it, and displays the encrypted or decrypted message along with the elapsed time.


### Preparation: 
1. git clone the repo
2. install: `make python_install`
3. run: `python main.py encrypt "Hello World"` or use your own string   

### Check Format and Test Erros: 
1. Format code `make python_format`
2. Lint code `make python_lint`
3. Test coce `make python_test`

## Speed and Resource Usage:
[Link to Rust runtime Markdown File](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/blob/main/rust_times.md)
[Link to Python runtime Markdown File](https://github.com/nogibjj/Jeremy_Tan_IDS706_Week8/blob/main/python_times.md)

You can view how long it takes to encrypt and decrypt the same messages above. Based on the speed, it's obvious Rust run on average 400 times faster than Python and we can infer why the resource usage is vastly smaller than Python. Rust outperforms Python in speed primarily due to its static typing, zero-cost abstractions, and absence of a Global Interpreter Lock (GIL). Rust's strict typing allows for more efficient compilation, while its ownership system enables high-performance abstractions without sacrificing safety. Additionally, Rust manages memory directly, avoiding the overhead of Python's garbage collector. The language also offers fine-grained control over memory, enabling low-level optimizations. These factors, combined with an optimized compiler and a performance-centric standard library, contribute to Rust's reputation for speed.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/DaGenix/rust-crypto/
* https://github.com/nogibjj/rust-data-engineering
