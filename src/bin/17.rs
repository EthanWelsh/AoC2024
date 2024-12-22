use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

advent_of_code::solution!(17);

struct Input {
    registers: (u64, u64, u64),
    program: Vec<u64>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, a) = preceded(tag("Register A: "), complete::u64)(input)?;
    let (input, b) = preceded(tag("\nRegister B: "), complete::u64)(input)?;
    let (input, c) = preceded(tag("\nRegister C: "), complete::u64)(input)?;
    let (input, program) = preceded(tag("\n\nProgram: "), separated_list1(tag(","), complete::u64))(input)?;
    Ok((input, Input { registers: (a, b, c), program }))
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, _) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
