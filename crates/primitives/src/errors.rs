//! Module for Errors on the evm implementation

#[derive(Debug)]
pub enum EvmErrors {
    StackOverFlow,
    StackTooDeep,
    StackUnderflow
}