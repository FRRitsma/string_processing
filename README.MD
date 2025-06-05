[Steps in creating a python api:

1. cargo add pyo3
2. add  file "lib.rs" in ./src, attach to main.rs: "mod lib;"

Adapt the cargo.toml, include:

[lib]
name = "name_in_python"

In lib.rs add:
#[pymodule]

fn name_in_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}


# Using in python
Build a .venv:

`python -m venv demo_env`

Activate environment:

`demo_env\Scripts\activate`

Install from wheel:

`pip install target/wheels/your_package_name-*.whl`