use crate::{
  // math::{lerp, Vec2}
  math,
  shader::{self, interp_attributes, Vertex},
};
#[derive(Clone, Copy, Debug)]
pub struct Edge {
  pub v1: Vertex,
  pub v2: Vertex,
}

pub struct Trapezoid {
  pub top: f32,
  pub bottom: f32,
  pub left: Edge,
  pub right: Edge,
}

impl Trapezoid {
  fn get_hang_trap(vertices: &[Vertex; 3]) -> Self {
    Trapezoid {
      top: vertices[0].position.y,
      bottom: vertices[2].position.y,
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

  fn get_portrait_trap(vertices: &[Vertex; 3]) -> Self {
    Trapezoid {
      top: vertices[0].position.y,
      bottom: vertices[2].position.y,
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

  pub fn from_triangle(vertices: &[Vertex; 3]) -> [Option<Self>; 2] {
    let mut vertices = *vertices;
    vertices.sort_by(|a, b| a.position.y.partial_cmp(&b.position.y).unwrap());

    if (vertices[0].position.x == vertices[1].position.x
      && vertices[0].position.x == vertices[2].position.x)
      || (vertices[0].position.y == vertices[1].position.y
        && vertices[0].position.y == vertices[2].position.y)
    {
      return [None, None];
    }

    if vertices[0].position.y == vertices[1].position.y {
      if vertices[0].position.x > vertices[1].position.x {
        vertices.swap(0, 1);
      }

      let trap = Self::get_hang_trap(&vertices);
      return [Some(trap), None];
    }

    if vertices[1].position.y == vertices[2].position.y {
      if vertices[1].position.x > vertices[2].position.x {
        vertices.swap(1, 2);
      }

      let trap = Self::get_portrait_trap(&vertices);
      return [Some(trap), None];
    }

    let t = (vertices[1].position.y - vertices[0].position.y)
      / (vertices[2].position.y - vertices[0].position.y);
    // let k = (vertices[2].position.y - vertices[0].position.y)
    // k = k => (y2 -y0)/(x2-x0) = (y1-y0)/(x? -x0) = > x? = (y1-y0)/k+x0
    let dx = t * (vertices[2].position.x - vertices[0].position.x) + vertices[0].position.x;

    let (mut v0, mut v2) = (vertices[0], vertices[2]);
    shader::vertex_rhw_init(&mut v0);
    shader::vertex_rhw_init(&mut v2);
    let mut d_vertex = shader::lerp_vertex(&v0, &v2, t);

    shader::attributes_foreach(&mut d_vertex.attributes, |v| v / d_vertex.position.z);
    d_vertex.position.z = 1.0 / d_vertex.position.z;

    if dx > vertices[1].position.x {
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
  pub vertex: Vertex,
  pub step: Vertex,
  pub y: f32,
  pub width: f32,
}

impl Scanline {
  pub fn from_trapezoid(trap: &Trapezoid, init_y: f32) -> Self {
    let t1 =
      (init_y - trap.left.v1.position.y) / (trap.left.v2.position.y - trap.left.v1.position.y);
    let t2 =
      (init_y - trap.right.v1.position.y) / (trap.right.v2.position.y - trap.right.v1.position.y);

    let vertex_left = shader::lerp_vertex(&trap.left.v1, &trap.left.v2, t1);
    let vertex_right = shader::lerp_vertex(&trap.right.v1, &trap.right.v2, t2);
    let width = vertex_right.position.x - vertex_left.position.x;
    let rh_width = 1.0 / width;

    let position_step = (vertex_right.position - vertex_left.position) * rh_width;
    let attribute_step = interp_attributes(
      &vertex_left.attributes,
      &vertex_right.attributes,
      |v1, v2, t| (v2 - v1) * t,
      rh_width,
    );

    Self {
      vertex: vertex_left,
      step: Vertex {
        position: position_step,
        attributes: attribute_step,
      },
      y: init_y,
      width,
    }
  }
}

fn near_plane_clip_line(out: &Vertex, inner: &Vertex, near_plane_z: f32) -> Vertex {
  let proportion = (near_plane_z - inner.position.z) / (out.position.z - inner.position.z);
  let position =  (out.position - inner.position) * proportion + inner.position;

  let attributes = interp_attributes(&inner.attributes, &out.attributes, math::lerp, proportion);

  Vertex {
    position,
    attributes,
  }
}

pub(crate) fn near_plane_clip(
  vertices: &[Vertex],
  near: f32,
) -> ([Vertex; 3], Option<[Vertex; 3]>) {
  let near = -near;
  if vertices[0].position.z > near {
    if vertices[1].position.z > near {
      let new_vertex1 = near_plane_clip_line(&vertices[0], &vertices[2], near);
      let new_vertex2 = near_plane_clip_line(&vertices[1], &vertices[2], near);
      ([new_vertex1, new_vertex2, vertices[2]], None)
    } else if vertices[2].position.z > near {
      let new_vertex1 = near_plane_clip_line(&vertices[0], &vertices[1], near);
      let new_vertex2 = near_plane_clip_line(&vertices[2], &vertices[1], near);
      return ([new_vertex1, vertices[1], new_vertex2], None);
    } else {
      let new_vertex1 = near_plane_clip_line(&vertices[0], &vertices[1], near);
      let new_vertex2 = near_plane_clip_line(&vertices[0], &vertices[2], near);
      return (
        [vertices[1], new_vertex2, new_vertex1],
        Some([vertices[1], vertices[2], new_vertex2]),
      );
    }
  } else if vertices[1].position.z > near {
    if vertices[2].position.z > near {
      let new_vertex1 = near_plane_clip_line(&vertices[1], &vertices[0], near);
      let new_vertex2 = near_plane_clip_line(&vertices[2], &vertices[0], near);
      return ([vertices[0], new_vertex1, new_vertex2], None);
    } else {
      let new_vertex1 = near_plane_clip_line(&vertices[2], &vertices[1], near);
      let new_vertex2 = near_plane_clip_line(&vertices[0], &vertices[1], near);
      return (
        [vertices[0], new_vertex2, new_vertex1],
        Some([vertices[0], new_vertex1, vertices[2]]),
      );
    }
  } else {
    let new_vertex1 = near_plane_clip_line(&vertices[2], &vertices[0], near);
    let new_vertex2 = near_plane_clip_line(&vertices[2], &vertices[1], near);
    return (
      [vertices[0], new_vertex2, new_vertex1],
      Some([vertices[0], vertices[1], new_vertex2]),
    );
  }
}
