use bevy::prelude::*;
use pyo3::prelude::*;


#[pyfunction]
fn main() {
    App::new()
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}


#[pymodule]
fn bevy_python_minimal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
