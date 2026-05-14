use crate::{
    model::elements::pos3::Pos3,
    screenspace::{elements::screenspace_position::ScreenPosition, screen::screen::Screen},
};

pub trait Drawable {
    fn draw(&self, screen: &mut Screen) -> Vec<ScreenPosition>;
    fn position(&self) -> Pos3;
}
