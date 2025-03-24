#![warn(clippy::pedantic, clippy::perf)]

//! # Refman
//!
//! A reference sequence registry manager designed for clean organization and
//! downloading of reference genomes and related files.
//!
//! ## Building From Source
//!
//! ```bash
//! # Install maturin
//! pip install maturin
//!
//! # Build and install in development mode
//! maturin develop
//!
//! # Build release wheel
//! maturin build --release
//! ```
//!
//! ## Python Usage
//!
//! ```python
//! # Initialize a registry
//! import refman
//! refman.init(title="My References", description="My genome references")
//!
//! # Register a dataset
//! refman.register(
//!     label="e_coli",
//!     fasta="path/to/ecoli.fasta",
//!     genbank="path/to/ecoli.gb"
//! )
//!
//! # List registered references
//! refman.list()
//!
//! # Download a reference
//! refman.download("e_coli", dest="output/dir")
//!
//! # Remove a reference
//! refman.remove("e_coli")
//! ```
//!
//! ## Python API
//!
//! - `init(title=None, description=None, requested_path=None, global_project=False)` - Initialize a new registry
//! - `register(label, fasta=None, genbank=None, gfa=None, gff=None, gtf=None, bed=None, registry=None, global_project=False)` - Register a dataset
//! - `download(label, dest=None, registry=None, global_project=False)` - Download a registered dataset
//! - `remove(label, registry=None, global_project=False)` - Remove a registered dataset
//! - `list(label=None, registry=None, global_project=False)` - List registered datasets
//!
//! ## Rust API
//!
//! The crate exposes the following main types:
//!
//! - `RefmanOptions` - Configuration options for registry initialization
//! - `RefmanProject` - Represents a reference registry project
//! - `PyRefDataset` - A reference dataset containing genomic data files
//!
//! The Python interface is implemented via `PyO3` bindings to these core Rust types.

use std::{env, fmt::Display, path::PathBuf};

use async_handling::async_runner;
use errors::IntoPyResult;
use pyo3::prelude::*;
use refman::prelude::*;

#[pyclass]
#[pyo3(name = "RegistryOptions")]
struct RefmanOptions(RegistryOptions);

#[pymethods]
impl RefmanOptions {
    #[staticmethod]
    #[pyo3(signature = (title = None, description = None, requested_path = None, global_project = false))]
    fn new(
        title: Option<String>,
        description: Option<String>,
        requested_path: Option<String>,
        global_project: bool,
    ) -> PyResult<Self> {
        let options = RegistryOptions::try_new(title, description, requested_path, global_project)
            .into_pyresult()?;

        Ok(RefmanOptions(options))
    }

    fn init_project(&self) -> PyResult<()> {
        self.0.init().into_pyresult()
    }

    fn read_registry(&self) -> PyResult<RefmanProject> {
        let project = self.0.read_registry().into_pyresult()?;
        Ok(RefmanProject(project))
    }

    fn write_registry(&self, project: &mut RefmanProject) -> PyResult<()> {
        let internal_project = &mut project.0;
        self.0.write_registry(internal_project).into_pyresult()?;
        Ok(())
    }
}

#[allow(dead_code)]
#[pyclass]
#[pyo3(name = "RefDataset")]
struct PyRefDataset(RefDataset);

#[allow(clippy::similar_names)]
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

    #[getter]
    fn label(&self) -> &str {
        &self.0.label
    }

    #[getter]
    fn fasta(&self) -> Option<&str> {
        self.0.fasta.as_deref()
    }

    #[getter]
    fn genbank(&self) -> Option<&str> {
        self.0.genbank.as_deref()
    }

    #[getter]
    fn gfa(&self) -> Option<&str> {
        self.0.gfa.as_deref()
    }

    #[getter]
    fn gff(&self) -> Option<&str> {
        self.0.gff.as_deref()
    }

    #[getter]
    fn gtf(&self) -> Option<&str> {
        self.0.gtf.as_deref()
    }

    #[getter]
    fn bed(&self) -> Option<&str> {
        self.0.bed.as_deref()
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
impl RefmanProject {
    #[staticmethod]
    #[pyo3(signature = (title=None, description=None, global_dataset=false))]
    fn new(title: Option<String>, description: Option<String>, global_dataset: bool) -> Self {
        let internal_project = Project::new(title, description, global_dataset);

        Self(internal_project)
    }

    fn datasets(&self) -> Vec<PyRefDataset> {
        let internal_datasets = self.0.datasets();
        let py_datasets: Vec<_> = internal_datasets
            .iter()
            .map(|dataset| PyRefDataset(dataset.clone()))
            .collect();

        py_datasets
    }

    fn get_dataset(&self, label: &str) -> PyResult<PyRefDataset> {
        let dataset =
            async_runner(|| async { self.0.get_dataset(label).await.map_err(anyhow::Error::from) })
                .into_pyresult()?;

        Ok(PyRefDataset(dataset.clone()))
    }

    fn get_dataset_urls(&self, label: &str) -> PyResult<Vec<String>> {
        let urls = async_runner(|| async {
            self.0
                .get_dataset_urls(label)
                .await
                .map_err(anyhow::Error::from)
        })
        .into_pyresult()?;

        Ok(urls)
    }

    fn is_registered(&self, label: &str) -> bool {
        self.0.is_registered(label)
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    #[pyo3(signature = (label, fasta=None, genbank=None, gfa=None, gff=None, gtf=None, bed=None))]
    fn register(
        &self,
        label: String,
        fasta: Option<String>,
        genbank: Option<String>,
        gfa: Option<String>,
        gff: Option<String>,
        gtf: Option<String>,
        bed: Option<String>,
    ) -> PyResult<Self> {
        let new_dataset = async_runner(|| async {
            RefDataset::try_new(label, fasta, genbank, gfa, gff, gtf, bed)
                .await
                .map_err(anyhow::Error::from)
        })
        .into_pyresult()?;
        let replacement_proj = self.0.clone().register(new_dataset).into_pyresult()?;
        Ok(RefmanProject(replacement_proj))
    }

    #[staticmethod]
    #[pyo3(signature = (global_dataset=false, title=None, description=None, requested_path=None))]
    fn read_registry(
        global_dataset: bool,
        title: Option<String>,
        description: Option<String>,
        requested_path: Option<String>,
    ) -> PyResult<RefmanProject> {
        let options = RegistryOptions::try_new(title, description, requested_path, global_dataset)
            .into_pyresult()?;
        let project = options.read_registry().into_pyresult()?;
        Ok(RefmanProject(project))
    }

    #[staticmethod]
    #[pyo3(signature = (project, global_dataset=false, title=None, description=None, requested_path=None))]
    fn write_registry(
        project: &mut RefmanProject,
        global_dataset: bool,
        title: Option<String>,
        description: Option<String>,
        requested_path: Option<String>,
    ) -> PyResult<()> {
        let options = RegistryOptions::try_new(title, description, requested_path, global_dataset)
            .into_pyresult()?;
        let internal_project = &mut project.0;
        options.write_registry(internal_project).into_pyresult()?;
        Ok(())
    }
}

// TOP-LEVEL FUNCTIONS
// ---------------------

#[pyfunction]
#[pyo3(signature = (title = None, description = None, requested_path = None, global_project = false))]
fn init(
    title: Option<String>,
    description: Option<String>,
    requested_path: Option<String>,
    global_project: bool,
) -> PyResult<()> {
    RefmanOptions::new(title, description, requested_path, global_project)?.init_project()
}

#[allow(clippy::too_many_arguments, clippy::similar_names)]
#[pyfunction]
#[pyo3(signature = (label, fasta=None, genbank=None, gfa=None, gff=None, gtf=None, bed=None, registry=None, global_project=false))]
fn register(
    label: String,
    fasta: Option<String>,
    genbank: Option<String>,
    gfa: Option<String>,
    gff: Option<String>,
    gtf: Option<String>,
    bed: Option<String>,
    registry: Option<String>,
    global_project: bool,
) -> PyResult<()> {
    let new_dataset = async_runner(|| async {
        RefDataset::try_new(label, fasta, genbank, gfa, gff, gtf, bed)
            .await
            .map_err(anyhow::Error::from)
    })
    .into_pyresult()?;
    let options = RegistryOptions::try_new(None, None, registry, global_project).into_pyresult()?;
    let mut project = options
        .read_registry()
        .into_pyresult()?
        .register(new_dataset)
        .into_pyresult()?;
    options.write_registry(&mut project).into_pyresult()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label, registry = None, global_project = false))]
fn remove(label: &str, registry: Option<String>, global_project: bool) -> PyResult<()> {
    let options = RegistryOptions::try_new(None, None, registry, global_project).into_pyresult()?;
    let mut project = options
        .read_registry()
        .into_pyresult()?
        .remove(label)
        .into_pyresult()?;
    options.write_registry(&mut project).into_pyresult()?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label, dest = None, registry = None, global_project = false))]
fn download(
    label: &str,
    dest: Option<String>,
    registry: Option<String>,
    global_project: bool,
) -> PyResult<()> {
    let options = RegistryOptions::try_new(None, None, registry, global_project).into_pyresult()?;
    let project = options.read_registry().into_pyresult()?;
    if !project.is_registered(label) {
        Err(RegistryError::NotRegistered(label.to_string())).into_pyresult()?;
    }
    let destination = match dest {
        Some(dest) => PathBuf::from(dest),
        None => env::current_dir()?,
    };

    async_runner(|| project.download_dataset(label, destination)).into_pyresult()?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (label = None, registry = None, global_project = false))]
fn list_datasets(
    label: Option<String>,
    registry: Option<String>,
    global_project: bool,
) -> PyResult<()> {
    RegistryOptions::try_new(None, None, registry, global_project)
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
    pymodule.add_function(wrap_pyfunction!(list_datasets, pymodule)?)?;

    Ok(())
}

pub(crate) mod async_handling {

    //! The asynchronous handling submodule.
    //!
    //! This module provides utilities for running asynchronous code and handling cancellation signals.
    //! The main functionality is exposed through the `async_runner` function which takes an async
    //! function and executes it while watching for Ctrl+C cancellation.

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
    //! external module errors and `PyO3`'s python errors.
    //!
    //! //! The errors module defines adaptors and wrappers to manage the conversion between
    //! error types from external crates when converting between Rust and Python values.
    //!
    //! # Error Conversion Types
    //!
    //! - `PyReport`: Wraps Anyhow `Report` errors for conversion to Python exceptions
    //! - `PyEntryError`: Wraps `EntryError` for file access/validation related errors
    //! - `PyDownloadError`: Wraps `DownloadError` for network/http related errors
    //! - `PyRegistryError`: Wraps `RegistryError` for registry operations errors
    //!
    //! # Error Conversion Flows
    //! This module implements two main error conversion patterns:
    //!
    //! 1. Direct error wrapping and conversion to Python exceptions via the `PyErr` type:
    //!    - `Report -> PyReport -> PyErr`
    //!    - `EntryError -> PyEntryError -> PyErr`
    //!    - `DownloadError -> PyDownloadError -> PyErr`
    //!    - `RegistryError -> PyRegistryError -> PyErr`
    //!
    //! 2. Convenient result conversion via the `IntoPyResult` trait:
    //!    - `Result<T, Report> -> PyResult<T>`
    //!    - `Result<T, EntryError> -> PyResult<T>`
    //!    - `Result<T, DownloadError> -> PyResult<T>`
    //!    - `Result<T, RegistryError> -> PyResult<T>`
    //!
    //! Each wrapped error provides its own Display implementation and is converted
    //! to a Python `ValueError` with an appropriate error message.

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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refman_options_new() {
        let options = RefmanOptions::new(
            Some("Test Title".to_string()),
            Some("Test Description".to_string()),
            None,
            false,
        );
        assert!(options.is_ok());
    }

    #[test]
    fn test_py_refdataset_new() {
        let dataset = PyRefDataset::try_new(
            "test_label".to_string(),
            Some("test.fasta".to_string()),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(dataset.is_ok());
    }

    #[test]
    fn test_init_with_empty_options() {
        let result = init(None, None, None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_with_title() {
        let result = init(Some("Test Registry".to_string()), None, None, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_invalid_path() {
        let result = register(
            "test".to_string(),
            Some("nonexistent.fasta".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        );
        assert!(result.is_err());
    }
}
