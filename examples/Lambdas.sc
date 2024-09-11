def nonValueArguments() : Int := cocase { ap(x:Int) => cocase { ap(y:Int) => y}}.ap(1 + 2).ap(3 + 4);

def higherOrder() : Int :=  cocase { ap(x:Int) => cocase { ap(y:Int) => x.ap(y) }}.ap(cocase { ap(z:Int) => 4 + z}).ap(3 + 1);

def main() := higherOrder();
