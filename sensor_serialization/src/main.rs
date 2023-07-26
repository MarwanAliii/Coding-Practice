use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    // Open the csv file
    let mut csv_file = File::open(&csv_path)?;

    // Create a vector to hold all the serialized records
    let mut all_bytes: Vec<u8> = Vec::new();

    let mut contents = String::new();
    csv_file.read_to_string(&mut contents)?;

    // Skip the header line
    let lines: Vec<&str> = contents.lines().skip(1).collect();

    // Iterate over each line in the CSV
    for line in lines {
        // Split the line into fields
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 14 {
            return Err("Incorrect number of fields in CSV line".into());
        }

        // Parse each field and create a SensorReadings struct
        let record = SensorReadings {
            timestamp: fields[0].parse()?,
            q_w: fields[1].parse()?,
            q_x: fields[2].parse()?,
            q_y: fields[3].parse()?,
            q_z: fields[4].parse()?,
            x: fields[5].parse()?,
            y: fields[6].parse()?,
            z: fields[7].parse()?,
            speed_x: fields[8].parse()?,
            speed_y: fields[9].parse()?,
            speed_z: fields[10].parse()?,
            gyr_x: fields[11].parse()?,
            gyr_y: fields[12].parse()?,
            gyr_z: fields[13].parse()?,
        };

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

