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
use aws_sdk_amplify::operation::create_app::{builders::*, *};
use aws_sdk_amplify::operation::create_backend_environment::{builders::*, *};
use aws_sdk_amplify::operation::create_branch::{builders::*, *};
use aws_sdk_amplify::operation::create_deployment::{builders::*, *};
use aws_sdk_amplify::operation::create_domain_association::{builders::*, *};
use aws_sdk_amplify::operation::create_webhook::{builders::*, *};
use aws_sdk_amplify::operation::delete_app::{builders::*, *};
use aws_sdk_amplify::operation::delete_backend_environment::{builders::*, *};
use aws_sdk_amplify::operation::delete_branch::{builders::*, *};
use aws_sdk_amplify::operation::delete_domain_association::{builders::*, *};
use aws_sdk_amplify::operation::delete_job::{builders::*, *};
use aws_sdk_amplify::operation::delete_webhook::{builders::*, *};
use aws_sdk_amplify::operation::generate_access_logs::{builders::*, *};
use aws_sdk_amplify::operation::get_app::{builders::*, *};
use aws_sdk_amplify::operation::get_artifact_url::{builders::*, *};
use aws_sdk_amplify::operation::get_backend_environment::{builders::*, *};
use aws_sdk_amplify::operation::get_branch::{builders::*, *};
use aws_sdk_amplify::operation::get_domain_association::{builders::*, *};
use aws_sdk_amplify::operation::get_job::{builders::*, *};
use aws_sdk_amplify::operation::get_webhook::{builders::*, *};
use aws_sdk_amplify::operation::list_apps::{builders::*, *};
use aws_sdk_amplify::operation::list_artifacts::{builders::*, *};
use aws_sdk_amplify::operation::list_backend_environments::{builders::*, *};
use aws_sdk_amplify::operation::list_branches::{builders::*, *};
use aws_sdk_amplify::operation::list_domain_associations::{builders::*, *};
use aws_sdk_amplify::operation::list_jobs::{builders::*, *};
use aws_sdk_amplify::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_amplify::operation::list_webhooks::{builders::*, *};
use aws_sdk_amplify::operation::start_deployment::{builders::*, *};
use aws_sdk_amplify::operation::start_job::{builders::*, *};
use aws_sdk_amplify::operation::stop_job::{builders::*, *};
use aws_sdk_amplify::operation::tag_resource::{builders::*, *};
use aws_sdk_amplify::operation::untag_resource::{builders::*, *};
use aws_sdk_amplify::operation::update_app::{builders::*, *};
use aws_sdk_amplify::operation::update_branch::{builders::*, *};
use aws_sdk_amplify::operation::update_domain_association::{builders::*, *};
use aws_sdk_amplify::operation::update_webhook::{builders::*, *};
use aws_sdk_amplify::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_amplify::Client;
use std::ops::Deref;

pub use aws_sdk_amplify::*;

pub struct AmplifyClientImpl(Client);
impl AmplifyClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AmplifyClient {
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>>;
    fn create_backend_environment(&self, builder: CreateBackendEnvironmentInputBuilder) -> impl Future<Output = Result<CreateBackendEnvironmentOutput, SdkError<CreateBackendEnvironmentError>>>;
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>>;
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>>;
    fn create_domain_association(&self, builder: CreateDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateDomainAssociationOutput, SdkError<CreateDomainAssociationError>>>;
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>>;
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>>;
    fn delete_backend_environment(&self, builder: DeleteBackendEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteBackendEnvironmentOutput, SdkError<DeleteBackendEnvironmentError>>>;
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>>;
    fn delete_domain_association(&self, builder: DeleteDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteDomainAssociationOutput, SdkError<DeleteDomainAssociationError>>>;
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>>;
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>>;
    fn generate_access_logs(&self, builder: GenerateAccessLogsInputBuilder) -> impl Future<Output = Result<GenerateAccessLogsOutput, SdkError<GenerateAccessLogsError>>>;
    fn get_app(&self, builder: GetAppInputBuilder) -> impl Future<Output = Result<GetAppOutput, SdkError<GetAppError>>>;
    fn get_artifact_url(&self, builder: GetArtifactUrlInputBuilder) -> impl Future<Output = Result<GetArtifactUrlOutput, SdkError<GetArtifactUrlError>>>;
    fn get_backend_environment(&self, builder: GetBackendEnvironmentInputBuilder) -> impl Future<Output = Result<GetBackendEnvironmentOutput, SdkError<GetBackendEnvironmentError>>>;
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>>;
    fn get_domain_association(&self, builder: GetDomainAssociationInputBuilder) -> impl Future<Output = Result<GetDomainAssociationOutput, SdkError<GetDomainAssociationError>>>;
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>>;
    fn get_webhook(&self, builder: GetWebhookInputBuilder) -> impl Future<Output = Result<GetWebhookOutput, SdkError<GetWebhookError>>>;
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>>;
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>>;
    fn list_backend_environments(&self, builder: ListBackendEnvironmentsInputBuilder) -> impl Future<Output = Result<ListBackendEnvironmentsOutput, SdkError<ListBackendEnvironmentsError>>>;
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>>;
    fn list_domain_associations(&self, builder: ListDomainAssociationsInputBuilder) -> impl Future<Output = Result<ListDomainAssociationsOutput, SdkError<ListDomainAssociationsError>>>;
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>>;
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>>;
    fn start_job(&self, builder: StartJobInputBuilder) -> impl Future<Output = Result<StartJobOutput, SdkError<StartJobError>>>;
    fn stop_job(&self, builder: StopJobInputBuilder) -> impl Future<Output = Result<StopJobOutput, SdkError<StopJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_app(&self, builder: UpdateAppInputBuilder) -> impl Future<Output = Result<UpdateAppOutput, SdkError<UpdateAppError>>>;
    fn update_branch(&self, builder: UpdateBranchInputBuilder) -> impl Future<Output = Result<UpdateBranchOutput, SdkError<UpdateBranchError>>>;
    fn update_domain_association(&self, builder: UpdateDomainAssociationInputBuilder) -> impl Future<Output = Result<UpdateDomainAssociationOutput, SdkError<UpdateDomainAssociationError>>>;
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>>;
}
impl AmplifyClient for AmplifyClientImpl {
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>> {
        builder.send_with(&self.0)
    }
    fn create_backend_environment(&self, builder: CreateBackendEnvironmentInputBuilder) -> impl Future<Output = Result<CreateBackendEnvironmentOutput, SdkError<CreateBackendEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>> {
        builder.send_with(&self.0)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain_association(&self, builder: CreateDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateDomainAssociationOutput, SdkError<CreateDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backend_environment(&self, builder: DeleteBackendEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteBackendEnvironmentOutput, SdkError<DeleteBackendEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain_association(&self, builder: DeleteDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteDomainAssociationOutput, SdkError<DeleteDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn generate_access_logs(&self, builder: GenerateAccessLogsInputBuilder) -> impl Future<Output = Result<GenerateAccessLogsOutput, SdkError<GenerateAccessLogsError>>> {
        builder.send_with(&self.0)
    }
    fn get_app(&self, builder: GetAppInputBuilder) -> impl Future<Output = Result<GetAppOutput, SdkError<GetAppError>>> {
        builder.send_with(&self.0)
    }
    fn get_artifact_url(&self, builder: GetArtifactUrlInputBuilder) -> impl Future<Output = Result<GetArtifactUrlOutput, SdkError<GetArtifactUrlError>>> {
        builder.send_with(&self.0)
    }
    fn get_backend_environment(&self, builder: GetBackendEnvironmentInputBuilder) -> impl Future<Output = Result<GetBackendEnvironmentOutput, SdkError<GetBackendEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_association(&self, builder: GetDomainAssociationInputBuilder) -> impl Future<Output = Result<GetDomainAssociationOutput, SdkError<GetDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_webhook(&self, builder: GetWebhookInputBuilder) -> impl Future<Output = Result<GetWebhookOutput, SdkError<GetWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>> {
        builder.send_with(&self.0)
    }
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>> {
        builder.send_with(&self.0)
    }
    fn list_backend_environments(&self, builder: ListBackendEnvironmentsInputBuilder) -> impl Future<Output = Result<ListBackendEnvironmentsOutput, SdkError<ListBackendEnvironmentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domain_associations(&self, builder: ListDomainAssociationsInputBuilder) -> impl Future<Output = Result<ListDomainAssociationsOutput, SdkError<ListDomainAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>> {
        builder.send_with(&self.0)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn start_job(&self, builder: StartJobInputBuilder) -> impl Future<Output = Result<StartJobOutput, SdkError<StartJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_job(&self, builder: StopJobInputBuilder) -> impl Future<Output = Result<StopJobOutput, SdkError<StopJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_app(&self, builder: UpdateAppInputBuilder) -> impl Future<Output = Result<UpdateAppOutput, SdkError<UpdateAppError>>> {
        builder.send_with(&self.0)
    }
    fn update_branch(&self, builder: UpdateBranchInputBuilder) -> impl Future<Output = Result<UpdateBranchOutput, SdkError<UpdateBranchError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain_association(&self, builder: UpdateDomainAssociationInputBuilder) -> impl Future<Output = Result<UpdateDomainAssociationOutput, SdkError<UpdateDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AmplifyClient for T
where T: Deref,
      T::Target: AmplifyClient {
    fn create_app(&self, builder: CreateAppInputBuilder) -> impl Future<Output = Result<CreateAppOutput, SdkError<CreateAppError>>> {
        self.deref().create_app(builder)
    }
    fn create_backend_environment(&self, builder: CreateBackendEnvironmentInputBuilder) -> impl Future<Output = Result<CreateBackendEnvironmentOutput, SdkError<CreateBackendEnvironmentError>>> {
        self.deref().create_backend_environment(builder)
    }
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>> {
        self.deref().create_branch(builder)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        self.deref().create_deployment(builder)
    }
    fn create_domain_association(&self, builder: CreateDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateDomainAssociationOutput, SdkError<CreateDomainAssociationError>>> {
        self.deref().create_domain_association(builder)
    }
    fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> impl Future<Output = Result<CreateWebhookOutput, SdkError<CreateWebhookError>>> {
        self.deref().create_webhook(builder)
    }
    fn delete_app(&self, builder: DeleteAppInputBuilder) -> impl Future<Output = Result<DeleteAppOutput, SdkError<DeleteAppError>>> {
        self.deref().delete_app(builder)
    }
    fn delete_backend_environment(&self, builder: DeleteBackendEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteBackendEnvironmentOutput, SdkError<DeleteBackendEnvironmentError>>> {
        self.deref().delete_backend_environment(builder)
    }
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>> {
        self.deref().delete_branch(builder)
    }
    fn delete_domain_association(&self, builder: DeleteDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteDomainAssociationOutput, SdkError<DeleteDomainAssociationError>>> {
        self.deref().delete_domain_association(builder)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        self.deref().delete_job(builder)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        self.deref().delete_webhook(builder)
    }
    fn generate_access_logs(&self, builder: GenerateAccessLogsInputBuilder) -> impl Future<Output = Result<GenerateAccessLogsOutput, SdkError<GenerateAccessLogsError>>> {
        self.deref().generate_access_logs(builder)
    }
    fn get_app(&self, builder: GetAppInputBuilder) -> impl Future<Output = Result<GetAppOutput, SdkError<GetAppError>>> {
        self.deref().get_app(builder)
    }
    fn get_artifact_url(&self, builder: GetArtifactUrlInputBuilder) -> impl Future<Output = Result<GetArtifactUrlOutput, SdkError<GetArtifactUrlError>>> {
        self.deref().get_artifact_url(builder)
    }
    fn get_backend_environment(&self, builder: GetBackendEnvironmentInputBuilder) -> impl Future<Output = Result<GetBackendEnvironmentOutput, SdkError<GetBackendEnvironmentError>>> {
        self.deref().get_backend_environment(builder)
    }
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>> {
        self.deref().get_branch(builder)
    }
    fn get_domain_association(&self, builder: GetDomainAssociationInputBuilder) -> impl Future<Output = Result<GetDomainAssociationOutput, SdkError<GetDomainAssociationError>>> {
        self.deref().get_domain_association(builder)
    }
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>> {
        self.deref().get_job(builder)
    }
    fn get_webhook(&self, builder: GetWebhookInputBuilder) -> impl Future<Output = Result<GetWebhookOutput, SdkError<GetWebhookError>>> {
        self.deref().get_webhook(builder)
    }
    fn list_apps(&self, builder: ListAppsInputBuilder) -> impl Future<Output = Result<ListAppsOutput, SdkError<ListAppsError>>> {
        self.deref().list_apps(builder)
    }
    fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> impl Future<Output = Result<ListArtifactsOutput, SdkError<ListArtifactsError>>> {
        self.deref().list_artifacts(builder)
    }
    fn list_backend_environments(&self, builder: ListBackendEnvironmentsInputBuilder) -> impl Future<Output = Result<ListBackendEnvironmentsOutput, SdkError<ListBackendEnvironmentsError>>> {
        self.deref().list_backend_environments(builder)
    }
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>> {
        self.deref().list_branches(builder)
    }
    fn list_domain_associations(&self, builder: ListDomainAssociationsInputBuilder) -> impl Future<Output = Result<ListDomainAssociationsOutput, SdkError<ListDomainAssociationsError>>> {
        self.deref().list_domain_associations(builder)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        self.deref().list_jobs(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>> {
        self.deref().list_webhooks(builder)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        self.deref().start_deployment(builder)
    }
    fn start_job(&self, builder: StartJobInputBuilder) -> impl Future<Output = Result<StartJobOutput, SdkError<StartJobError>>> {
        self.deref().start_job(builder)
    }
    fn stop_job(&self, builder: StopJobInputBuilder) -> impl Future<Output = Result<StopJobOutput, SdkError<StopJobError>>> {
        self.deref().stop_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_app(&self, builder: UpdateAppInputBuilder) -> impl Future<Output = Result<UpdateAppOutput, SdkError<UpdateAppError>>> {
        self.deref().update_app(builder)
    }
    fn update_branch(&self, builder: UpdateBranchInputBuilder) -> impl Future<Output = Result<UpdateBranchOutput, SdkError<UpdateBranchError>>> {
        self.deref().update_branch(builder)
    }
    fn update_domain_association(&self, builder: UpdateDomainAssociationInputBuilder) -> impl Future<Output = Result<UpdateDomainAssociationOutput, SdkError<UpdateDomainAssociationError>>> {
        self.deref().update_domain_association(builder)
    }
    fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> impl Future<Output = Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>> {
        self.deref().update_webhook(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAmplifyClient {}
    impl AmplifyClient for edAmplifyClient {
        async fn create_app(&self, builder: CreateAppInputBuilder) -> Result<CreateAppOutput, SdkError<CreateAppError>>;
        async fn create_backend_environment(&self, builder: CreateBackendEnvironmentInputBuilder) -> Result<CreateBackendEnvironmentOutput, SdkError<CreateBackendEnvironmentError>>;
        async fn create_branch(&self, builder: CreateBranchInputBuilder) -> Result<CreateBranchOutput, SdkError<CreateBranchError>>;
        async fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>;
        async fn create_domain_association(&self, builder: CreateDomainAssociationInputBuilder) -> Result<CreateDomainAssociationOutput, SdkError<CreateDomainAssociationError>>;
        async fn create_webhook(&self, builder: CreateWebhookInputBuilder) -> Result<CreateWebhookOutput, SdkError<CreateWebhookError>>;
        async fn delete_app(&self, builder: DeleteAppInputBuilder) -> Result<DeleteAppOutput, SdkError<DeleteAppError>>;
        async fn delete_backend_environment(&self, builder: DeleteBackendEnvironmentInputBuilder) -> Result<DeleteBackendEnvironmentOutput, SdkError<DeleteBackendEnvironmentError>>;
        async fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> Result<DeleteBranchOutput, SdkError<DeleteBranchError>>;
        async fn delete_domain_association(&self, builder: DeleteDomainAssociationInputBuilder) -> Result<DeleteDomainAssociationOutput, SdkError<DeleteDomainAssociationError>>;
        async fn delete_job(&self, builder: DeleteJobInputBuilder) -> Result<DeleteJobOutput, SdkError<DeleteJobError>>;
        async fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>;
        async fn generate_access_logs(&self, builder: GenerateAccessLogsInputBuilder) -> Result<GenerateAccessLogsOutput, SdkError<GenerateAccessLogsError>>;
        async fn get_app(&self, builder: GetAppInputBuilder) -> Result<GetAppOutput, SdkError<GetAppError>>;
        async fn get_artifact_url(&self, builder: GetArtifactUrlInputBuilder) -> Result<GetArtifactUrlOutput, SdkError<GetArtifactUrlError>>;
        async fn get_backend_environment(&self, builder: GetBackendEnvironmentInputBuilder) -> Result<GetBackendEnvironmentOutput, SdkError<GetBackendEnvironmentError>>;
        async fn get_branch(&self, builder: GetBranchInputBuilder) -> Result<GetBranchOutput, SdkError<GetBranchError>>;
        async fn get_domain_association(&self, builder: GetDomainAssociationInputBuilder) -> Result<GetDomainAssociationOutput, SdkError<GetDomainAssociationError>>;
        async fn get_job(&self, builder: GetJobInputBuilder) -> Result<GetJobOutput, SdkError<GetJobError>>;
        async fn get_webhook(&self, builder: GetWebhookInputBuilder) -> Result<GetWebhookOutput, SdkError<GetWebhookError>>;
        async fn list_apps(&self, builder: ListAppsInputBuilder) -> Result<ListAppsOutput, SdkError<ListAppsError>>;
        async fn list_artifacts(&self, builder: ListArtifactsInputBuilder) -> Result<ListArtifactsOutput, SdkError<ListArtifactsError>>;
        async fn list_backend_environments(&self, builder: ListBackendEnvironmentsInputBuilder) -> Result<ListBackendEnvironmentsOutput, SdkError<ListBackendEnvironmentsError>>;
        async fn list_branches(&self, builder: ListBranchesInputBuilder) -> Result<ListBranchesOutput, SdkError<ListBranchesError>>;
        async fn list_domain_associations(&self, builder: ListDomainAssociationsInputBuilder) -> Result<ListDomainAssociationsOutput, SdkError<ListDomainAssociationsError>>;
        async fn list_jobs(&self, builder: ListJobsInputBuilder) -> Result<ListJobsOutput, SdkError<ListJobsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> Result<ListWebhooksOutput, SdkError<ListWebhooksError>>;
        async fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> Result<StartDeploymentOutput, SdkError<StartDeploymentError>>;
        async fn start_job(&self, builder: StartJobInputBuilder) -> Result<StartJobOutput, SdkError<StartJobError>>;
        async fn stop_job(&self, builder: StopJobInputBuilder) -> Result<StopJobOutput, SdkError<StopJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_app(&self, builder: UpdateAppInputBuilder) -> Result<UpdateAppOutput, SdkError<UpdateAppError>>;
        async fn update_branch(&self, builder: UpdateBranchInputBuilder) -> Result<UpdateBranchOutput, SdkError<UpdateBranchError>>;
        async fn update_domain_association(&self, builder: UpdateDomainAssociationInputBuilder) -> Result<UpdateDomainAssociationOutput, SdkError<UpdateDomainAssociationError>>;
        async fn update_webhook(&self, builder: UpdateWebhookInputBuilder) -> Result<UpdateWebhookOutput, SdkError<UpdateWebhookError>>;
    }
}
