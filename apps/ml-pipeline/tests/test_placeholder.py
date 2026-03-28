from ml_pipeline import placeholder


def test_placeholder() -> None:
    assert placeholder() == "ml-pipeline"
