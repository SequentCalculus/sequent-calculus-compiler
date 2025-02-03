# Missing features for benchmarks

| Feature                       | Required for              | Optionally Required for                  |
| ----------------------------- | ------------------------- | ---------------------------------------- |
| `main` without return         |                           | `merge`, `divrec`, `primes`,`life`       |
| Runtime Errors                | `deriv`                   | `divrec`,`minimax`                       |
| Term-Level Recursion          | `primes`                  | `motzkin`, `motzkingoto`,`mandelbrot`    |
|                               |                           | `life`                                   |
| Arrays                        | `quicksort`,`mandelbrot`  |                                          |
| ifg/ifle                      | `primes`                  | `merge`                                  |
| long int                      | `sudan`                   |                                          |
| floats                        | `mandelbrot`, `barnes_hut`|                                          |
| Global Constants              |                           | `mandelbrot`,`life`,`minimax`            |
| Polymorphism                  | `mazefun`                 | `mandelbrot`                             |
| Channels                      | `cml_pingpong`,`cml_ring` |                                          |
|                               | `cml_spawn`, `ec_cml_*`   |                                          | 
| FFI                           | `ffi_fib`, `ffi_trigfib`  |                                          |
| Deep pattern matching         |                           | `deriv`                                  |
| Wildcard matching             |                           | `deriv`                                  | 
| Primitive Booleans            |                           | `deriv`,`evenodd`,`takl`,`life`,`minimax`| 
| Strings                       |                           | `mazefun`

# Not implemented 

So far, the following benchmarks are missing 

* `quicksort`, cannot work / is not comparable without some kind of array structure
* `mandelbrot`, requires both arrays and floats (pseudocode implementation available)
* `barnes_hut`, requires floats, possibly arrays as well
* `cml_pingpong, cml_ring`, `cml_spawn` and their corresponding `call/ec` implementations, all using channels
* `ffi_fib`, `ffi_trigfib`, use ffi calls
