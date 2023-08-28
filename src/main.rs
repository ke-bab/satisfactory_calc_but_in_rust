use blueprints::blueprint::BluePrint;
use blueprints::resources::{ResourceName};

mod factory;
mod blueprints;
mod game_data_extract;

fn main() {
    let b = BluePrint::new(ResourceName::IronIngot, Some(30.0), None);
    let f = b.new_factory(Some(2.0));
    println!("{:?}", f.multi);
    let classes = game_data_extract::extract(&"recipes.json".to_string());
    for cl in &classes {
        println!("{}", cl.FullName)
    };
}
