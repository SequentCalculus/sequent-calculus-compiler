def nonValueArguments := (\x => \y => y) (1 + 2) (3 + 4);

def higherOrder := (\x => \y => x y) (\z => 4 + z) (3 + 1);

def main := higherOrder();
