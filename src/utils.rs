use js_sys::Math::random;


// todo error checking with option
// because of floor it makes upper extremely unlikely
// todo add proper rounding
pub fn rand_int(lower:u32, upper:u32)->u32{
    let diff = upper - lower ;
    let mut res = lower +  (random() *  diff as f64) as u32;
    if res == upper{
        res = res - 1;
    }
    return res
}

pub fn rand_int_inclusive(lower:u32, upper:u32)->u32{
    let diff = upper - lower;
    return lower +  (random() *  diff as f64) as u32;
}



