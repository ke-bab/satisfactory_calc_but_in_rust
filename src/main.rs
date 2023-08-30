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
    // for cl in &classes {
    //     println!("{}", cl.mIngredients)
    // };
    parse_bracers(&classes[616].mIngredients);
    println!("{}", &classes[616].mIngredients)
}


#[derive(Debug)]
struct ResourceDesc {
    item_class: String,
    amount: i32,
}

fn parse_bracers(string: &str) -> Vec<ResourceDesc> {
    // here we trim outer braces, which always present
    let slice = (&string[1..string.len() - 1]);
    let mut i: usize = 0;
    let mut list: Vec<ResourceDesc> = Vec::new();
    while let Some(result) = next_desc(slice, i) {
        let new_desc = parse_desc(&result.0);
        list.push(new_desc);
        i = result.1;
    }
    println!("{:?}", &list);

    list
}

fn next_desc(string: &str, start: usize) -> Option<(String, usize)> {
    if start >= string.len() {
        return None;
    }
    let slice = &string[start..];
    let left_brace = slice.find("(");
    let right_brace = slice.find(")");
    if left_brace.is_some() && right_brace.is_some() {
        let left = left_brace.unwrap();
        let right = right_brace.unwrap();
        let new_desc = String::from(&slice[left + 1..right]);
        Some((new_desc, right + start + 1))
    } else {
        None
    }
}

fn parse_desc(string: &str) -> ResourceDesc {
    let mut item_and_amount: Vec<&str> = string.split(",").collect();
    let item_vec: Vec<&str> = item_and_amount[0].split("=").collect();
    let item_classname_vec: Vec<&str> = item_vec[1].split(".").collect();
    let item_classname = item_classname_vec[1];
    let bp_part = item_classname.find("BP_");
    let desc_part = item_classname.find("Desc_");
    let item: String;
    if bp_part.is_some() {
        item = item_classname[3..item_classname.len() - 4].to_string();
    } else if desc_part.is_some() {
        item = item_classname[5..item_classname.len() - 4].to_string();
    } else {
        item = item_classname[..item_classname.len() - 4].to_string();
    }

    let amount_vec: Vec<&str> = item_and_amount[1].split("=").collect();
    let amount: i32 = amount_vec[1].parse().unwrap();

    ResourceDesc { item_class: item, amount }
}
