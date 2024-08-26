use rocket::serde::{json::Json, Serialize};
use rocket::{
    State,
    data::{Data, ToByteUnit},
};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use zokrates_api::ops::proof::generate_proof;
use zokrates_api::utils::config::AppConfig;
use zokrates_api::utils::errors::{ApiError, ApiResult};
use zokrates_ark::Ark;
use zokrates_ast::ir::{self, ProgEnum};
use zokrates_proof_systems::GM17;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofResponseBody {
    // TODO: serialize TaggedProof
    pub payload: serde_json::Value,
    pub time: Duration,
}

#[openapi]
#[post("/<program_hash>/generate-proof", data = "<witness>")]
pub async fn post_generate_proof(
    program_hash: &str,
    witness: Data<'_>,
    config: &State<AppConfig>,
) -> ApiResult<GenerateProofResponseBody> {
    let now = Instant::now();
    // parse input program
    let program_dir = Path::new(&config.out_dir).join(program_hash);
    if !program_dir.is_dir() {
        return Err(ApiError::ResourceNotFound(format!(
            "Proof {program_hash} have not been registered",
        )));
    }

    // read binary file
    let mut path = program_dir.join("out");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "Binary file for proof {program_hash} does not exists. Commile the program first",
        )));
    }
    let program_file = File::open(&path).map_err(|e| ApiError::InternalError(e.to_string()))?;
    let mut reader = BufReader::new(program_file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(ApiError::InternalError)?;
    log::debug!("binary deserialization successfull");

    // read proving key
    path = program_dir.join("proving.key");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "Binary file for proof {program_hash} does not exists. Commile the program first",
        )));
    }
    let pk_file = File::open(&path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", path.display(), why))
    })?;
    let pk_reader = BufReader::new(pk_file);

    log::debug!("read proving key successfully");

    // read witness for request body
    let witness_path = program_dir.join("witness");
    witness
        .open(200.mebibytes())
        .into_file(&witness_path).await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let witness_file = File::open(&witness_path)
        .map_err(|why| ApiError::InternalError(why.to_string()))?;

    let witness_reader = BufReader::new(witness_file);
    let witness = ir::Witness::read(witness_reader)
        .map_err(|why| ApiError::InternalError(format!("Could not load witness: {why:?}")))?;
    log::debug!("read witness successfully");

    match prog {
        ProgEnum::Bn128Program(p) => {
            let proof = generate_proof::<_, _, GM17, Ark>(p, witness, pk_reader)
                .map_err(ApiError::CompilationError)?;

            let proof_str = serde_json::to_string_pretty(&proof).unwrap();
            log::debug!("Proof:\n{}", proof_str);
            let proof = serde_json::from_str(&proof_str).unwrap();

            let _ = fs::remove_file(witness_path);
            Ok(Json(GenerateProofResponseBody {
                time: now.elapsed(),
                payload: proof,
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

