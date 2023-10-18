import ctypes
import timeit
from functools import lru_cache


def rust_fibonacci(n):
    lib = ctypes.CDLL("target/release/libembedded_language.so")
    fib = lib.fibonacci
    fib.argtypes = [ctypes.c_uint]
    fib.restype = ctypes.c_uint

    return fib(n)


def python_fibonacci(n):
    match n:
        case 0:
            return 1
        case 1:
            return 1
        case _:
            return python_fibonacci(n - 1) + python_fibonacci(n - 2)


@lru_cache(1)
def python_fibonacci_with_cache(n):
    match n:
        case 0:
            return 1
        case 1:
            return 1
        case _:
            return python_fibonacci_with_cache(n - 1) + python_fibonacci_with_cache(n - 2)

if __name__ == "__main__":
    duration_rust = timeit.timeit("rust_fibonacci(20)", globals=globals(), number=10000)
    print(f"Rust: {duration_rust:>23.5f} seconds")
    # print(rust_fibonacci(100))
    duration_python = timeit.timeit("python_fibonacci(20)", globals=globals(), number=10000)
    print(f"Python: {duration_python:>21.5f} seconds")
    # print(python_fibonacci(100))
    duration_python_with_cache = timeit.timeit("python_fibonacci_with_cache(20)", globals=globals(), number=10000)
    print(f"Python with cache: {duration_python_with_cache:>10.5f} seconds")
    # print(python_fibonacci_with_cache(100))
