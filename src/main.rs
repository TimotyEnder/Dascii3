use crate::{
    model::elements::pos3::Pos3, scene::Scene, screenspace::elements::cell_color::CellColor,
};
pub mod ecs;
pub mod model;
pub mod scene;
pub mod screenspace;
fn main() {
    let mut scene = Scene::with_dimensions(&(60 as usize), &(200 as usize));
    // scene.add_object(Box::new(Line::from_to(
    //     &Pos3::new(&-10.0, &10.0, &30.0),
    //     &Pos3::new(&10.0, &-10.0, &30.0),
    // )));
    // scene.add_object(Box::new(Cube::from_center(
    //     &Pos3::new(&0.0, &0.0, &20.0),
    //     5 as usize,
    // )));
    // scene.add_object(Box::new(SpinningCube::new(Cube::from_center_filled(
    //     &Pos3::new(&0.0, &0.0, &20.0),
    //     4 as usize,
    //     &CellColor::WHITE,
    //     &CellColor::RED,
    // ))));
    // scene.add_object(Box::new(SpinningLine::new(Line::from_to(
    //     &Pos3::new(&-10.0, &-5.0, &100.0),
    //     &Pos3::new(&10.0, &5.0, &100.0),
    // ))));
    scene.run(&60);
}
