pub fn puzzle(input: usize) -> usize {
    let mut circle = vec![1; input];
    let mut current_elf = 0;

    loop {

        let next_elf_with_presents = circle[current_elf + 1..].iter()
            .position(|&num| num != 0);
        match next_elf_with_presents {
            Some(i) => {
                let absolute_next_elf = i + current_elf + 1;
                circle[current_elf] += circle[absolute_next_elf];
                circle[absolute_next_elf] = 0;
            },
            None => {
                let next_elf_with_presents = circle[0..current_elf].iter()
                    .position(|&num| num != 0);

                match next_elf_with_presents {
                    Some(i) => {
                        circle[current_elf] += circle[i];
                        circle[i] = 0;
                    },
                    None => return current_elf + 1,
                }
            },
        }

        match circle[current_elf + 1..].iter().position(|&num| num != 0) {
            Some(i) => current_elf = i,
            None => {
                let next_elf_with_presents = circle[0..current_elf].iter()
                    .position(|&num| num != 0);

                match next_elf_with_presents {
                    Some(i) => current_elf = i,
                    None => panic!("something has gone horribly wrong"),
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_elves() {
        let winner = puzzle(2);
        assert_eq!(winner, 1);
    }

}
