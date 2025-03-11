# Nofib

## Progress 

| Dir       | Benchmark             | Loc   | Impl  | matches MLScript  | Notes                             |
| --------- | --------------------- | ----- | ----- | ----------------- | --------------------------------- |
| gc        | constraints           | 270   | X     | X                 |
| spectral  | fish                  | 135   | X     | X                 |
| spectral  | cryptarithm1          | 168   | X     |                   |
| spectral  | gcd                   | 60    | X     |                   |
| spectral  | lcss                  | 61    | X     |                   |
| spectral  | integer               | 68    | X     |                   | 
| gc        | cacheprof             | 2100  | -     |                   | requires Strings                  |
| gc        | circsim               | 670   | -     |                   |
| gc        | fibheaps              | 300   | -     |                   |
| gc        | gc_bench              | 300   | -     |                   |
| gc        | fulsom                | 1392  | -     |                   | requires floats                   |
| gc        | happy                 | 27000 | -     |                   | requires strings                  |
| gc        | hash                  | 600   | -     |                   | uses arrays                       |
| gc        | lcss                  | 60    | -     |                   | spectral lcss                     |
| gc        | linear                | 2100  | -     |                   | requires floats                   |
| gc        | mutstore1             | 34    | -     |                   | requires references               |
| gc        | mutstore2             | 31    | -     |                   |
| gc        | power                 | 141   | -     |                   | requires floats                   |
| gc        | spellcheck            | 16    | -     |                   | requires strings and file io      |
| gc        | treejoin              | 121   | -     |                   | requires strings and file io      |
| imaginary | bernouilli            | 70    | -     |                   |
| imaginary | digits-of-e1          | 76    | -     |                   |
| imaginary | digits-of-e2          | 91    | -     |                   |
| imaginary | exp3_8                | 93    | -     |                   |
| imaginary | gen_regexps           | 73    | -     |                   | requires strings                  |
| imaginary | integrate             | 43    | -     |                   | requires floats                   |
| imaginary | kahan                 | 65    | -     |                   | requires floats                   |
| imaginary | paraffins             | 92    | -     |                   | uses arrays                       |
| imaginary | primes                | 16    | -     |                   | same as manticore primes          |
| imaginary | queens                | 19    | -     |                   | same as manticore queens          |
| imaginary | rfib                  | 12    | -     |                   | same as manticore fib             |
| imaginary | tak                   | 16    | -     |                   | same as manticore tak             |
| imaginary | wheel-sieve1          | 49    | -     |                   |
| imaginary | wheel-sieve2          | 53    | -     |                   |
| imaginary | x2n1                  | 35    | -     |                   | requires floats                   |
| parallel  | blackscholes          | 232   | -     |                   | requires floats                   |
| parallel  | cfd                   | 1106  | -     |                   | requires floats                   |
| parallel  | coins                 | 203   | -     |                   |
| parallel  | gray                  | 2457  | -     |                   | requires strings and floats       |
| parallel  | matmult               | 287   | -     |                   |
| parallel  | nbody                 | 150   | -     |                   | without parallel same as n-body   |
| parallel  | parfib                | 29    | -     |                   | without parallel same as rfib     |
| parallel  | partree               | 108   | -     |                   |
| parallel  | queens                | 28    | -     |                   | without parallel same as queens   |
| parallel  | ray                   | 263   | -     |                   | requires floats                   |
| parallel  | threadfib             | 263   | -     |                   | without parallel same as rfib     |
| parallel  | warshall              | 178   | -     |                   | 
| parallel  | dcbm                  | 610   | -     |                   | 
| parallel  | linsolv               | 870   | -     |                   | 
| parallel  | mandel                | 406   | -     |                   | without parallel same as mandel   |
| parallel  | minimax               | 324   | -     |                   | without parallel same as minimax  |
| parallel  | partak                | 24    | -     |                   | without parallel same as tak      |
| parallel  | prsa                  | 93    | -     |                   | without parallel same as rsa      |
| parallel  | quicksort             | 96    | -     |                   | same as manticore quicksort       |
| parallel  | sumeuler              | 367   | -     |                   | 
| parallel  | transclos             | 272   | -     |                   | 
| real      | anna                  | 9591  | -     |                   | requires strings                  |
| real      | cacheprof             | 2158  | -     |                   | same as gc cacheprof              |
| real      | compress2             | 237   | -     |                   | requires strings                  |
| real      | fem                   | 1371  | -     |                   | requires strings                  |
| real      | fulsom                | 1418  | -     |                   | requires floats                   |
| real      | gg                    | 844   | -     |                   | requires strings                  |
| real      | hidden                | 557   | -     |                   | requires floats                   |
| real      | infer                 | 623   | -     |                   | requires strings                  |
| real      | linear                | 2168  | -     |                   | requires floats                   |
| real      | parser                | 1412  | -     |                   | requires strings                  |
| real      | prolog                | 678   | -     |                   | requires strings                  |
| real      | rsa                   | 110   | -     |                   | requires strings and file io      |
| real      | symalg                | 1178  | -     |                   | requires strings                  |
| real      | bspt                  | 2173  | -     |                   | 
| real      | compress              | 773   | -     |                   | requires strings                  |
| real      | eff                   | 436   | -     |                   | 
| real      | fluid                 | 2439  | -     |                   | requires floats                   |
| real      | gamteb                | 731   | -     |                   | requires floats                   |
| real      | grep                  | 392   | -     |                   | requires strings                  |
| real      | hpg                   | 2102  | -     |                   | requires strings                  |
| real      | lift                  | 2060  | -     |                   | requires strings                  |
| real      | maillist              | 176   | -     |                   | requires strings and file io      |
| real      | mkhprog               | 838   | -     |                   | requires strings                  |
| real      | pic                   | 559   | -     |                   | requires floats                   |
| real      | reptile               | 1557  | -     |                   | requires chars                    |
| real      | scs                   | 586   | -     |                   | requires strings                  |
| real      | veritas               | 11164 | -     |                   | requires strings                  |
| shootout  | binary-trees          | 77    | -     |                   | 
| shootout  | fasta                 | 59    | -     |                   | requires strings                  |
| shootout  | pidigits              | 23    | -     |                   |
| shootout  | reverse-complement    | 90    | -     |                   | requires ffi and strings          |
| shootout  | fannkuch-redux        | 107   | -     |                   | uses pointers                     |
| shootout  | k-nucleotide          | 330   | -     |                   | requires strings                  |
| shootout  | n-body                | 189   | -     |                   | requires floats                   |
| shootout  | spectral-norm         | 112   | -     |                   | requires floats                   |
| smp       | callback001           | 43    | -     |                   | requires ffi                      |
| smp       | chan                  | 22    | -     |                   | requires channels                 |
| smp       | sieve                 | 29    | -     |                   | without parallel same as primes   |
| smp       | stm001                | 62    | -     |                   | requires channels                 |
| smp       | systolic              | 44    | -     |                   | requires channels, floats and rng |
| smp       | threads001            | 26    | -     |                   | requires threads                  |
| smp       | threads003            | 37    | -     |                   | requires threads                  |
| smp       | threads005            | 114   | -     |                   | requires threads                  |
| smp       | threads007            | 277   | -     |                   | requires threads                  |
| smp       | callback002           | 28    | -     |                   | requires ffi                      |
| smp       | stm002                | 164   | -     |                   | requires channels                 |
| smp       | tchan                 | 22    | -     |                   | requires channels                 |
| smp       | threads002            | 36    | -     |                   | requires threads                  |
| smp       | threads004            | 32    | -     |                   | requires threads                  |
| smp       | threads006            | 66    | -     |                   | requires threads                  |
| spectral  | ansi                  | 128   | -     |                   | requires strings                  |
| spectral  | boyer2                | 731   | -     |                   | requires strings                  |
| spectral  | constraints           | 272   | -     |                   |
| spectral  | eliza                 | 273   | -     |                   | requires strings                  |
| spectral  | lambda                | 276   | -     |                   | requires strings                  |
| spectral  | mandel                | 495   | -     |                   | requires floats                   |
| spectral  | para                  | 1785  | -     |                   | requires strings                  |
| specrral  | rewrite               | 636   | -     |                   | requires strings                  |
| spectral  | sphere                | 453   | -     |                   | requires floats                   |
| spectral  | atom                  | 188   | -     |                   | requires floats                   |
| spectral  | calendar              | 142   | -     |                   | requires strings                  |
| specrral  | exact-reals           | 225   | -     |                   |
| spectral  | last-piece            | 235   | -     |                   | requires strings                  |
| spectral  | mandel2               | 226   | -     |                   | requires floats                   |
| spectral  | power                 | 149   | -     |                   | requires floats                   |
| spectral  | scc                   | 100   | -     |                   | requires strings and file io      |
| spectral  | treejoin              | 122   | -     |                   | requires strings and file io      |
| spectral  | awards                | 116   | -     |                   | requires strings                  |
| spectral  | cichelli              | 240   | -     |                   | requires strings                  |
| spectral  | cryptarithm2          | 132   | -     |                   | requires strings                  |
| spectral  | expert                | 525   | -     |                   | requires strings and file io      |
| spectral  | hartel                | 13241 | -     |                   | requires floats                   |
| spectral  | mate                  | 425   | -     |                   | requires strings                  |
| spectral  | pretty                | 265   | -     |                   | requires strings                  |
| specrral  | secretary             | 75    | -     |                   | requires floats                   |
| specral   | banner                | 110   | -     |                   | requires strings                  |
| spectral  | circsim               | 670   | -     |                   | requires strings                  |
| spectral  | cse                   | 472   | -     |                   | requires strings                  |
| spectral  | fft2                  | 221   | -     |                   | requires floats                   |
| spectral  | life                  | 55    | -     |                   | almost exactly manticore life     |
| spectral  | minimax               | 244   | -     |                   | same as manticore minimax         |
| spectral  | primetest             | 305   | -     |                   | requires strings and file io      |
| spectral  | simple                | 1124  | -     |                   | 
| spectral  | boyer                 | 1021  | -     |                   |
| spectral  | clausify              | 185   | -     |                   | requires strings                  |
| spectral  | dom-lt                | 633   | -     |                   | requires strings and file io      |
| spectral  | fibheaps              | 304   | -     |                   | same as gc fibheaps               |
| spectral  | knights               | 886   | -     |                   | requires chars                    |
| spectral  | multiplier            | 501   | -     |                   | requires strings                  |
| spectral  | puzzle                | 175   | -     |                   | uses strings                      |
| spectral  | sorting               | 200   | -     |                   | requires strings                  | 

# Manticore 

## Missing features for benchmarks

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

## Not implemented

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

## Benchmarks progress

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

## WIP

| Benchmark             | Compiles  | matches Manticore | Tested | Args | Notes                            |
| --------------------- | --------- | ----------------- | ------ | ---- | -------------------------------- |
| Quicksort             | -         | -                 | -      | -    | requires ropes                   |
| Mandelbrot            | -         | -                 | -      | -    | requires floats and arrays       |
| Mazefun               | -         | -                 | -      | -    | requires strings                 |
| Mcray                 | -         | -                 | -      | -    | requires floats and rng          |
