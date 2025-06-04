use std::fs;

pub const LOCATION_OF_TEST_FILES: &'static str = "src/examples/";

pub fn list_txt_files(dir: &str) -> std::io::Result<Vec<String>> {
    let mut txt_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "txt" {
                    if let Some(filename) = path.file_name() {
                        txt_files.push(filename.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    Ok(txt_files)
}

pub fn load_txt_files_as_vector_of_str() -> Vec<String> {
    let txt_files = list_txt_files(LOCATION_OF_TEST_FILES).unwrap();
    let mut str_vec = vec![];
    for single_txt_file in txt_files.iter() {
        let content =
            fs::read_to_string(LOCATION_OF_TEST_FILES.to_owned() + single_txt_file).unwrap();
        str_vec.push(content);
    }
    str_vec
}
