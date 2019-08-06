use bitvec::prelude::*;
use std::iter;

fn get_bit(byte: u8, index: u8) -> bool {
    assert!(index < 8); // make sure index makes sense
    byte & (0b00000001 << index) != 0
}

fn set_bit(byte: u8, index: u8, value: bool) -> u8 {
    assert!(index < 8); // make sure index makes sense

    if get_bit(byte, index) != value {
        let flip = 0b00000001 << index;
        byte ^ flip
    } else {
        byte
    }
}

/// Determines the value of the cell below `input`, following `rule`  
pub fn test_rule(rule: u8, input: (bool, bool, bool)) -> bool {
    // Convert input to a u8, MSB-first
    let mut input_value: u8 = 0b000;
    input_value = set_bit(input_value, 2, input.0);
    input_value = set_bit(input_value, 1, input.1);
    input_value = set_bit(input_value, 0, input.2);

    // In a Wolfram code, the Nth bit of the base-2 representation of the rule number
    // represents the output cell of the Nth input, enumerated by base-2 addition.
    get_bit(rule, input_value)
}

/// Generates the next layer in the CA with the given `rule` and `input` layer above.
pub fn next_layer(rule: u8, input: &BitVec) -> BitVec {
    let mut out = BitVec::new();
    out.reserve(input.len() + 2); // Reserve the 2 new cells either side.

    // Function to get the input bit at a given location. If the location isn't
    // included in `input`, return false---the empty cell.
    let input_bit = |loc: isize| input.get(loc as usize).unwrap_or(false);

    for i in (-1 as isize)..(input.len() + 1) as isize {
        let input_triple = (input_bit(i - 1), input_bit(i), input_bit(i + 1));
        let cell = test_rule(rule, input_triple);
        out.push(cell)
    }

    out
}

/// Iterates through the layers of the given rule
pub fn iter_layers(rule: u8) -> impl Iterator<Item = BitVec> {
    iter::successors(Some(bitvec![1]), move |last| Some(next_layer(rule, last)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn bit_of_byte() {
        assert_eq!(get_bit(0b00000001, 0), true);
        assert_eq!(get_bit(0b00000000, 0), false);
        assert_eq!(get_bit(0b00010000, 4), true);
        assert_eq!(get_bit(0b11101111, 4), false);
    }

    #[test]
    pub fn rule_30_eval() {
        assert_eq!(test_rule(30, (true, true, true)), false);
        assert_eq!(test_rule(30, (true, true, false)), false);
        assert_eq!(test_rule(30, (true, false, true)), false);
        assert_eq!(test_rule(30, (true, false, false)), true);
        assert_eq!(test_rule(30, (false, true, true)), true);
        assert_eq!(test_rule(30, (false, true, false)), true);
        assert_eq!(test_rule(30, (false, false, true)), true);
        assert_eq!(test_rule(30, (false, false, false)), false);
    }

    #[test]
    pub fn rule_30_layer() {
        // from https://en.wikipedia.org/wiki/Rule_30#Rule_set
        let input = bitvec![1, 1, 0, 0, 1, 0, 0, 0, 1];
        let correct_output = bitvec![1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1];
        assert_eq!(next_layer(30, &input), correct_output);
    }

    #[test]
    pub fn rule_30_iter() {
        let layers = iter_layers(30);
        assert_eq!(
            layers.skip(5).next().unwrap(),
            bitvec![1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]
        )
    }
}
