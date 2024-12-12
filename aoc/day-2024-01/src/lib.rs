pub fn loop_match(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut ic: usize = 0;
    let mut jc: usize = 0;
    let mut n: usize = 0;
    let mut sum: usize = 0;

    loop {
        if i >= a.len() || j >= b.len() {
            sum += ic * jc * n;
            break;
        }

        match (a[i], b[j]) {
            (_, n_b) if n_b < n => j += 1, // Ensure `b[j]` keeps up with `a`
            (n_a, _) if n_a == n => {
                ic += 1;
                i += 1;
            }
            (_, n_b) if n_b == n => {
                jc += 1;
                j += 1;
            }
            _ => {
                sum += ic * jc * n;
                ic = 0;
                jc = 0;
                n = a[i];
            }
        }
    }

    sum
}

pub fn while_match(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut ic: usize = 0;
    let mut jc: usize = 0;
    let mut n: usize = 0;
    let mut sum: usize = 0;

    while j < b.len() && i < a.len() {
        match (a[i], b[j]) {
            (_n_a, n_b) if n_b < n => j += 1, // Ensure `b[j]` keeps up with `a`
            (n_a, _n_b) if n_a == n => {
                ic += 1;
                i += 1;
            }
            (_n_a, n_b) if n_b == n => {
                jc += 1;
                j += 1;
            }
            _ => {
                sum += ic * jc * n;
                ic = 0;
                jc = 0;
                n = a[i];
            }
        }
    }
    sum += ic * jc * n;
    sum
}

pub fn while_if(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut ic: usize = 0;
    let mut jc: usize = 0;
    let mut n: usize = 0;
    let mut sum: usize = 0;

    while j < b.len() && i < a.len() {
        // println!("i:{} j:{} ic:{} jc:{} n:{}", i, j, ic, jc, n);
        if b[j] < n {
            // make sure that b[j] doesnt fall behind
            // as a dictates where we're going
            j += 1;
        } else if a[i] == n {
            ic += 1;
            i += 1;
        } else if b[j] == n {
            jc += 1;
            j += 1;
        } else {
            sum += ic * jc * n;
            ic = 0;
            jc = 0;
            n = a[i];
        }
    }
    sum += ic * jc * n;

    sum
}

pub fn while_let(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut ic: usize = 0;
    let mut jc: usize = 0;
    let mut n: usize = 0;
    let mut sum: usize = 0;

    let mut a_iter = a.iter();
    let mut b_iter = b.iter();

    let mut a_val = a_iter.next();
    let mut b_val = b_iter.next();

    while let (Some(&n_a), Some(&n_b)) = (a_val, b_val) {
        match (n_a, n_b) {
            (_, nb) if nb < n => b_val = b_iter.next(),
            (na, _) if na == n => {
                ic += 1;
                a_val = a_iter.next();
            }
            (_, nb) if nb == n => {
                jc += 1;
                b_val = b_iter.next();
            }
            _ => {
                sum += ic * jc * n;
                ic = 0;
                jc = 0;
                n = n_a;
            }
        }
    }

    // Final addition for remaining matches
    sum += ic * jc * n;

    sum
}
