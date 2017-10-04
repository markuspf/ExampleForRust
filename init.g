#
# ExampleForRust: Example package for GAP Kernel Module written in Rust
#
# Reading the declaration part of the package.
#
_PATH_SO:=Filename(DirectoriesPackageLibrary("ExampleForRust", "target/release"), "libExampleForRust.so");
if _PATH_SO <> fail then
    LoadDynamicModule(_PATH_SO);
fi;
Unbind(_PATH_SO);

ReadPackage( "ExampleForRust", "gap/ExampleForRust.gd");
