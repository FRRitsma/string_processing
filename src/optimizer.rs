use crate::byte_sliceable_trait::ByteSliceable;
use crate::dev;
use crate::dev::find_second_to_longest;
use crate::has_len_trait::HasLen;
use std::ops::Div;

pub(crate) fn overlapping_hash_for_search_range<T: ByteSliceable>(
    collection: &[T],
    search_range: &SearchRange,
) -> Option<u64> {
    let hashset_vector = dev::convert_multiple_to_hashset(collection, search_range.midpoint());
    dev::overlapping_hash(hashset_vector)
}

pub fn length_and_hash_of_shared_substring<T: ByteSliceable + HasLen>(
    minimum_size: usize,
    maximum_size: Option<usize>,
    items: &[T],
) -> Option<(usize, u64)> {
    // Define the minimum and maximum possible sizes:
    let mut search_range: SearchRange;
    if let Some(maximum) = maximum_size {
        search_range = SearchRange::from_maximum(minimum_size, maximum);
    } else {
        search_range = SearchRange::from_collection(minimum_size, items);
    }

    let mut overlapping_hash = None;
    for _ in 0..64 {
        let loop_hash = overlapping_hash_for_search_range(items, &search_range);
        search_range.update(loop_hash.is_some());
        if let Some(hash) = loop_hash {
            overlapping_hash = Some(hash);
        }
        if search_range.is_finished() {
            break;
        }
    }

    search_range.best_estimate.zip(overlapping_hash)
}

#[derive(Debug)]
pub struct SearchRange {
    low: usize,
    high: usize,
    best_estimate: Option<usize>,
}

impl SearchRange {
    fn from_collection<T: HasLen>(minimum_size: usize, collection: &[T]) -> Self {
        let low = minimum_size;
        let high = find_second_to_longest(collection).max(low);
        SearchRange {
            low,
            high,
            best_estimate: None,
        }
    }

    fn from_maximum(minimum_size: usize, maximum_size: usize) -> Self {
        let low = minimum_size;
        let high = maximum_size + 1;
        SearchRange {
            low,
            high,
            best_estimate: None,
        }
    }

    fn midpoint(&self) -> usize {
        (self.low + self.high).div(2)
    }

    fn update(&mut self, is_substring_found: bool) {
        if is_substring_found {
            self.best_estimate = Some(self.midpoint());
            self.low = self.midpoint();
        } else {
            self.high = self.midpoint();
        }
    }

    fn is_finished(&self) -> bool {
        (self.high - self.low) <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn find_overlapping_hash_for_a_given_search_range_object() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let k: usize = 10;
        let search_range = SearchRange::from_collection(k, &txt_files_as_string);
        let overlapping_hash =
            overlapping_hash_for_search_range(&txt_files_as_string, &search_range);

        assert!(overlapping_hash.is_some())
    }

    #[test]
    fn test_optimizer_internal_logic() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let mut search_range = SearchRange::from_collection(6, &txt_files_as_string);

        for _ in 0..20 {
            let is_intersected =
                overlapping_hash_for_search_range(&txt_files_as_string, &search_range);
            search_range.update(is_intersected.is_some());

            if search_range.is_finished() {
                break;
            }
        }
        assert!(search_range.is_finished());
    }

    #[test]
    fn test_find_length_of_largest_shared_substring() {
        let txt_files_as_string: Vec<String> = test_utils::load_txt_files_as_vector_of_str();
        let (size, hash) =
            length_and_hash_of_shared_substring(10, None, &txt_files_as_string).unwrap();
        assert_eq!(size, 3059);
    }
}
