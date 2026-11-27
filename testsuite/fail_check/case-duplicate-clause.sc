data Bool { True, False }

def main(): i64 { True.case { True => 1, False => 2, True => 3 } }
