#![feature(portable_simd)]
fn main() {
    let a = "aita";
    let b = "maa";
    let m = levenstein::dynamic_wasteful(a, b);
    let n = levenstein::dynamic_ascii_case_sensitive(a, b);
    println!("Distance {a} <=> {b} = {n} (ref {m})");
}

//cargo rustc --release -- --emit asm