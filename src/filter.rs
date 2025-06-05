use crate::optimizer::length_and_hash_of_shared_substring;
use ahash::AHasher;
use std::hash::Hasher;

const MAX_LOOP: u32 = u32::MAX;

pub fn filter_forbidden_hash_multiple<T: AsRef<[u8]>>(
    collected_text: Vec<T>,
    length: usize,
    forbidden_hash: u64,
) -> Vec<String> {
    collected_text
        .into_iter()
        .map(|text| filter_forbidden_hash_single(&text, length, forbidden_hash))
        .collect()
}

pub fn filter_forbidden_hash_single<T: AsRef<[u8]>>(
    text: &T,
    length: usize,
    forbidden_hash: u64,
) -> String {
    let bytes = text.as_ref();

    if length == 0 || bytes.len() < length {
        return String::from_utf8(bytes.to_vec())
            .expect("Input was valid UTF-8 (guaranteed by AsRef<[u8]> for strings)");
    }
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i <= bytes.len().saturating_sub(length) {
        let substring = &bytes[i..i + length];
        let mut hasher = AHasher::default();
        hasher.write(substring);
        if hasher.finish() == forbidden_hash {
            i += length; // Skip forbidden substring
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }

    // Add remaining bytes
    if i < bytes.len() {
        result.extend_from_slice(&bytes[i..]);
    }

    String::from_utf8(result).unwrap()
}

pub fn remove_substrings(string_vector: Vec<String>, minimum_size: usize) -> Vec<String> {
    // Loop initialization:
    let mut inner_vector: Vec<String>;

    // First overlap search can not rely on previous computations (maximum_size = None):
    let length_and_hash = length_and_hash_of_shared_substring(minimum_size, None, &string_vector);

    // Enter loop if overlap is found:
    if let Some((length, forbidden_hash)) = length_and_hash {
        // Filter strings:
        inner_vector = filter_forbidden_hash_multiple(string_vector, length, forbidden_hash);

        // Keep filtering strings, until no overlap is found:
        for _ in 0..MAX_LOOP {
            // Find the length and hash of the longest shared substring:
            let length_and_hash =
                length_and_hash_of_shared_substring(minimum_size, Some(length), &inner_vector);
            // If an overlap is found, filter the strings and repeat:
            if let Some((length, forbidden_hash)) = length_and_hash {
                inner_vector = filter_forbidden_hash_multiple(inner_vector, length, forbidden_hash);
            } else {
                return inner_vector;
            }
        }
    }
    // Return original vector if no overlap was found:
    else {
        return string_vector;
    }
    inner_vector
}

#[cfg(test)]
mod tests {
    use crate::filter::remove_substrings;
    use crate::test_utils::load_txt_files_as_vector_of_str;

    #[test]
    fn filter_until_completion() {
        let string_vector: Vec<String> = load_txt_files_as_vector_of_str();
        let minimum_size = 25;

        let filtered_vector = remove_substrings(string_vector, minimum_size);

        println!("{:?}", filtered_vector);
    }
}
