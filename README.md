# Temperature Converter in Rust

This is a simple temperature converter written in Rust. It converts temperatures between Celsius, Fahrenheit, and Kelvin.

## Inssallation

### Linux
download the binary from the releases page and open the terminal in the directory where the binary is located and run the following command:
```bash
./temp_conv
```
And if doesnt work, you may need to give the binary the permission to execute:
```bash
chmod +x temp_conv
```

### Windows
download the binary from the releases page and open the command prompt in the directory where the binary is located and run the following command:
```cmd
temp_conv.exe
```
**Note:** I used `cargo build --release --target x86_64-pc-windows-gnu` to build the binary for Windows and it may raise false flags in some antivirus software. If you don't trust the binary, you can build the project from source.

## Building from source

To build the project from source, you need to have Rust and Cargo installed on your machine. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).
Clone the repository and navigate to the project directory and run the following command:
```bash
cargo build --release
```
The binary will be located in the `target/release` directory.

## Note

- I'm still learning Rust, so this is a simple project to help me learn the basics of the language.
- I'm open to feedback and suggestions on how to improve the code.
- I will refactor the code and make more modules to improve the readability (hopefully lol).
