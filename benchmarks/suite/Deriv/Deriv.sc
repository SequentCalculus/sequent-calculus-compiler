data ListExpr{Nil,Cons(e:Expr,es:ListExpr) }
data Unit { Unit }
data Bool { True, False } 
codata FunExprUnit { ApU(e:Expr) : Unit }
codata FunExprExpr { ApE(e:Expr) : Expr }

data Expr{
  Add(sums:ListExpr),
  Sub(subs:ListExpr),
  Mul(muls:ListExpr),
  Div(divs:ListExpr),
  Num(i:i64),
  X()
}

def map_list(f : FunExprExpr , l : ListExpr) : ListExpr { 
  l.case { 
    Nil => Nil,
  Cons(x : Expr, xs : ListExpr) => Cons(f.ApE(x), map_list(f, xs)) }
}

def map_expr(f:FunExprExpr,e:Expr) : Expr {
  e.case{
    Add(sums:ListExpr) => Add(map_list(f,sums)),
    Sub(subs:ListExpr) => Sub(map_list(f,subs)),
    Mul(muls:ListExpr) => Mul(map_list(f,muls)),
    Div(divs:ListExpr) => Div(map_list(f,divs)),
    Num(i:i64)         => f.ApE(Num(i)),
    X()                => f.ApE(X())
  }
}

def app_list(f:FunExprUnit,l:ListExpr) : Unit { 
  l.case {
    Nil => Unit,
    Cons(x:Expr,xs:ListExpr) => let x : Unit = f.ApU(x); app_list(f,xs)
  }
}

def app_exp(f:FunExprUnit,e:Expr) : Unit { 
  e.case{
    Add(sums:ListExpr) => app_list(f,sums),
    Sub(subs:ListExpr) => app_list(f,subs),
    Mul(muls:ListExpr) => app_list(f,muls),
    Div(divs:ListExpr) => app_list(f,divs),
    Num(i:i64)         => f.ApU(Num(i)),
    X()                => f.ApU(X())

  }
}

def and(b1:Bool,b2:Bool) : Bool {
  b1.case{
    False => False,
    True => b2
  }
}

def equal_list(l1:ListExpr,l2:ListExpr) : Bool {
  l1.case{
    Nil => l1.case {
      Nil => True,
      Cons(e:Expr,es:ListExpr) => False
    },
    Cons(e1:Expr,es1:ListExpr) => l2.case{
      Nil => False,
      Cons(e2:Expr,es2:ListExpr) => and(equal(e1,e2),equal_list(es1,es2))
    },
  }
}

def equal(exp1:Expr,exp2:Expr) : Bool {
  exp1.case {
    Add(sums1:ListExpr) => exp2.case {
      Add(sums2:ListExpr) => equal_list(sums1,sums2),
      Sub(subs:ListExpr) => False,
      Mul(muls:ListExpr) => False,
      Div(divs:ListExpr) => False,
      Num(i:i64) => False,
      X() => False
    },
    Sub(subs1:ListExpr) => exp2.case{
      Sub(subs2:ListExpr) => equal_list(subs1,subs2),
      Add(sums:ListExpr) => False,
      Mul(muls:ListExpr) => False,
      Div(divs:ListExpr) => False,
      Num(i:i64) => False,
      X() => False
    },
    Mul(muls1:ListExpr) => exp2.case{
      Mul(muls2:ListExpr) => equal_list(muls1,muls2),
      Add(sums:ListExpr) => False,
      Sub(subs:ListExpr) => False,
      Div(divs:ListExpr) => False,
      Num(i:i64) => False,
      X() => False

    },
    Div(divs1:ListExpr) => exp2.case{
      Div(divs2:ListExpr) => equal_list(divs1,divs2),
      Add(sums:ListExpr) => False,
      Sub(subs:ListExpr) => False,
      Mul(muls:ListExpr) => False,
      Num(i:i64) => False,
      X() => False
    },
    Num(i1:i64) => exp2.case{
      Num(i2:i64) => if i1== i2 { True } else { False },
      Add(sums:ListExpr) => False,
      Sub(subs:ListExpr) => False,
      Mul(muls:ListExpr) => False,
      Div(divs:ListExpr) => False,
      X() => False
    },
    X() => exp2.case{
      X() => True,
      Add(sums:ListExpr) => False,
      Sub(subs:ListExpr) => False,
      Mul(muls:ListExpr) => False,
      Div(divs:ListExpr) => False,
      Num(i:i64) => False
    }
  }
}

def deriv() : FunExprExpr {
  cocase { ApE(e:Expr) => 
    e.case{
      Add(sums:ListExpr) => Add(map_list(deriv(), sums)),
      Sub(subs:ListExpr) => Sub(map_list(deriv(), subs)),
      Mul(muls:ListExpr) => 
        Mul(Cons(
          e,
          Cons(Add(
            map_list(cocase{ApE(e:Expr) => Div(Cons(deriv().ApE(e),Cons(e,Nil)))},muls)),Nil)
        )),
        Div(divs:ListExpr) => divs.case{
          Nil => X(), // This should rais a runtime error 
          Cons(x:Expr,xs:ListExpr) => xs.case{
            Nil => X(), //This should rais a runtime error  
            Cons(y:Expr,ys:ListExpr) => ys.case{
              Nil => Sub(Cons(
                Div(Cons(deriv().ApE(x),Cons(y,Nil))), 
                Cons(Div(Cons(
                  x,Cons(Mul(Cons(y,Cons(y,Cons(deriv().ApE(y),Nil)))),Nil)
                )), Nil))),
              Cons(z:Expr,zs:ListExpr) => X() //This should rais a runtime error 
            }
          }
        },
        Num(i:i64) => Num(0),
        X() => Num(1)
    } 
  }
}

def main(n:i64,m:i64) : i64 {
  let exp : Expr =  Add(
    Cons(Mul(Cons(Num(3), Cons(X(), Cons(X(),Nil)))),
      Cons(Mul(Cons(Num(n), Cons(X(), Cons(X(),Nil)))),
        Cons(Mul(Cons(Num(m),Cons(X(),Nil))),
          Cons(Num(5),Nil )))));
  let res : Expr = deriv().ApE(exp); 
  0
}
