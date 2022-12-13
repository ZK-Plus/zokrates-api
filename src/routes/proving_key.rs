use zokrates_api::utils::config::AppConfig;
use zokrates_api::utils::errors::{ApiError, ApiResult};
use rocket::data::ToByteUnit;
use rocket::serde::{json::Json, Serialize};
use rocket::{Data, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use std::path::Path;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProvingKeyResponseBody {
    message: String,
}

#[openapi]
#[post("/<program_hash>/proving-key", data = "<upload>")]
pub async fn post_proving_key(
    program_hash: &str,
    upload: Data<'_>,
    config: &State<AppConfig>,
) -> ApiResult<ProvingKeyResponseBody> {
    // create a hash for the .zok code, if the hash exists return err
    let path = Path::new(&config.out_dir).join(program_hash);
    if !path.is_dir() {
        return Err(ApiError::ResourceNotFound(format!(
            "Proof {} have not been registered",
            program_hash
        )));
    }

    let permanent_location = path.join("proving.key");
    upload
        .open(2000.megabytes())
        .into_file(permanent_location)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(ProvingKeyResponseBody {
        message: format!("proving key recorded for proof {}", program_hash),
    }))
}
