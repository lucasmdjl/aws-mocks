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
use aws_sdk_efs::operation::create_access_point::{builders::*, *};
use aws_sdk_efs::operation::create_file_system::{builders::*, *};
use aws_sdk_efs::operation::create_mount_target::{builders::*, *};
use aws_sdk_efs::operation::create_replication_configuration::{builders::*, *};
use aws_sdk_efs::operation::delete_access_point::{builders::*, *};
use aws_sdk_efs::operation::delete_file_system::{builders::*, *};
use aws_sdk_efs::operation::delete_file_system_policy::{builders::*, *};
use aws_sdk_efs::operation::delete_mount_target::{builders::*, *};
use aws_sdk_efs::operation::delete_replication_configuration::{builders::*, *};
use aws_sdk_efs::operation::describe_access_points::{builders::*, *};
use aws_sdk_efs::operation::describe_account_preferences::{builders::*, *};
use aws_sdk_efs::operation::describe_backup_policy::{builders::*, *};
use aws_sdk_efs::operation::describe_file_system_policy::{builders::*, *};
use aws_sdk_efs::operation::describe_file_systems::{builders::*, *};
use aws_sdk_efs::operation::describe_lifecycle_configuration::{builders::*, *};
use aws_sdk_efs::operation::describe_mount_target_security_groups::{builders::*, *};
use aws_sdk_efs::operation::describe_mount_targets::{builders::*, *};
use aws_sdk_efs::operation::describe_replication_configurations::{builders::*, *};
use aws_sdk_efs::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_efs::operation::modify_mount_target_security_groups::{builders::*, *};
use aws_sdk_efs::operation::put_account_preferences::{builders::*, *};
use aws_sdk_efs::operation::put_backup_policy::{builders::*, *};
use aws_sdk_efs::operation::put_file_system_policy::{builders::*, *};
use aws_sdk_efs::operation::put_lifecycle_configuration::{builders::*, *};
use aws_sdk_efs::operation::tag_resource::{builders::*, *};
use aws_sdk_efs::operation::untag_resource::{builders::*, *};
use aws_sdk_efs::operation::update_file_system::{builders::*, *};
use aws_sdk_efs::operation::update_file_system_protection::{builders::*, *};
use aws_sdk_efs::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_efs::Client;

pub use aws_sdk_efs::*;

pub struct EFSClientImpl(Client);
impl EFSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait EFSClient {
    fn create_access_point(&self, builder: CreateAccessPointInputBuilder) -> impl Future<Output = Result<CreateAccessPointOutput, SdkError<CreateAccessPointError>>>;
    fn create_file_system(&self, builder: CreateFileSystemInputBuilder) -> impl Future<Output = Result<CreateFileSystemOutput, SdkError<CreateFileSystemError>>>;
    fn create_mount_target(&self, builder: CreateMountTargetInputBuilder) -> impl Future<Output = Result<CreateMountTargetOutput, SdkError<CreateMountTargetError>>>;
    fn create_replication_configuration(&self, builder: CreateReplicationConfigurationInputBuilder) -> impl Future<Output = Result<CreateReplicationConfigurationOutput, SdkError<CreateReplicationConfigurationError>>>;
    fn delete_access_point(&self, builder: DeleteAccessPointInputBuilder) -> impl Future<Output = Result<DeleteAccessPointOutput, SdkError<DeleteAccessPointError>>>;
    fn delete_file_system(&self, builder: DeleteFileSystemInputBuilder) -> impl Future<Output = Result<DeleteFileSystemOutput, SdkError<DeleteFileSystemError>>>;
    fn delete_file_system_policy(&self, builder: DeleteFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DeleteFileSystemPolicyOutput, SdkError<DeleteFileSystemPolicyError>>>;
    fn delete_mount_target(&self, builder: DeleteMountTargetInputBuilder) -> impl Future<Output = Result<DeleteMountTargetOutput, SdkError<DeleteMountTargetError>>>;
    fn delete_replication_configuration(&self, builder: DeleteReplicationConfigurationInputBuilder) -> impl Future<Output = Result<DeleteReplicationConfigurationOutput, SdkError<DeleteReplicationConfigurationError>>>;
    fn describe_access_points(&self, builder: DescribeAccessPointsInputBuilder) -> impl Future<Output = Result<DescribeAccessPointsOutput, SdkError<DescribeAccessPointsError>>>;
    fn describe_account_preferences(&self, builder: DescribeAccountPreferencesInputBuilder) -> impl Future<Output = Result<DescribeAccountPreferencesOutput, SdkError<DescribeAccountPreferencesError>>>;
    fn describe_backup_policy(&self, builder: DescribeBackupPolicyInputBuilder) -> impl Future<Output = Result<DescribeBackupPolicyOutput, SdkError<DescribeBackupPolicyError>>>;
    fn describe_file_system_policy(&self, builder: DescribeFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DescribeFileSystemPolicyOutput, SdkError<DescribeFileSystemPolicyError>>>;
    fn describe_file_systems(&self, builder: DescribeFileSystemsInputBuilder) -> impl Future<Output = Result<DescribeFileSystemsOutput, SdkError<DescribeFileSystemsError>>>;
    fn describe_lifecycle_configuration(&self, builder: DescribeLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLifecycleConfigurationOutput, SdkError<DescribeLifecycleConfigurationError>>>;
    fn describe_mount_target_security_groups(&self, builder: DescribeMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetSecurityGroupsOutput, SdkError<DescribeMountTargetSecurityGroupsError>>>;
    fn describe_mount_targets(&self, builder: DescribeMountTargetsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetsOutput, SdkError<DescribeMountTargetsError>>>;
    fn describe_replication_configurations(&self, builder: DescribeReplicationConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeReplicationConfigurationsOutput, SdkError<DescribeReplicationConfigurationsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn modify_mount_target_security_groups(&self, builder: ModifyMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<ModifyMountTargetSecurityGroupsOutput, SdkError<ModifyMountTargetSecurityGroupsError>>>;
    fn put_account_preferences(&self, builder: PutAccountPreferencesInputBuilder) -> impl Future<Output = Result<PutAccountPreferencesOutput, SdkError<PutAccountPreferencesError>>>;
    fn put_backup_policy(&self, builder: PutBackupPolicyInputBuilder) -> impl Future<Output = Result<PutBackupPolicyOutput, SdkError<PutBackupPolicyError>>>;
    fn put_file_system_policy(&self, builder: PutFileSystemPolicyInputBuilder) -> impl Future<Output = Result<PutFileSystemPolicyOutput, SdkError<PutFileSystemPolicyError>>>;
    fn put_lifecycle_configuration(&self, builder: PutLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutLifecycleConfigurationOutput, SdkError<PutLifecycleConfigurationError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_file_system(&self, builder: UpdateFileSystemInputBuilder) -> impl Future<Output = Result<UpdateFileSystemOutput, SdkError<UpdateFileSystemError>>>;
    fn update_file_system_protection(&self, builder: UpdateFileSystemProtectionInputBuilder) -> impl Future<Output = Result<UpdateFileSystemProtectionOutput, SdkError<UpdateFileSystemProtectionError>>>;
}
impl EFSClient for EFSClientImpl {
    fn create_access_point(&self, builder: CreateAccessPointInputBuilder) -> impl Future<Output = Result<CreateAccessPointOutput, SdkError<CreateAccessPointError>>> {
        builder.send_with(&self.0)
    }
    fn create_file_system(&self, builder: CreateFileSystemInputBuilder) -> impl Future<Output = Result<CreateFileSystemOutput, SdkError<CreateFileSystemError>>> {
        builder.send_with(&self.0)
    }
    fn create_mount_target(&self, builder: CreateMountTargetInputBuilder) -> impl Future<Output = Result<CreateMountTargetOutput, SdkError<CreateMountTargetError>>> {
        builder.send_with(&self.0)
    }
    fn create_replication_configuration(&self, builder: CreateReplicationConfigurationInputBuilder) -> impl Future<Output = Result<CreateReplicationConfigurationOutput, SdkError<CreateReplicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_access_point(&self, builder: DeleteAccessPointInputBuilder) -> impl Future<Output = Result<DeleteAccessPointOutput, SdkError<DeleteAccessPointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_file_system(&self, builder: DeleteFileSystemInputBuilder) -> impl Future<Output = Result<DeleteFileSystemOutput, SdkError<DeleteFileSystemError>>> {
        builder.send_with(&self.0)
    }
    fn delete_file_system_policy(&self, builder: DeleteFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DeleteFileSystemPolicyOutput, SdkError<DeleteFileSystemPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_mount_target(&self, builder: DeleteMountTargetInputBuilder) -> impl Future<Output = Result<DeleteMountTargetOutput, SdkError<DeleteMountTargetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_replication_configuration(&self, builder: DeleteReplicationConfigurationInputBuilder) -> impl Future<Output = Result<DeleteReplicationConfigurationOutput, SdkError<DeleteReplicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_access_points(&self, builder: DescribeAccessPointsInputBuilder) -> impl Future<Output = Result<DescribeAccessPointsOutput, SdkError<DescribeAccessPointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_preferences(&self, builder: DescribeAccountPreferencesInputBuilder) -> impl Future<Output = Result<DescribeAccountPreferencesOutput, SdkError<DescribeAccountPreferencesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_backup_policy(&self, builder: DescribeBackupPolicyInputBuilder) -> impl Future<Output = Result<DescribeBackupPolicyOutput, SdkError<DescribeBackupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn describe_file_system_policy(&self, builder: DescribeFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DescribeFileSystemPolicyOutput, SdkError<DescribeFileSystemPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn describe_file_systems(&self, builder: DescribeFileSystemsInputBuilder) -> impl Future<Output = Result<DescribeFileSystemsOutput, SdkError<DescribeFileSystemsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_lifecycle_configuration(&self, builder: DescribeLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLifecycleConfigurationOutput, SdkError<DescribeLifecycleConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_mount_target_security_groups(&self, builder: DescribeMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetSecurityGroupsOutput, SdkError<DescribeMountTargetSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_mount_targets(&self, builder: DescribeMountTargetsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetsOutput, SdkError<DescribeMountTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_replication_configurations(&self, builder: DescribeReplicationConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeReplicationConfigurationsOutput, SdkError<DescribeReplicationConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_mount_target_security_groups(&self, builder: ModifyMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<ModifyMountTargetSecurityGroupsOutput, SdkError<ModifyMountTargetSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn put_account_preferences(&self, builder: PutAccountPreferencesInputBuilder) -> impl Future<Output = Result<PutAccountPreferencesOutput, SdkError<PutAccountPreferencesError>>> {
        builder.send_with(&self.0)
    }
    fn put_backup_policy(&self, builder: PutBackupPolicyInputBuilder) -> impl Future<Output = Result<PutBackupPolicyOutput, SdkError<PutBackupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_file_system_policy(&self, builder: PutFileSystemPolicyInputBuilder) -> impl Future<Output = Result<PutFileSystemPolicyOutput, SdkError<PutFileSystemPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_lifecycle_configuration(&self, builder: PutLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutLifecycleConfigurationOutput, SdkError<PutLifecycleConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_file_system(&self, builder: UpdateFileSystemInputBuilder) -> impl Future<Output = Result<UpdateFileSystemOutput, SdkError<UpdateFileSystemError>>> {
        builder.send_with(&self.0)
    }
    fn update_file_system_protection(&self, builder: UpdateFileSystemProtectionInputBuilder) -> impl Future<Output = Result<UpdateFileSystemProtectionOutput, SdkError<UpdateFileSystemProtectionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: EFSClient> EFSClient for &T {
    fn create_access_point(&self, builder: CreateAccessPointInputBuilder) -> impl Future<Output = Result<CreateAccessPointOutput, SdkError<CreateAccessPointError>>> {
        (*self).create_access_point(builder)
    }
    fn create_file_system(&self, builder: CreateFileSystemInputBuilder) -> impl Future<Output = Result<CreateFileSystemOutput, SdkError<CreateFileSystemError>>> {
        (*self).create_file_system(builder)
    }
    fn create_mount_target(&self, builder: CreateMountTargetInputBuilder) -> impl Future<Output = Result<CreateMountTargetOutput, SdkError<CreateMountTargetError>>> {
        (*self).create_mount_target(builder)
    }
    fn create_replication_configuration(&self, builder: CreateReplicationConfigurationInputBuilder) -> impl Future<Output = Result<CreateReplicationConfigurationOutput, SdkError<CreateReplicationConfigurationError>>> {
        (*self).create_replication_configuration(builder)
    }
    fn delete_access_point(&self, builder: DeleteAccessPointInputBuilder) -> impl Future<Output = Result<DeleteAccessPointOutput, SdkError<DeleteAccessPointError>>> {
        (*self).delete_access_point(builder)
    }
    fn delete_file_system(&self, builder: DeleteFileSystemInputBuilder) -> impl Future<Output = Result<DeleteFileSystemOutput, SdkError<DeleteFileSystemError>>> {
        (*self).delete_file_system(builder)
    }
    fn delete_file_system_policy(&self, builder: DeleteFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DeleteFileSystemPolicyOutput, SdkError<DeleteFileSystemPolicyError>>> {
        (*self).delete_file_system_policy(builder)
    }
    fn delete_mount_target(&self, builder: DeleteMountTargetInputBuilder) -> impl Future<Output = Result<DeleteMountTargetOutput, SdkError<DeleteMountTargetError>>> {
        (*self).delete_mount_target(builder)
    }
    fn delete_replication_configuration(&self, builder: DeleteReplicationConfigurationInputBuilder) -> impl Future<Output = Result<DeleteReplicationConfigurationOutput, SdkError<DeleteReplicationConfigurationError>>> {
        (*self).delete_replication_configuration(builder)
    }
    fn describe_access_points(&self, builder: DescribeAccessPointsInputBuilder) -> impl Future<Output = Result<DescribeAccessPointsOutput, SdkError<DescribeAccessPointsError>>> {
        (*self).describe_access_points(builder)
    }
    fn describe_account_preferences(&self, builder: DescribeAccountPreferencesInputBuilder) -> impl Future<Output = Result<DescribeAccountPreferencesOutput, SdkError<DescribeAccountPreferencesError>>> {
        (*self).describe_account_preferences(builder)
    }
    fn describe_backup_policy(&self, builder: DescribeBackupPolicyInputBuilder) -> impl Future<Output = Result<DescribeBackupPolicyOutput, SdkError<DescribeBackupPolicyError>>> {
        (*self).describe_backup_policy(builder)
    }
    fn describe_file_system_policy(&self, builder: DescribeFileSystemPolicyInputBuilder) -> impl Future<Output = Result<DescribeFileSystemPolicyOutput, SdkError<DescribeFileSystemPolicyError>>> {
        (*self).describe_file_system_policy(builder)
    }
    fn describe_file_systems(&self, builder: DescribeFileSystemsInputBuilder) -> impl Future<Output = Result<DescribeFileSystemsOutput, SdkError<DescribeFileSystemsError>>> {
        (*self).describe_file_systems(builder)
    }
    fn describe_lifecycle_configuration(&self, builder: DescribeLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<DescribeLifecycleConfigurationOutput, SdkError<DescribeLifecycleConfigurationError>>> {
        (*self).describe_lifecycle_configuration(builder)
    }
    fn describe_mount_target_security_groups(&self, builder: DescribeMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetSecurityGroupsOutput, SdkError<DescribeMountTargetSecurityGroupsError>>> {
        (*self).describe_mount_target_security_groups(builder)
    }
    fn describe_mount_targets(&self, builder: DescribeMountTargetsInputBuilder) -> impl Future<Output = Result<DescribeMountTargetsOutput, SdkError<DescribeMountTargetsError>>> {
        (*self).describe_mount_targets(builder)
    }
    fn describe_replication_configurations(&self, builder: DescribeReplicationConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeReplicationConfigurationsOutput, SdkError<DescribeReplicationConfigurationsError>>> {
        (*self).describe_replication_configurations(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn modify_mount_target_security_groups(&self, builder: ModifyMountTargetSecurityGroupsInputBuilder) -> impl Future<Output = Result<ModifyMountTargetSecurityGroupsOutput, SdkError<ModifyMountTargetSecurityGroupsError>>> {
        (*self).modify_mount_target_security_groups(builder)
    }
    fn put_account_preferences(&self, builder: PutAccountPreferencesInputBuilder) -> impl Future<Output = Result<PutAccountPreferencesOutput, SdkError<PutAccountPreferencesError>>> {
        (*self).put_account_preferences(builder)
    }
    fn put_backup_policy(&self, builder: PutBackupPolicyInputBuilder) -> impl Future<Output = Result<PutBackupPolicyOutput, SdkError<PutBackupPolicyError>>> {
        (*self).put_backup_policy(builder)
    }
    fn put_file_system_policy(&self, builder: PutFileSystemPolicyInputBuilder) -> impl Future<Output = Result<PutFileSystemPolicyOutput, SdkError<PutFileSystemPolicyError>>> {
        (*self).put_file_system_policy(builder)
    }
    fn put_lifecycle_configuration(&self, builder: PutLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutLifecycleConfigurationOutput, SdkError<PutLifecycleConfigurationError>>> {
        (*self).put_lifecycle_configuration(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_file_system(&self, builder: UpdateFileSystemInputBuilder) -> impl Future<Output = Result<UpdateFileSystemOutput, SdkError<UpdateFileSystemError>>> {
        (*self).update_file_system(builder)
    }
    fn update_file_system_protection(&self, builder: UpdateFileSystemProtectionInputBuilder) -> impl Future<Output = Result<UpdateFileSystemProtectionOutput, SdkError<UpdateFileSystemProtectionError>>> {
        (*self).update_file_system_protection(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edEFSClient {}
    impl EFSClient for edEFSClient {
        async fn create_access_point(&self, builder: CreateAccessPointInputBuilder) -> Result<CreateAccessPointOutput, SdkError<CreateAccessPointError>>;
        async fn create_file_system(&self, builder: CreateFileSystemInputBuilder) -> Result<CreateFileSystemOutput, SdkError<CreateFileSystemError>>;
        async fn create_mount_target(&self, builder: CreateMountTargetInputBuilder) -> Result<CreateMountTargetOutput, SdkError<CreateMountTargetError>>;
        async fn create_replication_configuration(&self, builder: CreateReplicationConfigurationInputBuilder) -> Result<CreateReplicationConfigurationOutput, SdkError<CreateReplicationConfigurationError>>;
        async fn delete_access_point(&self, builder: DeleteAccessPointInputBuilder) -> Result<DeleteAccessPointOutput, SdkError<DeleteAccessPointError>>;
        async fn delete_file_system(&self, builder: DeleteFileSystemInputBuilder) -> Result<DeleteFileSystemOutput, SdkError<DeleteFileSystemError>>;
        async fn delete_file_system_policy(&self, builder: DeleteFileSystemPolicyInputBuilder) -> Result<DeleteFileSystemPolicyOutput, SdkError<DeleteFileSystemPolicyError>>;
        async fn delete_mount_target(&self, builder: DeleteMountTargetInputBuilder) -> Result<DeleteMountTargetOutput, SdkError<DeleteMountTargetError>>;
        async fn delete_replication_configuration(&self, builder: DeleteReplicationConfigurationInputBuilder) -> Result<DeleteReplicationConfigurationOutput, SdkError<DeleteReplicationConfigurationError>>;
        async fn describe_access_points(&self, builder: DescribeAccessPointsInputBuilder) -> Result<DescribeAccessPointsOutput, SdkError<DescribeAccessPointsError>>;
        async fn describe_account_preferences(&self, builder: DescribeAccountPreferencesInputBuilder) -> Result<DescribeAccountPreferencesOutput, SdkError<DescribeAccountPreferencesError>>;
        async fn describe_backup_policy(&self, builder: DescribeBackupPolicyInputBuilder) -> Result<DescribeBackupPolicyOutput, SdkError<DescribeBackupPolicyError>>;
        async fn describe_file_system_policy(&self, builder: DescribeFileSystemPolicyInputBuilder) -> Result<DescribeFileSystemPolicyOutput, SdkError<DescribeFileSystemPolicyError>>;
        async fn describe_file_systems(&self, builder: DescribeFileSystemsInputBuilder) -> Result<DescribeFileSystemsOutput, SdkError<DescribeFileSystemsError>>;
        async fn describe_lifecycle_configuration(&self, builder: DescribeLifecycleConfigurationInputBuilder) -> Result<DescribeLifecycleConfigurationOutput, SdkError<DescribeLifecycleConfigurationError>>;
        async fn describe_mount_target_security_groups(&self, builder: DescribeMountTargetSecurityGroupsInputBuilder) -> Result<DescribeMountTargetSecurityGroupsOutput, SdkError<DescribeMountTargetSecurityGroupsError>>;
        async fn describe_mount_targets(&self, builder: DescribeMountTargetsInputBuilder) -> Result<DescribeMountTargetsOutput, SdkError<DescribeMountTargetsError>>;
        async fn describe_replication_configurations(&self, builder: DescribeReplicationConfigurationsInputBuilder) -> Result<DescribeReplicationConfigurationsOutput, SdkError<DescribeReplicationConfigurationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn modify_mount_target_security_groups(&self, builder: ModifyMountTargetSecurityGroupsInputBuilder) -> Result<ModifyMountTargetSecurityGroupsOutput, SdkError<ModifyMountTargetSecurityGroupsError>>;
        async fn put_account_preferences(&self, builder: PutAccountPreferencesInputBuilder) -> Result<PutAccountPreferencesOutput, SdkError<PutAccountPreferencesError>>;
        async fn put_backup_policy(&self, builder: PutBackupPolicyInputBuilder) -> Result<PutBackupPolicyOutput, SdkError<PutBackupPolicyError>>;
        async fn put_file_system_policy(&self, builder: PutFileSystemPolicyInputBuilder) -> Result<PutFileSystemPolicyOutput, SdkError<PutFileSystemPolicyError>>;
        async fn put_lifecycle_configuration(&self, builder: PutLifecycleConfigurationInputBuilder) -> Result<PutLifecycleConfigurationOutput, SdkError<PutLifecycleConfigurationError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_file_system(&self, builder: UpdateFileSystemInputBuilder) -> Result<UpdateFileSystemOutput, SdkError<UpdateFileSystemError>>;
        async fn update_file_system_protection(&self, builder: UpdateFileSystemProtectionInputBuilder) -> Result<UpdateFileSystemProtectionOutput, SdkError<UpdateFileSystemProtectionError>>;
    }
}
