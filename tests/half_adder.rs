// Intergration Test

use logic_gates::{and, or, xor};

// Aliases purely for readability
pub type Sum = u8;
pub type Carry = u8;
pub type Bit = u8;

pub fn half_adder_input_output() -> Vec<((Bit, Bit), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

/// This function implements a half adder using primitve gates
fn half_adder(a: Bit, b: Bit) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for (inn, out) in half_adder_input_output() {
        let (a, b) = inn;
        println!("Testing: {}, {} -> {:?}", a, b, out);
        assert_eq!(half_adder(a, b), out);
    }
}
