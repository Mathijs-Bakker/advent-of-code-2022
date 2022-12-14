#![warn(clippy::all, clippy::pedantic)]
use std::iter::Peekable;

struct Dir(Vec<Dir>, usize);

pub fn main() {
    let (mut lines, mut sum) = (include_str!("../../input.txt").lines().peekable(), 0);
    get_dir_sizes(&mut lines, &mut sum);
    println!("Solution: {sum}");
}

fn get_dir_sizes(lines: &mut Peekable<impl Iterator<Item = &'static str>>, sum: &mut usize) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);

    while let Some(i) = lines.next() {
        match i {
            "$ cd .." => break,
            _ if i.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum();
            }
            _ => dirs.push(get_dir_sizes(lines, sum)),
        }
    }

    size += dirs.iter().map(|d| d.1).sum::<usize>();

    if size <= 100_000 {
        *sum += size;
    }

    Dir(dirs, size)
}
