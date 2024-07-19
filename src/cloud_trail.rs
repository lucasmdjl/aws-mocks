/*
 * aws_mock - A mocking library for AWS.
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
use aws_sdk_cloudtrail::operation::add_tags::{builders::*, *};
use aws_sdk_cloudtrail::operation::cancel_query::{builders::*, *};
use aws_sdk_cloudtrail::operation::create_channel::{builders::*, *};
use aws_sdk_cloudtrail::operation::create_event_data_store::{builders::*, *};
use aws_sdk_cloudtrail::operation::create_trail::{builders::*, *};
use aws_sdk_cloudtrail::operation::delete_channel::{builders::*, *};
use aws_sdk_cloudtrail::operation::delete_event_data_store::{builders::*, *};
use aws_sdk_cloudtrail::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_cloudtrail::operation::delete_trail::{builders::*, *};
use aws_sdk_cloudtrail::operation::deregister_organization_delegated_admin::{builders::*, *};
use aws_sdk_cloudtrail::operation::describe_query::{builders::*, *};
use aws_sdk_cloudtrail::operation::describe_trails::{builders::*, *};
use aws_sdk_cloudtrail::operation::disable_federation::{builders::*, *};
use aws_sdk_cloudtrail::operation::enable_federation::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_channel::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_event_data_store::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_event_selectors::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_import::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_insight_selectors::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_query_results::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_resource_policy::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_trail::{builders::*, *};
use aws_sdk_cloudtrail::operation::get_trail_status::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_channels::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_event_data_stores::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_import_failures::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_imports::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_insights_metric_data::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_public_keys::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_queries::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_tags::{builders::*, *};
use aws_sdk_cloudtrail::operation::list_trails::{builders::*, *};
use aws_sdk_cloudtrail::operation::lookup_events::{builders::*, *};
use aws_sdk_cloudtrail::operation::put_event_selectors::{builders::*, *};
use aws_sdk_cloudtrail::operation::put_insight_selectors::{builders::*, *};
use aws_sdk_cloudtrail::operation::put_resource_policy::{builders::*, *};
use aws_sdk_cloudtrail::operation::register_organization_delegated_admin::{builders::*, *};
use aws_sdk_cloudtrail::operation::remove_tags::{builders::*, *};
use aws_sdk_cloudtrail::operation::restore_event_data_store::{builders::*, *};
use aws_sdk_cloudtrail::operation::start_event_data_store_ingestion::{builders::*, *};
use aws_sdk_cloudtrail::operation::start_import::{builders::*, *};
use aws_sdk_cloudtrail::operation::start_logging::{builders::*, *};
use aws_sdk_cloudtrail::operation::start_query::{builders::*, *};
use aws_sdk_cloudtrail::operation::stop_event_data_store_ingestion::{builders::*, *};
use aws_sdk_cloudtrail::operation::stop_import::{builders::*, *};
use aws_sdk_cloudtrail::operation::stop_logging::{builders::*, *};
use aws_sdk_cloudtrail::operation::update_channel::{builders::*, *};
use aws_sdk_cloudtrail::operation::update_event_data_store::{builders::*, *};
use aws_sdk_cloudtrail::operation::update_trail::{builders::*, *};
use aws_sdk_cloudtrail::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_cloudtrail::Client;

pub use aws_sdk_cloudtrail::*;

pub struct CloudTrailClientImpl(Client);
impl CloudTrailClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CloudTrailClient {
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>>;
    fn cancel_query(&self, builder: CancelQueryInputBuilder) -> impl Future<Output = Result<CancelQueryOutput, SdkError<CancelQueryError>>>;
    fn create_channel(&self, builder: CreateChannelInputBuilder) -> impl Future<Output = Result<CreateChannelOutput, SdkError<CreateChannelError>>>;
    fn create_event_data_store(&self, builder: CreateEventDataStoreInputBuilder) -> impl Future<Output = Result<CreateEventDataStoreOutput, SdkError<CreateEventDataStoreError>>>;
    fn create_trail(&self, builder: CreateTrailInputBuilder) -> impl Future<Output = Result<CreateTrailOutput, SdkError<CreateTrailError>>>;
    fn delete_channel(&self, builder: DeleteChannelInputBuilder) -> impl Future<Output = Result<DeleteChannelOutput, SdkError<DeleteChannelError>>>;
    fn delete_event_data_store(&self, builder: DeleteEventDataStoreInputBuilder) -> impl Future<Output = Result<DeleteEventDataStoreOutput, SdkError<DeleteEventDataStoreError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_trail(&self, builder: DeleteTrailInputBuilder) -> impl Future<Output = Result<DeleteTrailOutput, SdkError<DeleteTrailError>>>;
    fn deregister_organization_delegated_admin(&self, builder: DeregisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<DeregisterOrganizationDelegatedAdminOutput, SdkError<DeregisterOrganizationDelegatedAdminError>>>;
    fn describe_query(&self, builder: DescribeQueryInputBuilder) -> impl Future<Output = Result<DescribeQueryOutput, SdkError<DescribeQueryError>>>;
    fn describe_trails(&self, builder: DescribeTrailsInputBuilder) -> impl Future<Output = Result<DescribeTrailsOutput, SdkError<DescribeTrailsError>>>;
    fn disable_federation(&self, builder: DisableFederationInputBuilder) -> impl Future<Output = Result<DisableFederationOutput, SdkError<DisableFederationError>>>;
    fn enable_federation(&self, builder: EnableFederationInputBuilder) -> impl Future<Output = Result<EnableFederationOutput, SdkError<EnableFederationError>>>;
    fn get_channel(&self, builder: GetChannelInputBuilder) -> impl Future<Output = Result<GetChannelOutput, SdkError<GetChannelError>>>;
    fn get_event_data_store(&self, builder: GetEventDataStoreInputBuilder) -> impl Future<Output = Result<GetEventDataStoreOutput, SdkError<GetEventDataStoreError>>>;
    fn get_event_selectors(&self, builder: GetEventSelectorsInputBuilder) -> impl Future<Output = Result<GetEventSelectorsOutput, SdkError<GetEventSelectorsError>>>;
    fn get_import(&self, builder: GetImportInputBuilder) -> impl Future<Output = Result<GetImportOutput, SdkError<GetImportError>>>;
    fn get_insight_selectors(&self, builder: GetInsightSelectorsInputBuilder) -> impl Future<Output = Result<GetInsightSelectorsOutput, SdkError<GetInsightSelectorsError>>>;
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn get_trail(&self, builder: GetTrailInputBuilder) -> impl Future<Output = Result<GetTrailOutput, SdkError<GetTrailError>>>;
    fn get_trail_status(&self, builder: GetTrailStatusInputBuilder) -> impl Future<Output = Result<GetTrailStatusOutput, SdkError<GetTrailStatusError>>>;
    fn list_channels(&self, builder: ListChannelsInputBuilder) -> impl Future<Output = Result<ListChannelsOutput, SdkError<ListChannelsError>>>;
    fn list_event_data_stores(&self, builder: ListEventDataStoresInputBuilder) -> impl Future<Output = Result<ListEventDataStoresOutput, SdkError<ListEventDataStoresError>>>;
    fn list_import_failures(&self, builder: ListImportFailuresInputBuilder) -> impl Future<Output = Result<ListImportFailuresOutput, SdkError<ListImportFailuresError>>>;
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>>;
    fn list_insights_metric_data(&self, builder: ListInsightsMetricDataInputBuilder) -> impl Future<Output = Result<ListInsightsMetricDataOutput, SdkError<ListInsightsMetricDataError>>>;
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>>;
    fn list_queries(&self, builder: ListQueriesInputBuilder) -> impl Future<Output = Result<ListQueriesOutput, SdkError<ListQueriesError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn list_trails(&self, builder: ListTrailsInputBuilder) -> impl Future<Output = Result<ListTrailsOutput, SdkError<ListTrailsError>>>;
    fn lookup_events(&self, builder: LookupEventsInputBuilder) -> impl Future<Output = Result<LookupEventsOutput, SdkError<LookupEventsError>>>;
    fn put_event_selectors(&self, builder: PutEventSelectorsInputBuilder) -> impl Future<Output = Result<PutEventSelectorsOutput, SdkError<PutEventSelectorsError>>>;
    fn put_insight_selectors(&self, builder: PutInsightSelectorsInputBuilder) -> impl Future<Output = Result<PutInsightSelectorsOutput, SdkError<PutInsightSelectorsError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn register_organization_delegated_admin(&self, builder: RegisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<RegisterOrganizationDelegatedAdminOutput, SdkError<RegisterOrganizationDelegatedAdminError>>>;
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>>;
    fn restore_event_data_store(&self, builder: RestoreEventDataStoreInputBuilder) -> impl Future<Output = Result<RestoreEventDataStoreOutput, SdkError<RestoreEventDataStoreError>>>;
    fn start_event_data_store_ingestion(&self, builder: StartEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StartEventDataStoreIngestionOutput, SdkError<StartEventDataStoreIngestionError>>>;
    fn start_import(&self, builder: StartImportInputBuilder) -> impl Future<Output = Result<StartImportOutput, SdkError<StartImportError>>>;
    fn start_logging(&self, builder: StartLoggingInputBuilder) -> impl Future<Output = Result<StartLoggingOutput, SdkError<StartLoggingError>>>;
    fn start_query(&self, builder: StartQueryInputBuilder) -> impl Future<Output = Result<StartQueryOutput, SdkError<StartQueryError>>>;
    fn stop_event_data_store_ingestion(&self, builder: StopEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StopEventDataStoreIngestionOutput, SdkError<StopEventDataStoreIngestionError>>>;
    fn stop_import(&self, builder: StopImportInputBuilder) -> impl Future<Output = Result<StopImportOutput, SdkError<StopImportError>>>;
    fn stop_logging(&self, builder: StopLoggingInputBuilder) -> impl Future<Output = Result<StopLoggingOutput, SdkError<StopLoggingError>>>;
    fn update_channel(&self, builder: UpdateChannelInputBuilder) -> impl Future<Output = Result<UpdateChannelOutput, SdkError<UpdateChannelError>>>;
    fn update_event_data_store(&self, builder: UpdateEventDataStoreInputBuilder) -> impl Future<Output = Result<UpdateEventDataStoreOutput, SdkError<UpdateEventDataStoreError>>>;
    fn update_trail(&self, builder: UpdateTrailInputBuilder) -> impl Future<Output = Result<UpdateTrailOutput, SdkError<UpdateTrailError>>>;
}
impl CloudTrailClient for CloudTrailClientImpl {
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_query(&self, builder: CancelQueryInputBuilder) -> impl Future<Output = Result<CancelQueryOutput, SdkError<CancelQueryError>>> {
        builder.send_with(&self.0)
    }
    fn create_channel(&self, builder: CreateChannelInputBuilder) -> impl Future<Output = Result<CreateChannelOutput, SdkError<CreateChannelError>>> {
        builder.send_with(&self.0)
    }
    fn create_event_data_store(&self, builder: CreateEventDataStoreInputBuilder) -> impl Future<Output = Result<CreateEventDataStoreOutput, SdkError<CreateEventDataStoreError>>> {
        builder.send_with(&self.0)
    }
    fn create_trail(&self, builder: CreateTrailInputBuilder) -> impl Future<Output = Result<CreateTrailOutput, SdkError<CreateTrailError>>> {
        builder.send_with(&self.0)
    }
    fn delete_channel(&self, builder: DeleteChannelInputBuilder) -> impl Future<Output = Result<DeleteChannelOutput, SdkError<DeleteChannelError>>> {
        builder.send_with(&self.0)
    }
    fn delete_event_data_store(&self, builder: DeleteEventDataStoreInputBuilder) -> impl Future<Output = Result<DeleteEventDataStoreOutput, SdkError<DeleteEventDataStoreError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_trail(&self, builder: DeleteTrailInputBuilder) -> impl Future<Output = Result<DeleteTrailOutput, SdkError<DeleteTrailError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_organization_delegated_admin(&self, builder: DeregisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<DeregisterOrganizationDelegatedAdminOutput, SdkError<DeregisterOrganizationDelegatedAdminError>>> {
        builder.send_with(&self.0)
    }
    fn describe_query(&self, builder: DescribeQueryInputBuilder) -> impl Future<Output = Result<DescribeQueryOutput, SdkError<DescribeQueryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_trails(&self, builder: DescribeTrailsInputBuilder) -> impl Future<Output = Result<DescribeTrailsOutput, SdkError<DescribeTrailsError>>> {
        builder.send_with(&self.0)
    }
    fn disable_federation(&self, builder: DisableFederationInputBuilder) -> impl Future<Output = Result<DisableFederationOutput, SdkError<DisableFederationError>>> {
        builder.send_with(&self.0)
    }
    fn enable_federation(&self, builder: EnableFederationInputBuilder) -> impl Future<Output = Result<EnableFederationOutput, SdkError<EnableFederationError>>> {
        builder.send_with(&self.0)
    }
    fn get_channel(&self, builder: GetChannelInputBuilder) -> impl Future<Output = Result<GetChannelOutput, SdkError<GetChannelError>>> {
        builder.send_with(&self.0)
    }
    fn get_event_data_store(&self, builder: GetEventDataStoreInputBuilder) -> impl Future<Output = Result<GetEventDataStoreOutput, SdkError<GetEventDataStoreError>>> {
        builder.send_with(&self.0)
    }
    fn get_event_selectors(&self, builder: GetEventSelectorsInputBuilder) -> impl Future<Output = Result<GetEventSelectorsOutput, SdkError<GetEventSelectorsError>>> {
        builder.send_with(&self.0)
    }
    fn get_import(&self, builder: GetImportInputBuilder) -> impl Future<Output = Result<GetImportOutput, SdkError<GetImportError>>> {
        builder.send_with(&self.0)
    }
    fn get_insight_selectors(&self, builder: GetInsightSelectorsInputBuilder) -> impl Future<Output = Result<GetInsightSelectorsOutput, SdkError<GetInsightSelectorsError>>> {
        builder.send_with(&self.0)
    }
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_trail(&self, builder: GetTrailInputBuilder) -> impl Future<Output = Result<GetTrailOutput, SdkError<GetTrailError>>> {
        builder.send_with(&self.0)
    }
    fn get_trail_status(&self, builder: GetTrailStatusInputBuilder) -> impl Future<Output = Result<GetTrailStatusOutput, SdkError<GetTrailStatusError>>> {
        builder.send_with(&self.0)
    }
    fn list_channels(&self, builder: ListChannelsInputBuilder) -> impl Future<Output = Result<ListChannelsOutput, SdkError<ListChannelsError>>> {
        builder.send_with(&self.0)
    }
    fn list_event_data_stores(&self, builder: ListEventDataStoresInputBuilder) -> impl Future<Output = Result<ListEventDataStoresOutput, SdkError<ListEventDataStoresError>>> {
        builder.send_with(&self.0)
    }
    fn list_import_failures(&self, builder: ListImportFailuresInputBuilder) -> impl Future<Output = Result<ListImportFailuresOutput, SdkError<ListImportFailuresError>>> {
        builder.send_with(&self.0)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_insights_metric_data(&self, builder: ListInsightsMetricDataInputBuilder) -> impl Future<Output = Result<ListInsightsMetricDataOutput, SdkError<ListInsightsMetricDataError>>> {
        builder.send_with(&self.0)
    }
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_queries(&self, builder: ListQueriesInputBuilder) -> impl Future<Output = Result<ListQueriesOutput, SdkError<ListQueriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_trails(&self, builder: ListTrailsInputBuilder) -> impl Future<Output = Result<ListTrailsOutput, SdkError<ListTrailsError>>> {
        builder.send_with(&self.0)
    }
    fn lookup_events(&self, builder: LookupEventsInputBuilder) -> impl Future<Output = Result<LookupEventsOutput, SdkError<LookupEventsError>>> {
        builder.send_with(&self.0)
    }
    fn put_event_selectors(&self, builder: PutEventSelectorsInputBuilder) -> impl Future<Output = Result<PutEventSelectorsOutput, SdkError<PutEventSelectorsError>>> {
        builder.send_with(&self.0)
    }
    fn put_insight_selectors(&self, builder: PutInsightSelectorsInputBuilder) -> impl Future<Output = Result<PutInsightSelectorsOutput, SdkError<PutInsightSelectorsError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn register_organization_delegated_admin(&self, builder: RegisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<RegisterOrganizationDelegatedAdminOutput, SdkError<RegisterOrganizationDelegatedAdminError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        builder.send_with(&self.0)
    }
    fn restore_event_data_store(&self, builder: RestoreEventDataStoreInputBuilder) -> impl Future<Output = Result<RestoreEventDataStoreOutput, SdkError<RestoreEventDataStoreError>>> {
        builder.send_with(&self.0)
    }
    fn start_event_data_store_ingestion(&self, builder: StartEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StartEventDataStoreIngestionOutput, SdkError<StartEventDataStoreIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn start_import(&self, builder: StartImportInputBuilder) -> impl Future<Output = Result<StartImportOutput, SdkError<StartImportError>>> {
        builder.send_with(&self.0)
    }
    fn start_logging(&self, builder: StartLoggingInputBuilder) -> impl Future<Output = Result<StartLoggingOutput, SdkError<StartLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn start_query(&self, builder: StartQueryInputBuilder) -> impl Future<Output = Result<StartQueryOutput, SdkError<StartQueryError>>> {
        builder.send_with(&self.0)
    }
    fn stop_event_data_store_ingestion(&self, builder: StopEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StopEventDataStoreIngestionOutput, SdkError<StopEventDataStoreIngestionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_import(&self, builder: StopImportInputBuilder) -> impl Future<Output = Result<StopImportOutput, SdkError<StopImportError>>> {
        builder.send_with(&self.0)
    }
    fn stop_logging(&self, builder: StopLoggingInputBuilder) -> impl Future<Output = Result<StopLoggingOutput, SdkError<StopLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn update_channel(&self, builder: UpdateChannelInputBuilder) -> impl Future<Output = Result<UpdateChannelOutput, SdkError<UpdateChannelError>>> {
        builder.send_with(&self.0)
    }
    fn update_event_data_store(&self, builder: UpdateEventDataStoreInputBuilder) -> impl Future<Output = Result<UpdateEventDataStoreOutput, SdkError<UpdateEventDataStoreError>>> {
        builder.send_with(&self.0)
    }
    fn update_trail(&self, builder: UpdateTrailInputBuilder) -> impl Future<Output = Result<UpdateTrailOutput, SdkError<UpdateTrailError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: CloudTrailClient> CloudTrailClient for &T {
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        (*self).add_tags(builder)
    }
    fn cancel_query(&self, builder: CancelQueryInputBuilder) -> impl Future<Output = Result<CancelQueryOutput, SdkError<CancelQueryError>>> {
        (*self).cancel_query(builder)
    }
    fn create_channel(&self, builder: CreateChannelInputBuilder) -> impl Future<Output = Result<CreateChannelOutput, SdkError<CreateChannelError>>> {
        (*self).create_channel(builder)
    }
    fn create_event_data_store(&self, builder: CreateEventDataStoreInputBuilder) -> impl Future<Output = Result<CreateEventDataStoreOutput, SdkError<CreateEventDataStoreError>>> {
        (*self).create_event_data_store(builder)
    }
    fn create_trail(&self, builder: CreateTrailInputBuilder) -> impl Future<Output = Result<CreateTrailOutput, SdkError<CreateTrailError>>> {
        (*self).create_trail(builder)
    }
    fn delete_channel(&self, builder: DeleteChannelInputBuilder) -> impl Future<Output = Result<DeleteChannelOutput, SdkError<DeleteChannelError>>> {
        (*self).delete_channel(builder)
    }
    fn delete_event_data_store(&self, builder: DeleteEventDataStoreInputBuilder) -> impl Future<Output = Result<DeleteEventDataStoreOutput, SdkError<DeleteEventDataStoreError>>> {
        (*self).delete_event_data_store(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        (*self).delete_resource_policy(builder)
    }
    fn delete_trail(&self, builder: DeleteTrailInputBuilder) -> impl Future<Output = Result<DeleteTrailOutput, SdkError<DeleteTrailError>>> {
        (*self).delete_trail(builder)
    }
    fn deregister_organization_delegated_admin(&self, builder: DeregisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<DeregisterOrganizationDelegatedAdminOutput, SdkError<DeregisterOrganizationDelegatedAdminError>>> {
        (*self).deregister_organization_delegated_admin(builder)
    }
    fn describe_query(&self, builder: DescribeQueryInputBuilder) -> impl Future<Output = Result<DescribeQueryOutput, SdkError<DescribeQueryError>>> {
        (*self).describe_query(builder)
    }
    fn describe_trails(&self, builder: DescribeTrailsInputBuilder) -> impl Future<Output = Result<DescribeTrailsOutput, SdkError<DescribeTrailsError>>> {
        (*self).describe_trails(builder)
    }
    fn disable_federation(&self, builder: DisableFederationInputBuilder) -> impl Future<Output = Result<DisableFederationOutput, SdkError<DisableFederationError>>> {
        (*self).disable_federation(builder)
    }
    fn enable_federation(&self, builder: EnableFederationInputBuilder) -> impl Future<Output = Result<EnableFederationOutput, SdkError<EnableFederationError>>> {
        (*self).enable_federation(builder)
    }
    fn get_channel(&self, builder: GetChannelInputBuilder) -> impl Future<Output = Result<GetChannelOutput, SdkError<GetChannelError>>> {
        (*self).get_channel(builder)
    }
    fn get_event_data_store(&self, builder: GetEventDataStoreInputBuilder) -> impl Future<Output = Result<GetEventDataStoreOutput, SdkError<GetEventDataStoreError>>> {
        (*self).get_event_data_store(builder)
    }
    fn get_event_selectors(&self, builder: GetEventSelectorsInputBuilder) -> impl Future<Output = Result<GetEventSelectorsOutput, SdkError<GetEventSelectorsError>>> {
        (*self).get_event_selectors(builder)
    }
    fn get_import(&self, builder: GetImportInputBuilder) -> impl Future<Output = Result<GetImportOutput, SdkError<GetImportError>>> {
        (*self).get_import(builder)
    }
    fn get_insight_selectors(&self, builder: GetInsightSelectorsInputBuilder) -> impl Future<Output = Result<GetInsightSelectorsOutput, SdkError<GetInsightSelectorsError>>> {
        (*self).get_insight_selectors(builder)
    }
    fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> impl Future<Output = Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> {
        (*self).get_query_results(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        (*self).get_resource_policy(builder)
    }
    fn get_trail(&self, builder: GetTrailInputBuilder) -> impl Future<Output = Result<GetTrailOutput, SdkError<GetTrailError>>> {
        (*self).get_trail(builder)
    }
    fn get_trail_status(&self, builder: GetTrailStatusInputBuilder) -> impl Future<Output = Result<GetTrailStatusOutput, SdkError<GetTrailStatusError>>> {
        (*self).get_trail_status(builder)
    }
    fn list_channels(&self, builder: ListChannelsInputBuilder) -> impl Future<Output = Result<ListChannelsOutput, SdkError<ListChannelsError>>> {
        (*self).list_channels(builder)
    }
    fn list_event_data_stores(&self, builder: ListEventDataStoresInputBuilder) -> impl Future<Output = Result<ListEventDataStoresOutput, SdkError<ListEventDataStoresError>>> {
        (*self).list_event_data_stores(builder)
    }
    fn list_import_failures(&self, builder: ListImportFailuresInputBuilder) -> impl Future<Output = Result<ListImportFailuresOutput, SdkError<ListImportFailuresError>>> {
        (*self).list_import_failures(builder)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        (*self).list_imports(builder)
    }
    fn list_insights_metric_data(&self, builder: ListInsightsMetricDataInputBuilder) -> impl Future<Output = Result<ListInsightsMetricDataOutput, SdkError<ListInsightsMetricDataError>>> {
        (*self).list_insights_metric_data(builder)
    }
    fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> impl Future<Output = Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>> {
        (*self).list_public_keys(builder)
    }
    fn list_queries(&self, builder: ListQueriesInputBuilder) -> impl Future<Output = Result<ListQueriesOutput, SdkError<ListQueriesError>>> {
        (*self).list_queries(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        (*self).list_tags(builder)
    }
    fn list_trails(&self, builder: ListTrailsInputBuilder) -> impl Future<Output = Result<ListTrailsOutput, SdkError<ListTrailsError>>> {
        (*self).list_trails(builder)
    }
    fn lookup_events(&self, builder: LookupEventsInputBuilder) -> impl Future<Output = Result<LookupEventsOutput, SdkError<LookupEventsError>>> {
        (*self).lookup_events(builder)
    }
    fn put_event_selectors(&self, builder: PutEventSelectorsInputBuilder) -> impl Future<Output = Result<PutEventSelectorsOutput, SdkError<PutEventSelectorsError>>> {
        (*self).put_event_selectors(builder)
    }
    fn put_insight_selectors(&self, builder: PutInsightSelectorsInputBuilder) -> impl Future<Output = Result<PutInsightSelectorsOutput, SdkError<PutInsightSelectorsError>>> {
        (*self).put_insight_selectors(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        (*self).put_resource_policy(builder)
    }
    fn register_organization_delegated_admin(&self, builder: RegisterOrganizationDelegatedAdminInputBuilder) -> impl Future<Output = Result<RegisterOrganizationDelegatedAdminOutput, SdkError<RegisterOrganizationDelegatedAdminError>>> {
        (*self).register_organization_delegated_admin(builder)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        (*self).remove_tags(builder)
    }
    fn restore_event_data_store(&self, builder: RestoreEventDataStoreInputBuilder) -> impl Future<Output = Result<RestoreEventDataStoreOutput, SdkError<RestoreEventDataStoreError>>> {
        (*self).restore_event_data_store(builder)
    }
    fn start_event_data_store_ingestion(&self, builder: StartEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StartEventDataStoreIngestionOutput, SdkError<StartEventDataStoreIngestionError>>> {
        (*self).start_event_data_store_ingestion(builder)
    }
    fn start_import(&self, builder: StartImportInputBuilder) -> impl Future<Output = Result<StartImportOutput, SdkError<StartImportError>>> {
        (*self).start_import(builder)
    }
    fn start_logging(&self, builder: StartLoggingInputBuilder) -> impl Future<Output = Result<StartLoggingOutput, SdkError<StartLoggingError>>> {
        (*self).start_logging(builder)
    }
    fn start_query(&self, builder: StartQueryInputBuilder) -> impl Future<Output = Result<StartQueryOutput, SdkError<StartQueryError>>> {
        (*self).start_query(builder)
    }
    fn stop_event_data_store_ingestion(&self, builder: StopEventDataStoreIngestionInputBuilder) -> impl Future<Output = Result<StopEventDataStoreIngestionOutput, SdkError<StopEventDataStoreIngestionError>>> {
        (*self).stop_event_data_store_ingestion(builder)
    }
    fn stop_import(&self, builder: StopImportInputBuilder) -> impl Future<Output = Result<StopImportOutput, SdkError<StopImportError>>> {
        (*self).stop_import(builder)
    }
    fn stop_logging(&self, builder: StopLoggingInputBuilder) -> impl Future<Output = Result<StopLoggingOutput, SdkError<StopLoggingError>>> {
        (*self).stop_logging(builder)
    }
    fn update_channel(&self, builder: UpdateChannelInputBuilder) -> impl Future<Output = Result<UpdateChannelOutput, SdkError<UpdateChannelError>>> {
        (*self).update_channel(builder)
    }
    fn update_event_data_store(&self, builder: UpdateEventDataStoreInputBuilder) -> impl Future<Output = Result<UpdateEventDataStoreOutput, SdkError<UpdateEventDataStoreError>>> {
        (*self).update_event_data_store(builder)
    }
    fn update_trail(&self, builder: UpdateTrailInputBuilder) -> impl Future<Output = Result<UpdateTrailOutput, SdkError<UpdateTrailError>>> {
        (*self).update_trail(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCloudTrailClient {}
    impl CloudTrailClient for edCloudTrailClient {
        async fn add_tags(&self, builder: AddTagsInputBuilder) -> Result<AddTagsOutput, SdkError<AddTagsError>>;
        async fn cancel_query(&self, builder: CancelQueryInputBuilder) -> Result<CancelQueryOutput, SdkError<CancelQueryError>>;
        async fn create_channel(&self, builder: CreateChannelInputBuilder) -> Result<CreateChannelOutput, SdkError<CreateChannelError>>;
        async fn create_event_data_store(&self, builder: CreateEventDataStoreInputBuilder) -> Result<CreateEventDataStoreOutput, SdkError<CreateEventDataStoreError>>;
        async fn create_trail(&self, builder: CreateTrailInputBuilder) -> Result<CreateTrailOutput, SdkError<CreateTrailError>>;
        async fn delete_channel(&self, builder: DeleteChannelInputBuilder) -> Result<DeleteChannelOutput, SdkError<DeleteChannelError>>;
        async fn delete_event_data_store(&self, builder: DeleteEventDataStoreInputBuilder) -> Result<DeleteEventDataStoreOutput, SdkError<DeleteEventDataStoreError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_trail(&self, builder: DeleteTrailInputBuilder) -> Result<DeleteTrailOutput, SdkError<DeleteTrailError>>;
        async fn deregister_organization_delegated_admin(&self, builder: DeregisterOrganizationDelegatedAdminInputBuilder) -> Result<DeregisterOrganizationDelegatedAdminOutput, SdkError<DeregisterOrganizationDelegatedAdminError>>;
        async fn describe_query(&self, builder: DescribeQueryInputBuilder) -> Result<DescribeQueryOutput, SdkError<DescribeQueryError>>;
        async fn describe_trails(&self, builder: DescribeTrailsInputBuilder) -> Result<DescribeTrailsOutput, SdkError<DescribeTrailsError>>;
        async fn disable_federation(&self, builder: DisableFederationInputBuilder) -> Result<DisableFederationOutput, SdkError<DisableFederationError>>;
        async fn enable_federation(&self, builder: EnableFederationInputBuilder) -> Result<EnableFederationOutput, SdkError<EnableFederationError>>;
        async fn get_channel(&self, builder: GetChannelInputBuilder) -> Result<GetChannelOutput, SdkError<GetChannelError>>;
        async fn get_event_data_store(&self, builder: GetEventDataStoreInputBuilder) -> Result<GetEventDataStoreOutput, SdkError<GetEventDataStoreError>>;
        async fn get_event_selectors(&self, builder: GetEventSelectorsInputBuilder) -> Result<GetEventSelectorsOutput, SdkError<GetEventSelectorsError>>;
        async fn get_import(&self, builder: GetImportInputBuilder) -> Result<GetImportOutput, SdkError<GetImportError>>;
        async fn get_insight_selectors(&self, builder: GetInsightSelectorsInputBuilder) -> Result<GetInsightSelectorsOutput, SdkError<GetInsightSelectorsError>>;
        async fn get_query_results(&self, builder: GetQueryResultsInputBuilder) -> Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn get_trail(&self, builder: GetTrailInputBuilder) -> Result<GetTrailOutput, SdkError<GetTrailError>>;
        async fn get_trail_status(&self, builder: GetTrailStatusInputBuilder) -> Result<GetTrailStatusOutput, SdkError<GetTrailStatusError>>;
        async fn list_channels(&self, builder: ListChannelsInputBuilder) -> Result<ListChannelsOutput, SdkError<ListChannelsError>>;
        async fn list_event_data_stores(&self, builder: ListEventDataStoresInputBuilder) -> Result<ListEventDataStoresOutput, SdkError<ListEventDataStoresError>>;
        async fn list_import_failures(&self, builder: ListImportFailuresInputBuilder) -> Result<ListImportFailuresOutput, SdkError<ListImportFailuresError>>;
        async fn list_imports(&self, builder: ListImportsInputBuilder) -> Result<ListImportsOutput, SdkError<ListImportsError>>;
        async fn list_insights_metric_data(&self, builder: ListInsightsMetricDataInputBuilder) -> Result<ListInsightsMetricDataOutput, SdkError<ListInsightsMetricDataError>>;
        async fn list_public_keys(&self, builder: ListPublicKeysInputBuilder) -> Result<ListPublicKeysOutput, SdkError<ListPublicKeysError>>;
        async fn list_queries(&self, builder: ListQueriesInputBuilder) -> Result<ListQueriesOutput, SdkError<ListQueriesError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn list_trails(&self, builder: ListTrailsInputBuilder) -> Result<ListTrailsOutput, SdkError<ListTrailsError>>;
        async fn lookup_events(&self, builder: LookupEventsInputBuilder) -> Result<LookupEventsOutput, SdkError<LookupEventsError>>;
        async fn put_event_selectors(&self, builder: PutEventSelectorsInputBuilder) -> Result<PutEventSelectorsOutput, SdkError<PutEventSelectorsError>>;
        async fn put_insight_selectors(&self, builder: PutInsightSelectorsInputBuilder) -> Result<PutInsightSelectorsOutput, SdkError<PutInsightSelectorsError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn register_organization_delegated_admin(&self, builder: RegisterOrganizationDelegatedAdminInputBuilder) -> Result<RegisterOrganizationDelegatedAdminOutput, SdkError<RegisterOrganizationDelegatedAdminError>>;
        async fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> Result<RemoveTagsOutput, SdkError<RemoveTagsError>>;
        async fn restore_event_data_store(&self, builder: RestoreEventDataStoreInputBuilder) -> Result<RestoreEventDataStoreOutput, SdkError<RestoreEventDataStoreError>>;
        async fn start_event_data_store_ingestion(&self, builder: StartEventDataStoreIngestionInputBuilder) -> Result<StartEventDataStoreIngestionOutput, SdkError<StartEventDataStoreIngestionError>>;
        async fn start_import(&self, builder: StartImportInputBuilder) -> Result<StartImportOutput, SdkError<StartImportError>>;
        async fn start_logging(&self, builder: StartLoggingInputBuilder) -> Result<StartLoggingOutput, SdkError<StartLoggingError>>;
        async fn start_query(&self, builder: StartQueryInputBuilder) -> Result<StartQueryOutput, SdkError<StartQueryError>>;
        async fn stop_event_data_store_ingestion(&self, builder: StopEventDataStoreIngestionInputBuilder) -> Result<StopEventDataStoreIngestionOutput, SdkError<StopEventDataStoreIngestionError>>;
        async fn stop_import(&self, builder: StopImportInputBuilder) -> Result<StopImportOutput, SdkError<StopImportError>>;
        async fn stop_logging(&self, builder: StopLoggingInputBuilder) -> Result<StopLoggingOutput, SdkError<StopLoggingError>>;
        async fn update_channel(&self, builder: UpdateChannelInputBuilder) -> Result<UpdateChannelOutput, SdkError<UpdateChannelError>>;
        async fn update_event_data_store(&self, builder: UpdateEventDataStoreInputBuilder) -> Result<UpdateEventDataStoreOutput, SdkError<UpdateEventDataStoreError>>;
        async fn update_trail(&self, builder: UpdateTrailInputBuilder) -> Result<UpdateTrailOutput, SdkError<UpdateTrailError>>;
    }
}
