codata FunIntInt { Ap(x:Int) : Int }
def nonValueArguments() : Int := cocase { Ap(x:Int) => cocase { Ap(y:Int) => y}}.Ap(1 + 2).Ap(3 + 4);

def higherOrder() : Int :=  cocase { Ap(x:Int) => cocase { Ap(y:Int) => x.Ap(y) }}.Ap(cocase { Ap(z:Int) => 4 + z}).Ap(3 + 1);

def main() : Int := higherOrder();
