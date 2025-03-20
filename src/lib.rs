// #![warn(clippy::pedantic, clippy::perf)]

use std::{env, fmt::Display, path::PathBuf};

use async_handling::async_runner;
use errors::*;
use pyo3::prelude::*;
use refman::prelude::*;

#[pyclass]
#[pyo3(name = "RegistryOptions")]
struct RefmanOptions(RegistryOptions);

#[pymethods]
impl RefmanOptions {
    #[staticmethod]
    #[pyo3(signature = (title = None, description = None, requested_path = None, global = false))]
    fn new(
        title: Option<String>,
        description: Option<String>,
        requested_path: Option<String>,
        global: bool,
    ) -> PyResult<Self> {
        let options =
            RegistryOptions::try_new(title, description, requested_path, global).into_pyresult()?;

        Ok(RefmanOptions(options))
    }

    fn init_project(&self) -> PyResult<()> {
        self.0.init().into_pyresult()
    }
}

#[allow(dead_code)]
#[pyclass]
#[pyo3(name = "RefDataset")]
struct PyRefDataset(RefDataset);

#[pymethods]
impl PyRefDataset {
    #[staticmethod]
    #[pyo3(signature = (label, fasta=None, genbank=None, gfa=None, gff=None, gtf=None, bed=None))]
    pub fn try_new(
        label: String,
        fasta: Option<String>,
        genbank: Option<String>,
        gfa: Option<String>,
        gff: Option<String>,
        gtf: Option<String>,
        bed: Option<String>,
    ) -> PyResult<PyRefDataset> {
        let new_dataset = async_runner(|| async {
            RefDataset::try_new(label, fasta, genbank, gfa, gff, gtf, bed)
                .await
                .map_err(anyhow::Error::from)
        })
        .into_pyresult()?;

        Ok(PyRefDataset(new_dataset))
    }
}

#[pyclass]
#[derive(Debug)]
struct RefmanProject(Project);

impl Display for RefmanProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[pymethods]
impl RefmanProject {}

#[pyfunction]
#[pyo3(signature = (title = None, description = None, requested_path = None, global = false))]
fn init(
    title: Option<String>,
    description: Option<String>,
    requested_path: Option<String>,
    global: bool,
) -> PyResult<()> {
    RefmanOptions::new(title, description, requested_path, global)?.init_project()
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (label, fasta=None, genbank=None, gfa=None, gff=None, gtf=None, bed=None, registry=None, global=false))]
fn register(
    label: String,
    fasta: Option<String>,
    genbank: Option<String>,
    gfa: Option<String>,
    gff: Option<String>,
    gtf: Option<String>,
    bed: Option<String>,
    registry: Option<String>,
    global: bool,
) -> PyResult<()> {
    let new_dataset = async_runner(|| async {
        RefDataset::try_new(label, fasta, genbank, gfa, gff, gtf, bed)
            .await
            .map_err(anyhow::Error::from)
    })
    .into_pyresult()?;
    let options = RegistryOptions::try_new(None, None, registry, global).into_pyresult()?;
    let mut project = options
        .read_registry()
        .into_pyresult()?
        .register(new_dataset)
        .into_pyresult()?;
    options.write_registry(&mut project).into_pyresult()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label, registry = None, global = false))]
fn remove(label: String, registry: Option<String>, global: bool) -> PyResult<()> {
    let options = RegistryOptions::try_new(None, None, registry, global).into_pyresult()?;
    let mut project = options
        .read_registry()
        .into_pyresult()?
        .remove(&label)
        .into_pyresult()?;
    options.write_registry(&mut project).into_pyresult()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label, dest = None, registry = None, global = false))]
fn download(
    label: String,
    dest: Option<String>,
    registry: Option<String>,
    global: bool,
) -> PyResult<()> {
    let options = RegistryOptions::try_new(None, None, registry, global).into_pyresult()?;
    let project = options.read_registry().into_pyresult()?;
    if !project.is_registered(&label) {
        Err(RegistryError::NotRegistered(label.clone())).into_pyresult()?;
    }
    let destination = match dest {
        Some(dest) => PathBuf::from(dest),
        None => env::current_dir()?,
    };

    async_runner(|| project.download_dataset(&label, destination)).into_pyresult()?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label = None, registry = None, global = false))]
fn list(label: Option<String>, registry: Option<String>, global: bool) -> PyResult<()> {
    RegistryOptions::try_new(None, None, registry, global)
        .into_pyresult()?
        .read_registry()
        .into_pyresult()?
        .prettyprint(label);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "refman")]
fn py_refman(_py: Python, pymodule: &PyModule) -> PyResult<()> {
    // add wrapped classes
    pymodule.add_class::<RefmanOptions>()?;
    pymodule.add_class::<RefmanProject>()?;
    pymodule.add_class::<PyRefDataset>()?;

    // add wrapped functions
    pymodule.add_function(wrap_pyfunction!(init, pymodule)?)?;
    pymodule.add_function(wrap_pyfunction!(register, pymodule)?)?;
    pymodule.add_function(wrap_pyfunction!(download, pymodule)?)?;
    pymodule.add_function(wrap_pyfunction!(remove, pymodule)?)?;
    pymodule.add_function(wrap_pyfunction!(list, pymodule)?)?;

    Ok(())
}

pub(crate) mod async_handling {

    use std::future::Future;

    use anyhow::{bail, Error};

    pub fn async_runner<F, Fut, T>(func: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let runtime = tokio::runtime::Runtime::new()
            .expect("Failed to launch multi-threaded asynchronous code runner. Aborting.");

        runtime.block_on(async {
            tokio::select! {
                result = func() => {
                    // If the main future completes first, just return its output.
                    result
                }
                _ = tokio::signal::ctrl_c() => {
                    // If Ctrl+C is pressed first, return an error or handle gracefully.
                    bail!("Operation canceled by Ctrl+C")
                }
            }
        })
    }
}

pub(crate) mod errors {
    //! A submodule to handle implicit conversions between orphan-rule-protected
    //! external module errors and PyO3's python errors.

    use std::fmt::Display;

    use anyhow::Error as Report;
    use pyo3::{exceptions::PyValueError, prelude::*};
    use refman::prelude::{DownloadError, EntryError, RegistryError};

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct PyReport(Report);

    #[derive(Debug)]
    pub struct PyEntryError(EntryError);

    #[derive(Debug)]
    pub struct PyDownloadError(DownloadError);

    #[derive(Debug)]
    pub struct PyRegistryError(RegistryError);

    impl Display for PyReport {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Display for PyEntryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Display for PyDownloadError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Display for PyRegistryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for PyReport {}
    impl std::error::Error for PyEntryError {}
    impl std::error::Error for PyDownloadError {}
    impl std::error::Error for PyRegistryError {}

    impl From<PyReport> for PyErr {
        fn from(value: PyReport) -> Self {
            PyValueError::new_err(value.to_string())
        }
    }

    impl From<Report> for PyReport {
        fn from(other: Report) -> Self {
            Self(other)
        }
    }

    impl From<PyEntryError> for PyErr {
        fn from(value: PyEntryError) -> Self {
            PyValueError::new_err(value.to_string())
        }
    }
    impl From<EntryError> for PyEntryError {
        fn from(value: EntryError) -> Self {
            Self(value)
        }
    }

    impl From<PyDownloadError> for PyErr {
        fn from(value: PyDownloadError) -> Self {
            PyValueError::new_err(value.to_string())
        }
    }
    impl From<DownloadError> for PyDownloadError {
        fn from(value: DownloadError) -> Self {
            Self(value)
        }
    }

    impl From<PyRegistryError> for PyErr {
        fn from(value: PyRegistryError) -> Self {
            PyValueError::new_err(value.to_string())
        }
    }
    impl From<RegistryError> for PyRegistryError {
        fn from(value: RegistryError) -> Self {
            Self(value)
        }
    }

    pub trait IntoPyResult<T> {
        fn into_pyresult(self) -> PyResult<T>;
    }

    impl<T> IntoPyResult<T> for Result<T, Report> {
        fn into_pyresult(self) -> PyResult<T> {
            self.map_err(|e| PyReport::from(e).into())
        }
    }

    impl<T> IntoPyResult<T> for Result<T, EntryError> {
        fn into_pyresult(self) -> PyResult<T> {
            self.map_err(|e| PyEntryError::from(e).into())
        }
    }

    impl<T> IntoPyResult<T> for Result<T, DownloadError> {
        fn into_pyresult(self) -> PyResult<T> {
            self.map_err(|e| PyDownloadError::from(e).into())
        }
    }

    impl<T> IntoPyResult<T> for Result<T, RegistryError> {
        fn into_pyresult(self) -> PyResult<T> {
            self.map_err(|e| PyRegistryError::from(e).into())
        }
    }
}
