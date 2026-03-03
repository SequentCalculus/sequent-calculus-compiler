//Fibonacci module test
module arith

import subtraction

public def fib(n : i64) : i64 {
    if n <= 0 {
        0
    }
    else {
        if n == 1 {
            1
        }
        else {
            arith::add2(fib(subtraction::sub2(n, 1)), fib(subtraction::sub2(n, 2)))
        }
    }
}

def main(n : i64) : i64 {
    println_i64(fib(n));
0}