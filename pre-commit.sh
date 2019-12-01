#!/bin/bash
cargo fmt --all -- --check --quiet
if [ $? -eq 0 ]; then
    exit 0
else
    echo "Run 'cargo fmt --all' before committing"
    exit 1
fi

