use blueprints::blueprint::BluePrint;
use blueprints::resources::{ResourceName};

mod factory;
mod blueprints;

fn main() {
    let b = BluePrint::new(ResourceName::IronIngot, None, None, None);
    let f = b.new_factory(Some(2.0));
    println!("{:?}", f.multi)
}
