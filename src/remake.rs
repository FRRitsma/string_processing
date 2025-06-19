fn get_largest_overlapping_sub_string(
    string_a: &String,
    string_b: &String,
    mininum_size: usize,
) -> String {
    // Early exit:
    if string_a.len() < mininum_size || string_b.len() < mininum_size {
        return String::new();
    }

    // As characters, to prevent cutting on invalid utf-8 bytes:
    let chars_a: Vec<char> = string_a.chars().collect();
    let chars_b: Vec<char> = string_b.chars().collect();

    for id_a in 0..chars_a.len().saturating_sub(mininum_size) {
        for id_b in 0..chars_b.len().saturating_sub(mininum_size) {
            let mut id_scan: usize = 0;
            while chars_a[id_a + id_scan] == chars_b[id_b + id_scan] {
                id_scan += 1;
            }
            if id_scan > mininum_size {
                return chars_a[id_a..id_scan].iter().collect();
            }
        }
    }
    return String::new();
}

fn remove_overlap_from_left_string(
    string_a: String,
    string_b: &String,
    mininum_size: usize,
) -> String {
    // Early exit:
    if string_a.len() < mininum_size || string_b.len() < mininum_size {
        return string_a;
    }

    // As characters, to prevent cutting on invalid utf-8 bytes:
    let mut chars_a: Vec<char> = string_a.chars().collect();
    let chars_b: Vec<char> = string_b.chars().collect();

    for id_a in 0..chars_a.len().saturating_sub(mininum_size) {
        for id_b in 0..chars_b.len().saturating_sub(mininum_size) {
            let mut id_scan: usize = 0;

            while id_a + id_scan < chars_a.len()
                && id_b + id_scan < chars_b.len()
                && chars_a[id_a + id_scan] == chars_b[id_b + id_scan]
            {
                id_scan += 1;
            }
            if id_scan >= mininum_size {
                chars_a.drain(id_a..id_a + id_scan);
            }
        }
    }
    return chars_a.into_iter().collect();
}

pub fn clean_vector_of_strings(
    mut vector_strings: Vec<String>,
    minimum_size: usize,
) -> Vec<String> {
    for i in 0..vector_strings.len() {
        for j in 0..vector_strings.len() {
            // No combinations with self:
            if i == j {
                continue;
            }
            vector_strings[i] = remove_overlap_from_left_string(
                vector_strings[i].clone(),
                &vector_strings[j],
                minimum_size,
            );
        }
    }
    vector_strings
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let shared = "xxxxxxx";
        let string_a = shared.to_string() + "aaaa";
        let string_b = shared.to_string() + "bbbb";

        let output = get_largest_overlapping_sub_string(&string_a, &string_b, 3);
        println!("{}", output);
    }

    #[test]
    fn second_test() {
        let shared = "xxx";
        let string_a = shared.to_string() + "aaaa" + shared;
        // let string_a = "aaaa".to_string() + shared;
        let string_b = shared.to_string() + "bbbb";
        let output = remove_overlap_from_left_string(string_a, &string_b, 3);
        println!("{}", output);
    }

    #[test]
    fn third_test() {
        let shared = "xxx";
        let vec = vec![
            shared.to_string() + "aaa" + shared,
            "bbb".to_string() + shared,
            "ccc".to_string() + shared,
        ];
        let output_vec = clean_vector_of_strings(vec, 2);
        println!("{:?}", output_vec);
    }
}
