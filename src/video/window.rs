extern crate sdl2;

use sdl2::video::WindowBuilder;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

pub struct Size {
    pub height: u32,
    pub width: u32,
}

pub struct Window   {
    window: WindowBuilder,
    sdl_context: Sdl
}

impl Window {
    pub fn new(title: &str, size: Size) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Window  {
            window: video_subsystem.window(title, size.width, size.height),
            sdl_context
        }
    }

    pub fn show(&mut self)   {
        let window = self.window.resizable().build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.clear();
        canvas.present();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop  {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>    {
                        break 'running
                    },
                    _ => {}
                }
            }
            canvas.present();
        }
    }
}

#[test]
fn test_window()    {
    let mut window = Window::new(
        "test",
        Size    {
            height: 200,
            width: 100,
        }
    );

    window.show();
}
