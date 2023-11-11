use std::{
    env,
    path::{Path, PathBuf},
};

pub fn create_path(file_name: &str) -> PathBuf {
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| ".".to_string());
    let path = Path::new(&data_dir).join(file_name);
    path.to_path_buf()
}

pub fn create_file(file_name: &str, content: &str) {
    let path = create_path(file_name);
    if !path.exists() {
        std::fs::write(path, content).expect("Unable to create file");
    }
}

pub fn read_count(file_name: &str) -> Result<i32, std::num::ParseIntError> {
    let path = create_path(file_name);
    let contents = std::fs::read_to_string(path).expect("Unable to read file");
    let count = contents.parse::<i32>()?;
    Ok(count)
}

pub fn write_count(file_name: &str, count: i32) {
    let path = create_path(file_name);
    std::fs::write(path, count.to_string()).expect("Unable to write file");
}
