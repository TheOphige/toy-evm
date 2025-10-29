//! This is an impl of the Stack DS
//! - Push and
//! - Pop

use alloy::primitives::B256;
use crate::{constants::MAX_STACK_DEPTH, errors::EvmErrors};

#[derive(Debug, Default, Clone)]
pub struct Stack {
    data : Vec<B256>
}

impl Stack {
    /// Function introduces a new item to the top of the stack
    pub fn push(&mut self, item: B256) -> Result<(), EvmErrors> {
        if self.data.len() >= MAX_STACK_DEPTH as usize {
            return Err(EvmErrors::StackOverFlow);
        }
        self.data.push(item);
        Ok(())
    }

    /// Function remove the last item from the stack
    pub fn pop(&mut self) -> Option<B256> {
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_b256() -> B256 {
        B256::from([0x42; 32])
    }

    #[test]
    fn test_new_stack_is_empty() {
        let stack = Stack::default();
        assert_eq!(stack.data.len(), 0);
    }

    #[test]
    fn test_push_success() {
        let mut stack = Stack::default();
        let value = create_test_b256();
        
        assert!(stack.push(value).is_ok());
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], value);
    }

    #[test]
    fn test_pop_from_non_empty_stack() {
        let mut stack = Stack::default();
        let value = create_test_b256();
        
        stack.push(value).unwrap();
        let popped = stack.pop();
        
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), value);
        assert_eq!(stack.data.len(), 0);
    }

    #[test]
    fn test_pop_from_empty_stack() {
        let mut stack = Stack::default();
        let popped = stack.pop();
        
        assert!(popped.is_none());
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::default();
        let value = create_test_b256();

        // Fill the stack to its maximum capacity
        for _ in 0..MAX_STACK_DEPTH {
            assert!(stack.push(value).is_ok());
        }

        // Attempt to push one more item should fail
        let result = stack.push(value);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvmErrors::StackOverFlow));
    }

    #[test]
    fn test_multiple_push_pop_operations() {
        let mut stack = Stack::default();
        let value1 = B256::from([0x11; 32]);
        let value2 = B256::from([0x22; 32]);
        let value3 = B256::from([0x33; 32]);

        // Push three values
        stack.push(value1).unwrap();
        stack.push(value2).unwrap();
        stack.push(value3).unwrap();

        // Pop them and verify LIFO order
        assert_eq!(stack.pop().unwrap(), value3);
        assert_eq!(stack.pop().unwrap(), value2);
        assert_eq!(stack.pop().unwrap(), value1);
        assert!(stack.pop().is_none());
    }
}