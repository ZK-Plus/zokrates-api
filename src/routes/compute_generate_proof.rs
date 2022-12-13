use zokrates_api::ops::proof::generate_proof;
use zokrates_api::ops::witness::compute_witness;
use zokrates_api::utils::config::AppConfig;
use zokrates_api::utils::errors::{ApiError, ApiResult};
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::openapi;
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zokrates_ark::Ark;
use zokrates_ast::ir::ProgEnum;
use zokrates_ast::typed::abi::Abi;
use zokrates_proof_systems::GM17;

use crate::compute_witness::WitnessRequestBody;
use crate::generate_proof::GenerateProofResponseBody;

#[openapi]
#[post(
    "/<program_hash>/compute-generate-proof",
    data = "<witness>",
    format = "json"
)] //
pub fn post_compute_generate_proof(
    program_hash: &str,
    witness: Json<WitnessRequestBody>,
    config: &State<AppConfig>,
) -> ApiResult<GenerateProofResponseBody> {
    // parse input program
    let program_dir = Path::new(&config.out_dir).join(program_hash);
    if !program_dir.is_dir() {
        return Err(ApiError::ResourceNotFound(format!(
            "Proof {} have not been registered",
            program_hash
        )));
    }

    log::debug!("Reading all necessary files...");

    //TODO: make file reading async
    // read binary file
    let bin_path = program_dir.join("out");
    if !bin_path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "Binary file for proof {} does not exists. Commile the program first",
            program_hash
        )));
    }
    let mut file = File::open(&bin_path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", program_dir.display(), why))
    })?;
    let mut reader = BufReader::new(file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(ApiError::InternalError)?;

    // read abi file
    let mut path = program_dir.join("abi.json");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "ABI file for proof {} does not exists. Commile the program first",
            program_hash
        )));
    }
    file = File::open(&path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", path.display(), why))
    })?;
    let mut reader = BufReader::new(file);
    let abi: Abi =
        from_reader(&mut reader).map_err(|why| ApiError::InternalError(why.to_string()))?;

    // read proving key
    path = program_dir.join("proving.key");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "Binary file for proof {} does not exists. Commile the program first",
            program_hash
        )));
    }
    let pk_file = File::open(&path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", path.display(), why))
    })?;
    let mut pk: Vec<u8> = Vec::new();
    let mut pk_reader = BufReader::new(pk_file);
    pk_reader.read_to_end(&mut pk).map_err(|why| {
        ApiError::InternalError(format!("Could not read {}: {}", path.display(), why))
    })?;
    log::debug!("read proving key successfully");

    match prog {
        ProgEnum::Bn128Program(p) => {
            log::debug!("Computing witness...");
            let (witness, _output) =
                compute_witness(p, witness.payload.clone(), abi).map_err(|err| {
                    ApiError::CompilationError(format!("error computing witness:\n {}", err))
                })?;

            log::debug!("Generating proof...");
            // TODO: binary is being read twice, due to move ownership in compute_witnes
            let file = File::open(&bin_path).map_err(|why| {
                ApiError::InternalError(format!(
                    "Could not open {}: {}",
                    program_dir.display(),
                    why
                ))
            })?;
            let mut reader = BufReader::new(file);
            let prog = ProgEnum::deserialize(&mut reader).map_err(ApiError::InternalError)?;

            let proof = match prog {
                ProgEnum::Bn128Program(p) => generate_proof::<_, _, GM17, Ark>(p, witness, pk)
                    .map_err(ApiError::CompilationError)?,
                _ => unreachable!(),
            };

            let proof_str = serde_json::to_string_pretty(&proof).unwrap();
            log::debug!("Proof:\n{}", proof_str);
            let proof_json = serde_json::from_str(&proof_str).unwrap();

            Ok(Json(GenerateProofResponseBody {
                payload: proof_json,
            }))
        }
        _ => unreachable!(),
    }
}

// FIXME: add unittest for route
// #[cfg(test)] use rocket::local::blocking::Client;
// #[cfg(test)] use rocket::http::{Status, ContentType};

// mock generate_proof function
//  #[test]
// fn test_post_generate_proof() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     let res = client.post("/generate-proof")
//         .header(ContentType::JSON)
//         .body(r##"{
//             "proving_key": "ridicolous text"
//         }"##)
//         .dispatch();
//     assert_eq!(res.status(), Status::Ok);
// }

//  #[test]
// fn test_generate_proof() {
//     let proof = let proof = generate_proof::<_, _, GM17, Ark>(p)
// .map_err(|e| NotFound(e.to_string()))?;
//     assert_eq!(proof, blablabla);
// }
