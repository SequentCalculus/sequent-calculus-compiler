data T1 { K1 }
data T2 { K2 }

def foo(x: T1): Int := x.case { K1 => 1, K2 => 2};
