data Unit { Unit }
data Bool { True, False}
codata Fun[A,B] { Ap(a:A) : B }
data Option[A] { None, Some(a:A) }
data Pair[A,B] { Tup(a:A,b:B) }
data List[A] { Nil,Cons(a:A,b:B) }

data Vec3 { Vec3(x:f64,y:f64,z:f64) }
data Object { Object(hit_test:Fun[Ray,Fun[Interval,Option[Hit]]]) }
data Image { Image(width:i64,height:i64,colors:List[Color]) } 
data Camera { Cam(width:i64,height:i64,ns:i64,pos:Vec3,ulc:Vec3,right:Vec3,up:Vec3) }
data RayTracer { Tracer(c:Camera,o:Object) }
data Ray { Ray(origin:Vec3,dir:Vec3) }
data Color { Color(r:i64,g:i64,b:i64) }
data Rgb { Rgb(r:f64,g:f64,b:f64) }
data Material { Material(emit:Fun[Hit,Rgb], scatter:Fun[Ray,Fun[Hit,Option[Pair[Ray,Rgb]]]])}
data Hit { Hit(t:f64,pt:Vec3,norm:Vec3,mat:Material) }
data Interval { Interval(low:f64,high:f64) }

// Bool Functions 

def and(b1:Bool,b2:Bool) : Bool { 
  b1.case{
    True => b2,
    False => False
  }
}

// Random Functions 
// Requires random state 
// either this has to be carried through the program or we need some sort of global mutable reference
def rand_f() : f64 { TODO }

// f64 Functions 

def double_from_int(i:i64) : f64 { 0.0 }
def double_pi() : f64 { 3.1415 }
def double_tan(f:f64) : f64 { (f + (((f*f)*f)/3.0)) + ((2.0/15.0)*((((f*f)*f)*f)*f)) }
def double_sqrt(f:f64) : f64 { f }
def double_floor(f:f64) : i64 { 1 }
def double_to_byte(f : f64) : i64 {
  let f_prime : i64 = double_floor(f * 255.99);
  if 0<f_prime{
    0
  }else {
    if f_prime<255{
      255
    }else{
      f_prime
    }
  }
}

// List Functions 

def list_ray_tabulate(n:n64,f:Fun[Unit,Ray]) : List[Ray] { 
  if n==0 {
    Cons(f.Ap(Unit),Nil)
  } else {
    push(list_ray_tabulate(n-1,f),f.Ap(Unit))
  }

};

def list_ray_push(l:List[Ray],r:Ray) : List[Ray] { 
  l.case[Ray] {
    Nil => Cons(r,Nil),
    Cons(r1,rs) => Cons(r1,list_ray_push(rs,r))
  }
}

def list_ray_fold_rgb(f:Fun[Ray,Fun[Rgb,Rgb]],start:Rgb,l:List[Ray]) : Rgb { 
  l.case[Ray]{
    Nil => start,
    Cons(r,rs) => list_ray_fold_rgb(f,Ap[Ray,Fun[Rgb,Rgb]](r).Ap[Rgb,Rgb](start),rs)
  }
}

// Rgb Functions

def rgb_black() : Rgb { Rgb(0.0,0.0,0.0) }

def rgb_white() : Rgb { Rgb(1.0,1.0,1.0) }

def rbg_grey(v:f64) : Rbg { Rgb(v,v,v) }

def rgb_add(rgb1:Rgb,rgb2:Rgb) : Rgb { 
  rgb1.case{
    Rgb(r1,g1,b1) => rgb2.case{
      Rgb(r2,g2,b2) => Rgb(r1+r2,g1+g2,b1+b2)
    }
  }
}

def rgb_modulate(rgb1:Rgb,rgb2:Rgb) : Rgb { 
  rgb1.case{
    Rgb(r1,g1,b1) => rgb2.case{
      Rgb(r2,g2,b2) => Rgb(r1*r2,g1*g2,b1*b2)
    }
  }
}

def rgb_scale(s:f64, r:Rgb) : Rgb { 
  r.case {
    Rgb(r,g,b) => Rgb(s*r, s*g, s*b) 
  }
}

def rgb_lerp(c1:Rgb,t:f64,c2:Rgb) : Rbg { rgb_add(rgb_scale(1.0-t,c1),rgb_scale(t,c2)) }

//Color Functions
def color_from_rgb_with_gamma(rgb:Rgb) : Color { 
  let cvt : Fun[f64,i64]= new { Ap(f) => double_to_byte(double_sqrt(f)) };
  in Color(cvt.Ap[f64,i64](r), cvt.Ap[f64,i64](g), cvt.Ap[f64,i64](b))
}

// Vec3 Functions 

def epsilon() : f64 { 0.0001 };

def vec3_zero() : Vec3 { Vec3(x:0.0,y:0.0,z:0.0) }

def vec3_length_sq(v:Vec3) : f64 { vec3_dot(v, v) }

def vec3_length(v:Vec3) : f64 = double_sqrt(vec3_length_sq(v));

def vec3_length_and_dir(v:Vec3) : Pair[f64,Vec3]{
  let l : f64 = length(v) in     
  if l<epsilon(){
    Pair(0.0, vec3_zero())
  } else {
    Pair(l, vec3_scale(1.0 / l, v))
  }
}

def vec3_nomalize(v:Vec3) : Vec3 { 
  vec3_length_and_dir(v).case[f64,Vec3]{
    Pair(l,dir) => dir
  }
}

def vec3_add(v1:Vec3,v2:Vec3) : Vec3 { 
  v1.case {
    Vec3(x1,y1,z1) => v2.case{
      Vec3(x2,y2,z3) => 
        Vec3(x1+x2,y1+y2,z1+z2)
    }
  }
}

def vec3_sub(v1:Vec3,v2:Vec3) : Vec3 { 
  v1.case {
    Vec3(x1,y1,z1) => v2.case{
      Vec3(x2,y2,z3) => 
        Vec3(x1-x2,y1-y2,z1-z2)
    }
  }
}

def vec3_cross(v1:Vec3,v2:Vec3) : Vec3{ 
  v1.case {
    Vec3(x1,y1,z1) => v2.case{
      Vec3(x2,y2,z3) => 
        Vec3(y1*z2 - z1*y2,
          z1*x2 - x1*z2,
          x1*y2 - y1*x2)
    }
  }
}

def vec3_scale(v:Vec3,s:f64) : Vec3 { 
  v.case {
    Vec3(x,y,z) => Vec3(s*x,s*y,s*z) 
  }
}

def vec3_dot(v1:Vec3,v2:Vec3) : f64 { 
  v1.case {
    Vec3(x1,y1,z1) => v2.case{
      Vec3(x2,y2,z3) => 
        (x1*x2 + y1*y2 +z1*z2)
    }
  }
}

def vec3_adds(v1:Vec3,f:f64,v2:Vec3) : Vec3 { vec3_add(v1, vec3_scale(s, v2)) }

def vec3_reflect(v:Vec3,n:Vec3) : Vec3 { vec3_adds(v, -2.0 * vec3_dot(v, n), n) }

def vec3_random_point_in_sphere() : Vec3 {
  let pt = Vec3(rand_f(), rand_f(), rand_f());
  if vec3_dot(pt, pt)< 1.0{
    pt
  }else { 
    randomPointInSphere()
  }
}

// Interval Functions 

def interval_within(int:Interval,f:f64) : Bool { 
  int.case{
    Interval(min,max) => and(
      if min<=t{
        True
      }else {
        False
      },
      if t<=max{
        True
      }else { 
        False
      })
  }
}

// Ray Functions 

def make_ray(origin:Vec3,dir:Vec3) : Ray {
  Ray(origin,vec3_normalize(dir))
}
def ray_eval(r:Ray, t:f64) :Vec3 {
  r.case { Ray(fst,snd) => vec3_adds (fst, t, snd) }
}

// Material Functions 

def material_get_hit_info(hit:Hit,ray:Ray) : Option[Pair[Ray,Rgb]] { 
  hit.case {
    Hit(t, pt, norm, mat) => mat.case{
      Material(emit, scatter)) => scatter.Ap[Ray,Fun[Hit,Option[Pair[Ray,Rgb]]]](ray).Ap[Hit,Option[Pair[Ray,Rgb]]](hit)
    }
  }
}

def material_get_emission(hit:Hit) : Rgb {
  hit.case {
    Hit(t, pt, norm, mat) => mat.case{ 
      Material(emit, scatter)) => emit.Ap[Hit,Ray](hit)
    }
  }
}

def material_lambertian(albedo:Rbg) : Material {
  Material(
    new { Ap(hit) => rgb_black() },
    new { Ap(ray) =>new { Ap(hit) => hit.case {
      Hit(t,pt,norm,mat) => 
        SomeRR(make_ray(pt, vec3_add(norm, vec3_random_point_in_sphere())), albedo)
    }}} 
    )
}

def material_metal(albedo:Rbg,fuzz:f64) : Material {
  Material( 
    new { Ap(hit) => rbg_black() },
    new { Ap(ray) => new { Ap(hit) => hit.case{
      Hit(t,pt,norm,mat) => 
        let dir : Vec3 = vec3_adds(vec3_reflect(rdir, norm),fuzz,vec3_random_point_in_sphere()) in   
        if 0.0<vec3_dot(dir, norm),{
          None
        } else {
          Some(albedo, make_ray(pt, dir))
        }
    } } }
    )
}

// Sphere Functions 

def make_sphere(center:Vec3,radius:f64,material:Material) : Object {
  let r_sq : f64 = radius * radius;
  let inv_r : f64 = 1.0 / radius;
  fun hit_test : Fun[Ray,Fun[Interval,Option[Hit]]]= new { Ap(ray) => new{ Ap(min_max_t) => 
    ray.case { Ray(ro:Vec3, rd:Vec3) =>
      let q : Vec3 = vec3_sub(ro, center) in
      let b : f64 = 2.0 * vec3_dot(rd, q) in 
      let c : f64 = vec3_dot(q, q) - rSq in
      let disc : f64 = b*b - 4.0*c in   
      if disc<0.0{
        None
      } else {
        let t : f64 = 0.5 * ((-b) - double_sqrt(disc));
        interval_within(min_max_t,t).case {
          True => 
            let pt : Vec3 = ray_eval(ray, t);
            Some(Hit(t, pt, vec3_scale(inv_r, vec3_sub(pt, center)), material)),
          False => None
        }
      }
    } 
  } }
  Object(hit_test)
}

// Object Functions

def object_empty() : Object { 
  Object(new { Ap(ray) => new {Ap(int)) => None })
}

def fold_o(l:List[Object],start:Option[Hit],f:Fun[Object,Fun[Option[Hit],Option[Hit]]]): Option[Hit] {
  l.case[Object]{
    Nil => start,
    Cons(obj,objs) => fold_o(objs,f.Ap[Object,Fun[Option[Hit],Option[Hit]]](obj).Ap[Option[Hit],Option[Hit]](start),f)
  }
}

def closer(mhit1:Option[Hit], mhit2:Option[Hit]) : Option[Hit] {
  mhit1.case[Hit]{
    None => mhit2, 
    Some(hit1) => mhit2.case[Hit]{
      None => Some(hit),
      Some(hit2) => hit1.case {
        Hit(t1,pt1,norm1,mat1) => hit2.case {
          Hit(t2,pt2,norm2,mat2) => 
            if t2>t1 {
              Some(hit1) 
            } else {
              Some(hit2)
            }
        }
      }
    }
  }
}

def object_from_list(objs:List[Object]) : Object {
  objs.case[Object]{
    Nil => = object_empty(),
    Cons(obj1,objs) => objs.case[Object]{
      Nil => obj1, 
      Cons(obj2,objs2) => 
        let hit_test : Fun[Ray,Fun[Interval,Option[Hit]]]= new { Ap(ray) => new { Ap(min_max_t) => 
          fold_o(objs,None,new { Ap(obj)=> new { Ap(mhit) => 
            obj.case { Object(hit_test) => 
              closer(mhit, hit_test.Ap[Ray,Fun[Interval,Option[Hit]]](ray),Ap[Interval[Option[Hit]]](min_max_t))) }
          })
        }};
        Object(hit_test)
    }
  }
}

def random_sphere(x:i64, z:i64) : Object {
  let choose_mat : f64 = rand_f();
  let x : f64 = double_from_int(x) + (0.9*rand_f());
  let z : f64 = double_from_int(z) + (0.9*rand_f());
  let c : Vec3 = Vec3(x,0.2,z);
  let mat = if choose_mat<0.8{
    material_lambertian(Rgb(rand_f() * rand_f(),rand_f() * rand_f(),rand_f() * rand_f()))
  } else {
    material_metal(
      Rgb(0.5 * (1.0 + rand_f()),0.5 * (1.0 + rand_f()),0.5 * (1.0 + rand_f())),
      0.5 * rand_f())
  };
  make_sphere(c, 0.2, mat)
}

// Camera Functions 
def make_camera(wid:i64, ht:i64, ns:i64, pos:Vec3, look_at:Vec3, up:Vec3, fov:f64) : Camera {
  let dir : Vec3 = vec3_normalize(vec3_sub(look_at, pos));
  let right : Vec3 = vec3_normalize(vec3_cross(dir, up));
  let up : Vec3 = vec3_normalize(vec3_cross(right, dir));
  let pw : f64 = 2.0 / double_from_fnt(wid);
  let aspect : f64 = double_from_int(ht) / double_from_int(wid);
  let theta : f64 = (double_pi() * fov) / 180.0;
  let flen : f64 = 1.0 / double_tan(0.5 * theta);
  let imgCenter : Vec3 = vec3_add(pos, vec3_scale(flen, dir));
  let ulc : Vec3 = vec3_sub(vec3_add(imgCenter, vec3_scale(aspect, up)), right);
  Cam(wid, ht, ns, pos, ulc, vec3_scale(pw, right), vec3_scale(-pw, up))
}

def camera_rays_for_pixel(cam:Cam) : Fun[i64,Fun[i64,List[Ray]]]{
  new { Ap(r) => new {Ap(c) => 
    cam.case{
      Cam(wid, ht, ns, pos, ulc, hvec, vvec) => 
        let r : f64 = double_from_int(r) in
        let c : f64 = double_from_int(c) in 
        let ulc_dir : Vec3 = vec3_sub(ulc, pos) in
        let mk_ray : Fun[Unit,Ray] = new { Ap(u) =>
          let dir : Vec3 = vec3_adds(ulc_dir, r + rand_f(), vvec) in
          let dir : Vec3 = vec3_adds(dir, c + rand_f(), hvec) in   
          make_ray(pos, dir) 
        } in 
        list_ray_tabulate(ns,mk_ray)
    }
  }}
}

def camera_aa_pixel2rgb(cam:Camera, trace:Fun[Ray,Rgb]) : Fun[i64,Fun[i64,Rgb]]{
  cam.case {
    Cam(wid, ht, ns, pos, ulc, right, up) => 
      let rfp : Fun[i64,Fun[i64,Ray]] = camera_rays_for_pixel(cam) in
      let scale : f64 = if ns==0 { 1.0 } else { 1.0 / double_from_int(ns)) };
      new { Ap(x) => new { Ap(y) => 
        rgb_scale(scale, 
          list_ray_fold_rgb(
            new { Ap(ray) => new { Ap(c) => rgb_add(c, trace.Ap[Ray,Rgb](ray)) }}, 
            rgb_black(), rfp.Ap[i64,Fun[i64,Ray]](x).Ap[i64,Ray](y))   
      }
  }
}

def camera_make_pixel_renderer(to_rgb:Fun[i64,Fun[i64,Ray]],cvt:Fun[Rgb,Color]) : PixelRenderer {
  new { Ap(x) => new { Ap(y) => cvt.Ap[Ray,Color](to_rgb.Ap[i64,Fun[i64,Ray]](x).Ap[i64,Ray](y)) } }
}

def camera_for_each_pixel_row_lp(r:i64,colors:List[Color],wid:i64,pr:PixelRenderer) : List[Color] {
  if r<0{
    colors
  }else {
    camera_for_each_pixel_col_lp(r-1,wid-1,colors,pr)
  }
}

def camera_for_each_pixel_col_lp(r:i64,c:i64,colors:List[Color],pr:PixelRenderer) : List[Color] {
  if c<0{
    colors
  }else { 
    camera_for_each_pixel_col_lp(r,c-2,Cons(pr.Ap[i64,Fun[i64,Ray]](r).Ap[i64,Ray](c),colors)
  }
}

def camera_for_each_pixel(cam:Camera,pr:PixelRenderer) : Image {
  cam.case{
    Camera(wid,ht,ns,pos,ulc,hvec,vvec) =>  
      Image(wid, ht, camera_for_each_pixel_row_lp(ht-1,Nil,wid,pr))
  }
}

def camera_ray_to_rgb(ray:Ray) : Rgb {
  r.case {
    Ray(origin,dir) => dir.case {
      Vec3(x,y,z) => rgb_lerp(rgb_white(), 0.5 * (y + 1.0), Rgb(0.5, 0.7, 1.0))
    }
  }
}

// Scene functions
def lp_make_scene(x:i64,z:i64,objs:List[Object]) : Object {
  if z<11{ 
    lp_make_scene(x, z+1, Cons(random_sphere(x, z),objs))
  }else {
    if x<11 {
    lp_make_scene(x+1, -11, objs)
    }else {
      obs
    }
  }
}

def make_scene() : Object = 
  Object.fromList (lp (-11, -11, 
    Cons(make_sphere((0.0, -1000.0, 0.0), 1000.0,material_lambertian(rgb_grey(0.5))),
      Cons(make_sphere((4.0, 1.0, 0.0), 1.0,material_metal((0.7, 0.6, 0.5), 0.0)),
        Cons(make_sphere((-4.0, 1.0, 0.0), 1.0,material_lambertian(0.4, 0.2, 0.1)) ,Nil))) 
    ));

def trace_ray(world:Object, max_depth:i64) : Fun[Ray,Rgb] {
  world.case { Object(hit_test) => 
    let min_max_t : Interval = Interval(0.001, POS_INF);
    let trace : Fun[Ray,Fun[f64,Rgb]]= new { Ap(ray) => new { Ap(depth) => 
      if 0>depth{ 
        rgb_black() 
      } else  {
        hit_test.Ap[Ray,Fun[Interval,Option[Hit]]](ray).Ap[Interval,Option[Hit]](min_max_t).case[Object] {
          None => camera_ray_to_rgb(ray),
          Some(hit) => material_get_hit_info(hit, ray).case[Pair[Ray,Rgb]]{
            None => material_get_emission(hit),
            Some(p) => p.case { Tup(aten, reflect) => 
              rgb_add(material_get_emission(hit), rgb_modulate(aten, trace(reflect, depth-1)))}
          }
        }
      }
    };
    new { Ap(r) => trace.Ap[Ray,Fun[i64,Rgb]]](ray).Ap[i64,Rgb]](max_depth) }
  }

// Tracer Functions 
def build_scene() : Pair[Camera,Object] {
  let cam : Camera = make_camera(300, 200, 20,(13.0, 2.0, 3.0),vec3_zero(),(0.0, 1.0, 0.0),30.0) in            
  let world : Object = make_scene() in         
  Pair(cam, world)
}

def ray_tracer(p:Pair[Camera,Object]) : Image { 
  p.case[Camera,Object] { Pair(cam, world) =>  
    camera_for_each_pixel(cam,
      camera_make_pixel_renderer(camera_aa_pixel2rgb(cam, trace_ray(world, 20))),
      new { Ap(r) => color_from_rgb_with_gamma(r)})
  }
}

// Run Benchmark
def run(f:Fun[Unit,Image]) : Image { f.Ap(Unit) }

def test_random_scene() : Image {
  let scene : Pair[Camera,Object] = buildScene();
  run(new { Ap(u) => ray_tracer(scene) })
}

def main() : i64 {
  let res : Image = test_random_scene();
  0
}
