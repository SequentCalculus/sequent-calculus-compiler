data TestTable { C0, C1, C2, C3 }

def main() : Int := let y: TestTable = C3 in
                    y.case { C0 => let x: Int = ((((((((((((((1+2)+3)+4)+5)+6)+7)+8)+9)+10)+11)+12)+13)+14)+15)+16 in x,
                             C1 => 1,
                             C2 => 2,
                             C3 => 3 };
