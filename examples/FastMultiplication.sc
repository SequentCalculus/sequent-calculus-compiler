// Fast multiplication function from the introduction of the paper.
def fmult(l : Listint) := label 'a { mult(l; 'a) };
def mult(l : Listint, 'a :cnt Int) := case l of { Nil => 1,
                              Cons(x :Int , xs :Listint) => ifz(x, goto(0; 'a), x * (mult(xs; 'a))) };
def main() := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))););
