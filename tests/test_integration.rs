use xensieve::Sieve;

//------------------------------------------------------------------------------
#[test]
fn test_sieve_new_a() {
    let s1: Sieve<u64> = Sieve::new("3@0|5@1|5@4");
    assert_eq!(s1.to_string(), "Sieve{3@0|5@1|5@4}");
}

#[test]
fn test_sieve_new_generic_u32() {
    let s1: Sieve<u32> = Sieve::new("3@0|5@1|5@4");
    assert_eq!(s1.to_string(), "Sieve{3@0|5@1|5@4}");
    assert_eq!(s1.contains(6), true);
}

#[test]
fn test_sieve_new_b() {
    let s1: Sieve<u64> = Sieve::new("!(3@0|5@1|5@4)|9@6");
    assert_eq!(s1.to_string(), "Sieve{!(3@0|5@1|5@4)|9@6}");
}

#[test]
fn test_sieve_new_c() {
    // examples in the README

    let s1: Sieve<u64> = Sieve::new("5@0");
    assert_eq!(
        s1.iter_value(0..50).collect::<Vec<_>>(),
        vec![0, 5, 10, 15, 20, 25, 30, 35, 40, 45]
    );

    let s2: Sieve<u64> = Sieve::new("30@10");
    assert_eq!(s2.iter_value(0..50).collect::<Vec<_>>(), vec![10, 40]);

    let s3: Sieve<u64> = Sieve::new("(5@0|4@2)&!30@10");
    assert_eq!(
        s3.iter_value(0..50).collect::<Vec<_>>(),
        vec![0, 2, 5, 6, 14, 15, 18, 20, 22, 25, 26, 30, 34, 35, 38, 42, 45, 46]
    );

    assert_eq!(
        s1.iter_state(0..10).collect::<Vec<_>>(),
        vec![true, false, false, false, false, true, false, false, false, false]
    );

    assert_eq!(
        s3.iter_state(0..10).collect::<Vec<_>>(),
        vec![true, false, true, false, false, true, true, false, false, false]
    );

    assert_eq!(s2.iter_interval(0..50).collect::<Vec<_>>(), vec![30]);
    assert_eq!(
        s3.iter_interval(0..50).collect::<Vec<_>>(),
        vec![2, 3, 1, 8, 1, 3, 2, 2, 3, 1, 4, 4, 1, 3, 4, 3, 1]
    );

    assert_eq!(s1.contains(5), true);
    assert_eq!(s1.contains(6), false);
    assert_eq!(s3.contains(10), false);
    assert_eq!(s3.contains(30), true);

    let s4: Sieve<u64> = (Sieve::new("5@0") | Sieve::new("4@2")) & !Sieve::new("30@10");
    assert_eq!(s4.to_string(), "Sieve{5@0|4@2&!(30@10)}");
    assert_eq!(
        s3.iter_value(0..100).collect::<Vec<_>>(),
        s4.iter_value(0..100).collect::<Vec<_>>()
    );
}

#[test]
fn test_sieve_new_d() {
    let s1: Sieve<u8> = Sieve::new("!(3@0|5@1|5@4)|9@6");
    assert_eq!(s1.to_string(), "Sieve{!(3@0|5@1|5@4)|9@6}");
}


//------------------------------------------------------------------------------

#[test]
fn test_sieve_contains_c() {
    let s1: Sieve<u64> = Sieve::new("5@0") | Sieve::new("5@1") | Sieve::new("5@4");

    assert_eq!(s1.contains(-2), false);
    assert_eq!(s1.contains(-1), true);
    assert_eq!(s1.contains(0), true);
    assert_eq!(s1.contains(1), true);
    assert_eq!(s1.contains(2), false);
    assert_eq!(s1.contains(3), false);
    assert_eq!(s1.contains(4), true);
    assert_eq!(s1.contains(5), true);
    assert_eq!(s1.contains(5), true);
}

#[test]
fn test_sieve_contains_d() {
    let s1: Sieve<u64> = !(Sieve::new("5@0") | Sieve::new("5@1") | Sieve::new("5@4"));

    assert_eq!(s1.to_string(), "Sieve{!(5@0|5@1|5@4)}");

    assert_eq!(s1.contains(-2), true);
    assert_eq!(s1.contains(-1), false);
    assert_eq!(s1.contains(0), false);
    assert_eq!(s1.contains(1), false);
    assert_eq!(s1.contains(2), true);
    assert_eq!(s1.contains(3), true);
    assert_eq!(s1.contains(4), false);
    assert_eq!(s1.contains(5), false);
    assert_eq!(s1.contains(5), false);
}

#[test]
fn test_sieve_iter_int_a() {
    let s1: Sieve<u64> = !(Sieve::new("5@0") | Sieve::new("5@1") | Sieve::new("5@4"));

    let post1: Vec<_> = s1.iter_value(0..10).collect();
    assert_eq!(post1, vec![2, 3, 7, 8]);

    let post2: Vec<_> = s1.iter_value(-10..10).collect();
    assert_eq!(post2, vec![-8, -7, -3, -2, 2, 3, 7, 8]);
}

#[test]
fn test_sieve_iter_int_b() {
    let s1: Sieve<u64> = Sieve::new("1@0");

    let post1: Vec<_> = s1.iter_value(0..4).collect();
    assert_eq!(post1, vec![0, 1, 2, 3]);

    let post2: Vec<_> = s1.iter_value(0..=4).collect();
    assert_eq!(post2, vec![0, 1, 2, 3, 4]);

    let post2: Vec<_> = s1.iter_value((0..=4).step_by(2)).collect();
    assert_eq!(post2, vec![0, 2, 4]);

    let post2: Vec<_> = s1.iter_value((0..=2).rev()).collect();
    assert_eq!(post2, vec![2, 1, 0]);
}

#[test]
fn test_sieve_iter_int_c() {
    let s1: Sieve<u64> = Sieve::new("3@0&4@0");
    let post1: Vec<_> = s1.iter_value(0..=24).collect();
    assert_eq!(post1, vec![0, 12, 24]);
}

#[test]
fn test_sieve_iter_int_d() {
    let s1: Sieve<u64> = Sieve::new("(7@0 | 8@1 | 8@6 ) & !(24@7 | 24@17)");
    let post1: Vec<_> = s1.iter_value(0..=24).collect();
    assert_eq!(post1, vec![0, 1, 6, 9, 14, 21, 22]);
}

#[test]
fn test_sieve_iter_int_e() {
    let s1: Sieve<u64> = Sieve::new("(3@0 | 4@1) & !(12@1 | 12@14)");
    let post1: Vec<_> = s1.iter_value(0..=12).collect();
    assert_eq!(post1, vec![0, 3, 5, 6, 9, 12]);
}

#[test]
fn test_sieve_iter_int_f() {
    let s1: Sieve<u64> = Sieve::new("0@0");
    let post1: Vec<_> = s1.iter_value(0..=12).collect();
    assert_eq!(post1, vec![]);
}

#[test]
fn test_sieve_iter_int_g() {
    let s1: Sieve<u64> = Sieve::new("3@0&3@2");
    let post1: Vec<_> = s1.iter_value(0..=12).collect();
    assert_eq!(post1, vec![]);
}

// 7@0|{-5@2&-4@3}'

#[test]
fn test_sieve_iter_int_h() {
    let s1: Sieve<u64> = Sieve::new("7@0 | (!5@2 & !4@3)");
    let post1: Vec<_> = s1.iter_value(0..=12).collect();
    assert_eq!(post1, vec![0, 1, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_sieve_iter_int_i() {
    let s1: Sieve<u64> = Sieve::new("!(10@1 | 10@2) ^ !(10@7 | 10@8)");
    let post1: Vec<_> = s1.iter_value(0..=10).collect();
    assert_eq!(post1, vec![1, 2, 7, 8]);
}

//------------------------------------------------------------------------------

#[test]
fn test_sieve_iter_state_a() {
    let s1: Sieve<u64> = Sieve::new("3@0") | Sieve::new("5@1");

    let post1: Vec<_> = s1.iter_state(0..10).collect();
    assert_eq!(
        post1,
        vec![true, true, false, true, false, false, true, false, false, true]
    );
}

//------------------------------------------------------------------------------

#[test]
fn test_sieve_iter_interval_a() {
    let s1: Sieve<u64> = Sieve::new("3@0") | Sieve::new("4@1");

    let post1: Vec<_> = s1.iter_interval(0..10).collect();
    assert_eq!(post1, vec![1, 2, 2, 1, 3]);
}

#[test]
fn test_sieve_iter_interval_b() {
    let s1: Sieve<u64> = Sieve::new("5@0") | Sieve::new("7@1");

    let post1: Vec<_> = s1.iter_interval(-20..30).collect();
    assert_eq!(post1, vec![5, 2, 3, 4, 1, 5, 1, 4, 3, 2, 5, 5, 2, 3, 4]);
}
