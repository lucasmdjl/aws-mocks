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
use aws_sdk_dynamodb::operation::batch_execute_statement::{builders::*, *};
use aws_sdk_dynamodb::operation::batch_get_item::{builders::*, *};
use aws_sdk_dynamodb::operation::batch_write_item::{builders::*, *};
use aws_sdk_dynamodb::operation::create_backup::{builders::*, *};
use aws_sdk_dynamodb::operation::create_global_table::{builders::*, *};
use aws_sdk_dynamodb::operation::create_table::{builders::*, *};
use aws_sdk_dynamodb::operation::delete_backup::{builders::*, *};
use aws_sdk_dynamodb::operation::delete_item::{builders::*, *};
use aws_sdk_dynamodb::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_dynamodb::operation::delete_table::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_backup::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_continuous_backups::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_contributor_insights::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_endpoints::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_export::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_global_table::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_global_table_settings::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_import::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_kinesis_streaming_destination::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_limits::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_table::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_table_replica_auto_scaling::{builders::*, *};
use aws_sdk_dynamodb::operation::describe_time_to_live::{builders::*, *};
use aws_sdk_dynamodb::operation::disable_kinesis_streaming_destination::{builders::*, *};
use aws_sdk_dynamodb::operation::enable_kinesis_streaming_destination::{builders::*, *};
use aws_sdk_dynamodb::operation::execute_statement::{builders::*, *};
use aws_sdk_dynamodb::operation::execute_transaction::{builders::*, *};
use aws_sdk_dynamodb::operation::export_table_to_point_in_time::{builders::*, *};
use aws_sdk_dynamodb::operation::get_item::{builders::*, *};
use aws_sdk_dynamodb::operation::get_resource_policy::{builders::*, *};
use aws_sdk_dynamodb::operation::import_table::{builders::*, *};
use aws_sdk_dynamodb::operation::list_backups::{builders::*, *};
use aws_sdk_dynamodb::operation::list_contributor_insights::{builders::*, *};
use aws_sdk_dynamodb::operation::list_exports::{builders::*, *};
use aws_sdk_dynamodb::operation::list_global_tables::{builders::*, *};
use aws_sdk_dynamodb::operation::list_imports::{builders::*, *};
use aws_sdk_dynamodb::operation::list_tables::{builders::*, *};
use aws_sdk_dynamodb::operation::list_tags_of_resource::{builders::*, *};
use aws_sdk_dynamodb::operation::put_item::{builders::*, *};
use aws_sdk_dynamodb::operation::put_resource_policy::{builders::*, *};
use aws_sdk_dynamodb::operation::query::{builders::*, *};
use aws_sdk_dynamodb::operation::restore_table_from_backup::{builders::*, *};
use aws_sdk_dynamodb::operation::restore_table_to_point_in_time::{builders::*, *};
use aws_sdk_dynamodb::operation::scan::{builders::*, *};
use aws_sdk_dynamodb::operation::tag_resource::{builders::*, *};
use aws_sdk_dynamodb::operation::transact_get_items::{builders::*, *};
use aws_sdk_dynamodb::operation::transact_write_items::{builders::*, *};
use aws_sdk_dynamodb::operation::untag_resource::{builders::*, *};
use aws_sdk_dynamodb::operation::update_continuous_backups::{builders::*, *};
use aws_sdk_dynamodb::operation::update_contributor_insights::{builders::*, *};
use aws_sdk_dynamodb::operation::update_global_table::{builders::*, *};
use aws_sdk_dynamodb::operation::update_global_table_settings::{builders::*, *};
use aws_sdk_dynamodb::operation::update_item::{builders::*, *};
use aws_sdk_dynamodb::operation::update_kinesis_streaming_destination::{builders::*, *};
use aws_sdk_dynamodb::operation::update_table::{builders::*, *};
use aws_sdk_dynamodb::operation::update_table_replica_auto_scaling::{builders::*, *};
use aws_sdk_dynamodb::operation::update_time_to_live::{builders::*, *};
use aws_sdk_dynamodb::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;
use std::ops::Deref;

pub use aws_sdk_dynamodb::*;

pub struct DynamoDBClientImpl(Client);
impl DynamoDBClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait DynamoDBClient {
    fn batch_execute_statement(&self, builder: BatchExecuteStatementInputBuilder) -> impl Future<Output = Result<BatchExecuteStatementOutput, SdkError<BatchExecuteStatementError>>>;
    fn batch_get_item(&self, builder: BatchGetItemInputBuilder) -> impl Future<Output = Result<BatchGetItemOutput, SdkError<BatchGetItemError>>>;
    fn batch_write_item(&self, builder: BatchWriteItemInputBuilder) -> impl Future<Output = Result<BatchWriteItemOutput, SdkError<BatchWriteItemError>>>;
    fn create_backup(&self, builder: CreateBackupInputBuilder) -> impl Future<Output = Result<CreateBackupOutput, SdkError<CreateBackupError>>>;
    fn create_global_table(&self, builder: CreateGlobalTableInputBuilder) -> impl Future<Output = Result<CreateGlobalTableOutput, SdkError<CreateGlobalTableError>>>;
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>>;
    fn delete_backup(&self, builder: DeleteBackupInputBuilder) -> impl Future<Output = Result<DeleteBackupOutput, SdkError<DeleteBackupError>>>;
    fn delete_item(&self, builder: DeleteItemInputBuilder) -> impl Future<Output = Result<DeleteItemOutput, SdkError<DeleteItemError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>>;
    fn describe_backup(&self, builder: DescribeBackupInputBuilder) -> impl Future<Output = Result<DescribeBackupOutput, SdkError<DescribeBackupError>>>;
    fn describe_continuous_backups(&self, builder: DescribeContinuousBackupsInputBuilder) -> impl Future<Output = Result<DescribeContinuousBackupsOutput, SdkError<DescribeContinuousBackupsError>>>;
    fn describe_contributor_insights(&self, builder: DescribeContributorInsightsInputBuilder) -> impl Future<Output = Result<DescribeContributorInsightsOutput, SdkError<DescribeContributorInsightsError>>>;
    fn describe_endpoints(&self, builder: DescribeEndpointsInputBuilder) -> impl Future<Output = Result<DescribeEndpointsOutput, SdkError<DescribeEndpointsError>>>;
    fn describe_export(&self, builder: DescribeExportInputBuilder) -> impl Future<Output = Result<DescribeExportOutput, SdkError<DescribeExportError>>>;
    fn describe_global_table(&self, builder: DescribeGlobalTableInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableOutput, SdkError<DescribeGlobalTableError>>>;
    fn describe_global_table_settings(&self, builder: DescribeGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableSettingsOutput, SdkError<DescribeGlobalTableSettingsError>>>;
    fn describe_import(&self, builder: DescribeImportInputBuilder) -> impl Future<Output = Result<DescribeImportOutput, SdkError<DescribeImportError>>>;
    fn describe_kinesis_streaming_destination(&self, builder: DescribeKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DescribeKinesisStreamingDestinationOutput, SdkError<DescribeKinesisStreamingDestinationError>>>;
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>>;
    fn describe_table(&self, builder: DescribeTableInputBuilder) -> impl Future<Output = Result<DescribeTableOutput, SdkError<DescribeTableError>>>;
    fn describe_table_replica_auto_scaling(&self, builder: DescribeTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<DescribeTableReplicaAutoScalingOutput, SdkError<DescribeTableReplicaAutoScalingError>>>;
    fn describe_time_to_live(&self, builder: DescribeTimeToLiveInputBuilder) -> impl Future<Output = Result<DescribeTimeToLiveOutput, SdkError<DescribeTimeToLiveError>>>;
    fn disable_kinesis_streaming_destination(&self, builder: DisableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DisableKinesisStreamingDestinationOutput, SdkError<DisableKinesisStreamingDestinationError>>>;
    fn enable_kinesis_streaming_destination(&self, builder: EnableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<EnableKinesisStreamingDestinationOutput, SdkError<EnableKinesisStreamingDestinationError>>>;
    fn execute_statement(&self, builder: ExecuteStatementInputBuilder) -> impl Future<Output = Result<ExecuteStatementOutput, SdkError<ExecuteStatementError>>>;
    fn execute_transaction(&self, builder: ExecuteTransactionInputBuilder) -> impl Future<Output = Result<ExecuteTransactionOutput, SdkError<ExecuteTransactionError>>>;
    fn export_table_to_point_in_time(&self, builder: ExportTableToPointInTimeInputBuilder) -> impl Future<Output = Result<ExportTableToPointInTimeOutput, SdkError<ExportTableToPointInTimeError>>>;
    fn get_item(&self, builder: GetItemInputBuilder) -> impl Future<Output = Result<GetItemOutput, SdkError<GetItemError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn import_table(&self, builder: ImportTableInputBuilder) -> impl Future<Output = Result<ImportTableOutput, SdkError<ImportTableError>>>;
    fn list_backups(&self, builder: ListBackupsInputBuilder) -> impl Future<Output = Result<ListBackupsOutput, SdkError<ListBackupsError>>>;
    fn list_contributor_insights(&self, builder: ListContributorInsightsInputBuilder) -> impl Future<Output = Result<ListContributorInsightsOutput, SdkError<ListContributorInsightsError>>>;
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>>;
    fn list_global_tables(&self, builder: ListGlobalTablesInputBuilder) -> impl Future<Output = Result<ListGlobalTablesOutput, SdkError<ListGlobalTablesError>>>;
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>>;
    fn list_tables(&self, builder: ListTablesInputBuilder) -> impl Future<Output = Result<ListTablesOutput, SdkError<ListTablesError>>>;
    fn list_tags_of_resource(&self, builder: ListTagsOfResourceInputBuilder) -> impl Future<Output = Result<ListTagsOfResourceOutput, SdkError<ListTagsOfResourceError>>>;
    fn put_item(&self, builder: PutItemInputBuilder) -> impl Future<Output = Result<PutItemOutput, SdkError<PutItemError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn query(&self, builder: QueryInputBuilder) -> impl Future<Output = Result<QueryOutput, SdkError<QueryError>>>;
    fn restore_table_from_backup(&self, builder: RestoreTableFromBackupInputBuilder) -> impl Future<Output = Result<RestoreTableFromBackupOutput, SdkError<RestoreTableFromBackupError>>>;
    fn restore_table_to_point_in_time(&self, builder: RestoreTableToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreTableToPointInTimeOutput, SdkError<RestoreTableToPointInTimeError>>>;
    fn scan(&self, builder: ScanInputBuilder) -> impl Future<Output = Result<ScanOutput, SdkError<ScanError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn transact_get_items(&self, builder: TransactGetItemsInputBuilder) -> impl Future<Output = Result<TransactGetItemsOutput, SdkError<TransactGetItemsError>>>;
    fn transact_write_items(&self, builder: TransactWriteItemsInputBuilder) -> impl Future<Output = Result<TransactWriteItemsOutput, SdkError<TransactWriteItemsError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_continuous_backups(&self, builder: UpdateContinuousBackupsInputBuilder) -> impl Future<Output = Result<UpdateContinuousBackupsOutput, SdkError<UpdateContinuousBackupsError>>>;
    fn update_contributor_insights(&self, builder: UpdateContributorInsightsInputBuilder) -> impl Future<Output = Result<UpdateContributorInsightsOutput, SdkError<UpdateContributorInsightsError>>>;
    fn update_global_table(&self, builder: UpdateGlobalTableInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableOutput, SdkError<UpdateGlobalTableError>>>;
    fn update_global_table_settings(&self, builder: UpdateGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableSettingsOutput, SdkError<UpdateGlobalTableSettingsError>>>;
    fn update_item(&self, builder: UpdateItemInputBuilder) -> impl Future<Output = Result<UpdateItemOutput, SdkError<UpdateItemError>>>;
    fn update_kinesis_streaming_destination(&self, builder: UpdateKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<UpdateKinesisStreamingDestinationOutput, SdkError<UpdateKinesisStreamingDestinationError>>>;
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>>;
    fn update_table_replica_auto_scaling(&self, builder: UpdateTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<UpdateTableReplicaAutoScalingOutput, SdkError<UpdateTableReplicaAutoScalingError>>>;
    fn update_time_to_live(&self, builder: UpdateTimeToLiveInputBuilder) -> impl Future<Output = Result<UpdateTimeToLiveOutput, SdkError<UpdateTimeToLiveError>>>;
}
impl DynamoDBClient for DynamoDBClientImpl {
    fn batch_execute_statement(&self, builder: BatchExecuteStatementInputBuilder) -> impl Future<Output = Result<BatchExecuteStatementOutput, SdkError<BatchExecuteStatementError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_item(&self, builder: BatchGetItemInputBuilder) -> impl Future<Output = Result<BatchGetItemOutput, SdkError<BatchGetItemError>>> {
        builder.send_with(&self.0)
    }
    fn batch_write_item(&self, builder: BatchWriteItemInputBuilder) -> impl Future<Output = Result<BatchWriteItemOutput, SdkError<BatchWriteItemError>>> {
        builder.send_with(&self.0)
    }
    fn create_backup(&self, builder: CreateBackupInputBuilder) -> impl Future<Output = Result<CreateBackupOutput, SdkError<CreateBackupError>>> {
        builder.send_with(&self.0)
    }
    fn create_global_table(&self, builder: CreateGlobalTableInputBuilder) -> impl Future<Output = Result<CreateGlobalTableOutput, SdkError<CreateGlobalTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_backup(&self, builder: DeleteBackupInputBuilder) -> impl Future<Output = Result<DeleteBackupOutput, SdkError<DeleteBackupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_item(&self, builder: DeleteItemInputBuilder) -> impl Future<Output = Result<DeleteItemOutput, SdkError<DeleteItemError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>> {
        builder.send_with(&self.0)
    }
    fn describe_backup(&self, builder: DescribeBackupInputBuilder) -> impl Future<Output = Result<DescribeBackupOutput, SdkError<DescribeBackupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_continuous_backups(&self, builder: DescribeContinuousBackupsInputBuilder) -> impl Future<Output = Result<DescribeContinuousBackupsOutput, SdkError<DescribeContinuousBackupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_contributor_insights(&self, builder: DescribeContributorInsightsInputBuilder) -> impl Future<Output = Result<DescribeContributorInsightsOutput, SdkError<DescribeContributorInsightsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoints(&self, builder: DescribeEndpointsInputBuilder) -> impl Future<Output = Result<DescribeEndpointsOutput, SdkError<DescribeEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_export(&self, builder: DescribeExportInputBuilder) -> impl Future<Output = Result<DescribeExportOutput, SdkError<DescribeExportError>>> {
        builder.send_with(&self.0)
    }
    fn describe_global_table(&self, builder: DescribeGlobalTableInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableOutput, SdkError<DescribeGlobalTableError>>> {
        builder.send_with(&self.0)
    }
    fn describe_global_table_settings(&self, builder: DescribeGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableSettingsOutput, SdkError<DescribeGlobalTableSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_import(&self, builder: DescribeImportInputBuilder) -> impl Future<Output = Result<DescribeImportOutput, SdkError<DescribeImportError>>> {
        builder.send_with(&self.0)
    }
    fn describe_kinesis_streaming_destination(&self, builder: DescribeKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DescribeKinesisStreamingDestinationOutput, SdkError<DescribeKinesisStreamingDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_table(&self, builder: DescribeTableInputBuilder) -> impl Future<Output = Result<DescribeTableOutput, SdkError<DescribeTableError>>> {
        builder.send_with(&self.0)
    }
    fn describe_table_replica_auto_scaling(&self, builder: DescribeTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<DescribeTableReplicaAutoScalingOutput, SdkError<DescribeTableReplicaAutoScalingError>>> {
        builder.send_with(&self.0)
    }
    fn describe_time_to_live(&self, builder: DescribeTimeToLiveInputBuilder) -> impl Future<Output = Result<DescribeTimeToLiveOutput, SdkError<DescribeTimeToLiveError>>> {
        builder.send_with(&self.0)
    }
    fn disable_kinesis_streaming_destination(&self, builder: DisableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DisableKinesisStreamingDestinationOutput, SdkError<DisableKinesisStreamingDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn enable_kinesis_streaming_destination(&self, builder: EnableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<EnableKinesisStreamingDestinationOutput, SdkError<EnableKinesisStreamingDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn execute_statement(&self, builder: ExecuteStatementInputBuilder) -> impl Future<Output = Result<ExecuteStatementOutput, SdkError<ExecuteStatementError>>> {
        builder.send_with(&self.0)
    }
    fn execute_transaction(&self, builder: ExecuteTransactionInputBuilder) -> impl Future<Output = Result<ExecuteTransactionOutput, SdkError<ExecuteTransactionError>>> {
        builder.send_with(&self.0)
    }
    fn export_table_to_point_in_time(&self, builder: ExportTableToPointInTimeInputBuilder) -> impl Future<Output = Result<ExportTableToPointInTimeOutput, SdkError<ExportTableToPointInTimeError>>> {
        builder.send_with(&self.0)
    }
    fn get_item(&self, builder: GetItemInputBuilder) -> impl Future<Output = Result<GetItemOutput, SdkError<GetItemError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn import_table(&self, builder: ImportTableInputBuilder) -> impl Future<Output = Result<ImportTableOutput, SdkError<ImportTableError>>> {
        builder.send_with(&self.0)
    }
    fn list_backups(&self, builder: ListBackupsInputBuilder) -> impl Future<Output = Result<ListBackupsOutput, SdkError<ListBackupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_contributor_insights(&self, builder: ListContributorInsightsInputBuilder) -> impl Future<Output = Result<ListContributorInsightsOutput, SdkError<ListContributorInsightsError>>> {
        builder.send_with(&self.0)
    }
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_global_tables(&self, builder: ListGlobalTablesInputBuilder) -> impl Future<Output = Result<ListGlobalTablesOutput, SdkError<ListGlobalTablesError>>> {
        builder.send_with(&self.0)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tables(&self, builder: ListTablesInputBuilder) -> impl Future<Output = Result<ListTablesOutput, SdkError<ListTablesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_of_resource(&self, builder: ListTagsOfResourceInputBuilder) -> impl Future<Output = Result<ListTagsOfResourceOutput, SdkError<ListTagsOfResourceError>>> {
        builder.send_with(&self.0)
    }
    fn put_item(&self, builder: PutItemInputBuilder) -> impl Future<Output = Result<PutItemOutput, SdkError<PutItemError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn query(&self, builder: QueryInputBuilder) -> impl Future<Output = Result<QueryOutput, SdkError<QueryError>>> {
        builder.send_with(&self.0)
    }
    fn restore_table_from_backup(&self, builder: RestoreTableFromBackupInputBuilder) -> impl Future<Output = Result<RestoreTableFromBackupOutput, SdkError<RestoreTableFromBackupError>>> {
        builder.send_with(&self.0)
    }
    fn restore_table_to_point_in_time(&self, builder: RestoreTableToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreTableToPointInTimeOutput, SdkError<RestoreTableToPointInTimeError>>> {
        builder.send_with(&self.0)
    }
    fn scan(&self, builder: ScanInputBuilder) -> impl Future<Output = Result<ScanOutput, SdkError<ScanError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn transact_get_items(&self, builder: TransactGetItemsInputBuilder) -> impl Future<Output = Result<TransactGetItemsOutput, SdkError<TransactGetItemsError>>> {
        builder.send_with(&self.0)
    }
    fn transact_write_items(&self, builder: TransactWriteItemsInputBuilder) -> impl Future<Output = Result<TransactWriteItemsOutput, SdkError<TransactWriteItemsError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_continuous_backups(&self, builder: UpdateContinuousBackupsInputBuilder) -> impl Future<Output = Result<UpdateContinuousBackupsOutput, SdkError<UpdateContinuousBackupsError>>> {
        builder.send_with(&self.0)
    }
    fn update_contributor_insights(&self, builder: UpdateContributorInsightsInputBuilder) -> impl Future<Output = Result<UpdateContributorInsightsOutput, SdkError<UpdateContributorInsightsError>>> {
        builder.send_with(&self.0)
    }
    fn update_global_table(&self, builder: UpdateGlobalTableInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableOutput, SdkError<UpdateGlobalTableError>>> {
        builder.send_with(&self.0)
    }
    fn update_global_table_settings(&self, builder: UpdateGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableSettingsOutput, SdkError<UpdateGlobalTableSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn update_item(&self, builder: UpdateItemInputBuilder) -> impl Future<Output = Result<UpdateItemOutput, SdkError<UpdateItemError>>> {
        builder.send_with(&self.0)
    }
    fn update_kinesis_streaming_destination(&self, builder: UpdateKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<UpdateKinesisStreamingDestinationOutput, SdkError<UpdateKinesisStreamingDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>> {
        builder.send_with(&self.0)
    }
    fn update_table_replica_auto_scaling(&self, builder: UpdateTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<UpdateTableReplicaAutoScalingOutput, SdkError<UpdateTableReplicaAutoScalingError>>> {
        builder.send_with(&self.0)
    }
    fn update_time_to_live(&self, builder: UpdateTimeToLiveInputBuilder) -> impl Future<Output = Result<UpdateTimeToLiveOutput, SdkError<UpdateTimeToLiveError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> DynamoDBClient for T
where T: Deref,
      T::Target: DynamoDBClient {
    fn batch_execute_statement(&self, builder: BatchExecuteStatementInputBuilder) -> impl Future<Output = Result<BatchExecuteStatementOutput, SdkError<BatchExecuteStatementError>>> {
        self.deref().batch_execute_statement(builder)
    }
    fn batch_get_item(&self, builder: BatchGetItemInputBuilder) -> impl Future<Output = Result<BatchGetItemOutput, SdkError<BatchGetItemError>>> {
        self.deref().batch_get_item(builder)
    }
    fn batch_write_item(&self, builder: BatchWriteItemInputBuilder) -> impl Future<Output = Result<BatchWriteItemOutput, SdkError<BatchWriteItemError>>> {
        self.deref().batch_write_item(builder)
    }
    fn create_backup(&self, builder: CreateBackupInputBuilder) -> impl Future<Output = Result<CreateBackupOutput, SdkError<CreateBackupError>>> {
        self.deref().create_backup(builder)
    }
    fn create_global_table(&self, builder: CreateGlobalTableInputBuilder) -> impl Future<Output = Result<CreateGlobalTableOutput, SdkError<CreateGlobalTableError>>> {
        self.deref().create_global_table(builder)
    }
    fn create_table(&self, builder: CreateTableInputBuilder) -> impl Future<Output = Result<CreateTableOutput, SdkError<CreateTableError>>> {
        self.deref().create_table(builder)
    }
    fn delete_backup(&self, builder: DeleteBackupInputBuilder) -> impl Future<Output = Result<DeleteBackupOutput, SdkError<DeleteBackupError>>> {
        self.deref().delete_backup(builder)
    }
    fn delete_item(&self, builder: DeleteItemInputBuilder) -> impl Future<Output = Result<DeleteItemOutput, SdkError<DeleteItemError>>> {
        self.deref().delete_item(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_table(&self, builder: DeleteTableInputBuilder) -> impl Future<Output = Result<DeleteTableOutput, SdkError<DeleteTableError>>> {
        self.deref().delete_table(builder)
    }
    fn describe_backup(&self, builder: DescribeBackupInputBuilder) -> impl Future<Output = Result<DescribeBackupOutput, SdkError<DescribeBackupError>>> {
        self.deref().describe_backup(builder)
    }
    fn describe_continuous_backups(&self, builder: DescribeContinuousBackupsInputBuilder) -> impl Future<Output = Result<DescribeContinuousBackupsOutput, SdkError<DescribeContinuousBackupsError>>> {
        self.deref().describe_continuous_backups(builder)
    }
    fn describe_contributor_insights(&self, builder: DescribeContributorInsightsInputBuilder) -> impl Future<Output = Result<DescribeContributorInsightsOutput, SdkError<DescribeContributorInsightsError>>> {
        self.deref().describe_contributor_insights(builder)
    }
    fn describe_endpoints(&self, builder: DescribeEndpointsInputBuilder) -> impl Future<Output = Result<DescribeEndpointsOutput, SdkError<DescribeEndpointsError>>> {
        self.deref().describe_endpoints(builder)
    }
    fn describe_export(&self, builder: DescribeExportInputBuilder) -> impl Future<Output = Result<DescribeExportOutput, SdkError<DescribeExportError>>> {
        self.deref().describe_export(builder)
    }
    fn describe_global_table(&self, builder: DescribeGlobalTableInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableOutput, SdkError<DescribeGlobalTableError>>> {
        self.deref().describe_global_table(builder)
    }
    fn describe_global_table_settings(&self, builder: DescribeGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<DescribeGlobalTableSettingsOutput, SdkError<DescribeGlobalTableSettingsError>>> {
        self.deref().describe_global_table_settings(builder)
    }
    fn describe_import(&self, builder: DescribeImportInputBuilder) -> impl Future<Output = Result<DescribeImportOutput, SdkError<DescribeImportError>>> {
        self.deref().describe_import(builder)
    }
    fn describe_kinesis_streaming_destination(&self, builder: DescribeKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DescribeKinesisStreamingDestinationOutput, SdkError<DescribeKinesisStreamingDestinationError>>> {
        self.deref().describe_kinesis_streaming_destination(builder)
    }
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>> {
        self.deref().describe_limits(builder)
    }
    fn describe_table(&self, builder: DescribeTableInputBuilder) -> impl Future<Output = Result<DescribeTableOutput, SdkError<DescribeTableError>>> {
        self.deref().describe_table(builder)
    }
    fn describe_table_replica_auto_scaling(&self, builder: DescribeTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<DescribeTableReplicaAutoScalingOutput, SdkError<DescribeTableReplicaAutoScalingError>>> {
        self.deref().describe_table_replica_auto_scaling(builder)
    }
    fn describe_time_to_live(&self, builder: DescribeTimeToLiveInputBuilder) -> impl Future<Output = Result<DescribeTimeToLiveOutput, SdkError<DescribeTimeToLiveError>>> {
        self.deref().describe_time_to_live(builder)
    }
    fn disable_kinesis_streaming_destination(&self, builder: DisableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<DisableKinesisStreamingDestinationOutput, SdkError<DisableKinesisStreamingDestinationError>>> {
        self.deref().disable_kinesis_streaming_destination(builder)
    }
    fn enable_kinesis_streaming_destination(&self, builder: EnableKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<EnableKinesisStreamingDestinationOutput, SdkError<EnableKinesisStreamingDestinationError>>> {
        self.deref().enable_kinesis_streaming_destination(builder)
    }
    fn execute_statement(&self, builder: ExecuteStatementInputBuilder) -> impl Future<Output = Result<ExecuteStatementOutput, SdkError<ExecuteStatementError>>> {
        self.deref().execute_statement(builder)
    }
    fn execute_transaction(&self, builder: ExecuteTransactionInputBuilder) -> impl Future<Output = Result<ExecuteTransactionOutput, SdkError<ExecuteTransactionError>>> {
        self.deref().execute_transaction(builder)
    }
    fn export_table_to_point_in_time(&self, builder: ExportTableToPointInTimeInputBuilder) -> impl Future<Output = Result<ExportTableToPointInTimeOutput, SdkError<ExportTableToPointInTimeError>>> {
        self.deref().export_table_to_point_in_time(builder)
    }
    fn get_item(&self, builder: GetItemInputBuilder) -> impl Future<Output = Result<GetItemOutput, SdkError<GetItemError>>> {
        self.deref().get_item(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        self.deref().get_resource_policy(builder)
    }
    fn import_table(&self, builder: ImportTableInputBuilder) -> impl Future<Output = Result<ImportTableOutput, SdkError<ImportTableError>>> {
        self.deref().import_table(builder)
    }
    fn list_backups(&self, builder: ListBackupsInputBuilder) -> impl Future<Output = Result<ListBackupsOutput, SdkError<ListBackupsError>>> {
        self.deref().list_backups(builder)
    }
    fn list_contributor_insights(&self, builder: ListContributorInsightsInputBuilder) -> impl Future<Output = Result<ListContributorInsightsOutput, SdkError<ListContributorInsightsError>>> {
        self.deref().list_contributor_insights(builder)
    }
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>> {
        self.deref().list_exports(builder)
    }
    fn list_global_tables(&self, builder: ListGlobalTablesInputBuilder) -> impl Future<Output = Result<ListGlobalTablesOutput, SdkError<ListGlobalTablesError>>> {
        self.deref().list_global_tables(builder)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        self.deref().list_imports(builder)
    }
    fn list_tables(&self, builder: ListTablesInputBuilder) -> impl Future<Output = Result<ListTablesOutput, SdkError<ListTablesError>>> {
        self.deref().list_tables(builder)
    }
    fn list_tags_of_resource(&self, builder: ListTagsOfResourceInputBuilder) -> impl Future<Output = Result<ListTagsOfResourceOutput, SdkError<ListTagsOfResourceError>>> {
        self.deref().list_tags_of_resource(builder)
    }
    fn put_item(&self, builder: PutItemInputBuilder) -> impl Future<Output = Result<PutItemOutput, SdkError<PutItemError>>> {
        self.deref().put_item(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn query(&self, builder: QueryInputBuilder) -> impl Future<Output = Result<QueryOutput, SdkError<QueryError>>> {
        self.deref().query(builder)
    }
    fn restore_table_from_backup(&self, builder: RestoreTableFromBackupInputBuilder) -> impl Future<Output = Result<RestoreTableFromBackupOutput, SdkError<RestoreTableFromBackupError>>> {
        self.deref().restore_table_from_backup(builder)
    }
    fn restore_table_to_point_in_time(&self, builder: RestoreTableToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreTableToPointInTimeOutput, SdkError<RestoreTableToPointInTimeError>>> {
        self.deref().restore_table_to_point_in_time(builder)
    }
    fn scan(&self, builder: ScanInputBuilder) -> impl Future<Output = Result<ScanOutput, SdkError<ScanError>>> {
        self.deref().scan(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn transact_get_items(&self, builder: TransactGetItemsInputBuilder) -> impl Future<Output = Result<TransactGetItemsOutput, SdkError<TransactGetItemsError>>> {
        self.deref().transact_get_items(builder)
    }
    fn transact_write_items(&self, builder: TransactWriteItemsInputBuilder) -> impl Future<Output = Result<TransactWriteItemsOutput, SdkError<TransactWriteItemsError>>> {
        self.deref().transact_write_items(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_continuous_backups(&self, builder: UpdateContinuousBackupsInputBuilder) -> impl Future<Output = Result<UpdateContinuousBackupsOutput, SdkError<UpdateContinuousBackupsError>>> {
        self.deref().update_continuous_backups(builder)
    }
    fn update_contributor_insights(&self, builder: UpdateContributorInsightsInputBuilder) -> impl Future<Output = Result<UpdateContributorInsightsOutput, SdkError<UpdateContributorInsightsError>>> {
        self.deref().update_contributor_insights(builder)
    }
    fn update_global_table(&self, builder: UpdateGlobalTableInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableOutput, SdkError<UpdateGlobalTableError>>> {
        self.deref().update_global_table(builder)
    }
    fn update_global_table_settings(&self, builder: UpdateGlobalTableSettingsInputBuilder) -> impl Future<Output = Result<UpdateGlobalTableSettingsOutput, SdkError<UpdateGlobalTableSettingsError>>> {
        self.deref().update_global_table_settings(builder)
    }
    fn update_item(&self, builder: UpdateItemInputBuilder) -> impl Future<Output = Result<UpdateItemOutput, SdkError<UpdateItemError>>> {
        self.deref().update_item(builder)
    }
    fn update_kinesis_streaming_destination(&self, builder: UpdateKinesisStreamingDestinationInputBuilder) -> impl Future<Output = Result<UpdateKinesisStreamingDestinationOutput, SdkError<UpdateKinesisStreamingDestinationError>>> {
        self.deref().update_kinesis_streaming_destination(builder)
    }
    fn update_table(&self, builder: UpdateTableInputBuilder) -> impl Future<Output = Result<UpdateTableOutput, SdkError<UpdateTableError>>> {
        self.deref().update_table(builder)
    }
    fn update_table_replica_auto_scaling(&self, builder: UpdateTableReplicaAutoScalingInputBuilder) -> impl Future<Output = Result<UpdateTableReplicaAutoScalingOutput, SdkError<UpdateTableReplicaAutoScalingError>>> {
        self.deref().update_table_replica_auto_scaling(builder)
    }
    fn update_time_to_live(&self, builder: UpdateTimeToLiveInputBuilder) -> impl Future<Output = Result<UpdateTimeToLiveOutput, SdkError<UpdateTimeToLiveError>>> {
        self.deref().update_time_to_live(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edDynamoDBClient {}
    impl DynamoDBClient for edDynamoDBClient {
        async fn batch_execute_statement(&self, builder: BatchExecuteStatementInputBuilder) -> Result<BatchExecuteStatementOutput, SdkError<BatchExecuteStatementError>>;
        async fn batch_get_item(&self, builder: BatchGetItemInputBuilder) -> Result<BatchGetItemOutput, SdkError<BatchGetItemError>>;
        async fn batch_write_item(&self, builder: BatchWriteItemInputBuilder) -> Result<BatchWriteItemOutput, SdkError<BatchWriteItemError>>;
        async fn create_backup(&self, builder: CreateBackupInputBuilder) -> Result<CreateBackupOutput, SdkError<CreateBackupError>>;
        async fn create_global_table(&self, builder: CreateGlobalTableInputBuilder) -> Result<CreateGlobalTableOutput, SdkError<CreateGlobalTableError>>;
        async fn create_table(&self, builder: CreateTableInputBuilder) -> Result<CreateTableOutput, SdkError<CreateTableError>>;
        async fn delete_backup(&self, builder: DeleteBackupInputBuilder) -> Result<DeleteBackupOutput, SdkError<DeleteBackupError>>;
        async fn delete_item(&self, builder: DeleteItemInputBuilder) -> Result<DeleteItemOutput, SdkError<DeleteItemError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_table(&self, builder: DeleteTableInputBuilder) -> Result<DeleteTableOutput, SdkError<DeleteTableError>>;
        async fn describe_backup(&self, builder: DescribeBackupInputBuilder) -> Result<DescribeBackupOutput, SdkError<DescribeBackupError>>;
        async fn describe_continuous_backups(&self, builder: DescribeContinuousBackupsInputBuilder) -> Result<DescribeContinuousBackupsOutput, SdkError<DescribeContinuousBackupsError>>;
        async fn describe_contributor_insights(&self, builder: DescribeContributorInsightsInputBuilder) -> Result<DescribeContributorInsightsOutput, SdkError<DescribeContributorInsightsError>>;
        async fn describe_endpoints(&self, builder: DescribeEndpointsInputBuilder) -> Result<DescribeEndpointsOutput, SdkError<DescribeEndpointsError>>;
        async fn describe_export(&self, builder: DescribeExportInputBuilder) -> Result<DescribeExportOutput, SdkError<DescribeExportError>>;
        async fn describe_global_table(&self, builder: DescribeGlobalTableInputBuilder) -> Result<DescribeGlobalTableOutput, SdkError<DescribeGlobalTableError>>;
        async fn describe_global_table_settings(&self, builder: DescribeGlobalTableSettingsInputBuilder) -> Result<DescribeGlobalTableSettingsOutput, SdkError<DescribeGlobalTableSettingsError>>;
        async fn describe_import(&self, builder: DescribeImportInputBuilder) -> Result<DescribeImportOutput, SdkError<DescribeImportError>>;
        async fn describe_kinesis_streaming_destination(&self, builder: DescribeKinesisStreamingDestinationInputBuilder) -> Result<DescribeKinesisStreamingDestinationOutput, SdkError<DescribeKinesisStreamingDestinationError>>;
        async fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>;
        async fn describe_table(&self, builder: DescribeTableInputBuilder) -> Result<DescribeTableOutput, SdkError<DescribeTableError>>;
        async fn describe_table_replica_auto_scaling(&self, builder: DescribeTableReplicaAutoScalingInputBuilder) -> Result<DescribeTableReplicaAutoScalingOutput, SdkError<DescribeTableReplicaAutoScalingError>>;
        async fn describe_time_to_live(&self, builder: DescribeTimeToLiveInputBuilder) -> Result<DescribeTimeToLiveOutput, SdkError<DescribeTimeToLiveError>>;
        async fn disable_kinesis_streaming_destination(&self, builder: DisableKinesisStreamingDestinationInputBuilder) -> Result<DisableKinesisStreamingDestinationOutput, SdkError<DisableKinesisStreamingDestinationError>>;
        async fn enable_kinesis_streaming_destination(&self, builder: EnableKinesisStreamingDestinationInputBuilder) -> Result<EnableKinesisStreamingDestinationOutput, SdkError<EnableKinesisStreamingDestinationError>>;
        async fn execute_statement(&self, builder: ExecuteStatementInputBuilder) -> Result<ExecuteStatementOutput, SdkError<ExecuteStatementError>>;
        async fn execute_transaction(&self, builder: ExecuteTransactionInputBuilder) -> Result<ExecuteTransactionOutput, SdkError<ExecuteTransactionError>>;
        async fn export_table_to_point_in_time(&self, builder: ExportTableToPointInTimeInputBuilder) -> Result<ExportTableToPointInTimeOutput, SdkError<ExportTableToPointInTimeError>>;
        async fn get_item(&self, builder: GetItemInputBuilder) -> Result<GetItemOutput, SdkError<GetItemError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn import_table(&self, builder: ImportTableInputBuilder) -> Result<ImportTableOutput, SdkError<ImportTableError>>;
        async fn list_backups(&self, builder: ListBackupsInputBuilder) -> Result<ListBackupsOutput, SdkError<ListBackupsError>>;
        async fn list_contributor_insights(&self, builder: ListContributorInsightsInputBuilder) -> Result<ListContributorInsightsOutput, SdkError<ListContributorInsightsError>>;
        async fn list_exports(&self, builder: ListExportsInputBuilder) -> Result<ListExportsOutput, SdkError<ListExportsError>>;
        async fn list_global_tables(&self, builder: ListGlobalTablesInputBuilder) -> Result<ListGlobalTablesOutput, SdkError<ListGlobalTablesError>>;
        async fn list_imports(&self, builder: ListImportsInputBuilder) -> Result<ListImportsOutput, SdkError<ListImportsError>>;
        async fn list_tables(&self, builder: ListTablesInputBuilder) -> Result<ListTablesOutput, SdkError<ListTablesError>>;
        async fn list_tags_of_resource(&self, builder: ListTagsOfResourceInputBuilder) -> Result<ListTagsOfResourceOutput, SdkError<ListTagsOfResourceError>>;
        async fn put_item(&self, builder: PutItemInputBuilder) -> Result<PutItemOutput, SdkError<PutItemError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn query(&self, builder: QueryInputBuilder) -> Result<QueryOutput, SdkError<QueryError>>;
        async fn restore_table_from_backup(&self, builder: RestoreTableFromBackupInputBuilder) -> Result<RestoreTableFromBackupOutput, SdkError<RestoreTableFromBackupError>>;
        async fn restore_table_to_point_in_time(&self, builder: RestoreTableToPointInTimeInputBuilder) -> Result<RestoreTableToPointInTimeOutput, SdkError<RestoreTableToPointInTimeError>>;
        async fn scan(&self, builder: ScanInputBuilder) -> Result<ScanOutput, SdkError<ScanError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn transact_get_items(&self, builder: TransactGetItemsInputBuilder) -> Result<TransactGetItemsOutput, SdkError<TransactGetItemsError>>;
        async fn transact_write_items(&self, builder: TransactWriteItemsInputBuilder) -> Result<TransactWriteItemsOutput, SdkError<TransactWriteItemsError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_continuous_backups(&self, builder: UpdateContinuousBackupsInputBuilder) -> Result<UpdateContinuousBackupsOutput, SdkError<UpdateContinuousBackupsError>>;
        async fn update_contributor_insights(&self, builder: UpdateContributorInsightsInputBuilder) -> Result<UpdateContributorInsightsOutput, SdkError<UpdateContributorInsightsError>>;
        async fn update_global_table(&self, builder: UpdateGlobalTableInputBuilder) -> Result<UpdateGlobalTableOutput, SdkError<UpdateGlobalTableError>>;
        async fn update_global_table_settings(&self, builder: UpdateGlobalTableSettingsInputBuilder) -> Result<UpdateGlobalTableSettingsOutput, SdkError<UpdateGlobalTableSettingsError>>;
        async fn update_item(&self, builder: UpdateItemInputBuilder) -> Result<UpdateItemOutput, SdkError<UpdateItemError>>;
        async fn update_kinesis_streaming_destination(&self, builder: UpdateKinesisStreamingDestinationInputBuilder) -> Result<UpdateKinesisStreamingDestinationOutput, SdkError<UpdateKinesisStreamingDestinationError>>;
        async fn update_table(&self, builder: UpdateTableInputBuilder) -> Result<UpdateTableOutput, SdkError<UpdateTableError>>;
        async fn update_table_replica_auto_scaling(&self, builder: UpdateTableReplicaAutoScalingInputBuilder) -> Result<UpdateTableReplicaAutoScalingOutput, SdkError<UpdateTableReplicaAutoScalingError>>;
        async fn update_time_to_live(&self, builder: UpdateTimeToLiveInputBuilder) -> Result<UpdateTimeToLiveOutput, SdkError<UpdateTimeToLiveError>>;
    }
}
