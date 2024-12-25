use miette::*;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let schematics: Vec<Schematic> = input.split("\n\n").map(Schematic::from_str).collect();

    let locks: Vec<&Schematic> = schematics.iter().filter(|s| s.is_lock).collect();
    let keys: Vec<&Schematic> = schematics.iter().filter(|s| !s.is_lock).collect();

    let valid_pairs = locks
        .iter()
        .flat_map(|lock| keys.iter().filter(|key| can_fit(lock, key)))
        .count();

    Ok(valid_pairs.to_string())
}

#[derive(Debug)]
struct Schematic {
    heights: Vec<i32>,
    is_lock: bool,
}

impl Schematic {
    fn from_str(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let is_lock = lines[0].chars().all(|c| c == '#');
        let width = lines[0].len();

        let mut heights = vec![0; width];

        for (col, height) in heights.iter_mut().enumerate() {
            let count = lines
                .iter()
                .filter(|line| line.chars().nth(col) == Some('#'))
                .count() as i32;
            *height = count - 1;
        }

        Self { heights, is_lock }
    }
}

fn can_fit(lock: &Schematic, key: &Schematic) -> bool {
    lock.heights
        .iter()
        .zip(&key.heights)
        .all(|(&l, &k)| l + k <= 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
