use api::server::services::movie_services::{MovieService, MovieServiceTrait};

#[tokio::test]
async fn decrypt_known_sample_successfully() {
    // arrange
    let service = MovieService::new();
    let encrypted_id = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA";

    // act
    let result = service.decrypt_movie_id(encrypted_id).await;

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.movie_id, "1311031");
    assert_eq!(response.timestamp, 1761037963);
    assert!(response.key_valid);
    assert!(response.timestamp_readable.is_some());
}

#[tokio::test]
async fn decrypt_second_known_sample_successfully() {
    // arrange
    let service = MovieService::new();
    let encrypted_id = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH9bwQ0Zdid-ohCscvgLYNcaCD89_dlYZ2cQif-vm";

    // act
    let result = service.decrypt_movie_id(encrypted_id).await;

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.movie_id, "617126");
    assert!(response.key_valid);
}

#[tokio::test]
async fn return_error_when_encrypted_id_is_invalid_base64() {
    // arrange
    let service = MovieService::new();
    let invalid_encrypted_id = "not-valid-base64!!!";

    // act
    let result = service.decrypt_movie_id(invalid_encrypted_id).await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn return_error_when_encrypted_data_is_too_short() {
    // arrange
    let service = MovieService::new();
    // Only 10 bytes of data (need at least 24 for nonce)
    let short_encrypted_id = "AAAAAAAAAAA";

    // act
    let result = service.decrypt_movie_id(short_encrypted_id).await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn return_error_when_decryption_fails() {
    // arrange
    let service = MovieService::new();
    // Valid base64 but random data that won't decrypt properly
    let random_data = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAArandomdatarandomdatarandomdatarandom";

    // act
    let result = service.decrypt_movie_id(random_data).await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn verify_key_returns_success() {
    // arrange
    let service = MovieService::new();

    // act
    let result = service.verify_key().await;

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.key_valid);
    assert_eq!(
        response.message,
        "Encryption key is valid and working correctly"
    );
    assert!(response.key_hex.is_some());
    assert_eq!(
        response.key_hex.unwrap(),
        "c75136c5668bbfe65a7ecad431a745db68b5f381555b38d8f6c699449cf11fcd"
    );
}

#[tokio::test]
async fn verify_key_decrypts_correctly_and_validates_values() {
    // arrange
    let service = MovieService::new();

    // act
    let verify_result = service.verify_key().await;

    // assert
    assert!(verify_result.is_ok());
    let verify_response = verify_result.unwrap();

    // Verify that the key verification actually decrypted the test sample
    // and got the expected values
    assert!(verify_response.key_valid);

    // Double check by decrypting the same sample manually
    let test_encrypted =
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA";
    let decrypt_result = service.decrypt_movie_id(test_encrypted).await;

    assert!(decrypt_result.is_ok());
    let decrypt_response = decrypt_result.unwrap();
    assert_eq!(decrypt_response.movie_id, "1311031");
    assert_eq!(decrypt_response.timestamp, 1761037963);
}

#[tokio::test]
async fn decrypt_handles_url_safe_base64_without_padding() {
    // arrange
    let service = MovieService::new();
    // URL-safe base64 with - and _ characters, no padding
    let url_safe_encrypted =
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH9bwQ0Zdid-ohCscvgLYNcaCD89_dlYZ2cQif-vm";

    // act
    let result = service.decrypt_movie_id(url_safe_encrypted).await;

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.movie_id, "617126");
}

#[tokio::test]
async fn decrypt_extracts_timestamp_correctly() {
    // arrange
    let service = MovieService::new();
    let encrypted_id = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA";

    // act
    let result = service.decrypt_movie_id(encrypted_id).await;

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();

    // Verify timestamp is within reasonable range (2025)
    assert!(response.timestamp > 1700000000); // After 2023
    assert!(response.timestamp < 1800000000); // Before 2027

    // Verify timestamp matches expected value
    assert_eq!(response.timestamp, 1761037963);

    // Verify readable timestamp is present and parseable
    assert!(response.timestamp_readable.is_some());
    let readable = response.timestamp_readable.unwrap();
    assert!(readable.contains("2025"));
}
