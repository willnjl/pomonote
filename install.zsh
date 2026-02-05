#!/bin/zsh

# Build the Rust application
cargo build --release

# Copy the binary to ~/bin (create if doesn't exist)
mkdir -p ~/bin
cp target/release/pomonote ~/bin/

echo "Installed pomonote to ~/bin. Make sure ~/bin is in your PATH."
