# Bitcoin Private Key Finder 🔍

This project uses **Rust** to search for a Bitcoin private key within a specified range and verify if it generates a specific Bitcoin address.

# Requirements 📝

- **Rust** [official toolchain](https://www.rust-lang.org/)
- Linux, macOS, or Windows operating system
- Basic terminal knowledge

# Installation 💽

### 1. Install Rust
Rust can be installed using the official `rustup` command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Verify the installation with:

```rustc --version```

### 2. Clone this repository

Download the project files to your local environment:

```bash
git clone <repository_url>
cd <project_folder_name>
```

### 3. Configure the Rust project

Ensure that dependencies are included. Open the Cargo.toml file and add:

```bash
[dependencies]
bitcoin = "0.30.0"
rand = "0.8"
```

### 4. Build the project

Build the project in release mode:

```cargo build --release```

# Execution 💻

### 1. Configure the program

In the main.rs file, modify the search range and the desired Bitcoin address:

```bash
let start: u128 = 0x80000000000000000;
let end: u128 = 0xfffffffffffffffff;
let target_address = "1MVDYgVaSN6iKKEsbzRUAYFrYJadLYZvvZ";
```

### 2. Start the search

Run the program in release mode for the best performance:

```cargo run --release```

### 3. Results

If the program finds a valid private key, it will save the results in a file called found_wallet.txt in the project's root directory.

#### Additional notes
- Speed: The speed of the program depends on the CPU power. For better performance, consider using a GPU or a parallel implementation.
- Limitations: This program is designed for educational purposes only. Use for illegal purposes is strictly prohibited.

# Troubleshooting 
### 1. Issues with the Bitcoin library?
Ensure that the bitcoin library is properly configured. Check the current version with:

```cargo update```

### 2. Rust is not working?
Make sure rustc and cargo are in your PATH:

```echo $PATH```

### 3. Runtime error?
Check that the search range and Bitcoin address are correctly formatted.

# Contributing 🫂

We are always open to improvements and new features. Feel free to fork the project, implement changes, and submit a pull request.

# License 

This project is released under the MIT license.
```
