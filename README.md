# kissa-template

kissabot's cargo-generate-template

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
   ```
   cargo install cargo-generate
   ```
3. Set up a sample project with this template
   ```
   cargo generate --git https://github.com/KissaBot/kissa-template --name my-plugin
   cd my-plugin
   ```
