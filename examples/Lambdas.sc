codata Fun[A, B] { Apply(x: A) : B }

def nonValueArguments() : i64 { new { Apply(x) => new { Apply(y) => y}}.Apply[i64, Fun[i64,i64]](1 + 2).Apply[i64, i64](3 + 4) }

def higherOrder() : i64 {  new { Apply(x) => new { Apply(y) => x.Apply[i64, i64](y) }}.Apply[Fun[i64,i64], Fun[i64,i64]](new { Apply(z) => 4 + z}).Apply[i64, i64](3 + 1) }

def main() : i64 { println_i64(higherOrder());
                   0 }
