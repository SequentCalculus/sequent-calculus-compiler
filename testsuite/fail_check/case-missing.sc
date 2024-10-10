data Bool { True, False }

def neg(x: Bool): Bool := x.case { True => False };
