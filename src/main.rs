use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} [program_name] [tsv_file] [num_columns] [columns_to_process (optional)]", args[0]);
        std::process::exit(1);
    }

    // Set variables
    let program_name = &args[1];      // Name of the external program
    let tsv_file_path = &args[2];     // Path to TSV file
    let num_columns: usize = args[3].parse()?;    // Number of columns in TSV file
    let columns_to_process = args.get(4).map(|s| {
        let indices: Result<Vec<usize>, _> = s.split(' ').map(|i| {
            let column_index = i.parse::<usize>()?;
            if column_index <= 0 || column_index > num_columns {
                return Err(format!("Invalid column index: {}", column_index).into());
            }
            Ok(column_index - 1)
        }).collect();
        indices
    }).transpose()?; // Propagate the error if any

    // Open log file
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();  // Timestamp for log file name
    let log_file_path = format!("{}.{}.log", tsv_file_path.trim_end_matches(".tsv"), timestamp);
    let mut log_file = File::create(log_file_path)?;

    // Write arguments to log file
    writeln!(log_file, "Arguments: {} {} {} {:?}", program_name, tsv_file_path, num_columns, columns_to_process)?;

    // Start timer
    let start_time = std::time::Instant::now();

    // Loop through TSV file and pass columns to program
    let tsv_file = File::open(tsv_file_path)?;
    let tsv_reader = BufReader::new(tsv_file);
    for line in tsv_reader.lines() {
        let line = line?;
        if line.starts_with('#') {   // Skip rows starting with '#'
            continue;
        }

        let row: Vec<&str> = line.split('\t').collect();
        let processed_columns = match &columns_to_process {
            Some(cols) => cols.iter().map(|i| row[*i]).collect::<Vec<_>>().join(" "),
            None => row.join(" "),
        };
        let output = Command::new(program_name).arg(processed_columns).output()?;
        if !output.status.success() {
            eprintln!("External program failed with exit code {:?}", output.status.code());
            std::process::exit(output.status.code().unwrap_or(1));
        }
    }

    // End timer and write duration to log file
    let duration = start_time.elapsed().as_secs();
    writeln!(log_file, "Start time: {} | End time: {} | Duration: {} seconds", start_time, chrono::Local::now(), duration)?;

    Ok(())
}
