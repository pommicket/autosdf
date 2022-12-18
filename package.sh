#!/bin/sh
cargo build --release || exit 1
tar -czf autosdf-src.tar.gz --transform "s,^,autosdf/," $(git ls-files)
rm -rf autosdf
mkdir autosdf
cp target/release/autosdf autosdf/
cp README.md autosdf/
cp example*.png autosdf/
cp settings.txt autosdf/
tar -czf autosdf-linux.tar.gz autosdf
