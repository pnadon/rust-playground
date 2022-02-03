use actix_web::{HttpServer, dev::Server, App, web, HttpResponse};

fn main() {
    println!("Hello, world!");
    dbg!(check_mean(&vec![1, 2, 3, 4, 5]));
}

fn check_mean(nums: &[usize]) -> Option<usize> {
    match nums.len() {
        0 => None,
        len => Some(nums.iter().sum::<usize>() / len),
    }
}

async fn return_ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn create_api() -> std::io::Result<Server> {
    Ok(HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(return_ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, time::Duration};

    use super::*;

    #[test]
    fn ensure_check_mean_returns_none_on_empty() {
        let result = check_mean(&vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn ensure_check_mean_works_given_slice_of_identical_nums() {
        let result = check_mean(&vec![3, 3, 3, 3, 3, 3, 3]);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn ensure_check_mean_works_given_specific_examples() {
        let input_and_expectations = vec![
            (vec![0], 0),
            (vec![2, 2, 2, 6, 6, 6], 4),
            (vec![1, 2, 3, 4, 6], 3), // actually equals 3.2, but should output usize=3
            (vec![9, 8, 7, 6, 5, 5, 4, 3, 2, 1], 5),
        ];

        for (input, expectation) in input_and_expectations {
            assert_eq!(check_mean(&input), Some(expectation));
        }
    }

    #[quickcheck_macros::quickcheck]
    fn check_mean_returns_some_for_any_nonempty_slice(nums: Vec<usize>, num: usize) -> bool {
        let mut input = vec![num];
        input.extend_from_slice(&nums);
        check_mean(&input).is_some()
    }

    #[actix_rt::test]
    async fn test_api_returns_200() {
        let _ = tokio::spawn(
            create_api().expect("failed ot start api"));
        let response = reqwest::get("http://127.0.0.1:8080/").await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }
}
