use crate::model::elements::pos3::Pos3;
#[derive(Hash, Clone, Copy, Eq)]
pub struct ScreenPosition {
    pub x: usize,
    pub y: usize,
}
impl ScreenPosition {
    pub fn with_pos(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl PartialEq for ScreenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
