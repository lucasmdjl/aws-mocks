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
use aws_sdk_quicksight::operation::batch_create_topic_reviewed_answer::{builders::*, *};
use aws_sdk_quicksight::operation::batch_delete_topic_reviewed_answer::{builders::*, *};
use aws_sdk_quicksight::operation::cancel_ingestion::{builders::*, *};
use aws_sdk_quicksight::operation::create_account_customization::{builders::*, *};
use aws_sdk_quicksight::operation::create_account_subscription::{builders::*, *};
use aws_sdk_quicksight::operation::create_analysis::{builders::*, *};
use aws_sdk_quicksight::operation::create_dashboard::{builders::*, *};
use aws_sdk_quicksight::operation::create_data_set::{builders::*, *};
use aws_sdk_quicksight::operation::create_data_source::{builders::*, *};
use aws_sdk_quicksight::operation::create_folder::{builders::*, *};
use aws_sdk_quicksight::operation::create_folder_membership::{builders::*, *};
use aws_sdk_quicksight::operation::create_group::{builders::*, *};
use aws_sdk_quicksight::operation::create_group_membership::{builders::*, *};
use aws_sdk_quicksight::operation::create_iam_policy_assignment::{builders::*, *};
use aws_sdk_quicksight::operation::create_ingestion::{builders::*, *};
use aws_sdk_quicksight::operation::create_namespace::{builders::*, *};
use aws_sdk_quicksight::operation::create_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::create_role_membership::{builders::*, *};
use aws_sdk_quicksight::operation::create_template::{builders::*, *};
use aws_sdk_quicksight::operation::create_template_alias::{builders::*, *};
use aws_sdk_quicksight::operation::create_theme::{builders::*, *};
use aws_sdk_quicksight::operation::create_theme_alias::{builders::*, *};
use aws_sdk_quicksight::operation::create_topic::{builders::*, *};
use aws_sdk_quicksight::operation::create_topic_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::create_vpc_connection::{builders::*, *};
use aws_sdk_quicksight::operation::delete_account_customization::{builders::*, *};
use aws_sdk_quicksight::operation::delete_account_subscription::{builders::*, *};
use aws_sdk_quicksight::operation::delete_analysis::{builders::*, *};
use aws_sdk_quicksight::operation::delete_dashboard::{builders::*, *};
use aws_sdk_quicksight::operation::delete_data_set::{builders::*, *};
use aws_sdk_quicksight::operation::delete_data_set_refresh_properties::{builders::*, *};
use aws_sdk_quicksight::operation::delete_data_source::{builders::*, *};
use aws_sdk_quicksight::operation::delete_folder::{builders::*, *};
use aws_sdk_quicksight::operation::delete_folder_membership::{builders::*, *};
use aws_sdk_quicksight::operation::delete_group::{builders::*, *};
use aws_sdk_quicksight::operation::delete_group_membership::{builders::*, *};
use aws_sdk_quicksight::operation::delete_iam_policy_assignment::{builders::*, *};
use aws_sdk_quicksight::operation::delete_identity_propagation_config::{builders::*, *};
use aws_sdk_quicksight::operation::delete_namespace::{builders::*, *};
use aws_sdk_quicksight::operation::delete_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::delete_role_custom_permission::{builders::*, *};
use aws_sdk_quicksight::operation::delete_role_membership::{builders::*, *};
use aws_sdk_quicksight::operation::delete_template::{builders::*, *};
use aws_sdk_quicksight::operation::delete_template_alias::{builders::*, *};
use aws_sdk_quicksight::operation::delete_theme::{builders::*, *};
use aws_sdk_quicksight::operation::delete_theme_alias::{builders::*, *};
use aws_sdk_quicksight::operation::delete_topic::{builders::*, *};
use aws_sdk_quicksight::operation::delete_topic_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::delete_user::{builders::*, *};
use aws_sdk_quicksight::operation::delete_user_by_principal_id::{builders::*, *};
use aws_sdk_quicksight::operation::delete_vpc_connection::{builders::*, *};
use aws_sdk_quicksight::operation::describe_account_customization::{builders::*, *};
use aws_sdk_quicksight::operation::describe_account_settings::{builders::*, *};
use aws_sdk_quicksight::operation::describe_account_subscription::{builders::*, *};
use aws_sdk_quicksight::operation::describe_analysis::{builders::*, *};
use aws_sdk_quicksight::operation::describe_analysis_definition::{builders::*, *};
use aws_sdk_quicksight::operation::describe_analysis_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_asset_bundle_export_job::{builders::*, *};
use aws_sdk_quicksight::operation::describe_asset_bundle_import_job::{builders::*, *};
use aws_sdk_quicksight::operation::describe_dashboard::{builders::*, *};
use aws_sdk_quicksight::operation::describe_dashboard_definition::{builders::*, *};
use aws_sdk_quicksight::operation::describe_dashboard_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_dashboard_snapshot_job::{builders::*, *};
use aws_sdk_quicksight::operation::describe_dashboard_snapshot_job_result::{builders::*, *};
use aws_sdk_quicksight::operation::describe_data_set::{builders::*, *};
use aws_sdk_quicksight::operation::describe_data_set_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_data_set_refresh_properties::{builders::*, *};
use aws_sdk_quicksight::operation::describe_data_source::{builders::*, *};
use aws_sdk_quicksight::operation::describe_data_source_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_folder::{builders::*, *};
use aws_sdk_quicksight::operation::describe_folder_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_folder_resolved_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_group::{builders::*, *};
use aws_sdk_quicksight::operation::describe_group_membership::{builders::*, *};
use aws_sdk_quicksight::operation::describe_iam_policy_assignment::{builders::*, *};
use aws_sdk_quicksight::operation::describe_ingestion::{builders::*, *};
use aws_sdk_quicksight::operation::describe_ip_restriction::{builders::*, *};
use aws_sdk_quicksight::operation::describe_key_registration::{builders::*, *};
use aws_sdk_quicksight::operation::describe_namespace::{builders::*, *};
use aws_sdk_quicksight::operation::describe_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::describe_role_custom_permission::{builders::*, *};
use aws_sdk_quicksight::operation::describe_template::{builders::*, *};
use aws_sdk_quicksight::operation::describe_template_alias::{builders::*, *};
use aws_sdk_quicksight::operation::describe_template_definition::{builders::*, *};
use aws_sdk_quicksight::operation::describe_template_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_theme::{builders::*, *};
use aws_sdk_quicksight::operation::describe_theme_alias::{builders::*, *};
use aws_sdk_quicksight::operation::describe_theme_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_topic::{builders::*, *};
use aws_sdk_quicksight::operation::describe_topic_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::describe_topic_refresh::{builders::*, *};
use aws_sdk_quicksight::operation::describe_topic_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::describe_user::{builders::*, *};
use aws_sdk_quicksight::operation::describe_vpc_connection::{builders::*, *};
use aws_sdk_quicksight::operation::generate_embed_url_for_anonymous_user::{builders::*, *};
use aws_sdk_quicksight::operation::generate_embed_url_for_registered_user::{builders::*, *};
use aws_sdk_quicksight::operation::get_dashboard_embed_url::{builders::*, *};
use aws_sdk_quicksight::operation::get_session_embed_url::{builders::*, *};
use aws_sdk_quicksight::operation::list_analyses::{builders::*, *};
use aws_sdk_quicksight::operation::list_asset_bundle_export_jobs::{builders::*, *};
use aws_sdk_quicksight::operation::list_asset_bundle_import_jobs::{builders::*, *};
use aws_sdk_quicksight::operation::list_dashboard_versions::{builders::*, *};
use aws_sdk_quicksight::operation::list_dashboards::{builders::*, *};
use aws_sdk_quicksight::operation::list_data_sets::{builders::*, *};
use aws_sdk_quicksight::operation::list_data_sources::{builders::*, *};
use aws_sdk_quicksight::operation::list_folder_members::{builders::*, *};
use aws_sdk_quicksight::operation::list_folders::{builders::*, *};
use aws_sdk_quicksight::operation::list_group_memberships::{builders::*, *};
use aws_sdk_quicksight::operation::list_groups::{builders::*, *};
use aws_sdk_quicksight::operation::list_iam_policy_assignments::{builders::*, *};
use aws_sdk_quicksight::operation::list_iam_policy_assignments_for_user::{builders::*, *};
use aws_sdk_quicksight::operation::list_identity_propagation_configs::{builders::*, *};
use aws_sdk_quicksight::operation::list_ingestions::{builders::*, *};
use aws_sdk_quicksight::operation::list_namespaces::{builders::*, *};
use aws_sdk_quicksight::operation::list_refresh_schedules::{builders::*, *};
use aws_sdk_quicksight::operation::list_role_memberships::{builders::*, *};
use aws_sdk_quicksight::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_quicksight::operation::list_template_aliases::{builders::*, *};
use aws_sdk_quicksight::operation::list_template_versions::{builders::*, *};
use aws_sdk_quicksight::operation::list_templates::{builders::*, *};
use aws_sdk_quicksight::operation::list_theme_aliases::{builders::*, *};
use aws_sdk_quicksight::operation::list_theme_versions::{builders::*, *};
use aws_sdk_quicksight::operation::list_themes::{builders::*, *};
use aws_sdk_quicksight::operation::list_topic_refresh_schedules::{builders::*, *};
use aws_sdk_quicksight::operation::list_topic_reviewed_answers::{builders::*, *};
use aws_sdk_quicksight::operation::list_topics::{builders::*, *};
use aws_sdk_quicksight::operation::list_user_groups::{builders::*, *};
use aws_sdk_quicksight::operation::list_users::{builders::*, *};
use aws_sdk_quicksight::operation::list_vpc_connections::{builders::*, *};
use aws_sdk_quicksight::operation::put_data_set_refresh_properties::{builders::*, *};
use aws_sdk_quicksight::operation::register_user::{builders::*, *};
use aws_sdk_quicksight::operation::restore_analysis::{builders::*, *};
use aws_sdk_quicksight::operation::search_analyses::{builders::*, *};
use aws_sdk_quicksight::operation::search_dashboards::{builders::*, *};
use aws_sdk_quicksight::operation::search_data_sets::{builders::*, *};
use aws_sdk_quicksight::operation::search_data_sources::{builders::*, *};
use aws_sdk_quicksight::operation::search_folders::{builders::*, *};
use aws_sdk_quicksight::operation::search_groups::{builders::*, *};
use aws_sdk_quicksight::operation::start_asset_bundle_export_job::{builders::*, *};
use aws_sdk_quicksight::operation::start_asset_bundle_import_job::{builders::*, *};
use aws_sdk_quicksight::operation::start_dashboard_snapshot_job::{builders::*, *};
use aws_sdk_quicksight::operation::tag_resource::{builders::*, *};
use aws_sdk_quicksight::operation::untag_resource::{builders::*, *};
use aws_sdk_quicksight::operation::update_account_customization::{builders::*, *};
use aws_sdk_quicksight::operation::update_account_settings::{builders::*, *};
use aws_sdk_quicksight::operation::update_analysis::{builders::*, *};
use aws_sdk_quicksight::operation::update_analysis_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_dashboard::{builders::*, *};
use aws_sdk_quicksight::operation::update_dashboard_links::{builders::*, *};
use aws_sdk_quicksight::operation::update_dashboard_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_dashboard_published_version::{builders::*, *};
use aws_sdk_quicksight::operation::update_data_set::{builders::*, *};
use aws_sdk_quicksight::operation::update_data_set_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_data_source::{builders::*, *};
use aws_sdk_quicksight::operation::update_data_source_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_folder::{builders::*, *};
use aws_sdk_quicksight::operation::update_folder_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_group::{builders::*, *};
use aws_sdk_quicksight::operation::update_iam_policy_assignment::{builders::*, *};
use aws_sdk_quicksight::operation::update_identity_propagation_config::{builders::*, *};
use aws_sdk_quicksight::operation::update_ip_restriction::{builders::*, *};
use aws_sdk_quicksight::operation::update_key_registration::{builders::*, *};
use aws_sdk_quicksight::operation::update_public_sharing_settings::{builders::*, *};
use aws_sdk_quicksight::operation::update_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::update_role_custom_permission::{builders::*, *};
use aws_sdk_quicksight::operation::update_spice_capacity_configuration::{builders::*, *};
use aws_sdk_quicksight::operation::update_template::{builders::*, *};
use aws_sdk_quicksight::operation::update_template_alias::{builders::*, *};
use aws_sdk_quicksight::operation::update_template_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_theme::{builders::*, *};
use aws_sdk_quicksight::operation::update_theme_alias::{builders::*, *};
use aws_sdk_quicksight::operation::update_theme_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_topic::{builders::*, *};
use aws_sdk_quicksight::operation::update_topic_permissions::{builders::*, *};
use aws_sdk_quicksight::operation::update_topic_refresh_schedule::{builders::*, *};
use aws_sdk_quicksight::operation::update_user::{builders::*, *};
use aws_sdk_quicksight::operation::update_vpc_connection::{builders::*, *};
use aws_sdk_quicksight::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_quicksight::Client;

pub use aws_sdk_quicksight::*;

pub struct QuickSightClientImpl(Client);
impl QuickSightClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait QuickSightClient {
    fn batch_create_topic_reviewed_answer(&self, builder: BatchCreateTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchCreateTopicReviewedAnswerOutput, SdkError<BatchCreateTopicReviewedAnswerError>>>;
    fn batch_delete_topic_reviewed_answer(&self, builder: BatchDeleteTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchDeleteTopicReviewedAnswerOutput, SdkError<BatchDeleteTopicReviewedAnswerError>>>;
    fn cancel_ingestion(&self, builder: CancelIngestionInputBuilder) -> impl Future<Output = Result<CancelIngestionOutput, SdkError<CancelIngestionError>>>;
    fn create_account_customization(&self, builder: CreateAccountCustomizationInputBuilder) -> impl Future<Output = Result<CreateAccountCustomizationOutput, SdkError<CreateAccountCustomizationError>>>;
    fn create_account_subscription(&self, builder: CreateAccountSubscriptionInputBuilder) -> impl Future<Output = Result<CreateAccountSubscriptionOutput, SdkError<CreateAccountSubscriptionError>>>;
    fn create_analysis(&self, builder: CreateAnalysisInputBuilder) -> impl Future<Output = Result<CreateAnalysisOutput, SdkError<CreateAnalysisError>>>;
    fn create_dashboard(&self, builder: CreateDashboardInputBuilder) -> impl Future<Output = Result<CreateDashboardOutput, SdkError<CreateDashboardError>>>;
    fn create_data_set(&self, builder: CreateDataSetInputBuilder) -> impl Future<Output = Result<CreateDataSetOutput, SdkError<CreateDataSetError>>>;
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>>;
    fn create_folder(&self, builder: CreateFolderInputBuilder) -> impl Future<Output = Result<CreateFolderOutput, SdkError<CreateFolderError>>>;
    fn create_folder_membership(&self, builder: CreateFolderMembershipInputBuilder) -> impl Future<Output = Result<CreateFolderMembershipOutput, SdkError<CreateFolderMembershipError>>>;
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>>;
    fn create_group_membership(&self, builder: CreateGroupMembershipInputBuilder) -> impl Future<Output = Result<CreateGroupMembershipOutput, SdkError<CreateGroupMembershipError>>>;
    fn create_iam_policy_assignment(&self, builder: CreateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<CreateIamPolicyAssignmentOutput, SdkError<CreateIAMPolicyAssignmentError>>>;
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>>;
    fn create_namespace(&self, builder: CreateNamespaceInputBuilder) -> impl Future<Output = Result<CreateNamespaceOutput, SdkError<CreateNamespaceError>>>;
    fn create_refresh_schedule(&self, builder: CreateRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateRefreshScheduleOutput, SdkError<CreateRefreshScheduleError>>>;
    fn create_role_membership(&self, builder: CreateRoleMembershipInputBuilder) -> impl Future<Output = Result<CreateRoleMembershipOutput, SdkError<CreateRoleMembershipError>>>;
    fn create_template(&self, builder: CreateTemplateInputBuilder) -> impl Future<Output = Result<CreateTemplateOutput, SdkError<CreateTemplateError>>>;
    fn create_template_alias(&self, builder: CreateTemplateAliasInputBuilder) -> impl Future<Output = Result<CreateTemplateAliasOutput, SdkError<CreateTemplateAliasError>>>;
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>>;
    fn create_theme_alias(&self, builder: CreateThemeAliasInputBuilder) -> impl Future<Output = Result<CreateThemeAliasOutput, SdkError<CreateThemeAliasError>>>;
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>>;
    fn create_topic_refresh_schedule(&self, builder: CreateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateTopicRefreshScheduleOutput, SdkError<CreateTopicRefreshScheduleError>>>;
    fn create_vpc_connection(&self, builder: CreateVpcConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcConnectionOutput, SdkError<CreateVPCConnectionError>>>;
    fn delete_account_customization(&self, builder: DeleteAccountCustomizationInputBuilder) -> impl Future<Output = Result<DeleteAccountCustomizationOutput, SdkError<DeleteAccountCustomizationError>>>;
    fn delete_account_subscription(&self, builder: DeleteAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteAccountSubscriptionOutput, SdkError<DeleteAccountSubscriptionError>>>;
    fn delete_analysis(&self, builder: DeleteAnalysisInputBuilder) -> impl Future<Output = Result<DeleteAnalysisOutput, SdkError<DeleteAnalysisError>>>;
    fn delete_dashboard(&self, builder: DeleteDashboardInputBuilder) -> impl Future<Output = Result<DeleteDashboardOutput, SdkError<DeleteDashboardError>>>;
    fn delete_data_set(&self, builder: DeleteDataSetInputBuilder) -> impl Future<Output = Result<DeleteDataSetOutput, SdkError<DeleteDataSetError>>>;
    fn delete_data_set_refresh_properties(&self, builder: DeleteDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DeleteDataSetRefreshPropertiesOutput, SdkError<DeleteDataSetRefreshPropertiesError>>>;
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>>;
    fn delete_folder(&self, builder: DeleteFolderInputBuilder) -> impl Future<Output = Result<DeleteFolderOutput, SdkError<DeleteFolderError>>>;
    fn delete_folder_membership(&self, builder: DeleteFolderMembershipInputBuilder) -> impl Future<Output = Result<DeleteFolderMembershipOutput, SdkError<DeleteFolderMembershipError>>>;
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>>;
    fn delete_group_membership(&self, builder: DeleteGroupMembershipInputBuilder) -> impl Future<Output = Result<DeleteGroupMembershipOutput, SdkError<DeleteGroupMembershipError>>>;
    fn delete_iam_policy_assignment(&self, builder: DeleteIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DeleteIamPolicyAssignmentOutput, SdkError<DeleteIAMPolicyAssignmentError>>>;
    fn delete_identity_propagation_config(&self, builder: DeleteIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<DeleteIdentityPropagationConfigOutput, SdkError<DeleteIdentityPropagationConfigError>>>;
    fn delete_namespace(&self, builder: DeleteNamespaceInputBuilder) -> impl Future<Output = Result<DeleteNamespaceOutput, SdkError<DeleteNamespaceError>>>;
    fn delete_refresh_schedule(&self, builder: DeleteRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteRefreshScheduleOutput, SdkError<DeleteRefreshScheduleError>>>;
    fn delete_role_custom_permission(&self, builder: DeleteRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DeleteRoleCustomPermissionOutput, SdkError<DeleteRoleCustomPermissionError>>>;
    fn delete_role_membership(&self, builder: DeleteRoleMembershipInputBuilder) -> impl Future<Output = Result<DeleteRoleMembershipOutput, SdkError<DeleteRoleMembershipError>>>;
    fn delete_template(&self, builder: DeleteTemplateInputBuilder) -> impl Future<Output = Result<DeleteTemplateOutput, SdkError<DeleteTemplateError>>>;
    fn delete_template_alias(&self, builder: DeleteTemplateAliasInputBuilder) -> impl Future<Output = Result<DeleteTemplateAliasOutput, SdkError<DeleteTemplateAliasError>>>;
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>>;
    fn delete_theme_alias(&self, builder: DeleteThemeAliasInputBuilder) -> impl Future<Output = Result<DeleteThemeAliasOutput, SdkError<DeleteThemeAliasError>>>;
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>>;
    fn delete_topic_refresh_schedule(&self, builder: DeleteTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteTopicRefreshScheduleOutput, SdkError<DeleteTopicRefreshScheduleError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn delete_user_by_principal_id(&self, builder: DeleteUserByPrincipalIdInputBuilder) -> impl Future<Output = Result<DeleteUserByPrincipalIdOutput, SdkError<DeleteUserByPrincipalIdError>>>;
    fn delete_vpc_connection(&self, builder: DeleteVpcConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectionOutput, SdkError<DeleteVPCConnectionError>>>;
    fn describe_account_customization(&self, builder: DescribeAccountCustomizationInputBuilder) -> impl Future<Output = Result<DescribeAccountCustomizationOutput, SdkError<DescribeAccountCustomizationError>>>;
    fn describe_account_settings(&self, builder: DescribeAccountSettingsInputBuilder) -> impl Future<Output = Result<DescribeAccountSettingsOutput, SdkError<DescribeAccountSettingsError>>>;
    fn describe_account_subscription(&self, builder: DescribeAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeAccountSubscriptionOutput, SdkError<DescribeAccountSubscriptionError>>>;
    fn describe_analysis(&self, builder: DescribeAnalysisInputBuilder) -> impl Future<Output = Result<DescribeAnalysisOutput, SdkError<DescribeAnalysisError>>>;
    fn describe_analysis_definition(&self, builder: DescribeAnalysisDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAnalysisDefinitionOutput, SdkError<DescribeAnalysisDefinitionError>>>;
    fn describe_analysis_permissions(&self, builder: DescribeAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<DescribeAnalysisPermissionsOutput, SdkError<DescribeAnalysisPermissionsError>>>;
    fn describe_asset_bundle_export_job(&self, builder: DescribeAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleExportJobOutput, SdkError<DescribeAssetBundleExportJobError>>>;
    fn describe_asset_bundle_import_job(&self, builder: DescribeAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleImportJobOutput, SdkError<DescribeAssetBundleImportJobError>>>;
    fn describe_dashboard(&self, builder: DescribeDashboardInputBuilder) -> impl Future<Output = Result<DescribeDashboardOutput, SdkError<DescribeDashboardError>>>;
    fn describe_dashboard_definition(&self, builder: DescribeDashboardDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDashboardDefinitionOutput, SdkError<DescribeDashboardDefinitionError>>>;
    fn describe_dashboard_permissions(&self, builder: DescribeDashboardPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDashboardPermissionsOutput, SdkError<DescribeDashboardPermissionsError>>>;
    fn describe_dashboard_snapshot_job(&self, builder: DescribeDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobOutput, SdkError<DescribeDashboardSnapshotJobError>>>;
    fn describe_dashboard_snapshot_job_result(&self, builder: DescribeDashboardSnapshotJobResultInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobResultOutput, SdkError<DescribeDashboardSnapshotJobResultError>>>;
    fn describe_data_set(&self, builder: DescribeDataSetInputBuilder) -> impl Future<Output = Result<DescribeDataSetOutput, SdkError<DescribeDataSetError>>>;
    fn describe_data_set_permissions(&self, builder: DescribeDataSetPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSetPermissionsOutput, SdkError<DescribeDataSetPermissionsError>>>;
    fn describe_data_set_refresh_properties(&self, builder: DescribeDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DescribeDataSetRefreshPropertiesOutput, SdkError<DescribeDataSetRefreshPropertiesError>>>;
    fn describe_data_source(&self, builder: DescribeDataSourceInputBuilder) -> impl Future<Output = Result<DescribeDataSourceOutput, SdkError<DescribeDataSourceError>>>;
    fn describe_data_source_permissions(&self, builder: DescribeDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSourcePermissionsOutput, SdkError<DescribeDataSourcePermissionsError>>>;
    fn describe_folder(&self, builder: DescribeFolderInputBuilder) -> impl Future<Output = Result<DescribeFolderOutput, SdkError<DescribeFolderError>>>;
    fn describe_folder_permissions(&self, builder: DescribeFolderPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderPermissionsOutput, SdkError<DescribeFolderPermissionsError>>>;
    fn describe_folder_resolved_permissions(&self, builder: DescribeFolderResolvedPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderResolvedPermissionsOutput, SdkError<DescribeFolderResolvedPermissionsError>>>;
    fn describe_group(&self, builder: DescribeGroupInputBuilder) -> impl Future<Output = Result<DescribeGroupOutput, SdkError<DescribeGroupError>>>;
    fn describe_group_membership(&self, builder: DescribeGroupMembershipInputBuilder) -> impl Future<Output = Result<DescribeGroupMembershipOutput, SdkError<DescribeGroupMembershipError>>>;
    fn describe_iam_policy_assignment(&self, builder: DescribeIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DescribeIamPolicyAssignmentOutput, SdkError<DescribeIAMPolicyAssignmentError>>>;
    fn describe_ingestion(&self, builder: DescribeIngestionInputBuilder) -> impl Future<Output = Result<DescribeIngestionOutput, SdkError<DescribeIngestionError>>>;
    fn describe_ip_restriction(&self, builder: DescribeIpRestrictionInputBuilder) -> impl Future<Output = Result<DescribeIpRestrictionOutput, SdkError<DescribeIpRestrictionError>>>;
    fn describe_key_registration(&self, builder: DescribeKeyRegistrationInputBuilder) -> impl Future<Output = Result<DescribeKeyRegistrationOutput, SdkError<DescribeKeyRegistrationError>>>;
    fn describe_namespace(&self, builder: DescribeNamespaceInputBuilder) -> impl Future<Output = Result<DescribeNamespaceOutput, SdkError<DescribeNamespaceError>>>;
    fn describe_refresh_schedule(&self, builder: DescribeRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeRefreshScheduleOutput, SdkError<DescribeRefreshScheduleError>>>;
    fn describe_role_custom_permission(&self, builder: DescribeRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DescribeRoleCustomPermissionOutput, SdkError<DescribeRoleCustomPermissionError>>>;
    fn describe_template(&self, builder: DescribeTemplateInputBuilder) -> impl Future<Output = Result<DescribeTemplateOutput, SdkError<DescribeTemplateError>>>;
    fn describe_template_alias(&self, builder: DescribeTemplateAliasInputBuilder) -> impl Future<Output = Result<DescribeTemplateAliasOutput, SdkError<DescribeTemplateAliasError>>>;
    fn describe_template_definition(&self, builder: DescribeTemplateDefinitionInputBuilder) -> impl Future<Output = Result<DescribeTemplateDefinitionOutput, SdkError<DescribeTemplateDefinitionError>>>;
    fn describe_template_permissions(&self, builder: DescribeTemplatePermissionsInputBuilder) -> impl Future<Output = Result<DescribeTemplatePermissionsOutput, SdkError<DescribeTemplatePermissionsError>>>;
    fn describe_theme(&self, builder: DescribeThemeInputBuilder) -> impl Future<Output = Result<DescribeThemeOutput, SdkError<DescribeThemeError>>>;
    fn describe_theme_alias(&self, builder: DescribeThemeAliasInputBuilder) -> impl Future<Output = Result<DescribeThemeAliasOutput, SdkError<DescribeThemeAliasError>>>;
    fn describe_theme_permissions(&self, builder: DescribeThemePermissionsInputBuilder) -> impl Future<Output = Result<DescribeThemePermissionsOutput, SdkError<DescribeThemePermissionsError>>>;
    fn describe_topic(&self, builder: DescribeTopicInputBuilder) -> impl Future<Output = Result<DescribeTopicOutput, SdkError<DescribeTopicError>>>;
    fn describe_topic_permissions(&self, builder: DescribeTopicPermissionsInputBuilder) -> impl Future<Output = Result<DescribeTopicPermissionsOutput, SdkError<DescribeTopicPermissionsError>>>;
    fn describe_topic_refresh(&self, builder: DescribeTopicRefreshInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshOutput, SdkError<DescribeTopicRefreshError>>>;
    fn describe_topic_refresh_schedule(&self, builder: DescribeTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshScheduleOutput, SdkError<DescribeTopicRefreshScheduleError>>>;
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>>;
    fn describe_vpc_connection(&self, builder: DescribeVpcConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectionOutput, SdkError<DescribeVPCConnectionError>>>;
    fn generate_embed_url_for_anonymous_user(&self, builder: GenerateEmbedUrlForAnonymousUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForAnonymousUserOutput, SdkError<GenerateEmbedUrlForAnonymousUserError>>>;
    fn generate_embed_url_for_registered_user(&self, builder: GenerateEmbedUrlForRegisteredUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForRegisteredUserOutput, SdkError<GenerateEmbedUrlForRegisteredUserError>>>;
    fn get_dashboard_embed_url(&self, builder: GetDashboardEmbedUrlInputBuilder) -> impl Future<Output = Result<GetDashboardEmbedUrlOutput, SdkError<GetDashboardEmbedUrlError>>>;
    fn get_session_embed_url(&self, builder: GetSessionEmbedUrlInputBuilder) -> impl Future<Output = Result<GetSessionEmbedUrlOutput, SdkError<GetSessionEmbedUrlError>>>;
    fn list_analyses(&self, builder: ListAnalysesInputBuilder) -> impl Future<Output = Result<ListAnalysesOutput, SdkError<ListAnalysesError>>>;
    fn list_asset_bundle_export_jobs(&self, builder: ListAssetBundleExportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleExportJobsOutput, SdkError<ListAssetBundleExportJobsError>>>;
    fn list_asset_bundle_import_jobs(&self, builder: ListAssetBundleImportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleImportJobsOutput, SdkError<ListAssetBundleImportJobsError>>>;
    fn list_dashboard_versions(&self, builder: ListDashboardVersionsInputBuilder) -> impl Future<Output = Result<ListDashboardVersionsOutput, SdkError<ListDashboardVersionsError>>>;
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>>;
    fn list_data_sets(&self, builder: ListDataSetsInputBuilder) -> impl Future<Output = Result<ListDataSetsOutput, SdkError<ListDataSetsError>>>;
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>>;
    fn list_folder_members(&self, builder: ListFolderMembersInputBuilder) -> impl Future<Output = Result<ListFolderMembersOutput, SdkError<ListFolderMembersError>>>;
    fn list_folders(&self, builder: ListFoldersInputBuilder) -> impl Future<Output = Result<ListFoldersOutput, SdkError<ListFoldersError>>>;
    fn list_group_memberships(&self, builder: ListGroupMembershipsInputBuilder) -> impl Future<Output = Result<ListGroupMembershipsOutput, SdkError<ListGroupMembershipsError>>>;
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>>;
    fn list_iam_policy_assignments(&self, builder: ListIamPolicyAssignmentsInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsOutput, SdkError<ListIAMPolicyAssignmentsError>>>;
    fn list_iam_policy_assignments_for_user(&self, builder: ListIamPolicyAssignmentsForUserInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsForUserOutput, SdkError<ListIAMPolicyAssignmentsForUserError>>>;
    fn list_identity_propagation_configs(&self, builder: ListIdentityPropagationConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityPropagationConfigsOutput, SdkError<ListIdentityPropagationConfigsError>>>;
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>>;
    fn list_namespaces(&self, builder: ListNamespacesInputBuilder) -> impl Future<Output = Result<ListNamespacesOutput, SdkError<ListNamespacesError>>>;
    fn list_refresh_schedules(&self, builder: ListRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListRefreshSchedulesOutput, SdkError<ListRefreshSchedulesError>>>;
    fn list_role_memberships(&self, builder: ListRoleMembershipsInputBuilder) -> impl Future<Output = Result<ListRoleMembershipsOutput, SdkError<ListRoleMembershipsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_template_aliases(&self, builder: ListTemplateAliasesInputBuilder) -> impl Future<Output = Result<ListTemplateAliasesOutput, SdkError<ListTemplateAliasesError>>>;
    fn list_template_versions(&self, builder: ListTemplateVersionsInputBuilder) -> impl Future<Output = Result<ListTemplateVersionsOutput, SdkError<ListTemplateVersionsError>>>;
    fn list_templates(&self, builder: ListTemplatesInputBuilder) -> impl Future<Output = Result<ListTemplatesOutput, SdkError<ListTemplatesError>>>;
    fn list_theme_aliases(&self, builder: ListThemeAliasesInputBuilder) -> impl Future<Output = Result<ListThemeAliasesOutput, SdkError<ListThemeAliasesError>>>;
    fn list_theme_versions(&self, builder: ListThemeVersionsInputBuilder) -> impl Future<Output = Result<ListThemeVersionsOutput, SdkError<ListThemeVersionsError>>>;
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>>;
    fn list_topic_refresh_schedules(&self, builder: ListTopicRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListTopicRefreshSchedulesOutput, SdkError<ListTopicRefreshSchedulesError>>>;
    fn list_topic_reviewed_answers(&self, builder: ListTopicReviewedAnswersInputBuilder) -> impl Future<Output = Result<ListTopicReviewedAnswersOutput, SdkError<ListTopicReviewedAnswersError>>>;
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>>;
    fn list_user_groups(&self, builder: ListUserGroupsInputBuilder) -> impl Future<Output = Result<ListUserGroupsOutput, SdkError<ListUserGroupsError>>>;
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>>;
    fn list_vpc_connections(&self, builder: ListVpcConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcConnectionsOutput, SdkError<ListVPCConnectionsError>>>;
    fn put_data_set_refresh_properties(&self, builder: PutDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<PutDataSetRefreshPropertiesOutput, SdkError<PutDataSetRefreshPropertiesError>>>;
    fn register_user(&self, builder: RegisterUserInputBuilder) -> impl Future<Output = Result<RegisterUserOutput, SdkError<RegisterUserError>>>;
    fn restore_analysis(&self, builder: RestoreAnalysisInputBuilder) -> impl Future<Output = Result<RestoreAnalysisOutput, SdkError<RestoreAnalysisError>>>;
    fn search_analyses(&self, builder: SearchAnalysesInputBuilder) -> impl Future<Output = Result<SearchAnalysesOutput, SdkError<SearchAnalysesError>>>;
    fn search_dashboards(&self, builder: SearchDashboardsInputBuilder) -> impl Future<Output = Result<SearchDashboardsOutput, SdkError<SearchDashboardsError>>>;
    fn search_data_sets(&self, builder: SearchDataSetsInputBuilder) -> impl Future<Output = Result<SearchDataSetsOutput, SdkError<SearchDataSetsError>>>;
    fn search_data_sources(&self, builder: SearchDataSourcesInputBuilder) -> impl Future<Output = Result<SearchDataSourcesOutput, SdkError<SearchDataSourcesError>>>;
    fn search_folders(&self, builder: SearchFoldersInputBuilder) -> impl Future<Output = Result<SearchFoldersOutput, SdkError<SearchFoldersError>>>;
    fn search_groups(&self, builder: SearchGroupsInputBuilder) -> impl Future<Output = Result<SearchGroupsOutput, SdkError<SearchGroupsError>>>;
    fn start_asset_bundle_export_job(&self, builder: StartAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleExportJobOutput, SdkError<StartAssetBundleExportJobError>>>;
    fn start_asset_bundle_import_job(&self, builder: StartAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleImportJobOutput, SdkError<StartAssetBundleImportJobError>>>;
    fn start_dashboard_snapshot_job(&self, builder: StartDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<StartDashboardSnapshotJobOutput, SdkError<StartDashboardSnapshotJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_account_customization(&self, builder: UpdateAccountCustomizationInputBuilder) -> impl Future<Output = Result<UpdateAccountCustomizationOutput, SdkError<UpdateAccountCustomizationError>>>;
    fn update_account_settings(&self, builder: UpdateAccountSettingsInputBuilder) -> impl Future<Output = Result<UpdateAccountSettingsOutput, SdkError<UpdateAccountSettingsError>>>;
    fn update_analysis(&self, builder: UpdateAnalysisInputBuilder) -> impl Future<Output = Result<UpdateAnalysisOutput, SdkError<UpdateAnalysisError>>>;
    fn update_analysis_permissions(&self, builder: UpdateAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<UpdateAnalysisPermissionsOutput, SdkError<UpdateAnalysisPermissionsError>>>;
    fn update_dashboard(&self, builder: UpdateDashboardInputBuilder) -> impl Future<Output = Result<UpdateDashboardOutput, SdkError<UpdateDashboardError>>>;
    fn update_dashboard_links(&self, builder: UpdateDashboardLinksInputBuilder) -> impl Future<Output = Result<UpdateDashboardLinksOutput, SdkError<UpdateDashboardLinksError>>>;
    fn update_dashboard_permissions(&self, builder: UpdateDashboardPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDashboardPermissionsOutput, SdkError<UpdateDashboardPermissionsError>>>;
    fn update_dashboard_published_version(&self, builder: UpdateDashboardPublishedVersionInputBuilder) -> impl Future<Output = Result<UpdateDashboardPublishedVersionOutput, SdkError<UpdateDashboardPublishedVersionError>>>;
    fn update_data_set(&self, builder: UpdateDataSetInputBuilder) -> impl Future<Output = Result<UpdateDataSetOutput, SdkError<UpdateDataSetError>>>;
    fn update_data_set_permissions(&self, builder: UpdateDataSetPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSetPermissionsOutput, SdkError<UpdateDataSetPermissionsError>>>;
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>>;
    fn update_data_source_permissions(&self, builder: UpdateDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSourcePermissionsOutput, SdkError<UpdateDataSourcePermissionsError>>>;
    fn update_folder(&self, builder: UpdateFolderInputBuilder) -> impl Future<Output = Result<UpdateFolderOutput, SdkError<UpdateFolderError>>>;
    fn update_folder_permissions(&self, builder: UpdateFolderPermissionsInputBuilder) -> impl Future<Output = Result<UpdateFolderPermissionsOutput, SdkError<UpdateFolderPermissionsError>>>;
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>>;
    fn update_iam_policy_assignment(&self, builder: UpdateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<UpdateIamPolicyAssignmentOutput, SdkError<UpdateIAMPolicyAssignmentError>>>;
    fn update_identity_propagation_config(&self, builder: UpdateIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<UpdateIdentityPropagationConfigOutput, SdkError<UpdateIdentityPropagationConfigError>>>;
    fn update_ip_restriction(&self, builder: UpdateIpRestrictionInputBuilder) -> impl Future<Output = Result<UpdateIpRestrictionOutput, SdkError<UpdateIpRestrictionError>>>;
    fn update_key_registration(&self, builder: UpdateKeyRegistrationInputBuilder) -> impl Future<Output = Result<UpdateKeyRegistrationOutput, SdkError<UpdateKeyRegistrationError>>>;
    fn update_public_sharing_settings(&self, builder: UpdatePublicSharingSettingsInputBuilder) -> impl Future<Output = Result<UpdatePublicSharingSettingsOutput, SdkError<UpdatePublicSharingSettingsError>>>;
    fn update_refresh_schedule(&self, builder: UpdateRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateRefreshScheduleOutput, SdkError<UpdateRefreshScheduleError>>>;
    fn update_role_custom_permission(&self, builder: UpdateRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<UpdateRoleCustomPermissionOutput, SdkError<UpdateRoleCustomPermissionError>>>;
    fn update_spice_capacity_configuration(&self, builder: UpdateSpiceCapacityConfigurationInputBuilder) -> impl Future<Output = Result<UpdateSpiceCapacityConfigurationOutput, SdkError<UpdateSPICECapacityConfigurationError>>>;
    fn update_template(&self, builder: UpdateTemplateInputBuilder) -> impl Future<Output = Result<UpdateTemplateOutput, SdkError<UpdateTemplateError>>>;
    fn update_template_alias(&self, builder: UpdateTemplateAliasInputBuilder) -> impl Future<Output = Result<UpdateTemplateAliasOutput, SdkError<UpdateTemplateAliasError>>>;
    fn update_template_permissions(&self, builder: UpdateTemplatePermissionsInputBuilder) -> impl Future<Output = Result<UpdateTemplatePermissionsOutput, SdkError<UpdateTemplatePermissionsError>>>;
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>>;
    fn update_theme_alias(&self, builder: UpdateThemeAliasInputBuilder) -> impl Future<Output = Result<UpdateThemeAliasOutput, SdkError<UpdateThemeAliasError>>>;
    fn update_theme_permissions(&self, builder: UpdateThemePermissionsInputBuilder) -> impl Future<Output = Result<UpdateThemePermissionsOutput, SdkError<UpdateThemePermissionsError>>>;
    fn update_topic(&self, builder: UpdateTopicInputBuilder) -> impl Future<Output = Result<UpdateTopicOutput, SdkError<UpdateTopicError>>>;
    fn update_topic_permissions(&self, builder: UpdateTopicPermissionsInputBuilder) -> impl Future<Output = Result<UpdateTopicPermissionsOutput, SdkError<UpdateTopicPermissionsError>>>;
    fn update_topic_refresh_schedule(&self, builder: UpdateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateTopicRefreshScheduleOutput, SdkError<UpdateTopicRefreshScheduleError>>>;
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>>;
    fn update_vpc_connection(&self, builder: UpdateVpcConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcConnectionOutput, SdkError<UpdateVPCConnectionError>>>;
}
impl QuickSightClient for QuickSightClientImpl {
    fn batch_create_topic_reviewed_answer(&self, builder: BatchCreateTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchCreateTopicReviewedAnswerOutput, SdkError<BatchCreateTopicReviewedAnswerError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_topic_reviewed_answer(&self, builder: BatchDeleteTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchDeleteTopicReviewedAnswerOutput, SdkError<BatchDeleteTopicReviewedAnswerError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_ingestion(&self, builder: CancelIngestionInputBuilder) -> impl Future<Output = Result<CancelIngestionOutput, SdkError<CancelIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn create_account_customization(&self, builder: CreateAccountCustomizationInputBuilder) -> impl Future<Output = Result<CreateAccountCustomizationOutput, SdkError<CreateAccountCustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn create_account_subscription(&self, builder: CreateAccountSubscriptionInputBuilder) -> impl Future<Output = Result<CreateAccountSubscriptionOutput, SdkError<CreateAccountSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_analysis(&self, builder: CreateAnalysisInputBuilder) -> impl Future<Output = Result<CreateAnalysisOutput, SdkError<CreateAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn create_dashboard(&self, builder: CreateDashboardInputBuilder) -> impl Future<Output = Result<CreateDashboardOutput, SdkError<CreateDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_set(&self, builder: CreateDataSetInputBuilder) -> impl Future<Output = Result<CreateDataSetOutput, SdkError<CreateDataSetError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn create_folder(&self, builder: CreateFolderInputBuilder) -> impl Future<Output = Result<CreateFolderOutput, SdkError<CreateFolderError>>> {
        builder.send_with(&self.0)
    }
    fn create_folder_membership(&self, builder: CreateFolderMembershipInputBuilder) -> impl Future<Output = Result<CreateFolderMembershipOutput, SdkError<CreateFolderMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_group_membership(&self, builder: CreateGroupMembershipInputBuilder) -> impl Future<Output = Result<CreateGroupMembershipOutput, SdkError<CreateGroupMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn create_iam_policy_assignment(&self, builder: CreateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<CreateIamPolicyAssignmentOutput, SdkError<CreateIAMPolicyAssignmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn create_namespace(&self, builder: CreateNamespaceInputBuilder) -> impl Future<Output = Result<CreateNamespaceOutput, SdkError<CreateNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_refresh_schedule(&self, builder: CreateRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateRefreshScheduleOutput, SdkError<CreateRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn create_role_membership(&self, builder: CreateRoleMembershipInputBuilder) -> impl Future<Output = Result<CreateRoleMembershipOutput, SdkError<CreateRoleMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn create_template(&self, builder: CreateTemplateInputBuilder) -> impl Future<Output = Result<CreateTemplateOutput, SdkError<CreateTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn create_template_alias(&self, builder: CreateTemplateAliasInputBuilder) -> impl Future<Output = Result<CreateTemplateAliasOutput, SdkError<CreateTemplateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>> {
        builder.send_with(&self.0)
    }
    fn create_theme_alias(&self, builder: CreateThemeAliasInputBuilder) -> impl Future<Output = Result<CreateThemeAliasOutput, SdkError<CreateThemeAliasError>>> {
        builder.send_with(&self.0)
    }
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>> {
        builder.send_with(&self.0)
    }
    fn create_topic_refresh_schedule(&self, builder: CreateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateTopicRefreshScheduleOutput, SdkError<CreateTopicRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_connection(&self, builder: CreateVpcConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcConnectionOutput, SdkError<CreateVPCConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_account_customization(&self, builder: DeleteAccountCustomizationInputBuilder) -> impl Future<Output = Result<DeleteAccountCustomizationOutput, SdkError<DeleteAccountCustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_account_subscription(&self, builder: DeleteAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteAccountSubscriptionOutput, SdkError<DeleteAccountSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_analysis(&self, builder: DeleteAnalysisInputBuilder) -> impl Future<Output = Result<DeleteAnalysisOutput, SdkError<DeleteAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dashboard(&self, builder: DeleteDashboardInputBuilder) -> impl Future<Output = Result<DeleteDashboardOutput, SdkError<DeleteDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_set(&self, builder: DeleteDataSetInputBuilder) -> impl Future<Output = Result<DeleteDataSetOutput, SdkError<DeleteDataSetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_set_refresh_properties(&self, builder: DeleteDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DeleteDataSetRefreshPropertiesOutput, SdkError<DeleteDataSetRefreshPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_folder(&self, builder: DeleteFolderInputBuilder) -> impl Future<Output = Result<DeleteFolderOutput, SdkError<DeleteFolderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_folder_membership(&self, builder: DeleteFolderMembershipInputBuilder) -> impl Future<Output = Result<DeleteFolderMembershipOutput, SdkError<DeleteFolderMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group_membership(&self, builder: DeleteGroupMembershipInputBuilder) -> impl Future<Output = Result<DeleteGroupMembershipOutput, SdkError<DeleteGroupMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn delete_iam_policy_assignment(&self, builder: DeleteIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DeleteIamPolicyAssignmentOutput, SdkError<DeleteIAMPolicyAssignmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_identity_propagation_config(&self, builder: DeleteIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<DeleteIdentityPropagationConfigOutput, SdkError<DeleteIdentityPropagationConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_namespace(&self, builder: DeleteNamespaceInputBuilder) -> impl Future<Output = Result<DeleteNamespaceOutput, SdkError<DeleteNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_refresh_schedule(&self, builder: DeleteRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteRefreshScheduleOutput, SdkError<DeleteRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_role_custom_permission(&self, builder: DeleteRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DeleteRoleCustomPermissionOutput, SdkError<DeleteRoleCustomPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_role_membership(&self, builder: DeleteRoleMembershipInputBuilder) -> impl Future<Output = Result<DeleteRoleMembershipOutput, SdkError<DeleteRoleMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn delete_template(&self, builder: DeleteTemplateInputBuilder) -> impl Future<Output = Result<DeleteTemplateOutput, SdkError<DeleteTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_template_alias(&self, builder: DeleteTemplateAliasInputBuilder) -> impl Future<Output = Result<DeleteTemplateAliasOutput, SdkError<DeleteTemplateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_theme_alias(&self, builder: DeleteThemeAliasInputBuilder) -> impl Future<Output = Result<DeleteThemeAliasOutput, SdkError<DeleteThemeAliasError>>> {
        builder.send_with(&self.0)
    }
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>> {
        builder.send_with(&self.0)
    }
    fn delete_topic_refresh_schedule(&self, builder: DeleteTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteTopicRefreshScheduleOutput, SdkError<DeleteTopicRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_by_principal_id(&self, builder: DeleteUserByPrincipalIdInputBuilder) -> impl Future<Output = Result<DeleteUserByPrincipalIdOutput, SdkError<DeleteUserByPrincipalIdError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_connection(&self, builder: DeleteVpcConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectionOutput, SdkError<DeleteVPCConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_customization(&self, builder: DescribeAccountCustomizationInputBuilder) -> impl Future<Output = Result<DescribeAccountCustomizationOutput, SdkError<DescribeAccountCustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_settings(&self, builder: DescribeAccountSettingsInputBuilder) -> impl Future<Output = Result<DescribeAccountSettingsOutput, SdkError<DescribeAccountSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_subscription(&self, builder: DescribeAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeAccountSubscriptionOutput, SdkError<DescribeAccountSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_analysis(&self, builder: DescribeAnalysisInputBuilder) -> impl Future<Output = Result<DescribeAnalysisOutput, SdkError<DescribeAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn describe_analysis_definition(&self, builder: DescribeAnalysisDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAnalysisDefinitionOutput, SdkError<DescribeAnalysisDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_analysis_permissions(&self, builder: DescribeAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<DescribeAnalysisPermissionsOutput, SdkError<DescribeAnalysisPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_asset_bundle_export_job(&self, builder: DescribeAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleExportJobOutput, SdkError<DescribeAssetBundleExportJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_asset_bundle_import_job(&self, builder: DescribeAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleImportJobOutput, SdkError<DescribeAssetBundleImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dashboard(&self, builder: DescribeDashboardInputBuilder) -> impl Future<Output = Result<DescribeDashboardOutput, SdkError<DescribeDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dashboard_definition(&self, builder: DescribeDashboardDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDashboardDefinitionOutput, SdkError<DescribeDashboardDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dashboard_permissions(&self, builder: DescribeDashboardPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDashboardPermissionsOutput, SdkError<DescribeDashboardPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dashboard_snapshot_job(&self, builder: DescribeDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobOutput, SdkError<DescribeDashboardSnapshotJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dashboard_snapshot_job_result(&self, builder: DescribeDashboardSnapshotJobResultInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobResultOutput, SdkError<DescribeDashboardSnapshotJobResultError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_set(&self, builder: DescribeDataSetInputBuilder) -> impl Future<Output = Result<DescribeDataSetOutput, SdkError<DescribeDataSetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_set_permissions(&self, builder: DescribeDataSetPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSetPermissionsOutput, SdkError<DescribeDataSetPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_set_refresh_properties(&self, builder: DescribeDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DescribeDataSetRefreshPropertiesOutput, SdkError<DescribeDataSetRefreshPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_source(&self, builder: DescribeDataSourceInputBuilder) -> impl Future<Output = Result<DescribeDataSourceOutput, SdkError<DescribeDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_source_permissions(&self, builder: DescribeDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSourcePermissionsOutput, SdkError<DescribeDataSourcePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_folder(&self, builder: DescribeFolderInputBuilder) -> impl Future<Output = Result<DescribeFolderOutput, SdkError<DescribeFolderError>>> {
        builder.send_with(&self.0)
    }
    fn describe_folder_permissions(&self, builder: DescribeFolderPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderPermissionsOutput, SdkError<DescribeFolderPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_folder_resolved_permissions(&self, builder: DescribeFolderResolvedPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderResolvedPermissionsOutput, SdkError<DescribeFolderResolvedPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_group(&self, builder: DescribeGroupInputBuilder) -> impl Future<Output = Result<DescribeGroupOutput, SdkError<DescribeGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_group_membership(&self, builder: DescribeGroupMembershipInputBuilder) -> impl Future<Output = Result<DescribeGroupMembershipOutput, SdkError<DescribeGroupMembershipError>>> {
        builder.send_with(&self.0)
    }
    fn describe_iam_policy_assignment(&self, builder: DescribeIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DescribeIamPolicyAssignmentOutput, SdkError<DescribeIAMPolicyAssignmentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ingestion(&self, builder: DescribeIngestionInputBuilder) -> impl Future<Output = Result<DescribeIngestionOutput, SdkError<DescribeIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ip_restriction(&self, builder: DescribeIpRestrictionInputBuilder) -> impl Future<Output = Result<DescribeIpRestrictionOutput, SdkError<DescribeIpRestrictionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_key_registration(&self, builder: DescribeKeyRegistrationInputBuilder) -> impl Future<Output = Result<DescribeKeyRegistrationOutput, SdkError<DescribeKeyRegistrationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_namespace(&self, builder: DescribeNamespaceInputBuilder) -> impl Future<Output = Result<DescribeNamespaceOutput, SdkError<DescribeNamespaceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_refresh_schedule(&self, builder: DescribeRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeRefreshScheduleOutput, SdkError<DescribeRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn describe_role_custom_permission(&self, builder: DescribeRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DescribeRoleCustomPermissionOutput, SdkError<DescribeRoleCustomPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_template(&self, builder: DescribeTemplateInputBuilder) -> impl Future<Output = Result<DescribeTemplateOutput, SdkError<DescribeTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn describe_template_alias(&self, builder: DescribeTemplateAliasInputBuilder) -> impl Future<Output = Result<DescribeTemplateAliasOutput, SdkError<DescribeTemplateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn describe_template_definition(&self, builder: DescribeTemplateDefinitionInputBuilder) -> impl Future<Output = Result<DescribeTemplateDefinitionOutput, SdkError<DescribeTemplateDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_template_permissions(&self, builder: DescribeTemplatePermissionsInputBuilder) -> impl Future<Output = Result<DescribeTemplatePermissionsOutput, SdkError<DescribeTemplatePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_theme(&self, builder: DescribeThemeInputBuilder) -> impl Future<Output = Result<DescribeThemeOutput, SdkError<DescribeThemeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_theme_alias(&self, builder: DescribeThemeAliasInputBuilder) -> impl Future<Output = Result<DescribeThemeAliasOutput, SdkError<DescribeThemeAliasError>>> {
        builder.send_with(&self.0)
    }
    fn describe_theme_permissions(&self, builder: DescribeThemePermissionsInputBuilder) -> impl Future<Output = Result<DescribeThemePermissionsOutput, SdkError<DescribeThemePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_topic(&self, builder: DescribeTopicInputBuilder) -> impl Future<Output = Result<DescribeTopicOutput, SdkError<DescribeTopicError>>> {
        builder.send_with(&self.0)
    }
    fn describe_topic_permissions(&self, builder: DescribeTopicPermissionsInputBuilder) -> impl Future<Output = Result<DescribeTopicPermissionsOutput, SdkError<DescribeTopicPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_topic_refresh(&self, builder: DescribeTopicRefreshInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshOutput, SdkError<DescribeTopicRefreshError>>> {
        builder.send_with(&self.0)
    }
    fn describe_topic_refresh_schedule(&self, builder: DescribeTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshScheduleOutput, SdkError<DescribeTopicRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_connection(&self, builder: DescribeVpcConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectionOutput, SdkError<DescribeVPCConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn generate_embed_url_for_anonymous_user(&self, builder: GenerateEmbedUrlForAnonymousUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForAnonymousUserOutput, SdkError<GenerateEmbedUrlForAnonymousUserError>>> {
        builder.send_with(&self.0)
    }
    fn generate_embed_url_for_registered_user(&self, builder: GenerateEmbedUrlForRegisteredUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForRegisteredUserOutput, SdkError<GenerateEmbedUrlForRegisteredUserError>>> {
        builder.send_with(&self.0)
    }
    fn get_dashboard_embed_url(&self, builder: GetDashboardEmbedUrlInputBuilder) -> impl Future<Output = Result<GetDashboardEmbedUrlOutput, SdkError<GetDashboardEmbedUrlError>>> {
        builder.send_with(&self.0)
    }
    fn get_session_embed_url(&self, builder: GetSessionEmbedUrlInputBuilder) -> impl Future<Output = Result<GetSessionEmbedUrlOutput, SdkError<GetSessionEmbedUrlError>>> {
        builder.send_with(&self.0)
    }
    fn list_analyses(&self, builder: ListAnalysesInputBuilder) -> impl Future<Output = Result<ListAnalysesOutput, SdkError<ListAnalysesError>>> {
        builder.send_with(&self.0)
    }
    fn list_asset_bundle_export_jobs(&self, builder: ListAssetBundleExportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleExportJobsOutput, SdkError<ListAssetBundleExportJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_asset_bundle_import_jobs(&self, builder: ListAssetBundleImportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleImportJobsOutput, SdkError<ListAssetBundleImportJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_dashboard_versions(&self, builder: ListDashboardVersionsInputBuilder) -> impl Future<Output = Result<ListDashboardVersionsOutput, SdkError<ListDashboardVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_sets(&self, builder: ListDataSetsInputBuilder) -> impl Future<Output = Result<ListDataSetsOutput, SdkError<ListDataSetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_folder_members(&self, builder: ListFolderMembersInputBuilder) -> impl Future<Output = Result<ListFolderMembersOutput, SdkError<ListFolderMembersError>>> {
        builder.send_with(&self.0)
    }
    fn list_folders(&self, builder: ListFoldersInputBuilder) -> impl Future<Output = Result<ListFoldersOutput, SdkError<ListFoldersError>>> {
        builder.send_with(&self.0)
    }
    fn list_group_memberships(&self, builder: ListGroupMembershipsInputBuilder) -> impl Future<Output = Result<ListGroupMembershipsOutput, SdkError<ListGroupMembershipsError>>> {
        builder.send_with(&self.0)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_iam_policy_assignments(&self, builder: ListIamPolicyAssignmentsInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsOutput, SdkError<ListIAMPolicyAssignmentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_iam_policy_assignments_for_user(&self, builder: ListIamPolicyAssignmentsForUserInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsForUserOutput, SdkError<ListIAMPolicyAssignmentsForUserError>>> {
        builder.send_with(&self.0)
    }
    fn list_identity_propagation_configs(&self, builder: ListIdentityPropagationConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityPropagationConfigsOutput, SdkError<ListIdentityPropagationConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_namespaces(&self, builder: ListNamespacesInputBuilder) -> impl Future<Output = Result<ListNamespacesOutput, SdkError<ListNamespacesError>>> {
        builder.send_with(&self.0)
    }
    fn list_refresh_schedules(&self, builder: ListRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListRefreshSchedulesOutput, SdkError<ListRefreshSchedulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_role_memberships(&self, builder: ListRoleMembershipsInputBuilder) -> impl Future<Output = Result<ListRoleMembershipsOutput, SdkError<ListRoleMembershipsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_template_aliases(&self, builder: ListTemplateAliasesInputBuilder) -> impl Future<Output = Result<ListTemplateAliasesOutput, SdkError<ListTemplateAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_template_versions(&self, builder: ListTemplateVersionsInputBuilder) -> impl Future<Output = Result<ListTemplateVersionsOutput, SdkError<ListTemplateVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_templates(&self, builder: ListTemplatesInputBuilder) -> impl Future<Output = Result<ListTemplatesOutput, SdkError<ListTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_theme_aliases(&self, builder: ListThemeAliasesInputBuilder) -> impl Future<Output = Result<ListThemeAliasesOutput, SdkError<ListThemeAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_theme_versions(&self, builder: ListThemeVersionsInputBuilder) -> impl Future<Output = Result<ListThemeVersionsOutput, SdkError<ListThemeVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>> {
        builder.send_with(&self.0)
    }
    fn list_topic_refresh_schedules(&self, builder: ListTopicRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListTopicRefreshSchedulesOutput, SdkError<ListTopicRefreshSchedulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_topic_reviewed_answers(&self, builder: ListTopicReviewedAnswersInputBuilder) -> impl Future<Output = Result<ListTopicReviewedAnswersOutput, SdkError<ListTopicReviewedAnswersError>>> {
        builder.send_with(&self.0)
    }
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_groups(&self, builder: ListUserGroupsInputBuilder) -> impl Future<Output = Result<ListUserGroupsOutput, SdkError<ListUserGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_connections(&self, builder: ListVpcConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcConnectionsOutput, SdkError<ListVPCConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn put_data_set_refresh_properties(&self, builder: PutDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<PutDataSetRefreshPropertiesOutput, SdkError<PutDataSetRefreshPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn register_user(&self, builder: RegisterUserInputBuilder) -> impl Future<Output = Result<RegisterUserOutput, SdkError<RegisterUserError>>> {
        builder.send_with(&self.0)
    }
    fn restore_analysis(&self, builder: RestoreAnalysisInputBuilder) -> impl Future<Output = Result<RestoreAnalysisOutput, SdkError<RestoreAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn search_analyses(&self, builder: SearchAnalysesInputBuilder) -> impl Future<Output = Result<SearchAnalysesOutput, SdkError<SearchAnalysesError>>> {
        builder.send_with(&self.0)
    }
    fn search_dashboards(&self, builder: SearchDashboardsInputBuilder) -> impl Future<Output = Result<SearchDashboardsOutput, SdkError<SearchDashboardsError>>> {
        builder.send_with(&self.0)
    }
    fn search_data_sets(&self, builder: SearchDataSetsInputBuilder) -> impl Future<Output = Result<SearchDataSetsOutput, SdkError<SearchDataSetsError>>> {
        builder.send_with(&self.0)
    }
    fn search_data_sources(&self, builder: SearchDataSourcesInputBuilder) -> impl Future<Output = Result<SearchDataSourcesOutput, SdkError<SearchDataSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn search_folders(&self, builder: SearchFoldersInputBuilder) -> impl Future<Output = Result<SearchFoldersOutput, SdkError<SearchFoldersError>>> {
        builder.send_with(&self.0)
    }
    fn search_groups(&self, builder: SearchGroupsInputBuilder) -> impl Future<Output = Result<SearchGroupsOutput, SdkError<SearchGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn start_asset_bundle_export_job(&self, builder: StartAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleExportJobOutput, SdkError<StartAssetBundleExportJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_asset_bundle_import_job(&self, builder: StartAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleImportJobOutput, SdkError<StartAssetBundleImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_dashboard_snapshot_job(&self, builder: StartDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<StartDashboardSnapshotJobOutput, SdkError<StartDashboardSnapshotJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_account_customization(&self, builder: UpdateAccountCustomizationInputBuilder) -> impl Future<Output = Result<UpdateAccountCustomizationOutput, SdkError<UpdateAccountCustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn update_account_settings(&self, builder: UpdateAccountSettingsInputBuilder) -> impl Future<Output = Result<UpdateAccountSettingsOutput, SdkError<UpdateAccountSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn update_analysis(&self, builder: UpdateAnalysisInputBuilder) -> impl Future<Output = Result<UpdateAnalysisOutput, SdkError<UpdateAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn update_analysis_permissions(&self, builder: UpdateAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<UpdateAnalysisPermissionsOutput, SdkError<UpdateAnalysisPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_dashboard(&self, builder: UpdateDashboardInputBuilder) -> impl Future<Output = Result<UpdateDashboardOutput, SdkError<UpdateDashboardError>>> {
        builder.send_with(&self.0)
    }
    fn update_dashboard_links(&self, builder: UpdateDashboardLinksInputBuilder) -> impl Future<Output = Result<UpdateDashboardLinksOutput, SdkError<UpdateDashboardLinksError>>> {
        builder.send_with(&self.0)
    }
    fn update_dashboard_permissions(&self, builder: UpdateDashboardPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDashboardPermissionsOutput, SdkError<UpdateDashboardPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_dashboard_published_version(&self, builder: UpdateDashboardPublishedVersionInputBuilder) -> impl Future<Output = Result<UpdateDashboardPublishedVersionOutput, SdkError<UpdateDashboardPublishedVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_set(&self, builder: UpdateDataSetInputBuilder) -> impl Future<Output = Result<UpdateDataSetOutput, SdkError<UpdateDataSetError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_set_permissions(&self, builder: UpdateDataSetPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSetPermissionsOutput, SdkError<UpdateDataSetPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_source_permissions(&self, builder: UpdateDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSourcePermissionsOutput, SdkError<UpdateDataSourcePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_folder(&self, builder: UpdateFolderInputBuilder) -> impl Future<Output = Result<UpdateFolderOutput, SdkError<UpdateFolderError>>> {
        builder.send_with(&self.0)
    }
    fn update_folder_permissions(&self, builder: UpdateFolderPermissionsInputBuilder) -> impl Future<Output = Result<UpdateFolderPermissionsOutput, SdkError<UpdateFolderPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_iam_policy_assignment(&self, builder: UpdateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<UpdateIamPolicyAssignmentOutput, SdkError<UpdateIAMPolicyAssignmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_identity_propagation_config(&self, builder: UpdateIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<UpdateIdentityPropagationConfigOutput, SdkError<UpdateIdentityPropagationConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_ip_restriction(&self, builder: UpdateIpRestrictionInputBuilder) -> impl Future<Output = Result<UpdateIpRestrictionOutput, SdkError<UpdateIpRestrictionError>>> {
        builder.send_with(&self.0)
    }
    fn update_key_registration(&self, builder: UpdateKeyRegistrationInputBuilder) -> impl Future<Output = Result<UpdateKeyRegistrationOutput, SdkError<UpdateKeyRegistrationError>>> {
        builder.send_with(&self.0)
    }
    fn update_public_sharing_settings(&self, builder: UpdatePublicSharingSettingsInputBuilder) -> impl Future<Output = Result<UpdatePublicSharingSettingsOutput, SdkError<UpdatePublicSharingSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn update_refresh_schedule(&self, builder: UpdateRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateRefreshScheduleOutput, SdkError<UpdateRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn update_role_custom_permission(&self, builder: UpdateRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<UpdateRoleCustomPermissionOutput, SdkError<UpdateRoleCustomPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn update_spice_capacity_configuration(&self, builder: UpdateSpiceCapacityConfigurationInputBuilder) -> impl Future<Output = Result<UpdateSpiceCapacityConfigurationOutput, SdkError<UpdateSPICECapacityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_template(&self, builder: UpdateTemplateInputBuilder) -> impl Future<Output = Result<UpdateTemplateOutput, SdkError<UpdateTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn update_template_alias(&self, builder: UpdateTemplateAliasInputBuilder) -> impl Future<Output = Result<UpdateTemplateAliasOutput, SdkError<UpdateTemplateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn update_template_permissions(&self, builder: UpdateTemplatePermissionsInputBuilder) -> impl Future<Output = Result<UpdateTemplatePermissionsOutput, SdkError<UpdateTemplatePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>> {
        builder.send_with(&self.0)
    }
    fn update_theme_alias(&self, builder: UpdateThemeAliasInputBuilder) -> impl Future<Output = Result<UpdateThemeAliasOutput, SdkError<UpdateThemeAliasError>>> {
        builder.send_with(&self.0)
    }
    fn update_theme_permissions(&self, builder: UpdateThemePermissionsInputBuilder) -> impl Future<Output = Result<UpdateThemePermissionsOutput, SdkError<UpdateThemePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_topic(&self, builder: UpdateTopicInputBuilder) -> impl Future<Output = Result<UpdateTopicOutput, SdkError<UpdateTopicError>>> {
        builder.send_with(&self.0)
    }
    fn update_topic_permissions(&self, builder: UpdateTopicPermissionsInputBuilder) -> impl Future<Output = Result<UpdateTopicPermissionsOutput, SdkError<UpdateTopicPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_topic_refresh_schedule(&self, builder: UpdateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateTopicRefreshScheduleOutput, SdkError<UpdateTopicRefreshScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        builder.send_with(&self.0)
    }
    fn update_vpc_connection(&self, builder: UpdateVpcConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcConnectionOutput, SdkError<UpdateVPCConnectionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: QuickSightClient> QuickSightClient for &T {
    fn batch_create_topic_reviewed_answer(&self, builder: BatchCreateTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchCreateTopicReviewedAnswerOutput, SdkError<BatchCreateTopicReviewedAnswerError>>> {
        (*self).batch_create_topic_reviewed_answer(builder)
    }
    fn batch_delete_topic_reviewed_answer(&self, builder: BatchDeleteTopicReviewedAnswerInputBuilder) -> impl Future<Output = Result<BatchDeleteTopicReviewedAnswerOutput, SdkError<BatchDeleteTopicReviewedAnswerError>>> {
        (*self).batch_delete_topic_reviewed_answer(builder)
    }
    fn cancel_ingestion(&self, builder: CancelIngestionInputBuilder) -> impl Future<Output = Result<CancelIngestionOutput, SdkError<CancelIngestionError>>> {
        (*self).cancel_ingestion(builder)
    }
    fn create_account_customization(&self, builder: CreateAccountCustomizationInputBuilder) -> impl Future<Output = Result<CreateAccountCustomizationOutput, SdkError<CreateAccountCustomizationError>>> {
        (*self).create_account_customization(builder)
    }
    fn create_account_subscription(&self, builder: CreateAccountSubscriptionInputBuilder) -> impl Future<Output = Result<CreateAccountSubscriptionOutput, SdkError<CreateAccountSubscriptionError>>> {
        (*self).create_account_subscription(builder)
    }
    fn create_analysis(&self, builder: CreateAnalysisInputBuilder) -> impl Future<Output = Result<CreateAnalysisOutput, SdkError<CreateAnalysisError>>> {
        (*self).create_analysis(builder)
    }
    fn create_dashboard(&self, builder: CreateDashboardInputBuilder) -> impl Future<Output = Result<CreateDashboardOutput, SdkError<CreateDashboardError>>> {
        (*self).create_dashboard(builder)
    }
    fn create_data_set(&self, builder: CreateDataSetInputBuilder) -> impl Future<Output = Result<CreateDataSetOutput, SdkError<CreateDataSetError>>> {
        (*self).create_data_set(builder)
    }
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>> {
        (*self).create_data_source(builder)
    }
    fn create_folder(&self, builder: CreateFolderInputBuilder) -> impl Future<Output = Result<CreateFolderOutput, SdkError<CreateFolderError>>> {
        (*self).create_folder(builder)
    }
    fn create_folder_membership(&self, builder: CreateFolderMembershipInputBuilder) -> impl Future<Output = Result<CreateFolderMembershipOutput, SdkError<CreateFolderMembershipError>>> {
        (*self).create_folder_membership(builder)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        (*self).create_group(builder)
    }
    fn create_group_membership(&self, builder: CreateGroupMembershipInputBuilder) -> impl Future<Output = Result<CreateGroupMembershipOutput, SdkError<CreateGroupMembershipError>>> {
        (*self).create_group_membership(builder)
    }
    fn create_iam_policy_assignment(&self, builder: CreateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<CreateIamPolicyAssignmentOutput, SdkError<CreateIAMPolicyAssignmentError>>> {
        (*self).create_iam_policy_assignment(builder)
    }
    fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> impl Future<Output = Result<CreateIngestionOutput, SdkError<CreateIngestionError>>> {
        (*self).create_ingestion(builder)
    }
    fn create_namespace(&self, builder: CreateNamespaceInputBuilder) -> impl Future<Output = Result<CreateNamespaceOutput, SdkError<CreateNamespaceError>>> {
        (*self).create_namespace(builder)
    }
    fn create_refresh_schedule(&self, builder: CreateRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateRefreshScheduleOutput, SdkError<CreateRefreshScheduleError>>> {
        (*self).create_refresh_schedule(builder)
    }
    fn create_role_membership(&self, builder: CreateRoleMembershipInputBuilder) -> impl Future<Output = Result<CreateRoleMembershipOutput, SdkError<CreateRoleMembershipError>>> {
        (*self).create_role_membership(builder)
    }
    fn create_template(&self, builder: CreateTemplateInputBuilder) -> impl Future<Output = Result<CreateTemplateOutput, SdkError<CreateTemplateError>>> {
        (*self).create_template(builder)
    }
    fn create_template_alias(&self, builder: CreateTemplateAliasInputBuilder) -> impl Future<Output = Result<CreateTemplateAliasOutput, SdkError<CreateTemplateAliasError>>> {
        (*self).create_template_alias(builder)
    }
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>> {
        (*self).create_theme(builder)
    }
    fn create_theme_alias(&self, builder: CreateThemeAliasInputBuilder) -> impl Future<Output = Result<CreateThemeAliasOutput, SdkError<CreateThemeAliasError>>> {
        (*self).create_theme_alias(builder)
    }
    fn create_topic(&self, builder: CreateTopicInputBuilder) -> impl Future<Output = Result<CreateTopicOutput, SdkError<CreateTopicError>>> {
        (*self).create_topic(builder)
    }
    fn create_topic_refresh_schedule(&self, builder: CreateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<CreateTopicRefreshScheduleOutput, SdkError<CreateTopicRefreshScheduleError>>> {
        (*self).create_topic_refresh_schedule(builder)
    }
    fn create_vpc_connection(&self, builder: CreateVpcConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcConnectionOutput, SdkError<CreateVPCConnectionError>>> {
        (*self).create_vpc_connection(builder)
    }
    fn delete_account_customization(&self, builder: DeleteAccountCustomizationInputBuilder) -> impl Future<Output = Result<DeleteAccountCustomizationOutput, SdkError<DeleteAccountCustomizationError>>> {
        (*self).delete_account_customization(builder)
    }
    fn delete_account_subscription(&self, builder: DeleteAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteAccountSubscriptionOutput, SdkError<DeleteAccountSubscriptionError>>> {
        (*self).delete_account_subscription(builder)
    }
    fn delete_analysis(&self, builder: DeleteAnalysisInputBuilder) -> impl Future<Output = Result<DeleteAnalysisOutput, SdkError<DeleteAnalysisError>>> {
        (*self).delete_analysis(builder)
    }
    fn delete_dashboard(&self, builder: DeleteDashboardInputBuilder) -> impl Future<Output = Result<DeleteDashboardOutput, SdkError<DeleteDashboardError>>> {
        (*self).delete_dashboard(builder)
    }
    fn delete_data_set(&self, builder: DeleteDataSetInputBuilder) -> impl Future<Output = Result<DeleteDataSetOutput, SdkError<DeleteDataSetError>>> {
        (*self).delete_data_set(builder)
    }
    fn delete_data_set_refresh_properties(&self, builder: DeleteDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DeleteDataSetRefreshPropertiesOutput, SdkError<DeleteDataSetRefreshPropertiesError>>> {
        (*self).delete_data_set_refresh_properties(builder)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        (*self).delete_data_source(builder)
    }
    fn delete_folder(&self, builder: DeleteFolderInputBuilder) -> impl Future<Output = Result<DeleteFolderOutput, SdkError<DeleteFolderError>>> {
        (*self).delete_folder(builder)
    }
    fn delete_folder_membership(&self, builder: DeleteFolderMembershipInputBuilder) -> impl Future<Output = Result<DeleteFolderMembershipOutput, SdkError<DeleteFolderMembershipError>>> {
        (*self).delete_folder_membership(builder)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        (*self).delete_group(builder)
    }
    fn delete_group_membership(&self, builder: DeleteGroupMembershipInputBuilder) -> impl Future<Output = Result<DeleteGroupMembershipOutput, SdkError<DeleteGroupMembershipError>>> {
        (*self).delete_group_membership(builder)
    }
    fn delete_iam_policy_assignment(&self, builder: DeleteIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DeleteIamPolicyAssignmentOutput, SdkError<DeleteIAMPolicyAssignmentError>>> {
        (*self).delete_iam_policy_assignment(builder)
    }
    fn delete_identity_propagation_config(&self, builder: DeleteIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<DeleteIdentityPropagationConfigOutput, SdkError<DeleteIdentityPropagationConfigError>>> {
        (*self).delete_identity_propagation_config(builder)
    }
    fn delete_namespace(&self, builder: DeleteNamespaceInputBuilder) -> impl Future<Output = Result<DeleteNamespaceOutput, SdkError<DeleteNamespaceError>>> {
        (*self).delete_namespace(builder)
    }
    fn delete_refresh_schedule(&self, builder: DeleteRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteRefreshScheduleOutput, SdkError<DeleteRefreshScheduleError>>> {
        (*self).delete_refresh_schedule(builder)
    }
    fn delete_role_custom_permission(&self, builder: DeleteRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DeleteRoleCustomPermissionOutput, SdkError<DeleteRoleCustomPermissionError>>> {
        (*self).delete_role_custom_permission(builder)
    }
    fn delete_role_membership(&self, builder: DeleteRoleMembershipInputBuilder) -> impl Future<Output = Result<DeleteRoleMembershipOutput, SdkError<DeleteRoleMembershipError>>> {
        (*self).delete_role_membership(builder)
    }
    fn delete_template(&self, builder: DeleteTemplateInputBuilder) -> impl Future<Output = Result<DeleteTemplateOutput, SdkError<DeleteTemplateError>>> {
        (*self).delete_template(builder)
    }
    fn delete_template_alias(&self, builder: DeleteTemplateAliasInputBuilder) -> impl Future<Output = Result<DeleteTemplateAliasOutput, SdkError<DeleteTemplateAliasError>>> {
        (*self).delete_template_alias(builder)
    }
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>> {
        (*self).delete_theme(builder)
    }
    fn delete_theme_alias(&self, builder: DeleteThemeAliasInputBuilder) -> impl Future<Output = Result<DeleteThemeAliasOutput, SdkError<DeleteThemeAliasError>>> {
        (*self).delete_theme_alias(builder)
    }
    fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> impl Future<Output = Result<DeleteTopicOutput, SdkError<DeleteTopicError>>> {
        (*self).delete_topic(builder)
    }
    fn delete_topic_refresh_schedule(&self, builder: DeleteTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DeleteTopicRefreshScheduleOutput, SdkError<DeleteTopicRefreshScheduleError>>> {
        (*self).delete_topic_refresh_schedule(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        (*self).delete_user(builder)
    }
    fn delete_user_by_principal_id(&self, builder: DeleteUserByPrincipalIdInputBuilder) -> impl Future<Output = Result<DeleteUserByPrincipalIdOutput, SdkError<DeleteUserByPrincipalIdError>>> {
        (*self).delete_user_by_principal_id(builder)
    }
    fn delete_vpc_connection(&self, builder: DeleteVpcConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectionOutput, SdkError<DeleteVPCConnectionError>>> {
        (*self).delete_vpc_connection(builder)
    }
    fn describe_account_customization(&self, builder: DescribeAccountCustomizationInputBuilder) -> impl Future<Output = Result<DescribeAccountCustomizationOutput, SdkError<DescribeAccountCustomizationError>>> {
        (*self).describe_account_customization(builder)
    }
    fn describe_account_settings(&self, builder: DescribeAccountSettingsInputBuilder) -> impl Future<Output = Result<DescribeAccountSettingsOutput, SdkError<DescribeAccountSettingsError>>> {
        (*self).describe_account_settings(builder)
    }
    fn describe_account_subscription(&self, builder: DescribeAccountSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeAccountSubscriptionOutput, SdkError<DescribeAccountSubscriptionError>>> {
        (*self).describe_account_subscription(builder)
    }
    fn describe_analysis(&self, builder: DescribeAnalysisInputBuilder) -> impl Future<Output = Result<DescribeAnalysisOutput, SdkError<DescribeAnalysisError>>> {
        (*self).describe_analysis(builder)
    }
    fn describe_analysis_definition(&self, builder: DescribeAnalysisDefinitionInputBuilder) -> impl Future<Output = Result<DescribeAnalysisDefinitionOutput, SdkError<DescribeAnalysisDefinitionError>>> {
        (*self).describe_analysis_definition(builder)
    }
    fn describe_analysis_permissions(&self, builder: DescribeAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<DescribeAnalysisPermissionsOutput, SdkError<DescribeAnalysisPermissionsError>>> {
        (*self).describe_analysis_permissions(builder)
    }
    fn describe_asset_bundle_export_job(&self, builder: DescribeAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleExportJobOutput, SdkError<DescribeAssetBundleExportJobError>>> {
        (*self).describe_asset_bundle_export_job(builder)
    }
    fn describe_asset_bundle_import_job(&self, builder: DescribeAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<DescribeAssetBundleImportJobOutput, SdkError<DescribeAssetBundleImportJobError>>> {
        (*self).describe_asset_bundle_import_job(builder)
    }
    fn describe_dashboard(&self, builder: DescribeDashboardInputBuilder) -> impl Future<Output = Result<DescribeDashboardOutput, SdkError<DescribeDashboardError>>> {
        (*self).describe_dashboard(builder)
    }
    fn describe_dashboard_definition(&self, builder: DescribeDashboardDefinitionInputBuilder) -> impl Future<Output = Result<DescribeDashboardDefinitionOutput, SdkError<DescribeDashboardDefinitionError>>> {
        (*self).describe_dashboard_definition(builder)
    }
    fn describe_dashboard_permissions(&self, builder: DescribeDashboardPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDashboardPermissionsOutput, SdkError<DescribeDashboardPermissionsError>>> {
        (*self).describe_dashboard_permissions(builder)
    }
    fn describe_dashboard_snapshot_job(&self, builder: DescribeDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobOutput, SdkError<DescribeDashboardSnapshotJobError>>> {
        (*self).describe_dashboard_snapshot_job(builder)
    }
    fn describe_dashboard_snapshot_job_result(&self, builder: DescribeDashboardSnapshotJobResultInputBuilder) -> impl Future<Output = Result<DescribeDashboardSnapshotJobResultOutput, SdkError<DescribeDashboardSnapshotJobResultError>>> {
        (*self).describe_dashboard_snapshot_job_result(builder)
    }
    fn describe_data_set(&self, builder: DescribeDataSetInputBuilder) -> impl Future<Output = Result<DescribeDataSetOutput, SdkError<DescribeDataSetError>>> {
        (*self).describe_data_set(builder)
    }
    fn describe_data_set_permissions(&self, builder: DescribeDataSetPermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSetPermissionsOutput, SdkError<DescribeDataSetPermissionsError>>> {
        (*self).describe_data_set_permissions(builder)
    }
    fn describe_data_set_refresh_properties(&self, builder: DescribeDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<DescribeDataSetRefreshPropertiesOutput, SdkError<DescribeDataSetRefreshPropertiesError>>> {
        (*self).describe_data_set_refresh_properties(builder)
    }
    fn describe_data_source(&self, builder: DescribeDataSourceInputBuilder) -> impl Future<Output = Result<DescribeDataSourceOutput, SdkError<DescribeDataSourceError>>> {
        (*self).describe_data_source(builder)
    }
    fn describe_data_source_permissions(&self, builder: DescribeDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<DescribeDataSourcePermissionsOutput, SdkError<DescribeDataSourcePermissionsError>>> {
        (*self).describe_data_source_permissions(builder)
    }
    fn describe_folder(&self, builder: DescribeFolderInputBuilder) -> impl Future<Output = Result<DescribeFolderOutput, SdkError<DescribeFolderError>>> {
        (*self).describe_folder(builder)
    }
    fn describe_folder_permissions(&self, builder: DescribeFolderPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderPermissionsOutput, SdkError<DescribeFolderPermissionsError>>> {
        (*self).describe_folder_permissions(builder)
    }
    fn describe_folder_resolved_permissions(&self, builder: DescribeFolderResolvedPermissionsInputBuilder) -> impl Future<Output = Result<DescribeFolderResolvedPermissionsOutput, SdkError<DescribeFolderResolvedPermissionsError>>> {
        (*self).describe_folder_resolved_permissions(builder)
    }
    fn describe_group(&self, builder: DescribeGroupInputBuilder) -> impl Future<Output = Result<DescribeGroupOutput, SdkError<DescribeGroupError>>> {
        (*self).describe_group(builder)
    }
    fn describe_group_membership(&self, builder: DescribeGroupMembershipInputBuilder) -> impl Future<Output = Result<DescribeGroupMembershipOutput, SdkError<DescribeGroupMembershipError>>> {
        (*self).describe_group_membership(builder)
    }
    fn describe_iam_policy_assignment(&self, builder: DescribeIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<DescribeIamPolicyAssignmentOutput, SdkError<DescribeIAMPolicyAssignmentError>>> {
        (*self).describe_iam_policy_assignment(builder)
    }
    fn describe_ingestion(&self, builder: DescribeIngestionInputBuilder) -> impl Future<Output = Result<DescribeIngestionOutput, SdkError<DescribeIngestionError>>> {
        (*self).describe_ingestion(builder)
    }
    fn describe_ip_restriction(&self, builder: DescribeIpRestrictionInputBuilder) -> impl Future<Output = Result<DescribeIpRestrictionOutput, SdkError<DescribeIpRestrictionError>>> {
        (*self).describe_ip_restriction(builder)
    }
    fn describe_key_registration(&self, builder: DescribeKeyRegistrationInputBuilder) -> impl Future<Output = Result<DescribeKeyRegistrationOutput, SdkError<DescribeKeyRegistrationError>>> {
        (*self).describe_key_registration(builder)
    }
    fn describe_namespace(&self, builder: DescribeNamespaceInputBuilder) -> impl Future<Output = Result<DescribeNamespaceOutput, SdkError<DescribeNamespaceError>>> {
        (*self).describe_namespace(builder)
    }
    fn describe_refresh_schedule(&self, builder: DescribeRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeRefreshScheduleOutput, SdkError<DescribeRefreshScheduleError>>> {
        (*self).describe_refresh_schedule(builder)
    }
    fn describe_role_custom_permission(&self, builder: DescribeRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<DescribeRoleCustomPermissionOutput, SdkError<DescribeRoleCustomPermissionError>>> {
        (*self).describe_role_custom_permission(builder)
    }
    fn describe_template(&self, builder: DescribeTemplateInputBuilder) -> impl Future<Output = Result<DescribeTemplateOutput, SdkError<DescribeTemplateError>>> {
        (*self).describe_template(builder)
    }
    fn describe_template_alias(&self, builder: DescribeTemplateAliasInputBuilder) -> impl Future<Output = Result<DescribeTemplateAliasOutput, SdkError<DescribeTemplateAliasError>>> {
        (*self).describe_template_alias(builder)
    }
    fn describe_template_definition(&self, builder: DescribeTemplateDefinitionInputBuilder) -> impl Future<Output = Result<DescribeTemplateDefinitionOutput, SdkError<DescribeTemplateDefinitionError>>> {
        (*self).describe_template_definition(builder)
    }
    fn describe_template_permissions(&self, builder: DescribeTemplatePermissionsInputBuilder) -> impl Future<Output = Result<DescribeTemplatePermissionsOutput, SdkError<DescribeTemplatePermissionsError>>> {
        (*self).describe_template_permissions(builder)
    }
    fn describe_theme(&self, builder: DescribeThemeInputBuilder) -> impl Future<Output = Result<DescribeThemeOutput, SdkError<DescribeThemeError>>> {
        (*self).describe_theme(builder)
    }
    fn describe_theme_alias(&self, builder: DescribeThemeAliasInputBuilder) -> impl Future<Output = Result<DescribeThemeAliasOutput, SdkError<DescribeThemeAliasError>>> {
        (*self).describe_theme_alias(builder)
    }
    fn describe_theme_permissions(&self, builder: DescribeThemePermissionsInputBuilder) -> impl Future<Output = Result<DescribeThemePermissionsOutput, SdkError<DescribeThemePermissionsError>>> {
        (*self).describe_theme_permissions(builder)
    }
    fn describe_topic(&self, builder: DescribeTopicInputBuilder) -> impl Future<Output = Result<DescribeTopicOutput, SdkError<DescribeTopicError>>> {
        (*self).describe_topic(builder)
    }
    fn describe_topic_permissions(&self, builder: DescribeTopicPermissionsInputBuilder) -> impl Future<Output = Result<DescribeTopicPermissionsOutput, SdkError<DescribeTopicPermissionsError>>> {
        (*self).describe_topic_permissions(builder)
    }
    fn describe_topic_refresh(&self, builder: DescribeTopicRefreshInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshOutput, SdkError<DescribeTopicRefreshError>>> {
        (*self).describe_topic_refresh(builder)
    }
    fn describe_topic_refresh_schedule(&self, builder: DescribeTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<DescribeTopicRefreshScheduleOutput, SdkError<DescribeTopicRefreshScheduleError>>> {
        (*self).describe_topic_refresh_schedule(builder)
    }
    fn describe_user(&self, builder: DescribeUserInputBuilder) -> impl Future<Output = Result<DescribeUserOutput, SdkError<DescribeUserError>>> {
        (*self).describe_user(builder)
    }
    fn describe_vpc_connection(&self, builder: DescribeVpcConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectionOutput, SdkError<DescribeVPCConnectionError>>> {
        (*self).describe_vpc_connection(builder)
    }
    fn generate_embed_url_for_anonymous_user(&self, builder: GenerateEmbedUrlForAnonymousUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForAnonymousUserOutput, SdkError<GenerateEmbedUrlForAnonymousUserError>>> {
        (*self).generate_embed_url_for_anonymous_user(builder)
    }
    fn generate_embed_url_for_registered_user(&self, builder: GenerateEmbedUrlForRegisteredUserInputBuilder) -> impl Future<Output = Result<GenerateEmbedUrlForRegisteredUserOutput, SdkError<GenerateEmbedUrlForRegisteredUserError>>> {
        (*self).generate_embed_url_for_registered_user(builder)
    }
    fn get_dashboard_embed_url(&self, builder: GetDashboardEmbedUrlInputBuilder) -> impl Future<Output = Result<GetDashboardEmbedUrlOutput, SdkError<GetDashboardEmbedUrlError>>> {
        (*self).get_dashboard_embed_url(builder)
    }
    fn get_session_embed_url(&self, builder: GetSessionEmbedUrlInputBuilder) -> impl Future<Output = Result<GetSessionEmbedUrlOutput, SdkError<GetSessionEmbedUrlError>>> {
        (*self).get_session_embed_url(builder)
    }
    fn list_analyses(&self, builder: ListAnalysesInputBuilder) -> impl Future<Output = Result<ListAnalysesOutput, SdkError<ListAnalysesError>>> {
        (*self).list_analyses(builder)
    }
    fn list_asset_bundle_export_jobs(&self, builder: ListAssetBundleExportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleExportJobsOutput, SdkError<ListAssetBundleExportJobsError>>> {
        (*self).list_asset_bundle_export_jobs(builder)
    }
    fn list_asset_bundle_import_jobs(&self, builder: ListAssetBundleImportJobsInputBuilder) -> impl Future<Output = Result<ListAssetBundleImportJobsOutput, SdkError<ListAssetBundleImportJobsError>>> {
        (*self).list_asset_bundle_import_jobs(builder)
    }
    fn list_dashboard_versions(&self, builder: ListDashboardVersionsInputBuilder) -> impl Future<Output = Result<ListDashboardVersionsOutput, SdkError<ListDashboardVersionsError>>> {
        (*self).list_dashboard_versions(builder)
    }
    fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> impl Future<Output = Result<ListDashboardsOutput, SdkError<ListDashboardsError>>> {
        (*self).list_dashboards(builder)
    }
    fn list_data_sets(&self, builder: ListDataSetsInputBuilder) -> impl Future<Output = Result<ListDataSetsOutput, SdkError<ListDataSetsError>>> {
        (*self).list_data_sets(builder)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        (*self).list_data_sources(builder)
    }
    fn list_folder_members(&self, builder: ListFolderMembersInputBuilder) -> impl Future<Output = Result<ListFolderMembersOutput, SdkError<ListFolderMembersError>>> {
        (*self).list_folder_members(builder)
    }
    fn list_folders(&self, builder: ListFoldersInputBuilder) -> impl Future<Output = Result<ListFoldersOutput, SdkError<ListFoldersError>>> {
        (*self).list_folders(builder)
    }
    fn list_group_memberships(&self, builder: ListGroupMembershipsInputBuilder) -> impl Future<Output = Result<ListGroupMembershipsOutput, SdkError<ListGroupMembershipsError>>> {
        (*self).list_group_memberships(builder)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        (*self).list_groups(builder)
    }
    fn list_iam_policy_assignments(&self, builder: ListIamPolicyAssignmentsInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsOutput, SdkError<ListIAMPolicyAssignmentsError>>> {
        (*self).list_iam_policy_assignments(builder)
    }
    fn list_iam_policy_assignments_for_user(&self, builder: ListIamPolicyAssignmentsForUserInputBuilder) -> impl Future<Output = Result<ListIamPolicyAssignmentsForUserOutput, SdkError<ListIAMPolicyAssignmentsForUserError>>> {
        (*self).list_iam_policy_assignments_for_user(builder)
    }
    fn list_identity_propagation_configs(&self, builder: ListIdentityPropagationConfigsInputBuilder) -> impl Future<Output = Result<ListIdentityPropagationConfigsOutput, SdkError<ListIdentityPropagationConfigsError>>> {
        (*self).list_identity_propagation_configs(builder)
    }
    fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> impl Future<Output = Result<ListIngestionsOutput, SdkError<ListIngestionsError>>> {
        (*self).list_ingestions(builder)
    }
    fn list_namespaces(&self, builder: ListNamespacesInputBuilder) -> impl Future<Output = Result<ListNamespacesOutput, SdkError<ListNamespacesError>>> {
        (*self).list_namespaces(builder)
    }
    fn list_refresh_schedules(&self, builder: ListRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListRefreshSchedulesOutput, SdkError<ListRefreshSchedulesError>>> {
        (*self).list_refresh_schedules(builder)
    }
    fn list_role_memberships(&self, builder: ListRoleMembershipsInputBuilder) -> impl Future<Output = Result<ListRoleMembershipsOutput, SdkError<ListRoleMembershipsError>>> {
        (*self).list_role_memberships(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn list_template_aliases(&self, builder: ListTemplateAliasesInputBuilder) -> impl Future<Output = Result<ListTemplateAliasesOutput, SdkError<ListTemplateAliasesError>>> {
        (*self).list_template_aliases(builder)
    }
    fn list_template_versions(&self, builder: ListTemplateVersionsInputBuilder) -> impl Future<Output = Result<ListTemplateVersionsOutput, SdkError<ListTemplateVersionsError>>> {
        (*self).list_template_versions(builder)
    }
    fn list_templates(&self, builder: ListTemplatesInputBuilder) -> impl Future<Output = Result<ListTemplatesOutput, SdkError<ListTemplatesError>>> {
        (*self).list_templates(builder)
    }
    fn list_theme_aliases(&self, builder: ListThemeAliasesInputBuilder) -> impl Future<Output = Result<ListThemeAliasesOutput, SdkError<ListThemeAliasesError>>> {
        (*self).list_theme_aliases(builder)
    }
    fn list_theme_versions(&self, builder: ListThemeVersionsInputBuilder) -> impl Future<Output = Result<ListThemeVersionsOutput, SdkError<ListThemeVersionsError>>> {
        (*self).list_theme_versions(builder)
    }
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>> {
        (*self).list_themes(builder)
    }
    fn list_topic_refresh_schedules(&self, builder: ListTopicRefreshSchedulesInputBuilder) -> impl Future<Output = Result<ListTopicRefreshSchedulesOutput, SdkError<ListTopicRefreshSchedulesError>>> {
        (*self).list_topic_refresh_schedules(builder)
    }
    fn list_topic_reviewed_answers(&self, builder: ListTopicReviewedAnswersInputBuilder) -> impl Future<Output = Result<ListTopicReviewedAnswersOutput, SdkError<ListTopicReviewedAnswersError>>> {
        (*self).list_topic_reviewed_answers(builder)
    }
    fn list_topics(&self, builder: ListTopicsInputBuilder) -> impl Future<Output = Result<ListTopicsOutput, SdkError<ListTopicsError>>> {
        (*self).list_topics(builder)
    }
    fn list_user_groups(&self, builder: ListUserGroupsInputBuilder) -> impl Future<Output = Result<ListUserGroupsOutput, SdkError<ListUserGroupsError>>> {
        (*self).list_user_groups(builder)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        (*self).list_users(builder)
    }
    fn list_vpc_connections(&self, builder: ListVpcConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcConnectionsOutput, SdkError<ListVPCConnectionsError>>> {
        (*self).list_vpc_connections(builder)
    }
    fn put_data_set_refresh_properties(&self, builder: PutDataSetRefreshPropertiesInputBuilder) -> impl Future<Output = Result<PutDataSetRefreshPropertiesOutput, SdkError<PutDataSetRefreshPropertiesError>>> {
        (*self).put_data_set_refresh_properties(builder)
    }
    fn register_user(&self, builder: RegisterUserInputBuilder) -> impl Future<Output = Result<RegisterUserOutput, SdkError<RegisterUserError>>> {
        (*self).register_user(builder)
    }
    fn restore_analysis(&self, builder: RestoreAnalysisInputBuilder) -> impl Future<Output = Result<RestoreAnalysisOutput, SdkError<RestoreAnalysisError>>> {
        (*self).restore_analysis(builder)
    }
    fn search_analyses(&self, builder: SearchAnalysesInputBuilder) -> impl Future<Output = Result<SearchAnalysesOutput, SdkError<SearchAnalysesError>>> {
        (*self).search_analyses(builder)
    }
    fn search_dashboards(&self, builder: SearchDashboardsInputBuilder) -> impl Future<Output = Result<SearchDashboardsOutput, SdkError<SearchDashboardsError>>> {
        (*self).search_dashboards(builder)
    }
    fn search_data_sets(&self, builder: SearchDataSetsInputBuilder) -> impl Future<Output = Result<SearchDataSetsOutput, SdkError<SearchDataSetsError>>> {
        (*self).search_data_sets(builder)
    }
    fn search_data_sources(&self, builder: SearchDataSourcesInputBuilder) -> impl Future<Output = Result<SearchDataSourcesOutput, SdkError<SearchDataSourcesError>>> {
        (*self).search_data_sources(builder)
    }
    fn search_folders(&self, builder: SearchFoldersInputBuilder) -> impl Future<Output = Result<SearchFoldersOutput, SdkError<SearchFoldersError>>> {
        (*self).search_folders(builder)
    }
    fn search_groups(&self, builder: SearchGroupsInputBuilder) -> impl Future<Output = Result<SearchGroupsOutput, SdkError<SearchGroupsError>>> {
        (*self).search_groups(builder)
    }
    fn start_asset_bundle_export_job(&self, builder: StartAssetBundleExportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleExportJobOutput, SdkError<StartAssetBundleExportJobError>>> {
        (*self).start_asset_bundle_export_job(builder)
    }
    fn start_asset_bundle_import_job(&self, builder: StartAssetBundleImportJobInputBuilder) -> impl Future<Output = Result<StartAssetBundleImportJobOutput, SdkError<StartAssetBundleImportJobError>>> {
        (*self).start_asset_bundle_import_job(builder)
    }
    fn start_dashboard_snapshot_job(&self, builder: StartDashboardSnapshotJobInputBuilder) -> impl Future<Output = Result<StartDashboardSnapshotJobOutput, SdkError<StartDashboardSnapshotJobError>>> {
        (*self).start_dashboard_snapshot_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_account_customization(&self, builder: UpdateAccountCustomizationInputBuilder) -> impl Future<Output = Result<UpdateAccountCustomizationOutput, SdkError<UpdateAccountCustomizationError>>> {
        (*self).update_account_customization(builder)
    }
    fn update_account_settings(&self, builder: UpdateAccountSettingsInputBuilder) -> impl Future<Output = Result<UpdateAccountSettingsOutput, SdkError<UpdateAccountSettingsError>>> {
        (*self).update_account_settings(builder)
    }
    fn update_analysis(&self, builder: UpdateAnalysisInputBuilder) -> impl Future<Output = Result<UpdateAnalysisOutput, SdkError<UpdateAnalysisError>>> {
        (*self).update_analysis(builder)
    }
    fn update_analysis_permissions(&self, builder: UpdateAnalysisPermissionsInputBuilder) -> impl Future<Output = Result<UpdateAnalysisPermissionsOutput, SdkError<UpdateAnalysisPermissionsError>>> {
        (*self).update_analysis_permissions(builder)
    }
    fn update_dashboard(&self, builder: UpdateDashboardInputBuilder) -> impl Future<Output = Result<UpdateDashboardOutput, SdkError<UpdateDashboardError>>> {
        (*self).update_dashboard(builder)
    }
    fn update_dashboard_links(&self, builder: UpdateDashboardLinksInputBuilder) -> impl Future<Output = Result<UpdateDashboardLinksOutput, SdkError<UpdateDashboardLinksError>>> {
        (*self).update_dashboard_links(builder)
    }
    fn update_dashboard_permissions(&self, builder: UpdateDashboardPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDashboardPermissionsOutput, SdkError<UpdateDashboardPermissionsError>>> {
        (*self).update_dashboard_permissions(builder)
    }
    fn update_dashboard_published_version(&self, builder: UpdateDashboardPublishedVersionInputBuilder) -> impl Future<Output = Result<UpdateDashboardPublishedVersionOutput, SdkError<UpdateDashboardPublishedVersionError>>> {
        (*self).update_dashboard_published_version(builder)
    }
    fn update_data_set(&self, builder: UpdateDataSetInputBuilder) -> impl Future<Output = Result<UpdateDataSetOutput, SdkError<UpdateDataSetError>>> {
        (*self).update_data_set(builder)
    }
    fn update_data_set_permissions(&self, builder: UpdateDataSetPermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSetPermissionsOutput, SdkError<UpdateDataSetPermissionsError>>> {
        (*self).update_data_set_permissions(builder)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        (*self).update_data_source(builder)
    }
    fn update_data_source_permissions(&self, builder: UpdateDataSourcePermissionsInputBuilder) -> impl Future<Output = Result<UpdateDataSourcePermissionsOutput, SdkError<UpdateDataSourcePermissionsError>>> {
        (*self).update_data_source_permissions(builder)
    }
    fn update_folder(&self, builder: UpdateFolderInputBuilder) -> impl Future<Output = Result<UpdateFolderOutput, SdkError<UpdateFolderError>>> {
        (*self).update_folder(builder)
    }
    fn update_folder_permissions(&self, builder: UpdateFolderPermissionsInputBuilder) -> impl Future<Output = Result<UpdateFolderPermissionsOutput, SdkError<UpdateFolderPermissionsError>>> {
        (*self).update_folder_permissions(builder)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        (*self).update_group(builder)
    }
    fn update_iam_policy_assignment(&self, builder: UpdateIamPolicyAssignmentInputBuilder) -> impl Future<Output = Result<UpdateIamPolicyAssignmentOutput, SdkError<UpdateIAMPolicyAssignmentError>>> {
        (*self).update_iam_policy_assignment(builder)
    }
    fn update_identity_propagation_config(&self, builder: UpdateIdentityPropagationConfigInputBuilder) -> impl Future<Output = Result<UpdateIdentityPropagationConfigOutput, SdkError<UpdateIdentityPropagationConfigError>>> {
        (*self).update_identity_propagation_config(builder)
    }
    fn update_ip_restriction(&self, builder: UpdateIpRestrictionInputBuilder) -> impl Future<Output = Result<UpdateIpRestrictionOutput, SdkError<UpdateIpRestrictionError>>> {
        (*self).update_ip_restriction(builder)
    }
    fn update_key_registration(&self, builder: UpdateKeyRegistrationInputBuilder) -> impl Future<Output = Result<UpdateKeyRegistrationOutput, SdkError<UpdateKeyRegistrationError>>> {
        (*self).update_key_registration(builder)
    }
    fn update_public_sharing_settings(&self, builder: UpdatePublicSharingSettingsInputBuilder) -> impl Future<Output = Result<UpdatePublicSharingSettingsOutput, SdkError<UpdatePublicSharingSettingsError>>> {
        (*self).update_public_sharing_settings(builder)
    }
    fn update_refresh_schedule(&self, builder: UpdateRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateRefreshScheduleOutput, SdkError<UpdateRefreshScheduleError>>> {
        (*self).update_refresh_schedule(builder)
    }
    fn update_role_custom_permission(&self, builder: UpdateRoleCustomPermissionInputBuilder) -> impl Future<Output = Result<UpdateRoleCustomPermissionOutput, SdkError<UpdateRoleCustomPermissionError>>> {
        (*self).update_role_custom_permission(builder)
    }
    fn update_spice_capacity_configuration(&self, builder: UpdateSpiceCapacityConfigurationInputBuilder) -> impl Future<Output = Result<UpdateSpiceCapacityConfigurationOutput, SdkError<UpdateSPICECapacityConfigurationError>>> {
        (*self).update_spice_capacity_configuration(builder)
    }
    fn update_template(&self, builder: UpdateTemplateInputBuilder) -> impl Future<Output = Result<UpdateTemplateOutput, SdkError<UpdateTemplateError>>> {
        (*self).update_template(builder)
    }
    fn update_template_alias(&self, builder: UpdateTemplateAliasInputBuilder) -> impl Future<Output = Result<UpdateTemplateAliasOutput, SdkError<UpdateTemplateAliasError>>> {
        (*self).update_template_alias(builder)
    }
    fn update_template_permissions(&self, builder: UpdateTemplatePermissionsInputBuilder) -> impl Future<Output = Result<UpdateTemplatePermissionsOutput, SdkError<UpdateTemplatePermissionsError>>> {
        (*self).update_template_permissions(builder)
    }
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>> {
        (*self).update_theme(builder)
    }
    fn update_theme_alias(&self, builder: UpdateThemeAliasInputBuilder) -> impl Future<Output = Result<UpdateThemeAliasOutput, SdkError<UpdateThemeAliasError>>> {
        (*self).update_theme_alias(builder)
    }
    fn update_theme_permissions(&self, builder: UpdateThemePermissionsInputBuilder) -> impl Future<Output = Result<UpdateThemePermissionsOutput, SdkError<UpdateThemePermissionsError>>> {
        (*self).update_theme_permissions(builder)
    }
    fn update_topic(&self, builder: UpdateTopicInputBuilder) -> impl Future<Output = Result<UpdateTopicOutput, SdkError<UpdateTopicError>>> {
        (*self).update_topic(builder)
    }
    fn update_topic_permissions(&self, builder: UpdateTopicPermissionsInputBuilder) -> impl Future<Output = Result<UpdateTopicPermissionsOutput, SdkError<UpdateTopicPermissionsError>>> {
        (*self).update_topic_permissions(builder)
    }
    fn update_topic_refresh_schedule(&self, builder: UpdateTopicRefreshScheduleInputBuilder) -> impl Future<Output = Result<UpdateTopicRefreshScheduleOutput, SdkError<UpdateTopicRefreshScheduleError>>> {
        (*self).update_topic_refresh_schedule(builder)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        (*self).update_user(builder)
    }
    fn update_vpc_connection(&self, builder: UpdateVpcConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcConnectionOutput, SdkError<UpdateVPCConnectionError>>> {
        (*self).update_vpc_connection(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edQuickSightClient {}
    impl QuickSightClient for edQuickSightClient {
        async fn batch_create_topic_reviewed_answer(&self, builder: BatchCreateTopicReviewedAnswerInputBuilder) -> Result<BatchCreateTopicReviewedAnswerOutput, SdkError<BatchCreateTopicReviewedAnswerError>>;
        async fn batch_delete_topic_reviewed_answer(&self, builder: BatchDeleteTopicReviewedAnswerInputBuilder) -> Result<BatchDeleteTopicReviewedAnswerOutput, SdkError<BatchDeleteTopicReviewedAnswerError>>;
        async fn cancel_ingestion(&self, builder: CancelIngestionInputBuilder) -> Result<CancelIngestionOutput, SdkError<CancelIngestionError>>;
        async fn create_account_customization(&self, builder: CreateAccountCustomizationInputBuilder) -> Result<CreateAccountCustomizationOutput, SdkError<CreateAccountCustomizationError>>;
        async fn create_account_subscription(&self, builder: CreateAccountSubscriptionInputBuilder) -> Result<CreateAccountSubscriptionOutput, SdkError<CreateAccountSubscriptionError>>;
        async fn create_analysis(&self, builder: CreateAnalysisInputBuilder) -> Result<CreateAnalysisOutput, SdkError<CreateAnalysisError>>;
        async fn create_dashboard(&self, builder: CreateDashboardInputBuilder) -> Result<CreateDashboardOutput, SdkError<CreateDashboardError>>;
        async fn create_data_set(&self, builder: CreateDataSetInputBuilder) -> Result<CreateDataSetOutput, SdkError<CreateDataSetError>>;
        async fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>;
        async fn create_folder(&self, builder: CreateFolderInputBuilder) -> Result<CreateFolderOutput, SdkError<CreateFolderError>>;
        async fn create_folder_membership(&self, builder: CreateFolderMembershipInputBuilder) -> Result<CreateFolderMembershipOutput, SdkError<CreateFolderMembershipError>>;
        async fn create_group(&self, builder: CreateGroupInputBuilder) -> Result<CreateGroupOutput, SdkError<CreateGroupError>>;
        async fn create_group_membership(&self, builder: CreateGroupMembershipInputBuilder) -> Result<CreateGroupMembershipOutput, SdkError<CreateGroupMembershipError>>;
        async fn create_iam_policy_assignment(&self, builder: CreateIamPolicyAssignmentInputBuilder) -> Result<CreateIamPolicyAssignmentOutput, SdkError<CreateIAMPolicyAssignmentError>>;
        async fn create_ingestion(&self, builder: CreateIngestionInputBuilder) -> Result<CreateIngestionOutput, SdkError<CreateIngestionError>>;
        async fn create_namespace(&self, builder: CreateNamespaceInputBuilder) -> Result<CreateNamespaceOutput, SdkError<CreateNamespaceError>>;
        async fn create_refresh_schedule(&self, builder: CreateRefreshScheduleInputBuilder) -> Result<CreateRefreshScheduleOutput, SdkError<CreateRefreshScheduleError>>;
        async fn create_role_membership(&self, builder: CreateRoleMembershipInputBuilder) -> Result<CreateRoleMembershipOutput, SdkError<CreateRoleMembershipError>>;
        async fn create_template(&self, builder: CreateTemplateInputBuilder) -> Result<CreateTemplateOutput, SdkError<CreateTemplateError>>;
        async fn create_template_alias(&self, builder: CreateTemplateAliasInputBuilder) -> Result<CreateTemplateAliasOutput, SdkError<CreateTemplateAliasError>>;
        async fn create_theme(&self, builder: CreateThemeInputBuilder) -> Result<CreateThemeOutput, SdkError<CreateThemeError>>;
        async fn create_theme_alias(&self, builder: CreateThemeAliasInputBuilder) -> Result<CreateThemeAliasOutput, SdkError<CreateThemeAliasError>>;
        async fn create_topic(&self, builder: CreateTopicInputBuilder) -> Result<CreateTopicOutput, SdkError<CreateTopicError>>;
        async fn create_topic_refresh_schedule(&self, builder: CreateTopicRefreshScheduleInputBuilder) -> Result<CreateTopicRefreshScheduleOutput, SdkError<CreateTopicRefreshScheduleError>>;
        async fn create_vpc_connection(&self, builder: CreateVpcConnectionInputBuilder) -> Result<CreateVpcConnectionOutput, SdkError<CreateVPCConnectionError>>;
        async fn delete_account_customization(&self, builder: DeleteAccountCustomizationInputBuilder) -> Result<DeleteAccountCustomizationOutput, SdkError<DeleteAccountCustomizationError>>;
        async fn delete_account_subscription(&self, builder: DeleteAccountSubscriptionInputBuilder) -> Result<DeleteAccountSubscriptionOutput, SdkError<DeleteAccountSubscriptionError>>;
        async fn delete_analysis(&self, builder: DeleteAnalysisInputBuilder) -> Result<DeleteAnalysisOutput, SdkError<DeleteAnalysisError>>;
        async fn delete_dashboard(&self, builder: DeleteDashboardInputBuilder) -> Result<DeleteDashboardOutput, SdkError<DeleteDashboardError>>;
        async fn delete_data_set(&self, builder: DeleteDataSetInputBuilder) -> Result<DeleteDataSetOutput, SdkError<DeleteDataSetError>>;
        async fn delete_data_set_refresh_properties(&self, builder: DeleteDataSetRefreshPropertiesInputBuilder) -> Result<DeleteDataSetRefreshPropertiesOutput, SdkError<DeleteDataSetRefreshPropertiesError>>;
        async fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>;
        async fn delete_folder(&self, builder: DeleteFolderInputBuilder) -> Result<DeleteFolderOutput, SdkError<DeleteFolderError>>;
        async fn delete_folder_membership(&self, builder: DeleteFolderMembershipInputBuilder) -> Result<DeleteFolderMembershipOutput, SdkError<DeleteFolderMembershipError>>;
        async fn delete_group(&self, builder: DeleteGroupInputBuilder) -> Result<DeleteGroupOutput, SdkError<DeleteGroupError>>;
        async fn delete_group_membership(&self, builder: DeleteGroupMembershipInputBuilder) -> Result<DeleteGroupMembershipOutput, SdkError<DeleteGroupMembershipError>>;
        async fn delete_iam_policy_assignment(&self, builder: DeleteIamPolicyAssignmentInputBuilder) -> Result<DeleteIamPolicyAssignmentOutput, SdkError<DeleteIAMPolicyAssignmentError>>;
        async fn delete_identity_propagation_config(&self, builder: DeleteIdentityPropagationConfigInputBuilder) -> Result<DeleteIdentityPropagationConfigOutput, SdkError<DeleteIdentityPropagationConfigError>>;
        async fn delete_namespace(&self, builder: DeleteNamespaceInputBuilder) -> Result<DeleteNamespaceOutput, SdkError<DeleteNamespaceError>>;
        async fn delete_refresh_schedule(&self, builder: DeleteRefreshScheduleInputBuilder) -> Result<DeleteRefreshScheduleOutput, SdkError<DeleteRefreshScheduleError>>;
        async fn delete_role_custom_permission(&self, builder: DeleteRoleCustomPermissionInputBuilder) -> Result<DeleteRoleCustomPermissionOutput, SdkError<DeleteRoleCustomPermissionError>>;
        async fn delete_role_membership(&self, builder: DeleteRoleMembershipInputBuilder) -> Result<DeleteRoleMembershipOutput, SdkError<DeleteRoleMembershipError>>;
        async fn delete_template(&self, builder: DeleteTemplateInputBuilder) -> Result<DeleteTemplateOutput, SdkError<DeleteTemplateError>>;
        async fn delete_template_alias(&self, builder: DeleteTemplateAliasInputBuilder) -> Result<DeleteTemplateAliasOutput, SdkError<DeleteTemplateAliasError>>;
        async fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> Result<DeleteThemeOutput, SdkError<DeleteThemeError>>;
        async fn delete_theme_alias(&self, builder: DeleteThemeAliasInputBuilder) -> Result<DeleteThemeAliasOutput, SdkError<DeleteThemeAliasError>>;
        async fn delete_topic(&self, builder: DeleteTopicInputBuilder) -> Result<DeleteTopicOutput, SdkError<DeleteTopicError>>;
        async fn delete_topic_refresh_schedule(&self, builder: DeleteTopicRefreshScheduleInputBuilder) -> Result<DeleteTopicRefreshScheduleOutput, SdkError<DeleteTopicRefreshScheduleError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn delete_user_by_principal_id(&self, builder: DeleteUserByPrincipalIdInputBuilder) -> Result<DeleteUserByPrincipalIdOutput, SdkError<DeleteUserByPrincipalIdError>>;
        async fn delete_vpc_connection(&self, builder: DeleteVpcConnectionInputBuilder) -> Result<DeleteVpcConnectionOutput, SdkError<DeleteVPCConnectionError>>;
        async fn describe_account_customization(&self, builder: DescribeAccountCustomizationInputBuilder) -> Result<DescribeAccountCustomizationOutput, SdkError<DescribeAccountCustomizationError>>;
        async fn describe_account_settings(&self, builder: DescribeAccountSettingsInputBuilder) -> Result<DescribeAccountSettingsOutput, SdkError<DescribeAccountSettingsError>>;
        async fn describe_account_subscription(&self, builder: DescribeAccountSubscriptionInputBuilder) -> Result<DescribeAccountSubscriptionOutput, SdkError<DescribeAccountSubscriptionError>>;
        async fn describe_analysis(&self, builder: DescribeAnalysisInputBuilder) -> Result<DescribeAnalysisOutput, SdkError<DescribeAnalysisError>>;
        async fn describe_analysis_definition(&self, builder: DescribeAnalysisDefinitionInputBuilder) -> Result<DescribeAnalysisDefinitionOutput, SdkError<DescribeAnalysisDefinitionError>>;
        async fn describe_analysis_permissions(&self, builder: DescribeAnalysisPermissionsInputBuilder) -> Result<DescribeAnalysisPermissionsOutput, SdkError<DescribeAnalysisPermissionsError>>;
        async fn describe_asset_bundle_export_job(&self, builder: DescribeAssetBundleExportJobInputBuilder) -> Result<DescribeAssetBundleExportJobOutput, SdkError<DescribeAssetBundleExportJobError>>;
        async fn describe_asset_bundle_import_job(&self, builder: DescribeAssetBundleImportJobInputBuilder) -> Result<DescribeAssetBundleImportJobOutput, SdkError<DescribeAssetBundleImportJobError>>;
        async fn describe_dashboard(&self, builder: DescribeDashboardInputBuilder) -> Result<DescribeDashboardOutput, SdkError<DescribeDashboardError>>;
        async fn describe_dashboard_definition(&self, builder: DescribeDashboardDefinitionInputBuilder) -> Result<DescribeDashboardDefinitionOutput, SdkError<DescribeDashboardDefinitionError>>;
        async fn describe_dashboard_permissions(&self, builder: DescribeDashboardPermissionsInputBuilder) -> Result<DescribeDashboardPermissionsOutput, SdkError<DescribeDashboardPermissionsError>>;
        async fn describe_dashboard_snapshot_job(&self, builder: DescribeDashboardSnapshotJobInputBuilder) -> Result<DescribeDashboardSnapshotJobOutput, SdkError<DescribeDashboardSnapshotJobError>>;
        async fn describe_dashboard_snapshot_job_result(&self, builder: DescribeDashboardSnapshotJobResultInputBuilder) -> Result<DescribeDashboardSnapshotJobResultOutput, SdkError<DescribeDashboardSnapshotJobResultError>>;
        async fn describe_data_set(&self, builder: DescribeDataSetInputBuilder) -> Result<DescribeDataSetOutput, SdkError<DescribeDataSetError>>;
        async fn describe_data_set_permissions(&self, builder: DescribeDataSetPermissionsInputBuilder) -> Result<DescribeDataSetPermissionsOutput, SdkError<DescribeDataSetPermissionsError>>;
        async fn describe_data_set_refresh_properties(&self, builder: DescribeDataSetRefreshPropertiesInputBuilder) -> Result<DescribeDataSetRefreshPropertiesOutput, SdkError<DescribeDataSetRefreshPropertiesError>>;
        async fn describe_data_source(&self, builder: DescribeDataSourceInputBuilder) -> Result<DescribeDataSourceOutput, SdkError<DescribeDataSourceError>>;
        async fn describe_data_source_permissions(&self, builder: DescribeDataSourcePermissionsInputBuilder) -> Result<DescribeDataSourcePermissionsOutput, SdkError<DescribeDataSourcePermissionsError>>;
        async fn describe_folder(&self, builder: DescribeFolderInputBuilder) -> Result<DescribeFolderOutput, SdkError<DescribeFolderError>>;
        async fn describe_folder_permissions(&self, builder: DescribeFolderPermissionsInputBuilder) -> Result<DescribeFolderPermissionsOutput, SdkError<DescribeFolderPermissionsError>>;
        async fn describe_folder_resolved_permissions(&self, builder: DescribeFolderResolvedPermissionsInputBuilder) -> Result<DescribeFolderResolvedPermissionsOutput, SdkError<DescribeFolderResolvedPermissionsError>>;
        async fn describe_group(&self, builder: DescribeGroupInputBuilder) -> Result<DescribeGroupOutput, SdkError<DescribeGroupError>>;
        async fn describe_group_membership(&self, builder: DescribeGroupMembershipInputBuilder) -> Result<DescribeGroupMembershipOutput, SdkError<DescribeGroupMembershipError>>;
        async fn describe_iam_policy_assignment(&self, builder: DescribeIamPolicyAssignmentInputBuilder) -> Result<DescribeIamPolicyAssignmentOutput, SdkError<DescribeIAMPolicyAssignmentError>>;
        async fn describe_ingestion(&self, builder: DescribeIngestionInputBuilder) -> Result<DescribeIngestionOutput, SdkError<DescribeIngestionError>>;
        async fn describe_ip_restriction(&self, builder: DescribeIpRestrictionInputBuilder) -> Result<DescribeIpRestrictionOutput, SdkError<DescribeIpRestrictionError>>;
        async fn describe_key_registration(&self, builder: DescribeKeyRegistrationInputBuilder) -> Result<DescribeKeyRegistrationOutput, SdkError<DescribeKeyRegistrationError>>;
        async fn describe_namespace(&self, builder: DescribeNamespaceInputBuilder) -> Result<DescribeNamespaceOutput, SdkError<DescribeNamespaceError>>;
        async fn describe_refresh_schedule(&self, builder: DescribeRefreshScheduleInputBuilder) -> Result<DescribeRefreshScheduleOutput, SdkError<DescribeRefreshScheduleError>>;
        async fn describe_role_custom_permission(&self, builder: DescribeRoleCustomPermissionInputBuilder) -> Result<DescribeRoleCustomPermissionOutput, SdkError<DescribeRoleCustomPermissionError>>;
        async fn describe_template(&self, builder: DescribeTemplateInputBuilder) -> Result<DescribeTemplateOutput, SdkError<DescribeTemplateError>>;
        async fn describe_template_alias(&self, builder: DescribeTemplateAliasInputBuilder) -> Result<DescribeTemplateAliasOutput, SdkError<DescribeTemplateAliasError>>;
        async fn describe_template_definition(&self, builder: DescribeTemplateDefinitionInputBuilder) -> Result<DescribeTemplateDefinitionOutput, SdkError<DescribeTemplateDefinitionError>>;
        async fn describe_template_permissions(&self, builder: DescribeTemplatePermissionsInputBuilder) -> Result<DescribeTemplatePermissionsOutput, SdkError<DescribeTemplatePermissionsError>>;
        async fn describe_theme(&self, builder: DescribeThemeInputBuilder) -> Result<DescribeThemeOutput, SdkError<DescribeThemeError>>;
        async fn describe_theme_alias(&self, builder: DescribeThemeAliasInputBuilder) -> Result<DescribeThemeAliasOutput, SdkError<DescribeThemeAliasError>>;
        async fn describe_theme_permissions(&self, builder: DescribeThemePermissionsInputBuilder) -> Result<DescribeThemePermissionsOutput, SdkError<DescribeThemePermissionsError>>;
        async fn describe_topic(&self, builder: DescribeTopicInputBuilder) -> Result<DescribeTopicOutput, SdkError<DescribeTopicError>>;
        async fn describe_topic_permissions(&self, builder: DescribeTopicPermissionsInputBuilder) -> Result<DescribeTopicPermissionsOutput, SdkError<DescribeTopicPermissionsError>>;
        async fn describe_topic_refresh(&self, builder: DescribeTopicRefreshInputBuilder) -> Result<DescribeTopicRefreshOutput, SdkError<DescribeTopicRefreshError>>;
        async fn describe_topic_refresh_schedule(&self, builder: DescribeTopicRefreshScheduleInputBuilder) -> Result<DescribeTopicRefreshScheduleOutput, SdkError<DescribeTopicRefreshScheduleError>>;
        async fn describe_user(&self, builder: DescribeUserInputBuilder) -> Result<DescribeUserOutput, SdkError<DescribeUserError>>;
        async fn describe_vpc_connection(&self, builder: DescribeVpcConnectionInputBuilder) -> Result<DescribeVpcConnectionOutput, SdkError<DescribeVPCConnectionError>>;
        async fn generate_embed_url_for_anonymous_user(&self, builder: GenerateEmbedUrlForAnonymousUserInputBuilder) -> Result<GenerateEmbedUrlForAnonymousUserOutput, SdkError<GenerateEmbedUrlForAnonymousUserError>>;
        async fn generate_embed_url_for_registered_user(&self, builder: GenerateEmbedUrlForRegisteredUserInputBuilder) -> Result<GenerateEmbedUrlForRegisteredUserOutput, SdkError<GenerateEmbedUrlForRegisteredUserError>>;
        async fn get_dashboard_embed_url(&self, builder: GetDashboardEmbedUrlInputBuilder) -> Result<GetDashboardEmbedUrlOutput, SdkError<GetDashboardEmbedUrlError>>;
        async fn get_session_embed_url(&self, builder: GetSessionEmbedUrlInputBuilder) -> Result<GetSessionEmbedUrlOutput, SdkError<GetSessionEmbedUrlError>>;
        async fn list_analyses(&self, builder: ListAnalysesInputBuilder) -> Result<ListAnalysesOutput, SdkError<ListAnalysesError>>;
        async fn list_asset_bundle_export_jobs(&self, builder: ListAssetBundleExportJobsInputBuilder) -> Result<ListAssetBundleExportJobsOutput, SdkError<ListAssetBundleExportJobsError>>;
        async fn list_asset_bundle_import_jobs(&self, builder: ListAssetBundleImportJobsInputBuilder) -> Result<ListAssetBundleImportJobsOutput, SdkError<ListAssetBundleImportJobsError>>;
        async fn list_dashboard_versions(&self, builder: ListDashboardVersionsInputBuilder) -> Result<ListDashboardVersionsOutput, SdkError<ListDashboardVersionsError>>;
        async fn list_dashboards(&self, builder: ListDashboardsInputBuilder) -> Result<ListDashboardsOutput, SdkError<ListDashboardsError>>;
        async fn list_data_sets(&self, builder: ListDataSetsInputBuilder) -> Result<ListDataSetsOutput, SdkError<ListDataSetsError>>;
        async fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>;
        async fn list_folder_members(&self, builder: ListFolderMembersInputBuilder) -> Result<ListFolderMembersOutput, SdkError<ListFolderMembersError>>;
        async fn list_folders(&self, builder: ListFoldersInputBuilder) -> Result<ListFoldersOutput, SdkError<ListFoldersError>>;
        async fn list_group_memberships(&self, builder: ListGroupMembershipsInputBuilder) -> Result<ListGroupMembershipsOutput, SdkError<ListGroupMembershipsError>>;
        async fn list_groups(&self, builder: ListGroupsInputBuilder) -> Result<ListGroupsOutput, SdkError<ListGroupsError>>;
        async fn list_iam_policy_assignments(&self, builder: ListIamPolicyAssignmentsInputBuilder) -> Result<ListIamPolicyAssignmentsOutput, SdkError<ListIAMPolicyAssignmentsError>>;
        async fn list_iam_policy_assignments_for_user(&self, builder: ListIamPolicyAssignmentsForUserInputBuilder) -> Result<ListIamPolicyAssignmentsForUserOutput, SdkError<ListIAMPolicyAssignmentsForUserError>>;
        async fn list_identity_propagation_configs(&self, builder: ListIdentityPropagationConfigsInputBuilder) -> Result<ListIdentityPropagationConfigsOutput, SdkError<ListIdentityPropagationConfigsError>>;
        async fn list_ingestions(&self, builder: ListIngestionsInputBuilder) -> Result<ListIngestionsOutput, SdkError<ListIngestionsError>>;
        async fn list_namespaces(&self, builder: ListNamespacesInputBuilder) -> Result<ListNamespacesOutput, SdkError<ListNamespacesError>>;
        async fn list_refresh_schedules(&self, builder: ListRefreshSchedulesInputBuilder) -> Result<ListRefreshSchedulesOutput, SdkError<ListRefreshSchedulesError>>;
        async fn list_role_memberships(&self, builder: ListRoleMembershipsInputBuilder) -> Result<ListRoleMembershipsOutput, SdkError<ListRoleMembershipsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_template_aliases(&self, builder: ListTemplateAliasesInputBuilder) -> Result<ListTemplateAliasesOutput, SdkError<ListTemplateAliasesError>>;
        async fn list_template_versions(&self, builder: ListTemplateVersionsInputBuilder) -> Result<ListTemplateVersionsOutput, SdkError<ListTemplateVersionsError>>;
        async fn list_templates(&self, builder: ListTemplatesInputBuilder) -> Result<ListTemplatesOutput, SdkError<ListTemplatesError>>;
        async fn list_theme_aliases(&self, builder: ListThemeAliasesInputBuilder) -> Result<ListThemeAliasesOutput, SdkError<ListThemeAliasesError>>;
        async fn list_theme_versions(&self, builder: ListThemeVersionsInputBuilder) -> Result<ListThemeVersionsOutput, SdkError<ListThemeVersionsError>>;
        async fn list_themes(&self, builder: ListThemesInputBuilder) -> Result<ListThemesOutput, SdkError<ListThemesError>>;
        async fn list_topic_refresh_schedules(&self, builder: ListTopicRefreshSchedulesInputBuilder) -> Result<ListTopicRefreshSchedulesOutput, SdkError<ListTopicRefreshSchedulesError>>;
        async fn list_topic_reviewed_answers(&self, builder: ListTopicReviewedAnswersInputBuilder) -> Result<ListTopicReviewedAnswersOutput, SdkError<ListTopicReviewedAnswersError>>;
        async fn list_topics(&self, builder: ListTopicsInputBuilder) -> Result<ListTopicsOutput, SdkError<ListTopicsError>>;
        async fn list_user_groups(&self, builder: ListUserGroupsInputBuilder) -> Result<ListUserGroupsOutput, SdkError<ListUserGroupsError>>;
        async fn list_users(&self, builder: ListUsersInputBuilder) -> Result<ListUsersOutput, SdkError<ListUsersError>>;
        async fn list_vpc_connections(&self, builder: ListVpcConnectionsInputBuilder) -> Result<ListVpcConnectionsOutput, SdkError<ListVPCConnectionsError>>;
        async fn put_data_set_refresh_properties(&self, builder: PutDataSetRefreshPropertiesInputBuilder) -> Result<PutDataSetRefreshPropertiesOutput, SdkError<PutDataSetRefreshPropertiesError>>;
        async fn register_user(&self, builder: RegisterUserInputBuilder) -> Result<RegisterUserOutput, SdkError<RegisterUserError>>;
        async fn restore_analysis(&self, builder: RestoreAnalysisInputBuilder) -> Result<RestoreAnalysisOutput, SdkError<RestoreAnalysisError>>;
        async fn search_analyses(&self, builder: SearchAnalysesInputBuilder) -> Result<SearchAnalysesOutput, SdkError<SearchAnalysesError>>;
        async fn search_dashboards(&self, builder: SearchDashboardsInputBuilder) -> Result<SearchDashboardsOutput, SdkError<SearchDashboardsError>>;
        async fn search_data_sets(&self, builder: SearchDataSetsInputBuilder) -> Result<SearchDataSetsOutput, SdkError<SearchDataSetsError>>;
        async fn search_data_sources(&self, builder: SearchDataSourcesInputBuilder) -> Result<SearchDataSourcesOutput, SdkError<SearchDataSourcesError>>;
        async fn search_folders(&self, builder: SearchFoldersInputBuilder) -> Result<SearchFoldersOutput, SdkError<SearchFoldersError>>;
        async fn search_groups(&self, builder: SearchGroupsInputBuilder) -> Result<SearchGroupsOutput, SdkError<SearchGroupsError>>;
        async fn start_asset_bundle_export_job(&self, builder: StartAssetBundleExportJobInputBuilder) -> Result<StartAssetBundleExportJobOutput, SdkError<StartAssetBundleExportJobError>>;
        async fn start_asset_bundle_import_job(&self, builder: StartAssetBundleImportJobInputBuilder) -> Result<StartAssetBundleImportJobOutput, SdkError<StartAssetBundleImportJobError>>;
        async fn start_dashboard_snapshot_job(&self, builder: StartDashboardSnapshotJobInputBuilder) -> Result<StartDashboardSnapshotJobOutput, SdkError<StartDashboardSnapshotJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_account_customization(&self, builder: UpdateAccountCustomizationInputBuilder) -> Result<UpdateAccountCustomizationOutput, SdkError<UpdateAccountCustomizationError>>;
        async fn update_account_settings(&self, builder: UpdateAccountSettingsInputBuilder) -> Result<UpdateAccountSettingsOutput, SdkError<UpdateAccountSettingsError>>;
        async fn update_analysis(&self, builder: UpdateAnalysisInputBuilder) -> Result<UpdateAnalysisOutput, SdkError<UpdateAnalysisError>>;
        async fn update_analysis_permissions(&self, builder: UpdateAnalysisPermissionsInputBuilder) -> Result<UpdateAnalysisPermissionsOutput, SdkError<UpdateAnalysisPermissionsError>>;
        async fn update_dashboard(&self, builder: UpdateDashboardInputBuilder) -> Result<UpdateDashboardOutput, SdkError<UpdateDashboardError>>;
        async fn update_dashboard_links(&self, builder: UpdateDashboardLinksInputBuilder) -> Result<UpdateDashboardLinksOutput, SdkError<UpdateDashboardLinksError>>;
        async fn update_dashboard_permissions(&self, builder: UpdateDashboardPermissionsInputBuilder) -> Result<UpdateDashboardPermissionsOutput, SdkError<UpdateDashboardPermissionsError>>;
        async fn update_dashboard_published_version(&self, builder: UpdateDashboardPublishedVersionInputBuilder) -> Result<UpdateDashboardPublishedVersionOutput, SdkError<UpdateDashboardPublishedVersionError>>;
        async fn update_data_set(&self, builder: UpdateDataSetInputBuilder) -> Result<UpdateDataSetOutput, SdkError<UpdateDataSetError>>;
        async fn update_data_set_permissions(&self, builder: UpdateDataSetPermissionsInputBuilder) -> Result<UpdateDataSetPermissionsOutput, SdkError<UpdateDataSetPermissionsError>>;
        async fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>;
        async fn update_data_source_permissions(&self, builder: UpdateDataSourcePermissionsInputBuilder) -> Result<UpdateDataSourcePermissionsOutput, SdkError<UpdateDataSourcePermissionsError>>;
        async fn update_folder(&self, builder: UpdateFolderInputBuilder) -> Result<UpdateFolderOutput, SdkError<UpdateFolderError>>;
        async fn update_folder_permissions(&self, builder: UpdateFolderPermissionsInputBuilder) -> Result<UpdateFolderPermissionsOutput, SdkError<UpdateFolderPermissionsError>>;
        async fn update_group(&self, builder: UpdateGroupInputBuilder) -> Result<UpdateGroupOutput, SdkError<UpdateGroupError>>;
        async fn update_iam_policy_assignment(&self, builder: UpdateIamPolicyAssignmentInputBuilder) -> Result<UpdateIamPolicyAssignmentOutput, SdkError<UpdateIAMPolicyAssignmentError>>;
        async fn update_identity_propagation_config(&self, builder: UpdateIdentityPropagationConfigInputBuilder) -> Result<UpdateIdentityPropagationConfigOutput, SdkError<UpdateIdentityPropagationConfigError>>;
        async fn update_ip_restriction(&self, builder: UpdateIpRestrictionInputBuilder) -> Result<UpdateIpRestrictionOutput, SdkError<UpdateIpRestrictionError>>;
        async fn update_key_registration(&self, builder: UpdateKeyRegistrationInputBuilder) -> Result<UpdateKeyRegistrationOutput, SdkError<UpdateKeyRegistrationError>>;
        async fn update_public_sharing_settings(&self, builder: UpdatePublicSharingSettingsInputBuilder) -> Result<UpdatePublicSharingSettingsOutput, SdkError<UpdatePublicSharingSettingsError>>;
        async fn update_refresh_schedule(&self, builder: UpdateRefreshScheduleInputBuilder) -> Result<UpdateRefreshScheduleOutput, SdkError<UpdateRefreshScheduleError>>;
        async fn update_role_custom_permission(&self, builder: UpdateRoleCustomPermissionInputBuilder) -> Result<UpdateRoleCustomPermissionOutput, SdkError<UpdateRoleCustomPermissionError>>;
        async fn update_spice_capacity_configuration(&self, builder: UpdateSpiceCapacityConfigurationInputBuilder) -> Result<UpdateSpiceCapacityConfigurationOutput, SdkError<UpdateSPICECapacityConfigurationError>>;
        async fn update_template(&self, builder: UpdateTemplateInputBuilder) -> Result<UpdateTemplateOutput, SdkError<UpdateTemplateError>>;
        async fn update_template_alias(&self, builder: UpdateTemplateAliasInputBuilder) -> Result<UpdateTemplateAliasOutput, SdkError<UpdateTemplateAliasError>>;
        async fn update_template_permissions(&self, builder: UpdateTemplatePermissionsInputBuilder) -> Result<UpdateTemplatePermissionsOutput, SdkError<UpdateTemplatePermissionsError>>;
        async fn update_theme(&self, builder: UpdateThemeInputBuilder) -> Result<UpdateThemeOutput, SdkError<UpdateThemeError>>;
        async fn update_theme_alias(&self, builder: UpdateThemeAliasInputBuilder) -> Result<UpdateThemeAliasOutput, SdkError<UpdateThemeAliasError>>;
        async fn update_theme_permissions(&self, builder: UpdateThemePermissionsInputBuilder) -> Result<UpdateThemePermissionsOutput, SdkError<UpdateThemePermissionsError>>;
        async fn update_topic(&self, builder: UpdateTopicInputBuilder) -> Result<UpdateTopicOutput, SdkError<UpdateTopicError>>;
        async fn update_topic_permissions(&self, builder: UpdateTopicPermissionsInputBuilder) -> Result<UpdateTopicPermissionsOutput, SdkError<UpdateTopicPermissionsError>>;
        async fn update_topic_refresh_schedule(&self, builder: UpdateTopicRefreshScheduleInputBuilder) -> Result<UpdateTopicRefreshScheduleOutput, SdkError<UpdateTopicRefreshScheduleError>>;
        async fn update_user(&self, builder: UpdateUserInputBuilder) -> Result<UpdateUserOutput, SdkError<UpdateUserError>>;
        async fn update_vpc_connection(&self, builder: UpdateVpcConnectionInputBuilder) -> Result<UpdateVpcConnectionOutput, SdkError<UpdateVPCConnectionError>>;
    }
}
