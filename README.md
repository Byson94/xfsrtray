# xfsrtray

![Rust CI](https://github.com/Byson94/xfsrtray/actions/workflows/rust.yml/badge.svg)
![Build CI](https://github.com/Byson94/xfsrtray/actions/workflows/build.yml/badge.svg)
![Aur CI](https://github.com/Byson94/xfsrtray/actions/workflows/aur-setup.yml/badge.svg)
[![AUR](https://img.shields.io/aur/version/xfsrtray?color=1793d1&logo=arch-linux&logoColor=white)](https://aur.archlinux.org/packages/xfsrtray)

A floating and customizable system tray for linux.

## Media

https://github.com/user-attachments/assets/056f0b82-a672-4585-929b-05dd1b15f79a

## Installing

To install **xfsrtray** on arch or any arch based system, you can use an aur wrapper like yay:

```bash
# Install the package (compile & install)
yay -S xfsrtray

# Install the bin (fastest)
yay -S xfsrtray-bin

# Install the git (latest dev version)
yay -S xfsrtray-git
```

If you are on any other system other than arch, you can build the binary by following the steps mentioned under the [building using cargo section](#building-using-cargo).

## Building Methods

You can build the package by using any of the following methods:

1. Using cargo to build the binary
2. Using `makepkg` to build the arch package for arch users

## Building using cargo

To build the package using cargo (rust), run the following command in the root of this project.

```bash
# Make sure that you have cargo installed.
cargo build --release --locked

# This will compile and create the binary at `/target/release/xfsrtray`
```

## Building using `makepkg` (arch only)

You can run the following command at the root of this project to create the local arch package.

```bash
makepkg # Creates the .pkg.tar.zst file
```

or you can running this command to build and install the package:

```bash
makepkg -si
```

## Documentation

You can read the documentation of xfsrtray on our [Github Wiki](https://github.com/Byson94/xfsrtray/wiki).


## Compatibility

#### Window Systems
- X11: Is made for and works; tested on Arch (i3wm, xfce4)
- Wayland: Works (with XWayland); tested on Arch (sway)

#### Operating Systems
- Arch Linux: Actively tested and developed on
- Other distributions (e.g., Fedora, Debian): *Not tested*. May require manual dependency resolution.
