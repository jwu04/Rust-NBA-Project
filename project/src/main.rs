use std::error::Error;
use std::fs;
use csv;
fn main() {
    let paths = fs::read_dir("../data").unwrap();

    for path in paths {
        let file = path.unwrap().path().display().to_string();
        
        if let Err(e) = read_file(&file) {
            eprintln!("{}", e);
        } 
    }
}

fn read_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;

    let hdr = rdr.headers()?;
    println!("{:?}", hdr);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

// fn read_file() -> String {
//     let mut file = File::open("../../data/teams.csv").expect("Could not open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Could not read file");
//     return contents;
// }
