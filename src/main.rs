#[test]
fn euler_problem_1_test_1() {
    assert_eq!(euler_problem_1(10), 23 );
}

fn euler_problem_1( _max: u64) -> u64 {

 (0.._max).map( |x| {println!("{}", x) ; x} ).filter(|n| n % 3 == 0 || n % 5 == 0).sum()

}