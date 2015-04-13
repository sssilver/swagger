extern crate piston;
extern crate opengl_graphics;

use character;


pub struct Game {
    pub gl: opengl_graphics::GlGraphics,  // OpenGL drawing backend

    main_character: character::Character
}

impl Game {
    pub fn new(gl: opengl_graphics::GlGraphics) -> Game {
        Game {
            gl: gl,
            main_character: character::Character::new("Silver".to_string())
        }
    }

    pub fn render(&mut self, args: &piston::event::RenderArgs) {
    }

    pub fn update(&mut self, args: &piston::event::UpdateArgs) {
    }
}
