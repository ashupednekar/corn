mod cmd;
mod pkg;

use pkg::server::serve;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

#[pyfunction]
fn start() -> PyResult<()> {
    tokio::task::block_in_place(move || {
        let rt = Runtime::new().expect("failed");
        rt.block_on(async {
            serve().await.unwrap();     
        });
    });
    Ok(())
}

#[pymodule]
fn corn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start, m)?)?;
    Ok(())
}
