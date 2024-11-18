codata FunIntInt { Ap(x:Int) : Int }

//def iterate(i: Int, f: FunIntInt, a: Int): Int := ifz(i, a, iterate(i - 1, f, f.Ap(a)));

def iterate(i: Int, f: FunIntInt, a: Int): Int := ifz(i, a, let j: Int = i - 1 in iterate(j, f, f.Ap(a)));

def main(n: Int): Int := iterate(n, cocase { Ap(x: Int) => x + 1}, 0);
