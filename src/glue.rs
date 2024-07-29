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
use aws_sdk_glue::operation::batch_create_partition::{builders::*, *};
use aws_sdk_glue::operation::batch_delete_connection::{builders::*, *};
use aws_sdk_glue::operation::batch_delete_partition::{builders::*, *};
use aws_sdk_glue::operation::batch_delete_table::{builders::*, *};
use aws_sdk_glue::operation::batch_delete_table_version::{builders::*, *};
use aws_sdk_glue::operation::batch_get_blueprints::{builders::*, *};
use aws_sdk_glue::operation::batch_get_crawlers::{builders::*, *};
use aws_sdk_glue::operation::batch_get_custom_entity_types::{builders::*, *};
use aws_sdk_glue::operation::batch_get_data_quality_result::{builders::*, *};
use aws_sdk_glue::operation::batch_get_dev_endpoints::{builders::*, *};
use aws_sdk_glue::operation::batch_get_jobs::{builders::*, *};
use aws_sdk_glue::operation::batch_get_partition::{builders::*, *};
use aws_sdk_glue::operation::batch_get_table_optimizer::{builders::*, *};
use aws_sdk_glue::operation::batch_get_triggers::{builders::*, *};
use aws_sdk_glue::operation::batch_get_workflows::{builders::*, *};
use aws_sdk_glue::operation::batch_stop_job_run::{builders::*, *};
use aws_sdk_glue::operation::batch_update_partition::{builders::*, *};
use aws_sdk_glue::operation::cancel_data_quality_rule_recommendation_run::{builders::*, *};
use aws_sdk_glue::operation::cancel_data_quality_ruleset_evaluation_run::{builders::*, *};
use aws_sdk_glue::operation::cancel_ml_task_run::{builders::*, *};
use aws_sdk_glue::operation::cancel_statement::{builders::*, *};
use aws_sdk_glue::operation::check_schema_version_validity::{builders::*, *};
use aws_sdk_glue::operation::create_blueprint::{builders::*, *};
use aws_sdk_glue::operation::create_classifier::{builders::*, *};
use aws_sdk_glue::operation::create_connection::{builders::*, *};
use aws_sdk_glue::operation::create_crawler::{builders::*, *};
use aws_sdk_glue::operation::create_custom_entity_type::{builders::*, *};
use aws_sdk_glue::operation::create_data_quality_ruleset::{builders::*, *};
use aws_sdk_glue::operation::create_database::{builders::*, *};
use aws_sdk_glue::operation::create_dev_endpoint::{builders::*, *};
use aws_sdk_glue::operation::create_job::{builders::*, *};
use aws_sdk_glue::operation::create_ml_transform::{builders::*, *};
use aws_sdk_glue::operation::create_partition::{builders::*, *};
use aws_sdk_glue::operation::create_partition_index::{builders::*, *};
use aws_sdk_glue::operation::create_registry::{builders::*, *};
use aws_sdk_glue::operation::create_schema::{builders::*, *};
use aws_sdk_glue::operation::create_script::{builders::*, *};
use aws_sdk_glue::operation::create_security_configuration::{builders::*, *};
use aws_sdk_glue::operation::create_session::{builders::*, *};
use aws_sdk_glue::operation::create_table::{builders::*, *};
use aws_sdk_glue::operation::create_table_optimizer::{builders::*, *};
use aws_sdk_glue::operation::create_trigger::{builders::*, *};
use aws_sdk_glue::operation::create_usage_profile::{builders::*, *};
use aws_sdk_glue::operation::create_user_defined_function::{builders::*, *};
use aws_sdk_glue::operation::create_workflow::{builders::*, *};
use aws_sdk_glue::operation::delete_blueprint::{builders::*, *};
use aws_sdk_glue::operation::delete_classifier::{builders::*, *};
use aws_sdk_glue::operation::delete_column_statistics_for_partition::{builders::*, *};
use aws_sdk_glue::operation::delete_column_statistics_for_table::{builders::*, *};
use aws_sdk_glue::operation::delete_connection::{builders::*, *};
use aws_sdk_glue::operation::delete_crawler::{builders::*, *};
use aws_sdk_glue::operation::delete_custom_entity_type::{builders::*, *};
use aws_sdk_glue::operation::delete_data_quality_ruleset::{builders::*, *};
use aws_sdk_glue::operation::delete_database::{builders::*, *};
use aws_sdk_glue::operation::delete_dev_endpoint::{builders::*, *};
use aws_sdk_glue::operation::delete_job::{builders::*, *};
use aws_sdk_glue::operation::delete_ml_transform::{builders::*, *};
use aws_sdk_glue::operation::delete_partition::{builders::*, *};
use aws_sdk_glue::operation::delete_partition_index::{builders::*, *};
use aws_sdk_glue::operation::delete_registry::{builders::*, *};
use aws_sdk_glue::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_glue::operation::delete_schema::{builders::*, *};
use aws_sdk_glue::operation::delete_schema_versions::{builders::*, *};
use aws_sdk_glue::operation::delete_security_configuration::{builders::*, *};
use aws_sdk_glue::operation::delete_session::{builders::*, *};
use aws_sdk_glue::operation::delete_table::{builders::*, *};
use aws_sdk_glue::operation::delete_table_optimizer::{builders::*, *};
use aws_sdk_glue::operation::delete_table_version::{builders::*, *};
use aws_sdk_glue::operation::delete_trigger::{builders::*, *};
use aws_sdk_glue::operation::delete_usage_profile::{builders::*, *};
use aws_sdk_glue::operation::delete_user_defined_function::{builders::*, *};
use aws_sdk_glue::operation::delete_workflow::{builders::*, *};
use aws_sdk_glue::operation::get_blueprint::{builders::*, *};
use aws_sdk_glue::operation::get_blueprint_run::{builders::*, *};
use aws_sdk_glue::operation::get_blueprint_runs::{builders::*, *};
use aws_sdk_glue::operation::get_catalog_import_status::{builders::*, *};
use aws_sdk_glue::operation::get_classifier::{builders::*, *};
use aws_sdk_glue::operation::get_classifiers::{builders::*, *};
use aws_sdk_glue::operation::get_column_statistics_for_partition::{builders::*, *};
use aws_sdk_glue::operation::get_column_statistics_for_table::{builders::*, *};
use aws_sdk_glue::operation::get_column_statistics_task_run::{builders::*, *};
use aws_sdk_glue::operation::get_column_statistics_task_runs::{builders::*, *};
use aws_sdk_glue::operation::get_connection::{builders::*, *};
use aws_sdk_glue::operation::get_connections::{builders::*, *};
use aws_sdk_glue::operation::get_crawler::{builders::*, *};
use aws_sdk_glue::operation::get_crawler_metrics::{builders::*, *};
use aws_sdk_glue::operation::get_crawlers::{builders::*, *};
use aws_sdk_glue::operation::get_custom_entity_type::{builders::*, *};
use aws_sdk_glue::operation::get_data_catalog_encryption_settings::{builders::*, *};
use aws_sdk_glue::operation::get_data_quality_result::{builders::*, *};
use aws_sdk_glue::operation::get_data_quality_rule_recommendation_run::{builders::*, *};
use aws_sdk_glue::operation::get_data_quality_ruleset::{builders::*, *};
use aws_sdk_glue::operation::get_data_quality_ruleset_evaluation_run::{builders::*, *};
use aws_sdk_glue::operation::get_database::{builders::*, *};
use aws_sdk_glue::operation::get_databases::{builders::*, *};
use aws_sdk_glue::operation::get_dataflow_graph::{builders::*, *};
use aws_sdk_glue::operation::get_dev_endpoint::{builders::*, *};
use aws_sdk_glue::operation::get_dev_endpoints::{builders::*, *};
use aws_sdk_glue::operation::get_job::{builders::*, *};
use aws_sdk_glue::operation::get_job_bookmark::{builders::*, *};
use aws_sdk_glue::operation::get_job_run::{builders::*, *};
use aws_sdk_glue::operation::get_job_runs::{builders::*, *};
use aws_sdk_glue::operation::get_jobs::{builders::*, *};
use aws_sdk_glue::operation::get_mapping::{builders::*, *};
use aws_sdk_glue::operation::get_ml_task_run::{builders::*, *};
use aws_sdk_glue::operation::get_ml_task_runs::{builders::*, *};
use aws_sdk_glue::operation::get_ml_transform::{builders::*, *};
use aws_sdk_glue::operation::get_ml_transforms::{builders::*, *};
use aws_sdk_glue::operation::get_partition::{builders::*, *};
use aws_sdk_glue::operation::get_partition_indexes::{builders::*, *};
use aws_sdk_glue::operation::get_partitions::{builders::*, *};
use aws_sdk_glue::operation::get_plan::{builders::*, *};
use aws_sdk_glue::operation::get_registry::{builders::*, *};
use aws_sdk_glue::operation::get_resource_policies::{builders::*, *};
use aws_sdk_glue::operation::get_resource_policy::{builders::*, *};
use aws_sdk_glue::operation::get_schema::{builders::*, *};
use aws_sdk_glue::operation::get_schema_by_definition::{builders::*, *};
use aws_sdk_glue::operation::get_schema_version::{builders::*, *};
use aws_sdk_glue::operation::get_schema_versions_diff::{builders::*, *};
use aws_sdk_glue::operation::get_security_configuration::{builders::*, *};
use aws_sdk_glue::operation::get_security_configurations::{builders::*, *};
use aws_sdk_glue::operation::get_session::{builders::*, *};
use aws_sdk_glue::operation::get_statement::{builders::*, *};
use aws_sdk_glue::operation::get_table::{builders::*, *};
use aws_sdk_glue::operation::get_table_optimizer::{builders::*, *};
use aws_sdk_glue::operation::get_table_version::{builders::*, *};
use aws_sdk_glue::operation::get_table_versions::{builders::*, *};
use aws_sdk_glue::operation::get_tables::{builders::*, *};
use aws_sdk_glue::operation::get_tags::{builders::*, *};
use aws_sdk_glue::operation::get_trigger::{builders::*, *};
use aws_sdk_glue::operation::get_triggers::{builders::*, *};
use aws_sdk_glue::operation::get_unfiltered_partition_metadata::{builders::*, *};
use aws_sdk_glue::operation::get_unfiltered_partitions_metadata::{builders::*, *};
use aws_sdk_glue::operation::get_unfiltered_table_metadata::{builders::*, *};
use aws_sdk_glue::operation::get_usage_profile::{builders::*, *};
use aws_sdk_glue::operation::get_user_defined_function::{builders::*, *};
use aws_sdk_glue::operation::get_user_defined_functions::{builders::*, *};
use aws_sdk_glue::operation::get_workflow::{builders::*, *};
use aws_sdk_glue::operation::get_workflow_run::{builders::*, *};
use aws_sdk_glue::operation::get_workflow_run_properties::{builders::*, *};
use aws_sdk_glue::operation::get_workflow_runs::{builders::*, *};
use aws_sdk_glue::operation::import_catalog_to_glue::{builders::*, *};
use aws_sdk_glue::operation::list_blueprints::{builders::*, *};
use aws_sdk_glue::operation::list_column_statistics_task_runs::{builders::*, *};
use aws_sdk_glue::operation::list_crawlers::{builders::*, *};
use aws_sdk_glue::operation::list_crawls::{builders::*, *};
use aws_sdk_glue::operation::list_custom_entity_types::{builders::*, *};
use aws_sdk_glue::operation::list_data_quality_results::{builders::*, *};
use aws_sdk_glue::operation::list_data_quality_rule_recommendation_runs::{builders::*, *};
use aws_sdk_glue::operation::list_data_quality_ruleset_evaluation_runs::{builders::*, *};
use aws_sdk_glue::operation::list_data_quality_rulesets::{builders::*, *};
use aws_sdk_glue::operation::list_dev_endpoints::{builders::*, *};
use aws_sdk_glue::operation::list_jobs::{builders::*, *};
use aws_sdk_glue::operation::list_ml_transforms::{builders::*, *};
use aws_sdk_glue::operation::list_registries::{builders::*, *};
use aws_sdk_glue::operation::list_schema_versions::{builders::*, *};
use aws_sdk_glue::operation::list_schemas::{builders::*, *};
use aws_sdk_glue::operation::list_sessions::{builders::*, *};
use aws_sdk_glue::operation::list_statements::{builders::*, *};
use aws_sdk_glue::operation::list_table_optimizer_runs::{builders::*, *};
use aws_sdk_glue::operation::list_triggers::{builders::*, *};
use aws_sdk_glue::operation::list_usage_profiles::{builders::*, *};
use aws_sdk_glue::operation::list_workflows::{builders::*, *};
use aws_sdk_glue::operation::put_data_catalog_encryption_settings::{builders::*, *};
use aws_sdk_glue::operation::put_resource_policy::{builders::*, *};
use aws_sdk_glue::operation::put_schema_version_metadata::{builders::*, *};
use aws_sdk_glue::operation::put_workflow_run_properties::{builders::*, *};
use aws_sdk_glue::operation::query_schema_version_metadata::{builders::*, *};
use aws_sdk_glue::operation::register_schema_version::{builders::*, *};
use aws_sdk_glue::operation::remove_schema_version_metadata::{builders::*, *};
use aws_sdk_glue::operation::reset_job_bookmark::{builders::*, *};
use aws_sdk_glue::operation::resume_workflow_run::{builders::*, *};
use aws_sdk_glue::operation::run_statement::{builders::*, *};
use aws_sdk_glue::operation::search_tables::{builders::*, *};
use aws_sdk_glue::operation::start_blueprint_run::{builders::*, *};
use aws_sdk_glue::operation::start_column_statistics_task_run::{builders::*, *};
use aws_sdk_glue::operation::start_crawler::{builders::*, *};
use aws_sdk_glue::operation::start_crawler_schedule::{builders::*, *};
use aws_sdk_glue::operation::start_data_quality_rule_recommendation_run::{builders::*, *};
use aws_sdk_glue::operation::start_data_quality_ruleset_evaluation_run::{builders::*, *};
use aws_sdk_glue::operation::start_export_labels_task_run::{builders::*, *};
use aws_sdk_glue::operation::start_import_labels_task_run::{builders::*, *};
use aws_sdk_glue::operation::start_job_run::{builders::*, *};
use aws_sdk_glue::operation::start_ml_evaluation_task_run::{builders::*, *};
use aws_sdk_glue::operation::start_ml_labeling_set_generation_task_run::{builders::*, *};
use aws_sdk_glue::operation::start_trigger::{builders::*, *};
use aws_sdk_glue::operation::start_workflow_run::{builders::*, *};
use aws_sdk_glue::operation::stop_column_statistics_task_run::{builders::*, *};
use aws_sdk_glue::operation::stop_crawler::{builders::*, *};
use aws_sdk_glue::operation::stop_crawler_schedule::{builders::*, *};
use aws_sdk_glue::operation::stop_session::{builders::*, *};
use aws_sdk_glue::operation::stop_trigger::{builders::*, *};
use aws_sdk_glue::operation::stop_workflow_run::{builders::*, *};
use aws_sdk_glue::operation::tag_resource::{builders::*, *};
use aws_sdk_glue::operation::untag_resource::{builders::*, *};
use aws_sdk_glue::operation::update_blueprint::{builders::*, *};
use aws_sdk_glue::operation::update_classifier::{builders::*, *};
use aws_sdk_glue::operation::update_column_statistics_for_partition::{builders::*, *};
use aws_sdk_glue::operation::update_column_statistics_for_table::{builders::*, *};
use aws_sdk_glue::operation::update_connection::{builders::*, *};
use aws_sdk_glue::operation::update_crawler::{builders::*, *};
use aws_sdk_glue::operation::update_crawler_schedule::{builders::*, *};
use aws_sdk_glue::operation::update_data_quality_ruleset::{builders::*, *};
use aws_sdk_glue::operation::update_database::{builders::*, *};
use aws_sdk_glue::operation::update_dev_endpoint::{builders::*, *};
use aws_sdk_glue::operation::update_job::{builders::*, *};
use aws_sdk_glue::operation::update_job_from_source_control::{builders::*, *};
use aws_sdk_glue::operation::update_ml_transform::{builders::*, *};
use aws_sdk_glue::operation::update_partition::{builders::*, *};
use aws_sdk_glue::operation::update_registry::{builders::*, *};
use aws_sdk_glue::operation::update_schema::{builders::*, *};
use aws_sdk_glue::operation::update_source_control_from_job::{builders::*, *};
use aws_sdk_glue::operation::update_table::{builders::*, *};
use aws_sdk_glue::operation::update_table_optimizer::{builders::*, *};
use aws_sdk_glue::operation::update_trigger::{builders::*, *};
use aws_sdk_glue::operation::update_usage_profile::{builders::*, *};
use aws_sdk_glue::operation::update_user_defined_function::{builders::*, *};
use aws_sdk_glue::operation::update_workflow::{builders::*, *};
use aws_sdk_glue::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_glue::Client;
use std::ops::Deref;

pub use aws_sdk_glue::*;

pub struct GlueClientImpl(Client);
impl GlueClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait GlueClient {
    fn batch_create_partition(&self, builder: BatchCreatePartitionInputBuilder) -> impl Future<Output = Result<BatchCreatePartitionOutput, SdkError<BatchCreatePartitionError>>>;
    fn batch_delete_connection(&self, builder: BatchDeleteConnectionInputBuilder) -> impl Future<Output = Result<BatchDeleteConnectionOutput, SdkError<BatchDeleteConnectionError>>>;
    fn batch_delete_partition(&self, builder: BatchDeletePartitionInputBuilder) -> impl Future<Output = Result<BatchDeletePartitionOutput, SdkError<BatchDeletePartitionError>>>;
    fn batch_delete_table(&self, builder: BatchDeleteTableInputBuilder) -> impl Future<Output = Result<BatchDeleteTableOutput, SdkError<BatchDeleteTableError>>>;
    fn batch_delete_table_version(&self, builder: BatchDeleteTableVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteTableVersionOutput, SdkError<BatchDeleteTableVersionError>>>;
    fn batch_get_blueprints(&self, builder: BatchGetBlueprintsInputBuilder) -> impl Future<Output = Result<BatchGetBlueprintsOutput, SdkError<BatchGetBlueprintsError>>>;
    fn batch_get_crawlers(&self, builder: BatchGetCrawlersInputBuilder) -> impl Future<Output = Result<BatchGetCrawlersOutput, SdkError<BatchGetCrawlersError>>>;
    fn batch_get_custom_entity_types(&self, builder: BatchGetCustomEntityTypesInputBuilder) -> impl Future<Output = Result<BatchGetCustomEntityTypesOutput, SdkError<BatchGetCustomEntityTypesError>>>;
    fn batch_get_data_quality_result(&self, builder: BatchGetDataQualityResultInputBuilder) -> impl Future<Output = Result<BatchGetDataQualityResultOutput, SdkError<BatchGetDataQualityResultError>>>;
    fn batch_get_dev_endpoints(&self, builder: BatchGetDevEndpointsInputBuilder) -> impl Future<Output = Result<BatchGetDevEndpointsOutput, SdkError<BatchGetDevEndpointsError>>>;
    fn batch_get_jobs(&self, builder: BatchGetJobsInputBuilder) -> impl Future<Output = Result<BatchGetJobsOutput, SdkError<BatchGetJobsError>>>;
    fn batch_get_partition(&self, builder: BatchGetPartitionInputBuilder) -> impl Future<Output = Result<BatchGetPartitionOutput, SdkError<BatchGetPartitionError>>>;
    fn batch_get_table_optimizer(&self, builder: BatchGetTableOptimizerInputBuilder) -> impl Future<Output = Result<BatchGetTableOptimizerOutput, SdkError<BatchGetTableOptimizerError>>>;
    fn batch_get_triggers(&self, builder: BatchGetTriggersInputBuilder) -> impl Future<Output = Result<BatchGetTriggersOutput, SdkError<BatchGetTriggersError>>>;
    fn batch_get_workflows(&self, builder: BatchGetWorkflowsInputBuilder) -> impl Future<Output = Result<BatchGetWorkflowsOutput, SdkError<BatchGetWorkflowsError>>>;
    fn batch_stop_job_run(&self, builder: BatchStopJobRunInputBuilder) -> impl Future<Output = Result<BatchStopJobRunOutput, SdkError<BatchStopJobRunError>>>;
    fn batch_update_partition(&self, builder: BatchUpdatePartitionInputBuilder) -> impl Future<Output = Result<BatchUpdatePartitionOutput, SdkError<BatchUpdatePartitionError>>>;
    fn cancel_data_quality_rule_recommendation_run(&self, builder: CancelDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRuleRecommendationRunOutput, SdkError<CancelDataQualityRuleRecommendationRunError>>>;
    fn cancel_data_quality_ruleset_evaluation_run(&self, builder: CancelDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRulesetEvaluationRunOutput, SdkError<CancelDataQualityRulesetEvaluationRunError>>>;
    fn cancel_ml_task_run(&self, builder: CancelMlTaskRunInputBuilder) -> impl Future<Output = Result<CancelMlTaskRunOutput, SdkError<CancelMLTaskRunError>>>;
    fn cancel_statement(&self, builder: CancelStatementInputBuilder) -> impl Future<Output = Result<CancelStatementOutput, SdkError<CancelStatementError>>>;
    fn check_schema_version_validity(&self, builder: CheckSchemaVersionValidityInputBuilder) -> impl Future<Output = Result<CheckSchemaVersionValidityOutput, SdkError<CheckSchemaVersionValidityError>>>;
    fn create_blueprint(&self, builder: CreateBlueprintInputBuilder) -> impl Future<Output = Result<CreateBlueprintOutput, SdkError<CreateBlueprintError>>>;
    fn create_classifier(&self, builder: CreateClassifierInputBuilder) -> impl Future<Output = Result<CreateClassifierOutput, SdkError<CreateClassifierError>>>;
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>>;
    fn create_crawler(&self, builder: CreateCrawlerInputBuilder) -> impl Future<Output = Result<CreateCrawlerOutput, SdkError<CreateCrawlerError>>>;
    fn create_custom_entity_type(&self, builder: CreateCustomEntityTypeInputBuilder) -> impl Future<Output = Result<CreateCustomEntityTypeOutput, SdkError<CreateCustomEntityTypeError>>>;
    fn create_data_quality_ruleset(&self, builder: CreateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<CreateDataQualityRulesetOutput, SdkError<CreateDataQualityRulesetError>>>;
    fn create_database(&self, builder: CreateDatabaseInputBuilder) -> impl Future<Output = Result<CreateDatabaseOutput, SdkError<CreateDatabaseError>>>;
    fn create_dev_endpoint(&self, builder: CreateDevEndpointInputBuilder) -> impl Future<Output = Result<CreateDevEndpointOutput, SdkError<CreateDevEndpointError>>>;
    fn create_job(&self, builder: CreateJobInputBuilder) -> impl Future<Output = Result<CreateJobOutput, SdkError<CreateJobError>>>;
    fn create_ml_transform(&self, builder: CreateMlTransformInputBuilder) -> impl Future<Output = Result<CreateMlTransformOutput, SdkError<CreateMLTransformError>>>;
    fn create_partition(&self, builder: CreatePartitionInputBuilder) -> impl Future<Output = Result<CreatePartitionOutput, SdkError<CreatePartitionError>>>;
    fn create_partition_index(&self, builder: CreatePartitionIndexInputBuilder) -> impl Future<Output = Result<CreatePartitionIndexOutput, SdkError<CreatePartitionIndexError>>>;
    fn create_registry(&self, builder: CreateRegistryInputBuilder) -> impl Future<Output = Result<CreateRegistryOutput, SdkError<CreateRegistryError>>>;
    fn create_schema(&self, builder: CreateSchemaInputBuilder) -> impl Future<Output = Result<CreateSchemaOutput, SdkError<CreateSchemaError>>>;
    fn create_script(&self, builder: CreateScriptInputBuilder) -> impl Future<Output = Result<CreateScriptOutput, SdkError<CreateScriptError>>>;
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>>;
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>>;
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>>;
    fn create_table_optimizer(&self, builder: CreateTableOptimizerInputBuilder) -> impl Future<Output = Result<CreateTableOptimizerOutput, SdkError<CreateTableOptimizerError>>>;
    fn create_trigger(&self, builder: CreateTriggerInputBuilder) -> impl Future<Output = Result<CreateTriggerOutput, SdkError<CreateTriggerError>>>;
    fn create_usage_profile(&self, builder: CreateUsageProfileInputBuilder) -> impl Future<Output = Result<CreateUsageProfileOutput, SdkError<CreateUsageProfileError>>>;
    fn create_user_defined_function(&self, builder: CreateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<CreateUserDefinedFunctionOutput, SdkError<CreateUserDefinedFunctionError>>>;
    fn create_workflow(&self, builder: CreateWorkflowInputBuilder) -> impl Future<Output = Result<CreateWorkflowOutput, SdkError<CreateWorkflowError>>>;
    fn delete_blueprint(&self, builder: DeleteBlueprintInputBuilder) -> impl Future<Output = Result<DeleteBlueprintOutput, SdkError<DeleteBlueprintError>>>;
    fn delete_classifier(&self, builder: DeleteClassifierInputBuilder) -> impl Future<Output = Result<DeleteClassifierOutput, SdkError<DeleteClassifierError>>>;
    fn delete_column_statistics_for_partition(&self, builder: DeleteColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForPartitionOutput, SdkError<DeleteColumnStatisticsForPartitionError>>>;
    fn delete_column_statistics_for_table(&self, builder: DeleteColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForTableOutput, SdkError<DeleteColumnStatisticsForTableError>>>;
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>>;
    fn delete_crawler(&self, builder: DeleteCrawlerInputBuilder) -> impl Future<Output = Result<DeleteCrawlerOutput, SdkError<DeleteCrawlerError>>>;
    fn delete_custom_entity_type(&self, builder: DeleteCustomEntityTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomEntityTypeOutput, SdkError<DeleteCustomEntityTypeError>>>;
    fn delete_data_quality_ruleset(&self, builder: DeleteDataQualityRulesetInputBuilder) -> impl Future<Output = Result<DeleteDataQualityRulesetOutput, SdkError<DeleteDataQualityRulesetError>>>;
    fn delete_database(&self, builder: DeleteDatabaseInputBuilder) -> impl Future<Output = Result<DeleteDatabaseOutput, SdkError<DeleteDatabaseError>>>;
    fn delete_dev_endpoint(&self, builder: DeleteDevEndpointInputBuilder) -> impl Future<Output = Result<DeleteDevEndpointOutput, SdkError<DeleteDevEndpointError>>>;
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>>;
    fn delete_ml_transform(&self, builder: DeleteMlTransformInputBuilder) -> impl Future<Output = Result<DeleteMlTransformOutput, SdkError<DeleteMLTransformError>>>;
    fn delete_partition(&self, builder: DeletePartitionInputBuilder) -> impl Future<Output = Result<DeletePartitionOutput, SdkError<DeletePartitionError>>>;
    fn delete_partition_index(&self, builder: DeletePartitionIndexInputBuilder) -> impl Future<Output = Result<DeletePartitionIndexOutput, SdkError<DeletePartitionIndexError>>>;
    fn delete_registry(&self, builder: DeleteRegistryInputBuilder) -> impl Future<Output = Result<DeleteRegistryOutput, SdkError<DeleteRegistryError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_schema(&self, builder: DeleteSchemaInputBuilder) -> impl Future<Output = Result<DeleteSchemaOutput, SdkError<DeleteSchemaError>>>;
    fn delete_schema_versions(&self, builder: DeleteSchemaVersionsInputBuilder) -> impl Future<Output = Result<DeleteSchemaVersionsOutput, SdkError<DeleteSchemaVersionsError>>>;
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>>;
    fn delete_session(&self, builder: DeleteSessionInputBuilder) -> impl Future<Output = Result<DeleteSessionOutput, SdkError<DeleteSessionError>>>;
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>>;
    fn delete_table_optimizer(&self, builder: DeleteTableOptimizerInputBuilder) -> impl Future<Output = Result<DeleteTableOptimizerOutput, SdkError<DeleteTableOptimizerError>>>;
    fn delete_table_version(&self, builder: DeleteTableVersionInputBuilder) -> impl Future<Output = Result<DeleteTableVersionOutput, SdkError<DeleteTableVersionError>>>;
    fn delete_trigger(&self, builder: DeleteTriggerInputBuilder) -> impl Future<Output = Result<DeleteTriggerOutput, SdkError<DeleteTriggerError>>>;
    fn delete_usage_profile(&self, builder: DeleteUsageProfileInputBuilder) -> impl Future<Output = Result<DeleteUsageProfileOutput, SdkError<DeleteUsageProfileError>>>;
    fn delete_user_defined_function(&self, builder: DeleteUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<DeleteUserDefinedFunctionOutput, SdkError<DeleteUserDefinedFunctionError>>>;
    fn delete_workflow(&self, builder: DeleteWorkflowInputBuilder) -> impl Future<Output = Result<DeleteWorkflowOutput, SdkError<DeleteWorkflowError>>>;
    fn get_blueprint(&self, builder: GetBlueprintInputBuilder) -> impl Future<Output = Result<GetBlueprintOutput, SdkError<GetBlueprintError>>>;
    fn get_blueprint_run(&self, builder: GetBlueprintRunInputBuilder) -> impl Future<Output = Result<GetBlueprintRunOutput, SdkError<GetBlueprintRunError>>>;
    fn get_blueprint_runs(&self, builder: GetBlueprintRunsInputBuilder) -> impl Future<Output = Result<GetBlueprintRunsOutput, SdkError<GetBlueprintRunsError>>>;
    fn get_catalog_import_status(&self, builder: GetCatalogImportStatusInputBuilder) -> impl Future<Output = Result<GetCatalogImportStatusOutput, SdkError<GetCatalogImportStatusError>>>;
    fn get_classifier(&self, builder: GetClassifierInputBuilder) -> impl Future<Output = Result<GetClassifierOutput, SdkError<GetClassifierError>>>;
    fn get_classifiers(&self, builder: GetClassifiersInputBuilder) -> impl Future<Output = Result<GetClassifiersOutput, SdkError<GetClassifiersError>>>;
    fn get_column_statistics_for_partition(&self, builder: GetColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForPartitionOutput, SdkError<GetColumnStatisticsForPartitionError>>>;
    fn get_column_statistics_for_table(&self, builder: GetColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForTableOutput, SdkError<GetColumnStatisticsForTableError>>>;
    fn get_column_statistics_task_run(&self, builder: GetColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunOutput, SdkError<GetColumnStatisticsTaskRunError>>>;
    fn get_column_statistics_task_runs(&self, builder: GetColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunsOutput, SdkError<GetColumnStatisticsTaskRunsError>>>;
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>>;
    fn get_connections(&self, builder: GetConnectionsInputBuilder) -> impl Future<Output = Result<GetConnectionsOutput, SdkError<GetConnectionsError>>>;
    fn get_crawler(&self, builder: GetCrawlerInputBuilder) -> impl Future<Output = Result<GetCrawlerOutput, SdkError<GetCrawlerError>>>;
    fn get_crawler_metrics(&self, builder: GetCrawlerMetricsInputBuilder) -> impl Future<Output = Result<GetCrawlerMetricsOutput, SdkError<GetCrawlerMetricsError>>>;
    fn get_crawlers(&self, builder: GetCrawlersInputBuilder) -> impl Future<Output = Result<GetCrawlersOutput, SdkError<GetCrawlersError>>>;
    fn get_custom_entity_type(&self, builder: GetCustomEntityTypeInputBuilder) -> impl Future<Output = Result<GetCustomEntityTypeOutput, SdkError<GetCustomEntityTypeError>>>;
    fn get_data_catalog_encryption_settings(&self, builder: GetDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<GetDataCatalogEncryptionSettingsOutput, SdkError<GetDataCatalogEncryptionSettingsError>>>;
    fn get_data_quality_result(&self, builder: GetDataQualityResultInputBuilder) -> impl Future<Output = Result<GetDataQualityResultOutput, SdkError<GetDataQualityResultError>>>;
    fn get_data_quality_rule_recommendation_run(&self, builder: GetDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRuleRecommendationRunOutput, SdkError<GetDataQualityRuleRecommendationRunError>>>;
    fn get_data_quality_ruleset(&self, builder: GetDataQualityRulesetInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetOutput, SdkError<GetDataQualityRulesetError>>>;
    fn get_data_quality_ruleset_evaluation_run(&self, builder: GetDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetEvaluationRunOutput, SdkError<GetDataQualityRulesetEvaluationRunError>>>;
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>>;
    fn get_databases(&self, builder: GetDatabasesInputBuilder) -> impl Future<Output = Result<GetDatabasesOutput, SdkError<GetDatabasesError>>>;
    fn get_dataflow_graph(&self, builder: GetDataflowGraphInputBuilder) -> impl Future<Output = Result<GetDataflowGraphOutput, SdkError<GetDataflowGraphError>>>;
    fn get_dev_endpoint(&self, builder: GetDevEndpointInputBuilder) -> impl Future<Output = Result<GetDevEndpointOutput, SdkError<GetDevEndpointError>>>;
    fn get_dev_endpoints(&self, builder: GetDevEndpointsInputBuilder) -> impl Future<Output = Result<GetDevEndpointsOutput, SdkError<GetDevEndpointsError>>>;
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>>;
    fn get_job_bookmark(&self, builder: GetJobBookmarkInputBuilder) -> impl Future<Output = Result<GetJobBookmarkOutput, SdkError<GetJobBookmarkError>>>;
    fn get_job_run(&self, builder: GetJobRunInputBuilder) -> impl Future<Output = Result<GetJobRunOutput, SdkError<GetJobRunError>>>;
    fn get_job_runs(&self, builder: GetJobRunsInputBuilder) -> impl Future<Output = Result<GetJobRunsOutput, SdkError<GetJobRunsError>>>;
    fn get_jobs(&self, builder: GetJobsInputBuilder) -> impl Future<Output = Result<GetJobsOutput, SdkError<GetJobsError>>>;
    fn get_mapping(&self, builder: GetMappingInputBuilder) -> impl Future<Output = Result<GetMappingOutput, SdkError<GetMappingError>>>;
    fn get_ml_task_run(&self, builder: GetMlTaskRunInputBuilder) -> impl Future<Output = Result<GetMlTaskRunOutput, SdkError<GetMLTaskRunError>>>;
    fn get_ml_task_runs(&self, builder: GetMlTaskRunsInputBuilder) -> impl Future<Output = Result<GetMlTaskRunsOutput, SdkError<GetMLTaskRunsError>>>;
    fn get_ml_transform(&self, builder: GetMlTransformInputBuilder) -> impl Future<Output = Result<GetMlTransformOutput, SdkError<GetMLTransformError>>>;
    fn get_ml_transforms(&self, builder: GetMlTransformsInputBuilder) -> impl Future<Output = Result<GetMlTransformsOutput, SdkError<GetMLTransformsError>>>;
    fn get_partition(&self, builder: GetPartitionInputBuilder) -> impl Future<Output = Result<GetPartitionOutput, SdkError<GetPartitionError>>>;
    fn get_partition_indexes(&self, builder: GetPartitionIndexesInputBuilder) -> impl Future<Output = Result<GetPartitionIndexesOutput, SdkError<GetPartitionIndexesError>>>;
    fn get_partitions(&self, builder: GetPartitionsInputBuilder) -> impl Future<Output = Result<GetPartitionsOutput, SdkError<GetPartitionsError>>>;
    fn get_plan(&self, builder: GetPlanInputBuilder) -> impl Future<Output = Result<GetPlanOutput, SdkError<GetPlanError>>>;
    fn get_registry(&self, builder: GetRegistryInputBuilder) -> impl Future<Output = Result<GetRegistryOutput, SdkError<GetRegistryError>>>;
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn get_schema(&self, builder: GetSchemaInputBuilder) -> impl Future<Output = Result<GetSchemaOutput, SdkError<GetSchemaError>>>;
    fn get_schema_by_definition(&self, builder: GetSchemaByDefinitionInputBuilder) -> impl Future<Output = Result<GetSchemaByDefinitionOutput, SdkError<GetSchemaByDefinitionError>>>;
    fn get_schema_version(&self, builder: GetSchemaVersionInputBuilder) -> impl Future<Output = Result<GetSchemaVersionOutput, SdkError<GetSchemaVersionError>>>;
    fn get_schema_versions_diff(&self, builder: GetSchemaVersionsDiffInputBuilder) -> impl Future<Output = Result<GetSchemaVersionsDiffOutput, SdkError<GetSchemaVersionsDiffError>>>;
    fn get_security_configuration(&self, builder: GetSecurityConfigurationInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationOutput, SdkError<GetSecurityConfigurationError>>>;
    fn get_security_configurations(&self, builder: GetSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationsOutput, SdkError<GetSecurityConfigurationsError>>>;
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>>;
    fn get_statement(&self, builder: GetStatementInputBuilder) -> impl Future<Output = Result<GetStatementOutput, SdkError<GetStatementError>>>;
    fn get_table(&self, builder: GetTableInputBuilder) -> impl Future<Output = Result<GetTableOutput, SdkError<GetTableError>>>;
    fn get_table_optimizer(&self, builder: GetTableOptimizerInputBuilder) -> impl Future<Output = Result<GetTableOptimizerOutput, SdkError<GetTableOptimizerError>>>;
    fn get_table_version(&self, builder: GetTableVersionInputBuilder) -> impl Future<Output = Result<GetTableVersionOutput, SdkError<GetTableVersionError>>>;
    fn get_table_versions(&self, builder: GetTableVersionsInputBuilder) -> impl Future<Output = Result<GetTableVersionsOutput, SdkError<GetTableVersionsError>>>;
    fn get_tables(&self, builder: GetTablesInputBuilder) -> impl Future<Output = Result<GetTablesOutput, SdkError<GetTablesError>>>;
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>>;
    fn get_trigger(&self, builder: GetTriggerInputBuilder) -> impl Future<Output = Result<GetTriggerOutput, SdkError<GetTriggerError>>>;
    fn get_triggers(&self, builder: GetTriggersInputBuilder) -> impl Future<Output = Result<GetTriggersOutput, SdkError<GetTriggersError>>>;
    fn get_unfiltered_partition_metadata(&self, builder: GetUnfilteredPartitionMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionMetadataOutput, SdkError<GetUnfilteredPartitionMetadataError>>>;
    fn get_unfiltered_partitions_metadata(&self, builder: GetUnfilteredPartitionsMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionsMetadataOutput, SdkError<GetUnfilteredPartitionsMetadataError>>>;
    fn get_unfiltered_table_metadata(&self, builder: GetUnfilteredTableMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredTableMetadataOutput, SdkError<GetUnfilteredTableMetadataError>>>;
    fn get_usage_profile(&self, builder: GetUsageProfileInputBuilder) -> impl Future<Output = Result<GetUsageProfileOutput, SdkError<GetUsageProfileError>>>;
    fn get_user_defined_function(&self, builder: GetUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionOutput, SdkError<GetUserDefinedFunctionError>>>;
    fn get_user_defined_functions(&self, builder: GetUserDefinedFunctionsInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionsOutput, SdkError<GetUserDefinedFunctionsError>>>;
    fn get_workflow(&self, builder: GetWorkflowInputBuilder) -> impl Future<Output = Result<GetWorkflowOutput, SdkError<GetWorkflowError>>>;
    fn get_workflow_run(&self, builder: GetWorkflowRunInputBuilder) -> impl Future<Output = Result<GetWorkflowRunOutput, SdkError<GetWorkflowRunError>>>;
    fn get_workflow_run_properties(&self, builder: GetWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<GetWorkflowRunPropertiesOutput, SdkError<GetWorkflowRunPropertiesError>>>;
    fn get_workflow_runs(&self, builder: GetWorkflowRunsInputBuilder) -> impl Future<Output = Result<GetWorkflowRunsOutput, SdkError<GetWorkflowRunsError>>>;
    fn import_catalog_to_glue(&self, builder: ImportCatalogToGlueInputBuilder) -> impl Future<Output = Result<ImportCatalogToGlueOutput, SdkError<ImportCatalogToGlueError>>>;
    fn list_blueprints(&self, builder: ListBlueprintsInputBuilder) -> impl Future<Output = Result<ListBlueprintsOutput, SdkError<ListBlueprintsError>>>;
    fn list_column_statistics_task_runs(&self, builder: ListColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<ListColumnStatisticsTaskRunsOutput, SdkError<ListColumnStatisticsTaskRunsError>>>;
    fn list_crawlers(&self, builder: ListCrawlersInputBuilder) -> impl Future<Output = Result<ListCrawlersOutput, SdkError<ListCrawlersError>>>;
    fn list_crawls(&self, builder: ListCrawlsInputBuilder) -> impl Future<Output = Result<ListCrawlsOutput, SdkError<ListCrawlsError>>>;
    fn list_custom_entity_types(&self, builder: ListCustomEntityTypesInputBuilder) -> impl Future<Output = Result<ListCustomEntityTypesOutput, SdkError<ListCustomEntityTypesError>>>;
    fn list_data_quality_results(&self, builder: ListDataQualityResultsInputBuilder) -> impl Future<Output = Result<ListDataQualityResultsOutput, SdkError<ListDataQualityResultsError>>>;
    fn list_data_quality_rule_recommendation_runs(&self, builder: ListDataQualityRuleRecommendationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRuleRecommendationRunsOutput, SdkError<ListDataQualityRuleRecommendationRunsError>>>;
    fn list_data_quality_ruleset_evaluation_runs(&self, builder: ListDataQualityRulesetEvaluationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetEvaluationRunsOutput, SdkError<ListDataQualityRulesetEvaluationRunsError>>>;
    fn list_data_quality_rulesets(&self, builder: ListDataQualityRulesetsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetsOutput, SdkError<ListDataQualityRulesetsError>>>;
    fn list_dev_endpoints(&self, builder: ListDevEndpointsInputBuilder) -> impl Future<Output = Result<ListDevEndpointsOutput, SdkError<ListDevEndpointsError>>>;
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>>;
    fn list_ml_transforms(&self, builder: ListMlTransformsInputBuilder) -> impl Future<Output = Result<ListMlTransformsOutput, SdkError<ListMLTransformsError>>>;
    fn list_registries(&self, builder: ListRegistriesInputBuilder) -> impl Future<Output = Result<ListRegistriesOutput, SdkError<ListRegistriesError>>>;
    fn list_schema_versions(&self, builder: ListSchemaVersionsInputBuilder) -> impl Future<Output = Result<ListSchemaVersionsOutput, SdkError<ListSchemaVersionsError>>>;
    fn list_schemas(&self, builder: ListSchemasInputBuilder) -> impl Future<Output = Result<ListSchemasOutput, SdkError<ListSchemasError>>>;
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>>;
    fn list_statements(&self, builder: ListStatementsInputBuilder) -> impl Future<Output = Result<ListStatementsOutput, SdkError<ListStatementsError>>>;
    fn list_table_optimizer_runs(&self, builder: ListTableOptimizerRunsInputBuilder) -> impl Future<Output = Result<ListTableOptimizerRunsOutput, SdkError<ListTableOptimizerRunsError>>>;
    fn list_triggers(&self, builder: ListTriggersInputBuilder) -> impl Future<Output = Result<ListTriggersOutput, SdkError<ListTriggersError>>>;
    fn list_usage_profiles(&self, builder: ListUsageProfilesInputBuilder) -> impl Future<Output = Result<ListUsageProfilesOutput, SdkError<ListUsageProfilesError>>>;
    fn list_workflows(&self, builder: ListWorkflowsInputBuilder) -> impl Future<Output = Result<ListWorkflowsOutput, SdkError<ListWorkflowsError>>>;
    fn put_data_catalog_encryption_settings(&self, builder: PutDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<PutDataCatalogEncryptionSettingsOutput, SdkError<PutDataCatalogEncryptionSettingsError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn put_schema_version_metadata(&self, builder: PutSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<PutSchemaVersionMetadataOutput, SdkError<PutSchemaVersionMetadataError>>>;
    fn put_workflow_run_properties(&self, builder: PutWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<PutWorkflowRunPropertiesOutput, SdkError<PutWorkflowRunPropertiesError>>>;
    fn query_schema_version_metadata(&self, builder: QuerySchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<QuerySchemaVersionMetadataOutput, SdkError<QuerySchemaVersionMetadataError>>>;
    fn register_schema_version(&self, builder: RegisterSchemaVersionInputBuilder) -> impl Future<Output = Result<RegisterSchemaVersionOutput, SdkError<RegisterSchemaVersionError>>>;
    fn remove_schema_version_metadata(&self, builder: RemoveSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<RemoveSchemaVersionMetadataOutput, SdkError<RemoveSchemaVersionMetadataError>>>;
    fn reset_job_bookmark(&self, builder: ResetJobBookmarkInputBuilder) -> impl Future<Output = Result<ResetJobBookmarkOutput, SdkError<ResetJobBookmarkError>>>;
    fn resume_workflow_run(&self, builder: ResumeWorkflowRunInputBuilder) -> impl Future<Output = Result<ResumeWorkflowRunOutput, SdkError<ResumeWorkflowRunError>>>;
    fn run_statement(&self, builder: RunStatementInputBuilder) -> impl Future<Output = Result<RunStatementOutput, SdkError<RunStatementError>>>;
    fn search_tables(&self, builder: SearchTablesInputBuilder) -> impl Future<Output = Result<SearchTablesOutput, SdkError<SearchTablesError>>>;
    fn start_blueprint_run(&self, builder: StartBlueprintRunInputBuilder) -> impl Future<Output = Result<StartBlueprintRunOutput, SdkError<StartBlueprintRunError>>>;
    fn start_column_statistics_task_run(&self, builder: StartColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StartColumnStatisticsTaskRunOutput, SdkError<StartColumnStatisticsTaskRunError>>>;
    fn start_crawler(&self, builder: StartCrawlerInputBuilder) -> impl Future<Output = Result<StartCrawlerOutput, SdkError<StartCrawlerError>>>;
    fn start_crawler_schedule(&self, builder: StartCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StartCrawlerScheduleOutput, SdkError<StartCrawlerScheduleError>>>;
    fn start_data_quality_rule_recommendation_run(&self, builder: StartDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRuleRecommendationRunOutput, SdkError<StartDataQualityRuleRecommendationRunError>>>;
    fn start_data_quality_ruleset_evaluation_run(&self, builder: StartDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRulesetEvaluationRunOutput, SdkError<StartDataQualityRulesetEvaluationRunError>>>;
    fn start_export_labels_task_run(&self, builder: StartExportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartExportLabelsTaskRunOutput, SdkError<StartExportLabelsTaskRunError>>>;
    fn start_import_labels_task_run(&self, builder: StartImportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartImportLabelsTaskRunOutput, SdkError<StartImportLabelsTaskRunError>>>;
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>>;
    fn start_ml_evaluation_task_run(&self, builder: StartMlEvaluationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlEvaluationTaskRunOutput, SdkError<StartMLEvaluationTaskRunError>>>;
    fn start_ml_labeling_set_generation_task_run(&self, builder: StartMlLabelingSetGenerationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlLabelingSetGenerationTaskRunOutput, SdkError<StartMLLabelingSetGenerationTaskRunError>>>;
    fn start_trigger(&self, builder: StartTriggerInputBuilder) -> impl Future<Output = Result<StartTriggerOutput, SdkError<StartTriggerError>>>;
    fn start_workflow_run(&self, builder: StartWorkflowRunInputBuilder) -> impl Future<Output = Result<StartWorkflowRunOutput, SdkError<StartWorkflowRunError>>>;
    fn stop_column_statistics_task_run(&self, builder: StopColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StopColumnStatisticsTaskRunOutput, SdkError<StopColumnStatisticsTaskRunError>>>;
    fn stop_crawler(&self, builder: StopCrawlerInputBuilder) -> impl Future<Output = Result<StopCrawlerOutput, SdkError<StopCrawlerError>>>;
    fn stop_crawler_schedule(&self, builder: StopCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StopCrawlerScheduleOutput, SdkError<StopCrawlerScheduleError>>>;
    fn stop_session(&self, builder: StopSessionInputBuilder) -> impl Future<Output = Result<StopSessionOutput, SdkError<StopSessionError>>>;
    fn stop_trigger(&self, builder: StopTriggerInputBuilder) -> impl Future<Output = Result<StopTriggerOutput, SdkError<StopTriggerError>>>;
    fn stop_workflow_run(&self, builder: StopWorkflowRunInputBuilder) -> impl Future<Output = Result<StopWorkflowRunOutput, SdkError<StopWorkflowRunError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_blueprint(&self, builder: UpdateBlueprintInputBuilder) -> impl Future<Output = Result<UpdateBlueprintOutput, SdkError<UpdateBlueprintError>>>;
    fn update_classifier(&self, builder: UpdateClassifierInputBuilder) -> impl Future<Output = Result<UpdateClassifierOutput, SdkError<UpdateClassifierError>>>;
    fn update_column_statistics_for_partition(&self, builder: UpdateColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForPartitionOutput, SdkError<UpdateColumnStatisticsForPartitionError>>>;
    fn update_column_statistics_for_table(&self, builder: UpdateColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForTableOutput, SdkError<UpdateColumnStatisticsForTableError>>>;
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>>;
    fn update_crawler(&self, builder: UpdateCrawlerInputBuilder) -> impl Future<Output = Result<UpdateCrawlerOutput, SdkError<UpdateCrawlerError>>>;
    fn update_crawler_schedule(&self, builder: UpdateCrawlerScheduleInputBuilder) -> impl Future<Output = Result<UpdateCrawlerScheduleOutput, SdkError<UpdateCrawlerScheduleError>>>;
    fn update_data_quality_ruleset(&self, builder: UpdateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<UpdateDataQualityRulesetOutput, SdkError<UpdateDataQualityRulesetError>>>;
    fn update_database(&self, builder: UpdateDatabaseInputBuilder) -> impl Future<Output = Result<UpdateDatabaseOutput, SdkError<UpdateDatabaseError>>>;
    fn update_dev_endpoint(&self, builder: UpdateDevEndpointInputBuilder) -> impl Future<Output = Result<UpdateDevEndpointOutput, SdkError<UpdateDevEndpointError>>>;
    fn update_job(&self, builder: UpdateJobInputBuilder) -> impl Future<Output = Result<UpdateJobOutput, SdkError<UpdateJobError>>>;
    fn update_job_from_source_control(&self, builder: UpdateJobFromSourceControlInputBuilder) -> impl Future<Output = Result<UpdateJobFromSourceControlOutput, SdkError<UpdateJobFromSourceControlError>>>;
    fn update_ml_transform(&self, builder: UpdateMlTransformInputBuilder) -> impl Future<Output = Result<UpdateMlTransformOutput, SdkError<UpdateMLTransformError>>>;
    fn update_partition(&self, builder: UpdatePartitionInputBuilder) -> impl Future<Output = Result<UpdatePartitionOutput, SdkError<UpdatePartitionError>>>;
    fn update_registry(&self, builder: UpdateRegistryInputBuilder) -> impl Future<Output = Result<UpdateRegistryOutput, SdkError<UpdateRegistryError>>>;
    fn update_schema(&self, builder: UpdateSchemaInputBuilder) -> impl Future<Output = Result<UpdateSchemaOutput, SdkError<UpdateSchemaError>>>;
    fn update_source_control_from_job(&self, builder: UpdateSourceControlFromJobInputBuilder) -> impl Future<Output = Result<UpdateSourceControlFromJobOutput, SdkError<UpdateSourceControlFromJobError>>>;
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>>;
    fn update_table_optimizer(&self, builder: UpdateTableOptimizerInputBuilder) -> impl Future<Output = Result<UpdateTableOptimizerOutput, SdkError<UpdateTableOptimizerError>>>;
    fn update_trigger(&self, builder: UpdateTriggerInputBuilder) -> impl Future<Output = Result<UpdateTriggerOutput, SdkError<UpdateTriggerError>>>;
    fn update_usage_profile(&self, builder: UpdateUsageProfileInputBuilder) -> impl Future<Output = Result<UpdateUsageProfileOutput, SdkError<UpdateUsageProfileError>>>;
    fn update_user_defined_function(&self, builder: UpdateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<UpdateUserDefinedFunctionOutput, SdkError<UpdateUserDefinedFunctionError>>>;
    fn update_workflow(&self, builder: UpdateWorkflowInputBuilder) -> impl Future<Output = Result<UpdateWorkflowOutput, SdkError<UpdateWorkflowError>>>;
}
impl GlueClient for GlueClientImpl {
    fn batch_create_partition(&self, builder: BatchCreatePartitionInputBuilder) -> impl Future<Output = Result<BatchCreatePartitionOutput, SdkError<BatchCreatePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_connection(&self, builder: BatchDeleteConnectionInputBuilder) -> impl Future<Output = Result<BatchDeleteConnectionOutput, SdkError<BatchDeleteConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_partition(&self, builder: BatchDeletePartitionInputBuilder) -> impl Future<Output = Result<BatchDeletePartitionOutput, SdkError<BatchDeletePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_table(&self, builder: BatchDeleteTableInputBuilder) -> impl Future<Output = Result<BatchDeleteTableOutput, SdkError<BatchDeleteTableError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_table_version(&self, builder: BatchDeleteTableVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteTableVersionOutput, SdkError<BatchDeleteTableVersionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_blueprints(&self, builder: BatchGetBlueprintsInputBuilder) -> impl Future<Output = Result<BatchGetBlueprintsOutput, SdkError<BatchGetBlueprintsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_crawlers(&self, builder: BatchGetCrawlersInputBuilder) -> impl Future<Output = Result<BatchGetCrawlersOutput, SdkError<BatchGetCrawlersError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_custom_entity_types(&self, builder: BatchGetCustomEntityTypesInputBuilder) -> impl Future<Output = Result<BatchGetCustomEntityTypesOutput, SdkError<BatchGetCustomEntityTypesError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_data_quality_result(&self, builder: BatchGetDataQualityResultInputBuilder) -> impl Future<Output = Result<BatchGetDataQualityResultOutput, SdkError<BatchGetDataQualityResultError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_dev_endpoints(&self, builder: BatchGetDevEndpointsInputBuilder) -> impl Future<Output = Result<BatchGetDevEndpointsOutput, SdkError<BatchGetDevEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_jobs(&self, builder: BatchGetJobsInputBuilder) -> impl Future<Output = Result<BatchGetJobsOutput, SdkError<BatchGetJobsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_partition(&self, builder: BatchGetPartitionInputBuilder) -> impl Future<Output = Result<BatchGetPartitionOutput, SdkError<BatchGetPartitionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_table_optimizer(&self, builder: BatchGetTableOptimizerInputBuilder) -> impl Future<Output = Result<BatchGetTableOptimizerOutput, SdkError<BatchGetTableOptimizerError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_triggers(&self, builder: BatchGetTriggersInputBuilder) -> impl Future<Output = Result<BatchGetTriggersOutput, SdkError<BatchGetTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_workflows(&self, builder: BatchGetWorkflowsInputBuilder) -> impl Future<Output = Result<BatchGetWorkflowsOutput, SdkError<BatchGetWorkflowsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_stop_job_run(&self, builder: BatchStopJobRunInputBuilder) -> impl Future<Output = Result<BatchStopJobRunOutput, SdkError<BatchStopJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn batch_update_partition(&self, builder: BatchUpdatePartitionInputBuilder) -> impl Future<Output = Result<BatchUpdatePartitionOutput, SdkError<BatchUpdatePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_data_quality_rule_recommendation_run(&self, builder: CancelDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRuleRecommendationRunOutput, SdkError<CancelDataQualityRuleRecommendationRunError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_data_quality_ruleset_evaluation_run(&self, builder: CancelDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRulesetEvaluationRunOutput, SdkError<CancelDataQualityRulesetEvaluationRunError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_ml_task_run(&self, builder: CancelMlTaskRunInputBuilder) -> impl Future<Output = Result<CancelMlTaskRunOutput, SdkError<CancelMLTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_statement(&self, builder: CancelStatementInputBuilder) -> impl Future<Output = Result<CancelStatementOutput, SdkError<CancelStatementError>>> {
        builder.send_with(&self.0)
    }
    fn check_schema_version_validity(&self, builder: CheckSchemaVersionValidityInputBuilder) -> impl Future<Output = Result<CheckSchemaVersionValidityOutput, SdkError<CheckSchemaVersionValidityError>>> {
        builder.send_with(&self.0)
    }
    fn create_blueprint(&self, builder: CreateBlueprintInputBuilder) -> impl Future<Output = Result<CreateBlueprintOutput, SdkError<CreateBlueprintError>>> {
        builder.send_with(&self.0)
    }
    fn create_classifier(&self, builder: CreateClassifierInputBuilder) -> impl Future<Output = Result<CreateClassifierOutput, SdkError<CreateClassifierError>>> {
        builder.send_with(&self.0)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_crawler(&self, builder: CreateCrawlerInputBuilder) -> impl Future<Output = Result<CreateCrawlerOutput, SdkError<CreateCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_entity_type(&self, builder: CreateCustomEntityTypeInputBuilder) -> impl Future<Output = Result<CreateCustomEntityTypeOutput, SdkError<CreateCustomEntityTypeError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_quality_ruleset(&self, builder: CreateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<CreateDataQualityRulesetOutput, SdkError<CreateDataQualityRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn create_database(&self, builder: CreateDatabaseInputBuilder) -> impl Future<Output = Result<CreateDatabaseOutput, SdkError<CreateDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn create_dev_endpoint(&self, builder: CreateDevEndpointInputBuilder) -> impl Future<Output = Result<CreateDevEndpointOutput, SdkError<CreateDevEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_job(&self, builder: CreateJobInputBuilder) -> impl Future<Output = Result<CreateJobOutput, SdkError<CreateJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_ml_transform(&self, builder: CreateMlTransformInputBuilder) -> impl Future<Output = Result<CreateMlTransformOutput, SdkError<CreateMLTransformError>>> {
        builder.send_with(&self.0)
    }
    fn create_partition(&self, builder: CreatePartitionInputBuilder) -> impl Future<Output = Result<CreatePartitionOutput, SdkError<CreatePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn create_partition_index(&self, builder: CreatePartitionIndexInputBuilder) -> impl Future<Output = Result<CreatePartitionIndexOutput, SdkError<CreatePartitionIndexError>>> {
        builder.send_with(&self.0)
    }
    fn create_registry(&self, builder: CreateRegistryInputBuilder) -> impl Future<Output = Result<CreateRegistryOutput, SdkError<CreateRegistryError>>> {
        builder.send_with(&self.0)
    }
    fn create_schema(&self, builder: CreateSchemaInputBuilder) -> impl Future<Output = Result<CreateSchemaOutput, SdkError<CreateSchemaError>>> {
        builder.send_with(&self.0)
    }
    fn create_script(&self, builder: CreateScriptInputBuilder) -> impl Future<Output = Result<CreateScriptOutput, SdkError<CreateScriptError>>> {
        builder.send_with(&self.0)
    }
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> {
        builder.send_with(&self.0)
    }
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_table_optimizer(&self, builder: CreateTableOptimizerInputBuilder) -> impl Future<Output = Result<CreateTableOptimizerOutput, SdkError<CreateTableOptimizerError>>> {
        builder.send_with(&self.0)
    }
    fn create_trigger(&self, builder: CreateTriggerInputBuilder) -> impl Future<Output = Result<CreateTriggerOutput, SdkError<CreateTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn create_usage_profile(&self, builder: CreateUsageProfileInputBuilder) -> impl Future<Output = Result<CreateUsageProfileOutput, SdkError<CreateUsageProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_defined_function(&self, builder: CreateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<CreateUserDefinedFunctionOutput, SdkError<CreateUserDefinedFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn create_workflow(&self, builder: CreateWorkflowInputBuilder) -> impl Future<Output = Result<CreateWorkflowOutput, SdkError<CreateWorkflowError>>> {
        builder.send_with(&self.0)
    }
    fn delete_blueprint(&self, builder: DeleteBlueprintInputBuilder) -> impl Future<Output = Result<DeleteBlueprintOutput, SdkError<DeleteBlueprintError>>> {
        builder.send_with(&self.0)
    }
    fn delete_classifier(&self, builder: DeleteClassifierInputBuilder) -> impl Future<Output = Result<DeleteClassifierOutput, SdkError<DeleteClassifierError>>> {
        builder.send_with(&self.0)
    }
    fn delete_column_statistics_for_partition(&self, builder: DeleteColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForPartitionOutput, SdkError<DeleteColumnStatisticsForPartitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_column_statistics_for_table(&self, builder: DeleteColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForTableOutput, SdkError<DeleteColumnStatisticsForTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_crawler(&self, builder: DeleteCrawlerInputBuilder) -> impl Future<Output = Result<DeleteCrawlerOutput, SdkError<DeleteCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_entity_type(&self, builder: DeleteCustomEntityTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomEntityTypeOutput, SdkError<DeleteCustomEntityTypeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_quality_ruleset(&self, builder: DeleteDataQualityRulesetInputBuilder) -> impl Future<Output = Result<DeleteDataQualityRulesetOutput, SdkError<DeleteDataQualityRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_database(&self, builder: DeleteDatabaseInputBuilder) -> impl Future<Output = Result<DeleteDatabaseOutput, SdkError<DeleteDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dev_endpoint(&self, builder: DeleteDevEndpointInputBuilder) -> impl Future<Output = Result<DeleteDevEndpointOutput, SdkError<DeleteDevEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ml_transform(&self, builder: DeleteMlTransformInputBuilder) -> impl Future<Output = Result<DeleteMlTransformOutput, SdkError<DeleteMLTransformError>>> {
        builder.send_with(&self.0)
    }
    fn delete_partition(&self, builder: DeletePartitionInputBuilder) -> impl Future<Output = Result<DeletePartitionOutput, SdkError<DeletePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_partition_index(&self, builder: DeletePartitionIndexInputBuilder) -> impl Future<Output = Result<DeletePartitionIndexOutput, SdkError<DeletePartitionIndexError>>> {
        builder.send_with(&self.0)
    }
    fn delete_registry(&self, builder: DeleteRegistryInputBuilder) -> impl Future<Output = Result<DeleteRegistryOutput, SdkError<DeleteRegistryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_schema(&self, builder: DeleteSchemaInputBuilder) -> impl Future<Output = Result<DeleteSchemaOutput, SdkError<DeleteSchemaError>>> {
        builder.send_with(&self.0)
    }
    fn delete_schema_versions(&self, builder: DeleteSchemaVersionsInputBuilder) -> impl Future<Output = Result<DeleteSchemaVersionsOutput, SdkError<DeleteSchemaVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_session(&self, builder: DeleteSessionInputBuilder) -> impl Future<Output = Result<DeleteSessionOutput, SdkError<DeleteSessionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_table_optimizer(&self, builder: DeleteTableOptimizerInputBuilder) -> impl Future<Output = Result<DeleteTableOptimizerOutput, SdkError<DeleteTableOptimizerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_table_version(&self, builder: DeleteTableVersionInputBuilder) -> impl Future<Output = Result<DeleteTableVersionOutput, SdkError<DeleteTableVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_trigger(&self, builder: DeleteTriggerInputBuilder) -> impl Future<Output = Result<DeleteTriggerOutput, SdkError<DeleteTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_usage_profile(&self, builder: DeleteUsageProfileInputBuilder) -> impl Future<Output = Result<DeleteUsageProfileOutput, SdkError<DeleteUsageProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_defined_function(&self, builder: DeleteUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<DeleteUserDefinedFunctionOutput, SdkError<DeleteUserDefinedFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_workflow(&self, builder: DeleteWorkflowInputBuilder) -> impl Future<Output = Result<DeleteWorkflowOutput, SdkError<DeleteWorkflowError>>> {
        builder.send_with(&self.0)
    }
    fn get_blueprint(&self, builder: GetBlueprintInputBuilder) -> impl Future<Output = Result<GetBlueprintOutput, SdkError<GetBlueprintError>>> {
        builder.send_with(&self.0)
    }
    fn get_blueprint_run(&self, builder: GetBlueprintRunInputBuilder) -> impl Future<Output = Result<GetBlueprintRunOutput, SdkError<GetBlueprintRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_blueprint_runs(&self, builder: GetBlueprintRunsInputBuilder) -> impl Future<Output = Result<GetBlueprintRunsOutput, SdkError<GetBlueprintRunsError>>> {
        builder.send_with(&self.0)
    }
    fn get_catalog_import_status(&self, builder: GetCatalogImportStatusInputBuilder) -> impl Future<Output = Result<GetCatalogImportStatusOutput, SdkError<GetCatalogImportStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_classifier(&self, builder: GetClassifierInputBuilder) -> impl Future<Output = Result<GetClassifierOutput, SdkError<GetClassifierError>>> {
        builder.send_with(&self.0)
    }
    fn get_classifiers(&self, builder: GetClassifiersInputBuilder) -> impl Future<Output = Result<GetClassifiersOutput, SdkError<GetClassifiersError>>> {
        builder.send_with(&self.0)
    }
    fn get_column_statistics_for_partition(&self, builder: GetColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForPartitionOutput, SdkError<GetColumnStatisticsForPartitionError>>> {
        builder.send_with(&self.0)
    }
    fn get_column_statistics_for_table(&self, builder: GetColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForTableOutput, SdkError<GetColumnStatisticsForTableError>>> {
        builder.send_with(&self.0)
    }
    fn get_column_statistics_task_run(&self, builder: GetColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunOutput, SdkError<GetColumnStatisticsTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_column_statistics_task_runs(&self, builder: GetColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunsOutput, SdkError<GetColumnStatisticsTaskRunsError>>> {
        builder.send_with(&self.0)
    }
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_connections(&self, builder: GetConnectionsInputBuilder) -> impl Future<Output = Result<GetConnectionsOutput, SdkError<GetConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_crawler(&self, builder: GetCrawlerInputBuilder) -> impl Future<Output = Result<GetCrawlerOutput, SdkError<GetCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn get_crawler_metrics(&self, builder: GetCrawlerMetricsInputBuilder) -> impl Future<Output = Result<GetCrawlerMetricsOutput, SdkError<GetCrawlerMetricsError>>> {
        builder.send_with(&self.0)
    }
    fn get_crawlers(&self, builder: GetCrawlersInputBuilder) -> impl Future<Output = Result<GetCrawlersOutput, SdkError<GetCrawlersError>>> {
        builder.send_with(&self.0)
    }
    fn get_custom_entity_type(&self, builder: GetCustomEntityTypeInputBuilder) -> impl Future<Output = Result<GetCustomEntityTypeOutput, SdkError<GetCustomEntityTypeError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_catalog_encryption_settings(&self, builder: GetDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<GetDataCatalogEncryptionSettingsOutput, SdkError<GetDataCatalogEncryptionSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_quality_result(&self, builder: GetDataQualityResultInputBuilder) -> impl Future<Output = Result<GetDataQualityResultOutput, SdkError<GetDataQualityResultError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_quality_rule_recommendation_run(&self, builder: GetDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRuleRecommendationRunOutput, SdkError<GetDataQualityRuleRecommendationRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_quality_ruleset(&self, builder: GetDataQualityRulesetInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetOutput, SdkError<GetDataQualityRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_quality_ruleset_evaluation_run(&self, builder: GetDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetEvaluationRunOutput, SdkError<GetDataQualityRulesetEvaluationRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn get_databases(&self, builder: GetDatabasesInputBuilder) -> impl Future<Output = Result<GetDatabasesOutput, SdkError<GetDatabasesError>>> {
        builder.send_with(&self.0)
    }
    fn get_dataflow_graph(&self, builder: GetDataflowGraphInputBuilder) -> impl Future<Output = Result<GetDataflowGraphOutput, SdkError<GetDataflowGraphError>>> {
        builder.send_with(&self.0)
    }
    fn get_dev_endpoint(&self, builder: GetDevEndpointInputBuilder) -> impl Future<Output = Result<GetDevEndpointOutput, SdkError<GetDevEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn get_dev_endpoints(&self, builder: GetDevEndpointsInputBuilder) -> impl Future<Output = Result<GetDevEndpointsOutput, SdkError<GetDevEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_job_bookmark(&self, builder: GetJobBookmarkInputBuilder) -> impl Future<Output = Result<GetJobBookmarkOutput, SdkError<GetJobBookmarkError>>> {
        builder.send_with(&self.0)
    }
    fn get_job_run(&self, builder: GetJobRunInputBuilder) -> impl Future<Output = Result<GetJobRunOutput, SdkError<GetJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_job_runs(&self, builder: GetJobRunsInputBuilder) -> impl Future<Output = Result<GetJobRunsOutput, SdkError<GetJobRunsError>>> {
        builder.send_with(&self.0)
    }
    fn get_jobs(&self, builder: GetJobsInputBuilder) -> impl Future<Output = Result<GetJobsOutput, SdkError<GetJobsError>>> {
        builder.send_with(&self.0)
    }
    fn get_mapping(&self, builder: GetMappingInputBuilder) -> impl Future<Output = Result<GetMappingOutput, SdkError<GetMappingError>>> {
        builder.send_with(&self.0)
    }
    fn get_ml_task_run(&self, builder: GetMlTaskRunInputBuilder) -> impl Future<Output = Result<GetMlTaskRunOutput, SdkError<GetMLTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_ml_task_runs(&self, builder: GetMlTaskRunsInputBuilder) -> impl Future<Output = Result<GetMlTaskRunsOutput, SdkError<GetMLTaskRunsError>>> {
        builder.send_with(&self.0)
    }
    fn get_ml_transform(&self, builder: GetMlTransformInputBuilder) -> impl Future<Output = Result<GetMlTransformOutput, SdkError<GetMLTransformError>>> {
        builder.send_with(&self.0)
    }
    fn get_ml_transforms(&self, builder: GetMlTransformsInputBuilder) -> impl Future<Output = Result<GetMlTransformsOutput, SdkError<GetMLTransformsError>>> {
        builder.send_with(&self.0)
    }
    fn get_partition(&self, builder: GetPartitionInputBuilder) -> impl Future<Output = Result<GetPartitionOutput, SdkError<GetPartitionError>>> {
        builder.send_with(&self.0)
    }
    fn get_partition_indexes(&self, builder: GetPartitionIndexesInputBuilder) -> impl Future<Output = Result<GetPartitionIndexesOutput, SdkError<GetPartitionIndexesError>>> {
        builder.send_with(&self.0)
    }
    fn get_partitions(&self, builder: GetPartitionsInputBuilder) -> impl Future<Output = Result<GetPartitionsOutput, SdkError<GetPartitionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_plan(&self, builder: GetPlanInputBuilder) -> impl Future<Output = Result<GetPlanOutput, SdkError<GetPlanError>>> {
        builder.send_with(&self.0)
    }
    fn get_registry(&self, builder: GetRegistryInputBuilder) -> impl Future<Output = Result<GetRegistryOutput, SdkError<GetRegistryError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_schema(&self, builder: GetSchemaInputBuilder) -> impl Future<Output = Result<GetSchemaOutput, SdkError<GetSchemaError>>> {
        builder.send_with(&self.0)
    }
    fn get_schema_by_definition(&self, builder: GetSchemaByDefinitionInputBuilder) -> impl Future<Output = Result<GetSchemaByDefinitionOutput, SdkError<GetSchemaByDefinitionError>>> {
        builder.send_with(&self.0)
    }
    fn get_schema_version(&self, builder: GetSchemaVersionInputBuilder) -> impl Future<Output = Result<GetSchemaVersionOutput, SdkError<GetSchemaVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_schema_versions_diff(&self, builder: GetSchemaVersionsDiffInputBuilder) -> impl Future<Output = Result<GetSchemaVersionsDiffOutput, SdkError<GetSchemaVersionsDiffError>>> {
        builder.send_with(&self.0)
    }
    fn get_security_configuration(&self, builder: GetSecurityConfigurationInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationOutput, SdkError<GetSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_security_configurations(&self, builder: GetSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationsOutput, SdkError<GetSecurityConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>> {
        builder.send_with(&self.0)
    }
    fn get_statement(&self, builder: GetStatementInputBuilder) -> impl Future<Output = Result<GetStatementOutput, SdkError<GetStatementError>>> {
        builder.send_with(&self.0)
    }
    fn get_table(&self, builder: GetTableInputBuilder) -> impl Future<Output = Result<GetTableOutput, SdkError<GetTableError>>> {
        builder.send_with(&self.0)
    }
    fn get_table_optimizer(&self, builder: GetTableOptimizerInputBuilder) -> impl Future<Output = Result<GetTableOptimizerOutput, SdkError<GetTableOptimizerError>>> {
        builder.send_with(&self.0)
    }
    fn get_table_version(&self, builder: GetTableVersionInputBuilder) -> impl Future<Output = Result<GetTableVersionOutput, SdkError<GetTableVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_table_versions(&self, builder: GetTableVersionsInputBuilder) -> impl Future<Output = Result<GetTableVersionsOutput, SdkError<GetTableVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_tables(&self, builder: GetTablesInputBuilder) -> impl Future<Output = Result<GetTablesOutput, SdkError<GetTablesError>>> {
        builder.send_with(&self.0)
    }
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> {
        builder.send_with(&self.0)
    }
    fn get_trigger(&self, builder: GetTriggerInputBuilder) -> impl Future<Output = Result<GetTriggerOutput, SdkError<GetTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn get_triggers(&self, builder: GetTriggersInputBuilder) -> impl Future<Output = Result<GetTriggersOutput, SdkError<GetTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn get_unfiltered_partition_metadata(&self, builder: GetUnfilteredPartitionMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionMetadataOutput, SdkError<GetUnfilteredPartitionMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_unfiltered_partitions_metadata(&self, builder: GetUnfilteredPartitionsMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionsMetadataOutput, SdkError<GetUnfilteredPartitionsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_unfiltered_table_metadata(&self, builder: GetUnfilteredTableMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredTableMetadataOutput, SdkError<GetUnfilteredTableMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_profile(&self, builder: GetUsageProfileInputBuilder) -> impl Future<Output = Result<GetUsageProfileOutput, SdkError<GetUsageProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_user_defined_function(&self, builder: GetUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionOutput, SdkError<GetUserDefinedFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn get_user_defined_functions(&self, builder: GetUserDefinedFunctionsInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionsOutput, SdkError<GetUserDefinedFunctionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_workflow(&self, builder: GetWorkflowInputBuilder) -> impl Future<Output = Result<GetWorkflowOutput, SdkError<GetWorkflowError>>> {
        builder.send_with(&self.0)
    }
    fn get_workflow_run(&self, builder: GetWorkflowRunInputBuilder) -> impl Future<Output = Result<GetWorkflowRunOutput, SdkError<GetWorkflowRunError>>> {
        builder.send_with(&self.0)
    }
    fn get_workflow_run_properties(&self, builder: GetWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<GetWorkflowRunPropertiesOutput, SdkError<GetWorkflowRunPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn get_workflow_runs(&self, builder: GetWorkflowRunsInputBuilder) -> impl Future<Output = Result<GetWorkflowRunsOutput, SdkError<GetWorkflowRunsError>>> {
        builder.send_with(&self.0)
    }
    fn import_catalog_to_glue(&self, builder: ImportCatalogToGlueInputBuilder) -> impl Future<Output = Result<ImportCatalogToGlueOutput, SdkError<ImportCatalogToGlueError>>> {
        builder.send_with(&self.0)
    }
    fn list_blueprints(&self, builder: ListBlueprintsInputBuilder) -> impl Future<Output = Result<ListBlueprintsOutput, SdkError<ListBlueprintsError>>> {
        builder.send_with(&self.0)
    }
    fn list_column_statistics_task_runs(&self, builder: ListColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<ListColumnStatisticsTaskRunsOutput, SdkError<ListColumnStatisticsTaskRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_crawlers(&self, builder: ListCrawlersInputBuilder) -> impl Future<Output = Result<ListCrawlersOutput, SdkError<ListCrawlersError>>> {
        builder.send_with(&self.0)
    }
    fn list_crawls(&self, builder: ListCrawlsInputBuilder) -> impl Future<Output = Result<ListCrawlsOutput, SdkError<ListCrawlsError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_entity_types(&self, builder: ListCustomEntityTypesInputBuilder) -> impl Future<Output = Result<ListCustomEntityTypesOutput, SdkError<ListCustomEntityTypesError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_quality_results(&self, builder: ListDataQualityResultsInputBuilder) -> impl Future<Output = Result<ListDataQualityResultsOutput, SdkError<ListDataQualityResultsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_quality_rule_recommendation_runs(&self, builder: ListDataQualityRuleRecommendationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRuleRecommendationRunsOutput, SdkError<ListDataQualityRuleRecommendationRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_quality_ruleset_evaluation_runs(&self, builder: ListDataQualityRulesetEvaluationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetEvaluationRunsOutput, SdkError<ListDataQualityRulesetEvaluationRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_quality_rulesets(&self, builder: ListDataQualityRulesetsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetsOutput, SdkError<ListDataQualityRulesetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_dev_endpoints(&self, builder: ListDevEndpointsInputBuilder) -> impl Future<Output = Result<ListDevEndpointsOutput, SdkError<ListDevEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_ml_transforms(&self, builder: ListMlTransformsInputBuilder) -> impl Future<Output = Result<ListMlTransformsOutput, SdkError<ListMLTransformsError>>> {
        builder.send_with(&self.0)
    }
    fn list_registries(&self, builder: ListRegistriesInputBuilder) -> impl Future<Output = Result<ListRegistriesOutput, SdkError<ListRegistriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_schema_versions(&self, builder: ListSchemaVersionsInputBuilder) -> impl Future<Output = Result<ListSchemaVersionsOutput, SdkError<ListSchemaVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_schemas(&self, builder: ListSchemasInputBuilder) -> impl Future<Output = Result<ListSchemasOutput, SdkError<ListSchemasError>>> {
        builder.send_with(&self.0)
    }
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_statements(&self, builder: ListStatementsInputBuilder) -> impl Future<Output = Result<ListStatementsOutput, SdkError<ListStatementsError>>> {
        builder.send_with(&self.0)
    }
    fn list_table_optimizer_runs(&self, builder: ListTableOptimizerRunsInputBuilder) -> impl Future<Output = Result<ListTableOptimizerRunsOutput, SdkError<ListTableOptimizerRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_triggers(&self, builder: ListTriggersInputBuilder) -> impl Future<Output = Result<ListTriggersOutput, SdkError<ListTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn list_usage_profiles(&self, builder: ListUsageProfilesInputBuilder) -> impl Future<Output = Result<ListUsageProfilesOutput, SdkError<ListUsageProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_workflows(&self, builder: ListWorkflowsInputBuilder) -> impl Future<Output = Result<ListWorkflowsOutput, SdkError<ListWorkflowsError>>> {
        builder.send_with(&self.0)
    }
    fn put_data_catalog_encryption_settings(&self, builder: PutDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<PutDataCatalogEncryptionSettingsOutput, SdkError<PutDataCatalogEncryptionSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_schema_version_metadata(&self, builder: PutSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<PutSchemaVersionMetadataOutput, SdkError<PutSchemaVersionMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn put_workflow_run_properties(&self, builder: PutWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<PutWorkflowRunPropertiesOutput, SdkError<PutWorkflowRunPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn query_schema_version_metadata(&self, builder: QuerySchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<QuerySchemaVersionMetadataOutput, SdkError<QuerySchemaVersionMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn register_schema_version(&self, builder: RegisterSchemaVersionInputBuilder) -> impl Future<Output = Result<RegisterSchemaVersionOutput, SdkError<RegisterSchemaVersionError>>> {
        builder.send_with(&self.0)
    }
    fn remove_schema_version_metadata(&self, builder: RemoveSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<RemoveSchemaVersionMetadataOutput, SdkError<RemoveSchemaVersionMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn reset_job_bookmark(&self, builder: ResetJobBookmarkInputBuilder) -> impl Future<Output = Result<ResetJobBookmarkOutput, SdkError<ResetJobBookmarkError>>> {
        builder.send_with(&self.0)
    }
    fn resume_workflow_run(&self, builder: ResumeWorkflowRunInputBuilder) -> impl Future<Output = Result<ResumeWorkflowRunOutput, SdkError<ResumeWorkflowRunError>>> {
        builder.send_with(&self.0)
    }
    fn run_statement(&self, builder: RunStatementInputBuilder) -> impl Future<Output = Result<RunStatementOutput, SdkError<RunStatementError>>> {
        builder.send_with(&self.0)
    }
    fn search_tables(&self, builder: SearchTablesInputBuilder) -> impl Future<Output = Result<SearchTablesOutput, SdkError<SearchTablesError>>> {
        builder.send_with(&self.0)
    }
    fn start_blueprint_run(&self, builder: StartBlueprintRunInputBuilder) -> impl Future<Output = Result<StartBlueprintRunOutput, SdkError<StartBlueprintRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_column_statistics_task_run(&self, builder: StartColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StartColumnStatisticsTaskRunOutput, SdkError<StartColumnStatisticsTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_crawler(&self, builder: StartCrawlerInputBuilder) -> impl Future<Output = Result<StartCrawlerOutput, SdkError<StartCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn start_crawler_schedule(&self, builder: StartCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StartCrawlerScheduleOutput, SdkError<StartCrawlerScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn start_data_quality_rule_recommendation_run(&self, builder: StartDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRuleRecommendationRunOutput, SdkError<StartDataQualityRuleRecommendationRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_data_quality_ruleset_evaluation_run(&self, builder: StartDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRulesetEvaluationRunOutput, SdkError<StartDataQualityRulesetEvaluationRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_export_labels_task_run(&self, builder: StartExportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartExportLabelsTaskRunOutput, SdkError<StartExportLabelsTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_import_labels_task_run(&self, builder: StartImportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartImportLabelsTaskRunOutput, SdkError<StartImportLabelsTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_ml_evaluation_task_run(&self, builder: StartMlEvaluationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlEvaluationTaskRunOutput, SdkError<StartMLEvaluationTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_ml_labeling_set_generation_task_run(&self, builder: StartMlLabelingSetGenerationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlLabelingSetGenerationTaskRunOutput, SdkError<StartMLLabelingSetGenerationTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn start_trigger(&self, builder: StartTriggerInputBuilder) -> impl Future<Output = Result<StartTriggerOutput, SdkError<StartTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn start_workflow_run(&self, builder: StartWorkflowRunInputBuilder) -> impl Future<Output = Result<StartWorkflowRunOutput, SdkError<StartWorkflowRunError>>> {
        builder.send_with(&self.0)
    }
    fn stop_column_statistics_task_run(&self, builder: StopColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StopColumnStatisticsTaskRunOutput, SdkError<StopColumnStatisticsTaskRunError>>> {
        builder.send_with(&self.0)
    }
    fn stop_crawler(&self, builder: StopCrawlerInputBuilder) -> impl Future<Output = Result<StopCrawlerOutput, SdkError<StopCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn stop_crawler_schedule(&self, builder: StopCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StopCrawlerScheduleOutput, SdkError<StopCrawlerScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn stop_session(&self, builder: StopSessionInputBuilder) -> impl Future<Output = Result<StopSessionOutput, SdkError<StopSessionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_trigger(&self, builder: StopTriggerInputBuilder) -> impl Future<Output = Result<StopTriggerOutput, SdkError<StopTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn stop_workflow_run(&self, builder: StopWorkflowRunInputBuilder) -> impl Future<Output = Result<StopWorkflowRunOutput, SdkError<StopWorkflowRunError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_blueprint(&self, builder: UpdateBlueprintInputBuilder) -> impl Future<Output = Result<UpdateBlueprintOutput, SdkError<UpdateBlueprintError>>> {
        builder.send_with(&self.0)
    }
    fn update_classifier(&self, builder: UpdateClassifierInputBuilder) -> impl Future<Output = Result<UpdateClassifierOutput, SdkError<UpdateClassifierError>>> {
        builder.send_with(&self.0)
    }
    fn update_column_statistics_for_partition(&self, builder: UpdateColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForPartitionOutput, SdkError<UpdateColumnStatisticsForPartitionError>>> {
        builder.send_with(&self.0)
    }
    fn update_column_statistics_for_table(&self, builder: UpdateColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForTableOutput, SdkError<UpdateColumnStatisticsForTableError>>> {
        builder.send_with(&self.0)
    }
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn update_crawler(&self, builder: UpdateCrawlerInputBuilder) -> impl Future<Output = Result<UpdateCrawlerOutput, SdkError<UpdateCrawlerError>>> {
        builder.send_with(&self.0)
    }
    fn update_crawler_schedule(&self, builder: UpdateCrawlerScheduleInputBuilder) -> impl Future<Output = Result<UpdateCrawlerScheduleOutput, SdkError<UpdateCrawlerScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_quality_ruleset(&self, builder: UpdateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<UpdateDataQualityRulesetOutput, SdkError<UpdateDataQualityRulesetError>>> {
        builder.send_with(&self.0)
    }
    fn update_database(&self, builder: UpdateDatabaseInputBuilder) -> impl Future<Output = Result<UpdateDatabaseOutput, SdkError<UpdateDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn update_dev_endpoint(&self, builder: UpdateDevEndpointInputBuilder) -> impl Future<Output = Result<UpdateDevEndpointOutput, SdkError<UpdateDevEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn update_job(&self, builder: UpdateJobInputBuilder) -> impl Future<Output = Result<UpdateJobOutput, SdkError<UpdateJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_job_from_source_control(&self, builder: UpdateJobFromSourceControlInputBuilder) -> impl Future<Output = Result<UpdateJobFromSourceControlOutput, SdkError<UpdateJobFromSourceControlError>>> {
        builder.send_with(&self.0)
    }
    fn update_ml_transform(&self, builder: UpdateMlTransformInputBuilder) -> impl Future<Output = Result<UpdateMlTransformOutput, SdkError<UpdateMLTransformError>>> {
        builder.send_with(&self.0)
    }
    fn update_partition(&self, builder: UpdatePartitionInputBuilder) -> impl Future<Output = Result<UpdatePartitionOutput, SdkError<UpdatePartitionError>>> {
        builder.send_with(&self.0)
    }
    fn update_registry(&self, builder: UpdateRegistryInputBuilder) -> impl Future<Output = Result<UpdateRegistryOutput, SdkError<UpdateRegistryError>>> {
        builder.send_with(&self.0)
    }
    fn update_schema(&self, builder: UpdateSchemaInputBuilder) -> impl Future<Output = Result<UpdateSchemaOutput, SdkError<UpdateSchemaError>>> {
        builder.send_with(&self.0)
    }
    fn update_source_control_from_job(&self, builder: UpdateSourceControlFromJobInputBuilder) -> impl Future<Output = Result<UpdateSourceControlFromJobOutput, SdkError<UpdateSourceControlFromJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>> {
        builder.send_with(&self.0)
    }
    fn update_table_optimizer(&self, builder: UpdateTableOptimizerInputBuilder) -> impl Future<Output = Result<UpdateTableOptimizerOutput, SdkError<UpdateTableOptimizerError>>> {
        builder.send_with(&self.0)
    }
    fn update_trigger(&self, builder: UpdateTriggerInputBuilder) -> impl Future<Output = Result<UpdateTriggerOutput, SdkError<UpdateTriggerError>>> {
        builder.send_with(&self.0)
    }
    fn update_usage_profile(&self, builder: UpdateUsageProfileInputBuilder) -> impl Future<Output = Result<UpdateUsageProfileOutput, SdkError<UpdateUsageProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_defined_function(&self, builder: UpdateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<UpdateUserDefinedFunctionOutput, SdkError<UpdateUserDefinedFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn update_workflow(&self, builder: UpdateWorkflowInputBuilder) -> impl Future<Output = Result<UpdateWorkflowOutput, SdkError<UpdateWorkflowError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> GlueClient for T
where T: Deref,
      T::Target: GlueClient {
    fn batch_create_partition(&self, builder: BatchCreatePartitionInputBuilder) -> impl Future<Output = Result<BatchCreatePartitionOutput, SdkError<BatchCreatePartitionError>>> {
        self.deref().batch_create_partition(builder)
    }
    fn batch_delete_connection(&self, builder: BatchDeleteConnectionInputBuilder) -> impl Future<Output = Result<BatchDeleteConnectionOutput, SdkError<BatchDeleteConnectionError>>> {
        self.deref().batch_delete_connection(builder)
    }
    fn batch_delete_partition(&self, builder: BatchDeletePartitionInputBuilder) -> impl Future<Output = Result<BatchDeletePartitionOutput, SdkError<BatchDeletePartitionError>>> {
        self.deref().batch_delete_partition(builder)
    }
    fn batch_delete_table(&self, builder: BatchDeleteTableInputBuilder) -> impl Future<Output = Result<BatchDeleteTableOutput, SdkError<BatchDeleteTableError>>> {
        self.deref().batch_delete_table(builder)
    }
    fn batch_delete_table_version(&self, builder: BatchDeleteTableVersionInputBuilder) -> impl Future<Output = Result<BatchDeleteTableVersionOutput, SdkError<BatchDeleteTableVersionError>>> {
        self.deref().batch_delete_table_version(builder)
    }
    fn batch_get_blueprints(&self, builder: BatchGetBlueprintsInputBuilder) -> impl Future<Output = Result<BatchGetBlueprintsOutput, SdkError<BatchGetBlueprintsError>>> {
        self.deref().batch_get_blueprints(builder)
    }
    fn batch_get_crawlers(&self, builder: BatchGetCrawlersInputBuilder) -> impl Future<Output = Result<BatchGetCrawlersOutput, SdkError<BatchGetCrawlersError>>> {
        self.deref().batch_get_crawlers(builder)
    }
    fn batch_get_custom_entity_types(&self, builder: BatchGetCustomEntityTypesInputBuilder) -> impl Future<Output = Result<BatchGetCustomEntityTypesOutput, SdkError<BatchGetCustomEntityTypesError>>> {
        self.deref().batch_get_custom_entity_types(builder)
    }
    fn batch_get_data_quality_result(&self, builder: BatchGetDataQualityResultInputBuilder) -> impl Future<Output = Result<BatchGetDataQualityResultOutput, SdkError<BatchGetDataQualityResultError>>> {
        self.deref().batch_get_data_quality_result(builder)
    }
    fn batch_get_dev_endpoints(&self, builder: BatchGetDevEndpointsInputBuilder) -> impl Future<Output = Result<BatchGetDevEndpointsOutput, SdkError<BatchGetDevEndpointsError>>> {
        self.deref().batch_get_dev_endpoints(builder)
    }
    fn batch_get_jobs(&self, builder: BatchGetJobsInputBuilder) -> impl Future<Output = Result<BatchGetJobsOutput, SdkError<BatchGetJobsError>>> {
        self.deref().batch_get_jobs(builder)
    }
    fn batch_get_partition(&self, builder: BatchGetPartitionInputBuilder) -> impl Future<Output = Result<BatchGetPartitionOutput, SdkError<BatchGetPartitionError>>> {
        self.deref().batch_get_partition(builder)
    }
    fn batch_get_table_optimizer(&self, builder: BatchGetTableOptimizerInputBuilder) -> impl Future<Output = Result<BatchGetTableOptimizerOutput, SdkError<BatchGetTableOptimizerError>>> {
        self.deref().batch_get_table_optimizer(builder)
    }
    fn batch_get_triggers(&self, builder: BatchGetTriggersInputBuilder) -> impl Future<Output = Result<BatchGetTriggersOutput, SdkError<BatchGetTriggersError>>> {
        self.deref().batch_get_triggers(builder)
    }
    fn batch_get_workflows(&self, builder: BatchGetWorkflowsInputBuilder) -> impl Future<Output = Result<BatchGetWorkflowsOutput, SdkError<BatchGetWorkflowsError>>> {
        self.deref().batch_get_workflows(builder)
    }
    fn batch_stop_job_run(&self, builder: BatchStopJobRunInputBuilder) -> impl Future<Output = Result<BatchStopJobRunOutput, SdkError<BatchStopJobRunError>>> {
        self.deref().batch_stop_job_run(builder)
    }
    fn batch_update_partition(&self, builder: BatchUpdatePartitionInputBuilder) -> impl Future<Output = Result<BatchUpdatePartitionOutput, SdkError<BatchUpdatePartitionError>>> {
        self.deref().batch_update_partition(builder)
    }
    fn cancel_data_quality_rule_recommendation_run(&self, builder: CancelDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRuleRecommendationRunOutput, SdkError<CancelDataQualityRuleRecommendationRunError>>> {
        self.deref().cancel_data_quality_rule_recommendation_run(builder)
    }
    fn cancel_data_quality_ruleset_evaluation_run(&self, builder: CancelDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<CancelDataQualityRulesetEvaluationRunOutput, SdkError<CancelDataQualityRulesetEvaluationRunError>>> {
        self.deref().cancel_data_quality_ruleset_evaluation_run(builder)
    }
    fn cancel_ml_task_run(&self, builder: CancelMlTaskRunInputBuilder) -> impl Future<Output = Result<CancelMlTaskRunOutput, SdkError<CancelMLTaskRunError>>> {
        self.deref().cancel_ml_task_run(builder)
    }
    fn cancel_statement(&self, builder: CancelStatementInputBuilder) -> impl Future<Output = Result<CancelStatementOutput, SdkError<CancelStatementError>>> {
        self.deref().cancel_statement(builder)
    }
    fn check_schema_version_validity(&self, builder: CheckSchemaVersionValidityInputBuilder) -> impl Future<Output = Result<CheckSchemaVersionValidityOutput, SdkError<CheckSchemaVersionValidityError>>> {
        self.deref().check_schema_version_validity(builder)
    }
    fn create_blueprint(&self, builder: CreateBlueprintInputBuilder) -> impl Future<Output = Result<CreateBlueprintOutput, SdkError<CreateBlueprintError>>> {
        self.deref().create_blueprint(builder)
    }
    fn create_classifier(&self, builder: CreateClassifierInputBuilder) -> impl Future<Output = Result<CreateClassifierOutput, SdkError<CreateClassifierError>>> {
        self.deref().create_classifier(builder)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        self.deref().create_connection(builder)
    }
    fn create_crawler(&self, builder: CreateCrawlerInputBuilder) -> impl Future<Output = Result<CreateCrawlerOutput, SdkError<CreateCrawlerError>>> {
        self.deref().create_crawler(builder)
    }
    fn create_custom_entity_type(&self, builder: CreateCustomEntityTypeInputBuilder) -> impl Future<Output = Result<CreateCustomEntityTypeOutput, SdkError<CreateCustomEntityTypeError>>> {
        self.deref().create_custom_entity_type(builder)
    }
    fn create_data_quality_ruleset(&self, builder: CreateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<CreateDataQualityRulesetOutput, SdkError<CreateDataQualityRulesetError>>> {
        self.deref().create_data_quality_ruleset(builder)
    }
    fn create_database(&self, builder: CreateDatabaseInputBuilder) -> impl Future<Output = Result<CreateDatabaseOutput, SdkError<CreateDatabaseError>>> {
        self.deref().create_database(builder)
    }
    fn create_dev_endpoint(&self, builder: CreateDevEndpointInputBuilder) -> impl Future<Output = Result<CreateDevEndpointOutput, SdkError<CreateDevEndpointError>>> {
        self.deref().create_dev_endpoint(builder)
    }
    fn create_job(&self, builder: CreateJobInputBuilder) -> impl Future<Output = Result<CreateJobOutput, SdkError<CreateJobError>>> {
        self.deref().create_job(builder)
    }
    fn create_ml_transform(&self, builder: CreateMlTransformInputBuilder) -> impl Future<Output = Result<CreateMlTransformOutput, SdkError<CreateMLTransformError>>> {
        self.deref().create_ml_transform(builder)
    }
    fn create_partition(&self, builder: CreatePartitionInputBuilder) -> impl Future<Output = Result<CreatePartitionOutput, SdkError<CreatePartitionError>>> {
        self.deref().create_partition(builder)
    }
    fn create_partition_index(&self, builder: CreatePartitionIndexInputBuilder) -> impl Future<Output = Result<CreatePartitionIndexOutput, SdkError<CreatePartitionIndexError>>> {
        self.deref().create_partition_index(builder)
    }
    fn create_registry(&self, builder: CreateRegistryInputBuilder) -> impl Future<Output = Result<CreateRegistryOutput, SdkError<CreateRegistryError>>> {
        self.deref().create_registry(builder)
    }
    fn create_schema(&self, builder: CreateSchemaInputBuilder) -> impl Future<Output = Result<CreateSchemaOutput, SdkError<CreateSchemaError>>> {
        self.deref().create_schema(builder)
    }
    fn create_script(&self, builder: CreateScriptInputBuilder) -> impl Future<Output = Result<CreateScriptOutput, SdkError<CreateScriptError>>> {
        self.deref().create_script(builder)
    }
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>> {
        self.deref().create_security_configuration(builder)
    }
    fn create_session(&self, builder: CreateSessionInputBuilder) -> impl Future<Output = Result<CreateSessionOutput, SdkError<CreateSessionError>>> {
        self.deref().create_session(builder)
    }
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>> {
        self.deref().create_table(builder)
    }
    fn create_table_optimizer(&self, builder: CreateTableOptimizerInputBuilder) -> impl Future<Output = Result<CreateTableOptimizerOutput, SdkError<CreateTableOptimizerError>>> {
        self.deref().create_table_optimizer(builder)
    }
    fn create_trigger(&self, builder: CreateTriggerInputBuilder) -> impl Future<Output = Result<CreateTriggerOutput, SdkError<CreateTriggerError>>> {
        self.deref().create_trigger(builder)
    }
    fn create_usage_profile(&self, builder: CreateUsageProfileInputBuilder) -> impl Future<Output = Result<CreateUsageProfileOutput, SdkError<CreateUsageProfileError>>> {
        self.deref().create_usage_profile(builder)
    }
    fn create_user_defined_function(&self, builder: CreateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<CreateUserDefinedFunctionOutput, SdkError<CreateUserDefinedFunctionError>>> {
        self.deref().create_user_defined_function(builder)
    }
    fn create_workflow(&self, builder: CreateWorkflowInputBuilder) -> impl Future<Output = Result<CreateWorkflowOutput, SdkError<CreateWorkflowError>>> {
        self.deref().create_workflow(builder)
    }
    fn delete_blueprint(&self, builder: DeleteBlueprintInputBuilder) -> impl Future<Output = Result<DeleteBlueprintOutput, SdkError<DeleteBlueprintError>>> {
        self.deref().delete_blueprint(builder)
    }
    fn delete_classifier(&self, builder: DeleteClassifierInputBuilder) -> impl Future<Output = Result<DeleteClassifierOutput, SdkError<DeleteClassifierError>>> {
        self.deref().delete_classifier(builder)
    }
    fn delete_column_statistics_for_partition(&self, builder: DeleteColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForPartitionOutput, SdkError<DeleteColumnStatisticsForPartitionError>>> {
        self.deref().delete_column_statistics_for_partition(builder)
    }
    fn delete_column_statistics_for_table(&self, builder: DeleteColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<DeleteColumnStatisticsForTableOutput, SdkError<DeleteColumnStatisticsForTableError>>> {
        self.deref().delete_column_statistics_for_table(builder)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        self.deref().delete_connection(builder)
    }
    fn delete_crawler(&self, builder: DeleteCrawlerInputBuilder) -> impl Future<Output = Result<DeleteCrawlerOutput, SdkError<DeleteCrawlerError>>> {
        self.deref().delete_crawler(builder)
    }
    fn delete_custom_entity_type(&self, builder: DeleteCustomEntityTypeInputBuilder) -> impl Future<Output = Result<DeleteCustomEntityTypeOutput, SdkError<DeleteCustomEntityTypeError>>> {
        self.deref().delete_custom_entity_type(builder)
    }
    fn delete_data_quality_ruleset(&self, builder: DeleteDataQualityRulesetInputBuilder) -> impl Future<Output = Result<DeleteDataQualityRulesetOutput, SdkError<DeleteDataQualityRulesetError>>> {
        self.deref().delete_data_quality_ruleset(builder)
    }
    fn delete_database(&self, builder: DeleteDatabaseInputBuilder) -> impl Future<Output = Result<DeleteDatabaseOutput, SdkError<DeleteDatabaseError>>> {
        self.deref().delete_database(builder)
    }
    fn delete_dev_endpoint(&self, builder: DeleteDevEndpointInputBuilder) -> impl Future<Output = Result<DeleteDevEndpointOutput, SdkError<DeleteDevEndpointError>>> {
        self.deref().delete_dev_endpoint(builder)
    }
    fn delete_job(&self, builder: DeleteJobInputBuilder) -> impl Future<Output = Result<DeleteJobOutput, SdkError<DeleteJobError>>> {
        self.deref().delete_job(builder)
    }
    fn delete_ml_transform(&self, builder: DeleteMlTransformInputBuilder) -> impl Future<Output = Result<DeleteMlTransformOutput, SdkError<DeleteMLTransformError>>> {
        self.deref().delete_ml_transform(builder)
    }
    fn delete_partition(&self, builder: DeletePartitionInputBuilder) -> impl Future<Output = Result<DeletePartitionOutput, SdkError<DeletePartitionError>>> {
        self.deref().delete_partition(builder)
    }
    fn delete_partition_index(&self, builder: DeletePartitionIndexInputBuilder) -> impl Future<Output = Result<DeletePartitionIndexOutput, SdkError<DeletePartitionIndexError>>> {
        self.deref().delete_partition_index(builder)
    }
    fn delete_registry(&self, builder: DeleteRegistryInputBuilder) -> impl Future<Output = Result<DeleteRegistryOutput, SdkError<DeleteRegistryError>>> {
        self.deref().delete_registry(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_schema(&self, builder: DeleteSchemaInputBuilder) -> impl Future<Output = Result<DeleteSchemaOutput, SdkError<DeleteSchemaError>>> {
        self.deref().delete_schema(builder)
    }
    fn delete_schema_versions(&self, builder: DeleteSchemaVersionsInputBuilder) -> impl Future<Output = Result<DeleteSchemaVersionsOutput, SdkError<DeleteSchemaVersionsError>>> {
        self.deref().delete_schema_versions(builder)
    }
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>> {
        self.deref().delete_security_configuration(builder)
    }
    fn delete_session(&self, builder: DeleteSessionInputBuilder) -> impl Future<Output = Result<DeleteSessionOutput, SdkError<DeleteSessionError>>> {
        self.deref().delete_session(builder)
    }
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>> {
        self.deref().delete_table(builder)
    }
    fn delete_table_optimizer(&self, builder: DeleteTableOptimizerInputBuilder) -> impl Future<Output = Result<DeleteTableOptimizerOutput, SdkError<DeleteTableOptimizerError>>> {
        self.deref().delete_table_optimizer(builder)
    }
    fn delete_table_version(&self, builder: DeleteTableVersionInputBuilder) -> impl Future<Output = Result<DeleteTableVersionOutput, SdkError<DeleteTableVersionError>>> {
        self.deref().delete_table_version(builder)
    }
    fn delete_trigger(&self, builder: DeleteTriggerInputBuilder) -> impl Future<Output = Result<DeleteTriggerOutput, SdkError<DeleteTriggerError>>> {
        self.deref().delete_trigger(builder)
    }
    fn delete_usage_profile(&self, builder: DeleteUsageProfileInputBuilder) -> impl Future<Output = Result<DeleteUsageProfileOutput, SdkError<DeleteUsageProfileError>>> {
        self.deref().delete_usage_profile(builder)
    }
    fn delete_user_defined_function(&self, builder: DeleteUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<DeleteUserDefinedFunctionOutput, SdkError<DeleteUserDefinedFunctionError>>> {
        self.deref().delete_user_defined_function(builder)
    }
    fn delete_workflow(&self, builder: DeleteWorkflowInputBuilder) -> impl Future<Output = Result<DeleteWorkflowOutput, SdkError<DeleteWorkflowError>>> {
        self.deref().delete_workflow(builder)
    }
    fn get_blueprint(&self, builder: GetBlueprintInputBuilder) -> impl Future<Output = Result<GetBlueprintOutput, SdkError<GetBlueprintError>>> {
        self.deref().get_blueprint(builder)
    }
    fn get_blueprint_run(&self, builder: GetBlueprintRunInputBuilder) -> impl Future<Output = Result<GetBlueprintRunOutput, SdkError<GetBlueprintRunError>>> {
        self.deref().get_blueprint_run(builder)
    }
    fn get_blueprint_runs(&self, builder: GetBlueprintRunsInputBuilder) -> impl Future<Output = Result<GetBlueprintRunsOutput, SdkError<GetBlueprintRunsError>>> {
        self.deref().get_blueprint_runs(builder)
    }
    fn get_catalog_import_status(&self, builder: GetCatalogImportStatusInputBuilder) -> impl Future<Output = Result<GetCatalogImportStatusOutput, SdkError<GetCatalogImportStatusError>>> {
        self.deref().get_catalog_import_status(builder)
    }
    fn get_classifier(&self, builder: GetClassifierInputBuilder) -> impl Future<Output = Result<GetClassifierOutput, SdkError<GetClassifierError>>> {
        self.deref().get_classifier(builder)
    }
    fn get_classifiers(&self, builder: GetClassifiersInputBuilder) -> impl Future<Output = Result<GetClassifiersOutput, SdkError<GetClassifiersError>>> {
        self.deref().get_classifiers(builder)
    }
    fn get_column_statistics_for_partition(&self, builder: GetColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForPartitionOutput, SdkError<GetColumnStatisticsForPartitionError>>> {
        self.deref().get_column_statistics_for_partition(builder)
    }
    fn get_column_statistics_for_table(&self, builder: GetColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsForTableOutput, SdkError<GetColumnStatisticsForTableError>>> {
        self.deref().get_column_statistics_for_table(builder)
    }
    fn get_column_statistics_task_run(&self, builder: GetColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunOutput, SdkError<GetColumnStatisticsTaskRunError>>> {
        self.deref().get_column_statistics_task_run(builder)
    }
    fn get_column_statistics_task_runs(&self, builder: GetColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<GetColumnStatisticsTaskRunsOutput, SdkError<GetColumnStatisticsTaskRunsError>>> {
        self.deref().get_column_statistics_task_runs(builder)
    }
    fn get_connection(&self, builder: GetConnectionInputBuilder) -> impl Future<Output = Result<GetConnectionOutput, SdkError<GetConnectionError>>> {
        self.deref().get_connection(builder)
    }
    fn get_connections(&self, builder: GetConnectionsInputBuilder) -> impl Future<Output = Result<GetConnectionsOutput, SdkError<GetConnectionsError>>> {
        self.deref().get_connections(builder)
    }
    fn get_crawler(&self, builder: GetCrawlerInputBuilder) -> impl Future<Output = Result<GetCrawlerOutput, SdkError<GetCrawlerError>>> {
        self.deref().get_crawler(builder)
    }
    fn get_crawler_metrics(&self, builder: GetCrawlerMetricsInputBuilder) -> impl Future<Output = Result<GetCrawlerMetricsOutput, SdkError<GetCrawlerMetricsError>>> {
        self.deref().get_crawler_metrics(builder)
    }
    fn get_crawlers(&self, builder: GetCrawlersInputBuilder) -> impl Future<Output = Result<GetCrawlersOutput, SdkError<GetCrawlersError>>> {
        self.deref().get_crawlers(builder)
    }
    fn get_custom_entity_type(&self, builder: GetCustomEntityTypeInputBuilder) -> impl Future<Output = Result<GetCustomEntityTypeOutput, SdkError<GetCustomEntityTypeError>>> {
        self.deref().get_custom_entity_type(builder)
    }
    fn get_data_catalog_encryption_settings(&self, builder: GetDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<GetDataCatalogEncryptionSettingsOutput, SdkError<GetDataCatalogEncryptionSettingsError>>> {
        self.deref().get_data_catalog_encryption_settings(builder)
    }
    fn get_data_quality_result(&self, builder: GetDataQualityResultInputBuilder) -> impl Future<Output = Result<GetDataQualityResultOutput, SdkError<GetDataQualityResultError>>> {
        self.deref().get_data_quality_result(builder)
    }
    fn get_data_quality_rule_recommendation_run(&self, builder: GetDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRuleRecommendationRunOutput, SdkError<GetDataQualityRuleRecommendationRunError>>> {
        self.deref().get_data_quality_rule_recommendation_run(builder)
    }
    fn get_data_quality_ruleset(&self, builder: GetDataQualityRulesetInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetOutput, SdkError<GetDataQualityRulesetError>>> {
        self.deref().get_data_quality_ruleset(builder)
    }
    fn get_data_quality_ruleset_evaluation_run(&self, builder: GetDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<GetDataQualityRulesetEvaluationRunOutput, SdkError<GetDataQualityRulesetEvaluationRunError>>> {
        self.deref().get_data_quality_ruleset_evaluation_run(builder)
    }
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>> {
        self.deref().get_database(builder)
    }
    fn get_databases(&self, builder: GetDatabasesInputBuilder) -> impl Future<Output = Result<GetDatabasesOutput, SdkError<GetDatabasesError>>> {
        self.deref().get_databases(builder)
    }
    fn get_dataflow_graph(&self, builder: GetDataflowGraphInputBuilder) -> impl Future<Output = Result<GetDataflowGraphOutput, SdkError<GetDataflowGraphError>>> {
        self.deref().get_dataflow_graph(builder)
    }
    fn get_dev_endpoint(&self, builder: GetDevEndpointInputBuilder) -> impl Future<Output = Result<GetDevEndpointOutput, SdkError<GetDevEndpointError>>> {
        self.deref().get_dev_endpoint(builder)
    }
    fn get_dev_endpoints(&self, builder: GetDevEndpointsInputBuilder) -> impl Future<Output = Result<GetDevEndpointsOutput, SdkError<GetDevEndpointsError>>> {
        self.deref().get_dev_endpoints(builder)
    }
    fn get_job(&self, builder: GetJobInputBuilder) -> impl Future<Output = Result<GetJobOutput, SdkError<GetJobError>>> {
        self.deref().get_job(builder)
    }
    fn get_job_bookmark(&self, builder: GetJobBookmarkInputBuilder) -> impl Future<Output = Result<GetJobBookmarkOutput, SdkError<GetJobBookmarkError>>> {
        self.deref().get_job_bookmark(builder)
    }
    fn get_job_run(&self, builder: GetJobRunInputBuilder) -> impl Future<Output = Result<GetJobRunOutput, SdkError<GetJobRunError>>> {
        self.deref().get_job_run(builder)
    }
    fn get_job_runs(&self, builder: GetJobRunsInputBuilder) -> impl Future<Output = Result<GetJobRunsOutput, SdkError<GetJobRunsError>>> {
        self.deref().get_job_runs(builder)
    }
    fn get_jobs(&self, builder: GetJobsInputBuilder) -> impl Future<Output = Result<GetJobsOutput, SdkError<GetJobsError>>> {
        self.deref().get_jobs(builder)
    }
    fn get_mapping(&self, builder: GetMappingInputBuilder) -> impl Future<Output = Result<GetMappingOutput, SdkError<GetMappingError>>> {
        self.deref().get_mapping(builder)
    }
    fn get_ml_task_run(&self, builder: GetMlTaskRunInputBuilder) -> impl Future<Output = Result<GetMlTaskRunOutput, SdkError<GetMLTaskRunError>>> {
        self.deref().get_ml_task_run(builder)
    }
    fn get_ml_task_runs(&self, builder: GetMlTaskRunsInputBuilder) -> impl Future<Output = Result<GetMlTaskRunsOutput, SdkError<GetMLTaskRunsError>>> {
        self.deref().get_ml_task_runs(builder)
    }
    fn get_ml_transform(&self, builder: GetMlTransformInputBuilder) -> impl Future<Output = Result<GetMlTransformOutput, SdkError<GetMLTransformError>>> {
        self.deref().get_ml_transform(builder)
    }
    fn get_ml_transforms(&self, builder: GetMlTransformsInputBuilder) -> impl Future<Output = Result<GetMlTransformsOutput, SdkError<GetMLTransformsError>>> {
        self.deref().get_ml_transforms(builder)
    }
    fn get_partition(&self, builder: GetPartitionInputBuilder) -> impl Future<Output = Result<GetPartitionOutput, SdkError<GetPartitionError>>> {
        self.deref().get_partition(builder)
    }
    fn get_partition_indexes(&self, builder: GetPartitionIndexesInputBuilder) -> impl Future<Output = Result<GetPartitionIndexesOutput, SdkError<GetPartitionIndexesError>>> {
        self.deref().get_partition_indexes(builder)
    }
    fn get_partitions(&self, builder: GetPartitionsInputBuilder) -> impl Future<Output = Result<GetPartitionsOutput, SdkError<GetPartitionsError>>> {
        self.deref().get_partitions(builder)
    }
    fn get_plan(&self, builder: GetPlanInputBuilder) -> impl Future<Output = Result<GetPlanOutput, SdkError<GetPlanError>>> {
        self.deref().get_plan(builder)
    }
    fn get_registry(&self, builder: GetRegistryInputBuilder) -> impl Future<Output = Result<GetRegistryOutput, SdkError<GetRegistryError>>> {
        self.deref().get_registry(builder)
    }
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>> {
        self.deref().get_resource_policies(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        self.deref().get_resource_policy(builder)
    }
    fn get_schema(&self, builder: GetSchemaInputBuilder) -> impl Future<Output = Result<GetSchemaOutput, SdkError<GetSchemaError>>> {
        self.deref().get_schema(builder)
    }
    fn get_schema_by_definition(&self, builder: GetSchemaByDefinitionInputBuilder) -> impl Future<Output = Result<GetSchemaByDefinitionOutput, SdkError<GetSchemaByDefinitionError>>> {
        self.deref().get_schema_by_definition(builder)
    }
    fn get_schema_version(&self, builder: GetSchemaVersionInputBuilder) -> impl Future<Output = Result<GetSchemaVersionOutput, SdkError<GetSchemaVersionError>>> {
        self.deref().get_schema_version(builder)
    }
    fn get_schema_versions_diff(&self, builder: GetSchemaVersionsDiffInputBuilder) -> impl Future<Output = Result<GetSchemaVersionsDiffOutput, SdkError<GetSchemaVersionsDiffError>>> {
        self.deref().get_schema_versions_diff(builder)
    }
    fn get_security_configuration(&self, builder: GetSecurityConfigurationInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationOutput, SdkError<GetSecurityConfigurationError>>> {
        self.deref().get_security_configuration(builder)
    }
    fn get_security_configurations(&self, builder: GetSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<GetSecurityConfigurationsOutput, SdkError<GetSecurityConfigurationsError>>> {
        self.deref().get_security_configurations(builder)
    }
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>> {
        self.deref().get_session(builder)
    }
    fn get_statement(&self, builder: GetStatementInputBuilder) -> impl Future<Output = Result<GetStatementOutput, SdkError<GetStatementError>>> {
        self.deref().get_statement(builder)
    }
    fn get_table(&self, builder: GetTableInputBuilder) -> impl Future<Output = Result<GetTableOutput, SdkError<GetTableError>>> {
        self.deref().get_table(builder)
    }
    fn get_table_optimizer(&self, builder: GetTableOptimizerInputBuilder) -> impl Future<Output = Result<GetTableOptimizerOutput, SdkError<GetTableOptimizerError>>> {
        self.deref().get_table_optimizer(builder)
    }
    fn get_table_version(&self, builder: GetTableVersionInputBuilder) -> impl Future<Output = Result<GetTableVersionOutput, SdkError<GetTableVersionError>>> {
        self.deref().get_table_version(builder)
    }
    fn get_table_versions(&self, builder: GetTableVersionsInputBuilder) -> impl Future<Output = Result<GetTableVersionsOutput, SdkError<GetTableVersionsError>>> {
        self.deref().get_table_versions(builder)
    }
    fn get_tables(&self, builder: GetTablesInputBuilder) -> impl Future<Output = Result<GetTablesOutput, SdkError<GetTablesError>>> {
        self.deref().get_tables(builder)
    }
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> {
        self.deref().get_tags(builder)
    }
    fn get_trigger(&self, builder: GetTriggerInputBuilder) -> impl Future<Output = Result<GetTriggerOutput, SdkError<GetTriggerError>>> {
        self.deref().get_trigger(builder)
    }
    fn get_triggers(&self, builder: GetTriggersInputBuilder) -> impl Future<Output = Result<GetTriggersOutput, SdkError<GetTriggersError>>> {
        self.deref().get_triggers(builder)
    }
    fn get_unfiltered_partition_metadata(&self, builder: GetUnfilteredPartitionMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionMetadataOutput, SdkError<GetUnfilteredPartitionMetadataError>>> {
        self.deref().get_unfiltered_partition_metadata(builder)
    }
    fn get_unfiltered_partitions_metadata(&self, builder: GetUnfilteredPartitionsMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredPartitionsMetadataOutput, SdkError<GetUnfilteredPartitionsMetadataError>>> {
        self.deref().get_unfiltered_partitions_metadata(builder)
    }
    fn get_unfiltered_table_metadata(&self, builder: GetUnfilteredTableMetadataInputBuilder) -> impl Future<Output = Result<GetUnfilteredTableMetadataOutput, SdkError<GetUnfilteredTableMetadataError>>> {
        self.deref().get_unfiltered_table_metadata(builder)
    }
    fn get_usage_profile(&self, builder: GetUsageProfileInputBuilder) -> impl Future<Output = Result<GetUsageProfileOutput, SdkError<GetUsageProfileError>>> {
        self.deref().get_usage_profile(builder)
    }
    fn get_user_defined_function(&self, builder: GetUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionOutput, SdkError<GetUserDefinedFunctionError>>> {
        self.deref().get_user_defined_function(builder)
    }
    fn get_user_defined_functions(&self, builder: GetUserDefinedFunctionsInputBuilder) -> impl Future<Output = Result<GetUserDefinedFunctionsOutput, SdkError<GetUserDefinedFunctionsError>>> {
        self.deref().get_user_defined_functions(builder)
    }
    fn get_workflow(&self, builder: GetWorkflowInputBuilder) -> impl Future<Output = Result<GetWorkflowOutput, SdkError<GetWorkflowError>>> {
        self.deref().get_workflow(builder)
    }
    fn get_workflow_run(&self, builder: GetWorkflowRunInputBuilder) -> impl Future<Output = Result<GetWorkflowRunOutput, SdkError<GetWorkflowRunError>>> {
        self.deref().get_workflow_run(builder)
    }
    fn get_workflow_run_properties(&self, builder: GetWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<GetWorkflowRunPropertiesOutput, SdkError<GetWorkflowRunPropertiesError>>> {
        self.deref().get_workflow_run_properties(builder)
    }
    fn get_workflow_runs(&self, builder: GetWorkflowRunsInputBuilder) -> impl Future<Output = Result<GetWorkflowRunsOutput, SdkError<GetWorkflowRunsError>>> {
        self.deref().get_workflow_runs(builder)
    }
    fn import_catalog_to_glue(&self, builder: ImportCatalogToGlueInputBuilder) -> impl Future<Output = Result<ImportCatalogToGlueOutput, SdkError<ImportCatalogToGlueError>>> {
        self.deref().import_catalog_to_glue(builder)
    }
    fn list_blueprints(&self, builder: ListBlueprintsInputBuilder) -> impl Future<Output = Result<ListBlueprintsOutput, SdkError<ListBlueprintsError>>> {
        self.deref().list_blueprints(builder)
    }
    fn list_column_statistics_task_runs(&self, builder: ListColumnStatisticsTaskRunsInputBuilder) -> impl Future<Output = Result<ListColumnStatisticsTaskRunsOutput, SdkError<ListColumnStatisticsTaskRunsError>>> {
        self.deref().list_column_statistics_task_runs(builder)
    }
    fn list_crawlers(&self, builder: ListCrawlersInputBuilder) -> impl Future<Output = Result<ListCrawlersOutput, SdkError<ListCrawlersError>>> {
        self.deref().list_crawlers(builder)
    }
    fn list_crawls(&self, builder: ListCrawlsInputBuilder) -> impl Future<Output = Result<ListCrawlsOutput, SdkError<ListCrawlsError>>> {
        self.deref().list_crawls(builder)
    }
    fn list_custom_entity_types(&self, builder: ListCustomEntityTypesInputBuilder) -> impl Future<Output = Result<ListCustomEntityTypesOutput, SdkError<ListCustomEntityTypesError>>> {
        self.deref().list_custom_entity_types(builder)
    }
    fn list_data_quality_results(&self, builder: ListDataQualityResultsInputBuilder) -> impl Future<Output = Result<ListDataQualityResultsOutput, SdkError<ListDataQualityResultsError>>> {
        self.deref().list_data_quality_results(builder)
    }
    fn list_data_quality_rule_recommendation_runs(&self, builder: ListDataQualityRuleRecommendationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRuleRecommendationRunsOutput, SdkError<ListDataQualityRuleRecommendationRunsError>>> {
        self.deref().list_data_quality_rule_recommendation_runs(builder)
    }
    fn list_data_quality_ruleset_evaluation_runs(&self, builder: ListDataQualityRulesetEvaluationRunsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetEvaluationRunsOutput, SdkError<ListDataQualityRulesetEvaluationRunsError>>> {
        self.deref().list_data_quality_ruleset_evaluation_runs(builder)
    }
    fn list_data_quality_rulesets(&self, builder: ListDataQualityRulesetsInputBuilder) -> impl Future<Output = Result<ListDataQualityRulesetsOutput, SdkError<ListDataQualityRulesetsError>>> {
        self.deref().list_data_quality_rulesets(builder)
    }
    fn list_dev_endpoints(&self, builder: ListDevEndpointsInputBuilder) -> impl Future<Output = Result<ListDevEndpointsOutput, SdkError<ListDevEndpointsError>>> {
        self.deref().list_dev_endpoints(builder)
    }
    fn list_jobs(&self, builder: ListJobsInputBuilder) -> impl Future<Output = Result<ListJobsOutput, SdkError<ListJobsError>>> {
        self.deref().list_jobs(builder)
    }
    fn list_ml_transforms(&self, builder: ListMlTransformsInputBuilder) -> impl Future<Output = Result<ListMlTransformsOutput, SdkError<ListMLTransformsError>>> {
        self.deref().list_ml_transforms(builder)
    }
    fn list_registries(&self, builder: ListRegistriesInputBuilder) -> impl Future<Output = Result<ListRegistriesOutput, SdkError<ListRegistriesError>>> {
        self.deref().list_registries(builder)
    }
    fn list_schema_versions(&self, builder: ListSchemaVersionsInputBuilder) -> impl Future<Output = Result<ListSchemaVersionsOutput, SdkError<ListSchemaVersionsError>>> {
        self.deref().list_schema_versions(builder)
    }
    fn list_schemas(&self, builder: ListSchemasInputBuilder) -> impl Future<Output = Result<ListSchemasOutput, SdkError<ListSchemasError>>> {
        self.deref().list_schemas(builder)
    }
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>> {
        self.deref().list_sessions(builder)
    }
    fn list_statements(&self, builder: ListStatementsInputBuilder) -> impl Future<Output = Result<ListStatementsOutput, SdkError<ListStatementsError>>> {
        self.deref().list_statements(builder)
    }
    fn list_table_optimizer_runs(&self, builder: ListTableOptimizerRunsInputBuilder) -> impl Future<Output = Result<ListTableOptimizerRunsOutput, SdkError<ListTableOptimizerRunsError>>> {
        self.deref().list_table_optimizer_runs(builder)
    }
    fn list_triggers(&self, builder: ListTriggersInputBuilder) -> impl Future<Output = Result<ListTriggersOutput, SdkError<ListTriggersError>>> {
        self.deref().list_triggers(builder)
    }
    fn list_usage_profiles(&self, builder: ListUsageProfilesInputBuilder) -> impl Future<Output = Result<ListUsageProfilesOutput, SdkError<ListUsageProfilesError>>> {
        self.deref().list_usage_profiles(builder)
    }
    fn list_workflows(&self, builder: ListWorkflowsInputBuilder) -> impl Future<Output = Result<ListWorkflowsOutput, SdkError<ListWorkflowsError>>> {
        self.deref().list_workflows(builder)
    }
    fn put_data_catalog_encryption_settings(&self, builder: PutDataCatalogEncryptionSettingsInputBuilder) -> impl Future<Output = Result<PutDataCatalogEncryptionSettingsOutput, SdkError<PutDataCatalogEncryptionSettingsError>>> {
        self.deref().put_data_catalog_encryption_settings(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn put_schema_version_metadata(&self, builder: PutSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<PutSchemaVersionMetadataOutput, SdkError<PutSchemaVersionMetadataError>>> {
        self.deref().put_schema_version_metadata(builder)
    }
    fn put_workflow_run_properties(&self, builder: PutWorkflowRunPropertiesInputBuilder) -> impl Future<Output = Result<PutWorkflowRunPropertiesOutput, SdkError<PutWorkflowRunPropertiesError>>> {
        self.deref().put_workflow_run_properties(builder)
    }
    fn query_schema_version_metadata(&self, builder: QuerySchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<QuerySchemaVersionMetadataOutput, SdkError<QuerySchemaVersionMetadataError>>> {
        self.deref().query_schema_version_metadata(builder)
    }
    fn register_schema_version(&self, builder: RegisterSchemaVersionInputBuilder) -> impl Future<Output = Result<RegisterSchemaVersionOutput, SdkError<RegisterSchemaVersionError>>> {
        self.deref().register_schema_version(builder)
    }
    fn remove_schema_version_metadata(&self, builder: RemoveSchemaVersionMetadataInputBuilder) -> impl Future<Output = Result<RemoveSchemaVersionMetadataOutput, SdkError<RemoveSchemaVersionMetadataError>>> {
        self.deref().remove_schema_version_metadata(builder)
    }
    fn reset_job_bookmark(&self, builder: ResetJobBookmarkInputBuilder) -> impl Future<Output = Result<ResetJobBookmarkOutput, SdkError<ResetJobBookmarkError>>> {
        self.deref().reset_job_bookmark(builder)
    }
    fn resume_workflow_run(&self, builder: ResumeWorkflowRunInputBuilder) -> impl Future<Output = Result<ResumeWorkflowRunOutput, SdkError<ResumeWorkflowRunError>>> {
        self.deref().resume_workflow_run(builder)
    }
    fn run_statement(&self, builder: RunStatementInputBuilder) -> impl Future<Output = Result<RunStatementOutput, SdkError<RunStatementError>>> {
        self.deref().run_statement(builder)
    }
    fn search_tables(&self, builder: SearchTablesInputBuilder) -> impl Future<Output = Result<SearchTablesOutput, SdkError<SearchTablesError>>> {
        self.deref().search_tables(builder)
    }
    fn start_blueprint_run(&self, builder: StartBlueprintRunInputBuilder) -> impl Future<Output = Result<StartBlueprintRunOutput, SdkError<StartBlueprintRunError>>> {
        self.deref().start_blueprint_run(builder)
    }
    fn start_column_statistics_task_run(&self, builder: StartColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StartColumnStatisticsTaskRunOutput, SdkError<StartColumnStatisticsTaskRunError>>> {
        self.deref().start_column_statistics_task_run(builder)
    }
    fn start_crawler(&self, builder: StartCrawlerInputBuilder) -> impl Future<Output = Result<StartCrawlerOutput, SdkError<StartCrawlerError>>> {
        self.deref().start_crawler(builder)
    }
    fn start_crawler_schedule(&self, builder: StartCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StartCrawlerScheduleOutput, SdkError<StartCrawlerScheduleError>>> {
        self.deref().start_crawler_schedule(builder)
    }
    fn start_data_quality_rule_recommendation_run(&self, builder: StartDataQualityRuleRecommendationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRuleRecommendationRunOutput, SdkError<StartDataQualityRuleRecommendationRunError>>> {
        self.deref().start_data_quality_rule_recommendation_run(builder)
    }
    fn start_data_quality_ruleset_evaluation_run(&self, builder: StartDataQualityRulesetEvaluationRunInputBuilder) -> impl Future<Output = Result<StartDataQualityRulesetEvaluationRunOutput, SdkError<StartDataQualityRulesetEvaluationRunError>>> {
        self.deref().start_data_quality_ruleset_evaluation_run(builder)
    }
    fn start_export_labels_task_run(&self, builder: StartExportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartExportLabelsTaskRunOutput, SdkError<StartExportLabelsTaskRunError>>> {
        self.deref().start_export_labels_task_run(builder)
    }
    fn start_import_labels_task_run(&self, builder: StartImportLabelsTaskRunInputBuilder) -> impl Future<Output = Result<StartImportLabelsTaskRunOutput, SdkError<StartImportLabelsTaskRunError>>> {
        self.deref().start_import_labels_task_run(builder)
    }
    fn start_job_run(&self, builder: StartJobRunInputBuilder) -> impl Future<Output = Result<StartJobRunOutput, SdkError<StartJobRunError>>> {
        self.deref().start_job_run(builder)
    }
    fn start_ml_evaluation_task_run(&self, builder: StartMlEvaluationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlEvaluationTaskRunOutput, SdkError<StartMLEvaluationTaskRunError>>> {
        self.deref().start_ml_evaluation_task_run(builder)
    }
    fn start_ml_labeling_set_generation_task_run(&self, builder: StartMlLabelingSetGenerationTaskRunInputBuilder) -> impl Future<Output = Result<StartMlLabelingSetGenerationTaskRunOutput, SdkError<StartMLLabelingSetGenerationTaskRunError>>> {
        self.deref().start_ml_labeling_set_generation_task_run(builder)
    }
    fn start_trigger(&self, builder: StartTriggerInputBuilder) -> impl Future<Output = Result<StartTriggerOutput, SdkError<StartTriggerError>>> {
        self.deref().start_trigger(builder)
    }
    fn start_workflow_run(&self, builder: StartWorkflowRunInputBuilder) -> impl Future<Output = Result<StartWorkflowRunOutput, SdkError<StartWorkflowRunError>>> {
        self.deref().start_workflow_run(builder)
    }
    fn stop_column_statistics_task_run(&self, builder: StopColumnStatisticsTaskRunInputBuilder) -> impl Future<Output = Result<StopColumnStatisticsTaskRunOutput, SdkError<StopColumnStatisticsTaskRunError>>> {
        self.deref().stop_column_statistics_task_run(builder)
    }
    fn stop_crawler(&self, builder: StopCrawlerInputBuilder) -> impl Future<Output = Result<StopCrawlerOutput, SdkError<StopCrawlerError>>> {
        self.deref().stop_crawler(builder)
    }
    fn stop_crawler_schedule(&self, builder: StopCrawlerScheduleInputBuilder) -> impl Future<Output = Result<StopCrawlerScheduleOutput, SdkError<StopCrawlerScheduleError>>> {
        self.deref().stop_crawler_schedule(builder)
    }
    fn stop_session(&self, builder: StopSessionInputBuilder) -> impl Future<Output = Result<StopSessionOutput, SdkError<StopSessionError>>> {
        self.deref().stop_session(builder)
    }
    fn stop_trigger(&self, builder: StopTriggerInputBuilder) -> impl Future<Output = Result<StopTriggerOutput, SdkError<StopTriggerError>>> {
        self.deref().stop_trigger(builder)
    }
    fn stop_workflow_run(&self, builder: StopWorkflowRunInputBuilder) -> impl Future<Output = Result<StopWorkflowRunOutput, SdkError<StopWorkflowRunError>>> {
        self.deref().stop_workflow_run(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_blueprint(&self, builder: UpdateBlueprintInputBuilder) -> impl Future<Output = Result<UpdateBlueprintOutput, SdkError<UpdateBlueprintError>>> {
        self.deref().update_blueprint(builder)
    }
    fn update_classifier(&self, builder: UpdateClassifierInputBuilder) -> impl Future<Output = Result<UpdateClassifierOutput, SdkError<UpdateClassifierError>>> {
        self.deref().update_classifier(builder)
    }
    fn update_column_statistics_for_partition(&self, builder: UpdateColumnStatisticsForPartitionInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForPartitionOutput, SdkError<UpdateColumnStatisticsForPartitionError>>> {
        self.deref().update_column_statistics_for_partition(builder)
    }
    fn update_column_statistics_for_table(&self, builder: UpdateColumnStatisticsForTableInputBuilder) -> impl Future<Output = Result<UpdateColumnStatisticsForTableOutput, SdkError<UpdateColumnStatisticsForTableError>>> {
        self.deref().update_column_statistics_for_table(builder)
    }
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>> {
        self.deref().update_connection(builder)
    }
    fn update_crawler(&self, builder: UpdateCrawlerInputBuilder) -> impl Future<Output = Result<UpdateCrawlerOutput, SdkError<UpdateCrawlerError>>> {
        self.deref().update_crawler(builder)
    }
    fn update_crawler_schedule(&self, builder: UpdateCrawlerScheduleInputBuilder) -> impl Future<Output = Result<UpdateCrawlerScheduleOutput, SdkError<UpdateCrawlerScheduleError>>> {
        self.deref().update_crawler_schedule(builder)
    }
    fn update_data_quality_ruleset(&self, builder: UpdateDataQualityRulesetInputBuilder) -> impl Future<Output = Result<UpdateDataQualityRulesetOutput, SdkError<UpdateDataQualityRulesetError>>> {
        self.deref().update_data_quality_ruleset(builder)
    }
    fn update_database(&self, builder: UpdateDatabaseInputBuilder) -> impl Future<Output = Result<UpdateDatabaseOutput, SdkError<UpdateDatabaseError>>> {
        self.deref().update_database(builder)
    }
    fn update_dev_endpoint(&self, builder: UpdateDevEndpointInputBuilder) -> impl Future<Output = Result<UpdateDevEndpointOutput, SdkError<UpdateDevEndpointError>>> {
        self.deref().update_dev_endpoint(builder)
    }
    fn update_job(&self, builder: UpdateJobInputBuilder) -> impl Future<Output = Result<UpdateJobOutput, SdkError<UpdateJobError>>> {
        self.deref().update_job(builder)
    }
    fn update_job_from_source_control(&self, builder: UpdateJobFromSourceControlInputBuilder) -> impl Future<Output = Result<UpdateJobFromSourceControlOutput, SdkError<UpdateJobFromSourceControlError>>> {
        self.deref().update_job_from_source_control(builder)
    }
    fn update_ml_transform(&self, builder: UpdateMlTransformInputBuilder) -> impl Future<Output = Result<UpdateMlTransformOutput, SdkError<UpdateMLTransformError>>> {
        self.deref().update_ml_transform(builder)
    }
    fn update_partition(&self, builder: UpdatePartitionInputBuilder) -> impl Future<Output = Result<UpdatePartitionOutput, SdkError<UpdatePartitionError>>> {
        self.deref().update_partition(builder)
    }
    fn update_registry(&self, builder: UpdateRegistryInputBuilder) -> impl Future<Output = Result<UpdateRegistryOutput, SdkError<UpdateRegistryError>>> {
        self.deref().update_registry(builder)
    }
    fn update_schema(&self, builder: UpdateSchemaInputBuilder) -> impl Future<Output = Result<UpdateSchemaOutput, SdkError<UpdateSchemaError>>> {
        self.deref().update_schema(builder)
    }
    fn update_source_control_from_job(&self, builder: UpdateSourceControlFromJobInputBuilder) -> impl Future<Output = Result<UpdateSourceControlFromJobOutput, SdkError<UpdateSourceControlFromJobError>>> {
        self.deref().update_source_control_from_job(builder)
    }
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>> {
        self.deref().update_table(builder)
    }
    fn update_table_optimizer(&self, builder: UpdateTableOptimizerInputBuilder) -> impl Future<Output = Result<UpdateTableOptimizerOutput, SdkError<UpdateTableOptimizerError>>> {
        self.deref().update_table_optimizer(builder)
    }
    fn update_trigger(&self, builder: UpdateTriggerInputBuilder) -> impl Future<Output = Result<UpdateTriggerOutput, SdkError<UpdateTriggerError>>> {
        self.deref().update_trigger(builder)
    }
    fn update_usage_profile(&self, builder: UpdateUsageProfileInputBuilder) -> impl Future<Output = Result<UpdateUsageProfileOutput, SdkError<UpdateUsageProfileError>>> {
        self.deref().update_usage_profile(builder)
    }
    fn update_user_defined_function(&self, builder: UpdateUserDefinedFunctionInputBuilder) -> impl Future<Output = Result<UpdateUserDefinedFunctionOutput, SdkError<UpdateUserDefinedFunctionError>>> {
        self.deref().update_user_defined_function(builder)
    }
    fn update_workflow(&self, builder: UpdateWorkflowInputBuilder) -> impl Future<Output = Result<UpdateWorkflowOutput, SdkError<UpdateWorkflowError>>> {
        self.deref().update_workflow(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edGlueClient {}
    impl GlueClient for edGlueClient {
        async fn batch_create_partition(&self, builder: BatchCreatePartitionInputBuilder) -> Result<BatchCreatePartitionOutput, SdkError<BatchCreatePartitionError>>;
        async fn batch_delete_connection(&self, builder: BatchDeleteConnectionInputBuilder) -> Result<BatchDeleteConnectionOutput, SdkError<BatchDeleteConnectionError>>;
        async fn batch_delete_partition(&self, builder: BatchDeletePartitionInputBuilder) -> Result<BatchDeletePartitionOutput, SdkError<BatchDeletePartitionError>>;
        async fn batch_delete_table(&self, builder: BatchDeleteTableInputBuilder) -> Result<BatchDeleteTableOutput, SdkError<BatchDeleteTableError>>;
        async fn batch_delete_table_version(&self, builder: BatchDeleteTableVersionInputBuilder) -> Result<BatchDeleteTableVersionOutput, SdkError<BatchDeleteTableVersionError>>;
        async fn batch_get_blueprints(&self, builder: BatchGetBlueprintsInputBuilder) -> Result<BatchGetBlueprintsOutput, SdkError<BatchGetBlueprintsError>>;
        async fn batch_get_crawlers(&self, builder: BatchGetCrawlersInputBuilder) -> Result<BatchGetCrawlersOutput, SdkError<BatchGetCrawlersError>>;
        async fn batch_get_custom_entity_types(&self, builder: BatchGetCustomEntityTypesInputBuilder) -> Result<BatchGetCustomEntityTypesOutput, SdkError<BatchGetCustomEntityTypesError>>;
        async fn batch_get_data_quality_result(&self, builder: BatchGetDataQualityResultInputBuilder) -> Result<BatchGetDataQualityResultOutput, SdkError<BatchGetDataQualityResultError>>;
        async fn batch_get_dev_endpoints(&self, builder: BatchGetDevEndpointsInputBuilder) -> Result<BatchGetDevEndpointsOutput, SdkError<BatchGetDevEndpointsError>>;
        async fn batch_get_jobs(&self, builder: BatchGetJobsInputBuilder) -> Result<BatchGetJobsOutput, SdkError<BatchGetJobsError>>;
        async fn batch_get_partition(&self, builder: BatchGetPartitionInputBuilder) -> Result<BatchGetPartitionOutput, SdkError<BatchGetPartitionError>>;
        async fn batch_get_table_optimizer(&self, builder: BatchGetTableOptimizerInputBuilder) -> Result<BatchGetTableOptimizerOutput, SdkError<BatchGetTableOptimizerError>>;
        async fn batch_get_triggers(&self, builder: BatchGetTriggersInputBuilder) -> Result<BatchGetTriggersOutput, SdkError<BatchGetTriggersError>>;
        async fn batch_get_workflows(&self, builder: BatchGetWorkflowsInputBuilder) -> Result<BatchGetWorkflowsOutput, SdkError<BatchGetWorkflowsError>>;
        async fn batch_stop_job_run(&self, builder: BatchStopJobRunInputBuilder) -> Result<BatchStopJobRunOutput, SdkError<BatchStopJobRunError>>;
        async fn batch_update_partition(&self, builder: BatchUpdatePartitionInputBuilder) -> Result<BatchUpdatePartitionOutput, SdkError<BatchUpdatePartitionError>>;
        async fn cancel_data_quality_rule_recommendation_run(&self, builder: CancelDataQualityRuleRecommendationRunInputBuilder) -> Result<CancelDataQualityRuleRecommendationRunOutput, SdkError<CancelDataQualityRuleRecommendationRunError>>;
        async fn cancel_data_quality_ruleset_evaluation_run(&self, builder: CancelDataQualityRulesetEvaluationRunInputBuilder) -> Result<CancelDataQualityRulesetEvaluationRunOutput, SdkError<CancelDataQualityRulesetEvaluationRunError>>;
        async fn cancel_ml_task_run(&self, builder: CancelMlTaskRunInputBuilder) -> Result<CancelMlTaskRunOutput, SdkError<CancelMLTaskRunError>>;
        async fn cancel_statement(&self, builder: CancelStatementInputBuilder) -> Result<CancelStatementOutput, SdkError<CancelStatementError>>;
        async fn check_schema_version_validity(&self, builder: CheckSchemaVersionValidityInputBuilder) -> Result<CheckSchemaVersionValidityOutput, SdkError<CheckSchemaVersionValidityError>>;
        async fn create_blueprint(&self, builder: CreateBlueprintInputBuilder) -> Result<CreateBlueprintOutput, SdkError<CreateBlueprintError>>;
        async fn create_classifier(&self, builder: CreateClassifierInputBuilder) -> Result<CreateClassifierOutput, SdkError<CreateClassifierError>>;
        async fn create_connection(&self, builder: CreateConnectionInputBuilder) -> Result<CreateConnectionOutput, SdkError<CreateConnectionError>>;
        async fn create_crawler(&self, builder: CreateCrawlerInputBuilder) -> Result<CreateCrawlerOutput, SdkError<CreateCrawlerError>>;
        async fn create_custom_entity_type(&self, builder: CreateCustomEntityTypeInputBuilder) -> Result<CreateCustomEntityTypeOutput, SdkError<CreateCustomEntityTypeError>>;
        async fn create_data_quality_ruleset(&self, builder: CreateDataQualityRulesetInputBuilder) -> Result<CreateDataQualityRulesetOutput, SdkError<CreateDataQualityRulesetError>>;
        async fn create_database(&self, builder: CreateDatabaseInputBuilder) -> Result<CreateDatabaseOutput, SdkError<CreateDatabaseError>>;
        async fn create_dev_endpoint(&self, builder: CreateDevEndpointInputBuilder) -> Result<CreateDevEndpointOutput, SdkError<CreateDevEndpointError>>;
        async fn create_job(&self, builder: CreateJobInputBuilder) -> Result<CreateJobOutput, SdkError<CreateJobError>>;
        async fn create_ml_transform(&self, builder: CreateMlTransformInputBuilder) -> Result<CreateMlTransformOutput, SdkError<CreateMLTransformError>>;
        async fn create_partition(&self, builder: CreatePartitionInputBuilder) -> Result<CreatePartitionOutput, SdkError<CreatePartitionError>>;
        async fn create_partition_index(&self, builder: CreatePartitionIndexInputBuilder) -> Result<CreatePartitionIndexOutput, SdkError<CreatePartitionIndexError>>;
        async fn create_registry(&self, builder: CreateRegistryInputBuilder) -> Result<CreateRegistryOutput, SdkError<CreateRegistryError>>;
        async fn create_schema(&self, builder: CreateSchemaInputBuilder) -> Result<CreateSchemaOutput, SdkError<CreateSchemaError>>;
        async fn create_script(&self, builder: CreateScriptInputBuilder) -> Result<CreateScriptOutput, SdkError<CreateScriptError>>;
        async fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>;
        async fn create_session(&self, builder: CreateSessionInputBuilder) -> Result<CreateSessionOutput, SdkError<CreateSessionError>>;
        async fn create_table(&self, builder: CreateTableInputBuilder) -> Result<CreateTableOutput, SdkError<CreateTableError>>;
        async fn create_table_optimizer(&self, builder: CreateTableOptimizerInputBuilder) -> Result<CreateTableOptimizerOutput, SdkError<CreateTableOptimizerError>>;
        async fn create_trigger(&self, builder: CreateTriggerInputBuilder) -> Result<CreateTriggerOutput, SdkError<CreateTriggerError>>;
        async fn create_usage_profile(&self, builder: CreateUsageProfileInputBuilder) -> Result<CreateUsageProfileOutput, SdkError<CreateUsageProfileError>>;
        async fn create_user_defined_function(&self, builder: CreateUserDefinedFunctionInputBuilder) -> Result<CreateUserDefinedFunctionOutput, SdkError<CreateUserDefinedFunctionError>>;
        async fn create_workflow(&self, builder: CreateWorkflowInputBuilder) -> Result<CreateWorkflowOutput, SdkError<CreateWorkflowError>>;
        async fn delete_blueprint(&self, builder: DeleteBlueprintInputBuilder) -> Result<DeleteBlueprintOutput, SdkError<DeleteBlueprintError>>;
        async fn delete_classifier(&self, builder: DeleteClassifierInputBuilder) -> Result<DeleteClassifierOutput, SdkError<DeleteClassifierError>>;
        async fn delete_column_statistics_for_partition(&self, builder: DeleteColumnStatisticsForPartitionInputBuilder) -> Result<DeleteColumnStatisticsForPartitionOutput, SdkError<DeleteColumnStatisticsForPartitionError>>;
        async fn delete_column_statistics_for_table(&self, builder: DeleteColumnStatisticsForTableInputBuilder) -> Result<DeleteColumnStatisticsForTableOutput, SdkError<DeleteColumnStatisticsForTableError>>;
        async fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>;
        async fn delete_crawler(&self, builder: DeleteCrawlerInputBuilder) -> Result<DeleteCrawlerOutput, SdkError<DeleteCrawlerError>>;
        async fn delete_custom_entity_type(&self, builder: DeleteCustomEntityTypeInputBuilder) -> Result<DeleteCustomEntityTypeOutput, SdkError<DeleteCustomEntityTypeError>>;
        async fn delete_data_quality_ruleset(&self, builder: DeleteDataQualityRulesetInputBuilder) -> Result<DeleteDataQualityRulesetOutput, SdkError<DeleteDataQualityRulesetError>>;
        async fn delete_database(&self, builder: DeleteDatabaseInputBuilder) -> Result<DeleteDatabaseOutput, SdkError<DeleteDatabaseError>>;
        async fn delete_dev_endpoint(&self, builder: DeleteDevEndpointInputBuilder) -> Result<DeleteDevEndpointOutput, SdkError<DeleteDevEndpointError>>;
        async fn delete_job(&self, builder: DeleteJobInputBuilder) -> Result<DeleteJobOutput, SdkError<DeleteJobError>>;
        async fn delete_ml_transform(&self, builder: DeleteMlTransformInputBuilder) -> Result<DeleteMlTransformOutput, SdkError<DeleteMLTransformError>>;
        async fn delete_partition(&self, builder: DeletePartitionInputBuilder) -> Result<DeletePartitionOutput, SdkError<DeletePartitionError>>;
        async fn delete_partition_index(&self, builder: DeletePartitionIndexInputBuilder) -> Result<DeletePartitionIndexOutput, SdkError<DeletePartitionIndexError>>;
        async fn delete_registry(&self, builder: DeleteRegistryInputBuilder) -> Result<DeleteRegistryOutput, SdkError<DeleteRegistryError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_schema(&self, builder: DeleteSchemaInputBuilder) -> Result<DeleteSchemaOutput, SdkError<DeleteSchemaError>>;
        async fn delete_schema_versions(&self, builder: DeleteSchemaVersionsInputBuilder) -> Result<DeleteSchemaVersionsOutput, SdkError<DeleteSchemaVersionsError>>;
        async fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>;
        async fn delete_session(&self, builder: DeleteSessionInputBuilder) -> Result<DeleteSessionOutput, SdkError<DeleteSessionError>>;
        async fn delete_table(&self, builder: DeleteTableInputBuilder) -> Result<DeleteTableOutput, SdkError<DeleteTableError>>;
        async fn delete_table_optimizer(&self, builder: DeleteTableOptimizerInputBuilder) -> Result<DeleteTableOptimizerOutput, SdkError<DeleteTableOptimizerError>>;
        async fn delete_table_version(&self, builder: DeleteTableVersionInputBuilder) -> Result<DeleteTableVersionOutput, SdkError<DeleteTableVersionError>>;
        async fn delete_trigger(&self, builder: DeleteTriggerInputBuilder) -> Result<DeleteTriggerOutput, SdkError<DeleteTriggerError>>;
        async fn delete_usage_profile(&self, builder: DeleteUsageProfileInputBuilder) -> Result<DeleteUsageProfileOutput, SdkError<DeleteUsageProfileError>>;
        async fn delete_user_defined_function(&self, builder: DeleteUserDefinedFunctionInputBuilder) -> Result<DeleteUserDefinedFunctionOutput, SdkError<DeleteUserDefinedFunctionError>>;
        async fn delete_workflow(&self, builder: DeleteWorkflowInputBuilder) -> Result<DeleteWorkflowOutput, SdkError<DeleteWorkflowError>>;
        async fn get_blueprint(&self, builder: GetBlueprintInputBuilder) -> Result<GetBlueprintOutput, SdkError<GetBlueprintError>>;
        async fn get_blueprint_run(&self, builder: GetBlueprintRunInputBuilder) -> Result<GetBlueprintRunOutput, SdkError<GetBlueprintRunError>>;
        async fn get_blueprint_runs(&self, builder: GetBlueprintRunsInputBuilder) -> Result<GetBlueprintRunsOutput, SdkError<GetBlueprintRunsError>>;
        async fn get_catalog_import_status(&self, builder: GetCatalogImportStatusInputBuilder) -> Result<GetCatalogImportStatusOutput, SdkError<GetCatalogImportStatusError>>;
        async fn get_classifier(&self, builder: GetClassifierInputBuilder) -> Result<GetClassifierOutput, SdkError<GetClassifierError>>;
        async fn get_classifiers(&self, builder: GetClassifiersInputBuilder) -> Result<GetClassifiersOutput, SdkError<GetClassifiersError>>;
        async fn get_column_statistics_for_partition(&self, builder: GetColumnStatisticsForPartitionInputBuilder) -> Result<GetColumnStatisticsForPartitionOutput, SdkError<GetColumnStatisticsForPartitionError>>;
        async fn get_column_statistics_for_table(&self, builder: GetColumnStatisticsForTableInputBuilder) -> Result<GetColumnStatisticsForTableOutput, SdkError<GetColumnStatisticsForTableError>>;
        async fn get_column_statistics_task_run(&self, builder: GetColumnStatisticsTaskRunInputBuilder) -> Result<GetColumnStatisticsTaskRunOutput, SdkError<GetColumnStatisticsTaskRunError>>;
        async fn get_column_statistics_task_runs(&self, builder: GetColumnStatisticsTaskRunsInputBuilder) -> Result<GetColumnStatisticsTaskRunsOutput, SdkError<GetColumnStatisticsTaskRunsError>>;
        async fn get_connection(&self, builder: GetConnectionInputBuilder) -> Result<GetConnectionOutput, SdkError<GetConnectionError>>;
        async fn get_connections(&self, builder: GetConnectionsInputBuilder) -> Result<GetConnectionsOutput, SdkError<GetConnectionsError>>;
        async fn get_crawler(&self, builder: GetCrawlerInputBuilder) -> Result<GetCrawlerOutput, SdkError<GetCrawlerError>>;
        async fn get_crawler_metrics(&self, builder: GetCrawlerMetricsInputBuilder) -> Result<GetCrawlerMetricsOutput, SdkError<GetCrawlerMetricsError>>;
        async fn get_crawlers(&self, builder: GetCrawlersInputBuilder) -> Result<GetCrawlersOutput, SdkError<GetCrawlersError>>;
        async fn get_custom_entity_type(&self, builder: GetCustomEntityTypeInputBuilder) -> Result<GetCustomEntityTypeOutput, SdkError<GetCustomEntityTypeError>>;
        async fn get_data_catalog_encryption_settings(&self, builder: GetDataCatalogEncryptionSettingsInputBuilder) -> Result<GetDataCatalogEncryptionSettingsOutput, SdkError<GetDataCatalogEncryptionSettingsError>>;
        async fn get_data_quality_result(&self, builder: GetDataQualityResultInputBuilder) -> Result<GetDataQualityResultOutput, SdkError<GetDataQualityResultError>>;
        async fn get_data_quality_rule_recommendation_run(&self, builder: GetDataQualityRuleRecommendationRunInputBuilder) -> Result<GetDataQualityRuleRecommendationRunOutput, SdkError<GetDataQualityRuleRecommendationRunError>>;
        async fn get_data_quality_ruleset(&self, builder: GetDataQualityRulesetInputBuilder) -> Result<GetDataQualityRulesetOutput, SdkError<GetDataQualityRulesetError>>;
        async fn get_data_quality_ruleset_evaluation_run(&self, builder: GetDataQualityRulesetEvaluationRunInputBuilder) -> Result<GetDataQualityRulesetEvaluationRunOutput, SdkError<GetDataQualityRulesetEvaluationRunError>>;
        async fn get_database(&self, builder: GetDatabaseInputBuilder) -> Result<GetDatabaseOutput, SdkError<GetDatabaseError>>;
        async fn get_databases(&self, builder: GetDatabasesInputBuilder) -> Result<GetDatabasesOutput, SdkError<GetDatabasesError>>;
        async fn get_dataflow_graph(&self, builder: GetDataflowGraphInputBuilder) -> Result<GetDataflowGraphOutput, SdkError<GetDataflowGraphError>>;
        async fn get_dev_endpoint(&self, builder: GetDevEndpointInputBuilder) -> Result<GetDevEndpointOutput, SdkError<GetDevEndpointError>>;
        async fn get_dev_endpoints(&self, builder: GetDevEndpointsInputBuilder) -> Result<GetDevEndpointsOutput, SdkError<GetDevEndpointsError>>;
        async fn get_job(&self, builder: GetJobInputBuilder) -> Result<GetJobOutput, SdkError<GetJobError>>;
        async fn get_job_bookmark(&self, builder: GetJobBookmarkInputBuilder) -> Result<GetJobBookmarkOutput, SdkError<GetJobBookmarkError>>;
        async fn get_job_run(&self, builder: GetJobRunInputBuilder) -> Result<GetJobRunOutput, SdkError<GetJobRunError>>;
        async fn get_job_runs(&self, builder: GetJobRunsInputBuilder) -> Result<GetJobRunsOutput, SdkError<GetJobRunsError>>;
        async fn get_jobs(&self, builder: GetJobsInputBuilder) -> Result<GetJobsOutput, SdkError<GetJobsError>>;
        async fn get_mapping(&self, builder: GetMappingInputBuilder) -> Result<GetMappingOutput, SdkError<GetMappingError>>;
        async fn get_ml_task_run(&self, builder: GetMlTaskRunInputBuilder) -> Result<GetMlTaskRunOutput, SdkError<GetMLTaskRunError>>;
        async fn get_ml_task_runs(&self, builder: GetMlTaskRunsInputBuilder) -> Result<GetMlTaskRunsOutput, SdkError<GetMLTaskRunsError>>;
        async fn get_ml_transform(&self, builder: GetMlTransformInputBuilder) -> Result<GetMlTransformOutput, SdkError<GetMLTransformError>>;
        async fn get_ml_transforms(&self, builder: GetMlTransformsInputBuilder) -> Result<GetMlTransformsOutput, SdkError<GetMLTransformsError>>;
        async fn get_partition(&self, builder: GetPartitionInputBuilder) -> Result<GetPartitionOutput, SdkError<GetPartitionError>>;
        async fn get_partition_indexes(&self, builder: GetPartitionIndexesInputBuilder) -> Result<GetPartitionIndexesOutput, SdkError<GetPartitionIndexesError>>;
        async fn get_partitions(&self, builder: GetPartitionsInputBuilder) -> Result<GetPartitionsOutput, SdkError<GetPartitionsError>>;
        async fn get_plan(&self, builder: GetPlanInputBuilder) -> Result<GetPlanOutput, SdkError<GetPlanError>>;
        async fn get_registry(&self, builder: GetRegistryInputBuilder) -> Result<GetRegistryOutput, SdkError<GetRegistryError>>;
        async fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn get_schema(&self, builder: GetSchemaInputBuilder) -> Result<GetSchemaOutput, SdkError<GetSchemaError>>;
        async fn get_schema_by_definition(&self, builder: GetSchemaByDefinitionInputBuilder) -> Result<GetSchemaByDefinitionOutput, SdkError<GetSchemaByDefinitionError>>;
        async fn get_schema_version(&self, builder: GetSchemaVersionInputBuilder) -> Result<GetSchemaVersionOutput, SdkError<GetSchemaVersionError>>;
        async fn get_schema_versions_diff(&self, builder: GetSchemaVersionsDiffInputBuilder) -> Result<GetSchemaVersionsDiffOutput, SdkError<GetSchemaVersionsDiffError>>;
        async fn get_security_configuration(&self, builder: GetSecurityConfigurationInputBuilder) -> Result<GetSecurityConfigurationOutput, SdkError<GetSecurityConfigurationError>>;
        async fn get_security_configurations(&self, builder: GetSecurityConfigurationsInputBuilder) -> Result<GetSecurityConfigurationsOutput, SdkError<GetSecurityConfigurationsError>>;
        async fn get_session(&self, builder: GetSessionInputBuilder) -> Result<GetSessionOutput, SdkError<GetSessionError>>;
        async fn get_statement(&self, builder: GetStatementInputBuilder) -> Result<GetStatementOutput, SdkError<GetStatementError>>;
        async fn get_table(&self, builder: GetTableInputBuilder) -> Result<GetTableOutput, SdkError<GetTableError>>;
        async fn get_table_optimizer(&self, builder: GetTableOptimizerInputBuilder) -> Result<GetTableOptimizerOutput, SdkError<GetTableOptimizerError>>;
        async fn get_table_version(&self, builder: GetTableVersionInputBuilder) -> Result<GetTableVersionOutput, SdkError<GetTableVersionError>>;
        async fn get_table_versions(&self, builder: GetTableVersionsInputBuilder) -> Result<GetTableVersionsOutput, SdkError<GetTableVersionsError>>;
        async fn get_tables(&self, builder: GetTablesInputBuilder) -> Result<GetTablesOutput, SdkError<GetTablesError>>;
        async fn get_tags(&self, builder: GetTagsInputBuilder) -> Result<GetTagsOutput, SdkError<GetTagsError>>;
        async fn get_trigger(&self, builder: GetTriggerInputBuilder) -> Result<GetTriggerOutput, SdkError<GetTriggerError>>;
        async fn get_triggers(&self, builder: GetTriggersInputBuilder) -> Result<GetTriggersOutput, SdkError<GetTriggersError>>;
        async fn get_unfiltered_partition_metadata(&self, builder: GetUnfilteredPartitionMetadataInputBuilder) -> Result<GetUnfilteredPartitionMetadataOutput, SdkError<GetUnfilteredPartitionMetadataError>>;
        async fn get_unfiltered_partitions_metadata(&self, builder: GetUnfilteredPartitionsMetadataInputBuilder) -> Result<GetUnfilteredPartitionsMetadataOutput, SdkError<GetUnfilteredPartitionsMetadataError>>;
        async fn get_unfiltered_table_metadata(&self, builder: GetUnfilteredTableMetadataInputBuilder) -> Result<GetUnfilteredTableMetadataOutput, SdkError<GetUnfilteredTableMetadataError>>;
        async fn get_usage_profile(&self, builder: GetUsageProfileInputBuilder) -> Result<GetUsageProfileOutput, SdkError<GetUsageProfileError>>;
        async fn get_user_defined_function(&self, builder: GetUserDefinedFunctionInputBuilder) -> Result<GetUserDefinedFunctionOutput, SdkError<GetUserDefinedFunctionError>>;
        async fn get_user_defined_functions(&self, builder: GetUserDefinedFunctionsInputBuilder) -> Result<GetUserDefinedFunctionsOutput, SdkError<GetUserDefinedFunctionsError>>;
        async fn get_workflow(&self, builder: GetWorkflowInputBuilder) -> Result<GetWorkflowOutput, SdkError<GetWorkflowError>>;
        async fn get_workflow_run(&self, builder: GetWorkflowRunInputBuilder) -> Result<GetWorkflowRunOutput, SdkError<GetWorkflowRunError>>;
        async fn get_workflow_run_properties(&self, builder: GetWorkflowRunPropertiesInputBuilder) -> Result<GetWorkflowRunPropertiesOutput, SdkError<GetWorkflowRunPropertiesError>>;
        async fn get_workflow_runs(&self, builder: GetWorkflowRunsInputBuilder) -> Result<GetWorkflowRunsOutput, SdkError<GetWorkflowRunsError>>;
        async fn import_catalog_to_glue(&self, builder: ImportCatalogToGlueInputBuilder) -> Result<ImportCatalogToGlueOutput, SdkError<ImportCatalogToGlueError>>;
        async fn list_blueprints(&self, builder: ListBlueprintsInputBuilder) -> Result<ListBlueprintsOutput, SdkError<ListBlueprintsError>>;
        async fn list_column_statistics_task_runs(&self, builder: ListColumnStatisticsTaskRunsInputBuilder) -> Result<ListColumnStatisticsTaskRunsOutput, SdkError<ListColumnStatisticsTaskRunsError>>;
        async fn list_crawlers(&self, builder: ListCrawlersInputBuilder) -> Result<ListCrawlersOutput, SdkError<ListCrawlersError>>;
        async fn list_crawls(&self, builder: ListCrawlsInputBuilder) -> Result<ListCrawlsOutput, SdkError<ListCrawlsError>>;
        async fn list_custom_entity_types(&self, builder: ListCustomEntityTypesInputBuilder) -> Result<ListCustomEntityTypesOutput, SdkError<ListCustomEntityTypesError>>;
        async fn list_data_quality_results(&self, builder: ListDataQualityResultsInputBuilder) -> Result<ListDataQualityResultsOutput, SdkError<ListDataQualityResultsError>>;
        async fn list_data_quality_rule_recommendation_runs(&self, builder: ListDataQualityRuleRecommendationRunsInputBuilder) -> Result<ListDataQualityRuleRecommendationRunsOutput, SdkError<ListDataQualityRuleRecommendationRunsError>>;
        async fn list_data_quality_ruleset_evaluation_runs(&self, builder: ListDataQualityRulesetEvaluationRunsInputBuilder) -> Result<ListDataQualityRulesetEvaluationRunsOutput, SdkError<ListDataQualityRulesetEvaluationRunsError>>;
        async fn list_data_quality_rulesets(&self, builder: ListDataQualityRulesetsInputBuilder) -> Result<ListDataQualityRulesetsOutput, SdkError<ListDataQualityRulesetsError>>;
        async fn list_dev_endpoints(&self, builder: ListDevEndpointsInputBuilder) -> Result<ListDevEndpointsOutput, SdkError<ListDevEndpointsError>>;
        async fn list_jobs(&self, builder: ListJobsInputBuilder) -> Result<ListJobsOutput, SdkError<ListJobsError>>;
        async fn list_ml_transforms(&self, builder: ListMlTransformsInputBuilder) -> Result<ListMlTransformsOutput, SdkError<ListMLTransformsError>>;
        async fn list_registries(&self, builder: ListRegistriesInputBuilder) -> Result<ListRegistriesOutput, SdkError<ListRegistriesError>>;
        async fn list_schema_versions(&self, builder: ListSchemaVersionsInputBuilder) -> Result<ListSchemaVersionsOutput, SdkError<ListSchemaVersionsError>>;
        async fn list_schemas(&self, builder: ListSchemasInputBuilder) -> Result<ListSchemasOutput, SdkError<ListSchemasError>>;
        async fn list_sessions(&self, builder: ListSessionsInputBuilder) -> Result<ListSessionsOutput, SdkError<ListSessionsError>>;
        async fn list_statements(&self, builder: ListStatementsInputBuilder) -> Result<ListStatementsOutput, SdkError<ListStatementsError>>;
        async fn list_table_optimizer_runs(&self, builder: ListTableOptimizerRunsInputBuilder) -> Result<ListTableOptimizerRunsOutput, SdkError<ListTableOptimizerRunsError>>;
        async fn list_triggers(&self, builder: ListTriggersInputBuilder) -> Result<ListTriggersOutput, SdkError<ListTriggersError>>;
        async fn list_usage_profiles(&self, builder: ListUsageProfilesInputBuilder) -> Result<ListUsageProfilesOutput, SdkError<ListUsageProfilesError>>;
        async fn list_workflows(&self, builder: ListWorkflowsInputBuilder) -> Result<ListWorkflowsOutput, SdkError<ListWorkflowsError>>;
        async fn put_data_catalog_encryption_settings(&self, builder: PutDataCatalogEncryptionSettingsInputBuilder) -> Result<PutDataCatalogEncryptionSettingsOutput, SdkError<PutDataCatalogEncryptionSettingsError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn put_schema_version_metadata(&self, builder: PutSchemaVersionMetadataInputBuilder) -> Result<PutSchemaVersionMetadataOutput, SdkError<PutSchemaVersionMetadataError>>;
        async fn put_workflow_run_properties(&self, builder: PutWorkflowRunPropertiesInputBuilder) -> Result<PutWorkflowRunPropertiesOutput, SdkError<PutWorkflowRunPropertiesError>>;
        async fn query_schema_version_metadata(&self, builder: QuerySchemaVersionMetadataInputBuilder) -> Result<QuerySchemaVersionMetadataOutput, SdkError<QuerySchemaVersionMetadataError>>;
        async fn register_schema_version(&self, builder: RegisterSchemaVersionInputBuilder) -> Result<RegisterSchemaVersionOutput, SdkError<RegisterSchemaVersionError>>;
        async fn remove_schema_version_metadata(&self, builder: RemoveSchemaVersionMetadataInputBuilder) -> Result<RemoveSchemaVersionMetadataOutput, SdkError<RemoveSchemaVersionMetadataError>>;
        async fn reset_job_bookmark(&self, builder: ResetJobBookmarkInputBuilder) -> Result<ResetJobBookmarkOutput, SdkError<ResetJobBookmarkError>>;
        async fn resume_workflow_run(&self, builder: ResumeWorkflowRunInputBuilder) -> Result<ResumeWorkflowRunOutput, SdkError<ResumeWorkflowRunError>>;
        async fn run_statement(&self, builder: RunStatementInputBuilder) -> Result<RunStatementOutput, SdkError<RunStatementError>>;
        async fn search_tables(&self, builder: SearchTablesInputBuilder) -> Result<SearchTablesOutput, SdkError<SearchTablesError>>;
        async fn start_blueprint_run(&self, builder: StartBlueprintRunInputBuilder) -> Result<StartBlueprintRunOutput, SdkError<StartBlueprintRunError>>;
        async fn start_column_statistics_task_run(&self, builder: StartColumnStatisticsTaskRunInputBuilder) -> Result<StartColumnStatisticsTaskRunOutput, SdkError<StartColumnStatisticsTaskRunError>>;
        async fn start_crawler(&self, builder: StartCrawlerInputBuilder) -> Result<StartCrawlerOutput, SdkError<StartCrawlerError>>;
        async fn start_crawler_schedule(&self, builder: StartCrawlerScheduleInputBuilder) -> Result<StartCrawlerScheduleOutput, SdkError<StartCrawlerScheduleError>>;
        async fn start_data_quality_rule_recommendation_run(&self, builder: StartDataQualityRuleRecommendationRunInputBuilder) -> Result<StartDataQualityRuleRecommendationRunOutput, SdkError<StartDataQualityRuleRecommendationRunError>>;
        async fn start_data_quality_ruleset_evaluation_run(&self, builder: StartDataQualityRulesetEvaluationRunInputBuilder) -> Result<StartDataQualityRulesetEvaluationRunOutput, SdkError<StartDataQualityRulesetEvaluationRunError>>;
        async fn start_export_labels_task_run(&self, builder: StartExportLabelsTaskRunInputBuilder) -> Result<StartExportLabelsTaskRunOutput, SdkError<StartExportLabelsTaskRunError>>;
        async fn start_import_labels_task_run(&self, builder: StartImportLabelsTaskRunInputBuilder) -> Result<StartImportLabelsTaskRunOutput, SdkError<StartImportLabelsTaskRunError>>;
        async fn start_job_run(&self, builder: StartJobRunInputBuilder) -> Result<StartJobRunOutput, SdkError<StartJobRunError>>;
        async fn start_ml_evaluation_task_run(&self, builder: StartMlEvaluationTaskRunInputBuilder) -> Result<StartMlEvaluationTaskRunOutput, SdkError<StartMLEvaluationTaskRunError>>;
        async fn start_ml_labeling_set_generation_task_run(&self, builder: StartMlLabelingSetGenerationTaskRunInputBuilder) -> Result<StartMlLabelingSetGenerationTaskRunOutput, SdkError<StartMLLabelingSetGenerationTaskRunError>>;
        async fn start_trigger(&self, builder: StartTriggerInputBuilder) -> Result<StartTriggerOutput, SdkError<StartTriggerError>>;
        async fn start_workflow_run(&self, builder: StartWorkflowRunInputBuilder) -> Result<StartWorkflowRunOutput, SdkError<StartWorkflowRunError>>;
        async fn stop_column_statistics_task_run(&self, builder: StopColumnStatisticsTaskRunInputBuilder) -> Result<StopColumnStatisticsTaskRunOutput, SdkError<StopColumnStatisticsTaskRunError>>;
        async fn stop_crawler(&self, builder: StopCrawlerInputBuilder) -> Result<StopCrawlerOutput, SdkError<StopCrawlerError>>;
        async fn stop_crawler_schedule(&self, builder: StopCrawlerScheduleInputBuilder) -> Result<StopCrawlerScheduleOutput, SdkError<StopCrawlerScheduleError>>;
        async fn stop_session(&self, builder: StopSessionInputBuilder) -> Result<StopSessionOutput, SdkError<StopSessionError>>;
        async fn stop_trigger(&self, builder: StopTriggerInputBuilder) -> Result<StopTriggerOutput, SdkError<StopTriggerError>>;
        async fn stop_workflow_run(&self, builder: StopWorkflowRunInputBuilder) -> Result<StopWorkflowRunOutput, SdkError<StopWorkflowRunError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_blueprint(&self, builder: UpdateBlueprintInputBuilder) -> Result<UpdateBlueprintOutput, SdkError<UpdateBlueprintError>>;
        async fn update_classifier(&self, builder: UpdateClassifierInputBuilder) -> Result<UpdateClassifierOutput, SdkError<UpdateClassifierError>>;
        async fn update_column_statistics_for_partition(&self, builder: UpdateColumnStatisticsForPartitionInputBuilder) -> Result<UpdateColumnStatisticsForPartitionOutput, SdkError<UpdateColumnStatisticsForPartitionError>>;
        async fn update_column_statistics_for_table(&self, builder: UpdateColumnStatisticsForTableInputBuilder) -> Result<UpdateColumnStatisticsForTableOutput, SdkError<UpdateColumnStatisticsForTableError>>;
        async fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>;
        async fn update_crawler(&self, builder: UpdateCrawlerInputBuilder) -> Result<UpdateCrawlerOutput, SdkError<UpdateCrawlerError>>;
        async fn update_crawler_schedule(&self, builder: UpdateCrawlerScheduleInputBuilder) -> Result<UpdateCrawlerScheduleOutput, SdkError<UpdateCrawlerScheduleError>>;
        async fn update_data_quality_ruleset(&self, builder: UpdateDataQualityRulesetInputBuilder) -> Result<UpdateDataQualityRulesetOutput, SdkError<UpdateDataQualityRulesetError>>;
        async fn update_database(&self, builder: UpdateDatabaseInputBuilder) -> Result<UpdateDatabaseOutput, SdkError<UpdateDatabaseError>>;
        async fn update_dev_endpoint(&self, builder: UpdateDevEndpointInputBuilder) -> Result<UpdateDevEndpointOutput, SdkError<UpdateDevEndpointError>>;
        async fn update_job(&self, builder: UpdateJobInputBuilder) -> Result<UpdateJobOutput, SdkError<UpdateJobError>>;
        async fn update_job_from_source_control(&self, builder: UpdateJobFromSourceControlInputBuilder) -> Result<UpdateJobFromSourceControlOutput, SdkError<UpdateJobFromSourceControlError>>;
        async fn update_ml_transform(&self, builder: UpdateMlTransformInputBuilder) -> Result<UpdateMlTransformOutput, SdkError<UpdateMLTransformError>>;
        async fn update_partition(&self, builder: UpdatePartitionInputBuilder) -> Result<UpdatePartitionOutput, SdkError<UpdatePartitionError>>;
        async fn update_registry(&self, builder: UpdateRegistryInputBuilder) -> Result<UpdateRegistryOutput, SdkError<UpdateRegistryError>>;
        async fn update_schema(&self, builder: UpdateSchemaInputBuilder) -> Result<UpdateSchemaOutput, SdkError<UpdateSchemaError>>;
        async fn update_source_control_from_job(&self, builder: UpdateSourceControlFromJobInputBuilder) -> Result<UpdateSourceControlFromJobOutput, SdkError<UpdateSourceControlFromJobError>>;
        async fn update_table(&self, builder: UpdateTableInputBuilder) -> Result<UpdateTableOutput, SdkError<UpdateTableError>>;
        async fn update_table_optimizer(&self, builder: UpdateTableOptimizerInputBuilder) -> Result<UpdateTableOptimizerOutput, SdkError<UpdateTableOptimizerError>>;
        async fn update_trigger(&self, builder: UpdateTriggerInputBuilder) -> Result<UpdateTriggerOutput, SdkError<UpdateTriggerError>>;
        async fn update_usage_profile(&self, builder: UpdateUsageProfileInputBuilder) -> Result<UpdateUsageProfileOutput, SdkError<UpdateUsageProfileError>>;
        async fn update_user_defined_function(&self, builder: UpdateUserDefinedFunctionInputBuilder) -> Result<UpdateUserDefinedFunctionOutput, SdkError<UpdateUserDefinedFunctionError>>;
        async fn update_workflow(&self, builder: UpdateWorkflowInputBuilder) -> Result<UpdateWorkflowOutput, SdkError<UpdateWorkflowError>>;
    }
}
