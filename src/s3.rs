/*
 * aws-mocks - A mocking library for AWS.
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
use aws_sdk_s3::operation::abort_multipart_upload::{builders::*, *};
use aws_sdk_s3::operation::complete_multipart_upload::{builders::*, *};
use aws_sdk_s3::operation::copy_object::{builders::*, *};
use aws_sdk_s3::operation::create_bucket::{builders::*, *};
use aws_sdk_s3::operation::create_multipart_upload::{builders::*, *};
use aws_sdk_s3::operation::create_session::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_analytics_configuration::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_cors::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_encryption::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_intelligent_tiering_configuration::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_inventory_configuration::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_lifecycle::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_metrics_configuration::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_ownership_controls::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_policy::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_replication::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_tagging::{builders::*, *};
use aws_sdk_s3::operation::delete_bucket_website::{builders::*, *};
use aws_sdk_s3::operation::delete_object::{builders::*, *};
use aws_sdk_s3::operation::delete_object_tagging::{builders::*, *};
use aws_sdk_s3::operation::delete_objects::{builders::*, *};
use aws_sdk_s3::operation::delete_public_access_block::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_accelerate_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_acl::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_analytics_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_cors::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_encryption::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_intelligent_tiering_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_inventory_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_lifecycle_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_location::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_logging::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_metrics_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_notification_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_ownership_controls::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_policy::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_policy_status::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_replication::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_request_payment::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_tagging::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_versioning::{builders::*, *};
use aws_sdk_s3::operation::get_bucket_website::{builders::*, *};
use aws_sdk_s3::operation::get_object::{builders::*, *};
use aws_sdk_s3::operation::get_object_acl::{builders::*, *};
use aws_sdk_s3::operation::get_object_attributes::{builders::*, *};
use aws_sdk_s3::operation::get_object_legal_hold::{builders::*, *};
use aws_sdk_s3::operation::get_object_lock_configuration::{builders::*, *};
use aws_sdk_s3::operation::get_object_retention::{builders::*, *};
use aws_sdk_s3::operation::get_object_tagging::{builders::*, *};
use aws_sdk_s3::operation::get_object_torrent::{builders::*, *};
use aws_sdk_s3::operation::get_public_access_block::{builders::*, *};
use aws_sdk_s3::operation::head_bucket::{builders::*, *};
use aws_sdk_s3::operation::head_object::{builders::*, *};
use aws_sdk_s3::operation::list_bucket_analytics_configurations::{builders::*, *};
use aws_sdk_s3::operation::list_bucket_intelligent_tiering_configurations::{builders::*, *};
use aws_sdk_s3::operation::list_bucket_inventory_configurations::{builders::*, *};
use aws_sdk_s3::operation::list_bucket_metrics_configurations::{builders::*, *};
use aws_sdk_s3::operation::list_buckets::{builders::*, *};
use aws_sdk_s3::operation::list_directory_buckets::{builders::*, *};
use aws_sdk_s3::operation::list_multipart_uploads::{builders::*, *};
use aws_sdk_s3::operation::list_object_versions::{builders::*, *};
use aws_sdk_s3::operation::list_objects::{builders::*, *};
use aws_sdk_s3::operation::list_objects_v2::{builders::*, *};
use aws_sdk_s3::operation::list_parts::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_accelerate_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_acl::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_analytics_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_cors::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_encryption::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_intelligent_tiering_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_inventory_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_lifecycle_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_logging::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_metrics_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_notification_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_ownership_controls::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_policy::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_replication::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_request_payment::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_tagging::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_versioning::{builders::*, *};
use aws_sdk_s3::operation::put_bucket_website::{builders::*, *};
use aws_sdk_s3::operation::put_object::{builders::*, *};
use aws_sdk_s3::operation::put_object_acl::{builders::*, *};
use aws_sdk_s3::operation::put_object_legal_hold::{builders::*, *};
use aws_sdk_s3::operation::put_object_lock_configuration::{builders::*, *};
use aws_sdk_s3::operation::put_object_retention::{builders::*, *};
use aws_sdk_s3::operation::put_object_tagging::{builders::*, *};
use aws_sdk_s3::operation::put_public_access_block::{builders::*, *};
use aws_sdk_s3::operation::restore_object::{builders::*, *};
use aws_sdk_s3::operation::select_object_content::{builders::*, *};
use aws_sdk_s3::operation::upload_part::{builders::*, *};
use aws_sdk_s3::operation::upload_part_copy::{builders::*, *};
use aws_sdk_s3::operation::write_get_object_response::{builders::*, *};
use aws_sdk_s3::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_s3::Client;
use std::ops::Deref;

pub use aws_sdk_s3::*;

pub struct S3ClientImpl(Client);
impl S3ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait S3Client {
    fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> impl Future<Output = Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>> + Send;
    fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> impl Future<Output = Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>> + Send;
    fn copy_object(&self, builder: CopyObjectInputBuilder) -> impl Future<Output = Result<CopyObjectOutput, SdkError<CopyObjectError>>> + Send;
    fn create_bucket(&self, builder: CreateBucketInputBuilder) -> impl Future<Output = Result<CreateBucketOutput, SdkError<CreateBucketError>>> + Send;
    fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> impl Future<Output = Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>> + Send;
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> + Send;
    fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> impl Future<Output = Result<DeleteBucketOutput, SdkError<DeleteBucketError>>> + Send;
    fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>> + Send;
    fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> impl Future<Output = Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>> + Send;
    fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> impl Future<Output = Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>> + Send;
    fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>> + Send;
    fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>> + Send;
    fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> impl Future<Output = Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>> + Send;
    fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>> + Send;
    fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>> + Send;
    fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> impl Future<Output = Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>> + Send;
    fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> impl Future<Output = Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>> + Send;
    fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> impl Future<Output = Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>> + Send;
    fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> impl Future<Output = Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>> + Send;
    fn delete_object(&self, builder: DeleteObjectInputBuilder) -> impl Future<Output = Result<DeleteObjectOutput, SdkError<DeleteObjectError>>> + Send;
    fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> impl Future<Output = Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>> + Send;
    fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> impl Future<Output = Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>> + Send;
    fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> impl Future<Output = Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>> + Send;
    fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>> + Send;
    fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> impl Future<Output = Result<GetBucketAclOutput, SdkError<GetBucketAclError>>> + Send;
    fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>> + Send;
    fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> impl Future<Output = Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>> + Send;
    fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> impl Future<Output = Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>> + Send;
    fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>> + Send;
    fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>> + Send;
    fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>> + Send;
    fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> impl Future<Output = Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>> + Send;
    fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> impl Future<Output = Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>> + Send;
    fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>> + Send;
    fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>> + Send;
    fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>> + Send;
    fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> impl Future<Output = Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>> + Send;
    fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> impl Future<Output = Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>> + Send;
    fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> impl Future<Output = Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>> + Send;
    fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>> + Send;
    fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> impl Future<Output = Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>> + Send;
    fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> impl Future<Output = Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>> + Send;
    fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> impl Future<Output = Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>> + Send;
    fn get_object(&self, builder: GetObjectInputBuilder) -> impl Future<Output = Result<GetObjectOutput, SdkError<GetObjectError>>> + Send;
    fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> impl Future<Output = Result<GetObjectAclOutput, SdkError<GetObjectAclError>>> + Send;
    fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> impl Future<Output = Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>> + Send;
    fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> impl Future<Output = Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>> + Send;
    fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>> + Send;
    fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> impl Future<Output = Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>> + Send;
    fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> impl Future<Output = Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>> + Send;
    fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> impl Future<Output = Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>> + Send;
    fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> impl Future<Output = Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>> + Send;
    fn head_bucket(&self, builder: HeadBucketInputBuilder) -> impl Future<Output = Result<HeadBucketOutput, SdkError<HeadBucketError>>> + Send;
    fn head_object(&self, builder: HeadObjectInputBuilder) -> impl Future<Output = Result<HeadObjectOutput, SdkError<HeadObjectError>>> + Send;
    fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>> + Send;
    fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>> + Send;
    fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>> + Send;
    fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>> + Send;
    fn list_buckets(&self, builder: ListBucketsInputBuilder) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>> + Send;
    fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> impl Future<Output = Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>> + Send;
    fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> impl Future<Output = Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>> + Send;
    fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> impl Future<Output = Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>> + Send;
    fn list_objects(&self, builder: ListObjectsInputBuilder) -> impl Future<Output = Result<ListObjectsOutput, SdkError<ListObjectsError>>> + Send;
    fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> impl Future<Output = Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>> + Send;
    fn list_parts(&self, builder: ListPartsInputBuilder) -> impl Future<Output = Result<ListPartsOutput, SdkError<ListPartsError>>> + Send;
    fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>> + Send;
    fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> impl Future<Output = Result<PutBucketAclOutput, SdkError<PutBucketAclError>>> + Send;
    fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>> + Send;
    fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> impl Future<Output = Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>> + Send;
    fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> impl Future<Output = Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>> + Send;
    fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>> + Send;
    fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>> + Send;
    fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>> + Send;
    fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> impl Future<Output = Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>> + Send;
    fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>> + Send;
    fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>> + Send;
    fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>> + Send;
    fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> impl Future<Output = Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>> + Send;
    fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> impl Future<Output = Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>> + Send;
    fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>> + Send;
    fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> impl Future<Output = Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>> + Send;
    fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> impl Future<Output = Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>> + Send;
    fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> impl Future<Output = Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>> + Send;
    fn put_object(&self, builder: PutObjectInputBuilder) -> impl Future<Output = Result<PutObjectOutput, SdkError<PutObjectError>>> + Send;
    fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> impl Future<Output = Result<PutObjectAclOutput, SdkError<PutObjectAclError>>> + Send;
    fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> impl Future<Output = Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>> + Send;
    fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>> + Send;
    fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> impl Future<Output = Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>> + Send;
    fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> impl Future<Output = Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>> + Send;
    fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> impl Future<Output = Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>> + Send;
    fn restore_object(&self, builder: RestoreObjectInputBuilder) -> impl Future<Output = Result<RestoreObjectOutput, SdkError<RestoreObjectError>>> + Send;
    fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> impl Future<Output = Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>> + Send;
    fn upload_part(&self, builder: UploadPartInputBuilder) -> impl Future<Output = Result<UploadPartOutput, SdkError<UploadPartError>>> + Send;
    fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> impl Future<Output = Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>> + Send;
    fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> impl Future<Output = Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>> + Send;
}
impl S3Client for S3ClientImpl {
    fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> impl Future<Output = Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>> {
        builder.send_with(&self.0)
    }
    fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> impl Future<Output = Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>> {
        builder.send_with(&self.0)
    }
    fn copy_object(&self, builder: CopyObjectInputBuilder) -> impl Future<Output = Result<CopyObjectOutput, SdkError<CopyObjectError>>> {
        builder.send_with(&self.0)
    }
    fn create_bucket(&self, builder: CreateBucketInputBuilder) -> impl Future<Output = Result<CreateBucketOutput, SdkError<CreateBucketError>>> {
        builder.send_with(&self.0)
    }
    fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> impl Future<Output = Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>> {
        builder.send_with(&self.0)
    }
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> impl Future<Output = Result<DeleteBucketOutput, SdkError<DeleteBucketError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> impl Future<Output = Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> impl Future<Output = Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> impl Future<Output = Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> impl Future<Output = Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> impl Future<Output = Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> impl Future<Output = Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> impl Future<Output = Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_object(&self, builder: DeleteObjectInputBuilder) -> impl Future<Output = Result<DeleteObjectOutput, SdkError<DeleteObjectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> impl Future<Output = Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> impl Future<Output = Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> impl Future<Output = Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> impl Future<Output = Result<GetBucketAclOutput, SdkError<GetBucketAclError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> impl Future<Output = Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> impl Future<Output = Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> impl Future<Output = Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> impl Future<Output = Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> impl Future<Output = Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> impl Future<Output = Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> impl Future<Output = Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> impl Future<Output = Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> impl Future<Output = Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> impl Future<Output = Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>> {
        builder.send_with(&self.0)
    }
    fn get_object(&self, builder: GetObjectInputBuilder) -> impl Future<Output = Result<GetObjectOutput, SdkError<GetObjectError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> impl Future<Output = Result<GetObjectAclOutput, SdkError<GetObjectAclError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> impl Future<Output = Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> impl Future<Output = Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> impl Future<Output = Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> impl Future<Output = Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> impl Future<Output = Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>> {
        builder.send_with(&self.0)
    }
    fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> impl Future<Output = Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>> {
        builder.send_with(&self.0)
    }
    fn head_bucket(&self, builder: HeadBucketInputBuilder) -> impl Future<Output = Result<HeadBucketOutput, SdkError<HeadBucketError>>> {
        builder.send_with(&self.0)
    }
    fn head_object(&self, builder: HeadObjectInputBuilder) -> impl Future<Output = Result<HeadObjectOutput, SdkError<HeadObjectError>>> {
        builder.send_with(&self.0)
    }
    fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_buckets(&self, builder: ListBucketsInputBuilder) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>> {
        builder.send_with(&self.0)
    }
    fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> impl Future<Output = Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>> {
        builder.send_with(&self.0)
    }
    fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> impl Future<Output = Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>> {
        builder.send_with(&self.0)
    }
    fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> impl Future<Output = Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_objects(&self, builder: ListObjectsInputBuilder) -> impl Future<Output = Result<ListObjectsOutput, SdkError<ListObjectsError>>> {
        builder.send_with(&self.0)
    }
    fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> impl Future<Output = Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>> {
        builder.send_with(&self.0)
    }
    fn list_parts(&self, builder: ListPartsInputBuilder) -> impl Future<Output = Result<ListPartsOutput, SdkError<ListPartsError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> impl Future<Output = Result<PutBucketAclOutput, SdkError<PutBucketAclError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> impl Future<Output = Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> impl Future<Output = Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> impl Future<Output = Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> impl Future<Output = Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> impl Future<Output = Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> impl Future<Output = Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> impl Future<Output = Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>> {
        builder.send_with(&self.0)
    }
    fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> impl Future<Output = Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>> {
        builder.send_with(&self.0)
    }
    fn put_object(&self, builder: PutObjectInputBuilder) -> impl Future<Output = Result<PutObjectOutput, SdkError<PutObjectError>>> {
        builder.send_with(&self.0)
    }
    fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> impl Future<Output = Result<PutObjectAclOutput, SdkError<PutObjectAclError>>> {
        builder.send_with(&self.0)
    }
    fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> impl Future<Output = Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> impl Future<Output = Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>> {
        builder.send_with(&self.0)
    }
    fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> impl Future<Output = Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>> {
        builder.send_with(&self.0)
    }
    fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> impl Future<Output = Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>> {
        builder.send_with(&self.0)
    }
    fn restore_object(&self, builder: RestoreObjectInputBuilder) -> impl Future<Output = Result<RestoreObjectOutput, SdkError<RestoreObjectError>>> {
        builder.send_with(&self.0)
    }
    fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> impl Future<Output = Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>> {
        builder.send_with(&self.0)
    }
    fn upload_part(&self, builder: UploadPartInputBuilder) -> impl Future<Output = Result<UploadPartOutput, SdkError<UploadPartError>>> {
        builder.send_with(&self.0)
    }
    fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> impl Future<Output = Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>> {
        builder.send_with(&self.0)
    }
    fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> impl Future<Output = Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> S3Client for T
where T: Deref,
      T::Target: S3Client {
    fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> impl Future<Output = Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>> {
        self.deref().abort_multipart_upload(builder)
    }
    fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> impl Future<Output = Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>> {
        self.deref().complete_multipart_upload(builder)
    }
    fn copy_object(&self, builder: CopyObjectInputBuilder) -> impl Future<Output = Result<CopyObjectOutput, SdkError<CopyObjectError>>> {
        self.deref().copy_object(builder)
    }
    fn create_bucket(&self, builder: CreateBucketInputBuilder) -> impl Future<Output = Result<CreateBucketOutput, SdkError<CreateBucketError>>> {
        self.deref().create_bucket(builder)
    }
    fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> impl Future<Output = Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>> {
        self.deref().create_multipart_upload(builder)
    }
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> {
        self.deref().create_session(builder)
    }
    fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> impl Future<Output = Result<DeleteBucketOutput, SdkError<DeleteBucketError>>> {
        self.deref().delete_bucket(builder)
    }
    fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>> {
        self.deref().delete_bucket_analytics_configuration(builder)
    }
    fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> impl Future<Output = Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>> {
        self.deref().delete_bucket_cors(builder)
    }
    fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> impl Future<Output = Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>> {
        self.deref().delete_bucket_encryption(builder)
    }
    fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>> {
        self.deref().delete_bucket_intelligent_tiering_configuration(builder)
    }
    fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>> {
        self.deref().delete_bucket_inventory_configuration(builder)
    }
    fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> impl Future<Output = Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>> {
        self.deref().delete_bucket_lifecycle(builder)
    }
    fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>> {
        self.deref().delete_bucket_metrics_configuration(builder)
    }
    fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>> {
        self.deref().delete_bucket_ownership_controls(builder)
    }
    fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> impl Future<Output = Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>> {
        self.deref().delete_bucket_policy(builder)
    }
    fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> impl Future<Output = Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>> {
        self.deref().delete_bucket_replication(builder)
    }
    fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> impl Future<Output = Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>> {
        self.deref().delete_bucket_tagging(builder)
    }
    fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> impl Future<Output = Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>> {
        self.deref().delete_bucket_website(builder)
    }
    fn delete_object(&self, builder: DeleteObjectInputBuilder) -> impl Future<Output = Result<DeleteObjectOutput, SdkError<DeleteObjectError>>> {
        self.deref().delete_object(builder)
    }
    fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> impl Future<Output = Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>> {
        self.deref().delete_object_tagging(builder)
    }
    fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> impl Future<Output = Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>> {
        self.deref().delete_objects(builder)
    }
    fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> impl Future<Output = Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>> {
        self.deref().delete_public_access_block(builder)
    }
    fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>> {
        self.deref().get_bucket_accelerate_configuration(builder)
    }
    fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> impl Future<Output = Result<GetBucketAclOutput, SdkError<GetBucketAclError>>> {
        self.deref().get_bucket_acl(builder)
    }
    fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>> {
        self.deref().get_bucket_analytics_configuration(builder)
    }
    fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> impl Future<Output = Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>> {
        self.deref().get_bucket_cors(builder)
    }
    fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> impl Future<Output = Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>> {
        self.deref().get_bucket_encryption(builder)
    }
    fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>> {
        self.deref().get_bucket_intelligent_tiering_configuration(builder)
    }
    fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>> {
        self.deref().get_bucket_inventory_configuration(builder)
    }
    fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>> {
        self.deref().get_bucket_lifecycle_configuration(builder)
    }
    fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> impl Future<Output = Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>> {
        self.deref().get_bucket_location(builder)
    }
    fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> impl Future<Output = Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>> {
        self.deref().get_bucket_logging(builder)
    }
    fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>> {
        self.deref().get_bucket_metrics_configuration(builder)
    }
    fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>> {
        self.deref().get_bucket_notification_configuration(builder)
    }
    fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>> {
        self.deref().get_bucket_ownership_controls(builder)
    }
    fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> impl Future<Output = Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>> {
        self.deref().get_bucket_policy(builder)
    }
    fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> impl Future<Output = Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>> {
        self.deref().get_bucket_policy_status(builder)
    }
    fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> impl Future<Output = Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>> {
        self.deref().get_bucket_replication(builder)
    }
    fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>> {
        self.deref().get_bucket_request_payment(builder)
    }
    fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> impl Future<Output = Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>> {
        self.deref().get_bucket_tagging(builder)
    }
    fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> impl Future<Output = Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>> {
        self.deref().get_bucket_versioning(builder)
    }
    fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> impl Future<Output = Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>> {
        self.deref().get_bucket_website(builder)
    }
    fn get_object(&self, builder: GetObjectInputBuilder) -> impl Future<Output = Result<GetObjectOutput, SdkError<GetObjectError>>> {
        self.deref().get_object(builder)
    }
    fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> impl Future<Output = Result<GetObjectAclOutput, SdkError<GetObjectAclError>>> {
        self.deref().get_object_acl(builder)
    }
    fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> impl Future<Output = Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>> {
        self.deref().get_object_attributes(builder)
    }
    fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> impl Future<Output = Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>> {
        self.deref().get_object_legal_hold(builder)
    }
    fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>> {
        self.deref().get_object_lock_configuration(builder)
    }
    fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> impl Future<Output = Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>> {
        self.deref().get_object_retention(builder)
    }
    fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> impl Future<Output = Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>> {
        self.deref().get_object_tagging(builder)
    }
    fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> impl Future<Output = Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>> {
        self.deref().get_object_torrent(builder)
    }
    fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> impl Future<Output = Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>> {
        self.deref().get_public_access_block(builder)
    }
    fn head_bucket(&self, builder: HeadBucketInputBuilder) -> impl Future<Output = Result<HeadBucketOutput, SdkError<HeadBucketError>>> {
        self.deref().head_bucket(builder)
    }
    fn head_object(&self, builder: HeadObjectInputBuilder) -> impl Future<Output = Result<HeadObjectOutput, SdkError<HeadObjectError>>> {
        self.deref().head_object(builder)
    }
    fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>> {
        self.deref().list_bucket_analytics_configurations(builder)
    }
    fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>> {
        self.deref().list_bucket_intelligent_tiering_configurations(builder)
    }
    fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>> {
        self.deref().list_bucket_inventory_configurations(builder)
    }
    fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>> {
        self.deref().list_bucket_metrics_configurations(builder)
    }
    fn list_buckets(&self, builder: ListBucketsInputBuilder) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>> {
        self.deref().list_buckets(builder)
    }
    fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> impl Future<Output = Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>> {
        self.deref().list_directory_buckets(builder)
    }
    fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> impl Future<Output = Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>> {
        self.deref().list_multipart_uploads(builder)
    }
    fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> impl Future<Output = Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>> {
        self.deref().list_object_versions(builder)
    }
    fn list_objects(&self, builder: ListObjectsInputBuilder) -> impl Future<Output = Result<ListObjectsOutput, SdkError<ListObjectsError>>> {
        self.deref().list_objects(builder)
    }
    fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> impl Future<Output = Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>> {
        self.deref().list_objects_v2(builder)
    }
    fn list_parts(&self, builder: ListPartsInputBuilder) -> impl Future<Output = Result<ListPartsOutput, SdkError<ListPartsError>>> {
        self.deref().list_parts(builder)
    }
    fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>> {
        self.deref().put_bucket_accelerate_configuration(builder)
    }
    fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> impl Future<Output = Result<PutBucketAclOutput, SdkError<PutBucketAclError>>> {
        self.deref().put_bucket_acl(builder)
    }
    fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>> {
        self.deref().put_bucket_analytics_configuration(builder)
    }
    fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> impl Future<Output = Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>> {
        self.deref().put_bucket_cors(builder)
    }
    fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> impl Future<Output = Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>> {
        self.deref().put_bucket_encryption(builder)
    }
    fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>> {
        self.deref().put_bucket_intelligent_tiering_configuration(builder)
    }
    fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>> {
        self.deref().put_bucket_inventory_configuration(builder)
    }
    fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>> {
        self.deref().put_bucket_lifecycle_configuration(builder)
    }
    fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> impl Future<Output = Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>> {
        self.deref().put_bucket_logging(builder)
    }
    fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>> {
        self.deref().put_bucket_metrics_configuration(builder)
    }
    fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>> {
        self.deref().put_bucket_notification_configuration(builder)
    }
    fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>> {
        self.deref().put_bucket_ownership_controls(builder)
    }
    fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> impl Future<Output = Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>> {
        self.deref().put_bucket_policy(builder)
    }
    fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> impl Future<Output = Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>> {
        self.deref().put_bucket_replication(builder)
    }
    fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>> {
        self.deref().put_bucket_request_payment(builder)
    }
    fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> impl Future<Output = Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>> {
        self.deref().put_bucket_tagging(builder)
    }
    fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> impl Future<Output = Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>> {
        self.deref().put_bucket_versioning(builder)
    }
    fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> impl Future<Output = Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>> {
        self.deref().put_bucket_website(builder)
    }
    fn put_object(&self, builder: PutObjectInputBuilder) -> impl Future<Output = Result<PutObjectOutput, SdkError<PutObjectError>>> {
        self.deref().put_object(builder)
    }
    fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> impl Future<Output = Result<PutObjectAclOutput, SdkError<PutObjectAclError>>> {
        self.deref().put_object_acl(builder)
    }
    fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> impl Future<Output = Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>> {
        self.deref().put_object_legal_hold(builder)
    }
    fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>> {
        self.deref().put_object_lock_configuration(builder)
    }
    fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> impl Future<Output = Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>> {
        self.deref().put_object_retention(builder)
    }
    fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> impl Future<Output = Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>> {
        self.deref().put_object_tagging(builder)
    }
    fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> impl Future<Output = Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>> {
        self.deref().put_public_access_block(builder)
    }
    fn restore_object(&self, builder: RestoreObjectInputBuilder) -> impl Future<Output = Result<RestoreObjectOutput, SdkError<RestoreObjectError>>> {
        self.deref().restore_object(builder)
    }
    fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> impl Future<Output = Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>> {
        self.deref().select_object_content(builder)
    }
    fn upload_part(&self, builder: UploadPartInputBuilder) -> impl Future<Output = Result<UploadPartOutput, SdkError<UploadPartError>>> {
        self.deref().upload_part(builder)
    }
    fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> impl Future<Output = Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>> {
        self.deref().upload_part_copy(builder)
    }
    fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> impl Future<Output = Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>> {
        self.deref().write_get_object_response(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edS3Client {}
    impl S3Client for edS3Client {
        async fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>;
        async fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>;
        async fn copy_object(&self, builder: CopyObjectInputBuilder) -> Result<CopyObjectOutput, SdkError<CopyObjectError>>;
        async fn create_bucket(&self, builder: CreateBucketInputBuilder) -> Result<CreateBucketOutput, SdkError<CreateBucketError>>;
        async fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>;
        async fn create_session(&self, builder: CreateSessionInputBuilder) -> Result<CreateSessionOutput, SdkError<CreateSessionError>>;
        async fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> Result<DeleteBucketOutput, SdkError<DeleteBucketError>>;
        async fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>;
        async fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>;
        async fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>;
        async fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>;
        async fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>;
        async fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>;
        async fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>;
        async fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>;
        async fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>;
        async fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>;
        async fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>;
        async fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>;
        async fn delete_object(&self, builder: DeleteObjectInputBuilder) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>>;
        async fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>;
        async fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>;
        async fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>;
        async fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>;
        async fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> Result<GetBucketAclOutput, SdkError<GetBucketAclError>>;
        async fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>;
        async fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>;
        async fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>;
        async fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>;
        async fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>;
        async fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>;
        async fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>;
        async fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>;
        async fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>;
        async fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>;
        async fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>;
        async fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>;
        async fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>;
        async fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>;
        async fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>;
        async fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>;
        async fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>;
        async fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>;
        async fn get_object(&self, builder: GetObjectInputBuilder) -> Result<GetObjectOutput, SdkError<GetObjectError>>;
        async fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> Result<GetObjectAclOutput, SdkError<GetObjectAclError>>;
        async fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>;
        async fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>;
        async fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>;
        async fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>;
        async fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>;
        async fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>;
        async fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>;
        async fn head_bucket(&self, builder: HeadBucketInputBuilder) -> Result<HeadBucketOutput, SdkError<HeadBucketError>>;
        async fn head_object(&self, builder: HeadObjectInputBuilder) -> Result<HeadObjectOutput, SdkError<HeadObjectError>>;
        async fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>;
        async fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>;
        async fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>;
        async fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>;
        async fn list_buckets(&self, builder: ListBucketsInputBuilder) -> Result<ListBucketsOutput, SdkError<ListBucketsError>>;
        async fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>;
        async fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>;
        async fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>;
        async fn list_objects(&self, builder: ListObjectsInputBuilder) -> Result<ListObjectsOutput, SdkError<ListObjectsError>>;
        async fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>;
        async fn list_parts(&self, builder: ListPartsInputBuilder) -> Result<ListPartsOutput, SdkError<ListPartsError>>;
        async fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>;
        async fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> Result<PutBucketAclOutput, SdkError<PutBucketAclError>>;
        async fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>;
        async fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>;
        async fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>;
        async fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>;
        async fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>;
        async fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>;
        async fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>;
        async fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>;
        async fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>;
        async fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>;
        async fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>;
        async fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>;
        async fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>;
        async fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>;
        async fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>;
        async fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>;
        async fn put_object(&self, builder: PutObjectInputBuilder) -> Result<PutObjectOutput, SdkError<PutObjectError>>;
        async fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> Result<PutObjectAclOutput, SdkError<PutObjectAclError>>;
        async fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>;
        async fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>;
        async fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>;
        async fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>;
        async fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>;
        async fn restore_object(&self, builder: RestoreObjectInputBuilder) -> Result<RestoreObjectOutput, SdkError<RestoreObjectError>>;
        async fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>;
        async fn upload_part(&self, builder: UploadPartInputBuilder) -> Result<UploadPartOutput, SdkError<UploadPartError>>;
        async fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>;
        async fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>;
    }
}
