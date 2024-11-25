codata LPairIntInt { Fst : Int, Snd : Int } 
data TupIntInt { Tup(x:Int,y:Int) }

// Swap the elements of a lazy pair.
def swapLazy(x:LPairIntInt) : LPairIntInt := cocase { Fst => x.Snd, Snd => x.Fst };

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairIntInt) : TupIntInt := Tup(x.Fst, x.Snd);

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupIntInt) : LPairIntInt := x.case { Tup(a:Int, b:Int) => cocase { Fst => a, Snd => b }};

def pairSum(x:LPairIntInt) : Int := (x.Fst) + (x.Snd);

def main() : Int := pairSum(cocase { Fst => 1, Snd => 2});
