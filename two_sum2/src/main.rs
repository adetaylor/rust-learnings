use std::collections::HashMap;

fn main() {
  let inputs= [2, 7, 11, 15];
  let target= 18;
  let result= two_sum(&inputs, target);
  println!("result: {}, {}", result.0, result.1);
}

fn two_sum(inputs: &[i32], target: i32) -> (usize, usize) {
  let mut positions = HashMap::new();
  // O(n)
  for pos in 0..inputs.len() {
    positions.insert(inputs[pos], pos);
  }
  // O(n)
  for pos in 0..inputs.len() {
    let desired = target - inputs[pos];
    match positions.get(&desired) {
      None => continue,
      Some(second_pos) => return (pos, *second_pos)
    };
  }
  // Could do this second loop in O(log n) using binary
  // search of a sorted array, but there is no statement
  // that the array is sorted, so that would be O(n log n)
  panic!("No match found");
}

#[test]
#[should_panic]
fn empty() {
  assert_eq!(two_sum(&[], 9), (0,1));
}
#[test]
fn a() {
  assert_eq!(two_sum(&[2,7,11,15], 9), (0,1));
}
#[test]
#[should_panic]
fn nearlyblank() {
  assert_eq!(two_sum(&[0], 0), (0,1));
}
#[test]
fn b() {
  assert_eq!(two_sum(&[2,7,11,15], 18), (1, 2));
}
#[test]
#[should_panic]
fn c() {
  assert_eq!(two_sum(&[2,7,11,15], 399), (1, 2));
}
