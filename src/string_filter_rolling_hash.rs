use cyclic_poly_23::CyclicPoly64;
use rayon::prelude::*;
use std::collections::HashSet;
use std::ops::Range;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct StringSupervisor {
    base_string: String,
    window_size: usize,
    byte_offsets: Vec<usize>,
    hash_vec: Vec<u64>,
    complete_hashset: HashSet<u64>,
    duplicate_hashset: HashSet<u64>,
}

impl StringSupervisor {
    // Constructor:
    pub fn from_string(base_string: String, window_size: usize) -> Self {
        if window_size > base_string.len() {
            return StringSupervisor {
                base_string,
                window_size,
                byte_offsets: vec![],
                hash_vec: vec![],
                complete_hashset: HashSet::new(),
                duplicate_hashset: HashSet::new(),
            };
        }

        // Initialize vectors:
        let len = base_string.len();
        let index: Vec<(usize, char)> = base_string.char_indices().collect();
        let mut byte_offsets = Vec::with_capacity(len);
        let mut chars = Vec::with_capacity(len);

        for (o, c) in index.into_iter() {
            byte_offsets.push(o);
            chars.push(c);
        }
        let mut bytes = Vec::with_capacity(len);
        bytes.extend(chars.drain(..).map(|c| c as u8));

        // Perform hashing:
        let (hash_vec, complete_hash_set) = get_hash_vec_and_hash_set(bytes, window_size);

        StringSupervisor {
            base_string,
            window_size,
            byte_offsets,
            hash_vec,
            complete_hashset: complete_hash_set,
            duplicate_hashset: HashSet::new(),
        }
    }

    fn reducable(&self) -> bool {
        self.window_size < self.base_string.len()
    }

    pub fn compare(&mut self, other: &mut StringSupervisor) {
        if self.reducable() && other.reducable() {
            for item in self.complete_hashset.intersection(&other.complete_hashset) {
                self.duplicate_hashset.insert(*item);
                other.duplicate_hashset.insert(*item);
            }
        }
    }

    pub fn is_duplicate_mask(&self, filter_hashset: &HashSet<u64>) -> Vec<bool> {
        // Returns a mask that is true where hash_vec has duplicated string windows
        let is_duplicate = self
            .hash_vec
            .iter()
            .map(|x| filter_hashset.contains(&x))
            .collect();
        is_duplicate
    }

    pub fn filter_range(&self, filter_hashset: &HashSet<u64>) -> Vec<Range<usize>> {
        let mut filter_range_vec: Vec<Range<usize>> = vec![];

        let mut duplicate_mask = self.is_duplicate_mask(filter_hashset);
        duplicate_mask.push(false);

        let mut range_start = 0;
        let mut range_end = self.window_size;
        for (index, window) in duplicate_mask.windows(2).enumerate() {
            if let [i0, i1] = window {
                match (*i0, *i1) {
                    // Continuation of window:
                    (true, true) => {
                        range_end += 1;
                    }
                    // End of window:
                    (true, false) => filter_range_vec.push(range_start..range_end),
                    // Start of window:
                    (false, true) => {
                        range_start = index;
                        range_end = index + self.window_size;
                    }
                    (false, false) => {}
                }
            }
        }
        filter_range_vec
    }

    pub fn filter_string(&mut self) -> String {
        if self.reducable() {
            for range in self.filter_range(&self.duplicate_hashset).into_iter().rev() {
                // Convert char-index range to byte-index range using byte_offsets
                let byte_start = self.byte_offsets[range.start];
                let byte_end = self.byte_offsets[range.end];
                self.base_string.drain(byte_start..byte_end);
            }
        }
        self.base_string.clone()
    }

    pub fn filter_string_from_hashset(&mut self, filter_hashset: &HashSet<u64>) -> String {
        if self.reducable() {
            for range in self.filter_range(filter_hashset).into_iter().rev() {
                // Convert char-index range to byte-index range using byte_offsets
                let byte_start = self.byte_offsets[range.start];
                let byte_end = self.byte_offsets[range.end];
                self.base_string.drain(byte_start..byte_end);
            }
        }
        self.base_string.clone() // This clone shouldn't be necessary, but somehow it is
    }
}

fn get_hash_vec_and_hash_set(bytes: Vec<u8>, window_size: usize) -> (Vec<u64>, HashSet<u64>) {
    let mut hash_vec: Vec<u64> = Vec::with_capacity(bytes.len().saturating_sub(window_size) + 1);
    let mut hash_set: HashSet<u64> = HashSet::new();
    let mut hasher = CyclicPoly64::from_block(&bytes[0..window_size]);

    hash_set.insert(hasher.value());
    hash_vec.push(hasher.value());
    for remove in 0..bytes.len().saturating_sub(window_size) {
        let add = remove + window_size;
        hasher.rotate(bytes[remove], bytes[add]);
        hash_set.insert(hasher.value());
        hash_vec.push(hasher.value());
    }
    (hash_vec, hash_set)
}

fn track_first_and_second_occurrence_of_substring(
    first_set: &mut HashSet<u64>,
    second_set: &mut HashSet<u64>,
    sv: &StringSupervisor,
) {
    for &item in &sv.complete_hashset {
        if first_set.contains(&item) {
            second_set.insert(item);
        } else {
            first_set.insert(item);
        }
    }
}

fn get_all_second_occurrences_of_substrings(
    supervisor_vector: &Vec<StringSupervisor>,
) -> HashSet<u64> {
    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();

    for supervisor in supervisor_vector {
        track_first_and_second_occurrence_of_substring(&mut first_set, &mut second_set, supervisor);
    }

    second_set
}

pub(crate) fn clean_list_of_strings_parallel(
    strings: Vec<String>,
    minimum_size: usize,
) -> Vec<String> {
    // Wrap each StringSupervisor in a Mutex
    let supervisor_vec: Vec<Mutex<StringSupervisor>> = strings
        .into_par_iter()
        .map(|s| Mutex::new(StringSupervisor::from_string(s, minimum_size)))
        .collect();

    // Parallel comparison
    (0..supervisor_vec.len()).into_par_iter().for_each(|i| {
        for j in (i + 1)..supervisor_vec.len() {
            let mut visor1 = supervisor_vec[i].lock().unwrap();
            let mut visor2 = supervisor_vec[j].lock().unwrap();
            visor1.compare(&mut visor2);
        }
    });

    // Extract the results
    supervisor_vec
        .into_par_iter()
        .map(|m| m.into_inner().unwrap().filter_string())
        .collect()
}

fn clean_list_of_strings_single_pass(strings: Vec<String>, minimum_size: usize) -> Vec<String> {
    let supervisor_vec: Vec<StringSupervisor> = strings
        .into_iter()
        .map(|s| StringSupervisor::from_string(s, minimum_size))
        .collect();
    let filter_hashset = get_all_second_occurrences_of_substrings(&supervisor_vec);
    supervisor_vec
        .into_iter()
        .map(|mut m| m.filter_string_from_hashset(&filter_hashset))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::string_filter_rolling_hash::StringSupervisor;
    use crate::string_filter_rolling_hash::clean_list_of_strings_single_pass;
    use crate::string_filter_rolling_hash::track_first_and_second_occurrence_of_substring;
    use crate::test_utils::list_txt_files;
    use std::collections::HashSet;

    #[test]
    fn counting_occurrences_first_only() {
        let string_supervisor = StringSupervisor::from_string("hell".to_string(), 3);
        let mut first_set: HashSet<u64> = HashSet::new();
        let mut second_set: HashSet<u64> = HashSet::new();

        track_first_and_second_occurrence_of_substring(
            &mut first_set,
            &mut second_set,
            &string_supervisor,
        );
        assert_eq!(first_set, string_supervisor.complete_hashset);
        assert_ne!(second_set, first_set);
    }

    #[test]
    fn debug2() {
        let string_supervisor = StringSupervisor::from_string("hell".to_string(), 3);
        assert_eq!(string_supervisor.hash_vec.len(), 2);
        println!("{:?}", string_supervisor);
    }

    #[test]
    fn debug3() {
        let mut string_supervisor_1 = StringSupervisor::from_string("hel".to_string(), 3);
        let mut string_supervisor_2 = StringSupervisor::from_string("hel000".to_string(), 3);

        string_supervisor_1.compare(&mut string_supervisor_2);

        println!("{:?}", string_supervisor_1.filter_string());
    }

    // #[test]
    // fn clean_large_set_of_files() {
    //     use std::fs::{self, File};
    //
    //     let wiki_files_dir = "src/wiki_files/";
    //     let txt_files = list_txt_files(wiki_files_dir).unwrap();
    //
    //     // Read batch of files into strings
    //     let mut strings: Vec<String> = vec![];
    //     for single_txt_file in txt_files.iter() {
    //         let path = format!("{}{}", wiki_files_dir, single_txt_file);
    //         let content = fs::read_to_string(path).unwrap();
    //         strings.push(content);
    //     }
    //
    //     let clean_strings = clean_list_of_strings_parallel(strings, 50);
    //     println!("{:?}", clean_strings);
    // }

    #[test]
    fn clean_large_set_of_files_single_pass() {
        use std::fs::{self, File};

        let wiki_files_dir = "src/wiki_files/";
        let txt_files = list_txt_files(wiki_files_dir).unwrap();

        // Read batch of files into strings
        let mut strings: Vec<String> = vec![];
        for single_txt_file in txt_files.iter() {
            let path = format!("{}{}", wiki_files_dir, single_txt_file);
            let content = fs::read_to_string(path).unwrap();
            strings.push(content);
        }

        let clean_strings = clean_list_of_strings_single_pass(strings, 50);

        // // Read batch of files into strings
        // let mut strings: Vec<String> = vec![];
        // for single_txt_file in txt_files.iter() {
        //     let path = format!("{}{}", wiki_files_dir, single_txt_file);
        //     let content = fs::read_to_string(path).unwrap();
        //     strings.push(content);
        // }
        //
        // let clean_strings_old = clean_list_of_strings_parallel(strings, 50);
        // assert_eq!(clean_strings_old, clean_strings);
        //
        // println!("{:?}", clean_strings);
    }
}
