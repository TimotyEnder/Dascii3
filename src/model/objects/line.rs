use std::cmp::{Ordering, max};
use std::collections::HashSet;

use crate::{
    model::elements::pos3::{self, Pos3},
    screenspace::{
        elements::{
            cell_color::CellColor,
            drawable::{self, Drawable},
            screenspace_position::ScreenPosition,
        },
        screen::screen::Screen,
    },
};

pub struct Line {
    from: Pos3,
    to: Pos3,
    color: CellColor,
}
impl Line {
    pub fn from_to(from: &Pos3, to: &Pos3) -> Self {
        Self {
            from: *from,
            to: *to,
            color: CellColor::WHITE,
        }
    }
    pub fn from_to_with_color(from: &Pos3, to: &Pos3, color: &CellColor) -> Self {
        Self {
            from: *from,
            to: *to,
            color: *color,
        }
    }
    fn mid_point(&self) -> Pos3 {
        Pos3::new(
            &(self.from.x() + self.to.x() / 2.0),
            &(self.from.y() + self.to.y() / 2.0),
            &(self.from.z() + self.to.z() / 2.0),
        )
    }
    // literally just https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
    fn bresenham_line_algorithm(
        &self,
        from: &ScreenPosition,
        to: &ScreenPosition,
        screen: &mut Screen,
    ) -> HashSet<ScreenPosition> {
        let dx = (to.x() as isize - from.x() as isize).abs(); //total x distance
        let dy = (to.y() as isize - from.y() as isize).abs(); //total y distance
        let sx = if to.x() >= from.x() { 1 } else { -1 }; //step for x
        let sy = if to.y() >= from.y() { 1 } else { -1 }; //step for y
        let mut err = dx - dy; //deviation from mathematical line and actual pixel position, decides next movement
        let mut x = from.x() as isize;
        let mut y = from.y() as isize;
        let mut colored_pos = HashSet::new();
        colored_pos.insert(*from);
        loop {
            let to_color = ScreenPosition::with_pos(&(x as usize), &(y as usize));
            colored_pos.insert(to_color);
            screen.color_cell(&to_color, &self.color);

            if x == to.x() as isize && y == to.y() as isize {
                break;
            }
            //if 2*err > -dy, then take x step
            //if 2*err < dx, then take y step
            let e2 = 2 * err; //avoids fractions and enables the use of integer maths
            if e2 > -dy {
                //above the y of the current line so go forward
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                //in front of the x  so go up to be on the same height
                err += dx;
                y += sy;
            }
        }
        colored_pos
    }
}
impl Drawable for Line {
    fn draw(&self, screen: &mut Screen) -> HashSet<ScreenPosition> {
        let from_pos: ScreenPosition = screen.project_point(&self.from);
        let to_pos: ScreenPosition = screen.project_point(&self.to);
        self.bresenham_line_algorithm(&from_pos, &to_pos, screen)
    }

    fn position(&self) -> Pos3 {
        self.mid_point()
    }
}
