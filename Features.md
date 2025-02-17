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

| Benchmark             | Compiles  | matches Manticore | Tested | Args | Notes                            |
| --------------------- | --------- | ----------------- | ------ | ---- | -------------------------------- |
| Ack                   | X         | X                 | X      | X    |                                  |
| AckGoto               | X         | X                 | X      | X    |                                  |
| Cpstak                | X         | X                 | X      | X    |                                  | 
| Evenodd               | X         | X                 | X      | X    | args differ from evenodd         |
| Fib                   | X         | X                 | X      | X    |                                  |
| Motzkin               | X         | X                 | X      | X    |                                  |
| MotzkinGoto           | X         | X                 | X      | X    |                                  |
| Primes                | X         | X                 | X      | X    |                                  |
| Sudan                 | X         | X                 | X      | X    |                                  |
| SudanGoto             | X         | X                 | X      | X    |                                  |
| Life                  | X         | X                 | X      | X    |                                  |
| Merge                 | X         | -                 | X      | X    | runtime errors                   |
| Deriv                 | X         | -                 | X      | X    | runtime errors                   |
| Divrec                | X         | -                 | X      | X    | runtime errors                   |
| Takl                  | X         | X                 | -      | X    | runtime errors                   |
|                       |           |                   |        |      | less than 5ms with adjusted args |
| Perm                  | X         | X                 | -      | X    | segfault                         |
| Nqueens               | X         | X                 | -      | X    | segfault                         |
| EraseUnused           | X         | N/A               | X      | -    | hyperfine non-zero               |
| EvenoddGoto           | X         | X                 | X      | X    | hyperfine non-zero               |
| SumRange              | X         | N/A               | X      | -    | hyperfine non-zero               | 
| FactorialAccumulator  | X         | N/A               | X      | -    | hyperfine non-zero for n>13      |
| FibonacciRecursive    | X         | N/A               | X      | -    | hyperfine non-zero               |
| IterateIncrement      | X         | N/A               | X      | -    | hyperfine non-zero               |
| LookupTree            | X         | N/A               | X      | -    | hyperfine non-zero               |
| TailFib               | X         | X                 | -      | X    | less than 5ms with adjusted args |
| Tak                   | X         | X                 | X      | X    | less than 5ms with adjusted args |
| TakGoto               | X         | X                 | -      | X    | less than 5ms with adjusted args |
| Mandelbrot            | -         | -                 | -      | -    | requires floats and arrays       |
| Mazefun               | -         | -                 | -      | -    | requires strings                 |
| Mcray                 | -         | -                 | -      | -    | requires floats and rng          | 
| Minimax               | -         | -                 | -      | -    | requires arrays                  |
