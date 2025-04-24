fn expand(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> i64 {
    f + 10 * e + 100 * d + 1000 * c + 10000 * b + 100000 * a
}

fn condition(thirywelvn: &[i64]) -> bool {
    if thirywelvn.len() != 10 {
        return false;
    }
    let t = thirywelvn[0];
    let h = thirywelvn[1];
    let i = thirywelvn[2];
    let r = thirywelvn[3];
    let y = thirywelvn[4];
    let w = thirywelvn[5];
    let e = thirywelvn[6];
    let l = thirywelvn[7];
    let v = thirywelvn[8];
    let n = thirywelvn[9];
    expand(t, h, i, r, t, y) + 5 * expand(t, w, e, l, v, e) == expand(n, i, n, e, t, y)
}

fn addj(j: i64, mut ls: Vec<i64>) -> Vec<Vec<i64>> {
    if ls.is_empty() {
        return vec![vec![j]];
    }
    let k = ls.remove(0);
    let lscomp = |p1: Vec<Vec<i64>>| {
        p1.into_iter()
            .map(|mut h1| {
                h1.insert(0, k);
                h1
            })
            .collect()
    };
    let mut res: Vec<Vec<i64>> = lscomp(addj(j, ls.clone()));
    ls.insert(0, k);
    ls.insert(0, j);
    res.insert(0, ls);
    res
}

fn perm_lscomp1(mut p1: Vec<Vec<i64>>, j: i64) -> Vec<Vec<i64>> {
    if p1.is_empty() {
        return vec![];
    }
    let pjs = p1.remove(0);
    perm_lscomp2(addj(j, pjs), p1, j)
}

fn perm_lscomp2(mut p2: Vec<Vec<i64>>, t1: Vec<Vec<i64>>, j: i64) -> Vec<Vec<i64>> {
    if p2.is_empty() {
        return perm_lscomp1(t1, j);
    }

    let r = p2.remove(0);
    let mut res = perm_lscomp2(p2, t1, j);
    res.insert(0, r);
    res
}

fn permutations(mut ls: Vec<i64>) -> Vec<Vec<i64>> {
    if ls.is_empty() {
        return vec![vec![]];
    }
    let j = ls.remove(0);
    perm_lscomp1(permutations(ls), j)
}

fn test_cryptarithm_nofib(n: i64) -> Vec<Vec<Vec<i64>>> {
    (1..=n)
        .map(|i| {
            let p0: Vec<i64> = (0..=(9 + i)).take(10).collect();
            permutations(p0)
                .into_iter()
                .filter(|l| condition(l))
                .collect()
        })
        .collect()
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_cryptarithm_nofib(n);
    if iters == 1 {
        println!("{:?}", res);
        0
    } else {
        main_loop(iters - 1, n)
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let iters = args
        .next()
        .expect("Missing Argument iterations")
        .parse::<u64>()
        .expect("Iterations must be a number");
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<i64>()
        .expect("m must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
