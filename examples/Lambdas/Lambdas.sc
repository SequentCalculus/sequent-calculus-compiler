codata Fun[A+, B+] { apply(x: A) : B }
codata FunToFun[A+, B-] { applyToFun(x: A) : B }
codata FunFromFun[A-, B-] { applyFromFun(x: A) : B }

def nonValueArguments() : i64 { new { applyToFun(x) => new { apply(y) => y}}.applyToFun[i64, Fun[i64,i64]](1 + 2).apply[i64, i64](3 + 4) }

def higherOrder() : i64 {  new { applyFromFun(x) => new { apply(y) => x.apply[i64, i64](y) }}.applyFromFun[Fun[i64,i64], Fun[i64,i64]](new { apply(z) => 4 + z}).apply[i64, i64](3 + 1) }

def main() : i64 { println_i64(higherOrder());
                   0 }
