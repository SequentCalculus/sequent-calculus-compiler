<<<<<<< HEAD
<<<<<<< HEAD
def nonValueArguments() : Int := cocase { ap(x:Int) => cocase { ap(y:Int) => y}}.ap(1 + 2).ap(3 + 4);

def higherOrder() : Int :=  cocase { ap(x:Int) => cocase { ap(y:Int) => x.ap(y) }}.ap(cocase { ap(z:Int) => 4 + z}).ap(3 + 1);

def main() : Int := higherOrder(;);
=======
def nonValueArguments() := cocase { ap(x:Int) => cocase { ap(y:Int) => y}}.ap(1 + 2).ap(3 + 4);
=======
def nonValueArguments() : Int := cocase { ap(x:Int) => cocase { ap(y:Int) => y}}.ap(1 + 2).ap(3 + 4);
>>>>>>> 27de75f (added parsing definition types)

def higherOrder() : Int :=  cocase { ap(x:Int) => cocase { ap(y:Int) => x.ap(y) }}.ap(cocase { ap(z:Int) => 4 + z}).ap(3 + 1);

<<<<<<< HEAD
<<<<<<< HEAD
def main() := higherOrder(;);
>>>>>>> 8eb76bc (fixed integration tests)
=======
def main() := higherOrder();
>>>>>>> 7b89b63 (fixed integration tests)
=======
def main() : Int := higherOrder(;);
>>>>>>> 27de75f (added parsing definition types)
