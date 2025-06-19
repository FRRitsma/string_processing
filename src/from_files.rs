fn filter_files() {}

#[cfg(test)]

mod tests {
    use crate::filter;
    use crate::remake::clean_vector_of_strings;
    use crate::string_filter_rolling_hash::StringSupervisor;
    use crate::test_utils::list_txt_files;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn debug() {
        use std::fs::{self, File};
        use std::io::Write;

        let wiki_files_dir = "src/wiki_files/";
        let txt_files = list_txt_files(wiki_files_dir).unwrap();
        let mut str_vec = vec![];

        // Read batch of files into strings
        for single_txt_file in txt_files.iter() {
            let path = format!("{}{}", wiki_files_dir, single_txt_file);
            let content = fs::read_to_string(path).unwrap();
            str_vec.push(content);
        }

        let clean_files = clean_vector_of_strings(str_vec, 100);
        println!("{:?}", clean_files);
    }

    #[test]
    fn debug_1() {
        use std::fs::{self, File};
        use std::io::Write;

        let wiki_files_dir = "src/wiki_files/";
        let txt_files = list_txt_files(wiki_files_dir).unwrap();

        // Read batch of files into strings
        let mut supervisor_vec: Vec<StringSupervisor> = vec![];

        for single_txt_file in txt_files.iter() {
            let path = format!("{}{}", wiki_files_dir, single_txt_file);
            let content = fs::read_to_string(path).unwrap();
            supervisor_vec.push(StringSupervisor::from_string(content, 50));
        }

        for i in 0..supervisor_vec.len() {
            for j in (i + 1)..supervisor_vec.len() {
                if i == j {
                    continue;
                }
                unsafe {
                    let visor1 = &mut *supervisor_vec.as_mut_ptr().add(i);
                    let visor2 = &mut *supervisor_vec.as_mut_ptr().add(j);
                    visor1.compare(visor2);
                }
            }
        }
    }

    #[test]
    fn list_all_files_in_path_in_batches() {
        use std::fs::{self, File};
        use std::io::Write;

        let wiki_files_dir = "src/wiki_files/";
        let txt_files = list_txt_files(wiki_files_dir).unwrap();
        let batch_size = 10;

        let target_dir = "src/processed_wiki_files/";
        for batch in txt_files.chunks(batch_size) {
            let mut str_vec = vec![];

            // Read batch of files into strings
            for single_txt_file in batch.iter() {
                let path = format!("{}{}", wiki_files_dir, single_txt_file);
                let content = fs::read_to_string(path).unwrap();
                str_vec.push(content);
            }

            // Filter batch
            let filtered_strings = filter::remove_substrings(str_vec.clone(), 200);

            // Save results for this batch
            for (name, content) in batch.iter().zip(filtered_strings) {
                let out_path = format!("{}processed_{}", target_dir, name);
                let mut file = File::create(out_path).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }

            println!("Processed batch: {:?}", batch);
        }
    }
}
