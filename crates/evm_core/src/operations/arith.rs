use alloy::primitives::{I256, U256};

use crate::{Evm, ProgramExitStatus};

pub fn stop(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Success;
}

pub fn add(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b).unwrap();
}

pub fn mul(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a * b).unwrap();
}

pub fn sub(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a - b).unwrap();
}

pub fn div(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a / b).unwrap();
    }
}

pub fn sdiv(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());

    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int / b_int;
        let result_unsigned = U256::from_limbs(*result.as_limbs());
        evm.stack.push(result_unsigned).unwrap();
    }
}

pub fn modulo(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

pub fn smod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());

    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int % b_int;
        let result_unsigned = U256::from_limbs(*result.as_limbs());
        evm.stack.push(result_unsigned).unwrap();
    }
}

pub fn addmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();

    let result = a + b ;

    if result == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(result % n).unwrap();
    }
}

pub fn mulmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();

    let result = a * b ;

    if result == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(result % n).unwrap();
    }
}