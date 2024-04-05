

use serde::{Deserialize, Serialize};
use toml::Table;
use std::fs::File;
use std::io::prelude::*;

pub fn create_config_file(file_type: String) -> Result<(), String> {
    if file_type != "json" && file_type != "toml" {
       return  Err("config file can only be json or toml".to_string())
    }
    let file_name = format!("rodeo.{file_type}");
    let mut file = File::create(file_name).unwrap();

    if file_type == "json"{
        file.write_all(br#"{
  "port": "5000",
  "time_out": "10s",
  "services": [
    {
      "base_url": "0.0.0.0:5001/v1",
      "path" : "/auth"
    },
    {
      "base_url": "0.0.0.0:5002/admin",
      "path" : "/admin"
    }
  ]
}
        "#).unwrap();
    }

    if file_type == "toml"{
        file.write_all(br#"port = 5000
timeout = '10s'

[services]
auth = {path = "auth", base_url = "http://0.0.0.0:5001/v1"}
admin = {path = "admin", base_url = "http://0.0.0.0:5002/v2"}
        "#).unwrap();
    }

    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    name: String,
    version: String,
    pub base_url: String,
}

impl Service {
    pub fn new(name: &str, base_url: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            base_url: base_url.to_string(),
            version: version.to_string(),
        }
    }
}
pub fn parse_config(service_id: &str) -> Result<Service, String> {
    // read the service configuration, or throw error
    // let Some(config) = fs::read_to_string("./proxy.toml").ok() else {
    //     return Err(String::from("error reading config"));
    // };
    let config = std::include_bytes!("../proxy.toml");
    let config = String::from_utf8_lossy(config);
    //  convert the file into Table destructure provided by Toml parser
    let config = config.parse::<Table>().unwrap();
    let Some(service) = &config["services"].get(service_id) else {
        return Err(String::from("error parsing config"));
    };

    // convert to Services Struct
    let name = service_id;
    let version = format!("v{}", service.get("version").unwrap().as_str().unwrap());
    let base_url = service.get("base_url").unwrap().as_str().unwrap();

    Ok(Service::new(name, base_url, &version))
}