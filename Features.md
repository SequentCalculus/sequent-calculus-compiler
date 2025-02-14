# Missing features for benchmarks

| Feature                       | Required for              | Optionally Required for                  |
| ----------------------------- | ------------------------- | ---------------------------------------- |
| `main` without return         |                           | `merge`, `divrec`, `primes`,`life`       |
| Runtime Errors                | `deriv`,`scc`             | `divrec`,`minimax`,`mc_ray`              |
| Term-Level Recursion          | `primes`                  | `motzkin`, `motzkingoto`,`mandelbrot`    |
|                               |                           | `life`                                   |
| Arrays                        | `quicksort`,`mandelbrot`  |                                          |
|                               | `minimax`                 |                                          |
| long int                      | `sudan`                   |                                          |
| floats                        | `mandelbrot`, `barnes_hut`|                                          |
|                               | `mc_ray`                  |                                          | 
| Global Constants              |                           | `mandelbrot`,`life`,`minimax`            |
| Polymorphism                  | `mazefun`                 | `mandelbrot`                             |
| Channels                      | `cml_pingpong`,`cml_ring` |                                          |
|                               | `cml_spawn`, `ec_cml_*`   |                                          | 
| FFI                           | `ffi_fib`, `ffi_trigfib`  |                                          |
| Deep pattern matching         |                           | `deriv`                                  |
| Wildcard matching             |                           | `deriv`                                  | 
| Primitive Booleans            |                           | `deriv`,`evenodd`,`takl`,`life`,`minimax`| 
| Strings                       | `scc`                     | `mazefun`                                |
| Random Number generation      | `mc_ray`                  |                                          |
| File IO                       | `scc`                     |                                          |

# Not implemented 

So far, the following benchmarks are missing 

* `quicksort`, cannot work / is not comparable without some kind of array structure
* `mandelbrot`, requires both arrays and floats (pseudocode implementation available)
* `barnes_hut`, requires floats, possibly arrays as well
* `cml_pingpong, cml_ring`, `cml_spawn` and their corresponding `call/ec` implementations, all using channels
* `ffi_fib`, `ffi_trigfib`, use ffi calls
* `scc` uses both strings and file io
* `minimax` uses two different versions of minimax, `minimax` and `minimax_trans`
    regular `minimax` is implemented, but `minimax_trans` uses array functions

# Benchmarks progress 

| Benchmark             | Compiles  | matches Manticore | Tested | Adjusted Args | Notes |
| --------------------- | --------- | ----------------- | ------ | ------------- | ----- |
| Ack                   | X         | X                 | X      |
| AckGoto               | X         | X                 | X      |
| Cpstak                | X         | X                 | X      |
| Deriv                 | X         | -                 | -      | | | runtime errors, will test with more prints |
| Divrec                | X         | -                 | X      | | | runtime errors |
| Evenodd               | X         | X                 | X      | | |  |
| EvenoddGoto           | X         | X                 | X      | | |  |
| EraseUnused           | X         | N/A               | X      | 
| FactorialAccumulator  | X         | N/A               | X      |
| Fib                   | X         | X                 | X      |
| FibonacciRecursive    | X         | N/A               | X      |
| IterateIncrement      | X         | N/A               | X      |
| Life                  | X         | X                 | -      | | | | will test with more prints
| LookupTree            | X         | N/A               | X      | 
| Merge                 | X         | -                 | X      | - | runtime errors |
| Motzkin               | X         | X                 | X      |
| MotzkinGoto           | X         | X                 | X      |
| Nqueens               | X         | X                 | -      |   | result is always 0
| Perm                  | X         | X                 | X      |
| Primes                | X         | X                 | X      |
| Sudan                 | X         | X                 | X      |
| SudanGoto             | X         | X                 | X      |
| SumRange              | X         | N/A               | X      |
| TailFib               | X         | X                 | X      |
| Tak                   | X         | X                 | X      |
| TakGoto               | X         | X                 | X      |
| Takl                  | X         | X                 | -      | - | runtime errors, causes segfault |
| Mandelbrot            | -         | -                 | -      | - | requires floats and arrays |
| Mazefun               | -         | -                 | -      | - | requires strings           |
| Mcray                 | -         | -                 | -      | - | requires floats and rng    | 
| Minimax               | -         | -                 | -      | - | requires arrays
