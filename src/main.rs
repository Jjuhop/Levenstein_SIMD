fn main() {
    //let a = "rasvakeitin";
    //let b = "reissumies";
    let a = "kuusi";
    let b = "kusi";
    //let a = "korintin sota kaytiin vuosina 395-387 eaa";
    //let b = "sparta hyokkasi muun muassa liittolaisensa";
    //println!("{}, {}", a.chars().count(), b.chars().count());
    //let n = levenstein::dynamic_wasteful(a, b);
    let n = levenstein::dynamic_ascii_case_sensitive(a, b);
    println!("Distance {a} <=> {b} = {n}");
}