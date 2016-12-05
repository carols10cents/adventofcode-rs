
pub fn puzzle(input: &str) -> u32 {
    let mut num_triangles = 0;

    // Deciding to stream rather than collect and use slice chunk
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        // Kinda... not great
        let chunk = [
            lines.next().expect("No first line"),
            lines.next().expect("No second line"),
            lines.next().expect("No third line"),
        ];

        let chunk: Vec<Vec<u32>> = chunk.iter().map(|line| {
            line.split_whitespace().map(|dim| {
                dim.parse::<u32>().expect("Could not parse!")
            }).collect()
        }).collect();

        for i in 0..2 {
            let dims: Vec<u32> = chunk.iter().map(|v| v[i] ).collect();
            println!("dimensions are: {}, {}, {}", dims[0], dims[1], dims[2]);

            if is_a_triangle(dims[0], dims[1], dims[2]) {
                num_triangles += 1;
            }
        }
    }

    num_triangles
}

pub fn is_a_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && b + c > a && c + a > b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equilateral_triangle() {
        assert!(is_a_triangle(3, 3, 3));
    }

    #[test]
    fn isoceles_triangle() {
        assert!(is_a_triangle(13, 13, 3));
    }

    #[test]
    fn acute_triangle() { // so cute
        assert!(is_a_triangle(208, 203, 145));
    }

    #[test]
    fn obtuse_triangle() {
        assert!(is_a_triangle(244, 360, 159));
    }

    #[test]
    fn not_a_triangle() {
        assert!( ! is_a_triangle(5, 10, 25) );
        assert!( ! is_a_triangle(5, 25, 10) );
        assert!( ! is_a_triangle(10, 5, 25) );
        assert!( ! is_a_triangle(10, 25, 5) );
        assert!( ! is_a_triangle(25, 5, 10) );
        assert!( ! is_a_triangle(25, 10, 5) );
    }

    #[test]
    fn whole_puzzle() {
        let input = "\
  785  516  744
  272  511  358
    1  791  600
";
        assert_eq!(puzzle(input), 2);
    }
}
