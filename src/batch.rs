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
use aws_sdk_batch::operation::cancel_job::{builders::*, *};
use aws_sdk_batch::operation::create_compute_environment::{builders::*, *};
use aws_sdk_batch::operation::create_job_queue::{builders::*, *};
use aws_sdk_batch::operation::create_scheduling_policy::{builders::*, *};
use aws_sdk_batch::operation::delete_compute_environment::{builders::*, *};
use aws_sdk_batch::operation::delete_job_queue::{builders::*, *};
use aws_sdk_batch::operation::delete_scheduling_policy::{builders::*, *};
use aws_sdk_batch::operation::deregister_job_definition::{builders::*, *};
use aws_sdk_batch::operation::describe_compute_environments::{builders::*, *};
use aws_sdk_batch::operation::describe_job_definitions::{builders::*, *};
use aws_sdk_batch::operation::describe_job_queues::{builders::*, *};
use aws_sdk_batch::operation::describe_jobs::{builders::*, *};
use aws_sdk_batch::operation::describe_scheduling_policies::{builders::*, *};
use aws_sdk_batch::operation::get_job_queue_snapshot::{builders::*, *};
use aws_sdk_batch::operation::list_jobs::{builders::*, *};
use aws_sdk_batch::operation::list_scheduling_policies::{builders::*, *};
use aws_sdk_batch::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_batch::operation::register_job_definition::{builders::*, *};
use aws_sdk_batch::operation::submit_job::{builders::*, *};
use aws_sdk_batch::operation::tag_resource::{builders::*, *};
use aws_sdk_batch::operation::terminate_job::{builders::*, *};
use aws_sdk_batch::operation::untag_resource::{builders::*, *};
use aws_sdk_batch::operation::update_compute_environment::{builders::*, *};
use aws_sdk_batch::operation::update_job_queue::{builders::*, *};
use aws_sdk_batch::operation::update_scheduling_policy::{builders::*, *};
use aws_sdk_batch::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_batch::Client;
use std::ops::Deref;

pub use aws_sdk_batch::*;

pub struct BatchClientImpl(Client);
impl BatchClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait BatchClient {
    fn cancel_job(&self, builder: CancelJobInputBuilder) -> impl Future<Output = Result<CancelJobOutput, SdkError<CancelJobError>>>;
    fn create_compute_environment(&self, builder: CreateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<CreateComputeEnvironmentOutput, SdkError<CreateComputeEnvironmentError>>>;
    fn create_job_queue(&self, builder: CreateJobQueueInputBuilder) -> impl Future<Output = Result<CreateJobQueueOutput, SdkError<CreateJobQueueError>>>;
    fn create_scheduling_policy(&self, builder: CreateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<CreateSchedulingPolicyOutput, SdkError<CreateSchedulingPolicyError>>>;
    fn delete_compute_environment(&self, builder: DeleteComputeEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteComputeEnvironmentOutput, SdkError<DeleteComputeEnvironmentError>>>;
    fn delete_job_queue(&self, builder: DeleteJobQueueInputBuilder) -> impl Future<Output = Result<DeleteJobQueueOutput, SdkError<DeleteJobQueueError>>>;
    fn delete_scheduling_policy(&self, builder: DeleteSchedulingPolicyInputBuilder) -> impl Future<Output = Result<DeleteSchedulingPolicyOutput, SdkError<DeleteSchedulingPolicyError>>>;
    fn deregister_job_definition(&self, builder: DeregisterJobDefinitionInputBuilder) -> impl Future<Output = Result<DeregisterJobDefinitionOutput, SdkError<DeregisterJobDefinitionError>>>;
    fn describe_compute_environments(&self, builder: DescribeComputeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeComputeEnvironmentsOutput, SdkError<DescribeComputeEnvironmentsError>>>;
    fn describe_job_definitions(&self, builder: DescribeJobDefinitionsInputBuilder) -> impl Future<Output = Result<DescribeJobDefinitionsOutput, SdkError<DescribeJobDefinitionsError>>>;
    fn describe_job_queues(&self, builder: DescribeJobQueuesInputBuilder) -> impl Future<Output = Result<DescribeJobQueuesOutput, SdkError<DescribeJobQueuesError>>>;
    fn describe_jobs(&self, builder: DescribeJobsInputBuilder) -> impl Future<Output = Result<DescribeJobsOutput, SdkError<DescribeJobsError>>>;
    fn describe_scheduling_policies(&self, builder: DescribeSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<DescribeSchedulingPoliciesOutput, SdkError<DescribeSchedulingPoliciesError>>>;
    fn get_job_queue_snapshot(&self, builder: GetJobQueueSnapshotInputBuilder) -> impl Future<Output = Result<GetJobQueueSnapshotOutput, SdkError<GetJobQueueSnapshotError>>>;
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>>;
    fn list_scheduling_policies(&self, builder: ListSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<ListSchedulingPoliciesOutput, SdkError<ListSchedulingPoliciesError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn register_job_definition(&self, builder: RegisterJobDefinitionInputBuilder) -> impl Future<Output = Result<RegisterJobDefinitionOutput, SdkError<RegisterJobDefinitionError>>>;
    fn submit_job(&self, builder: SubmitJobInputBuilder) -> impl Future<Output = Result<SubmitJobOutput, SdkError<SubmitJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn terminate_job(&self, builder: TerminateJobInputBuilder) -> impl Future<Output = Result<TerminateJobOutput, SdkError<TerminateJobError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_compute_environment(&self, builder: UpdateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateComputeEnvironmentOutput, SdkError<UpdateComputeEnvironmentError>>>;
    fn update_job_queue(&self, builder: UpdateJobQueueInputBuilder) -> impl Future<Output = Result<UpdateJobQueueOutput, SdkError<UpdateJobQueueError>>>;
    fn update_scheduling_policy(&self, builder: UpdateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<UpdateSchedulingPolicyOutput, SdkError<UpdateSchedulingPolicyError>>>;
}
impl BatchClient for BatchClientImpl {
    fn cancel_job(&self, builder: CancelJobInputBuilder) -> impl Future<Output = Result<CancelJobOutput, SdkError<CancelJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_compute_environment(&self, builder: CreateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<CreateComputeEnvironmentOutput, SdkError<CreateComputeEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_job_queue(&self, builder: CreateJobQueueInputBuilder) -> impl Future<Output = Result<CreateJobQueueOutput, SdkError<CreateJobQueueError>>> {
        builder.send_with(&self.0)
    }
    fn create_scheduling_policy(&self, builder: CreateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<CreateSchedulingPolicyOutput, SdkError<CreateSchedulingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_compute_environment(&self, builder: DeleteComputeEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteComputeEnvironmentOutput, SdkError<DeleteComputeEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_job_queue(&self, builder: DeleteJobQueueInputBuilder) -> impl Future<Output = Result<DeleteJobQueueOutput, SdkError<DeleteJobQueueError>>> {
        builder.send_with(&self.0)
    }
    fn delete_scheduling_policy(&self, builder: DeleteSchedulingPolicyInputBuilder) -> impl Future<Output = Result<DeleteSchedulingPolicyOutput, SdkError<DeleteSchedulingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_job_definition(&self, builder: DeregisterJobDefinitionInputBuilder) -> impl Future<Output = Result<DeregisterJobDefinitionOutput, SdkError<DeregisterJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_compute_environments(&self, builder: DescribeComputeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeComputeEnvironmentsOutput, SdkError<DescribeComputeEnvironmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_job_definitions(&self, builder: DescribeJobDefinitionsInputBuilder) -> impl Future<Output = Result<DescribeJobDefinitionsOutput, SdkError<DescribeJobDefinitionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_job_queues(&self, builder: DescribeJobQueuesInputBuilder) -> impl Future<Output = Result<DescribeJobQueuesOutput, SdkError<DescribeJobQueuesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_jobs(&self, builder: DescribeJobsInputBuilder) -> impl Future<Output = Result<DescribeJobsOutput, SdkError<DescribeJobsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_scheduling_policies(&self, builder: DescribeSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<DescribeSchedulingPoliciesOutput, SdkError<DescribeSchedulingPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn get_job_queue_snapshot(&self, builder: GetJobQueueSnapshotInputBuilder) -> impl Future<Output = Result<GetJobQueueSnapshotOutput, SdkError<GetJobQueueSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_scheduling_policies(&self, builder: ListSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<ListSchedulingPoliciesOutput, SdkError<ListSchedulingPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn register_job_definition(&self, builder: RegisterJobDefinitionInputBuilder) -> impl Future<Output = Result<RegisterJobDefinitionOutput, SdkError<RegisterJobDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn submit_job(&self, builder: SubmitJobInputBuilder) -> impl Future<Output = Result<SubmitJobOutput, SdkError<SubmitJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_job(&self, builder: TerminateJobInputBuilder) -> impl Future<Output = Result<TerminateJobOutput, SdkError<TerminateJobError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_compute_environment(&self, builder: UpdateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateComputeEnvironmentOutput, SdkError<UpdateComputeEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_job_queue(&self, builder: UpdateJobQueueInputBuilder) -> impl Future<Output = Result<UpdateJobQueueOutput, SdkError<UpdateJobQueueError>>> {
        builder.send_with(&self.0)
    }
    fn update_scheduling_policy(&self, builder: UpdateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<UpdateSchedulingPolicyOutput, SdkError<UpdateSchedulingPolicyError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> BatchClient for T
where T: Deref,
      T::Target: BatchClient {
    fn cancel_job(&self, builder: CancelJobInputBuilder) -> impl Future<Output = Result<CancelJobOutput, SdkError<CancelJobError>>> {
        self.deref().cancel_job(builder)
    }
    fn create_compute_environment(&self, builder: CreateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<CreateComputeEnvironmentOutput, SdkError<CreateComputeEnvironmentError>>> {
        self.deref().create_compute_environment(builder)
    }
    fn create_job_queue(&self, builder: CreateJobQueueInputBuilder) -> impl Future<Output = Result<CreateJobQueueOutput, SdkError<CreateJobQueueError>>> {
        self.deref().create_job_queue(builder)
    }
    fn create_scheduling_policy(&self, builder: CreateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<CreateSchedulingPolicyOutput, SdkError<CreateSchedulingPolicyError>>> {
        self.deref().create_scheduling_policy(builder)
    }
    fn delete_compute_environment(&self, builder: DeleteComputeEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteComputeEnvironmentOutput, SdkError<DeleteComputeEnvironmentError>>> {
        self.deref().delete_compute_environment(builder)
    }
    fn delete_job_queue(&self, builder: DeleteJobQueueInputBuilder) -> impl Future<Output = Result<DeleteJobQueueOutput, SdkError<DeleteJobQueueError>>> {
        self.deref().delete_job_queue(builder)
    }
    fn delete_scheduling_policy(&self, builder: DeleteSchedulingPolicyInputBuilder) -> impl Future<Output = Result<DeleteSchedulingPolicyOutput, SdkError<DeleteSchedulingPolicyError>>> {
        self.deref().delete_scheduling_policy(builder)
    }
    fn deregister_job_definition(&self, builder: DeregisterJobDefinitionInputBuilder) -> impl Future<Output = Result<DeregisterJobDefinitionOutput, SdkError<DeregisterJobDefinitionError>>> {
        self.deref().deregister_job_definition(builder)
    }
    fn describe_compute_environments(&self, builder: DescribeComputeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeComputeEnvironmentsOutput, SdkError<DescribeComputeEnvironmentsError>>> {
        self.deref().describe_compute_environments(builder)
    }
    fn describe_job_definitions(&self, builder: DescribeJobDefinitionsInputBuilder) -> impl Future<Output = Result<DescribeJobDefinitionsOutput, SdkError<DescribeJobDefinitionsError>>> {
        self.deref().describe_job_definitions(builder)
    }
    fn describe_job_queues(&self, builder: DescribeJobQueuesInputBuilder) -> impl Future<Output = Result<DescribeJobQueuesOutput, SdkError<DescribeJobQueuesError>>> {
        self.deref().describe_job_queues(builder)
    }
    fn describe_jobs(&self, builder: DescribeJobsInputBuilder) -> impl Future<Output = Result<DescribeJobsOutput, SdkError<DescribeJobsError>>> {
        self.deref().describe_jobs(builder)
    }
    fn describe_scheduling_policies(&self, builder: DescribeSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<DescribeSchedulingPoliciesOutput, SdkError<DescribeSchedulingPoliciesError>>> {
        self.deref().describe_scheduling_policies(builder)
    }
    fn get_job_queue_snapshot(&self, builder: GetJobQueueSnapshotInputBuilder) -> impl Future<Output = Result<GetJobQueueSnapshotOutput, SdkError<GetJobQueueSnapshotError>>> {
        self.deref().get_job_queue_snapshot(builder)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        self.deref().list_jobs(builder)
    }
    fn list_scheduling_policies(&self, builder: ListSchedulingPoliciesInputBuilder) -> impl Future<Output = Result<ListSchedulingPoliciesOutput, SdkError<ListSchedulingPoliciesError>>> {
        self.deref().list_scheduling_policies(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn register_job_definition(&self, builder: RegisterJobDefinitionInputBuilder) -> impl Future<Output = Result<RegisterJobDefinitionOutput, SdkError<RegisterJobDefinitionError>>> {
        self.deref().register_job_definition(builder)
    }
    fn submit_job(&self, builder: SubmitJobInputBuilder) -> impl Future<Output = Result<SubmitJobOutput, SdkError<SubmitJobError>>> {
        self.deref().submit_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn terminate_job(&self, builder: TerminateJobInputBuilder) -> impl Future<Output = Result<TerminateJobOutput, SdkError<TerminateJobError>>> {
        self.deref().terminate_job(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_compute_environment(&self, builder: UpdateComputeEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateComputeEnvironmentOutput, SdkError<UpdateComputeEnvironmentError>>> {
        self.deref().update_compute_environment(builder)
    }
    fn update_job_queue(&self, builder: UpdateJobQueueInputBuilder) -> impl Future<Output = Result<UpdateJobQueueOutput, SdkError<UpdateJobQueueError>>> {
        self.deref().update_job_queue(builder)
    }
    fn update_scheduling_policy(&self, builder: UpdateSchedulingPolicyInputBuilder) -> impl Future<Output = Result<UpdateSchedulingPolicyOutput, SdkError<UpdateSchedulingPolicyError>>> {
        self.deref().update_scheduling_policy(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edBatchClient {}
    impl BatchClient for edBatchClient {
        async fn cancel_job(&self, builder: CancelJobInputBuilder) -> Result<CancelJobOutput, SdkError<CancelJobError>>;
        async fn create_compute_environment(&self, builder: CreateComputeEnvironmentInputBuilder) -> Result<CreateComputeEnvironmentOutput, SdkError<CreateComputeEnvironmentError>>;
        async fn create_job_queue(&self, builder: CreateJobQueueInputBuilder) -> Result<CreateJobQueueOutput, SdkError<CreateJobQueueError>>;
        async fn create_scheduling_policy(&self, builder: CreateSchedulingPolicyInputBuilder) -> Result<CreateSchedulingPolicyOutput, SdkError<CreateSchedulingPolicyError>>;
        async fn delete_compute_environment(&self, builder: DeleteComputeEnvironmentInputBuilder) -> Result<DeleteComputeEnvironmentOutput, SdkError<DeleteComputeEnvironmentError>>;
        async fn delete_job_queue(&self, builder: DeleteJobQueueInputBuilder) -> Result<DeleteJobQueueOutput, SdkError<DeleteJobQueueError>>;
        async fn delete_scheduling_policy(&self, builder: DeleteSchedulingPolicyInputBuilder) -> Result<DeleteSchedulingPolicyOutput, SdkError<DeleteSchedulingPolicyError>>;
        async fn deregister_job_definition(&self, builder: DeregisterJobDefinitionInputBuilder) -> Result<DeregisterJobDefinitionOutput, SdkError<DeregisterJobDefinitionError>>;
        async fn describe_compute_environments(&self, builder: DescribeComputeEnvironmentsInputBuilder) -> Result<DescribeComputeEnvironmentsOutput, SdkError<DescribeComputeEnvironmentsError>>;
        async fn describe_job_definitions(&self, builder: DescribeJobDefinitionsInputBuilder) -> Result<DescribeJobDefinitionsOutput, SdkError<DescribeJobDefinitionsError>>;
        async fn describe_job_queues(&self, builder: DescribeJobQueuesInputBuilder) -> Result<DescribeJobQueuesOutput, SdkError<DescribeJobQueuesError>>;
        async fn describe_jobs(&self, builder: DescribeJobsInputBuilder) -> Result<DescribeJobsOutput, SdkError<DescribeJobsError>>;
        async fn describe_scheduling_policies(&self, builder: DescribeSchedulingPoliciesInputBuilder) -> Result<DescribeSchedulingPoliciesOutput, SdkError<DescribeSchedulingPoliciesError>>;
        async fn get_job_queue_snapshot(&self, builder: GetJobQueueSnapshotInputBuilder) -> Result<GetJobQueueSnapshotOutput, SdkError<GetJobQueueSnapshotError>>;
        async fn list_jobs(&self, builder: ListJobsInputBuilder) -> Result<ListJobsOutput, SdkError<ListJobsError>>;
        async fn list_scheduling_policies(&self, builder: ListSchedulingPoliciesInputBuilder) -> Result<ListSchedulingPoliciesOutput, SdkError<ListSchedulingPoliciesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn register_job_definition(&self, builder: RegisterJobDefinitionInputBuilder) -> Result<RegisterJobDefinitionOutput, SdkError<RegisterJobDefinitionError>>;
        async fn submit_job(&self, builder: SubmitJobInputBuilder) -> Result<SubmitJobOutput, SdkError<SubmitJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn terminate_job(&self, builder: TerminateJobInputBuilder) -> Result<TerminateJobOutput, SdkError<TerminateJobError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_compute_environment(&self, builder: UpdateComputeEnvironmentInputBuilder) -> Result<UpdateComputeEnvironmentOutput, SdkError<UpdateComputeEnvironmentError>>;
        async fn update_job_queue(&self, builder: UpdateJobQueueInputBuilder) -> Result<UpdateJobQueueOutput, SdkError<UpdateJobQueueError>>;
        async fn update_scheduling_policy(&self, builder: UpdateSchedulingPolicyInputBuilder) -> Result<UpdateSchedulingPolicyOutput, SdkError<UpdateSchedulingPolicyError>>;
    }
}
