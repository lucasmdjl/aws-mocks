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
use aws_sdk_cloudfront::operation::associate_alias::{builders::*, *};
use aws_sdk_cloudfront::operation::copy_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::create_cache_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::create_cloud_front_origin_access_identity::{builders::*, *};
use aws_sdk_cloudfront::operation::create_continuous_deployment_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::create_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::create_distribution_with_tags::{builders::*, *};
use aws_sdk_cloudfront::operation::create_field_level_encryption_config::{builders::*, *};
use aws_sdk_cloudfront::operation::create_field_level_encryption_profile::{builders::*, *};
use aws_sdk_cloudfront::operation::create_function::{builders::*, *};
use aws_sdk_cloudfront::operation::create_invalidation::{builders::*, *};
use aws_sdk_cloudfront::operation::create_key_group::{builders::*, *};
use aws_sdk_cloudfront::operation::create_key_value_store::{builders::*, *};
use aws_sdk_cloudfront::operation::create_monitoring_subscription::{builders::*, *};
use aws_sdk_cloudfront::operation::create_origin_access_control::{builders::*, *};
use aws_sdk_cloudfront::operation::create_origin_request_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::create_public_key::{builders::*, *};
use aws_sdk_cloudfront::operation::create_realtime_log_config::{builders::*, *};
use aws_sdk_cloudfront::operation::create_response_headers_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::create_streaming_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::create_streaming_distribution_with_tags::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_cache_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_cloud_front_origin_access_identity::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_continuous_deployment_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_field_level_encryption_config::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_field_level_encryption_profile::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_function::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_key_group::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_key_value_store::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_monitoring_subscription::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_origin_access_control::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_origin_request_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_public_key::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_realtime_log_config::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_response_headers_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::delete_streaming_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::describe_function::{builders::*, *};
use aws_sdk_cloudfront::operation::describe_key_value_store::{builders::*, *};
use aws_sdk_cloudfront::operation::get_cache_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::get_cache_policy_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_cloud_front_origin_access_identity::{builders::*, *};
use aws_sdk_cloudfront::operation::get_cloud_front_origin_access_identity_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_continuous_deployment_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::get_continuous_deployment_policy_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::get_distribution_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_field_level_encryption::{builders::*, *};
use aws_sdk_cloudfront::operation::get_field_level_encryption_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_field_level_encryption_profile::{builders::*, *};
use aws_sdk_cloudfront::operation::get_field_level_encryption_profile_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_function::{builders::*, *};
use aws_sdk_cloudfront::operation::get_invalidation::{builders::*, *};
use aws_sdk_cloudfront::operation::get_key_group::{builders::*, *};
use aws_sdk_cloudfront::operation::get_key_group_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_monitoring_subscription::{builders::*, *};
use aws_sdk_cloudfront::operation::get_origin_access_control::{builders::*, *};
use aws_sdk_cloudfront::operation::get_origin_access_control_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_origin_request_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::get_origin_request_policy_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_public_key::{builders::*, *};
use aws_sdk_cloudfront::operation::get_public_key_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_realtime_log_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_response_headers_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::get_response_headers_policy_config::{builders::*, *};
use aws_sdk_cloudfront::operation::get_streaming_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::get_streaming_distribution_config::{builders::*, *};
use aws_sdk_cloudfront::operation::list_cache_policies::{builders::*, *};
use aws_sdk_cloudfront::operation::list_cloud_front_origin_access_identities::{builders::*, *};
use aws_sdk_cloudfront::operation::list_conflicting_aliases::{builders::*, *};
use aws_sdk_cloudfront::operation::list_continuous_deployment_policies::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_cache_policy_id::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_key_group::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_origin_request_policy_id::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_realtime_log_config::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_response_headers_policy_id::{builders::*, *};
use aws_sdk_cloudfront::operation::list_distributions_by_web_acl_id::{builders::*, *};
use aws_sdk_cloudfront::operation::list_field_level_encryption_configs::{builders::*, *};
use aws_sdk_cloudfront::operation::list_field_level_encryption_profiles::{builders::*, *};
use aws_sdk_cloudfront::operation::list_functions::{builders::*, *};
use aws_sdk_cloudfront::operation::list_invalidations::{builders::*, *};
use aws_sdk_cloudfront::operation::list_key_groups::{builders::*, *};
use aws_sdk_cloudfront::operation::list_key_value_stores::{builders::*, *};
use aws_sdk_cloudfront::operation::list_origin_access_controls::{builders::*, *};
use aws_sdk_cloudfront::operation::list_origin_request_policies::{builders::*, *};
use aws_sdk_cloudfront::operation::list_public_keys::{builders::*, *};
use aws_sdk_cloudfront::operation::list_realtime_log_configs::{builders::*, *};
use aws_sdk_cloudfront::operation::list_response_headers_policies::{builders::*, *};
use aws_sdk_cloudfront::operation::list_streaming_distributions::{builders::*, *};
use aws_sdk_cloudfront::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_cloudfront::operation::publish_function::{builders::*, *};
use aws_sdk_cloudfront::operation::tag_resource::{builders::*, *};
use aws_sdk_cloudfront::operation::test_function::{builders::*, *};
use aws_sdk_cloudfront::operation::untag_resource::{builders::*, *};
use aws_sdk_cloudfront::operation::update_cache_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::update_cloud_front_origin_access_identity::{builders::*, *};
use aws_sdk_cloudfront::operation::update_continuous_deployment_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::update_distribution::{builders::*, *};
use aws_sdk_cloudfront::operation::update_distribution_with_staging_config::{builders::*, *};
use aws_sdk_cloudfront::operation::update_field_level_encryption_config::{builders::*, *};
use aws_sdk_cloudfront::operation::update_field_level_encryption_profile::{builders::*, *};
use aws_sdk_cloudfront::operation::update_function::{builders::*, *};
use aws_sdk_cloudfront::operation::update_key_group::{builders::*, *};
use aws_sdk_cloudfront::operation::update_key_value_store::{builders::*, *};
use aws_sdk_cloudfront::operation::update_origin_access_control::{builders::*, *};
use aws_sdk_cloudfront::operation::update_origin_request_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::update_public_key::{builders::*, *};
use aws_sdk_cloudfront::operation::update_realtime_log_config::{builders::*, *};
use aws_sdk_cloudfront::operation::update_response_headers_policy::{builders::*, *};
use aws_sdk_cloudfront::operation::update_streaming_distribution::{builders::*, *};
use aws_sdk_cloudfront::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_cloudfront::Client;
use std::ops::Deref;

pub use aws_sdk_cloudfront::*;

pub struct CloudFrontClientImpl(Client);
impl CloudFrontClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CloudFrontClient {
    fn associate_alias(&self, builder: AssociateAliasInputBuilder) -> impl Future<Output = Result<AssociateAliasOutput, SdkError<AssociateAliasError>>> + Send;
    fn copy_distribution(&self, builder: CopyDistributionInputBuilder) -> impl Future<Output = Result<CopyDistributionOutput, SdkError<CopyDistributionError>>> + Send;
    fn create_cache_policy(&self, builder: CreateCachePolicyInputBuilder) -> impl Future<Output = Result<CreateCachePolicyOutput, SdkError<CreateCachePolicyError>>> + Send;
    fn create_cloud_front_origin_access_identity(&self, builder: CreateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<CreateCloudFrontOriginAccessIdentityOutput, SdkError<CreateCloudFrontOriginAccessIdentityError>>> + Send;
    fn create_continuous_deployment_policy(&self, builder: CreateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<CreateContinuousDeploymentPolicyOutput, SdkError<CreateContinuousDeploymentPolicyError>>> + Send;
    fn create_distribution(&self, builder: CreateDistributionInputBuilder) -> impl Future<Output = Result<CreateDistributionOutput, SdkError<CreateDistributionError>>> + Send;
    fn create_distribution_with_tags(&self, builder: CreateDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateDistributionWithTagsOutput, SdkError<CreateDistributionWithTagsError>>> + Send;
    fn create_field_level_encryption_config(&self, builder: CreateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionConfigOutput, SdkError<CreateFieldLevelEncryptionConfigError>>> + Send;
    fn create_field_level_encryption_profile(&self, builder: CreateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionProfileOutput, SdkError<CreateFieldLevelEncryptionProfileError>>> + Send;
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> + Send;
    fn create_invalidation(&self, builder: CreateInvalidationInputBuilder) -> impl Future<Output = Result<CreateInvalidationOutput, SdkError<CreateInvalidationError>>> + Send;
    fn create_key_group(&self, builder: CreateKeyGroupInputBuilder) -> impl Future<Output = Result<CreateKeyGroupOutput, SdkError<CreateKeyGroupError>>> + Send;
    fn create_key_value_store(&self, builder: CreateKeyValueStoreInputBuilder) -> impl Future<Output = Result<CreateKeyValueStoreOutput, SdkError<CreateKeyValueStoreError>>> + Send;
    fn create_monitoring_subscription(&self, builder: CreateMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<CreateMonitoringSubscriptionOutput, SdkError<CreateMonitoringSubscriptionError>>> + Send;
    fn create_origin_access_control(&self, builder: CreateOriginAccessControlInputBuilder) -> impl Future<Output = Result<CreateOriginAccessControlOutput, SdkError<CreateOriginAccessControlError>>> + Send;
    fn create_origin_request_policy(&self, builder: CreateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<CreateOriginRequestPolicyOutput, SdkError<CreateOriginRequestPolicyError>>> + Send;
    fn create_public_key(&self, builder: CreatePublicKeyInputBuilder) -> impl Future<Output = Result<CreatePublicKeyOutput, SdkError<CreatePublicKeyError>>> + Send;
    fn create_realtime_log_config(&self, builder: CreateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<CreateRealtimeLogConfigOutput, SdkError<CreateRealtimeLogConfigError>>> + Send;
    fn create_response_headers_policy(&self, builder: CreateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<CreateResponseHeadersPolicyOutput, SdkError<CreateResponseHeadersPolicyError>>> + Send;
    fn create_streaming_distribution(&self, builder: CreateStreamingDistributionInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionOutput, SdkError<CreateStreamingDistributionError>>> + Send;
    fn create_streaming_distribution_with_tags(&self, builder: CreateStreamingDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionWithTagsOutput, SdkError<CreateStreamingDistributionWithTagsError>>> + Send;
    fn delete_cache_policy(&self, builder: DeleteCachePolicyInputBuilder) -> impl Future<Output = Result<DeleteCachePolicyOutput, SdkError<DeleteCachePolicyError>>> + Send;
    fn delete_cloud_front_origin_access_identity(&self, builder: DeleteCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<DeleteCloudFrontOriginAccessIdentityOutput, SdkError<DeleteCloudFrontOriginAccessIdentityError>>> + Send;
    fn delete_continuous_deployment_policy(&self, builder: DeleteContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<DeleteContinuousDeploymentPolicyOutput, SdkError<DeleteContinuousDeploymentPolicyError>>> + Send;
    fn delete_distribution(&self, builder: DeleteDistributionInputBuilder) -> impl Future<Output = Result<DeleteDistributionOutput, SdkError<DeleteDistributionError>>> + Send;
    fn delete_field_level_encryption_config(&self, builder: DeleteFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionConfigOutput, SdkError<DeleteFieldLevelEncryptionConfigError>>> + Send;
    fn delete_field_level_encryption_profile(&self, builder: DeleteFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionProfileOutput, SdkError<DeleteFieldLevelEncryptionProfileError>>> + Send;
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> + Send;
    fn delete_key_group(&self, builder: DeleteKeyGroupInputBuilder) -> impl Future<Output = Result<DeleteKeyGroupOutput, SdkError<DeleteKeyGroupError>>> + Send;
    fn delete_key_value_store(&self, builder: DeleteKeyValueStoreInputBuilder) -> impl Future<Output = Result<DeleteKeyValueStoreOutput, SdkError<DeleteKeyValueStoreError>>> + Send;
    fn delete_monitoring_subscription(&self, builder: DeleteMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteMonitoringSubscriptionOutput, SdkError<DeleteMonitoringSubscriptionError>>> + Send;
    fn delete_origin_access_control(&self, builder: DeleteOriginAccessControlInputBuilder) -> impl Future<Output = Result<DeleteOriginAccessControlOutput, SdkError<DeleteOriginAccessControlError>>> + Send;
    fn delete_origin_request_policy(&self, builder: DeleteOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<DeleteOriginRequestPolicyOutput, SdkError<DeleteOriginRequestPolicyError>>> + Send;
    fn delete_public_key(&self, builder: DeletePublicKeyInputBuilder) -> impl Future<Output = Result<DeletePublicKeyOutput, SdkError<DeletePublicKeyError>>> + Send;
    fn delete_realtime_log_config(&self, builder: DeleteRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<DeleteRealtimeLogConfigOutput, SdkError<DeleteRealtimeLogConfigError>>> + Send;
    fn delete_response_headers_policy(&self, builder: DeleteResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<DeleteResponseHeadersPolicyOutput, SdkError<DeleteResponseHeadersPolicyError>>> + Send;
    fn delete_streaming_distribution(&self, builder: DeleteStreamingDistributionInputBuilder) -> impl Future<Output = Result<DeleteStreamingDistributionOutput, SdkError<DeleteStreamingDistributionError>>> + Send;
    fn describe_function(&self, builder: DescribeFunctionInputBuilder) -> impl Future<Output = Result<DescribeFunctionOutput, SdkError<DescribeFunctionError>>> + Send;
    fn describe_key_value_store(&self, builder: DescribeKeyValueStoreInputBuilder) -> impl Future<Output = Result<DescribeKeyValueStoreOutput, SdkError<DescribeKeyValueStoreError>>> + Send;
    fn get_cache_policy(&self, builder: GetCachePolicyInputBuilder) -> impl Future<Output = Result<GetCachePolicyOutput, SdkError<GetCachePolicyError>>> + Send;
    fn get_cache_policy_config(&self, builder: GetCachePolicyConfigInputBuilder) -> impl Future<Output = Result<GetCachePolicyConfigOutput, SdkError<GetCachePolicyConfigError>>> + Send;
    fn get_cloud_front_origin_access_identity(&self, builder: GetCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityOutput, SdkError<GetCloudFrontOriginAccessIdentityError>>> + Send;
    fn get_cloud_front_origin_access_identity_config(&self, builder: GetCloudFrontOriginAccessIdentityConfigInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityConfigOutput, SdkError<GetCloudFrontOriginAccessIdentityConfigError>>> + Send;
    fn get_continuous_deployment_policy(&self, builder: GetContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyOutput, SdkError<GetContinuousDeploymentPolicyError>>> + Send;
    fn get_continuous_deployment_policy_config(&self, builder: GetContinuousDeploymentPolicyConfigInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyConfigOutput, SdkError<GetContinuousDeploymentPolicyConfigError>>> + Send;
    fn get_distribution(&self, builder: GetDistributionInputBuilder) -> impl Future<Output = Result<GetDistributionOutput, SdkError<GetDistributionError>>> + Send;
    fn get_distribution_config(&self, builder: GetDistributionConfigInputBuilder) -> impl Future<Output = Result<GetDistributionConfigOutput, SdkError<GetDistributionConfigError>>> + Send;
    fn get_field_level_encryption(&self, builder: GetFieldLevelEncryptionInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionOutput, SdkError<GetFieldLevelEncryptionError>>> + Send;
    fn get_field_level_encryption_config(&self, builder: GetFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionConfigOutput, SdkError<GetFieldLevelEncryptionConfigError>>> + Send;
    fn get_field_level_encryption_profile(&self, builder: GetFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileOutput, SdkError<GetFieldLevelEncryptionProfileError>>> + Send;
    fn get_field_level_encryption_profile_config(&self, builder: GetFieldLevelEncryptionProfileConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileConfigOutput, SdkError<GetFieldLevelEncryptionProfileConfigError>>> + Send;
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> + Send;
    fn get_invalidation(&self, builder: GetInvalidationInputBuilder) -> impl Future<Output = Result<GetInvalidationOutput, SdkError<GetInvalidationError>>> + Send;
    fn get_key_group(&self, builder: GetKeyGroupInputBuilder) -> impl Future<Output = Result<GetKeyGroupOutput, SdkError<GetKeyGroupError>>> + Send;
    fn get_key_group_config(&self, builder: GetKeyGroupConfigInputBuilder) -> impl Future<Output = Result<GetKeyGroupConfigOutput, SdkError<GetKeyGroupConfigError>>> + Send;
    fn get_monitoring_subscription(&self, builder: GetMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<GetMonitoringSubscriptionOutput, SdkError<GetMonitoringSubscriptionError>>> + Send;
    fn get_origin_access_control(&self, builder: GetOriginAccessControlInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlOutput, SdkError<GetOriginAccessControlError>>> + Send;
    fn get_origin_access_control_config(&self, builder: GetOriginAccessControlConfigInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlConfigOutput, SdkError<GetOriginAccessControlConfigError>>> + Send;
    fn get_origin_request_policy(&self, builder: GetOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyOutput, SdkError<GetOriginRequestPolicyError>>> + Send;
    fn get_origin_request_policy_config(&self, builder: GetOriginRequestPolicyConfigInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyConfigOutput, SdkError<GetOriginRequestPolicyConfigError>>> + Send;
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> + Send;
    fn get_public_key_config(&self, builder: GetPublicKeyConfigInputBuilder) -> impl Future<Output = Result<GetPublicKeyConfigOutput, SdkError<GetPublicKeyConfigError>>> + Send;
    fn get_realtime_log_config(&self, builder: GetRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<GetRealtimeLogConfigOutput, SdkError<GetRealtimeLogConfigError>>> + Send;
    fn get_response_headers_policy(&self, builder: GetResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyOutput, SdkError<GetResponseHeadersPolicyError>>> + Send;
    fn get_response_headers_policy_config(&self, builder: GetResponseHeadersPolicyConfigInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyConfigOutput, SdkError<GetResponseHeadersPolicyConfigError>>> + Send;
    fn get_streaming_distribution(&self, builder: GetStreamingDistributionInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionOutput, SdkError<GetStreamingDistributionError>>> + Send;
    fn get_streaming_distribution_config(&self, builder: GetStreamingDistributionConfigInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionConfigOutput, SdkError<GetStreamingDistributionConfigError>>> + Send;
    fn list_cache_policies(&self, builder: ListCachePoliciesInputBuilder) -> impl Future<Output = Result<ListCachePoliciesOutput, SdkError<ListCachePoliciesError>>> + Send;
    fn list_cloud_front_origin_access_identities(&self, builder: ListCloudFrontOriginAccessIdentitiesInputBuilder) -> impl Future<Output = Result<ListCloudFrontOriginAccessIdentitiesOutput, SdkError<ListCloudFrontOriginAccessIdentitiesError>>> + Send;
    fn list_conflicting_aliases(&self, builder: ListConflictingAliasesInputBuilder) -> impl Future<Output = Result<ListConflictingAliasesOutput, SdkError<ListConflictingAliasesError>>> + Send;
    fn list_continuous_deployment_policies(&self, builder: ListContinuousDeploymentPoliciesInputBuilder) -> impl Future<Output = Result<ListContinuousDeploymentPoliciesOutput, SdkError<ListContinuousDeploymentPoliciesError>>> + Send;
    fn list_distributions(&self, builder: ListDistributionsInputBuilder) -> impl Future<Output = Result<ListDistributionsOutput, SdkError<ListDistributionsError>>> + Send;
    fn list_distributions_by_cache_policy_id(&self, builder: ListDistributionsByCachePolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByCachePolicyIdOutput, SdkError<ListDistributionsByCachePolicyIdError>>> + Send;
    fn list_distributions_by_key_group(&self, builder: ListDistributionsByKeyGroupInputBuilder) -> impl Future<Output = Result<ListDistributionsByKeyGroupOutput, SdkError<ListDistributionsByKeyGroupError>>> + Send;
    fn list_distributions_by_origin_request_policy_id(&self, builder: ListDistributionsByOriginRequestPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByOriginRequestPolicyIdOutput, SdkError<ListDistributionsByOriginRequestPolicyIdError>>> + Send;
    fn list_distributions_by_realtime_log_config(&self, builder: ListDistributionsByRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<ListDistributionsByRealtimeLogConfigOutput, SdkError<ListDistributionsByRealtimeLogConfigError>>> + Send;
    fn list_distributions_by_response_headers_policy_id(&self, builder: ListDistributionsByResponseHeadersPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByResponseHeadersPolicyIdOutput, SdkError<ListDistributionsByResponseHeadersPolicyIdError>>> + Send;
    fn list_distributions_by_web_acl_id(&self, builder: ListDistributionsByWebAclIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByWebAclIdOutput, SdkError<ListDistributionsByWebACLIdError>>> + Send;
    fn list_field_level_encryption_configs(&self, builder: ListFieldLevelEncryptionConfigsInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionConfigsOutput, SdkError<ListFieldLevelEncryptionConfigsError>>> + Send;
    fn list_field_level_encryption_profiles(&self, builder: ListFieldLevelEncryptionProfilesInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionProfilesOutput, SdkError<ListFieldLevelEncryptionProfilesError>>> + Send;
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> + Send;
    fn list_invalidations(&self, builder: ListInvalidationsInputBuilder) -> impl Future<Output = Result<ListInvalidationsOutput, SdkError<ListInvalidationsError>>> + Send;
    fn list_key_groups(&self, builder: ListKeyGroupsInputBuilder) -> impl Future<Output = Result<ListKeyGroupsOutput, SdkError<ListKeyGroupsError>>> + Send;
    fn list_key_value_stores(&self, builder: ListKeyValueStoresInputBuilder) -> impl Future<Output = Result<ListKeyValueStoresOutput, SdkError<ListKeyValueStoresError>>> + Send;
    fn list_origin_access_controls(&self, builder: ListOriginAccessControlsInputBuilder) -> impl Future<Output = Result<ListOriginAccessControlsOutput, SdkError<ListOriginAccessControlsError>>> + Send;
    fn list_origin_request_policies(&self, builder: ListOriginRequestPoliciesInputBuilder) -> impl Future<Output = Result<ListOriginRequestPoliciesOutput, SdkError<ListOriginRequestPoliciesError>>> + Send;
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>> + Send;
    fn list_realtime_log_configs(&self, builder: ListRealtimeLogConfigsInputBuilder) -> impl Future<Output = Result<ListRealtimeLogConfigsOutput, SdkError<ListRealtimeLogConfigsError>>> + Send;
    fn list_response_headers_policies(&self, builder: ListResponseHeadersPoliciesInputBuilder) -> impl Future<Output = Result<ListResponseHeadersPoliciesOutput, SdkError<ListResponseHeadersPoliciesError>>> + Send;
    fn list_streaming_distributions(&self, builder: ListStreamingDistributionsInputBuilder) -> impl Future<Output = Result<ListStreamingDistributionsOutput, SdkError<ListStreamingDistributionsError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn publish_function(&self, builder: PublishFunctionInputBuilder) -> impl Future<Output = Result<PublishFunctionOutput, SdkError<PublishFunctionError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn test_function(&self, builder: TestFunctionInputBuilder) -> impl Future<Output = Result<TestFunctionOutput, SdkError<TestFunctionError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_cache_policy(&self, builder: UpdateCachePolicyInputBuilder) -> impl Future<Output = Result<UpdateCachePolicyOutput, SdkError<UpdateCachePolicyError>>> + Send;
    fn update_cloud_front_origin_access_identity(&self, builder: UpdateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<UpdateCloudFrontOriginAccessIdentityOutput, SdkError<UpdateCloudFrontOriginAccessIdentityError>>> + Send;
    fn update_continuous_deployment_policy(&self, builder: UpdateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<UpdateContinuousDeploymentPolicyOutput, SdkError<UpdateContinuousDeploymentPolicyError>>> + Send;
    fn update_distribution(&self, builder: UpdateDistributionInputBuilder) -> impl Future<Output = Result<UpdateDistributionOutput, SdkError<UpdateDistributionError>>> + Send;
    fn update_distribution_with_staging_config(&self, builder: UpdateDistributionWithStagingConfigInputBuilder) -> impl Future<Output = Result<UpdateDistributionWithStagingConfigOutput, SdkError<UpdateDistributionWithStagingConfigError>>> + Send;
    fn update_field_level_encryption_config(&self, builder: UpdateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionConfigOutput, SdkError<UpdateFieldLevelEncryptionConfigError>>> + Send;
    fn update_field_level_encryption_profile(&self, builder: UpdateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionProfileOutput, SdkError<UpdateFieldLevelEncryptionProfileError>>> + Send;
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>> + Send;
    fn update_key_group(&self, builder: UpdateKeyGroupInputBuilder) -> impl Future<Output = Result<UpdateKeyGroupOutput, SdkError<UpdateKeyGroupError>>> + Send;
    fn update_key_value_store(&self, builder: UpdateKeyValueStoreInputBuilder) -> impl Future<Output = Result<UpdateKeyValueStoreOutput, SdkError<UpdateKeyValueStoreError>>> + Send;
    fn update_origin_access_control(&self, builder: UpdateOriginAccessControlInputBuilder) -> impl Future<Output = Result<UpdateOriginAccessControlOutput, SdkError<UpdateOriginAccessControlError>>> + Send;
    fn update_origin_request_policy(&self, builder: UpdateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<UpdateOriginRequestPolicyOutput, SdkError<UpdateOriginRequestPolicyError>>> + Send;
    fn update_public_key(&self, builder: UpdatePublicKeyInputBuilder) -> impl Future<Output = Result<UpdatePublicKeyOutput, SdkError<UpdatePublicKeyError>>> + Send;
    fn update_realtime_log_config(&self, builder: UpdateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<UpdateRealtimeLogConfigOutput, SdkError<UpdateRealtimeLogConfigError>>> + Send;
    fn update_response_headers_policy(&self, builder: UpdateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<UpdateResponseHeadersPolicyOutput, SdkError<UpdateResponseHeadersPolicyError>>> + Send;
    fn update_streaming_distribution(&self, builder: UpdateStreamingDistributionInputBuilder) -> impl Future<Output = Result<UpdateStreamingDistributionOutput, SdkError<UpdateStreamingDistributionError>>> + Send;
}
impl CloudFrontClient for CloudFrontClientImpl {
    fn associate_alias(&self, builder: AssociateAliasInputBuilder) -> impl Future<Output = Result<AssociateAliasOutput, SdkError<AssociateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn copy_distribution(&self, builder: CopyDistributionInputBuilder) -> impl Future<Output = Result<CopyDistributionOutput, SdkError<CopyDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn create_cache_policy(&self, builder: CreateCachePolicyInputBuilder) -> impl Future<Output = Result<CreateCachePolicyOutput, SdkError<CreateCachePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn create_cloud_front_origin_access_identity(&self, builder: CreateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<CreateCloudFrontOriginAccessIdentityOutput, SdkError<CreateCloudFrontOriginAccessIdentityError>>> {
        builder.send_with(&self.0)
    }
    fn create_continuous_deployment_policy(&self, builder: CreateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<CreateContinuousDeploymentPolicyOutput, SdkError<CreateContinuousDeploymentPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn create_distribution(&self, builder: CreateDistributionInputBuilder) -> impl Future<Output = Result<CreateDistributionOutput, SdkError<CreateDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn create_distribution_with_tags(&self, builder: CreateDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateDistributionWithTagsOutput, SdkError<CreateDistributionWithTagsError>>> {
        builder.send_with(&self.0)
    }
    fn create_field_level_encryption_config(&self, builder: CreateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionConfigOutput, SdkError<CreateFieldLevelEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_field_level_encryption_profile(&self, builder: CreateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionProfileOutput, SdkError<CreateFieldLevelEncryptionProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn create_invalidation(&self, builder: CreateInvalidationInputBuilder) -> impl Future<Output = Result<CreateInvalidationOutput, SdkError<CreateInvalidationError>>> {
        builder.send_with(&self.0)
    }
    fn create_key_group(&self, builder: CreateKeyGroupInputBuilder) -> impl Future<Output = Result<CreateKeyGroupOutput, SdkError<CreateKeyGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_key_value_store(&self, builder: CreateKeyValueStoreInputBuilder) -> impl Future<Output = Result<CreateKeyValueStoreOutput, SdkError<CreateKeyValueStoreError>>> {
        builder.send_with(&self.0)
    }
    fn create_monitoring_subscription(&self, builder: CreateMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<CreateMonitoringSubscriptionOutput, SdkError<CreateMonitoringSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_origin_access_control(&self, builder: CreateOriginAccessControlInputBuilder) -> impl Future<Output = Result<CreateOriginAccessControlOutput, SdkError<CreateOriginAccessControlError>>> {
        builder.send_with(&self.0)
    }
    fn create_origin_request_policy(&self, builder: CreateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<CreateOriginRequestPolicyOutput, SdkError<CreateOriginRequestPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn create_public_key(&self, builder: CreatePublicKeyInputBuilder) -> impl Future<Output = Result<CreatePublicKeyOutput, SdkError<CreatePublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn create_realtime_log_config(&self, builder: CreateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<CreateRealtimeLogConfigOutput, SdkError<CreateRealtimeLogConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_response_headers_policy(&self, builder: CreateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<CreateResponseHeadersPolicyOutput, SdkError<CreateResponseHeadersPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn create_streaming_distribution(&self, builder: CreateStreamingDistributionInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionOutput, SdkError<CreateStreamingDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn create_streaming_distribution_with_tags(&self, builder: CreateStreamingDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionWithTagsOutput, SdkError<CreateStreamingDistributionWithTagsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cache_policy(&self, builder: DeleteCachePolicyInputBuilder) -> impl Future<Output = Result<DeleteCachePolicyOutput, SdkError<DeleteCachePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cloud_front_origin_access_identity(&self, builder: DeleteCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<DeleteCloudFrontOriginAccessIdentityOutput, SdkError<DeleteCloudFrontOriginAccessIdentityError>>> {
        builder.send_with(&self.0)
    }
    fn delete_continuous_deployment_policy(&self, builder: DeleteContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<DeleteContinuousDeploymentPolicyOutput, SdkError<DeleteContinuousDeploymentPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_distribution(&self, builder: DeleteDistributionInputBuilder) -> impl Future<Output = Result<DeleteDistributionOutput, SdkError<DeleteDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_field_level_encryption_config(&self, builder: DeleteFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionConfigOutput, SdkError<DeleteFieldLevelEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_field_level_encryption_profile(&self, builder: DeleteFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionProfileOutput, SdkError<DeleteFieldLevelEncryptionProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_key_group(&self, builder: DeleteKeyGroupInputBuilder) -> impl Future<Output = Result<DeleteKeyGroupOutput, SdkError<DeleteKeyGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_key_value_store(&self, builder: DeleteKeyValueStoreInputBuilder) -> impl Future<Output = Result<DeleteKeyValueStoreOutput, SdkError<DeleteKeyValueStoreError>>> {
        builder.send_with(&self.0)
    }
    fn delete_monitoring_subscription(&self, builder: DeleteMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteMonitoringSubscriptionOutput, SdkError<DeleteMonitoringSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_origin_access_control(&self, builder: DeleteOriginAccessControlInputBuilder) -> impl Future<Output = Result<DeleteOriginAccessControlOutput, SdkError<DeleteOriginAccessControlError>>> {
        builder.send_with(&self.0)
    }
    fn delete_origin_request_policy(&self, builder: DeleteOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<DeleteOriginRequestPolicyOutput, SdkError<DeleteOriginRequestPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_public_key(&self, builder: DeletePublicKeyInputBuilder) -> impl Future<Output = Result<DeletePublicKeyOutput, SdkError<DeletePublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_realtime_log_config(&self, builder: DeleteRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<DeleteRealtimeLogConfigOutput, SdkError<DeleteRealtimeLogConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_response_headers_policy(&self, builder: DeleteResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<DeleteResponseHeadersPolicyOutput, SdkError<DeleteResponseHeadersPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_streaming_distribution(&self, builder: DeleteStreamingDistributionInputBuilder) -> impl Future<Output = Result<DeleteStreamingDistributionOutput, SdkError<DeleteStreamingDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_function(&self, builder: DescribeFunctionInputBuilder) -> impl Future<Output = Result<DescribeFunctionOutput, SdkError<DescribeFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_key_value_store(&self, builder: DescribeKeyValueStoreInputBuilder) -> impl Future<Output = Result<DescribeKeyValueStoreOutput, SdkError<DescribeKeyValueStoreError>>> {
        builder.send_with(&self.0)
    }
    fn get_cache_policy(&self, builder: GetCachePolicyInputBuilder) -> impl Future<Output = Result<GetCachePolicyOutput, SdkError<GetCachePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_cache_policy_config(&self, builder: GetCachePolicyConfigInputBuilder) -> impl Future<Output = Result<GetCachePolicyConfigOutput, SdkError<GetCachePolicyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_cloud_front_origin_access_identity(&self, builder: GetCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityOutput, SdkError<GetCloudFrontOriginAccessIdentityError>>> {
        builder.send_with(&self.0)
    }
    fn get_cloud_front_origin_access_identity_config(&self, builder: GetCloudFrontOriginAccessIdentityConfigInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityConfigOutput, SdkError<GetCloudFrontOriginAccessIdentityConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_continuous_deployment_policy(&self, builder: GetContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyOutput, SdkError<GetContinuousDeploymentPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_continuous_deployment_policy_config(&self, builder: GetContinuousDeploymentPolicyConfigInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyConfigOutput, SdkError<GetContinuousDeploymentPolicyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_distribution(&self, builder: GetDistributionInputBuilder) -> impl Future<Output = Result<GetDistributionOutput, SdkError<GetDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn get_distribution_config(&self, builder: GetDistributionConfigInputBuilder) -> impl Future<Output = Result<GetDistributionConfigOutput, SdkError<GetDistributionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_field_level_encryption(&self, builder: GetFieldLevelEncryptionInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionOutput, SdkError<GetFieldLevelEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn get_field_level_encryption_config(&self, builder: GetFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionConfigOutput, SdkError<GetFieldLevelEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_field_level_encryption_profile(&self, builder: GetFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileOutput, SdkError<GetFieldLevelEncryptionProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_field_level_encryption_profile_config(&self, builder: GetFieldLevelEncryptionProfileConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileConfigOutput, SdkError<GetFieldLevelEncryptionProfileConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn get_invalidation(&self, builder: GetInvalidationInputBuilder) -> impl Future<Output = Result<GetInvalidationOutput, SdkError<GetInvalidationError>>> {
        builder.send_with(&self.0)
    }
    fn get_key_group(&self, builder: GetKeyGroupInputBuilder) -> impl Future<Output = Result<GetKeyGroupOutput, SdkError<GetKeyGroupError>>> {
        builder.send_with(&self.0)
    }
    fn get_key_group_config(&self, builder: GetKeyGroupConfigInputBuilder) -> impl Future<Output = Result<GetKeyGroupConfigOutput, SdkError<GetKeyGroupConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_monitoring_subscription(&self, builder: GetMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<GetMonitoringSubscriptionOutput, SdkError<GetMonitoringSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn get_origin_access_control(&self, builder: GetOriginAccessControlInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlOutput, SdkError<GetOriginAccessControlError>>> {
        builder.send_with(&self.0)
    }
    fn get_origin_access_control_config(&self, builder: GetOriginAccessControlConfigInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlConfigOutput, SdkError<GetOriginAccessControlConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_origin_request_policy(&self, builder: GetOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyOutput, SdkError<GetOriginRequestPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_origin_request_policy_config(&self, builder: GetOriginRequestPolicyConfigInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyConfigOutput, SdkError<GetOriginRequestPolicyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn get_public_key_config(&self, builder: GetPublicKeyConfigInputBuilder) -> impl Future<Output = Result<GetPublicKeyConfigOutput, SdkError<GetPublicKeyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_realtime_log_config(&self, builder: GetRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<GetRealtimeLogConfigOutput, SdkError<GetRealtimeLogConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_response_headers_policy(&self, builder: GetResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyOutput, SdkError<GetResponseHeadersPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_response_headers_policy_config(&self, builder: GetResponseHeadersPolicyConfigInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyConfigOutput, SdkError<GetResponseHeadersPolicyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_streaming_distribution(&self, builder: GetStreamingDistributionInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionOutput, SdkError<GetStreamingDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn get_streaming_distribution_config(&self, builder: GetStreamingDistributionConfigInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionConfigOutput, SdkError<GetStreamingDistributionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn list_cache_policies(&self, builder: ListCachePoliciesInputBuilder) -> impl Future<Output = Result<ListCachePoliciesOutput, SdkError<ListCachePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_cloud_front_origin_access_identities(&self, builder: ListCloudFrontOriginAccessIdentitiesInputBuilder) -> impl Future<Output = Result<ListCloudFrontOriginAccessIdentitiesOutput, SdkError<ListCloudFrontOriginAccessIdentitiesError>>> {
        builder.send_with(&self.0)
    }
    fn list_conflicting_aliases(&self, builder: ListConflictingAliasesInputBuilder) -> impl Future<Output = Result<ListConflictingAliasesOutput, SdkError<ListConflictingAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_continuous_deployment_policies(&self, builder: ListContinuousDeploymentPoliciesInputBuilder) -> impl Future<Output = Result<ListContinuousDeploymentPoliciesOutput, SdkError<ListContinuousDeploymentPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions(&self, builder: ListDistributionsInputBuilder) -> impl Future<Output = Result<ListDistributionsOutput, SdkError<ListDistributionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_cache_policy_id(&self, builder: ListDistributionsByCachePolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByCachePolicyIdOutput, SdkError<ListDistributionsByCachePolicyIdError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_key_group(&self, builder: ListDistributionsByKeyGroupInputBuilder) -> impl Future<Output = Result<ListDistributionsByKeyGroupOutput, SdkError<ListDistributionsByKeyGroupError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_origin_request_policy_id(&self, builder: ListDistributionsByOriginRequestPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByOriginRequestPolicyIdOutput, SdkError<ListDistributionsByOriginRequestPolicyIdError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_realtime_log_config(&self, builder: ListDistributionsByRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<ListDistributionsByRealtimeLogConfigOutput, SdkError<ListDistributionsByRealtimeLogConfigError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_response_headers_policy_id(&self, builder: ListDistributionsByResponseHeadersPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByResponseHeadersPolicyIdOutput, SdkError<ListDistributionsByResponseHeadersPolicyIdError>>> {
        builder.send_with(&self.0)
    }
    fn list_distributions_by_web_acl_id(&self, builder: ListDistributionsByWebAclIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByWebAclIdOutput, SdkError<ListDistributionsByWebACLIdError>>> {
        builder.send_with(&self.0)
    }
    fn list_field_level_encryption_configs(&self, builder: ListFieldLevelEncryptionConfigsInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionConfigsOutput, SdkError<ListFieldLevelEncryptionConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_field_level_encryption_profiles(&self, builder: ListFieldLevelEncryptionProfilesInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionProfilesOutput, SdkError<ListFieldLevelEncryptionProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_invalidations(&self, builder: ListInvalidationsInputBuilder) -> impl Future<Output = Result<ListInvalidationsOutput, SdkError<ListInvalidationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_key_groups(&self, builder: ListKeyGroupsInputBuilder) -> impl Future<Output = Result<ListKeyGroupsOutput, SdkError<ListKeyGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_key_value_stores(&self, builder: ListKeyValueStoresInputBuilder) -> impl Future<Output = Result<ListKeyValueStoresOutput, SdkError<ListKeyValueStoresError>>> {
        builder.send_with(&self.0)
    }
    fn list_origin_access_controls(&self, builder: ListOriginAccessControlsInputBuilder) -> impl Future<Output = Result<ListOriginAccessControlsOutput, SdkError<ListOriginAccessControlsError>>> {
        builder.send_with(&self.0)
    }
    fn list_origin_request_policies(&self, builder: ListOriginRequestPoliciesInputBuilder) -> impl Future<Output = Result<ListOriginRequestPoliciesOutput, SdkError<ListOriginRequestPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_realtime_log_configs(&self, builder: ListRealtimeLogConfigsInputBuilder) -> impl Future<Output = Result<ListRealtimeLogConfigsOutput, SdkError<ListRealtimeLogConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_response_headers_policies(&self, builder: ListResponseHeadersPoliciesInputBuilder) -> impl Future<Output = Result<ListResponseHeadersPoliciesOutput, SdkError<ListResponseHeadersPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_streaming_distributions(&self, builder: ListStreamingDistributionsInputBuilder) -> impl Future<Output = Result<ListStreamingDistributionsOutput, SdkError<ListStreamingDistributionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn publish_function(&self, builder: PublishFunctionInputBuilder) -> impl Future<Output = Result<PublishFunctionOutput, SdkError<PublishFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn test_function(&self, builder: TestFunctionInputBuilder) -> impl Future<Output = Result<TestFunctionOutput, SdkError<TestFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_cache_policy(&self, builder: UpdateCachePolicyInputBuilder) -> impl Future<Output = Result<UpdateCachePolicyOutput, SdkError<UpdateCachePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_cloud_front_origin_access_identity(&self, builder: UpdateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<UpdateCloudFrontOriginAccessIdentityOutput, SdkError<UpdateCloudFrontOriginAccessIdentityError>>> {
        builder.send_with(&self.0)
    }
    fn update_continuous_deployment_policy(&self, builder: UpdateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<UpdateContinuousDeploymentPolicyOutput, SdkError<UpdateContinuousDeploymentPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_distribution(&self, builder: UpdateDistributionInputBuilder) -> impl Future<Output = Result<UpdateDistributionOutput, SdkError<UpdateDistributionError>>> {
        builder.send_with(&self.0)
    }
    fn update_distribution_with_staging_config(&self, builder: UpdateDistributionWithStagingConfigInputBuilder) -> impl Future<Output = Result<UpdateDistributionWithStagingConfigOutput, SdkError<UpdateDistributionWithStagingConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_field_level_encryption_config(&self, builder: UpdateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionConfigOutput, SdkError<UpdateFieldLevelEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_field_level_encryption_profile(&self, builder: UpdateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionProfileOutput, SdkError<UpdateFieldLevelEncryptionProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn update_key_group(&self, builder: UpdateKeyGroupInputBuilder) -> impl Future<Output = Result<UpdateKeyGroupOutput, SdkError<UpdateKeyGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_key_value_store(&self, builder: UpdateKeyValueStoreInputBuilder) -> impl Future<Output = Result<UpdateKeyValueStoreOutput, SdkError<UpdateKeyValueStoreError>>> {
        builder.send_with(&self.0)
    }
    fn update_origin_access_control(&self, builder: UpdateOriginAccessControlInputBuilder) -> impl Future<Output = Result<UpdateOriginAccessControlOutput, SdkError<UpdateOriginAccessControlError>>> {
        builder.send_with(&self.0)
    }
    fn update_origin_request_policy(&self, builder: UpdateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<UpdateOriginRequestPolicyOutput, SdkError<UpdateOriginRequestPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_public_key(&self, builder: UpdatePublicKeyInputBuilder) -> impl Future<Output = Result<UpdatePublicKeyOutput, SdkError<UpdatePublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_realtime_log_config(&self, builder: UpdateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<UpdateRealtimeLogConfigOutput, SdkError<UpdateRealtimeLogConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_response_headers_policy(&self, builder: UpdateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<UpdateResponseHeadersPolicyOutput, SdkError<UpdateResponseHeadersPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_streaming_distribution(&self, builder: UpdateStreamingDistributionInputBuilder) -> impl Future<Output = Result<UpdateStreamingDistributionOutput, SdkError<UpdateStreamingDistributionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> CloudFrontClient for T
where T: Deref,
      T::Target: CloudFrontClient {
    fn associate_alias(&self, builder: AssociateAliasInputBuilder) -> impl Future<Output = Result<AssociateAliasOutput, SdkError<AssociateAliasError>>> {
        self.deref().associate_alias(builder)
    }
    fn copy_distribution(&self, builder: CopyDistributionInputBuilder) -> impl Future<Output = Result<CopyDistributionOutput, SdkError<CopyDistributionError>>> {
        self.deref().copy_distribution(builder)
    }
    fn create_cache_policy(&self, builder: CreateCachePolicyInputBuilder) -> impl Future<Output = Result<CreateCachePolicyOutput, SdkError<CreateCachePolicyError>>> {
        self.deref().create_cache_policy(builder)
    }
    fn create_cloud_front_origin_access_identity(&self, builder: CreateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<CreateCloudFrontOriginAccessIdentityOutput, SdkError<CreateCloudFrontOriginAccessIdentityError>>> {
        self.deref().create_cloud_front_origin_access_identity(builder)
    }
    fn create_continuous_deployment_policy(&self, builder: CreateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<CreateContinuousDeploymentPolicyOutput, SdkError<CreateContinuousDeploymentPolicyError>>> {
        self.deref().create_continuous_deployment_policy(builder)
    }
    fn create_distribution(&self, builder: CreateDistributionInputBuilder) -> impl Future<Output = Result<CreateDistributionOutput, SdkError<CreateDistributionError>>> {
        self.deref().create_distribution(builder)
    }
    fn create_distribution_with_tags(&self, builder: CreateDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateDistributionWithTagsOutput, SdkError<CreateDistributionWithTagsError>>> {
        self.deref().create_distribution_with_tags(builder)
    }
    fn create_field_level_encryption_config(&self, builder: CreateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionConfigOutput, SdkError<CreateFieldLevelEncryptionConfigError>>> {
        self.deref().create_field_level_encryption_config(builder)
    }
    fn create_field_level_encryption_profile(&self, builder: CreateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<CreateFieldLevelEncryptionProfileOutput, SdkError<CreateFieldLevelEncryptionProfileError>>> {
        self.deref().create_field_level_encryption_profile(builder)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        self.deref().create_function(builder)
    }
    fn create_invalidation(&self, builder: CreateInvalidationInputBuilder) -> impl Future<Output = Result<CreateInvalidationOutput, SdkError<CreateInvalidationError>>> {
        self.deref().create_invalidation(builder)
    }
    fn create_key_group(&self, builder: CreateKeyGroupInputBuilder) -> impl Future<Output = Result<CreateKeyGroupOutput, SdkError<CreateKeyGroupError>>> {
        self.deref().create_key_group(builder)
    }
    fn create_key_value_store(&self, builder: CreateKeyValueStoreInputBuilder) -> impl Future<Output = Result<CreateKeyValueStoreOutput, SdkError<CreateKeyValueStoreError>>> {
        self.deref().create_key_value_store(builder)
    }
    fn create_monitoring_subscription(&self, builder: CreateMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<CreateMonitoringSubscriptionOutput, SdkError<CreateMonitoringSubscriptionError>>> {
        self.deref().create_monitoring_subscription(builder)
    }
    fn create_origin_access_control(&self, builder: CreateOriginAccessControlInputBuilder) -> impl Future<Output = Result<CreateOriginAccessControlOutput, SdkError<CreateOriginAccessControlError>>> {
        self.deref().create_origin_access_control(builder)
    }
    fn create_origin_request_policy(&self, builder: CreateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<CreateOriginRequestPolicyOutput, SdkError<CreateOriginRequestPolicyError>>> {
        self.deref().create_origin_request_policy(builder)
    }
    fn create_public_key(&self, builder: CreatePublicKeyInputBuilder) -> impl Future<Output = Result<CreatePublicKeyOutput, SdkError<CreatePublicKeyError>>> {
        self.deref().create_public_key(builder)
    }
    fn create_realtime_log_config(&self, builder: CreateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<CreateRealtimeLogConfigOutput, SdkError<CreateRealtimeLogConfigError>>> {
        self.deref().create_realtime_log_config(builder)
    }
    fn create_response_headers_policy(&self, builder: CreateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<CreateResponseHeadersPolicyOutput, SdkError<CreateResponseHeadersPolicyError>>> {
        self.deref().create_response_headers_policy(builder)
    }
    fn create_streaming_distribution(&self, builder: CreateStreamingDistributionInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionOutput, SdkError<CreateStreamingDistributionError>>> {
        self.deref().create_streaming_distribution(builder)
    }
    fn create_streaming_distribution_with_tags(&self, builder: CreateStreamingDistributionWithTagsInputBuilder) -> impl Future<Output = Result<CreateStreamingDistributionWithTagsOutput, SdkError<CreateStreamingDistributionWithTagsError>>> {
        self.deref().create_streaming_distribution_with_tags(builder)
    }
    fn delete_cache_policy(&self, builder: DeleteCachePolicyInputBuilder) -> impl Future<Output = Result<DeleteCachePolicyOutput, SdkError<DeleteCachePolicyError>>> {
        self.deref().delete_cache_policy(builder)
    }
    fn delete_cloud_front_origin_access_identity(&self, builder: DeleteCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<DeleteCloudFrontOriginAccessIdentityOutput, SdkError<DeleteCloudFrontOriginAccessIdentityError>>> {
        self.deref().delete_cloud_front_origin_access_identity(builder)
    }
    fn delete_continuous_deployment_policy(&self, builder: DeleteContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<DeleteContinuousDeploymentPolicyOutput, SdkError<DeleteContinuousDeploymentPolicyError>>> {
        self.deref().delete_continuous_deployment_policy(builder)
    }
    fn delete_distribution(&self, builder: DeleteDistributionInputBuilder) -> impl Future<Output = Result<DeleteDistributionOutput, SdkError<DeleteDistributionError>>> {
        self.deref().delete_distribution(builder)
    }
    fn delete_field_level_encryption_config(&self, builder: DeleteFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionConfigOutput, SdkError<DeleteFieldLevelEncryptionConfigError>>> {
        self.deref().delete_field_level_encryption_config(builder)
    }
    fn delete_field_level_encryption_profile(&self, builder: DeleteFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<DeleteFieldLevelEncryptionProfileOutput, SdkError<DeleteFieldLevelEncryptionProfileError>>> {
        self.deref().delete_field_level_encryption_profile(builder)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        self.deref().delete_function(builder)
    }
    fn delete_key_group(&self, builder: DeleteKeyGroupInputBuilder) -> impl Future<Output = Result<DeleteKeyGroupOutput, SdkError<DeleteKeyGroupError>>> {
        self.deref().delete_key_group(builder)
    }
    fn delete_key_value_store(&self, builder: DeleteKeyValueStoreInputBuilder) -> impl Future<Output = Result<DeleteKeyValueStoreOutput, SdkError<DeleteKeyValueStoreError>>> {
        self.deref().delete_key_value_store(builder)
    }
    fn delete_monitoring_subscription(&self, builder: DeleteMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteMonitoringSubscriptionOutput, SdkError<DeleteMonitoringSubscriptionError>>> {
        self.deref().delete_monitoring_subscription(builder)
    }
    fn delete_origin_access_control(&self, builder: DeleteOriginAccessControlInputBuilder) -> impl Future<Output = Result<DeleteOriginAccessControlOutput, SdkError<DeleteOriginAccessControlError>>> {
        self.deref().delete_origin_access_control(builder)
    }
    fn delete_origin_request_policy(&self, builder: DeleteOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<DeleteOriginRequestPolicyOutput, SdkError<DeleteOriginRequestPolicyError>>> {
        self.deref().delete_origin_request_policy(builder)
    }
    fn delete_public_key(&self, builder: DeletePublicKeyInputBuilder) -> impl Future<Output = Result<DeletePublicKeyOutput, SdkError<DeletePublicKeyError>>> {
        self.deref().delete_public_key(builder)
    }
    fn delete_realtime_log_config(&self, builder: DeleteRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<DeleteRealtimeLogConfigOutput, SdkError<DeleteRealtimeLogConfigError>>> {
        self.deref().delete_realtime_log_config(builder)
    }
    fn delete_response_headers_policy(&self, builder: DeleteResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<DeleteResponseHeadersPolicyOutput, SdkError<DeleteResponseHeadersPolicyError>>> {
        self.deref().delete_response_headers_policy(builder)
    }
    fn delete_streaming_distribution(&self, builder: DeleteStreamingDistributionInputBuilder) -> impl Future<Output = Result<DeleteStreamingDistributionOutput, SdkError<DeleteStreamingDistributionError>>> {
        self.deref().delete_streaming_distribution(builder)
    }
    fn describe_function(&self, builder: DescribeFunctionInputBuilder) -> impl Future<Output = Result<DescribeFunctionOutput, SdkError<DescribeFunctionError>>> {
        self.deref().describe_function(builder)
    }
    fn describe_key_value_store(&self, builder: DescribeKeyValueStoreInputBuilder) -> impl Future<Output = Result<DescribeKeyValueStoreOutput, SdkError<DescribeKeyValueStoreError>>> {
        self.deref().describe_key_value_store(builder)
    }
    fn get_cache_policy(&self, builder: GetCachePolicyInputBuilder) -> impl Future<Output = Result<GetCachePolicyOutput, SdkError<GetCachePolicyError>>> {
        self.deref().get_cache_policy(builder)
    }
    fn get_cache_policy_config(&self, builder: GetCachePolicyConfigInputBuilder) -> impl Future<Output = Result<GetCachePolicyConfigOutput, SdkError<GetCachePolicyConfigError>>> {
        self.deref().get_cache_policy_config(builder)
    }
    fn get_cloud_front_origin_access_identity(&self, builder: GetCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityOutput, SdkError<GetCloudFrontOriginAccessIdentityError>>> {
        self.deref().get_cloud_front_origin_access_identity(builder)
    }
    fn get_cloud_front_origin_access_identity_config(&self, builder: GetCloudFrontOriginAccessIdentityConfigInputBuilder) -> impl Future<Output = Result<GetCloudFrontOriginAccessIdentityConfigOutput, SdkError<GetCloudFrontOriginAccessIdentityConfigError>>> {
        self.deref().get_cloud_front_origin_access_identity_config(builder)
    }
    fn get_continuous_deployment_policy(&self, builder: GetContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyOutput, SdkError<GetContinuousDeploymentPolicyError>>> {
        self.deref().get_continuous_deployment_policy(builder)
    }
    fn get_continuous_deployment_policy_config(&self, builder: GetContinuousDeploymentPolicyConfigInputBuilder) -> impl Future<Output = Result<GetContinuousDeploymentPolicyConfigOutput, SdkError<GetContinuousDeploymentPolicyConfigError>>> {
        self.deref().get_continuous_deployment_policy_config(builder)
    }
    fn get_distribution(&self, builder: GetDistributionInputBuilder) -> impl Future<Output = Result<GetDistributionOutput, SdkError<GetDistributionError>>> {
        self.deref().get_distribution(builder)
    }
    fn get_distribution_config(&self, builder: GetDistributionConfigInputBuilder) -> impl Future<Output = Result<GetDistributionConfigOutput, SdkError<GetDistributionConfigError>>> {
        self.deref().get_distribution_config(builder)
    }
    fn get_field_level_encryption(&self, builder: GetFieldLevelEncryptionInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionOutput, SdkError<GetFieldLevelEncryptionError>>> {
        self.deref().get_field_level_encryption(builder)
    }
    fn get_field_level_encryption_config(&self, builder: GetFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionConfigOutput, SdkError<GetFieldLevelEncryptionConfigError>>> {
        self.deref().get_field_level_encryption_config(builder)
    }
    fn get_field_level_encryption_profile(&self, builder: GetFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileOutput, SdkError<GetFieldLevelEncryptionProfileError>>> {
        self.deref().get_field_level_encryption_profile(builder)
    }
    fn get_field_level_encryption_profile_config(&self, builder: GetFieldLevelEncryptionProfileConfigInputBuilder) -> impl Future<Output = Result<GetFieldLevelEncryptionProfileConfigOutput, SdkError<GetFieldLevelEncryptionProfileConfigError>>> {
        self.deref().get_field_level_encryption_profile_config(builder)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        self.deref().get_function(builder)
    }
    fn get_invalidation(&self, builder: GetInvalidationInputBuilder) -> impl Future<Output = Result<GetInvalidationOutput, SdkError<GetInvalidationError>>> {
        self.deref().get_invalidation(builder)
    }
    fn get_key_group(&self, builder: GetKeyGroupInputBuilder) -> impl Future<Output = Result<GetKeyGroupOutput, SdkError<GetKeyGroupError>>> {
        self.deref().get_key_group(builder)
    }
    fn get_key_group_config(&self, builder: GetKeyGroupConfigInputBuilder) -> impl Future<Output = Result<GetKeyGroupConfigOutput, SdkError<GetKeyGroupConfigError>>> {
        self.deref().get_key_group_config(builder)
    }
    fn get_monitoring_subscription(&self, builder: GetMonitoringSubscriptionInputBuilder) -> impl Future<Output = Result<GetMonitoringSubscriptionOutput, SdkError<GetMonitoringSubscriptionError>>> {
        self.deref().get_monitoring_subscription(builder)
    }
    fn get_origin_access_control(&self, builder: GetOriginAccessControlInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlOutput, SdkError<GetOriginAccessControlError>>> {
        self.deref().get_origin_access_control(builder)
    }
    fn get_origin_access_control_config(&self, builder: GetOriginAccessControlConfigInputBuilder) -> impl Future<Output = Result<GetOriginAccessControlConfigOutput, SdkError<GetOriginAccessControlConfigError>>> {
        self.deref().get_origin_access_control_config(builder)
    }
    fn get_origin_request_policy(&self, builder: GetOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyOutput, SdkError<GetOriginRequestPolicyError>>> {
        self.deref().get_origin_request_policy(builder)
    }
    fn get_origin_request_policy_config(&self, builder: GetOriginRequestPolicyConfigInputBuilder) -> impl Future<Output = Result<GetOriginRequestPolicyConfigOutput, SdkError<GetOriginRequestPolicyConfigError>>> {
        self.deref().get_origin_request_policy_config(builder)
    }
    fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> impl Future<Output = Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>> {
        self.deref().get_public_key(builder)
    }
    fn get_public_key_config(&self, builder: GetPublicKeyConfigInputBuilder) -> impl Future<Output = Result<GetPublicKeyConfigOutput, SdkError<GetPublicKeyConfigError>>> {
        self.deref().get_public_key_config(builder)
    }
    fn get_realtime_log_config(&self, builder: GetRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<GetRealtimeLogConfigOutput, SdkError<GetRealtimeLogConfigError>>> {
        self.deref().get_realtime_log_config(builder)
    }
    fn get_response_headers_policy(&self, builder: GetResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyOutput, SdkError<GetResponseHeadersPolicyError>>> {
        self.deref().get_response_headers_policy(builder)
    }
    fn get_response_headers_policy_config(&self, builder: GetResponseHeadersPolicyConfigInputBuilder) -> impl Future<Output = Result<GetResponseHeadersPolicyConfigOutput, SdkError<GetResponseHeadersPolicyConfigError>>> {
        self.deref().get_response_headers_policy_config(builder)
    }
    fn get_streaming_distribution(&self, builder: GetStreamingDistributionInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionOutput, SdkError<GetStreamingDistributionError>>> {
        self.deref().get_streaming_distribution(builder)
    }
    fn get_streaming_distribution_config(&self, builder: GetStreamingDistributionConfigInputBuilder) -> impl Future<Output = Result<GetStreamingDistributionConfigOutput, SdkError<GetStreamingDistributionConfigError>>> {
        self.deref().get_streaming_distribution_config(builder)
    }
    fn list_cache_policies(&self, builder: ListCachePoliciesInputBuilder) -> impl Future<Output = Result<ListCachePoliciesOutput, SdkError<ListCachePoliciesError>>> {
        self.deref().list_cache_policies(builder)
    }
    fn list_cloud_front_origin_access_identities(&self, builder: ListCloudFrontOriginAccessIdentitiesInputBuilder) -> impl Future<Output = Result<ListCloudFrontOriginAccessIdentitiesOutput, SdkError<ListCloudFrontOriginAccessIdentitiesError>>> {
        self.deref().list_cloud_front_origin_access_identities(builder)
    }
    fn list_conflicting_aliases(&self, builder: ListConflictingAliasesInputBuilder) -> impl Future<Output = Result<ListConflictingAliasesOutput, SdkError<ListConflictingAliasesError>>> {
        self.deref().list_conflicting_aliases(builder)
    }
    fn list_continuous_deployment_policies(&self, builder: ListContinuousDeploymentPoliciesInputBuilder) -> impl Future<Output = Result<ListContinuousDeploymentPoliciesOutput, SdkError<ListContinuousDeploymentPoliciesError>>> {
        self.deref().list_continuous_deployment_policies(builder)
    }
    fn list_distributions(&self, builder: ListDistributionsInputBuilder) -> impl Future<Output = Result<ListDistributionsOutput, SdkError<ListDistributionsError>>> {
        self.deref().list_distributions(builder)
    }
    fn list_distributions_by_cache_policy_id(&self, builder: ListDistributionsByCachePolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByCachePolicyIdOutput, SdkError<ListDistributionsByCachePolicyIdError>>> {
        self.deref().list_distributions_by_cache_policy_id(builder)
    }
    fn list_distributions_by_key_group(&self, builder: ListDistributionsByKeyGroupInputBuilder) -> impl Future<Output = Result<ListDistributionsByKeyGroupOutput, SdkError<ListDistributionsByKeyGroupError>>> {
        self.deref().list_distributions_by_key_group(builder)
    }
    fn list_distributions_by_origin_request_policy_id(&self, builder: ListDistributionsByOriginRequestPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByOriginRequestPolicyIdOutput, SdkError<ListDistributionsByOriginRequestPolicyIdError>>> {
        self.deref().list_distributions_by_origin_request_policy_id(builder)
    }
    fn list_distributions_by_realtime_log_config(&self, builder: ListDistributionsByRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<ListDistributionsByRealtimeLogConfigOutput, SdkError<ListDistributionsByRealtimeLogConfigError>>> {
        self.deref().list_distributions_by_realtime_log_config(builder)
    }
    fn list_distributions_by_response_headers_policy_id(&self, builder: ListDistributionsByResponseHeadersPolicyIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByResponseHeadersPolicyIdOutput, SdkError<ListDistributionsByResponseHeadersPolicyIdError>>> {
        self.deref().list_distributions_by_response_headers_policy_id(builder)
    }
    fn list_distributions_by_web_acl_id(&self, builder: ListDistributionsByWebAclIdInputBuilder) -> impl Future<Output = Result<ListDistributionsByWebAclIdOutput, SdkError<ListDistributionsByWebACLIdError>>> {
        self.deref().list_distributions_by_web_acl_id(builder)
    }
    fn list_field_level_encryption_configs(&self, builder: ListFieldLevelEncryptionConfigsInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionConfigsOutput, SdkError<ListFieldLevelEncryptionConfigsError>>> {
        self.deref().list_field_level_encryption_configs(builder)
    }
    fn list_field_level_encryption_profiles(&self, builder: ListFieldLevelEncryptionProfilesInputBuilder) -> impl Future<Output = Result<ListFieldLevelEncryptionProfilesOutput, SdkError<ListFieldLevelEncryptionProfilesError>>> {
        self.deref().list_field_level_encryption_profiles(builder)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        self.deref().list_functions(builder)
    }
    fn list_invalidations(&self, builder: ListInvalidationsInputBuilder) -> impl Future<Output = Result<ListInvalidationsOutput, SdkError<ListInvalidationsError>>> {
        self.deref().list_invalidations(builder)
    }
    fn list_key_groups(&self, builder: ListKeyGroupsInputBuilder) -> impl Future<Output = Result<ListKeyGroupsOutput, SdkError<ListKeyGroupsError>>> {
        self.deref().list_key_groups(builder)
    }
    fn list_key_value_stores(&self, builder: ListKeyValueStoresInputBuilder) -> impl Future<Output = Result<ListKeyValueStoresOutput, SdkError<ListKeyValueStoresError>>> {
        self.deref().list_key_value_stores(builder)
    }
    fn list_origin_access_controls(&self, builder: ListOriginAccessControlsInputBuilder) -> impl Future<Output = Result<ListOriginAccessControlsOutput, SdkError<ListOriginAccessControlsError>>> {
        self.deref().list_origin_access_controls(builder)
    }
    fn list_origin_request_policies(&self, builder: ListOriginRequestPoliciesInputBuilder) -> impl Future<Output = Result<ListOriginRequestPoliciesOutput, SdkError<ListOriginRequestPoliciesError>>> {
        self.deref().list_origin_request_policies(builder)
    }
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>> {
        self.deref().list_public_keys(builder)
    }
    fn list_realtime_log_configs(&self, builder: ListRealtimeLogConfigsInputBuilder) -> impl Future<Output = Result<ListRealtimeLogConfigsOutput, SdkError<ListRealtimeLogConfigsError>>> {
        self.deref().list_realtime_log_configs(builder)
    }
    fn list_response_headers_policies(&self, builder: ListResponseHeadersPoliciesInputBuilder) -> impl Future<Output = Result<ListResponseHeadersPoliciesOutput, SdkError<ListResponseHeadersPoliciesError>>> {
        self.deref().list_response_headers_policies(builder)
    }
    fn list_streaming_distributions(&self, builder: ListStreamingDistributionsInputBuilder) -> impl Future<Output = Result<ListStreamingDistributionsOutput, SdkError<ListStreamingDistributionsError>>> {
        self.deref().list_streaming_distributions(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn publish_function(&self, builder: PublishFunctionInputBuilder) -> impl Future<Output = Result<PublishFunctionOutput, SdkError<PublishFunctionError>>> {
        self.deref().publish_function(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn test_function(&self, builder: TestFunctionInputBuilder) -> impl Future<Output = Result<TestFunctionOutput, SdkError<TestFunctionError>>> {
        self.deref().test_function(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_cache_policy(&self, builder: UpdateCachePolicyInputBuilder) -> impl Future<Output = Result<UpdateCachePolicyOutput, SdkError<UpdateCachePolicyError>>> {
        self.deref().update_cache_policy(builder)
    }
    fn update_cloud_front_origin_access_identity(&self, builder: UpdateCloudFrontOriginAccessIdentityInputBuilder) -> impl Future<Output = Result<UpdateCloudFrontOriginAccessIdentityOutput, SdkError<UpdateCloudFrontOriginAccessIdentityError>>> {
        self.deref().update_cloud_front_origin_access_identity(builder)
    }
    fn update_continuous_deployment_policy(&self, builder: UpdateContinuousDeploymentPolicyInputBuilder) -> impl Future<Output = Result<UpdateContinuousDeploymentPolicyOutput, SdkError<UpdateContinuousDeploymentPolicyError>>> {
        self.deref().update_continuous_deployment_policy(builder)
    }
    fn update_distribution(&self, builder: UpdateDistributionInputBuilder) -> impl Future<Output = Result<UpdateDistributionOutput, SdkError<UpdateDistributionError>>> {
        self.deref().update_distribution(builder)
    }
    fn update_distribution_with_staging_config(&self, builder: UpdateDistributionWithStagingConfigInputBuilder) -> impl Future<Output = Result<UpdateDistributionWithStagingConfigOutput, SdkError<UpdateDistributionWithStagingConfigError>>> {
        self.deref().update_distribution_with_staging_config(builder)
    }
    fn update_field_level_encryption_config(&self, builder: UpdateFieldLevelEncryptionConfigInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionConfigOutput, SdkError<UpdateFieldLevelEncryptionConfigError>>> {
        self.deref().update_field_level_encryption_config(builder)
    }
    fn update_field_level_encryption_profile(&self, builder: UpdateFieldLevelEncryptionProfileInputBuilder) -> impl Future<Output = Result<UpdateFieldLevelEncryptionProfileOutput, SdkError<UpdateFieldLevelEncryptionProfileError>>> {
        self.deref().update_field_level_encryption_profile(builder)
    }
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>> {
        self.deref().update_function(builder)
    }
    fn update_key_group(&self, builder: UpdateKeyGroupInputBuilder) -> impl Future<Output = Result<UpdateKeyGroupOutput, SdkError<UpdateKeyGroupError>>> {
        self.deref().update_key_group(builder)
    }
    fn update_key_value_store(&self, builder: UpdateKeyValueStoreInputBuilder) -> impl Future<Output = Result<UpdateKeyValueStoreOutput, SdkError<UpdateKeyValueStoreError>>> {
        self.deref().update_key_value_store(builder)
    }
    fn update_origin_access_control(&self, builder: UpdateOriginAccessControlInputBuilder) -> impl Future<Output = Result<UpdateOriginAccessControlOutput, SdkError<UpdateOriginAccessControlError>>> {
        self.deref().update_origin_access_control(builder)
    }
    fn update_origin_request_policy(&self, builder: UpdateOriginRequestPolicyInputBuilder) -> impl Future<Output = Result<UpdateOriginRequestPolicyOutput, SdkError<UpdateOriginRequestPolicyError>>> {
        self.deref().update_origin_request_policy(builder)
    }
    fn update_public_key(&self, builder: UpdatePublicKeyInputBuilder) -> impl Future<Output = Result<UpdatePublicKeyOutput, SdkError<UpdatePublicKeyError>>> {
        self.deref().update_public_key(builder)
    }
    fn update_realtime_log_config(&self, builder: UpdateRealtimeLogConfigInputBuilder) -> impl Future<Output = Result<UpdateRealtimeLogConfigOutput, SdkError<UpdateRealtimeLogConfigError>>> {
        self.deref().update_realtime_log_config(builder)
    }
    fn update_response_headers_policy(&self, builder: UpdateResponseHeadersPolicyInputBuilder) -> impl Future<Output = Result<UpdateResponseHeadersPolicyOutput, SdkError<UpdateResponseHeadersPolicyError>>> {
        self.deref().update_response_headers_policy(builder)
    }
    fn update_streaming_distribution(&self, builder: UpdateStreamingDistributionInputBuilder) -> impl Future<Output = Result<UpdateStreamingDistributionOutput, SdkError<UpdateStreamingDistributionError>>> {
        self.deref().update_streaming_distribution(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCloudFrontClient {}
    impl CloudFrontClient for edCloudFrontClient {
        async fn associate_alias(&self, builder: AssociateAliasInputBuilder) -> Result<AssociateAliasOutput, SdkError<AssociateAliasError>>;
        async fn copy_distribution(&self, builder: CopyDistributionInputBuilder) -> Result<CopyDistributionOutput, SdkError<CopyDistributionError>>;
        async fn create_cache_policy(&self, builder: CreateCachePolicyInputBuilder) -> Result<CreateCachePolicyOutput, SdkError<CreateCachePolicyError>>;
        async fn create_cloud_front_origin_access_identity(&self, builder: CreateCloudFrontOriginAccessIdentityInputBuilder) -> Result<CreateCloudFrontOriginAccessIdentityOutput, SdkError<CreateCloudFrontOriginAccessIdentityError>>;
        async fn create_continuous_deployment_policy(&self, builder: CreateContinuousDeploymentPolicyInputBuilder) -> Result<CreateContinuousDeploymentPolicyOutput, SdkError<CreateContinuousDeploymentPolicyError>>;
        async fn create_distribution(&self, builder: CreateDistributionInputBuilder) -> Result<CreateDistributionOutput, SdkError<CreateDistributionError>>;
        async fn create_distribution_with_tags(&self, builder: CreateDistributionWithTagsInputBuilder) -> Result<CreateDistributionWithTagsOutput, SdkError<CreateDistributionWithTagsError>>;
        async fn create_field_level_encryption_config(&self, builder: CreateFieldLevelEncryptionConfigInputBuilder) -> Result<CreateFieldLevelEncryptionConfigOutput, SdkError<CreateFieldLevelEncryptionConfigError>>;
        async fn create_field_level_encryption_profile(&self, builder: CreateFieldLevelEncryptionProfileInputBuilder) -> Result<CreateFieldLevelEncryptionProfileOutput, SdkError<CreateFieldLevelEncryptionProfileError>>;
        async fn create_function(&self, builder: CreateFunctionInputBuilder) -> Result<CreateFunctionOutput, SdkError<CreateFunctionError>>;
        async fn create_invalidation(&self, builder: CreateInvalidationInputBuilder) -> Result<CreateInvalidationOutput, SdkError<CreateInvalidationError>>;
        async fn create_key_group(&self, builder: CreateKeyGroupInputBuilder) -> Result<CreateKeyGroupOutput, SdkError<CreateKeyGroupError>>;
        async fn create_key_value_store(&self, builder: CreateKeyValueStoreInputBuilder) -> Result<CreateKeyValueStoreOutput, SdkError<CreateKeyValueStoreError>>;
        async fn create_monitoring_subscription(&self, builder: CreateMonitoringSubscriptionInputBuilder) -> Result<CreateMonitoringSubscriptionOutput, SdkError<CreateMonitoringSubscriptionError>>;
        async fn create_origin_access_control(&self, builder: CreateOriginAccessControlInputBuilder) -> Result<CreateOriginAccessControlOutput, SdkError<CreateOriginAccessControlError>>;
        async fn create_origin_request_policy(&self, builder: CreateOriginRequestPolicyInputBuilder) -> Result<CreateOriginRequestPolicyOutput, SdkError<CreateOriginRequestPolicyError>>;
        async fn create_public_key(&self, builder: CreatePublicKeyInputBuilder) -> Result<CreatePublicKeyOutput, SdkError<CreatePublicKeyError>>;
        async fn create_realtime_log_config(&self, builder: CreateRealtimeLogConfigInputBuilder) -> Result<CreateRealtimeLogConfigOutput, SdkError<CreateRealtimeLogConfigError>>;
        async fn create_response_headers_policy(&self, builder: CreateResponseHeadersPolicyInputBuilder) -> Result<CreateResponseHeadersPolicyOutput, SdkError<CreateResponseHeadersPolicyError>>;
        async fn create_streaming_distribution(&self, builder: CreateStreamingDistributionInputBuilder) -> Result<CreateStreamingDistributionOutput, SdkError<CreateStreamingDistributionError>>;
        async fn create_streaming_distribution_with_tags(&self, builder: CreateStreamingDistributionWithTagsInputBuilder) -> Result<CreateStreamingDistributionWithTagsOutput, SdkError<CreateStreamingDistributionWithTagsError>>;
        async fn delete_cache_policy(&self, builder: DeleteCachePolicyInputBuilder) -> Result<DeleteCachePolicyOutput, SdkError<DeleteCachePolicyError>>;
        async fn delete_cloud_front_origin_access_identity(&self, builder: DeleteCloudFrontOriginAccessIdentityInputBuilder) -> Result<DeleteCloudFrontOriginAccessIdentityOutput, SdkError<DeleteCloudFrontOriginAccessIdentityError>>;
        async fn delete_continuous_deployment_policy(&self, builder: DeleteContinuousDeploymentPolicyInputBuilder) -> Result<DeleteContinuousDeploymentPolicyOutput, SdkError<DeleteContinuousDeploymentPolicyError>>;
        async fn delete_distribution(&self, builder: DeleteDistributionInputBuilder) -> Result<DeleteDistributionOutput, SdkError<DeleteDistributionError>>;
        async fn delete_field_level_encryption_config(&self, builder: DeleteFieldLevelEncryptionConfigInputBuilder) -> Result<DeleteFieldLevelEncryptionConfigOutput, SdkError<DeleteFieldLevelEncryptionConfigError>>;
        async fn delete_field_level_encryption_profile(&self, builder: DeleteFieldLevelEncryptionProfileInputBuilder) -> Result<DeleteFieldLevelEncryptionProfileOutput, SdkError<DeleteFieldLevelEncryptionProfileError>>;
        async fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>;
        async fn delete_key_group(&self, builder: DeleteKeyGroupInputBuilder) -> Result<DeleteKeyGroupOutput, SdkError<DeleteKeyGroupError>>;
        async fn delete_key_value_store(&self, builder: DeleteKeyValueStoreInputBuilder) -> Result<DeleteKeyValueStoreOutput, SdkError<DeleteKeyValueStoreError>>;
        async fn delete_monitoring_subscription(&self, builder: DeleteMonitoringSubscriptionInputBuilder) -> Result<DeleteMonitoringSubscriptionOutput, SdkError<DeleteMonitoringSubscriptionError>>;
        async fn delete_origin_access_control(&self, builder: DeleteOriginAccessControlInputBuilder) -> Result<DeleteOriginAccessControlOutput, SdkError<DeleteOriginAccessControlError>>;
        async fn delete_origin_request_policy(&self, builder: DeleteOriginRequestPolicyInputBuilder) -> Result<DeleteOriginRequestPolicyOutput, SdkError<DeleteOriginRequestPolicyError>>;
        async fn delete_public_key(&self, builder: DeletePublicKeyInputBuilder) -> Result<DeletePublicKeyOutput, SdkError<DeletePublicKeyError>>;
        async fn delete_realtime_log_config(&self, builder: DeleteRealtimeLogConfigInputBuilder) -> Result<DeleteRealtimeLogConfigOutput, SdkError<DeleteRealtimeLogConfigError>>;
        async fn delete_response_headers_policy(&self, builder: DeleteResponseHeadersPolicyInputBuilder) -> Result<DeleteResponseHeadersPolicyOutput, SdkError<DeleteResponseHeadersPolicyError>>;
        async fn delete_streaming_distribution(&self, builder: DeleteStreamingDistributionInputBuilder) -> Result<DeleteStreamingDistributionOutput, SdkError<DeleteStreamingDistributionError>>;
        async fn describe_function(&self, builder: DescribeFunctionInputBuilder) -> Result<DescribeFunctionOutput, SdkError<DescribeFunctionError>>;
        async fn describe_key_value_store(&self, builder: DescribeKeyValueStoreInputBuilder) -> Result<DescribeKeyValueStoreOutput, SdkError<DescribeKeyValueStoreError>>;
        async fn get_cache_policy(&self, builder: GetCachePolicyInputBuilder) -> Result<GetCachePolicyOutput, SdkError<GetCachePolicyError>>;
        async fn get_cache_policy_config(&self, builder: GetCachePolicyConfigInputBuilder) -> Result<GetCachePolicyConfigOutput, SdkError<GetCachePolicyConfigError>>;
        async fn get_cloud_front_origin_access_identity(&self, builder: GetCloudFrontOriginAccessIdentityInputBuilder) -> Result<GetCloudFrontOriginAccessIdentityOutput, SdkError<GetCloudFrontOriginAccessIdentityError>>;
        async fn get_cloud_front_origin_access_identity_config(&self, builder: GetCloudFrontOriginAccessIdentityConfigInputBuilder) -> Result<GetCloudFrontOriginAccessIdentityConfigOutput, SdkError<GetCloudFrontOriginAccessIdentityConfigError>>;
        async fn get_continuous_deployment_policy(&self, builder: GetContinuousDeploymentPolicyInputBuilder) -> Result<GetContinuousDeploymentPolicyOutput, SdkError<GetContinuousDeploymentPolicyError>>;
        async fn get_continuous_deployment_policy_config(&self, builder: GetContinuousDeploymentPolicyConfigInputBuilder) -> Result<GetContinuousDeploymentPolicyConfigOutput, SdkError<GetContinuousDeploymentPolicyConfigError>>;
        async fn get_distribution(&self, builder: GetDistributionInputBuilder) -> Result<GetDistributionOutput, SdkError<GetDistributionError>>;
        async fn get_distribution_config(&self, builder: GetDistributionConfigInputBuilder) -> Result<GetDistributionConfigOutput, SdkError<GetDistributionConfigError>>;
        async fn get_field_level_encryption(&self, builder: GetFieldLevelEncryptionInputBuilder) -> Result<GetFieldLevelEncryptionOutput, SdkError<GetFieldLevelEncryptionError>>;
        async fn get_field_level_encryption_config(&self, builder: GetFieldLevelEncryptionConfigInputBuilder) -> Result<GetFieldLevelEncryptionConfigOutput, SdkError<GetFieldLevelEncryptionConfigError>>;
        async fn get_field_level_encryption_profile(&self, builder: GetFieldLevelEncryptionProfileInputBuilder) -> Result<GetFieldLevelEncryptionProfileOutput, SdkError<GetFieldLevelEncryptionProfileError>>;
        async fn get_field_level_encryption_profile_config(&self, builder: GetFieldLevelEncryptionProfileConfigInputBuilder) -> Result<GetFieldLevelEncryptionProfileConfigOutput, SdkError<GetFieldLevelEncryptionProfileConfigError>>;
        async fn get_function(&self, builder: GetFunctionInputBuilder) -> Result<GetFunctionOutput, SdkError<GetFunctionError>>;
        async fn get_invalidation(&self, builder: GetInvalidationInputBuilder) -> Result<GetInvalidationOutput, SdkError<GetInvalidationError>>;
        async fn get_key_group(&self, builder: GetKeyGroupInputBuilder) -> Result<GetKeyGroupOutput, SdkError<GetKeyGroupError>>;
        async fn get_key_group_config(&self, builder: GetKeyGroupConfigInputBuilder) -> Result<GetKeyGroupConfigOutput, SdkError<GetKeyGroupConfigError>>;
        async fn get_monitoring_subscription(&self, builder: GetMonitoringSubscriptionInputBuilder) -> Result<GetMonitoringSubscriptionOutput, SdkError<GetMonitoringSubscriptionError>>;
        async fn get_origin_access_control(&self, builder: GetOriginAccessControlInputBuilder) -> Result<GetOriginAccessControlOutput, SdkError<GetOriginAccessControlError>>;
        async fn get_origin_access_control_config(&self, builder: GetOriginAccessControlConfigInputBuilder) -> Result<GetOriginAccessControlConfigOutput, SdkError<GetOriginAccessControlConfigError>>;
        async fn get_origin_request_policy(&self, builder: GetOriginRequestPolicyInputBuilder) -> Result<GetOriginRequestPolicyOutput, SdkError<GetOriginRequestPolicyError>>;
        async fn get_origin_request_policy_config(&self, builder: GetOriginRequestPolicyConfigInputBuilder) -> Result<GetOriginRequestPolicyConfigOutput, SdkError<GetOriginRequestPolicyConfigError>>;
        async fn get_public_key(&self, builder: GetPublicKeyInputBuilder) -> Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>;
        async fn get_public_key_config(&self, builder: GetPublicKeyConfigInputBuilder) -> Result<GetPublicKeyConfigOutput, SdkError<GetPublicKeyConfigError>>;
        async fn get_realtime_log_config(&self, builder: GetRealtimeLogConfigInputBuilder) -> Result<GetRealtimeLogConfigOutput, SdkError<GetRealtimeLogConfigError>>;
        async fn get_response_headers_policy(&self, builder: GetResponseHeadersPolicyInputBuilder) -> Result<GetResponseHeadersPolicyOutput, SdkError<GetResponseHeadersPolicyError>>;
        async fn get_response_headers_policy_config(&self, builder: GetResponseHeadersPolicyConfigInputBuilder) -> Result<GetResponseHeadersPolicyConfigOutput, SdkError<GetResponseHeadersPolicyConfigError>>;
        async fn get_streaming_distribution(&self, builder: GetStreamingDistributionInputBuilder) -> Result<GetStreamingDistributionOutput, SdkError<GetStreamingDistributionError>>;
        async fn get_streaming_distribution_config(&self, builder: GetStreamingDistributionConfigInputBuilder) -> Result<GetStreamingDistributionConfigOutput, SdkError<GetStreamingDistributionConfigError>>;
        async fn list_cache_policies(&self, builder: ListCachePoliciesInputBuilder) -> Result<ListCachePoliciesOutput, SdkError<ListCachePoliciesError>>;
        async fn list_cloud_front_origin_access_identities(&self, builder: ListCloudFrontOriginAccessIdentitiesInputBuilder) -> Result<ListCloudFrontOriginAccessIdentitiesOutput, SdkError<ListCloudFrontOriginAccessIdentitiesError>>;
        async fn list_conflicting_aliases(&self, builder: ListConflictingAliasesInputBuilder) -> Result<ListConflictingAliasesOutput, SdkError<ListConflictingAliasesError>>;
        async fn list_continuous_deployment_policies(&self, builder: ListContinuousDeploymentPoliciesInputBuilder) -> Result<ListContinuousDeploymentPoliciesOutput, SdkError<ListContinuousDeploymentPoliciesError>>;
        async fn list_distributions(&self, builder: ListDistributionsInputBuilder) -> Result<ListDistributionsOutput, SdkError<ListDistributionsError>>;
        async fn list_distributions_by_cache_policy_id(&self, builder: ListDistributionsByCachePolicyIdInputBuilder) -> Result<ListDistributionsByCachePolicyIdOutput, SdkError<ListDistributionsByCachePolicyIdError>>;
        async fn list_distributions_by_key_group(&self, builder: ListDistributionsByKeyGroupInputBuilder) -> Result<ListDistributionsByKeyGroupOutput, SdkError<ListDistributionsByKeyGroupError>>;
        async fn list_distributions_by_origin_request_policy_id(&self, builder: ListDistributionsByOriginRequestPolicyIdInputBuilder) -> Result<ListDistributionsByOriginRequestPolicyIdOutput, SdkError<ListDistributionsByOriginRequestPolicyIdError>>;
        async fn list_distributions_by_realtime_log_config(&self, builder: ListDistributionsByRealtimeLogConfigInputBuilder) -> Result<ListDistributionsByRealtimeLogConfigOutput, SdkError<ListDistributionsByRealtimeLogConfigError>>;
        async fn list_distributions_by_response_headers_policy_id(&self, builder: ListDistributionsByResponseHeadersPolicyIdInputBuilder) -> Result<ListDistributionsByResponseHeadersPolicyIdOutput, SdkError<ListDistributionsByResponseHeadersPolicyIdError>>;
        async fn list_distributions_by_web_acl_id(&self, builder: ListDistributionsByWebAclIdInputBuilder) -> Result<ListDistributionsByWebAclIdOutput, SdkError<ListDistributionsByWebACLIdError>>;
        async fn list_field_level_encryption_configs(&self, builder: ListFieldLevelEncryptionConfigsInputBuilder) -> Result<ListFieldLevelEncryptionConfigsOutput, SdkError<ListFieldLevelEncryptionConfigsError>>;
        async fn list_field_level_encryption_profiles(&self, builder: ListFieldLevelEncryptionProfilesInputBuilder) -> Result<ListFieldLevelEncryptionProfilesOutput, SdkError<ListFieldLevelEncryptionProfilesError>>;
        async fn list_functions(&self, builder: ListFunctionsInputBuilder) -> Result<ListFunctionsOutput, SdkError<ListFunctionsError>>;
        async fn list_invalidations(&self, builder: ListInvalidationsInputBuilder) -> Result<ListInvalidationsOutput, SdkError<ListInvalidationsError>>;
        async fn list_key_groups(&self, builder: ListKeyGroupsInputBuilder) -> Result<ListKeyGroupsOutput, SdkError<ListKeyGroupsError>>;
        async fn list_key_value_stores(&self, builder: ListKeyValueStoresInputBuilder) -> Result<ListKeyValueStoresOutput, SdkError<ListKeyValueStoresError>>;
        async fn list_origin_access_controls(&self, builder: ListOriginAccessControlsInputBuilder) -> Result<ListOriginAccessControlsOutput, SdkError<ListOriginAccessControlsError>>;
        async fn list_origin_request_policies(&self, builder: ListOriginRequestPoliciesInputBuilder) -> Result<ListOriginRequestPoliciesOutput, SdkError<ListOriginRequestPoliciesError>>;
        async fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>;
        async fn list_realtime_log_configs(&self, builder: ListRealtimeLogConfigsInputBuilder) -> Result<ListRealtimeLogConfigsOutput, SdkError<ListRealtimeLogConfigsError>>;
        async fn list_response_headers_policies(&self, builder: ListResponseHeadersPoliciesInputBuilder) -> Result<ListResponseHeadersPoliciesOutput, SdkError<ListResponseHeadersPoliciesError>>;
        async fn list_streaming_distributions(&self, builder: ListStreamingDistributionsInputBuilder) -> Result<ListStreamingDistributionsOutput, SdkError<ListStreamingDistributionsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn publish_function(&self, builder: PublishFunctionInputBuilder) -> Result<PublishFunctionOutput, SdkError<PublishFunctionError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn test_function(&self, builder: TestFunctionInputBuilder) -> Result<TestFunctionOutput, SdkError<TestFunctionError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_cache_policy(&self, builder: UpdateCachePolicyInputBuilder) -> Result<UpdateCachePolicyOutput, SdkError<UpdateCachePolicyError>>;
        async fn update_cloud_front_origin_access_identity(&self, builder: UpdateCloudFrontOriginAccessIdentityInputBuilder) -> Result<UpdateCloudFrontOriginAccessIdentityOutput, SdkError<UpdateCloudFrontOriginAccessIdentityError>>;
        async fn update_continuous_deployment_policy(&self, builder: UpdateContinuousDeploymentPolicyInputBuilder) -> Result<UpdateContinuousDeploymentPolicyOutput, SdkError<UpdateContinuousDeploymentPolicyError>>;
        async fn update_distribution(&self, builder: UpdateDistributionInputBuilder) -> Result<UpdateDistributionOutput, SdkError<UpdateDistributionError>>;
        async fn update_distribution_with_staging_config(&self, builder: UpdateDistributionWithStagingConfigInputBuilder) -> Result<UpdateDistributionWithStagingConfigOutput, SdkError<UpdateDistributionWithStagingConfigError>>;
        async fn update_field_level_encryption_config(&self, builder: UpdateFieldLevelEncryptionConfigInputBuilder) -> Result<UpdateFieldLevelEncryptionConfigOutput, SdkError<UpdateFieldLevelEncryptionConfigError>>;
        async fn update_field_level_encryption_profile(&self, builder: UpdateFieldLevelEncryptionProfileInputBuilder) -> Result<UpdateFieldLevelEncryptionProfileOutput, SdkError<UpdateFieldLevelEncryptionProfileError>>;
        async fn update_function(&self, builder: UpdateFunctionInputBuilder) -> Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>;
        async fn update_key_group(&self, builder: UpdateKeyGroupInputBuilder) -> Result<UpdateKeyGroupOutput, SdkError<UpdateKeyGroupError>>;
        async fn update_key_value_store(&self, builder: UpdateKeyValueStoreInputBuilder) -> Result<UpdateKeyValueStoreOutput, SdkError<UpdateKeyValueStoreError>>;
        async fn update_origin_access_control(&self, builder: UpdateOriginAccessControlInputBuilder) -> Result<UpdateOriginAccessControlOutput, SdkError<UpdateOriginAccessControlError>>;
        async fn update_origin_request_policy(&self, builder: UpdateOriginRequestPolicyInputBuilder) -> Result<UpdateOriginRequestPolicyOutput, SdkError<UpdateOriginRequestPolicyError>>;
        async fn update_public_key(&self, builder: UpdatePublicKeyInputBuilder) -> Result<UpdatePublicKeyOutput, SdkError<UpdatePublicKeyError>>;
        async fn update_realtime_log_config(&self, builder: UpdateRealtimeLogConfigInputBuilder) -> Result<UpdateRealtimeLogConfigOutput, SdkError<UpdateRealtimeLogConfigError>>;
        async fn update_response_headers_policy(&self, builder: UpdateResponseHeadersPolicyInputBuilder) -> Result<UpdateResponseHeadersPolicyOutput, SdkError<UpdateResponseHeadersPolicyError>>;
        async fn update_streaming_distribution(&self, builder: UpdateStreamingDistributionInputBuilder) -> Result<UpdateStreamingDistributionOutput, SdkError<UpdateStreamingDistributionError>>;
    }
}
