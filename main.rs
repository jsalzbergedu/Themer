// Stolen with love from orbutils' editor
//#![deny(warnings)]

extern crate orbclient;
extern crate orbtk;
extern crate extra;

use orbtk::{Action, Button, Menu, Point, Rect, Separator, TextBox, Window};
use orbtk::traits::{Click, Place, Text};

use std::{cmp, env};



use::std::error::Error;
use::std::io::prelude::*;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::{stdout, stderr, Write, Read};
use std::process::exit;

use extra::io::fail;
use extra::option::OptionalExt;


/*
fn main(){
    let path_option = env::args().nth(1);

    let title = if let Some(ref path) = path_option {
        format!("{} - Editor", path)
    } else {
        format!("Editor")
    };

    let (display_width, display_height) = orbclient::get_display_size().expect("viewer: failed to get display size");
    let (width, height) = (cmp::min(800, display_width * 4/5), cmp::min(576, display_height * 4/5));

    let mut window = Window::new(Rect::new(-1, -1, width, height), &title);

    let text_box = TextBox::new();
    text_box.position(0, 16)
        .size(width, height - 16);
    window.add(&text_box);

    if let Some(ref path) = path_option {
        match File::open(path) {
            Ok(mut file) => {
                let mut text = String::new();
                match file.read_to_string(&mut text) {
                    Ok(_) => text_box.text.set(text),
                    Err(err) => println!("Failed to read {}: {}", path, err)
                }
            },
            Err(err) => println!("Failed to open {}: {}", path, err)
        }
    }

    let menu = Menu::new("File");
    menu.position(0, 0).size(32, 16);

    let open_action = Action::new("Open");
    open_action.on_click(|_action: &Action, _point: Point| {
        println!("Open");
    });
    menu.add(&open_action);

    menu.add(&Separator::new());

    let save_action = Action::new("Save");
    let save_path_option = path_option.clone();
    let save_text_box = text_box.clone();
    save_action.on_click(move |_action: &Action, _point: Point| {
        println!("Save");
        if let Some(ref path) = save_path_option {
            println!("Create {}", path);
            match File::create(path) {
                Ok(mut file) => {
                    let text = save_text_box.text.borrow();
                    match file.write(&text.as_bytes()) {
                        Ok(_) => match file.set_len(text.len() as u64) {
                            Ok(_) => println!("Successfully saved {}", path),
                            Err(err) => println!("Failed to truncate {}: {}", path, err)
                        },
                        Err(err) => println!("Failed to write {}: {}", path, err)
                    }
                },
                Err(err) => println!("Failed to open {}: {}", path, err)
            }
        } else {
            println!("Need to create file!");
        }
    });
    menu.add(&save_action);

    let save_as_action = Action::new("Save As");
    let save_as_path_option = path_option.clone();
    save_as_action.on_click(move |_action: &Action, _point: Point| {
        println!("Save As");
        let mut window = Window::new(Rect::new(-1, -1, 320, 32), "Save As");

        let text_box = TextBox::new();
        text_box.position(0, 0)
            .size(320, 16);
        window.add(&text_box);

        if let Some(ref path) = save_as_path_option {
            text_box.text.set(path.clone());
        }

        {
            let window_cancel = &mut window as *mut Window;
            let button = Button::new();
            button.position(0, 16)
                .size(320/2, 16)
                .text("Cancel")
                .on_click(move |_button: &Button, _point: Point| {
                    unsafe { (&mut *window_cancel).close(); }
                });
            window.add(&button);
        }

        {
            let window_save_as = &mut window as *mut Window;
            let button = Button::new();
            button.position(320/2, 16)
                .size(320/2, 16)
                .text("Save As")
                .on_click(move |_button: &Button, _point: Point| {
                    println!("Save {}", text_box.text.get());
                    unsafe { (&mut *window_save_as).close(); }
                });
            window.add(&button);
        }

        window.exec();
    });
    menu.add(&save_as_action);

    menu.add(&Separator::new());

    let close_action = Action::new("Close");
    let window_close = &mut window as *mut Window;
    close_action.on_click(move |_action: &Action, _point: Point| {
        println!("Close");
        unsafe { (&mut *window_close).close(); }
    });
    menu.add(&close_action);

    window.add(&menu);

    window.exec();
}
*/

struct OrbitalConfFile {
    // TODO: Figure out how to use less Strings
    
    background: String,
    background_mode: String,
    cursor: String,
    bottom_right_corner: String,
    bottom_side: String,
    right_side: String,
    window_max: String,
    window_max_unfocused: String,
    window_close: String,
    window_close_unfocused: String,
    // orbclient_theme: String, // One can hope
}

fn main() {
    // Why doesn't this work????
    // Is File::create even implemented? I'm not sure that it is working on the "editor"
    let default_conf_string = "background=/ui/background.png\nbackground_mode=zoom\ncursor=/ui/left_ptr.png\nbottom_right_corner=/ui/bottom_right_corner.png\nbottom_side=/ui/bottom_side.png\nright_side=/ui/right_side.png\nwindow_max=/ui/window_max.png\nwindow_max_unfocused=/ui/window_max_unfocused.png\nwindow_close=/ui/window_close.png\nwindow_close_unfocused=/ui/window_close_unfocused.png";
    let title = { format!("Themer") };
    let (display_width, display_height) = orbclient::get_display_size().expect("viewer: failed to get display size");
    let (width, height) = (cmp::min(800, display_width * 4/5), cmp::min(576, display_height * 4/5));
    let mut window = Window::new(Rect::new(-1, -1, width, height), &title);
    let mut orbital_conf_source_path = Path::new("/ui/orbital.conf");
    let mut stderr = stderr();
/*    if !orbital_conf_source_path.exists() {
        let mut default_conf = match File::create(orbital_conf_source_path) {
            Err(why) => panic!("Error in creating default orbital.conf, why: {}", why.description()),
            Ok(default_conf) => default_conf,
        };
        match default_conf.write_all(default_conf_string.as_bytes()) {
            Err(why) => {panic!("Error in writing to default orbital.conf, why: {}", why.description())},
            Ok(_) => println!("Wrote default options to /ui/orbital.conf"),
        }
    }
     */
    if !orbital_conf_source_path.exists() {
        let mut default_conf = File::create("/ui/orbital.conf").try(&mut stderr);
        default_conf.write(default_conf_string.as_bytes()).try(&mut stderr);
    }
    window.exec();
}

