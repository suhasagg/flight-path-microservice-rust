use std::time::Instant;

use crate::models::*;
use crate::solution::find_total_path;
use axum::{http::StatusCode, response::IntoResponse, Json};

// basic health handler
pub async fn check_health() -> &'static str {
    "OK"
}

// solution handler
pub async fn check_solution(Json(req): Json<SolutionReq>) -> impl IntoResponse {
    // simple validate input
    if req.flights.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(SolutionResp {
                code: -1,
                message: "bad input, flights is empty".to_string(),
                answer: vec![],
            }),
        );
    }
    for flight in &req.flights {
        if flight.len() != 2 {
            return (
                StatusCode::BAD_REQUEST,
                Json(SolutionResp {
                    code: -2,
                    message: "Bad input, some flight len is not 2".to_string(),
                    answer: vec![],
                }),
            );
        }
        if flight[0] == flight[1] {
            return (
                StatusCode::BAD_REQUEST,
                Json(SolutionResp {
                    code: -3,
                    message: "Bad input, some flight start == end".to_string(),
                    answer: vec![],
                }),
            );
        }
        for p in flight {
            if p.trim().is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(SolutionResp {
                        code: -4,
                        message: "Bad input, some flight is empty".to_string(),
                        answer: vec![],
                    }),
                );
            }
        }
    }

    // solution
    tracing::debug!("input {:?}", &req.flights);
    let now = Instant::now();
    let res = find_total_path(req.flights);
    let elapsed = now.elapsed();
    tracing::debug!("output {:?}", &res);

    (
        StatusCode::CREATED,
        Json(SolutionResp {
            code: 0,
            message: format!("Success, elapsed time: {:.2?}", elapsed),
            answer: res,
        }),
    )
}

// global 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Wrong path, Post /solution")
}
