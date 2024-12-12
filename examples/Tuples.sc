data TupIntInt { Tup(x:Int, y:Int) }

def first(x : TupIntInt) : Int := x.case { Tup(a : Int, b : Int) => a };

def main() : Int := first(Tup(2+3, 2));
