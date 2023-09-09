## Rust Hash Calculator

A recursive, performant hash calculator written in Rust!

## Setup

Clone the repository.

Run `cargo build`.

If you get this error:

```console
/usr/bin/ld: cannot find Scrt1.o: No such file or directory
/usr/bin/ld: cannot find crti.o: No such file or directory
```

Run on Ubuntu ([source](https://stackoverflow.com/questions/6329887/how-to-fix-linker-error-cannot-find-crt1-o)):

```sh
sudo apt install gcc-multilib
```

Use the app like so:

```sh
./target/debug/hash-calculator --algorithm ALGORITHM DIRECTORY
# ALGORITHM can be either `sha1` or `sha256`.
```

## Random Files for Testing

Use the script for creating random files: `scripts/generate_example_files.sh`

Check the script code for available options.
