use std::any::Any;

use crate::ecs::gameobject::{self, GameObject};

pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn component_name(&self) -> &'static str;
    fn set_parent(&mut self, gameobject: &GameObject);
}
#[macro_export]
macro_rules! impl_component {
    ($struct_name:ty) => {
        use crate::ecs::component_system::component::Component;
        use std::any::Any;
        impl Component for $struct_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn component_name(&self) -> &'static str {
                std::any::type_name::<$struct_name>()
            }
            fn set_parent(&mut self, gameobject: &GameObject) {
                self.parent = Some(gameobject);
            }
        }
    };
}
