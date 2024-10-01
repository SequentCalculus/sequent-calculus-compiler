codata LPairIntInt { Fst : Int, Snd : Int } 

// Swap the elements of a lazy pair.
def swapLazy(x:LPairIntInt) : LPairIntInt := cocase { Fst => x.Snd, Snd => x.Fst };

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairInt) : TupIntInt := Tup(x.Fst, x.Snd);

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupIntInt) : LPairIntInt := case x of { Tup(a:Int, b:Int) => cocase { Fst => a, Snd => b }};

def main() : TupIntInt := toTuple(fromTuple(Tup(1, 2)));
