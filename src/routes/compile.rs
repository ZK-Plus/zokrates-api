use zokrates_api::ops::compilation::api_compile;
use zokrates_api::utils::config::AppConfig;
use zokrates_api::utils::errors::{ApiError, ApiResult};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Data, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde_json::to_writer_pretty;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, remove_dir_all, write, File};
use std::io::BufWriter;
use std::path::Path;
use typed_arena::Arena;
use zokrates_field::Bn128Field;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CompileResponseBody {
    program_hash: String,
    // Abi type is not supported by JsonSchema
    abi: serde_json::Value,
}

#[openapi]
#[post("/compile", data = "<program_file>")]
pub async fn post_compile_zokrates(
    program_file: Data<'_>,
    config: &State<AppConfig>,
) -> ApiResult<CompileResponseBody> {
    // create a hash for the .zok code, if the hash exists return err
    let file_size_limit = config.zok_program_size_limit.parse().unwrap();
    let program = program_file
        .open(file_size_limit)
        .into_string()
        .await
        .unwrap()
        .into_inner();
    let program_hash = format!("{:X}", Sha256::digest(&program));
    let path = Path::new(&config.out_dir).join(&program_hash);
    if path.is_dir() {
        return Err(ApiError::ResourceAlreadyExists(String::from(
            "proof already exists",
        )));
    }

    // create all file paths
    let program_path = path.join("program.zok");
    let bin_output_path = path.join("out");
    let abi_spec_path = path.join("abi.json");
    let arena = Arena::new();

    // compile .zok code
    let compilation_artifacts = api_compile::<Bn128Field>(&program, &program_path, &arena)
        .map_err(ApiError::CompilationError)?;
    let (compiled_program, abi) = compilation_artifacts.into_inner();

    // create dir with the hash of the program
    create_dir_all(&path).map_err(|e| ApiError::InternalError(e.to_string()))?;

    // if compilation successful write .zok, binary and abi file under the hash folder
    let write_outputs = || -> Result<usize, String> {
        // serialize flattened program and write to binary file
        log::debug!("Serialize program");
        let bin_output_file = File::create(&bin_output_path)
            .map_err(|why| format!("Could not create {}: {}", bin_output_path.display(), why))?;
        let mut writer = BufWriter::new(bin_output_file);
        let constrain_count = compiled_program
            .serialize(&mut writer)
            .map_err(|e| e.to_string())?;

        // serialize ABI spec and write to JSON file
        log::debug!("Serialize ABI");
        let abi_spec_file = File::create(&abi_spec_path)
            .map_err(|why| format!("Could not create {}: {}", abi_spec_path.display(), why))?;
        let mut writer = BufWriter::new(abi_spec_file);
        to_writer_pretty(&mut writer, &abi)
            .map_err(|_| "Unable to write data to file.".to_string())?;

        // write .zok file in folder
        write(&program_path, &program).expect("Unable to write .zok file");

        Ok(constrain_count)
    };

    match write_outputs() {
        Ok(constrain_count) => {
            log::info!("zokrates program written to '{}'", program_path.display());
            log::info!("Compiled code written to '{}'", bin_output_path.display());
            log::info!("abi file written to '{}'", abi_spec_path.display());
            log::info!("Number of constraints: {}", constrain_count);

            // convert abi type to json value
            let abi_str = serde_json::to_string_pretty(&abi).unwrap();
            log::debug!("Proof:\n{}", abi_str);
            let abi_json = serde_json::from_str(&abi_str).unwrap();

            Ok(Json(CompileResponseBody {
                program_hash,
                abi: abi_json,
            }))
        }
        Err(e) => {
            // something wrong happened, clean up
            remove_dir_all(path).unwrap();
            Err(ApiError::InternalError(e))
        }
    }
}


#[cfg(test)]
mod test {
    use super::super::super::rocket;
    use super::*;
    use std::fs::read_to_string;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;


    #[test]
    fn successful_compilation() {
        let program_hash =
            "FF2482276ADCD956ACB349EC598F31C33DA08B210567E5847D46D18C73855365".to_string();
        let program_abi_str = r#"{
            "inputs": [
                {
                    "name": "N",
                    "public": true,
                    "type": "field"
                }
            ],
            "output": {
                "type": "bool"
            }
        }"#;
        let program_abi: serde_json::Value =
            serde_json::from_str(program_abi_str).expect("correct json abi string");

        
        let file = read_to_string("tests/test.zok").unwrap();
        let client = Client::tracked(rocket()).unwrap();
        let res = client
            .post(uri!(post_compile_zokrates))
            .body(file)
            .dispatch();

        println!("{:?}", res);
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.content_type(), Some(ContentType::JSON));

        let compilation = res
            .into_json::<CompileResponseBody>()
            .expect("Compile Response Body");
        assert_eq!(compilation.program_hash, program_hash);
        assert_eq!(compilation.abi, program_abi);

        // delete compilation outputs
        remove_dir_all(format!("out/{}", program_hash)).unwrap();
    }
}
