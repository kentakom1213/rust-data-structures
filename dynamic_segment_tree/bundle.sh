#!/bin/zsh

IFS=$'\n'

files=(
    "alg"
    "dynamic_segment_tree"
    "node"
    "print_util"
)

for file in $files; do
    echo "mod $file {"
    cat "src/$file.rs"
    echo "}\n"
done
