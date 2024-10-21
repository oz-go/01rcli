use csv::Reader;
use serde::{Deserialize,Serialize};
use std::fs;
use anyhow;
use serde_json::Value;

use crate::opts::OutputFormat;


#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player{
    name:String,
    position: String,
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,
    #[serde(rename="Kit Number")] 
    kit:u8,
}


pub fn process_csv(input:&str,output:String,format:OutputFormat) -> anyhow::Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records(){
        let record = result?;

        // let iter = headers.iter().zip(record.iter());
        let json_value = headers.iter().zip(record.iter()).collect::<serde_json::Value>();
        ret.push(json_value);
        
    }
    let  content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output,content)?;
    Ok(()) 
}