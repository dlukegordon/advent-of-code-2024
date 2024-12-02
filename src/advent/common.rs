#[macro_export]
macro_rules! get_data_filepath {
    () => {{
        let filepath = std::path::Path::new(file!());
        let filestem = filepath.file_stem().unwrap().to_string_lossy();
        std::path::PathBuf::from(format!("data/{}.txt", filestem))
    }};
}
