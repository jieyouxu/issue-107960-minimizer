use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;


#[derive(Default, Clone)]
pub struct ConfigStruct {
}



pub fn initial_config() -> ConfigStruct {

    let mut config = ConfigStruct {
    };
    return config;
}
