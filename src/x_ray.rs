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
use aws_sdk_xray::operation::batch_get_traces::{builders::*, *};
use aws_sdk_xray::operation::create_group::{builders::*, *};
use aws_sdk_xray::operation::create_sampling_rule::{builders::*, *};
use aws_sdk_xray::operation::delete_group::{builders::*, *};
use aws_sdk_xray::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_xray::operation::delete_sampling_rule::{builders::*, *};
use aws_sdk_xray::operation::get_encryption_config::{builders::*, *};
use aws_sdk_xray::operation::get_group::{builders::*, *};
use aws_sdk_xray::operation::get_groups::{builders::*, *};
use aws_sdk_xray::operation::get_insight::{builders::*, *};
use aws_sdk_xray::operation::get_insight_events::{builders::*, *};
use aws_sdk_xray::operation::get_insight_impact_graph::{builders::*, *};
use aws_sdk_xray::operation::get_insight_summaries::{builders::*, *};
use aws_sdk_xray::operation::get_sampling_rules::{builders::*, *};
use aws_sdk_xray::operation::get_sampling_statistic_summaries::{builders::*, *};
use aws_sdk_xray::operation::get_sampling_targets::{builders::*, *};
use aws_sdk_xray::operation::get_service_graph::{builders::*, *};
use aws_sdk_xray::operation::get_time_series_service_statistics::{builders::*, *};
use aws_sdk_xray::operation::get_trace_graph::{builders::*, *};
use aws_sdk_xray::operation::get_trace_summaries::{builders::*, *};
use aws_sdk_xray::operation::list_resource_policies::{builders::*, *};
use aws_sdk_xray::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_xray::operation::put_encryption_config::{builders::*, *};
use aws_sdk_xray::operation::put_resource_policy::{builders::*, *};
use aws_sdk_xray::operation::put_telemetry_records::{builders::*, *};
use aws_sdk_xray::operation::put_trace_segments::{builders::*, *};
use aws_sdk_xray::operation::tag_resource::{builders::*, *};
use aws_sdk_xray::operation::untag_resource::{builders::*, *};
use aws_sdk_xray::operation::update_group::{builders::*, *};
use aws_sdk_xray::operation::update_sampling_rule::{builders::*, *};
use aws_sdk_xray::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_xray::Client;
use std::ops::Deref;

pub use aws_sdk_xray::*;

pub struct XRayClientImpl(Client);
impl XRayClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait XRayClient {
    fn batch_get_traces(&self, builder: BatchGetTracesInputBuilder) -> impl Future<Output = Result<BatchGetTracesOutput, SdkError<BatchGetTracesError>>> + Send;
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> + Send;
    fn create_sampling_rule(&self, builder: CreateSamplingRuleInputBuilder) -> impl Future<Output = Result<CreateSamplingRuleOutput, SdkError<CreateSamplingRuleError>>> + Send;
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> + Send;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> + Send;
    fn delete_sampling_rule(&self, builder: DeleteSamplingRuleInputBuilder) -> impl Future<Output = Result<DeleteSamplingRuleOutput, SdkError<DeleteSamplingRuleError>>> + Send;
    fn get_encryption_config(&self, builder: GetEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetEncryptionConfigOutput, SdkError<GetEncryptionConfigError>>> + Send;
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> + Send;
    fn get_groups(&self, builder: GetGroupsInputBuilder) -> impl Future<Output = Result<GetGroupsOutput, SdkError<GetGroupsError>>> + Send;
    fn get_insight(&self, builder: GetInsightInputBuilder) -> impl Future<Output = Result<GetInsightOutput, SdkError<GetInsightError>>> + Send;
    fn get_insight_events(&self, builder: GetInsightEventsInputBuilder) -> impl Future<Output = Result<GetInsightEventsOutput, SdkError<GetInsightEventsError>>> + Send;
    fn get_insight_impact_graph(&self, builder: GetInsightImpactGraphInputBuilder) -> impl Future<Output = Result<GetInsightImpactGraphOutput, SdkError<GetInsightImpactGraphError>>> + Send;
    fn get_insight_summaries(&self, builder: GetInsightSummariesInputBuilder) -> impl Future<Output = Result<GetInsightSummariesOutput, SdkError<GetInsightSummariesError>>> + Send;
    fn get_sampling_rules(&self, builder: GetSamplingRulesInputBuilder) -> impl Future<Output = Result<GetSamplingRulesOutput, SdkError<GetSamplingRulesError>>> + Send;
    fn get_sampling_statistic_summaries(&self, builder: GetSamplingStatisticSummariesInputBuilder) -> impl Future<Output = Result<GetSamplingStatisticSummariesOutput, SdkError<GetSamplingStatisticSummariesError>>> + Send;
    fn get_sampling_targets(&self, builder: GetSamplingTargetsInputBuilder) -> impl Future<Output = Result<GetSamplingTargetsOutput, SdkError<GetSamplingTargetsError>>> + Send;
    fn get_service_graph(&self, builder: GetServiceGraphInputBuilder) -> impl Future<Output = Result<GetServiceGraphOutput, SdkError<GetServiceGraphError>>> + Send;
    fn get_time_series_service_statistics(&self, builder: GetTimeSeriesServiceStatisticsInputBuilder) -> impl Future<Output = Result<GetTimeSeriesServiceStatisticsOutput, SdkError<GetTimeSeriesServiceStatisticsError>>> + Send;
    fn get_trace_graph(&self, builder: GetTraceGraphInputBuilder) -> impl Future<Output = Result<GetTraceGraphOutput, SdkError<GetTraceGraphError>>> + Send;
    fn get_trace_summaries(&self, builder: GetTraceSummariesInputBuilder) -> impl Future<Output = Result<GetTraceSummariesOutput, SdkError<GetTraceSummariesError>>> + Send;
    fn list_resource_policies(&self, builder: ListResourcePoliciesInputBuilder) -> impl Future<Output = Result<ListResourcePoliciesOutput, SdkError<ListResourcePoliciesError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn put_encryption_config(&self, builder: PutEncryptionConfigInputBuilder) -> impl Future<Output = Result<PutEncryptionConfigOutput, SdkError<PutEncryptionConfigError>>> + Send;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> + Send;
    fn put_telemetry_records(&self, builder: PutTelemetryRecordsInputBuilder) -> impl Future<Output = Result<PutTelemetryRecordsOutput, SdkError<PutTelemetryRecordsError>>> + Send;
    fn put_trace_segments(&self, builder: PutTraceSegmentsInputBuilder) -> impl Future<Output = Result<PutTraceSegmentsOutput, SdkError<PutTraceSegmentsError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> + Send;
    fn update_sampling_rule(&self, builder: UpdateSamplingRuleInputBuilder) -> impl Future<Output = Result<UpdateSamplingRuleOutput, SdkError<UpdateSamplingRuleError>>> + Send;
}
impl XRayClient for XRayClientImpl {
    fn batch_get_traces(&self, builder: BatchGetTracesInputBuilder) -> impl Future<Output = Result<BatchGetTracesOutput, SdkError<BatchGetTracesError>>> {
        builder.send_with(&self.0)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_sampling_rule(&self, builder: CreateSamplingRuleInputBuilder) -> impl Future<Output = Result<CreateSamplingRuleOutput, SdkError<CreateSamplingRuleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_sampling_rule(&self, builder: DeleteSamplingRuleInputBuilder) -> impl Future<Output = Result<DeleteSamplingRuleOutput, SdkError<DeleteSamplingRuleError>>> {
        builder.send_with(&self.0)
    }
    fn get_encryption_config(&self, builder: GetEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetEncryptionConfigOutput, SdkError<GetEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn get_groups(&self, builder: GetGroupsInputBuilder) -> impl Future<Output = Result<GetGroupsOutput, SdkError<GetGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight(&self, builder: GetInsightInputBuilder) -> impl Future<Output = Result<GetInsightOutput, SdkError<GetInsightError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight_events(&self, builder: GetInsightEventsInputBuilder) -> impl Future<Output = Result<GetInsightEventsOutput, SdkError<GetInsightEventsError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight_impact_graph(&self, builder: GetInsightImpactGraphInputBuilder) -> impl Future<Output = Result<GetInsightImpactGraphOutput, SdkError<GetInsightImpactGraphError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight_summaries(&self, builder: GetInsightSummariesInputBuilder) -> impl Future<Output = Result<GetInsightSummariesOutput, SdkError<GetInsightSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sampling_rules(&self, builder: GetSamplingRulesInputBuilder) -> impl Future<Output = Result<GetSamplingRulesOutput, SdkError<GetSamplingRulesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sampling_statistic_summaries(&self, builder: GetSamplingStatisticSummariesInputBuilder) -> impl Future<Output = Result<GetSamplingStatisticSummariesOutput, SdkError<GetSamplingStatisticSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sampling_targets(&self, builder: GetSamplingTargetsInputBuilder) -> impl Future<Output = Result<GetSamplingTargetsOutput, SdkError<GetSamplingTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn get_service_graph(&self, builder: GetServiceGraphInputBuilder) -> impl Future<Output = Result<GetServiceGraphOutput, SdkError<GetServiceGraphError>>> {
        builder.send_with(&self.0)
    }
    fn get_time_series_service_statistics(&self, builder: GetTimeSeriesServiceStatisticsInputBuilder) -> impl Future<Output = Result<GetTimeSeriesServiceStatisticsOutput, SdkError<GetTimeSeriesServiceStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_trace_graph(&self, builder: GetTraceGraphInputBuilder) -> impl Future<Output = Result<GetTraceGraphOutput, SdkError<GetTraceGraphError>>> {
        builder.send_with(&self.0)
    }
    fn get_trace_summaries(&self, builder: GetTraceSummariesInputBuilder) -> impl Future<Output = Result<GetTraceSummariesOutput, SdkError<GetTraceSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_policies(&self, builder: ListResourcePoliciesInputBuilder) -> impl Future<Output = Result<ListResourcePoliciesOutput, SdkError<ListResourcePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn put_encryption_config(&self, builder: PutEncryptionConfigInputBuilder) -> impl Future<Output = Result<PutEncryptionConfigOutput, SdkError<PutEncryptionConfigError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_telemetry_records(&self, builder: PutTelemetryRecordsInputBuilder) -> impl Future<Output = Result<PutTelemetryRecordsOutput, SdkError<PutTelemetryRecordsError>>> {
        builder.send_with(&self.0)
    }
    fn put_trace_segments(&self, builder: PutTraceSegmentsInputBuilder) -> impl Future<Output = Result<PutTraceSegmentsOutput, SdkError<PutTraceSegmentsError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_sampling_rule(&self, builder: UpdateSamplingRuleInputBuilder) -> impl Future<Output = Result<UpdateSamplingRuleOutput, SdkError<UpdateSamplingRuleError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> XRayClient for T
where T: Deref,
      T::Target: XRayClient {
    fn batch_get_traces(&self, builder: BatchGetTracesInputBuilder) -> impl Future<Output = Result<BatchGetTracesOutput, SdkError<BatchGetTracesError>>> {
        self.deref().batch_get_traces(builder)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        self.deref().create_group(builder)
    }
    fn create_sampling_rule(&self, builder: CreateSamplingRuleInputBuilder) -> impl Future<Output = Result<CreateSamplingRuleOutput, SdkError<CreateSamplingRuleError>>> {
        self.deref().create_sampling_rule(builder)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        self.deref().delete_group(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_sampling_rule(&self, builder: DeleteSamplingRuleInputBuilder) -> impl Future<Output = Result<DeleteSamplingRuleOutput, SdkError<DeleteSamplingRuleError>>> {
        self.deref().delete_sampling_rule(builder)
    }
    fn get_encryption_config(&self, builder: GetEncryptionConfigInputBuilder) -> impl Future<Output = Result<GetEncryptionConfigOutput, SdkError<GetEncryptionConfigError>>> {
        self.deref().get_encryption_config(builder)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        self.deref().get_group(builder)
    }
    fn get_groups(&self, builder: GetGroupsInputBuilder) -> impl Future<Output = Result<GetGroupsOutput, SdkError<GetGroupsError>>> {
        self.deref().get_groups(builder)
    }
    fn get_insight(&self, builder: GetInsightInputBuilder) -> impl Future<Output = Result<GetInsightOutput, SdkError<GetInsightError>>> {
        self.deref().get_insight(builder)
    }
    fn get_insight_events(&self, builder: GetInsightEventsInputBuilder) -> impl Future<Output = Result<GetInsightEventsOutput, SdkError<GetInsightEventsError>>> {
        self.deref().get_insight_events(builder)
    }
    fn get_insight_impact_graph(&self, builder: GetInsightImpactGraphInputBuilder) -> impl Future<Output = Result<GetInsightImpactGraphOutput, SdkError<GetInsightImpactGraphError>>> {
        self.deref().get_insight_impact_graph(builder)
    }
    fn get_insight_summaries(&self, builder: GetInsightSummariesInputBuilder) -> impl Future<Output = Result<GetInsightSummariesOutput, SdkError<GetInsightSummariesError>>> {
        self.deref().get_insight_summaries(builder)
    }
    fn get_sampling_rules(&self, builder: GetSamplingRulesInputBuilder) -> impl Future<Output = Result<GetSamplingRulesOutput, SdkError<GetSamplingRulesError>>> {
        self.deref().get_sampling_rules(builder)
    }
    fn get_sampling_statistic_summaries(&self, builder: GetSamplingStatisticSummariesInputBuilder) -> impl Future<Output = Result<GetSamplingStatisticSummariesOutput, SdkError<GetSamplingStatisticSummariesError>>> {
        self.deref().get_sampling_statistic_summaries(builder)
    }
    fn get_sampling_targets(&self, builder: GetSamplingTargetsInputBuilder) -> impl Future<Output = Result<GetSamplingTargetsOutput, SdkError<GetSamplingTargetsError>>> {
        self.deref().get_sampling_targets(builder)
    }
    fn get_service_graph(&self, builder: GetServiceGraphInputBuilder) -> impl Future<Output = Result<GetServiceGraphOutput, SdkError<GetServiceGraphError>>> {
        self.deref().get_service_graph(builder)
    }
    fn get_time_series_service_statistics(&self, builder: GetTimeSeriesServiceStatisticsInputBuilder) -> impl Future<Output = Result<GetTimeSeriesServiceStatisticsOutput, SdkError<GetTimeSeriesServiceStatisticsError>>> {
        self.deref().get_time_series_service_statistics(builder)
    }
    fn get_trace_graph(&self, builder: GetTraceGraphInputBuilder) -> impl Future<Output = Result<GetTraceGraphOutput, SdkError<GetTraceGraphError>>> {
        self.deref().get_trace_graph(builder)
    }
    fn get_trace_summaries(&self, builder: GetTraceSummariesInputBuilder) -> impl Future<Output = Result<GetTraceSummariesOutput, SdkError<GetTraceSummariesError>>> {
        self.deref().get_trace_summaries(builder)
    }
    fn list_resource_policies(&self, builder: ListResourcePoliciesInputBuilder) -> impl Future<Output = Result<ListResourcePoliciesOutput, SdkError<ListResourcePoliciesError>>> {
        self.deref().list_resource_policies(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn put_encryption_config(&self, builder: PutEncryptionConfigInputBuilder) -> impl Future<Output = Result<PutEncryptionConfigOutput, SdkError<PutEncryptionConfigError>>> {
        self.deref().put_encryption_config(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn put_telemetry_records(&self, builder: PutTelemetryRecordsInputBuilder) -> impl Future<Output = Result<PutTelemetryRecordsOutput, SdkError<PutTelemetryRecordsError>>> {
        self.deref().put_telemetry_records(builder)
    }
    fn put_trace_segments(&self, builder: PutTraceSegmentsInputBuilder) -> impl Future<Output = Result<PutTraceSegmentsOutput, SdkError<PutTraceSegmentsError>>> {
        self.deref().put_trace_segments(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        self.deref().update_group(builder)
    }
    fn update_sampling_rule(&self, builder: UpdateSamplingRuleInputBuilder) -> impl Future<Output = Result<UpdateSamplingRuleOutput, SdkError<UpdateSamplingRuleError>>> {
        self.deref().update_sampling_rule(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edXRayClient {}
    impl XRayClient for edXRayClient {
        async fn batch_get_traces(&self, builder: BatchGetTracesInputBuilder) -> Result<BatchGetTracesOutput, SdkError<BatchGetTracesError>>;
        async fn create_group(&self, builder: CreateGroupInputBuilder) -> Result<CreateGroupOutput, SdkError<CreateGroupError>>;
        async fn create_sampling_rule(&self, builder: CreateSamplingRuleInputBuilder) -> Result<CreateSamplingRuleOutput, SdkError<CreateSamplingRuleError>>;
        async fn delete_group(&self, builder: DeleteGroupInputBuilder) -> Result<DeleteGroupOutput, SdkError<DeleteGroupError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_sampling_rule(&self, builder: DeleteSamplingRuleInputBuilder) -> Result<DeleteSamplingRuleOutput, SdkError<DeleteSamplingRuleError>>;
        async fn get_encryption_config(&self, builder: GetEncryptionConfigInputBuilder) -> Result<GetEncryptionConfigOutput, SdkError<GetEncryptionConfigError>>;
        async fn get_group(&self, builder: GetGroupInputBuilder) -> Result<GetGroupOutput, SdkError<GetGroupError>>;
        async fn get_groups(&self, builder: GetGroupsInputBuilder) -> Result<GetGroupsOutput, SdkError<GetGroupsError>>;
        async fn get_insight(&self, builder: GetInsightInputBuilder) -> Result<GetInsightOutput, SdkError<GetInsightError>>;
        async fn get_insight_events(&self, builder: GetInsightEventsInputBuilder) -> Result<GetInsightEventsOutput, SdkError<GetInsightEventsError>>;
        async fn get_insight_impact_graph(&self, builder: GetInsightImpactGraphInputBuilder) -> Result<GetInsightImpactGraphOutput, SdkError<GetInsightImpactGraphError>>;
        async fn get_insight_summaries(&self, builder: GetInsightSummariesInputBuilder) -> Result<GetInsightSummariesOutput, SdkError<GetInsightSummariesError>>;
        async fn get_sampling_rules(&self, builder: GetSamplingRulesInputBuilder) -> Result<GetSamplingRulesOutput, SdkError<GetSamplingRulesError>>;
        async fn get_sampling_statistic_summaries(&self, builder: GetSamplingStatisticSummariesInputBuilder) -> Result<GetSamplingStatisticSummariesOutput, SdkError<GetSamplingStatisticSummariesError>>;
        async fn get_sampling_targets(&self, builder: GetSamplingTargetsInputBuilder) -> Result<GetSamplingTargetsOutput, SdkError<GetSamplingTargetsError>>;
        async fn get_service_graph(&self, builder: GetServiceGraphInputBuilder) -> Result<GetServiceGraphOutput, SdkError<GetServiceGraphError>>;
        async fn get_time_series_service_statistics(&self, builder: GetTimeSeriesServiceStatisticsInputBuilder) -> Result<GetTimeSeriesServiceStatisticsOutput, SdkError<GetTimeSeriesServiceStatisticsError>>;
        async fn get_trace_graph(&self, builder: GetTraceGraphInputBuilder) -> Result<GetTraceGraphOutput, SdkError<GetTraceGraphError>>;
        async fn get_trace_summaries(&self, builder: GetTraceSummariesInputBuilder) -> Result<GetTraceSummariesOutput, SdkError<GetTraceSummariesError>>;
        async fn list_resource_policies(&self, builder: ListResourcePoliciesInputBuilder) -> Result<ListResourcePoliciesOutput, SdkError<ListResourcePoliciesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn put_encryption_config(&self, builder: PutEncryptionConfigInputBuilder) -> Result<PutEncryptionConfigOutput, SdkError<PutEncryptionConfigError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn put_telemetry_records(&self, builder: PutTelemetryRecordsInputBuilder) -> Result<PutTelemetryRecordsOutput, SdkError<PutTelemetryRecordsError>>;
        async fn put_trace_segments(&self, builder: PutTraceSegmentsInputBuilder) -> Result<PutTraceSegmentsOutput, SdkError<PutTraceSegmentsError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_group(&self, builder: UpdateGroupInputBuilder) -> Result<UpdateGroupOutput, SdkError<UpdateGroupError>>;
        async fn update_sampling_rule(&self, builder: UpdateSamplingRuleInputBuilder) -> Result<UpdateSamplingRuleOutput, SdkError<UpdateSamplingRuleError>>;
    }
}
