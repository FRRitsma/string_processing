mod byte_sliceable_trait;
mod dev;
mod filter;
mod has_len_trait;
mod optimizer;
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

#[pymodule]
fn string_processing(module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the function with proper docstring visibility
    let function = wrap_pyfunction!(filter_list_of_strings, module)?;
    module.add_function(function)?;
    
    // Explicitly add to `__all__` for Python visibility
    module.add("__all__", vec!["filter_list_of_strings"])?;
    
    Ok(())
}