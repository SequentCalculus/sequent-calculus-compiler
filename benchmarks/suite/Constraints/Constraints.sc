data List[A] { Nil, Cons(a:A,as:List[A]) }
data Pair[A,B] { Tup(a:A,b:B) }
data Option[A] { None, Some(a:A) }
data Bool { True, False } 
codata Fun[A,B] { Ap(a:A) : B }

data Assign{ Assign(varr: i64, value: i64) }
data CSP { CSP(vars: i64, vals: i64, rel: Fun[Assign,Fun[Assign,Bool]]) }
data Node[T] { Node(lab: T, children: List[Node[T]]) }
data ConflictSet { Known(vs:List[i64]), Unknown }

def abs_i(i:i64) : i64 {
  if i<0{
    -1*i
  }else{
    i
  }
}

def eq(i1:i64,i2:i64) : Bool {
  if i1 == i2{
    True
  }else{
    False
  }
}

def and(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => b2,
    False => False
  }
}

def not(b:Bool):Bool{
  b.case {
    True => False,
    False => True
  }
}

def reverse_loop(ls:List[Assign],acc:List[Assign]) : List[Assign]{
  ls.case[Assign] {
    Nil => acc,
    Cons(a,as) => reverse_loop(as,Cons(a,acc))
  }
}

def reverse(ls:List[Assign]):List[Assign]{
  reverse_loop(ls,Nil)
}

def null_(ls:List[Pair[i64,Assign]]): Bool{
  ls.case[Pair[i64,Assign]]{
    Nil => True,
    Cons(a,as) => False
  }
}

// Not sure if this is the correct "a > b" 
def gr_assign(a1:Assign,a2:Assign) : Bool {
  a1.case{
    Assign(var1,val1) => a2.case {
      Assign(var2,val2) => 
        if var2 < var1{
          True
        }else{
          if val2 < val1{
            True
          }else{
            False
          }
        }
    }
  }
}

// Sorting
// not sure if i64 is correct for the sorting functions 
def qsort(le:Fun[i64,Fun[i64,Bool]], ls: List[i64] , r:List[i64]):List[i64] { 
  ls.case[i64]{
    Nil => r,
    Cons(x,xs) => xs.case[i64]{
      Nil => Cons(x,r),
      Cons(x, xs) => qpart(le, x, xs, Nil, Nil, r)
    }
  }
}

def qpart(le:Fun[i64,Fun[i64,Bool]], x:i64, ls:List[i64] , rlt:List[i64], rge:List[i64], r:List[i64]) : List[i64]{
  ls.case[i64] {
    Nil => rqsort(le, rlt, Cons(x,rqsort(le, rge, r))),
    Cons(y,ys) => le.Ap[i64,Fun[i64,Bool]](x).Ap[i64,Bool](y).case{
      True => qpart(le, x, ys, rlt, Cons(y,rge), r),
      False => qpart(le, x, ys, Cons(y,rlt), rge, r)
    }
  }
}

def rqsort(le:Fun[i64,Fun[i64,Bool]], ls:List[i64], r:List[i64]) : List[i64] {
  ls.case[i64]{
    Nil => r,
    Cons(x,xs) => xs.case[i64]{
      Nil => Cons(x,r),
      Cons(x,xs) => rqpart(le, x, xs, Nil, Nil, r) 
    }
  }
}

def rqpart(le:Fun[i64,Fun[i64,Bool]], x:i64, ls:List[i64], rle:List[i64], rgt:List[i64], r:List[i64]) : List[i64] {
  ls.case[i64] {
    Nil => rqsort(le, rle, Cons(x,qsort(le, rgt, r))),
    Cons(y,ys) => le.Ap[i64,Fun[i64,Bool]](y).Ap[i64,Bool](x).case{
      True => rqpart(le, x, ys, Cons(y,rle), rgt, r),
      False => rqpart(le, x, ys, rle, Cons(y,rgt), r)
    }
  }
}

def level(a:Assign) : i64 { 
  a.case{ 
    Assign(v, value) => v 
  }
}


def value(a:Assign) : i64 { 
  a.case { 
    Assign(varr, v) => v 
  }
}

def max_level(ls:List[Assign]): i64 {
  ls.case[Assign]{
    Nil => 0,
    Cons(a,t) => a.case{
      Assign(v,value) => v
    }
  }
}

def complete(csp:CSP, s:List[Assign]) : Bool {
  csp.case {
    CSP(v,vals,rel) => eq(max_level(s),v)
  }
}

// Generate 
def gen_lscomp2(ls:List[List[Assign]],t1:List[i64],vals:i64,var_:i64,val_:i64) : List[List[Assign]]{
  ls.case[List[Assign]] {
    Nil => gen_lscomp1(t1,vals,var_),
    Cons(st,t2) => Cons(Cons(Assign(var_, val_),st),gen_lscomp2(t2,t1,vals,var_,val_))
  }
}

def gen_lscomp1(ls:List[i64],vals:i64,var_:i64) : List[List[Assign]] { 
  ls.case[i64] {
    Nil => Nil,
    Cons(val_ ,t1) =>  gen_lscomp2(gen_g(vals, var_ - 1),t1,vals,var_,val_)
  }
}

def gen_g(vals:i64, var_: i64) : List[List[Assign]]{
  if var_ == 0 {
    Cons(Nil,Nil)
  } else{
    gen_lscomp1(enum_from_to(1, vals),vals,var_)
  }
}

def generate(csp:CSP): List[List[Assign]]{
  csp.case {
    CSP(vars, vals, rel) => gen_g(vals, vars)
  }
}

def enum_from_to(from:i64,to_:i64) : List[i64]{
  if from<=to_{
    Cons(from,enum_from_to(from+1,to_))
  }else{
    Nil
  }
}

// inconsidtencies
def inc_lscomp2(ls:List[Assign],t1:List[Assign],a:Assign,rel:Fun[Assign,Fun[Assign,Bool]],as_:List[Assign]) : List[Pair[i64,Assign]] {
  ls.case[Assign] {
    Nil => inc_lscomp1(t1,rel,as_),
    Cons(b,t2) => and(gr_assign(a,b),not(rel.Ap[Assign,Fun[Assign,Bool]](a).Ap[Assign,Bool](b))).case{
      True => Cons(Tup(level(a), b),inc_lscomp2(t2,t1,a,rel,as_)),
      False => inc_lscomp2(t2,t1,a,rel,as_)
    }
  }
}

def inc_lscomp1(ls:List[Assign],rel:Fun[Assign,Fun[Assign,Bool]],as_:List[Assign]) : List[Pair[i64,Assign]] { 
  ls.case[Assign]{
    Nil => Nil,
    Cons(a,t1) => inc_lscomp2(reverse(as_),t1,a,rel,as_)
  }
}

def inconsistencies(csp:CSP, as_:List[Assign]) : List[Pair[i64,Assign]] {
  csp.case { 
    CSP(vars, vals, rel) => inc_lscomp1(as_,rel,as_)
  }
}

def consistent(csp:CSP, x:List[Assign]) : Bool{
  null_(inconsistencies(csp, x)) 
}

//test 
def test_filter(f:Fun[List[Assign],Bool],ls:List[List[Assign]]) : List[List[Assign]] {
  ls.case[List[Assign]]{
    Nil => Nil,
    Cons(l,ls) => f.Ap[List[Assign],Bool](l).case{
      True => Cons(l,test_filter(f,ls)),
      False => test_filter(f,ls)
    }
  }
}

def test(csp:CSP,x:List[List[Assign]]) : List[List[Assign]] {
  test_filter(new { Ap(x) => consistent(csp,x)} ,x ) 
}

def solver(csp:CSP) : List[List[Assign]] {
  test(csp, generate(csp)) 
}

def safe(as1:Assign, as2:Assign) : Bool{ 
  as1.case{
    Assign(i,m) => as2.case{
      Assign(j,n) => and(not(eq(m,n)),not(eq(abs_i(i - j),abs_i(m - n))))
    }
  }
}


def queens(n:i64) : CSP {
  CSP(n, n, new { Ap(x) => new { Ap(y) => safe(x,y)}} )
}

// mk_tree 

def mk_map(f:Fun[List[Assign],Node[List[Assign]]],ls:List[List[Assign]]) : List[Node[List[Assign]]]{
  ls.case[List[Assign]]{
    Nil => Nil,
    Cons(l,ls) => Cons(f.Ap[List[Assign],Node[List[Assign]]](l),mk_map(f,ls))
  }
}

def mk_init_tree(f:Fun[List[Assign],List[List[Assign]]], x:List[Assign]) :Node[List[Assign]] {
  Node(x, mk_map(new { Ap(y) => mk_init_tree(f, y) }, f.Ap[List[Assign],List[List[Assign]]](x))) 
}


def mk_lscomp1(ls: List[i64],ss:List[Assign]) : List[List[Assign]] {
  ls.case[i64]{
    Nil => Nil,
    Cons(j,t1) => Cons(Cons(Assign(max_level(ss) + 1, j),ss),mk_lscomp1(t1,ss))
  }
}


def mk_tree(csp:CSP) : Node[List[Assign]] {
  csp.case {
    CSP(vars, vals, rel) =>
      let next: Fun[List[Assign],List[List[Assign]]] = 
        new { Ap(ss) => 
          if max_level(ss) < vars {
            mk_lscomp1(enum_from_to(1, vals),ss)
          }else {
            Nil     
          }
        };
        mk_init_tree(next, Nil) 
  }
}

// earilest_inconsistency 

def ear_inc_filter(f:Fun[Assign,Bool],ls:List[Assign]):List[Assign]{
  ls.case[Assign]{
    Nil => Nil,
    Cons(a,as) => f.Ap[Assign,Bool](a).case{
      True => Cons(a,ear_inc_filter(f,as)),
      False => ear_inc_filter(f,as)
    }
  }
}

def earliest_inconsistency(csp:CSP, aas:List[Assign]): Option[Pair[i64,i64]] {
  csp.case{
    CSP(vars, vals, rel) => aas.case[Assign]{
      Nil => None,
      Cons(a,as_) =>  ear_inc_filter(
        new { Ap(x) => not(rel.Ap[Assign,Fun[Assign,Bool]](a).Ap[Assign,Bool](x)) }, 
        reverse(as_)).case[Assign]{
          Nil => None,
          Cons(b,bs_) => Some(Tup(level(a), level(b)))
        }
    }
  }
}

// label inconsistencies 
def label_map(f:Fun[Node[List[Assign]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]],ls:List[Node[List[Assign]]]) : 
List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]{
  ls.case[Node[List[Assign]]]{
    Nil => Nil,
    Cons(l,ls) => Cons(f.Ap[Node[List[Assign]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]](l),label_map(f,ls))
  }
}

def label_map_tree(f:Fun[List[Assign],Pair[List[Assign],Option[Pair[i64,i64]]]], n:Node[List[Assign]]): 
Node[Pair[List[Assign],Option[Pair[i64,i64]]]] {
  n.case[List[Assign]] {
    Node(l,c) => 
      Node(f.Ap[List[Assign],Pair[List[Assign],Option[Pair[i64,i64]]]](l),
        label_map(new { Ap(x) => label_map_tree(f,x) },c))
  }
}

def label_inconsistencies(csp:CSP, t:Node[List[Assign]]) : Node[Pair[List[Assign],Option[Pair[i64,i64]]]] {
  let f2: Fun[List[Assign],Pair[List[Assign],Option[Pair[i64,i64]]]] = 
    new { Ap(s) => Tup(s, earliest_inconsistency(csp, s)) };
    label_map_tree(f2, t) 
}

//btsolver0

def filter(f:Fun[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Bool],ls:List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]) :
List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]{
  ls.case[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]{
    Nil => Nil,
    Cons(p,ps) => f.Ap[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Bool](p).case{
      True => Cons(p,filter(f,ps)),
      False => filter(f,ps)
    }
  }
}

def lab(n:Node[Pair[List[Assign],Option[Pair[i64,i64]]]]):Pair[List[Assign],Option[Pair[i64,i64]]] {
  n.case[Pair[List[Assign],Option[Pair[i64,i64]]]]{
    Node(l,ch) => l
  }
}

def solv_map2(f:Fun[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]], n:List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]):
List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]] 
{
  n.case[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]] {
    Nil => Nil,
    Cons(p,ps) => Cons(f.Ap[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]](p),solv_map2(f,ps))
  }
}

def fold_tree(f:Fun[Pair[List[Assign],Option[Pair[i64,i64]]],Fun[List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]],
  n:Node[Pair[List[Assign],Option[Pair[i64,i64]]]]) :
Node[Pair[List[Assign],Option[Pair[i64,i64]]]] {
  n.case[Pair[List[Assign],Option[Pair[i64,i64]]]]{
    Node(l,c) => f.Ap[Pair[List[Assign],Option[Pair[i64,i64]]],Fun[List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]](l).Ap[List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]](solv_map2(new { Ap(x) => fold_tree(f,x)},c))
  }
}

def filter_tree(p: Fun[Pair[List[Assign],Option[Pair[i64,i64]]],Bool], t: Node[Pair[List[Assign],Option[Pair[i64,i64]]]]  ) : 
Node[Pair[List[Assign],Option[Pair[i64,i64]]]] {
  let f1 : Fun[Pair[List[Assign],Option[Pair[i64,i64]]],Fun[List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]],Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]] = 
    new{ Ap(a) => new { Ap(cs) => Node(a, filter(new {Ap(x) => p.Ap[Pair[List[Assign],Option[Pair[i64,i64]]],Bool](lab(x)) }, cs)) } };
    fold_tree(f1, t) 
}

def prune(p:Fun[Pair[List[Assign],Option[Pair[i64,i64]]],Bool], t:Node[Pair[List[Assign],Option[Pair[i64,i64]]]]) : 
Node[Pair[List[Assign],Option[Pair[i64,i64]]]] {
  filter_tree(new { Ap(x) => not(p.Ap[Pair[List[Assign],Option[Pair[i64,i64]]],Bool](x)) }, t) 
}

def fst(p:Pair[List[Assign],Option[Pair[i64,i64]]]) : List[Assign]{
  p.case[List[Assign],Option[Pair[i64,i64]]]{
    Tup(ls,o) => ls
  }
}

def snd(p:Pair[List[Assign],Option[Pair[i64,i64]]]) : Option[Pair[i64,i64]]{
  p.case[List[Assign],Option[Pair[i64,i64]]] {
    Tup(ls,p) => p
  }
}

def option_eq(o1:Option[Pair[i64,i64]],o2:Option[Pair[i64,i64]]) : Bool{
  o1.case[Pair[i64,i64]]{
    None => o2.case[Pair[i64,i64]]{
      None => True,
      Some(p) => False
    },
    Some(p1) => o2.case[Pair[i64,i64]]{
      None => False,
      Some(p2) => p1.case[i64,i64] {
        Tup(i11,i12) => p2.case[i64,i64]{
          Tup(i21,i22) => and(eq(i11,i21),eq(i12,i22))
        }
      }
    }
  }
}

def solv_map_tree(f:Fun[Pair[List[Assign],Option[Pair[i64,i64]]],List[Assign]], n:Node[Pair[List[Assign],Option[Pair[i64,i64]]]]): 
Node[List[Assign]] {
  n.case[Pair[List[Assign],Option[Pair[i64,i64]]]] {
    Node(l,c) => 
      Node(f.Ap[Pair[List[Assign],Option[Pair[i64,i64]]],List[Assign]](l),
        solv_map(new { Ap(x) => solv_map_tree(f,x) },c))
  }
}

def solv_map(f:Fun[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Node[List[Assign]]],ls:List[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]) : 
List[Node[List[Assign]]]{
  ls.case[Node[Pair[List[Assign],Option[Pair[i64,i64]]]]]{
    Nil => Nil,
    Cons(l,ls) => Cons(f.Ap[Node[Pair[List[Assign],Option[Pair[i64,i64]]]],Node[List[Assign]]](l),solv_map(f,ls))
  }
}

def concat_2(l1:List[List[Assign]],l2:List[List[Assign]]) : List[List[Assign]]{
  l1.case[List[Assign]]{
    Nil => l2,
    Cons(l1,l1s) => Cons(l1,concat_2(l1s,l2))
  }
}

def concat(ls:List[List[List[Assign]]]) : List[List[Assign]]{
  ls.case[List[List[Assign]]]{
    Nil => Nil,
    Cons(l,ls) => concat_2(l,concat(ls))
  }
}

def solv_map3(f:Fun[Node[List[Assign]],List[List[Assign]]],cs:List[Node[List[Assign]]]) : List[List[List[Assign]]]{
  cs.case[Node[List[Assign]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[List[Assign]],List[List[Assign]]](n),solv_map3(f,ns))
  }
}

def leaves(t:Node[List[Assign]]): List[List[Assign]] {
  t.case[List[Assign]] {
    Node(leaf,cs) => cs.case[Node[List[Assign]]]{
      Nil => Cons(leaf,Nil),
      Cons(c,cs) => concat(solv_map3(new { Ap(x) => leaves(x) } ,Cons(c,cs)))
    }
  }
}

def solv_filter(f:Fun[List[Assign],Bool],ls:List[List[Assign]]) : List[List[Assign]]{
  ls.case[List[Assign]]{
    Nil => Nil,
    Cons(l,ls) => f.Ap[List[Assign],Bool](l).case{
      True => Cons(l,solv_filter(f,ls)),
      False => solv_filter(f,ls)
    }
  }
}


def btsolver0(csp:CSP) : List[List[Assign]] {
  solv_filter(new {Ap(x) => complete(csp, x) },
    leaves( 
      solv_map_tree(new { Ap(p) => fst(p) },
        prune(new { Ap(x) => not(option_eq(snd(x),None)) }, label_inconsistencies(csp, mk_tree(csp)))))) 
}


def known_conflict(c:ConflictSet): Bool {
  c.case {
    Known(vs) => vs.case[i64]{
      Nil => False,
      Cons(v,vs) => True
    },
    Unknown => False
  }
}

def known_solution(c:ConflictSet): Bool {
  c.case {
    Known(vs) => vs.case[i64]{
      Nil => True,
      Cons(v,vs) => False
    },
    Unknown => False 
  }
}

def check_complete(csp:CSP, s:List[Assign]): ConflictSet {
  complete(csp,s).case{
    True => Known(Nil),
    False => Unknown
  }
}

def search_filter(f:Fun[Node[Pair[List[Assign],ConflictSet]],Bool],l:List[Node[Pair[List[Assign],ConflictSet]]]) : List[Node[Pair[List[Assign],ConflictSet]]]{
  l.case[Node[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(p,ps) => f.Ap[Node[Pair[List[Assign],ConflictSet]],Bool](p).case{
      True => Cons(p,search_filter(f,ps)),
      False => search_filter(f,ps)
    }
  }
}

def search_label(n:Node[Pair[List[Assign],ConflictSet]]) : Pair[List[Assign],ConflictSet]{
  n.case[Pair[List[Assign],ConflictSet]]{
    Node(p,cs) => p
  }
}

def search_map(f:Fun[Node[Pair[List[Assign],ConflictSet]],Node[Pair[List[Assign],ConflictSet]]], ls:List[Node[Pair[List[Assign],ConflictSet]]]) : List[Node[Pair[List[Assign],ConflictSet]]]{
  ls.case[Node[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[List[Assign],ConflictSet]],Node[Pair[List[Assign],ConflictSet]]](n),search_map(f,ns))
  }
}

def search_fold_tree(f:Fun[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]], n:Node[Pair[List[Assign],ConflictSet]]) :
Node[Pair[List[Assign],ConflictSet]] {
  n.case[Pair[List[Assign],ConflictSet]]{
    Node(l,c) => f.Ap[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]](l).Ap[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]](search_map(new { Ap(x) => search_fold_tree(f,x)},c))
  }
}

def search_filter_tree(p: Fun[Pair[List[Assign],ConflictSet],Bool], n: Node[Pair[List[Assign],ConflictSet]]) : 
Node[Pair[List[Assign],ConflictSet]] {
  let f1: Fun[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]] = 
    new{ Ap(a) => new { Ap(cs) => Node(a, search_filter(new {Ap(x) => p.Ap[Pair[List[Assign],ConflictSet],Bool](search_label(x)) }, cs)) } };
    search_fold_tree(f1, n) 
}

def search_prune(f:Fun[Pair[List[Assign],ConflictSet],Bool],n:Node[Pair[List[Assign],ConflictSet]]) : Node[Pair[List[Assign],ConflictSet]]{
  search_filter_tree(new { Ap(x) => not(f.Ap[Pair[List[Assign],ConflictSet],Bool](x)) }, n) 
}

def search_map2(f:Fun[Node[Pair[List[Assign],ConflictSet]],List[Pair[List[Assign],ConflictSet]]],l:List[Node[Pair[List[Assign],ConflictSet]]]):List[List[Pair[List[Assign],ConflictSet]]]{
  l.case[Node[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[List[Assign],ConflictSet]],List[Pair[List[Assign],ConflictSet]]](n),search_map2(f,ns))
  }
}

def search_concat2(l1:List[Pair[List[Assign],ConflictSet]],l2:List[Pair[List[Assign],ConflictSet]]) : List[Pair[List[Assign],ConflictSet]]{
  l1.case[Pair[List[Assign],ConflictSet]]{
    Nil => l2,
    Cons(p,ps) => Cons(p,search_concat2(ps,l2))
  }
}

def search_concat(ls:List[List[Pair[List[Assign],ConflictSet]]]) : List[Pair[List[Assign],ConflictSet]]{
  ls.case[List[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(l,ls) => search_concat2(l,search_concat(ls))
  }
}

def search_leaves(n:Node[Pair[List[Assign],ConflictSet]]) : List[Pair[List[Assign],ConflictSet]]{
  n.case[Pair[List[Assign],ConflictSet]] {
    Node(leaf,cs) => cs.case[Node[Pair[List[Assign],ConflictSet]]]{
      Nil => Cons(leaf,Nil),
      Cons(c,cs) => search_concat(search_map2(new { Ap(x) => search_leaves(x) } ,Cons(c,cs)))
    }
  }
}

def search_filter2(f:Fun[Pair[List[Assign],ConflictSet],Bool], l:List[Pair[List[Assign],ConflictSet]]) : List[Pair[List[Assign],ConflictSet]]{
  l.case[Pair[List[Assign],ConflictSet]]{
    Nil => Nil,
    Cons(p,ps) => f.Ap[Pair[List[Assign],ConflictSet],Bool](p).case{
      True => Cons(p,search_filter2(f,ps)),
      False => search_filter2(f,ps)
    }
  }
}

def search_map3(f:Fun[Pair[List[Assign],ConflictSet],List[Assign]],ls:List[Pair[List[Assign],ConflictSet]]) : List[List[Assign]]{
  ls.case[Pair[List[Assign],ConflictSet]]{
    Nil => Nil,
    Cons(p,ps) => Cons(f.Ap[Pair[List[Assign],ConflictSet],List[Assign]](p),search_map3(f,ps))
  }
}

def search_fst(p:Pair[List[Assign],ConflictSet]) : List[Assign]{
  p.case[List[Assign],ConflictSet]{
    Tup(l,c) => l
  }
}

def search_snd(p:Pair[List[Assign],ConflictSet]):ConflictSet{
  p.case[List[Assign],ConflictSet]{
    Tup(l,c) => c
  }
}

def search(labeler:Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]], csp:CSP) : List[List[Assign]] {
  search_map3(new { Ap(x) => search_fst(x)} ,
    search_filter2(new { Ap(x) => known_solution(search_snd(x)) },
      search_leaves( 
        search_prune(new { Ap(x) => known_conflict(search_snd(x)) }, 
          labeler.Ap[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]](csp).Ap[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]](mk_tree(csp))))))
}

// bt
def bt_map(f:Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]],ls:List[Node[List[Assign]]]) : List[Node[Pair[List[Assign],ConflictSet]]]{
  ls.case[Node[List[Assign]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]](n),bt_map(f,ns))
  }
}

def bt_map_tree(f:Fun[List[Assign],Pair[List[Assign],ConflictSet]],n:Node[List[Assign]]) : Node[Pair[List[Assign],ConflictSet]]{
  n.case[List[Assign]]{
    Node(l,ls) => Node(f.Ap[List[Assign],Pair[List[Assign],ConflictSet]](l),
      bt_map(new { Ap(x) => bt_map_tree(f,x) },ls))
  }
}

def bt(csp:CSP, t:Node[List[Assign]] ) : Node[Pair[List[Assign],ConflictSet]] {
  let f3 : Fun[List[Assign],Pair[List[Assign],ConflictSet]] = 
    new { Ap(s) => 
      Tup(s, (earliest_inconsistency(csp, s).case[Pair[i64,i64]]{
        Some(p) => p.case[i64,i64]{
          Tup(a, b) => Known(Cons(a,Cons(b,Nil)))
        },
        None => check_complete(csp, s)
      }))
    };
    bt_map_tree(f3, t) 
}

//empty_table 

def empt_lscomp2(ls: List[i64]) : List[ConflictSet]{
  ls.case[i64]{
    Nil => Nil,
    Cons(m,t2) => Cons(Unknown,empt_lscomp2(t2))
  }
}

def empt_lscomp1(ls:List[i64],vals:i64) : List[List[ConflictSet]] {
  ls.case[i64]{
    Nil => Nil,
    Cons(n, t1) =>  Cons(empt_lscomp2(enum_from_to(1, vals)),empt_lscomp1(t1,vals))
  }
}

def empty_table(csp:CSP) : List[List[ConflictSet]] {
  csp.case {
    CSP(vars, vals, rel) => Cons(Nil,empt_lscomp1(enum_from_to(1, vars),vals))
  }
}

//fill_table
def fill_lscomp2(ls:List[i64],varrr:i64) : List[Pair[i64,i64]]{
  ls.case[i64]{ 
    Nil => Nil, 
    Cons(valll, t2) => Cons(Tup(varrr, valll),fill_lscomp2(t2,varrr))
  }
}

def fill_lscomp1(ls:List[i64],vals:i64) : List[List[Pair[i64,i64]]] {
  ls.case[i64]{ 
    Nil => Nil, 
    Cons(varrr,t1) => Cons(fill_lscomp2(enum_from_to(1, vals),varrr),fill_lscomp1(t1,vals))
  }
}

def fill_zip_with(f:Fun[ConflictSet,Fun[Pair[i64,i64],ConflictSet]],x:List[ConflictSet],y:List[Pair[i64,i64]]) : 
  List[ConflictSet] {
  x.case[ConflictSet]{
    Nil => Nil,
    Cons(c,cs) => y.case[Pair[i64,i64]]{
      Nil => Nil,
      Cons(p,ps) => Cons(f.Ap[ConflictSet,Fun[Pair[i64,i64],ConflictSet]](c).Ap[Pair[i64,i64],ConflictSet](p),fill_zip_with(f,cs,ps))
    }
  }
}

def fill_zip_with2(f:Fun[List[ConflictSet],Fun[List[Pair[i64,i64]],List[ConflictSet]]], tbl:List[List[ConflictSet]], 
    ls:List[List[Pair[i64,i64]]]) : List[List[ConflictSet]]{
      tbl.case[List[ConflictSet]]{
        Nil => Nil,
        Cons(cs,css) => ls.case[List[Pair[i64,i64]]] {
          Nil => Nil,
          Cons(ps,pss) => Cons(f.Ap[List[ConflictSet],Fun[List[Pair[i64,i64]],List[ConflictSet]]](cs).Ap[List[Pair[i64,i64]],List[ConflictSet]](ps),fill_zip_with2(f,css,pss))
        }
      }
}

def fill_table(s:List[Assign], csp:CSP, tbl:List[List[ConflictSet]]):List[List[ConflictSet]] {
  s.case[Assign]{
    Nil => tbl,
    Cons(as,as_) => as.case { Assign(var_, val_) => 
      csp.case{
        CSP(vars, vals, rel) => 
          let f4: Fun[ConflictSet,Fun[Pair[i64,i64],ConflictSet]] = new { Ap(cs) => new {Ap(varval) => 
            varval.case[i64,i64]{
              Tup(varr,vall) => cs.case{
                Known(vs) => cs,
                Unknown => not(rel.Ap[Assign,Fun[Assign,Bool]](Assign(var_,val_)).Ap[Assign,Bool](Assign(varr,vall))).case{
                  True => Known(Cons(var_,Cons(varr,Nil))),
                  False => cs
                }
              }
          }}};
          fill_zip_with2(new { Ap(x) => new { Ap(y) => fill_zip_with(f4, x, y) } }, tbl, fill_lscomp1(enum_from_to(var_ + 1, vars),vals))
      }


    }
  }

}

// lookup_cache

def lookup_map(f:Fun[Node[Pair[List[Assign],List[List[ConflictSet]]]],Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]],l:List[Node[Pair[List[Assign],List[List[ConflictSet]]]]]) : List[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]]{
  l.case[Node[Pair[List[Assign],List[List[ConflictSet]]]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[List[Assign],List[List[ConflictSet]]]],Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]](n),lookup_map(f,ns))
  }
}

def lookup_map_tree(f:Fun[Pair[List[Assign],List[List[ConflictSet]]],Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]],t:Node[Pair[List[Assign],List[List[ConflictSet]]]]) : Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]{
  t.case[Pair[List[Assign],List[List[ConflictSet]]]] {
    Node(p,ps) => 
      Node(f.Ap[Pair[List[Assign],List[List[ConflictSet]]],Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]](p),
        lookup_map(new { Ap(x) => lookup_map_tree(f,x) },ps))
  }
}

def lookup_at_index(ind:i64,ls:List[ConflictSet]) : ConflictSet{
  ls.case[ConflictSet]{
    Nil => Unknown, // runtime error,
    Cons(c,cs) => if ind==0 { c } else { lookup_at_index(ind-1,cs) }
  }
}

def lookup_head(tbl:List[List[ConflictSet]]) : List[ConflictSet]{
  tbl.case[List[ConflictSet]]{
    Nil => Nil, // runtime error
    Cons(cs,css) => cs
  }
}

def lookup_cache(csp:CSP, t:Node[Pair[List[Assign],List[List[ConflictSet]]]]) :
  Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]{
  let f5: Fun[CSP,
    Fun[Pair[List[Assign],List[List[ConflictSet]]],Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]] =
    new { Ap(csp) => new { Ap(tp) =>
    tp.case[List[Assign],List[List[ConflictSet]]]{
    Tup(ls,tbl) => ls.case[Assign]{
      Nil=> Tup(Tup(Nil,Unknown),tbl),
      Cons(a,as_) =>
        let table_entry: ConflictSet = lookup_at_index(value(a) - 1, lookup_head(tbl));
        let cs: ConflictSet = table_entry.case{
          Unknown => check_complete(csp, Cons(a,as_)),
          Known(vals) => table_entry
        };
        Tup(Tup(Cons(a,as_), cs), tbl)
    }
  }
  }};
  lookup_map_tree(new { Ap(x) => 
    f5.Ap[CSP,Fun[Pair[List[Assign],List[List[ConflictSet]]],Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]](csp)
      .Ap[Pair[List[Assign],List[List[ConflictSet]]],Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]](x) }, t)
}


// cache_checks

def checks_map(f:Fun[Node[List[Assign]],Node[Pair[List[Assign],List[List[ConflictSet]]]]],ls:List[Node[List[Assign]]]) : List[Node[Pair[List[Assign],List[List[ConflictSet]]]]] {
  ls.case[Node[List[Assign]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[List[Assign]],Node[Pair[List[Assign],List[List[ConflictSet]]]]](n),checks_map(f,ns))
  }
}

def checks_tail(ls:List[List[ConflictSet]]) : List[List[ConflictSet]]{
  ls.case[List[ConflictSet]]{
    Nil => Nil, // runtime error
    Cons(l,ls) => ls
  }
}

def cache_checks(csp:CSP, tbl:List[List[ConflictSet]], n:Node[List[Assign]]) : Node[Pair[List[Assign],List[List[ConflictSet]]]] {
  n.case[List[Assign]]{
    Node(s,cs) => Node(Tup(s,tbl),checks_map(new { Ap(x) => cache_checks(csp, fill_table(s, csp, checks_tail(tbl)), x) }, cs))
  }
}

//bm
def bm_fst(x:Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]) : Pair[List[Assign],ConflictSet] {
  x.case[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]{
    Tup(p,ls) => p
  }
}

def bm_map(f:Fun[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]],Node[Pair[List[Assign],ConflictSet]]],ls:List[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]]) : List[Node[Pair[List[Assign],ConflictSet]]]{
  ls.case[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]],Node[Pair[List[Assign],ConflictSet]]](n),bm_map(f,ns))
  }
}

def bm_map_tree(f:Fun[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]],Pair[List[Assign],ConflictSet]],t:Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]) : Node[Pair[List[Assign],ConflictSet]]{
t.case[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]] {
    Node(p,ps) => 
      Node(f.Ap[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]],Pair[List[Assign],ConflictSet]](p),
        bm_map(new { Ap(x) => bm_map_tree(f,x) },ps))
  }
}

def bm(csp:CSP, t:Node[List[Assign]])  : Node[Pair[List[Assign],ConflictSet]] {
  bm_map_tree(new { Ap(x) => bm_fst(x) }, lookup_cache(csp, cache_checks(csp, empty_table(csp), t))) 
}

// combine

def in_list(i:i64,ls:List[i64]) : Bool{
  ls.case[i64]{
    Nil => False,
    Cons(j,js) => if i==j { True } else { in_list(i,js) }
  }
}

def not_elem(i:i64,ls:List[i64]) : Bool{
  not(in_list(i,ls))
}

def append(l1:List[i64],l2:List[i64]):List[i64]{
  l1.case[i64]{
    Nil => l2,
    Cons(i,is) => Cons(i,append(is,l2))
  }
}

def delete_by(f:Fun[i64,Fun[i64,Bool]], x:i64, ys:List[i64]) : List[i64] {
  ys.case[i64]{
    Nil => Nil,
    Cons(y,ys) => f.Ap[i64,Fun[i64,Bool]](x).Ap[i64,Bool](y).case{
      True => ys,
      False => Cons(y,delete_by(f,x,ys))
    }
  }
}

def nub_by(f:Fun[i64,Fun[i64,Bool]], ls:List[i64]) : List[i64] {
  ls.case[i64]{
    Nil => Nil,
    Cons(h,t) => Cons(h,nub_by(f,filter_union(new { Ap(y) => not(f.Ap[i64,Fun[i64,Bool]](h).Ap[i64,Bool](y)) },t)))
  }
}

def filter_union(f:Fun[i64,Bool],ls:List[i64]) : List[i64]{
  ls.case[i64]{
    Nil => Nil,
    Cons(i,is) => f.Ap[i64,Bool](i).case{
      True => Cons(i,filter_union(f,is)),
      False => filter_union(f,is)
    }
  }
}

def foldl(f:Fun[List[i64],Fun[i64,List[i64]]], a:List[i64], xs:List[i64]) : List[i64] { 
  xs.case[i64]{
    Nil => a,
    Cons(h,t) => foldl(f,f.Ap[List[i64],Fun[i64,List[i64]]](a).Ap[i64,List[i64]](h),t)
  }
}

def union_by(f:Fun[i64,Fun[i64,Bool]],l1:List[i64],l2:List[i64]) : List[i64]{
  append(l1, foldl(new { Ap(acc) => new { Ap(y) => delete_by(f, y, acc) } }, nub_by(f, l2), l1))
}

def union(l1:List[i64],l2:List[i64]):List[i64]{
  union_by(new { Ap(x) => new { Ap(y) => eq(x,y) } },l1,l2) 
}

def combine(ls:List[Pair[List[Assign],ConflictSet]], acc:List[i64]) : List[i64]{
  ls.case[Pair[List[Assign],ConflictSet]]{
    Nil => acc,
    Cons(p,css) => p.case[List[Assign],ConflictSet]{
      Tup(s,cs) => cs.case{
        Known(cs) => not_elem(max_level(s), cs).case{
          True => cs,
          False => combine(css, union(cs, acc))
        },
        Unknown => acc
      }
    }
  }
}

//bj_

def bj_(csp:CSP, t:Node[Pair[List[Assign],ConflictSet]]):Node[Pair[List[Assign],ConflictSet]] {
  let f7: Fun[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]] = 
    new { Ap(tp2) => new { Ap(chs) =>
      tp2.case[List[Assign],ConflictSet]{ Tup(a,conf) => conf.case{
        Known(cs) => Node(Tup(a, Known(cs)), chs),
        Unknown => 
          let cs_: ConflictSet = Known(combine(bj_map(new { Ap(x) => bj_label(x) }, chs), Nil));
         known_conflict(cs_).case{
            True => Node(Tup(a, cs_), Nil),
            False => Node(Tup(a, cs_), chs)
          }
      }}
    } };
    bj_fold_tree(f7, t) 
}

// bj
def bj_label(n:Node[Pair[List[Assign],ConflictSet]]) : Pair[List[Assign],ConflictSet]{
  n.case[Pair[List[Assign],ConflictSet]]{
    Node(l,cs) => l
  }
}

def bj_map(f:Fun[Node[Pair[List[Assign],ConflictSet]],Pair[List[Assign],ConflictSet]],l:List[Node[Pair[List[Assign],ConflictSet]]]) : List[Pair[List[Assign],ConflictSet]]{
  l.case[Node[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[List[Assign],ConflictSet]],Pair[List[Assign],ConflictSet]](n),bj_map(f,ns))
  }
}

def bj_map2(f:Fun[Node[Pair[List[Assign],ConflictSet]],Node[Pair[List[Assign],ConflictSet]]],l:List[Node[Pair[List[Assign],ConflictSet]]]) : List[Node[Pair[List[Assign],ConflictSet]]] {
  l.case[Node[Pair[List[Assign],ConflictSet]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[List[Assign],ConflictSet]],Node[Pair[List[Assign],ConflictSet]]](n),bj_map2(f,ns))
  }
}

def bj_fold_tree(f:Fun[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]],t:Node[Pair[List[Assign],ConflictSet]]) : Node[Pair[List[Assign],ConflictSet]]{
  t.case[Pair[List[Assign],ConflictSet]]{
    Node(l,c) => f.Ap[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]](l).Ap[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]](bj_map2(new { Ap(x) => bj_fold_tree(f,x)},c))
  }
}

def bj(csp:CSP, t:Node[Pair[List[Assign],ConflictSet]]):Node[Pair[List[Assign],ConflictSet]] {
  let f6: Fun[Pair[List[Assign],ConflictSet],Fun[List[Node[Pair[List[Assign],ConflictSet]]],Node[Pair[List[Assign],ConflictSet]]]] = 
    new { Ap(tp2) => new { Ap(chs) => 
      tp2.case[List[Assign],ConflictSet]{
        Tup(a,conf) => conf.case{
          Known(cs) => Node(Tup(a, Known(cs)), chs),
          Unknown =>  Node(Tup(a, Known(combine(bj_map(new { Ap(x) => bj_label(x) }, chs), Nil))), chs) 
        }
      }
    } };
    bj_fold_tree(f6, t) 
}

def bjbt(csp:CSP, t:Node[List[Assign]]) : Node[Pair[List[Assign],ConflictSet]]{
  bj(csp, bt(csp, t)) 
}


def bjbt_(csp:CSP, t:Node[List[Assign]]) : Node[Pair[List[Assign],ConflictSet]] {
 bj_(csp, bt(csp, t)) 
}

def collect(ls:List[ConflictSet]) : List[i64]{
  ls.case[ConflictSet]{
    Nil=>Nil,
    Cons(conf,css) => conf.case{
      Known(cs) => union(cs, collect(css)),
      Unknown => Nil
    }
  }
}

//domain_wipeout

def wipe_all(f:Fun[ConflictSet,Bool],ls:List[ConflictSet]):Bool{
  ls.case[ConflictSet]{
    Nil => True,
    Cons(c,cs) => f.Ap[ConflictSet,Bool](c).case{
      True => wipe_all(f,cs),
      False => False 
    }
  }
}

def wipe_lscomp1(ls:List[List[ConflictSet]]) : List[List[ConflictSet]] {
  ls.case[List[ConflictSet]]{
    Nil => Nil,
    Cons(vs,t1) => wipe_all(new{ Ap(x) => known_conflict(x)}, vs).case{
      True => Cons(vs,wipe_lscomp1(t1)),
      False => wipe_lscomp1(t1)
    }
  }
}

def wipe_null_(ls:List[List[ConflictSet]]):Bool{
  ls.case[List[ConflictSet]]{
    Nil => True,
    Cons(l,ls) => False
  }
}

def wipe_head(ls:List[List[ConflictSet]]):List[ConflictSet]{
  ls.case[List[ConflictSet]]{
    Nil => Nil, //runtime error
    Cons(l,ls)=>l
  }
}

def wipe_map(f:Fun[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]],Node[Pair[List[Assign],ConflictSet]]],ls:List[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]]) : List[Node[Pair[List[Assign],ConflictSet]]]{
  ls.case[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]]{
    Nil => Nil,
    Cons(n,ns) => Cons(f.Ap[Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]],Node[Pair[List[Assign],ConflictSet]]](n),wipe_map(f,ns))
  
  }
}

def wipe_map_tree(f:Fun[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]],Pair[List[Assign],ConflictSet]],t:Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]) : Node[Pair[List[Assign],ConflictSet]]{
  t.case[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]] {
    Node(l,c) => 
      Node(f.Ap[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]],Pair[List[Assign],ConflictSet]](l),
        wipe_map(new { Ap(x) => wipe_map_tree(f,x) },c))
  }
}

def domain_wipeout(csp:CSP, t:Node[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]]):Node[Pair[List[Assign],ConflictSet]] {
  csp.case{ 
    CSP(vars, vals, rel) =>  
      let f8: Fun[Pair[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]],Pair[List[Assign],ConflictSet]] = 
        new { Ap(tp2) =>
          tp2.case[Pair[List[Assign],ConflictSet],List[List[ConflictSet]]]{
            Tup(p, tbl) => p.case[List[Assign],ConflictSet]{
              Tup(as_, cs) =>
                let wiped_domains: List[List[ConflictSet]]= wipe_lscomp1(tbl);
                let cs_: ConflictSet = wipe_null_(wiped_domains).case{
                  True => cs,
                  False => Known(collect(wipe_head(wiped_domains)))
                };
                Tup(as_, cs_)
          }}
        };
        wipe_map_tree(f8, t) 
  }
}

def fc(csp:CSP, t:Node[List[Assign]]): Node[Pair[List[Assign],ConflictSet]]{
  domain_wipeout(csp, lookup_cache(csp, cache_checks(csp, empty_table(csp), t))) 
}

def list_len(l:List[List[Assign]]) : i64{
  l.case[List[Assign]]{
    Nil => 0,
    Cons(l,ls) => 1+list_len(ls)
  }
}

def try_(n:i64, algorithm:Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]]) : i64 {
  list_len(search(algorithm, queens(n))) 
}

def test_map(f:Fun[Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]],i64],l:List[Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]]]) : List[i64]{
  l.case[Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]]]{
    Nil => Nil,
    Cons(g,gs) => Cons(f.Ap[Fun[CSP,Fun[Node[List[Assign]],Node[Pair[List[Assign],ConflictSet]]]],i64](g),test_map(f,gs))
  }
}

def test_constraints_nofib(n:i64) : List[i64] {
  test_map(new { Ap(x) => try_(n, x) }, 
    Cons(new { Ap(csp) => new { Ap(n) => bt(csp,n) }},
      Cons(new { Ap(csp) => new { Ap(n) => bm(csp,n) }},
        Cons(new { Ap(csp) => new { Ap(n) => bjbt(csp,n) }},
          Cons(new { Ap(csp) => new { Ap(n) => bjbt_(csp,n) }},
            Cons(new { Ap(csp) => new { Ap(n) => fc(csp,n) }}, 
              Nil))))))
}

def head(l:List[i64]):i64 { 
  l.case[i64]{
    Nil => -1,
    Cons(x,xs) => x
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters == 1{
    let res: List[i64] = test_constraints_nofib(n);
    println_i64(head(res));
    0
  }else{
    let res: List[i64] = test_constraints_nofib(n);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64{
  main_loop(iters,n)
}
