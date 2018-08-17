#[test]
fn euler_problem_1_test_1() {
    assert_eq!(euler_problem_1(10), 23 );
}

fn euler_problem_1( _max: u64) -> u64 {
  
  let mut result = 1;
  let mut i = 0;
  loop {
    if i >= _max {
      break;
    }
    if i % 3 == 0 || i % 5 == 0 {
      result += i;
    }
    i += 1;
  }
  result
}