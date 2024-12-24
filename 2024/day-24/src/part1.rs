use miette::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut wires: HashMap<String, u8> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();
    let mut is_parsing_gates = false;

    for line in input.lines() {
        if line.is_empty() {
            is_parsing_gates = true;
            continue;
        }

        if !is_parsing_gates {
            let parts: Vec<&str> = line.split(": ").collect();
            wires.insert(parts[0].to_string(), parts[1].parse::<u8>().unwrap());
        } else {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let gate_parts: Vec<&str> = parts[0].split(' ').collect();
            let output = parts[1].to_string();

            match gate_parts.len() {
                3 => {
                    let gate = match gate_parts[1] {
                        "AND" => {
                            Gate::And(gate_parts[0].to_string(), gate_parts[2].to_string(), output)
                        }
                        "OR" => {
                            Gate::Or(gate_parts[0].to_string(), gate_parts[2].to_string(), output)
                        }
                        "XOR" => {
                            Gate::Xor(gate_parts[0].to_string(), gate_parts[2].to_string(), output)
                        }
                        _ => continue,
                    };
                    gates.push(gate);
                }
                _ => continue,
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for gate in &gates {
            match gate {
                Gate::And(in1, in2, out) => {
                    if let (Some(&v1), Some(&v2)) = (wires.get(in1), wires.get(in2)) {
                        if !wires.contains_key(out) {
                            wires.insert(out.clone(), v1 & v2);
                            changed = true;
                        }
                    }
                }
                Gate::Or(in1, in2, out) => {
                    if let (Some(&v1), Some(&v2)) = (wires.get(in1), wires.get(in2)) {
                        if !wires.contains_key(out) {
                            wires.insert(out.clone(), v1 | v2);
                            changed = true;
                        }
                    }
                }
                Gate::Xor(in1, in2, out) => {
                    if let (Some(&v1), Some(&v2)) = (wires.get(in1), wires.get(in2)) {
                        if !wires.contains_key(out) {
                            wires.insert(out.clone(), v1 ^ v2);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    let mut result = 0u64;
    let mut z_wires: Vec<_> = wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, &v)| (k.clone(), v))
        .collect();
    z_wires.sort_by(|(a, _), (b, _)| b.cmp(a));

    for (_, value) in z_wires {
        result = (result << 1) | (value as u64);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(input)?);
        Ok(())
    }
}
