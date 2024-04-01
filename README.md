# Token Counter (ttc)

`ttc` is a command line tool written in Rust that reads text from multiple text files or standard input and counts the number of tokens in it.

## Installation

Firstly, you need to install Rust. See the [official documentation](https://www.rust-lang.org/tools/install) for details. 

Then you can build `ttc` using the following command:

    $ cargo build --release

Copy the output executable (`target/release/ttc`) to a directory that is on your PATH.

Alternatively, you can directly install it using:

    $ cargo install --git https://github.com/burke/ttc/


## Usage

To count tokens in multiple files, use:

```shell
$ ttc [FILE]...
```
    
To count tokens from the standard input:
```shell
$ ttc < [FILE]
or
$ [command] | ttc
```

Example:
```shell
$ git ls-files | xargs ttc
       2 .gitignore
    4902 Cargo.lock
      72 Cargo.toml
     201 LICENSE
     157 README.md
     501 src/main.rs
    5835 total
$ ttc < README.md
157
```

## Options

- `--encoding` / `-e`: Specify the encoding model to use for tokenization. The default is `cl100k_base`, you can choose from `p50k_base`, `p50k_edit`, `r50k_base`.

## License

This software is released under the MIT License. For more details, see `LICENSE`.