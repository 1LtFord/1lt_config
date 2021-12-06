#[derive(PartialEq,Clone, Debug)]
pub struct ConfigAttribute {
    pub name: String,
    pub value: String,
}

impl ConfigAttribute {
    pub fn new(att_name: String, att_value: String) -> Result<ConfigAttribute, String> {
        if !att_name.is_empty() || !att_value.is_empty() {
            Ok(ConfigAttribute {
                name: att_name,
                value: att_value,
            })
        }
        else {
            Err(format!("1lt_config error: new ConfigAttribute: nor att_name or att_value can be empty"))
        }
        
    }
}