# Python Bindings for `RefMan`, a simple biological reference manager written in Rust

If you do bioinformatics, chances are your projects need a variety of different biological reference files. Depending on the input data you're running analyses on, these files may be required in a myriad of combinations, which can be difficult to keep track of. Worse, organizing these files can be overwhelming for beginners, or can even be impossible if the files aren't findable, which breaks the chain of scientific reproducibility.

`refman` offers one solution to this problem: manage all the combinations of reference datasets for your project(s) in one human-readable configuration file.

The `refman` Python interface is available on [PyPI](), so simply run `pip install py-refman` in the command line to get started.

## Basic Usage

`refman` centers around a workflow of three functions: `init()`, `register()`, and `download()`:

```python
import refman

# Initialize a project with some metadata, which can be used to register new datasets
refman.init(
    title="Demo Project",
    description="Demo of the Python RefMan interface for the py-refman README"
)

# Register a couple datasets in a couple of bioinformatic file formats with the
# `refman.toml`, which records the project's available datasets
refman.register(
    label="test1",
    fasta="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/MN908947.3.fasta",
    gff="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/MN908947.3_corrected_orf1.gff",
)

# Register a second dataset for an alternative configuration of the current project
refman.register(
    label="test2",
    genbank="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/MN908947.3.gbk",
    bed="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/qiaseq_direct_boosted.bed",
)

# Download just the first set
refman.download("test1")

```

(These examples come from [our own](https://dho.pathology.wisc.edu/) [internal registry of bioinformatic resources](https://dholk.primate.wisc.edu/project/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/begin.view))

More extensive API documentation will be available soon!

## The Command-Line Interface

`refman` was originally a command-line interface. If you're interested in using it there instead of in Python, [see the original `refman` repository](https://github.com/nrminor/refman) for installation and usage docs. These docs also include more detailed usage instructions for the command-line interface, which is closely (though by choice not fully) mirrored in the Python interface.

