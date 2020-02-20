use std::fs::File;
use std::io::Read;

use zip::ZipArchive;

pub fn read_zip_file(file_path: String) {
    let mut file = File::open(file_path).unwrap();
    let mut archive = ZipArchive::new(file).expect("failed to open file");
    for i in 0..archive.len()
    {
        let mut file = archive.by_index(i).unwrap();
        println!("Filename: {}", file.name());
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        println!("{:?}", buffer);
    }
}
