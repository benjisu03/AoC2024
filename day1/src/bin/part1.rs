use anyhow::Context;
use day1::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input/input.txt");
    let result = process(file).context("Part 1")?;
    println!("{:?}", result);
    Ok(())
}