use fltk::{self, app::set_visual, enums::Mode, prelude::*, text, window::Window};

pub struct Sandbox {
  width: i32,
  height: i32,
  redraw: bool,
}

impl Sandbox {
  pub fn new(width: i32, height: i32, redraw: bool) -> Self {
    Self {
      width,
      height,
      redraw,
    }
  }

  pub fn run_fltk<F>(&self, cb: F)
  where
    F: FnMut(&mut Window) + 'static,
  {
    let app = fltk::app::App::default();

    let mut window = Window::new(100, 100, self.width, self.height, "runner");

    window.draw(cb);

    window.handle(move |_, event| false);
    window.end();
    set_visual(Mode::Rgb).unwrap();
    window.show();

    if self.redraw {
      fltk::app::add_idle3(move |_| {
        window.redraw();
      });
    }

    app.run().unwrap();
  }

  pub fn make_draw_image(&self) -> Box<dyn Fn(&[u8])> {
    let width = self.width;
    let height = self.height;

    let f = move |pixels_buffer: &[u8]| {
      fltk::draw::draw_image(
        pixels_buffer,
        0,
        0,
        width,
        height,
        fltk::enums::ColorDepth::Rgb8,
      )
      .unwrap();
    };
    Box::new(f)
  }
}
