//! An Arithmetic Machine.
//!
//! This module contains an example of a basic calculator and tests which
//! verify it's output.

use super::super::*;

/// Pushes an piece of data from the data section onto the operand stack.
fn push(machine: &mut Machine<f64>, args: &[usize]) {
    let arg = machine.builder.data[args[0]];
    machine.operand_push(arg)
}

/// Pops two operands off the top of the stack, adds them together and
/// pushes the result back onto the stack.
fn add(machine: &mut Machine<f64>, _args: &[usize]) {
    let rhs = machine.operand_pop();
    let lhs = machine.operand_pop();
    machine.operand_push(lhs + rhs);
}

/// Pops two operands off the top of the stack, subtracts the first from the
/// second and pushes the result back onto the stack.
fn sub(machine: &mut Machine<f64>, _args: &[usize]) {
    let rhs = machine.operand_pop();
    let lhs = machine.operand_pop();
    machine.operand_push(lhs - rhs);
}

/// Pops two operands off the top of the stack, divides the first from the
/// second and pushes the result back onto the stack.
fn div(machine: &mut Machine<f64>, _args: &[usize]) {
    let rhs = machine.operand_pop();
    let lhs = machine.operand_pop();
    machine.operand_push(lhs / rhs);
}

/// Pops two operands off the top of the stack, multiples them together and
/// pushes the result back onto the stack.
fn mult(machine: &mut Machine<f64>, _args: &[usize]) {
    let rhs = machine.operand_pop();
    let lhs = machine.operand_pop();
    machine.operand_push(lhs * rhs);
}

/// Build an instruction table based on the instructions outlined above.
fn instruction_table() -> InstructionTable<f64> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add",  0, add));
    it.insert(Instruction::new(2, "sub",  0, sub));
    it.insert(Instruction::new(3, "div",  0, div));
    it.insert(Instruction::new(4, "mult", 0, mult));
    it
}

#[test]
fn addition_example() {
    let it = instruction_table();
    let mut builder: Builder<f64> = Builder::new(&it);
    builder.push(0, vec![2.0]);
    builder.push(0, vec![3.0]);
    builder.push(1, vec![]);
    builder.push(0, vec![4.0]);
    builder.push(1, vec![]);
    let constants: WriteManyTable<f64> = WriteManyTable::new();
    let machine = Machine::new(builder, &constants);
    let mut machine = Machine::run(machine);
    let result = machine.operand_pop();
    assert_eq!(result, 9.0);
}

#[test]
fn subtraction_example() {
    let it = instruction_table();
    let mut builder: Builder<f64> = Builder::new(&it);
    builder.push(0, vec![3.0]);
    builder.push(0, vec![4.0]);
    builder.push(1, vec![]);
    builder.push(0, vec![2.0]);
    builder.push(2, vec![]);
    let constants: WriteManyTable<f64> = WriteManyTable::new();
    let machine = Machine::new(builder, &constants);
    let mut machine = Machine::run(machine);
    let result = machine.operand_pop();
    assert_eq!(result, 5.0);
}

#[test]
fn division_example() {
    let it = instruction_table();
    let mut builder: Builder<f64> = Builder::new(&it);
    builder.push(0, vec![3.0]);
    builder.push(0, vec![4.0]);
    builder.push(3, vec![]);
    let constants: WriteManyTable<f64> = WriteManyTable::new();
    let machine = Machine::new(builder, &constants);
    let mut machine = Machine::run(machine);
    let result = machine.operand_pop();
    assert_eq!(result, 0.75);
}

#[test]
fn multiplication_example() {
    let it = instruction_table();
    let mut builder: Builder<f64> = Builder::new(&it);
    builder.push(0, vec![3.0]);
    builder.push(0, vec![4.0]);
    builder.push(4, vec![]);
    let constants: WriteManyTable<f64> = WriteManyTable::new();
    let machine = Machine::new(builder, &constants);
    let mut machine = Machine::run(machine);
    let result = machine.operand_pop();
    assert_eq!(result, 12.0);
}
