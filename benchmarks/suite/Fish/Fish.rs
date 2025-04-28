use std::{
    ops::{Add, Sub},
    rc::Rc,
};

#[derive(Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, as_) => 1 + as_.len(),
        }
    }

    fn head(self) -> A {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(a, _) => a,
        }
    }

    fn map<B>(self, f: impl Fn(A) -> B) -> List<B>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => List::Cons(f(a), Rc::new(Rc::unwrap_or_clone(as_).map(f))),
        }
    }

    fn append(self, other: List<A>) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => other,
            List::Cons(a, as_) => List::Cons(a, Rc::new(Rc::unwrap_or_clone(as_).append(other))),
        }
    }

    fn from_iterator<T>(t: T) -> List<A>
    where
        T: Iterator<Item = A>,
    {
        let mut ls = List::Nil;
        for it in t {
            ls = List::Cons(it, Rc::new(ls));
        }
        ls
    }
}

impl<A, const N: usize> From<[A; N]> for List<A> {
    fn from(arr: [A; N]) -> List<A> {
        let mut ls = List::Nil;
        for a in arr {
            ls = List::Cons(a, Rc::new(ls));
        }
        ls
    }
}

#[derive(Debug, Clone)]
struct Vec4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Vec4 {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Vec4 {
        Vec4 { x, y, z, w }
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn scale(self, a: i64, b: i64) -> Vec2 {
        Vec2 {
            x: (self.x * a) / b,
            y: (self.x * a) / b,
        }
    }

    fn tup2(self, other: Vec2) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: other.x,
            w: other.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn p_tile() -> List<Vec4> {
    let p5: List<Vec4> = [
        Vec4::new(10, 4, 13, 5),
        Vec4::new(13, 5, 16, 4),
        Vec4::new(11, 0, 14, 2),
        Vec4::new(14, 2, 16, 2),
    ]
    .into();
    let p4 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(8, 12, 16, 10),
        Vec4::new(8, 8, 12, 9),
        Vec4::new(12, 9, 16, 8),
        Vec4::new(9, 6, 12, 7),
        Vec4::new(12, 7, 16, 6),
    ])
    .append(p5);
    let p3 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(10, 16, 12, 14),
        Vec4::new(12, 14, 16, 13),
        Vec4::new(12, 16, 13, 15),
        Vec4::new(13, 15, 16, 14),
        Vec4::new(14, 16, 16, 15),
    ])
    .append(p4);
    let p2 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(4, 13, 0, 16),
        Vec4::new(0, 16, 6, 15),
        Vec4::new(6, 15, 8, 16),
        Vec4::new(8, 16, 12, 12),
        Vec4::new(12, 12, 16, 12),
    ])
    .append(p3);
    let p1 = <[Vec4; 6] as Into<List<Vec4>>>::into([
        Vec4::new(4, 10, 7, 6),
        Vec4::new(7, 6, 4, 5),
        Vec4::new(11, 0, 10, 4),
        Vec4::new(10, 4, 9, 6),
        Vec4::new(9, 6, 8, 8),
        Vec4::new(8, 8, 4, 13),
    ])
    .append(p2);
    let p = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 3, 3, 4),
        Vec4::new(3, 4, 0, 8),
        Vec4::new(0, 8, 0, 3),
        Vec4::new(6, 0, 4, 4),
        Vec4::new(4, 5, 4, 10),
    ])
    .append(p1);
    p
}

fn q_tile() -> List<Vec4> {
    let q7 = List::Cons(
        Vec4::new(0, 0, 0, 8),
        Rc::new(List::Cons(Vec4::new(0, 12, 0, 16), Rc::new(List::Nil))),
    );
    let q6 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(13, 0, 16, 6),
        Vec4::new(14, 0, 16, 4),
        Vec4::new(15, 0, 16, 2),
        Vec4::new(0, 0, 8, 0),
        Vec4::new(12, 0, 16, 0),
    ])
    .append(q7);
    let q5 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(10, 0, 14, 11),
        Vec4::new(12, 0, 13, 4),
        Vec4::new(13, 4, 16, 8),
        Vec4::new(16, 8, 15, 10),
        Vec4::new(15, 10, 16, 16),
    ])
    .append(q6);
    let q4 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(4, 5, 4, 7),
        Vec4::new(4, 0, 6, 5),
        Vec4::new(6, 5, 6, 7),
        Vec4::new(6, 0, 8, 5),
        Vec4::new(8, 5, 8, 8),
    ])
    .append(q5);
    let q3 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(11, 15, 9, 13),
        Vec4::new(10, 10, 8, 12),
        Vec4::new(8, 12, 12, 12),
        Vec4::new(12, 12, 10, 10),
        Vec4::new(2, 0, 4, 5),
    ])
    .append(q4);
    let q2 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(4, 16, 5, 14),
        Vec4::new(6, 16, 7, 15),
        Vec4::new(0, 10, 7, 11),
        Vec4::new(9, 13, 8, 15),
        Vec4::new(8, 15, 11, 15),
    ])
    .append(q3);
    let q1 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 12, 3, 13),
        Vec4::new(3, 13, 5, 14),
        Vec4::new(5, 14, 7, 15),
        Vec4::new(7, 15, 8, 16),
        Vec4::new(2, 16, 3, 13),
    ])
    .append(q2);
    let q = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 8, 4, 7),
        Vec4::new(4, 7, 6, 7),
        Vec4::new(6, 7, 8, 8),
        Vec4::new(8, 8, 12, 10),
        Vec4::new(12, 10, 16, 16),
    ])
    .append(q1);
    q
}

fn r_tile() -> List<Vec4> {
    let r4 = [
        Vec4::new(11, 16, 12, 12),
        Vec4::new(12, 12, 16, 8),
        Vec4::new(13, 13, 16, 10),
        Vec4::new(14, 14, 16, 12),
        Vec4::new(15, 15, 16, 14),
    ]
    .into();

    let r3 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(2, 2, 8, 0),
        Vec4::new(3, 3, 8, 2),
        Vec4::new(8, 2, 12, 0),
        Vec4::new(5, 5, 12, 3),
        Vec4::new(12, 3, 16, 0),
    ])
    .append(r4);
    let r2 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(5, 10, 2, 12),
        Vec4::new(2, 12, 0, 16),
        Vec4::new(16, 8, 12, 12),
        Vec4::new(12, 12, 11, 16),
        Vec4::new(1, 1, 4, 0),
    ])
    .append(r3);
    let r1 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(16, 6, 11, 10),
        Vec4::new(11, 10, 6, 16),
        Vec4::new(16, 4, 14, 6),
        Vec4::new(14, 6, 8, 8),
        Vec4::new(8, 8, 5, 10),
    ])
    .append(r2);
    let r = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 0, 8, 8),
        Vec4::new(12, 12, 16, 16),
        Vec4::new(0, 4, 5, 10),
        Vec4::new(0, 8, 2, 12),
        Vec4::new(0, 12, 1, 14),
    ])
    .append(r1);
    r
}

fn s_tile() -> List<Vec4> {
    let s5: List<Vec4> = [
        Vec4::new(15, 5, 13, 7),
        Vec4::new(13, 7, 15, 8),
        Vec4::new(15, 8, 15, 5),
    ]
    .into();

    let s4 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(15, 9, 16, 8),
        Vec4::new(10, 16, 11, 10),
        Vec4::new(12, 4, 10, 6),
        Vec4::new(10, 6, 12, 7),
        Vec4::new(12, 7, 12, 4),
    ])
    .append(s5);
    let s3 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(7, 8, 7, 13),
        Vec4::new(7, 13, 8, 16),
        Vec4::new(12, 16, 13, 13),
        Vec4::new(13, 13, 14, 11),
        Vec4::new(14, 11, 15, 9),
    ])
    .append(s4);
    let s2 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(14, 11, 16, 12),
        Vec4::new(15, 9, 16, 10),
        Vec4::new(16, 0, 10, 4),
        Vec4::new(10, 4, 8, 6),
        Vec4::new(8, 6, 7, 8),
    ])
    .append(s3);
    let s1 = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 8, 8, 6),
        Vec4::new(0, 10, 7, 8),
        Vec4::new(0, 12, 7, 10),
        Vec4::new(0, 14, 7, 13),
        Vec4::new(13, 13, 16, 14),
    ])
    .append(s2);
    let s = <[Vec4; 5] as Into<List<Vec4>>>::into([
        Vec4::new(0, 0, 4, 2),
        Vec4::new(4, 2, 8, 2),
        Vec4::new(8, 2, 16, 0),
        Vec4::new(0, 4, 2, 1),
        Vec4::new(0, 6, 7, 4),
    ])
    .append(s1);
    s
}

fn grid(m: i64, n: i64, segments: List<Vec4>, a: Vec2, b: Vec2, c: Vec2) -> List<Vec4> {
    segments.map(|v| {
        ((a + b.scale(v.x, m)) + c.scale(v.y, n)).tup2((a + b.scale(v.z, m)) + c.scale(v.w, n))
    })
}

fn tile_to_grid(arg: List<Vec4>, arg2: Vec2, arg3: Vec2, arg4: Vec2) -> List<Vec4> {
    grid(16, 16, arg, arg2, arg3, arg4)
}

fn beside(
    m: i64,
    n: i64,
    p: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    q: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) -> List<Vec4> {
    let res = p(a, b.scale(m, m + n), c);
    res.append(q(a + b.scale(m, m + n), b.scale(n, n + m), c))
}

fn above(
    m: i64,
    n: i64,
    p: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    q: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) -> List<Vec4> {
    let res = p(a + c.scale(n, m + n), b, c.scale(m, n + m));
    res.append(q(a, b, c.scale(n, m + n)))
}

fn nil(_: Vec2, _: Vec2, _: Vec2) -> List<Vec4> {
    List::Nil
}

fn side1(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(&nil, &nil, &|a, b, c| rot(&t, a, b, c), &t, arg, q6, q7)
}

fn side2(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(&side1, &side1, &|a, b, c| rot(&t, a, b, c), &t, arg, q6, q7)
}

fn p(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    tile_to_grid(p_tile(), arg, q6, q7)
}

fn q(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    tile_to_grid(q_tile(), arg, q6, q7)
}

fn r(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    tile_to_grid(r_tile(), arg, q6, q7)
}

fn s(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    tile_to_grid(s_tile(), arg, q6, q7)
}

fn t(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(&p, &q, &r, &s, arg, q6, q7)
}

fn u(arg: Vec2, p2: Vec2, p3: Vec2) -> List<Vec4> {
    cycle_(&|a, b, c| rot(&q, a, b, c), arg, p2, p3)
}

fn corner1(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(&nil, &nil, &nil, &nil, arg, q6, q7)
}

fn corner2(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(
        &corner1,
        &side1,
        &|a, b, c| rot(&|a, b, c| side1(a, b, c), a, b, c),
        &|a, b, c| rot(&|a, b, c| u(a, b, c), a, b, c),
        arg,
        q6,
        q7,
    )
}

fn rot(p: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>, a: Vec2, b: Vec2, c: Vec2) -> List<Vec4> {
    p(a + b, c, Vec2 { x: 0, y: 0 } - b)
}

fn cycle_(
    p1: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    arg: Vec2,
    p3: Vec2,
    p4: Vec2,
) -> List<Vec4> {
    quartet(
        p1,
        &move |a, b, c| rot(&move |a, b, c| rot(p1, a, b, c), a, b, c),
        &move |a, b, c| rot(p1, a, b, c),
        &move |a, b, c| rot(&move |a, b, c| rot(p1, a, b, c), a, b, c),
        arg,
        p3,
        p4,
    )
}

fn quartet(
    a: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    b: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    c: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    d: &impl Fn(Vec2, Vec2, Vec2) -> List<Vec4>,
    arg: Vec2,
    a6: Vec2,
    a7: Vec2,
) -> List<Vec4> {
    above(
        1,
        1,
        &move |p5, p6, p7| beside(1, 1, a, b, p5, p6, p7),
        &move |p5, p6, p7| beside(1, 1, c, d, p5, p6, p7),
        arg,
        a6,
        a7,
    )
}

fn pseudocorner(arg: Vec2, q6: Vec2, q7: Vec2) -> List<Vec4> {
    quartet(
        &corner2,
        &side2,
        &|a, b, c| rot(&|a, b, c| side2(a, b, c), a, b, c),
        &|a, b, c| rot(&|a, b, c| t(a, b, c), a, b, c),
        arg,
        q6,
        q7,
    )
}

fn pseudolimit(arg: Vec2, p2: Vec2, p3: Vec2) -> List<Vec4> {
    cycle_(&|a, b, c| pseudocorner(a, b, c), arg, p2, p3)
}

fn test_fish_nofib(n: i64) -> List<List<Vec4>> {
    List::from_iterator((1..=n).map(|i| {
        let n = i.min(0);
        pseudolimit(
            Vec2 { x: 0, y: 0 },
            Vec2 { x: 640 + n, y: 0 },
            Vec2 { x: 0, y: 640 + n },
        )
    }))
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<i64>()
        .expect("n must be a number");
    println!("{:?}", test_fish_nofib(n).head().len());
}
