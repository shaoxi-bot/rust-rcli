use core::fmt;
use std::fs;

use csv::Reader;
use serde::{Serialize, Deserialize};
use serde_json::{self};

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 重命名规则
pub struct Player {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    // #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()>{
    let mut reader = Reader::from_path(input)?;
    // let redcords = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();
    let mut redcords = Vec::with_capacity(100);
    let headers = reader.headers()?.clone();
    for row in reader.records(){
        let record= row?;
        // let iter = headers.iter().zip(record.iter());
        // let value = match format {
        //     OutputFormat::Json => iter.collect::<serde_json::Value>(),
        //     OutputFormat::Yaml => iter.collect::<serde_yaml::Value>(),
        //     // OutputFormat::Json => iter.collect::<toml::Value>(),
        // };
        // 使用zip. 拼接两个迭代器
        let value = headers.iter().zip(record.iter()).collect::<serde_json::Value>();
        redcords.push(value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&redcords)?,
        OutputFormat::Yaml => serde_yaml::to_string(&redcords)?,
        // OutputFormat::Toml => toml::to_string(&redcords)?,
    };
    fs::write(output, content)?;
    Ok(())
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{}", Into::<&str>::into(*self))
    }
}