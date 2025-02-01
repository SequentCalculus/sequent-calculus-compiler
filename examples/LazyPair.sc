codata LPairI64I64 { Fst : i64, Snd : i64 } 
data TupI64I64 { Tup(x:i64,y:i64) }

// Swap the elements of a lazy pair.
def swapLazy(x:LPairI64I64) : LPairI64I64 { cocase { Fst => x.Snd, Snd => x.Fst } }

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairI64I64) : TupI64I64 { Tup(x.Fst, x.Snd) }

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupI64I64) : LPairI64I64 { x.case { Tup(a:i64, b:i64) => cocase { Fst => a, Snd => b }} }

def pairSum(x:LPairI64I64) : i64 { (x.Fst) + (x.Snd) }

def main() : i64 { println_i64(pairSum(cocase { Fst => 1, Snd => 2}));
                   0 }
