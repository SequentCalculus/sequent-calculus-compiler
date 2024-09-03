def nonValueArguments(;) := cocase { ap(x) => cocase { ap(y) => y}}.ap(1 + 2).ap(3 + 4);

def higherOrder(;) :=  cocase { ap(x) => cocase { ap(y) => x.ap(y) }}.ap(cocase { ap(z) => 4 + z}).ap(3 + 1);

def main(;) := higherOrder(;);
