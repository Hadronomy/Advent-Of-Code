use miette::*;
use nom::{bytes::complete::take, combinator::map_res, multi::many1, IResult};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, mut disk_map) = parse_disk_map(input).map_err(|_| miette!("Error parsing input"))?;
    disk_map = compact_disk(disk_map);
    let checksum = calculate_checksum(&disk_map);
    Ok(checksum.to_string())
}

#[derive(Debug, Clone)]
enum Block {
    File(usize),
    Free,
}

fn parse_disk_map(input: &str) -> IResult<&str, Vec<Block>> {
    let (input, digits) = many1(map_res(take(1usize), |s: &str| s.parse::<usize>()))(input)?;

    let mut disk_map = Vec::new();
    let mut iter = digits.into_iter();
    let mut file_id = 0;

    while let Some(file_len) = iter.next() {
        let free_len = iter.next().unwrap_or(0);
        for _ in 0..file_len {
            disk_map.push(Block::File(file_id));
        }
        for _ in 0..free_len {
            disk_map.push(Block::Free);
        }
        file_id += 1;
    }

    Ok((input, disk_map))
}

fn compact_disk(disk_map: Vec<Block>) -> Vec<Block> {
    let mut compacted = Vec::new();
    let mut free_space = 0;
    let mut rightmost_files: Vec<&Block> = disk_map
        .iter()
        .filter(|b| matches!(b, Block::File(_)))
        .collect();

    for block in disk_map.clone().iter() {
        if let Block::Free = block {
            free_space += 1;
            if let Some(Block::File(id)) = rightmost_files.pop() {
                compacted.push(Block::File(*id));
            }
            continue;
        }

        if let Some(Block::File(id)) = rightmost_files.first() {
            compacted.push(Block::File(*id));
            rightmost_files = rightmost_files[1..].to_vec();
        }
    }

    // Add remaining free space at the end
    for _ in 0..free_space {
        compacted.push(Block::Free);
    }

    compacted
}

fn calculate_checksum(disk_map: &[Block]) -> usize {
    let mut checksum = 0;
    for (pos, block) in disk_map.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += pos * id;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
