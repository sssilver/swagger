extern crate piston;
extern crate image;

use std;
//use image::GenericImage;


pub struct Character {
    name: String,
    //sprite: image::DynamicImage
}

impl Character {
    pub fn new(name: String) -> Character {
        /*let sprite = image::open(
            &std::path::Path::new("test.png")
        ).unwrap();*/

        Character {
            name: name
            //sprite: sprite
        }
    }

    pub fn render(&mut self) {
    }
}
