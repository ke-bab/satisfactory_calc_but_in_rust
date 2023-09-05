use std::fs::File;
use std::str::FromStr;

use serde::Deserialize;

use parse::ResourceDesc;

mod parse;

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
        let ingred_desc = parse::parse_descriptions(&class.m_ingredients);
        let prod_desc = parse::parse_descriptions(&class.m_product);
        let float_duration: f32 = class.m_manufactoring_duration.parse().unwrap();
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

