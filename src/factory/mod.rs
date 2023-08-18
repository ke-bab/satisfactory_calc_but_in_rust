use crate::blueprints::resources::ResourceName;


pub struct Factory {
    pub resource: ResourceName,
    pub multiplier: f32,
}

impl Factory {
    pub fn new(
        resource: ResourceName,
        multiplier: Option<f32>,
    ) -> Factory {
        Factory {
            resource,
            multiplier: multiplier.unwrap_or(1.0),
        }
    }
}
