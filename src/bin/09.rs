advent_of_code::solution!(9);
use std::collections::HashSet;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = Vec::new();
    for (i, c) in input.trim().bytes().enumerate() {
        let num = c - b'0';
        for _ in 0..num {
            blocks.push((i % 2 == 0).then_some(i / 2));
        }
    }

    let mut i = 0;
    let mut j = blocks.len() - 1;
    // dbg!(&blocks);
    while i < j {
        while i < j && blocks[i].is_some() {
            i += 1;
        }
        while j > 0 && blocks[j].is_none() {
            j -= 1;
        }
        if i < j {
            blocks.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    // dbg!(&blocks);
    Some(
        blocks
            .into_iter()
            .enumerate()
            .filter_map(|(i, t)| Some(i * t?))
            .sum(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty(usize),
    Full { size: usize, num: usize },
}

fn print_tiles(tiles: &[Tile]) {
    for t in tiles {
        match t {
            Tile::Empty(size) => print!("{}", ".".repeat(*size)),
            Tile::Full { size, num } => print!("{}", num.to_string().repeat(*size)),
        }
    }
    println!("");
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks = Vec::new();
    for (i, c) in input.trim().bytes().enumerate() {
        let num = c - b'0';
        if i % 2 == 0 {
            blocks.push(Tile::Full {
                size: num as usize,
                num: i / 2,
            })
        } else {
            blocks.push(Tile::Empty(num as usize))
        }
    }

    let mut tried = HashSet::new();
    loop {
        let Some((j, size, num)) = blocks.iter().enumerate().rev().find_map(|(i, t)| match t {
            Tile::Full { size, num } if !tried.contains(num) => Some((i, *size, *num)),
            _ => None,
        }) else {
            break;
        };

        tried.insert(num);

        let Some((first_empty_i, feblock)) = blocks.iter().enumerate().find_map(|(i, t)| {
            let Tile::Empty(esize) = t else {
                return None;
            };

            (*esize >= size).then_some((i, *esize))
        }) else {
            continue;
        };

        // print_tiles(&blocks);
        // println!(
        //     "j{} num{} fei{} feblock{} j{}",
        //     j, num, first_empty_i, feblock, j
        // );

        if first_empty_i > j {
            continue;
        }

        blocks.swap(j, first_empty_i);
        blocks[j] = Tile::Empty(size);
        blocks.insert(first_empty_i + 1, Tile::Empty(feblock - size));
        // print_tiles(&blocks);
    }

    // for (j, to_move) in blocks.iter().cloned().enumerate().rev() {
    //     assert_eq!(to_move, blocks[j]);
    //     let Tile::Full { size, num } = to_move else {
    //         continue;
    //     };
    //
    //     let Some((first_empty_i, feblock)) = blocks.iter().enumerate().find_map(|(i, t)| {
    //         let Tile::Empty(esize) = t else {
    //             return None;
    //         };
    //
    //         (*esize >= size).then_some((i, *esize))
    //     }) else {
    //         continue;
    //     };
    //
    // }

    let mut ans = 0;
    let mut i = 0;
    // dbg!(&blocks);
    for tile in blocks {
        match tile {
            Tile::Empty(size) => i += size,
            Tile::Full { size, num } => {
                for n in 0..size {
                    ans += (i + n) * num;
                }
                i += size;
            }
        }
    }
    Some(ans)
    // Some(
    //     blocks
    //         .into_iter()
    //         .enumerate()
    //         .filter_map(|(i, t)| Some(i * t?))
    //         .sum(),
    // )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
