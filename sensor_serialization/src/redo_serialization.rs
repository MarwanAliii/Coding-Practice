use std::fs::File;
use std::path::Path;
use std::error::Error;

use rkyv;
use crate::serialization::SensorReadings;

pub fn bin_to_csv() -> Result<(), Box<dyn Error>> {
    let bin_path = Path::new("./sensor_readings.bin");
    let csv_file = File::create("./output_sensor_readings.csv")?;

    let bin_data = std::fs::read(bin_path)?;

    // Find the root of the archived data. Basically find the data before it was serialized
    let archived: &rkyv::vec::ArchivedVec<crate::serialization::ArchivedSensorReadings> = unsafe{ rkyv::archived_root::<Vec<SensorReadings>>(&bin_data) };

    // Start CSV writer
    let mut wtr = csv::Writer::from_writer(csv_file);

    // Write header
    wtr.write_record(&[
        "timestamp",
        "q_w",
        "q_x",
        "q_y",
        "q_z",
        "x",
        "y",
        "z",
        "speed_x",
        "speed_y",
        "speed_z",
        "gyr_x",
        "gyr_y",
        "gyr_z",
    ])?;

    // Iterate over each reading and write it to the CSV
    for reading in archived.iter() {
        wtr.write_record(&[
            reading.timestamp.to_string(),
            reading.q_w.to_string(),
            reading.q_x.to_string(),
            reading.q_y.to_string(),
            reading.q_z.to_string(),
            reading.x.to_string(),
            reading.y.to_string(),
            reading.z.to_string(),
            reading.speed_x.to_string(),
            reading.speed_y.to_string(),
            reading.speed_z.to_string(),
            reading.gyr_x.to_string(),
            reading.gyr_y.to_string(),
            reading.gyr_z.to_string(),
        ])?;
    }

    // Flush the CSV writer to make sure all data is written
    wtr.flush()?;

    Ok(())
}
