extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate sprite;

mod game;
mod character;

use piston::window::{ WindowSettings, Size };
use piston::event::{ RenderArgs, UpdateArgs, Events };
use sdl2_window::Sdl2Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {
    let opengl = OpenGL::_3_2;
    let (width, height) = (640, 480);

    // Create an SDL window.
    let window = sdl2_window::Sdl2Window::new(
        opengl,
        piston::window::WindowSettings::new(
            "Swagger".to_string(),  // Window title
            piston::window::Size {width: width, height: height}  // Dimensions
        ).exit_on_esc(true).fullscreen(true)
    );
    let window = std::rc::Rc::new(std::cell::RefCell::new(window));

    // Create the sprite and load it
    // TODO: This should probably move into Game for now and Character later
    let id;
    let mut scene = sprite::Scene::new();
    let tex = std::path::Path::new("test.png");
    let tex = std::rc::Rc::new(opengl_graphics::Texture::from_path(&tex).unwrap());
    let mut sprite = sprite::Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    id = scene.add_child(sprite);

    // Create a new game and run it.
    let mut game = game::Game::new(GlGraphics::new(opengl));

    // Process the events
    for e in window.events() {
        use piston::event::*;

        if let Some(r) = e.render_args() {
            game.gl.draw(r.viewport(), |c, gl| {
                graphics::clear([0.0, 1.0, 1.0, 1.0], gl);
                scene.draw(c.transform, gl);
            });

            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
