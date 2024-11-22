data OptionInt { None, Some(x: Int) }

def attempt(i: Int): OptionInt := ifz(i, Some(i), (attempt(i - 1)).case { None => None,
                                                                          Some(x: Int) => Some(x + 1) });

def main(n: Int): Int := (attempt(n)).case { None => 0 - 1,
                                             Some(x: Int) => x };
