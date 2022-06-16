// Short comments
/*
    Long comments
*/

// What is Cargo.toml file?
// Cargo.toml is a configuration file for the Rust project.
// It contains information about the project, such as the name of the project,
// the version of the project, the authors of the project, and so on.
// Cargo.toml is a file that is used to specify the dependencies of the project.

fn main() {
    println!("Hello, world!");
}

// Rust is a statically typed language.
// Rust is a compiled language.

// Commons commands:
// rustc -> compile a .rs file to .o file
// rustc -o hello_rust hello_rust.rs -> compile a .rs file to a .o file and link it to an executable file
// rustc -o hello_rust hello_rust.rs && ./hello_rust -> compile a .rs file to a .o file and run it
// TODO: Investigate more about the commands and their usage

// cargo build, b  -> Compile the current package
// cargo check, c  -> Analyze the current package and report errors, but don't build object files
// cargo clean     -> Remove the target directory
// cargo doc, d    -> Build this package's and its dependencies' documentation
// cargo new       -> Create a new cargo package
// cargo init      -> Create a new cargo package in an existing directory
// cargo run, r    -> Run a binary or example of the local package
// cargo test, t   -> Run the tests
// cargo bench     -> Run the benchmarks
// cargo update    -> Update dependencies listed in Cargo.lock
// cargo search    -> Search registry for crates
// cargo publish   -> Package and upload this package to the registry
// cargo install   -> Install a Rust binary. Default location is $HOME/.cargo/bin
// cargo uninstall -> Uninstall a Rust binary
