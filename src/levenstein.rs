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

fn ascii_case_sens_inner(a: &[u8], b: &[u8]) -> usize {
    if a.is_empty() {
        b.len()
    } else if b.is_empty() {
        a.len()
    } else if a[0] == b[0] {
        ascii_case_sens_inner(&a[1..], &b[1..])
    } else {
        ascii_case_sens_inner(a, &b[1..])
            .min(ascii_case_sens_inner(&a[1..], b))
            .min(ascii_case_sens_inner(&a[1..], &b[1..])) + 1
    }
}

pub fn recursive_ascii_case_sens(a: &str, b: &str) -> usize {
    assert!(a.is_ascii() && b.is_ascii(), "Both strings must be valid ascii!");
    ascii_case_sens_inner(a.as_bytes(), b.as_bytes())
}

// **** DYN

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
    //         print!("{:>2} ", storage[j * w + i]);
    //     }
    //     print!("\n");
    // }

    storage[m * w + n]
}

pub fn dynamic_ascii_case_sensitive(a: &str, b: &str) -> usize {
    // Make it so that a is always shorter
    assert!(a.is_ascii() && b.is_ascii(), "Both strings must be ascii!");
    // Check if quick return is possible
    if a.is_empty() {
        return b.len();
    } else if b.is_empty() {
        return a.len();
    }
    // Must compute
    let (a, b) = if a.len() <= b.len() {
        (a.as_bytes(), b.as_bytes())
    } else {
        (b.as_bytes(), a.as_bytes())
    };
    let n = a.len();
    let mut storage = vec![0; 2 * n];
    // First row
    let mut found = false;
    let bfirst = b[0];
    for (ind, ach) in a.iter().enumerate() {
        found |= bfirst == *ach;
        storage[ind] = if found { ind } else { ind + 1 };
    }
    // "Pointers" to switch between the two parts
    let mut current = n;
    let mut previous = 0;
    // Rest of the rows
    for (bind, bch) in b.iter().enumerate().skip(1) {
        // First elem in a
        storage[current] = if *bch == a[0] { bind } else {bind + 1};
        // The rest
        for (aind, ach) in a.iter().enumerate().skip(1) {
            let diag = storage[previous + aind - 1] + if ach == bch { 0 } else { 1 };
            storage[current + aind] = diag.min(
                storage[current + aind - 1].min(storage[previous + aind]) + 1
            );
        }
        // println!("-------------");
        // for num in &storage[..n] {
        //     print!("{num} ");
        // }
        // print!("| ");
        // for num in &storage[n..] {
        //     print!("{num} ");
        // }
        // print!("\n");
        // Swap
        let temp = current;
        current = previous;
        previous = temp;
    }
    storage[previous + n - 1]
}