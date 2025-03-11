data Vec { Vec(x:i64,y:i64) }
data Vec4 { Vec4(x:i64,y:i64,z:i64,w:i64) }
data List[A] { Nil,Cons(a:A,as:List[A]) }
codata Fun[A,B] { Ap(a:A) : B }

def vec_add(v1:Vec, v2:Vec) : Vec {
  v1.case{
    Vec(x1,y1) => v2.case{
      Vec(x2,y2) => Vec(x1+x2,y1+y2)
    }
  }
}

def vec_sub(v1:Vec, v2:Vec) : Vec{
  v1.case{
    Vec(x1,y1) => v2.case{
      Vec(x2,y2) => Vec(x1-x2,y1-y2)
    }
  }
}

def scale_vec2(v:Vec, a:i64, b:i64) : Vec{
  v.case{
    Vec(x,y) => Vec((x*a)/b,(y*a)/b)
  }
}

def p_tile() : List[Vec4] {
  Cons(Vec4(0, 3, 3, 4),Cons(Vec4(3, 4, 0, 8),Cons(Vec4(0, 8, 0, 3),Cons(Vec4(6, 0, 4, 4),Cons(Vec4(4, 5, 4, 10),
    Cons(Vec4(4, 10, 7, 6),Cons(Vec4(7, 6, 4, 5),Cons(Vec4(11, 0, 10, 4),Cons(Vec4(10, 4, 9, 6),Cons(Vec4(9, 6, 8, 8),
      Cons(Vec4(8, 8, 4, 13),Cons(Vec4(4, 13, 0, 16),Cons(Vec4(0, 16, 6, 15),Cons(Vec4(6, 15, 8, 16),
        Cons(Vec4(8, 16, 12, 12),Cons(Vec4(12, 12, 16, 12),Cons(Vec4(10, 16, 12, 14),Cons(Vec4(12, 14, 16, 13),
          Cons(Vec4(12, 16, 13, 15),Cons(Vec4(13, 15, 16, 14),Cons(Vec4(14, 16, 16, 15),Cons(Vec4(8, 12, 16, 10),
            Cons(Vec4(8, 8, 12, 9),Cons(Vec4(12, 9, 16, 8),Cons(Vec4(9, 6, 12, 7),Cons(Vec4(12, 7, 16, 6),
              Cons(Vec4(10, 4, 13, 5),Cons(Vec4(13, 5, 16, 4),Cons(Vec4(11, 0, 14, 2),Cons(Vec4(14, 2, 16, 2),
                Nil)))))))))))))))))))))))))))))) 
}


def q_tile() : List[Vec4] {
  Cons(Vec4(0, 8, 4, 7), Cons(Vec4(4, 7, 6, 7), Cons(Vec4(6, 7, 8, 8), Cons(Vec4(8, 8, 12, 10),Cons(Vec4(12, 10, 16, 16), 
    Cons(Vec4(0, 12, 3, 13), Cons(Vec4(3, 13, 5, 14), Cons(Vec4(5, 14, 7, 15), Cons(Vec4(7, 15, 8, 16), 
      Cons(Vec4(2, 16, 3, 13), Cons(  Vec4(4, 16, 5, 14), Cons(Vec4(6, 16, 7, 15), Cons(Vec4(0, 10, 7, 11), 
        Cons(Vec4(9, 13, 8, 15), Cons(Vec4(8, 15, 11, 15), Cons(Vec4(11, 15, 9, 13), Cons(Vec4(10, 10, 8, 12), 
          Cons(Vec4(8, 12, 12, 12), Cons(Vec4(12, 12, 10, 10), Cons(Vec4(2, 0, 4, 5), Cons(Vec4(4, 5, 4, 7), 
            Cons(Vec4(4, 0, 6, 5), Cons(Vec4(6, 5, 6, 7), Cons(Vec4(6, 0, 8, 5), Cons(Vec4(8, 5, 8, 8), 
              Cons(Vec4(10, 0, 14, 11), Cons(Vec4(12, 0, 13, 4), Cons(Vec4(13, 4, 16, 8), Cons(Vec4(16, 8, 15, 10), 
                Cons(Vec4(15, 10, 16, 16), Cons(  Vec4(13, 0, 16, 6), Cons(Vec4(14, 0, 16, 4), Cons(Vec4(15, 0, 16, 2),
                  Cons(Vec4(0, 0, 8, 0), Cons(Vec4(12, 0, 16, 0), Cons(  Vec4(0, 0, 0, 8), Cons(Vec4(0, 12, 0, 16), 
                    Nil)))))))))))))))))))))))))))))))))))))
}

def r_tile() : List[Vec4] {
  Cons(Vec4(0, 0, 8, 8), Cons(Vec4(12, 12, 16, 16), Cons(Vec4(0, 4, 5, 10), Cons(Vec4(0, 8, 2, 12), 
    Cons(Vec4(0, 12, 1, 14), Cons(  Vec4(16, 6, 11, 10), Cons(Vec4(11, 10, 6, 16), Cons(Vec4(16, 4, 14, 6),
      Cons(Vec4(14, 6, 8, 8), Cons(Vec4(8, 8, 5, 10), Cons(  Vec4(5, 10, 2, 12), Cons(Vec4(2, 12, 0, 16), 
        Cons(Vec4(16, 8, 12, 12), Cons(Vec4(12, 12, 11, 16), Cons(Vec4(1, 1, 4, 0), Cons(  Vec4(2, 2, 8, 0), 
          Cons(Vec4(3, 3, 8, 2), Cons(Vec4(8, 2, 12, 0), Cons(Vec4(5, 5, 12, 3), Cons(Vec4(12, 3, 16, 0), 
            Cons(Vec4(11, 16, 12, 12), Cons(Vec4(12, 12, 16, 8), Cons(Vec4(13, 13, 16, 10), 
              Cons(Vec4(14, 14, 16, 12), Cons(Vec4(15, 15, 16, 14), 
                Nil)))))))))))))))))))))))))
}

def s_tile() : List[Vec4] {
  Cons(Vec4(0, 0, 4, 2), Cons(Vec4(4, 2, 8, 2), Cons(Vec4(8, 2, 16, 0), Cons(Vec4(0, 4, 2, 1), Cons(Vec4(0, 6, 7, 4), 
    Cons(Vec4(0, 8, 8, 6), Cons(Vec4(0, 10, 7, 8), Cons(Vec4(0, 12, 7, 10), Cons(Vec4(0, 14, 7, 13), 
      Cons(Vec4(13, 13, 16, 14), Cons(  Vec4(14, 11, 16, 12), Cons(Vec4(15, 9, 16, 10), Cons(Vec4(16, 0, 10, 4), 
        Cons(Vec4(10, 4, 8, 6), Cons(Vec4(8, 6, 7, 8), Cons(  Vec4(7, 8, 7, 13), Cons(Vec4(7, 13, 8, 16), 
          Cons(Vec4(12, 16, 13, 13), Cons(Vec4(13, 13, 14, 11), Cons(Vec4(14, 11, 15, 9), Cons(Vec4(15, 9, 16, 8), 
            Cons(Vec4(10, 16, 11, 10), Cons(Vec4(12, 4, 10, 6), Cons(Vec4(10, 6, 12, 7), Cons(Vec4(12, 7, 12, 4), 
              Cons(Vec4(15, 5, 13, 7), Cons(Vec4(13, 7, 15, 8), Cons(Vec4(15, 8, 15, 5), 
                Nil))))))))))))))))))))))))))))
}

def nil(a:Vec, b:Vec, c:Vec) :List[Vec4] { Nil }

def tup2(a_b:Vec, c_d:Vec) : Vec4 {
  a_b.case{
    Vec(a,b) => c_d.case{
      Vec(c,d) => Vec4(a,b,c,d)
    }
  }
}

def grid_lscomp(ls:List[Vec4],m:i64,n:i64,a:Vec,b:Vec,c:Vec) : List[Vec4]{
  ls.case[Vec4]{
    Nil => Nil,
    Cons(v,t) => v.case{
      Vec4(x0, y0, x1, y1) => 
        Cons(
          tup2(
            vec_add(vec_add(a, scale_vec2(b, x0, m)), scale_vec2(c, y0, n)),
            vec_add(vec_add(a, scale_vec2(b, x1, m)), scale_vec2(c, y1, n))),
          grid_lscomp(t,m,n,a,b,c))
    }
  }
}

def grid(m:i64, n:i64, segments:List[Vec4], a:Vec, b:Vec, c:Vec): List[Vec4]{
  grid_lscomp(segments,m,n,a,b,c) 
}

def rot(p:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], a:Vec, b:Vec, c:Vec): List[Vec4] {
  p.Ap[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]](vec_add(a, b))
    .Ap[Vec,Fun[Vec,List[Vec4]]](c)
    .Ap[Vec,List[Vec4]](vec_sub(Vec(0, 0), b))
}

def append_vec4(l1:List[Vec4],l2:List[Vec4]) : List[Vec4]{
  l1.case[Vec4]{
    Nil => l2,
    Cons(v,vs) => Cons(v,append_vec4(vs,l2))
  }
}

def beside(m:i64, n:i64, p:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], q:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  a:Vec, b:Vec, c:Vec) : List[Vec4] {
    append_vec4(p.Ap[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]](a)
      .Ap[Vec,Fun[Vec,List[Vec4]]](scale_vec2(b, m, m + n))
      .Ap[Vec,List[Vec4]](c) 
      ,
      q.Ap[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]](vec_add(a, scale_vec2(b, m, m + n)))
        .Ap[Vec,Fun[Vec,List[Vec4]]](scale_vec2(b, n, n + m))
        .Ap[Vec,List[Vec4]](c) )
}


def above(m:i64, n:i64, p:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  q:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], a:Vec, b:Vec, c:Vec) : List[Vec4] {
    append_vec4( 
      p.Ap[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]](vec_add(a, scale_vec2(c, n, m + n)))
        .Ap[Vec,Fun[Vec,List[Vec4]]](b)
        .Ap[Vec,List[Vec4]](scale_vec2(c, m, n + m)),
        q.Ap[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]](a)
          .Ap[Vec,Fun[Vec,List[Vec4]]](b)
          .Ap[Vec,List[Vec4]](scale_vec2(c, n, m + n)))
}

def tile_to_grid(arg:List[Vec4], arg2:Vec, arg3:Vec, arg4:Vec):List[Vec4] {
  grid(16, 16, arg, arg2, arg3, arg4) 
}
//
def p(arg:Vec, q6:Vec, q7:Vec): List[Vec4]{
  tile_to_grid(p_tile(), arg, q6, q7) 
}

def q(arg:Vec, q6:Vec, q7:Vec): List[Vec4] {
  tile_to_grid(q_tile(), arg, q6, q7) 
}

def r(arg:Vec, q6:Vec, q7:Vec) : List[Vec4] {
  tile_to_grid(r_tile(), arg, q6, q7) 
}

def s(arg:Vec, q6:Vec, q7:Vec) :List[Vec4]{
  tile_to_grid(s_tile(), arg, q6, q7) 
}

def quartet(a:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  b:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  c:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  d:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  arg:Vec, a6:Vec, a7:Vec) : List[Vec4] {
    above(1,1,
      new { Ap(p5) => new { Ap(p6) => new { Ap(p7) => beside(1, 1, a, b, p5, p6, p7) }}},
      new { Ap(p5) => new { Ap(p6) => new { Ap(p7) => beside(1, 1, c, d, p5, p6, p7) }}},
      arg,a6,a7 )
}

def t(arg:Vec, q6:Vec, q7:Vec) : List[Vec4]{
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => p(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => q(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => r(a,b,c) }}},
    new { Ap(a) => new { Ap(b) => new { Ap(c) => s(a,b,c) }}}, 
    arg, q6, q7) 
}


def cycle_(p1:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], arg:Vec, p3:Vec, p4:Vec):List[Vec4]{
  quartet(p1,
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => 
          rot( new { Ap(a) => new { Ap(b) => new { Ap(c) => rot(p1, a, b, c) }}},a,b,c) 
        }}},
        a,b,c)
    }}},
    new { Ap(a) => new { Ap(b) => new { Ap(c) => rot(p1, a, b, c) }}},
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => rot(p1, a, b, c) }}},
        a,b,c)
    }}},
    arg,p3,p4)
}

def u(arg:Vec, p2:Vec, p3:Vec) : List[Vec4]{
  cycle_(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => q(a,b,c)}}}, 
        a, b, c)
    }}}, 
    arg, p2, p3) 
}

def side1(arg:Vec, q6:Vec, q7:Vec) : List[Vec4]{
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => nil(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => nil(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => t(a,b,c) } } }, 
        a, b, c) 
    }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => t(a,b,c) }}},
    arg, q6, q7) 
}


def side2(arg:Vec, q6:Vec, q7:Vec) : List[Vec4]{
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => side1(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => side1(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => t(a,b,c) }}}, 
        a, b, c) 
    }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => t(a,b,c) }}}, 
    arg, q6, q7) 
}

def corner1(arg:Vec, q6:Vec, q7:Vec):List[Vec4] {
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => nil(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => nil(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => nil(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => u(a,b,c) }}}, 
    arg, q6, q7) 
}

def corner2(arg:Vec, q6:Vec, q7:Vec) : List[Vec4]{
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => corner1(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => side1(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => side1(a,b,c) }}}, 
        a, b, c) 
    }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => u(a,b,c) }}}, 
    arg, q6, q7) 
}

def pseudocorner(arg:Vec, q6:Vec, q7:Vec) :List[Vec4]{
  quartet(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => corner2(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => side2(a,b,c) }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => side2(a,b,c) }}}, 
        a, b, c) 
    }}}, 
    new { Ap(a) => new { Ap(b) => new { Ap(c) => 
      rot(
        new { Ap(a) => new { Ap(b) => new { Ap(c) => t(a,b,c) }}}, 
        a, b, c) 
    }}}, 
    arg, q6, q7) 
}

def pseudolimit(arg:Vec, p2:Vec, p3:Vec) :List[Vec4]{
  cycle_(
    new { Ap(a) => new { Ap(b) => new { Ap(c) => pseudocorner(a,b,c) }}}, 
    arg, p2, p3) 
}

def nonet(p1:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], p2:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  p3:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], p4:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  p5:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], p6:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  p7:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], p8:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], 
  p9:Fun[Vec,Fun[Vec,Fun[Vec,List[Vec4]]]], arg1:Vec, arg2:Vec) :Fun[Vec,List[Vec4]] {
    new { Ap(v) => 
      above(1,2,
        new { Ap(b5) => new { Ap(b6) => new { Ap(b7) => 
          beside(1, 2, p1, 
            new { Ap(b5) => new { Ap(b6) => new { Ap(b7) => 
              beside(1, 1, p2, p3, b5, b6, b7) 
            }}},
            b5, b6, b7) 
        }}},
        new { Ap(a1) => new { Ap(a2) => new { Ap(a3) => 
          above(1,1,
            new { Ap(b5) => new { Ap(b6) => new { Ap(b7) => 
              beside(1, 2, p4, 
                new { Ap(b5) => new { Ap(b6) => new { Ap(b7) => beside(1, 1, p5, p6, b5, b6, b7) }}}, 
                b5, b6, b7) 
            }}},
            new { Ap(b5) => new { Ap(b6) => new{ Ap(b7) => 
              beside(1, 2, p7, 
                new { Ap(b5) => new { Ap(b6) => new { Ap(b7) => beside(1, 1, p8, p9, b5, b6, b7) }}},
                b5, b6, b7) 
            }}},
            a1,a2,a3) 
        }}},
        arg1,arg2,v)
    }
}

def enum_from_to(from:i64,t:i64) : List[i64]{
  if from<=t{
    Cons(from,enum_from_to(from+1,t))
  }else{
    Nil
  }
}

def min(i1:i64,i2:i64) : i64{
  if i1<i2{
    i1
  }else {
    i2
  }
}

def map(f:Fun[i64,List[Vec4]],l:List[i64]):List[List[Vec4]]{
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => Cons(f.Ap[i64,List[Vec4]](i),map(f,is))
  }
}

def test_fish_nofib(n:i64) : List[List[Vec4]]{
  map(
    new { Ap(i) => 
      let n: i64 = min(0, i);
      pseudolimit(Vec(0, 0), Vec(640 + n, 0), Vec(0, 640 + n)) 
    },     
    enum_from_to(0, n))
}

def first(l:List[List[Vec4]]) : i64{
  l.case[List[Vec4]]{
    Nil => -1,
    Cons(l,ls) => l.case[Vec4]{
      Nil => -1,
      Cons(v,vs) => v.case{
        Vec4(a,b,c,d) => a
      }
    }
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==1{
    let res: List[List[Vec4]] = test_fish_nofib(n);
    println_i64(first(res));
    0
  }else{
    let res: List[List[Vec4]] = test_fish_nofib(n);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64{
  main_loop(iters,n)
}
