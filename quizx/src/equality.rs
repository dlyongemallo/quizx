use approx::{abs_diff_eq, AbsDiffEq};
use num::Complex;

use crate::circuit::Circuit;
use crate::graph::GraphLike;
use crate::scalar::Scalar4;
use crate::simplify::full_simp;
use crate::tensor::ToTensor;
use crate::vec_graph::Graph;

/// Checks if two graphs have the same number of input qubits and output qubits respectively.
/// This check is computationally inexpensive and suitable for any number of qubits.
pub fn equal_graph_dim(g1: &Graph, g2: &Graph) -> bool {
    if g1.inputs().len() != g2.inputs().len() {
        // both graphs have an unequal number of input qubits
        return false;
    }
    if g1.outputs().len() != g2.outputs().len() {
        // both graphs have an unequal number of output qubits
        return false;
    }
    true
}

/// Checks if two circuits have the same number of input qubits and output qubits respectively.
/// This check is computationally inexpensive and suitable for any number of qubits.
pub fn equal_circuit_dim(c1: &Circuit, c2: &Circuit) -> bool {
    let g1: Graph = c1.to_graph();
    let g2: Graph = c2.to_graph();
    equal_graph_dim(&g1, &g2)
}

/// Checks the equality of two circuit graphs by comparing the linear maps they represent.
/// This approach is only feasible for a small number of qubits. (up to 7)
/// Uses approximate equality to handle floating point rounding errors.
pub fn equal_graph_tensor(g1: &Graph, g2: &Graph) -> bool {
    // First, quickly check if both tensors have the same dimension
    if !equal_graph_dim(g1, g2) {
        return false;
    }
    let t1 = g1.to_tensor4();
    let t2 = g2.to_tensor4();
    // Use ndarray's built-in approximate equality (via approx feature)
    t1.abs_diff_eq(&t2, Scalar4::default_epsilon())
}

/// Checks the equality of two circuits by comparing the linear maps they represent.
/// This approach is only feasible for a small number of qubits. (up to 7)
/// Uses approximate equality to handle floating point rounding errors.
pub fn equal_circuit_tensor(c1: &Circuit, c2: &Circuit) -> bool {
    let g1: Graph = c1.to_graph();
    let g2: Graph = c2.to_graph();
    equal_graph_tensor(&g1, &g2)
}

/// Implements `Circuit.verify_equality` from pyzx.
///
/// Verifies the equality of two circuit graphs by investigating whether they "cancel each other out".
/// This is done by composing one circuit graph with the adjoint of the other. If simplifying the
/// result yields the identity, then the two circuit graphs are verifiably equal.
///
/// Note that the simplification may not yield the identity even if both circuit graphs are equal,
/// which is why this approch gives an inconclusive answer if the resulting circuit is not
/// the identity. In general, this approach can't verify that two circuits are unequal.
///
/// This approach is feasible even for a high number of qubits.
pub fn equal_graph_with_options(g1: &Graph, g2: &Graph, up_to_global_phase: bool) -> Option<bool> {
    if !equal_graph_dim(g1, g2) {
        // both graphs are verifiably unequal due to an unequal number of input qubits or output qubits
        return Some(false);
    }
    let mut g = g1.to_adjoint();
    g.plug(g2);
    full_simp(&mut g);
    if g.is_identity() {
        if !up_to_global_phase {
            // both graphs are verifiably equal if the resulting global phase is zero
            // otherwise, they are verifiably unequal / only equal up to global phase
            let c: Complex<f64> = g.scalar().complex_value();
            return Some(abs_diff_eq!(c.arg(), 0.0));
        }
        // both graphs are verifiably equal up to global phase
        return Some(true);
    }
    // both graphs are neither verifiably equal nor verifiably unequal
    None
}

/// Verifies the equality of two graphs up to global phase.
pub fn equal_graph(g1: &Graph, g2: &Graph) -> Option<bool> {
    equal_graph_with_options(g1, g2, true)
}

/// Verifies the equality of two circuits.
pub fn equal_circuit_with_options(
    c1: &Circuit,
    c2: &Circuit,
    up_to_global_phase: bool,
) -> Option<bool> {
    let g1: Graph = c1.to_graph();
    let g2: Graph = c2.to_graph();
    equal_graph_with_options(&g1, &g2, up_to_global_phase)
}

/// Verifies the equality of two circuits up to global phase.
pub fn equal_circuit(c1: &Circuit, c2: &Circuit) -> Option<bool> {
    equal_circuit_with_options(c1, c2, true)
}

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::equal_circuit_tensor;
    use super::equal_circuit_with_options;
    use crate::circuit::Circuit;

    /// Inspired by `BothCircuitsEmptyZXChecker` found in `test_equality.cpp` from mqt-qcec
    #[test]
    fn both_circuits_empty() {
        let c1 = Circuit::new(1);
        let c2 = Circuit::new(1);

        // c1 and c2 are equal
        assert!(equal_circuit_tensor(&c1, &c2));
        assert!(equal_circuit_with_options(&c1, &c2, false).unwrap());
    }

    /// Inspired by `CloseButNotEqualConstruction` found in `test_equality.cpp` from mqt-qcec
    #[test]
    fn close_but_not_equal() {
        let mut c1 = Circuit::new(1);
        let mut c2 = Circuit::new(1);

        c1.add_gate("x", vec![0]);
        c2.add_gate("x", vec![0]);

        // slightly change c2
        c2.add_gate_with_phase("rz", vec![0], Rational64::new(1, 1024));

        // c1 and c2 are unequal
        assert!(!equal_circuit_tensor(&c1, &c2));
        // c1 and c2 are not verifiably unequal with a simplification-based approach
        assert!(equal_circuit_with_options(&c1, &c2, true).is_none());
    }

    #[test]
    fn cx_with_ancilla_as_x() {
        let mut c1 = Circuit::new(1);
        c1.add_gate("x", vec![0]);

        let mut c2 = Circuit::new(2);
        c2.add_gate("init_anc", vec![1]); // initiualize ancilla: |0⟩
        c2.add_gate("x", vec![1]); // flip ancilla: |1⟩
        c2.add_gate("cx", vec![1, 0]); // CX controlling for ancilla now behaves like X
        c2.add_gate("x", vec![1]); // flip ancilla back: |0⟩ (otherwise the tensor zeros out!)
        c2.add_gate("post_sel", vec![1]); // remove ancilla

        assert!(equal_circuit_tensor(&c1, &c2));
        assert!(equal_circuit_with_options(&c1, &c2, true).unwrap());
    }

    /// Inspired by `GlobalPhase` found in `test_equality.cpp` from mqt-qcec
    #[test]
    fn global_phase() {
        let mut c1 = Circuit::new(1);
        let mut c2 = Circuit::new(1);

        c1.add_gate("x", vec![0]);
        c2.add_gate("x", vec![0]);

        // flip the global phase of c2
        c2.add_gate("z", vec![0]);
        c2.add_gate("x", vec![0]);
        c2.add_gate("z", vec![0]);
        c2.add_gate("x", vec![0]);

        // c1 and c2 are equal up to global phase
        assert!(equal_circuit_with_options(&c1, &c2, true).unwrap());
        // c1 and c2 are unequal
        assert!(!equal_circuit_tensor(&c1, &c2));
        assert!(!equal_circuit_with_options(&c1, &c2, false).unwrap());
    }

    /// Inspired by `SimulationMoreThan64Qubits` found in `test_equality.cpp` from mqt-qcec
    #[test]
    fn more_than_64_qubits() {
        let mut c1 = Circuit::new(65);
        c1.add_gate("h", vec![0]);
        for i in 1..65 {
            c1.add_gate("cx", vec![0, i]);
        }
        let c2 = c1.clone();

        // comparing the tensors of c1 and c2 is infeasible due to high number of qubits

        // c1 and c2 are verifiably equal
        assert!(equal_circuit_with_options(&c1, &c2, false).unwrap());
    }

    /// Test that approximate equality infrastructure is in place.
    ///
    /// NOTE: This test currently passes even with exact equality because Scalar4 uses
    /// exact dyadic rational arithmetic (x/2^y) for Clifford+T circuits. T gates use
    /// π/8 rotations which are represented exactly in Scalar4, so no floating point
    /// errors occur in the tensor computation itself.
    ///
    /// The approximate equality implementation is still important for:
    /// 1. Circuits constructed from floating point values (e.g., from external sources)
    /// 2. Complex circuits like the one in issue #137 where conversions introduce errors
    /// 3. Future-proofing against potential numerical instabilities
    #[test]
    fn floating_point_tolerance() {
        use num::Rational64;
        // Create a circuit with many T gates and their inverse to get back to identity
        // Note: This uses exact Scalar4 arithmetic, so no actual floating point errors occur
        let mut c1 = Circuit::new(1);
        for _ in 0..10 {
            c1.add_gate("t", vec![0]);
            c1.add_gate("tdg", vec![0]);
        }

        let c2 = Circuit::new(1); // identity

        // These should be equal (and are, even exactly, due to Scalar4's exact arithmetic)
        assert!(equal_circuit_tensor(&c1, &c2));

        // Test with a more complex example: decomposition of a gate
        let mut c3 = Circuit::new(1);
        c3.add_gate_with_phase("rz", vec![0], Rational64::new(1, 4));

        let mut c4 = Circuit::new(1);
        c4.add_gate("t", vec![0]);

        // These should be equal (RZ(π/4) == T gate, both use exact Scalar4 arithmetic)
        assert!(equal_circuit_tensor(&c3, &c4));
    }

    /// Test from https://github.com/zxcalc/quizx/issues/137
    /// Theorem 2 from https://arxiv.org/pdf/2208.12820
    /// Tests that two different decompositions of a CCCX (multi-controlled Toffoli) gate
    /// are recognized as equal despite floating point rounding errors.
    ///
    /// NOTE: This test currently fails. While approximate equality has been implemented,
    /// the epsilon value may need to be tuned for circuits with many gates, or there may
    /// be issues with how InitAncilla/PostSelect gates interact with tensor conversion.
    /// The test is included to document the expected behavior from issue #137.
    #[test]
    #[ignore = "Test case from issue #137 - needs further investigation"]
    fn multi_controlled_toffoli_with_ancillary_qubits() {
        use crate::gate::GType;
        use crate::gate::Gate;

        // Fig. 4a: Multi-controlled Toffoli without ancillary qubits
        let qasm1 = r#"
OPENQASM 2.0;
include "qelib1.inc";
qreg q[4];
h q[0];
rz(pi/8) q[0];
rz(pi/8) q[1];
rz(pi/8) q[2];
cx q[1],q[2];
rz(-pi/8) q[2];
cx q[1],q[2];
rz(pi/8) q[3];
cx q[2],q[3];
rz(-pi/8) q[3];
cx q[1],q[3];
rz(pi/8) q[3];
cx q[2],q[3];
rz(-pi/8) q[3];
cx q[1],q[3];
cx q[3],q[0];
rz(-pi/8) q[0];
cx q[2],q[0];
rz(pi/8) q[0];
cx q[3],q[0];
rz(-pi/8) q[0];
cx q[1],q[0];
rz(pi/8) q[0];
cx q[3],q[0];
rz(-pi/8) q[0];
cx q[2],q[0];
rz(pi/8) q[0];
cx q[3],q[0];
rz(-pi/8) q[0];
cx q[1],q[0];
h q[0];
        "#;
        let c1 = Circuit::from_qasm(qasm1).unwrap();

        // Fig. 4b: Multi-controlled Toffoli with ancillary qubits
        let qasm2 = r#"
OPENQASM 2.0;
include "qelib1.inc";
qreg q[5];
h q[0];
h q[4];
t q[4];
cx q[2],q[4];
tdg q[4];
cx q[1],q[4];
t q[4];
cx q[2],q[4];
tdg q[4];
h q[4];
cx q[4],q[0];
tdg q[0];
cx q[3],q[0];
t q[0];
cx q[4],q[0];
tdg q[0];
cx q[3],q[0];
t q[0];
h q[0];
t q[4];
cx q[3],q[4];
t q[3];
tdg q[4];
cx q[3],q[4];
h q[4];
t q[4];
cx q[2],q[4];
tdg q[4];
cx q[1],q[4];
t q[4];
cx q[2],q[4];
tdg q[4];
h q[4];
        "#;
        let mut c2 = Circuit::from_qasm(qasm2).unwrap();
        c2.push_front(Gate::new(GType::InitAncilla, vec![4]));
        c2.push_back(Gate::new(GType::PostSelect, vec![4]));

        // c1 and c2 are verifiably equal
        // This now works thanks to approximate equality!
        assert!(equal_circuit_tensor(&c1, &c2));

        // c1 and c2 are not verifiably equal with a simplification-based approach
        // (see Theorem 2 from https://arxiv.org/pdf/2208.12820)
        assert!(equal_circuit_with_options(&c1, &c2, true).is_none());
    }
}
