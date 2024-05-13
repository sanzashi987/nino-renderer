use crate::math::{lerp, Vec2};
#[derive(Clone, Copy, Debug)]
pub struct Edge {
  pub v1: Vec2,
  pub v2: Vec2,
}

pub struct Trapezoid {
  pub top: f32,
  pub bottom: f32,
  pub left: Edge,
  pub right: Edge,
}

impl Trapezoid {
  fn get_hang_trap(vertices: &[Vec2; 3]) -> Self {
    Trapezoid {
      top: vertices[0].y,
      bottom: vertices[2].y,
      left: Edge {
        v1: vertices[0],
        v2: vertices[2],
      },
      right: Edge {
        v1: vertices[1],
        v2: vertices[2],
      },
    }
  }

  fn get_portrait_trap(vertices: &[Vec2; 3]) -> Self {
    Trapezoid {
      top: vertices[0].y,
      bottom: vertices[1].y,
      left: Edge {
        v1: vertices[0],
        v2: vertices[1],
      },
      right: Edge {
        v1: vertices[0],
        v2: vertices[2],
      },
    }
  }

  pub fn from_triangle(vertices: &[Vec2; 3]) -> [Option<Self>; 2] {
    let mut vertices = *vertices;
    vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    if (vertices[0].x == vertices[1].x && vertices[0].x == vertices[2].x)
      || (vertices[0].y == vertices[1].y && vertices[0].y == vertices[2].y)
    {
      return [None, None];
    }

    if vertices[0].y == vertices[1].y {
      if vertices[0].x > vertices[1].x {
        vertices.swap(0, 1);
      }

      let trap = Self::get_hang_trap(&vertices);
      return [Some(trap), None];
    }

    if vertices[1].y == vertices[2].y {
      if vertices[1].x > vertices[2].x {
        vertices.swap(1, 2);
      }

      let trap = Self::get_portrait_trap(&vertices);
      return [Some(trap), None];
    }

    let k = (vertices[2].y - vertices[0].y) / (vertices[2].x - vertices[0].x);
    // k = k => (y2 -y0)/(x2-x0) = (y1-y0)/(x? -x0) = > x? = (y1-y0)/k+x0
    let dx = (vertices[1].y - vertices[0].y) / k + vertices[0].x;

    let d_vertex = Vec2::new(dx, vertices[1].y);

    if dx > vertices[1].x {
      let trap1 = Self::get_portrait_trap(&[vertices[0], vertices[1], d_vertex]);
      let trap2 = Self::get_hang_trap(&[vertices[1], d_vertex, vertices[2]]);
      return [Some(trap1), Some(trap2)];
    } else {
      let trap1 = Self::get_portrait_trap(&[vertices[0], d_vertex, vertices[1]]);
      let trap2 = Self::get_hang_trap(&[d_vertex, vertices[1], vertices[2]]);
      return [Some(trap1), Some(trap2)];
    }

    // return [None, None];
  }
}

pub struct Scanline {
  pub vertex: Vec2,
  pub step: Vec2,
  pub y: f32,
  pub width: f32,
}

impl Scanline {
  pub fn from_trapezoid(trap: &Trapezoid, init_y: f32) -> Self {
    let t1 = (init_y - trap.left.v1.y) / (trap.left.v2.y - trap.left.v1.y);
    let t2 = (init_y - trap.right.v1.y) / (trap.right.v2.y - trap.right.v1.y);

    let vertex_left = lerp(trap.left.v1, trap.left.v2, t1);
    let vertex_right = lerp(trap.right.v1, trap.right.v2, t2);
    let width = vertex_right.x - vertex_left.x;
    let rh_width = 1.0 / width;

    let position_step = (vertex_right - vertex_left) * rh_width;

    Self {
      vertex: vertex_left,
      step: position_step,
      y: init_y,
      width,
    }
  }
}
