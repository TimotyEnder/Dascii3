use crate::{
    model::{
        elements::pos3::Pos3,
        objects::{dot::Dot, line::Line},
    },
    scene::Scene,
};

pub mod model;
pub mod scene;
pub mod screenspace;
fn main() {
    let mut scene = Scene::with_dimensions(&(25 as usize), &(100 as usize));
    scene.add_object(Box::new(Line::from_to(
        &Pos3::new(&-10, &10, &1),
        &Pos3::new(&10, &-10, &1),
    )));
    scene.run(&15);
}
