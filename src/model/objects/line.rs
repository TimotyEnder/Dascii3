use std::cmp::{Ordering, max};

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
    fn distanceVector(from: &ScreenPosition, to: &ScreenPosition) -> (f64, f64) {
        let diffVector = (
            (to.x() as isize) - (from.x() as isize),
            (to.y() as isize) - (from.y() as isize),
        );
        let magnitude = ((diffVector.0 as f64).powi(2) + (diffVector.1 as f64).powi(2)).sqrt();
        (
            diffVector.0 as f64 / magnitude,
            diffVector.1 as f64 / magnitude,
        )
    }
    //TODO: change to Bresenham's line algorithm
    fn move_pos_towards_dir(x_dir: &f64, y_dir: &f64, pos: &mut ScreenPosition) {
        if (x_dir.abs() - y_dir.abs()).abs() < 0.5 {
            match *y_dir {
                n if n < 0.0 => pos.set_y(&(pos.y() - 1)),
                _ => pos.set_y(&(pos.y() + 1)),
            };
            match *x_dir {
                n if n < 0.0 => pos.set_x(&(pos.x() - 1)),
                _ => pos.set_x(&(pos.x() + 1)),
            };
        } else if x_dir.abs() > y_dir.abs() {
            if *x_dir > 0.0 {
                pos.set_x(&(pos.x() + 1));
            } else {
                pos.set_x(&(pos.x() - 1));
            }
        } else {
            if *y_dir > 0.0 {
                pos.set_y(&(pos.y() + 1));
            } else {
                pos.set_y(&(pos.y() - 1));
            }
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
    ) {
        let dx = (to.x() as isize - from.x() as isize).abs(); //total x distance
        let dy = (to.y() as isize - from.y() as isize).abs(); //total y distance
        let sx = if to.x() >= from.x() { 1 } else { -1 }; //step for x
        let sy = if to.y() >= from.y() { 1 } else { -1 }; //step for y
        let mut err = dx - dy; //deviation from mathematical line and actual pixel position, decides next movement
        let mut x = from.x() as isize;
        let mut y = from.y() as isize;

        loop {
            screen.color_cell(
                &ScreenPosition::with_pos(&(x as usize), &(y as usize)),
                &self.color,
            );
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
    }
}
impl Drawable for Line {
    fn draw(&self, screen: &mut Screen) {
        let from_pos: ScreenPosition = screen.project_point(&self.from);
        let to_pos: ScreenPosition = screen.project_point(&self.to);
        // screen.color_cell(&from_pos, &self.color);
        // let mut cur_pos = from_pos;
        // while (cur_pos != to_pos) {
        //     let (x_dir, y_dir) = Self::distanceVector(&cur_pos, &to_pos);
        //     Self::move_pos_towards_dir(&x_dir, &y_dir, &mut cur_pos);
        //     screen.color_cell(&cur_pos, &self.color);
        // }
        self.bresenham_line_algorithm(&from_pos, &to_pos, screen);
    }

    fn position(&self) -> Pos3 {
        self.mid_point()
    }
}
