use itertools::Itertools;
use miette::*;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut formulas = HashMap::new();
    let mut is_parsing_gates = false;

    for line in input.lines() {
        if line.is_empty() {
            is_parsing_gates = true;
            continue;
        }

        if is_parsing_gates {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let gate_parts: Vec<&str> = parts[0].split(' ').collect();
            let output = parts[1].to_string();

            if gate_parts.len() == 3 {
                formulas.insert(
                    output,
                    Formula {
                        op: gate_parts[1].to_string(),
                        x: gate_parts[0].to_string(),
                        y: gate_parts[2].to_string(),
                    },
                );
            }
        }
    }

    let mut swaps = Vec::new();
    for _ in 0..4 {
        let baseline = progress(&formulas);
        let keys: Vec<_> = formulas.keys().cloned().collect();
        'outer: for x in &keys {
            for y in &keys {
                if x == y {
                    continue;
                }
                let x_formula = formulas[x].clone();
                let y_formula = formulas[y].clone();
                formulas.insert(x.clone(), y_formula.clone());
                formulas.insert(y.clone(), x_formula.clone());

                if progress(&formulas) > baseline {
                    swaps.push(x.clone());
                    swaps.push(y.clone());
                    break 'outer;
                }
                formulas.insert(x.clone(), x_formula);
                formulas.insert(y.clone(), y_formula);
            }
        }
    }

    Ok(swaps.iter().sorted().join(","))
}

#[derive(Debug, Clone)]
struct Formula {
    op: String,
    x: String,
    y: String,
}

fn make_wire(char: &str, num: u32) -> String {
    format!("{}{:02}", char, num)
}

fn verify_z(formulas: &HashMap<String, Formula>, wire: &str, num: u32) -> bool {
    if let Some(formula) = formulas.get(wire) {
        if formula.op != "XOR" {
            return false;
        }
        if num == 0 {
            return [&formula.x, &formula.y]
                .iter()
                .sorted()
                .collect::<Vec<_>>()
                == vec![&"x00", &"y00"];
        }
        (verify_intermediate_xor(formulas, &formula.x, num)
            && verify_carry_bit(formulas, &formula.y, num))
            || (verify_intermediate_xor(formulas, &formula.y, num)
                && verify_carry_bit(formulas, &formula.x, num))
    } else {
        false
    }
}

fn verify_intermediate_xor(formulas: &HashMap<String, Formula>, wire: &str, num: u32) -> bool {
    if let Some(formula) = formulas.get(wire) {
        if formula.op != "XOR" {
            return false;
        }
        [formula.x.as_str(), formula.y.as_str()]
            .iter()
            .sorted()
            .collect::<Vec<_>>()
            == vec![&make_wire("x", num).as_str(), &make_wire("y", num).as_str()]
    } else {
        false
    }
}

fn verify_carry_bit(formulas: &HashMap<String, Formula>, wire: &str, num: u32) -> bool {
    if let Some(formula) = formulas.get(wire) {
        if num == 1 {
            if formula.op != "AND" {
                return false;
            }
            return [&formula.x, &formula.y]
                .iter()
                .sorted()
                .collect::<Vec<_>>()
                == vec![&"x00", &"y00"];
        }
        if formula.op != "OR" {
            return false;
        }
        (verify_direct_carry(formulas, &formula.x, num - 1)
            && verify_recarry(formulas, &formula.y, num - 1))
            || (verify_direct_carry(formulas, &formula.y, num - 1)
                && verify_recarry(formulas, &formula.x, num - 1))
    } else {
        false
    }
}

fn verify_direct_carry(formulas: &HashMap<String, Formula>, wire: &str, num: u32) -> bool {
    if let Some(formula) = formulas.get(wire) {
        if formula.op != "AND" {
            return false;
        }
        [&formula.x, &formula.y]
            .iter()
            .sorted()
            .collect::<Vec<_>>()
            == vec![&make_wire("x", num).as_str(), &make_wire("y", num).as_str()]
    } else {
        false
    }
}

fn verify_recarry(formulas: &HashMap<String, Formula>, wire: &str, num: u32) -> bool {
    if let Some(formula) = formulas.get(wire) {
        if formula.op != "AND" {
            return false;
        }
        (verify_intermediate_xor(formulas, &formula.x, num)
            && verify_carry_bit(formulas, &formula.y, num))
            || (verify_intermediate_xor(formulas, &formula.y, num)
                && verify_carry_bit(formulas, &formula.x, num))
    } else {
        false
    }
}

fn progress(formulas: &HashMap<String, Formula>) -> u32 {
    let mut i = 0;
    while verify_z(formulas, &make_wire("z", i), i) {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!("aaa,aoc,bbb,ccc,eee,ooo,z24,z99", process(input)?);
        Ok(())
    }
}
