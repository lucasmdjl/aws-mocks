/*
 * aws_mock - A mocking library for AWS.
 *
 * Copyright (C) 2024 Lucas M. de Jong Larrarte
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use aws_sdk_amplifybackend::operation::clone_backend::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_backend::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_backend_api::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_backend_auth::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_backend_config::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_backend_storage::{builders::*, *};
use aws_sdk_amplifybackend::operation::create_token::{builders::*, *};
use aws_sdk_amplifybackend::operation::delete_backend::{builders::*, *};
use aws_sdk_amplifybackend::operation::delete_backend_api::{builders::*, *};
use aws_sdk_amplifybackend::operation::delete_backend_auth::{builders::*, *};
use aws_sdk_amplifybackend::operation::delete_backend_storage::{builders::*, *};
use aws_sdk_amplifybackend::operation::delete_token::{builders::*, *};
use aws_sdk_amplifybackend::operation::generate_backend_api_models::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend_api::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend_api_models::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend_auth::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend_job::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_backend_storage::{builders::*, *};
use aws_sdk_amplifybackend::operation::get_token::{builders::*, *};
use aws_sdk_amplifybackend::operation::import_backend_auth::{builders::*, *};
use aws_sdk_amplifybackend::operation::import_backend_storage::{builders::*, *};
use aws_sdk_amplifybackend::operation::list_backend_jobs::{builders::*, *};
use aws_sdk_amplifybackend::operation::list_s3_buckets::{builders::*, *};
use aws_sdk_amplifybackend::operation::remove_all_backends::{builders::*, *};
use aws_sdk_amplifybackend::operation::remove_backend_config::{builders::*, *};
use aws_sdk_amplifybackend::operation::update_backend_api::{builders::*, *};
use aws_sdk_amplifybackend::operation::update_backend_auth::{builders::*, *};
use aws_sdk_amplifybackend::operation::update_backend_config::{builders::*, *};
use aws_sdk_amplifybackend::operation::update_backend_job::{builders::*, *};
use aws_sdk_amplifybackend::operation::update_backend_storage::{builders::*, *};
use aws_sdk_amplifybackend::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_amplifybackend::Client;

pub use aws_sdk_amplifybackend::*;

pub struct AmplifyBackendClientImpl(Client);
impl AmplifyBackendClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AmplifyBackendClient {
    fn clone_backend(&self, builder: CloneBackendInputBuilder) -> impl Future<Output = Result<CloneBackendOutput, SdkError<CloneBackendError>>>;
    fn create_backend(&self, builder: CreateBackendInputBuilder) -> impl Future<Output = Result<CreateBackendOutput, SdkError<CreateBackendError>>>;
    fn create_backend_api(&self, builder: CreateBackendApiInputBuilder) -> impl Future<Output = Result<CreateBackendApiOutput, SdkError<CreateBackendAPIError>>>;
    fn create_backend_auth(&self, builder: CreateBackendAuthInputBuilder) -> impl Future<Output = Result<CreateBackendAuthOutput, SdkError<CreateBackendAuthError>>>;
    fn create_backend_config(&self, builder: CreateBackendConfigInputBuilder) -> impl Future<Output = Result<CreateBackendConfigOutput, SdkError<CreateBackendConfigError>>>;
    fn create_backend_storage(&self, builder: CreateBackendStorageInputBuilder) -> impl Future<Output = Result<CreateBackendStorageOutput, SdkError<CreateBackendStorageError>>>;
    fn create_token(&self, builder: CreateTokenInputBuilder) -> impl Future<Output = Result<CreateTokenOutput, SdkError<CreateTokenError>>>;
    fn delete_backend(&self, builder: DeleteBackendInputBuilder) -> impl Future<Output = Result<DeleteBackendOutput, SdkError<DeleteBackendError>>>;
    fn delete_backend_api(&self, builder: DeleteBackendApiInputBuilder) -> impl Future<Output = Result<DeleteBackendApiOutput, SdkError<DeleteBackendAPIError>>>;
    fn delete_backend_auth(&self, builder: DeleteBackendAuthInputBuilder) -> impl Future<Output = Result<DeleteBackendAuthOutput, SdkError<DeleteBackendAuthError>>>;
    fn delete_backend_storage(&self, builder: DeleteBackendStorageInputBuilder) -> impl Future<Output = Result<DeleteBackendStorageOutput, SdkError<DeleteBackendStorageError>>>;
    fn delete_token(&self, builder: DeleteTokenInputBuilder) -> impl Future<Output = Result<DeleteTokenOutput, SdkError<DeleteTokenError>>>;
    fn generate_backend_api_models(&self, builder: GenerateBackendApiModelsInputBuilder) -> impl Future<Output = Result<GenerateBackendApiModelsOutput, SdkError<GenerateBackendAPIModelsError>>>;
    fn get_backend(&self, builder: GetBackendInputBuilder) -> impl Future<Output = Result<GetBackendOutput, SdkError<GetBackendError>>>;
    fn get_backend_api(&self, builder: GetBackendApiInputBuilder) -> impl Future<Output = Result<GetBackendApiOutput, SdkError<GetBackendAPIError>>>;
    fn get_backend_api_models(&self, builder: GetBackendApiModelsInputBuilder) -> impl Future<Output = Result<GetBackendApiModelsOutput, SdkError<GetBackendAPIModelsError>>>;
    fn get_backend_auth(&self, builder: GetBackendAuthInputBuilder) -> impl Future<Output = Result<GetBackendAuthOutput, SdkError<GetBackendAuthError>>>;
    fn get_backend_job(&self, builder: GetBackendJobInputBuilder) -> impl Future<Output = Result<GetBackendJobOutput, SdkError<GetBackendJobError>>>;
    fn get_backend_storage(&self, builder: GetBackendStorageInputBuilder) -> impl Future<Output = Result<GetBackendStorageOutput, SdkError<GetBackendStorageError>>>;
    fn get_token(&self, builder: GetTokenInputBuilder) -> impl Future<Output = Result<GetTokenOutput, SdkError<GetTokenError>>>;
    fn import_backend_auth(&self, builder: ImportBackendAuthInputBuilder) -> impl Future<Output = Result<ImportBackendAuthOutput, SdkError<ImportBackendAuthError>>>;
    fn import_backend_storage(&self, builder: ImportBackendStorageInputBuilder) -> impl Future<Output = Result<ImportBackendStorageOutput, SdkError<ImportBackendStorageError>>>;
    fn list_backend_jobs(&self, builder: ListBackendJobsInputBuilder) -> impl Future<Output = Result<ListBackendJobsOutput, SdkError<ListBackendJobsError>>>;
    fn list_s3_buckets(&self, builder: ListS3BucketsInputBuilder) -> impl Future<Output = Result<ListS3BucketsOutput, SdkError<ListS3BucketsError>>>;
    fn remove_all_backends(&self, builder: RemoveAllBackendsInputBuilder) -> impl Future<Output = Result<RemoveAllBackendsOutput, SdkError<RemoveAllBackendsError>>>;
    fn remove_backend_config(&self, builder: RemoveBackendConfigInputBuilder) -> impl Future<Output = Result<RemoveBackendConfigOutput, SdkError<RemoveBackendConfigError>>>;
    fn update_backend_api(&self, builder: UpdateBackendApiInputBuilder) -> impl Future<Output = Result<UpdateBackendApiOutput, SdkError<UpdateBackendAPIError>>>;
    fn update_backend_auth(&self, builder: UpdateBackendAuthInputBuilder) -> impl Future<Output = Result<UpdateBackendAuthOutput, SdkError<UpdateBackendAuthError>>>;
    fn update_backend_config(&self, builder: UpdateBackendConfigInputBuilder) -> impl Future<Output = Result<UpdateBackendConfigOutput, SdkError<UpdateBackendConfigError>>>;
    fn update_backend_job(&self, builder: UpdateBackendJobInputBuilder) -> impl Future<Output = Result<UpdateBackendJobOutput, SdkError<UpdateBackendJobError>>>;
    fn update_backend_storage(&self, builder: UpdateBackendStorageInputBuilder) -> impl Future<Output = Result<UpdateBackendStorageOutput, SdkError<UpdateBackendStorageError>>>;
}
impl AmplifyBackendClient for AmplifyBackendClientImpl {
    fn clone_backend(&self, builder: CloneBackendInputBuilder) -> impl Future<Output = Result<CloneBackendOutput, SdkError<CloneBackendError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend(&self, builder: CreateBackendInputBuilder) -> impl Future<Output = Result<CreateBackendOutput, SdkError<CreateBackendError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend_api(&self, builder: CreateBackendApiInputBuilder) -> impl Future<Output = Result<CreateBackendApiOutput, SdkError<CreateBackendAPIError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend_auth(&self, builder: CreateBackendAuthInputBuilder) -> impl Future<Output = Result<CreateBackendAuthOutput, SdkError<CreateBackendAuthError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend_config(&self, builder: CreateBackendConfigInputBuilder) -> impl Future<Output = Result<CreateBackendConfigOutput, SdkError<CreateBackendConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend_storage(&self, builder: CreateBackendStorageInputBuilder) -> impl Future<Output = Result<CreateBackendStorageOutput, SdkError<CreateBackendStorageError>>> {
        builder.send_with(&self.0)
    }
    fn create_token(&self, builder: CreateTokenInputBuilder) -> impl Future<Output = Result<CreateTokenOutput, SdkError<CreateTokenError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backend(&self, builder: DeleteBackendInputBuilder) -> impl Future<Output = Result<DeleteBackendOutput, SdkError<DeleteBackendError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backend_api(&self, builder: DeleteBackendApiInputBuilder) -> impl Future<Output = Result<DeleteBackendApiOutput, SdkError<DeleteBackendAPIError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backend_auth(&self, builder: DeleteBackendAuthInputBuilder) -> impl Future<Output = Result<DeleteBackendAuthOutput, SdkError<DeleteBackendAuthError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backend_storage(&self, builder: DeleteBackendStorageInputBuilder) -> impl Future<Output = Result<DeleteBackendStorageOutput, SdkError<DeleteBackendStorageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_token(&self, builder: DeleteTokenInputBuilder) -> impl Future<Output = Result<DeleteTokenOutput, SdkError<DeleteTokenError>>> {
        builder.send_with(&self.0)
    }
    fn generate_backend_api_models(&self, builder: GenerateBackendApiModelsInputBuilder) -> impl Future<Output = Result<GenerateBackendApiModelsOutput, SdkError<GenerateBackendAPIModelsError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend(&self, builder: GetBackendInputBuilder) -> impl Future<Output = Result<GetBackendOutput, SdkError<GetBackendError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_api(&self, builder: GetBackendApiInputBuilder) -> impl Future<Output = Result<GetBackendApiOutput, SdkError<GetBackendAPIError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_api_models(&self, builder: GetBackendApiModelsInputBuilder) -> impl Future<Output = Result<GetBackendApiModelsOutput, SdkError<GetBackendAPIModelsError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_auth(&self, builder: GetBackendAuthInputBuilder) -> impl Future<Output = Result<GetBackendAuthOutput, SdkError<GetBackendAuthError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_job(&self, builder: GetBackendJobInputBuilder) -> impl Future<Output = Result<GetBackendJobOutput, SdkError<GetBackendJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_storage(&self, builder: GetBackendStorageInputBuilder) -> impl Future<Output = Result<GetBackendStorageOutput, SdkError<GetBackendStorageError>>> {
        builder.send_with(&self.0)
    }
    fn get_token(&self, builder: GetTokenInputBuilder) -> impl Future<Output = Result<GetTokenOutput, SdkError<GetTokenError>>> {
        builder.send_with(&self.0)
    }
    fn import_backend_auth(&self, builder: ImportBackendAuthInputBuilder) -> impl Future<Output = Result<ImportBackendAuthOutput, SdkError<ImportBackendAuthError>>> {
        builder.send_with(&self.0)
    }
    fn import_backend_storage(&self, builder: ImportBackendStorageInputBuilder) -> impl Future<Output = Result<ImportBackendStorageOutput, SdkError<ImportBackendStorageError>>> {
        builder.send_with(&self.0)
    }
    fn list_backend_jobs(&self, builder: ListBackendJobsInputBuilder) -> impl Future<Output = Result<ListBackendJobsOutput, SdkError<ListBackendJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_s3_buckets(&self, builder: ListS3BucketsInputBuilder) -> impl Future<Output = Result<ListS3BucketsOutput, SdkError<ListS3BucketsError>>> {
        builder.send_with(&self.0)
    }
    fn remove_all_backends(&self, builder: RemoveAllBackendsInputBuilder) -> impl Future<Output = Result<RemoveAllBackendsOutput, SdkError<RemoveAllBackendsError>>> {
        builder.send_with(&self.0)
    }
    fn remove_backend_config(&self, builder: RemoveBackendConfigInputBuilder) -> impl Future<Output = Result<RemoveBackendConfigOutput, SdkError<RemoveBackendConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_backend_api(&self, builder: UpdateBackendApiInputBuilder) -> impl Future<Output = Result<UpdateBackendApiOutput, SdkError<UpdateBackendAPIError>>> {
        builder.send_with(&self.0)
    }
    fn update_backend_auth(&self, builder: UpdateBackendAuthInputBuilder) -> impl Future<Output = Result<UpdateBackendAuthOutput, SdkError<UpdateBackendAuthError>>> {
        builder.send_with(&self.0)
    }
    fn update_backend_config(&self, builder: UpdateBackendConfigInputBuilder) -> impl Future<Output = Result<UpdateBackendConfigOutput, SdkError<UpdateBackendConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_backend_job(&self, builder: UpdateBackendJobInputBuilder) -> impl Future<Output = Result<UpdateBackendJobOutput, SdkError<UpdateBackendJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_backend_storage(&self, builder: UpdateBackendStorageInputBuilder) -> impl Future<Output = Result<UpdateBackendStorageOutput, SdkError<UpdateBackendStorageError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: AmplifyBackendClient> AmplifyBackendClient for &T {
    fn clone_backend(&self, builder: CloneBackendInputBuilder) -> impl Future<Output = Result<CloneBackendOutput, SdkError<CloneBackendError>>> {
        (*self).clone_backend(builder)
    }
    fn create_backend(&self, builder: CreateBackendInputBuilder) -> impl Future<Output = Result<CreateBackendOutput, SdkError<CreateBackendError>>> {
        (*self).create_backend(builder)
    }
    fn create_backend_api(&self, builder: CreateBackendApiInputBuilder) -> impl Future<Output = Result<CreateBackendApiOutput, SdkError<CreateBackendAPIError>>> {
        (*self).create_backend_api(builder)
    }
    fn create_backend_auth(&self, builder: CreateBackendAuthInputBuilder) -> impl Future<Output = Result<CreateBackendAuthOutput, SdkError<CreateBackendAuthError>>> {
        (*self).create_backend_auth(builder)
    }
    fn create_backend_config(&self, builder: CreateBackendConfigInputBuilder) -> impl Future<Output = Result<CreateBackendConfigOutput, SdkError<CreateBackendConfigError>>> {
        (*self).create_backend_config(builder)
    }
    fn create_backend_storage(&self, builder: CreateBackendStorageInputBuilder) -> impl Future<Output = Result<CreateBackendStorageOutput, SdkError<CreateBackendStorageError>>> {
        (*self).create_backend_storage(builder)
    }
    fn create_token(&self, builder: CreateTokenInputBuilder) -> impl Future<Output = Result<CreateTokenOutput, SdkError<CreateTokenError>>> {
        (*self).create_token(builder)
    }
    fn delete_backend(&self, builder: DeleteBackendInputBuilder) -> impl Future<Output = Result<DeleteBackendOutput, SdkError<DeleteBackendError>>> {
        (*self).delete_backend(builder)
    }
    fn delete_backend_api(&self, builder: DeleteBackendApiInputBuilder) -> impl Future<Output = Result<DeleteBackendApiOutput, SdkError<DeleteBackendAPIError>>> {
        (*self).delete_backend_api(builder)
    }
    fn delete_backend_auth(&self, builder: DeleteBackendAuthInputBuilder) -> impl Future<Output = Result<DeleteBackendAuthOutput, SdkError<DeleteBackendAuthError>>> {
        (*self).delete_backend_auth(builder)
    }
    fn delete_backend_storage(&self, builder: DeleteBackendStorageInputBuilder) -> impl Future<Output = Result<DeleteBackendStorageOutput, SdkError<DeleteBackendStorageError>>> {
        (*self).delete_backend_storage(builder)
    }
    fn delete_token(&self, builder: DeleteTokenInputBuilder) -> impl Future<Output = Result<DeleteTokenOutput, SdkError<DeleteTokenError>>> {
        (*self).delete_token(builder)
    }
    fn generate_backend_api_models(&self, builder: GenerateBackendApiModelsInputBuilder) -> impl Future<Output = Result<GenerateBackendApiModelsOutput, SdkError<GenerateBackendAPIModelsError>>> {
        (*self).generate_backend_api_models(builder)
    }
    fn get_backend(&self, builder: GetBackendInputBuilder) -> impl Future<Output = Result<GetBackendOutput, SdkError<GetBackendError>>> {
        (*self).get_backend(builder)
    }
    fn get_backend_api(&self, builder: GetBackendApiInputBuilder) -> impl Future<Output = Result<GetBackendApiOutput, SdkError<GetBackendAPIError>>> {
        (*self).get_backend_api(builder)
    }
    fn get_backend_api_models(&self, builder: GetBackendApiModelsInputBuilder) -> impl Future<Output = Result<GetBackendApiModelsOutput, SdkError<GetBackendAPIModelsError>>> {
        (*self).get_backend_api_models(builder)
    }
    fn get_backend_auth(&self, builder: GetBackendAuthInputBuilder) -> impl Future<Output = Result<GetBackendAuthOutput, SdkError<GetBackendAuthError>>> {
        (*self).get_backend_auth(builder)
    }
    fn get_backend_job(&self, builder: GetBackendJobInputBuilder) -> impl Future<Output = Result<GetBackendJobOutput, SdkError<GetBackendJobError>>> {
        (*self).get_backend_job(builder)
    }
    fn get_backend_storage(&self, builder: GetBackendStorageInputBuilder) -> impl Future<Output = Result<GetBackendStorageOutput, SdkError<GetBackendStorageError>>> {
        (*self).get_backend_storage(builder)
    }
    fn get_token(&self, builder: GetTokenInputBuilder) -> impl Future<Output = Result<GetTokenOutput, SdkError<GetTokenError>>> {
        (*self).get_token(builder)
    }
    fn import_backend_auth(&self, builder: ImportBackendAuthInputBuilder) -> impl Future<Output = Result<ImportBackendAuthOutput, SdkError<ImportBackendAuthError>>> {
        (*self).import_backend_auth(builder)
    }
    fn import_backend_storage(&self, builder: ImportBackendStorageInputBuilder) -> impl Future<Output = Result<ImportBackendStorageOutput, SdkError<ImportBackendStorageError>>> {
        (*self).import_backend_storage(builder)
    }
    fn list_backend_jobs(&self, builder: ListBackendJobsInputBuilder) -> impl Future<Output = Result<ListBackendJobsOutput, SdkError<ListBackendJobsError>>> {
        (*self).list_backend_jobs(builder)
    }
    fn list_s3_buckets(&self, builder: ListS3BucketsInputBuilder) -> impl Future<Output = Result<ListS3BucketsOutput, SdkError<ListS3BucketsError>>> {
        (*self).list_s3_buckets(builder)
    }
    fn remove_all_backends(&self, builder: RemoveAllBackendsInputBuilder) -> impl Future<Output = Result<RemoveAllBackendsOutput, SdkError<RemoveAllBackendsError>>> {
        (*self).remove_all_backends(builder)
    }
    fn remove_backend_config(&self, builder: RemoveBackendConfigInputBuilder) -> impl Future<Output = Result<RemoveBackendConfigOutput, SdkError<RemoveBackendConfigError>>> {
        (*self).remove_backend_config(builder)
    }
    fn update_backend_api(&self, builder: UpdateBackendApiInputBuilder) -> impl Future<Output = Result<UpdateBackendApiOutput, SdkError<UpdateBackendAPIError>>> {
        (*self).update_backend_api(builder)
    }
    fn update_backend_auth(&self, builder: UpdateBackendAuthInputBuilder) -> impl Future<Output = Result<UpdateBackendAuthOutput, SdkError<UpdateBackendAuthError>>> {
        (*self).update_backend_auth(builder)
    }
    fn update_backend_config(&self, builder: UpdateBackendConfigInputBuilder) -> impl Future<Output = Result<UpdateBackendConfigOutput, SdkError<UpdateBackendConfigError>>> {
        (*self).update_backend_config(builder)
    }
    fn update_backend_job(&self, builder: UpdateBackendJobInputBuilder) -> impl Future<Output = Result<UpdateBackendJobOutput, SdkError<UpdateBackendJobError>>> {
        (*self).update_backend_job(builder)
    }
    fn update_backend_storage(&self, builder: UpdateBackendStorageInputBuilder) -> impl Future<Output = Result<UpdateBackendStorageOutput, SdkError<UpdateBackendStorageError>>> {
        (*self).update_backend_storage(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAmplifyBackendClient {}
    impl AmplifyBackendClient for edAmplifyBackendClient {
        async fn clone_backend(&self, builder: CloneBackendInputBuilder) -> Result<CloneBackendOutput, SdkError<CloneBackendError>>;
        async fn create_backend(&self, builder: CreateBackendInputBuilder) -> Result<CreateBackendOutput, SdkError<CreateBackendError>>;
        async fn create_backend_api(&self, builder: CreateBackendApiInputBuilder) -> Result<CreateBackendApiOutput, SdkError<CreateBackendAPIError>>;
        async fn create_backend_auth(&self, builder: CreateBackendAuthInputBuilder) -> Result<CreateBackendAuthOutput, SdkError<CreateBackendAuthError>>;
        async fn create_backend_config(&self, builder: CreateBackendConfigInputBuilder) -> Result<CreateBackendConfigOutput, SdkError<CreateBackendConfigError>>;
        async fn create_backend_storage(&self, builder: CreateBackendStorageInputBuilder) -> Result<CreateBackendStorageOutput, SdkError<CreateBackendStorageError>>;
        async fn create_token(&self, builder: CreateTokenInputBuilder) -> Result<CreateTokenOutput, SdkError<CreateTokenError>>;
        async fn delete_backend(&self, builder: DeleteBackendInputBuilder) -> Result<DeleteBackendOutput, SdkError<DeleteBackendError>>;
        async fn delete_backend_api(&self, builder: DeleteBackendApiInputBuilder) -> Result<DeleteBackendApiOutput, SdkError<DeleteBackendAPIError>>;
        async fn delete_backend_auth(&self, builder: DeleteBackendAuthInputBuilder) -> Result<DeleteBackendAuthOutput, SdkError<DeleteBackendAuthError>>;
        async fn delete_backend_storage(&self, builder: DeleteBackendStorageInputBuilder) -> Result<DeleteBackendStorageOutput, SdkError<DeleteBackendStorageError>>;
        async fn delete_token(&self, builder: DeleteTokenInputBuilder) -> Result<DeleteTokenOutput, SdkError<DeleteTokenError>>;
        async fn generate_backend_api_models(&self, builder: GenerateBackendApiModelsInputBuilder) -> Result<GenerateBackendApiModelsOutput, SdkError<GenerateBackendAPIModelsError>>;
        async fn get_backend(&self, builder: GetBackendInputBuilder) -> Result<GetBackendOutput, SdkError<GetBackendError>>;
        async fn get_backend_api(&self, builder: GetBackendApiInputBuilder) -> Result<GetBackendApiOutput, SdkError<GetBackendAPIError>>;
        async fn get_backend_api_models(&self, builder: GetBackendApiModelsInputBuilder) -> Result<GetBackendApiModelsOutput, SdkError<GetBackendAPIModelsError>>;
        async fn get_backend_auth(&self, builder: GetBackendAuthInputBuilder) -> Result<GetBackendAuthOutput, SdkError<GetBackendAuthError>>;
        async fn get_backend_job(&self, builder: GetBackendJobInputBuilder) -> Result<GetBackendJobOutput, SdkError<GetBackendJobError>>;
        async fn get_backend_storage(&self, builder: GetBackendStorageInputBuilder) -> Result<GetBackendStorageOutput, SdkError<GetBackendStorageError>>;
        async fn get_token(&self, builder: GetTokenInputBuilder) -> Result<GetTokenOutput, SdkError<GetTokenError>>;
        async fn import_backend_auth(&self, builder: ImportBackendAuthInputBuilder) -> Result<ImportBackendAuthOutput, SdkError<ImportBackendAuthError>>;
        async fn import_backend_storage(&self, builder: ImportBackendStorageInputBuilder) -> Result<ImportBackendStorageOutput, SdkError<ImportBackendStorageError>>;
        async fn list_backend_jobs(&self, builder: ListBackendJobsInputBuilder) -> Result<ListBackendJobsOutput, SdkError<ListBackendJobsError>>;
        async fn list_s3_buckets(&self, builder: ListS3BucketsInputBuilder) -> Result<ListS3BucketsOutput, SdkError<ListS3BucketsError>>;
        async fn remove_all_backends(&self, builder: RemoveAllBackendsInputBuilder) -> Result<RemoveAllBackendsOutput, SdkError<RemoveAllBackendsError>>;
        async fn remove_backend_config(&self, builder: RemoveBackendConfigInputBuilder) -> Result<RemoveBackendConfigOutput, SdkError<RemoveBackendConfigError>>;
        async fn update_backend_api(&self, builder: UpdateBackendApiInputBuilder) -> Result<UpdateBackendApiOutput, SdkError<UpdateBackendAPIError>>;
        async fn update_backend_auth(&self, builder: UpdateBackendAuthInputBuilder) -> Result<UpdateBackendAuthOutput, SdkError<UpdateBackendAuthError>>;
        async fn update_backend_config(&self, builder: UpdateBackendConfigInputBuilder) -> Result<UpdateBackendConfigOutput, SdkError<UpdateBackendConfigError>>;
        async fn update_backend_job(&self, builder: UpdateBackendJobInputBuilder) -> Result<UpdateBackendJobOutput, SdkError<UpdateBackendJobError>>;
        async fn update_backend_storage(&self, builder: UpdateBackendStorageInputBuilder) -> Result<UpdateBackendStorageOutput, SdkError<UpdateBackendStorageError>>;
    }
}
