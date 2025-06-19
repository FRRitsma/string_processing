use cyclic_poly_23::CyclicPoly64;
use std::collections::HashSet;
use std::ops::Range;
use rayon::prelude::*;

#[derive(Debug)]
pub struct StringSupervisor {
    base_string: String,
    window_size: usize,
    byte_offsets: Vec<usize>,
    hash_vec: Vec<u64>,
    complete_hashset: HashSet<u64>,
    duplicate_hash_set: HashSet<u64>,
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
                duplicate_hash_set: HashSet::new(),
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
            duplicate_hash_set: HashSet::new(),
        }

    }

    fn reducable(&self) -> bool {
        self.window_size < self.base_string.len()
    }

    pub fn compare(&mut self, other: &mut StringSupervisor) {
        if self.reducable() && other.reducable() {
            for item in self.complete_hashset.intersection(&other.complete_hashset) {
                self.duplicate_hash_set.insert(*item);
                other.duplicate_hash_set.insert(*item);
            }
        }
    }

    pub fn is_duplicate_mask(&self) -> Vec<bool> {
        let is_duplicate = self
            .hash_vec
            .iter()
            .map(|x| self.duplicate_hash_set.contains(&x))
            .collect();
        is_duplicate
    }

    pub fn filter_range(&self) -> Vec<Range<usize>> {
        let mut filter_range_vec: Vec<Range<usize>> = vec![];

        let mut duplicate_mask = self.is_duplicate_mask();
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
        for range in self.filter_range().into_iter().rev() {
            // Convert char-index range to byte-index range using byte_offsets
            let byte_start = self.byte_offsets[range.start];
            let byte_end = self.byte_offsets[range.end];
            self.base_string.drain(byte_start..byte_end);
        }
        self.base_string.clone()
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

fn clean_list_of_strings(strings: Vec<String>, minimum_size: usize) -> Vec<String>{
    let mut supervisor_vec: Vec<StringSupervisor> = strings.into_iter().map(|x| StringSupervisor::from_string(x, minimum_size)).collect();

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

    supervisor_vec.into_par_iter().map(|mut x| x.filter_string()).collect()


}


#[cfg(test)]
mod tests {
    use crate::remake_hash::clean_list_of_strings;
    use crate::test_utils::list_txt_files;
    use crate::remake_hash::StringSupervisor;

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

    #[test]
    fn clean_large_set_of_files(){
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

        let clean_strings = clean_list_of_strings(strings, 50);
        println!("{:?}", clean_strings);
    }


}
