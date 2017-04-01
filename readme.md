## Themer
Themer is intended to change the look of redox' graphical user interface, namely, orbital, orbital's wiget toolkit, and orbclient.

## Installation
Under redox/programs, make a directory themer. Copy the Cargo.toml to the directory, and make a directory /src. Copy the main.rs
to the directory. Go up to redox/Cargo.toml, and, in a line under `"programs/userutils",`, a line `"programs/themer",`.
Then, in the file redox/mk/userspace/mod.mk, add, in a line under `filesystem/bin/tar \` (making sure to add the backslash),
`filesystem/bin/themer \`. Copy the window\_close and left\_ptr files from redox/filesystem/ui to redox/filesystem/home/user/. Run `make` and `make qemu`.

## Running
Run `make qemu` from your redox directory. In redox, open the terminal and run `sudo themer --setup`. 

## Roadmap
This program currently does not do much, but I hope to have it change the default /ui/orbital.conf as well as the default icons,
and, if themes are ever implemented as some part of redox' orbital, configure the themes.
