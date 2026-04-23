codata Stream { apply(x: i64): i64 }

def main() : i64 {
  let i: i64 = (println_i64(0); new { apply(x) => x }).apply(print_i64(1); 42);
  (label a { println_i64(0); new { apply(x) => x } }).apply(print_i64(1); 42)
}
