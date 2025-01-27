data Rbg { Color(r:f64,g:f64,b:f64) }
codata Fun[A,B]{ Ap(x:A) : B }
data Pair[A,B] { Tup(fst:A,snd:B) }
data Unit { Unit } 

def x_base() : f64 { 1.0 };
def y_base() : f64 { 1.25 };
def side() : f64 { 2.5 };
def max_count() : i64 { 1000 };

// we will probably need some primitives for f64 to implement this efficiently
def round(f:f64) : i64 { 0 };
// random number generation 
def rand_int(lo:i64,hi:i64) : i64 { lo };
// casting
def i642f64(i:i64) : f64 { 1.0 };
// pixel array 
def generate_pixels(n:i64) : [[Pair[i64,i64]]] { 
  let gen_fun : Fun[Pair[i64,i64],i64]:= new { Ap(x) => 
    x.case[i64,i64] { Tup(fst,snd) => rgb2i64(pix2rbg(elt(i,j,n))) } };
    [i in 0..(n-1) =>  [ j in 0..(n-1) => gen_fun.Ap(Tup(i,j)) ] ]
}


def pix2rgb(cnt:i64) : Rgb { 
  if max_count<cnt{
    Color(0.0,0.0,0.0)
  } else {
    let w = cnt / (max_count-1);
    Color(w,w,0.25 + (w*0.75))
  }
}

def rgb2i64(col:Rgb) : i64 {
  let f : Fun[f64,i64]= new { Ap(c) => round(255.0*c) };
  col.case { 
    Color(r,b,g) => (65536*(f.Ap(r))) + (256*(f.Ap(g))) + (f.Ap(b)) 
  }
}

def spoofRgb2i64(col:Rbg) : i64 {
  let r_prime = rand_int(0,256) in 
  let g_prime = rand_int(0,256) in 
  let b_prime = rand_int(0,256) in 
  (256*(256*r_prime)) + (256+g_prime) + b_prime
}

// can't be inlined because of recursion
def brot_loop(cnt:i64,z_re:f64,z_im:f64) : i64 {
  if cnt<maxCount{
    let z_re_sq : f64 = z_re * z_re;
    let z_im_sq : f64 = z_im * z_im;
    if 4.0<(z_re_sq+z_im_sq) {
      cnt
    }else {
      let z_re_im = z_re * z_im;
      brot_loop(cnt+1, (z_re_sq - z_im_sq) + c_re, (z_re_im + z_re_im) + c_im)
    }
  } else {
    cnt
  }
}

def elt(i:i64,j:i64,n:i64) : i64 {
    let delta : f64 = side / (i642f64(n-1));
    let c_re : f64 = x_base + (delta * i642f64(j));
    let c_im : f64 = y_base - (delta * i642f64(i));
    brot_loop (0, c_re, c_im)
}

def mandelbrot(n:i64) : [[Color]] {
fun mandelbrot N = 
  let pixels : [[Pair[i64,i64]]] := generate_pixels(n);
  let image  : [[Color]] := [i in 0..(n-1) => [ j in 0..(n-1) => Color(0.0,0.0,0.0) ]];
  // commented out in the original implementation as well
  // note that pixels are explicitely integers, while update requires colors to update the image
  //let update : FunImage := codata { ApI(i:i64,j:i64,col:Color,im:[[Color]]  => im[i][j] = col } in 
  //map_array(image,update)
  image
}

def main(n:i64) : i64 { 
  let res : [[Color]] = mandelbrot(n);
  0 
};
