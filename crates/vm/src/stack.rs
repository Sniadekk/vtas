use crate::{runtime_error::RuntimeErrorCause, MachineResult, RuntimeValue, VM};

impl VM {
    pub(crate) fn pop_operand(&mut self) -> MachineResult<RuntimeValue> {
        match self.operands.pop() {
            Some(value) => Ok(value),
            None => self.error(RuntimeErrorCause::PoppedFromEmptyStack),
        }
    }

    pub(crate) fn pop_two_operands(&mut self) -> MachineResult<(RuntimeValue, RuntimeValue)> {
        let a = self.pop_operand()?;
        let b = self.pop_operand()?;
        Ok((a, b))
    }
}

#[cfg(test)]
mod test {

    use bytecode::chunk::Chunk;
    use lasso::Spur;

    use crate::{runtime_value::RuntimeValue, test::new_vm};

    #[test]
    fn pop_operand() {
        let mut vm = new_vm(Chunk::default());
        vm.operands = vec![
            RuntimeValue::Number(10.0),
            RuntimeValue::String(Spur::default()),
            RuntimeValue::Bool(false),
            RuntimeValue::Bool(true),
        ];
        assert!(vm
            .pop_operand()
            .unwrap()
            .eq(RuntimeValue::Bool(true), &mut vm)
            .unwrap());

        assert!(vm
            .pop_operand()
            .unwrap()
            .eq(RuntimeValue::Bool(false), &mut vm)
            .unwrap());

        assert!(vm
            .pop_operand()
            .unwrap()
            .eq(RuntimeValue::String(Spur::default()), &mut vm)
            .unwrap());

        assert!(vm
            .pop_operand()
            .unwrap()
            .eq(RuntimeValue::Number(10.0), &mut vm)
            .unwrap());
    }
}
