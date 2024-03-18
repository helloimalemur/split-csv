use std::env::args;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use csv::{Error, ErrorKind, StringRecord};

fn read_csv(path: String) -> Result<(StringRecord, Vec<StringRecord>), Error> {
    // Build the CSV reader and iterate over each record.
    let file = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut rows: Vec<StringRecord> = vec![];
    let headers = rdr.headers().unwrap().clone();

    for result in rdr.records() {
        match result {
            Ok(res) => {
                let r = res.clone();
                // println!("{:?}", r);
                rows.push(r);
            }
            Err(_) => {}
        }
    }



    Ok((headers, rows))
}

pub fn div_up(a: usize, b: usize) -> usize {
    (a + (b - 1)) / b
}

fn main() {
    let args: Vec<String> = args().collect();
    let csv_path = args.get(1).unwrap();
    let limit = args.get(3).unwrap().parse::<usize>().unwrap();

    let mut out_file_num = 0;
    let out = args.get(2).unwrap();


    if let Ok(mut rows) = read_csv(csv_path.to_string()) {

        while rows.1.len() > 0 {
            let mut count = 0;

            let files = div_up(rows.1.len(), limit);
            println!("total files: {}", files);

            for file in 0..files {
                println!("Starting file: {}", file);
                let out_file_name = format!("./{}{}{}", out, out_file_num, ".csv");
                let mut out_file = File::create(out_file_name).unwrap();
                let mut wtr = csv::Writer::from_writer(out_file);

                wtr.write_record(rows.0.as_byte_record()).unwrap();

                while count < limit {

                    println!("Adding row: {}", count);
                    match rows.1.pop() {
                        None => {break}
                        Some(e) => {
                            wtr.write_record(e.as_byte_record()).unwrap();
                        }
                    }
                    count += 1;
                }
                out_file_num += 1;
                count = 0;
            }

        }
    }
}
