# Nofib

## Progress 

| Dir       | Benchmark             | Loc   | Impl  | matches MLScript  | Notes                             |
| --------- | --------------------- | ----- | ----- | ----------------- | --------------------------------- |
| gc        | constraints           | 270   | X     | X                 |
| spectral  | fish                  | 135   | X     | X                 |
| spectral  | cryptarithm1          | 168   | X     | X                 |
| spectral  | gcd                   | 60    | X     | X                 |
| spectral  | lcss                  | 61    | X     | X                 |
| spectral  | integer               | 68    | X     | X                 | 
| spectral  | boyer                 | 1021  | -     |                   |

## Not implemented 

* Missing strings:
    * `gc/cacheprof`
    * `gc/happy`
    * `gc/spellcheck`
    * `gc/treejoin`
    * `imaginary/gen_regexps`
    * `parallel/gray`
    * `real/anna`
    * `real/compress2`
    * `real/fem`
    * `real/gg`
    * `real/real`
    * `real/prolog`
    * `real/infer`
    * `real/rsa`
    * `real/symalg`
    * `real/compress`
    * `real/grep`
    * `real/hpg`
    * `real/lift`
    * `real/maillist`
    * `real/mkhprog`
    * `real/scs`
    * `real/veritas`
    * `shootout/fasta`
    * `shootout/reverse-complement`
    * `shootout/k-nucleotide`
    * `smp/systolic`
    * `spectral/ansi`
    * `spectral/boyer2`
    * `spectral/eliza`
    * `spectral/lambda`
    * `spectral/para`
    * `spectral/rewrite`
    * `spectral/calendar`
    * `spectral/last-piece`
    * `spectral/scc`
    * `spectral/treejoin`
    * `spectral/awards`
    * `spectral/cichelli`
    * `spectral/cryptarithm2`
    * `spectral/expert`
    * `spectral/mate`
    * `spectral/pretty`
    * `spectral/banner`
    * `spectral/circsim`
    * `spectral/cse`
    * `spectral/primetest`
    * `spectral/clausify`
    * `spectral/dom-lt`
    * `spectral/multiplier`
    * `spectral/puzzle`
    * `spectral/sorting`
* Missing floats:
    * `gc/fulsom`
    * `gc/linear`
    * `gc/power`
    * `imaginary/integrate`
    * `imaginary/kahan`
    * `imaginary/tak`
    * `imaginary/x2n1`
    * `parallel/blackscholes`
    * `parallel/cfd`
    * `parallel/ray`
    * `real/fulsom`
    * `real/hidden`
    * `real/linear`
    * `real/fluid`
    * `real/gamteb`
    * `real/pic`
    * `shootout/n-body`
    * `shootout/spectral-norm`
    * `smp/systolic`
    * `spectral/mandel`
    * `spectral/sphere`
    * `spectral/atom`
    * `spectral/mandel2`
    * `spectral/power`
    * `spectral/hartel`
    * `spectral/secretary`
    * `spectral/fft2`
* Missing chars: 
    * `real/reptile`
    * `spectral/knights`
* Missing arrays:
    `gc/hash`
* Missing File IO:
    * `gc/spellcheck`
    * `gc/treejoin`
    * `real/rsa`
    * `real/maillist`
    * `spectral/scc`
    * `spectral/treejoin`
    * `spectral/expert`
    * `spectral/primetest`
    * `spectral/dom-lt`
* Missing FFI
    * `shootout/reverse-complement`
    * `smp/callback001`
    * `smp/callback002`
* Missing References:
    * `gc/mutstore1`
    * `shootout/fannkuch-redux`
* Missing arrays:
    * `imaginary/paraffins`
* Missing channels
    * `smp/chan`
    * `smp/stm001`
    * `smp/systolic`
    * `smp/stm002`
    * `smp/tchan`
* Missing threads: 
    * `smp/threads001`
    * `smp/threads002`
    * `smp/threads003`
    * `smp/threads004`
    * `smp/threads005`
    * `smp/threads006`
    * `smp/threads007`
* Not in MlScript:
    * `gc/circsim`
    * `gc/fibheaps`
    * `gc/gc-bench`
    * `gc/mutstore2`
    * `imaginary/bernouilli`
    * `imaginary/digits-of-e1`
    * `imaginary/digits-of-e2`
    * `imaginary/exp3_8`
    * `imaginary/wheel-sieve1`
    * `imaginary/wheel-sieve2`
    * `parallel/coins`
    * `parallel/gray`
    * `parallel/matmult`
    * `parallel/partree`
    * `parallel/warshall`
    * `parallel/dcbm`
    * `parallel/sumeuler`
    * `parallel/transclos`
    * `real/bspt`
    * `real/eff`
    * `shootout/binary-trees`
    * `shootout/pidigits`
    * `spectral/exact-reals`
    * `spectral/simple`
    * `spectral/exact-reals`
    * `spectral/simple`
* Same as 
    * `gc/lcss` = `spectral/lcss`
    * `imaginary/primes` = `manticore/primes`
    * `imaginary/queens` = `manticore/queens`
    * `imaginary/rfib` = `manticore/fib`
    * `parallel/quicksort` = `manticore/quicksort` (not implemented)
    * `real/cacheprof` = `gc/cacheprof`
    * `spectral/life` = `manticore/life`
    * `spectral/minimax` = `manticore/minimax`
    * `spectral/fibheaps` = `gc/fibheaps`
    * `spectral/constraints` = `gc/constraints`
* Missing Parallelism (same benchmark exists without parallel)
    * `parallel/nbody`
    * `parallel/parfib`
    * `parallel/queens`
    * `parallel/threadfib`
    * `parallel/linsolv`
    * `parallel/mandel`
    * `parallel/minimax`
    * `parallel/partak`
    * `parallel/prsa`
    * `smp/sieve`
    
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
