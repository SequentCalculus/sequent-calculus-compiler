data OptionInt { None, Some(x: Int) }

//def attempt(i: Int): OptionInt := ifz(i, Some(i), case attempt(i - 1) of { None => None,
//                                                                           Some(x: Int) => Some(x + 1) });
//
//def main(n: Int): Int := case attempt(n) of { None => 0 - 1,
//                                              Some(x: Int) => x };

def attempt(i: Int): OptionInt := ifz(i, Some(i), let j: Int = i - 1 in (attempt(j)).case { None => None,
                                                                                            Some(x: Int) => let y: Int = x + 1 in Some(x) });

def main(n: Int): Int := (attempt(n)).case { None => let r: Int = 0 - 1 in r,
                                             Some(x: Int) => x };
