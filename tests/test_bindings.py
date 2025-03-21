from pathlib import Path

import refman


def test_workflow_integration() -> None:
    refman.init("integration-test", "Temporary configuration to be used for integration testing", global_project=False)
    refman.register(
        "test1",
        fasta="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/MN908947.3.fasta",
    )
    refman.register(
        "test1",
        genbank="https://dholk.primate.wisc.edu/_webdav/dho/public/DHO%20Lab%20Bespoke%20Reference%20Dataset%20Registry/Pathogen%20Genomics/%40files/sars-cov-2/MN908947.3.gbk",
    )
    refman.download("test1")
    refman.remove("test1")

    return cleanup_datasets()


def cleanup_datasets() -> None:
    for file in Path.cwd().glob("*.fasta"):
        file.unlink()
    for file in Path.cwd().glob("*.gbk"):
        file.unlink()
    for file in Path.cwd().glob("*.gfa"):
        file.unlink()
    for file in Path.cwd().glob("*.gff"):
        file.unlink()
    for file in Path.cwd().glob("*.gtf"):
        file.unlink()
    for file in Path.cwd().glob("*.bed"):
        file.unlink()
