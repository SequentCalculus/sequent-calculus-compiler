codata FunIntInt { Ap(x:Int) : Int }
codata FunIntCurried { Ap2(x:Int) : FunIntInt }
codata FunFun { Ap3(x:FunIntInt) : FunIntInt }
def nonValueArguments() : Int := cocase { Ap2(x:Int) => cocase { Ap(y:Int) => y}}.Ap2(1 + 2).Ap(3 + 4);

def higherOrder() : Int :=  cocase { Ap3(x:FunIntInt) => cocase { Ap(y:Int) => x.Ap(y) }}.Ap3(cocase { Ap(z:Int) => 4 + z}).Ap(3 + 1);

def main() : Int := higherOrder();
