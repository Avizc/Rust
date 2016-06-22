/// GETTING STARTED - 0113220616/0513220616Z
/// https://doc.rust-lang.org/book/getting-started.html
///
/// Installation was pretty simple.
/// $ curl -sSf https://static.rust-lang.org/rustup.sh | sh
///
/// Quick search while I wait: /// is a doc comment, // is a line comment
///
/// Rust is ready to roll, here we go.
///
/// Hello, World!
///
/// Just in case I forget where I put it:
/// mkdir ~/rustprojects
///
/// Name syntax: more than one word use an underscore.
/// File extension: .rs
///
/// main.rs
fn main() {
    println!("Hello, world!");
}
/// It worked! That was darling.
///
/// How Does It Work?
///
fn main(){ // This is the function, arguments if the function returned something goes inside the () brackets

} // The {} bracket syntax, same line and at the end
///
/// Holy shucks this requires four spaces. I do three usually, erm, time to adjust. 
/// Spaces versus Tabs just added a whole new gallon of fuel to the bus I'm on.
///
println!() // Rust macro, metaprogramming in Rust this way
println() // This is just a normal function
///
/// Expression-oriented language, Rust can emote better than I can with ;
///
/// Compiling and Running
///
/// Before running Rust programs, compile it.
///
/// rustc examplerabbit.rs // rustc command and source file name
///
/// Compiled sucessfully creates a binary executable:
/// main   main.rs
///
/// To run it:
/// ./main
///
/// Rust = Ahead-of-time compiled language
/// Differs from other languages (e.g. Ruby, Python, Javascript): requires language implementation installed to open,
/// just a single command to compile and run program. Rust anyone can open without installing :>
///
/// Hello, Cargo!
///
/// Cargo: Builds code, download libraries code depends on, build those libraries
///
/// Cargo-fy
/// 1.) "Put your source file in the right directory."
/// 2.) "Get rid of the old executable and make a new one."
/// 3.) Make a Cargo configuration file.
///
/// Cargo-fy main.rs
/// mkdir src // Expects source files in the src directory, top level project directory just for READMEs, license, misc.
/// mv main.rs src/main.rs // If you want a library instead of an executable it's lib.rs
/// rm main
///
/// New file: Cargo.toml // Need to capitalize C, else Cargo won't know what to do with the config file
/// TOML = Tom's Obvious, Minimal Language
///
/// Cargo.toml
[package] // Indicate following statements config a package

name = "hello_world" // Required
version = "0.0.1" // Required
authors = [ "Avi Z <omitted on Github>" ] // Required
///
/// Since Cargo.toml is now in the project root directory of hello_world good to build and run the program.
///
/// cargo build
/// Response was: Compiling hello_world v0.0.1 (file:///Users/avi/rustprojects/hello_world)
/// ./target/debug/hello_world
///
/// Turns out you can build and run it all in one line with cargo run instead of cargo build.
/// Cargo only rebuilds if you've modified the source code, leaves out the response otherwise.
///
/// If this is going for release: cargo build --release
/// This compiles the project with optimizations
/// Code runs faster, program takes longer to compile
///
/// If you use cargo build it makes a Cargo.lock
/// Cargo uses this to keep track of dependencies. Let Cargo handle it, leave it alone. :>
///
/// Note on how to start Rust projects:
/// $ git clone someurl.com/foo
/// $ cd foo
/// $ cargo build
///
/// Cargo can make a skeleton project directory: cargo new hello_world --bin
/// --bin // Makes an executable (aka binary) application over a library
/// 
/// That's it for now!
