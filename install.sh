#!/usr/bin/env bash

echo "===[ Running tests ]==="
cargo nextest run

echo "===[ Building ]==="
cargo build --release

echo "===[ Installing ]==="
cp -v target/release/spomo "$HOME"/bin