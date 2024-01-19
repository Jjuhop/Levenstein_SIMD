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
    let a_wide: Vec<u32> = a.chars()
        .map(|c| c as u32)
        .collect();
    let b_wide: Vec<u32> = b.chars()
        .map(|c| c as u32)
        .collect();
    let w = a_wide.len() + 1;    // The width of the dyn prog matrix
    let h = b_wide.len() + 1;
    let mut storage = vec![0; w * h];
    for c in 1..w {
        storage[c] = c;
    }
    // Now w is the actual needed width
    for r in 1..h {
        storage[r * w] = r;
    }

    let mut j = 1;
    for bc in &b_wide {
        let mut i = 1;
        for ac in &a_wide {
            let diag = storage[(j - 1) * w + i - 1] + if ac == bc { 0 } else { 1 };
            storage[j * w + i] = diag.min(
                storage[(j - 1) * w + i].min(storage[j * w + i - 1]) + 1
            );
            i += 1;
        }
        j += 1;
    }

    // for j in 0..h {
    //     for i in 0..w {
    //         print!("{:>2} ", storage[j * w + i]);
    //     }
    //     print!("\n");
    // }

    storage[h * w - 1]
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


use std::simd::{*, cmp::{SimdPartialEq, SimdOrd}};

const LANE_COUNT: usize = 16;

/*
The usual way to do Levenstein distance dynamically is to compute the (n+1)*(m+1)
matrix where n and m are the lengths of the words:

    00 01 02 03 04 05 06 07
    01 ?? ?? ?? ?? ?? ?? ??
    02 ?? ?? ?? ?? ?? ?? ??
    03 ?? ?? ?? ?? ?? ?? ??
    04 ?? ?? ?? ?? ?? ?? ??
    05 ?? ?? ?? ?? ?? ?? ??
    06 ?? ?? ?? ?? ?? ?? ??
    07 ?? ?? ?? ?? ?? ?? **

The first row and column are always like that because they correspond to situations
in which we take 0 characters from one word and some characters from the other word.
The interesting part is the n*m part filled with ?? and the final result will be at
the lower right corner **.

Since each entry in the ?? part depends on all the values to the left and up, we can
use SIMD vectors (here we assume 4 lanes) in the following manner

    00   01   02   03   04   05   06   07
    01   ??   ??  ?v0?  v1   v2   v3   v4 |v5  v6  v7
    02   ??  ?v0?  v1   v2   v3   v4   v5 |v6  v7  (needs some padding in the input character sequences)
    03  ?v0?  v1   v2   v3   v4   v5   v6 |v7
04->v0   v1   v2   v3   v4   v5   v6   v7
    05   ??   ??  ?v8?  v9  v10  v11  v12 |v13  v14
    06   ??  ?v8?  v9  v10  v11  v12  v13 |v14
    07  ?v8?  v9  v10  v11  v12  v13 *v14* <- ANSWER
08->v8   v9  v10  v11  v12  v13  v14       <- (This extra row needs also some padding in the input char sequences)

The storage for the SIMD vectors will have sufficient space for all of the needed
(n + 1) * round_up(m / LANES) vectors. Each SIMD vector will be oriented such that

     high end
       /
     /
  low end

The triangles that are left ??, also including ?v0? must be computed separately in the beginning.
The hypotenuse of each triangle overlaps with the first SIMD (v0 and v8 here), and those are created
based on the triangle and the border values.
*/

pub fn dynamic_simd_wasteful(a: &str, b: &str) -> usize {
    // Check if quick return is possible
    if a.is_empty() {
        return b.len();
    } else if b.is_empty() {
        return a.len();
    }

    let mut a_wide: Vec<u32> = a.chars()
        .map(|c| c as u32)
        .collect();
    let mut b_wide: Vec<u32> = b.chars()
        .map(|c| c as u32)
        .collect();
    if a_wide.len() < b_wide.len() {
        let temp = a_wide;
        a_wide = b_wide;
        b_wide = temp;
    }
    if a_wide.len() < LANE_COUNT {
        return dynamic_wasteful(a, b);
    }
    a_wide.extend(std::iter::repeat(0).take(LANE_COUNT - 1));

    // The width of the dyn prog matrix is a's original char len + 1
    let w = a_wide.len() - LANE_COUNT + 2;
    let h_scalar = b_wide.len();
    let h_simd = (h_scalar + LANE_COUNT - 1) / LANE_COUNT;
    let mut stor_simd = vec![Simd::<u32, LANE_COUNT>::splat(0); w * h_simd];
    let mut triangle: Vec<u32> = vec![0; (LANE_COUNT - 1) * (LANE_COUNT - 1)];    // Allocate memory equal to the square, not the triangle for simplicity

    // Pad also b_wide so that we get even blocks
    let pad_len = h_simd * LANE_COUNT - h_scalar;
    b_wide.extend(std::iter::repeat(0).take(pad_len));
    let mut b_iter = b_wide.into_iter().array_chunks::<LANE_COUNT>();

    // Helper const
    let simd_1 = Simd::<_, LANE_COUNT>::splat(1);

    let mut r_simd = 0;
    // The first row of the SIMDs needs special care since it needs some of the constants on the first row of the whole matrix
    if let Some(mut b_chunk) = b_iter.next() {
        // To match up with the a chunks, this needs to be reversed
        b_chunk.reverse();
        let b_chunk_simd = Simd::<u32, LANE_COUNT>::from_array(b_chunk);

        // Do the triangle part
        for r in 0..(LANE_COUNT - 1) {
            for c in 0..(LANE_COUNT - 1 - r) {
                let u: u32 = if r == 0 { c as u32 + 1 } else { triangle[(r - 1) * (LANE_COUNT - 1) + c] };
                let l: u32 = if c == 0 { r as u32 + 1 } else { triangle[r * (LANE_COUNT - 1) + c - 1] };
                let lu: u32 = if r == 0 || c == 0 { r.max(c) as u32 } else { triangle[(r - 1) * (LANE_COUNT - 1) + c - 1] };
                let diag = lu + if a_wide[c] == b_chunk_simd[LANE_COUNT - 1 - r] { 0 } else { 1 };
                triangle[r * (LANE_COUNT - 1) + c] = diag.min( l.min(u) + 1 );
            }
        }

        // Get ready to construct SIMDs v0 and v1 (need two in the beginning to get the loop for rest started)

        // Helpers
        let l_col: Vec<u32> = std::iter::once(LANE_COUNT as u32)
            .chain((0..(LANE_COUNT - 1)).map(|i| triangle[(LANE_COUNT - 2 - i) * (LANE_COUNT - 1) + i]))
            .collect();
        let l_simd = Simd::<u32, LANE_COUNT>::from_slice(&l_col);
        let u_simd = l_simd.rotate_elements_left::<1>();
        let lu_col: Vec<u32> = std::iter::once(LANE_COUNT as u32 - 1)
            .chain((0..(LANE_COUNT - 2)).map(|i| triangle[(LANE_COUNT - 3 - i) * (LANE_COUNT - 1) + i]))
            .chain(std::iter::once(LANE_COUNT as u32 - 1))
            .collect();
        let lu_simd = Simd::<u32, LANE_COUNT>::from_slice(&lu_col);

        // Get all the a windows that are gone through in this row
        let mut a_wnds = a_wide.windows(LANE_COUNT);
        let a_wnd_simd = Simd::<u32, LANE_COUNT>::from_slice(a_wnds.next().expect("Should have at least 1 window"));
        // Whether the characters at corresponding locations in a and b were equal
        let char_mask = a_wnd_simd.simd_eq(b_chunk_simd);
        let diag_delta_simd = Simd::<u32, LANE_COUNT>::gather_select(
            &[0],
            char_mask.into(),
            Simd::<_, LANE_COUNT>::splat(0),
            simd_1
        );
        let diag = lu_simd + diag_delta_simd;
        let v0 = diag.simd_min( l_simd.simd_min(u_simd) + simd_1 );
        
        stor_simd[0] = l_simd;
        stor_simd[1] = v0;

        //println!("Diag delta simd 1 {:?}", diag_delta_simd);

        // Now handle the rest of the first SIMD row
        let mut c = 2;
        for a_wnd in a_wnds {
            let l_simd = stor_simd[c - 1];
            let mut u_simd = l_simd.rotate_elements_left::<1>();
            u_simd[LANE_COUNT - 1] = (LANE_COUNT - 1 + c) as u32;
            let mut lu_simd = stor_simd[c - 2].rotate_elements_left::<1>();
            lu_simd[LANE_COUNT - 1] = (LANE_COUNT - 2 + c) as u32;
            
            //println!("Col {c}");
            //println!("Diag delta simd {:?}", diag_delta_simd);
            //println!("l simd {l_simd:?}\nu simd {u_simd:?}\nlu simd {lu_simd:?}");

            let a_wnd_simd = Simd::<u32, LANE_COUNT>::from_slice(a_wnd);
            let char_mask = a_wnd_simd.simd_eq(b_chunk_simd);
            let diag_delta_simd = Simd::<u32, LANE_COUNT>::gather_select(
                &[0],
                char_mask.into(),
                Simd::<_, LANE_COUNT>::splat(0),
                simd_1
            );
            let diag = lu_simd + diag_delta_simd;

            stor_simd[c] = diag.simd_min( l_simd.simd_min(u_simd) + simd_1 );
            c += 1;
        }

        r_simd += 1;
    }

    // Rest of the SIMD rows
    for mut b_chunk in b_iter {
        b_chunk.reverse();
        let b_chunk_simd = Simd::<u32, LANE_COUNT>::from_array(b_chunk);

        // Do the triangle part
        for r in 0..(LANE_COUNT - 1) {
            for c in 0..(LANE_COUNT - 1 - r) {
                let u: u32 = if r == 0 { stor_simd[(r_simd - 1) * w + c + 1][0] } else { triangle[(r - 1) * (LANE_COUNT - 1) + c] };
                let l: u32 = if c == 0 { (r + LANE_COUNT * r_simd) as u32 + 1 } else { triangle[r * (LANE_COUNT - 1) + c - 1] };
                let lu: u32 = if c == 0 { (r + LANE_COUNT * r_simd) as u32 }
                              else if r == 0 { stor_simd[(r_simd - 1) * w + c][0] }
                              else { triangle[(r - 1) * (LANE_COUNT - 1) + c - 1] };
                let diag = lu + if a_wide[c] == b_chunk_simd[LANE_COUNT - 1 - r] { 0 } else { 1 };
                triangle[r * (LANE_COUNT - 1) + c] = diag.min( l.min(u) + 1 );
            }
        }

        // Get ready to construct the first 2 SIMDs of this row

        // Helpers
        let l_col: Vec<u32> = std::iter::once((LANE_COUNT * (r_simd + 1)) as u32)
            .chain((0..(LANE_COUNT - 1)).map(|i| triangle[(LANE_COUNT - 2 - i) * (LANE_COUNT - 1) + i]))
            .collect();
        let l_simd = Simd::<u32, LANE_COUNT>::from_slice(&l_col);
        let mut u_simd = l_simd.rotate_elements_left::<1>();
        // Get the appropriate last element from the previous simd row
        u_simd[LANE_COUNT - 1] = stor_simd[(r_simd - 1) * w + LANE_COUNT][0];
        let lu_col: Vec<u32> = std::iter::once((LANE_COUNT * (r_simd + 1)) as u32 - 1)
            .chain((0..(LANE_COUNT - 2)).map(|i| triangle[(LANE_COUNT - 3 - i) * (LANE_COUNT - 1) + i]))
            .chain(std::iter::once(stor_simd[(r_simd - 1) * w + LANE_COUNT - 1][0]))
            .collect();
        let lu_simd = Simd::<u32, LANE_COUNT>::from_slice(&lu_col);

        // Get all the a windows that are gone through in this row
        let mut a_wnds = a_wide.windows(LANE_COUNT);
        let a_wnd_simd = Simd::<u32, LANE_COUNT>::from_slice(a_wnds.next().expect("Should have at least 1 window"));
        // Whether the characters at corresponding locations in a and b were equal
        let char_mask = a_wnd_simd.simd_eq(b_chunk_simd);
        let diag_delta_simd = Simd::<u32, LANE_COUNT>::gather_select(
            &[0],
            char_mask.into(),
            Simd::<_, LANE_COUNT>::splat(0),
            simd_1
        );
        let diag = lu_simd + diag_delta_simd;
        let v_first_of_row = diag.simd_min( l_simd.simd_min(u_simd) + simd_1 );
        
        stor_simd[r_simd * w] = l_simd;
        stor_simd[r_simd * w + 1] = v_first_of_row;

        // Now handle the rest of this SIMD row
        let mut c = 2;
        for a_wnd in a_wnds {
            let l_simd = stor_simd[r_simd * w + c - 1];
            let mut u_simd = l_simd.rotate_elements_left::<1>();
            u_simd[LANE_COUNT - 1] = stor_simd[(r_simd - 1) * w + c + LANE_COUNT - 1][0];
            let mut lu_simd = stor_simd[r_simd * w + c - 2].rotate_elements_left::<1>();
            lu_simd[LANE_COUNT - 1] = stor_simd[(r_simd - 1) * w + c + LANE_COUNT - 2][0];
            
            //println!("Col {c}");
            //println!("Diag delta simd {:?}", diag_delta_simd);
            //println!("l simd {l_simd:?}\nu simd {u_simd:?}\nlu simd {lu_simd:?}");

            let a_wnd_simd = Simd::<u32, LANE_COUNT>::from_slice(a_wnd);
            let char_mask = a_wnd_simd.simd_eq(b_chunk_simd);
            let diag_delta_simd = Simd::<u32, LANE_COUNT>::gather_select(
                &[0],
                char_mask.into(),
                Simd::<_, LANE_COUNT>::splat(0),
                simd_1
            );
            unsafe {
                std::arch::asm!("# Is_this_optimized;;;");
            }
            let diag = lu_simd + diag_delta_simd;

            stor_simd[r_simd * w + c] = diag.simd_min( l_simd.simd_min(u_simd) + simd_1 );
            c += 1;
        }

        r_simd += 1;
    }

    // for row_simd in stor_simd.chunks(w) {
    //     for ss in row_simd {
    //         println!("{:?}", ss);
    //     }
    //     println!("*******\n");
    // }
    println!("h_simd {h_simd}, pad_len {pad_len}");
    stor_simd[h_simd * w - 1 - pad_len][pad_len] as usize
}