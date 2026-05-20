use crate::{
    ecs::{
        component_system::core_components::{
            body::{self, Body},
            script_component::{self, ScriptComponent},
        },
        gameobject::{self, GameObject},
    },
    screenspace::screen::screen::Screen,
}; // Add this line
use std::io::{self, Write};
use std::{collections::HashMap, thread::sleep, time::Duration};
pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<usize, GameObject>,
    scripts: HashMap<usize, ScriptComponent>,
}
impl Scene {
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            gameobjects: HashMap::new(),
            scripts: HashMap::new(),
        }
    }
    pub fn add_object(&mut self, object: GameObject) {
        self.gameobjects.insert(object.get_id(), object);
    }
    pub fn add_script(&mut self, script: ScriptComponent, gameobject_id: usize) {
        self.scripts.insert(gameobject_id, script);
    }
    pub fn run(&mut self, fps: u64) {
        let sleep_time: Duration = Duration::from_secs_f64(1.0 / fps as f64);
        let delta_time: f64 = 1.0 / fps as f64;
        print!("\x1B[?1049h\x1B[?25l");
        io::stdout().flush().unwrap();
        self.start_objects();
        loop {
            self.update_objects(delta_time);
            self.draw_objects();
            self.screen.draw_and_flush();
            sleep(sleep_time);
        }
    }
    fn draw_objects(&mut self) {
        for object in self.gameobjects.iter_mut() {
            if let Some(body) = object.1.get_component_mut::<Body>() {
                body.draw(&mut self.screen);
            }
        }
    }
    fn start_objects(&mut self) {
        for (id, script) in &mut self.scripts {
            script.start(self.gameobjects.get_mut(id).unwrap());
        }
    }
    fn update_objects(&mut self, delta_time: f64) {
        for (id, script) in &mut self.scripts {
            script.update(self.gameobjects.get_mut(id).unwrap(), delta_time);
        }
    }
}
