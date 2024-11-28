codata StreamInt { Hd : Int, Tl : StreamInt }
data ListInt { Nil, Cons(x:Int,xs:ListInt) }

def repeat(x: Int) : StreamInt := cocase { Hd => x, Tl => repeat(x) };
def const1() : StreamInt := cocase { Hd => 1, Tl => const1() };

def take(n:Int,x:StreamInt) : ListInt := ifz(n,Nil,Cons(x.Hd,take(n-1,x.Tl)));
def sumList(ls:ListInt) : Int := ls.case { Nil=>0, Cons(x:Int, xs:ListInt) => x+(sumList(xs)) };

def main() : Int := sumList(take(5,repeat(5)));
