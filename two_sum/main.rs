fn main() {
	let inputs= [2, 7, 11, 15];
	let target= 18;
	let result= two_sum(&inputs, target);
	println!("result: {}, {}", result.0, result.1);
}

fn two_sum(inputs: &[i32], target: i32) -> (usize, usize) {
  // Start with naïve approach
	for first in 0..inputs.len(){
		for second in 0..inputs.len(){
			if first == second {
				continue;
			}
		  if target - inputs[first] == inputs[second] {
			  return (first, second)
		  }
		}
	}
	panic!("No match found");
}
