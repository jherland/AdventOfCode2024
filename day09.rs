use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::iter;

fn part1_expand(files: &Vec<usize>, frees: &Vec<usize>) -> Vec<usize> {
    assert_eq!(files.len(), frees.len());
    let mut result: Vec<usize> = Vec::with_capacity(files.iter().sum());
    let mut blocks: VecDeque<usize> = files
        .iter()
        .enumerate()
        .map(|(id, count)| iter::repeat_n(id, *count))
        .flatten()
        .collect();
    for (file, free) in iter::zip(files, frees) {
        for _ in 0..*file {
            match blocks.pop_front() {
                None => break,
                Some(id) => result.push(id),
            }
        }
        for _ in 0..*free {
            match blocks.pop_back() {
                None => break,
                Some(id) => result.push(id),
            }
        }
    }
    result
}

#[derive(Clone, Debug)]
struct FreeBlock {
    size: usize,
}

impl FreeBlock {
    fn from_vec(sizes: &Vec<usize>) -> Vec<Self> {
        sizes.iter().map(|size| Self { size: *size }).collect()
    }
}

#[derive(Clone, Debug)]
struct FileBlock {
    size: usize,
    id: usize,
    moved: bool,
}

impl FileBlock {
    fn from_vec(sizes: &Vec<usize>) -> Vec<Self> {
        sizes.iter().enumerate().map(|(id, size)| Self { size: *size, id, moved: false }).collect()
    }

    fn fits_in(&self, free: &FreeBlock) -> bool {
        self.size <= free.size
    }

    fn split(&self, free: &FreeBlock) -> (FileBlock, FreeBlock) {
        assert!(self.size <= free.size);
        (FileBlock { moved: true, ..self.clone() }, FreeBlock { size: free.size - self.size })
    }

    fn find_insert_point(&self, frees: &Vec<FreeBlock>) -> Option<usize> {
        for (i, free) in frees.iter().enumerate() {
            if self.fits_in(free) {
                return Some(i);
            }
        }
        None
    }
}

fn part2_expand(files: &Vec<usize>, frees: &Vec<usize>) -> Vec<usize> {
    assert_eq!(files.len(), frees.len());
    let mut fileblocks = FileBlock::from_vec(files);
    let mut freeblocks = FreeBlock::from_vec(frees);
    let mut extract_i = fileblocks.len() - 1;
    loop {
        while extract_i > 0 && fileblocks[extract_i].moved {
            extract_i -= 1;
        }
        if extract_i == 0 {
            break;
        }
        let insert_i = match fileblocks[extract_i].find_insert_point(&freeblocks) {
            Some(i) if i < extract_i => i,
            _ => { // Cannot move file
                extract_i -= 1;
                continue;
            }
        };
        // Move fileblock from file_i to ins_i
        let moving = fileblocks.remove(extract_i);
        let free_after = freeblocks.remove(extract_i);
        assert!(insert_i + 1 < freeblocks.len());
        let (moved, shrunk_free) = moving.split(&freeblocks[insert_i]);
        fileblocks.insert(insert_i + 1, moved);
        // Adjust freeblocks around insertion point: 0 before, free.size - file.size after
        freeblocks[insert_i] = FreeBlock { size: 0 };
        freeblocks.insert(insert_i + 1, shrunk_free);
        // Collapse freeblocks around extraction point
        freeblocks[extract_i].size += moving.size + free_after.size;
    }
    iter::zip(fileblocks, freeblocks)
        .map(|(fileblock, freeblock)|
            iter::repeat_n(fileblock.id, fileblock.size)
                .chain(iter::repeat_n(0, freeblock.size))
        )
        .flatten()
        .collect()
}

fn checksum(blocks: Vec<usize>) -> usize {
    blocks.iter().enumerate().map(|(i, id)| i * id).sum::<usize>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let (files, frees): (Vec<usize>, Vec<usize>) = nums
        .chunks(2)
        .map(|arr| match arr {
            [a, b] => (*a, *b),
            [a] => (*a, 0),
            _ => panic!(),
        })
        .unzip();

    println!("Part 1: {}", checksum(part1_expand(&files, &frees)));
    println!("Part 2: {}", checksum(part2_expand(&files, &frees)));
}
