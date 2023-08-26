use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::{model::Data, rotex::RotexData};

pub fn cmd() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new("rotex-data.json");
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let rotex_data: RotexData = serde_json::from_reader(reader)?;

    let data: Data = rotex_data.into();

    let devices_output_path = Path::new("data/devices.toml");
    fs::write(devices_output_path, toml::to_string(&data.devices)?)?;

    let parameters_output_path = Path::new("data/parameters.toml");
    fs::write(parameters_output_path, toml::to_string(&data.parameters)?)?;

    Ok(())
}
