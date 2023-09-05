use blueprints::blueprint::BluePrint;
use blueprints::resources::{ResourceName};

mod factory;
mod blueprints;
mod game_data;

fn main() {
    let b = BluePrint::new(ResourceName::IronIngot, Some(30.0), None);
    let _f = b.new_factory(Some(2.0));
    // println!("{:?}", f.multi);
    let classes = game_data::extract(&"recipes.json".to_string());

    println!("{:?}", &classes[616].manufactoring_duration)
}

