use crate::blueprints::resources::ResourceName;
use crate::factory::Factory;

pub struct BluePrint {
    pub resource: ResourceName,
    pub byproduct: Option<ResourceName>,
    pub production_rate: f32,
    pub is_alternate: bool,
    pub is_primitive: bool,
    pub is_recursive: bool,
    pub ingredients: Vec<ResourceName>,
}

impl BluePrint {
    pub fn new(
        resource: ResourceName,
        production_rate: Option<f32>,
        is_alternate: Option<bool>,
    ) -> BluePrint {
        BluePrint {
            resource,
            byproduct: None,
            is_primitive: false,
            is_recursive: false,
            production_rate: production_rate.unwrap_or(1.0),
            is_alternate: is_alternate.unwrap_or(false),
            ingredients: Vec::new(),
        }
    }
    pub fn new_factory(&self, multi: Option<f32>) -> Factory {
        Factory {
            multi: multi.unwrap_or(1.0)
        }
    }
}
