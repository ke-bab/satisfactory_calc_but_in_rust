#[derive(Debug)]
pub struct ResourceDesc {
    pub name: String,
    pub amount: i32,
}

/// parse_descriptions heavily rely on string format, so any difference will break it.
/// # string example:
/// ((ItemClass=BlueprintGeneratedClass'"/Game/FactoryGame/Resource/Parts/IronRod/Desc_IronRod.Desc_IronRod_C"',Amount=2),(ItemClass=BlueprintGeneratedClass'"/Game/FactoryGame/Resource/Parts/IronPlate/Desc_IronPlate.Desc_IronPlate_C"',Amount=2),(ItemClass=BlueprintGeneratedClass'"/Game/FactoryGame/Resource/Parts/Cement/Desc_Cement.Desc_Cement_C"',Amount=2))
///
/// use at your own risk.
pub fn parse_descriptions(string: &str) -> Vec<ResourceDesc> {
    // here we trim outer braces, which always present
    let slice = (&string[1..string.len() - 1]);
    let mut start = 0;
    let mut list: Vec<ResourceDesc> = Vec::new();
    while let Some(r_pos) = has_next(slice, start) {
        let new_desc = parse_item_and_amount(&slice[start + 1..r_pos]);
        list.push(new_desc);
        start = r_pos + 2;
    }
    fn has_next(string: &str, pos: usize) -> Option<usize> {
        if pos > string.len() {
            return None;
        }
        let found_brace = (&string[pos..]).find(")");
        if found_brace.is_some() {
            // here we adding "start pos" + "pos of right brace" because we want absolute index
            // from beginning of entire string, not from a slice
            Some(found_brace.unwrap() + pos)
        } else {
            None
        }
    }

    list
}

pub fn strip_class_name(string: &str) -> &str {
    &string[7..string.len()-2]
}

/// * `string` - item value and amount value without braces, divided by comma.
/// # example:
/// ItemClass=BlueprintGeneratedClass'"/Game/FactoryGame/Resource/Parts/Cement/Desc_Cement.Desc_Cement_C"',Amount=3
fn parse_item_and_amount(string: &str) -> ResourceDesc {
    let comma_pos = string.find(",").unwrap();
    let item = parse_item(&string[..comma_pos]);
    let amount = parse_amount(&string[comma_pos+1..]);

    ResourceDesc { name: item.to_string(), amount }
}

fn parse_item(string: &str) -> &str {
    let slice = get_value_after_equal_sign(&string);
    fn strip_item_prefix_and_suffix(string: &str) -> &str {
        let bp_part = string.find("BP_");
        let desc_part = string.find("Desc_");
        if bp_part.is_some() {
            &string[3..string.len() - 4]
        } else if desc_part.is_some() {
            &string[5..string.len() - 4]
        } else {
            &string[..string.len() - 4]
        }
    }
    fn pick_name_after_dot(string: &str) -> &str {
        let dot_pos = string.find(".").unwrap();
        &string[dot_pos+1..]
    }
    let name_of_class = pick_name_after_dot(&slice);
    strip_item_prefix_and_suffix(&name_of_class)
}

fn get_value_after_equal_sign(string: &str) -> &str {
    let eq_pos = string.find("=").unwrap();
    &string[eq_pos + 1..]
}

fn parse_amount(string: &str) -> i32 {
    get_value_after_equal_sign(&string).parse::<i32>().unwrap()
}