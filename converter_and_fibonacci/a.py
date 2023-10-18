from functools import lru_cache

@lru_cache
def factorial(n):
    if n == 1:
        return 1
    else:
        return n + (factorial (n - 1))

print(factorial(1000))
