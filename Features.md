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
| Ack                   | X         | X                 | 
| AckGoto               | X         | X                 |
| Cpstak                | X         | X                 |
| Deriv                 | X         | -                 | - | | | requires runtime errors |
| Divrec                | X         | -                 | - | | | requires runtime errors |
| Evenodd               | X         | X                 | 
| EvenoddGoto           | X         | X                 |
| EraseUnused           | X         | N/A               |
| FactorialAccumulator  | X         | N/A               |
| Fib                   | X         | X                 |
| FibonacciRecursive    | X         | N/A               |
| IterateIncrement      | X         | N/A               |
| Life                  | X         | X                 |
| LookupTree            | X         | N/A               |
| Mandelbrot            | -         | -                 | - | - | requires floats and arrays |
| Mazefun               | -         | -                 | - | - | requires strings           |
| Mcray                 | -         | -                 | - | - | requires floats and rng    | 
| Merge                 | X         | -                 | - | - | requires runtime errors |
| Minimax               | -         | -                 | - | - | requires arrays
| Motzkin               | X         | X                 |
| MotzkinGoto           | X         | X                 |
| Nqueens               | X         | X                 |
| Perm                  | X         | X                 |
| Primes                | X         | X                 |
| Sudan                 | X         | X                 |
| SudanGoto             | X         | X                 |
| SumRange              | X         | N/A               |
| TailFib               | X         | X                 |
| Tak                   | X         | X                 |
| TakGoto               | X         | X                 |
| Takl                  | X         | X                 | - | - | requires runtime errors |
