use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Your Program")
        .arg(Arg::with_name("program_name")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Name of the external program"))
        .arg(Arg::with_name("tsv_file")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("Path to TSV file"))
        .arg(Arg::with_name("num_columns")
            .required(true)
            .takes_value(true)
            .index(3)
            .help("Number of columns in TSV file"))
        .arg(Arg::with_name("columns_to_process")
            .takes_value(true)
            .index(4)
            .help("Columns to process (optional)"))
        .get_matches();

    let program_name = matches.value_of("program_name").unwrap();
    let tsv_file_path = matches.value_of("tsv_file").unwrap();
    let num_columns: usize = matches.value_of("num_columns").unwrap().parse()?;
    let columns_to_process = matches.value_of("columns_to_process").map(|s| {
        s.split(' ').map(|i| {
            let column_index = i.parse::<usize>()?;
            if column_index <= 0 || column_index > num_columns {
                return Err(format!("Invalid column index: {}", column_index).into());
            }
            Ok(column_index - 1)
        }).collect::<Result<Vec<_>, _>>()
    }).transpose()?; // Propagate the error if any

    let tsv_file = File::open(tsv_file_path)?;
    let tsv_reader = BufReader::new(tsv_file);

    for line in tsv_reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }

        let row: Vec<&str> = line.split('\t').collect();
        let processed_columns = match &columns_to_process {
            Some(cols) => cols.iter().map(|i| row[*i]).collect::<Vec<_>>().join(" "),
            None => row.join(" "),
        };

        // Call external program with processed columns
        // ...

        // Handle external program output
        // ...
    }

    Ok(())
}
