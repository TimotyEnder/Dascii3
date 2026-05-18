use crate::{
    ecs::gameobject::{self, GameObject},
    impl_component,
    scene::Scene,
};

pub struct ScriptComponent {
    pub name: String,
    pub enabled: bool,
    behavior: Box<dyn ScriptBehavior>,
    gameobject: &'static mut GameObject,
}
impl ScriptComponent {
    pub fn new<T: ScriptBehavior + 'static>(
        name: &str,
        behavior: T,
        gameobject: &'static mut GameObject,
    ) -> Self {
        Self {
            name: name.to_string(),
            enabled: true,
            behavior: Box::new(behavior),
            gameobject: gameobject,
        }
    }

    pub fn start(&mut self) {
        if self.enabled {
            self.behavior.start(self.gameobject);
        }
    }

    pub fn update(&mut self, delta_time: &f64) {
        if self.enabled {
            self.behavior.update(self.gameobject, delta_time);
        }
    }
    pub fn set_enabled(&mut self, set: bool) {
        self.enabled = set;
    }
}
impl_component!(ScriptComponent);
pub trait ScriptBehavior: Send + Sync {
    fn start(&mut self, gameobject: &mut GameObject);
    fn update(&mut self, gameobject: &mut GameObject, delta_time: &f64);
    fn clone_box(&self) -> Box<dyn ScriptBehavior>;
}
impl Clone for Box<dyn ScriptBehavior> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
