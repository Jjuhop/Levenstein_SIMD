//#![feature(portable_simd)]
//use std::simd::{*, num::SimdFloat};

fn main() {
    //let a = "rasvakeitin";
    //let b = "reissumies";
    //let a = "kuusi";
    //let b = "kusi";
    //let a = "korintin sota kaytiin vuosina 395-387 eaa";
    //let b = "sparta hyokkasi muun muassa liittolaisensa";
    //let a = "ffakdlfhaslkjdflaskjdfblaskjdbhflaksjdhfal";
    //let b = "fbawkjehbfaksdjnfasklfdjhgdbfkjhsabfkjahsdf";
    let a = "aita";
    let b = "maa";
    //println!("{}, {}", a.chars().count(), b.chars().count());
    let m = levenstein::dynamic_wasteful(a, b);
    //let n = levenstein::dynamic_ascii_case_sensitive(a, b);
    let n = levenstein::dynamic_simd_wasteful(a, b);
    println!("Distance {a} <=> {b} = {n} (ref {m})");

    // let v1 = f32x4::splat(1.0);
    // let v2 = f32x4::from_array([0.0, 0.5, 1.0, 2.0]);
    // let v3 = v1 + v2;
    // println!("{:?}", v3);
    // let v4: i32x4 = v2.cast();
    // let v5: i32x4 = v3.cast();
    // let v6 = v4 & v5;
    // println!("{:?}", v6);
}