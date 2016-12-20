pub fn puzzle(input: usize) -> usize {
    let mut circle: Vec<_> = (1..(input + 1)).collect();
    let mut start_at_0 = true;

    while circle.len() > 1 {

        let orig_len = circle.len();

        let next_circle: Vec<_> = match (circle.len() % 2 == 0, start_at_0) {
            (_, true)  => {
                circle.into_iter()
                      .enumerate()
                      .filter(|&(i, _)| i % 2 == 0)
                      .map(|(_, val)| val).collect()
            },
            (_, _)  => {
                circle.into_iter()
                      .enumerate()
                      .filter(|&(i, _)| i % 2 != 0)
                      .map(|(_, val)| val).collect()
            },
        };

        let started_even_length = orig_len % 2 == 0;
        let started_odd_length = !started_even_length;

        if start_at_0 && started_even_length {
            start_at_0 = true;
        } else if start_at_0 && started_odd_length {
            start_at_0 = false;
        } else if !start_at_0 && started_odd_length {
            start_at_0 = true;
        } else { // !start_at_0 && started_even_length
            start_at_0 = false;
        }
        circle = next_circle;
    }
    circle.pop().expect("horrible things")
}

// start_at_0 = false
// vec![1, 2, 3, 4] -
// remove even, 0-based positions
// start_at_0 = false

// vec![1, 2, 3, 4, 5, 6] - len().is_even() && start == 0 => remove odd, 0-based positions
// vec![1, 3, 5] - len().is_odd() && start == 0 => remove odd, 0-based positions
// vec![1, 5] - len().is_even() && start == end => remove even, 0-based positions
// vec![5]
//
// vec![1, 2, 3, 4, 5] - len().is_odd() && start == 0 => remove odd, 0-based positions
// vec![1, 3, 5] len().is_odd() && start == end => remove even, 0-based positions
// vec![3]




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_elves() {
        let winner = puzzle(2);
        assert_eq!(winner, 1);
    }

    #[test]
    fn five_elves() {
        let winner = puzzle(5);
        assert_eq!(winner, 3);
    }

}
