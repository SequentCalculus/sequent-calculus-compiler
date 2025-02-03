def fibonacci(i: i64): i64 { ifz(i) {i} else {ife(i, 1) {i} else {fibonacci(i - 1) + fibonacci(i - 2)}} }

def main(n: i64): i64 { println_i64(fibonacci(n));
                        0 }
