use glam::IVec2;
use miette::*;
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

impl Robot {
    fn new(position: IVec2, velocity: IVec2) -> Self {
        Self { position, velocity }
    }

    fn update_position(&mut self, width: i32, height: i32) {
        self.position = IVec2::new(
            (self.position.x + self.velocity.x).rem_euclid(width),
            (self.position.y + self.velocity.y).rem_euclid(height),
        );
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((
            preceded(tag("p="), separated_pair(nom_i32, tag(","), nom_i32)),
            preceded(tag(" v="), separated_pair(nom_i32, tag(","), nom_i32)),
        )),
        |((px, py), (vx, vy))| Robot::new(IVec2::new(px, py), IVec2::new(vx, vy)),
    )(input)
}

fn parse(input: &str) -> Vec<Robot> {
    separated_list1(newline, parse_robot)(input).unwrap().1
}

fn simulate(robots: &[Robot], width: i32, height: i32) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| {
            let mut new_robot = *robot;
            new_robot.update_position(width, height);
            new_robot
        })
        .collect()
}

pub fn process(input: &str) -> Result<String> {
    let width = 101;
    let height = 103;
    let mut robots = parse(input);

    for _ in 0..100 {
        robots = simulate(&robots, width, height);
    }

    let mut quadrants = [0; 4];
    for robot in robots {
        if robot.position.x == width / 2 || robot.position.y == height / 2 {
            continue;
        }
        let quadrant = match (robot.position.x < width / 2, robot.position.y < height / 2) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };
        quadrants[quadrant] += 1;
    }

    let safety_factor = quadrants.iter().product::<i32>();
    Ok(safety_factor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(process(input)?, "21");
        Ok(())
    }
}
