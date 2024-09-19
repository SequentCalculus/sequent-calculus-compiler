// Swap the elements of a lazy pair.
def swapLazy(x:LPairIntInt) : LPairIntInt := cocase { fst => x.snd, snd => x.fst };

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairInt) : TupIntInt := Tup(x.fst, x.snd);

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupIntInt) : LPairIntInt := case x of { Tup(a:Int, b:Int) => cocase { fst => a, snd => b }};

def main() : TupIntInt := toTuple(fromTuple(Tup(1, 2)));
