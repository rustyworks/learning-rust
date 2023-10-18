import ctypes
import timeit
from functools import lru_cache


def rust_fibonacci_debug(n):
    lib = ctypes.CDLL("target/debug/libembedded_language.so")
    fib = lib.fibonacci
    fib.argtypes = [ctypes.c_uint]
    fib.restype = ctypes.c_uint

    return fib(n)


def rust_fibonacci_release(n):
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
    duration_rust_debug = timeit.timeit("rust_fibonacci_debug(20)", globals=globals(), number=10000)
    print(f"Rust Debug: {duration_rust_debug:>17.5f} seconds, Result: {rust_fibonacci_debug(20)}")

    duration_rust_release = timeit.timeit("rust_fibonacci_release(20)", globals=globals(), number=10000)
    print(f"Rust Release: {duration_rust_release:>15.5f} seconds, Result: {rust_fibonacci_release(20)}")

    duration_python = timeit.timeit("python_fibonacci(20)", globals=globals(), number=10000)
    print(f"Python: {duration_python:>21.5f} seconds, Result: {python_fibonacci(20)}")

    duration_python_with_cache = timeit.timeit("python_fibonacci_with_cache(20)", globals=globals(), number=10000)
    print(f"Python with cache: {duration_python_with_cache:>10.5f} seconds, Result: {python_fibonacci_with_cache(20)}")
