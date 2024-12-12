#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -euo pipefail

# Change the working directory to the directory of the script
cd "$(dirname "$0")"

# Optional: Print the current working directory for verification
echo "Current working directory is: $(pwd)"


# Concatenate all arguments into a single line
all_arguments="$*"

cargo +nightly -Zscript get-aoc-input-code.rs $all_arguments
