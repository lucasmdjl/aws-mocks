/*
 * aws_mocks - A mocking library for AWS.
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
use aws_sdk_secretsmanager::operation::batch_get_secret_value::{builders::*, *};
use aws_sdk_secretsmanager::operation::cancel_rotate_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::create_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_secretsmanager::operation::delete_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::describe_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::get_random_password::{builders::*, *};
use aws_sdk_secretsmanager::operation::get_resource_policy::{builders::*, *};
use aws_sdk_secretsmanager::operation::get_secret_value::{builders::*, *};
use aws_sdk_secretsmanager::operation::list_secret_version_ids::{builders::*, *};
use aws_sdk_secretsmanager::operation::list_secrets::{builders::*, *};
use aws_sdk_secretsmanager::operation::put_resource_policy::{builders::*, *};
use aws_sdk_secretsmanager::operation::put_secret_value::{builders::*, *};
use aws_sdk_secretsmanager::operation::remove_regions_from_replication::{builders::*, *};
use aws_sdk_secretsmanager::operation::replicate_secret_to_regions::{builders::*, *};
use aws_sdk_secretsmanager::operation::restore_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::rotate_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::stop_replication_to_replica::{builders::*, *};
use aws_sdk_secretsmanager::operation::tag_resource::{builders::*, *};
use aws_sdk_secretsmanager::operation::untag_resource::{builders::*, *};
use aws_sdk_secretsmanager::operation::update_secret::{builders::*, *};
use aws_sdk_secretsmanager::operation::update_secret_version_stage::{builders::*, *};
use aws_sdk_secretsmanager::operation::validate_resource_policy::{builders::*, *};
use aws_sdk_secretsmanager::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_secretsmanager::Client;

pub use aws_sdk_secretsmanager::*;

pub struct SecretsManagerClientImpl(Client);
impl SecretsManagerClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait SecretsManagerClient {
    fn batch_get_secret_value(&self, builder: BatchGetSecretValueInputBuilder) -> impl Future<Output = Result<BatchGetSecretValueOutput, SdkError<BatchGetSecretValueError>>>;
    fn cancel_rotate_secret(&self, builder: CancelRotateSecretInputBuilder) -> impl Future<Output = Result<CancelRotateSecretOutput, SdkError<CancelRotateSecretError>>>;
    fn create_secret(&self, builder: CreateSecretInputBuilder) -> impl Future<Output = Result<CreateSecretOutput, SdkError<CreateSecretError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_secret(&self, builder: DeleteSecretInputBuilder) -> impl Future<Output = Result<DeleteSecretOutput, SdkError<DeleteSecretError>>>;
    fn describe_secret(&self, builder: DescribeSecretInputBuilder) -> impl Future<Output = Result<DescribeSecretOutput, SdkError<DescribeSecretError>>>;
    fn get_random_password(&self, builder: GetRandomPasswordInputBuilder) -> impl Future<Output = Result<GetRandomPasswordOutput, SdkError<GetRandomPasswordError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn get_secret_value(&self, builder: GetSecretValueInputBuilder) -> impl Future<Output = Result<GetSecretValueOutput, SdkError<GetSecretValueError>>>;
    fn list_secret_version_ids(&self, builder: ListSecretVersionIdsInputBuilder) -> impl Future<Output = Result<ListSecretVersionIdsOutput, SdkError<ListSecretVersionIdsError>>>;
    fn list_secrets(&self, builder: ListSecretsInputBuilder) -> impl Future<Output = Result<ListSecretsOutput, SdkError<ListSecretsError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn put_secret_value(&self, builder: PutSecretValueInputBuilder) -> impl Future<Output = Result<PutSecretValueOutput, SdkError<PutSecretValueError>>>;
    fn remove_regions_from_replication(&self, builder: RemoveRegionsFromReplicationInputBuilder) -> impl Future<Output = Result<RemoveRegionsFromReplicationOutput, SdkError<RemoveRegionsFromReplicationError>>>;
    fn replicate_secret_to_regions(&self, builder: ReplicateSecretToRegionsInputBuilder) -> impl Future<Output = Result<ReplicateSecretToRegionsOutput, SdkError<ReplicateSecretToRegionsError>>>;
    fn restore_secret(&self, builder: RestoreSecretInputBuilder) -> impl Future<Output = Result<RestoreSecretOutput, SdkError<RestoreSecretError>>>;
    fn rotate_secret(&self, builder: RotateSecretInputBuilder) -> impl Future<Output = Result<RotateSecretOutput, SdkError<RotateSecretError>>>;
    fn stop_replication_to_replica(&self, builder: StopReplicationToReplicaInputBuilder) -> impl Future<Output = Result<StopReplicationToReplicaOutput, SdkError<StopReplicationToReplicaError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_secret(&self, builder: UpdateSecretInputBuilder) -> impl Future<Output = Result<UpdateSecretOutput, SdkError<UpdateSecretError>>>;
    fn update_secret_version_stage(&self, builder: UpdateSecretVersionStageInputBuilder) -> impl Future<Output = Result<UpdateSecretVersionStageOutput, SdkError<UpdateSecretVersionStageError>>>;
    fn validate_resource_policy(&self, builder: ValidateResourcePolicyInputBuilder) -> impl Future<Output = Result<ValidateResourcePolicyOutput, SdkError<ValidateResourcePolicyError>>>;
}
impl SecretsManagerClient for SecretsManagerClientImpl {
    fn batch_get_secret_value(&self, builder: BatchGetSecretValueInputBuilder) -> impl Future<Output = Result<BatchGetSecretValueOutput, SdkError<BatchGetSecretValueError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_rotate_secret(&self, builder: CancelRotateSecretInputBuilder) -> impl Future<Output = Result<CancelRotateSecretOutput, SdkError<CancelRotateSecretError>>> {
        builder.send_with(&self.0)
    }
    fn create_secret(&self, builder: CreateSecretInputBuilder) -> impl Future<Output = Result<CreateSecretOutput, SdkError<CreateSecretError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_secret(&self, builder: DeleteSecretInputBuilder) -> impl Future<Output = Result<DeleteSecretOutput, SdkError<DeleteSecretError>>> {
        builder.send_with(&self.0)
    }
    fn describe_secret(&self, builder: DescribeSecretInputBuilder) -> impl Future<Output = Result<DescribeSecretOutput, SdkError<DescribeSecretError>>> {
        builder.send_with(&self.0)
    }
    fn get_random_password(&self, builder: GetRandomPasswordInputBuilder) -> impl Future<Output = Result<GetRandomPasswordOutput, SdkError<GetRandomPasswordError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_secret_value(&self, builder: GetSecretValueInputBuilder) -> impl Future<Output = Result<GetSecretValueOutput, SdkError<GetSecretValueError>>> {
        builder.send_with(&self.0)
    }
    fn list_secret_version_ids(&self, builder: ListSecretVersionIdsInputBuilder) -> impl Future<Output = Result<ListSecretVersionIdsOutput, SdkError<ListSecretVersionIdsError>>> {
        builder.send_with(&self.0)
    }
    fn list_secrets(&self, builder: ListSecretsInputBuilder) -> impl Future<Output = Result<ListSecretsOutput, SdkError<ListSecretsError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_secret_value(&self, builder: PutSecretValueInputBuilder) -> impl Future<Output = Result<PutSecretValueOutput, SdkError<PutSecretValueError>>> {
        builder.send_with(&self.0)
    }
    fn remove_regions_from_replication(&self, builder: RemoveRegionsFromReplicationInputBuilder) -> impl Future<Output = Result<RemoveRegionsFromReplicationOutput, SdkError<RemoveRegionsFromReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn replicate_secret_to_regions(&self, builder: ReplicateSecretToRegionsInputBuilder) -> impl Future<Output = Result<ReplicateSecretToRegionsOutput, SdkError<ReplicateSecretToRegionsError>>> {
        builder.send_with(&self.0)
    }
    fn restore_secret(&self, builder: RestoreSecretInputBuilder) -> impl Future<Output = Result<RestoreSecretOutput, SdkError<RestoreSecretError>>> {
        builder.send_with(&self.0)
    }
    fn rotate_secret(&self, builder: RotateSecretInputBuilder) -> impl Future<Output = Result<RotateSecretOutput, SdkError<RotateSecretError>>> {
        builder.send_with(&self.0)
    }
    fn stop_replication_to_replica(&self, builder: StopReplicationToReplicaInputBuilder) -> impl Future<Output = Result<StopReplicationToReplicaOutput, SdkError<StopReplicationToReplicaError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_secret(&self, builder: UpdateSecretInputBuilder) -> impl Future<Output = Result<UpdateSecretOutput, SdkError<UpdateSecretError>>> {
        builder.send_with(&self.0)
    }
    fn update_secret_version_stage(&self, builder: UpdateSecretVersionStageInputBuilder) -> impl Future<Output = Result<UpdateSecretVersionStageOutput, SdkError<UpdateSecretVersionStageError>>> {
        builder.send_with(&self.0)
    }
    fn validate_resource_policy(&self, builder: ValidateResourcePolicyInputBuilder) -> impl Future<Output = Result<ValidateResourcePolicyOutput, SdkError<ValidateResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: SecretsManagerClient> SecretsManagerClient for &T {
    fn batch_get_secret_value(&self, builder: BatchGetSecretValueInputBuilder) -> impl Future<Output = Result<BatchGetSecretValueOutput, SdkError<BatchGetSecretValueError>>> {
        (*self).batch_get_secret_value(builder)
    }
    fn cancel_rotate_secret(&self, builder: CancelRotateSecretInputBuilder) -> impl Future<Output = Result<CancelRotateSecretOutput, SdkError<CancelRotateSecretError>>> {
        (*self).cancel_rotate_secret(builder)
    }
    fn create_secret(&self, builder: CreateSecretInputBuilder) -> impl Future<Output = Result<CreateSecretOutput, SdkError<CreateSecretError>>> {
        (*self).create_secret(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        (*self).delete_resource_policy(builder)
    }
    fn delete_secret(&self, builder: DeleteSecretInputBuilder) -> impl Future<Output = Result<DeleteSecretOutput, SdkError<DeleteSecretError>>> {
        (*self).delete_secret(builder)
    }
    fn describe_secret(&self, builder: DescribeSecretInputBuilder) -> impl Future<Output = Result<DescribeSecretOutput, SdkError<DescribeSecretError>>> {
        (*self).describe_secret(builder)
    }
    fn get_random_password(&self, builder: GetRandomPasswordInputBuilder) -> impl Future<Output = Result<GetRandomPasswordOutput, SdkError<GetRandomPasswordError>>> {
        (*self).get_random_password(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        (*self).get_resource_policy(builder)
    }
    fn get_secret_value(&self, builder: GetSecretValueInputBuilder) -> impl Future<Output = Result<GetSecretValueOutput, SdkError<GetSecretValueError>>> {
        (*self).get_secret_value(builder)
    }
    fn list_secret_version_ids(&self, builder: ListSecretVersionIdsInputBuilder) -> impl Future<Output = Result<ListSecretVersionIdsOutput, SdkError<ListSecretVersionIdsError>>> {
        (*self).list_secret_version_ids(builder)
    }
    fn list_secrets(&self, builder: ListSecretsInputBuilder) -> impl Future<Output = Result<ListSecretsOutput, SdkError<ListSecretsError>>> {
        (*self).list_secrets(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        (*self).put_resource_policy(builder)
    }
    fn put_secret_value(&self, builder: PutSecretValueInputBuilder) -> impl Future<Output = Result<PutSecretValueOutput, SdkError<PutSecretValueError>>> {
        (*self).put_secret_value(builder)
    }
    fn remove_regions_from_replication(&self, builder: RemoveRegionsFromReplicationInputBuilder) -> impl Future<Output = Result<RemoveRegionsFromReplicationOutput, SdkError<RemoveRegionsFromReplicationError>>> {
        (*self).remove_regions_from_replication(builder)
    }
    fn replicate_secret_to_regions(&self, builder: ReplicateSecretToRegionsInputBuilder) -> impl Future<Output = Result<ReplicateSecretToRegionsOutput, SdkError<ReplicateSecretToRegionsError>>> {
        (*self).replicate_secret_to_regions(builder)
    }
    fn restore_secret(&self, builder: RestoreSecretInputBuilder) -> impl Future<Output = Result<RestoreSecretOutput, SdkError<RestoreSecretError>>> {
        (*self).restore_secret(builder)
    }
    fn rotate_secret(&self, builder: RotateSecretInputBuilder) -> impl Future<Output = Result<RotateSecretOutput, SdkError<RotateSecretError>>> {
        (*self).rotate_secret(builder)
    }
    fn stop_replication_to_replica(&self, builder: StopReplicationToReplicaInputBuilder) -> impl Future<Output = Result<StopReplicationToReplicaOutput, SdkError<StopReplicationToReplicaError>>> {
        (*self).stop_replication_to_replica(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_secret(&self, builder: UpdateSecretInputBuilder) -> impl Future<Output = Result<UpdateSecretOutput, SdkError<UpdateSecretError>>> {
        (*self).update_secret(builder)
    }
    fn update_secret_version_stage(&self, builder: UpdateSecretVersionStageInputBuilder) -> impl Future<Output = Result<UpdateSecretVersionStageOutput, SdkError<UpdateSecretVersionStageError>>> {
        (*self).update_secret_version_stage(builder)
    }
    fn validate_resource_policy(&self, builder: ValidateResourcePolicyInputBuilder) -> impl Future<Output = Result<ValidateResourcePolicyOutput, SdkError<ValidateResourcePolicyError>>> {
        (*self).validate_resource_policy(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edSecretsManagerClient {}
    impl SecretsManagerClient for edSecretsManagerClient {
        async fn batch_get_secret_value(&self, builder: BatchGetSecretValueInputBuilder) -> Result<BatchGetSecretValueOutput, SdkError<BatchGetSecretValueError>>;
        async fn cancel_rotate_secret(&self, builder: CancelRotateSecretInputBuilder) -> Result<CancelRotateSecretOutput, SdkError<CancelRotateSecretError>>;
        async fn create_secret(&self, builder: CreateSecretInputBuilder) -> Result<CreateSecretOutput, SdkError<CreateSecretError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_secret(&self, builder: DeleteSecretInputBuilder) -> Result<DeleteSecretOutput, SdkError<DeleteSecretError>>;
        async fn describe_secret(&self, builder: DescribeSecretInputBuilder) -> Result<DescribeSecretOutput, SdkError<DescribeSecretError>>;
        async fn get_random_password(&self, builder: GetRandomPasswordInputBuilder) -> Result<GetRandomPasswordOutput, SdkError<GetRandomPasswordError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn get_secret_value(&self, builder: GetSecretValueInputBuilder) -> Result<GetSecretValueOutput, SdkError<GetSecretValueError>>;
        async fn list_secret_version_ids(&self, builder: ListSecretVersionIdsInputBuilder) -> Result<ListSecretVersionIdsOutput, SdkError<ListSecretVersionIdsError>>;
        async fn list_secrets(&self, builder: ListSecretsInputBuilder) -> Result<ListSecretsOutput, SdkError<ListSecretsError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn put_secret_value(&self, builder: PutSecretValueInputBuilder) -> Result<PutSecretValueOutput, SdkError<PutSecretValueError>>;
        async fn remove_regions_from_replication(&self, builder: RemoveRegionsFromReplicationInputBuilder) -> Result<RemoveRegionsFromReplicationOutput, SdkError<RemoveRegionsFromReplicationError>>;
        async fn replicate_secret_to_regions(&self, builder: ReplicateSecretToRegionsInputBuilder) -> Result<ReplicateSecretToRegionsOutput, SdkError<ReplicateSecretToRegionsError>>;
        async fn restore_secret(&self, builder: RestoreSecretInputBuilder) -> Result<RestoreSecretOutput, SdkError<RestoreSecretError>>;
        async fn rotate_secret(&self, builder: RotateSecretInputBuilder) -> Result<RotateSecretOutput, SdkError<RotateSecretError>>;
        async fn stop_replication_to_replica(&self, builder: StopReplicationToReplicaInputBuilder) -> Result<StopReplicationToReplicaOutput, SdkError<StopReplicationToReplicaError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_secret(&self, builder: UpdateSecretInputBuilder) -> Result<UpdateSecretOutput, SdkError<UpdateSecretError>>;
        async fn update_secret_version_stage(&self, builder: UpdateSecretVersionStageInputBuilder) -> Result<UpdateSecretVersionStageOutput, SdkError<UpdateSecretVersionStageError>>;
        async fn validate_resource_policy(&self, builder: ValidateResourcePolicyInputBuilder) -> Result<ValidateResourcePolicyOutput, SdkError<ValidateResourcePolicyError>>;
    }
}
