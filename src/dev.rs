use crate::byte_sliceable_trait::ByteSliceable;
use crate::has_len_trait::HasLen;
use ahash::AHasher;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hasher;
use std::fs;

pub fn convert_to_hashset<T: ByteSliceable>(text: &T, k: usize) -> HashSet<u64> {
    // Early return for edge cases
    if k == 0 || text.len() < k {
        return HashSet::new();
    }

    let mut hash_set = HashSet::with_capacity(text.len().saturating_sub(k) + 1);
    let bytes = text.as_bytes(); // Work with raw bytes for faster slicing

    for i in 0..=bytes.len().saturating_sub(k) {
        let substring = &bytes[i..i + k];
        let mut hasher = AHasher::default();
        hasher.write(substring);
        hash_set.insert(hasher.finish());
    }

    hash_set
}

pub fn convert_multiple_to_hashset<T: ByteSliceable>(items: &[T], k: usize) -> Vec<HashSet<u64>> {
    items
        .into_iter()
        .map(|item| convert_to_hashset(&item, k))
        .collect()
}

pub(crate) fn overlapping_hash(hashset_vector: Vec<HashSet<u64>>) -> Option<u64> {
    for pair in hashset_vector.iter().combinations(2) {
        let [set_a, set_b] = pair.as_slice() else {
            unreachable!()
        };
        if let Some(&common_hash) = set_a.intersection(set_b).next() {
            return Some(common_hash);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev::find_second_to_longest;
    use crate::test_utils::list_txt_files;
    use crate::{filter, optimizer, test_utils};
    use optimizer::length_and_hash_of_shared_substring;
    use test_utils::LOCATION_OF_TEST_FILES;

    #[test]
    fn show_all_txt_files_in_directory() {
        let txt_files = list_txt_files(LOCATION_OF_TEST_FILES).unwrap();
        assert_eq!(txt_files.len(), 11);
    }

    #[test]
    fn load_txt_as_str() {
        let txt_files = list_txt_files(LOCATION_OF_TEST_FILES).unwrap();
        let single_txt_file = txt_files.iter().next().unwrap();
        let content =
            fs::read_to_string(LOCATION_OF_TEST_FILES.to_owned() + single_txt_file).unwrap();
        assert_eq!(content.len(), 3881);
    }

    #[test]
    fn load_all_txt_files() {
        let txt_files_as_string = test_utils::load_txt_files_as_vector_of_str();
        assert_eq!(txt_files_as_string.len(), 11);
    }

    #[test]
    fn convert_all_txt_files_to_hashset() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let txt_files_as_hashset = convert_multiple_to_hashset(&txt_files_as_string, 128);
        assert_eq!(txt_files_as_hashset.len(), 11);
    }

    #[test]
    fn find_if_hashsets_intersect() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let txt_files_as_hashset = convert_multiple_to_hashset(&txt_files_as_string, 128);

        // Iterate over all unique pairs of sets
        assert!(overlapping_hash(txt_files_as_hashset).is_some());
    }

    #[test]
    fn find_second_longest_repeating_string() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let second_longest = find_second_to_longest(&txt_files_as_string);
        assert_eq!(second_longest, 5907);
    }

    #[test]
    fn test_remove_substring_single() {
        // For a given length, find the shared substring
        // TODO: Return hash of shared substring
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let length_and_hash = length_and_hash_of_shared_substring(10, None, &txt_files_as_string);
        let big_string = txt_files_as_string.first().unwrap();

        if let Some((length, forbidden_hash)) = length_and_hash {
            let result = filter::filter_forbidden_hash_single(big_string, length, forbidden_hash);
            assert_eq!(result.len() + length, big_string.len());
        }
    }

    #[test]
    fn test_remove_substring_multiple() {
        let mut txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let (length, forbidden_hash) =
            length_and_hash_of_shared_substring(10, None, &txt_files_as_string).unwrap();
        println!("{:?}", length);

        txt_files_as_string =
            filter::filter_forbidden_hash_multiple(txt_files_as_string, length, forbidden_hash);

        let (length, forbidden_hash) =
            length_and_hash_of_shared_substring(10, Some(length), &txt_files_as_string).unwrap();
        println!("{:?}", length);

        println!("{:?}", txt_files_as_string);
    }
}

pub(crate) fn find_second_to_longest<T: HasLen>(items: &[T]) -> usize {
    let mut first: usize = 0;
    let mut second: usize = 0;

    for item in items.iter() {
        let current = item.len();
        if current > first {
            second = first;
            first = current;
        } else if current > second {
            second = current;
        }
    }
    second
}
