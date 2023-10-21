use std::{env, fs, fs::File, io::Write, path::Path};

fn main() -> Result<(), String> {
    let mut args = env::args_os();

    let file_target_path = match args.nth(1) {
        Some(file_target_path) => file_target_path,
        None => return Err("No target file specified!".to_string()),
    };

    let file_target_data = match fs::read(&file_target_path) {
        Ok(file_target_data) => file_target_data,
        Err(_) => return Err("Failed to read target file!".to_string()),
    };

    let mut out_file_path = Path::new(&file_target_path)
        .file_stem()
        .unwrap() // We know that it is not going to be a directory from reading it first.
        .to_os_string();

    out_file_path.push("_bytes.txt");

    let mut out_file = match File::create(out_file_path) {
        Ok(out_file) => out_file,
        Err(_) => return Err("Failed to create out file!".to_string()),
    };

    let mut byte_string_array_count: usize = 0;

    for byte in &file_target_data {
        let mut byte_string = format!("{:03} ", byte);
        byte_string_array_count += 1;

        if byte_string_array_count % 8 == 0 {
            byte_string.push(' ');
        }

        if byte_string_array_count == file_target_data.len() {
            byte_string = byte_string.trim_end().to_string();
        }

        if byte_string_array_count % 32 == 0 {
            byte_string = byte_string.trim_end().to_string();
            byte_string.push('\n');
        }

        match out_file.write_all(byte_string.as_bytes()) {
            Ok(_) => continue,
            Err(_) => return Err("Failed to write to out file!".to_string()),
        };
    }

    Ok(())
}