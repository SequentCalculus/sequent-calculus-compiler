use std::{
    ops::{Add, Sub},
    rc::Rc,
};

#[derive(Debug)]
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

fn p_tile() -> Vec<Vec4> {
    let p5 = vec![
        Vec4::new(10, 4, 13, 5),
        Vec4::new(13, 5, 16, 4),
        Vec4::new(11, 0, 14, 2),
        Vec4::new(14, 2, 16, 2),
    ];
    let mut p4 = vec![
        Vec4::new(8, 12, 16, 10),
        Vec4::new(8, 8, 12, 9),
        Vec4::new(12, 9, 16, 8),
        Vec4::new(9, 6, 12, 7),
        Vec4::new(12, 7, 16, 6),
    ];
    p4.extend(p5.into_iter());
    let mut p3 = vec![
        Vec4::new(10, 16, 12, 14),
        Vec4::new(12, 14, 16, 13),
        Vec4::new(12, 16, 13, 15),
        Vec4::new(13, 15, 16, 14),
        Vec4::new(14, 16, 16, 15),
    ];
    p3.extend(p4.into_iter());
    let mut p2 = vec![
        Vec4::new(4, 13, 0, 16),
        Vec4::new(0, 16, 6, 15),
        Vec4::new(6, 15, 8, 16),
        Vec4::new(8, 16, 12, 12),
        Vec4::new(12, 12, 16, 12),
    ];
    p2.extend(p3.into_iter());
    let mut p1 = vec![
        Vec4::new(4, 10, 7, 6),
        Vec4::new(7, 6, 4, 5),
        Vec4::new(11, 0, 10, 4),
        Vec4::new(10, 4, 9, 6),
        Vec4::new(9, 6, 8, 8),
        Vec4::new(8, 8, 4, 13),
    ];
    p1.extend(p2.into_iter());
    let mut p = vec![
        Vec4::new(0, 3, 3, 4),
        Vec4::new(3, 4, 0, 8),
        Vec4::new(0, 8, 0, 3),
        Vec4::new(6, 0, 4, 4),
        Vec4::new(4, 5, 4, 10),
    ];
    p.extend(p1.into_iter());
    p
}

fn q_tile() -> Vec<Vec4> {
    let q7 = vec![Vec4::new(0, 0, 0, 8), Vec4::new(0, 12, 0, 16)];
    let mut q6 = vec![
        Vec4::new(13, 0, 16, 6),
        Vec4::new(14, 0, 16, 4),
        Vec4::new(15, 0, 16, 2),
        Vec4::new(0, 0, 8, 0),
        Vec4::new(12, 0, 16, 0),
    ];
    q6.extend(q7.into_iter());
    let mut q5 = vec![
        Vec4::new(10, 0, 14, 11),
        Vec4::new(12, 0, 13, 4),
        Vec4::new(13, 4, 16, 8),
        Vec4::new(16, 8, 15, 10),
        Vec4::new(15, 10, 16, 16),
    ];
    q5.extend(q6.into_iter());
    let mut q4 = vec![
        Vec4::new(4, 5, 4, 7),
        Vec4::new(4, 0, 6, 5),
        Vec4::new(6, 5, 6, 7),
        Vec4::new(6, 0, 8, 5),
        Vec4::new(8, 5, 8, 8),
    ];
    q4.extend(q5.into_iter());
    let mut q3 = vec![
        Vec4::new(11, 15, 9, 13),
        Vec4::new(10, 10, 8, 12),
        Vec4::new(8, 12, 12, 12),
        Vec4::new(12, 12, 10, 10),
        Vec4::new(2, 0, 4, 5),
    ];
    q3.extend(q4.into_iter());
    let mut q2 = vec![
        Vec4::new(4, 16, 5, 14),
        Vec4::new(6, 16, 7, 15),
        Vec4::new(0, 10, 7, 11),
        Vec4::new(9, 13, 8, 15),
        Vec4::new(8, 15, 11, 15),
    ];
    q2.extend(q3.into_iter());
    let mut q1 = vec![
        Vec4::new(0, 12, 3, 13),
        Vec4::new(3, 13, 5, 14),
        Vec4::new(5, 14, 7, 15),
        Vec4::new(7, 15, 8, 16),
        Vec4::new(2, 16, 3, 13),
    ];
    q1.extend(q2.into_iter());
    let mut q = vec![
        Vec4::new(0, 8, 4, 7),
        Vec4::new(4, 7, 6, 7),
        Vec4::new(6, 7, 8, 8),
        Vec4::new(8, 8, 12, 10),
        Vec4::new(12, 10, 16, 16),
    ];
    q.extend(q1.into_iter());
    q
}

fn r_tile() -> Vec<Vec4> {
    let r4 = vec![
        Vec4::new(11, 16, 12, 12),
        Vec4::new(12, 12, 16, 8),
        Vec4::new(13, 13, 16, 10),
        Vec4::new(14, 14, 16, 12),
        Vec4::new(15, 15, 16, 14),
    ];
    let mut r3 = vec![
        Vec4::new(2, 2, 8, 0),
        Vec4::new(3, 3, 8, 2),
        Vec4::new(8, 2, 12, 0),
        Vec4::new(5, 5, 12, 3),
        Vec4::new(12, 3, 16, 0),
    ];
    r3.extend(r4.into_iter());
    let mut r2 = vec![
        Vec4::new(5, 10, 2, 12),
        Vec4::new(2, 12, 0, 16),
        Vec4::new(16, 8, 12, 12),
        Vec4::new(12, 12, 11, 16),
        Vec4::new(1, 1, 4, 0),
    ];
    r2.extend(r3.into_iter());
    let mut r1 = vec![
        Vec4::new(16, 6, 11, 10),
        Vec4::new(11, 10, 6, 16),
        Vec4::new(16, 4, 14, 6),
        Vec4::new(14, 6, 8, 8),
        Vec4::new(8, 8, 5, 10),
    ];
    r1.extend(r2.into_iter());
    let mut r = vec![
        Vec4::new(0, 0, 8, 8),
        Vec4::new(12, 12, 16, 16),
        Vec4::new(0, 4, 5, 10),
        Vec4::new(0, 8, 2, 12),
        Vec4::new(0, 12, 1, 14),
    ];
    r.extend(r1.into_iter());
    r
}

fn s_tile() -> Vec<Vec4> {
    let s5 = vec![
        Vec4::new(15, 5, 13, 7),
        Vec4::new(13, 7, 15, 8),
        Vec4::new(15, 8, 15, 5),
    ];
    let mut s4 = vec![
        Vec4::new(15, 9, 16, 8),
        Vec4::new(10, 16, 11, 10),
        Vec4::new(12, 4, 10, 6),
        Vec4::new(10, 6, 12, 7),
        Vec4::new(12, 7, 12, 4),
    ];
    s4.extend(s5.into_iter());
    let mut s3 = vec![
        Vec4::new(7, 8, 7, 13),
        Vec4::new(7, 13, 8, 16),
        Vec4::new(12, 16, 13, 13),
        Vec4::new(13, 13, 14, 11),
        Vec4::new(14, 11, 15, 9),
    ];
    s3.extend(s4.into_iter());
    let mut s2 = vec![
        Vec4::new(14, 11, 16, 12),
        Vec4::new(15, 9, 16, 10),
        Vec4::new(16, 0, 10, 4),
        Vec4::new(10, 4, 8, 6),
        Vec4::new(8, 6, 7, 8),
    ];
    s2.extend(s3.into_iter());
    let mut s1 = vec![
        Vec4::new(0, 8, 8, 6),
        Vec4::new(0, 10, 7, 8),
        Vec4::new(0, 12, 7, 10),
        Vec4::new(0, 14, 7, 13),
        Vec4::new(13, 13, 16, 14),
    ];
    s1.extend(s2.into_iter());
    let mut s = vec![
        Vec4::new(0, 0, 4, 2),
        Vec4::new(4, 2, 8, 2),
        Vec4::new(8, 2, 16, 0),
        Vec4::new(0, 4, 2, 1),
        Vec4::new(0, 6, 7, 4),
    ];
    s.extend(s1.into_iter());
    s
}

fn grid(m: i64, n: i64, segments: Vec<Vec4>, a: Vec2, b: Vec2, c: Vec2) -> Vec<Vec4> {
    segments
        .into_iter()
        .map(|v| {
            ((a + b.scale(v.x, m)) + c.scale(v.y, n)).tup2((a + b.scale(v.z, m)) + c.scale(v.w, n))
        })
        .collect()
}

fn tile_to_grid(arg: Vec<Vec4>, arg2: Vec2, arg3: Vec2, arg4: Vec2) -> Vec<Vec4> {
    grid(16, 16, arg, arg2, arg3, arg4)
}

fn beside(
    m: i64,
    n: i64,
    p: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    q: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) -> Vec<Vec4> {
    let mut res = p(a, b.scale(m, m + n), c);
    res.extend(q(a + b.scale(m, m + n), b.scale(n, n + m), c).into_iter());
    res
}

fn above(
    m: i64,
    n: i64,
    p: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    q: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) -> Vec<Vec4> {
    let mut res = p(a + c.scale(n, m + n), b, c.scale(m, n + m));
    res.extend(q(a, b, c.scale(n, m + n)).into_iter());
    res
}

fn nil(_: Vec2, _: Vec2, _: Vec2) -> Vec<Vec4> {
    vec![]
}

fn side1(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(
        Rc::new(nil),
        Rc::new(nil),
        Rc::new(|a, b, c| rot(Rc::new(t), a, b, c)),
        Rc::new(t),
        arg,
        q6,
        q7,
    )
}

fn side2(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(
        Rc::new(side1),
        Rc::new(side1),
        Rc::new(|a, b, c| rot(Rc::new(t), a, b, c)),
        Rc::new(t),
        arg,
        q6,
        q7,
    )
}

fn p(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    tile_to_grid(p_tile(), arg, q6, q7)
}

fn q(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    tile_to_grid(q_tile(), arg, q6, q7)
}

fn r(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    tile_to_grid(r_tile(), arg, q6, q7)
}

fn s(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    tile_to_grid(s_tile(), arg, q6, q7)
}

fn t(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(Rc::new(p), Rc::new(q), Rc::new(r), Rc::new(s), arg, q6, q7)
}

fn u(arg: Vec2, p2: Vec2, p3: Vec2) -> Vec<Vec4> {
    cycle_(Rc::new(|a, b, c| rot(Rc::new(q), a, b, c)), arg, p2, p3)
}

fn corner1(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(
        Rc::new(nil),
        Rc::new(nil),
        Rc::new(nil),
        Rc::new(nil),
        arg,
        q6,
        q7,
    )
}

fn corner2(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(
        Rc::new(corner1),
        Rc::new(side1),
        Rc::new(|a, b, c| rot(Rc::new(|a, b, c| side1(a, b, c)), a, b, c)),
        Rc::new(|a, b, c| rot(Rc::new(|a, b, c| u(a, b, c)), a, b, c)),
        arg,
        q6,
        q7,
    )
}

fn rot(p: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>, a: Vec2, b: Vec2, c: Vec2) -> Vec<Vec4> {
    p(a + b, c, Vec2 { x: 0, y: 0 } - b)
}

fn cycle_(
    p1: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    arg: Vec2,
    p3: Vec2,
    p4: Vec2,
) -> Vec<Vec4> {
    let p1_a = p1.clone();
    let p1_b = p1.clone();
    let p1_c = p1.clone();
    quartet(
        p1.clone(),
        Rc::new(move |a, b, c| {
            let p1_a_ = p1_a.clone();
            rot(Rc::new(move |a, b, c| rot(p1_a_.clone(), a, b, c)), a, b, c)
        }),
        Rc::new(move |a, b, c| rot(p1_b.clone(), a, b, c)),
        Rc::new(move |a, b, c| {
            let p1_c_ = p1_c.clone();
            rot(Rc::new(move |a, b, c| rot(p1_c_.clone(), a, b, c)), a, b, c)
        }),
        arg,
        p3,
        p4,
    )
}

fn quartet(
    a: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    b: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    c: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    d: Rc<dyn Fn(Vec2, Vec2, Vec2) -> Vec<Vec4>>,
    arg: Vec2,
    a6: Vec2,
    a7: Vec2,
) -> Vec<Vec4> {
    above(
        1,
        1,
        Rc::new(move |p5, p6, p7| beside(1, 1, a.clone(), b.clone(), p5, p6, p7)),
        Rc::new(move |p5, p6, p7| beside(1, 1, c.clone(), d.clone(), p5, p6, p7)),
        arg,
        a6,
        a7,
    )
}

fn pseudocorner(arg: Vec2, q6: Vec2, q7: Vec2) -> Vec<Vec4> {
    quartet(
        Rc::new(corner2),
        Rc::new(side2),
        Rc::new(|a, b, c| rot(Rc::new(|a, b, c| side2(a, b, c)), a, b, c)),
        Rc::new(|a, b, c| rot(Rc::new(|a, b, c| t(a, b, c)), a, b, c)),
        arg,
        q6,
        q7,
    )
}

fn pseudolimit(arg: Vec2, p2: Vec2, p3: Vec2) -> Vec<Vec4> {
    cycle_(Rc::new(|a, b, c| pseudocorner(a, b, c)), arg, p2, p3)
}

fn test_fish_nofib(n: i64) -> Vec<Vec<Vec4>> {
    (1..=n)
        .map(|i| {
            let n = i.min(0);
            pseudolimit(
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 640 + n, y: 0 },
                Vec2 { x: 0, y: 640 + n },
            )
        })
        .collect()
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<i64>()
        .expect("n must be a number");
    println!("{:?}", test_fish_nofib(n).iter().first().len());
}
