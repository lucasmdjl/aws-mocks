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
use aws_sdk_codebuild::operation::batch_delete_builds::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_build_batches::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_builds::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_fleets::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_projects::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_report_groups::{builders::*, *};
use aws_sdk_codebuild::operation::batch_get_reports::{builders::*, *};
use aws_sdk_codebuild::operation::create_fleet::{builders::*, *};
use aws_sdk_codebuild::operation::create_project::{builders::*, *};
use aws_sdk_codebuild::operation::create_report_group::{builders::*, *};
use aws_sdk_codebuild::operation::create_webhook::{builders::*, *};
use aws_sdk_codebuild::operation::delete_build_batch::{builders::*, *};
use aws_sdk_codebuild::operation::delete_fleet::{builders::*, *};
use aws_sdk_codebuild::operation::delete_project::{builders::*, *};
use aws_sdk_codebuild::operation::delete_report::{builders::*, *};
use aws_sdk_codebuild::operation::delete_report_group::{builders::*, *};
use aws_sdk_codebuild::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_codebuild::operation::delete_source_credentials::{builders::*, *};
use aws_sdk_codebuild::operation::delete_webhook::{builders::*, *};
use aws_sdk_codebuild::operation::describe_code_coverages::{builders::*, *};
use aws_sdk_codebuild::operation::describe_test_cases::{builders::*, *};
use aws_sdk_codebuild::operation::get_report_group_trend::{builders::*, *};
use aws_sdk_codebuild::operation::get_resource_policy::{builders::*, *};
use aws_sdk_codebuild::operation::import_source_credentials::{builders::*, *};
use aws_sdk_codebuild::operation::invalidate_project_cache::{builders::*, *};
use aws_sdk_codebuild::operation::list_build_batches::{builders::*, *};
use aws_sdk_codebuild::operation::list_build_batches_for_project::{builders::*, *};
use aws_sdk_codebuild::operation::list_builds::{builders::*, *};
use aws_sdk_codebuild::operation::list_builds_for_project::{builders::*, *};
use aws_sdk_codebuild::operation::list_curated_environment_images::{builders::*, *};
use aws_sdk_codebuild::operation::list_fleets::{builders::*, *};
use aws_sdk_codebuild::operation::list_projects::{builders::*, *};
use aws_sdk_codebuild::operation::list_report_groups::{builders::*, *};
use aws_sdk_codebuild::operation::list_reports::{builders::*, *};
use aws_sdk_codebuild::operation::list_reports_for_report_group::{builders::*, *};
use aws_sdk_codebuild::operation::list_shared_projects::{builders::*, *};
use aws_sdk_codebuild::operation::list_shared_report_groups::{builders::*, *};
use aws_sdk_codebuild::operation::list_source_credentials::{builders::*, *};
use aws_sdk_codebuild::operation::put_resource_policy::{builders::*, *};
use aws_sdk_codebuild::operation::retry_build::{builders::*, *};
use aws_sdk_codebuild::operation::retry_build_batch::{builders::*, *};
use aws_sdk_codebuild::operation::start_build::{builders::*, *};
use aws_sdk_codebuild::operation::start_build_batch::{builders::*, *};
use aws_sdk_codebuild::operation::stop_build::{builders::*, *};
use aws_sdk_codebuild::operation::stop_build_batch::{builders::*, *};
use aws_sdk_codebuild::operation::update_fleet::{builders::*, *};
use aws_sdk_codebuild::operation::update_project::{builders::*, *};
use aws_sdk_codebuild::operation::update_project_visibility::{builders::*, *};
use aws_sdk_codebuild::operation::update_report_group::{builders::*, *};
use aws_sdk_codebuild::operation::update_webhook::{builders::*, *};
use aws_sdk_codebuild::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_codebuild::Client;
use std::ops::Deref;

pub use aws_sdk_codebuild::*;

pub struct CodeBuildClientImpl(Client);
impl CodeBuildClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CodeBuildClient {
    fn batch_delete_builds(&self, builder: BatchDeleteBuildsInputBuilder) -> impl Future<Output = Result<BatchDeleteBuildsOutput, SdkError<BatchDeleteBuildsError>>>;
    fn batch_get_build_batches(&self, builder: BatchGetBuildBatchesInputBuilder) -> impl Future<Output = Result<BatchGetBuildBatchesOutput, SdkError<BatchGetBuildBatchesError>>>;
    fn batch_get_builds(&self, builder: BatchGetBuildsInputBuilder) -> impl Future<Output = Result<BatchGetBuildsOutput, SdkError<BatchGetBuildsError>>>;
    fn batch_get_fleets(&self, builder: BatchGetFleetsInputBuilder) -> impl Future<Output = Result<BatchGetFleetsOutput, SdkError<BatchGetFleetsError>>>;
    fn batch_get_projects(&self, builder: BatchGetProjectsInputBuilder) -> impl Future<Output = Result<BatchGetProjectsOutput, SdkError<BatchGetProjectsError>>>;
    fn batch_get_report_groups(&self, builder: BatchGetReportGroupsInputBuilder) -> impl Future<Output = Result<BatchGetReportGroupsOutput, SdkError<BatchGetReportGroupsError>>>;
    fn batch_get_reports(&self, builder: BatchGetReportsInputBuilder) -> impl Future<Output = Result<BatchGetReportsOutput, SdkError<BatchGetReportsError>>>;
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>>;
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>>;
    fn create_report_group(&self, builder: CreateReportGroupInputBuilder) -> impl Future<Output = Result<CreateReportGroupOutput, SdkError<CreateReportGroupError>>>;
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>>;
    fn delete_build_batch(&self, builder: DeleteBuildBatchInputBuilder) -> impl Future<Output = Result<DeleteBuildBatchOutput, SdkError<DeleteBuildBatchError>>>;
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>>;
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>>;
    fn delete_report(&self, builder: DeleteReportInputBuilder) -> impl Future<Output = Result<DeleteReportOutput, SdkError<DeleteReportError>>>;
    fn delete_report_group(&self, builder: DeleteReportGroupInputBuilder) -> impl Future<Output = Result<DeleteReportGroupOutput, SdkError<DeleteReportGroupError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_source_credentials(&self, builder: DeleteSourceCredentialsInputBuilder) -> impl Future<Output = Result<DeleteSourceCredentialsOutput, SdkError<DeleteSourceCredentialsError>>>;
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>>;
    fn describe_code_coverages(&self, builder: DescribeCodeCoveragesInputBuilder) -> impl Future<Output = Result<DescribeCodeCoveragesOutput, SdkError<DescribeCodeCoveragesError>>>;
    fn describe_test_cases(&self, builder: DescribeTestCasesInputBuilder) -> impl Future<Output = Result<DescribeTestCasesOutput, SdkError<DescribeTestCasesError>>>;
    fn get_report_group_trend(&self, builder: GetReportGroupTrendInputBuilder) -> impl Future<Output = Result<GetReportGroupTrendOutput, SdkError<GetReportGroupTrendError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn import_source_credentials(&self, builder: ImportSourceCredentialsInputBuilder) -> impl Future<Output = Result<ImportSourceCredentialsOutput, SdkError<ImportSourceCredentialsError>>>;
    fn invalidate_project_cache(&self, builder: InvalidateProjectCacheInputBuilder) -> impl Future<Output = Result<InvalidateProjectCacheOutput, SdkError<InvalidateProjectCacheError>>>;
    fn list_build_batches(&self, builder: ListBuildBatchesInputBuilder) -> impl Future<Output = Result<ListBuildBatchesOutput, SdkError<ListBuildBatchesError>>>;
    fn list_build_batches_for_project(&self, builder: ListBuildBatchesForProjectInputBuilder) -> impl Future<Output = Result<ListBuildBatchesForProjectOutput, SdkError<ListBuildBatchesForProjectError>>>;
    fn list_builds(&self, builder: ListBuildsInputBuilder) -> impl Future<Output = Result<ListBuildsOutput, SdkError<ListBuildsError>>>;
    fn list_builds_for_project(&self, builder: ListBuildsForProjectInputBuilder) -> impl Future<Output = Result<ListBuildsForProjectOutput, SdkError<ListBuildsForProjectError>>>;
    fn list_curated_environment_images(&self, builder: ListCuratedEnvironmentImagesInputBuilder) -> impl Future<Output = Result<ListCuratedEnvironmentImagesOutput, SdkError<ListCuratedEnvironmentImagesError>>>;
    fn list_fleets(&self, builder: ListFleetsInputBuilder) -> impl Future<Output = Result<ListFleetsOutput, SdkError<ListFleetsError>>>;
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>>;
    fn list_report_groups(&self, builder: ListReportGroupsInputBuilder) -> impl Future<Output = Result<ListReportGroupsOutput, SdkError<ListReportGroupsError>>>;
    fn list_reports(&self, builder: ListReportsInputBuilder) -> impl Future<Output = Result<ListReportsOutput, SdkError<ListReportsError>>>;
    fn list_reports_for_report_group(&self, builder: ListReportsForReportGroupInputBuilder) -> impl Future<Output = Result<ListReportsForReportGroupOutput, SdkError<ListReportsForReportGroupError>>>;
    fn list_shared_projects(&self, builder: ListSharedProjectsInputBuilder) -> impl Future<Output = Result<ListSharedProjectsOutput, SdkError<ListSharedProjectsError>>>;
    fn list_shared_report_groups(&self, builder: ListSharedReportGroupsInputBuilder) -> impl Future<Output = Result<ListSharedReportGroupsOutput, SdkError<ListSharedReportGroupsError>>>;
    fn list_source_credentials(&self, builder: ListSourceCredentialsInputBuilder) -> impl Future<Output = Result<ListSourceCredentialsOutput, SdkError<ListSourceCredentialsError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn retry_build(&self, builder: RetryBuildInputBuilder) -> impl Future<Output = Result<RetryBuildOutput, SdkError<RetryBuildError>>>;
    fn retry_build_batch(&self, builder: RetryBuildBatchInputBuilder) -> impl Future<Output = Result<RetryBuildBatchOutput, SdkError<RetryBuildBatchError>>>;
    fn start_build(&self, builder: StartBuildInputBuilder) -> impl Future<Output = Result<StartBuildOutput, SdkError<StartBuildError>>>;
    fn start_build_batch(&self, builder: StartBuildBatchInputBuilder) -> impl Future<Output = Result<StartBuildBatchOutput, SdkError<StartBuildBatchError>>>;
    fn stop_build(&self, builder: StopBuildInputBuilder) -> impl Future<Output = Result<StopBuildOutput, SdkError<StopBuildError>>>;
    fn stop_build_batch(&self, builder: StopBuildBatchInputBuilder) -> impl Future<Output = Result<StopBuildBatchOutput, SdkError<StopBuildBatchError>>>;
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>>;
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>>;
    fn update_project_visibility(&self, builder: UpdateProjectVisibilityInputBuilder) -> impl Future<Output = Result<UpdateProjectVisibilityOutput, SdkError<UpdateProjectVisibilityError>>>;
    fn update_report_group(&self, builder: UpdateReportGroupInputBuilder) -> impl Future<Output = Result<UpdateReportGroupOutput, SdkError<UpdateReportGroupError>>>;
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>>;
}
impl CodeBuildClient for CodeBuildClientImpl {
    fn batch_delete_builds(&self, builder: BatchDeleteBuildsInputBuilder) -> impl Future<Output = Result<BatchDeleteBuildsOutput, SdkError<BatchDeleteBuildsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_build_batches(&self, builder: BatchGetBuildBatchesInputBuilder) -> impl Future<Output = Result<BatchGetBuildBatchesOutput, SdkError<BatchGetBuildBatchesError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_builds(&self, builder: BatchGetBuildsInputBuilder) -> impl Future<Output = Result<BatchGetBuildsOutput, SdkError<BatchGetBuildsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_fleets(&self, builder: BatchGetFleetsInputBuilder) -> impl Future<Output = Result<BatchGetFleetsOutput, SdkError<BatchGetFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_projects(&self, builder: BatchGetProjectsInputBuilder) -> impl Future<Output = Result<BatchGetProjectsOutput, SdkError<BatchGetProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_report_groups(&self, builder: BatchGetReportGroupsInputBuilder) -> impl Future<Output = Result<BatchGetReportGroupsOutput, SdkError<BatchGetReportGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_reports(&self, builder: BatchGetReportsInputBuilder) -> impl Future<Output = Result<BatchGetReportsOutput, SdkError<BatchGetReportsError>>> {
        builder.send_with(&self.0)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn create_report_group(&self, builder: CreateReportGroupInputBuilder) -> impl Future<Output = Result<CreateReportGroupOutput, SdkError<CreateReportGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn delete_build_batch(&self, builder: DeleteBuildBatchInputBuilder) -> impl Future<Output = Result<DeleteBuildBatchOutput, SdkError<DeleteBuildBatchError>>> {
        builder.send_with(&self.0)
    }
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_report(&self, builder: DeleteReportInputBuilder) -> impl Future<Output = Result<DeleteReportOutput, SdkError<DeleteReportError>>> {
        builder.send_with(&self.0)
    }
    fn delete_report_group(&self, builder: DeleteReportGroupInputBuilder) -> impl Future<Output = Result<DeleteReportGroupOutput, SdkError<DeleteReportGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_source_credentials(&self, builder: DeleteSourceCredentialsInputBuilder) -> impl Future<Output = Result<DeleteSourceCredentialsOutput, SdkError<DeleteSourceCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn describe_code_coverages(&self, builder: DescribeCodeCoveragesInputBuilder) -> impl Future<Output = Result<DescribeCodeCoveragesOutput, SdkError<DescribeCodeCoveragesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_test_cases(&self, builder: DescribeTestCasesInputBuilder) -> impl Future<Output = Result<DescribeTestCasesOutput, SdkError<DescribeTestCasesError>>> {
        builder.send_with(&self.0)
    }
    fn get_report_group_trend(&self, builder: GetReportGroupTrendInputBuilder) -> impl Future<Output = Result<GetReportGroupTrendOutput, SdkError<GetReportGroupTrendError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn import_source_credentials(&self, builder: ImportSourceCredentialsInputBuilder) -> impl Future<Output = Result<ImportSourceCredentialsOutput, SdkError<ImportSourceCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn invalidate_project_cache(&self, builder: InvalidateProjectCacheInputBuilder) -> impl Future<Output = Result<InvalidateProjectCacheOutput, SdkError<InvalidateProjectCacheError>>> {
        builder.send_with(&self.0)
    }
    fn list_build_batches(&self, builder: ListBuildBatchesInputBuilder) -> impl Future<Output = Result<ListBuildBatchesOutput, SdkError<ListBuildBatchesError>>> {
        builder.send_with(&self.0)
    }
    fn list_build_batches_for_project(&self, builder: ListBuildBatchesForProjectInputBuilder) -> impl Future<Output = Result<ListBuildBatchesForProjectOutput, SdkError<ListBuildBatchesForProjectError>>> {
        builder.send_with(&self.0)
    }
    fn list_builds(&self, builder: ListBuildsInputBuilder) -> impl Future<Output = Result<ListBuildsOutput, SdkError<ListBuildsError>>> {
        builder.send_with(&self.0)
    }
    fn list_builds_for_project(&self, builder: ListBuildsForProjectInputBuilder) -> impl Future<Output = Result<ListBuildsForProjectOutput, SdkError<ListBuildsForProjectError>>> {
        builder.send_with(&self.0)
    }
    fn list_curated_environment_images(&self, builder: ListCuratedEnvironmentImagesInputBuilder) -> impl Future<Output = Result<ListCuratedEnvironmentImagesOutput, SdkError<ListCuratedEnvironmentImagesError>>> {
        builder.send_with(&self.0)
    }
    fn list_fleets(&self, builder: ListFleetsInputBuilder) -> impl Future<Output = Result<ListFleetsOutput, SdkError<ListFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn list_report_groups(&self, builder: ListReportGroupsInputBuilder) -> impl Future<Output = Result<ListReportGroupsOutput, SdkError<ListReportGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_reports(&self, builder: ListReportsInputBuilder) -> impl Future<Output = Result<ListReportsOutput, SdkError<ListReportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_reports_for_report_group(&self, builder: ListReportsForReportGroupInputBuilder) -> impl Future<Output = Result<ListReportsForReportGroupOutput, SdkError<ListReportsForReportGroupError>>> {
        builder.send_with(&self.0)
    }
    fn list_shared_projects(&self, builder: ListSharedProjectsInputBuilder) -> impl Future<Output = Result<ListSharedProjectsOutput, SdkError<ListSharedProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn list_shared_report_groups(&self, builder: ListSharedReportGroupsInputBuilder) -> impl Future<Output = Result<ListSharedReportGroupsOutput, SdkError<ListSharedReportGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_source_credentials(&self, builder: ListSourceCredentialsInputBuilder) -> impl Future<Output = Result<ListSourceCredentialsOutput, SdkError<ListSourceCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn retry_build(&self, builder: RetryBuildInputBuilder) -> impl Future<Output = Result<RetryBuildOutput, SdkError<RetryBuildError>>> {
        builder.send_with(&self.0)
    }
    fn retry_build_batch(&self, builder: RetryBuildBatchInputBuilder) -> impl Future<Output = Result<RetryBuildBatchOutput, SdkError<RetryBuildBatchError>>> {
        builder.send_with(&self.0)
    }
    fn start_build(&self, builder: StartBuildInputBuilder) -> impl Future<Output = Result<StartBuildOutput, SdkError<StartBuildError>>> {
        builder.send_with(&self.0)
    }
    fn start_build_batch(&self, builder: StartBuildBatchInputBuilder) -> impl Future<Output = Result<StartBuildBatchOutput, SdkError<StartBuildBatchError>>> {
        builder.send_with(&self.0)
    }
    fn stop_build(&self, builder: StopBuildInputBuilder) -> impl Future<Output = Result<StopBuildOutput, SdkError<StopBuildError>>> {
        builder.send_with(&self.0)
    }
    fn stop_build_batch(&self, builder: StopBuildBatchInputBuilder) -> impl Future<Output = Result<StopBuildBatchOutput, SdkError<StopBuildBatchError>>> {
        builder.send_with(&self.0)
    }
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn update_project_visibility(&self, builder: UpdateProjectVisibilityInputBuilder) -> impl Future<Output = Result<UpdateProjectVisibilityOutput, SdkError<UpdateProjectVisibilityError>>> {
        builder.send_with(&self.0)
    }
    fn update_report_group(&self, builder: UpdateReportGroupInputBuilder) -> impl Future<Output = Result<UpdateReportGroupOutput, SdkError<UpdateReportGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> CodeBuildClient for T
where T: Deref,
      T::Target: CodeBuildClient {
    fn batch_delete_builds(&self, builder: BatchDeleteBuildsInputBuilder) -> impl Future<Output = Result<BatchDeleteBuildsOutput, SdkError<BatchDeleteBuildsError>>> {
        self.deref().batch_delete_builds(builder)
    }
    fn batch_get_build_batches(&self, builder: BatchGetBuildBatchesInputBuilder) -> impl Future<Output = Result<BatchGetBuildBatchesOutput, SdkError<BatchGetBuildBatchesError>>> {
        self.deref().batch_get_build_batches(builder)
    }
    fn batch_get_builds(&self, builder: BatchGetBuildsInputBuilder) -> impl Future<Output = Result<BatchGetBuildsOutput, SdkError<BatchGetBuildsError>>> {
        self.deref().batch_get_builds(builder)
    }
    fn batch_get_fleets(&self, builder: BatchGetFleetsInputBuilder) -> impl Future<Output = Result<BatchGetFleetsOutput, SdkError<BatchGetFleetsError>>> {
        self.deref().batch_get_fleets(builder)
    }
    fn batch_get_projects(&self, builder: BatchGetProjectsInputBuilder) -> impl Future<Output = Result<BatchGetProjectsOutput, SdkError<BatchGetProjectsError>>> {
        self.deref().batch_get_projects(builder)
    }
    fn batch_get_report_groups(&self, builder: BatchGetReportGroupsInputBuilder) -> impl Future<Output = Result<BatchGetReportGroupsOutput, SdkError<BatchGetReportGroupsError>>> {
        self.deref().batch_get_report_groups(builder)
    }
    fn batch_get_reports(&self, builder: BatchGetReportsInputBuilder) -> impl Future<Output = Result<BatchGetReportsOutput, SdkError<BatchGetReportsError>>> {
        self.deref().batch_get_reports(builder)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        self.deref().create_fleet(builder)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        self.deref().create_project(builder)
    }
    fn create_report_group(&self, builder: CreateReportGroupInputBuilder) -> impl Future<Output = Result<CreateReportGroupOutput, SdkError<CreateReportGroupError>>> {
        self.deref().create_report_group(builder)
    }
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>> {
        self.deref().create_webhook(builder)
    }
    fn delete_build_batch(&self, builder: DeleteBuildBatchInputBuilder) -> impl Future<Output = Result<DeleteBuildBatchOutput, SdkError<DeleteBuildBatchError>>> {
        self.deref().delete_build_batch(builder)
    }
    fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> impl Future<Output = Result<DeleteFleetOutput, SdkError<DeleteFleetError>>> {
        self.deref().delete_fleet(builder)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        self.deref().delete_project(builder)
    }
    fn delete_report(&self, builder: DeleteReportInputBuilder) -> impl Future<Output = Result<DeleteReportOutput, SdkError<DeleteReportError>>> {
        self.deref().delete_report(builder)
    }
    fn delete_report_group(&self, builder: DeleteReportGroupInputBuilder) -> impl Future<Output = Result<DeleteReportGroupOutput, SdkError<DeleteReportGroupError>>> {
        self.deref().delete_report_group(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_source_credentials(&self, builder: DeleteSourceCredentialsInputBuilder) -> impl Future<Output = Result<DeleteSourceCredentialsOutput, SdkError<DeleteSourceCredentialsError>>> {
        self.deref().delete_source_credentials(builder)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        self.deref().delete_webhook(builder)
    }
    fn describe_code_coverages(&self, builder: DescribeCodeCoveragesInputBuilder) -> impl Future<Output = Result<DescribeCodeCoveragesOutput, SdkError<DescribeCodeCoveragesError>>> {
        self.deref().describe_code_coverages(builder)
    }
    fn describe_test_cases(&self, builder: DescribeTestCasesInputBuilder) -> impl Future<Output = Result<DescribeTestCasesOutput, SdkError<DescribeTestCasesError>>> {
        self.deref().describe_test_cases(builder)
    }
    fn get_report_group_trend(&self, builder: GetReportGroupTrendInputBuilder) -> impl Future<Output = Result<GetReportGroupTrendOutput, SdkError<GetReportGroupTrendError>>> {
        self.deref().get_report_group_trend(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        self.deref().get_resource_policy(builder)
    }
    fn import_source_credentials(&self, builder: ImportSourceCredentialsInputBuilder) -> impl Future<Output = Result<ImportSourceCredentialsOutput, SdkError<ImportSourceCredentialsError>>> {
        self.deref().import_source_credentials(builder)
    }
    fn invalidate_project_cache(&self, builder: InvalidateProjectCacheInputBuilder) -> impl Future<Output = Result<InvalidateProjectCacheOutput, SdkError<InvalidateProjectCacheError>>> {
        self.deref().invalidate_project_cache(builder)
    }
    fn list_build_batches(&self, builder: ListBuildBatchesInputBuilder) -> impl Future<Output = Result<ListBuildBatchesOutput, SdkError<ListBuildBatchesError>>> {
        self.deref().list_build_batches(builder)
    }
    fn list_build_batches_for_project(&self, builder: ListBuildBatchesForProjectInputBuilder) -> impl Future<Output = Result<ListBuildBatchesForProjectOutput, SdkError<ListBuildBatchesForProjectError>>> {
        self.deref().list_build_batches_for_project(builder)
    }
    fn list_builds(&self, builder: ListBuildsInputBuilder) -> impl Future<Output = Result<ListBuildsOutput, SdkError<ListBuildsError>>> {
        self.deref().list_builds(builder)
    }
    fn list_builds_for_project(&self, builder: ListBuildsForProjectInputBuilder) -> impl Future<Output = Result<ListBuildsForProjectOutput, SdkError<ListBuildsForProjectError>>> {
        self.deref().list_builds_for_project(builder)
    }
    fn list_curated_environment_images(&self, builder: ListCuratedEnvironmentImagesInputBuilder) -> impl Future<Output = Result<ListCuratedEnvironmentImagesOutput, SdkError<ListCuratedEnvironmentImagesError>>> {
        self.deref().list_curated_environment_images(builder)
    }
    fn list_fleets(&self, builder: ListFleetsInputBuilder) -> impl Future<Output = Result<ListFleetsOutput, SdkError<ListFleetsError>>> {
        self.deref().list_fleets(builder)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        self.deref().list_projects(builder)
    }
    fn list_report_groups(&self, builder: ListReportGroupsInputBuilder) -> impl Future<Output = Result<ListReportGroupsOutput, SdkError<ListReportGroupsError>>> {
        self.deref().list_report_groups(builder)
    }
    fn list_reports(&self, builder: ListReportsInputBuilder) -> impl Future<Output = Result<ListReportsOutput, SdkError<ListReportsError>>> {
        self.deref().list_reports(builder)
    }
    fn list_reports_for_report_group(&self, builder: ListReportsForReportGroupInputBuilder) -> impl Future<Output = Result<ListReportsForReportGroupOutput, SdkError<ListReportsForReportGroupError>>> {
        self.deref().list_reports_for_report_group(builder)
    }
    fn list_shared_projects(&self, builder: ListSharedProjectsInputBuilder) -> impl Future<Output = Result<ListSharedProjectsOutput, SdkError<ListSharedProjectsError>>> {
        self.deref().list_shared_projects(builder)
    }
    fn list_shared_report_groups(&self, builder: ListSharedReportGroupsInputBuilder) -> impl Future<Output = Result<ListSharedReportGroupsOutput, SdkError<ListSharedReportGroupsError>>> {
        self.deref().list_shared_report_groups(builder)
    }
    fn list_source_credentials(&self, builder: ListSourceCredentialsInputBuilder) -> impl Future<Output = Result<ListSourceCredentialsOutput, SdkError<ListSourceCredentialsError>>> {
        self.deref().list_source_credentials(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn retry_build(&self, builder: RetryBuildInputBuilder) -> impl Future<Output = Result<RetryBuildOutput, SdkError<RetryBuildError>>> {
        self.deref().retry_build(builder)
    }
    fn retry_build_batch(&self, builder: RetryBuildBatchInputBuilder) -> impl Future<Output = Result<RetryBuildBatchOutput, SdkError<RetryBuildBatchError>>> {
        self.deref().retry_build_batch(builder)
    }
    fn start_build(&self, builder: StartBuildInputBuilder) -> impl Future<Output = Result<StartBuildOutput, SdkError<StartBuildError>>> {
        self.deref().start_build(builder)
    }
    fn start_build_batch(&self, builder: StartBuildBatchInputBuilder) -> impl Future<Output = Result<StartBuildBatchOutput, SdkError<StartBuildBatchError>>> {
        self.deref().start_build_batch(builder)
    }
    fn stop_build(&self, builder: StopBuildInputBuilder) -> impl Future<Output = Result<StopBuildOutput, SdkError<StopBuildError>>> {
        self.deref().stop_build(builder)
    }
    fn stop_build_batch(&self, builder: StopBuildBatchInputBuilder) -> impl Future<Output = Result<StopBuildBatchOutput, SdkError<StopBuildBatchError>>> {
        self.deref().stop_build_batch(builder)
    }
    fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> impl Future<Output = Result<UpdateFleetOutput, SdkError<UpdateFleetError>>> {
        self.deref().update_fleet(builder)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        self.deref().update_project(builder)
    }
    fn update_project_visibility(&self, builder: UpdateProjectVisibilityInputBuilder) -> impl Future<Output = Result<UpdateProjectVisibilityOutput, SdkError<UpdateProjectVisibilityError>>> {
        self.deref().update_project_visibility(builder)
    }
    fn update_report_group(&self, builder: UpdateReportGroupInputBuilder) -> impl Future<Output = Result<UpdateReportGroupOutput, SdkError<UpdateReportGroupError>>> {
        self.deref().update_report_group(builder)
    }
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>> {
        self.deref().update_webhook(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCodeBuildClient {}
    impl CodeBuildClient for edCodeBuildClient {
        async fn batch_delete_builds(&self, builder: BatchDeleteBuildsInputBuilder) -> Result<BatchDeleteBuildsOutput, SdkError<BatchDeleteBuildsError>>;
        async fn batch_get_build_batches(&self, builder: BatchGetBuildBatchesInputBuilder) -> Result<BatchGetBuildBatchesOutput, SdkError<BatchGetBuildBatchesError>>;
        async fn batch_get_builds(&self, builder: BatchGetBuildsInputBuilder) -> Result<BatchGetBuildsOutput, SdkError<BatchGetBuildsError>>;
        async fn batch_get_fleets(&self, builder: BatchGetFleetsInputBuilder) -> Result<BatchGetFleetsOutput, SdkError<BatchGetFleetsError>>;
        async fn batch_get_projects(&self, builder: BatchGetProjectsInputBuilder) -> Result<BatchGetProjectsOutput, SdkError<BatchGetProjectsError>>;
        async fn batch_get_report_groups(&self, builder: BatchGetReportGroupsInputBuilder) -> Result<BatchGetReportGroupsOutput, SdkError<BatchGetReportGroupsError>>;
        async fn batch_get_reports(&self, builder: BatchGetReportsInputBuilder) -> Result<BatchGetReportsOutput, SdkError<BatchGetReportsError>>;
        async fn create_fleet(&self, builder: CreateFleetInputBuilder) -> Result<CreateFleetOutput, SdkError<CreateFleetError>>;
        async fn create_project(&self, builder: CreateProjectInputBuilder) -> Result<CreateProjectOutput, SdkError<CreateProjectError>>;
        async fn create_report_group(&self, builder: CreateReportGroupInputBuilder) -> Result<CreateReportGroupOutput, SdkError<CreateReportGroupError>>;
        async fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> Result<CreateWebhookOutput, SdkError<CreateWebhookError>>;
        async fn delete_build_batch(&self, builder: DeleteBuildBatchInputBuilder) -> Result<DeleteBuildBatchOutput, SdkError<DeleteBuildBatchError>>;
        async fn delete_fleet(&self, builder: DeleteFleetInputBuilder) -> Result<DeleteFleetOutput, SdkError<DeleteFleetError>>;
        async fn delete_project(&self, builder: DeleteProjectInputBuilder) -> Result<DeleteProjectOutput, SdkError<DeleteProjectError>>;
        async fn delete_report(&self, builder: DeleteReportInputBuilder) -> Result<DeleteReportOutput, SdkError<DeleteReportError>>;
        async fn delete_report_group(&self, builder: DeleteReportGroupInputBuilder) -> Result<DeleteReportGroupOutput, SdkError<DeleteReportGroupError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_source_credentials(&self, builder: DeleteSourceCredentialsInputBuilder) -> Result<DeleteSourceCredentialsOutput, SdkError<DeleteSourceCredentialsError>>;
        async fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>;
        async fn describe_code_coverages(&self, builder: DescribeCodeCoveragesInputBuilder) -> Result<DescribeCodeCoveragesOutput, SdkError<DescribeCodeCoveragesError>>;
        async fn describe_test_cases(&self, builder: DescribeTestCasesInputBuilder) -> Result<DescribeTestCasesOutput, SdkError<DescribeTestCasesError>>;
        async fn get_report_group_trend(&self, builder: GetReportGroupTrendInputBuilder) -> Result<GetReportGroupTrendOutput, SdkError<GetReportGroupTrendError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn import_source_credentials(&self, builder: ImportSourceCredentialsInputBuilder) -> Result<ImportSourceCredentialsOutput, SdkError<ImportSourceCredentialsError>>;
        async fn invalidate_project_cache(&self, builder: InvalidateProjectCacheInputBuilder) -> Result<InvalidateProjectCacheOutput, SdkError<InvalidateProjectCacheError>>;
        async fn list_build_batches(&self, builder: ListBuildBatchesInputBuilder) -> Result<ListBuildBatchesOutput, SdkError<ListBuildBatchesError>>;
        async fn list_build_batches_for_project(&self, builder: ListBuildBatchesForProjectInputBuilder) -> Result<ListBuildBatchesForProjectOutput, SdkError<ListBuildBatchesForProjectError>>;
        async fn list_builds(&self, builder: ListBuildsInputBuilder) -> Result<ListBuildsOutput, SdkError<ListBuildsError>>;
        async fn list_builds_for_project(&self, builder: ListBuildsForProjectInputBuilder) -> Result<ListBuildsForProjectOutput, SdkError<ListBuildsForProjectError>>;
        async fn list_curated_environment_images(&self, builder: ListCuratedEnvironmentImagesInputBuilder) -> Result<ListCuratedEnvironmentImagesOutput, SdkError<ListCuratedEnvironmentImagesError>>;
        async fn list_fleets(&self, builder: ListFleetsInputBuilder) -> Result<ListFleetsOutput, SdkError<ListFleetsError>>;
        async fn list_projects(&self, builder: ListProjectsInputBuilder) -> Result<ListProjectsOutput, SdkError<ListProjectsError>>;
        async fn list_report_groups(&self, builder: ListReportGroupsInputBuilder) -> Result<ListReportGroupsOutput, SdkError<ListReportGroupsError>>;
        async fn list_reports(&self, builder: ListReportsInputBuilder) -> Result<ListReportsOutput, SdkError<ListReportsError>>;
        async fn list_reports_for_report_group(&self, builder: ListReportsForReportGroupInputBuilder) -> Result<ListReportsForReportGroupOutput, SdkError<ListReportsForReportGroupError>>;
        async fn list_shared_projects(&self, builder: ListSharedProjectsInputBuilder) -> Result<ListSharedProjectsOutput, SdkError<ListSharedProjectsError>>;
        async fn list_shared_report_groups(&self, builder: ListSharedReportGroupsInputBuilder) -> Result<ListSharedReportGroupsOutput, SdkError<ListSharedReportGroupsError>>;
        async fn list_source_credentials(&self, builder: ListSourceCredentialsInputBuilder) -> Result<ListSourceCredentialsOutput, SdkError<ListSourceCredentialsError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn retry_build(&self, builder: RetryBuildInputBuilder) -> Result<RetryBuildOutput, SdkError<RetryBuildError>>;
        async fn retry_build_batch(&self, builder: RetryBuildBatchInputBuilder) -> Result<RetryBuildBatchOutput, SdkError<RetryBuildBatchError>>;
        async fn start_build(&self, builder: StartBuildInputBuilder) -> Result<StartBuildOutput, SdkError<StartBuildError>>;
        async fn start_build_batch(&self, builder: StartBuildBatchInputBuilder) -> Result<StartBuildBatchOutput, SdkError<StartBuildBatchError>>;
        async fn stop_build(&self, builder: StopBuildInputBuilder) -> Result<StopBuildOutput, SdkError<StopBuildError>>;
        async fn stop_build_batch(&self, builder: StopBuildBatchInputBuilder) -> Result<StopBuildBatchOutput, SdkError<StopBuildBatchError>>;
        async fn update_fleet(&self, builder: UpdateFleetInputBuilder) -> Result<UpdateFleetOutput, SdkError<UpdateFleetError>>;
        async fn update_project(&self, builder: UpdateProjectInputBuilder) -> Result<UpdateProjectOutput, SdkError<UpdateProjectError>>;
        async fn update_project_visibility(&self, builder: UpdateProjectVisibilityInputBuilder) -> Result<UpdateProjectVisibilityOutput, SdkError<UpdateProjectVisibilityError>>;
        async fn update_report_group(&self, builder: UpdateReportGroupInputBuilder) -> Result<UpdateReportGroupOutput, SdkError<UpdateReportGroupError>>;
        async fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>;
    }
}
