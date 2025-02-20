# Missing features for benchmarks

| Feature                       | Required for               | Optionally Required for                |
| ----------------------------- | -------------------------- | -------------------------------------- |
| Arrays                        | `mandelbrot`, `nbody`      | `quicksort`, `minimax`                 |
| Floats                        | `mandelbrot`, `barnes_hut` |                                        |
|                               | `mc_ray`, `nbody`          |                                        |
| Strings                       | `scc`, `boyer`, `mazefun`  |                                        |
| Deep pattern matching         |                            | `deriv`                                |
| Wildcard matching             |                            | `deriv`                                |
| Primitive Booleans (for if)   |                            | `deriv`, `evenodd`, `takl`             |
|                               |                            | `life`, `minimax`                      |
| Global constants              |                            | `mandelbrot`, `life`, `minimax`        |
| Type synonyms                 |                            | `life`, `minimax`                      |
| Term-level recursion          |                            | `motzkin`, `motzkingoto`, `mandelbrot` |
|                               |                            | `life`, `primes`                       |
| Runtime errors                | `scc`                      | `divrec`, `minimax`, `mc_ray`, `deriv` |
| File IO                       | `scc`                      |                                        |
| Mutable references            | `zebra`                    |                                        |
| Random number generation      | `mc_ray`                   |                                        |
| Channels                      | `cml_pingpong`, `cml_ring` |                                        |
|                               | `cml_spawn`, `ec_cml_*`    |                                        |
| FFI                           | `ffi_fib`, `ffi_trigfib`   |                                        |

# Not implemented

So far, the following benchmarks are missing

* `quicksort` uses ropes (which are hard to implement); usually it is done with arrays
* `mandelbrot` requires both arrays and floats (pseudocode implementation available)
* `barnes_hut` requires floats, possibly arrays as well
* `cml_pingpong, cml_ring`, `cml_spawn` and their corresponding `call/ec` implementations, all using channels
* `ffi_fib`, `ffi_trigfib` use ffi calls
* `scc` uses both strings and file io
* `zebra` uses mutable references (and excpetions which we could probably model with label/returnTo)
* `nbody` requires both floats and arrays
* `minimax` uses two different versions of minimax, `minimax` and `minimax_trans`
    regular `minimax` is implemented, but `minimax_trans` uses array functions, so we leave the latter out

# Benchmarks progress

| Benchmark             | Compiles  | matches Manticore | Tested | Args | Notes                            |
| --------------------- | --------- | ----------------- | ------ | ---- | -------------------------------- |
| Ack                   | X         | X                 | X      | X    |                                  |
| AckGoto               | X         | X                 | X      | X    |                                  |
| Cpstak                | X         | X                 | X      | X    |                                  |
| Evenodd               | X         | X                 | X      | X    | args differ from evenoddGoto     |
| EvenoddGoto           | X         | X                 | X      | X    |                                  |
| Fib                   | X         | X                 | X      | X    |                                  |
| Life                  | X         | X                 | X      | X    |                                  |
| Motzkin               | X         | X                 | X      | X    |                                  |
| MotzkinGoto           | X         | X                 | X      | X    |                                  |
| Primes                | X         | X                 | X      | X    |                                  |
| Sudan                 | X         | X                 | X      | X    |                                  |
| SudanGoto             | X         | X                 | X      | X    |                                  |
| TailFib               | X         | X                 | X      | X    |                                  |
| Tak                   | X         | X                 | X      | X    |                                  |
| TakGoto               | X         | X                 | X      | X    |                                  |
| Takl                  | X         | X                 | X      | X    | runtime errors, long runtime     |
| Merge                 | X         | -                 | X      | X    | runtime errors                   |
| Deriv                 | X         | -                 | X      | X    | runtime errors                   |
| Divrec                | X         | -                 | X      | X    | runtime errors                   |
| Perm                  | X         | X                 | X      | X    | requires 105MB heap              |
| Nqueens               | X         | X                 | X      | X    | requires 347MB heap              |
| Minimax               | X         | X                 | X      | X    | requires 450MB heap, no arrays   |
| EraseUnused           | X         | N/A               | X      | X    | added iters                      |
| SumRange              | X         | N/A               | X      | X    | added iters                      |
| FactorialAccumulator  | X         | N/A               | X      | X    | added iters                      |
| FibonacciRecursive    | X         | N/A               | X      | X    | added iters                      |
| IterateIncrement      | X         | N/A               | X      | X    | added iters                      |
| LookupTree            | X         | N/A               | X      | X    | added iters                      |

# WIP

| Benchmark             | Compiles  | matches Manticore | Tested | Args | Notes                            |
| --------------------- | --------- | ----------------- | ------ | ---- | -------------------------------- |
| Quicksort             | -         | -                 | -      | -    | requires ropes                   |
| Mandelbrot            | -         | -                 | -      | -    | requires floats and arrays       |
| Mazefun               | -         | -                 | -      | -    | requires strings                 |
| Mcray                 | -         | -                 | -      | -    | requires floats and rng          |
