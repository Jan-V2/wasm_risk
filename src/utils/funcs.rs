#![allow(dead_code)]
use js_sys::Math::random;



pub fn rand_int(lower:u32, upper:u32)->u32{
    let diff = upper - lower ;
    (lower as f64 +  (random() *  diff as f64)).round() as u32
}

pub fn rand_int_inclusive(lower:u32, upper:u32)->u32{
    let diff = upper - lower;
    return (lower as f64 +  (random() *  diff as f64)).round() as u32;
}




