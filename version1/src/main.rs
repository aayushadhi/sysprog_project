use std:: {
    fs, path::Path, time::Instant,
};

mod lib_mod;

fn main() {
    lib_mod::init_summary_file();
    let data_path = "data";
    let mut branch_dirs: Vec<String> = fs::read_dir(data_path)
        .expect("Cannot read data folder")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() && !path.ends_with("weekly_summary") {
                Some(path.display().to_string())
            } else {
                None
            }
        })
        .collect();

    branch_dirs.sort();

    println!("Found {} branch folders.", branch_dirs.len());

    let output_folder = format!("{}/weekly_summary", data_path);
    if !Path::new(&output_folder).exists() {
        fs::create_dir_all(&output_folder).expect("Unable to create a summary folder.");
    }

    let start_time = Instant::now();

    let result = lib_mod::process_input_files(&branch_dirs);

    println!("File processing result: {}", result);

    let duration = start_time.elapsed();
    println!("Time taken: {:.2?}", duration);
    println!("Phew! I am done.");
}

