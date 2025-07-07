#!/bin/bash

COMMIT_MESSAGE=$1

if [ "$COMMIT_MESSAGE" = "" ]; then
    echo "No commit message provided."
    echo "Please provide a commit message to push to the AUR."

    exit 1
fi

# Functions
move_to_dir() {
    directory=$1

    if [ -d "$directory" ]; then
        cd "$directory"
        return 0
    fi

    echo "Failed moving into $directory."
    exit 1
}

git_push() {
    git add .
    git commit -m "$COMMIT_MESSAGE"
    git push
}

# Pushing the bin
move_to_dir "./bin"
git_push

# Pushing to build
move_to_dir "../build"
git_push

# Pushing to git
move_to_dir "../git"
git_push