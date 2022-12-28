type Input = Vec<Vec<u64>>;
type Output = u64;

pub fn parser(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|snack| snack.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(input: &Input) -> Output {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .unwrap()
}

pub fn part_2(input: &Input) -> Output {
    let mut elf_totals = input
        .into_iter()
        .map(|chunk| chunk.into_iter().sum())
        .collect::<Vec<_>>();
    elf_totals.sort_unstable();
    elf_totals.iter().rev().take(3).sum()
}
