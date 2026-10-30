def unused(): i64 {
    1
}

def getsInlined(y: i64): i64 {
    y
}

def main(): i64 {
    let not_used: i64 = 4;
    let used_once: i64 = 4 + 1;
    let x: i64 = 36;
    getsInlined(x) + used_once
}