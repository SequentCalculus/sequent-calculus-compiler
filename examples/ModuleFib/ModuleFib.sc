//Fibonacci module test

import Subtraction

module Arith

public def fib(n: i64) : i64 {
    if n <= 0 {
        0
    }
    else {
        if n == 1 {
            1
        }
        else {
            Arith::add2(fib(Subtraction::sub3(n, 1)), fib(Subtraction::sub2(n, 2)))
            //Arith::add2(fib(n - 1), fib(n - 2))
            //fib(n - 1) + fib(n - 2)
            //fib(Subtraction::sub3(n, 1)) + fib(Subtraction::sub3(n, 2))
        }
    }
}

def main(n : i64) : i64 {
    println_i64(fib(n));
0}