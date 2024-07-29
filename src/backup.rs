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
use aws_sdk_backup::operation::cancel_legal_hold::{builders::*, *};
use aws_sdk_backup::operation::create_backup_plan::{builders::*, *};
use aws_sdk_backup::operation::create_backup_selection::{builders::*, *};
use aws_sdk_backup::operation::create_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::create_framework::{builders::*, *};
use aws_sdk_backup::operation::create_legal_hold::{builders::*, *};
use aws_sdk_backup::operation::create_logically_air_gapped_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::create_report_plan::{builders::*, *};
use aws_sdk_backup::operation::create_restore_testing_plan::{builders::*, *};
use aws_sdk_backup::operation::create_restore_testing_selection::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_plan::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_selection::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_vault_access_policy::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_vault_lock_configuration::{builders::*, *};
use aws_sdk_backup::operation::delete_backup_vault_notifications::{builders::*, *};
use aws_sdk_backup::operation::delete_framework::{builders::*, *};
use aws_sdk_backup::operation::delete_recovery_point::{builders::*, *};
use aws_sdk_backup::operation::delete_report_plan::{builders::*, *};
use aws_sdk_backup::operation::delete_restore_testing_plan::{builders::*, *};
use aws_sdk_backup::operation::delete_restore_testing_selection::{builders::*, *};
use aws_sdk_backup::operation::describe_backup_job::{builders::*, *};
use aws_sdk_backup::operation::describe_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::describe_copy_job::{builders::*, *};
use aws_sdk_backup::operation::describe_framework::{builders::*, *};
use aws_sdk_backup::operation::describe_global_settings::{builders::*, *};
use aws_sdk_backup::operation::describe_protected_resource::{builders::*, *};
use aws_sdk_backup::operation::describe_recovery_point::{builders::*, *};
use aws_sdk_backup::operation::describe_region_settings::{builders::*, *};
use aws_sdk_backup::operation::describe_report_job::{builders::*, *};
use aws_sdk_backup::operation::describe_report_plan::{builders::*, *};
use aws_sdk_backup::operation::describe_restore_job::{builders::*, *};
use aws_sdk_backup::operation::disassociate_recovery_point::{builders::*, *};
use aws_sdk_backup::operation::disassociate_recovery_point_from_parent::{builders::*, *};
use aws_sdk_backup::operation::export_backup_plan_template::{builders::*, *};
use aws_sdk_backup::operation::get_backup_plan::{builders::*, *};
use aws_sdk_backup::operation::get_backup_plan_from_json::{builders::*, *};
use aws_sdk_backup::operation::get_backup_plan_from_template::{builders::*, *};
use aws_sdk_backup::operation::get_backup_selection::{builders::*, *};
use aws_sdk_backup::operation::get_backup_vault_access_policy::{builders::*, *};
use aws_sdk_backup::operation::get_backup_vault_notifications::{builders::*, *};
use aws_sdk_backup::operation::get_legal_hold::{builders::*, *};
use aws_sdk_backup::operation::get_recovery_point_restore_metadata::{builders::*, *};
use aws_sdk_backup::operation::get_restore_job_metadata::{builders::*, *};
use aws_sdk_backup::operation::get_restore_testing_inferred_metadata::{builders::*, *};
use aws_sdk_backup::operation::get_restore_testing_plan::{builders::*, *};
use aws_sdk_backup::operation::get_restore_testing_selection::{builders::*, *};
use aws_sdk_backup::operation::get_supported_resource_types::{builders::*, *};
use aws_sdk_backup::operation::list_backup_job_summaries::{builders::*, *};
use aws_sdk_backup::operation::list_backup_jobs::{builders::*, *};
use aws_sdk_backup::operation::list_backup_plan_templates::{builders::*, *};
use aws_sdk_backup::operation::list_backup_plan_versions::{builders::*, *};
use aws_sdk_backup::operation::list_backup_plans::{builders::*, *};
use aws_sdk_backup::operation::list_backup_selections::{builders::*, *};
use aws_sdk_backup::operation::list_backup_vaults::{builders::*, *};
use aws_sdk_backup::operation::list_copy_job_summaries::{builders::*, *};
use aws_sdk_backup::operation::list_copy_jobs::{builders::*, *};
use aws_sdk_backup::operation::list_frameworks::{builders::*, *};
use aws_sdk_backup::operation::list_legal_holds::{builders::*, *};
use aws_sdk_backup::operation::list_protected_resources::{builders::*, *};
use aws_sdk_backup::operation::list_protected_resources_by_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::list_recovery_points_by_backup_vault::{builders::*, *};
use aws_sdk_backup::operation::list_recovery_points_by_legal_hold::{builders::*, *};
use aws_sdk_backup::operation::list_recovery_points_by_resource::{builders::*, *};
use aws_sdk_backup::operation::list_report_jobs::{builders::*, *};
use aws_sdk_backup::operation::list_report_plans::{builders::*, *};
use aws_sdk_backup::operation::list_restore_job_summaries::{builders::*, *};
use aws_sdk_backup::operation::list_restore_jobs::{builders::*, *};
use aws_sdk_backup::operation::list_restore_jobs_by_protected_resource::{builders::*, *};
use aws_sdk_backup::operation::list_restore_testing_plans::{builders::*, *};
use aws_sdk_backup::operation::list_restore_testing_selections::{builders::*, *};
use aws_sdk_backup::operation::list_tags::{builders::*, *};
use aws_sdk_backup::operation::put_backup_vault_access_policy::{builders::*, *};
use aws_sdk_backup::operation::put_backup_vault_lock_configuration::{builders::*, *};
use aws_sdk_backup::operation::put_backup_vault_notifications::{builders::*, *};
use aws_sdk_backup::operation::put_restore_validation_result::{builders::*, *};
use aws_sdk_backup::operation::start_backup_job::{builders::*, *};
use aws_sdk_backup::operation::start_copy_job::{builders::*, *};
use aws_sdk_backup::operation::start_report_job::{builders::*, *};
use aws_sdk_backup::operation::start_restore_job::{builders::*, *};
use aws_sdk_backup::operation::stop_backup_job::{builders::*, *};
use aws_sdk_backup::operation::tag_resource::{builders::*, *};
use aws_sdk_backup::operation::untag_resource::{builders::*, *};
use aws_sdk_backup::operation::update_backup_plan::{builders::*, *};
use aws_sdk_backup::operation::update_framework::{builders::*, *};
use aws_sdk_backup::operation::update_global_settings::{builders::*, *};
use aws_sdk_backup::operation::update_recovery_point_lifecycle::{builders::*, *};
use aws_sdk_backup::operation::update_region_settings::{builders::*, *};
use aws_sdk_backup::operation::update_report_plan::{builders::*, *};
use aws_sdk_backup::operation::update_restore_testing_plan::{builders::*, *};
use aws_sdk_backup::operation::update_restore_testing_selection::{builders::*, *};
use aws_sdk_backup::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_backup::Client;
use std::ops::Deref;

pub use aws_sdk_backup::*;

pub struct BackupClientImpl(Client);
impl BackupClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait BackupClient {
    fn cancel_legal_hold(&self, builder: CancelLegalHoldInputBuilder) -> impl Future<Output = Result<CancelLegalHoldOutput, SdkError<CancelLegalHoldError>>>;
    fn create_backup_plan(&self, builder: CreateBackupPlanInputBuilder) -> impl Future<Output = Result<CreateBackupPlanOutput, SdkError<CreateBackupPlanError>>>;
    fn create_backup_selection(&self, builder: CreateBackupSelectionInputBuilder) -> impl Future<Output = Result<CreateBackupSelectionOutput, SdkError<CreateBackupSelectionError>>>;
    fn create_backup_vault(&self, builder: CreateBackupVaultInputBuilder) -> impl Future<Output = Result<CreateBackupVaultOutput, SdkError<CreateBackupVaultError>>>;
    fn create_framework(&self, builder: CreateFrameworkInputBuilder) -> impl Future<Output = Result<CreateFrameworkOutput, SdkError<CreateFrameworkError>>>;
    fn create_legal_hold(&self, builder: CreateLegalHoldInputBuilder) -> impl Future<Output = Result<CreateLegalHoldOutput, SdkError<CreateLegalHoldError>>>;
    fn create_logically_air_gapped_backup_vault(&self, builder: CreateLogicallyAirGappedBackupVaultInputBuilder) -> impl Future<Output = Result<CreateLogicallyAirGappedBackupVaultOutput, SdkError<CreateLogicallyAirGappedBackupVaultError>>>;
    fn create_report_plan(&self, builder: CreateReportPlanInputBuilder) -> impl Future<Output = Result<CreateReportPlanOutput, SdkError<CreateReportPlanError>>>;
    fn create_restore_testing_plan(&self, builder: CreateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingPlanOutput, SdkError<CreateRestoreTestingPlanError>>>;
    fn create_restore_testing_selection(&self, builder: CreateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingSelectionOutput, SdkError<CreateRestoreTestingSelectionError>>>;
    fn delete_backup_plan(&self, builder: DeleteBackupPlanInputBuilder) -> impl Future<Output = Result<DeleteBackupPlanOutput, SdkError<DeleteBackupPlanError>>>;
    fn delete_backup_selection(&self, builder: DeleteBackupSelectionInputBuilder) -> impl Future<Output = Result<DeleteBackupSelectionOutput, SdkError<DeleteBackupSelectionError>>>;
    fn delete_backup_vault(&self, builder: DeleteBackupVaultInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultOutput, SdkError<DeleteBackupVaultError>>>;
    fn delete_backup_vault_access_policy(&self, builder: DeleteBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultAccessPolicyOutput, SdkError<DeleteBackupVaultAccessPolicyError>>>;
    fn delete_backup_vault_lock_configuration(&self, builder: DeleteBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultLockConfigurationOutput, SdkError<DeleteBackupVaultLockConfigurationError>>>;
    fn delete_backup_vault_notifications(&self, builder: DeleteBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultNotificationsOutput, SdkError<DeleteBackupVaultNotificationsError>>>;
    fn delete_framework(&self, builder: DeleteFrameworkInputBuilder) -> impl Future<Output = Result<DeleteFrameworkOutput, SdkError<DeleteFrameworkError>>>;
    fn delete_recovery_point(&self, builder: DeleteRecoveryPointInputBuilder) -> impl Future<Output = Result<DeleteRecoveryPointOutput, SdkError<DeleteRecoveryPointError>>>;
    fn delete_report_plan(&self, builder: DeleteReportPlanInputBuilder) -> impl Future<Output = Result<DeleteReportPlanOutput, SdkError<DeleteReportPlanError>>>;
    fn delete_restore_testing_plan(&self, builder: DeleteRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingPlanOutput, SdkError<DeleteRestoreTestingPlanError>>>;
    fn delete_restore_testing_selection(&self, builder: DeleteRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingSelectionOutput, SdkError<DeleteRestoreTestingSelectionError>>>;
    fn describe_backup_job(&self, builder: DescribeBackupJobInputBuilder) -> impl Future<Output = Result<DescribeBackupJobOutput, SdkError<DescribeBackupJobError>>>;
    fn describe_backup_vault(&self, builder: DescribeBackupVaultInputBuilder) -> impl Future<Output = Result<DescribeBackupVaultOutput, SdkError<DescribeBackupVaultError>>>;
    fn describe_copy_job(&self, builder: DescribeCopyJobInputBuilder) -> impl Future<Output = Result<DescribeCopyJobOutput, SdkError<DescribeCopyJobError>>>;
    fn describe_framework(&self, builder: DescribeFrameworkInputBuilder) -> impl Future<Output = Result<DescribeFrameworkOutput, SdkError<DescribeFrameworkError>>>;
    fn describe_global_settings(&self, builder: DescribeGlobalSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalSettingsOutput, SdkError<DescribeGlobalSettingsError>>>;
    fn describe_protected_resource(&self, builder: DescribeProtectedResourceInputBuilder) -> impl Future<Output = Result<DescribeProtectedResourceOutput, SdkError<DescribeProtectedResourceError>>>;
    fn describe_recovery_point(&self, builder: DescribeRecoveryPointInputBuilder) -> impl Future<Output = Result<DescribeRecoveryPointOutput, SdkError<DescribeRecoveryPointError>>>;
    fn describe_region_settings(&self, builder: DescribeRegionSettingsInputBuilder) -> impl Future<Output = Result<DescribeRegionSettingsOutput, SdkError<DescribeRegionSettingsError>>>;
    fn describe_report_job(&self, builder: DescribeReportJobInputBuilder) -> impl Future<Output = Result<DescribeReportJobOutput, SdkError<DescribeReportJobError>>>;
    fn describe_report_plan(&self, builder: DescribeReportPlanInputBuilder) -> impl Future<Output = Result<DescribeReportPlanOutput, SdkError<DescribeReportPlanError>>>;
    fn describe_restore_job(&self, builder: DescribeRestoreJobInputBuilder) -> impl Future<Output = Result<DescribeRestoreJobOutput, SdkError<DescribeRestoreJobError>>>;
    fn disassociate_recovery_point(&self, builder: DisassociateRecoveryPointInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointOutput, SdkError<DisassociateRecoveryPointError>>>;
    fn disassociate_recovery_point_from_parent(&self, builder: DisassociateRecoveryPointFromParentInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointFromParentOutput, SdkError<DisassociateRecoveryPointFromParentError>>>;
    fn export_backup_plan_template(&self, builder: ExportBackupPlanTemplateInputBuilder) -> impl Future<Output = Result<ExportBackupPlanTemplateOutput, SdkError<ExportBackupPlanTemplateError>>>;
    fn get_backup_plan(&self, builder: GetBackupPlanInputBuilder) -> impl Future<Output = Result<GetBackupPlanOutput, SdkError<GetBackupPlanError>>>;
    fn get_backup_plan_from_json(&self, builder: GetBackupPlanFromJsonInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromJsonOutput, SdkError<GetBackupPlanFromJSONError>>>;
    fn get_backup_plan_from_template(&self, builder: GetBackupPlanFromTemplateInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromTemplateOutput, SdkError<GetBackupPlanFromTemplateError>>>;
    fn get_backup_selection(&self, builder: GetBackupSelectionInputBuilder) -> impl Future<Output = Result<GetBackupSelectionOutput, SdkError<GetBackupSelectionError>>>;
    fn get_backup_vault_access_policy(&self, builder: GetBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<GetBackupVaultAccessPolicyOutput, SdkError<GetBackupVaultAccessPolicyError>>>;
    fn get_backup_vault_notifications(&self, builder: GetBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<GetBackupVaultNotificationsOutput, SdkError<GetBackupVaultNotificationsError>>>;
    fn get_legal_hold(&self, builder: GetLegalHoldInputBuilder) -> impl Future<Output = Result<GetLegalHoldOutput, SdkError<GetLegalHoldError>>>;
    fn get_recovery_point_restore_metadata(&self, builder: GetRecoveryPointRestoreMetadataInputBuilder) -> impl Future<Output = Result<GetRecoveryPointRestoreMetadataOutput, SdkError<GetRecoveryPointRestoreMetadataError>>>;
    fn get_restore_job_metadata(&self, builder: GetRestoreJobMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreJobMetadataOutput, SdkError<GetRestoreJobMetadataError>>>;
    fn get_restore_testing_inferred_metadata(&self, builder: GetRestoreTestingInferredMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreTestingInferredMetadataOutput, SdkError<GetRestoreTestingInferredMetadataError>>>;
    fn get_restore_testing_plan(&self, builder: GetRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<GetRestoreTestingPlanOutput, SdkError<GetRestoreTestingPlanError>>>;
    fn get_restore_testing_selection(&self, builder: GetRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<GetRestoreTestingSelectionOutput, SdkError<GetRestoreTestingSelectionError>>>;
    fn get_supported_resource_types(&self, builder: GetSupportedResourceTypesInputBuilder) -> impl Future<Output = Result<GetSupportedResourceTypesOutput, SdkError<GetSupportedResourceTypesError>>>;
    fn list_backup_job_summaries(&self, builder: ListBackupJobSummariesInputBuilder) -> impl Future<Output = Result<ListBackupJobSummariesOutput, SdkError<ListBackupJobSummariesError>>>;
    fn list_backup_jobs(&self, builder: ListBackupJobsInputBuilder) -> impl Future<Output = Result<ListBackupJobsOutput, SdkError<ListBackupJobsError>>>;
    fn list_backup_plan_templates(&self, builder: ListBackupPlanTemplatesInputBuilder) -> impl Future<Output = Result<ListBackupPlanTemplatesOutput, SdkError<ListBackupPlanTemplatesError>>>;
    fn list_backup_plan_versions(&self, builder: ListBackupPlanVersionsInputBuilder) -> impl Future<Output = Result<ListBackupPlanVersionsOutput, SdkError<ListBackupPlanVersionsError>>>;
    fn list_backup_plans(&self, builder: ListBackupPlansInputBuilder) -> impl Future<Output = Result<ListBackupPlansOutput, SdkError<ListBackupPlansError>>>;
    fn list_backup_selections(&self, builder: ListBackupSelectionsInputBuilder) -> impl Future<Output = Result<ListBackupSelectionsOutput, SdkError<ListBackupSelectionsError>>>;
    fn list_backup_vaults(&self, builder: ListBackupVaultsInputBuilder) -> impl Future<Output = Result<ListBackupVaultsOutput, SdkError<ListBackupVaultsError>>>;
    fn list_copy_job_summaries(&self, builder: ListCopyJobSummariesInputBuilder) -> impl Future<Output = Result<ListCopyJobSummariesOutput, SdkError<ListCopyJobSummariesError>>>;
    fn list_copy_jobs(&self, builder: ListCopyJobsInputBuilder) -> impl Future<Output = Result<ListCopyJobsOutput, SdkError<ListCopyJobsError>>>;
    fn list_frameworks(&self, builder: ListFrameworksInputBuilder) -> impl Future<Output = Result<ListFrameworksOutput, SdkError<ListFrameworksError>>>;
    fn list_legal_holds(&self, builder: ListLegalHoldsInputBuilder) -> impl Future<Output = Result<ListLegalHoldsOutput, SdkError<ListLegalHoldsError>>>;
    fn list_protected_resources(&self, builder: ListProtectedResourcesInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesOutput, SdkError<ListProtectedResourcesError>>>;
    fn list_protected_resources_by_backup_vault(&self, builder: ListProtectedResourcesByBackupVaultInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesByBackupVaultOutput, SdkError<ListProtectedResourcesByBackupVaultError>>>;
    fn list_recovery_points_by_backup_vault(&self, builder: ListRecoveryPointsByBackupVaultInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByBackupVaultOutput, SdkError<ListRecoveryPointsByBackupVaultError>>>;
    fn list_recovery_points_by_legal_hold(&self, builder: ListRecoveryPointsByLegalHoldInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByLegalHoldOutput, SdkError<ListRecoveryPointsByLegalHoldError>>>;
    fn list_recovery_points_by_resource(&self, builder: ListRecoveryPointsByResourceInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByResourceOutput, SdkError<ListRecoveryPointsByResourceError>>>;
    fn list_report_jobs(&self, builder: ListReportJobsInputBuilder) -> impl Future<Output = Result<ListReportJobsOutput, SdkError<ListReportJobsError>>>;
    fn list_report_plans(&self, builder: ListReportPlansInputBuilder) -> impl Future<Output = Result<ListReportPlansOutput, SdkError<ListReportPlansError>>>;
    fn list_restore_job_summaries(&self, builder: ListRestoreJobSummariesInputBuilder) -> impl Future<Output = Result<ListRestoreJobSummariesOutput, SdkError<ListRestoreJobSummariesError>>>;
    fn list_restore_jobs(&self, builder: ListRestoreJobsInputBuilder) -> impl Future<Output = Result<ListRestoreJobsOutput, SdkError<ListRestoreJobsError>>>;
    fn list_restore_jobs_by_protected_resource(&self, builder: ListRestoreJobsByProtectedResourceInputBuilder) -> impl Future<Output = Result<ListRestoreJobsByProtectedResourceOutput, SdkError<ListRestoreJobsByProtectedResourceError>>>;
    fn list_restore_testing_plans(&self, builder: ListRestoreTestingPlansInputBuilder) -> impl Future<Output = Result<ListRestoreTestingPlansOutput, SdkError<ListRestoreTestingPlansError>>>;
    fn list_restore_testing_selections(&self, builder: ListRestoreTestingSelectionsInputBuilder) -> impl Future<Output = Result<ListRestoreTestingSelectionsOutput, SdkError<ListRestoreTestingSelectionsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn put_backup_vault_access_policy(&self, builder: PutBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<PutBackupVaultAccessPolicyOutput, SdkError<PutBackupVaultAccessPolicyError>>>;
    fn put_backup_vault_lock_configuration(&self, builder: PutBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<PutBackupVaultLockConfigurationOutput, SdkError<PutBackupVaultLockConfigurationError>>>;
    fn put_backup_vault_notifications(&self, builder: PutBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<PutBackupVaultNotificationsOutput, SdkError<PutBackupVaultNotificationsError>>>;
    fn put_restore_validation_result(&self, builder: PutRestoreValidationResultInputBuilder) -> impl Future<Output = Result<PutRestoreValidationResultOutput, SdkError<PutRestoreValidationResultError>>>;
    fn start_backup_job(&self, builder: StartBackupJobInputBuilder) -> impl Future<Output = Result<StartBackupJobOutput, SdkError<StartBackupJobError>>>;
    fn start_copy_job(&self, builder: StartCopyJobInputBuilder) -> impl Future<Output = Result<StartCopyJobOutput, SdkError<StartCopyJobError>>>;
    fn start_report_job(&self, builder: StartReportJobInputBuilder) -> impl Future<Output = Result<StartReportJobOutput, SdkError<StartReportJobError>>>;
    fn start_restore_job(&self, builder: StartRestoreJobInputBuilder) -> impl Future<Output = Result<StartRestoreJobOutput, SdkError<StartRestoreJobError>>>;
    fn stop_backup_job(&self, builder: StopBackupJobInputBuilder) -> impl Future<Output = Result<StopBackupJobOutput, SdkError<StopBackupJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_backup_plan(&self, builder: UpdateBackupPlanInputBuilder) -> impl Future<Output = Result<UpdateBackupPlanOutput, SdkError<UpdateBackupPlanError>>>;
    fn update_framework(&self, builder: UpdateFrameworkInputBuilder) -> impl Future<Output = Result<UpdateFrameworkOutput, SdkError<UpdateFrameworkError>>>;
    fn update_global_settings(&self, builder: UpdateGlobalSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalSettingsOutput, SdkError<UpdateGlobalSettingsError>>>;
    fn update_recovery_point_lifecycle(&self, builder: UpdateRecoveryPointLifecycleInputBuilder) -> impl Future<Output = Result<UpdateRecoveryPointLifecycleOutput, SdkError<UpdateRecoveryPointLifecycleError>>>;
    fn update_region_settings(&self, builder: UpdateRegionSettingsInputBuilder) -> impl Future<Output = Result<UpdateRegionSettingsOutput, SdkError<UpdateRegionSettingsError>>>;
    fn update_report_plan(&self, builder: UpdateReportPlanInputBuilder) -> impl Future<Output = Result<UpdateReportPlanOutput, SdkError<UpdateReportPlanError>>>;
    fn update_restore_testing_plan(&self, builder: UpdateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingPlanOutput, SdkError<UpdateRestoreTestingPlanError>>>;
    fn update_restore_testing_selection(&self, builder: UpdateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingSelectionOutput, SdkError<UpdateRestoreTestingSelectionError>>>;
}
impl BackupClient for BackupClientImpl {
    fn cancel_legal_hold(&self, builder: CancelLegalHoldInputBuilder) -> impl Future<Output = Result<CancelLegalHoldOutput, SdkError<CancelLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn create_backup_plan(&self, builder: CreateBackupPlanInputBuilder) -> impl Future<Output = Result<CreateBackupPlanOutput, SdkError<CreateBackupPlanError>>> {
        builder.send_with(&self.0)
    }
    fn create_backup_selection(&self, builder: CreateBackupSelectionInputBuilder) -> impl Future<Output = Result<CreateBackupSelectionOutput, SdkError<CreateBackupSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_backup_vault(&self, builder: CreateBackupVaultInputBuilder) -> impl Future<Output = Result<CreateBackupVaultOutput, SdkError<CreateBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn create_framework(&self, builder: CreateFrameworkInputBuilder) -> impl Future<Output = Result<CreateFrameworkOutput, SdkError<CreateFrameworkError>>> {
        builder.send_with(&self.0)
    }
    fn create_legal_hold(&self, builder: CreateLegalHoldInputBuilder) -> impl Future<Output = Result<CreateLegalHoldOutput, SdkError<CreateLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn create_logically_air_gapped_backup_vault(&self, builder: CreateLogicallyAirGappedBackupVaultInputBuilder) -> impl Future<Output = Result<CreateLogicallyAirGappedBackupVaultOutput, SdkError<CreateLogicallyAirGappedBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn create_report_plan(&self, builder: CreateReportPlanInputBuilder) -> impl Future<Output = Result<CreateReportPlanOutput, SdkError<CreateReportPlanError>>> {
        builder.send_with(&self.0)
    }
    fn create_restore_testing_plan(&self, builder: CreateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingPlanOutput, SdkError<CreateRestoreTestingPlanError>>> {
        builder.send_with(&self.0)
    }
    fn create_restore_testing_selection(&self, builder: CreateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingSelectionOutput, SdkError<CreateRestoreTestingSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_plan(&self, builder: DeleteBackupPlanInputBuilder) -> impl Future<Output = Result<DeleteBackupPlanOutput, SdkError<DeleteBackupPlanError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_selection(&self, builder: DeleteBackupSelectionInputBuilder) -> impl Future<Output = Result<DeleteBackupSelectionOutput, SdkError<DeleteBackupSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_vault(&self, builder: DeleteBackupVaultInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultOutput, SdkError<DeleteBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_vault_access_policy(&self, builder: DeleteBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultAccessPolicyOutput, SdkError<DeleteBackupVaultAccessPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_vault_lock_configuration(&self, builder: DeleteBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultLockConfigurationOutput, SdkError<DeleteBackupVaultLockConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup_vault_notifications(&self, builder: DeleteBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultNotificationsOutput, SdkError<DeleteBackupVaultNotificationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_framework(&self, builder: DeleteFrameworkInputBuilder) -> impl Future<Output = Result<DeleteFrameworkOutput, SdkError<DeleteFrameworkError>>> {
        builder.send_with(&self.0)
    }
    fn delete_recovery_point(&self, builder: DeleteRecoveryPointInputBuilder) -> impl Future<Output = Result<DeleteRecoveryPointOutput, SdkError<DeleteRecoveryPointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_report_plan(&self, builder: DeleteReportPlanInputBuilder) -> impl Future<Output = Result<DeleteReportPlanOutput, SdkError<DeleteReportPlanError>>> {
        builder.send_with(&self.0)
    }
    fn delete_restore_testing_plan(&self, builder: DeleteRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingPlanOutput, SdkError<DeleteRestoreTestingPlanError>>> {
        builder.send_with(&self.0)
    }
    fn delete_restore_testing_selection(&self, builder: DeleteRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingSelectionOutput, SdkError<DeleteRestoreTestingSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_backup_job(&self, builder: DescribeBackupJobInputBuilder) -> impl Future<Output = Result<DescribeBackupJobOutput, SdkError<DescribeBackupJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_backup_vault(&self, builder: DescribeBackupVaultInputBuilder) -> impl Future<Output = Result<DescribeBackupVaultOutput, SdkError<DescribeBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn describe_copy_job(&self, builder: DescribeCopyJobInputBuilder) -> impl Future<Output = Result<DescribeCopyJobOutput, SdkError<DescribeCopyJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_framework(&self, builder: DescribeFrameworkInputBuilder) -> impl Future<Output = Result<DescribeFrameworkOutput, SdkError<DescribeFrameworkError>>> {
        builder.send_with(&self.0)
    }
    fn describe_global_settings(&self, builder: DescribeGlobalSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalSettingsOutput, SdkError<DescribeGlobalSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_protected_resource(&self, builder: DescribeProtectedResourceInputBuilder) -> impl Future<Output = Result<DescribeProtectedResourceOutput, SdkError<DescribeProtectedResourceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_recovery_point(&self, builder: DescribeRecoveryPointInputBuilder) -> impl Future<Output = Result<DescribeRecoveryPointOutput, SdkError<DescribeRecoveryPointError>>> {
        builder.send_with(&self.0)
    }
    fn describe_region_settings(&self, builder: DescribeRegionSettingsInputBuilder) -> impl Future<Output = Result<DescribeRegionSettingsOutput, SdkError<DescribeRegionSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_report_job(&self, builder: DescribeReportJobInputBuilder) -> impl Future<Output = Result<DescribeReportJobOutput, SdkError<DescribeReportJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_report_plan(&self, builder: DescribeReportPlanInputBuilder) -> impl Future<Output = Result<DescribeReportPlanOutput, SdkError<DescribeReportPlanError>>> {
        builder.send_with(&self.0)
    }
    fn describe_restore_job(&self, builder: DescribeRestoreJobInputBuilder) -> impl Future<Output = Result<DescribeRestoreJobOutput, SdkError<DescribeRestoreJobError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_recovery_point(&self, builder: DisassociateRecoveryPointInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointOutput, SdkError<DisassociateRecoveryPointError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_recovery_point_from_parent(&self, builder: DisassociateRecoveryPointFromParentInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointFromParentOutput, SdkError<DisassociateRecoveryPointFromParentError>>> {
        builder.send_with(&self.0)
    }
    fn export_backup_plan_template(&self, builder: ExportBackupPlanTemplateInputBuilder) -> impl Future<Output = Result<ExportBackupPlanTemplateOutput, SdkError<ExportBackupPlanTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_plan(&self, builder: GetBackupPlanInputBuilder) -> impl Future<Output = Result<GetBackupPlanOutput, SdkError<GetBackupPlanError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_plan_from_json(&self, builder: GetBackupPlanFromJsonInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromJsonOutput, SdkError<GetBackupPlanFromJSONError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_plan_from_template(&self, builder: GetBackupPlanFromTemplateInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromTemplateOutput, SdkError<GetBackupPlanFromTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_selection(&self, builder: GetBackupSelectionInputBuilder) -> impl Future<Output = Result<GetBackupSelectionOutput, SdkError<GetBackupSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_vault_access_policy(&self, builder: GetBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<GetBackupVaultAccessPolicyOutput, SdkError<GetBackupVaultAccessPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_backup_vault_notifications(&self, builder: GetBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<GetBackupVaultNotificationsOutput, SdkError<GetBackupVaultNotificationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_legal_hold(&self, builder: GetLegalHoldInputBuilder) -> impl Future<Output = Result<GetLegalHoldOutput, SdkError<GetLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn get_recovery_point_restore_metadata(&self, builder: GetRecoveryPointRestoreMetadataInputBuilder) -> impl Future<Output = Result<GetRecoveryPointRestoreMetadataOutput, SdkError<GetRecoveryPointRestoreMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_restore_job_metadata(&self, builder: GetRestoreJobMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreJobMetadataOutput, SdkError<GetRestoreJobMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_restore_testing_inferred_metadata(&self, builder: GetRestoreTestingInferredMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreTestingInferredMetadataOutput, SdkError<GetRestoreTestingInferredMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_restore_testing_plan(&self, builder: GetRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<GetRestoreTestingPlanOutput, SdkError<GetRestoreTestingPlanError>>> {
        builder.send_with(&self.0)
    }
    fn get_restore_testing_selection(&self, builder: GetRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<GetRestoreTestingSelectionOutput, SdkError<GetRestoreTestingSelectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_supported_resource_types(&self, builder: GetSupportedResourceTypesInputBuilder) -> impl Future<Output = Result<GetSupportedResourceTypesOutput, SdkError<GetSupportedResourceTypesError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_job_summaries(&self, builder: ListBackupJobSummariesInputBuilder) -> impl Future<Output = Result<ListBackupJobSummariesOutput, SdkError<ListBackupJobSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_jobs(&self, builder: ListBackupJobsInputBuilder) -> impl Future<Output = Result<ListBackupJobsOutput, SdkError<ListBackupJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_plan_templates(&self, builder: ListBackupPlanTemplatesInputBuilder) -> impl Future<Output = Result<ListBackupPlanTemplatesOutput, SdkError<ListBackupPlanTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_plan_versions(&self, builder: ListBackupPlanVersionsInputBuilder) -> impl Future<Output = Result<ListBackupPlanVersionsOutput, SdkError<ListBackupPlanVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_plans(&self, builder: ListBackupPlansInputBuilder) -> impl Future<Output = Result<ListBackupPlansOutput, SdkError<ListBackupPlansError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_selections(&self, builder: ListBackupSelectionsInputBuilder) -> impl Future<Output = Result<ListBackupSelectionsOutput, SdkError<ListBackupSelectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_backup_vaults(&self, builder: ListBackupVaultsInputBuilder) -> impl Future<Output = Result<ListBackupVaultsOutput, SdkError<ListBackupVaultsError>>> {
        builder.send_with(&self.0)
    }
    fn list_copy_job_summaries(&self, builder: ListCopyJobSummariesInputBuilder) -> impl Future<Output = Result<ListCopyJobSummariesOutput, SdkError<ListCopyJobSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_copy_jobs(&self, builder: ListCopyJobsInputBuilder) -> impl Future<Output = Result<ListCopyJobsOutput, SdkError<ListCopyJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_frameworks(&self, builder: ListFrameworksInputBuilder) -> impl Future<Output = Result<ListFrameworksOutput, SdkError<ListFrameworksError>>> {
        builder.send_with(&self.0)
    }
    fn list_legal_holds(&self, builder: ListLegalHoldsInputBuilder) -> impl Future<Output = Result<ListLegalHoldsOutput, SdkError<ListLegalHoldsError>>> {
        builder.send_with(&self.0)
    }
    fn list_protected_resources(&self, builder: ListProtectedResourcesInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesOutput, SdkError<ListProtectedResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_protected_resources_by_backup_vault(&self, builder: ListProtectedResourcesByBackupVaultInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesByBackupVaultOutput, SdkError<ListProtectedResourcesByBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn list_recovery_points_by_backup_vault(&self, builder: ListRecoveryPointsByBackupVaultInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByBackupVaultOutput, SdkError<ListRecoveryPointsByBackupVaultError>>> {
        builder.send_with(&self.0)
    }
    fn list_recovery_points_by_legal_hold(&self, builder: ListRecoveryPointsByLegalHoldInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByLegalHoldOutput, SdkError<ListRecoveryPointsByLegalHoldError>>> {
        builder.send_with(&self.0)
    }
    fn list_recovery_points_by_resource(&self, builder: ListRecoveryPointsByResourceInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByResourceOutput, SdkError<ListRecoveryPointsByResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_report_jobs(&self, builder: ListReportJobsInputBuilder) -> impl Future<Output = Result<ListReportJobsOutput, SdkError<ListReportJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_report_plans(&self, builder: ListReportPlansInputBuilder) -> impl Future<Output = Result<ListReportPlansOutput, SdkError<ListReportPlansError>>> {
        builder.send_with(&self.0)
    }
    fn list_restore_job_summaries(&self, builder: ListRestoreJobSummariesInputBuilder) -> impl Future<Output = Result<ListRestoreJobSummariesOutput, SdkError<ListRestoreJobSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_restore_jobs(&self, builder: ListRestoreJobsInputBuilder) -> impl Future<Output = Result<ListRestoreJobsOutput, SdkError<ListRestoreJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_restore_jobs_by_protected_resource(&self, builder: ListRestoreJobsByProtectedResourceInputBuilder) -> impl Future<Output = Result<ListRestoreJobsByProtectedResourceOutput, SdkError<ListRestoreJobsByProtectedResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_restore_testing_plans(&self, builder: ListRestoreTestingPlansInputBuilder) -> impl Future<Output = Result<ListRestoreTestingPlansOutput, SdkError<ListRestoreTestingPlansError>>> {
        builder.send_with(&self.0)
    }
    fn list_restore_testing_selections(&self, builder: ListRestoreTestingSelectionsInputBuilder) -> impl Future<Output = Result<ListRestoreTestingSelectionsOutput, SdkError<ListRestoreTestingSelectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn put_backup_vault_access_policy(&self, builder: PutBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<PutBackupVaultAccessPolicyOutput, SdkError<PutBackupVaultAccessPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_backup_vault_lock_configuration(&self, builder: PutBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<PutBackupVaultLockConfigurationOutput, SdkError<PutBackupVaultLockConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_backup_vault_notifications(&self, builder: PutBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<PutBackupVaultNotificationsOutput, SdkError<PutBackupVaultNotificationsError>>> {
        builder.send_with(&self.0)
    }
    fn put_restore_validation_result(&self, builder: PutRestoreValidationResultInputBuilder) -> impl Future<Output = Result<PutRestoreValidationResultOutput, SdkError<PutRestoreValidationResultError>>> {
        builder.send_with(&self.0)
    }
    fn start_backup_job(&self, builder: StartBackupJobInputBuilder) -> impl Future<Output = Result<StartBackupJobOutput, SdkError<StartBackupJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_copy_job(&self, builder: StartCopyJobInputBuilder) -> impl Future<Output = Result<StartCopyJobOutput, SdkError<StartCopyJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_report_job(&self, builder: StartReportJobInputBuilder) -> impl Future<Output = Result<StartReportJobOutput, SdkError<StartReportJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_restore_job(&self, builder: StartRestoreJobInputBuilder) -> impl Future<Output = Result<StartRestoreJobOutput, SdkError<StartRestoreJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_backup_job(&self, builder: StopBackupJobInputBuilder) -> impl Future<Output = Result<StopBackupJobOutput, SdkError<StopBackupJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_backup_plan(&self, builder: UpdateBackupPlanInputBuilder) -> impl Future<Output = Result<UpdateBackupPlanOutput, SdkError<UpdateBackupPlanError>>> {
        builder.send_with(&self.0)
    }
    fn update_framework(&self, builder: UpdateFrameworkInputBuilder) -> impl Future<Output = Result<UpdateFrameworkOutput, SdkError<UpdateFrameworkError>>> {
        builder.send_with(&self.0)
    }
    fn update_global_settings(&self, builder: UpdateGlobalSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalSettingsOutput, SdkError<UpdateGlobalSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn update_recovery_point_lifecycle(&self, builder: UpdateRecoveryPointLifecycleInputBuilder) -> impl Future<Output = Result<UpdateRecoveryPointLifecycleOutput, SdkError<UpdateRecoveryPointLifecycleError>>> {
        builder.send_with(&self.0)
    }
    fn update_region_settings(&self, builder: UpdateRegionSettingsInputBuilder) -> impl Future<Output = Result<UpdateRegionSettingsOutput, SdkError<UpdateRegionSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn update_report_plan(&self, builder: UpdateReportPlanInputBuilder) -> impl Future<Output = Result<UpdateReportPlanOutput, SdkError<UpdateReportPlanError>>> {
        builder.send_with(&self.0)
    }
    fn update_restore_testing_plan(&self, builder: UpdateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingPlanOutput, SdkError<UpdateRestoreTestingPlanError>>> {
        builder.send_with(&self.0)
    }
    fn update_restore_testing_selection(&self, builder: UpdateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingSelectionOutput, SdkError<UpdateRestoreTestingSelectionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> BackupClient for T
where T: Deref,
      T::Target: BackupClient {
    fn cancel_legal_hold(&self, builder: CancelLegalHoldInputBuilder) -> impl Future<Output = Result<CancelLegalHoldOutput, SdkError<CancelLegalHoldError>>> {
        self.deref().cancel_legal_hold(builder)
    }
    fn create_backup_plan(&self, builder: CreateBackupPlanInputBuilder) -> impl Future<Output = Result<CreateBackupPlanOutput, SdkError<CreateBackupPlanError>>> {
        self.deref().create_backup_plan(builder)
    }
    fn create_backup_selection(&self, builder: CreateBackupSelectionInputBuilder) -> impl Future<Output = Result<CreateBackupSelectionOutput, SdkError<CreateBackupSelectionError>>> {
        self.deref().create_backup_selection(builder)
    }
    fn create_backup_vault(&self, builder: CreateBackupVaultInputBuilder) -> impl Future<Output = Result<CreateBackupVaultOutput, SdkError<CreateBackupVaultError>>> {
        self.deref().create_backup_vault(builder)
    }
    fn create_framework(&self, builder: CreateFrameworkInputBuilder) -> impl Future<Output = Result<CreateFrameworkOutput, SdkError<CreateFrameworkError>>> {
        self.deref().create_framework(builder)
    }
    fn create_legal_hold(&self, builder: CreateLegalHoldInputBuilder) -> impl Future<Output = Result<CreateLegalHoldOutput, SdkError<CreateLegalHoldError>>> {
        self.deref().create_legal_hold(builder)
    }
    fn create_logically_air_gapped_backup_vault(&self, builder: CreateLogicallyAirGappedBackupVaultInputBuilder) -> impl Future<Output = Result<CreateLogicallyAirGappedBackupVaultOutput, SdkError<CreateLogicallyAirGappedBackupVaultError>>> {
        self.deref().create_logically_air_gapped_backup_vault(builder)
    }
    fn create_report_plan(&self, builder: CreateReportPlanInputBuilder) -> impl Future<Output = Result<CreateReportPlanOutput, SdkError<CreateReportPlanError>>> {
        self.deref().create_report_plan(builder)
    }
    fn create_restore_testing_plan(&self, builder: CreateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingPlanOutput, SdkError<CreateRestoreTestingPlanError>>> {
        self.deref().create_restore_testing_plan(builder)
    }
    fn create_restore_testing_selection(&self, builder: CreateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<CreateRestoreTestingSelectionOutput, SdkError<CreateRestoreTestingSelectionError>>> {
        self.deref().create_restore_testing_selection(builder)
    }
    fn delete_backup_plan(&self, builder: DeleteBackupPlanInputBuilder) -> impl Future<Output = Result<DeleteBackupPlanOutput, SdkError<DeleteBackupPlanError>>> {
        self.deref().delete_backup_plan(builder)
    }
    fn delete_backup_selection(&self, builder: DeleteBackupSelectionInputBuilder) -> impl Future<Output = Result<DeleteBackupSelectionOutput, SdkError<DeleteBackupSelectionError>>> {
        self.deref().delete_backup_selection(builder)
    }
    fn delete_backup_vault(&self, builder: DeleteBackupVaultInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultOutput, SdkError<DeleteBackupVaultError>>> {
        self.deref().delete_backup_vault(builder)
    }
    fn delete_backup_vault_access_policy(&self, builder: DeleteBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultAccessPolicyOutput, SdkError<DeleteBackupVaultAccessPolicyError>>> {
        self.deref().delete_backup_vault_access_policy(builder)
    }
    fn delete_backup_vault_lock_configuration(&self, builder: DeleteBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultLockConfigurationOutput, SdkError<DeleteBackupVaultLockConfigurationError>>> {
        self.deref().delete_backup_vault_lock_configuration(builder)
    }
    fn delete_backup_vault_notifications(&self, builder: DeleteBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<DeleteBackupVaultNotificationsOutput, SdkError<DeleteBackupVaultNotificationsError>>> {
        self.deref().delete_backup_vault_notifications(builder)
    }
    fn delete_framework(&self, builder: DeleteFrameworkInputBuilder) -> impl Future<Output = Result<DeleteFrameworkOutput, SdkError<DeleteFrameworkError>>> {
        self.deref().delete_framework(builder)
    }
    fn delete_recovery_point(&self, builder: DeleteRecoveryPointInputBuilder) -> impl Future<Output = Result<DeleteRecoveryPointOutput, SdkError<DeleteRecoveryPointError>>> {
        self.deref().delete_recovery_point(builder)
    }
    fn delete_report_plan(&self, builder: DeleteReportPlanInputBuilder) -> impl Future<Output = Result<DeleteReportPlanOutput, SdkError<DeleteReportPlanError>>> {
        self.deref().delete_report_plan(builder)
    }
    fn delete_restore_testing_plan(&self, builder: DeleteRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingPlanOutput, SdkError<DeleteRestoreTestingPlanError>>> {
        self.deref().delete_restore_testing_plan(builder)
    }
    fn delete_restore_testing_selection(&self, builder: DeleteRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<DeleteRestoreTestingSelectionOutput, SdkError<DeleteRestoreTestingSelectionError>>> {
        self.deref().delete_restore_testing_selection(builder)
    }
    fn describe_backup_job(&self, builder: DescribeBackupJobInputBuilder) -> impl Future<Output = Result<DescribeBackupJobOutput, SdkError<DescribeBackupJobError>>> {
        self.deref().describe_backup_job(builder)
    }
    fn describe_backup_vault(&self, builder: DescribeBackupVaultInputBuilder) -> impl Future<Output = Result<DescribeBackupVaultOutput, SdkError<DescribeBackupVaultError>>> {
        self.deref().describe_backup_vault(builder)
    }
    fn describe_copy_job(&self, builder: DescribeCopyJobInputBuilder) -> impl Future<Output = Result<DescribeCopyJobOutput, SdkError<DescribeCopyJobError>>> {
        self.deref().describe_copy_job(builder)
    }
    fn describe_framework(&self, builder: DescribeFrameworkInputBuilder) -> impl Future<Output = Result<DescribeFrameworkOutput, SdkError<DescribeFrameworkError>>> {
        self.deref().describe_framework(builder)
    }
    fn describe_global_settings(&self, builder: DescribeGlobalSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalSettingsOutput, SdkError<DescribeGlobalSettingsError>>> {
        self.deref().describe_global_settings(builder)
    }
    fn describe_protected_resource(&self, builder: DescribeProtectedResourceInputBuilder) -> impl Future<Output = Result<DescribeProtectedResourceOutput, SdkError<DescribeProtectedResourceError>>> {
        self.deref().describe_protected_resource(builder)
    }
    fn describe_recovery_point(&self, builder: DescribeRecoveryPointInputBuilder) -> impl Future<Output = Result<DescribeRecoveryPointOutput, SdkError<DescribeRecoveryPointError>>> {
        self.deref().describe_recovery_point(builder)
    }
    fn describe_region_settings(&self, builder: DescribeRegionSettingsInputBuilder) -> impl Future<Output = Result<DescribeRegionSettingsOutput, SdkError<DescribeRegionSettingsError>>> {
        self.deref().describe_region_settings(builder)
    }
    fn describe_report_job(&self, builder: DescribeReportJobInputBuilder) -> impl Future<Output = Result<DescribeReportJobOutput, SdkError<DescribeReportJobError>>> {
        self.deref().describe_report_job(builder)
    }
    fn describe_report_plan(&self, builder: DescribeReportPlanInputBuilder) -> impl Future<Output = Result<DescribeReportPlanOutput, SdkError<DescribeReportPlanError>>> {
        self.deref().describe_report_plan(builder)
    }
    fn describe_restore_job(&self, builder: DescribeRestoreJobInputBuilder) -> impl Future<Output = Result<DescribeRestoreJobOutput, SdkError<DescribeRestoreJobError>>> {
        self.deref().describe_restore_job(builder)
    }
    fn disassociate_recovery_point(&self, builder: DisassociateRecoveryPointInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointOutput, SdkError<DisassociateRecoveryPointError>>> {
        self.deref().disassociate_recovery_point(builder)
    }
    fn disassociate_recovery_point_from_parent(&self, builder: DisassociateRecoveryPointFromParentInputBuilder) -> impl Future<Output = Result<DisassociateRecoveryPointFromParentOutput, SdkError<DisassociateRecoveryPointFromParentError>>> {
        self.deref().disassociate_recovery_point_from_parent(builder)
    }
    fn export_backup_plan_template(&self, builder: ExportBackupPlanTemplateInputBuilder) -> impl Future<Output = Result<ExportBackupPlanTemplateOutput, SdkError<ExportBackupPlanTemplateError>>> {
        self.deref().export_backup_plan_template(builder)
    }
    fn get_backup_plan(&self, builder: GetBackupPlanInputBuilder) -> impl Future<Output = Result<GetBackupPlanOutput, SdkError<GetBackupPlanError>>> {
        self.deref().get_backup_plan(builder)
    }
    fn get_backup_plan_from_json(&self, builder: GetBackupPlanFromJsonInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromJsonOutput, SdkError<GetBackupPlanFromJSONError>>> {
        self.deref().get_backup_plan_from_json(builder)
    }
    fn get_backup_plan_from_template(&self, builder: GetBackupPlanFromTemplateInputBuilder) -> impl Future<Output = Result<GetBackupPlanFromTemplateOutput, SdkError<GetBackupPlanFromTemplateError>>> {
        self.deref().get_backup_plan_from_template(builder)
    }
    fn get_backup_selection(&self, builder: GetBackupSelectionInputBuilder) -> impl Future<Output = Result<GetBackupSelectionOutput, SdkError<GetBackupSelectionError>>> {
        self.deref().get_backup_selection(builder)
    }
    fn get_backup_vault_access_policy(&self, builder: GetBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<GetBackupVaultAccessPolicyOutput, SdkError<GetBackupVaultAccessPolicyError>>> {
        self.deref().get_backup_vault_access_policy(builder)
    }
    fn get_backup_vault_notifications(&self, builder: GetBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<GetBackupVaultNotificationsOutput, SdkError<GetBackupVaultNotificationsError>>> {
        self.deref().get_backup_vault_notifications(builder)
    }
    fn get_legal_hold(&self, builder: GetLegalHoldInputBuilder) -> impl Future<Output = Result<GetLegalHoldOutput, SdkError<GetLegalHoldError>>> {
        self.deref().get_legal_hold(builder)
    }
    fn get_recovery_point_restore_metadata(&self, builder: GetRecoveryPointRestoreMetadataInputBuilder) -> impl Future<Output = Result<GetRecoveryPointRestoreMetadataOutput, SdkError<GetRecoveryPointRestoreMetadataError>>> {
        self.deref().get_recovery_point_restore_metadata(builder)
    }
    fn get_restore_job_metadata(&self, builder: GetRestoreJobMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreJobMetadataOutput, SdkError<GetRestoreJobMetadataError>>> {
        self.deref().get_restore_job_metadata(builder)
    }
    fn get_restore_testing_inferred_metadata(&self, builder: GetRestoreTestingInferredMetadataInputBuilder) -> impl Future<Output = Result<GetRestoreTestingInferredMetadataOutput, SdkError<GetRestoreTestingInferredMetadataError>>> {
        self.deref().get_restore_testing_inferred_metadata(builder)
    }
    fn get_restore_testing_plan(&self, builder: GetRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<GetRestoreTestingPlanOutput, SdkError<GetRestoreTestingPlanError>>> {
        self.deref().get_restore_testing_plan(builder)
    }
    fn get_restore_testing_selection(&self, builder: GetRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<GetRestoreTestingSelectionOutput, SdkError<GetRestoreTestingSelectionError>>> {
        self.deref().get_restore_testing_selection(builder)
    }
    fn get_supported_resource_types(&self, builder: GetSupportedResourceTypesInputBuilder) -> impl Future<Output = Result<GetSupportedResourceTypesOutput, SdkError<GetSupportedResourceTypesError>>> {
        self.deref().get_supported_resource_types(builder)
    }
    fn list_backup_job_summaries(&self, builder: ListBackupJobSummariesInputBuilder) -> impl Future<Output = Result<ListBackupJobSummariesOutput, SdkError<ListBackupJobSummariesError>>> {
        self.deref().list_backup_job_summaries(builder)
    }
    fn list_backup_jobs(&self, builder: ListBackupJobsInputBuilder) -> impl Future<Output = Result<ListBackupJobsOutput, SdkError<ListBackupJobsError>>> {
        self.deref().list_backup_jobs(builder)
    }
    fn list_backup_plan_templates(&self, builder: ListBackupPlanTemplatesInputBuilder) -> impl Future<Output = Result<ListBackupPlanTemplatesOutput, SdkError<ListBackupPlanTemplatesError>>> {
        self.deref().list_backup_plan_templates(builder)
    }
    fn list_backup_plan_versions(&self, builder: ListBackupPlanVersionsInputBuilder) -> impl Future<Output = Result<ListBackupPlanVersionsOutput, SdkError<ListBackupPlanVersionsError>>> {
        self.deref().list_backup_plan_versions(builder)
    }
    fn list_backup_plans(&self, builder: ListBackupPlansInputBuilder) -> impl Future<Output = Result<ListBackupPlansOutput, SdkError<ListBackupPlansError>>> {
        self.deref().list_backup_plans(builder)
    }
    fn list_backup_selections(&self, builder: ListBackupSelectionsInputBuilder) -> impl Future<Output = Result<ListBackupSelectionsOutput, SdkError<ListBackupSelectionsError>>> {
        self.deref().list_backup_selections(builder)
    }
    fn list_backup_vaults(&self, builder: ListBackupVaultsInputBuilder) -> impl Future<Output = Result<ListBackupVaultsOutput, SdkError<ListBackupVaultsError>>> {
        self.deref().list_backup_vaults(builder)
    }
    fn list_copy_job_summaries(&self, builder: ListCopyJobSummariesInputBuilder) -> impl Future<Output = Result<ListCopyJobSummariesOutput, SdkError<ListCopyJobSummariesError>>> {
        self.deref().list_copy_job_summaries(builder)
    }
    fn list_copy_jobs(&self, builder: ListCopyJobsInputBuilder) -> impl Future<Output = Result<ListCopyJobsOutput, SdkError<ListCopyJobsError>>> {
        self.deref().list_copy_jobs(builder)
    }
    fn list_frameworks(&self, builder: ListFrameworksInputBuilder) -> impl Future<Output = Result<ListFrameworksOutput, SdkError<ListFrameworksError>>> {
        self.deref().list_frameworks(builder)
    }
    fn list_legal_holds(&self, builder: ListLegalHoldsInputBuilder) -> impl Future<Output = Result<ListLegalHoldsOutput, SdkError<ListLegalHoldsError>>> {
        self.deref().list_legal_holds(builder)
    }
    fn list_protected_resources(&self, builder: ListProtectedResourcesInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesOutput, SdkError<ListProtectedResourcesError>>> {
        self.deref().list_protected_resources(builder)
    }
    fn list_protected_resources_by_backup_vault(&self, builder: ListProtectedResourcesByBackupVaultInputBuilder) -> impl Future<Output = Result<ListProtectedResourcesByBackupVaultOutput, SdkError<ListProtectedResourcesByBackupVaultError>>> {
        self.deref().list_protected_resources_by_backup_vault(builder)
    }
    fn list_recovery_points_by_backup_vault(&self, builder: ListRecoveryPointsByBackupVaultInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByBackupVaultOutput, SdkError<ListRecoveryPointsByBackupVaultError>>> {
        self.deref().list_recovery_points_by_backup_vault(builder)
    }
    fn list_recovery_points_by_legal_hold(&self, builder: ListRecoveryPointsByLegalHoldInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByLegalHoldOutput, SdkError<ListRecoveryPointsByLegalHoldError>>> {
        self.deref().list_recovery_points_by_legal_hold(builder)
    }
    fn list_recovery_points_by_resource(&self, builder: ListRecoveryPointsByResourceInputBuilder) -> impl Future<Output = Result<ListRecoveryPointsByResourceOutput, SdkError<ListRecoveryPointsByResourceError>>> {
        self.deref().list_recovery_points_by_resource(builder)
    }
    fn list_report_jobs(&self, builder: ListReportJobsInputBuilder) -> impl Future<Output = Result<ListReportJobsOutput, SdkError<ListReportJobsError>>> {
        self.deref().list_report_jobs(builder)
    }
    fn list_report_plans(&self, builder: ListReportPlansInputBuilder) -> impl Future<Output = Result<ListReportPlansOutput, SdkError<ListReportPlansError>>> {
        self.deref().list_report_plans(builder)
    }
    fn list_restore_job_summaries(&self, builder: ListRestoreJobSummariesInputBuilder) -> impl Future<Output = Result<ListRestoreJobSummariesOutput, SdkError<ListRestoreJobSummariesError>>> {
        self.deref().list_restore_job_summaries(builder)
    }
    fn list_restore_jobs(&self, builder: ListRestoreJobsInputBuilder) -> impl Future<Output = Result<ListRestoreJobsOutput, SdkError<ListRestoreJobsError>>> {
        self.deref().list_restore_jobs(builder)
    }
    fn list_restore_jobs_by_protected_resource(&self, builder: ListRestoreJobsByProtectedResourceInputBuilder) -> impl Future<Output = Result<ListRestoreJobsByProtectedResourceOutput, SdkError<ListRestoreJobsByProtectedResourceError>>> {
        self.deref().list_restore_jobs_by_protected_resource(builder)
    }
    fn list_restore_testing_plans(&self, builder: ListRestoreTestingPlansInputBuilder) -> impl Future<Output = Result<ListRestoreTestingPlansOutput, SdkError<ListRestoreTestingPlansError>>> {
        self.deref().list_restore_testing_plans(builder)
    }
    fn list_restore_testing_selections(&self, builder: ListRestoreTestingSelectionsInputBuilder) -> impl Future<Output = Result<ListRestoreTestingSelectionsOutput, SdkError<ListRestoreTestingSelectionsError>>> {
        self.deref().list_restore_testing_selections(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        self.deref().list_tags(builder)
    }
    fn put_backup_vault_access_policy(&self, builder: PutBackupVaultAccessPolicyInputBuilder) -> impl Future<Output = Result<PutBackupVaultAccessPolicyOutput, SdkError<PutBackupVaultAccessPolicyError>>> {
        self.deref().put_backup_vault_access_policy(builder)
    }
    fn put_backup_vault_lock_configuration(&self, builder: PutBackupVaultLockConfigurationInputBuilder) -> impl Future<Output = Result<PutBackupVaultLockConfigurationOutput, SdkError<PutBackupVaultLockConfigurationError>>> {
        self.deref().put_backup_vault_lock_configuration(builder)
    }
    fn put_backup_vault_notifications(&self, builder: PutBackupVaultNotificationsInputBuilder) -> impl Future<Output = Result<PutBackupVaultNotificationsOutput, SdkError<PutBackupVaultNotificationsError>>> {
        self.deref().put_backup_vault_notifications(builder)
    }
    fn put_restore_validation_result(&self, builder: PutRestoreValidationResultInputBuilder) -> impl Future<Output = Result<PutRestoreValidationResultOutput, SdkError<PutRestoreValidationResultError>>> {
        self.deref().put_restore_validation_result(builder)
    }
    fn start_backup_job(&self, builder: StartBackupJobInputBuilder) -> impl Future<Output = Result<StartBackupJobOutput, SdkError<StartBackupJobError>>> {
        self.deref().start_backup_job(builder)
    }
    fn start_copy_job(&self, builder: StartCopyJobInputBuilder) -> impl Future<Output = Result<StartCopyJobOutput, SdkError<StartCopyJobError>>> {
        self.deref().start_copy_job(builder)
    }
    fn start_report_job(&self, builder: StartReportJobInputBuilder) -> impl Future<Output = Result<StartReportJobOutput, SdkError<StartReportJobError>>> {
        self.deref().start_report_job(builder)
    }
    fn start_restore_job(&self, builder: StartRestoreJobInputBuilder) -> impl Future<Output = Result<StartRestoreJobOutput, SdkError<StartRestoreJobError>>> {
        self.deref().start_restore_job(builder)
    }
    fn stop_backup_job(&self, builder: StopBackupJobInputBuilder) -> impl Future<Output = Result<StopBackupJobOutput, SdkError<StopBackupJobError>>> {
        self.deref().stop_backup_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_backup_plan(&self, builder: UpdateBackupPlanInputBuilder) -> impl Future<Output = Result<UpdateBackupPlanOutput, SdkError<UpdateBackupPlanError>>> {
        self.deref().update_backup_plan(builder)
    }
    fn update_framework(&self, builder: UpdateFrameworkInputBuilder) -> impl Future<Output = Result<UpdateFrameworkOutput, SdkError<UpdateFrameworkError>>> {
        self.deref().update_framework(builder)
    }
    fn update_global_settings(&self, builder: UpdateGlobalSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalSettingsOutput, SdkError<UpdateGlobalSettingsError>>> {
        self.deref().update_global_settings(builder)
    }
    fn update_recovery_point_lifecycle(&self, builder: UpdateRecoveryPointLifecycleInputBuilder) -> impl Future<Output = Result<UpdateRecoveryPointLifecycleOutput, SdkError<UpdateRecoveryPointLifecycleError>>> {
        self.deref().update_recovery_point_lifecycle(builder)
    }
    fn update_region_settings(&self, builder: UpdateRegionSettingsInputBuilder) -> impl Future<Output = Result<UpdateRegionSettingsOutput, SdkError<UpdateRegionSettingsError>>> {
        self.deref().update_region_settings(builder)
    }
    fn update_report_plan(&self, builder: UpdateReportPlanInputBuilder) -> impl Future<Output = Result<UpdateReportPlanOutput, SdkError<UpdateReportPlanError>>> {
        self.deref().update_report_plan(builder)
    }
    fn update_restore_testing_plan(&self, builder: UpdateRestoreTestingPlanInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingPlanOutput, SdkError<UpdateRestoreTestingPlanError>>> {
        self.deref().update_restore_testing_plan(builder)
    }
    fn update_restore_testing_selection(&self, builder: UpdateRestoreTestingSelectionInputBuilder) -> impl Future<Output = Result<UpdateRestoreTestingSelectionOutput, SdkError<UpdateRestoreTestingSelectionError>>> {
        self.deref().update_restore_testing_selection(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edBackupClient {}
    impl BackupClient for edBackupClient {
        async fn cancel_legal_hold(&self, builder: CancelLegalHoldInputBuilder) -> Result<CancelLegalHoldOutput, SdkError<CancelLegalHoldError>>;
        async fn create_backup_plan(&self, builder: CreateBackupPlanInputBuilder) -> Result<CreateBackupPlanOutput, SdkError<CreateBackupPlanError>>;
        async fn create_backup_selection(&self, builder: CreateBackupSelectionInputBuilder) -> Result<CreateBackupSelectionOutput, SdkError<CreateBackupSelectionError>>;
        async fn create_backup_vault(&self, builder: CreateBackupVaultInputBuilder) -> Result<CreateBackupVaultOutput, SdkError<CreateBackupVaultError>>;
        async fn create_framework(&self, builder: CreateFrameworkInputBuilder) -> Result<CreateFrameworkOutput, SdkError<CreateFrameworkError>>;
        async fn create_legal_hold(&self, builder: CreateLegalHoldInputBuilder) -> Result<CreateLegalHoldOutput, SdkError<CreateLegalHoldError>>;
        async fn create_logically_air_gapped_backup_vault(&self, builder: CreateLogicallyAirGappedBackupVaultInputBuilder) -> Result<CreateLogicallyAirGappedBackupVaultOutput, SdkError<CreateLogicallyAirGappedBackupVaultError>>;
        async fn create_report_plan(&self, builder: CreateReportPlanInputBuilder) -> Result<CreateReportPlanOutput, SdkError<CreateReportPlanError>>;
        async fn create_restore_testing_plan(&self, builder: CreateRestoreTestingPlanInputBuilder) -> Result<CreateRestoreTestingPlanOutput, SdkError<CreateRestoreTestingPlanError>>;
        async fn create_restore_testing_selection(&self, builder: CreateRestoreTestingSelectionInputBuilder) -> Result<CreateRestoreTestingSelectionOutput, SdkError<CreateRestoreTestingSelectionError>>;
        async fn delete_backup_plan(&self, builder: DeleteBackupPlanInputBuilder) -> Result<DeleteBackupPlanOutput, SdkError<DeleteBackupPlanError>>;
        async fn delete_backup_selection(&self, builder: DeleteBackupSelectionInputBuilder) -> Result<DeleteBackupSelectionOutput, SdkError<DeleteBackupSelectionError>>;
        async fn delete_backup_vault(&self, builder: DeleteBackupVaultInputBuilder) -> Result<DeleteBackupVaultOutput, SdkError<DeleteBackupVaultError>>;
        async fn delete_backup_vault_access_policy(&self, builder: DeleteBackupVaultAccessPolicyInputBuilder) -> Result<DeleteBackupVaultAccessPolicyOutput, SdkError<DeleteBackupVaultAccessPolicyError>>;
        async fn delete_backup_vault_lock_configuration(&self, builder: DeleteBackupVaultLockConfigurationInputBuilder) -> Result<DeleteBackupVaultLockConfigurationOutput, SdkError<DeleteBackupVaultLockConfigurationError>>;
        async fn delete_backup_vault_notifications(&self, builder: DeleteBackupVaultNotificationsInputBuilder) -> Result<DeleteBackupVaultNotificationsOutput, SdkError<DeleteBackupVaultNotificationsError>>;
        async fn delete_framework(&self, builder: DeleteFrameworkInputBuilder) -> Result<DeleteFrameworkOutput, SdkError<DeleteFrameworkError>>;
        async fn delete_recovery_point(&self, builder: DeleteRecoveryPointInputBuilder) -> Result<DeleteRecoveryPointOutput, SdkError<DeleteRecoveryPointError>>;
        async fn delete_report_plan(&self, builder: DeleteReportPlanInputBuilder) -> Result<DeleteReportPlanOutput, SdkError<DeleteReportPlanError>>;
        async fn delete_restore_testing_plan(&self, builder: DeleteRestoreTestingPlanInputBuilder) -> Result<DeleteRestoreTestingPlanOutput, SdkError<DeleteRestoreTestingPlanError>>;
        async fn delete_restore_testing_selection(&self, builder: DeleteRestoreTestingSelectionInputBuilder) -> Result<DeleteRestoreTestingSelectionOutput, SdkError<DeleteRestoreTestingSelectionError>>;
        async fn describe_backup_job(&self, builder: DescribeBackupJobInputBuilder) -> Result<DescribeBackupJobOutput, SdkError<DescribeBackupJobError>>;
        async fn describe_backup_vault(&self, builder: DescribeBackupVaultInputBuilder) -> Result<DescribeBackupVaultOutput, SdkError<DescribeBackupVaultError>>;
        async fn describe_copy_job(&self, builder: DescribeCopyJobInputBuilder) -> Result<DescribeCopyJobOutput, SdkError<DescribeCopyJobError>>;
        async fn describe_framework(&self, builder: DescribeFrameworkInputBuilder) -> Result<DescribeFrameworkOutput, SdkError<DescribeFrameworkError>>;
        async fn describe_global_settings(&self, builder: DescribeGlobalSettingsInputBuilder) -> Result<DescribeGlobalSettingsOutput, SdkError<DescribeGlobalSettingsError>>;
        async fn describe_protected_resource(&self, builder: DescribeProtectedResourceInputBuilder) -> Result<DescribeProtectedResourceOutput, SdkError<DescribeProtectedResourceError>>;
        async fn describe_recovery_point(&self, builder: DescribeRecoveryPointInputBuilder) -> Result<DescribeRecoveryPointOutput, SdkError<DescribeRecoveryPointError>>;
        async fn describe_region_settings(&self, builder: DescribeRegionSettingsInputBuilder) -> Result<DescribeRegionSettingsOutput, SdkError<DescribeRegionSettingsError>>;
        async fn describe_report_job(&self, builder: DescribeReportJobInputBuilder) -> Result<DescribeReportJobOutput, SdkError<DescribeReportJobError>>;
        async fn describe_report_plan(&self, builder: DescribeReportPlanInputBuilder) -> Result<DescribeReportPlanOutput, SdkError<DescribeReportPlanError>>;
        async fn describe_restore_job(&self, builder: DescribeRestoreJobInputBuilder) -> Result<DescribeRestoreJobOutput, SdkError<DescribeRestoreJobError>>;
        async fn disassociate_recovery_point(&self, builder: DisassociateRecoveryPointInputBuilder) -> Result<DisassociateRecoveryPointOutput, SdkError<DisassociateRecoveryPointError>>;
        async fn disassociate_recovery_point_from_parent(&self, builder: DisassociateRecoveryPointFromParentInputBuilder) -> Result<DisassociateRecoveryPointFromParentOutput, SdkError<DisassociateRecoveryPointFromParentError>>;
        async fn export_backup_plan_template(&self, builder: ExportBackupPlanTemplateInputBuilder) -> Result<ExportBackupPlanTemplateOutput, SdkError<ExportBackupPlanTemplateError>>;
        async fn get_backup_plan(&self, builder: GetBackupPlanInputBuilder) -> Result<GetBackupPlanOutput, SdkError<GetBackupPlanError>>;
        async fn get_backup_plan_from_json(&self, builder: GetBackupPlanFromJsonInputBuilder) -> Result<GetBackupPlanFromJsonOutput, SdkError<GetBackupPlanFromJSONError>>;
        async fn get_backup_plan_from_template(&self, builder: GetBackupPlanFromTemplateInputBuilder) -> Result<GetBackupPlanFromTemplateOutput, SdkError<GetBackupPlanFromTemplateError>>;
        async fn get_backup_selection(&self, builder: GetBackupSelectionInputBuilder) -> Result<GetBackupSelectionOutput, SdkError<GetBackupSelectionError>>;
        async fn get_backup_vault_access_policy(&self, builder: GetBackupVaultAccessPolicyInputBuilder) -> Result<GetBackupVaultAccessPolicyOutput, SdkError<GetBackupVaultAccessPolicyError>>;
        async fn get_backup_vault_notifications(&self, builder: GetBackupVaultNotificationsInputBuilder) -> Result<GetBackupVaultNotificationsOutput, SdkError<GetBackupVaultNotificationsError>>;
        async fn get_legal_hold(&self, builder: GetLegalHoldInputBuilder) -> Result<GetLegalHoldOutput, SdkError<GetLegalHoldError>>;
        async fn get_recovery_point_restore_metadata(&self, builder: GetRecoveryPointRestoreMetadataInputBuilder) -> Result<GetRecoveryPointRestoreMetadataOutput, SdkError<GetRecoveryPointRestoreMetadataError>>;
        async fn get_restore_job_metadata(&self, builder: GetRestoreJobMetadataInputBuilder) -> Result<GetRestoreJobMetadataOutput, SdkError<GetRestoreJobMetadataError>>;
        async fn get_restore_testing_inferred_metadata(&self, builder: GetRestoreTestingInferredMetadataInputBuilder) -> Result<GetRestoreTestingInferredMetadataOutput, SdkError<GetRestoreTestingInferredMetadataError>>;
        async fn get_restore_testing_plan(&self, builder: GetRestoreTestingPlanInputBuilder) -> Result<GetRestoreTestingPlanOutput, SdkError<GetRestoreTestingPlanError>>;
        async fn get_restore_testing_selection(&self, builder: GetRestoreTestingSelectionInputBuilder) -> Result<GetRestoreTestingSelectionOutput, SdkError<GetRestoreTestingSelectionError>>;
        async fn get_supported_resource_types(&self, builder: GetSupportedResourceTypesInputBuilder) -> Result<GetSupportedResourceTypesOutput, SdkError<GetSupportedResourceTypesError>>;
        async fn list_backup_job_summaries(&self, builder: ListBackupJobSummariesInputBuilder) -> Result<ListBackupJobSummariesOutput, SdkError<ListBackupJobSummariesError>>;
        async fn list_backup_jobs(&self, builder: ListBackupJobsInputBuilder) -> Result<ListBackupJobsOutput, SdkError<ListBackupJobsError>>;
        async fn list_backup_plan_templates(&self, builder: ListBackupPlanTemplatesInputBuilder) -> Result<ListBackupPlanTemplatesOutput, SdkError<ListBackupPlanTemplatesError>>;
        async fn list_backup_plan_versions(&self, builder: ListBackupPlanVersionsInputBuilder) -> Result<ListBackupPlanVersionsOutput, SdkError<ListBackupPlanVersionsError>>;
        async fn list_backup_plans(&self, builder: ListBackupPlansInputBuilder) -> Result<ListBackupPlansOutput, SdkError<ListBackupPlansError>>;
        async fn list_backup_selections(&self, builder: ListBackupSelectionsInputBuilder) -> Result<ListBackupSelectionsOutput, SdkError<ListBackupSelectionsError>>;
        async fn list_backup_vaults(&self, builder: ListBackupVaultsInputBuilder) -> Result<ListBackupVaultsOutput, SdkError<ListBackupVaultsError>>;
        async fn list_copy_job_summaries(&self, builder: ListCopyJobSummariesInputBuilder) -> Result<ListCopyJobSummariesOutput, SdkError<ListCopyJobSummariesError>>;
        async fn list_copy_jobs(&self, builder: ListCopyJobsInputBuilder) -> Result<ListCopyJobsOutput, SdkError<ListCopyJobsError>>;
        async fn list_frameworks(&self, builder: ListFrameworksInputBuilder) -> Result<ListFrameworksOutput, SdkError<ListFrameworksError>>;
        async fn list_legal_holds(&self, builder: ListLegalHoldsInputBuilder) -> Result<ListLegalHoldsOutput, SdkError<ListLegalHoldsError>>;
        async fn list_protected_resources(&self, builder: ListProtectedResourcesInputBuilder) -> Result<ListProtectedResourcesOutput, SdkError<ListProtectedResourcesError>>;
        async fn list_protected_resources_by_backup_vault(&self, builder: ListProtectedResourcesByBackupVaultInputBuilder) -> Result<ListProtectedResourcesByBackupVaultOutput, SdkError<ListProtectedResourcesByBackupVaultError>>;
        async fn list_recovery_points_by_backup_vault(&self, builder: ListRecoveryPointsByBackupVaultInputBuilder) -> Result<ListRecoveryPointsByBackupVaultOutput, SdkError<ListRecoveryPointsByBackupVaultError>>;
        async fn list_recovery_points_by_legal_hold(&self, builder: ListRecoveryPointsByLegalHoldInputBuilder) -> Result<ListRecoveryPointsByLegalHoldOutput, SdkError<ListRecoveryPointsByLegalHoldError>>;
        async fn list_recovery_points_by_resource(&self, builder: ListRecoveryPointsByResourceInputBuilder) -> Result<ListRecoveryPointsByResourceOutput, SdkError<ListRecoveryPointsByResourceError>>;
        async fn list_report_jobs(&self, builder: ListReportJobsInputBuilder) -> Result<ListReportJobsOutput, SdkError<ListReportJobsError>>;
        async fn list_report_plans(&self, builder: ListReportPlansInputBuilder) -> Result<ListReportPlansOutput, SdkError<ListReportPlansError>>;
        async fn list_restore_job_summaries(&self, builder: ListRestoreJobSummariesInputBuilder) -> Result<ListRestoreJobSummariesOutput, SdkError<ListRestoreJobSummariesError>>;
        async fn list_restore_jobs(&self, builder: ListRestoreJobsInputBuilder) -> Result<ListRestoreJobsOutput, SdkError<ListRestoreJobsError>>;
        async fn list_restore_jobs_by_protected_resource(&self, builder: ListRestoreJobsByProtectedResourceInputBuilder) -> Result<ListRestoreJobsByProtectedResourceOutput, SdkError<ListRestoreJobsByProtectedResourceError>>;
        async fn list_restore_testing_plans(&self, builder: ListRestoreTestingPlansInputBuilder) -> Result<ListRestoreTestingPlansOutput, SdkError<ListRestoreTestingPlansError>>;
        async fn list_restore_testing_selections(&self, builder: ListRestoreTestingSelectionsInputBuilder) -> Result<ListRestoreTestingSelectionsOutput, SdkError<ListRestoreTestingSelectionsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn put_backup_vault_access_policy(&self, builder: PutBackupVaultAccessPolicyInputBuilder) -> Result<PutBackupVaultAccessPolicyOutput, SdkError<PutBackupVaultAccessPolicyError>>;
        async fn put_backup_vault_lock_configuration(&self, builder: PutBackupVaultLockConfigurationInputBuilder) -> Result<PutBackupVaultLockConfigurationOutput, SdkError<PutBackupVaultLockConfigurationError>>;
        async fn put_backup_vault_notifications(&self, builder: PutBackupVaultNotificationsInputBuilder) -> Result<PutBackupVaultNotificationsOutput, SdkError<PutBackupVaultNotificationsError>>;
        async fn put_restore_validation_result(&self, builder: PutRestoreValidationResultInputBuilder) -> Result<PutRestoreValidationResultOutput, SdkError<PutRestoreValidationResultError>>;
        async fn start_backup_job(&self, builder: StartBackupJobInputBuilder) -> Result<StartBackupJobOutput, SdkError<StartBackupJobError>>;
        async fn start_copy_job(&self, builder: StartCopyJobInputBuilder) -> Result<StartCopyJobOutput, SdkError<StartCopyJobError>>;
        async fn start_report_job(&self, builder: StartReportJobInputBuilder) -> Result<StartReportJobOutput, SdkError<StartReportJobError>>;
        async fn start_restore_job(&self, builder: StartRestoreJobInputBuilder) -> Result<StartRestoreJobOutput, SdkError<StartRestoreJobError>>;
        async fn stop_backup_job(&self, builder: StopBackupJobInputBuilder) -> Result<StopBackupJobOutput, SdkError<StopBackupJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_backup_plan(&self, builder: UpdateBackupPlanInputBuilder) -> Result<UpdateBackupPlanOutput, SdkError<UpdateBackupPlanError>>;
        async fn update_framework(&self, builder: UpdateFrameworkInputBuilder) -> Result<UpdateFrameworkOutput, SdkError<UpdateFrameworkError>>;
        async fn update_global_settings(&self, builder: UpdateGlobalSettingsInputBuilder) -> Result<UpdateGlobalSettingsOutput, SdkError<UpdateGlobalSettingsError>>;
        async fn update_recovery_point_lifecycle(&self, builder: UpdateRecoveryPointLifecycleInputBuilder) -> Result<UpdateRecoveryPointLifecycleOutput, SdkError<UpdateRecoveryPointLifecycleError>>;
        async fn update_region_settings(&self, builder: UpdateRegionSettingsInputBuilder) -> Result<UpdateRegionSettingsOutput, SdkError<UpdateRegionSettingsError>>;
        async fn update_report_plan(&self, builder: UpdateReportPlanInputBuilder) -> Result<UpdateReportPlanOutput, SdkError<UpdateReportPlanError>>;
        async fn update_restore_testing_plan(&self, builder: UpdateRestoreTestingPlanInputBuilder) -> Result<UpdateRestoreTestingPlanOutput, SdkError<UpdateRestoreTestingPlanError>>;
        async fn update_restore_testing_selection(&self, builder: UpdateRestoreTestingSelectionInputBuilder) -> Result<UpdateRestoreTestingSelectionOutput, SdkError<UpdateRestoreTestingSelectionError>>;
    }
}
