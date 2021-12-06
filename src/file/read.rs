use crate::data::config_file as cf;
use crate::data::config_attribute as ca;

use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn read_config_file(file_path: String) -> Result<cf::ConfigFile, String> {
    if file_path.is_empty() {
        return Err(format!("1lt_config error: read_config_file: file_path can't be empty"));
    }
    
    let file = match File::open(file_path.clone()) {
        Ok(value) => value,
        Err(error) => return Err(format!("1lt_config error: read_config_file: Could not open config file - {}", error))
    };

    get_config_from_file(BufReader::new(file), cf::ConfigFile::new(file_path.clone()))
}

fn get_config_from_file(reader: BufReader<File>,mut config_file: cf::ConfigFile) -> Result<cf::ConfigFile, String> {
    let mut grouped = false;
    
    for line in reader.lines() {
        let mut text = match line {
            Ok(value) => value,
            Err(error) => return Err(format!("1lt_config error: get_config_from_file: config file does contain invalid UTF-8 - {}", error))
        };

        //Group
        if text.starts_with("[") && text.ends_with("]") {
            text.remove(0);
            text.remove(text.len()-1);

            if !text.is_empty() {
                    config_file.add_config_group(text.clone())?;
                    grouped = true;
                    continue;
            }
            else {
                return Err(format!("1lt_config error: get_config_from_file: a group name can't be empty"));
            }   
        }

        //Attribute
        else if text.contains("=") {
            let split: Vec<&str> = text.splitn(2, "=").collect();
            if split.len() == 2 {
                let mut name = split[0].to_string();
                let mut value = split[1].to_string();
                
                //remove space surrounding equal sign
                if name.ends_with(" ") {
                    name.remove(name.len()-1);
                }
                if value.starts_with(" ") {
                    value.remove(0);
                }

                if grouped {
                    if !name.is_empty() && !value.is_empty() {
                        let index = config_file.config_groups.len()-1;
                        config_file.config_groups[index].add_config_attribute(ca::ConfigAttribute::new(name, value)?);
                        continue;
                    }
                    else {
                        if name.is_empty() && !value.is_empty(){
                            return Err(format!("1lt_config error: get_config_from_file: config attribute name can't be empty: {}", text))
                        }
                        else if !name.is_empty() && value.is_empty() {
                            return Err(format!("1lt_config error: get_config_from_file: config attribute value can't be empty: {}", text))
                        }
                        else {
                            return Err(format!("1lt_config error: get_config_from_file: nor config attribute name or value can be empty: {}", text))
                        }
                    }
                }
                else {
                    return Err(format!("1lt_config error: get_config_from_file: a config attribute must be assigned to a group: {}", text))
                }

            }
            else {
                return Err(format!("1lt_config error: get_config_from_file: nor config attribute name or value can be empty: {} {}", text, split.len()))
            }
        }
    }

    Ok(config_file)
}




//-------------------------------------------------------------------------
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::data::config_file as cf;
    use crate::data::config_group as cg;
    use crate::data::config_attribute as ca;
    use crate::file::read as read;

    #[test]
    fn read_file(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("test/read_test.txt"));
        cfile.add_config_group(format!("group1"));
        cfile.add_config_group(format!("group2"));
        cfile.config_groups[0].add_config_attribute(ca::ConfigAttribute::new(format!("name1"), format!("value1")).unwrap());
        cfile.config_groups[1].add_config_attribute(ca::ConfigAttribute::new(format!("name2"), format!("value2")).unwrap());
        assert_eq!(
            cfile,
            read::read_config_file(format!("test/read_test.txt")).unwrap()
        )
    }
}