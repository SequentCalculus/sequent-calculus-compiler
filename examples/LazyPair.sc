// Swap the elements of a lazy pair.
<<<<<<< HEAD
<<<<<<< HEAD
def swapLazy(x:LPairIntInt) : LPairIntInt := cocase { fst => x.snd, snd => x.fst };

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairInt) : TupIntInt := Tup(x.fst, x.snd);

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupIntInt) : LPairIntInt := case x of { Tup(a:Int, b:Int) => cocase { fst => a, snd => b }};

def main() : TupIntInt := toTuple(fromTuple(Tup(1, 2);););
=======
def swapLazy(x:LPairInt) := cocase { fst => x.snd, snd => x.fst };
=======
def swapLazy(x:LPairIntInt) : LPairIntInt := cocase { fst => x.snd, snd => x.fst };
>>>>>>> 27de75f (added parsing definition types)

// Convert a lazy tuple to a strict tuple.
def toTuple(x:LPairInt) : TupIntInt := Tup(x.fst, x.snd);

// Convert a strict tuple to a lazy tuple.
def fromTuple(x:TupIntInt) : LPairIntInt := case x of { Tup(a:Int, b:Int) => cocase { fst => a, snd => b }};

<<<<<<< HEAD
<<<<<<< HEAD
def main() := toTuple(fromTuple(Tup(1, 2);););
>>>>>>> 8eb76bc (fixed integration tests)
=======
def main() := toTuple(fromTuple(Tup(1, 2)));
>>>>>>> 7b89b63 (fixed integration tests)
=======
def main() : TupIntInt := toTuple(fromTuple(Tup(1, 2);););
>>>>>>> 27de75f (added parsing definition types)
