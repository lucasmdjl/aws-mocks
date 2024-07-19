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
use aws_sdk_datapipeline::operation::activate_pipeline::{builders::*, *};
use aws_sdk_datapipeline::operation::add_tags::{builders::*, *};
use aws_sdk_datapipeline::operation::create_pipeline::{builders::*, *};
use aws_sdk_datapipeline::operation::deactivate_pipeline::{builders::*, *};
use aws_sdk_datapipeline::operation::delete_pipeline::{builders::*, *};
use aws_sdk_datapipeline::operation::describe_objects::{builders::*, *};
use aws_sdk_datapipeline::operation::describe_pipelines::{builders::*, *};
use aws_sdk_datapipeline::operation::evaluate_expression::{builders::*, *};
use aws_sdk_datapipeline::operation::get_pipeline_definition::{builders::*, *};
use aws_sdk_datapipeline::operation::list_pipelines::{builders::*, *};
use aws_sdk_datapipeline::operation::poll_for_task::{builders::*, *};
use aws_sdk_datapipeline::operation::put_pipeline_definition::{builders::*, *};
use aws_sdk_datapipeline::operation::query_objects::{builders::*, *};
use aws_sdk_datapipeline::operation::remove_tags::{builders::*, *};
use aws_sdk_datapipeline::operation::report_task_progress::{builders::*, *};
use aws_sdk_datapipeline::operation::report_task_runner_heartbeat::{builders::*, *};
use aws_sdk_datapipeline::operation::set_status::{builders::*, *};
use aws_sdk_datapipeline::operation::set_task_status::{builders::*, *};
use aws_sdk_datapipeline::operation::validate_pipeline_definition::{builders::*, *};
use aws_sdk_datapipeline::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_datapipeline::Client;

pub use aws_sdk_datapipeline::*;

pub struct DataPipelineClientImpl(Client);
impl DataPipelineClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait DataPipelineClient {
    fn activate_pipeline(&self, builder: ActivatePipelineInputBuilder) -> impl Future<Output = Result<ActivatePipelineOutput, SdkError<ActivatePipelineError>>>;
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>>;
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>>;
    fn deactivate_pipeline(&self, builder: DeactivatePipelineInputBuilder) -> impl Future<Output = Result<DeactivatePipelineOutput, SdkError<DeactivatePipelineError>>>;
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>>;
    fn describe_objects(&self, builder: DescribeObjectsInputBuilder) -> impl Future<Output = Result<DescribeObjectsOutput, SdkError<DescribeObjectsError>>>;
    fn describe_pipelines(&self, builder: DescribePipelinesInputBuilder) -> impl Future<Output = Result<DescribePipelinesOutput, SdkError<DescribePipelinesError>>>;
    fn evaluate_expression(&self, builder: EvaluateExpressionInputBuilder) -> impl Future<Output = Result<EvaluateExpressionOutput, SdkError<EvaluateExpressionError>>>;
    fn get_pipeline_definition(&self, builder: GetPipelineDefinitionInputBuilder) -> impl Future<Output = Result<GetPipelineDefinitionOutput, SdkError<GetPipelineDefinitionError>>>;
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>>;
    fn poll_for_task(&self, builder: PollForTaskInputBuilder) -> impl Future<Output = Result<PollForTaskOutput, SdkError<PollForTaskError>>>;
    fn put_pipeline_definition(&self, builder: PutPipelineDefinitionInputBuilder) -> impl Future<Output = Result<PutPipelineDefinitionOutput, SdkError<PutPipelineDefinitionError>>>;
    fn query_objects(&self, builder: QueryObjectsInputBuilder) -> impl Future<Output = Result<QueryObjectsOutput, SdkError<QueryObjectsError>>>;
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>>;
    fn report_task_progress(&self, builder: ReportTaskProgressInputBuilder) -> impl Future<Output = Result<ReportTaskProgressOutput, SdkError<ReportTaskProgressError>>>;
    fn report_task_runner_heartbeat(&self, builder: ReportTaskRunnerHeartbeatInputBuilder) -> impl Future<Output = Result<ReportTaskRunnerHeartbeatOutput, SdkError<ReportTaskRunnerHeartbeatError>>>;
    fn set_status(&self, builder: SetStatusInputBuilder) -> impl Future<Output = Result<SetStatusOutput, SdkError<SetStatusError>>>;
    fn set_task_status(&self, builder: SetTaskStatusInputBuilder) -> impl Future<Output = Result<SetTaskStatusOutput, SdkError<SetTaskStatusError>>>;
    fn validate_pipeline_definition(&self, builder: ValidatePipelineDefinitionInputBuilder) -> impl Future<Output = Result<ValidatePipelineDefinitionOutput, SdkError<ValidatePipelineDefinitionError>>>;
}
impl DataPipelineClient for DataPipelineClientImpl {
    fn activate_pipeline(&self, builder: ActivatePipelineInputBuilder) -> impl Future<Output = Result<ActivatePipelineOutput, SdkError<ActivatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        builder.send_with(&self.0)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn deactivate_pipeline(&self, builder: DeactivatePipelineInputBuilder) -> impl Future<Output = Result<DeactivatePipelineOutput, SdkError<DeactivatePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        builder.send_with(&self.0)
    }
    fn describe_objects(&self, builder: DescribeObjectsInputBuilder) -> impl Future<Output = Result<DescribeObjectsOutput, SdkError<DescribeObjectsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pipelines(&self, builder: DescribePipelinesInputBuilder) -> impl Future<Output = Result<DescribePipelinesOutput, SdkError<DescribePipelinesError>>> {
        builder.send_with(&self.0)
    }
    fn evaluate_expression(&self, builder: EvaluateExpressionInputBuilder) -> impl Future<Output = Result<EvaluateExpressionOutput, SdkError<EvaluateExpressionError>>> {
        builder.send_with(&self.0)
    }
    fn get_pipeline_definition(&self, builder: GetPipelineDefinitionInputBuilder) -> impl Future<Output = Result<GetPipelineDefinitionOutput, SdkError<GetPipelineDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        builder.send_with(&self.0)
    }
    fn poll_for_task(&self, builder: PollForTaskInputBuilder) -> impl Future<Output = Result<PollForTaskOutput, SdkError<PollForTaskError>>> {
        builder.send_with(&self.0)
    }
    fn put_pipeline_definition(&self, builder: PutPipelineDefinitionInputBuilder) -> impl Future<Output = Result<PutPipelineDefinitionOutput, SdkError<PutPipelineDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn query_objects(&self, builder: QueryObjectsInputBuilder) -> impl Future<Output = Result<QueryObjectsOutput, SdkError<QueryObjectsError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        builder.send_with(&self.0)
    }
    fn report_task_progress(&self, builder: ReportTaskProgressInputBuilder) -> impl Future<Output = Result<ReportTaskProgressOutput, SdkError<ReportTaskProgressError>>> {
        builder.send_with(&self.0)
    }
    fn report_task_runner_heartbeat(&self, builder: ReportTaskRunnerHeartbeatInputBuilder) -> impl Future<Output = Result<ReportTaskRunnerHeartbeatOutput, SdkError<ReportTaskRunnerHeartbeatError>>> {
        builder.send_with(&self.0)
    }
    fn set_status(&self, builder: SetStatusInputBuilder) -> impl Future<Output = Result<SetStatusOutput, SdkError<SetStatusError>>> {
        builder.send_with(&self.0)
    }
    fn set_task_status(&self, builder: SetTaskStatusInputBuilder) -> impl Future<Output = Result<SetTaskStatusOutput, SdkError<SetTaskStatusError>>> {
        builder.send_with(&self.0)
    }
    fn validate_pipeline_definition(&self, builder: ValidatePipelineDefinitionInputBuilder) -> impl Future<Output = Result<ValidatePipelineDefinitionOutput, SdkError<ValidatePipelineDefinitionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: DataPipelineClient> DataPipelineClient for &T {
    fn activate_pipeline(&self, builder: ActivatePipelineInputBuilder) -> impl Future<Output = Result<ActivatePipelineOutput, SdkError<ActivatePipelineError>>> {
        (*self).activate_pipeline(builder)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        (*self).add_tags(builder)
    }
    fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> impl Future<Output = Result<CreatePipelineOutput, SdkError<CreatePipelineError>>> {
        (*self).create_pipeline(builder)
    }
    fn deactivate_pipeline(&self, builder: DeactivatePipelineInputBuilder) -> impl Future<Output = Result<DeactivatePipelineOutput, SdkError<DeactivatePipelineError>>> {
        (*self).deactivate_pipeline(builder)
    }
    fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> impl Future<Output = Result<DeletePipelineOutput, SdkError<DeletePipelineError>>> {
        (*self).delete_pipeline(builder)
    }
    fn describe_objects(&self, builder: DescribeObjectsInputBuilder) -> impl Future<Output = Result<DescribeObjectsOutput, SdkError<DescribeObjectsError>>> {
        (*self).describe_objects(builder)
    }
    fn describe_pipelines(&self, builder: DescribePipelinesInputBuilder) -> impl Future<Output = Result<DescribePipelinesOutput, SdkError<DescribePipelinesError>>> {
        (*self).describe_pipelines(builder)
    }
    fn evaluate_expression(&self, builder: EvaluateExpressionInputBuilder) -> impl Future<Output = Result<EvaluateExpressionOutput, SdkError<EvaluateExpressionError>>> {
        (*self).evaluate_expression(builder)
    }
    fn get_pipeline_definition(&self, builder: GetPipelineDefinitionInputBuilder) -> impl Future<Output = Result<GetPipelineDefinitionOutput, SdkError<GetPipelineDefinitionError>>> {
        (*self).get_pipeline_definition(builder)
    }
    fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> impl Future<Output = Result<ListPipelinesOutput, SdkError<ListPipelinesError>>> {
        (*self).list_pipelines(builder)
    }
    fn poll_for_task(&self, builder: PollForTaskInputBuilder) -> impl Future<Output = Result<PollForTaskOutput, SdkError<PollForTaskError>>> {
        (*self).poll_for_task(builder)
    }
    fn put_pipeline_definition(&self, builder: PutPipelineDefinitionInputBuilder) -> impl Future<Output = Result<PutPipelineDefinitionOutput, SdkError<PutPipelineDefinitionError>>> {
        (*self).put_pipeline_definition(builder)
    }
    fn query_objects(&self, builder: QueryObjectsInputBuilder) -> impl Future<Output = Result<QueryObjectsOutput, SdkError<QueryObjectsError>>> {
        (*self).query_objects(builder)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        (*self).remove_tags(builder)
    }
    fn report_task_progress(&self, builder: ReportTaskProgressInputBuilder) -> impl Future<Output = Result<ReportTaskProgressOutput, SdkError<ReportTaskProgressError>>> {
        (*self).report_task_progress(builder)
    }
    fn report_task_runner_heartbeat(&self, builder: ReportTaskRunnerHeartbeatInputBuilder) -> impl Future<Output = Result<ReportTaskRunnerHeartbeatOutput, SdkError<ReportTaskRunnerHeartbeatError>>> {
        (*self).report_task_runner_heartbeat(builder)
    }
    fn set_status(&self, builder: SetStatusInputBuilder) -> impl Future<Output = Result<SetStatusOutput, SdkError<SetStatusError>>> {
        (*self).set_status(builder)
    }
    fn set_task_status(&self, builder: SetTaskStatusInputBuilder) -> impl Future<Output = Result<SetTaskStatusOutput, SdkError<SetTaskStatusError>>> {
        (*self).set_task_status(builder)
    }
    fn validate_pipeline_definition(&self, builder: ValidatePipelineDefinitionInputBuilder) -> impl Future<Output = Result<ValidatePipelineDefinitionOutput, SdkError<ValidatePipelineDefinitionError>>> {
        (*self).validate_pipeline_definition(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edDataPipelineClient {}
    impl DataPipelineClient for edDataPipelineClient {
        async fn activate_pipeline(&self, builder: ActivatePipelineInputBuilder) -> Result<ActivatePipelineOutput, SdkError<ActivatePipelineError>>;
        async fn add_tags(&self, builder: AddTagsInputBuilder) -> Result<AddTagsOutput, SdkError<AddTagsError>>;
        async fn create_pipeline(&self, builder: CreatePipelineInputBuilder) -> Result<CreatePipelineOutput, SdkError<CreatePipelineError>>;
        async fn deactivate_pipeline(&self, builder: DeactivatePipelineInputBuilder) -> Result<DeactivatePipelineOutput, SdkError<DeactivatePipelineError>>;
        async fn delete_pipeline(&self, builder: DeletePipelineInputBuilder) -> Result<DeletePipelineOutput, SdkError<DeletePipelineError>>;
        async fn describe_objects(&self, builder: DescribeObjectsInputBuilder) -> Result<DescribeObjectsOutput, SdkError<DescribeObjectsError>>;
        async fn describe_pipelines(&self, builder: DescribePipelinesInputBuilder) -> Result<DescribePipelinesOutput, SdkError<DescribePipelinesError>>;
        async fn evaluate_expression(&self, builder: EvaluateExpressionInputBuilder) -> Result<EvaluateExpressionOutput, SdkError<EvaluateExpressionError>>;
        async fn get_pipeline_definition(&self, builder: GetPipelineDefinitionInputBuilder) -> Result<GetPipelineDefinitionOutput, SdkError<GetPipelineDefinitionError>>;
        async fn list_pipelines(&self, builder: ListPipelinesInputBuilder) -> Result<ListPipelinesOutput, SdkError<ListPipelinesError>>;
        async fn poll_for_task(&self, builder: PollForTaskInputBuilder) -> Result<PollForTaskOutput, SdkError<PollForTaskError>>;
        async fn put_pipeline_definition(&self, builder: PutPipelineDefinitionInputBuilder) -> Result<PutPipelineDefinitionOutput, SdkError<PutPipelineDefinitionError>>;
        async fn query_objects(&self, builder: QueryObjectsInputBuilder) -> Result<QueryObjectsOutput, SdkError<QueryObjectsError>>;
        async fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> Result<RemoveTagsOutput, SdkError<RemoveTagsError>>;
        async fn report_task_progress(&self, builder: ReportTaskProgressInputBuilder) -> Result<ReportTaskProgressOutput, SdkError<ReportTaskProgressError>>;
        async fn report_task_runner_heartbeat(&self, builder: ReportTaskRunnerHeartbeatInputBuilder) -> Result<ReportTaskRunnerHeartbeatOutput, SdkError<ReportTaskRunnerHeartbeatError>>;
        async fn set_status(&self, builder: SetStatusInputBuilder) -> Result<SetStatusOutput, SdkError<SetStatusError>>;
        async fn set_task_status(&self, builder: SetTaskStatusInputBuilder) -> Result<SetTaskStatusOutput, SdkError<SetTaskStatusError>>;
        async fn validate_pipeline_definition(&self, builder: ValidatePipelineDefinitionInputBuilder) -> Result<ValidatePipelineDefinitionOutput, SdkError<ValidatePipelineDefinitionError>>;
    }
}
