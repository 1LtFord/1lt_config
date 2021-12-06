use crate::data::config_group as cg;

#[derive(PartialEq, Clone, Debug)]
pub struct ConfigFile{
    pub file_path: String,
    pub config_groups: Vec<cg::ConfigGroup>,
}

impl ConfigFile {
    pub fn new(file_path: String) -> ConfigFile {
        ConfigFile { 
            file_path: file_path,
            config_groups: Vec::new()
        }
    }

    pub fn add_config_group(&mut self, grp_name: String) -> Result<(), String> {
        let mut found: bool = false;
        if !grp_name.is_empty() {
            for conf_grp in &self.config_groups {
                if conf_grp.group_name() == grp_name {
                    found = true;
                }
            }   
            if found {
                return Err(format!("1lt_config error: add_config_group: group name {} already exists", grp_name));
            }
            else {
                self.config_groups.push(cg::ConfigGroup::new(grp_name).unwrap());
                return Ok(());
            }
        }
        else {
            return Err(format!("1lt_config error: add_config_group: group name is empty"));
        }
    }

    pub fn get_config_group(&self, grp_name: String) -> Result<cg::ConfigGroup, String> {
        if !grp_name.is_empty() {
            for conf_grp in &self.config_groups {
                if conf_grp.group_name() == grp_name {
                    return Ok(conf_grp.clone());
                }
            }
            return Err(format!("1lt_config error: get_config_group: {} not found", grp_name));
        }
        else {
            return Err(format!("1lt_config error: get_config_group: group name is empty"));
        }
    }
}



//-----------------------------------------------------------
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::data::config_file as cf;
    use crate::data::config_group as cg;

    #[test]
    fn error_add_config_group_name_already_exists(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("asdf"));
        cfile.add_config_group(format!("a"));
        assert_eq!(
            cfile.add_config_group(format!("a")),
            Err(format!("1lt_config error: add_config_group: group name a already exists"))
        )
    }
    #[test]
    fn error_add_config_group_name_empty(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("asdf"));
        assert_eq!(
            cfile.add_config_group(format!("")),
            Err(format!("1lt_config error: add_config_group: group name is empty"))
        )
    }
    #[test]
    fn error_get_config_group_not_found(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("asdf"));
        cfile.add_config_group(format!("a"));
        assert_eq!(
            cfile.get_config_group(format!("b")),
            Err(format!("1lt_config error: get_config_group: b not found"))
        )
    }
    #[test]
    fn error_get_config_group_name_empty(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("asdf"));
        assert_eq!(
            cfile.get_config_group(format!("")),
            Err(format!("1lt_config error: get_config_group: group name is empty"))
        )
    }
    #[test]
    fn get_config_group(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("asdf"));
        cfile.add_config_group(format!("a"));
        cfile.add_config_group(format!("b"));
        cfile.add_config_group(format!("c"));
        let mut cgroup: cg::ConfigGroup = cg::ConfigGroup::new(format!("b")).unwrap();
        assert_eq!(
            cfile.get_config_group(format!("b")).unwrap(),
            cgroup
        )
    }
}