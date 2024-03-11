#[cfg(test)]
mod tests {
    use axum::http;
    use http::StatusCode;
    use authentication_service::routes::health; // Assuming health.rs is within the routes module

    #[test]
    fn test_health() {
        let response = health();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
