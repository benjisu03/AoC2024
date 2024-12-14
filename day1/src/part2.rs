use std::collections::HashMap;

pub fn process(input: &str) -> anyhow::Result<String> {
	let mut left = HashMap::new();
	let mut right = HashMap::new();

	for line in input.lines() {
		let mut items = line.split_whitespace();

		let l = items.next().unwrap().parse::<i32>().unwrap();
		let r = items.next().unwrap().parse::<i32>().unwrap();

		*left.entry(l).or_insert(0) += 1;
		*right.entry(r).or_insert(0) += 1;
	}

	let result: i32 = left.iter()
		.map(|(val, l_count)| {
			let r_count = right.get(val).unwrap_or(&0);
			val * l_count * r_count
		})
		.sum();

	Ok(result.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> anyhow::Result<()> {
		let input = "3   4
4   3
2   5
1   3
3   9
3   3";
		assert_eq!("31", process(input)?);
		Ok(())
	}
}