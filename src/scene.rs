use crate::{
    ecs::{
        component_system::core_components::{
            mesh_renderer::{self, MeshRender},
            script_component::{self, ScriptComponent},
        },
        gameobject::GameObject,
    },
    model::elements::pos3::Pos3,
    screenspace::{elements::drawable::Drawable, screen::screen::Screen},
}; // Add this line
use std::io::{self, Write};
use std::{collections::HashMap, thread::sleep, time::Duration};
pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<usize, GameObject>,
    current_id: usize,
}
impl Scene {
    pub fn with_dimensions(height: &usize, width: &usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            gameobjects: HashMap::new(),
            current_id: 0,
        }
    }
    pub fn add_object(&mut self, object: GameObject) {
        self.current_id += 1;
        self.gameobjects.insert(self.current_id, object);
        self.gameobjects
            .get_mut(&self.current_id)
            .unwrap()
            .set_id(&self.current_id);
    }
    pub fn get_object_with_id_mut_unwrap(&mut self, id: &usize) -> &mut GameObject {
        self.gameobjects.get_mut(&id).unwrap()
    }
    pub fn run(&mut self, fps: &u64) {
        let sleep_time: Duration = Duration::from_secs_f64(1.0 / *fps as f64);
        let delta_time: f64 = 1.0 / *fps as f64;
        print!("\x1B[?1049h\x1B[?25l");
        io::stdout().flush().unwrap();
        self.start_objects();
        loop {
            self.update_objects(&delta_time);
            self.draw_objects();
            self.screen.draw_and_flush();
            sleep(sleep_time);
        }
    }
    fn draw_objects(&mut self) {
        for object in self.gameobjects.iter_mut() {
            if let Some(mesh_renderer) = object.1.get_component_mut::<MeshRender>() {
                mesh_renderer.draw(&mut self.screen);
            }
        }
    }
    fn start_objects(&mut self) {
        for (id, object) in self.gameobjects.iter_mut() {
            let script_component = object.get_component_mut::<ScriptComponent>();
            if let Some(script_component) = object.get_component_mut::<ScriptComponent>() {
                script_component.start(self.get_object_with_id_mut_unwrap(id));
            }
        }
    }
    fn update_objects(&mut self, delta_time: &f64) {
        for (id, object) in self.gameobjects.iter_mut() {
            let script_component = object.get_component_mut::<ScriptComponent>();
            if let Some(script_component) = object.get_component_mut::<ScriptComponent>() {
                script_component.update(self.get_object_with_id_mut_unwrap(id), delta_time);
            }
        }
    }
}
