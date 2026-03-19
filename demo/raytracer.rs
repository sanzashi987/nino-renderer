const ASPECT_RATIO: f32 = 16. / 9.;
const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / ASPECT_RATIO;

fn main() {
  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| {
    for j in (0..WINDOW_HEIGHT as usize).step_by(1) {
      for i in (0..WINDOW_WIDTH as usize).step_by(1) {
        // test_render(i, j, &mut buffer);
      }
    }
  });
}
