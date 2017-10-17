# plug2pack

Migrate from [vim-plug](https://github.com/junegunn/vim-plug) to [pack](https://github.com/maralla/pack), an async vim8 package manager written rust.

# Install
```
git clone https://github.com/bennyyip/plug2pack
cd plug2pack
cargo build --release
./target/release/plug2pack --help
```

# Usage

```
plug2pack 0.1.0
BennyYip <yebenmy@protonmail.com>
convert vim-plug config to pack

USAGE:
    plug2pack [OPTIONS] --input <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --category <category>    specify a category for all package
    -i, --input <input>          Input file
        --local <local>          make all package local
        --optional <optional>    make all package optional
    -o, --output <output>        Output file, stdout if not present
```

# License
MIT
