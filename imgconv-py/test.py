import sys
import imgconv_py

print(f"Imported imgconv_py version: {imgconv_py.__version__}")  # pylint: disable=no-member

if len(sys.argv) == 3:
    input_path = sys.argv[1]
    output_path = sys.argv[2]
    imgconv_py.convert_image_py(input_path, output_path)
    print(f"Converted {input_path} -> {output_path}")
else:
    print("Usage: python test.py <input_path> <output_path>")