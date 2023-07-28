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



Redoing Serialization:
Redoing the Serialization was surprisingly tough. Here is a nice description or 'Archived' data from rkyv that had me puzzled throughout the project:

When you serialized your data into binary using rkyv, it was transformed into an Archived version of your original data type, in this case, Vec<SensorReadings> became ArchivedVec<ArchivedSensorReadings>.

Now, you might be wondering, "what's an Archived version?"

The Archived version of a type is a special version designed for efficient serialization and deserialization. The key idea here is that it represents your data in a form that can be directly written to or read from disk, without needing to perform any conversion operations. It's basically the same data, but in a format that is easy to save and load.

When you use rkyv::archived_root to get the root of your archived data, it returns an ArchivedVec<ArchivedSensorReadings>. This is not the same as your original Vec<SensorReadings>, but it's structured in a very similar way and can be interacted with almost like the original Vec.

The beauty of rkyv is that it provides methods on Archived types that mimic the methods on the original types. So you can use an ArchivedVec<ArchivedSensorReadings> as if it were a normal Vec<SensorReadings>. For example, you can call .iter() on it and iterate over the items in the Vec, as you are doing in your bin_to_csv function.

So yes, in essence, the Archived version is a more disk-friendly representation of the data it once was before serialization, and rkyv allows you to interact with this Archived data as if it were the original data, making the serialization/deserialization process seamless and efficient.
