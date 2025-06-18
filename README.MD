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


# Local install:
Build a .venv:

`python -m venv demo_env`

Activate environment:

`demo_env\Scripts\activate`

Install from wheel:

`pip install target/wheels/string_processing-0.1.0-cp311-cp311-win_amd64.whl`

# Pip install from github:

`pip install https://github.com/FRRitsma/string_processing/releases/download/v1/string_processing-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl`