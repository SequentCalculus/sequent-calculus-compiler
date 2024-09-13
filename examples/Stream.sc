<<<<<<< HEAD
def repeat(x: Int) : StreamInt := cocase { hd => x, tl => repeat(x) };
def const1() : StreamInt := cocase { hd => 1, tl => const1() };

def main() : StreamInt := repeat(1);
=======
def repeat(x: Int) : StreamInt := cocase { hd => x, tl => repeat(x;) };
def const1() : StreamInt := cocase { hd => 1, tl => const1(;) };

def main() : StreamInt := repeat(1;);
>>>>>>> 27de75f (added parsing definition types)
