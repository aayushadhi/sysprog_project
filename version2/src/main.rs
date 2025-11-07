use std::{
    fs::{self}, sync::mpsc, thread, time::Instant
};

mod lib_mod;

fn main() {
    let data_path = "data";
    lib_mod::init_summary_file(); 
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

    let chunk_size = 10;
    let groups: Vec<Vec<String>> = branch_dirs
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let (tx, rx) = mpsc::channel::<String>();
    let start_time = Instant::now();
    let mut handles = vec![];

    for group in groups {
        let thread_tx = tx.clone();
        let handle = thread::spawn(move || {
            lib_mod::process_input_file(group, thread_tx)
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        if let Ok(status) = handle.join() {
            println!("Thread finished with status: {}", status);
        }
    }
    for received in rx {
        //println!("{}", received);
        lib_mod::write_to_summary_file(&received);
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:.2?}", duration);
    println!("Phew! I am done.");
}
