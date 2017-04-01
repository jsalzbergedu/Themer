// Stolen with love from orbutils' editor 
//#![deny(warnings)]

extern crate orbclient;
extern crate orbtk;
extern crate extra;
extern crate coreutils;

use orbtk::{Rect, Window};
use std::cmp;
use::std::error::Error;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{stderr, Write};
use extra::io::fail;
use extra::option::OptionalExt;
use std::env;
use coreutils::ArgParser;

fn setup_mode() {
    // Must be run w/ superuser. Replaces orbital.conf with one that points to /home/user/.themes. (In theory)
    let mut stderr = stderr();
    let default_conf_string = "background=/ui/background.png
background_mode=zoom
cursor=/home/user/.themes/left_ptr.png
bottom_right_corner=/ui/bottom_right_corner.png
bottom_side=/ui/bottom_side.png
right_side=/ui/right_side.png
window_max=/ui/window_max.png
window_max_unfocused=/ui/window_max_unfocused.png
window_close=/home/user/.themes/window_close.png
window_close_unfocused=/ui/window_close_unfocused.png";
    let mut orbital_conf_source_path = Path::new("/ui/orbital.conf");
    let mut default_conf = File::create("/ui/orbital.conf").try(&mut stderr);
    default_conf.write(default_conf_string.as_bytes()).try(&mut stderr);
    fs::create_dir(Path::new("/home/user/.themes/")).try(&mut stderr);
    File::create(Path::new("/home/user/.themes/left_ptr.png")).try(&mut stderr);
    File::create(Path::new("home/user/.themes/window_close.png")).try(&mut stderr);
    fs::copy(Path::new("/home/user/left_ptr.png"), Path::new("/home/user/.themes/left_ptr.png")).try(&mut stderr);
    fs::copy(Path::new("/home/user/window_close.png"), Path::new("/home/user/.themes/window_close.png")).try(&mut stderr);
}


fn normal_mode() {
    let title = { format!("Themer") };
    let (display_width, display_height) = orbclient::get_display_size().expect("viewer: failed to get display size");
    let (width, height) = (cmp::min(800, display_width * 4/5), cmp::min(576, display_height * 4/5));
    let mut window = Window::new(Rect::new(-1, -1, width, height), &title);
    window.exec();
}

fn main() {
    // Why doesn't this work????
    // Is File::create even implemented? I'm not sure that it is working on the "editor"
    let mut stderr = stderr();
    let mut parser = ArgParser::new(1).add_flag(&["setup","setup"]);
    parser.parse(env::args());
    if parser.found("setup") {
        setup_mode();
    } else {
        normal_mode();
    }

}

