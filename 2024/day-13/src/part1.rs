use miette::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map_res,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct ButtonConfig {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Prize {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: ButtonConfig,
    button_b: ButtonConfig,
    prize: Prize,
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_button_config(input: &str) -> IResult<&str, ButtonConfig> {
    let (input, (_, x, _, y)) = tuple((tag("X+"), parse_i32, tag(", Y+"), parse_i32))(input)?;
    Ok((input, ButtonConfig { x, y }))
}

fn parse_prize(input: &str) -> IResult<&str, Prize> {
    let (input, (_, x, _, y)) =
        tuple((tag("Prize: X="), parse_i32, tag(", Y="), parse_i32))(input)?;
    Ok((input, Prize { x, y }))
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, (button_a, _, button_b, _, prize)) = tuple((
        preceded(tag("Button A: "), parse_button_config),
        multispace1,
        preceded(tag("Button B: "), parse_button_config),
        multispace1,
        parse_prize,
    ))(input)?;
    Ok((
        input,
        ClawMachine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    let mut machines = Vec::new();
    let mut remaining_input = input;
    while !remaining_input.is_empty() {
        let (input, machine) = parse_claw_machine(remaining_input)?;
        machines.push(machine);
        remaining_input = input.trim_start();
    }
    Ok((remaining_input, machines))
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (_, machines) =
        parse_input(input).map_err(|e| miette!("Failed to parse input: {:?}", e))?;

    let mut total_tokens = 0;
    for machine in machines {
        if let Some(tokens) = calculate_min_tokens(&machine) {
            total_tokens += tokens;
        }
    }

    Ok(total_tokens.to_string())
}

fn calculate_min_tokens(machine: &ClawMachine) -> Option<i32> {
    let ClawMachine {
        button_a,
        button_b,
        prize,
    } = machine;
    let (ax, ay) = (button_a.x, button_a.y);
    let (bx, by) = (button_b.x, button_b.y);
    let (px, py) = (prize.x, prize.y);

    for a in 0..=100 {
        for b in 0..=100 {
            if a * ax + b * bx == px && a * ay + b * by == py {
                return Some(a * 3 + b);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
