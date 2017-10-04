#
# ExampleForRust: Example package for GAP Kernel Module written in Rust
#
# This file runs package tests. It is also referenced in the package
# metadata in PackageInfo.g.
#
LoadPackage( "ExampleForRust" );

TestDirectory(DirectoriesPackageLibrary( "ExampleForRust", "tst" ),
  rec(exitGAP := true));

FORCE_QUIT_GAP(1); # if we ever get here, there was an error
