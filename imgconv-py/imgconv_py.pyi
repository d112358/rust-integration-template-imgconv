__all__ = ["convert_image_py", "__version__"]

def convert_image_py(input: str, output: str) -> None:  # pylint: disable=redefined-builtin, unused-argument
    """
    Convert an image to another format based on file extension.
    
    Args:
        input: Path to the input image file.
        output: Path to the output image file.
    
    Raises:
        RuntimeError: If the conversion fails.
    """

__version__: str