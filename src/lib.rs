#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::instruction::Instruction;

    #[test]
    fn test_hello_world() {
        let program_id_keypair_bytes = std::fs::read("deploy/sbpf-asm-abort-keypair.json").unwrap()
            [..32]
            .try_into()
            .expect("slice with incorrect length");
        let program_id = Pubkey::new_from_array(program_id_keypair_bytes);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![]
        );

        let mollusk = Mollusk::new(&program_id, "deploy/sbpf-asm-abort");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[],
            &[Check::success()]
        );
        assert!(!result.program_result.is_err());
    }
}