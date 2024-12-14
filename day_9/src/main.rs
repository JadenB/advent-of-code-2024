use std::fs;

#[derive(Debug, Clone, Copy)]
enum Contents {
    Free,
    File(usize),
}

#[derive(Debug, Clone, Copy)]
struct Block {
    length: usize,
    contents: Contents,
}

fn compacted(disk: &[Block]) -> Vec<Block> {
    let mut disk = disk.to_vec();

    let mut front: usize = 0;
    let mut back: usize = 0;
    while front < disk.len() && back < disk.len() && front + back < disk.len() - 1 {
        let fi = front;
        let bi = disk.len() - 1 - back;
        match (disk[fi].contents, disk[bi].contents) {
            (Contents::File(_), _) => {
                front += 1;
            }
            (_, Contents::Free) => {
                back += 1;
            }
            (Contents::Free, Contents::File(_)) if disk[fi].length < disk[bi].length => {
                disk[bi].length -= disk[fi].length;
                disk[fi].contents = disk[bi].contents;
            }
            (Contents::Free, Contents::File(_)) if disk[fi].length > disk[bi].length => {
                let file_block = disk.remove(bi);
                disk.insert(
                    fi + 1,
                    Block {
                        length: disk[fi].length - file_block.length,
                        contents: Contents::Free,
                    },
                );
                disk[fi] = file_block;
            }
            (Contents::Free, Contents::File(_)) => {
                disk[fi] = disk[bi];
                disk.remove(bi);
            }
        }
    }

    disk
}

fn compacted_2(disk: &[Block]) -> Vec<Block> {
    let mut disk = disk.to_vec();

    let mut back: usize = 0;
    while back < disk.len() {
        let bi = disk.len() - 1 - back;
        match disk[bi].contents {
            Contents::File(_) => {
                for fi in 0..bi {
                    if matches!(disk[fi].contents, Contents::Free)
                        && disk[fi].length >= disk[bi].length
                    {
                        let remaining = disk[fi].length - disk[bi].length;
                        disk[fi].length = disk[bi].length;
                        disk.swap(fi, bi);
                        if remaining > 0 {
                            disk.insert(
                                fi + 1,
                                Block {
                                    length: remaining,
                                    contents: Contents::Free,
                                },
                            );
                        }
                        break;
                    }
                }
                back += 1;
            }
            Contents::Free => {
                back += 1;
            }
        }
    }

    disk
}

fn compute_checksum(disk: &[Block]) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for block in disk {
        if let Contents::File(id) = block.contents {
            result += (pos..pos + block.length).map(|p| p * id).sum::<usize>();
        }
        pos += block.length;
    }

    result
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();

    let disk: Vec<Block> = input
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let digit = c.to_digit(10).unwrap() as usize;
            if digit == 0 {
                return None;
            }

            let contents = if i % 2 == 0 {
                Contents::File(i / 2)
            } else {
                Contents::Free
            };
            Some(Block {
                length: digit,
                contents,
            })
        })
        .collect();

    let compacted = compacted(&disk);
    let result = compute_checksum(&compacted);
    println!("{result}");

    let compacted = compacted_2(&disk);
    let result = compute_checksum(&compacted);
    println!("{result}");
}
