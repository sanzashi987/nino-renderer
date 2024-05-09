use crate::math::Vec2;

const INSIDE: u8 = 0b0000;
const LEFT: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const TOP: u8 = 0b1000;

fn get_outcode(p: &Vec2, rect_min: &Vec2, rect_max: &Vec2) -> u8 {
  let mut code: u8 = 0;
  code |= if p.x < rect_min.x {
    LEFT
  } else if p.x > rect_max.x {
    RIGHT
  } else {
    INSIDE
  };
  code |= if p.y < rect_min.y {
    TOP
  } else if p.y > rect_max.y {
    BOTTOM
  } else {
    INSIDE
  };
  code
}

pub fn clip(
  p1: &Vec2,
  p2: &Vec2,
  rect_min: &Vec2,
  rect_max: &Vec2,
) -> Option<(Vec2, Vec2)> {
  // math::Vec2 {}
  let mut pt1 = *p1;
  let mut pt2 = *p2;

  let mut outcode1 = get_outcode(&pt1, rect_min, rect_max);
  let mut outcode2 = get_outcode(&pt2, rect_min, rect_max);

  loop {
    if outcode1 & outcode2 != 0 {
      return None;
    } else if (outcode1 | outcode2) == 0 {
      return Some((pt1, pt2));
    }

    let mut p = Vec2 { x: 0.0, y: 0.0 };

    let outcode = if outcode2 > outcode1 {
      outcode2
    } else {
      outcode1
    };

    if outcode & TOP != 0 {
      p.x = p1.x + (pt2.x - pt1.x) * (rect_min.y - pt1.y) / (pt2.y - pt1.y);
      p.y = rect_min.y;
    } else if outcode & BOTTOM != 0 {
      p.x = p1.x + (pt2.x - pt1.x) * (rect_max.y - pt1.y) / (pt2.y - pt1.y);
      p.y = rect_max.y;
    } else if outcode & LEFT != 0 {
      p.y = p1.y + (pt2.y - pt1.y) * (rect_min.x - pt1.x) / (pt2.x - pt1.x);
      p.x = rect_min.x
    } else if outcode & RIGHT != 0 {
      p.y = p1.y + (pt2.y - pt1.y) * (rect_max.x - pt1.x) / (pt2.x - pt1.x);
      p.x = rect_max.x
    }

    if outcode == outcode1 {
      pt1 = p;
      outcode1 = get_outcode(&pt1, rect_min, rect_max);
    } else {
      pt2 = p;
      outcode2 = get_outcode(&pt2, rect_min, rect_max);
    }
  }
}
