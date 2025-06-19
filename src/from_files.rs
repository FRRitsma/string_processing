
#[cfg(test)]

mod tests {
    use crate::string_filter_rolling_hash::StringSupervisor;
    use crate::test_utils::list_txt_files;
    use std::fs;



    #[test]
    fn debug_1() {
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

}
