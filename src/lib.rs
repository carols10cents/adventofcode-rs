
pub fn puzzle(input: &str) -> u32 {
    let mut num_triangles = 0;

    for line in input.lines() {
        let dims = line.split_whitespace();

        let dims = dims.map(|dim| {
            dim.parse::<u32>().expect("Could not parse!")
        }).collect::<Vec<_>>();

        println!("dimensions are: {}, {}, {}", dims[0], dims[1], dims[2]);

        if is_a_triangle(dims[0], dims[1], dims[2]) {
            num_triangles += 1;
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
  801  791  693
  572  150   74";
        assert_eq!(puzzle(input), 3);
    }
}
