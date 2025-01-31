# Missing features for benchmarks

| Feature                       | Required for              | Optionally Required for                  |
| ----------------------------- | ------------------------- | ---------------------------------------- |
| `main` without return         |                           | `merge`, `divrec`, `primes`              |
| Runtime Errors                |                           | `divrec`                                 |
| Term-Level Recursion          | `primes`                  | `motzkin`, `motzkingoto`,`mandelbrot`    |
| Arrays                        | `quicksort`,`mandelbrot`  |                                          |
| ifg/ifle                      | `primes`                  | `merge`                                  |
| long int                      | `sudan`                   |                                          |
| floats                        | `mandelbrot`              |                                          |
| Global Constants              |                           | `mandelbrot`                             |
| Polymorphism                  |                           | `mandelbrot`                             |
| Channels                      | `cml_pingpong`,`cml_ring` |                                          |
|                               | `cml_spawn`, `ec_cml_*`   |                                          | 
| FFI                           | `ffi_fib`, `ffi_trigfib`  |                                          |

# Not implemented 

So far, the following benchmarks are missing 

* `quicksort`, cannot work / is not comparable without some kind of array structure
* `mandelbrot`, requires both arrays and floats (pseudocode implementation available)
* `cml_pingpong, cml_ring`, `cml_spawn` and their corresponding `call/ec` implementations, all using channels
* `ffi_fib`, `ffi_trigfib`, use ffi calls
