mod serialization;
mod redo_serialization;

fn main(){
    serialization::csv_to_bin().unwrap();
    redo_serialization::bin_to_csv().unwrap();
}