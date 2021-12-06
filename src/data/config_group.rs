use crate::data::config_attribute as ca;

#[derive(PartialEq, Clone, Debug)]
pub struct ConfigGroup {
    group_name: String,
    config_attributes: Vec<ca::ConfigAttribute>,
}

impl ConfigGroup {
    pub fn new(grp_name: String) -> Result<ConfigGroup,String> {
        if !grp_name.is_empty() {
            Ok(ConfigGroup {
                group_name: grp_name,
                config_attributes: Vec::new(),
            })
        }
        else {
            Err(format!("1lt_config error: new ConfigGroup: grp_name can't be empty"))
        }
        
    }

    pub fn add_config_attribute(&mut self, config_att: ca::ConfigAttribute) -> Result<(), String> {
        let mut found: bool = false;
        for existing_att in &self.config_attributes {
            if existing_att.name == config_att.name {
                found = true;
            }
        }
        if found {
            return Err(format!("1lt_config error: add_config_attribute: config name {0} already exists within config group {1}", config_att.name, self.group_name));
        }
        else {
            self.config_attributes.push(config_att);
            return Ok(());
        }
    }

    pub fn update_config_attribute(&mut self, updated_att: ca::ConfigAttribute) -> Result<(), String> {

        //Prüfen ob mehr als 0 Objekte in der Gruppe sind
        if self.config_attributes.len() >= 1 { 
            //Attribut in der Gruppe finden
            for old_att in &mut self.config_attributes {
                if old_att.name == updated_att.name {
                    //Attribut aktualisieren
                    *old_att = updated_att.clone();
                    return Ok(());
                }
            }
        }
        else {
            //Error: Keine Attribute in der Gruppe
            return Err(format!("1lt_config error: config group: {g} config attribute: {a} | There are no config attributes aviable to update (config attribute count is < 1)",g = &self.group_name, a = &updated_att.name));
        }

        //attribute could not be found and updated
        Err(format!("1lt_config error: config group: {g} config attribute: {a} | There is no config attribute with the name {a} in the config group {g} to update", g= &self.group_name, a= &updated_att.name))
    }

    pub fn get_config_attribute(&self, att_name: String) -> Result<ca::ConfigAttribute, ()> {
        if self.config_attributes.len() >= 1 {
            for conf_att in &self.config_attributes {
                if conf_att.name == att_name {
                    return Ok(conf_att.clone());
                }
            }
        }
        //Attribute not found
        Err(())
    }

    pub fn group_name(&self) -> String {
        self.group_name.clone()
    }

    pub fn config_attributes(&self) -> Vec<ca::ConfigAttribute> {
        self.config_attributes.clone()
    }
        
}



//-----------------------------------------------------------
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::data::config_group as cg;
    use crate::data::config_attribute as ca;
    #[test]
    fn error_config_group_update_attribute_count_below_one(){
        let mut cfggrp = cg::ConfigGroup::new(format!("grp_name")).unwrap();
        assert_eq!(
            cfggrp.update_config_attribute(ca::ConfigAttribute::new(format!("att_name"), format!("att_value")).unwrap()),
            Err(format!("1lt_config error: config group: grp_name config attribute: att_name | There are no config attributes aviable to update (config attribute count is < 1)"))
        );
    }
    #[test]
    fn error_config_group_update_attribute_not_found(){
        let mut cfggrp = cg::ConfigGroup::new(format!("grp_name")).unwrap();
        cfggrp.add_config_attribute(ca::ConfigAttribute::new(format!("att_name"), format!("att_value")).unwrap());
        assert_eq!(
            cfggrp.update_config_attribute(ca::ConfigAttribute::new(format!("att_name2"), format!("att_value2")).unwrap()),
            Err(format!("1lt_config error: config group: grp_name config attribute: att_name2 | There is no config attribute with the name att_name2 in the config group grp_name to update"))
        );
    }
    #[test]
    fn find_attribute_in_group(){
        let mut cfggrp = cg::ConfigGroup::new(format!("grp_name")).unwrap();
        let cfgatt1 = ca::ConfigAttribute::new(format!("att_name1"), format!("att_value1")).unwrap();
        let cfgatt2 = ca::ConfigAttribute::new(format!("att_name2"), format!("att_value2")).unwrap();
        let cfgatt3 = ca::ConfigAttribute::new(format!("att_name3"), format!("att_value3")).unwrap();
        cfggrp.add_config_attribute(cfgatt1);
        cfggrp.add_config_attribute(cfgatt2.clone());
        cfggrp.add_config_attribute(cfgatt3);
        assert_eq!(
            cfggrp.get_config_attribute(format!("att_name2")).unwrap(),
            cfgatt2
        )
    }
    #[test]
    fn update_attribute(){
        let mut cfggrp = cg::ConfigGroup::new(format!("grp_name")).unwrap();
        let cfgatt1 = ca::ConfigAttribute::new(format!("att_name1"), format!("att_value1")).unwrap();
        let cfgatt2 = ca::ConfigAttribute::new(format!("att_name2"), format!("att_value2")).unwrap();
        let mut cfgatt3 = ca::ConfigAttribute::new(format!("att_name3"), format!("att_value3")).unwrap();
        cfggrp.add_config_attribute(cfgatt1);
        cfggrp.add_config_attribute(cfgatt2);
        cfggrp.add_config_attribute(cfgatt3.clone());
        cfgatt3 = ca::ConfigAttribute::new(format!("att_name3"), format!("att_value4")).unwrap();
        match cfggrp.update_config_attribute(cfgatt3.clone()) {
            Ok(()) => {},
            Err(error) => panic!("{}", error)
        };
        assert_eq!(
            cfggrp.get_config_attribute(format!("att_name3")).unwrap(),
            cfgatt3
        )
    }
}