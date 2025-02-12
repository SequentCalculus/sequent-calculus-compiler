data Bool { True, False}

def not(b:Bool) : Bool{
  b.case{
    False => True,
    True => False
  }
}

def not2(b:Bool) : Bool{
  b.case{
    True => False,
    False => True
  }
}


def main() : i64 {     

  let res : Bool = not(True);
  println_i64(res.case{True=>1,False=>-1});
  let res : Bool = not2(True);
  println_i64(res.case{True=>1,False=>-1});
  0
}
