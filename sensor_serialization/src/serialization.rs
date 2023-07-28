use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use csv;
use rkyv::ser::{serializers::AllocSerializer, Serializer};

use rkyv_derive::{Archive, Serialize};

use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize, Archive)]
pub struct SensorReadings {
    pub timestamp: f64,
    pub q_w: f64,
    pub q_x: f64,
    pub q_y: f64,
    pub q_z: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub speed_x: f64,
    pub speed_y: f64,
    pub speed_z: f64,
    pub gyr_x: f64,
    pub gyr_y: f64,
    pub gyr_z: f64,
}

pub fn csv_to_bin() -> Result<(), Box<dyn Error>> {
    let csv_path = Path::new("./sensor_readings.csv");
    let mut binary_file = File::create("./sensor_readings.bin")?;

    let mut reader = csv::Reader::from_path(csv_path)?;

    // Create a vector to hold all the SensorReadings records
    let mut all_records: Vec<SensorReadings> = Vec::new();

    // Deserialize each record into a SensorReadings and push it to the vector
    for result in reader.deserialize() {
        let record: SensorReadings = result?;
        all_records.push(record);
    }

    // Serialize the whole vector
    let mut serializer = AllocSerializer::<4096>::default();
    serializer.serialize_value(&all_records)?;
    let bytes = serializer.into_serializer().into_inner();

    // Write all the serialized data to the file at once
    binary_file.write_all(&bytes)?;

    Ok(())
}