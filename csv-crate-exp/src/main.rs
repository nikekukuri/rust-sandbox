use std::{error::Error, io, process};
use std::fs::File;
use std::any::type_name;

fn example() -> Result<(), Box<dyn Error>> {
    let file_path = "data/example.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // ヘッダー行を取得
    let headers = rdr.headers()?.clone();
    let header_vec: Vec<String> = headers.iter().map(|s| s.to_string()).collect();

    // 各列のインデックスを取得
    let mut header_idx_vec = Vec::new();
    for header in headers.iter() {
        let h_idx = header_vec.iter().position(|h| h == header).unwrap();
        header_idx_vec.push(h_idx);
    }
    let last_idx = header_idx_vec.len() - 1;

    let mut param: Vec<Vec<String>> = Vec::new();
    //let mut output: Vec<String> = Vec::new(); 

    for result in rdr.records() {
        let record = result?;
        let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        // paramにcity, region, countryの値を格納
        let import_col = [String::from("city"), String::from("region")];
        let mut record_tmp = Vec::new();
        for h_idx in header_idx_vec.iter() {
            let value = fields[*h_idx].clone();
            println!("{:?}", &value);
            if import_col.contains(&value) {
                record_tmp.push(value);
            }
        }
        param.push(record_tmp);

        // outputにpopulationの値を格納
        //let population = fields[3].clone();
        //output.push(population);
    }

    // 結果を表示
    println!("Param: {:?}", param);
    //println!("Output: {:?}", output);

    Ok(())
}

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
