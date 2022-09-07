// This is free and unencumbered software released into the public domain.

#[cfg(test)]
mod tests {
    use crate::ops::*;
    use crate::state::*;
    use ux::*;

    #[test]
    fn test_stop() {}

    #[test]
    fn test_add() {
        unsafe {
            EVM.reset();
            push1(6);
            push1(7);
            add();
            assert_eq!(EVM.stack.peek(), 13);
        }
    }

    #[test]
    fn test_mul() {
        unsafe {
            EVM.reset();
            push1(6);
            push1(7);
            mul();
            assert_eq!(EVM.stack.peek(), 42);
        }
    }

    #[test]
    fn test_sub() {
        unsafe {
            EVM.reset();
            push1(6);
            push1(7);
            sub();
            assert_eq!(EVM.stack.peek(), 1);
        }
    }

    #[test]
    fn test_div() {
        unsafe {
            EVM.reset();
            push1(6);
            push1(42);
            div();
            assert_eq!(EVM.stack.peek(), 7);
        }
    }

    #[test]
    fn test_sdiv() {}

    #[test]
    fn test_mod() {}

    #[test]
    fn test_smod() {}

    #[test]
    fn test_addmod() {}

    #[test]
    fn test_mulmod() {}

    #[test]
    fn test_exp() {
        unsafe {
            EVM.reset();
            push1(4);
            push1(2);
            exp();
            assert_eq!(EVM.stack.peek(), 16);
        }
    }

    #[test]
    fn test_signextend() {}

    #[test]
    fn test_lt() {}

    #[test]
    fn test_gt() {}

    #[test]
    fn test_slt() {}

    #[test]
    fn test_sgt() {}

    #[test]
    fn test_eq() {}

    #[test]
    fn test_iszero() {}

    #[test]
    fn test_and() {}

    #[test]
    fn test_or() {}

    #[test]
    fn test_xor() {}

    #[test]
    fn test_not() {}

    #[test]
    fn test_byte() {}

    #[test]
    fn test_shl() {}

    #[test]
    fn test_shr() {}

    #[test]
    fn test_sar() {}

    #[test]
    fn test_sha3() {}

    #[test]
    fn test_address() {}

    #[test]
    fn test_balance() {}

    #[test]
    fn test_origin() {}

    #[test]
    fn test_caller() {}

    #[test]
    fn test_callvalue() {}

    #[test]
    fn test_calldataload() {}

    #[test]
    fn test_calldatasize() {}

    #[test]
    fn test_calldatacopy() {}

    #[test]
    fn test_codesize() {}

    #[test]
    fn test_codecopy() {}

    #[test]
    fn test_gasprice() {}

    #[test]
    fn test_extcodesize() {}

    #[test]
    fn test_extcodecopy() {}

    #[test]
    fn test_returndatasize() {}

    #[test]
    fn test_returndatacopy() {}

    #[test]
    fn test_extcodehash() {}

    #[test]
    fn test_blockhash() {}

    #[test]
    fn test_coinbase() {}

    #[test]
    fn test_timestamp() {}

    #[test]
    fn test_number() {}

    #[test]
    fn test_difficulty() {}

    #[test]
    fn test_gaslimit() {}

    #[test]
    fn test_chainid() {}

    #[test]
    fn test_selfbalance() {}

    #[test]
    fn test_basefee() {}

    #[test]
    fn test_pop() {
        unsafe {
            EVM.reset();
            push1(42);
            assert_eq!(EVM.stack.depth, 1);
            pop();
            assert_eq!(EVM.stack.depth, 0);
        }
    }

    #[test]
    fn test_mload() {
        unsafe {
            EVM.reset();
            push1(0);
            mload();
            assert_eq!(EVM.stack.peek(), 0);
        }
    }

    #[test]
    fn test_mstore() {
        unsafe {
            EVM.reset();
            push1(42);
            push1(0);
            mstore();
            assert_eq!(EVM.memory.load_word(0), 42);
        }
    }

    #[test]
    fn test_mstore8() {
        unsafe {
            EVM.reset();
            push1(42);
            push1(0);
            mstore8();
            assert_eq!(EVM.memory.load_word(0), 42);
        }
    }

    #[test]
    fn test_sload() {
        unsafe {
            EVM.reset();
            EVM.storage.store_word(Word::from(42u8), Word::from(123u8));
            push1(42);
            sload();
            assert_eq!(EVM.stack.peek(), 123);
        }
    }

    #[test]
    fn test_sstore() {
        unsafe {
            EVM.reset();
            push1(6);
            push1(7);
            sstore();
            assert_eq!(EVM.stack.depth, 0);
            assert_eq!(EVM.storage.load_word(Word::from(7u8)), 6);
        }
    }

    #[test]
    fn test_jump() {}

    #[test]
    fn test_jumpi() {}

    #[test]
    fn test_pc() {}

    #[test]
    fn test_msize() {}

    #[test]
    fn test_gas() {}

    #[test]
    fn test_jumpdest() {}

    #[test]
    fn test_push1() {
        unsafe {
            EVM.reset();
            push1(0x12u8);
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x12u8));
        }
    }

    #[test]
    fn test_push2() {
        unsafe {
            EVM.reset();
            push2(0x1234u16);
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x1234u16));
        }
    }

    #[test]
    fn test_push3() {
        unsafe {
            EVM.reset();
            push3(u24::try_from(0x123456u32).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456u32));
        }
    }

    #[test]
    fn test_push4() {
        unsafe {
            EVM.reset();
            push4(u32::try_from(0x12345678u32).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x12345678u32));
        }
    }

    #[test]
    fn test_push5() {
        unsafe {
            EVM.reset();
            push5(u40::try_from(0x123456789Au64).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789Au64));
        }
    }

    #[test]
    fn test_push6() {
        unsafe {
            EVM.reset();
            push6(u48::try_from(0x123456789ABCu64).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCu64));
        }
    }

    #[test]
    fn test_push7() {
        unsafe {
            EVM.reset();
            push7(u56::try_from(0x123456789ABCDEu64).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCDEu64));
        }
    }

    #[test]
    fn test_push8() {
        unsafe {
            EVM.reset();
            push8(u64::try_from(0x123456789ABCDEF0u64).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCDEF0u64));
        }
    }

    #[test]
    fn test_push9() {
        unsafe {
            EVM.reset();
            push9(u128::try_from(0x123456789ABCDEF011u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCDEF011u128));
        }
    }

    #[test]
    fn test_push10() {
        unsafe {
            EVM.reset();
            push10(u128::try_from(0x123456789ABCDEF01122u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCDEF01122u128));
        }
    }

    #[test]
    fn test_push11() {
        unsafe {
            EVM.reset();
            push11(u128::try_from(0x123456789ABCDEF0112233u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], Word::from(0x123456789ABCDEF0112233u128));
        }
    }

    #[test]
    fn test_push12() {
        unsafe {
            EVM.reset();
            push12(u128::try_from(0x123456789ABCDEF011223344u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from(0x123456789ABCDEF011223344u128)
            );
        }
    }

    #[test]
    fn test_push13() {
        unsafe {
            EVM.reset();
            push13(u128::try_from(0x123456789ABCDEF01122334455u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from(0x123456789ABCDEF01122334455u128)
            );
        }
    }

    #[test]
    fn test_push14() {
        unsafe {
            EVM.reset();
            push14(u128::try_from(0x123456789ABCDEF0112233445566u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from(0x123456789ABCDEF0112233445566u128)
            );
        }
    }

    #[test]
    fn test_push15() {
        unsafe {
            EVM.reset();
            push15(u128::try_from(0x123456789ABCDEF011223344556677u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from(0x123456789ABCDEF011223344556677u128)
            );
        }
    }

    #[test]
    fn test_push16() {
        unsafe {
            EVM.reset();
            push16(u128::try_from(0x123456789ABCDEF01122334455667788u128).unwrap());
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from(0x123456789ABCDEF01122334455667788u128)
            );
        }
    }

    #[test]
    fn test_push17() {}

    #[test]
    fn test_push18() {}

    #[test]
    fn test_push19() {}

    #[test]
    fn test_push20() {}

    #[test]
    fn test_push21() {}

    #[test]
    fn test_push22() {}

    #[test]
    fn test_push23() {}

    #[test]
    fn test_push24() {
        unsafe {
            EVM.reset();
            push24(0x99AABBCCDDEEFF00, 0x1122334455667788, 0x123456789ABCDEF0);
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from_str_hex("0x123456789ABCDEF0112233445566778899AABBCCDDEEFF00").unwrap()
            );
        }
    }

    #[test]
    fn test_push25() {}

    #[test]
    fn test_push26() {}

    #[test]
    fn test_push27() {}

    #[test]
    fn test_push28() {}

    #[test]
    fn test_push29() {}

    #[test]
    fn test_push30() {}

    #[test]
    fn test_push31() {}

    #[test]
    fn test_push32() {
        unsafe {
            EVM.reset();
            push32(
                0x99AABBCCDDEEFF00,
                0x1122334455667788,
                0x123456789ABCDEF0,
                0xCAFEBABEDECAFBAD,
            );
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(
                EVM.stack.slots[0],
                Word::from_str_hex(
                    "0xCAFEBABEDECAFBAD123456789ABCDEF0112233445566778899AABBCCDDEEFF00"
                )
                .unwrap()
            );
        }
    }

    #[test]
    fn test_dup1() {
        unsafe {
            EVM.reset();
            push1(42);
            assert_eq!(EVM.stack.depth, 1);
            assert_eq!(EVM.stack.slots[0], 42);
            dup1();
            assert_eq!(EVM.stack.depth, 2);
            assert_eq!(EVM.stack.slots[1], 42);
        }
    }

    #[test]
    fn test_dup2() {
        unsafe {
            EVM.reset();
            push1(34);
            push1(12);
            assert_eq!(EVM.stack.depth, 2);
            assert_eq!(EVM.stack.slots[0], 34);
            assert_eq!(EVM.stack.slots[1], 12);
            dup2();
            assert_eq!(EVM.stack.depth, 3);
            assert_eq!(EVM.stack.slots[0], 34);
            assert_eq!(EVM.stack.slots[1], 12);
            assert_eq!(EVM.stack.slots[2], 34);
        }
    }

    #[test]
    fn test_dup3() {}

    #[test]
    fn test_dup4() {}

    #[test]
    fn test_dup5() {}

    #[test]
    fn test_dup6() {}

    #[test]
    fn test_dup7() {}

    #[test]
    fn test_dup8() {}

    #[test]
    fn test_dup9() {}

    #[test]
    fn test_dup10() {}

    #[test]
    fn test_dup11() {}

    #[test]
    fn test_dup12() {}

    #[test]
    fn test_dup13() {}

    #[test]
    fn test_dup14() {}

    #[test]
    fn test_dup15() {}

    #[test]
    fn test_dup16() {}

    #[test]
    fn test_swap1() {
        unsafe {
            EVM.reset();
            push1(34);
            push1(12);
            assert_eq!(EVM.stack.depth, 2);
            assert_eq!(EVM.stack.slots[0], 34);
            assert_eq!(EVM.stack.slots[1], 12);
            swap1();
            assert_eq!(EVM.stack.depth, 2);
            assert_eq!(EVM.stack.slots[0], 12);
            assert_eq!(EVM.stack.slots[1], 34);
        }
    }

    #[test]
    fn test_swap2() {}

    #[test]
    fn test_swap3() {}

    #[test]
    fn test_swap4() {}

    #[test]
    fn test_swap5() {}

    #[test]
    fn test_swap6() {}

    #[test]
    fn test_swap7() {}

    #[test]
    fn test_swap8() {}

    #[test]
    fn test_swap9() {}

    #[test]
    fn test_swap10() {}

    #[test]
    fn test_swap11() {}

    #[test]
    fn test_swap12() {}

    #[test]
    fn test_swap13() {}

    #[test]
    fn test_swap14() {}

    #[test]
    fn test_swap15() {}

    #[test]
    fn test_swap16() {}

    #[test]
    fn test_log0() {}

    #[test]
    fn test_log1() {}

    #[test]
    fn test_log2() {}

    #[test]
    fn test_log3() {}

    #[test]
    fn test_log4() {}

    #[test]
    fn test_create() {}

    #[test]
    fn test_call() {}

    #[test]
    fn test_callcode() {}

    #[test]
    fn test_return() {}

    #[test]
    fn test_delegatecall() {}

    #[test]
    fn test_create2() {}

    #[test]
    fn test_staticcall() {}

    #[test]
    fn test_revert() {}

    #[test]
    fn test_invalid() {}

    #[test]
    fn test_selfdestruct() {}
}