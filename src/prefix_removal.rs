pub(crate) fn remove_shared_prefix(
    mut compare_strings: Vec<String>,
    minimum_size: usize,
) -> Vec<String> {
    /*
    Keeps at least one prefix:
    ["aaaaaaaacccc", "aaaaaaaabbbb", "aaaaaaaaddddd"] -> ["cccc", "bbbb", "aaaaaaaaddddd"]
    To prevent removal of essential information.
    */
    for outer_index in 0..compare_strings.len() {
        let change_string = &compare_strings[outer_index].clone();

        // Early exit:
        if minimum_size > change_string.len() {
            continue;
        }

        // Initialize inner loop:
        let mut size_of_overlap: usize = 0;
        let mut prefix = &change_string[0..minimum_size];

        for inner_index in 0..compare_strings.len() {
            // Don't compare against self:
            if inner_index == outer_index {
                continue;
            }
            let other_string = &compare_strings[inner_index];

            // Early exit:
            if minimum_size > other_string.len() {
                continue;
            }

            while other_string.starts_with(prefix) && (size_of_overlap < change_string.len()) {
                size_of_overlap = size_of_overlap.max(minimum_size) + 1;
                prefix = &change_string[0..size_of_overlap];
            }
        }

        // Update the string in place
        if size_of_overlap.saturating_sub(1) > 0 {
            compare_strings[outer_index] =
                change_string[size_of_overlap.saturating_sub(1)..].to_owned();
        }
    }

    compare_strings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::list_txt_files;
    use std::fs;

    #[test]
    fn prefix_removal() {
        let string_vec: Vec<String> = vec![
            "aaaaaaaacccc".to_owned(),
            "aaaaaaaabbbb".to_owned(),
            "aaaaaaaaddddd".to_owned(),
        ];
        let clean_string = remove_shared_prefix(string_vec, 4);
        assert_eq!(
            vec![
                "cccc".to_owned(),
                "bbbb".to_owned(),
                "aaaaaaaaddddd".to_owned()
            ],
            clean_string
        );
    }
}
