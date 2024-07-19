# AWS Mock

A mocking library for AWS. This is NOT an official AWS library.

## Description

This library provides a set of mocks for various AWS services, allowing you to test your code without actually making real API calls. It is based on the AWS SDKs for Rust.

## Installation

Add `aws_mocks` to your `Cargo.toml` file with the features corresponding to the services you need:

```toml
[dependencies]
aws_mocks = { version = "0.1.0", features = ["s3", "api-gateway", "kms"]}

[dev-dependencies]
aws_mocks = { version = "0.1.0", features = ["mocks"]}
```

## Usage
To use the mocks, you need to set up the mocks you want to use and then use the aws_sdk_mock crate to swap out the real AWS SDKs with the mocks. Here's an example:
```rust
use aws_config::BehaviorVersion;
use aws_mock::s3::operation::get_object::GetObjectOutput;
use aws_mock::s3::operation::get_object::GetObjectInput;
use aws_mock::s3::S3Client;
use aws_mock::s3::S3ClientImpl;
use aws_mock::s3::MockedS3Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let s3_client = S3ClientImpl::new(&config);
    let output = my_function(&s3_client).await;
    // Use output
    // -- snip --
}

async fn my_function(s3_client: &impl S3Client) -> GetObjectOutput {
    s3_client.get_object(
        GetObjectInput::builder().bucket("my_bucket").key("my_key")
    ).await.unwrap()
}

#[tokio::test]
async fn test() {
    let mut mock_s3_client = MockedS3Client::new();
    // Configure mock
    // -- snip --
    let output = my_function(&mock_s3_client);
    // Assert on output
    // -- snip --
}
```

## Supported Services
The following AWS services are currently supported:
- ApiGateway
- CognitoIdentityProvider
- DynamoDB
- KMS
- S3
- SecretsManager

## Contributing
Contributions are welcome! If you find any issues or have any feature requests, please open an issue or submit a pull request.

## License 
This project is licensed under the GPL-v3.0 License - see the [LICENSE](./LICENSE) for details.