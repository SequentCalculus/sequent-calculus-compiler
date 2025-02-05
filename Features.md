# Missing features for benchmarks

| Feature                       | Required for              | Optionally Required for                  |
| ----------------------------- | ------------------------- | ---------------------------------------- |
| `main` without return         |                           | `merge`, `divrec`, `primes`,`life`       |
| Runtime Errors                | `deriv`,`scc`             | `divrec`,`minimax`,`mc_ray`              |
| Term-Level Recursion          | `primes`                  | `motzkin`, `motzkingoto`,`mandelbrot`    |
|                               |                           | `life`                                   |
| Arrays                        | `quicksort`,`mandelbrot`  |                                          |
| ifg/ifle                      | `primes`                  | `merge`                                  |
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

# Benchmarks progress 

| Benchmark             | Compiles  | matches Manticore | Tested | Adjusted Args | Notes |
| --------------------- | --------- | ----------------- | ------ | ------------- | ----- |
| Ack                   | X         | 
| AckGoto               | X         |
| Cpstak                | X         |
| Deriv                 | X         |
| Divrec                | X         |
| Evenodd               | X         |
| EvenoddGoto           | X         |
| EraseUnused           | X         | 
| FactorialAccumulator  | X         |
| Fib                   | X         |
| FibonacciRecursive    | X         |
| IterateIncrement      | X         |
| Life                  | X         |
| LookupTree            | 
| Mandelbrot            | 
| MatchOptions          |
| Mazefun               | 
| Mcray                 | 
| Merge                 | 
| Minimax               |
| Motzkin               | 
| MotzkinGoto           | 
| Nqueens               | 
| Perm                  | 
| Primes                | 
| Sudan                 | 
| SudanGoto             | 
| SumRange              | 
| TailFib               |
| Tak                   |
| TakGoto               |
| Takl                  |
