"""
A Python interface for managing genomic reference data files.

RefMan provides a set of tools and APIs for organizing, validating, and accessing genomic
reference datasets. It enables efficient management of FASTA, GenBank, GFA, GFF, GTF, and
BED files through a registry system.

This module provides both class-based and function-based interfaces for:
- Initializing reference data registries
- Registering reference datasets with associated genomic files
- Listing registered datasets
- Downloading registered datasets
- Removing datasets from the registry

Classes:
    RefDataset: Represents a reference dataset containing genomic data files
    RegistryOptions: Configuration options for initializing a reference dataset registry
    RefmanProject: Represents a reference registry project

Functions:
    init: Initialize a new RefMan project registry
    register: Register a new reference dataset
    list_datasets: List registered reference datasets
    download: Download a registered reference dataset
    remove: Remove a dataset from the registry
"""

class RefDataset:
    """
    A reference dataset containing genomic data files.

    Methods:
        try_new: Creates a new reference dataset.
        label: Returns the dataset label.
        fasta: Returns the associated FASTA file URL.
        genbank: Returns the associated Genbank file URL.
        gfa: Returns the associated GFA file URL.
        gff: Returns the associated GFF file URL.
        gtf: Returns the associated GTF file URL.
        bed: Returns the associated BED file URL.
    """

    @staticmethod
    def try_new(
        title: str | None = None,
        description: str | None = None,
        requested_path: str | None = None,
        global_project: bool = False,
    ) -> "RefDataset":
        """ """
        ...

    def label(self) -> str:
        """
        Return the label for a reference dataset.
        """
        ...

    def fasta(self) -> str | None:
        """
        Return a FASTA file URL registered with a reference dataset, if available.
        """
        ...

    def genbank(self) -> str | None:
        """
        Return a Genbank file URL registered with a reference dataset, if available.
        """
        ...

    def gfa(self) -> str | None:
        """
        Return a GFA file URL registered with a reference dataset, if available.
        """
        ...

    def gff(self) -> str | None:
        """
        Return a GFF file URL registered with a reference dataset, if available.
        """
        ...

    def gtf(self) -> str | None:
        """
        Return a GTF file URL registered with a reference dataset, if available.
        """
        ...

    def bed(self) -> str | None:
        """
        Return a BED file URL registered with a reference dataset, if available.
        """
        ...

class RegistryOptions:
    """
    Configuration options for initializing a reference dataset registry, which includes resolving the file path to the `refman.toml` used to cache validated URLs on disk.

    Methods:
        new: Creates a new RegistryOptions instance.
        init_project: Initializes a new registry project.
        read_registry: Reads an existing registry and returns a RefmanProject.
        write_registry: Writes the current registry state to disk.
    """

    @staticmethod
    def new(
        title: str | None = None,
        description: str | None = None,
        requested_path: str | None = None,
        global_project: bool = False,
    ) -> RegistryOptions:
        """
        Create a new instance of `RegistryOptions` while validating that the path to the `refman.toml` file exists under the hood.
        """
        ...

    def init_project(self):
        """
        Create an `refman.toml` based on the metadata in the registry options that does not yet contain any registered datasets.
        """
        ...

    def read_registry(self) -> "RefmanProject":
        """
        Use the resolved `refman.toml` file path to read project data into memory so it
        can be used or manipulated.
        """
        ...

    def write_registry(self, project: RefmanProject) -> None:
        """
        Use the registry options and an instance of a RefMan project to write information to the `refman.toml`.
        """
        ...

class RefmanProject:
    """
    Represents a reference registry project.

    Methods:
        new: Creates a new RefmanProject.
        datasets: Returns a list of registered reference datasets.
        get_dataset: Retrieves a specific dataset by label.
        get_dataset_urls: Returns a list of URLs for a given dataset.
        is_registered: Checks if a dataset with the given label is registered.
        register: Registers a new dataset and returns an updated project.
        read_registry: Reads an existing registry and returns a RefmanProject.
        write_registry: Writes the current registry state to disk.
    """
    @staticmethod
    def new(
        title: str | None = None,
        description: str | None = None,
        global_dataset: bool = False,
    ) -> "RefmanProject":
        """
        Create a new RefmanProject instance with an empty reference dataset registry.

        Args:
            title: Optional title for the registry.
            description: Optional description.
            global_dataset: Whether to create a global registry.

        Returns:
            A new RefmanProject instance.
        """
        ...

    def get_datasets(self) -> list[RefDataset]:
        """
        Return a list of registered reference datasets.
        """
        ...

    def get_dataset_urls(self) -> list[str]:
        """
        Return a list of URLs associated with a reference dataset.

        Returns:
            A list of URLs associated with the reference datasets.
        """
        ...

    def register(
        self,
        label: str,
        fasta: str | None = None,
        genbank: str | None = None,
        gfa: str | None = None,
        gff: str | None = None,
        gtf: str | None = None,
        bed: str | None = None,
    ) -> "RefmanProject":
        """
        Register a new reference dataset with a RefMan project.

        Args:
            label: Identifier for the dataset.
            fasta: Path to the FASTA file.
            genbank: Path to the GenBank file.
            gfa: Path to the GFA file.
            gff: Path to the GFF file.
            gtf: Path to the GTF file.
            bed: Path to the BED file.

        Returns:
            An updated RefmanProject instance.
        """
        ...

    @staticmethod
    def read_registry(
        global_dataset: bool = False,
        title: str | None = None,
        description: str | None = None,
        requested_path: str | None = None,
    ) -> "RefmanProject":
        """
        Use individually specified project options to read an existing registry from disk into memory.

        Args:
            global_dataset: Whether to use a global registry.
            title: Optional title for the registry.
            description: Optional description.
            requested_path: Optional path to the registry file.

        Returns:
            A RefmanProject instance loaded from the registry.
        """
        ...
    @staticmethod
    def write_registry(
        project: "RefmanProject",
        global_dataset: bool = False,
        title: str | None = None,
        description: str | None = None,
        requested_path: str | None = None,
    ) -> None:
        """
        Write a RefmanProject to disk using individually specified project options.

        Args:
            project: The RefmanProject instance to write.
            global_dataset: Whether to use a global registry.
            title: Optional title for the registry.
            description: Optional description.
            requested_path: Optional path to the registry file.
        """
        ...

def init(
    title: str | None = None,
    description: str | None = None,
    global_project: bool = False,
) -> None:
    """
    Initialize a new RefMan project with metadata and an empty registry of datasets.

    Args:
        title: Optional title for the registry.
        description: Optional description.
        requested_path: Optional path for the registry.
        global_project: Whether to initialize a global registry.
    """
    ...

def register(
    label: str,
    fasta: str | None = None,
    genbank: str | None = None,
    gfa: str | None = None,
    gff: str | None = None,
    gtf: str | None = None,
    bed: str | None = None,
    registry: str | None = None,
    global_project: bool = False,
) -> None:
    """
    Register a new reference dataset with a RefMan project. This function is different
    from class-based register methods in this module in that it doesn't register the
    dataset to an in-memory instance of a project, and instead caches everything on
    disk.

    Args:
        label: Identifier for the dataset.
        fasta: Path to the FASTA file.
        genbank: Path to the GenBank file.
        gfa: Path to the GFA file.
        gff: Path to the GFF file.
        gtf: Path to the GTF file.
        bed: Path to the BED file.
        registry: Optional registry path.
        global_project: Whether to register in a global registry.
    """
    ...

def list_datasets(
    label: str | None = None,
    registry: str | None = None,
    global_project: bool = False,
) -> None:
    """
    List registered reference datasets.

    Args:
        label: Optional label to filter the list.
        registry: Optional registry path.
        global_project: Whether to list datasets from a global registry.
    """
    ...

def download(
    label: str | None = None,
    dest: str | None = None,
    registry: str | None = None,
    global_project: bool = False,
) -> None:
    """
    Download a reference dataset registered in `refman.toml`.

    Args:
        label: Identifier of the dataset to download.
        dest: Destination directory (defaults to the current directory if not specified).
        registry: Optional registry path.
        global_project: Whether to use a global registry.
    """
    ...

def remove(
    label: str | None = None,
    registry: str | None = None,
    global_project: bool = False,
) -> None:
    """
    Remove a registered reference dataset from the project using its label.

    Args:
        label: Identifier of the dataset to remove.
        registry: Optional registry path.
        global_project: Whether to operate on a global registry.
    """
    ...
