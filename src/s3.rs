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
use aws_sdk_s3::Client;

pub use aws_sdk_s3::*;

pub struct S3ClientImpl(Client);
impl S3ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait S3Client {
    fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> impl Future<Output = Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>>;
    fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> impl Future<Output = Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>>;
    fn copy_object(&self, builder: CopyObjectInputBuilder) -> impl Future<Output = Result<CopyObjectOutput, SdkError<CopyObjectError>>>;
    fn create_bucket(&self, builder: CreateBucketInputBuilder) -> impl Future<Output = Result<CreateBucketOutput, SdkError<CreateBucketError>>>;
    fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> impl Future<Output = Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>>;
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>>;
    fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> impl Future<Output = Result<DeleteBucketOutput, SdkError<DeleteBucketError>>>;
    fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>>;
    fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> impl Future<Output = Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>>;
    fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> impl Future<Output = Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>>;
    fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>>;
    fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>>;
    fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> impl Future<Output = Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>>;
    fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>>;
    fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>>;
    fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> impl Future<Output = Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>>;
    fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> impl Future<Output = Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>>;
    fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> impl Future<Output = Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>>;
    fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> impl Future<Output = Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>>;
    fn delete_object(&self, builder: DeleteObjectInputBuilder) -> impl Future<Output = Result<DeleteObjectOutput, SdkError<DeleteObjectError>>>;
    fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> impl Future<Output = Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>>;
    fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> impl Future<Output = Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>>;
    fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> impl Future<Output = Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>>;
    fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>>;
    fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> impl Future<Output = Result<GetBucketAclOutput, SdkError<GetBucketAclError>>>;
    fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>>;
    fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> impl Future<Output = Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>>;
    fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> impl Future<Output = Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>>;
    fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>>;
    fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>>;
    fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>>;
    fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> impl Future<Output = Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>>;
    fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> impl Future<Output = Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>>;
    fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>>;
    fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>>;
    fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>>;
    fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> impl Future<Output = Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>>;
    fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> impl Future<Output = Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>>;
    fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> impl Future<Output = Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>>;
    fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>>;
    fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> impl Future<Output = Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>>;
    fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> impl Future<Output = Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>>;
    fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> impl Future<Output = Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>>;
    fn get_object(&self, builder: GetObjectInputBuilder) -> impl Future<Output = Result<GetObjectOutput, SdkError<GetObjectError>>>;
    fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> impl Future<Output = Result<GetObjectAclOutput, SdkError<GetObjectAclError>>>;
    fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> impl Future<Output = Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>>;
    fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> impl Future<Output = Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>>;
    fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>>;
    fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> impl Future<Output = Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>>;
    fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> impl Future<Output = Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>>;
    fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> impl Future<Output = Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>>;
    fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> impl Future<Output = Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>>;
    fn head_bucket(&self, builder: HeadBucketInputBuilder) -> impl Future<Output = Result<HeadBucketOutput, SdkError<HeadBucketError>>>;
    fn head_object(&self, builder: HeadObjectInputBuilder) -> impl Future<Output = Result<HeadObjectOutput, SdkError<HeadObjectError>>>;
    fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>>;
    fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>>;
    fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>>;
    fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>>;
    fn list_buckets(&self, builder: ListBucketsInputBuilder) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>>;
    fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> impl Future<Output = Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>>;
    fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> impl Future<Output = Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>>;
    fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> impl Future<Output = Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>>;
    fn list_objects(&self, builder: ListObjectsInputBuilder) -> impl Future<Output = Result<ListObjectsOutput, SdkError<ListObjectsError>>>;
    fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> impl Future<Output = Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>>;
    fn list_parts(&self, builder: ListPartsInputBuilder) -> impl Future<Output = Result<ListPartsOutput, SdkError<ListPartsError>>>;
    fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>>;
    fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> impl Future<Output = Result<PutBucketAclOutput, SdkError<PutBucketAclError>>>;
    fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>>;
    fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> impl Future<Output = Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>>;
    fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> impl Future<Output = Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>>;
    fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>>;
    fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>>;
    fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>>;
    fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> impl Future<Output = Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>>;
    fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>>;
    fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>>;
    fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>>;
    fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> impl Future<Output = Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>>;
    fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> impl Future<Output = Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>>;
    fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>>;
    fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> impl Future<Output = Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>>;
    fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> impl Future<Output = Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>>;
    fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> impl Future<Output = Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>>;
    fn put_object(&self, builder: PutObjectInputBuilder) -> impl Future<Output = Result<PutObjectOutput, SdkError<PutObjectError>>>;
    fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> impl Future<Output = Result<PutObjectAclOutput, SdkError<PutObjectAclError>>>;
    fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> impl Future<Output = Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>>;
    fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>>;
    fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> impl Future<Output = Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>>;
    fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> impl Future<Output = Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>>;
    fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> impl Future<Output = Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>>;
    fn restore_object(&self, builder: RestoreObjectInputBuilder) -> impl Future<Output = Result<RestoreObjectOutput, SdkError<RestoreObjectError>>>;
    fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> impl Future<Output = Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>>;
    fn upload_part(&self, builder: UploadPartInputBuilder) -> impl Future<Output = Result<UploadPartOutput, SdkError<UploadPartError>>>;
    fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> impl Future<Output = Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>>;
    fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> impl Future<Output = Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>>;
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
impl <T: S3Client> S3Client for &T {
    fn abort_multipart_upload(&self, builder: AbortMultipartUploadInputBuilder) -> impl Future<Output = Result<AbortMultipartUploadOutput, SdkError<AbortMultipartUploadError>>> {
        (*self).abort_multipart_upload(builder)
    }
    fn complete_multipart_upload(&self, builder: CompleteMultipartUploadInputBuilder) -> impl Future<Output = Result<CompleteMultipartUploadOutput, SdkError<CompleteMultipartUploadError>>> {
        (*self).complete_multipart_upload(builder)
    }
    fn copy_object(&self, builder: CopyObjectInputBuilder) -> impl Future<Output = Result<CopyObjectOutput, SdkError<CopyObjectError>>> {
        (*self).copy_object(builder)
    }
    fn create_bucket(&self, builder: CreateBucketInputBuilder) -> impl Future<Output = Result<CreateBucketOutput, SdkError<CreateBucketError>>> {
        (*self).create_bucket(builder)
    }
    fn create_multipart_upload(&self, builder: CreateMultipartUploadInputBuilder) -> impl Future<Output = Result<CreateMultipartUploadOutput, SdkError<CreateMultipartUploadError>>> {
        (*self).create_multipart_upload(builder)
    }
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> {
        (*self).create_session(builder)
    }
    fn delete_bucket(&self, builder: DeleteBucketInputBuilder) -> impl Future<Output = Result<DeleteBucketOutput, SdkError<DeleteBucketError>>> {
        (*self).delete_bucket(builder)
    }
    fn delete_bucket_analytics_configuration(&self, builder: DeleteBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketAnalyticsConfigurationOutput, SdkError<DeleteBucketAnalyticsConfigurationError>>> {
        (*self).delete_bucket_analytics_configuration(builder)
    }
    fn delete_bucket_cors(&self, builder: DeleteBucketCorsInputBuilder) -> impl Future<Output = Result<DeleteBucketCorsOutput, SdkError<DeleteBucketCorsError>>> {
        (*self).delete_bucket_cors(builder)
    }
    fn delete_bucket_encryption(&self, builder: DeleteBucketEncryptionInputBuilder) -> impl Future<Output = Result<DeleteBucketEncryptionOutput, SdkError<DeleteBucketEncryptionError>>> {
        (*self).delete_bucket_encryption(builder)
    }
    fn delete_bucket_intelligent_tiering_configuration(&self, builder: DeleteBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketIntelligentTieringConfigurationOutput, SdkError<DeleteBucketIntelligentTieringConfigurationError>>> {
        (*self).delete_bucket_intelligent_tiering_configuration(builder)
    }
    fn delete_bucket_inventory_configuration(&self, builder: DeleteBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketInventoryConfigurationOutput, SdkError<DeleteBucketInventoryConfigurationError>>> {
        (*self).delete_bucket_inventory_configuration(builder)
    }
    fn delete_bucket_lifecycle(&self, builder: DeleteBucketLifecycleInputBuilder) -> impl Future<Output = Result<DeleteBucketLifecycleOutput, SdkError<DeleteBucketLifecycleError>>> {
        (*self).delete_bucket_lifecycle(builder)
    }
    fn delete_bucket_metrics_configuration(&self, builder: DeleteBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBucketMetricsConfigurationOutput, SdkError<DeleteBucketMetricsConfigurationError>>> {
        (*self).delete_bucket_metrics_configuration(builder)
    }
    fn delete_bucket_ownership_controls(&self, builder: DeleteBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<DeleteBucketOwnershipControlsOutput, SdkError<DeleteBucketOwnershipControlsError>>> {
        (*self).delete_bucket_ownership_controls(builder)
    }
    fn delete_bucket_policy(&self, builder: DeleteBucketPolicyInputBuilder) -> impl Future<Output = Result<DeleteBucketPolicyOutput, SdkError<DeleteBucketPolicyError>>> {
        (*self).delete_bucket_policy(builder)
    }
    fn delete_bucket_replication(&self, builder: DeleteBucketReplicationInputBuilder) -> impl Future<Output = Result<DeleteBucketReplicationOutput, SdkError<DeleteBucketReplicationError>>> {
        (*self).delete_bucket_replication(builder)
    }
    fn delete_bucket_tagging(&self, builder: DeleteBucketTaggingInputBuilder) -> impl Future<Output = Result<DeleteBucketTaggingOutput, SdkError<DeleteBucketTaggingError>>> {
        (*self).delete_bucket_tagging(builder)
    }
    fn delete_bucket_website(&self, builder: DeleteBucketWebsiteInputBuilder) -> impl Future<Output = Result<DeleteBucketWebsiteOutput, SdkError<DeleteBucketWebsiteError>>> {
        (*self).delete_bucket_website(builder)
    }
    fn delete_object(&self, builder: DeleteObjectInputBuilder) -> impl Future<Output = Result<DeleteObjectOutput, SdkError<DeleteObjectError>>> {
        (*self).delete_object(builder)
    }
    fn delete_object_tagging(&self, builder: DeleteObjectTaggingInputBuilder) -> impl Future<Output = Result<DeleteObjectTaggingOutput, SdkError<DeleteObjectTaggingError>>> {
        (*self).delete_object_tagging(builder)
    }
    fn delete_objects(&self, builder: DeleteObjectsInputBuilder) -> impl Future<Output = Result<DeleteObjectsOutput, SdkError<DeleteObjectsError>>> {
        (*self).delete_objects(builder)
    }
    fn delete_public_access_block(&self, builder: DeletePublicAccessBlockInputBuilder) -> impl Future<Output = Result<DeletePublicAccessBlockOutput, SdkError<DeletePublicAccessBlockError>>> {
        (*self).delete_public_access_block(builder)
    }
    fn get_bucket_accelerate_configuration(&self, builder: GetBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAccelerateConfigurationOutput, SdkError<GetBucketAccelerateConfigurationError>>> {
        (*self).get_bucket_accelerate_configuration(builder)
    }
    fn get_bucket_acl(&self, builder: GetBucketAclInputBuilder) -> impl Future<Output = Result<GetBucketAclOutput, SdkError<GetBucketAclError>>> {
        (*self).get_bucket_acl(builder)
    }
    fn get_bucket_analytics_configuration(&self, builder: GetBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketAnalyticsConfigurationOutput, SdkError<GetBucketAnalyticsConfigurationError>>> {
        (*self).get_bucket_analytics_configuration(builder)
    }
    fn get_bucket_cors(&self, builder: GetBucketCorsInputBuilder) -> impl Future<Output = Result<GetBucketCorsOutput, SdkError<GetBucketCorsError>>> {
        (*self).get_bucket_cors(builder)
    }
    fn get_bucket_encryption(&self, builder: GetBucketEncryptionInputBuilder) -> impl Future<Output = Result<GetBucketEncryptionOutput, SdkError<GetBucketEncryptionError>>> {
        (*self).get_bucket_encryption(builder)
    }
    fn get_bucket_intelligent_tiering_configuration(&self, builder: GetBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketIntelligentTieringConfigurationOutput, SdkError<GetBucketIntelligentTieringConfigurationError>>> {
        (*self).get_bucket_intelligent_tiering_configuration(builder)
    }
    fn get_bucket_inventory_configuration(&self, builder: GetBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketInventoryConfigurationOutput, SdkError<GetBucketInventoryConfigurationError>>> {
        (*self).get_bucket_inventory_configuration(builder)
    }
    fn get_bucket_lifecycle_configuration(&self, builder: GetBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketLifecycleConfigurationOutput, SdkError<GetBucketLifecycleConfigurationError>>> {
        (*self).get_bucket_lifecycle_configuration(builder)
    }
    fn get_bucket_location(&self, builder: GetBucketLocationInputBuilder) -> impl Future<Output = Result<GetBucketLocationOutput, SdkError<GetBucketLocationError>>> {
        (*self).get_bucket_location(builder)
    }
    fn get_bucket_logging(&self, builder: GetBucketLoggingInputBuilder) -> impl Future<Output = Result<GetBucketLoggingOutput, SdkError<GetBucketLoggingError>>> {
        (*self).get_bucket_logging(builder)
    }
    fn get_bucket_metrics_configuration(&self, builder: GetBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketMetricsConfigurationOutput, SdkError<GetBucketMetricsConfigurationError>>> {
        (*self).get_bucket_metrics_configuration(builder)
    }
    fn get_bucket_notification_configuration(&self, builder: GetBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<GetBucketNotificationConfigurationOutput, SdkError<GetBucketNotificationConfigurationError>>> {
        (*self).get_bucket_notification_configuration(builder)
    }
    fn get_bucket_ownership_controls(&self, builder: GetBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<GetBucketOwnershipControlsOutput, SdkError<GetBucketOwnershipControlsError>>> {
        (*self).get_bucket_ownership_controls(builder)
    }
    fn get_bucket_policy(&self, builder: GetBucketPolicyInputBuilder) -> impl Future<Output = Result<GetBucketPolicyOutput, SdkError<GetBucketPolicyError>>> {
        (*self).get_bucket_policy(builder)
    }
    fn get_bucket_policy_status(&self, builder: GetBucketPolicyStatusInputBuilder) -> impl Future<Output = Result<GetBucketPolicyStatusOutput, SdkError<GetBucketPolicyStatusError>>> {
        (*self).get_bucket_policy_status(builder)
    }
    fn get_bucket_replication(&self, builder: GetBucketReplicationInputBuilder) -> impl Future<Output = Result<GetBucketReplicationOutput, SdkError<GetBucketReplicationError>>> {
        (*self).get_bucket_replication(builder)
    }
    fn get_bucket_request_payment(&self, builder: GetBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<GetBucketRequestPaymentOutput, SdkError<GetBucketRequestPaymentError>>> {
        (*self).get_bucket_request_payment(builder)
    }
    fn get_bucket_tagging(&self, builder: GetBucketTaggingInputBuilder) -> impl Future<Output = Result<GetBucketTaggingOutput, SdkError<GetBucketTaggingError>>> {
        (*self).get_bucket_tagging(builder)
    }
    fn get_bucket_versioning(&self, builder: GetBucketVersioningInputBuilder) -> impl Future<Output = Result<GetBucketVersioningOutput, SdkError<GetBucketVersioningError>>> {
        (*self).get_bucket_versioning(builder)
    }
    fn get_bucket_website(&self, builder: GetBucketWebsiteInputBuilder) -> impl Future<Output = Result<GetBucketWebsiteOutput, SdkError<GetBucketWebsiteError>>> {
        (*self).get_bucket_website(builder)
    }
    fn get_object(&self, builder: GetObjectInputBuilder) -> impl Future<Output = Result<GetObjectOutput, SdkError<GetObjectError>>> {
        (*self).get_object(builder)
    }
    fn get_object_acl(&self, builder: GetObjectAclInputBuilder) -> impl Future<Output = Result<GetObjectAclOutput, SdkError<GetObjectAclError>>> {
        (*self).get_object_acl(builder)
    }
    fn get_object_attributes(&self, builder: GetObjectAttributesInputBuilder) -> impl Future<Output = Result<GetObjectAttributesOutput, SdkError<GetObjectAttributesError>>> {
        (*self).get_object_attributes(builder)
    }
    fn get_object_legal_hold(&self, builder: GetObjectLegalHoldInputBuilder) -> impl Future<Output = Result<GetObjectLegalHoldOutput, SdkError<GetObjectLegalHoldError>>> {
        (*self).get_object_legal_hold(builder)
    }
    fn get_object_lock_configuration(&self, builder: GetObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<GetObjectLockConfigurationOutput, SdkError<GetObjectLockConfigurationError>>> {
        (*self).get_object_lock_configuration(builder)
    }
    fn get_object_retention(&self, builder: GetObjectRetentionInputBuilder) -> impl Future<Output = Result<GetObjectRetentionOutput, SdkError<GetObjectRetentionError>>> {
        (*self).get_object_retention(builder)
    }
    fn get_object_tagging(&self, builder: GetObjectTaggingInputBuilder) -> impl Future<Output = Result<GetObjectTaggingOutput, SdkError<GetObjectTaggingError>>> {
        (*self).get_object_tagging(builder)
    }
    fn get_object_torrent(&self, builder: GetObjectTorrentInputBuilder) -> impl Future<Output = Result<GetObjectTorrentOutput, SdkError<GetObjectTorrentError>>> {
        (*self).get_object_torrent(builder)
    }
    fn get_public_access_block(&self, builder: GetPublicAccessBlockInputBuilder) -> impl Future<Output = Result<GetPublicAccessBlockOutput, SdkError<GetPublicAccessBlockError>>> {
        (*self).get_public_access_block(builder)
    }
    fn head_bucket(&self, builder: HeadBucketInputBuilder) -> impl Future<Output = Result<HeadBucketOutput, SdkError<HeadBucketError>>> {
        (*self).head_bucket(builder)
    }
    fn head_object(&self, builder: HeadObjectInputBuilder) -> impl Future<Output = Result<HeadObjectOutput, SdkError<HeadObjectError>>> {
        (*self).head_object(builder)
    }
    fn list_bucket_analytics_configurations(&self, builder: ListBucketAnalyticsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketAnalyticsConfigurationsOutput, SdkError<ListBucketAnalyticsConfigurationsError>>> {
        (*self).list_bucket_analytics_configurations(builder)
    }
    fn list_bucket_intelligent_tiering_configurations(&self, builder: ListBucketIntelligentTieringConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketIntelligentTieringConfigurationsOutput, SdkError<ListBucketIntelligentTieringConfigurationsError>>> {
        (*self).list_bucket_intelligent_tiering_configurations(builder)
    }
    fn list_bucket_inventory_configurations(&self, builder: ListBucketInventoryConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketInventoryConfigurationsOutput, SdkError<ListBucketInventoryConfigurationsError>>> {
        (*self).list_bucket_inventory_configurations(builder)
    }
    fn list_bucket_metrics_configurations(&self, builder: ListBucketMetricsConfigurationsInputBuilder) -> impl Future<Output = Result<ListBucketMetricsConfigurationsOutput, SdkError<ListBucketMetricsConfigurationsError>>> {
        (*self).list_bucket_metrics_configurations(builder)
    }
    fn list_buckets(&self, builder: ListBucketsInputBuilder) -> impl Future<Output = Result<ListBucketsOutput, SdkError<ListBucketsError>>> {
        (*self).list_buckets(builder)
    }
    fn list_directory_buckets(&self, builder: ListDirectoryBucketsInputBuilder) -> impl Future<Output = Result<ListDirectoryBucketsOutput, SdkError<ListDirectoryBucketsError>>> {
        (*self).list_directory_buckets(builder)
    }
    fn list_multipart_uploads(&self, builder: ListMultipartUploadsInputBuilder) -> impl Future<Output = Result<ListMultipartUploadsOutput, SdkError<ListMultipartUploadsError>>> {
        (*self).list_multipart_uploads(builder)
    }
    fn list_object_versions(&self, builder: ListObjectVersionsInputBuilder) -> impl Future<Output = Result<ListObjectVersionsOutput, SdkError<ListObjectVersionsError>>> {
        (*self).list_object_versions(builder)
    }
    fn list_objects(&self, builder: ListObjectsInputBuilder) -> impl Future<Output = Result<ListObjectsOutput, SdkError<ListObjectsError>>> {
        (*self).list_objects(builder)
    }
    fn list_objects_v2(&self, builder: ListObjectsV2InputBuilder) -> impl Future<Output = Result<ListObjectsV2Output, SdkError<ListObjectsV2Error>>> {
        (*self).list_objects_v2(builder)
    }
    fn list_parts(&self, builder: ListPartsInputBuilder) -> impl Future<Output = Result<ListPartsOutput, SdkError<ListPartsError>>> {
        (*self).list_parts(builder)
    }
    fn put_bucket_accelerate_configuration(&self, builder: PutBucketAccelerateConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAccelerateConfigurationOutput, SdkError<PutBucketAccelerateConfigurationError>>> {
        (*self).put_bucket_accelerate_configuration(builder)
    }
    fn put_bucket_acl(&self, builder: PutBucketAclInputBuilder) -> impl Future<Output = Result<PutBucketAclOutput, SdkError<PutBucketAclError>>> {
        (*self).put_bucket_acl(builder)
    }
    fn put_bucket_analytics_configuration(&self, builder: PutBucketAnalyticsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketAnalyticsConfigurationOutput, SdkError<PutBucketAnalyticsConfigurationError>>> {
        (*self).put_bucket_analytics_configuration(builder)
    }
    fn put_bucket_cors(&self, builder: PutBucketCorsInputBuilder) -> impl Future<Output = Result<PutBucketCorsOutput, SdkError<PutBucketCorsError>>> {
        (*self).put_bucket_cors(builder)
    }
    fn put_bucket_encryption(&self, builder: PutBucketEncryptionInputBuilder) -> impl Future<Output = Result<PutBucketEncryptionOutput, SdkError<PutBucketEncryptionError>>> {
        (*self).put_bucket_encryption(builder)
    }
    fn put_bucket_intelligent_tiering_configuration(&self, builder: PutBucketIntelligentTieringConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketIntelligentTieringConfigurationOutput, SdkError<PutBucketIntelligentTieringConfigurationError>>> {
        (*self).put_bucket_intelligent_tiering_configuration(builder)
    }
    fn put_bucket_inventory_configuration(&self, builder: PutBucketInventoryConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketInventoryConfigurationOutput, SdkError<PutBucketInventoryConfigurationError>>> {
        (*self).put_bucket_inventory_configuration(builder)
    }
    fn put_bucket_lifecycle_configuration(&self, builder: PutBucketLifecycleConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketLifecycleConfigurationOutput, SdkError<PutBucketLifecycleConfigurationError>>> {
        (*self).put_bucket_lifecycle_configuration(builder)
    }
    fn put_bucket_logging(&self, builder: PutBucketLoggingInputBuilder) -> impl Future<Output = Result<PutBucketLoggingOutput, SdkError<PutBucketLoggingError>>> {
        (*self).put_bucket_logging(builder)
    }
    fn put_bucket_metrics_configuration(&self, builder: PutBucketMetricsConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketMetricsConfigurationOutput, SdkError<PutBucketMetricsConfigurationError>>> {
        (*self).put_bucket_metrics_configuration(builder)
    }
    fn put_bucket_notification_configuration(&self, builder: PutBucketNotificationConfigurationInputBuilder) -> impl Future<Output = Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>>> {
        (*self).put_bucket_notification_configuration(builder)
    }
    fn put_bucket_ownership_controls(&self, builder: PutBucketOwnershipControlsInputBuilder) -> impl Future<Output = Result<PutBucketOwnershipControlsOutput, SdkError<PutBucketOwnershipControlsError>>> {
        (*self).put_bucket_ownership_controls(builder)
    }
    fn put_bucket_policy(&self, builder: PutBucketPolicyInputBuilder) -> impl Future<Output = Result<PutBucketPolicyOutput, SdkError<PutBucketPolicyError>>> {
        (*self).put_bucket_policy(builder)
    }
    fn put_bucket_replication(&self, builder: PutBucketReplicationInputBuilder) -> impl Future<Output = Result<PutBucketReplicationOutput, SdkError<PutBucketReplicationError>>> {
        (*self).put_bucket_replication(builder)
    }
    fn put_bucket_request_payment(&self, builder: PutBucketRequestPaymentInputBuilder) -> impl Future<Output = Result<PutBucketRequestPaymentOutput, SdkError<PutBucketRequestPaymentError>>> {
        (*self).put_bucket_request_payment(builder)
    }
    fn put_bucket_tagging(&self, builder: PutBucketTaggingInputBuilder) -> impl Future<Output = Result<PutBucketTaggingOutput, SdkError<PutBucketTaggingError>>> {
        (*self).put_bucket_tagging(builder)
    }
    fn put_bucket_versioning(&self, builder: PutBucketVersioningInputBuilder) -> impl Future<Output = Result<PutBucketVersioningOutput, SdkError<PutBucketVersioningError>>> {
        (*self).put_bucket_versioning(builder)
    }
    fn put_bucket_website(&self, builder: PutBucketWebsiteInputBuilder) -> impl Future<Output = Result<PutBucketWebsiteOutput, SdkError<PutBucketWebsiteError>>> {
        (*self).put_bucket_website(builder)
    }
    fn put_object(&self, builder: PutObjectInputBuilder) -> impl Future<Output = Result<PutObjectOutput, SdkError<PutObjectError>>> {
        (*self).put_object(builder)
    }
    fn put_object_acl(&self, builder: PutObjectAclInputBuilder) -> impl Future<Output = Result<PutObjectAclOutput, SdkError<PutObjectAclError>>> {
        (*self).put_object_acl(builder)
    }
    fn put_object_legal_hold(&self, builder: PutObjectLegalHoldInputBuilder) -> impl Future<Output = Result<PutObjectLegalHoldOutput, SdkError<PutObjectLegalHoldError>>> {
        (*self).put_object_legal_hold(builder)
    }
    fn put_object_lock_configuration(&self, builder: PutObjectLockConfigurationInputBuilder) -> impl Future<Output = Result<PutObjectLockConfigurationOutput, SdkError<PutObjectLockConfigurationError>>> {
        (*self).put_object_lock_configuration(builder)
    }
    fn put_object_retention(&self, builder: PutObjectRetentionInputBuilder) -> impl Future<Output = Result<PutObjectRetentionOutput, SdkError<PutObjectRetentionError>>> {
        (*self).put_object_retention(builder)
    }
    fn put_object_tagging(&self, builder: PutObjectTaggingInputBuilder) -> impl Future<Output = Result<PutObjectTaggingOutput, SdkError<PutObjectTaggingError>>> {
        (*self).put_object_tagging(builder)
    }
    fn put_public_access_block(&self, builder: PutPublicAccessBlockInputBuilder) -> impl Future<Output = Result<PutPublicAccessBlockOutput, SdkError<PutPublicAccessBlockError>>> {
        (*self).put_public_access_block(builder)
    }
    fn restore_object(&self, builder: RestoreObjectInputBuilder) -> impl Future<Output = Result<RestoreObjectOutput, SdkError<RestoreObjectError>>> {
        (*self).restore_object(builder)
    }
    fn select_object_content(&self, builder: SelectObjectContentInputBuilder) -> impl Future<Output = Result<SelectObjectContentOutput, SdkError<SelectObjectContentError>>> {
        (*self).select_object_content(builder)
    }
    fn upload_part(&self, builder: UploadPartInputBuilder) -> impl Future<Output = Result<UploadPartOutput, SdkError<UploadPartError>>> {
        (*self).upload_part(builder)
    }
    fn upload_part_copy(&self, builder: UploadPartCopyInputBuilder) -> impl Future<Output = Result<UploadPartCopyOutput, SdkError<UploadPartCopyError>>> {
        (*self).upload_part_copy(builder)
    }
    fn write_get_object_response(&self, builder: WriteGetObjectResponseInputBuilder) -> impl Future<Output = Result<WriteGetObjectResponseOutput, SdkError<WriteGetObjectResponseError>>> {
        (*self).write_get_object_response(builder)
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
