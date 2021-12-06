use crate::data::config_file as cf;
use crate::data::config_group as cg;
use crate::data::config_attribute as ca;

use std::io::{BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn write_config_file(config_file: &cf::ConfigFile) -> Result<(),String> {
    let mut file = match get_file_writer(&config_file.file_path) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };

    match write_config_to_file(&config_file, file) {
        Ok(()) => return Ok(()),
        Err(error) => return Err(error)
    };
}

fn get_file_writer(path: &String) -> Result<File, String> {
    match OpenOptions::new().read(true).write(true).create(true).open(&path) {
        Ok(value) => return Ok(value),
        Err(error) => return Err(format!("1lt_config error: get_file_writer: Could not create or open config file - {}", error))
    };
}

fn write_config_to_file(config_file: &cf::ConfigFile, file: File) -> Result<(), String> {
    let mut stream = BufWriter::new(file);
    let mut first = true;

    for config_group in &config_file.config_groups {
        if !first{
            stream.write(format!("\n\n").as_bytes());
        }
        stream.write(format!("[{}]\n",&config_group.group_name()).as_bytes());

        for config_attribute in &config_group.config_attributes() {
            stream.write(format!("{} = {}\n", config_attribute.name, config_attribute.value).as_bytes());
        }

        first = false;
    }
    
    stream.flush();
    Ok(())
}


//----------------------------------------------------------------------
#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::data::config_file as cf;
    use crate::data::config_group as cg;
    use crate::data::config_attribute as ca;
    use crate::file::write as write;
    use crate::file::read as read;

    #[test]
    fn write_and_read_file(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("test/write_test.txt"));
        cfile.add_config_group(format!("group1"));
        cfile.add_config_group(format!("group2"));
        cfile.config_groups[0].add_config_attribute(ca::ConfigAttribute::new(format!("name1"), format!("value1")).unwrap());
        cfile.config_groups[1].add_config_attribute(ca::ConfigAttribute::new(format!("name2"), format!("value2")).unwrap());

        write::write_config_file(&cfile);
        assert_eq!(
            cfile,
            read::read_config_file(format!("test/write_test.txt")).unwrap()
        )
    }

    #[test]
    fn overwrite_file(){
        let mut cfile: cf::ConfigFile = cf::ConfigFile::new(format!("test/overwrite_test.txt"));
        cfile.add_config_group(format!("group1"));
        cfile.add_config_group(format!("group2"));
        cfile.config_groups[0].add_config_attribute(ca::ConfigAttribute::new(format!("name1"), format!("value1")).unwrap());
        cfile.config_groups[1].add_config_attribute(ca::ConfigAttribute::new(format!("name2"), format!("value2")).unwrap());

        write::write_config_file(&cfile);
        let ofile = read::read_config_file(format!("test/overwrite_test.txt")).unwrap();

        cfile.config_groups[0].add_config_attribute(ca::ConfigAttribute::new(format!("name3"), format!("value3")).unwrap());
        write::write_config_file(&cfile);

        assert_eq!(
            assert_ne!(
                ofile,
                read::read_config_file(format!("test/overwrite_test.txt")).unwrap()
            ),
            assert_eq!(
                cfile,
                read::read_config_file(format!("test/overwrite_test.txt")).unwrap())
        )
    }
}