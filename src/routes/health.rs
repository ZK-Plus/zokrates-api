use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::{openapi, JsonSchema};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    service_name: String,
    version: String,
    zokrates_version: String,
}

#[openapi]
#[get("/health", format = "json")]
pub fn health() -> Json<Task> {
    Json(Task {
        // TODO: add version to response
        service_name: "Zokrates Prover".to_string(),
        version: "0.1.1".to_string(),
        zokrates_version: "8.3".to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::super::super::*;
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn json_test_index() {
        let client = Client::tracked(rocket()).unwrap();
        let res = client
            .get(uri!(health))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
    }
}
