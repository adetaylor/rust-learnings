fn main() {
    println!("Hello, world!");
}

fn next_permutation(list: &mut Vec<i32>) {
	list[0] = 1;
	list[1] = 3;
	list[2] = 2;
}

#[test]
fn a() {
	let mut a = [1,2,3].to_vec();
	next_permutation(&mut a);
	assert!(a == [1,3,2].to_vec())
}
#[test]
fn b() {
	let mut a = [3,2,1].to_vec();
	next_permutation(&mut a);
	assert!(a == [1,2,3].to_vec())
}
#[test]
fn c() {
	let mut a = [1,1,5].to_vec();
	next_permutation(&mut a);
	assert!(a == [1,5,1].to_vec())
}
#[test]
fn empty() {
	let mut a = [].to_vec();
	next_permutation(&mut a);
	assert!(a == [].to_vec())
}
#[test]
fn single() {
	let mut a = [6].to_vec();
	next_permutation(&mut a);
	assert!(a == [6].to_vec())
}
