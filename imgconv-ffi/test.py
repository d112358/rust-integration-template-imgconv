import ctypes
import sys
from pathlib import Path

# ---- Check command line arguments ----
if len(sys.argv) == 3:
    input_path = sys.argv[1]
    output_path = sys.argv[2]
else:
    print("Usage: python test.py <input_path> <output_path>")
    sys.exit(1)

# ---- Load the shared library ----
if sys.platform.startswith("win"):
    libname = "imgconv_ffi.dll"
elif sys.platform == "darwin":
    libname = "libimgconv_ffi.dylib"
else:  # Linux and others
    libname = "libimgconv_ffi.so"

libpath = Path(__file__).parent.parent / "target" / "debug" / libname

ffi = ctypes.CDLL(libpath)

# ---- Define function signatures ----
# int convert_image_c(const char* input, const char* output);
ffi.convert_image_c.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
ffi.convert_image_c.restype = ctypes.c_int

# ---- Test conversion ----
input_file = input_path.encode('utf-8')
output_file = output_path.encode('utf-8')
result = ffi.convert_image_c(input_file, output_file)

if result == 0:
    print(f"Success: converted {input_file.decode()} -> {output_file.decode()}")
else:
    print(f"Error: convert_image_c returned {result}")