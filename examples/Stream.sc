<<<<<<< HEAD
def repeat(x: Int) : StreamInt := cocase { hd => x, tl => repeat(x) };
def const1() : StreamInt := cocase { hd => 1, tl => const1() };

def main() : StreamInt := repeat(1);
=======
def repeat(x: Int) := cocase { hd => x, tl => repeat(x;) };
def const1() := cocase { hd => 1, tl => const1(;) };

def main() := repeat(1;);
>>>>>>> 8eb76bc (fixed integration tests)
