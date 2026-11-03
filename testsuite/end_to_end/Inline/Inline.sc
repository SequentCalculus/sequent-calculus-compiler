def unused(): i64 {
    1
}

def getsInlined(y: i64): i64 {
    y
}

def unusedArg(x: i64): i64 {
    4
}

def main(): i64 {
    let not_used: i64 = 4;
    let used_once: i64 = unusedArg(not_used) + 1;
    let unused_comp: i64 = not_used + 3;
    let x: i64 = 36;
    getsInlined(x) + used_once
}