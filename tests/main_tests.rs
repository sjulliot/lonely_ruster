use num_rational::Rational32;
use lonely_ruster::{is_solution, find_lonely_time};

#[test]
fn test_is_solution() {
    let speeds = vec![2, 3, 5];
    let t = Rational32::new(1, 4);
    assert!(is_solution(t, &speeds));

    let t = Rational32::new(1, 3);
    assert!(!is_solution(t, &speeds));
}

#[test]
fn test_find_lonely_time() {
    let speeds = vec![2, 3, 5];
    let result = find_lonely_time(&speeds);
    assert_eq!(result, Some(Rational32::new(1, 8)));
}