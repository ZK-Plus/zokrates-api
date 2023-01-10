use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{post, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zokrates_api::ops::witness::compute_witness;
use zokrates_api::utils::config::AppConfig;
use zokrates_api::utils::errors::{ApiError, ApiResult};
use zokrates_ast::ir::ProgEnum;
use zokrates_ast::typed::abi::Abi;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[schemars(example = "request_example")]
pub struct WitnessRequestBody {
    pub payload: serde_json::Value,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct WitnessResponseBody {
    output: serde_json::Value,
    witness: String,
}

#[openapi]
#[post("/<program_hash>/compute-witness", data = "<witness>", format = "json")] //
pub fn post_witness(
    program_hash: &str,
    witness: Json<WitnessRequestBody>,
    config: &State<AppConfig>,
) -> ApiResult<WitnessResponseBody> {
    // parse input program
    let program_dir = Path::new(&config.out_dir).join(program_hash);
    if !program_dir.is_dir() {
        return Err(ApiError::ResourceNotFound(format!(
            "Proof {program_hash} have not been registered",
        )));
    }

    //TODO: make file reading async
    // read binary file
    let mut path = program_dir.join("out");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "Binary file for proof {program_hash} does not exists. Commile the program first",
        )));
    }
    let mut file = File::open(&path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", program_dir.display(), why))
    })?;
    let mut reader = BufReader::new(file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(ApiError::InternalError)?;

    // read abi file
    path = program_dir.join("abi.json");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!(
            "ABI file for proof {program_hash} does not exists. Commile the program first",
        )));
    }
    file = File::open(&path).map_err(|why| {
        ApiError::InternalError(format!("Could not open {}: {}", path.display(), why))
    })?;
    let mut reader = BufReader::new(file);
    let abi: Abi =
        from_reader(&mut reader).map_err(|why| ApiError::InternalError(why.to_string()))?;

    match prog {
        ProgEnum::Bn128Program(p) => match compute_witness(p, witness.payload.clone(), abi) {
            Ok((witness, output)) => Ok(Json(WitnessResponseBody {
                witness: witness.to_string(),
                output,
            })),
            Err(err) => Err(ApiError::CompilationError(format!(
                "error computing witness:\n {err}",
            ))),
        },
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

fn request_example() -> WitnessRequestBody {
    let payload = serde_json::to_value(["1"]).unwrap();

    WitnessRequestBody { payload }
}
