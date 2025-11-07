use::std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
use chrono::Local;

fn log_message(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("[{}] - {}", timestamp, message);

    writeln!(file, "{}", log_entry).expect("Failed to write to log file");
}

pub fn process_input_files(branch_dirs: &[String]) -> String {
    OpenOptions::new().create(true).write(true).truncate(true).open("log.txt").expect("Failed to open log file");
    for dir in branch_dirs {
        let input_path = format!("{}/branch_weekly_sales.txt", dir);
        let input_file = File::open(&input_path);

        let file = match input_file {
            Ok(f) => f, 
            Err(e) => {
                eprintln!("Error opening {}: {}", input_path, e);
                return "Error".to_string();
            }
        };

        let reader = BufReader::new(file);
        let mut total_quantity = 0;
        let mut branch_code = String::new();
        let mut product_code = String::new();

        for line in reader.lines() {
            if let Ok(line_content) = line {
                let parts: Vec<&str> = line_content.split(',').map(|s| s.trim()).collect();
                if parts.len() == 4 {
                    branch_code = parts[0].to_string();
                    product_code = parts[1].to_string();
                    if let Ok(qty) = parts[2].parse::<i32>() {
                        total_quantity += qty;
                    }
                }
            }
        }

        let summary_line = format!("{}, {}, {}", branch_code, product_code, total_quantity);
        if let Err(e) = write_to_summary_file(&summary_line) {
            eprintln!("Error writing to summary file for summary {}: {}", branch_code, e);
            return "Error".to_string();
        }
        let log_entry = format!("Processed folder: {}", dir);
        log_message(&log_entry);
    }
    "OK".to_string()
}

pub fn init_summary_file() {
    let output_folder = "data/weekly_summary";
    if !std::path::Path::new(output_folder).exists() {
        fs::create_dir_all(output_folder).expect("Unable to create summary folder");
    }

    let output_path = format!("{}/weekly_sales_summary.txt", output_folder);
    fs::write(&output_path, "").expect("failed to clear log from previous executions.");
}

pub fn write_to_summary_file(line: &str) -> io::Result<()> {
    let output_folder = "data/weekly_summary";
    if !Path::new(output_folder).exists() {
        fs::create_dir_all(output_folder)?;
    }

    let output_path = format!("{}/weekly_sales_summary.txt", output_folder);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}
