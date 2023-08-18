use crate::blueprints::resources::ResourceName;


pub struct Factory {
    pub multi: f32,
}

impl Factory {
    pub fn new(
        multi: Option<f32>,
    ) -> Factory {
        Factory {
            multi: multi.unwrap_or(1.0),
        }
    }
}
