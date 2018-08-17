#[test]
fn euler_problem_1_test_1() {
    assert_eq!(euler_problem_1(10), 23 );
}

fn euler_problem_1( _max: u64) -> u64 {

  let mut result = 0;
  for i in 0.._max {
    if i % 3 == 0 || i % 5 == 0 {
      result += i;
    }
  }
  result

}