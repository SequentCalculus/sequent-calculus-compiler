codata LazyPair[A, B] { fst : A, snd : B }
data Pair[A, B] { Tup(x:A,y:B) }

// Swap the elements of a lazy pair.
def swapLazy(x:LazyPair[i64,i64]) : LazyPair[i64, i64] { new { fst => x.snd[i64, i64], snd => x.fst[i64, i64] } }

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LazyPair[i64,i64]) : Pair[i64,i64] { Tup(x.fst[i64, i64], x.snd[i64, i64]) }

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:Pair[i64,i64]) : LazyPair[i64,i64] { x.case[i64,i64] { Tup(a, b) => new { fst => a, snd => b }} }

def pairSum(x:LazyPair[i64,i64]) : i64 { (x.fst[i64, i64]) + (x.snd[i64, i64]) }

def main() : i64 { println_i64(pairSum(new { fst => 1, snd => 2}));
                   0 }
