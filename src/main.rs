use std::{env, fs, path::Path, cell::RefCell};
use fltk::{app, frame::Frame, prelude::*, window::Window, enums::{Event, Key}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: vwr <image>");
        std::process::exit(1);
    }
    let app = app::App::default();
    app::background(0, 0, 0);
    let image = fltk::image::SharedImage::load(&std::path::Path::new(&args[1])).unwrap();
    let path = Path::new(&args[1]).parent().unwrap().to_str().unwrap();
    
    // Iterate over the files to add them to our vector
    let mut images: Vec<String> = Vec::new();
    for path in fs::read_dir(path)? {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if fs::metadata(path).unwrap().is_file() {
            if path.to_lowercase().ends_with(".jpg") || path.to_lowercase().ends_with(".png") || path.to_lowercase().ends_with(".jpeg") {
                images.push(path.to_string());
            }
        }
    }

    let (mut window, mut frame) = setup_window(image.w(), image.h());
    frame.set_image(Some(image));
    let image_index = RefCell::new(0);
    window.clone().handle(move |_, ev| match ev {
        Event::KeyDown => {
            let key = app::event_key();
            match key {
                Key::Right => {
                    if *image_index.borrow() < images.len() - 1{ 
                        *image_index.borrow_mut() += 1;
                        let mut image = fltk::image::SharedImage::load(&std::path::Path::new(&images[*image_index.borrow()])).unwrap();
                        if image.w() > window.w() || image.h() > window.h() {
                            let (width, height) = (window.w(), window.h());
                            image.scale(width, height, true, false);
                        }
                        frame.set_image(Some(image));
                        window.redraw();
                    }
                },
                Key::Left => {
                    if *image_index.borrow() > 0 {
                        *image_index.borrow_mut() -= 1;
                        let mut image = fltk::image::SharedImage::load(&std::path::Path::new(&images[*image_index.borrow()])).unwrap();
                        if image.w() > window.w() || image.h() > window.h() {
                            let (width, height) = (window.w(), window.h());
                            image.scale(width, height, true, false);
                        }
                        frame.set_image(Some(image));
                        window.redraw();
                    }
                },
                _ => {}
            }
            true
        },
        _ => false,
    });
    app.run().unwrap();
    Ok(())
}

fn setup_window(image_w: i32, image_h: i32) -> (Window, Frame) {
    let mut window = Window::new(100, 100, image_w, image_h - 15, "vwr"); // -15 to avoid empty space at the bottom TODO: fix this better
    let frame = Frame::new(0, 0, image_w, image_h, ""); 
    window.end();
    window.show();
    return (window, frame)
}