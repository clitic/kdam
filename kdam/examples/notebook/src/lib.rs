use kdam::{tqdm, BarExt};
use pyo3::prelude::*;

#[pyfunction]
fn set_notebook(running: bool) -> PyResult<()> {
    kdam::set_notebook(running);
    Ok(())
}

#[pyfunction]
fn progress_bar() -> PyResult<()> {
    let mut pb = tqdm!(total = 300, force_refresh = true);

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1).unwrap();
    }

    Ok(())
}

#[pyfunction]
fn progress_bar_with_template() -> PyResult<()> {
    let mut pb = tqdm!(
        total = 300,
        force_refresh = true,
        bar_format = "{animation} {percentage:.2}% @ {rate:.1} it/s"
    );

    for _ in 0..300 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.02));
        pb.update(1).unwrap();
    }

    Ok(())
}

#[pymodule]
fn kdam_notebook(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_notebook, m)?)?;
    m.add_function(wrap_pyfunction!(progress_bar, m)?)?;
    m.add_function(wrap_pyfunction!(progress_bar_with_template, m)?)?;
    Ok(())
}
