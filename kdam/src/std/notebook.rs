use std::sync::atomic::{AtomicBool, Ordering};
use pyo3::{Bound, Py, PyAny, Python};

static RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub(crate) struct PyContainer(Py<PyAny>);

impl Clone for PyContainer {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            PyContainer(self.0.clone_ref(py))
        })
    }
}

impl PyContainer {
    #[inline]
    pub fn bind<'py>(&self, py: Python<'py>) -> &Bound<'py, PyAny> {
        self.0.bind(py)
    }
}

impl<'py> From<Bound<'py, PyAny>> for PyContainer {
    fn from(value: Bound<'py, PyAny>) -> Self {
        Self(value.into())
    }
}


/// Set whether `kdam` is running inside a jupyter notebook or not.
pub fn set_notebook(running: bool) {
    RUNNING.store(running, Ordering::SeqCst);
}

pub(super) fn running() -> bool {
    RUNNING.load(Ordering::Acquire)
}
