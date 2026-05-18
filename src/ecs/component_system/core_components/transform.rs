use crate::{ecs::gameobject::GameObject, impl_component, model::elements::pos3::Pos3};

pub struct Transform {
    position: Pos3,
    rotation: (f64, f64, f64),
}
impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Pos3::new(&0.0, &0.0, &0.0),
            rotation: (0.0, 0.0, 0.0),
        }
    }
    pub fn get_position(&self) -> Pos3 {
        self.position
    }
    pub fn get_rotation(&self) -> (f64, f64, f64) {
        self.rotation
    }
}
impl_component!(Transform);
