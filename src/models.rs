use serde_derive::{Deserialize, Serialize};

// the input to our `check_solution` handler
#[derive(Debug, Deserialize)]
pub struct SolutionReq {
    pub flights: Vec<Vec<String>>,
}

// the output to our `check_solution` handler
#[derive(Debug, Serialize)]
pub struct SolutionResp {
    pub code: i32,
    pub message: String,
    pub answer: Vec<String>,
}
