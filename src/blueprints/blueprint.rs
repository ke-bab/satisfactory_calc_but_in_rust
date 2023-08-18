use crate::blueprints::resources::ResourceName;
use crate::factory::Factory;

pub struct BluePrint {
    pub resource: ResourceName,
    pub production_rate: f32,
    pub is_alternate: bool,
    pub available_blueprints: Vec<BluePrint>,
}

impl BluePrint {
    pub fn new(
        resource: ResourceName,
        production_rate: Option<f32>,
        is_alternate: Option<bool>,
        available_blueprints: Option<Vec<BluePrint>>,
    ) -> BluePrint {
        BluePrint {
            resource,
            production_rate: production_rate.unwrap_or(1.0),
            is_alternate: is_alternate.unwrap_or(false),
            available_blueprints: available_blueprints.unwrap_or(Vec::new()),
        }
    }
    pub fn new_factory(&self, multi: Option<f32>) -> Factory {
        Factory {
            multi: multi.unwrap_or(1.0)
        }
    }
}
