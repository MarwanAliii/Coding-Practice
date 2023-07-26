use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use csv;
use rkyv::ser::{serializers::AllocSerializer, Serializer};

use rkyv_derive::{Archive, Serialize};

use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize, Archive)]
struct SensorReadings {
    timestamp: f64,
    q_w: f64,
    q_x: f64,
    q_y: f64,
    q_z: f64,
    x: f64,
    y: f64,
    z: f64,
    speed_x: f64,
    speed_y: f64,
    speed_z: f64,
    gyr_x: f64,
    gyr_y: f64,
    gyr_z: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define input csv file and output binary file
    let csv_path = Path::new("./sensor_readings.csv");
    let mut binary_file = File::create("./sensor_readings.bin")?;

    // Create a CSV parser
    let mut reader = csv::Reader::from_path(csv_path)?;

    // Create a vector to hold all the serialized records
    let mut all_bytes: Vec<u8> = Vec::new();

    // Iterate over each record in the CSV
    for result in reader.deserialize() {
        // Deserialize the record into SensorReadings struct
        let record: SensorReadings = result?;

        // Serialize the struct to binary using rkyv
        let mut serializer = AllocSerializer::<4096>::default();
        serializer.serialize_value(&record)?;
        let bytes = serializer.into_serializer().into_inner();

        // Add the serialized bytes to the vector
        all_bytes.extend_from_slice(&bytes);
    }

    // Write all the serialized data to the file at once
    binary_file.write_all(&all_bytes)?;

    Ok(())
}
