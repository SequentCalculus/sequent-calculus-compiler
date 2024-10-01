codata StreamInt { Hd : Int, Tl : StreamIt }
def repeat(x: Int) : StreamInt := cocase { Hd => x, Tl => repeat(x) };
def const1() : StreamInt := cocase { Hd => 1, Tl => const1() };

def main() : StreamInt := repeat(1);
