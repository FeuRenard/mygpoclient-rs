#!/bin/bash

cargo check

if [ ! $? -eq 0 ]; then
    echo "Fix rustc errors before committing"
    exit 1
fi

cargo clippy

if [ ! $? -eq 0 ]; then
    echo "Fix clippy errors before committing"
    exit 1
fi

cargo fmt --all -- --check --quiet

if [ ! $? -eq 0 ]; then
    echo "Run 'cargo fmt --all' before committing"
    exit 1
fi

exit 0

