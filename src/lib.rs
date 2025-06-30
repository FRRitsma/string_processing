extern crate cyclic_poly_23;
extern crate pyo3;

mod string_filter_rolling_hash;
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
    let filtered_strings =
        string_filter_rolling_hash::clean_list_of_strings_single_pass(strings, minimum_size);
    Ok(filtered_strings)
}

#[pymodule]
fn string_processing(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add functions
    module.add_function(wrap_pyfunction!(filter_list_of_strings, module)?)?;

    // Add __all__ (correct modern PyO3 way)
    let all = vec!["filter_list_of_strings"];
    module.add("__all__", all)?;

    Ok(())
}
