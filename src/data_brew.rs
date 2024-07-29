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
use aws_sdk_databrew::operation::batch_delete_recipe_version::{builders::*, *};
use aws_sdk_databrew::operation::create_dataset::{builders::*, *};
use aws_sdk_databrew::operation::create_profile_job::{builders::*, *};
use aws_sdk_databrew::operation::create_project::{builders::*, *};
use aws_sdk_databrew::operation::create_recipe::{builders::*, *};
use aws_sdk_databrew::operation::create_recipe_job::{builders::*, *};
use aws_sdk_databrew::operation::create_ruleset::{builders::*, *};
use aws_sdk_databrew::operation::create_schedule::{builders::*, *};
use aws_sdk_databrew::operation::delete_dataset::{builders::*, *};
use aws_sdk_databrew::operation::delete_job::{builders::*, *};
use aws_sdk_databrew::operation::delete_project::{builders::*, *};
use aws_sdk_databrew::operation::delete_recipe_version::{builders::*, *};
use aws_sdk_databrew::operation::delete_ruleset::{builders::*, *};
use aws_sdk_databrew::operation::delete_schedule::{builders::*, *};
use aws_sdk_databrew::operation::describe_dataset::{builders::*, *};
use aws_sdk_databrew::operation::describe_job::{builders::*, *};
use aws_sdk_databrew::operation::describe_job_run::{builders::*, *};
use aws_sdk_databrew::operation::describe_project::{builders::*, *};
use aws_sdk_databrew::operation::describe_recipe::{builders::*, *};
use aws_sdk_databrew::operation::describe_ruleset::{builders::*, *};
use aws_sdk_databrew::operation::describe_schedule::{builders::*, *};
use aws_sdk_databrew::operation::list_datasets::{builders::*, *};
use aws_sdk_databrew::operation::list_job_runs::{builders::*, *};
use aws_sdk_databrew::operation::list_jobs::{builders::*, *};
use aws_sdk_databrew::operation::list_projects::{builders::*, *};
use aws_sdk_databrew::operation::list_recipe_versions::{builders::*, *};
use aws_sdk_databrew::operation::list_recipes::{builders::*, *};
use aws_sdk_databrew::operation::list_rulesets::{builders::*, *};
use aws_sdk_databrew::operation::list_schedules::{builders::*, *};
use aws_sdk_databrew::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_databrew::operation::publish_recipe::{builders::*, *};
use aws_sdk_databrew::operation::send_project_session_action::{builders::*, *};
use aws_sdk_databrew::operation::start_job_run::{builders::*, *};
use aws_sdk_databrew::operation::start_project_session::{builders::*, *};
use aws_sdk_databrew::operation::stop_job_run::{builders::*, *};
use aws_sdk_databrew::operation::tag_resource::{builders::*, *};
use aws_sdk_databrew::operation::untag_resource::{builders::*, *};
use aws_sdk_databrew::operation::update_dataset::{builders::*, *};
use aws_sdk_databrew::operation::update_profile_job::{builders::*, *};
use aws_sdk_databrew::operation::update_project::{builders::*, *};
use aws_sdk_databrew::operation::update_recipe::{builders::*, *};
use aws_sdk_databrew::operation::update_recipe_job::{builders::*, *};
use aws_sdk_databrew::operation::update_ruleset::{builders::*, *};
use aws_sdk_databrew::operation::update_schedule::{builders::*, *};
use aws_sdk_databrew::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_databrew::Client;
use std::ops::Deref;

pub use aws_sdk_databrew::*;

pub struct DataBrewClientImpl(Client);
impl DataBrewClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait DataBrewClient {
    fn batch_delete_recipe_version(&self, builder: BatchDeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteRecipeVersionOutput, SdkError<BatchDeleteRecipeVersionError>>>;
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>>;
    fn create_profile_job(&self, builder: CreateProfileJobInputBuilder) -> impl Future<Output = Result<CreateProfileJobOutput, SdkError<CreateProfileJobError>>>;
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>>;
    fn create_recipe(&self, builder: CreateRecipeInputBuilder) -> impl Future<Output = Result<CreateRecipeOutput, SdkError<CreateRecipeError>>>;
    fn create_recipe_job(&self, builder: CreateRecipeJobInputBuilder) -> impl Future<Output = Result<CreateRecipeJobOutput, SdkError<CreateRecipeJobError>>>;
    fn create_ruleset(&self, builder: CreateRulesetInputBuilder) -> impl Future<Output = Result<CreateRulesetOutput, SdkError<CreateRulesetError>>>;
    fn create_schedule(&self, builder: CreateScheduleInputBuilder) -> impl Future<Output = Result<CreateScheduleOutput, SdkError<CreateScheduleError>>>;
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>>;
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>>;
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>>;
    fn delete_recipe_version(&self, builder: DeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<DeleteRecipeVersionOutput, SdkError<DeleteRecipeVersionError>>>;
    fn delete_ruleset(&self, builder: DeleteRulesetInputBuilder) -> impl Future<Output = Result<DeleteRulesetOutput, SdkError<DeleteRulesetError>>>;
    fn delete_schedule(&self, builder: DeleteScheduleInputBuilder) -> impl Future<Output = Result<DeleteScheduleOutput, SdkError<DeleteScheduleError>>>;
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>>;
    fn describe_job(&self, builder: DescribeJobInputBuilder) -> impl Future<Output = Result<DescribeJobOutput, SdkError<DescribeJobError>>>;
    fn describe_job_run(&self, builder: DescribeJobRunInputBuilder) -> impl Future<Output = Result<DescribeJobRunOutput, SdkError<DescribeJobRunError>>>;
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>>;
    fn describe_recipe(&self, builder: DescribeRecipeInputBuilder) -> impl Future<Output = Result<DescribeRecipeOutput, SdkError<DescribeRecipeError>>>;
    fn describe_ruleset(&self, builder: DescribeRulesetInputBuilder) -> impl Future<Output = Result<DescribeRulesetOutput, SdkError<DescribeRulesetError>>>;
    fn describe_schedule(&self, builder: DescribeScheduleInputBuilder) -> impl Future<Output = Result<DescribeScheduleOutput, SdkError<DescribeScheduleError>>>;
    fn list_datasets(&self, builder: ListDatasetsInputBuilder) -> impl Future<Output = Result<ListDatasetsOutput, SdkError<ListDatasetsError>>>;
    fn list_job_runs(&self, builder: ListJobRunsInputBuilder) -> impl Future<Output = Result<ListJobRunsOutput, SdkError<ListJobRunsError>>>;
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>>;
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>>;
    fn list_recipe_versions(&self, builder: ListRecipeVersionsInputBuilder) -> impl Future<Output = Result<ListRecipeVersionsOutput, SdkError<ListRecipeVersionsError>>>;
    fn list_recipes(&self, builder: ListRecipesInputBuilder) -> impl Future<Output = Result<ListRecipesOutput, SdkError<ListRecipesError>>>;
    fn list_rulesets(&self, builder: ListRulesetsInputBuilder) -> impl Future<Output = Result<ListRulesetsOutput, SdkError<ListRulesetsError>>>;
    fn list_schedules(&self, builder: ListSchedulesInputBuilder) -> impl Future<Output = Result<ListSchedulesOutput, SdkError<ListSchedulesError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn publish_recipe(&self, builder: PublishRecipeInputBuilder) -> impl Future<Output = Result<PublishRecipeOutput, SdkError<PublishRecipeError>>>;
    fn send_project_session_action(&self, builder: SendProjectSessionActionInputBuilder) -> impl Future<Output = Result<SendProjectSessionActionOutput, SdkError<SendProjectSessionActionError>>>;
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>>;
    fn start_project_session(&self, builder: StartProjectSessionInputBuilder) -> impl Future<Output = Result<StartProjectSessionOutput, SdkError<StartProjectSessionError>>>;
    fn stop_job_run(&self, builder: StopJobRunInputBuilder) -> impl Future<Output = Result<StopJobRunOutput, SdkError<StopJobRunError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_dataset(&self, builder: UpdateDatasetInputBuilder) -> impl Future<Output = Result<UpdateDatasetOutput, SdkError<UpdateDatasetError>>>;
    fn update_profile_job(&self, builder: UpdateProfileJobInputBuilder) -> impl Future<Output = Result<UpdateProfileJobOutput, SdkError<UpdateProfileJobError>>>;
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>>;
    fn update_recipe(&self, builder: UpdateRecipeInputBuilder) -> impl Future<Output = Result<UpdateRecipeOutput, SdkError<UpdateRecipeError>>>;
    fn update_recipe_job(&self, builder: UpdateRecipeJobInputBuilder) -> impl Future<Output = Result<UpdateRecipeJobOutput, SdkError<UpdateRecipeJobError>>>;
    fn update_ruleset(&self, builder: UpdateRulesetInputBuilder) -> impl Future<Output = Result<UpdateRulesetOutput, SdkError<UpdateRulesetError>>>;
    fn update_schedule(&self, builder: UpdateScheduleInputBuilder) -> impl Future<Output = Result<UpdateScheduleOutput, SdkError<UpdateScheduleError>>>;
}
impl DataBrewClient for DataBrewClientImpl {
    fn batch_delete_recipe_version(&self, builder: BatchDeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteRecipeVersionOutput, SdkError<BatchDeleteRecipeVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn create_profile_job(&self, builder: CreateProfileJobInputBuilder) -> impl Future<Output = Result<CreateProfileJobOutput, SdkError<CreateProfileJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn create_recipe(&self, builder: CreateRecipeInputBuilder) -> impl Future<Output = Result<CreateRecipeOutput, SdkError<CreateRecipeError>>> {
        builder.send_with(&self.0)
    }
    fn create_recipe_job(&self, builder: CreateRecipeJobInputBuilder) -> impl Future<Output = Result<CreateRecipeJobOutput, SdkError<CreateRecipeJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_ruleset(&self, builder: CreateRulesetInputBuilder) -> impl Future<Output = Result<CreateRulesetOutput, SdkError<CreateRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn create_schedule(&self, builder: CreateScheduleInputBuilder) -> impl Future<Output = Result<CreateScheduleOutput, SdkError<CreateScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_recipe_version(&self, builder: DeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<DeleteRecipeVersionOutput, SdkError<DeleteRecipeVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ruleset(&self, builder: DeleteRulesetInputBuilder) -> impl Future<Output = Result<DeleteRulesetOutput, SdkError<DeleteRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_schedule(&self, builder: DeleteScheduleInputBuilder) -> impl Future<Output = Result<DeleteScheduleOutput, SdkError<DeleteScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_job(&self, builder: DescribeJobInputBuilder) -> impl Future<Output = Result<DescribeJobOutput, SdkError<DescribeJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_job_run(&self, builder: DescribeJobRunInputBuilder) -> impl Future<Output = Result<DescribeJobRunOutput, SdkError<DescribeJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>> {
        builder.send_with(&self.0)
    }
    fn describe_recipe(&self, builder: DescribeRecipeInputBuilder) -> impl Future<Output = Result<DescribeRecipeOutput, SdkError<DescribeRecipeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ruleset(&self, builder: DescribeRulesetInputBuilder) -> impl Future<Output = Result<DescribeRulesetOutput, SdkError<DescribeRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_schedule(&self, builder: DescribeScheduleInputBuilder) -> impl Future<Output = Result<DescribeScheduleOutput, SdkError<DescribeScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn list_datasets(&self, builder: ListDatasetsInputBuilder) -> impl Future<Output = Result<ListDatasetsOutput, SdkError<ListDatasetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_job_runs(&self, builder: ListJobRunsInputBuilder) -> impl Future<Output = Result<ListJobRunsOutput, SdkError<ListJobRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn list_recipe_versions(&self, builder: ListRecipeVersionsInputBuilder) -> impl Future<Output = Result<ListRecipeVersionsOutput, SdkError<ListRecipeVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_recipes(&self, builder: ListRecipesInputBuilder) -> impl Future<Output = Result<ListRecipesOutput, SdkError<ListRecipesError>>> {
        builder.send_with(&self.0)
    }
    fn list_rulesets(&self, builder: ListRulesetsInputBuilder) -> impl Future<Output = Result<ListRulesetsOutput, SdkError<ListRulesetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_schedules(&self, builder: ListSchedulesInputBuilder) -> impl Future<Output = Result<ListSchedulesOutput, SdkError<ListSchedulesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn publish_recipe(&self, builder: PublishRecipeInputBuilder) -> impl Future<Output = Result<PublishRecipeOutput, SdkError<PublishRecipeError>>> {
        builder.send_with(&self.0)
    }
    fn send_project_session_action(&self, builder: SendProjectSessionActionInputBuilder) -> impl Future<Output = Result<SendProjectSessionActionOutput, SdkError<SendProjectSessionActionError>>> {
        builder.send_with(&self.0)
    }
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_project_session(&self, builder: StartProjectSessionInputBuilder) -> impl Future<Output = Result<StartProjectSessionOutput, SdkError<StartProjectSessionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_job_run(&self, builder: StopJobRunInputBuilder) -> impl Future<Output = Result<StopJobRunOutput, SdkError<StopJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_dataset(&self, builder: UpdateDatasetInputBuilder) -> impl Future<Output = Result<UpdateDatasetOutput, SdkError<UpdateDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn update_profile_job(&self, builder: UpdateProfileJobInputBuilder) -> impl Future<Output = Result<UpdateProfileJobOutput, SdkError<UpdateProfileJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn update_recipe(&self, builder: UpdateRecipeInputBuilder) -> impl Future<Output = Result<UpdateRecipeOutput, SdkError<UpdateRecipeError>>> {
        builder.send_with(&self.0)
    }
    fn update_recipe_job(&self, builder: UpdateRecipeJobInputBuilder) -> impl Future<Output = Result<UpdateRecipeJobOutput, SdkError<UpdateRecipeJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_ruleset(&self, builder: UpdateRulesetInputBuilder) -> impl Future<Output = Result<UpdateRulesetOutput, SdkError<UpdateRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn update_schedule(&self, builder: UpdateScheduleInputBuilder) -> impl Future<Output = Result<UpdateScheduleOutput, SdkError<UpdateScheduleError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> DataBrewClient for T
where T: Deref,
      T::Target: DataBrewClient {
    fn batch_delete_recipe_version(&self, builder: BatchDeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteRecipeVersionOutput, SdkError<BatchDeleteRecipeVersionError>>> {
        self.deref().batch_delete_recipe_version(builder)
    }
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>> {
        self.deref().create_dataset(builder)
    }
    fn create_profile_job(&self, builder: CreateProfileJobInputBuilder) -> impl Future<Output = Result<CreateProfileJobOutput, SdkError<CreateProfileJobError>>> {
        self.deref().create_profile_job(builder)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        self.deref().create_project(builder)
    }
    fn create_recipe(&self, builder: CreateRecipeInputBuilder) -> impl Future<Output = Result<CreateRecipeOutput, SdkError<CreateRecipeError>>> {
        self.deref().create_recipe(builder)
    }
    fn create_recipe_job(&self, builder: CreateRecipeJobInputBuilder) -> impl Future<Output = Result<CreateRecipeJobOutput, SdkError<CreateRecipeJobError>>> {
        self.deref().create_recipe_job(builder)
    }
    fn create_ruleset(&self, builder: CreateRulesetInputBuilder) -> impl Future<Output = Result<CreateRulesetOutput, SdkError<CreateRulesetError>>> {
        self.deref().create_ruleset(builder)
    }
    fn create_schedule(&self, builder: CreateScheduleInputBuilder) -> impl Future<Output = Result<CreateScheduleOutput, SdkError<CreateScheduleError>>> {
        self.deref().create_schedule(builder)
    }
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>> {
        self.deref().delete_dataset(builder)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        self.deref().delete_job(builder)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        self.deref().delete_project(builder)
    }
    fn delete_recipe_version(&self, builder: DeleteRecipeVersionInputBuilder) -> impl Future<Output = Result<DeleteRecipeVersionOutput, SdkError<DeleteRecipeVersionError>>> {
        self.deref().delete_recipe_version(builder)
    }
    fn delete_ruleset(&self, builder: DeleteRulesetInputBuilder) -> impl Future<Output = Result<DeleteRulesetOutput, SdkError<DeleteRulesetError>>> {
        self.deref().delete_ruleset(builder)
    }
    fn delete_schedule(&self, builder: DeleteScheduleInputBuilder) -> impl Future<Output = Result<DeleteScheduleOutput, SdkError<DeleteScheduleError>>> {
        self.deref().delete_schedule(builder)
    }
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>> {
        self.deref().describe_dataset(builder)
    }
    fn describe_job(&self, builder: DescribeJobInputBuilder) -> impl Future<Output = Result<DescribeJobOutput, SdkError<DescribeJobError>>> {
        self.deref().describe_job(builder)
    }
    fn describe_job_run(&self, builder: DescribeJobRunInputBuilder) -> impl Future<Output = Result<DescribeJobRunOutput, SdkError<DescribeJobRunError>>> {
        self.deref().describe_job_run(builder)
    }
    fn describe_project(&self, builder: DescribeProjectInputBuilder) -> impl Future<Output = Result<DescribeProjectOutput, SdkError<DescribeProjectError>>> {
        self.deref().describe_project(builder)
    }
    fn describe_recipe(&self, builder: DescribeRecipeInputBuilder) -> impl Future<Output = Result<DescribeRecipeOutput, SdkError<DescribeRecipeError>>> {
        self.deref().describe_recipe(builder)
    }
    fn describe_ruleset(&self, builder: DescribeRulesetInputBuilder) -> impl Future<Output = Result<DescribeRulesetOutput, SdkError<DescribeRulesetError>>> {
        self.deref().describe_ruleset(builder)
    }
    fn describe_schedule(&self, builder: DescribeScheduleInputBuilder) -> impl Future<Output = Result<DescribeScheduleOutput, SdkError<DescribeScheduleError>>> {
        self.deref().describe_schedule(builder)
    }
    fn list_datasets(&self, builder: ListDatasetsInputBuilder) -> impl Future<Output = Result<ListDatasetsOutput, SdkError<ListDatasetsError>>> {
        self.deref().list_datasets(builder)
    }
    fn list_job_runs(&self, builder: ListJobRunsInputBuilder) -> impl Future<Output = Result<ListJobRunsOutput, SdkError<ListJobRunsError>>> {
        self.deref().list_job_runs(builder)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        self.deref().list_jobs(builder)
    }
    fn list_projects(&self, builder: ListProjectsInputBuilder) -> impl Future<Output = Result<ListProjectsOutput, SdkError<ListProjectsError>>> {
        self.deref().list_projects(builder)
    }
    fn list_recipe_versions(&self, builder: ListRecipeVersionsInputBuilder) -> impl Future<Output = Result<ListRecipeVersionsOutput, SdkError<ListRecipeVersionsError>>> {
        self.deref().list_recipe_versions(builder)
    }
    fn list_recipes(&self, builder: ListRecipesInputBuilder) -> impl Future<Output = Result<ListRecipesOutput, SdkError<ListRecipesError>>> {
        self.deref().list_recipes(builder)
    }
    fn list_rulesets(&self, builder: ListRulesetsInputBuilder) -> impl Future<Output = Result<ListRulesetsOutput, SdkError<ListRulesetsError>>> {
        self.deref().list_rulesets(builder)
    }
    fn list_schedules(&self, builder: ListSchedulesInputBuilder) -> impl Future<Output = Result<ListSchedulesOutput, SdkError<ListSchedulesError>>> {
        self.deref().list_schedules(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn publish_recipe(&self, builder: PublishRecipeInputBuilder) -> impl Future<Output = Result<PublishRecipeOutput, SdkError<PublishRecipeError>>> {
        self.deref().publish_recipe(builder)
    }
    fn send_project_session_action(&self, builder: SendProjectSessionActionInputBuilder) -> impl Future<Output = Result<SendProjectSessionActionOutput, SdkError<SendProjectSessionActionError>>> {
        self.deref().send_project_session_action(builder)
    }
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>> {
        self.deref().start_job_run(builder)
    }
    fn start_project_session(&self, builder: StartProjectSessionInputBuilder) -> impl Future<Output = Result<StartProjectSessionOutput, SdkError<StartProjectSessionError>>> {
        self.deref().start_project_session(builder)
    }
    fn stop_job_run(&self, builder: StopJobRunInputBuilder) -> impl Future<Output = Result<StopJobRunOutput, SdkError<StopJobRunError>>> {
        self.deref().stop_job_run(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_dataset(&self, builder: UpdateDatasetInputBuilder) -> impl Future<Output = Result<UpdateDatasetOutput, SdkError<UpdateDatasetError>>> {
        self.deref().update_dataset(builder)
    }
    fn update_profile_job(&self, builder: UpdateProfileJobInputBuilder) -> impl Future<Output = Result<UpdateProfileJobOutput, SdkError<UpdateProfileJobError>>> {
        self.deref().update_profile_job(builder)
    }
    fn update_project(&self, builder: UpdateProjectInputBuilder) -> impl Future<Output = Result<UpdateProjectOutput, SdkError<UpdateProjectError>>> {
        self.deref().update_project(builder)
    }
    fn update_recipe(&self, builder: UpdateRecipeInputBuilder) -> impl Future<Output = Result<UpdateRecipeOutput, SdkError<UpdateRecipeError>>> {
        self.deref().update_recipe(builder)
    }
    fn update_recipe_job(&self, builder: UpdateRecipeJobInputBuilder) -> impl Future<Output = Result<UpdateRecipeJobOutput, SdkError<UpdateRecipeJobError>>> {
        self.deref().update_recipe_job(builder)
    }
    fn update_ruleset(&self, builder: UpdateRulesetInputBuilder) -> impl Future<Output = Result<UpdateRulesetOutput, SdkError<UpdateRulesetError>>> {
        self.deref().update_ruleset(builder)
    }
    fn update_schedule(&self, builder: UpdateScheduleInputBuilder) -> impl Future<Output = Result<UpdateScheduleOutput, SdkError<UpdateScheduleError>>> {
        self.deref().update_schedule(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edDataBrewClient {}
    impl DataBrewClient for edDataBrewClient {
        async fn batch_delete_recipe_version(&self, builder: BatchDeleteRecipeVersionInputBuilder) -> Result<BatchDeleteRecipeVersionOutput, SdkError<BatchDeleteRecipeVersionError>>;
        async fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> Result<CreateDatasetOutput, SdkError<CreateDatasetError>>;
        async fn create_profile_job(&self, builder: CreateProfileJobInputBuilder) -> Result<CreateProfileJobOutput, SdkError<CreateProfileJobError>>;
        async fn create_project(&self, builder: CreateProjectInputBuilder) -> Result<CreateProjectOutput, SdkError<CreateProjectError>>;
        async fn create_recipe(&self, builder: CreateRecipeInputBuilder) -> Result<CreateRecipeOutput, SdkError<CreateRecipeError>>;
        async fn create_recipe_job(&self, builder: CreateRecipeJobInputBuilder) -> Result<CreateRecipeJobOutput, SdkError<CreateRecipeJobError>>;
        async fn create_ruleset(&self, builder: CreateRulesetInputBuilder) -> Result<CreateRulesetOutput, SdkError<CreateRulesetError>>;
        async fn create_schedule(&self, builder: CreateScheduleInputBuilder) -> Result<CreateScheduleOutput, SdkError<CreateScheduleError>>;
        async fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>;
        async fn delete_job(&self, builder: DeleteJobInputBuilder) -> Result<DeleteJobOutput, SdkError<DeleteJobError>>;
        async fn delete_project(&self, builder: DeleteProjectInputBuilder) -> Result<DeleteProjectOutput, SdkError<DeleteProjectError>>;
        async fn delete_recipe_version(&self, builder: DeleteRecipeVersionInputBuilder) -> Result<DeleteRecipeVersionOutput, SdkError<DeleteRecipeVersionError>>;
        async fn delete_ruleset(&self, builder: DeleteRulesetInputBuilder) -> Result<DeleteRulesetOutput, SdkError<DeleteRulesetError>>;
        async fn delete_schedule(&self, builder: DeleteScheduleInputBuilder) -> Result<DeleteScheduleOutput, SdkError<DeleteScheduleError>>;
        async fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>;
        async fn describe_job(&self, builder: DescribeJobInputBuilder) -> Result<DescribeJobOutput, SdkError<DescribeJobError>>;
        async fn describe_job_run(&self, builder: DescribeJobRunInputBuilder) -> Result<DescribeJobRunOutput, SdkError<DescribeJobRunError>>;
        async fn describe_project(&self, builder: DescribeProjectInputBuilder) -> Result<DescribeProjectOutput, SdkError<DescribeProjectError>>;
        async fn describe_recipe(&self, builder: DescribeRecipeInputBuilder) -> Result<DescribeRecipeOutput, SdkError<DescribeRecipeError>>;
        async fn describe_ruleset(&self, builder: DescribeRulesetInputBuilder) -> Result<DescribeRulesetOutput, SdkError<DescribeRulesetError>>;
        async fn describe_schedule(&self, builder: DescribeScheduleInputBuilder) -> Result<DescribeScheduleOutput, SdkError<DescribeScheduleError>>;
        async fn list_datasets(&self, builder: ListDatasetsInputBuilder) -> Result<ListDatasetsOutput, SdkError<ListDatasetsError>>;
        async fn list_job_runs(&self, builder: ListJobRunsInputBuilder) -> Result<ListJobRunsOutput, SdkError<ListJobRunsError>>;
        async fn list_jobs(&self, builder: ListJobsInputBuilder) -> Result<ListJobsOutput, SdkError<ListJobsError>>;
        async fn list_projects(&self, builder: ListProjectsInputBuilder) -> Result<ListProjectsOutput, SdkError<ListProjectsError>>;
        async fn list_recipe_versions(&self, builder: ListRecipeVersionsInputBuilder) -> Result<ListRecipeVersionsOutput, SdkError<ListRecipeVersionsError>>;
        async fn list_recipes(&self, builder: ListRecipesInputBuilder) -> Result<ListRecipesOutput, SdkError<ListRecipesError>>;
        async fn list_rulesets(&self, builder: ListRulesetsInputBuilder) -> Result<ListRulesetsOutput, SdkError<ListRulesetsError>>;
        async fn list_schedules(&self, builder: ListSchedulesInputBuilder) -> Result<ListSchedulesOutput, SdkError<ListSchedulesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn publish_recipe(&self, builder: PublishRecipeInputBuilder) -> Result<PublishRecipeOutput, SdkError<PublishRecipeError>>;
        async fn send_project_session_action(&self, builder: SendProjectSessionActionInputBuilder) -> Result<SendProjectSessionActionOutput, SdkError<SendProjectSessionActionError>>;
        async fn start_job_run(&self, builder: StartJobRunInputBuilder) -> Result<StartJobRunOutput, SdkError<StartJobRunError>>;
        async fn start_project_session(&self, builder: StartProjectSessionInputBuilder) -> Result<StartProjectSessionOutput, SdkError<StartProjectSessionError>>;
        async fn stop_job_run(&self, builder: StopJobRunInputBuilder) -> Result<StopJobRunOutput, SdkError<StopJobRunError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_dataset(&self, builder: UpdateDatasetInputBuilder) -> Result<UpdateDatasetOutput, SdkError<UpdateDatasetError>>;
        async fn update_profile_job(&self, builder: UpdateProfileJobInputBuilder) -> Result<UpdateProfileJobOutput, SdkError<UpdateProfileJobError>>;
        async fn update_project(&self, builder: UpdateProjectInputBuilder) -> Result<UpdateProjectOutput, SdkError<UpdateProjectError>>;
        async fn update_recipe(&self, builder: UpdateRecipeInputBuilder) -> Result<UpdateRecipeOutput, SdkError<UpdateRecipeError>>;
        async fn update_recipe_job(&self, builder: UpdateRecipeJobInputBuilder) -> Result<UpdateRecipeJobOutput, SdkError<UpdateRecipeJobError>>;
        async fn update_ruleset(&self, builder: UpdateRulesetInputBuilder) -> Result<UpdateRulesetOutput, SdkError<UpdateRulesetError>>;
        async fn update_schedule(&self, builder: UpdateScheduleInputBuilder) -> Result<UpdateScheduleOutput, SdkError<UpdateScheduleError>>;
    }
}
