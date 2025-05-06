use std::collections::HashMap;

use stable_matching::stable_matching_asymmetric;

fn main() {
    // Residents and hospitals
    let residents = vec!["R1", "R2", "R3", "R4"];
    let hospitals = vec!["H1", "H2"];

    // Hospital capacities
    let capacities: HashMap<&str, usize> = [("H1", 2), ("H2", 1)].iter().cloned().collect();

    // Resident preference function
    let resident_preference = |resident: &&str, hospital: &&str| match (*resident, *hospital) {
        ("R1", "H1") => 1,
        ("R1", "H2") => 2,
        ("R2", "H1") => 1,
        ("R2", "H2") => 3,
        ("R3", "H1") => 2,
        ("R3", "H2") => 1,
        ("R4", "H1") => 3,
        ("R4", "H2") => 1,
        _ => 10,
    };

    // Hospital preference function
    let hospital_preference = |resident: &&str, hospital: &&str| match (*hospital, *resident) {
        ("H1", "R1") => 1,
        ("H1", "R2") => 2,
        ("H1", "R3") => 3,
        ("H1", "R4") => 4,
        ("H2", "R1") => 4,
        ("H2", "R2") => 3,
        ("H2", "R3") => 2,
        ("H2", "R4") => 1,
        _ => 10,
    };

    // Initial matching
    let mut matches = stable_matching_asymmetric(
        &residents,
        resident_preference,
        &hospitals,
        hospital_preference,
    );
    println!("{:?}", matches); // `{:?}` is used for printing a debug representation

    let v: Vec<i32> = vec![1, 2, 3];
    let u: Vec<i32> = vec![4, 5, 6];

    let ans = stable_matching::stable_matching_asymmetric(
        &u,
        |p1, p2| -(p1 - p2).abs(),
        &v,
        |p1, p2| (p1 - p2).abs(),
    );
    println!("{:?}", ans); // `{:?}` is used for printing a debug representation

    // println!("Hello, world!");
    //
    // let group1: Vec<i64> = vec![1, 2, 3, 4, 5];
    // let group2: Vec<i64> = vec![8, 9, 10, 11];
    //
    // let pairs =
    //     stable_matching::stable_matching_distance(&group1, &group2, |p1, p2| (p1 - p2).abs());
    // assert_eq!(pairs, vec![(4, 0), (3, 1), (2, 2), (1, 3)]);
    //
    // let pairs =
    //     stable_matching::stable_matching_distance(&group2, &group1, |p1, p2| (p1 - p2).abs());
    // assert_eq!(pairs, vec![(3, 1), (2, 2), (1, 3), (0, 4)]);
}
