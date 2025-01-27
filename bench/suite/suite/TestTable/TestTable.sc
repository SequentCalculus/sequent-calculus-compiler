data TestTable { C0, C1, C2, C3 }

def main() : i64 { let y: TestTable = C3;
                   let res: i64 = y.case { C0 => ((((((((((((((1+2)+3)+4)+5)+6)+7)+8)+9)+10)+11)+12)+13)+14)+15)+16,
                                           C1 => 1,
                                           C2 => 2,
                                           C3 => 3 };
                   println_i64(res);
                   0 }
