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
use aws_sdk_kinesis::operation::add_tags_to_stream::{builders::*, *};
use aws_sdk_kinesis::operation::create_stream::{builders::*, *};
use aws_sdk_kinesis::operation::decrease_stream_retention_period::{builders::*, *};
use aws_sdk_kinesis::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_kinesis::operation::delete_stream::{builders::*, *};
use aws_sdk_kinesis::operation::deregister_stream_consumer::{builders::*, *};
use aws_sdk_kinesis::operation::describe_limits::{builders::*, *};
use aws_sdk_kinesis::operation::describe_stream::{builders::*, *};
use aws_sdk_kinesis::operation::describe_stream_consumer::{builders::*, *};
use aws_sdk_kinesis::operation::describe_stream_summary::{builders::*, *};
use aws_sdk_kinesis::operation::disable_enhanced_monitoring::{builders::*, *};
use aws_sdk_kinesis::operation::enable_enhanced_monitoring::{builders::*, *};
use aws_sdk_kinesis::operation::get_records::{builders::*, *};
use aws_sdk_kinesis::operation::get_resource_policy::{builders::*, *};
use aws_sdk_kinesis::operation::get_shard_iterator::{builders::*, *};
use aws_sdk_kinesis::operation::increase_stream_retention_period::{builders::*, *};
use aws_sdk_kinesis::operation::list_shards::{builders::*, *};
use aws_sdk_kinesis::operation::list_stream_consumers::{builders::*, *};
use aws_sdk_kinesis::operation::list_streams::{builders::*, *};
use aws_sdk_kinesis::operation::list_tags_for_stream::{builders::*, *};
use aws_sdk_kinesis::operation::merge_shards::{builders::*, *};
use aws_sdk_kinesis::operation::put_record::{builders::*, *};
use aws_sdk_kinesis::operation::put_records::{builders::*, *};
use aws_sdk_kinesis::operation::put_resource_policy::{builders::*, *};
use aws_sdk_kinesis::operation::register_stream_consumer::{builders::*, *};
use aws_sdk_kinesis::operation::remove_tags_from_stream::{builders::*, *};
use aws_sdk_kinesis::operation::split_shard::{builders::*, *};
use aws_sdk_kinesis::operation::start_stream_encryption::{builders::*, *};
use aws_sdk_kinesis::operation::stop_stream_encryption::{builders::*, *};
use aws_sdk_kinesis::operation::update_shard_count::{builders::*, *};
use aws_sdk_kinesis::operation::update_stream_mode::{builders::*, *};
use aws_sdk_kinesis::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_kinesis::Client;
use std::ops::Deref;

pub use aws_sdk_kinesis::*;

pub struct KinesisClientImpl(Client);
impl KinesisClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait KinesisClient {
    fn add_tags_to_stream(&self, builder: AddTagsToStreamInputBuilder) -> impl Future<Output = Result<AddTagsToStreamOutput, SdkError<AddTagsToStreamError>>> + Send;
    fn create_stream(&self, builder: CreateStreamInputBuilder) -> impl Future<Output = Result<CreateStreamOutput, SdkError<CreateStreamError>>> + Send;
    fn decrease_stream_retention_period(&self, builder: DecreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<DecreaseStreamRetentionPeriodOutput, SdkError<DecreaseStreamRetentionPeriodError>>> + Send;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> + Send;
    fn delete_stream(&self, builder: DeleteStreamInputBuilder) -> impl Future<Output = Result<DeleteStreamOutput, SdkError<DeleteStreamError>>> + Send;
    fn deregister_stream_consumer(&self, builder: DeregisterStreamConsumerInputBuilder) -> impl Future<Output = Result<DeregisterStreamConsumerOutput, SdkError<DeregisterStreamConsumerError>>> + Send;
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>> + Send;
    fn describe_stream(&self, builder: DescribeStreamInputBuilder) -> impl Future<Output = Result<DescribeStreamOutput, SdkError<DescribeStreamError>>> + Send;
    fn describe_stream_consumer(&self, builder: DescribeStreamConsumerInputBuilder) -> impl Future<Output = Result<DescribeStreamConsumerOutput, SdkError<DescribeStreamConsumerError>>> + Send;
    fn describe_stream_summary(&self, builder: DescribeStreamSummaryInputBuilder) -> impl Future<Output = Result<DescribeStreamSummaryOutput, SdkError<DescribeStreamSummaryError>>> + Send;
    fn disable_enhanced_monitoring(&self, builder: DisableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<DisableEnhancedMonitoringOutput, SdkError<DisableEnhancedMonitoringError>>> + Send;
    fn enable_enhanced_monitoring(&self, builder: EnableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<EnableEnhancedMonitoringOutput, SdkError<EnableEnhancedMonitoringError>>> + Send;
    fn get_records(&self, builder: GetRecordsInputBuilder) -> impl Future<Output = Result<GetRecordsOutput, SdkError<GetRecordsError>>> + Send;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> + Send;
    fn get_shard_iterator(&self, builder: GetShardIteratorInputBuilder) -> impl Future<Output = Result<GetShardIteratorOutput, SdkError<GetShardIteratorError>>> + Send;
    fn increase_stream_retention_period(&self, builder: IncreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<IncreaseStreamRetentionPeriodOutput, SdkError<IncreaseStreamRetentionPeriodError>>> + Send;
    fn list_shards(&self, builder: ListShardsInputBuilder) -> impl Future<Output = Result<ListShardsOutput, SdkError<ListShardsError>>> + Send;
    fn list_stream_consumers(&self, builder: ListStreamConsumersInputBuilder) -> impl Future<Output = Result<ListStreamConsumersOutput, SdkError<ListStreamConsumersError>>> + Send;
    fn list_streams(&self, builder: ListStreamsInputBuilder) -> impl Future<Output = Result<ListStreamsOutput, SdkError<ListStreamsError>>> + Send;
    fn list_tags_for_stream(&self, builder: ListTagsForStreamInputBuilder) -> impl Future<Output = Result<ListTagsForStreamOutput, SdkError<ListTagsForStreamError>>> + Send;
    fn merge_shards(&self, builder: MergeShardsInputBuilder) -> impl Future<Output = Result<MergeShardsOutput, SdkError<MergeShardsError>>> + Send;
    fn put_record(&self, builder: PutRecordInputBuilder) -> impl Future<Output = Result<PutRecordOutput, SdkError<PutRecordError>>> + Send;
    fn put_records(&self, builder: PutRecordsInputBuilder) -> impl Future<Output = Result<PutRecordsOutput, SdkError<PutRecordsError>>> + Send;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> + Send;
    fn register_stream_consumer(&self, builder: RegisterStreamConsumerInputBuilder) -> impl Future<Output = Result<RegisterStreamConsumerOutput, SdkError<RegisterStreamConsumerError>>> + Send;
    fn remove_tags_from_stream(&self, builder: RemoveTagsFromStreamInputBuilder) -> impl Future<Output = Result<RemoveTagsFromStreamOutput, SdkError<RemoveTagsFromStreamError>>> + Send;
    fn split_shard(&self, builder: SplitShardInputBuilder) -> impl Future<Output = Result<SplitShardOutput, SdkError<SplitShardError>>> + Send;
    fn start_stream_encryption(&self, builder: StartStreamEncryptionInputBuilder) -> impl Future<Output = Result<StartStreamEncryptionOutput, SdkError<StartStreamEncryptionError>>> + Send;
    fn stop_stream_encryption(&self, builder: StopStreamEncryptionInputBuilder) -> impl Future<Output = Result<StopStreamEncryptionOutput, SdkError<StopStreamEncryptionError>>> + Send;
    fn update_shard_count(&self, builder: UpdateShardCountInputBuilder) -> impl Future<Output = Result<UpdateShardCountOutput, SdkError<UpdateShardCountError>>> + Send;
    fn update_stream_mode(&self, builder: UpdateStreamModeInputBuilder) -> impl Future<Output = Result<UpdateStreamModeOutput, SdkError<UpdateStreamModeError>>> + Send;
}
impl KinesisClient for KinesisClientImpl {
    fn add_tags_to_stream(&self, builder: AddTagsToStreamInputBuilder) -> impl Future<Output = Result<AddTagsToStreamOutput, SdkError<AddTagsToStreamError>>> {
        builder.send_with(&self.0)
    }
    fn create_stream(&self, builder: CreateStreamInputBuilder) -> impl Future<Output = Result<CreateStreamOutput, SdkError<CreateStreamError>>> {
        builder.send_with(&self.0)
    }
    fn decrease_stream_retention_period(&self, builder: DecreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<DecreaseStreamRetentionPeriodOutput, SdkError<DecreaseStreamRetentionPeriodError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stream(&self, builder: DeleteStreamInputBuilder) -> impl Future<Output = Result<DeleteStreamOutput, SdkError<DeleteStreamError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_stream_consumer(&self, builder: DeregisterStreamConsumerInputBuilder) -> impl Future<Output = Result<DeregisterStreamConsumerOutput, SdkError<DeregisterStreamConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stream(&self, builder: DescribeStreamInputBuilder) -> impl Future<Output = Result<DescribeStreamOutput, SdkError<DescribeStreamError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stream_consumer(&self, builder: DescribeStreamConsumerInputBuilder) -> impl Future<Output = Result<DescribeStreamConsumerOutput, SdkError<DescribeStreamConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stream_summary(&self, builder: DescribeStreamSummaryInputBuilder) -> impl Future<Output = Result<DescribeStreamSummaryOutput, SdkError<DescribeStreamSummaryError>>> {
        builder.send_with(&self.0)
    }
    fn disable_enhanced_monitoring(&self, builder: DisableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<DisableEnhancedMonitoringOutput, SdkError<DisableEnhancedMonitoringError>>> {
        builder.send_with(&self.0)
    }
    fn enable_enhanced_monitoring(&self, builder: EnableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<EnableEnhancedMonitoringOutput, SdkError<EnableEnhancedMonitoringError>>> {
        builder.send_with(&self.0)
    }
    fn get_records(&self, builder: GetRecordsInputBuilder) -> impl Future<Output = Result<GetRecordsOutput, SdkError<GetRecordsError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_shard_iterator(&self, builder: GetShardIteratorInputBuilder) -> impl Future<Output = Result<GetShardIteratorOutput, SdkError<GetShardIteratorError>>> {
        builder.send_with(&self.0)
    }
    fn increase_stream_retention_period(&self, builder: IncreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<IncreaseStreamRetentionPeriodOutput, SdkError<IncreaseStreamRetentionPeriodError>>> {
        builder.send_with(&self.0)
    }
    fn list_shards(&self, builder: ListShardsInputBuilder) -> impl Future<Output = Result<ListShardsOutput, SdkError<ListShardsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stream_consumers(&self, builder: ListStreamConsumersInputBuilder) -> impl Future<Output = Result<ListStreamConsumersOutput, SdkError<ListStreamConsumersError>>> {
        builder.send_with(&self.0)
    }
    fn list_streams(&self, builder: ListStreamsInputBuilder) -> impl Future<Output = Result<ListStreamsOutput, SdkError<ListStreamsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_stream(&self, builder: ListTagsForStreamInputBuilder) -> impl Future<Output = Result<ListTagsForStreamOutput, SdkError<ListTagsForStreamError>>> {
        builder.send_with(&self.0)
    }
    fn merge_shards(&self, builder: MergeShardsInputBuilder) -> impl Future<Output = Result<MergeShardsOutput, SdkError<MergeShardsError>>> {
        builder.send_with(&self.0)
    }
    fn put_record(&self, builder: PutRecordInputBuilder) -> impl Future<Output = Result<PutRecordOutput, SdkError<PutRecordError>>> {
        builder.send_with(&self.0)
    }
    fn put_records(&self, builder: PutRecordsInputBuilder) -> impl Future<Output = Result<PutRecordsOutput, SdkError<PutRecordsError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn register_stream_consumer(&self, builder: RegisterStreamConsumerInputBuilder) -> impl Future<Output = Result<RegisterStreamConsumerOutput, SdkError<RegisterStreamConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags_from_stream(&self, builder: RemoveTagsFromStreamInputBuilder) -> impl Future<Output = Result<RemoveTagsFromStreamOutput, SdkError<RemoveTagsFromStreamError>>> {
        builder.send_with(&self.0)
    }
    fn split_shard(&self, builder: SplitShardInputBuilder) -> impl Future<Output = Result<SplitShardOutput, SdkError<SplitShardError>>> {
        builder.send_with(&self.0)
    }
    fn start_stream_encryption(&self, builder: StartStreamEncryptionInputBuilder) -> impl Future<Output = Result<StartStreamEncryptionOutput, SdkError<StartStreamEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_stream_encryption(&self, builder: StopStreamEncryptionInputBuilder) -> impl Future<Output = Result<StopStreamEncryptionOutput, SdkError<StopStreamEncryptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_shard_count(&self, builder: UpdateShardCountInputBuilder) -> impl Future<Output = Result<UpdateShardCountOutput, SdkError<UpdateShardCountError>>> {
        builder.send_with(&self.0)
    }
    fn update_stream_mode(&self, builder: UpdateStreamModeInputBuilder) -> impl Future<Output = Result<UpdateStreamModeOutput, SdkError<UpdateStreamModeError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> KinesisClient for T
where T: Deref,
      T::Target: KinesisClient {
    fn add_tags_to_stream(&self, builder: AddTagsToStreamInputBuilder) -> impl Future<Output = Result<AddTagsToStreamOutput, SdkError<AddTagsToStreamError>>> {
        self.deref().add_tags_to_stream(builder)
    }
    fn create_stream(&self, builder: CreateStreamInputBuilder) -> impl Future<Output = Result<CreateStreamOutput, SdkError<CreateStreamError>>> {
        self.deref().create_stream(builder)
    }
    fn decrease_stream_retention_period(&self, builder: DecreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<DecreaseStreamRetentionPeriodOutput, SdkError<DecreaseStreamRetentionPeriodError>>> {
        self.deref().decrease_stream_retention_period(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_stream(&self, builder: DeleteStreamInputBuilder) -> impl Future<Output = Result<DeleteStreamOutput, SdkError<DeleteStreamError>>> {
        self.deref().delete_stream(builder)
    }
    fn deregister_stream_consumer(&self, builder: DeregisterStreamConsumerInputBuilder) -> impl Future<Output = Result<DeregisterStreamConsumerOutput, SdkError<DeregisterStreamConsumerError>>> {
        self.deref().deregister_stream_consumer(builder)
    }
    fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> impl Future<Output = Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>> {
        self.deref().describe_limits(builder)
    }
    fn describe_stream(&self, builder: DescribeStreamInputBuilder) -> impl Future<Output = Result<DescribeStreamOutput, SdkError<DescribeStreamError>>> {
        self.deref().describe_stream(builder)
    }
    fn describe_stream_consumer(&self, builder: DescribeStreamConsumerInputBuilder) -> impl Future<Output = Result<DescribeStreamConsumerOutput, SdkError<DescribeStreamConsumerError>>> {
        self.deref().describe_stream_consumer(builder)
    }
    fn describe_stream_summary(&self, builder: DescribeStreamSummaryInputBuilder) -> impl Future<Output = Result<DescribeStreamSummaryOutput, SdkError<DescribeStreamSummaryError>>> {
        self.deref().describe_stream_summary(builder)
    }
    fn disable_enhanced_monitoring(&self, builder: DisableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<DisableEnhancedMonitoringOutput, SdkError<DisableEnhancedMonitoringError>>> {
        self.deref().disable_enhanced_monitoring(builder)
    }
    fn enable_enhanced_monitoring(&self, builder: EnableEnhancedMonitoringInputBuilder) -> impl Future<Output = Result<EnableEnhancedMonitoringOutput, SdkError<EnableEnhancedMonitoringError>>> {
        self.deref().enable_enhanced_monitoring(builder)
    }
    fn get_records(&self, builder: GetRecordsInputBuilder) -> impl Future<Output = Result<GetRecordsOutput, SdkError<GetRecordsError>>> {
        self.deref().get_records(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        self.deref().get_resource_policy(builder)
    }
    fn get_shard_iterator(&self, builder: GetShardIteratorInputBuilder) -> impl Future<Output = Result<GetShardIteratorOutput, SdkError<GetShardIteratorError>>> {
        self.deref().get_shard_iterator(builder)
    }
    fn increase_stream_retention_period(&self, builder: IncreaseStreamRetentionPeriodInputBuilder) -> impl Future<Output = Result<IncreaseStreamRetentionPeriodOutput, SdkError<IncreaseStreamRetentionPeriodError>>> {
        self.deref().increase_stream_retention_period(builder)
    }
    fn list_shards(&self, builder: ListShardsInputBuilder) -> impl Future<Output = Result<ListShardsOutput, SdkError<ListShardsError>>> {
        self.deref().list_shards(builder)
    }
    fn list_stream_consumers(&self, builder: ListStreamConsumersInputBuilder) -> impl Future<Output = Result<ListStreamConsumersOutput, SdkError<ListStreamConsumersError>>> {
        self.deref().list_stream_consumers(builder)
    }
    fn list_streams(&self, builder: ListStreamsInputBuilder) -> impl Future<Output = Result<ListStreamsOutput, SdkError<ListStreamsError>>> {
        self.deref().list_streams(builder)
    }
    fn list_tags_for_stream(&self, builder: ListTagsForStreamInputBuilder) -> impl Future<Output = Result<ListTagsForStreamOutput, SdkError<ListTagsForStreamError>>> {
        self.deref().list_tags_for_stream(builder)
    }
    fn merge_shards(&self, builder: MergeShardsInputBuilder) -> impl Future<Output = Result<MergeShardsOutput, SdkError<MergeShardsError>>> {
        self.deref().merge_shards(builder)
    }
    fn put_record(&self, builder: PutRecordInputBuilder) -> impl Future<Output = Result<PutRecordOutput, SdkError<PutRecordError>>> {
        self.deref().put_record(builder)
    }
    fn put_records(&self, builder: PutRecordsInputBuilder) -> impl Future<Output = Result<PutRecordsOutput, SdkError<PutRecordsError>>> {
        self.deref().put_records(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn register_stream_consumer(&self, builder: RegisterStreamConsumerInputBuilder) -> impl Future<Output = Result<RegisterStreamConsumerOutput, SdkError<RegisterStreamConsumerError>>> {
        self.deref().register_stream_consumer(builder)
    }
    fn remove_tags_from_stream(&self, builder: RemoveTagsFromStreamInputBuilder) -> impl Future<Output = Result<RemoveTagsFromStreamOutput, SdkError<RemoveTagsFromStreamError>>> {
        self.deref().remove_tags_from_stream(builder)
    }
    fn split_shard(&self, builder: SplitShardInputBuilder) -> impl Future<Output = Result<SplitShardOutput, SdkError<SplitShardError>>> {
        self.deref().split_shard(builder)
    }
    fn start_stream_encryption(&self, builder: StartStreamEncryptionInputBuilder) -> impl Future<Output = Result<StartStreamEncryptionOutput, SdkError<StartStreamEncryptionError>>> {
        self.deref().start_stream_encryption(builder)
    }
    fn stop_stream_encryption(&self, builder: StopStreamEncryptionInputBuilder) -> impl Future<Output = Result<StopStreamEncryptionOutput, SdkError<StopStreamEncryptionError>>> {
        self.deref().stop_stream_encryption(builder)
    }
    fn update_shard_count(&self, builder: UpdateShardCountInputBuilder) -> impl Future<Output = Result<UpdateShardCountOutput, SdkError<UpdateShardCountError>>> {
        self.deref().update_shard_count(builder)
    }
    fn update_stream_mode(&self, builder: UpdateStreamModeInputBuilder) -> impl Future<Output = Result<UpdateStreamModeOutput, SdkError<UpdateStreamModeError>>> {
        self.deref().update_stream_mode(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edKinesisClient {}
    impl KinesisClient for edKinesisClient {
        async fn add_tags_to_stream(&self, builder: AddTagsToStreamInputBuilder) -> Result<AddTagsToStreamOutput, SdkError<AddTagsToStreamError>>;
        async fn create_stream(&self, builder: CreateStreamInputBuilder) -> Result<CreateStreamOutput, SdkError<CreateStreamError>>;
        async fn decrease_stream_retention_period(&self, builder: DecreaseStreamRetentionPeriodInputBuilder) -> Result<DecreaseStreamRetentionPeriodOutput, SdkError<DecreaseStreamRetentionPeriodError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_stream(&self, builder: DeleteStreamInputBuilder) -> Result<DeleteStreamOutput, SdkError<DeleteStreamError>>;
        async fn deregister_stream_consumer(&self, builder: DeregisterStreamConsumerInputBuilder) -> Result<DeregisterStreamConsumerOutput, SdkError<DeregisterStreamConsumerError>>;
        async fn describe_limits(&self, builder: DescribeLimitsInputBuilder) -> Result<DescribeLimitsOutput, SdkError<DescribeLimitsError>>;
        async fn describe_stream(&self, builder: DescribeStreamInputBuilder) -> Result<DescribeStreamOutput, SdkError<DescribeStreamError>>;
        async fn describe_stream_consumer(&self, builder: DescribeStreamConsumerInputBuilder) -> Result<DescribeStreamConsumerOutput, SdkError<DescribeStreamConsumerError>>;
        async fn describe_stream_summary(&self, builder: DescribeStreamSummaryInputBuilder) -> Result<DescribeStreamSummaryOutput, SdkError<DescribeStreamSummaryError>>;
        async fn disable_enhanced_monitoring(&self, builder: DisableEnhancedMonitoringInputBuilder) -> Result<DisableEnhancedMonitoringOutput, SdkError<DisableEnhancedMonitoringError>>;
        async fn enable_enhanced_monitoring(&self, builder: EnableEnhancedMonitoringInputBuilder) -> Result<EnableEnhancedMonitoringOutput, SdkError<EnableEnhancedMonitoringError>>;
        async fn get_records(&self, builder: GetRecordsInputBuilder) -> Result<GetRecordsOutput, SdkError<GetRecordsError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn get_shard_iterator(&self, builder: GetShardIteratorInputBuilder) -> Result<GetShardIteratorOutput, SdkError<GetShardIteratorError>>;
        async fn increase_stream_retention_period(&self, builder: IncreaseStreamRetentionPeriodInputBuilder) -> Result<IncreaseStreamRetentionPeriodOutput, SdkError<IncreaseStreamRetentionPeriodError>>;
        async fn list_shards(&self, builder: ListShardsInputBuilder) -> Result<ListShardsOutput, SdkError<ListShardsError>>;
        async fn list_stream_consumers(&self, builder: ListStreamConsumersInputBuilder) -> Result<ListStreamConsumersOutput, SdkError<ListStreamConsumersError>>;
        async fn list_streams(&self, builder: ListStreamsInputBuilder) -> Result<ListStreamsOutput, SdkError<ListStreamsError>>;
        async fn list_tags_for_stream(&self, builder: ListTagsForStreamInputBuilder) -> Result<ListTagsForStreamOutput, SdkError<ListTagsForStreamError>>;
        async fn merge_shards(&self, builder: MergeShardsInputBuilder) -> Result<MergeShardsOutput, SdkError<MergeShardsError>>;
        async fn put_record(&self, builder: PutRecordInputBuilder) -> Result<PutRecordOutput, SdkError<PutRecordError>>;
        async fn put_records(&self, builder: PutRecordsInputBuilder) -> Result<PutRecordsOutput, SdkError<PutRecordsError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn register_stream_consumer(&self, builder: RegisterStreamConsumerInputBuilder) -> Result<RegisterStreamConsumerOutput, SdkError<RegisterStreamConsumerError>>;
        async fn remove_tags_from_stream(&self, builder: RemoveTagsFromStreamInputBuilder) -> Result<RemoveTagsFromStreamOutput, SdkError<RemoveTagsFromStreamError>>;
        async fn split_shard(&self, builder: SplitShardInputBuilder) -> Result<SplitShardOutput, SdkError<SplitShardError>>;
        async fn start_stream_encryption(&self, builder: StartStreamEncryptionInputBuilder) -> Result<StartStreamEncryptionOutput, SdkError<StartStreamEncryptionError>>;
        async fn stop_stream_encryption(&self, builder: StopStreamEncryptionInputBuilder) -> Result<StopStreamEncryptionOutput, SdkError<StopStreamEncryptionError>>;
        async fn update_shard_count(&self, builder: UpdateShardCountInputBuilder) -> Result<UpdateShardCountOutput, SdkError<UpdateShardCountError>>;
        async fn update_stream_mode(&self, builder: UpdateStreamModeInputBuilder) -> Result<UpdateStreamModeOutput, SdkError<UpdateStreamModeError>>;
    }
}
