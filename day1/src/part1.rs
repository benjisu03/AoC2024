
pub fn process(input: &str) -> anyhow::Result<String> {
	let mut left = Vec::new();
	let mut right = Vec::new();

	for line in input.lines() {
		let mut items = line.split_whitespace();
		left.push(
			items.next().unwrap().parse::<i32>().unwrap()
		);
		right.push(
			items.next().unwrap().parse::<i32>().unwrap()
		);
	}

	left.sort();
	right.sort();

	let result: i32 = std::iter::zip(left, right)
		.map(|(l, r)| (l - r).abs())
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
		assert_eq!("11", process(input)?);
		Ok(())
	}
}