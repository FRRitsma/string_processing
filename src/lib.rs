mod byte_sliceable_trait;
mod dev;
mod filter;
mod from_files;
mod has_len_trait;
mod optimizer;
mod prefix_removal;
mod test_utils;

use pyo3::prelude::*;

/// Filters a list of strings by removing shared substrings.
///
/// # Arguments
/// * `strings` - A list of strings to filter (Vec<String>)
/// * `minimum_size` - Minimum length of shared substrings to remove (usize)
///
/// # Examples
/// ```
/// >>> filter_list_of_strings(["hello".to_string(), "hell".to_string()], 3)
/// ["hello"]
/// ```
#[pyfunction]
#[pyo3(signature = (strings, minimum_size))]
#[pyo3(text_signature = "(strings: list[str], minimum_size: int) -> list[str]")]
fn filter_list_of_strings(strings: Vec<String>, minimum_size: usize) -> PyResult<Vec<String>> {
    let filtered_strings = filter::remove_substrings(strings, minimum_size);
    Ok(filtered_strings)
}

/// Filters a list of strings by removing shared prefixes.
///
/// # Arguments
/// * `strings` - A list of strings to filter (Vec<String>)
/// * `minimum_size` - Minimum length of prefix to remove (usize)
///
/// # Examples
/// ```
/// >>> filter_list_of_strings_prefix(["aaaabbb".to_string(), "aaaaccc".to_string()], 3)
/// ["bbb", "aaaaccc"]
/// ```
#[pyfunction]
#[pyo3(signature = (strings, minimum_size))]
#[pyo3(text_signature = "(strings: list[str], minimum_size: int) -> list[str]")]
fn filter_list_of_strings_prefix(
    strings: Vec<String>,
    minimum_size: usize,
) -> PyResult<Vec<String>> {
    let filtered_strings = prefix_removal::remove_shared_prefix(strings, minimum_size);
    Ok(filtered_strings)
}

#[pymodule]
fn string_processing(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add functions
    module.add_function(wrap_pyfunction!(filter_list_of_strings, module)?)?;
    module.add_function(wrap_pyfunction!(filter_list_of_strings_prefix, module)?)?;

    // Add __all__ (correct modern PyO3 way)
    let all = vec!["filter_list_of_strings", "filter_list_of_strings_prefix"];
    module.add("__all__", all)?;

    Ok(())
}
