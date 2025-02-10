codata LazyPair[A, B] { Fst : A, Snd : B }
data Pair[A, B] { Tup(x:A,y:B) }

// Swap the elements of a lazy pair.
def swapLazy(x:LazyPair[i64,i64]) : LazyPair[i64, i64] { cocase { Fst => x.Snd[i64, i64], Snd => x.Fst[i64, i64] } }

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LazyPair[i64,i64]) : Pair[i64,i64] { Tup(x.Fst[i64, i64], x.Snd[i64, i64]) }

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:Pair[i64,i64]) : LazyPair[i64,i64] { x.case[i64,i64] { Tup(a:i64, b:i64) => cocase { Fst => a, Snd => b }} }

def pairSum(x:LazyPair[i64,i64]) : i64 { (x.Fst[i64, i64]) + (x.Snd[i64, i64]) }

def main() : i64 { println_i64(pairSum(cocase { Fst => 1, Snd => 2}));
                   0 }
