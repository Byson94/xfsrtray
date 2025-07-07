#!/bin/bash

set -e  # Exit on error

COMMIT_MESSAGE=$1

if [ -z "$COMMIT_MESSAGE" ]; then
    echo "No commit message provided."
    echo "Please provide a commit message to push to the AUR."
    exit 1
fi

# Absolute path to script's directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Function to push a directory
git_push() {
    dir=$1
    echo "Pushing $dir"

    if [ ! -d "$dir" ]; then
        echo "Directory $dir not found."
        exit 1
    fi

    pushd "$dir" > /dev/null

    git add .
    git commit -m "$COMMIT_MESSAGE" || echo "Nothing to commit in $dir"
    git push

    popd > /dev/null
}

# Push each directory
git_push "$SCRIPT_DIR/bin"
git_push "$SCRIPT_DIR/build"
git_push "$SCRIPT_DIR/git"
