use zokrates_ast::ir;
use zokrates_field::Field;
use zokrates_proof_systems::*;

pub fn generate_proof<
    T: Field,
    I: Iterator<Item = ir::Statement<T>>,
    S: Scheme<T>,
    B: Backend<T, S>,
>(
    program: ir::ProgIterator<T, I>,
    witness: ir::Witness<T>,
    pk: std::vec::Vec<u8>,
) -> Result<TaggedProof<T, S>, String> {
    log::info!("Generating proof...");
    let proof = B::generate_proof(program, witness, pk);
    Ok(TaggedProof::<T, S>::new(proof.proof, proof.inputs))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use zokrates_ark::Ark;
    use zokrates_ast::ir::ProgEnum;
    use zokrates_proof_systems::GM17;

    #[test]
    fn test_generate_correct_proof() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();

        let witness_str = r#"~out_0 1
~one 1
_0 1
_2 0
_3 1"#;
        let witness = ir::Witness::read(witness_str.as_bytes()).unwrap();

        let pk_file = File::open("tests/proving.key").unwrap();
        let mut pk: Vec<u8> = Vec::new();
        let mut pk_reader = BufReader::new(pk_file);
        pk_reader.read_to_end(&mut pk).unwrap();

        let proof = match prog {
            ProgEnum::Bn128Program(p) => generate_proof::<_, _, GM17, Ark>(p, witness, pk),
            _ => unreachable!(),
        };
        assert!(proof.is_ok());

        // let (_, output) = witness.unwrap();
        // assert_eq!(output[0], true);
    }

    #[test]
    fn test_generate_wrong_proof() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();

        let witness_str = r#"~out_0 0
~one 1
_0 2
_2 1
_3 1"#;
        let witness = ir::Witness::read(witness_str.as_bytes()).unwrap();

        let pk_file = File::open("tests/proving.key").unwrap();
        let mut pk: Vec<u8> = Vec::new();
        let mut pk_reader = BufReader::new(pk_file);
        pk_reader.read_to_end(&mut pk).unwrap();

        let proof = match prog {
            ProgEnum::Bn128Program(p) => generate_proof::<_, _, GM17, Ark>(p, witness, pk),
            _ => unreachable!(),
        };

        assert!(proof.is_ok());
    }
}
