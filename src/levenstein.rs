pub fn recursive(a: &str, b: &str) -> usize {
    if a.is_empty() {
        b.chars().count()
    } else if b.is_empty() {
        a.chars().count()
    } else {
        let mut ait = a.chars();
        let mut bit = b.chars();
        if ait.next().expect("non-empty") == bit.next().expect("non-empty") {
        recursive(ait.as_str(), bit.as_str())
        } else {
            recursive(a, bit.as_str())
                .min(recursive(ait.as_str(), b))
                .min(recursive(ait.as_str(), bit.as_str())) + 1
        }
    }
}

pub fn dynamic_wasteful(a: &str, b: &str) -> usize {
    let mut n = a.len();
    let mut m = b.len();
    let mut storage = vec![0; (n + 1) * (m + 1)];
    n = 0;
    for _ac in a.chars() {
        n += 1;
        storage[n] = n;
    }
    let w = n + 1;  // For indexing
    m = 0;
    for _bc in b.chars() {
        m += 1;
        storage[m * w] = m;
    }

    let mut j = 1;
    for bc in b.chars() {
        let mut i = 1;
        for ac in a.chars() {
            let diag = storage[(j - 1) * w + i - 1] + if ac == bc { 0 } else { 1 };
            storage[j * w + i] = diag.min(
                storage[(j - 1) * w + i].min(storage[j * w + i - 1]) + 1
            );
            i += 1;
        }
        j += 1;
    }

    // for j in 0..=m {
    //     for i in 0..w {
    //         print!("{} ", storage[j * w + i]);
    //     }
    //     print!("\n");
    // }

    storage[m * w + n]
}