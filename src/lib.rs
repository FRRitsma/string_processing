mod byte_sliceable_trait;
mod dev;
mod filter;
mod has_len_trait;
mod optimizer;
mod test_utils;

use pyo3::prelude::*;

#[pyfunction]
fn filter_list_of_strings(strings: Vec<String>, minimum_size: usize) -> PyResult<Vec<String>> {
    let filtered_strings = filter::remove_substrings(strings, minimum_size);
    Ok(filtered_strings)
}

#[pymodule]
fn string_processing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filter_list_of_strings, m)?)?;
    Ok(())
}
