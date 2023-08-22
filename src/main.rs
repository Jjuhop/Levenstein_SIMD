fn main() {
    let a = "öylätti";
    let b = "höylä";
    //println!("{}, {}", a.chars().count(), b.chars().count());
    let n = levenstein::dynamic_wasteful(a, b);
    println!("Distance {a} <=> {b} = {n}");
}