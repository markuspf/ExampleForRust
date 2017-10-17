# GAP 4 package `ExampleForRust'

This is an experimental package that provides infrastructure
to write [GAP](https://github.com/gap-system/gap) kernel modules
in [Rust](https://rust-lang.org/).

To build the package `cargo build --release` should suffice. Currently
only the released library is loaded by the startup-code, but this
should be easily adaptable to debugging versions.

This package bears some relationship with an effort to make GAP usable
as a dynamic library.
