data Bool { True, False }

data Wrapped { Wrap(value: i64) }

codata Comp {
    calc(init: i64): i64
}

def unused(): i64 {
    -1
}

def getsInlined(y: i64): i64 {
    let x: i64 = y + 1;
    unusedArg(x)
}

def unusedArg(x: i64): i64 {
    40
}

def even(z: i64): Bool {
    if z <= 0 {
        True
    } else {
        odd(z - 1)
    }
}

def odd(z: i64): Bool {
    if z <= 0 {
        False
    } else {
        even(z - 1)
    }
}

def fac(z: i64): i64 {
    if z <= 1 {
        1
    } else {
        z * fac(z - 1)
    }
}

def callMain(): i64 {
    main()
}

def main(): i64 {
    if 2 == 1 {
        callMain()
    } else {
        let comp: Comp = new {
            calc(init) =>
                let not_really_used: i64 = init;
                let used_once: i64 = 1 + 1;
                let unused_comp: i64 = not_really_used + 3;
                getsInlined(unused_comp) + used_once
        };
        let wrapped: Wrapped = Wrap(1);
        wrapped.case {
            Wrap(value) =>
                let x: i64 = comp.calc(value);
                odd(x).case {
                    True => println_i64(7); 7,
                    False => println_i64(fac(3)); 6
                }
        }
    }
}