use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub out_dir: String,
    pub zok_program_size_limit: String,
    pub proving_key_file_size_limit: String,
}
