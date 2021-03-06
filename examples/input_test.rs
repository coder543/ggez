extern crate ggez;

use ggez::*;
use ggez::event::*;
use ggez::graphics::{DrawMode, Point2};

struct MainState {
    pos_x: i32,
    mouse_down: bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: 100,
            mouse_down: false,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x as f32, 380.0),
                         100.0,
                         1.0)?;
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self,
                               _ctx: &mut Context,
                               button: MouseButton,
                               x: i32,
                               y: i32) {
        self.mouse_down = true;
        self.pos_x = x;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.mouse_down = false;
        self.pos_x = x;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self,
                          _ctx: &mut Context,
                          _state: MouseState,
                          x: i32,
                          y: i32,
                          xrel: i32,
                          yrel: i32) {
        if self.mouse_down {
            self.pos_x = x;
        }
        println!("Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
                 x,
                 y,
                 xrel,
                 yrel);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }


    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("Key pressed: {:?}, modifier {:?}, repeat: {}",
                 keycode,
                 keymod,
                 repeat);
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("Key released: {:?}, modifier {:?}, repeat: {}",
                 keycode,
                 keymod,
                 repeat);
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        println!("Controller button pressed: {:?} Controller_Id: {}",
                 btn,
                 instance_id);
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        println!("Controller button released: {:?} Controller_Id: {}",
                 btn,
                 instance_id);
    }

    fn controller_axis_event(&mut self,
                             _ctx: &mut Context,
                             axis: Axis,
                             value: i16,
                             instance_id: i32) {
        println!("Axis Event: {:?} Value: {} Controller_Id: {}",
                 axis,
                 value,
                 instance_id);
    }


    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("event_test", "ggez", c).unwrap();
    let state = &mut MainState::new();
    event::run(ctx, state).unwrap();
}
