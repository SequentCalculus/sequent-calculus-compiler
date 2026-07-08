codata Fun[A, B] { apply(x : A): B }

def foo(): i64 { new { apply(f) => new { apply(x) => f.apply(x) } }
                   .apply(new { apply(y) => y })
                   .apply(3) }

def main(): i64 {
    println_i64(foo());
    0
}