use miette::*;
use nom::{bytes::complete::take, combinator::map_res, multi::many1, IResult};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, mut disk_map) = parse_disk_map(input).map_err(|_| miette!("Error parsing input"))?;
    disk_map = compact_disk(disk_map);
    let checksum = calculate_checksum(&disk_map);
    Ok(checksum.to_string())
}

#[derive(Debug, Clone, PartialEq)]
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
    let mut compacted = disk_map.clone();
    let mut files: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    let mut current_file_id = None;
    let mut current_file_length = 0;
    let mut current_file_positions = Vec::new();

    // Collect files and their lengths and positions
    for (i, block) in disk_map.iter().enumerate() {
        match block {
            Block::File(id) => {
                if Some(*id) != current_file_id {
                    if let Some(file_id) = current_file_id {
                        files.push((file_id, current_file_length, current_file_positions.clone()));
                    }
                    current_file_id = Some(*id);
                    current_file_length = 1;
                    current_file_positions = vec![i];
                } else {
                    current_file_length += 1;
                    current_file_positions.push(i);
                }
            }
            Block::Free => {
                if let Some(file_id) = current_file_id {
                    files.push((file_id, current_file_length, current_file_positions.clone()));
                    current_file_id = None;
                    current_file_length = 0;
                    current_file_positions.clear();
                }
            }
        }
    }
    if let Some(file_id) = current_file_id {
        files.push((file_id, current_file_length, current_file_positions.clone()));
    }

    // Sort files by inversed position and then by descending ID
    files.sort_by(|a, b| {
        let a_pos = a.2[0];
        let b_pos = b.2[0];
        b_pos.cmp(&a_pos).then_with(|| b.0.cmp(&a.0))
    });

    // Place files in the leftmost free space that is to the left of their current position
    for (file_id, file_length, positions) in files {
        let mut free_span_start = None;
        let mut free_span_length = 0;

        for (i, block) in compacted.iter().enumerate() {
            if i >= positions[0] {
                break; // Stop if we reach the current position of the file
            }

            match block {
                Block::Free => {
                    if free_span_start.is_none() {
                        free_span_start = Some(i);
                    }
                    free_span_length += 1;

                    if free_span_length == file_length {
                        let start = free_span_start.unwrap();
                        for j in 0..file_length {
                            compacted[start + j] = Block::File(file_id);
                        }
                        // Clear the original positions
                        for pos in positions {
                            compacted[pos] = Block::Free;
                        }
                        break;
                    }
                }
                Block::File(_) => {
                    free_span_start = None;
                    free_span_length = 0;
                }
            }
        }
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
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
