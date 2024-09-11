// Fast multiplication function from the introduction of the paper.
<<<<<<< HEAD
<<<<<<< HEAD
def fmult(l : ListInt) : Int := label 'a { mult(l; 'a) };
def mult(l : ListInt, 'a :cnt Int) : Int := case l of { Nil => 1,
                              Cons(x :Int , xs :ListInt) => ifz(x, goto(0; 'a), x * (mult(xs; 'a))) };
def main() : Int := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))););
=======
def fmult(l : Listint) := label 'a { mult(l; 'a) };
def mult(l : Listint, 'a :cnt Int) := case l of { Nil => 1,
                              Cons(x :Int , xs :Listint) => ifz(x, goto(0; 'a), x * (mult(xs; 'a))) };
def main() := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))););
>>>>>>> 8eb76bc (fixed integration tests)
=======
def fmult(l : Listint) := label 'a { mult(l, 'a) };
def mult(l : Listint, 'a : Int) := case l of { Nil => 1,
                              Cons(x :Int , xs :Listint) => ifz(x, goto(0; 'a), x * (mult(xs, 'a))) };
def main() := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))));
>>>>>>> 7b89b63 (fixed integration tests)
