use crate::blueprints::blueprint::BluePrint;
use serde_json::{Result, Value};
use serde::Deserialize;
use std::fmt::Error;
use std::fs::File;
use std::str::FromStr;


#[derive(Deserialize)]
struct Json {
    #[serde(rename = "Classes")]
    classes: Vec<Class>,
}

#[derive(Deserialize)]
pub struct Class {
    #[serde(rename = "ClassName")]
    pub class_name: String,
    #[serde(rename = "mDisplayName")]
    pub m_display_name: String,
    #[serde(rename = "mIngredients")]
    pub m_ingredients: String,
    #[serde(rename = "mProduct")]
    pub m_product: String,
    #[serde(rename = "mManufactoringDuration")]
    pub m_manufactoring_duration: String,
}

pub struct ParsedClass {
    pub class_name: String,
    pub display_name: String,
    pub ingredients: Vec<ResourceDesc>,
    pub products: Vec<ResourceDesc>,
    pub manufactoring_duration: i32,
}

pub fn extract(file: &str) -> Vec<ParsedClass> {
    let file = File::open(file).unwrap();
    let json: Json = serde_json::from_reader(file).unwrap();
    let mut parsed_vec: Vec<ParsedClass> = Vec::new();
    for class in &json.classes {
        let ingred_desc = parse_bracers(&class.m_ingredients);
        let prod_desc = parse_bracers(&class.m_product);
        let float_duration:f32 = class.m_manufactoring_duration.parse().unwrap();
        let parsed = ParsedClass {
            class_name: class.class_name.clone(),
            display_name: class.m_display_name.clone(),
            ingredients: ingred_desc,
            products: prod_desc,
            manufactoring_duration: float_duration as i32,
        };
        parsed_vec.push(parsed);
    }
    parsed_vec
    // remember to exclude recursive recipes.
    // to check rule 2 we need go up and check is there any blueprint already with same resource as "result resource"
}


#[derive(Debug)]
pub struct ResourceDesc {
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

/// * `string` - item value and amount value without braces, divided by comma.
/// # example:
/// ItemClass=BlueprintGeneratedClass'"/Game/FactoryGame/Resource/Parts/Cement/Desc_Cement.Desc_Cement_C"',Amount=3
fn parse_desc(string: &str) -> ResourceDesc {
    let comma_pos = string.find(",").unwrap();
    // find value of item after =
    let item_value = parse_item_value(&string[..comma_pos]);
    // strip value from suffix
    let item_value_stripped = strip_item_prefix_and_suffix(&item_value);
    // find value of amount after =
    // parse int
    // return new struct


    let item_and_amount: Vec<&str> = string.split(",").collect();
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

fn parse_item_value(string: &str) -> &str {
    let eq_pos = string.find("=").unwrap();

    return &string[eq_pos+1..]
}

fn strip_item_prefix_and_suffix(string: &str) -> &str {

}
