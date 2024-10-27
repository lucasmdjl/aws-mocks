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
use aws_sdk_athena::operation::batch_get_named_query::{builders::*, *};
use aws_sdk_athena::operation::batch_get_prepared_statement::{builders::*, *};
use aws_sdk_athena::operation::batch_get_query_execution::{builders::*, *};
use aws_sdk_athena::operation::cancel_capacity_reservation::{builders::*, *};
use aws_sdk_athena::operation::create_capacity_reservation::{builders::*, *};
use aws_sdk_athena::operation::create_data_catalog::{builders::*, *};
use aws_sdk_athena::operation::create_named_query::{builders::*, *};
use aws_sdk_athena::operation::create_notebook::{builders::*, *};
use aws_sdk_athena::operation::create_prepared_statement::{builders::*, *};
use aws_sdk_athena::operation::create_presigned_notebook_url::{builders::*, *};
use aws_sdk_athena::operation::create_work_group::{builders::*, *};
use aws_sdk_athena::operation::delete_capacity_reservation::{builders::*, *};
use aws_sdk_athena::operation::delete_data_catalog::{builders::*, *};
use aws_sdk_athena::operation::delete_named_query::{builders::*, *};
use aws_sdk_athena::operation::delete_notebook::{builders::*, *};
use aws_sdk_athena::operation::delete_prepared_statement::{builders::*, *};
use aws_sdk_athena::operation::delete_work_group::{builders::*, *};
use aws_sdk_athena::operation::export_notebook::{builders::*, *};
use aws_sdk_athena::operation::get_calculation_execution::{builders::*, *};
use aws_sdk_athena::operation::get_calculation_execution_code::{builders::*, *};
use aws_sdk_athena::operation::get_calculation_execution_status::{builders::*, *};
use aws_sdk_athena::operation::get_capacity_assignment_configuration::{builders::*, *};
use aws_sdk_athena::operation::get_capacity_reservation::{builders::*, *};
use aws_sdk_athena::operation::get_data_catalog::{builders::*, *};
use aws_sdk_athena::operation::get_database::{builders::*, *};
use aws_sdk_athena::operation::get_named_query::{builders::*, *};
use aws_sdk_athena::operation::get_notebook_metadata::{builders::*, *};
use aws_sdk_athena::operation::get_prepared_statement::{builders::*, *};
use aws_sdk_athena::operation::get_query_execution::{builders::*, *};
use aws_sdk_athena::operation::get_query_results::{builders::*, *};
use aws_sdk_athena::operation::get_query_runtime_statistics::{builders::*, *};
use aws_sdk_athena::operation::get_session::{builders::*, *};
use aws_sdk_athena::operation::get_session_status::{builders::*, *};
use aws_sdk_athena::operation::get_table_metadata::{builders::*, *};
use aws_sdk_athena::operation::get_work_group::{builders::*, *};
use aws_sdk_athena::operation::import_notebook::{builders::*, *};
use aws_sdk_athena::operation::list_application_dpu_sizes::{builders::*, *};
use aws_sdk_athena::operation::list_calculation_executions::{builders::*, *};
use aws_sdk_athena::operation::list_capacity_reservations::{builders::*, *};
use aws_sdk_athena::operation::list_data_catalogs::{builders::*, *};
use aws_sdk_athena::operation::list_databases::{builders::*, *};
use aws_sdk_athena::operation::list_engine_versions::{builders::*, *};
use aws_sdk_athena::operation::list_executors::{builders::*, *};
use aws_sdk_athena::operation::list_named_queries::{builders::*, *};
use aws_sdk_athena::operation::list_notebook_metadata::{builders::*, *};
use aws_sdk_athena::operation::list_notebook_sessions::{builders::*, *};
use aws_sdk_athena::operation::list_prepared_statements::{builders::*, *};
use aws_sdk_athena::operation::list_query_executions::{builders::*, *};
use aws_sdk_athena::operation::list_sessions::{builders::*, *};
use aws_sdk_athena::operation::list_table_metadata::{builders::*, *};
use aws_sdk_athena::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_athena::operation::list_work_groups::{builders::*, *};
use aws_sdk_athena::operation::put_capacity_assignment_configuration::{builders::*, *};
use aws_sdk_athena::operation::start_calculation_execution::{builders::*, *};
use aws_sdk_athena::operation::start_query_execution::{builders::*, *};
use aws_sdk_athena::operation::start_session::{builders::*, *};
use aws_sdk_athena::operation::stop_calculation_execution::{builders::*, *};
use aws_sdk_athena::operation::stop_query_execution::{builders::*, *};
use aws_sdk_athena::operation::tag_resource::{builders::*, *};
use aws_sdk_athena::operation::terminate_session::{builders::*, *};
use aws_sdk_athena::operation::untag_resource::{builders::*, *};
use aws_sdk_athena::operation::update_capacity_reservation::{builders::*, *};
use aws_sdk_athena::operation::update_data_catalog::{builders::*, *};
use aws_sdk_athena::operation::update_named_query::{builders::*, *};
use aws_sdk_athena::operation::update_notebook::{builders::*, *};
use aws_sdk_athena::operation::update_notebook_metadata::{builders::*, *};
use aws_sdk_athena::operation::update_prepared_statement::{builders::*, *};
use aws_sdk_athena::operation::update_work_group::{builders::*, *};
use aws_sdk_athena::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_athena::Client;
use std::ops::Deref;

pub use aws_sdk_athena::*;

pub struct AthenaClientImpl(Client);
impl AthenaClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AthenaClient {
    fn batch_get_named_query(&self, builder: BatchGetNamedQueryInputBuilder) -> impl Future<Output = Result<BatchGetNamedQueryOutput, SdkError<BatchGetNamedQueryError>>> + Send;
    fn batch_get_prepared_statement(&self, builder: BatchGetPreparedStatementInputBuilder) -> impl Future<Output = Result<BatchGetPreparedStatementOutput, SdkError<BatchGetPreparedStatementError>>> + Send;
    fn batch_get_query_execution(&self, builder: BatchGetQueryExecutionInputBuilder) -> impl Future<Output = Result<BatchGetQueryExecutionOutput, SdkError<BatchGetQueryExecutionError>>> + Send;
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>> + Send;
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>> + Send;
    fn create_data_catalog(&self, builder: CreateDataCatalogInputBuilder) -> impl Future<Output = Result<CreateDataCatalogOutput, SdkError<CreateDataCatalogError>>> + Send;
    fn create_named_query(&self, builder: CreateNamedQueryInputBuilder) -> impl Future<Output = Result<CreateNamedQueryOutput, SdkError<CreateNamedQueryError>>> + Send;
    fn create_notebook(&self, builder: CreateNotebookInputBuilder) -> impl Future<Output = Result<CreateNotebookOutput, SdkError<CreateNotebookError>>> + Send;
    fn create_prepared_statement(&self, builder: CreatePreparedStatementInputBuilder) -> impl Future<Output = Result<CreatePreparedStatementOutput, SdkError<CreatePreparedStatementError>>> + Send;
    fn create_presigned_notebook_url(&self, builder: CreatePresignedNotebookUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookUrlOutput, SdkError<CreatePresignedNotebookUrlError>>> + Send;
    fn create_work_group(&self, builder: CreateWorkGroupInputBuilder) -> impl Future<Output = Result<CreateWorkGroupOutput, SdkError<CreateWorkGroupError>>> + Send;
    fn delete_capacity_reservation(&self, builder: DeleteCapacityReservationInputBuilder) -> impl Future<Output = Result<DeleteCapacityReservationOutput, SdkError<DeleteCapacityReservationError>>> + Send;
    fn delete_data_catalog(&self, builder: DeleteDataCatalogInputBuilder) -> impl Future<Output = Result<DeleteDataCatalogOutput, SdkError<DeleteDataCatalogError>>> + Send;
    fn delete_named_query(&self, builder: DeleteNamedQueryInputBuilder) -> impl Future<Output = Result<DeleteNamedQueryOutput, SdkError<DeleteNamedQueryError>>> + Send;
    fn delete_notebook(&self, builder: DeleteNotebookInputBuilder) -> impl Future<Output = Result<DeleteNotebookOutput, SdkError<DeleteNotebookError>>> + Send;
    fn delete_prepared_statement(&self, builder: DeletePreparedStatementInputBuilder) -> impl Future<Output = Result<DeletePreparedStatementOutput, SdkError<DeletePreparedStatementError>>> + Send;
    fn delete_work_group(&self, builder: DeleteWorkGroupInputBuilder) -> impl Future<Output = Result<DeleteWorkGroupOutput, SdkError<DeleteWorkGroupError>>> + Send;
    fn export_notebook(&self, builder: ExportNotebookInputBuilder) -> impl Future<Output = Result<ExportNotebookOutput, SdkError<ExportNotebookError>>> + Send;
    fn get_calculation_execution(&self, builder: GetCalculationExecutionInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionOutput, SdkError<GetCalculationExecutionError>>> + Send;
    fn get_calculation_execution_code(&self, builder: GetCalculationExecutionCodeInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionCodeOutput, SdkError<GetCalculationExecutionCodeError>>> + Send;
    fn get_calculation_execution_status(&self, builder: GetCalculationExecutionStatusInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionStatusOutput, SdkError<GetCalculationExecutionStatusError>>> + Send;
    fn get_capacity_assignment_configuration(&self, builder: GetCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<GetCapacityAssignmentConfigurationOutput, SdkError<GetCapacityAssignmentConfigurationError>>> + Send;
    fn get_capacity_reservation(&self, builder: GetCapacityReservationInputBuilder) -> impl Future<Output = Result<GetCapacityReservationOutput, SdkError<GetCapacityReservationError>>> + Send;
    fn get_data_catalog(&self, builder: GetDataCatalogInputBuilder) -> impl Future<Output = Result<GetDataCatalogOutput, SdkError<GetDataCatalogError>>> + Send;
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>> + Send;
    fn get_named_query(&self, builder: GetNamedQueryInputBuilder) -> impl Future<Output = Result<GetNamedQueryOutput, SdkError<GetNamedQueryError>>> + Send;
    fn get_notebook_metadata(&self, builder: GetNotebookMetadataInputBuilder) -> impl Future<Output = Result<GetNotebookMetadataOutput, SdkError<GetNotebookMetadataError>>> + Send;
    fn get_prepared_statement(&self, builder: GetPreparedStatementInputBuilder) -> impl Future<Output = Result<GetPreparedStatementOutput, SdkError<GetPreparedStatementError>>> + Send;
    fn get_query_execution(&self, builder: GetQueryExecutionInputBuilder) -> impl Future<Output = Result<GetQueryExecutionOutput, SdkError<GetQueryExecutionError>>> + Send;
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> + Send;
    fn get_query_runtime_statistics(&self, builder: GetQueryRuntimeStatisticsInputBuilder) -> impl Future<Output = Result<GetQueryRuntimeStatisticsOutput, SdkError<GetQueryRuntimeStatisticsError>>> + Send;
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>> + Send;
    fn get_session_status(&self, builder: GetSessionStatusInputBuilder) -> impl Future<Output = Result<GetSessionStatusOutput, SdkError<GetSessionStatusError>>> + Send;
    fn get_table_metadata(&self, builder: GetTableMetadataInputBuilder) -> impl Future<Output = Result<GetTableMetadataOutput, SdkError<GetTableMetadataError>>> + Send;
    fn get_work_group(&self, builder: GetWorkGroupInputBuilder) -> impl Future<Output = Result<GetWorkGroupOutput, SdkError<GetWorkGroupError>>> + Send;
    fn import_notebook(&self, builder: ImportNotebookInputBuilder) -> impl Future<Output = Result<ImportNotebookOutput, SdkError<ImportNotebookError>>> + Send;
    fn list_application_dpu_sizes(&self, builder: ListApplicationDpuSizesInputBuilder) -> impl Future<Output = Result<ListApplicationDpuSizesOutput, SdkError<ListApplicationDPUSizesError>>> + Send;
    fn list_calculation_executions(&self, builder: ListCalculationExecutionsInputBuilder) -> impl Future<Output = Result<ListCalculationExecutionsOutput, SdkError<ListCalculationExecutionsError>>> + Send;
    fn list_capacity_reservations(&self, builder: ListCapacityReservationsInputBuilder) -> impl Future<Output = Result<ListCapacityReservationsOutput, SdkError<ListCapacityReservationsError>>> + Send;
    fn list_data_catalogs(&self, builder: ListDataCatalogsInputBuilder) -> impl Future<Output = Result<ListDataCatalogsOutput, SdkError<ListDataCatalogsError>>> + Send;
    fn list_databases(&self, builder: ListDatabasesInputBuilder) -> impl Future<Output = Result<ListDatabasesOutput, SdkError<ListDatabasesError>>> + Send;
    fn list_engine_versions(&self, builder: ListEngineVersionsInputBuilder) -> impl Future<Output = Result<ListEngineVersionsOutput, SdkError<ListEngineVersionsError>>> + Send;
    fn list_executors(&self, builder: ListExecutorsInputBuilder) -> impl Future<Output = Result<ListExecutorsOutput, SdkError<ListExecutorsError>>> + Send;
    fn list_named_queries(&self, builder: ListNamedQueriesInputBuilder) -> impl Future<Output = Result<ListNamedQueriesOutput, SdkError<ListNamedQueriesError>>> + Send;
    fn list_notebook_metadata(&self, builder: ListNotebookMetadataInputBuilder) -> impl Future<Output = Result<ListNotebookMetadataOutput, SdkError<ListNotebookMetadataError>>> + Send;
    fn list_notebook_sessions(&self, builder: ListNotebookSessionsInputBuilder) -> impl Future<Output = Result<ListNotebookSessionsOutput, SdkError<ListNotebookSessionsError>>> + Send;
    fn list_prepared_statements(&self, builder: ListPreparedStatementsInputBuilder) -> impl Future<Output = Result<ListPreparedStatementsOutput, SdkError<ListPreparedStatementsError>>> + Send;
    fn list_query_executions(&self, builder: ListQueryExecutionsInputBuilder) -> impl Future<Output = Result<ListQueryExecutionsOutput, SdkError<ListQueryExecutionsError>>> + Send;
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>> + Send;
    fn list_table_metadata(&self, builder: ListTableMetadataInputBuilder) -> impl Future<Output = Result<ListTableMetadataOutput, SdkError<ListTableMetadataError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_work_groups(&self, builder: ListWorkGroupsInputBuilder) -> impl Future<Output = Result<ListWorkGroupsOutput, SdkError<ListWorkGroupsError>>> + Send;
    fn put_capacity_assignment_configuration(&self, builder: PutCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<PutCapacityAssignmentConfigurationOutput, SdkError<PutCapacityAssignmentConfigurationError>>> + Send;
    fn start_calculation_execution(&self, builder: StartCalculationExecutionInputBuilder) -> impl Future<Output = Result<StartCalculationExecutionOutput, SdkError<StartCalculationExecutionError>>> + Send;
    fn start_query_execution(&self, builder: StartQueryExecutionInputBuilder) -> impl Future<Output = Result<StartQueryExecutionOutput, SdkError<StartQueryExecutionError>>> + Send;
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>> + Send;
    fn stop_calculation_execution(&self, builder: StopCalculationExecutionInputBuilder) -> impl Future<Output = Result<StopCalculationExecutionOutput, SdkError<StopCalculationExecutionError>>> + Send;
    fn stop_query_execution(&self, builder: StopQueryExecutionInputBuilder) -> impl Future<Output = Result<StopQueryExecutionOutput, SdkError<StopQueryExecutionError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_capacity_reservation(&self, builder: UpdateCapacityReservationInputBuilder) -> impl Future<Output = Result<UpdateCapacityReservationOutput, SdkError<UpdateCapacityReservationError>>> + Send;
    fn update_data_catalog(&self, builder: UpdateDataCatalogInputBuilder) -> impl Future<Output = Result<UpdateDataCatalogOutput, SdkError<UpdateDataCatalogError>>> + Send;
    fn update_named_query(&self, builder: UpdateNamedQueryInputBuilder) -> impl Future<Output = Result<UpdateNamedQueryOutput, SdkError<UpdateNamedQueryError>>> + Send;
    fn update_notebook(&self, builder: UpdateNotebookInputBuilder) -> impl Future<Output = Result<UpdateNotebookOutput, SdkError<UpdateNotebookError>>> + Send;
    fn update_notebook_metadata(&self, builder: UpdateNotebookMetadataInputBuilder) -> impl Future<Output = Result<UpdateNotebookMetadataOutput, SdkError<UpdateNotebookMetadataError>>> + Send;
    fn update_prepared_statement(&self, builder: UpdatePreparedStatementInputBuilder) -> impl Future<Output = Result<UpdatePreparedStatementOutput, SdkError<UpdatePreparedStatementError>>> + Send;
    fn update_work_group(&self, builder: UpdateWorkGroupInputBuilder) -> impl Future<Output = Result<UpdateWorkGroupOutput, SdkError<UpdateWorkGroupError>>> + Send;
}
impl AthenaClient for AthenaClientImpl {
    fn batch_get_named_query(&self, builder: BatchGetNamedQueryInputBuilder) -> impl Future<Output = Result<BatchGetNamedQueryOutput, SdkError<BatchGetNamedQueryError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_prepared_statement(&self, builder: BatchGetPreparedStatementInputBuilder) -> impl Future<Output = Result<BatchGetPreparedStatementOutput, SdkError<BatchGetPreparedStatementError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_query_execution(&self, builder: BatchGetQueryExecutionInputBuilder) -> impl Future<Output = Result<BatchGetQueryExecutionOutput, SdkError<BatchGetQueryExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_catalog(&self, builder: CreateDataCatalogInputBuilder) -> impl Future<Output = Result<CreateDataCatalogOutput, SdkError<CreateDataCatalogError>>> {
        builder.send_with(&self.0)
    }
    fn create_named_query(&self, builder: CreateNamedQueryInputBuilder) -> impl Future<Output = Result<CreateNamedQueryOutput, SdkError<CreateNamedQueryError>>> {
        builder.send_with(&self.0)
    }
    fn create_notebook(&self, builder: CreateNotebookInputBuilder) -> impl Future<Output = Result<CreateNotebookOutput, SdkError<CreateNotebookError>>> {
        builder.send_with(&self.0)
    }
    fn create_prepared_statement(&self, builder: CreatePreparedStatementInputBuilder) -> impl Future<Output = Result<CreatePreparedStatementOutput, SdkError<CreatePreparedStatementError>>> {
        builder.send_with(&self.0)
    }
    fn create_presigned_notebook_url(&self, builder: CreatePresignedNotebookUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookUrlOutput, SdkError<CreatePresignedNotebookUrlError>>> {
        builder.send_with(&self.0)
    }
    fn create_work_group(&self, builder: CreateWorkGroupInputBuilder) -> impl Future<Output = Result<CreateWorkGroupOutput, SdkError<CreateWorkGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_capacity_reservation(&self, builder: DeleteCapacityReservationInputBuilder) -> impl Future<Output = Result<DeleteCapacityReservationOutput, SdkError<DeleteCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_catalog(&self, builder: DeleteDataCatalogInputBuilder) -> impl Future<Output = Result<DeleteDataCatalogOutput, SdkError<DeleteDataCatalogError>>> {
        builder.send_with(&self.0)
    }
    fn delete_named_query(&self, builder: DeleteNamedQueryInputBuilder) -> impl Future<Output = Result<DeleteNamedQueryOutput, SdkError<DeleteNamedQueryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_notebook(&self, builder: DeleteNotebookInputBuilder) -> impl Future<Output = Result<DeleteNotebookOutput, SdkError<DeleteNotebookError>>> {
        builder.send_with(&self.0)
    }
    fn delete_prepared_statement(&self, builder: DeletePreparedStatementInputBuilder) -> impl Future<Output = Result<DeletePreparedStatementOutput, SdkError<DeletePreparedStatementError>>> {
        builder.send_with(&self.0)
    }
    fn delete_work_group(&self, builder: DeleteWorkGroupInputBuilder) -> impl Future<Output = Result<DeleteWorkGroupOutput, SdkError<DeleteWorkGroupError>>> {
        builder.send_with(&self.0)
    }
    fn export_notebook(&self, builder: ExportNotebookInputBuilder) -> impl Future<Output = Result<ExportNotebookOutput, SdkError<ExportNotebookError>>> {
        builder.send_with(&self.0)
    }
    fn get_calculation_execution(&self, builder: GetCalculationExecutionInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionOutput, SdkError<GetCalculationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn get_calculation_execution_code(&self, builder: GetCalculationExecutionCodeInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionCodeOutput, SdkError<GetCalculationExecutionCodeError>>> {
        builder.send_with(&self.0)
    }
    fn get_calculation_execution_status(&self, builder: GetCalculationExecutionStatusInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionStatusOutput, SdkError<GetCalculationExecutionStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_capacity_assignment_configuration(&self, builder: GetCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<GetCapacityAssignmentConfigurationOutput, SdkError<GetCapacityAssignmentConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_capacity_reservation(&self, builder: GetCapacityReservationInputBuilder) -> impl Future<Output = Result<GetCapacityReservationOutput, SdkError<GetCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_catalog(&self, builder: GetDataCatalogInputBuilder) -> impl Future<Output = Result<GetDataCatalogOutput, SdkError<GetDataCatalogError>>> {
        builder.send_with(&self.0)
    }
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn get_named_query(&self, builder: GetNamedQueryInputBuilder) -> impl Future<Output = Result<GetNamedQueryOutput, SdkError<GetNamedQueryError>>> {
        builder.send_with(&self.0)
    }
    fn get_notebook_metadata(&self, builder: GetNotebookMetadataInputBuilder) -> impl Future<Output = Result<GetNotebookMetadataOutput, SdkError<GetNotebookMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_prepared_statement(&self, builder: GetPreparedStatementInputBuilder) -> impl Future<Output = Result<GetPreparedStatementOutput, SdkError<GetPreparedStatementError>>> {
        builder.send_with(&self.0)
    }
    fn get_query_execution(&self, builder: GetQueryExecutionInputBuilder) -> impl Future<Output = Result<GetQueryExecutionOutput, SdkError<GetQueryExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> {
        builder.send_with(&self.0)
    }
    fn get_query_runtime_statistics(&self, builder: GetQueryRuntimeStatisticsInputBuilder) -> impl Future<Output = Result<GetQueryRuntimeStatisticsOutput, SdkError<GetQueryRuntimeStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>> {
        builder.send_with(&self.0)
    }
    fn get_session_status(&self, builder: GetSessionStatusInputBuilder) -> impl Future<Output = Result<GetSessionStatusOutput, SdkError<GetSessionStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_table_metadata(&self, builder: GetTableMetadataInputBuilder) -> impl Future<Output = Result<GetTableMetadataOutput, SdkError<GetTableMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_work_group(&self, builder: GetWorkGroupInputBuilder) -> impl Future<Output = Result<GetWorkGroupOutput, SdkError<GetWorkGroupError>>> {
        builder.send_with(&self.0)
    }
    fn import_notebook(&self, builder: ImportNotebookInputBuilder) -> impl Future<Output = Result<ImportNotebookOutput, SdkError<ImportNotebookError>>> {
        builder.send_with(&self.0)
    }
    fn list_application_dpu_sizes(&self, builder: ListApplicationDpuSizesInputBuilder) -> impl Future<Output = Result<ListApplicationDpuSizesOutput, SdkError<ListApplicationDPUSizesError>>> {
        builder.send_with(&self.0)
    }
    fn list_calculation_executions(&self, builder: ListCalculationExecutionsInputBuilder) -> impl Future<Output = Result<ListCalculationExecutionsOutput, SdkError<ListCalculationExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_capacity_reservations(&self, builder: ListCapacityReservationsInputBuilder) -> impl Future<Output = Result<ListCapacityReservationsOutput, SdkError<ListCapacityReservationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_catalogs(&self, builder: ListDataCatalogsInputBuilder) -> impl Future<Output = Result<ListDataCatalogsOutput, SdkError<ListDataCatalogsError>>> {
        builder.send_with(&self.0)
    }
    fn list_databases(&self, builder: ListDatabasesInputBuilder) -> impl Future<Output = Result<ListDatabasesOutput, SdkError<ListDatabasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_engine_versions(&self, builder: ListEngineVersionsInputBuilder) -> impl Future<Output = Result<ListEngineVersionsOutput, SdkError<ListEngineVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_executors(&self, builder: ListExecutorsInputBuilder) -> impl Future<Output = Result<ListExecutorsOutput, SdkError<ListExecutorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_named_queries(&self, builder: ListNamedQueriesInputBuilder) -> impl Future<Output = Result<ListNamedQueriesOutput, SdkError<ListNamedQueriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_notebook_metadata(&self, builder: ListNotebookMetadataInputBuilder) -> impl Future<Output = Result<ListNotebookMetadataOutput, SdkError<ListNotebookMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn list_notebook_sessions(&self, builder: ListNotebookSessionsInputBuilder) -> impl Future<Output = Result<ListNotebookSessionsOutput, SdkError<ListNotebookSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_prepared_statements(&self, builder: ListPreparedStatementsInputBuilder) -> impl Future<Output = Result<ListPreparedStatementsOutput, SdkError<ListPreparedStatementsError>>> {
        builder.send_with(&self.0)
    }
    fn list_query_executions(&self, builder: ListQueryExecutionsInputBuilder) -> impl Future<Output = Result<ListQueryExecutionsOutput, SdkError<ListQueryExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_table_metadata(&self, builder: ListTableMetadataInputBuilder) -> impl Future<Output = Result<ListTableMetadataOutput, SdkError<ListTableMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_work_groups(&self, builder: ListWorkGroupsInputBuilder) -> impl Future<Output = Result<ListWorkGroupsOutput, SdkError<ListWorkGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn put_capacity_assignment_configuration(&self, builder: PutCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<PutCapacityAssignmentConfigurationOutput, SdkError<PutCapacityAssignmentConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn start_calculation_execution(&self, builder: StartCalculationExecutionInputBuilder) -> impl Future<Output = Result<StartCalculationExecutionOutput, SdkError<StartCalculationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn start_query_execution(&self, builder: StartQueryExecutionInputBuilder) -> impl Future<Output = Result<StartQueryExecutionOutput, SdkError<StartQueryExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_calculation_execution(&self, builder: StopCalculationExecutionInputBuilder) -> impl Future<Output = Result<StopCalculationExecutionOutput, SdkError<StopCalculationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_query_execution(&self, builder: StopQueryExecutionInputBuilder) -> impl Future<Output = Result<StopQueryExecutionOutput, SdkError<StopQueryExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_capacity_reservation(&self, builder: UpdateCapacityReservationInputBuilder) -> impl Future<Output = Result<UpdateCapacityReservationOutput, SdkError<UpdateCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_catalog(&self, builder: UpdateDataCatalogInputBuilder) -> impl Future<Output = Result<UpdateDataCatalogOutput, SdkError<UpdateDataCatalogError>>> {
        builder.send_with(&self.0)
    }
    fn update_named_query(&self, builder: UpdateNamedQueryInputBuilder) -> impl Future<Output = Result<UpdateNamedQueryOutput, SdkError<UpdateNamedQueryError>>> {
        builder.send_with(&self.0)
    }
    fn update_notebook(&self, builder: UpdateNotebookInputBuilder) -> impl Future<Output = Result<UpdateNotebookOutput, SdkError<UpdateNotebookError>>> {
        builder.send_with(&self.0)
    }
    fn update_notebook_metadata(&self, builder: UpdateNotebookMetadataInputBuilder) -> impl Future<Output = Result<UpdateNotebookMetadataOutput, SdkError<UpdateNotebookMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn update_prepared_statement(&self, builder: UpdatePreparedStatementInputBuilder) -> impl Future<Output = Result<UpdatePreparedStatementOutput, SdkError<UpdatePreparedStatementError>>> {
        builder.send_with(&self.0)
    }
    fn update_work_group(&self, builder: UpdateWorkGroupInputBuilder) -> impl Future<Output = Result<UpdateWorkGroupOutput, SdkError<UpdateWorkGroupError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AthenaClient for T
where T: Deref,
      T::Target: AthenaClient {
    fn batch_get_named_query(&self, builder: BatchGetNamedQueryInputBuilder) -> impl Future<Output = Result<BatchGetNamedQueryOutput, SdkError<BatchGetNamedQueryError>>> {
        self.deref().batch_get_named_query(builder)
    }
    fn batch_get_prepared_statement(&self, builder: BatchGetPreparedStatementInputBuilder) -> impl Future<Output = Result<BatchGetPreparedStatementOutput, SdkError<BatchGetPreparedStatementError>>> {
        self.deref().batch_get_prepared_statement(builder)
    }
    fn batch_get_query_execution(&self, builder: BatchGetQueryExecutionInputBuilder) -> impl Future<Output = Result<BatchGetQueryExecutionOutput, SdkError<BatchGetQueryExecutionError>>> {
        self.deref().batch_get_query_execution(builder)
    }
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>> {
        self.deref().cancel_capacity_reservation(builder)
    }
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>> {
        self.deref().create_capacity_reservation(builder)
    }
    fn create_data_catalog(&self, builder: CreateDataCatalogInputBuilder) -> impl Future<Output = Result<CreateDataCatalogOutput, SdkError<CreateDataCatalogError>>> {
        self.deref().create_data_catalog(builder)
    }
    fn create_named_query(&self, builder: CreateNamedQueryInputBuilder) -> impl Future<Output = Result<CreateNamedQueryOutput, SdkError<CreateNamedQueryError>>> {
        self.deref().create_named_query(builder)
    }
    fn create_notebook(&self, builder: CreateNotebookInputBuilder) -> impl Future<Output = Result<CreateNotebookOutput, SdkError<CreateNotebookError>>> {
        self.deref().create_notebook(builder)
    }
    fn create_prepared_statement(&self, builder: CreatePreparedStatementInputBuilder) -> impl Future<Output = Result<CreatePreparedStatementOutput, SdkError<CreatePreparedStatementError>>> {
        self.deref().create_prepared_statement(builder)
    }
    fn create_presigned_notebook_url(&self, builder: CreatePresignedNotebookUrlInputBuilder) -> impl Future<Output = Result<CreatePresignedNotebookUrlOutput, SdkError<CreatePresignedNotebookUrlError>>> {
        self.deref().create_presigned_notebook_url(builder)
    }
    fn create_work_group(&self, builder: CreateWorkGroupInputBuilder) -> impl Future<Output = Result<CreateWorkGroupOutput, SdkError<CreateWorkGroupError>>> {
        self.deref().create_work_group(builder)
    }
    fn delete_capacity_reservation(&self, builder: DeleteCapacityReservationInputBuilder) -> impl Future<Output = Result<DeleteCapacityReservationOutput, SdkError<DeleteCapacityReservationError>>> {
        self.deref().delete_capacity_reservation(builder)
    }
    fn delete_data_catalog(&self, builder: DeleteDataCatalogInputBuilder) -> impl Future<Output = Result<DeleteDataCatalogOutput, SdkError<DeleteDataCatalogError>>> {
        self.deref().delete_data_catalog(builder)
    }
    fn delete_named_query(&self, builder: DeleteNamedQueryInputBuilder) -> impl Future<Output = Result<DeleteNamedQueryOutput, SdkError<DeleteNamedQueryError>>> {
        self.deref().delete_named_query(builder)
    }
    fn delete_notebook(&self, builder: DeleteNotebookInputBuilder) -> impl Future<Output = Result<DeleteNotebookOutput, SdkError<DeleteNotebookError>>> {
        self.deref().delete_notebook(builder)
    }
    fn delete_prepared_statement(&self, builder: DeletePreparedStatementInputBuilder) -> impl Future<Output = Result<DeletePreparedStatementOutput, SdkError<DeletePreparedStatementError>>> {
        self.deref().delete_prepared_statement(builder)
    }
    fn delete_work_group(&self, builder: DeleteWorkGroupInputBuilder) -> impl Future<Output = Result<DeleteWorkGroupOutput, SdkError<DeleteWorkGroupError>>> {
        self.deref().delete_work_group(builder)
    }
    fn export_notebook(&self, builder: ExportNotebookInputBuilder) -> impl Future<Output = Result<ExportNotebookOutput, SdkError<ExportNotebookError>>> {
        self.deref().export_notebook(builder)
    }
    fn get_calculation_execution(&self, builder: GetCalculationExecutionInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionOutput, SdkError<GetCalculationExecutionError>>> {
        self.deref().get_calculation_execution(builder)
    }
    fn get_calculation_execution_code(&self, builder: GetCalculationExecutionCodeInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionCodeOutput, SdkError<GetCalculationExecutionCodeError>>> {
        self.deref().get_calculation_execution_code(builder)
    }
    fn get_calculation_execution_status(&self, builder: GetCalculationExecutionStatusInputBuilder) -> impl Future<Output = Result<GetCalculationExecutionStatusOutput, SdkError<GetCalculationExecutionStatusError>>> {
        self.deref().get_calculation_execution_status(builder)
    }
    fn get_capacity_assignment_configuration(&self, builder: GetCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<GetCapacityAssignmentConfigurationOutput, SdkError<GetCapacityAssignmentConfigurationError>>> {
        self.deref().get_capacity_assignment_configuration(builder)
    }
    fn get_capacity_reservation(&self, builder: GetCapacityReservationInputBuilder) -> impl Future<Output = Result<GetCapacityReservationOutput, SdkError<GetCapacityReservationError>>> {
        self.deref().get_capacity_reservation(builder)
    }
    fn get_data_catalog(&self, builder: GetDataCatalogInputBuilder) -> impl Future<Output = Result<GetDataCatalogOutput, SdkError<GetDataCatalogError>>> {
        self.deref().get_data_catalog(builder)
    }
    fn get_database(&self, builder: GetDatabaseInputBuilder) -> impl Future<Output = Result<GetDatabaseOutput, SdkError<GetDatabaseError>>> {
        self.deref().get_database(builder)
    }
    fn get_named_query(&self, builder: GetNamedQueryInputBuilder) -> impl Future<Output = Result<GetNamedQueryOutput, SdkError<GetNamedQueryError>>> {
        self.deref().get_named_query(builder)
    }
    fn get_notebook_metadata(&self, builder: GetNotebookMetadataInputBuilder) -> impl Future<Output = Result<GetNotebookMetadataOutput, SdkError<GetNotebookMetadataError>>> {
        self.deref().get_notebook_metadata(builder)
    }
    fn get_prepared_statement(&self, builder: GetPreparedStatementInputBuilder) -> impl Future<Output = Result<GetPreparedStatementOutput, SdkError<GetPreparedStatementError>>> {
        self.deref().get_prepared_statement(builder)
    }
    fn get_query_execution(&self, builder: GetQueryExecutionInputBuilder) -> impl Future<Output = Result<GetQueryExecutionOutput, SdkError<GetQueryExecutionError>>> {
        self.deref().get_query_execution(builder)
    }
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> {
        self.deref().get_query_results(builder)
    }
    fn get_query_runtime_statistics(&self, builder: GetQueryRuntimeStatisticsInputBuilder) -> impl Future<Output = Result<GetQueryRuntimeStatisticsOutput, SdkError<GetQueryRuntimeStatisticsError>>> {
        self.deref().get_query_runtime_statistics(builder)
    }
    fn get_session(&self, builder: GetSessionInputBuilder) -> impl Future<Output = Result<GetSessionOutput, SdkError<GetSessionError>>> {
        self.deref().get_session(builder)
    }
    fn get_session_status(&self, builder: GetSessionStatusInputBuilder) -> impl Future<Output = Result<GetSessionStatusOutput, SdkError<GetSessionStatusError>>> {
        self.deref().get_session_status(builder)
    }
    fn get_table_metadata(&self, builder: GetTableMetadataInputBuilder) -> impl Future<Output = Result<GetTableMetadataOutput, SdkError<GetTableMetadataError>>> {
        self.deref().get_table_metadata(builder)
    }
    fn get_work_group(&self, builder: GetWorkGroupInputBuilder) -> impl Future<Output = Result<GetWorkGroupOutput, SdkError<GetWorkGroupError>>> {
        self.deref().get_work_group(builder)
    }
    fn import_notebook(&self, builder: ImportNotebookInputBuilder) -> impl Future<Output = Result<ImportNotebookOutput, SdkError<ImportNotebookError>>> {
        self.deref().import_notebook(builder)
    }
    fn list_application_dpu_sizes(&self, builder: ListApplicationDpuSizesInputBuilder) -> impl Future<Output = Result<ListApplicationDpuSizesOutput, SdkError<ListApplicationDPUSizesError>>> {
        self.deref().list_application_dpu_sizes(builder)
    }
    fn list_calculation_executions(&self, builder: ListCalculationExecutionsInputBuilder) -> impl Future<Output = Result<ListCalculationExecutionsOutput, SdkError<ListCalculationExecutionsError>>> {
        self.deref().list_calculation_executions(builder)
    }
    fn list_capacity_reservations(&self, builder: ListCapacityReservationsInputBuilder) -> impl Future<Output = Result<ListCapacityReservationsOutput, SdkError<ListCapacityReservationsError>>> {
        self.deref().list_capacity_reservations(builder)
    }
    fn list_data_catalogs(&self, builder: ListDataCatalogsInputBuilder) -> impl Future<Output = Result<ListDataCatalogsOutput, SdkError<ListDataCatalogsError>>> {
        self.deref().list_data_catalogs(builder)
    }
    fn list_databases(&self, builder: ListDatabasesInputBuilder) -> impl Future<Output = Result<ListDatabasesOutput, SdkError<ListDatabasesError>>> {
        self.deref().list_databases(builder)
    }
    fn list_engine_versions(&self, builder: ListEngineVersionsInputBuilder) -> impl Future<Output = Result<ListEngineVersionsOutput, SdkError<ListEngineVersionsError>>> {
        self.deref().list_engine_versions(builder)
    }
    fn list_executors(&self, builder: ListExecutorsInputBuilder) -> impl Future<Output = Result<ListExecutorsOutput, SdkError<ListExecutorsError>>> {
        self.deref().list_executors(builder)
    }
    fn list_named_queries(&self, builder: ListNamedQueriesInputBuilder) -> impl Future<Output = Result<ListNamedQueriesOutput, SdkError<ListNamedQueriesError>>> {
        self.deref().list_named_queries(builder)
    }
    fn list_notebook_metadata(&self, builder: ListNotebookMetadataInputBuilder) -> impl Future<Output = Result<ListNotebookMetadataOutput, SdkError<ListNotebookMetadataError>>> {
        self.deref().list_notebook_metadata(builder)
    }
    fn list_notebook_sessions(&self, builder: ListNotebookSessionsInputBuilder) -> impl Future<Output = Result<ListNotebookSessionsOutput, SdkError<ListNotebookSessionsError>>> {
        self.deref().list_notebook_sessions(builder)
    }
    fn list_prepared_statements(&self, builder: ListPreparedStatementsInputBuilder) -> impl Future<Output = Result<ListPreparedStatementsOutput, SdkError<ListPreparedStatementsError>>> {
        self.deref().list_prepared_statements(builder)
    }
    fn list_query_executions(&self, builder: ListQueryExecutionsInputBuilder) -> impl Future<Output = Result<ListQueryExecutionsOutput, SdkError<ListQueryExecutionsError>>> {
        self.deref().list_query_executions(builder)
    }
    fn list_sessions(&self, builder: ListSessionsInputBuilder) -> impl Future<Output = Result<ListSessionsOutput, SdkError<ListSessionsError>>> {
        self.deref().list_sessions(builder)
    }
    fn list_table_metadata(&self, builder: ListTableMetadataInputBuilder) -> impl Future<Output = Result<ListTableMetadataOutput, SdkError<ListTableMetadataError>>> {
        self.deref().list_table_metadata(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_work_groups(&self, builder: ListWorkGroupsInputBuilder) -> impl Future<Output = Result<ListWorkGroupsOutput, SdkError<ListWorkGroupsError>>> {
        self.deref().list_work_groups(builder)
    }
    fn put_capacity_assignment_configuration(&self, builder: PutCapacityAssignmentConfigurationInputBuilder) -> impl Future<Output = Result<PutCapacityAssignmentConfigurationOutput, SdkError<PutCapacityAssignmentConfigurationError>>> {
        self.deref().put_capacity_assignment_configuration(builder)
    }
    fn start_calculation_execution(&self, builder: StartCalculationExecutionInputBuilder) -> impl Future<Output = Result<StartCalculationExecutionOutput, SdkError<StartCalculationExecutionError>>> {
        self.deref().start_calculation_execution(builder)
    }
    fn start_query_execution(&self, builder: StartQueryExecutionInputBuilder) -> impl Future<Output = Result<StartQueryExecutionOutput, SdkError<StartQueryExecutionError>>> {
        self.deref().start_query_execution(builder)
    }
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>> {
        self.deref().start_session(builder)
    }
    fn stop_calculation_execution(&self, builder: StopCalculationExecutionInputBuilder) -> impl Future<Output = Result<StopCalculationExecutionOutput, SdkError<StopCalculationExecutionError>>> {
        self.deref().stop_calculation_execution(builder)
    }
    fn stop_query_execution(&self, builder: StopQueryExecutionInputBuilder) -> impl Future<Output = Result<StopQueryExecutionOutput, SdkError<StopQueryExecutionError>>> {
        self.deref().stop_query_execution(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>> {
        self.deref().terminate_session(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_capacity_reservation(&self, builder: UpdateCapacityReservationInputBuilder) -> impl Future<Output = Result<UpdateCapacityReservationOutput, SdkError<UpdateCapacityReservationError>>> {
        self.deref().update_capacity_reservation(builder)
    }
    fn update_data_catalog(&self, builder: UpdateDataCatalogInputBuilder) -> impl Future<Output = Result<UpdateDataCatalogOutput, SdkError<UpdateDataCatalogError>>> {
        self.deref().update_data_catalog(builder)
    }
    fn update_named_query(&self, builder: UpdateNamedQueryInputBuilder) -> impl Future<Output = Result<UpdateNamedQueryOutput, SdkError<UpdateNamedQueryError>>> {
        self.deref().update_named_query(builder)
    }
    fn update_notebook(&self, builder: UpdateNotebookInputBuilder) -> impl Future<Output = Result<UpdateNotebookOutput, SdkError<UpdateNotebookError>>> {
        self.deref().update_notebook(builder)
    }
    fn update_notebook_metadata(&self, builder: UpdateNotebookMetadataInputBuilder) -> impl Future<Output = Result<UpdateNotebookMetadataOutput, SdkError<UpdateNotebookMetadataError>>> {
        self.deref().update_notebook_metadata(builder)
    }
    fn update_prepared_statement(&self, builder: UpdatePreparedStatementInputBuilder) -> impl Future<Output = Result<UpdatePreparedStatementOutput, SdkError<UpdatePreparedStatementError>>> {
        self.deref().update_prepared_statement(builder)
    }
    fn update_work_group(&self, builder: UpdateWorkGroupInputBuilder) -> impl Future<Output = Result<UpdateWorkGroupOutput, SdkError<UpdateWorkGroupError>>> {
        self.deref().update_work_group(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAthenaClient {}
    impl AthenaClient for edAthenaClient {
        async fn batch_get_named_query(&self, builder: BatchGetNamedQueryInputBuilder) -> Result<BatchGetNamedQueryOutput, SdkError<BatchGetNamedQueryError>>;
        async fn batch_get_prepared_statement(&self, builder: BatchGetPreparedStatementInputBuilder) -> Result<BatchGetPreparedStatementOutput, SdkError<BatchGetPreparedStatementError>>;
        async fn batch_get_query_execution(&self, builder: BatchGetQueryExecutionInputBuilder) -> Result<BatchGetQueryExecutionOutput, SdkError<BatchGetQueryExecutionError>>;
        async fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>;
        async fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>;
        async fn create_data_catalog(&self, builder: CreateDataCatalogInputBuilder) -> Result<CreateDataCatalogOutput, SdkError<CreateDataCatalogError>>;
        async fn create_named_query(&self, builder: CreateNamedQueryInputBuilder) -> Result<CreateNamedQueryOutput, SdkError<CreateNamedQueryError>>;
        async fn create_notebook(&self, builder: CreateNotebookInputBuilder) -> Result<CreateNotebookOutput, SdkError<CreateNotebookError>>;
        async fn create_prepared_statement(&self, builder: CreatePreparedStatementInputBuilder) -> Result<CreatePreparedStatementOutput, SdkError<CreatePreparedStatementError>>;
        async fn create_presigned_notebook_url(&self, builder: CreatePresignedNotebookUrlInputBuilder) -> Result<CreatePresignedNotebookUrlOutput, SdkError<CreatePresignedNotebookUrlError>>;
        async fn create_work_group(&self, builder: CreateWorkGroupInputBuilder) -> Result<CreateWorkGroupOutput, SdkError<CreateWorkGroupError>>;
        async fn delete_capacity_reservation(&self, builder: DeleteCapacityReservationInputBuilder) -> Result<DeleteCapacityReservationOutput, SdkError<DeleteCapacityReservationError>>;
        async fn delete_data_catalog(&self, builder: DeleteDataCatalogInputBuilder) -> Result<DeleteDataCatalogOutput, SdkError<DeleteDataCatalogError>>;
        async fn delete_named_query(&self, builder: DeleteNamedQueryInputBuilder) -> Result<DeleteNamedQueryOutput, SdkError<DeleteNamedQueryError>>;
        async fn delete_notebook(&self, builder: DeleteNotebookInputBuilder) -> Result<DeleteNotebookOutput, SdkError<DeleteNotebookError>>;
        async fn delete_prepared_statement(&self, builder: DeletePreparedStatementInputBuilder) -> Result<DeletePreparedStatementOutput, SdkError<DeletePreparedStatementError>>;
        async fn delete_work_group(&self, builder: DeleteWorkGroupInputBuilder) -> Result<DeleteWorkGroupOutput, SdkError<DeleteWorkGroupError>>;
        async fn export_notebook(&self, builder: ExportNotebookInputBuilder) -> Result<ExportNotebookOutput, SdkError<ExportNotebookError>>;
        async fn get_calculation_execution(&self, builder: GetCalculationExecutionInputBuilder) -> Result<GetCalculationExecutionOutput, SdkError<GetCalculationExecutionError>>;
        async fn get_calculation_execution_code(&self, builder: GetCalculationExecutionCodeInputBuilder) -> Result<GetCalculationExecutionCodeOutput, SdkError<GetCalculationExecutionCodeError>>;
        async fn get_calculation_execution_status(&self, builder: GetCalculationExecutionStatusInputBuilder) -> Result<GetCalculationExecutionStatusOutput, SdkError<GetCalculationExecutionStatusError>>;
        async fn get_capacity_assignment_configuration(&self, builder: GetCapacityAssignmentConfigurationInputBuilder) -> Result<GetCapacityAssignmentConfigurationOutput, SdkError<GetCapacityAssignmentConfigurationError>>;
        async fn get_capacity_reservation(&self, builder: GetCapacityReservationInputBuilder) -> Result<GetCapacityReservationOutput, SdkError<GetCapacityReservationError>>;
        async fn get_data_catalog(&self, builder: GetDataCatalogInputBuilder) -> Result<GetDataCatalogOutput, SdkError<GetDataCatalogError>>;
        async fn get_database(&self, builder: GetDatabaseInputBuilder) -> Result<GetDatabaseOutput, SdkError<GetDatabaseError>>;
        async fn get_named_query(&self, builder: GetNamedQueryInputBuilder) -> Result<GetNamedQueryOutput, SdkError<GetNamedQueryError>>;
        async fn get_notebook_metadata(&self, builder: GetNotebookMetadataInputBuilder) -> Result<GetNotebookMetadataOutput, SdkError<GetNotebookMetadataError>>;
        async fn get_prepared_statement(&self, builder: GetPreparedStatementInputBuilder) -> Result<GetPreparedStatementOutput, SdkError<GetPreparedStatementError>>;
        async fn get_query_execution(&self, builder: GetQueryExecutionInputBuilder) -> Result<GetQueryExecutionOutput, SdkError<GetQueryExecutionError>>;
        async fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>;
        async fn get_query_runtime_statistics(&self, builder: GetQueryRuntimeStatisticsInputBuilder) -> Result<GetQueryRuntimeStatisticsOutput, SdkError<GetQueryRuntimeStatisticsError>>;
        async fn get_session(&self, builder: GetSessionInputBuilder) -> Result<GetSessionOutput, SdkError<GetSessionError>>;
        async fn get_session_status(&self, builder: GetSessionStatusInputBuilder) -> Result<GetSessionStatusOutput, SdkError<GetSessionStatusError>>;
        async fn get_table_metadata(&self, builder: GetTableMetadataInputBuilder) -> Result<GetTableMetadataOutput, SdkError<GetTableMetadataError>>;
        async fn get_work_group(&self, builder: GetWorkGroupInputBuilder) -> Result<GetWorkGroupOutput, SdkError<GetWorkGroupError>>;
        async fn import_notebook(&self, builder: ImportNotebookInputBuilder) -> Result<ImportNotebookOutput, SdkError<ImportNotebookError>>;
        async fn list_application_dpu_sizes(&self, builder: ListApplicationDpuSizesInputBuilder) -> Result<ListApplicationDpuSizesOutput, SdkError<ListApplicationDPUSizesError>>;
        async fn list_calculation_executions(&self, builder: ListCalculationExecutionsInputBuilder) -> Result<ListCalculationExecutionsOutput, SdkError<ListCalculationExecutionsError>>;
        async fn list_capacity_reservations(&self, builder: ListCapacityReservationsInputBuilder) -> Result<ListCapacityReservationsOutput, SdkError<ListCapacityReservationsError>>;
        async fn list_data_catalogs(&self, builder: ListDataCatalogsInputBuilder) -> Result<ListDataCatalogsOutput, SdkError<ListDataCatalogsError>>;
        async fn list_databases(&self, builder: ListDatabasesInputBuilder) -> Result<ListDatabasesOutput, SdkError<ListDatabasesError>>;
        async fn list_engine_versions(&self, builder: ListEngineVersionsInputBuilder) -> Result<ListEngineVersionsOutput, SdkError<ListEngineVersionsError>>;
        async fn list_executors(&self, builder: ListExecutorsInputBuilder) -> Result<ListExecutorsOutput, SdkError<ListExecutorsError>>;
        async fn list_named_queries(&self, builder: ListNamedQueriesInputBuilder) -> Result<ListNamedQueriesOutput, SdkError<ListNamedQueriesError>>;
        async fn list_notebook_metadata(&self, builder: ListNotebookMetadataInputBuilder) -> Result<ListNotebookMetadataOutput, SdkError<ListNotebookMetadataError>>;
        async fn list_notebook_sessions(&self, builder: ListNotebookSessionsInputBuilder) -> Result<ListNotebookSessionsOutput, SdkError<ListNotebookSessionsError>>;
        async fn list_prepared_statements(&self, builder: ListPreparedStatementsInputBuilder) -> Result<ListPreparedStatementsOutput, SdkError<ListPreparedStatementsError>>;
        async fn list_query_executions(&self, builder: ListQueryExecutionsInputBuilder) -> Result<ListQueryExecutionsOutput, SdkError<ListQueryExecutionsError>>;
        async fn list_sessions(&self, builder: ListSessionsInputBuilder) -> Result<ListSessionsOutput, SdkError<ListSessionsError>>;
        async fn list_table_metadata(&self, builder: ListTableMetadataInputBuilder) -> Result<ListTableMetadataOutput, SdkError<ListTableMetadataError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_work_groups(&self, builder: ListWorkGroupsInputBuilder) -> Result<ListWorkGroupsOutput, SdkError<ListWorkGroupsError>>;
        async fn put_capacity_assignment_configuration(&self, builder: PutCapacityAssignmentConfigurationInputBuilder) -> Result<PutCapacityAssignmentConfigurationOutput, SdkError<PutCapacityAssignmentConfigurationError>>;
        async fn start_calculation_execution(&self, builder: StartCalculationExecutionInputBuilder) -> Result<StartCalculationExecutionOutput, SdkError<StartCalculationExecutionError>>;
        async fn start_query_execution(&self, builder: StartQueryExecutionInputBuilder) -> Result<StartQueryExecutionOutput, SdkError<StartQueryExecutionError>>;
        async fn start_session(&self, builder: StartSessionInputBuilder) -> Result<StartSessionOutput, SdkError<StartSessionError>>;
        async fn stop_calculation_execution(&self, builder: StopCalculationExecutionInputBuilder) -> Result<StopCalculationExecutionOutput, SdkError<StopCalculationExecutionError>>;
        async fn stop_query_execution(&self, builder: StopQueryExecutionInputBuilder) -> Result<StopQueryExecutionOutput, SdkError<StopQueryExecutionError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> Result<TerminateSessionOutput, SdkError<TerminateSessionError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_capacity_reservation(&self, builder: UpdateCapacityReservationInputBuilder) -> Result<UpdateCapacityReservationOutput, SdkError<UpdateCapacityReservationError>>;
        async fn update_data_catalog(&self, builder: UpdateDataCatalogInputBuilder) -> Result<UpdateDataCatalogOutput, SdkError<UpdateDataCatalogError>>;
        async fn update_named_query(&self, builder: UpdateNamedQueryInputBuilder) -> Result<UpdateNamedQueryOutput, SdkError<UpdateNamedQueryError>>;
        async fn update_notebook(&self, builder: UpdateNotebookInputBuilder) -> Result<UpdateNotebookOutput, SdkError<UpdateNotebookError>>;
        async fn update_notebook_metadata(&self, builder: UpdateNotebookMetadataInputBuilder) -> Result<UpdateNotebookMetadataOutput, SdkError<UpdateNotebookMetadataError>>;
        async fn update_prepared_statement(&self, builder: UpdatePreparedStatementInputBuilder) -> Result<UpdatePreparedStatementOutput, SdkError<UpdatePreparedStatementError>>;
        async fn update_work_group(&self, builder: UpdateWorkGroupInputBuilder) -> Result<UpdateWorkGroupOutput, SdkError<UpdateWorkGroupError>>;
    }
}
