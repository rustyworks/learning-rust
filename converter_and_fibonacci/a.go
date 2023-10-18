package main

import (
    "math/big"
)

func factorial(n int) *big.Int {
    if n == 1 {
        return big.NewInt(1)
    }
    return new(big.Int).Mul(big.NewInt(int64(n)), factorial(n-1))
}

func main() {
    n := 10000 // Replace with the desired value for n
    result := factorial(n)
    println(result.String())
}
