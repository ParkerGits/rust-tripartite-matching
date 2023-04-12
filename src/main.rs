use std::{collections::HashSet, vec};

fn main() {
    let a1 = HashSet::from([1, 2, 3]);
    let b1 = HashSet::from([4, 5, 6]);
    let c1 = HashSet::from([7, 8, 9]);

    // expected matching: {1,4,7}, {2,5,8}, {3,6,9}
    let m1 = HashSet::from([
        [1, 5, 9],
        [1, 5, 6],
        [1, 4, 7],
        [2, 6, 9],
        [2, 5, 8],
        [3, 4, 7],
        [3, 6, 9],
        [3, 5, 9],
    ]);
    // expected matching: {1,6,9}, {2,4,7}, {3,5,8}
    let m2 = HashSet::from([
        [1, 6, 9],
        [1, 4, 9],
        [2, 4, 7],
        [2, 4, 8],
        [2, 6, 9],
        [3, 5, 8],
        [3, 5, 9],
        [3, 5, 7],
    ]);

    println!("{:?}", tripartite_matching(&a1, &b1, &c1, m1));
    println!("{:?}", tripartite_matching(&a1, &b1, &c1, m2));

    let a2 = HashSet::from([1, 2, 3, 4, 5]);
    let b2 = HashSet::from([6, 7, 8, 9, 10]);
    let c2 = HashSet::from([11, 12, 13, 14, 15]);

    // expected matching: {1,8,13}, {2,6,11}, {3,7,15}, {4,10,12}, {5,9,14}
    let m3 = HashSet::from([
        [1, 8, 13],
        [2, 6, 11],
        [3, 7, 15],
        [4, 10, 12],
        [5, 9, 14],
        [1, 6, 11],
        [1, 6, 12],
        [1, 9, 14],
        [2, 7, 13],
        [2, 9, 13],
        [3, 6, 15],
        [3, 9, 11],
        [4, 6, 15],
        [4, 7, 13],
        [5, 6, 11],
        [5, 10, 11],
    ]);
    println!("{:?}", tripartite_matching(&a2, &b2, &c2, m3));
}

fn tripartite_matching(
    a: &HashSet<i32>,
    b: &HashSet<i32>,
    c: &HashSet<i32>,
    m: HashSet<[i32; 3]>,
) -> Vec<HashSet<i32>> {
    // ensure a, b, c disjoint
    if !all_disjoint(&vec![a.clone(), b.clone(), c.clone()]) {
        panic!("A, B, C not disjoint")
    }

    // ensure |A| = |B| = |C|
    if !(a.len() == b.len() || a.len() == c.len() || b.len() == c.len()) {
        panic!("A, B, C unequal cardinality")
    }

    let k = a.len(); // = b.len() = c.len();

    // Construct universe U
    let mut u: HashSet<i32> = HashSet::new();
    for element in a {
        u.insert(*element);
    }
    for element in b {
        u.insert(*element);
    }
    for element in c {
        u.insert(*element);
    }

    // Construct set of subsets from set of triples
    let s: Vec<HashSet<i32>> = m.iter().map(|triple| HashSet::from(*triple)).collect();

    // Construction complete, set_cover returns matchings
    return set_cover(u, s, k);
}

fn set_cover(u: HashSet<i32>, s: Vec<HashSet<i32>>, k: usize) -> Vec<HashSet<i32>> {
    // Ensure all sets in S are subsets of U
    for set in &s {
        if !set.is_subset(&u) {
            panic!("S contains a set not of elements not contained in U")
        }
    }
    // number of combinations of subsets
    let num_combinations = 1 << s.len();
    for combination_bits in 1..num_combinations {
        let mut index = 0;
        let mut bits = combination_bits;
        let mut sets: Vec<HashSet<i32>> = vec![];
        while bits > 0 {
            if bits & 1 == 1 {
                // current combination selects set at index, add it to sets
                sets.push(s.get(index).unwrap().clone());
            }
            index += 1;
            bits >>= 1;
        }
        // We only care about combinations of k sets
        if sets.len() < k {
            continue;
        }

        // If selected sets are disjoint, we have set cover of size at least k, return true
        if all_disjoint(&sets) {
            return sets;
        }
    }
    return vec![];
}

fn all_disjoint(sets: &Vec<HashSet<i32>>) -> bool {
    let mut union: HashSet<&i32> = HashSet::new();
    for set in sets {
        for element in set {
            if union.contains(&element) {
                return false;
            }
            union.insert(element);
        }
    }
    return true;
}
