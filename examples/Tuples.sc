def swap(x) := case x of { Tup(a, b) => Tup(b, a) };
def diag(x) := Tup(x, x);
def first(x) := case x of { Tup(a, b) => a };
def second(x) := case x of { Tup(a, b) => b };
def toList(x) := case x of { Tup(a, b) => Cons(a, Cons(b, Nil)) };

def main := toList(Tup(1, 2));
