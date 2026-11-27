data Bool { True, False }

def f(x: i64): i64 { x }

def f(x: Bool): i64 { 0 }

def main(): i64 { let y: i64 = True; f(1) }
