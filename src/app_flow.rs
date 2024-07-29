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
use aws_sdk_appflow::operation::cancel_flow_executions::{builders::*, *};
use aws_sdk_appflow::operation::create_connector_profile::{builders::*, *};
use aws_sdk_appflow::operation::create_flow::{builders::*, *};
use aws_sdk_appflow::operation::delete_connector_profile::{builders::*, *};
use aws_sdk_appflow::operation::delete_flow::{builders::*, *};
use aws_sdk_appflow::operation::describe_connector::{builders::*, *};
use aws_sdk_appflow::operation::describe_connector_entity::{builders::*, *};
use aws_sdk_appflow::operation::describe_connector_profiles::{builders::*, *};
use aws_sdk_appflow::operation::describe_connectors::{builders::*, *};
use aws_sdk_appflow::operation::describe_flow::{builders::*, *};
use aws_sdk_appflow::operation::describe_flow_execution_records::{builders::*, *};
use aws_sdk_appflow::operation::list_connector_entities::{builders::*, *};
use aws_sdk_appflow::operation::list_connectors::{builders::*, *};
use aws_sdk_appflow::operation::list_flows::{builders::*, *};
use aws_sdk_appflow::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appflow::operation::register_connector::{builders::*, *};
use aws_sdk_appflow::operation::reset_connector_metadata_cache::{builders::*, *};
use aws_sdk_appflow::operation::start_flow::{builders::*, *};
use aws_sdk_appflow::operation::stop_flow::{builders::*, *};
use aws_sdk_appflow::operation::tag_resource::{builders::*, *};
use aws_sdk_appflow::operation::unregister_connector::{builders::*, *};
use aws_sdk_appflow::operation::untag_resource::{builders::*, *};
use aws_sdk_appflow::operation::update_connector_profile::{builders::*, *};
use aws_sdk_appflow::operation::update_connector_registration::{builders::*, *};
use aws_sdk_appflow::operation::update_flow::{builders::*, *};
use aws_sdk_appflow::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appflow::Client;
use std::ops::Deref;

pub use aws_sdk_appflow::*;

pub struct AppFlowClientImpl(Client);
impl AppFlowClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppFlowClient {
    fn cancel_flow_executions(&self, builder: CancelFlowExecutionsInputBuilder) -> impl Future<Output = Result<CancelFlowExecutionsOutput, SdkError<CancelFlowExecutionsError>>>;
    fn create_connector_profile(&self, builder: CreateConnectorProfileInputBuilder) -> impl Future<Output = Result<CreateConnectorProfileOutput, SdkError<CreateConnectorProfileError>>>;
    fn create_flow(&self, builder: CreateFlowInputBuilder) -> impl Future<Output = Result<CreateFlowOutput, SdkError<CreateFlowError>>>;
    fn delete_connector_profile(&self, builder: DeleteConnectorProfileInputBuilder) -> impl Future<Output = Result<DeleteConnectorProfileOutput, SdkError<DeleteConnectorProfileError>>>;
    fn delete_flow(&self, builder: DeleteFlowInputBuilder) -> impl Future<Output = Result<DeleteFlowOutput, SdkError<DeleteFlowError>>>;
    fn describe_connector(&self, builder: DescribeConnectorInputBuilder) -> impl Future<Output = Result<DescribeConnectorOutput, SdkError<DescribeConnectorError>>>;
    fn describe_connector_entity(&self, builder: DescribeConnectorEntityInputBuilder) -> impl Future<Output = Result<DescribeConnectorEntityOutput, SdkError<DescribeConnectorEntityError>>>;
    fn describe_connector_profiles(&self, builder: DescribeConnectorProfilesInputBuilder) -> impl Future<Output = Result<DescribeConnectorProfilesOutput, SdkError<DescribeConnectorProfilesError>>>;
    fn describe_connectors(&self, builder: DescribeConnectorsInputBuilder) -> impl Future<Output = Result<DescribeConnectorsOutput, SdkError<DescribeConnectorsError>>>;
    fn describe_flow(&self, builder: DescribeFlowInputBuilder) -> impl Future<Output = Result<DescribeFlowOutput, SdkError<DescribeFlowError>>>;
    fn describe_flow_execution_records(&self, builder: DescribeFlowExecutionRecordsInputBuilder) -> impl Future<Output = Result<DescribeFlowExecutionRecordsOutput, SdkError<DescribeFlowExecutionRecordsError>>>;
    fn list_connector_entities(&self, builder: ListConnectorEntitiesInputBuilder) -> impl Future<Output = Result<ListConnectorEntitiesOutput, SdkError<ListConnectorEntitiesError>>>;
    fn list_connectors(&self, builder: ListConnectorsInputBuilder) -> impl Future<Output = Result<ListConnectorsOutput, SdkError<ListConnectorsError>>>;
    fn list_flows(&self, builder: ListFlowsInputBuilder) -> impl Future<Output = Result<ListFlowsOutput, SdkError<ListFlowsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn register_connector(&self, builder: RegisterConnectorInputBuilder) -> impl Future<Output = Result<RegisterConnectorOutput, SdkError<RegisterConnectorError>>>;
    fn reset_connector_metadata_cache(&self, builder: ResetConnectorMetadataCacheInputBuilder) -> impl Future<Output = Result<ResetConnectorMetadataCacheOutput, SdkError<ResetConnectorMetadataCacheError>>>;
    fn start_flow(&self, builder: StartFlowInputBuilder) -> impl Future<Output = Result<StartFlowOutput, SdkError<StartFlowError>>>;
    fn stop_flow(&self, builder: StopFlowInputBuilder) -> impl Future<Output = Result<StopFlowOutput, SdkError<StopFlowError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn unregister_connector(&self, builder: UnregisterConnectorInputBuilder) -> impl Future<Output = Result<UnregisterConnectorOutput, SdkError<UnregisterConnectorError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_connector_profile(&self, builder: UpdateConnectorProfileInputBuilder) -> impl Future<Output = Result<UpdateConnectorProfileOutput, SdkError<UpdateConnectorProfileError>>>;
    fn update_connector_registration(&self, builder: UpdateConnectorRegistrationInputBuilder) -> impl Future<Output = Result<UpdateConnectorRegistrationOutput, SdkError<UpdateConnectorRegistrationError>>>;
    fn update_flow(&self, builder: UpdateFlowInputBuilder) -> impl Future<Output = Result<UpdateFlowOutput, SdkError<UpdateFlowError>>>;
}
impl AppFlowClient for AppFlowClientImpl {
    fn cancel_flow_executions(&self, builder: CancelFlowExecutionsInputBuilder) -> impl Future<Output = Result<CancelFlowExecutionsOutput, SdkError<CancelFlowExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn create_connector_profile(&self, builder: CreateConnectorProfileInputBuilder) -> impl Future<Output = Result<CreateConnectorProfileOutput, SdkError<CreateConnectorProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_flow(&self, builder: CreateFlowInputBuilder) -> impl Future<Output = Result<CreateFlowOutput, SdkError<CreateFlowError>>> {
        builder.send_with(&self.0)
    }
    fn delete_connector_profile(&self, builder: DeleteConnectorProfileInputBuilder) -> impl Future<Output = Result<DeleteConnectorProfileOutput, SdkError<DeleteConnectorProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_flow(&self, builder: DeleteFlowInputBuilder) -> impl Future<Output = Result<DeleteFlowOutput, SdkError<DeleteFlowError>>> {
        builder.send_with(&self.0)
    }
    fn describe_connector(&self, builder: DescribeConnectorInputBuilder) -> impl Future<Output = Result<DescribeConnectorOutput, SdkError<DescribeConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn describe_connector_entity(&self, builder: DescribeConnectorEntityInputBuilder) -> impl Future<Output = Result<DescribeConnectorEntityOutput, SdkError<DescribeConnectorEntityError>>> {
        builder.send_with(&self.0)
    }
    fn describe_connector_profiles(&self, builder: DescribeConnectorProfilesInputBuilder) -> impl Future<Output = Result<DescribeConnectorProfilesOutput, SdkError<DescribeConnectorProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_connectors(&self, builder: DescribeConnectorsInputBuilder) -> impl Future<Output = Result<DescribeConnectorsOutput, SdkError<DescribeConnectorsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_flow(&self, builder: DescribeFlowInputBuilder) -> impl Future<Output = Result<DescribeFlowOutput, SdkError<DescribeFlowError>>> {
        builder.send_with(&self.0)
    }
    fn describe_flow_execution_records(&self, builder: DescribeFlowExecutionRecordsInputBuilder) -> impl Future<Output = Result<DescribeFlowExecutionRecordsOutput, SdkError<DescribeFlowExecutionRecordsError>>> {
        builder.send_with(&self.0)
    }
    fn list_connector_entities(&self, builder: ListConnectorEntitiesInputBuilder) -> impl Future<Output = Result<ListConnectorEntitiesOutput, SdkError<ListConnectorEntitiesError>>> {
        builder.send_with(&self.0)
    }
    fn list_connectors(&self, builder: ListConnectorsInputBuilder) -> impl Future<Output = Result<ListConnectorsOutput, SdkError<ListConnectorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_flows(&self, builder: ListFlowsInputBuilder) -> impl Future<Output = Result<ListFlowsOutput, SdkError<ListFlowsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn register_connector(&self, builder: RegisterConnectorInputBuilder) -> impl Future<Output = Result<RegisterConnectorOutput, SdkError<RegisterConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn reset_connector_metadata_cache(&self, builder: ResetConnectorMetadataCacheInputBuilder) -> impl Future<Output = Result<ResetConnectorMetadataCacheOutput, SdkError<ResetConnectorMetadataCacheError>>> {
        builder.send_with(&self.0)
    }
    fn start_flow(&self, builder: StartFlowInputBuilder) -> impl Future<Output = Result<StartFlowOutput, SdkError<StartFlowError>>> {
        builder.send_with(&self.0)
    }
    fn stop_flow(&self, builder: StopFlowInputBuilder) -> impl Future<Output = Result<StopFlowOutput, SdkError<StopFlowError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn unregister_connector(&self, builder: UnregisterConnectorInputBuilder) -> impl Future<Output = Result<UnregisterConnectorOutput, SdkError<UnregisterConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_connector_profile(&self, builder: UpdateConnectorProfileInputBuilder) -> impl Future<Output = Result<UpdateConnectorProfileOutput, SdkError<UpdateConnectorProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_connector_registration(&self, builder: UpdateConnectorRegistrationInputBuilder) -> impl Future<Output = Result<UpdateConnectorRegistrationOutput, SdkError<UpdateConnectorRegistrationError>>> {
        builder.send_with(&self.0)
    }
    fn update_flow(&self, builder: UpdateFlowInputBuilder) -> impl Future<Output = Result<UpdateFlowOutput, SdkError<UpdateFlowError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppFlowClient for T
where T: Deref,
      T::Target: AppFlowClient {
    fn cancel_flow_executions(&self, builder: CancelFlowExecutionsInputBuilder) -> impl Future<Output = Result<CancelFlowExecutionsOutput, SdkError<CancelFlowExecutionsError>>> {
        self.deref().cancel_flow_executions(builder)
    }
    fn create_connector_profile(&self, builder: CreateConnectorProfileInputBuilder) -> impl Future<Output = Result<CreateConnectorProfileOutput, SdkError<CreateConnectorProfileError>>> {
        self.deref().create_connector_profile(builder)
    }
    fn create_flow(&self, builder: CreateFlowInputBuilder) -> impl Future<Output = Result<CreateFlowOutput, SdkError<CreateFlowError>>> {
        self.deref().create_flow(builder)
    }
    fn delete_connector_profile(&self, builder: DeleteConnectorProfileInputBuilder) -> impl Future<Output = Result<DeleteConnectorProfileOutput, SdkError<DeleteConnectorProfileError>>> {
        self.deref().delete_connector_profile(builder)
    }
    fn delete_flow(&self, builder: DeleteFlowInputBuilder) -> impl Future<Output = Result<DeleteFlowOutput, SdkError<DeleteFlowError>>> {
        self.deref().delete_flow(builder)
    }
    fn describe_connector(&self, builder: DescribeConnectorInputBuilder) -> impl Future<Output = Result<DescribeConnectorOutput, SdkError<DescribeConnectorError>>> {
        self.deref().describe_connector(builder)
    }
    fn describe_connector_entity(&self, builder: DescribeConnectorEntityInputBuilder) -> impl Future<Output = Result<DescribeConnectorEntityOutput, SdkError<DescribeConnectorEntityError>>> {
        self.deref().describe_connector_entity(builder)
    }
    fn describe_connector_profiles(&self, builder: DescribeConnectorProfilesInputBuilder) -> impl Future<Output = Result<DescribeConnectorProfilesOutput, SdkError<DescribeConnectorProfilesError>>> {
        self.deref().describe_connector_profiles(builder)
    }
    fn describe_connectors(&self, builder: DescribeConnectorsInputBuilder) -> impl Future<Output = Result<DescribeConnectorsOutput, SdkError<DescribeConnectorsError>>> {
        self.deref().describe_connectors(builder)
    }
    fn describe_flow(&self, builder: DescribeFlowInputBuilder) -> impl Future<Output = Result<DescribeFlowOutput, SdkError<DescribeFlowError>>> {
        self.deref().describe_flow(builder)
    }
    fn describe_flow_execution_records(&self, builder: DescribeFlowExecutionRecordsInputBuilder) -> impl Future<Output = Result<DescribeFlowExecutionRecordsOutput, SdkError<DescribeFlowExecutionRecordsError>>> {
        self.deref().describe_flow_execution_records(builder)
    }
    fn list_connector_entities(&self, builder: ListConnectorEntitiesInputBuilder) -> impl Future<Output = Result<ListConnectorEntitiesOutput, SdkError<ListConnectorEntitiesError>>> {
        self.deref().list_connector_entities(builder)
    }
    fn list_connectors(&self, builder: ListConnectorsInputBuilder) -> impl Future<Output = Result<ListConnectorsOutput, SdkError<ListConnectorsError>>> {
        self.deref().list_connectors(builder)
    }
    fn list_flows(&self, builder: ListFlowsInputBuilder) -> impl Future<Output = Result<ListFlowsOutput, SdkError<ListFlowsError>>> {
        self.deref().list_flows(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn register_connector(&self, builder: RegisterConnectorInputBuilder) -> impl Future<Output = Result<RegisterConnectorOutput, SdkError<RegisterConnectorError>>> {
        self.deref().register_connector(builder)
    }
    fn reset_connector_metadata_cache(&self, builder: ResetConnectorMetadataCacheInputBuilder) -> impl Future<Output = Result<ResetConnectorMetadataCacheOutput, SdkError<ResetConnectorMetadataCacheError>>> {
        self.deref().reset_connector_metadata_cache(builder)
    }
    fn start_flow(&self, builder: StartFlowInputBuilder) -> impl Future<Output = Result<StartFlowOutput, SdkError<StartFlowError>>> {
        self.deref().start_flow(builder)
    }
    fn stop_flow(&self, builder: StopFlowInputBuilder) -> impl Future<Output = Result<StopFlowOutput, SdkError<StopFlowError>>> {
        self.deref().stop_flow(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn unregister_connector(&self, builder: UnregisterConnectorInputBuilder) -> impl Future<Output = Result<UnregisterConnectorOutput, SdkError<UnregisterConnectorError>>> {
        self.deref().unregister_connector(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_connector_profile(&self, builder: UpdateConnectorProfileInputBuilder) -> impl Future<Output = Result<UpdateConnectorProfileOutput, SdkError<UpdateConnectorProfileError>>> {
        self.deref().update_connector_profile(builder)
    }
    fn update_connector_registration(&self, builder: UpdateConnectorRegistrationInputBuilder) -> impl Future<Output = Result<UpdateConnectorRegistrationOutput, SdkError<UpdateConnectorRegistrationError>>> {
        self.deref().update_connector_registration(builder)
    }
    fn update_flow(&self, builder: UpdateFlowInputBuilder) -> impl Future<Output = Result<UpdateFlowOutput, SdkError<UpdateFlowError>>> {
        self.deref().update_flow(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppFlowClient {}
    impl AppFlowClient for edAppFlowClient {
        async fn cancel_flow_executions(&self, builder: CancelFlowExecutionsInputBuilder) -> Result<CancelFlowExecutionsOutput, SdkError<CancelFlowExecutionsError>>;
        async fn create_connector_profile(&self, builder: CreateConnectorProfileInputBuilder) -> Result<CreateConnectorProfileOutput, SdkError<CreateConnectorProfileError>>;
        async fn create_flow(&self, builder: CreateFlowInputBuilder) -> Result<CreateFlowOutput, SdkError<CreateFlowError>>;
        async fn delete_connector_profile(&self, builder: DeleteConnectorProfileInputBuilder) -> Result<DeleteConnectorProfileOutput, SdkError<DeleteConnectorProfileError>>;
        async fn delete_flow(&self, builder: DeleteFlowInputBuilder) -> Result<DeleteFlowOutput, SdkError<DeleteFlowError>>;
        async fn describe_connector(&self, builder: DescribeConnectorInputBuilder) -> Result<DescribeConnectorOutput, SdkError<DescribeConnectorError>>;
        async fn describe_connector_entity(&self, builder: DescribeConnectorEntityInputBuilder) -> Result<DescribeConnectorEntityOutput, SdkError<DescribeConnectorEntityError>>;
        async fn describe_connector_profiles(&self, builder: DescribeConnectorProfilesInputBuilder) -> Result<DescribeConnectorProfilesOutput, SdkError<DescribeConnectorProfilesError>>;
        async fn describe_connectors(&self, builder: DescribeConnectorsInputBuilder) -> Result<DescribeConnectorsOutput, SdkError<DescribeConnectorsError>>;
        async fn describe_flow(&self, builder: DescribeFlowInputBuilder) -> Result<DescribeFlowOutput, SdkError<DescribeFlowError>>;
        async fn describe_flow_execution_records(&self, builder: DescribeFlowExecutionRecordsInputBuilder) -> Result<DescribeFlowExecutionRecordsOutput, SdkError<DescribeFlowExecutionRecordsError>>;
        async fn list_connector_entities(&self, builder: ListConnectorEntitiesInputBuilder) -> Result<ListConnectorEntitiesOutput, SdkError<ListConnectorEntitiesError>>;
        async fn list_connectors(&self, builder: ListConnectorsInputBuilder) -> Result<ListConnectorsOutput, SdkError<ListConnectorsError>>;
        async fn list_flows(&self, builder: ListFlowsInputBuilder) -> Result<ListFlowsOutput, SdkError<ListFlowsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn register_connector(&self, builder: RegisterConnectorInputBuilder) -> Result<RegisterConnectorOutput, SdkError<RegisterConnectorError>>;
        async fn reset_connector_metadata_cache(&self, builder: ResetConnectorMetadataCacheInputBuilder) -> Result<ResetConnectorMetadataCacheOutput, SdkError<ResetConnectorMetadataCacheError>>;
        async fn start_flow(&self, builder: StartFlowInputBuilder) -> Result<StartFlowOutput, SdkError<StartFlowError>>;
        async fn stop_flow(&self, builder: StopFlowInputBuilder) -> Result<StopFlowOutput, SdkError<StopFlowError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn unregister_connector(&self, builder: UnregisterConnectorInputBuilder) -> Result<UnregisterConnectorOutput, SdkError<UnregisterConnectorError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_connector_profile(&self, builder: UpdateConnectorProfileInputBuilder) -> Result<UpdateConnectorProfileOutput, SdkError<UpdateConnectorProfileError>>;
        async fn update_connector_registration(&self, builder: UpdateConnectorRegistrationInputBuilder) -> Result<UpdateConnectorRegistrationOutput, SdkError<UpdateConnectorRegistrationError>>;
        async fn update_flow(&self, builder: UpdateFlowInputBuilder) -> Result<UpdateFlowOutput, SdkError<UpdateFlowError>>;
    }
}
