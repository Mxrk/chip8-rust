use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Window", 640, 320)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; 64]; 32]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = x as u32 * 10;
                let y = y as u32 * 10;
                self.canvas
                    .set_draw_color(pixels::Color::RGB(0, col * 255, 0));
                let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, 10, 10));
            }
        }
        self.canvas.present();
    }
}
