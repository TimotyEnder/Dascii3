use std::collections::HashSet;

use crate::{
    model::elements::pos3::Pos3,
    screenspace::{
        elements::{cell_color::CellColor, drawable::Drawable, screenspace_position::ScreenPosition},
        screen::screen::Screen,
    },
};

pub struct Dot {
    position: Pos3,
    color: CellColor,
}
impl Dot {
    pub fn at_pos(pos: &Pos3) -> Self {
        Self {
            position: *pos,
            color: CellColor::WHITE,
        }
    }
    pub fn at_pos_with_color(pos: &Pos3, color: &CellColor) -> Self {
        Self {
            position: *pos,
            color: *color,
        }
    }
}
impl Drawable for Dot {
    fn draw(&self, screen: &mut Screen) -> HashSet<ScreenPosition> {
        let screen_pos: ScreenPosition = screen.project_point(&self.position);
        screen.color_cell(&screen_pos, &self.color);
        HashSet::from([screen_pos])
    }

    fn position(&self) -> Pos3 {
        self.position
    }
}
