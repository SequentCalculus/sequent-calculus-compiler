codata FunI64I64 { Ap(x:i64) : i64 }
codata FunI64Curried { Ap2(x:i64) : FunI64I64 }
codata FunFun { Ap3(x:FunI64I64) : FunI64I64 }
def nonValueArguments() : i64 { cocase { Ap2(x:i64) => cocase { Ap(y:i64) => y}}.Ap2(1 + 2).Ap(3 + 4) }

def higherOrder() : i64 {  cocase { Ap3(x:FunI64I64) => cocase { Ap(y:i64) => x.Ap(y) }}.Ap3(cocase { Ap(z:i64) => 4 + z}).Ap(3 + 1) }

def main() : i64 { println_i64(higherOrder());
                    0 }
