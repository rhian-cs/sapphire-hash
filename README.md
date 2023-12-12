## Rust Hash Calculator

A recursive, performant hash calculator written in Rust!

## Usage

### Basic Usage

Compile the app, and use:

```sh
recursive-hash-calculator-cli --algorithm ALGORITHM DIRECTORY
```

Where:

- `DIRECTORY` can be either the relative or the full path to the desired directory or file
- `ALGORITHM` is the desired hash algorithm. You may choose one of the following:
  - `md5`
  - `sha1`
  - `sha224`
  - `sha256`
  - `sha384`
  - `sha512`
  - `sha3_224`
  - `sha3_256`
  - `sha3_384`
  - `sha3_512`
  - `shake128`
  - `shake256`
  - `ripemd160`
  - `sm3`

### Logging

You may also enable logging by using the `RUST_LOG=trace` environment variable.

If you want to pipe the results and log results to separate files, you may use:

```sh
RUST_LOG=trace recursive-hash-calculator-cli --algorithm ALGORITHM DIRECTORY | tee result.txt 2>/dev/stdout | tee log.txt
```

## Development

### Dependencies

- Rust (https://rustup.rs/)
- OpenSSL (For Ubuntu install the `libssl-dev` package)
- [gui] GTK 3 (For Ubuntu install the `libgtk-3-dev` package)

### Setup

Clone the repository.

Run `cargo build`.

Use the app like so:

```sh
./target/debug/recursive-hash-calculator-cli --algorithm ALGORITHM DIRECTORY
```

### Building for Production

Run:

```sh
OPENSSL_STATIC=true cargo build --release
```

Use the app like so:

```sh
./target/release/recursive-hash-calculator-cli --algorithm ALGORITHM DIRECTORY
```

### Running Automated Tests

Run:

```sh
cargo test
```

### Creating Files for Manual Testing

Use the script for creating random files: `scripts/generate_example_files.sh`

Check the script code for available options.

### Linting

Run:

```sh
RUSTFLAGS=-Dwarnings cargo clippy --all-targets --all-features
```
