data Unit { Unit }
data Bool { True, False}

data Vec3 { Vec3(x:f64,y:f64,z:f64) }
data Object { Object(hit_test:FunRayIntOptionHit) }
data Image { Image(width:i64,height:i64,colors:ListColor) } 
data Camera { Cam(width:i64,height:i64,ns:Vec3,pos:Vec3,ulc:Vec3,right:Vec3,up:Vec3) }
data RayTracer { Tracer(Camera,Object) }
data Ray { Ray(origin:Vec3,dir:Vec3) }
data Color { Color(r:i64,g:i64,b:i64) }
data Rgb { Rgb(r:f64,g:f64,b:f64) }
data Material { Material(emit:FunHitRgb, scatter:FunRayHitOptionRayRgb)}
data Hit { Hit(t:f64,pt:Vec3,norm:Vec3,mat:Material) }
data Interval { Interval(low:f64,high:f64) }

data OptionTwoRgb { None2R, Some2R(r1: Rgb,r2: Rgb) }
data OptionRayRgb { NoneRR, SomeRR(ray: Ray,rgb: Rgb) }
data OptionHit { Miss, SomeH(hit:Hit) }

data PairCameraObject { PairCO(Object,Camera) }

data ListObject { NilO, ConsO(o:Object,os:ListObject)}
data ListColor { NilC, ConsC(c:Color,cs:ListColor)}

codata FunUnitImage { ApUI(u:Unit) : Image }
codata FunRayI64Rgb { ApRFR(r:Ray,i:i64) : Rgb } 
codata FunRayRgb { ApRR(r:Ray) : Rgb }
codata FunHitRgb { ApHR(h:Hit) : Rgb }
codata FunRayHitOptionRayRgb { ApRHORR(r:Ray,h:Hit) : OptionRayRgb }
codata FunRayIntOptionHit { ApRIOH(r:Ray,int:Interval) : OptionHit }
codata FunObjectOptionHitOptionHit { ApOOHOOH(obj:Object,oh:OptionHit) : OptionHit }

// Random functions 

def rand_f() : f64 := TODO;

// f64 functions 

def double_from_int(i:i64) : f64 := 0.0;
def double_pi() : f64 := 3.1415;
def double_tan(f:f64) : f64 := (f + (((f*f)*f)/3.0)) + ((2.0/15.0)*((((f*f)*f)*f)*f));
def double_sqrt(f:f64) : f64 := f;

// Rgb functions

def rgb_black() : Rgb :=  Rgb(0.0,0.0,0.0);

def rbg_grey(v:f64) : Rbg := Rgb(v,v,v);

def rgb_add(rgb1:Rgb,rgb2:Rgb) : Rgb := rgb1.case{
  Rgb(r1:f64,g1:f64,b1:f64) => rgb2.case{
    Rgb(r2:f64,g2:f64,b2:f64) => Rgb(r1+r2,g1+g2,b1+b2)
  }
};

def rgb_modulate(rgb1:Rgb,rgb2:Rgb) : Rgb := rgb1.case{
  Rgb(r1:f64,g1:f64,b1:f64) => rgb2.case{
    Rgb(r2:f64,g2:f64,b2:f64) => Rgb(r1*r2,g1*g2,b1*b2)
  }
};

// Vec3 Functions 

def vec3_zero() : Vec3 := Vec3(x:0.0,y:0.0,z:0.0);

def vec3_nomalize(v:Vec3) : Vec3 := TODO;
def vec3_add(v1:Vec3,v2:Vec3) :Vec3:= TODO;
def vec3_sub(v1:Vec3,v2:Vec3) : Vec3 := TODO;
def vec3_cross(v1:Vec3,v2:Vec3) : Vec3:= TODO;
def vec3_scale(v:Vec3,s:f64) : Vec3 :=TODO;
def vec3_dot(v1:Vec3,v2:Vec3) : f64 :=TODO;
def vec3_adds(v1:Vec3,f:f64,v2:Vec3) : Vec3 := TODO;
def vec3_reflect(v1:Vec3,v2:Vec3):Vec3:=TODO;

def vec3_random_point_in_sphere() : Vec3 := 
  let pt = Vec3(rand_f(), rand_f(), rand_f()) in  
  ifl(vec3_dot(pt, pt),1.0,pt,randomPointInSphere());

// Interval Functions 

def interval_within(int:Interval,f:f64) : Bool :=TODO;

// Ray Functions 

def make_ray(origin:Vec3,dir:Vec3) : Ray := Ray(origin,vec3_normalize(dir));
def ray_eval(r:Ray, t:f64) :Vec3 := r.case { Ray(fst,snd) => vec3_adds (fst, t, snd) };

// Material Functions 

def material_get_hit_info(hit:Hit,ray:Ray) : OptionRayRgb := hit.case {
  Hit(t:f64, pt:Vec3, norm:Vec3, mat:Material) => mat.case{
    Material(emit:FunHitRgb, scatter:FunRayHitOptionRayRgb)) => scatter.ApRHORR(ray, hit)
  }
};

def material_get_emission(hit:Hit) : Rgb := hit.case {
  Hit(t:f64, pt:Vec3, norm:Vec3, mat:Material) => mat.case{ 
    Material(emit:FunHitRgb, scatter:FunRayHitOptionRayRgb)) => emit.ApHR(hit)
  }
};

def material_lambertian(albedo:Rbg) : Material := 
  Material(
    cocase { ApHR(hit:Hit) => rgb_black() },
    cocase { ApRHORR(ray:Ray,hit:Hit) => hit.case {
      Hit(t:f64,pt:Vec3,norm:Vec3,mat:Material) => 
        SomeRR(make_ray(pt, vec3_add(norm, vec3_random_point_in_sphere())), albedo)
    }} 
  );

def material_metal(albedo:Rbg,fuzz:f64) : Material := 
  Material( 
    cocase { ApHR(hit:Hit) => rbg_black() },
    cocase { ApRHORR(ray:Ray,hit:Hit) => hit.case{
      Hit(t:f64,pt:Vec3,norm:Vec3,mat:Material) => 
        let dir : Vec3 = vec3_adds(vec3_reflect(rdir, norm),fuzz,vec3_random_point_in_sphere()) in   
        ifl(0.0,vec3_dot(dir, norm),
          None,SomeRR(albedo, make_ray(pt, dir)))
    }}
  );

// Sphere Functions 

def make_sphere(center:Vec3,radius:f64,material:Material) : Object := 
  let r_sq : f64 = radius * radius in 
  let inv_r : f64 = 1.0 / radius in 
  fun hit_test : FunRayIntOptionHit = cocase { ApRIOH(ray:Ray, min_max_t:Interval) => 
    ray.case { Ray(ro:Vec3, rd:Vec3) =>
      let q : Vec3 = vec3_sub(ro, center) in
      let b : f64 = 2.0 * vec3_dot(rd, q) in 
      let c : f64 = vec3_dot(q, q) - rSq in
      let disc : f64 = b*b - 4.0*c in   
      ifl(disc,0.0,Miss,
        let t : f64 = 0.5 * ((-b) - double_sqrt(disc)) in 
        interval_within(min_max_t,t).case {
          True => 
            let pt : Vec3 = ray_eval(ray, t) in       
            SomeH(Hit(t, pt, vec3_scale(inv_r, vec3_sub(pt, center)), material)),
          False => Miss
        }
  } } in
  Object(hit_test);

// Object Functions

def object_empty() : Object := Object(cocase { ApRIOH(ray:Ray,int:Interval) => Miss });

def fold_o(l:ListObject,start:OptionHit,f:FunObjectOptionHitOptionHit): OptionHit := l.case{
  NilO => start,
  ConsO(obj:Object,objs:ListObject) => fold_o(objs,f.ApOOHOOH(obj,start),f)
};

def closer(mhit1:OptionHit, mhit2:OptionHit) : OptionHit := mhit1.case{
  Miss => mhit2, 
  SomeH(hit1:Hit) => mhit2.case{
    Miss => Some(hit),
    Some(hit2:Hit) => hit1.case {
      Hit(t1:f64,pt1:Vec3,norm1:Vec3,mat1:Material) => hit2.case {
        Hit(t2:f64,pt2:Vec3,norm2:Vec3,mat2:Material) => ifg(t2,t1,SomeH(hit1),SomeH(hit2))
      }
    }
  }
};

def object_from_list(objs:ListObject) : Object := objs.case{
  NilO => = object_empty()      
  ConsO(obj1:Object,objs:ListObject) => objs.case{
    NilO => obj1, 
    Cons(obj2:Object,objs2:ListObject) => 
      let hit_test : FunRayIntOptionHit = cocase { ApRIOH(ray:Ray,min_max_t:Interval) => 
        fold_o(objs,Miss,cocase { ApOOHOOH(obj:Object, mhit:OptionHit) => 
          obj.case { Object(hit_test:FunRayIntOptionHit) => closer(mhit, hit_test.ApRIOH(ray, min_max_t)) }
      }) in
      Object(hit_test)
  }
};

def random_sphere(x:i64, z:i64) : Object = 
  let choose_mat : f64 = rand_f() in
  let x : f64 = double_from_int(x) + (0.9*rand_f()) in 
  let z : f64 = double_from_int(z) + (0.9*rand_f()) in 
  let c : Vec3 = Vec3(x,0.2,z) in 
  let mat = ifl(choose_mat,0.8, 
    material_lambertian(Rgb(rand_f() * rand_f(),rand_f() * rand_f(),rand_f() * rand_f())),
    material_metal(
      Rgb(0.5 * (1.0 + rand_f()),0.5 * (1.0 + rand_f()),0.5 * (1.0 + rand_f())),
      0.5 * rand_f())) in     
  make_sphere(c, 0.2, mat);

// Camera Functions 
def make_camera(wid:i64, ht:i64, ns:Vec3, pos:Vec3, look_at:Vec3, up:Vec3, fov:f64) : Camera := 
  let dir : Vec3 = vec3_normalize(vec3_sub(look_at, pos)) in
  let right : Vec3 = vec3_normalize(vec3_cross(dir, up)) in 
  let up : Vec3 = vec3_normalize(vec3_cross(right, dir)) in 
  let pw : f64 = 2.0 / double_from_fnt(wid) in 
  let aspect : f64 = double_from_int(ht) / double_from_int(wid) in
  let theta : f64 = (double_pi() * fov) / 180.0 in
  let flen : f64 = 1.0 / double_tan(0.5 * theta) in
  let imgCenter : Vec3 = vec3_add(pos, vec3_scale(flen, dir)) in
  let ulc : Vec3 = vec3_sub(vec3_add(imgCenter, vec3_scale(aspect, up)), right) in
  Cam(wid, ht, ns, pos, ulc, vec3_scale(pw, right), vec3_scale(-pw, up));


// Scene functions
def lp_make_scene(x:i64,z:i64,objs:ListObject) = ifl(z,11,
  lp_make_scene(x, z+1, ConsO(random_sphere(x, z),objs)),
  ifl(x,11,lp_make_scene(x+1, -11, objs),obs));

def make_scene() : Object = 
  Object.fromList (lp (-11, -11, 
    ConsO(make_sphere((0.0, -1000.0, 0.0), 1000.0,material_lambertian(rgb_grey(0.5))),
      ConsO(make_sphere((4.0, 1.0, 0.0), 1.0,material_metal((0.7, 0.6, 0.5), 0.0)),
        ConsO(make_sphere((-4.0, 1.0, 0.0), 1.0,material_lambertian(0.4, 0.2, 0.1)) ,NilO))) 
    ));

def trace_ray(world:Object, max_depth:i64) : FunRayRgb := 
  let minMaxT : Interval = Interval(0.001, POS_INF) in  
  let trace : FunRayF64Rgb = cocase { ApRFR(ray:Ray, depth:i64) => ifg(0,depth,rgb_black(),
  case Object.hitTest(world, ray, minMaxT).case {    
    Object.Miss => Camera.rayToRGB ray     
    Object.Hit hit => material_get_hit_info(hit, ray).case{
      None2R => material_get_emission(hit),
      Some2R(aten:Rgb, reflect:Rgb) => rgb_add(material_get_emission(hit), rgb_modulate(aten, trace(reflect, depth-1)))
    }
  }) in
  cocase { ApRR(r:ray) => trace.ApRFR(ray, max_depth) };

// Tracer Functions 
def build_scene() : PairCameraObject := 
  let cam : Camera = make_camera(300, 200, 20,(13.0, 2.0, 3.0),vec3_zero(),(0.0, 1.0, 0.0),30.0) in            
  let world : Object = make_scene() in         
  PairCO(cam, world);

def ray_tracer(p:PairCameraObject) : Image := p.case { PairCO(cam:Camera, world:Object) =>  
  Camera.foreachPixel(cam,
    Camera.makePixelRenderer(Camera.aaPixelToRGB(cam, trace_ray(world, 20)),Color.fromRGBWithGamma))
};

// Run Benchmark
def run(f:FunUnitImage) : Image := f.ApUI(Unit);

def test_random_scene() : Image := 
  let scene : PairCameraObject = buildScene() in         
run(cocase { ApUI(u:Unit) => ray_tracer(scene) });

def main() : i64 := let res : Image = test_random_scene() in 0;
