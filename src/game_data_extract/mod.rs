use crate::blueprints::blueprint::BluePrint;
use serde_json::{Result, Value};
use serde::Deserialize;
use std::fmt::Error;
use std::fs::File;


#[derive(Deserialize)]
struct Json {
    NativeClass: String,
    Classes: Vec<Class>,
}

#[derive(Deserialize)]
pub struct Class {
    pub ClassName: String,
    pub FullName: String,
    pub mDisplayName: String,
    pub mIngredients: String,
    pub mProduct: String,
    pub mManufacturingMenuPriority: String,
    pub mManufactoringDuration: String,
    pub mManualManufacturingMultiplier: String,
    pub mProducedIn: String,
    pub mRelevantEvents: String,
    pub mVariablePowerConsumptionConstant: String,
    pub mVariablePowerConsumptionFactor: String,
}

pub fn extract(file: &str) -> Vec<Class> {
    let mut file = File::open(file).unwrap();
    let json: Json = serde_json::from_reader(file).unwrap();
    json.Classes
    // remember to exclude recursive recipes such as:
    // 1) unpack recipes - like water + canister (check it by hardcoded name)
    // 2) recipes where resource can be made from itself with byproduct
    // to check rule 2 we need go up and check is there any blueprint already with same resource as "result resource"
}