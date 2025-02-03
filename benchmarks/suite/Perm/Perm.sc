data Unit { Unit } 
data Bool {True, False}
data ListI64{NilI,ConsI(i:i64,is:ListI64)}
data ListListI64 { NilL, ConsL(is:ListI64,iss:ListListI64) }
data PairListListI64ListI64 { TupLLILI(fst:ListListI64,snd:ListI64) }
codata FunUnitListListI64 { ApULLI(u:Unit) : ListListI64 }
codata FunListListI64Bool { ApLLIB(l:ListListI64) : Bool }

def empty_i(l:ListI64) : Bool := l.case{
  NilI => True,
  ConsI(i:i64,is:ListI64) => False
};

def tail_i(l:ListI64) : ListI64 := l.case{
  NilI => NilI, //should give a runtime error 
  ConsI(i:i64,is:ListI64) => is
};

def head_i(l:ListI64) : i64 := l.case{
  NilI => 0, //should giva a runtime error 
  ConsI(i:i64,is:ListI64) => i
};

def len_i(l:ListI64) : i64 := l.case{
  NilI => 0,
  ConsI(i:i64,is:ListI64) => 1+len_i(is)
};

def empty_l(l:ListListI64) : Bool := l.case{
  NilL => True,
  ConsL(is:ListI64,iss:ListListI64) => False
};

def tail_l(l:ListListI64) : ListListI64 := l.case{
  NilL => NilL, //should give a runtime error
  ConsL(is:ListI64,iss:ListListI64) => iss
};

def head_l(l:ListListI64) : ListI64 := l.case{
  NilL => NilI,//should give a runtime error 
  ConsL(is:ListI64,iss:ListListI64) => is
};

def loop_p(j:i64,perms:ListListI64,x:ListI64,n:i64) :PairListListI64ListI64 := 
  ifz(j,p(n-1,perms,x),
    let pair_perms_x : PairListListI64ListI64 = p(n-1, perms, x) in
    pair_perms_x.case {
      TupLLILI(perms:ListListI64, x:ListI64) =>   
        let pair_perms_x : PairListListI64ListI64 = f(n, perms, x)in 
        pair_perms_x.case{
          TupLLILI(perms:ListListI64,x:ListI64) => loop_p(j-1, perms, x,n)
        }
    });

def p(n:i64,perms:ListListI64,x:ListI64) : PairListListI64ListI64 := 
  ifl(1,n,loop_p(n-1, perms,x,n),TupLLILI(perms, x));

def f(n:i64,perms:ListListI64,x:ListI64) : PairListListI64ListI64 := 
  let x : ListI64 = rev_loop(x,n,list_tail(x, n)) in
  let new_perms : ListListI64 = ConsL(x,perms) in 
  TupLLILI(new_perms, x);

def rev_loop(x:ListI64, n:i64, y:ListI64) : ListI64 := ifz(n,y,rev_loop(tail_i(x),n-1,ConsI(head_i(x),y)));

def list_tail(x:ListI64, n:i64) : ListI64 := ifz(n,x,list_tail(tail_i(x),n-1));

def permutations(x0:ListI64) :ListListI64 := p(len_i(x0), ConsL(x0,NilL), x0).case{
  TupLLILI(final_perms:ListListI64,x:ListI64) => final_perms
};

def factorial(n:i64) : i64 := ife(n,1,1,n*factorial(n-1));

def loop_run(iters:i64,work:FunUnitListListI64,result:FunListListI64Bool) : Unit := 
  ifz(iters,Unit,
    let res : Bool = result.ApLLIB(work.ApULLI(Unit)) in 
    loop_run(iters-1,work,result));

def run_benchmark(iters:i64, work:FunUnitListListI64, result:FunListListI64Bool) : Unit := loop_run(iters,work,result);

def loop_work(m:i64,perms:ListListI64) : ListListI64 := ifz(m,perms,loop_work(m-1,permutations(head_l(perms))));

def loop_sum1(x:ListListI64,sum:i64) : i64 := empty_l(x).case{
  True => sum,
  False => loop_sum1(tail_l(x),loop_sum2(head_l(x),sum))
};

def loop_sum2(y:ListI64,sum:i64) : i64 := empty_i(y).case{ 
  True => sum,
  False => loop_sum2(tail_i(y),sum+head_i(y))
};

def sumlists(x:ListListI64) : i64 := loop_sum1(x, 0);

def loop_one2n(n:i64,p:ListI64) : ListI64 := ifz(n,p,loop_one2n(n-1,ConsI(n,p)));
def one2n(n:i64) : ListI64 := loop_one2n(n, NilI);

def perm9(m:i64,n:i64) : Unit :=
  run_benchmark (1, 
    cocase { ApULLI(u:Unit) =>  loop_work(m, permutations(one2n(n))) },
    cocase { ApLLIB(result:ListListI64) =>  ife(sumlists(result), ((n * (n + 1)) * factorial(n))/2,True,False) }
  );

def loop(iters:i64,m:i64,n:i64) : i64 := ifz(iters,0,let res : Unit = perm9(m,n) in loop(iters-1,m,n));

def main(iters:i64,m:i64,n:i64) : i64 := loop(iters,m,n);
