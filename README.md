V2 of implementing my own programming language.

So far the ClI framework and the tokenizer are functional. Feel free to play with it.

## Setup
To copile and run this compiler a rustcompiler is needed. The recommended way of installing it is via 'rustup' together with 'cargo'. For more information see https://www.rust-lang.org/tools/install \\

For people without 'cargo' experience:\\
'cargo' can either `run` or `build` the project. Where `build` just creates a executable and `run` also runs the executable. To suply comand line arguments to the executable started with `run` one can use `--` after `cargo run`. Everything following `--` will be supplied to the executable. \\
e.g. Run the compiler on the supplied 'hello.nop' file:\\
`cargo run -- hello.nop`