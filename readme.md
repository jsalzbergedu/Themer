## Themer
Themer is intended to change the look of redox' graphical user interface, namely, orbital, orbital's wiget toolkit, and orbclient.

## Installation
Install redox using the instructions for manual setup, then clone this repository. Replace `redox/programs/orbutils/Cargo.toml`,
as well as `redox/mk/userspace/orbutils`, with the files in this repo. Make a directory `redox/orbutils/themer`, and put the `main.rs`
of this repo in that directory.

## Running
Run `make qemu` from your redox directory. Open the terminal in redox, and run `cd /ui/bin`, `sudo ./themer`.

## Roadmap
This program currently does nothing, but I hope to have it change the default /ui/orbital.conf as well as the default icons,
and, if themes are ever implemented as some part of redox' orbital, configure the themes.