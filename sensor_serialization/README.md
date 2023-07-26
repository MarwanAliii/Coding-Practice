This project must:

1. create a struct SensorReadings, which contains all the columns of the csv ("acc_x", "gyro_x", ...)
2. read one csv line, parse the numbers and create the SensorReadings based on parsed numbers
3. use rkyv to serialize the SensorReadings struct to binary
4. append the binary to a file
5. repeat step 2-4 until the end of csv


Progress:

Very helpful link for CSV and Serde: https://docs.rs/csv/latest/csv/tutorial/index.html
Serde Link: https://serde.rs/
rkyv link: https://docs.rs/rkyv/latest/rkyv/trait.Archive.html
