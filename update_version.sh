#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <new_version>"
  exit 1
fi

new_version="$1"

# Update PKGBUILD
if [[ -f PKGBUILD ]]; then
  sed -i "s/^pkgver=.*/pkgver=$new_version/" PKGBUILD
  echo "Updated pkgver in PKGBUILD to $new_version"
else
  echo "Error: PKGBUILD not found."
  exit 1
fi

# Update Cargo.toml
cargo_toml="./Cargo.toml"
if [[ -f "$cargo_toml" ]]; then
  awk -v ver="$new_version" '
    BEGIN { in_package = 0 }
    /^\[package\]/ { in_package = 1; print; next }
    /^\[.*\]/ { in_package = 0; print; next }
    in_package && /^version *=/ {
      print "version = \"" ver "\""; next
    }
    { print }
  ' "$cargo_toml" > "$cargo_toml.tmp" && mv "$cargo_toml.tmp" "$cargo_toml"
  echo "Updated version in $cargo_toml to $new_version"
else
  echo "Error: $cargo_toml not found."
  exit 1
fi
