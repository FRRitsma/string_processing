use cyclic_poly_23::CyclicPoly64;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct StringSupervisor {
    base_string: String,
    window_size: usize,
    hash_vec: Vec<u64>,
    complete_hashset: HashSet<u64>,
    character_count: usize,
}

#[derive(Debug)]
struct DeleteRange{
    start: usize,
    end: usize,
}


impl DeleteRange{
    fn new(start: usize, end: usize) -> Self{
        DeleteRange{start, end}
    }
}

impl StringSupervisor {
    // Constructor:
    fn from_string(base_string: String, window_size: usize) -> Self {

        let character_count = base_string.chars().count();

        if window_size > character_count {
            return StringSupervisor {
                base_string,
                window_size,
                hash_vec: vec![],
                complete_hashset: HashSet::new(),
                character_count,
            };
        }

        // Initialize vectors:
        let index: Vec<(usize, char)> = base_string.char_indices().collect();
        let mut chars = Vec::with_capacity(character_count);

        for (_, c) in index.into_iter() {
            chars.push(c);
        }

        let mut bytes = Vec::with_capacity(character_count);
        bytes.extend(chars.drain(..).map(|c| c as u8));

        // Perform hashing:
        let (hash_vec, complete_hash_set) = get_hash_vec_and_hash_set(bytes, window_size);

        StringSupervisor {
            base_string,
            window_size,
            hash_vec,
            complete_hashset: complete_hash_set,
            character_count,
        }
    }

    fn reducible(&self) -> bool {
        self.window_size < self.base_string.len()
    }

    fn is_duplicate_mask(&self, filter_hashset: &HashSet<u64>) -> Vec<bool> {
        // Returns a mask that is true where hash_vec has duplicated string windows
        let is_duplicate = self
            .hash_vec
            .iter()
            .map(|x| filter_hashset.contains(&x))
            .collect();
        is_duplicate
    }

    fn filter_range(&self, filter_hashset: &HashSet<u64>) -> Vec<DeleteRange> {
        let mut filter_range_vec: Vec<DeleteRange> = vec![];

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
                    (true, false) => filter_range_vec.push(DeleteRange::new(range_start, range_end)),
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

    // fn filter_mask(&self, filter_hashset: &HashSet<u64>) -> Vec<bool>{
    //     let mask: Vec<bool> = Vec::reserve_exact(self.base_string.chars().count());
    //     for (index, hash) in self.hash_vec.iter().enumerate(){
    //
    //     }
    //     todo!()
    // }


    fn filter_string_from_hashset(&mut self, filter_hashset: &HashSet<u64>) {
        if self.reducible() {
            let mut byte_offsets: Vec<usize> =
                self.base_string.char_indices().map(|(i, _)| i).collect();
            byte_offsets.push(self.base_string.len());

            let clear_ranges: Vec<DeleteRange> = self.filter_range(filter_hashset).into_iter().rev().collect();

            for range in clear_ranges {
                let byte_start = byte_offsets[range.start];
                let byte_end = byte_offsets
                    .get(range.end)
                    .copied()
                    .unwrap_or(self.base_string.len());

                self.base_string.drain(byte_start..byte_end);
                byte_offsets.drain(range.start..range.end);
            }
        }
    }
}

fn get_hash_vec_and_hash_set(bytes: Vec<u8>, window_size: usize) -> (Vec<u64>, HashSet<u64>) {
    let amount_of_hashes = bytes.len().saturating_sub(window_size) + 1;

    // Initialize vector:
    let mut hash_vec: Vec<u64> = Vec::new();
    hash_vec.reserve_exact(amount_of_hashes);

    // Initialize hashset:
    let mut hash_set: HashSet<u64> = HashSet::new();
    hash_set.reserve(amount_of_hashes);

    let mut hasher = CyclicPoly64::from_block(&bytes[0..window_size]);

    hash_set.insert(hasher.value());
    hash_vec.push(hasher.value());
    for remove in 0..(amount_of_hashes.saturating_sub(1)) {
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

pub(crate) fn clean_list_of_strings_single_pass(
    strings: Vec<String>,
    minimum_size: usize,
) -> Vec<String> {

    // Create a supervisor for each string:
    let mut supervisor_vec: Vec<StringSupervisor> = strings
        .into_iter()
        .map(|s| StringSupervisor::from_string(s, minimum_size))
        .collect();

    // Compute second occurrences:
    let filter_hashset = get_all_second_occurrences_of_substrings(&supervisor_vec);

    for s in supervisor_vec.iter_mut(){
        s.filter_string_from_hashset(&filter_hashset)
    }

    supervisor_vec.into_iter().map(|m| m.base_string).collect()
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
    fn counting_occurrences_first_and_second() {
        let example_string = "hell";
        let string_supervisor_1 = StringSupervisor::from_string(example_string.to_string(), 3);
        let string_supervisor_2 = StringSupervisor::from_string(example_string.to_string(), 3);
        let mut first_set: HashSet<u64> = HashSet::new();
        let mut second_set: HashSet<u64> = HashSet::new();

        track_first_and_second_occurrence_of_substring(
            &mut first_set,
            &mut second_set,
            &string_supervisor_1,
        );
        track_first_and_second_occurrence_of_substring(
            &mut first_set,
            &mut second_set,
            &string_supervisor_2,
        );

        assert_eq!(first_set, string_supervisor_1.complete_hashset);
        assert_eq!(second_set, first_set);
    }

    #[test]
    fn clean_large_set_of_files_single_pass() {
        use std::fs::{self};

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
        println!("{:?}", clean_strings);
        assert_eq!(clean_strings.len(), 148)
    }



    #[test]
    fn hash_vec_length_equals_none(){
        let string = "cccccc".to_string();
        let string_supervisor: StringSupervisor = StringSupervisor::from_string(string, 7);
        assert_eq!(string_supervisor.hash_vec.len(), 0);
    }


    #[test]
    fn hash_vec_length_equals_one(){
        let string = "cccccc".to_string();
        let string_supervisor: StringSupervisor = StringSupervisor::from_string(string, 6);
        assert_eq!(string_supervisor.hash_vec.len(), 1);
    }

    #[test]
    fn hash_vec_length_equals_two(){
        let string = "cccccc".to_string();
        let string_supervisor: StringSupervisor = StringSupervisor::from_string(string, 5);
        assert_eq!(string_supervisor.hash_vec.len(), 2);
    }

    #[test]
    fn clean_two_simple_strings_with_overlap() {
        let sub_string = "cccccd";
        let string_a = "aaaaaaaa";
        let string_b = "bbbbbbb";

        let mut sub_string_owned = sub_string.to_string();

        let mut byte_offsets: Vec<usize> =
                sub_string_owned.char_indices().map(|(i, _)| i).collect();

        sub_string_owned.drain(0..5);
        println!("{:?}", byte_offsets);

        //
        // let strings: Vec<String> = vec![string_a.to_string() + sub_string, string_b.to_string() + sub_string];
        // let clean_strings = clean_list_of_strings_single_pass(strings, 3);
        // assert_eq!(clean_strings, vec![string_a.to_string(), string_b.to_string()]);
        // println!("{:?}", clean_strings)
    }

    #[test]
    fn clean_two_simple_strings_no_overlap() {
        let sub_string = "";
        let string_a = "aaaaaaaa";
        let string_b = "bbbbbbb";
        let strings: Vec<String> = vec![string_a.to_string() + sub_string, string_b.to_string() + sub_string];
        let clean_strings = clean_list_of_strings_single_pass(strings, 3);
        assert_eq!(clean_strings, vec![string_a.to_string(), string_b.to_string()]);
        println!("{:?}", clean_strings)
    }
}
