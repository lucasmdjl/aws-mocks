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
use aws_sdk_codepipeline::operation::acknowledge_job::{builders::*, *};
use aws_sdk_codepipeline::operation::acknowledge_third_party_job::{builders::*, *};
use aws_sdk_codepipeline::operation::create_custom_action_type::{builders::*, *};
use aws_sdk_codepipeline::operation::create_pipeline::{builders::*, *};
use aws_sdk_codepipeline::operation::delete_custom_action_type::{builders::*, *};
use aws_sdk_codepipeline::operation::delete_pipeline::{builders::*, *};
use aws_sdk_codepipeline::operation::delete_webhook::{builders::*, *};
use aws_sdk_codepipeline::operation::deregister_webhook_with_third_party::{builders::*, *};
use aws_sdk_codepipeline::operation::disable_stage_transition::{builders::*, *};
use aws_sdk_codepipeline::operation::enable_stage_transition::{builders::*, *};
use aws_sdk_codepipeline::operation::get_action_type::{builders::*, *};
use aws_sdk_codepipeline::operation::get_job_details::{builders::*, *};
use aws_sdk_codepipeline::operation::get_pipeline::{builders::*, *};
use aws_sdk_codepipeline::operation::get_pipeline_execution::{builders::*, *};
use aws_sdk_codepipeline::operation::get_pipeline_state::{builders::*, *};
use aws_sdk_codepipeline::operation::get_third_party_job_details::{builders::*, *};
use aws_sdk_codepipeline::operation::list_action_executions::{builders::*, *};
use aws_sdk_codepipeline::operation::list_action_types::{builders::*, *};
use aws_sdk_codepipeline::operation::list_pipeline_executions::{builders::*, *};
use aws_sdk_codepipeline::operation::list_pipelines::{builders::*, *};
use aws_sdk_codepipeline::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_codepipeline::operation::list_webhooks::{builders::*, *};
use aws_sdk_codepipeline::operation::poll_for_jobs::{builders::*, *};
use aws_sdk_codepipeline::operation::poll_for_third_party_jobs::{builders::*, *};
use aws_sdk_codepipeline::operation::put_action_revision::{builders::*, *};
use aws_sdk_codepipeline::operation::put_approval_result::{builders::*, *};
use aws_sdk_codepipeline::operation::put_job_failure_result::{builders::*, *};
use aws_sdk_codepipeline::operation::put_job_success_result::{builders::*, *};
use aws_sdk_codepipeline::operation::put_third_party_job_failure_result::{builders::*, *};
use aws_sdk_codepipeline::operation::put_third_party_job_success_result::{builders::*, *};
use aws_sdk_codepipeline::operation::put_webhook::{builders::*, *};
use aws_sdk_codepipeline::operation::register_webhook_with_third_party::{builders::*, *};
use aws_sdk_codepipeline::operation::retry_stage_execution::{builders::*, *};
use aws_sdk_codepipeline::operation::rollback_stage::{builders::*, *};
use aws_sdk_codepipeline::operation::start_pipeline_execution::{builders::*, *};
use aws_sdk_codepipeline::operation::stop_pipeline_execution::{builders::*, *};
use aws_sdk_codepipeline::operation::tag_resource::{builders::*, *};
use aws_sdk_codepipeline::operation::untag_resource::{builders::*, *};
use aws_sdk_codepipeline::operation::update_action_type::{builders::*, *};
use aws_sdk_codepipeline::operation::update_pipeline::{builders::*, *};
use aws_sdk_codepipeline::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_codepipeline::Client;
use std::ops::Deref;

pub use aws_sdk_codepipeline::*;

pub struct CodePipelineClientImpl(Client);
impl CodePipelineClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CodePipelineClient {
    fn acknowledge_job(&self, builder: AcknowledgeJobInputBuilder) -> impl Future<Output = Result<AcknowledgeJobOutput, SdkError<AcknowledgeJobError>>> + Send;
    fn acknowledge_third_party_job(&self, builder: AcknowledgeThirdPartyJobInputBuilder) -> impl Future<Output = Result<AcknowledgeThirdPartyJobOutput, SdkError<AcknowledgeThirdPartyJobError>>> + Send;
    fn create_custom_action_type(&self, builder: CreateCustomActionTypeInputBuilder) -> impl Future<Output = Result<CreateCustomActionTypeOutput, SdkError<CreateCustomActionTypeError>>> + Send;
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> + Send;
    fn delete_custom_action_type(&self, builder: DeleteCustomActionTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomActionTypeOutput, SdkError<DeleteCustomActionTypeError>>> + Send;
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> + Send;
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> + Send;
    fn deregister_webhook_with_third_party(&self, builder: DeregisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<DeregisterWebhookWithThirdPartyOutput, SdkError<DeregisterWebhookWithThirdPartyError>>> + Send;
    fn disable_stage_transition(&self, builder: DisableStageTransitionInputBuilder) -> impl Future<Output = Result<DisableStageTransitionOutput, SdkError<DisableStageTransitionError>>> + Send;
    fn enable_stage_transition(&self, builder: EnableStageTransitionInputBuilder) -> impl Future<Output = Result<EnableStageTransitionOutput, SdkError<EnableStageTransitionError>>> + Send;
    fn get_action_type(&self, builder: GetActionTypeInputBuilder) -> impl Future<Output = Result<GetActionTypeOutput, SdkError<GetActionTypeError>>> + Send;
    fn get_job_details(&self, builder: GetJobDetailsInputBuilder) -> impl Future<Output = Result<GetJobDetailsOutput, SdkError<GetJobDetailsError>>> + Send;
    fn get_pipeline(&self, builder: GetPipelineInputBuilder) -> impl Future<Output = Result<GetPipelineOutput, SdkError<GetPipelineError>>> + Send;
    fn get_pipeline_execution(&self, builder: GetPipelineExecutionInputBuilder) -> impl Future<Output = Result<GetPipelineExecutionOutput, SdkError<GetPipelineExecutionError>>> + Send;
    fn get_pipeline_state(&self, builder: GetPipelineStateInputBuilder) -> impl Future<Output = Result<GetPipelineStateOutput, SdkError<GetPipelineStateError>>> + Send;
    fn get_third_party_job_details(&self, builder: GetThirdPartyJobDetailsInputBuilder) -> impl Future<Output = Result<GetThirdPartyJobDetailsOutput, SdkError<GetThirdPartyJobDetailsError>>> + Send;
    fn list_action_executions(&self, builder: ListActionExecutionsInputBuilder) -> impl Future<Output = Result<ListActionExecutionsOutput, SdkError<ListActionExecutionsError>>> + Send;
    fn list_action_types(&self, builder: ListActionTypesInputBuilder) -> impl Future<Output = Result<ListActionTypesOutput, SdkError<ListActionTypesError>>> + Send;
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>> + Send;
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>> + Send;
    fn poll_for_jobs(&self, builder: PollForJobsInputBuilder) -> impl Future<Output = Result<PollForJobsOutput, SdkError<PollForJobsError>>> + Send;
    fn poll_for_third_party_jobs(&self, builder: PollForThirdPartyJobsInputBuilder) -> impl Future<Output = Result<PollForThirdPartyJobsOutput, SdkError<PollForThirdPartyJobsError>>> + Send;
    fn put_action_revision(&self, builder: PutActionRevisionInputBuilder) -> impl Future<Output = Result<PutActionRevisionOutput, SdkError<PutActionRevisionError>>> + Send;
    fn put_approval_result(&self, builder: PutApprovalResultInputBuilder) -> impl Future<Output = Result<PutApprovalResultOutput, SdkError<PutApprovalResultError>>> + Send;
    fn put_job_failure_result(&self, builder: PutJobFailureResultInputBuilder) -> impl Future<Output = Result<PutJobFailureResultOutput, SdkError<PutJobFailureResultError>>> + Send;
    fn put_job_success_result(&self, builder: PutJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutJobSuccessResultOutput, SdkError<PutJobSuccessResultError>>> + Send;
    fn put_third_party_job_failure_result(&self, builder: PutThirdPartyJobFailureResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobFailureResultOutput, SdkError<PutThirdPartyJobFailureResultError>>> + Send;
    fn put_third_party_job_success_result(&self, builder: PutThirdPartyJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobSuccessResultOutput, SdkError<PutThirdPartyJobSuccessResultError>>> + Send;
    fn put_webhook(&self, builder: PutWebhookInputBuilder) -> impl Future<Output = Result<PutWebhookOutput, SdkError<PutWebhookError>>> + Send;
    fn register_webhook_with_third_party(&self, builder: RegisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<RegisterWebhookWithThirdPartyOutput, SdkError<RegisterWebhookWithThirdPartyError>>> + Send;
    fn retry_stage_execution(&self, builder: RetryStageExecutionInputBuilder) -> impl Future<Output = Result<RetryStageExecutionOutput, SdkError<RetryStageExecutionError>>> + Send;
    fn rollback_stage(&self, builder: RollbackStageInputBuilder) -> impl Future<Output = Result<RollbackStageOutput, SdkError<RollbackStageError>>> + Send;
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>> + Send;
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_action_type(&self, builder: UpdateActionTypeInputBuilder) -> impl Future<Output = Result<UpdateActionTypeOutput, SdkError<UpdateActionTypeError>>> + Send;
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>> + Send;
}
impl CodePipelineClient for CodePipelineClientImpl {
    fn acknowledge_job(&self, builder: AcknowledgeJobInputBuilder) -> impl Future<Output = Result<AcknowledgeJobOutput, SdkError<AcknowledgeJobError>>> {
        builder.send_with(&self.0)
    }
    fn acknowledge_third_party_job(&self, builder: AcknowledgeThirdPartyJobInputBuilder) -> impl Future<Output = Result<AcknowledgeThirdPartyJobOutput, SdkError<AcknowledgeThirdPartyJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_action_type(&self, builder: CreateCustomActionTypeInputBuilder) -> impl Future<Output = Result<CreateCustomActionTypeOutput, SdkError<CreateCustomActionTypeError>>> {
        builder.send_with(&self.0)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_action_type(&self, builder: DeleteCustomActionTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomActionTypeOutput, SdkError<DeleteCustomActionTypeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_webhook_with_third_party(&self, builder: DeregisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<DeregisterWebhookWithThirdPartyOutput, SdkError<DeregisterWebhookWithThirdPartyError>>> {
        builder.send_with(&self.0)
    }
    fn disable_stage_transition(&self, builder: DisableStageTransitionInputBuilder) -> impl Future<Output = Result<DisableStageTransitionOutput, SdkError<DisableStageTransitionError>>> {
        builder.send_with(&self.0)
    }
    fn enable_stage_transition(&self, builder: EnableStageTransitionInputBuilder) -> impl Future<Output = Result<EnableStageTransitionOutput, SdkError<EnableStageTransitionError>>> {
        builder.send_with(&self.0)
    }
    fn get_action_type(&self, builder: GetActionTypeInputBuilder) -> impl Future<Output = Result<GetActionTypeOutput, SdkError<GetActionTypeError>>> {
        builder.send_with(&self.0)
    }
    fn get_job_details(&self, builder: GetJobDetailsInputBuilder) -> impl Future<Output = Result<GetJobDetailsOutput, SdkError<GetJobDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn get_pipeline(&self, builder: GetPipelineInputBuilder) -> impl Future<Output = Result<GetPipelineOutput, SdkError<GetPipelineError>>> {
        builder.send_with(&self.0)
    }
    fn get_pipeline_execution(&self, builder: GetPipelineExecutionInputBuilder) -> impl Future<Output = Result<GetPipelineExecutionOutput, SdkError<GetPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn get_pipeline_state(&self, builder: GetPipelineStateInputBuilder) -> impl Future<Output = Result<GetPipelineStateOutput, SdkError<GetPipelineStateError>>> {
        builder.send_with(&self.0)
    }
    fn get_third_party_job_details(&self, builder: GetThirdPartyJobDetailsInputBuilder) -> impl Future<Output = Result<GetThirdPartyJobDetailsOutput, SdkError<GetThirdPartyJobDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn list_action_executions(&self, builder: ListActionExecutionsInputBuilder) -> impl Future<Output = Result<ListActionExecutionsOutput, SdkError<ListActionExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_action_types(&self, builder: ListActionTypesInputBuilder) -> impl Future<Output = Result<ListActionTypesOutput, SdkError<ListActionTypesError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>> {
        builder.send_with(&self.0)
    }
    fn poll_for_jobs(&self, builder: PollForJobsInputBuilder) -> impl Future<Output = Result<PollForJobsOutput, SdkError<PollForJobsError>>> {
        builder.send_with(&self.0)
    }
    fn poll_for_third_party_jobs(&self, builder: PollForThirdPartyJobsInputBuilder) -> impl Future<Output = Result<PollForThirdPartyJobsOutput, SdkError<PollForThirdPartyJobsError>>> {
        builder.send_with(&self.0)
    }
    fn put_action_revision(&self, builder: PutActionRevisionInputBuilder) -> impl Future<Output = Result<PutActionRevisionOutput, SdkError<PutActionRevisionError>>> {
        builder.send_with(&self.0)
    }
    fn put_approval_result(&self, builder: PutApprovalResultInputBuilder) -> impl Future<Output = Result<PutApprovalResultOutput, SdkError<PutApprovalResultError>>> {
        builder.send_with(&self.0)
    }
    fn put_job_failure_result(&self, builder: PutJobFailureResultInputBuilder) -> impl Future<Output = Result<PutJobFailureResultOutput, SdkError<PutJobFailureResultError>>> {
        builder.send_with(&self.0)
    }
    fn put_job_success_result(&self, builder: PutJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutJobSuccessResultOutput, SdkError<PutJobSuccessResultError>>> {
        builder.send_with(&self.0)
    }
    fn put_third_party_job_failure_result(&self, builder: PutThirdPartyJobFailureResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobFailureResultOutput, SdkError<PutThirdPartyJobFailureResultError>>> {
        builder.send_with(&self.0)
    }
    fn put_third_party_job_success_result(&self, builder: PutThirdPartyJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobSuccessResultOutput, SdkError<PutThirdPartyJobSuccessResultError>>> {
        builder.send_with(&self.0)
    }
    fn put_webhook(&self, builder: PutWebhookInputBuilder) -> impl Future<Output = Result<PutWebhookOutput, SdkError<PutWebhookError>>> {
        builder.send_with(&self.0)
    }
    fn register_webhook_with_third_party(&self, builder: RegisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<RegisterWebhookWithThirdPartyOutput, SdkError<RegisterWebhookWithThirdPartyError>>> {
        builder.send_with(&self.0)
    }
    fn retry_stage_execution(&self, builder: RetryStageExecutionInputBuilder) -> impl Future<Output = Result<RetryStageExecutionOutput, SdkError<RetryStageExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn rollback_stage(&self, builder: RollbackStageInputBuilder) -> impl Future<Output = Result<RollbackStageOutput, SdkError<RollbackStageError>>> {
        builder.send_with(&self.0)
    }
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_action_type(&self, builder: UpdateActionTypeInputBuilder) -> impl Future<Output = Result<UpdateActionTypeOutput, SdkError<UpdateActionTypeError>>> {
        builder.send_with(&self.0)
    }
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> CodePipelineClient for T
where T: Deref,
      T::Target: CodePipelineClient {
    fn acknowledge_job(&self, builder: AcknowledgeJobInputBuilder) -> impl Future<Output = Result<AcknowledgeJobOutput, SdkError<AcknowledgeJobError>>> {
        self.deref().acknowledge_job(builder)
    }
    fn acknowledge_third_party_job(&self, builder: AcknowledgeThirdPartyJobInputBuilder) -> impl Future<Output = Result<AcknowledgeThirdPartyJobOutput, SdkError<AcknowledgeThirdPartyJobError>>> {
        self.deref().acknowledge_third_party_job(builder)
    }
    fn create_custom_action_type(&self, builder: CreateCustomActionTypeInputBuilder) -> impl Future<Output = Result<CreateCustomActionTypeOutput, SdkError<CreateCustomActionTypeError>>> {
        self.deref().create_custom_action_type(builder)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        self.deref().create_pipeline(builder)
    }
    fn delete_custom_action_type(&self, builder: DeleteCustomActionTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomActionTypeOutput, SdkError<DeleteCustomActionTypeError>>> {
        self.deref().delete_custom_action_type(builder)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        self.deref().delete_pipeline(builder)
    }
    fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> impl Future<Output = Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>> {
        self.deref().delete_webhook(builder)
    }
    fn deregister_webhook_with_third_party(&self, builder: DeregisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<DeregisterWebhookWithThirdPartyOutput, SdkError<DeregisterWebhookWithThirdPartyError>>> {
        self.deref().deregister_webhook_with_third_party(builder)
    }
    fn disable_stage_transition(&self, builder: DisableStageTransitionInputBuilder) -> impl Future<Output = Result<DisableStageTransitionOutput, SdkError<DisableStageTransitionError>>> {
        self.deref().disable_stage_transition(builder)
    }
    fn enable_stage_transition(&self, builder: EnableStageTransitionInputBuilder) -> impl Future<Output = Result<EnableStageTransitionOutput, SdkError<EnableStageTransitionError>>> {
        self.deref().enable_stage_transition(builder)
    }
    fn get_action_type(&self, builder: GetActionTypeInputBuilder) -> impl Future<Output = Result<GetActionTypeOutput, SdkError<GetActionTypeError>>> {
        self.deref().get_action_type(builder)
    }
    fn get_job_details(&self, builder: GetJobDetailsInputBuilder) -> impl Future<Output = Result<GetJobDetailsOutput, SdkError<GetJobDetailsError>>> {
        self.deref().get_job_details(builder)
    }
    fn get_pipeline(&self, builder: GetPipelineInputBuilder) -> impl Future<Output = Result<GetPipelineOutput, SdkError<GetPipelineError>>> {
        self.deref().get_pipeline(builder)
    }
    fn get_pipeline_execution(&self, builder: GetPipelineExecutionInputBuilder) -> impl Future<Output = Result<GetPipelineExecutionOutput, SdkError<GetPipelineExecutionError>>> {
        self.deref().get_pipeline_execution(builder)
    }
    fn get_pipeline_state(&self, builder: GetPipelineStateInputBuilder) -> impl Future<Output = Result<GetPipelineStateOutput, SdkError<GetPipelineStateError>>> {
        self.deref().get_pipeline_state(builder)
    }
    fn get_third_party_job_details(&self, builder: GetThirdPartyJobDetailsInputBuilder) -> impl Future<Output = Result<GetThirdPartyJobDetailsOutput, SdkError<GetThirdPartyJobDetailsError>>> {
        self.deref().get_third_party_job_details(builder)
    }
    fn list_action_executions(&self, builder: ListActionExecutionsInputBuilder) -> impl Future<Output = Result<ListActionExecutionsOutput, SdkError<ListActionExecutionsError>>> {
        self.deref().list_action_executions(builder)
    }
    fn list_action_types(&self, builder: ListActionTypesInputBuilder) -> impl Future<Output = Result<ListActionTypesOutput, SdkError<ListActionTypesError>>> {
        self.deref().list_action_types(builder)
    }
    fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> impl Future<Output = Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>> {
        self.deref().list_pipeline_executions(builder)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        self.deref().list_pipelines(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> impl Future<Output = Result<ListWebhooksOutput, SdkError<ListWebhooksError>>> {
        self.deref().list_webhooks(builder)
    }
    fn poll_for_jobs(&self, builder: PollForJobsInputBuilder) -> impl Future<Output = Result<PollForJobsOutput, SdkError<PollForJobsError>>> {
        self.deref().poll_for_jobs(builder)
    }
    fn poll_for_third_party_jobs(&self, builder: PollForThirdPartyJobsInputBuilder) -> impl Future<Output = Result<PollForThirdPartyJobsOutput, SdkError<PollForThirdPartyJobsError>>> {
        self.deref().poll_for_third_party_jobs(builder)
    }
    fn put_action_revision(&self, builder: PutActionRevisionInputBuilder) -> impl Future<Output = Result<PutActionRevisionOutput, SdkError<PutActionRevisionError>>> {
        self.deref().put_action_revision(builder)
    }
    fn put_approval_result(&self, builder: PutApprovalResultInputBuilder) -> impl Future<Output = Result<PutApprovalResultOutput, SdkError<PutApprovalResultError>>> {
        self.deref().put_approval_result(builder)
    }
    fn put_job_failure_result(&self, builder: PutJobFailureResultInputBuilder) -> impl Future<Output = Result<PutJobFailureResultOutput, SdkError<PutJobFailureResultError>>> {
        self.deref().put_job_failure_result(builder)
    }
    fn put_job_success_result(&self, builder: PutJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutJobSuccessResultOutput, SdkError<PutJobSuccessResultError>>> {
        self.deref().put_job_success_result(builder)
    }
    fn put_third_party_job_failure_result(&self, builder: PutThirdPartyJobFailureResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobFailureResultOutput, SdkError<PutThirdPartyJobFailureResultError>>> {
        self.deref().put_third_party_job_failure_result(builder)
    }
    fn put_third_party_job_success_result(&self, builder: PutThirdPartyJobSuccessResultInputBuilder) -> impl Future<Output = Result<PutThirdPartyJobSuccessResultOutput, SdkError<PutThirdPartyJobSuccessResultError>>> {
        self.deref().put_third_party_job_success_result(builder)
    }
    fn put_webhook(&self, builder: PutWebhookInputBuilder) -> impl Future<Output = Result<PutWebhookOutput, SdkError<PutWebhookError>>> {
        self.deref().put_webhook(builder)
    }
    fn register_webhook_with_third_party(&self, builder: RegisterWebhookWithThirdPartyInputBuilder) -> impl Future<Output = Result<RegisterWebhookWithThirdPartyOutput, SdkError<RegisterWebhookWithThirdPartyError>>> {
        self.deref().register_webhook_with_third_party(builder)
    }
    fn retry_stage_execution(&self, builder: RetryStageExecutionInputBuilder) -> impl Future<Output = Result<RetryStageExecutionOutput, SdkError<RetryStageExecutionError>>> {
        self.deref().retry_stage_execution(builder)
    }
    fn rollback_stage(&self, builder: RollbackStageInputBuilder) -> impl Future<Output = Result<RollbackStageOutput, SdkError<RollbackStageError>>> {
        self.deref().rollback_stage(builder)
    }
    fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> impl Future<Output = Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>> {
        self.deref().start_pipeline_execution(builder)
    }
    fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> impl Future<Output = Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>> {
        self.deref().stop_pipeline_execution(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_action_type(&self, builder: UpdateActionTypeInputBuilder) -> impl Future<Output = Result<UpdateActionTypeOutput, SdkError<UpdateActionTypeError>>> {
        self.deref().update_action_type(builder)
    }
    fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> impl Future<Output = Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>> {
        self.deref().update_pipeline(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCodePipelineClient {}
    impl CodePipelineClient for edCodePipelineClient {
        async fn acknowledge_job(&self, builder: AcknowledgeJobInputBuilder) -> Result<AcknowledgeJobOutput, SdkError<AcknowledgeJobError>>;
        async fn acknowledge_third_party_job(&self, builder: AcknowledgeThirdPartyJobInputBuilder) -> Result<AcknowledgeThirdPartyJobOutput, SdkError<AcknowledgeThirdPartyJobError>>;
        async fn create_custom_action_type(&self, builder: CreateCustomActionTypeInputBuilder) -> Result<CreateCustomActionTypeOutput, SdkError<CreateCustomActionTypeError>>;
        async fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> Result<CreatePipelineOutput, SdkError<CreatePipelineError>>;
        async fn delete_custom_action_type(&self, builder: DeleteCustomActionTypeInputBuilder) -> Result<DeleteCustomActionTypeOutput, SdkError<DeleteCustomActionTypeError>>;
        async fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> Result<DeletePipelineOutput, SdkError<DeletePipelineError>>;
        async fn delete_webhook(&self, builder: DeleteWebhookInputBuilder) -> Result<DeleteWebhookOutput, SdkError<DeleteWebhookError>>;
        async fn deregister_webhook_with_third_party(&self, builder: DeregisterWebhookWithThirdPartyInputBuilder) -> Result<DeregisterWebhookWithThirdPartyOutput, SdkError<DeregisterWebhookWithThirdPartyError>>;
        async fn disable_stage_transition(&self, builder: DisableStageTransitionInputBuilder) -> Result<DisableStageTransitionOutput, SdkError<DisableStageTransitionError>>;
        async fn enable_stage_transition(&self, builder: EnableStageTransitionInputBuilder) -> Result<EnableStageTransitionOutput, SdkError<EnableStageTransitionError>>;
        async fn get_action_type(&self, builder: GetActionTypeInputBuilder) -> Result<GetActionTypeOutput, SdkError<GetActionTypeError>>;
        async fn get_job_details(&self, builder: GetJobDetailsInputBuilder) -> Result<GetJobDetailsOutput, SdkError<GetJobDetailsError>>;
        async fn get_pipeline(&self, builder: GetPipelineInputBuilder) -> Result<GetPipelineOutput, SdkError<GetPipelineError>>;
        async fn get_pipeline_execution(&self, builder: GetPipelineExecutionInputBuilder) -> Result<GetPipelineExecutionOutput, SdkError<GetPipelineExecutionError>>;
        async fn get_pipeline_state(&self, builder: GetPipelineStateInputBuilder) -> Result<GetPipelineStateOutput, SdkError<GetPipelineStateError>>;
        async fn get_third_party_job_details(&self, builder: GetThirdPartyJobDetailsInputBuilder) -> Result<GetThirdPartyJobDetailsOutput, SdkError<GetThirdPartyJobDetailsError>>;
        async fn list_action_executions(&self, builder: ListActionExecutionsInputBuilder) -> Result<ListActionExecutionsOutput, SdkError<ListActionExecutionsError>>;
        async fn list_action_types(&self, builder: ListActionTypesInputBuilder) -> Result<ListActionTypesOutput, SdkError<ListActionTypesError>>;
        async fn list_pipeline_executions(&self, builder: ListPipelineExecutionsInputBuilder) -> Result<ListPipelineExecutionsOutput, SdkError<ListPipelineExecutionsError>>;
        async fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> Result<ListPipelinesOutput, SdkError<ListPipelinesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_webhooks(&self, builder: ListWebhooksInputBuilder) -> Result<ListWebhooksOutput, SdkError<ListWebhooksError>>;
        async fn poll_for_jobs(&self, builder: PollForJobsInputBuilder) -> Result<PollForJobsOutput, SdkError<PollForJobsError>>;
        async fn poll_for_third_party_jobs(&self, builder: PollForThirdPartyJobsInputBuilder) -> Result<PollForThirdPartyJobsOutput, SdkError<PollForThirdPartyJobsError>>;
        async fn put_action_revision(&self, builder: PutActionRevisionInputBuilder) -> Result<PutActionRevisionOutput, SdkError<PutActionRevisionError>>;
        async fn put_approval_result(&self, builder: PutApprovalResultInputBuilder) -> Result<PutApprovalResultOutput, SdkError<PutApprovalResultError>>;
        async fn put_job_failure_result(&self, builder: PutJobFailureResultInputBuilder) -> Result<PutJobFailureResultOutput, SdkError<PutJobFailureResultError>>;
        async fn put_job_success_result(&self, builder: PutJobSuccessResultInputBuilder) -> Result<PutJobSuccessResultOutput, SdkError<PutJobSuccessResultError>>;
        async fn put_third_party_job_failure_result(&self, builder: PutThirdPartyJobFailureResultInputBuilder) -> Result<PutThirdPartyJobFailureResultOutput, SdkError<PutThirdPartyJobFailureResultError>>;
        async fn put_third_party_job_success_result(&self, builder: PutThirdPartyJobSuccessResultInputBuilder) -> Result<PutThirdPartyJobSuccessResultOutput, SdkError<PutThirdPartyJobSuccessResultError>>;
        async fn put_webhook(&self, builder: PutWebhookInputBuilder) -> Result<PutWebhookOutput, SdkError<PutWebhookError>>;
        async fn register_webhook_with_third_party(&self, builder: RegisterWebhookWithThirdPartyInputBuilder) -> Result<RegisterWebhookWithThirdPartyOutput, SdkError<RegisterWebhookWithThirdPartyError>>;
        async fn retry_stage_execution(&self, builder: RetryStageExecutionInputBuilder) -> Result<RetryStageExecutionOutput, SdkError<RetryStageExecutionError>>;
        async fn rollback_stage(&self, builder: RollbackStageInputBuilder) -> Result<RollbackStageOutput, SdkError<RollbackStageError>>;
        async fn start_pipeline_execution(&self, builder: StartPipelineExecutionInputBuilder) -> Result<StartPipelineExecutionOutput, SdkError<StartPipelineExecutionError>>;
        async fn stop_pipeline_execution(&self, builder: StopPipelineExecutionInputBuilder) -> Result<StopPipelineExecutionOutput, SdkError<StopPipelineExecutionError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_action_type(&self, builder: UpdateActionTypeInputBuilder) -> Result<UpdateActionTypeOutput, SdkError<UpdateActionTypeError>>;
        async fn update_pipeline(&self, builder: UpdatePipelineInputBuilder) -> Result<UpdatePipelineOutput, SdkError<UpdatePipelineError>>;
    }
}
