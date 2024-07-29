# AWS Mocks

A mocking library for AWS. This is NOT an official AWS library.

## Description

This library provides a set of mocks for various AWS services, allowing you to test your code without actually making real API calls. It is based on the AWS SDKs for Rust.

## Installation

Add `aws-mocks` to your `Cargo.toml` file with the features corresponding to the services you need:

```toml
[dependencies]
aws-mocks = { version = "0.2.1", features = ["s3", "api-gateway", "kms"]}

[dev-dependencies]
aws-mocks = { version = "0.2.1", features = ["mockall"]}
```

## Usage
To use the mocks, you need to add the features for the services you want to use and use the corresponding traits for dependency injection.
Then add the `mockall` feature to your `[dev-dependencies]` and pass the mock services to your functions.
Here's an example for S3:
```rust
use aws_config::BehaviorVersion;
use aws_mocks::s3::operation::get_object::GetObjectOutput;
use aws_mocks::s3::operation::get_object::GetObjectInput;
use aws_mocks::s3::S3Client;
use aws_mocks::s3::S3ClientImpl;
use aws_mocks::s3::MockedS3Client;

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
The following are the AWS services currently supported and the features to enable them:

| Service                 | Feature                   |
|-------------------------|---------------------------|
| AccessAnalyzer          | access-analyzer           |
| Account                 | account                   |
| ACM                     | acm                       |
| ACM PCA                 | acm-pca                   |
| AMP                     | amp                       |
| Amplify                 | amplify                   |
| Amplify Backend         | amplify-backend           |
| Amplify UI Builder      | amplify-ui-builder        |
| ApiGateway              | api-gateway               |
| ApiGateway Management   | api-gateway-management    |
| ApiGateway V2           | api-gateway-v2            |
| AppConfig               | app-config                |
| AppConfigData           | app-config-data           |
| AppFabric               | app-fabric                |
| AppFlow                 | app-flow                  |
| AppIntegrations         | app-integrations          |
| AppMesh                 | app-mesh                  |
| AppRunner               | app-runner                |
| AppStream               | app-stream                |
| AppSync                 | app-sync                  |
| AppTest                 | app-test                  |
| Athena                  | athena                    |
| Backup                  | backup                    |
| Batch                   | batch                     |
| CloudFormation          | cloud-formation           |
| CloudFront              | cloud-front               |
| CloudTrail              | cloud-trail               |
| CloudWatch              | cloud-watch               |
| CognitoIdentityProvider | cognito-identity-provider |
| CodeBuild               | code-build                |
| CodeCommit              | code-commit               |
| CodePipeline            | code-pipeline             |
| DataBrew                | data-brew                 |
| DataPipeline            | data-pipeline             |
| DirectConnect           | direct-connect            |
| DynamoDB                | dynamo-db                 |
| EC2                     | ec2                       |
| EFS                     | efs                       |
| EKS                     | eks                       |
| ElastiCache             | elasti-cache              |
| Elastic Beanstalk       | elastic-beanstalk         |
| EMR                     | emr                       |
| GlobalAccelerator       | global-accelerator        | 
| Glue                    | glue                      | 
| IAM                     | iam                       |
| Kinesis                 | kinesis                   |
| KMS                     | kms                       |
| Lambda                  | lambda                    |
| Macie 2                 | macie2                    |
| MQ                      | mq                        |
| OpenSearch              | open-search               |
| Polly                   | polly                     |
| QuickSight              | quick-sight               |
| RDS                     | rds                       |
| Redshift                | redshift                  |
| Rekognition             | rekognition               |
| S3                      | s3                        |
| SageMaker               | sage-maker                |
| SecretsManager          | secrets-manager           |
| SNS                     | sns                       |
| SQS                     | sqs                       |
| SSM                     | ssm                       |
| Textract                | textract                  |
| Transcribe              | transcribe                |
| Translate               | translate                 |
| X-Ray                   | x-ray                     |

Not every command for every service is supported. Notably, commands that require subcommands are not yet supported.

## Contributing
Contributions are welcome! If you find any issues or have any feature requests, please open an issue or submit a pull request.

## License 
This project is licensed under the GPL-v3.0 License - see the [LICENSE](./LICENSE) for details.