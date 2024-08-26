use rand::{rngs::StdRng, SeedableRng};
use std::io::Read;

use zokrates_ast::ir;
use zokrates_field::Field;
use zokrates_proof_systems::*;

pub fn generate_proof<
    'a,
    T: Field,
    I: Iterator<Item = ir::Statement<'a, T>>,
    S: Scheme<T>,
    B: Backend<T, S>,
>(
    program: ir::ProgIterator<'a, T, I>,
    witness: ir::Witness<T>,
    pk: impl Read,
) -> Result<TaggedProof<T, S>, String> {
    log::info!("Generating proof...");
    let mut rng = StdRng::from_entropy();
    let proof = B::generate_proof(program, witness, pk, &mut rng);
    Ok(TaggedProof::<T, S>::new(proof.proof, proof.inputs))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use zokrates_ark::Ark;
    use zokrates_ast::ir::ProgEnum;
    use zokrates_proof_systems::GM17;

    #[test]
    fn test_generate_correct_proof() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();

        let witness_file: File = File::open("tests/witness").unwrap();
        let witness_reader = BufReader::new(witness_file);
        let witness = ir::Witness::read(witness_reader).unwrap();

        let pk_file = File::open("tests/proving.key").unwrap();
        let pk_reader = BufReader::new(pk_file);

        let proof = match prog {
            ProgEnum::Bn128Program(p) => generate_proof::<_, _, GM17, Ark>(p, witness, pk_reader),
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

        let witness_file: File = File::open("tests/witness").unwrap();
        let witness_reader = BufReader::new(witness_file);
        let witness = ir::Witness::read(witness_reader).unwrap();

        let pk_file = File::open("tests/proving.key").unwrap();
        let pk_reader = BufReader::new(pk_file);

        let proof = match prog {
            ProgEnum::Bn128Program(p) => generate_proof::<_, _, GM17, Ark>(p, witness, pk_reader),
            _ => unreachable!(),
        };

        assert!(proof.is_ok());
    }
}
