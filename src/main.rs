mod parts;

fn get_input_lines(filename: &str) -> Vec<String> {
    let lines: Vec<String> = std::fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect();

    lines
}

fn main() {
    let lines: Vec<String> = get_input_lines("inputs/input.txt");

    println!("part 1: {}", parts::part1(&lines));
    println!("part 2: {}", parts::part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1_RESULT: &str = "";
    const TEST2_RESULT: &str = "";

    #[test]
    fn test1() {
        let lines: Vec<String> = get_input_lines("inputs/test1.txt");
        
        let result: String = parts::part1(&lines);

        assert_eq!(result, TEST1_RESULT);
    }

    #[test]
    fn test2() {
        let lines: Vec<String> = get_input_lines("inputs/test2.txt");
        
        let result: String = parts::part2(&lines);

        assert_eq!(result, TEST2_RESULT);
    }
}
