use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> IResult<&str, Vec<usize>> {
    many1(map_res(one_of("0123456789"), |c: char| {
        c.to_string().parse::<usize>()
    }))(input)
}

fn build_disk(input: &str) -> Vec<Option<usize>> {
    let nums = parse_input(input).unwrap().1;
    nums.iter()
        .enumerate()
        .flat_map(|(i, &c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c]
            } else {
                vec![None; c]
            }
        })
        .collect()
}

fn compact_disk(disk: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        while left < right && disk[left].is_some() {
            left += 1;
        }
        while left < right && disk[right].is_none() {
            right -= 1;
        }
        disk.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn calculate_checksum(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|x| x * i))
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = build_disk(input);
    compact_disk(&mut disk);
    Some(calculate_checksum(&disk))
}

fn find_next_free_block(disk: &[Option<usize>], file_size: usize) -> Option<usize> {
    let mut i = 0; // Start at 0
    while i + file_size <= disk.len() {
        if disk[i..i + file_size].iter().all(|x| x.is_none()) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn move_file(disk: &mut [Option<usize>], source_start: usize, dest_start: usize, file_size: usize) {
    for i in 0..file_size {
        disk[dest_start + i] = disk[source_start + i];
    }
    for i in source_start..source_start + file_size {
        disk[i] = None;
    }
}

fn get_file_ids(disk: &Vec<Option<usize>>) -> Vec<usize> {
    let mut file_ids: Vec<usize> = disk
        .iter()
        .filter_map(|x| *x)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    file_ids.sort_by(|a, b| b.cmp(a));
    file_ids
}

fn move_file_to_leftmost_space(disk: &mut Vec<Option<usize>>, file_id: usize) {
    let mut read_index = 0;
    while read_index < disk.len() {
        if disk[read_index] == Some(file_id) {
            let file_size = disk[read_index..]
                .iter()
                .take_while(|&x| x == &Some(file_id))
                .count();

            if let Some(free_block_start) = find_next_free_block(disk, file_size) {
                if free_block_start < read_index {
                    // Only move if the free block is to the left
                    move_file(disk, read_index, free_block_start, file_size);
                }
                break; // Move to the next file_id regardless of whether it was moved
            } else {
                read_index += file_size;
            }
        } else {
            read_index += 1;
        }
    }
}

fn compact_disk_part2(disk: &mut Vec<Option<usize>>) {
    let file_ids = get_file_ids(disk);

    for file_id in file_ids {
        move_file_to_leftmost_space(disk, file_id);
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk = build_disk(input);
    compact_disk_part2(&mut disk);
    Some(calculate_checksum(&disk))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("123").unwrap(), ("", vec![1, 2, 3]));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2333133121414131402"), Some(1928));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2333133121414131402"), Some(2858));
    }
}
