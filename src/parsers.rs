use crate::errors::Error;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
pub fn create_config_file(file_type: String) -> Result<(), String> {
    if file_type != "json" && file_type != "toml" {
       return  Err("config file can only be json or toml".to_string())
    }
    let file_name = format!("rodeo.{file_type}");
    let mut file = File::create(file_name).unwrap();

    if file_type == "json"{
        file.write_all(br#"
        {
        "services" :[
        {"path":"", "base_url" :""}
        ]
        }
        "#).unwrap();
    }

    if file_type == "json"{
        file.write_all(br#"
    port = 5000
timeout = '10s'

[services]
admin = {version = "1", base_url = "http://0.0.0.0:5003"}
auth = {version = "1", base_url = "http://0.0.0.0:5001/"}
student = {version = "1", base_url = "http://0.0.0.0:5002"}

        "#).unwrap();
    }

    // file.write_all(b"Hello, world!")?;
    Ok(())
}
