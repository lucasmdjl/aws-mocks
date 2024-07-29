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
use aws_sdk_globalaccelerator::operation::add_custom_routing_endpoints::{builders::*, *};
use aws_sdk_globalaccelerator::operation::add_endpoints::{builders::*, *};
use aws_sdk_globalaccelerator::operation::advertise_byoip_cidr::{builders::*, *};
use aws_sdk_globalaccelerator::operation::allow_custom_routing_traffic::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_cross_account_attachment::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_custom_routing_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_custom_routing_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_custom_routing_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::create_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_cross_account_attachment::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_custom_routing_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_custom_routing_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_custom_routing_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::delete_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::deny_custom_routing_traffic::{builders::*, *};
use aws_sdk_globalaccelerator::operation::deprovision_byoip_cidr::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_accelerator_attributes::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_cross_account_attachment::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_custom_routing_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_custom_routing_accelerator_attributes::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_custom_routing_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_custom_routing_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::describe_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_accelerators::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_byoip_cidrs::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_cross_account_attachments::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_cross_account_resource_accounts::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_cross_account_resources::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_custom_routing_accelerators::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_custom_routing_endpoint_groups::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_custom_routing_listeners::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_custom_routing_port_mappings::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_custom_routing_port_mappings_by_destination::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_endpoint_groups::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_listeners::{builders::*, *};
use aws_sdk_globalaccelerator::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_globalaccelerator::operation::provision_byoip_cidr::{builders::*, *};
use aws_sdk_globalaccelerator::operation::remove_custom_routing_endpoints::{builders::*, *};
use aws_sdk_globalaccelerator::operation::remove_endpoints::{builders::*, *};
use aws_sdk_globalaccelerator::operation::tag_resource::{builders::*, *};
use aws_sdk_globalaccelerator::operation::untag_resource::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_accelerator_attributes::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_cross_account_attachment::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_custom_routing_accelerator::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_custom_routing_accelerator_attributes::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_custom_routing_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_endpoint_group::{builders::*, *};
use aws_sdk_globalaccelerator::operation::update_listener::{builders::*, *};
use aws_sdk_globalaccelerator::operation::withdraw_byoip_cidr::{builders::*, *};
use aws_sdk_globalaccelerator::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_globalaccelerator::Client;
use std::ops::Deref;

pub use aws_sdk_globalaccelerator::*;

pub struct GlobalAcceleratorClientImpl(Client);
impl GlobalAcceleratorClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait GlobalAcceleratorClient {
    fn add_custom_routing_endpoints(&self, builder: AddCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<AddCustomRoutingEndpointsOutput, SdkError<AddCustomRoutingEndpointsError>>>;
    fn add_endpoints(&self, builder: AddEndpointsInputBuilder) -> impl Future<Output = Result<AddEndpointsOutput, SdkError<AddEndpointsError>>>;
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>>;
    fn allow_custom_routing_traffic(&self, builder: AllowCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<AllowCustomRoutingTrafficOutput, SdkError<AllowCustomRoutingTrafficError>>>;
    fn create_accelerator(&self, builder: CreateAcceleratorInputBuilder) -> impl Future<Output = Result<CreateAcceleratorOutput, SdkError<CreateAcceleratorError>>>;
    fn create_cross_account_attachment(&self, builder: CreateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<CreateCrossAccountAttachmentOutput, SdkError<CreateCrossAccountAttachmentError>>>;
    fn create_custom_routing_accelerator(&self, builder: CreateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingAcceleratorOutput, SdkError<CreateCustomRoutingAcceleratorError>>>;
    fn create_custom_routing_endpoint_group(&self, builder: CreateCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingEndpointGroupOutput, SdkError<CreateCustomRoutingEndpointGroupError>>>;
    fn create_custom_routing_listener(&self, builder: CreateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingListenerOutput, SdkError<CreateCustomRoutingListenerError>>>;
    fn create_endpoint_group(&self, builder: CreateEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateEndpointGroupOutput, SdkError<CreateEndpointGroupError>>>;
    fn create_listener(&self, builder: CreateListenerInputBuilder) -> impl Future<Output = Result<CreateListenerOutput, SdkError<CreateListenerError>>>;
    fn delete_accelerator(&self, builder: DeleteAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteAcceleratorOutput, SdkError<DeleteAcceleratorError>>>;
    fn delete_cross_account_attachment(&self, builder: DeleteCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DeleteCrossAccountAttachmentOutput, SdkError<DeleteCrossAccountAttachmentError>>>;
    fn delete_custom_routing_accelerator(&self, builder: DeleteCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingAcceleratorOutput, SdkError<DeleteCustomRoutingAcceleratorError>>>;
    fn delete_custom_routing_endpoint_group(&self, builder: DeleteCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingEndpointGroupOutput, SdkError<DeleteCustomRoutingEndpointGroupError>>>;
    fn delete_custom_routing_listener(&self, builder: DeleteCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingListenerOutput, SdkError<DeleteCustomRoutingListenerError>>>;
    fn delete_endpoint_group(&self, builder: DeleteEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteEndpointGroupOutput, SdkError<DeleteEndpointGroupError>>>;
    fn delete_listener(&self, builder: DeleteListenerInputBuilder) -> impl Future<Output = Result<DeleteListenerOutput, SdkError<DeleteListenerError>>>;
    fn deny_custom_routing_traffic(&self, builder: DenyCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<DenyCustomRoutingTrafficOutput, SdkError<DenyCustomRoutingTrafficError>>>;
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>>;
    fn describe_accelerator(&self, builder: DescribeAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorOutput, SdkError<DescribeAcceleratorError>>>;
    fn describe_accelerator_attributes(&self, builder: DescribeAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorAttributesOutput, SdkError<DescribeAcceleratorAttributesError>>>;
    fn describe_cross_account_attachment(&self, builder: DescribeCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DescribeCrossAccountAttachmentOutput, SdkError<DescribeCrossAccountAttachmentError>>>;
    fn describe_custom_routing_accelerator(&self, builder: DescribeCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorOutput, SdkError<DescribeCustomRoutingAcceleratorError>>>;
    fn describe_custom_routing_accelerator_attributes(&self, builder: DescribeCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorAttributesOutput, SdkError<DescribeCustomRoutingAcceleratorAttributesError>>>;
    fn describe_custom_routing_endpoint_group(&self, builder: DescribeCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingEndpointGroupOutput, SdkError<DescribeCustomRoutingEndpointGroupError>>>;
    fn describe_custom_routing_listener(&self, builder: DescribeCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingListenerOutput, SdkError<DescribeCustomRoutingListenerError>>>;
    fn describe_endpoint_group(&self, builder: DescribeEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeEndpointGroupOutput, SdkError<DescribeEndpointGroupError>>>;
    fn describe_listener(&self, builder: DescribeListenerInputBuilder) -> impl Future<Output = Result<DescribeListenerOutput, SdkError<DescribeListenerError>>>;
    fn list_accelerators(&self, builder: ListAcceleratorsInputBuilder) -> impl Future<Output = Result<ListAcceleratorsOutput, SdkError<ListAcceleratorsError>>>;
    fn list_byoip_cidrs(&self, builder: ListByoipCidrsInputBuilder) -> impl Future<Output = Result<ListByoipCidrsOutput, SdkError<ListByoipCidrsError>>>;
    fn list_cross_account_attachments(&self, builder: ListCrossAccountAttachmentsInputBuilder) -> impl Future<Output = Result<ListCrossAccountAttachmentsOutput, SdkError<ListCrossAccountAttachmentsError>>>;
    fn list_cross_account_resource_accounts(&self, builder: ListCrossAccountResourceAccountsInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourceAccountsOutput, SdkError<ListCrossAccountResourceAccountsError>>>;
    fn list_cross_account_resources(&self, builder: ListCrossAccountResourcesInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourcesOutput, SdkError<ListCrossAccountResourcesError>>>;
    fn list_custom_routing_accelerators(&self, builder: ListCustomRoutingAcceleratorsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingAcceleratorsOutput, SdkError<ListCustomRoutingAcceleratorsError>>>;
    fn list_custom_routing_endpoint_groups(&self, builder: ListCustomRoutingEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingEndpointGroupsOutput, SdkError<ListCustomRoutingEndpointGroupsError>>>;
    fn list_custom_routing_listeners(&self, builder: ListCustomRoutingListenersInputBuilder) -> impl Future<Output = Result<ListCustomRoutingListenersOutput, SdkError<ListCustomRoutingListenersError>>>;
    fn list_custom_routing_port_mappings(&self, builder: ListCustomRoutingPortMappingsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsOutput, SdkError<ListCustomRoutingPortMappingsError>>>;
    fn list_custom_routing_port_mappings_by_destination(&self, builder: ListCustomRoutingPortMappingsByDestinationInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsByDestinationOutput, SdkError<ListCustomRoutingPortMappingsByDestinationError>>>;
    fn list_endpoint_groups(&self, builder: ListEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListEndpointGroupsOutput, SdkError<ListEndpointGroupsError>>>;
    fn list_listeners(&self, builder: ListListenersInputBuilder) -> impl Future<Output = Result<ListListenersOutput, SdkError<ListListenersError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>>;
    fn remove_custom_routing_endpoints(&self, builder: RemoveCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<RemoveCustomRoutingEndpointsOutput, SdkError<RemoveCustomRoutingEndpointsError>>>;
    fn remove_endpoints(&self, builder: RemoveEndpointsInputBuilder) -> impl Future<Output = Result<RemoveEndpointsOutput, SdkError<RemoveEndpointsError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_accelerator(&self, builder: UpdateAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorOutput, SdkError<UpdateAcceleratorError>>>;
    fn update_accelerator_attributes(&self, builder: UpdateAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorAttributesOutput, SdkError<UpdateAcceleratorAttributesError>>>;
    fn update_cross_account_attachment(&self, builder: UpdateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<UpdateCrossAccountAttachmentOutput, SdkError<UpdateCrossAccountAttachmentError>>>;
    fn update_custom_routing_accelerator(&self, builder: UpdateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorOutput, SdkError<UpdateCustomRoutingAcceleratorError>>>;
    fn update_custom_routing_accelerator_attributes(&self, builder: UpdateCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorAttributesOutput, SdkError<UpdateCustomRoutingAcceleratorAttributesError>>>;
    fn update_custom_routing_listener(&self, builder: UpdateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingListenerOutput, SdkError<UpdateCustomRoutingListenerError>>>;
    fn update_endpoint_group(&self, builder: UpdateEndpointGroupInputBuilder) -> impl Future<Output = Result<UpdateEndpointGroupOutput, SdkError<UpdateEndpointGroupError>>>;
    fn update_listener(&self, builder: UpdateListenerInputBuilder) -> impl Future<Output = Result<UpdateListenerOutput, SdkError<UpdateListenerError>>>;
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>>;
}
impl GlobalAcceleratorClient for GlobalAcceleratorClientImpl {
    fn add_custom_routing_endpoints(&self, builder: AddCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<AddCustomRoutingEndpointsOutput, SdkError<AddCustomRoutingEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn add_endpoints(&self, builder: AddEndpointsInputBuilder) -> impl Future<Output = Result<AddEndpointsOutput, SdkError<AddEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn allow_custom_routing_traffic(&self, builder: AllowCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<AllowCustomRoutingTrafficOutput, SdkError<AllowCustomRoutingTrafficError>>> {
        builder.send_with(&self.0)
    }
    fn create_accelerator(&self, builder: CreateAcceleratorInputBuilder) -> impl Future<Output = Result<CreateAcceleratorOutput, SdkError<CreateAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn create_cross_account_attachment(&self, builder: CreateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<CreateCrossAccountAttachmentOutput, SdkError<CreateCrossAccountAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_routing_accelerator(&self, builder: CreateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingAcceleratorOutput, SdkError<CreateCustomRoutingAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_routing_endpoint_group(&self, builder: CreateCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingEndpointGroupOutput, SdkError<CreateCustomRoutingEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_routing_listener(&self, builder: CreateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingListenerOutput, SdkError<CreateCustomRoutingListenerError>>> {
        builder.send_with(&self.0)
    }
    fn create_endpoint_group(&self, builder: CreateEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateEndpointGroupOutput, SdkError<CreateEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_listener(&self, builder: CreateListenerInputBuilder) -> impl Future<Output = Result<CreateListenerOutput, SdkError<CreateListenerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_accelerator(&self, builder: DeleteAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteAcceleratorOutput, SdkError<DeleteAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cross_account_attachment(&self, builder: DeleteCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DeleteCrossAccountAttachmentOutput, SdkError<DeleteCrossAccountAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_routing_accelerator(&self, builder: DeleteCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingAcceleratorOutput, SdkError<DeleteCustomRoutingAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_routing_endpoint_group(&self, builder: DeleteCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingEndpointGroupOutput, SdkError<DeleteCustomRoutingEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_routing_listener(&self, builder: DeleteCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingListenerOutput, SdkError<DeleteCustomRoutingListenerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_endpoint_group(&self, builder: DeleteEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteEndpointGroupOutput, SdkError<DeleteEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_listener(&self, builder: DeleteListenerInputBuilder) -> impl Future<Output = Result<DeleteListenerOutput, SdkError<DeleteListenerError>>> {
        builder.send_with(&self.0)
    }
    fn deny_custom_routing_traffic(&self, builder: DenyCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<DenyCustomRoutingTrafficOutput, SdkError<DenyCustomRoutingTrafficError>>> {
        builder.send_with(&self.0)
    }
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn describe_accelerator(&self, builder: DescribeAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorOutput, SdkError<DescribeAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn describe_accelerator_attributes(&self, builder: DescribeAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorAttributesOutput, SdkError<DescribeAcceleratorAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cross_account_attachment(&self, builder: DescribeCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DescribeCrossAccountAttachmentOutput, SdkError<DescribeCrossAccountAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_routing_accelerator(&self, builder: DescribeCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorOutput, SdkError<DescribeCustomRoutingAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_routing_accelerator_attributes(&self, builder: DescribeCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorAttributesOutput, SdkError<DescribeCustomRoutingAcceleratorAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_routing_endpoint_group(&self, builder: DescribeCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingEndpointGroupOutput, SdkError<DescribeCustomRoutingEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_routing_listener(&self, builder: DescribeCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingListenerOutput, SdkError<DescribeCustomRoutingListenerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoint_group(&self, builder: DescribeEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeEndpointGroupOutput, SdkError<DescribeEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_listener(&self, builder: DescribeListenerInputBuilder) -> impl Future<Output = Result<DescribeListenerOutput, SdkError<DescribeListenerError>>> {
        builder.send_with(&self.0)
    }
    fn list_accelerators(&self, builder: ListAcceleratorsInputBuilder) -> impl Future<Output = Result<ListAcceleratorsOutput, SdkError<ListAcceleratorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_byoip_cidrs(&self, builder: ListByoipCidrsInputBuilder) -> impl Future<Output = Result<ListByoipCidrsOutput, SdkError<ListByoipCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn list_cross_account_attachments(&self, builder: ListCrossAccountAttachmentsInputBuilder) -> impl Future<Output = Result<ListCrossAccountAttachmentsOutput, SdkError<ListCrossAccountAttachmentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_cross_account_resource_accounts(&self, builder: ListCrossAccountResourceAccountsInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourceAccountsOutput, SdkError<ListCrossAccountResourceAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn list_cross_account_resources(&self, builder: ListCrossAccountResourcesInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourcesOutput, SdkError<ListCrossAccountResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_routing_accelerators(&self, builder: ListCustomRoutingAcceleratorsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingAcceleratorsOutput, SdkError<ListCustomRoutingAcceleratorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_routing_endpoint_groups(&self, builder: ListCustomRoutingEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingEndpointGroupsOutput, SdkError<ListCustomRoutingEndpointGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_routing_listeners(&self, builder: ListCustomRoutingListenersInputBuilder) -> impl Future<Output = Result<ListCustomRoutingListenersOutput, SdkError<ListCustomRoutingListenersError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_routing_port_mappings(&self, builder: ListCustomRoutingPortMappingsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsOutput, SdkError<ListCustomRoutingPortMappingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_routing_port_mappings_by_destination(&self, builder: ListCustomRoutingPortMappingsByDestinationInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsByDestinationOutput, SdkError<ListCustomRoutingPortMappingsByDestinationError>>> {
        builder.send_with(&self.0)
    }
    fn list_endpoint_groups(&self, builder: ListEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListEndpointGroupsOutput, SdkError<ListEndpointGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_listeners(&self, builder: ListListenersInputBuilder) -> impl Future<Output = Result<ListListenersOutput, SdkError<ListListenersError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn remove_custom_routing_endpoints(&self, builder: RemoveCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<RemoveCustomRoutingEndpointsOutput, SdkError<RemoveCustomRoutingEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn remove_endpoints(&self, builder: RemoveEndpointsInputBuilder) -> impl Future<Output = Result<RemoveEndpointsOutput, SdkError<RemoveEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_accelerator(&self, builder: UpdateAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorOutput, SdkError<UpdateAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn update_accelerator_attributes(&self, builder: UpdateAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorAttributesOutput, SdkError<UpdateAcceleratorAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn update_cross_account_attachment(&self, builder: UpdateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<UpdateCrossAccountAttachmentOutput, SdkError<UpdateCrossAccountAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_custom_routing_accelerator(&self, builder: UpdateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorOutput, SdkError<UpdateCustomRoutingAcceleratorError>>> {
        builder.send_with(&self.0)
    }
    fn update_custom_routing_accelerator_attributes(&self, builder: UpdateCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorAttributesOutput, SdkError<UpdateCustomRoutingAcceleratorAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn update_custom_routing_listener(&self, builder: UpdateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingListenerOutput, SdkError<UpdateCustomRoutingListenerError>>> {
        builder.send_with(&self.0)
    }
    fn update_endpoint_group(&self, builder: UpdateEndpointGroupInputBuilder) -> impl Future<Output = Result<UpdateEndpointGroupOutput, SdkError<UpdateEndpointGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_listener(&self, builder: UpdateListenerInputBuilder) -> impl Future<Output = Result<UpdateListenerOutput, SdkError<UpdateListenerError>>> {
        builder.send_with(&self.0)
    }
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> GlobalAcceleratorClient for T
where T: Deref,
      T::Target: GlobalAcceleratorClient {
    fn add_custom_routing_endpoints(&self, builder: AddCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<AddCustomRoutingEndpointsOutput, SdkError<AddCustomRoutingEndpointsError>>> {
        self.deref().add_custom_routing_endpoints(builder)
    }
    fn add_endpoints(&self, builder: AddEndpointsInputBuilder) -> impl Future<Output = Result<AddEndpointsOutput, SdkError<AddEndpointsError>>> {
        self.deref().add_endpoints(builder)
    }
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>> {
        self.deref().advertise_byoip_cidr(builder)
    }
    fn allow_custom_routing_traffic(&self, builder: AllowCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<AllowCustomRoutingTrafficOutput, SdkError<AllowCustomRoutingTrafficError>>> {
        self.deref().allow_custom_routing_traffic(builder)
    }
    fn create_accelerator(&self, builder: CreateAcceleratorInputBuilder) -> impl Future<Output = Result<CreateAcceleratorOutput, SdkError<CreateAcceleratorError>>> {
        self.deref().create_accelerator(builder)
    }
    fn create_cross_account_attachment(&self, builder: CreateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<CreateCrossAccountAttachmentOutput, SdkError<CreateCrossAccountAttachmentError>>> {
        self.deref().create_cross_account_attachment(builder)
    }
    fn create_custom_routing_accelerator(&self, builder: CreateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingAcceleratorOutput, SdkError<CreateCustomRoutingAcceleratorError>>> {
        self.deref().create_custom_routing_accelerator(builder)
    }
    fn create_custom_routing_endpoint_group(&self, builder: CreateCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingEndpointGroupOutput, SdkError<CreateCustomRoutingEndpointGroupError>>> {
        self.deref().create_custom_routing_endpoint_group(builder)
    }
    fn create_custom_routing_listener(&self, builder: CreateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<CreateCustomRoutingListenerOutput, SdkError<CreateCustomRoutingListenerError>>> {
        self.deref().create_custom_routing_listener(builder)
    }
    fn create_endpoint_group(&self, builder: CreateEndpointGroupInputBuilder) -> impl Future<Output = Result<CreateEndpointGroupOutput, SdkError<CreateEndpointGroupError>>> {
        self.deref().create_endpoint_group(builder)
    }
    fn create_listener(&self, builder: CreateListenerInputBuilder) -> impl Future<Output = Result<CreateListenerOutput, SdkError<CreateListenerError>>> {
        self.deref().create_listener(builder)
    }
    fn delete_accelerator(&self, builder: DeleteAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteAcceleratorOutput, SdkError<DeleteAcceleratorError>>> {
        self.deref().delete_accelerator(builder)
    }
    fn delete_cross_account_attachment(&self, builder: DeleteCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DeleteCrossAccountAttachmentOutput, SdkError<DeleteCrossAccountAttachmentError>>> {
        self.deref().delete_cross_account_attachment(builder)
    }
    fn delete_custom_routing_accelerator(&self, builder: DeleteCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingAcceleratorOutput, SdkError<DeleteCustomRoutingAcceleratorError>>> {
        self.deref().delete_custom_routing_accelerator(builder)
    }
    fn delete_custom_routing_endpoint_group(&self, builder: DeleteCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingEndpointGroupOutput, SdkError<DeleteCustomRoutingEndpointGroupError>>> {
        self.deref().delete_custom_routing_endpoint_group(builder)
    }
    fn delete_custom_routing_listener(&self, builder: DeleteCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DeleteCustomRoutingListenerOutput, SdkError<DeleteCustomRoutingListenerError>>> {
        self.deref().delete_custom_routing_listener(builder)
    }
    fn delete_endpoint_group(&self, builder: DeleteEndpointGroupInputBuilder) -> impl Future<Output = Result<DeleteEndpointGroupOutput, SdkError<DeleteEndpointGroupError>>> {
        self.deref().delete_endpoint_group(builder)
    }
    fn delete_listener(&self, builder: DeleteListenerInputBuilder) -> impl Future<Output = Result<DeleteListenerOutput, SdkError<DeleteListenerError>>> {
        self.deref().delete_listener(builder)
    }
    fn deny_custom_routing_traffic(&self, builder: DenyCustomRoutingTrafficInputBuilder) -> impl Future<Output = Result<DenyCustomRoutingTrafficOutput, SdkError<DenyCustomRoutingTrafficError>>> {
        self.deref().deny_custom_routing_traffic(builder)
    }
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>> {
        self.deref().deprovision_byoip_cidr(builder)
    }
    fn describe_accelerator(&self, builder: DescribeAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorOutput, SdkError<DescribeAcceleratorError>>> {
        self.deref().describe_accelerator(builder)
    }
    fn describe_accelerator_attributes(&self, builder: DescribeAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeAcceleratorAttributesOutput, SdkError<DescribeAcceleratorAttributesError>>> {
        self.deref().describe_accelerator_attributes(builder)
    }
    fn describe_cross_account_attachment(&self, builder: DescribeCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<DescribeCrossAccountAttachmentOutput, SdkError<DescribeCrossAccountAttachmentError>>> {
        self.deref().describe_cross_account_attachment(builder)
    }
    fn describe_custom_routing_accelerator(&self, builder: DescribeCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorOutput, SdkError<DescribeCustomRoutingAcceleratorError>>> {
        self.deref().describe_custom_routing_accelerator(builder)
    }
    fn describe_custom_routing_accelerator_attributes(&self, builder: DescribeCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingAcceleratorAttributesOutput, SdkError<DescribeCustomRoutingAcceleratorAttributesError>>> {
        self.deref().describe_custom_routing_accelerator_attributes(builder)
    }
    fn describe_custom_routing_endpoint_group(&self, builder: DescribeCustomRoutingEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingEndpointGroupOutput, SdkError<DescribeCustomRoutingEndpointGroupError>>> {
        self.deref().describe_custom_routing_endpoint_group(builder)
    }
    fn describe_custom_routing_listener(&self, builder: DescribeCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<DescribeCustomRoutingListenerOutput, SdkError<DescribeCustomRoutingListenerError>>> {
        self.deref().describe_custom_routing_listener(builder)
    }
    fn describe_endpoint_group(&self, builder: DescribeEndpointGroupInputBuilder) -> impl Future<Output = Result<DescribeEndpointGroupOutput, SdkError<DescribeEndpointGroupError>>> {
        self.deref().describe_endpoint_group(builder)
    }
    fn describe_listener(&self, builder: DescribeListenerInputBuilder) -> impl Future<Output = Result<DescribeListenerOutput, SdkError<DescribeListenerError>>> {
        self.deref().describe_listener(builder)
    }
    fn list_accelerators(&self, builder: ListAcceleratorsInputBuilder) -> impl Future<Output = Result<ListAcceleratorsOutput, SdkError<ListAcceleratorsError>>> {
        self.deref().list_accelerators(builder)
    }
    fn list_byoip_cidrs(&self, builder: ListByoipCidrsInputBuilder) -> impl Future<Output = Result<ListByoipCidrsOutput, SdkError<ListByoipCidrsError>>> {
        self.deref().list_byoip_cidrs(builder)
    }
    fn list_cross_account_attachments(&self, builder: ListCrossAccountAttachmentsInputBuilder) -> impl Future<Output = Result<ListCrossAccountAttachmentsOutput, SdkError<ListCrossAccountAttachmentsError>>> {
        self.deref().list_cross_account_attachments(builder)
    }
    fn list_cross_account_resource_accounts(&self, builder: ListCrossAccountResourceAccountsInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourceAccountsOutput, SdkError<ListCrossAccountResourceAccountsError>>> {
        self.deref().list_cross_account_resource_accounts(builder)
    }
    fn list_cross_account_resources(&self, builder: ListCrossAccountResourcesInputBuilder) -> impl Future<Output = Result<ListCrossAccountResourcesOutput, SdkError<ListCrossAccountResourcesError>>> {
        self.deref().list_cross_account_resources(builder)
    }
    fn list_custom_routing_accelerators(&self, builder: ListCustomRoutingAcceleratorsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingAcceleratorsOutput, SdkError<ListCustomRoutingAcceleratorsError>>> {
        self.deref().list_custom_routing_accelerators(builder)
    }
    fn list_custom_routing_endpoint_groups(&self, builder: ListCustomRoutingEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingEndpointGroupsOutput, SdkError<ListCustomRoutingEndpointGroupsError>>> {
        self.deref().list_custom_routing_endpoint_groups(builder)
    }
    fn list_custom_routing_listeners(&self, builder: ListCustomRoutingListenersInputBuilder) -> impl Future<Output = Result<ListCustomRoutingListenersOutput, SdkError<ListCustomRoutingListenersError>>> {
        self.deref().list_custom_routing_listeners(builder)
    }
    fn list_custom_routing_port_mappings(&self, builder: ListCustomRoutingPortMappingsInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsOutput, SdkError<ListCustomRoutingPortMappingsError>>> {
        self.deref().list_custom_routing_port_mappings(builder)
    }
    fn list_custom_routing_port_mappings_by_destination(&self, builder: ListCustomRoutingPortMappingsByDestinationInputBuilder) -> impl Future<Output = Result<ListCustomRoutingPortMappingsByDestinationOutput, SdkError<ListCustomRoutingPortMappingsByDestinationError>>> {
        self.deref().list_custom_routing_port_mappings_by_destination(builder)
    }
    fn list_endpoint_groups(&self, builder: ListEndpointGroupsInputBuilder) -> impl Future<Output = Result<ListEndpointGroupsOutput, SdkError<ListEndpointGroupsError>>> {
        self.deref().list_endpoint_groups(builder)
    }
    fn list_listeners(&self, builder: ListListenersInputBuilder) -> impl Future<Output = Result<ListListenersOutput, SdkError<ListListenersError>>> {
        self.deref().list_listeners(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>> {
        self.deref().provision_byoip_cidr(builder)
    }
    fn remove_custom_routing_endpoints(&self, builder: RemoveCustomRoutingEndpointsInputBuilder) -> impl Future<Output = Result<RemoveCustomRoutingEndpointsOutput, SdkError<RemoveCustomRoutingEndpointsError>>> {
        self.deref().remove_custom_routing_endpoints(builder)
    }
    fn remove_endpoints(&self, builder: RemoveEndpointsInputBuilder) -> impl Future<Output = Result<RemoveEndpointsOutput, SdkError<RemoveEndpointsError>>> {
        self.deref().remove_endpoints(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_accelerator(&self, builder: UpdateAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorOutput, SdkError<UpdateAcceleratorError>>> {
        self.deref().update_accelerator(builder)
    }
    fn update_accelerator_attributes(&self, builder: UpdateAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateAcceleratorAttributesOutput, SdkError<UpdateAcceleratorAttributesError>>> {
        self.deref().update_accelerator_attributes(builder)
    }
    fn update_cross_account_attachment(&self, builder: UpdateCrossAccountAttachmentInputBuilder) -> impl Future<Output = Result<UpdateCrossAccountAttachmentOutput, SdkError<UpdateCrossAccountAttachmentError>>> {
        self.deref().update_cross_account_attachment(builder)
    }
    fn update_custom_routing_accelerator(&self, builder: UpdateCustomRoutingAcceleratorInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorOutput, SdkError<UpdateCustomRoutingAcceleratorError>>> {
        self.deref().update_custom_routing_accelerator(builder)
    }
    fn update_custom_routing_accelerator_attributes(&self, builder: UpdateCustomRoutingAcceleratorAttributesInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingAcceleratorAttributesOutput, SdkError<UpdateCustomRoutingAcceleratorAttributesError>>> {
        self.deref().update_custom_routing_accelerator_attributes(builder)
    }
    fn update_custom_routing_listener(&self, builder: UpdateCustomRoutingListenerInputBuilder) -> impl Future<Output = Result<UpdateCustomRoutingListenerOutput, SdkError<UpdateCustomRoutingListenerError>>> {
        self.deref().update_custom_routing_listener(builder)
    }
    fn update_endpoint_group(&self, builder: UpdateEndpointGroupInputBuilder) -> impl Future<Output = Result<UpdateEndpointGroupOutput, SdkError<UpdateEndpointGroupError>>> {
        self.deref().update_endpoint_group(builder)
    }
    fn update_listener(&self, builder: UpdateListenerInputBuilder) -> impl Future<Output = Result<UpdateListenerOutput, SdkError<UpdateListenerError>>> {
        self.deref().update_listener(builder)
    }
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>> {
        self.deref().withdraw_byoip_cidr(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edGlobalAcceleratorClient {}
    impl GlobalAcceleratorClient for edGlobalAcceleratorClient {
        async fn add_custom_routing_endpoints(&self, builder: AddCustomRoutingEndpointsInputBuilder) -> Result<AddCustomRoutingEndpointsOutput, SdkError<AddCustomRoutingEndpointsError>>;
        async fn add_endpoints(&self, builder: AddEndpointsInputBuilder) -> Result<AddEndpointsOutput, SdkError<AddEndpointsError>>;
        async fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>;
        async fn allow_custom_routing_traffic(&self, builder: AllowCustomRoutingTrafficInputBuilder) -> Result<AllowCustomRoutingTrafficOutput, SdkError<AllowCustomRoutingTrafficError>>;
        async fn create_accelerator(&self, builder: CreateAcceleratorInputBuilder) -> Result<CreateAcceleratorOutput, SdkError<CreateAcceleratorError>>;
        async fn create_cross_account_attachment(&self, builder: CreateCrossAccountAttachmentInputBuilder) -> Result<CreateCrossAccountAttachmentOutput, SdkError<CreateCrossAccountAttachmentError>>;
        async fn create_custom_routing_accelerator(&self, builder: CreateCustomRoutingAcceleratorInputBuilder) -> Result<CreateCustomRoutingAcceleratorOutput, SdkError<CreateCustomRoutingAcceleratorError>>;
        async fn create_custom_routing_endpoint_group(&self, builder: CreateCustomRoutingEndpointGroupInputBuilder) -> Result<CreateCustomRoutingEndpointGroupOutput, SdkError<CreateCustomRoutingEndpointGroupError>>;
        async fn create_custom_routing_listener(&self, builder: CreateCustomRoutingListenerInputBuilder) -> Result<CreateCustomRoutingListenerOutput, SdkError<CreateCustomRoutingListenerError>>;
        async fn create_endpoint_group(&self, builder: CreateEndpointGroupInputBuilder) -> Result<CreateEndpointGroupOutput, SdkError<CreateEndpointGroupError>>;
        async fn create_listener(&self, builder: CreateListenerInputBuilder) -> Result<CreateListenerOutput, SdkError<CreateListenerError>>;
        async fn delete_accelerator(&self, builder: DeleteAcceleratorInputBuilder) -> Result<DeleteAcceleratorOutput, SdkError<DeleteAcceleratorError>>;
        async fn delete_cross_account_attachment(&self, builder: DeleteCrossAccountAttachmentInputBuilder) -> Result<DeleteCrossAccountAttachmentOutput, SdkError<DeleteCrossAccountAttachmentError>>;
        async fn delete_custom_routing_accelerator(&self, builder: DeleteCustomRoutingAcceleratorInputBuilder) -> Result<DeleteCustomRoutingAcceleratorOutput, SdkError<DeleteCustomRoutingAcceleratorError>>;
        async fn delete_custom_routing_endpoint_group(&self, builder: DeleteCustomRoutingEndpointGroupInputBuilder) -> Result<DeleteCustomRoutingEndpointGroupOutput, SdkError<DeleteCustomRoutingEndpointGroupError>>;
        async fn delete_custom_routing_listener(&self, builder: DeleteCustomRoutingListenerInputBuilder) -> Result<DeleteCustomRoutingListenerOutput, SdkError<DeleteCustomRoutingListenerError>>;
        async fn delete_endpoint_group(&self, builder: DeleteEndpointGroupInputBuilder) -> Result<DeleteEndpointGroupOutput, SdkError<DeleteEndpointGroupError>>;
        async fn delete_listener(&self, builder: DeleteListenerInputBuilder) -> Result<DeleteListenerOutput, SdkError<DeleteListenerError>>;
        async fn deny_custom_routing_traffic(&self, builder: DenyCustomRoutingTrafficInputBuilder) -> Result<DenyCustomRoutingTrafficOutput, SdkError<DenyCustomRoutingTrafficError>>;
        async fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>;
        async fn describe_accelerator(&self, builder: DescribeAcceleratorInputBuilder) -> Result<DescribeAcceleratorOutput, SdkError<DescribeAcceleratorError>>;
        async fn describe_accelerator_attributes(&self, builder: DescribeAcceleratorAttributesInputBuilder) -> Result<DescribeAcceleratorAttributesOutput, SdkError<DescribeAcceleratorAttributesError>>;
        async fn describe_cross_account_attachment(&self, builder: DescribeCrossAccountAttachmentInputBuilder) -> Result<DescribeCrossAccountAttachmentOutput, SdkError<DescribeCrossAccountAttachmentError>>;
        async fn describe_custom_routing_accelerator(&self, builder: DescribeCustomRoutingAcceleratorInputBuilder) -> Result<DescribeCustomRoutingAcceleratorOutput, SdkError<DescribeCustomRoutingAcceleratorError>>;
        async fn describe_custom_routing_accelerator_attributes(&self, builder: DescribeCustomRoutingAcceleratorAttributesInputBuilder) -> Result<DescribeCustomRoutingAcceleratorAttributesOutput, SdkError<DescribeCustomRoutingAcceleratorAttributesError>>;
        async fn describe_custom_routing_endpoint_group(&self, builder: DescribeCustomRoutingEndpointGroupInputBuilder) -> Result<DescribeCustomRoutingEndpointGroupOutput, SdkError<DescribeCustomRoutingEndpointGroupError>>;
        async fn describe_custom_routing_listener(&self, builder: DescribeCustomRoutingListenerInputBuilder) -> Result<DescribeCustomRoutingListenerOutput, SdkError<DescribeCustomRoutingListenerError>>;
        async fn describe_endpoint_group(&self, builder: DescribeEndpointGroupInputBuilder) -> Result<DescribeEndpointGroupOutput, SdkError<DescribeEndpointGroupError>>;
        async fn describe_listener(&self, builder: DescribeListenerInputBuilder) -> Result<DescribeListenerOutput, SdkError<DescribeListenerError>>;
        async fn list_accelerators(&self, builder: ListAcceleratorsInputBuilder) -> Result<ListAcceleratorsOutput, SdkError<ListAcceleratorsError>>;
        async fn list_byoip_cidrs(&self, builder: ListByoipCidrsInputBuilder) -> Result<ListByoipCidrsOutput, SdkError<ListByoipCidrsError>>;
        async fn list_cross_account_attachments(&self, builder: ListCrossAccountAttachmentsInputBuilder) -> Result<ListCrossAccountAttachmentsOutput, SdkError<ListCrossAccountAttachmentsError>>;
        async fn list_cross_account_resource_accounts(&self, builder: ListCrossAccountResourceAccountsInputBuilder) -> Result<ListCrossAccountResourceAccountsOutput, SdkError<ListCrossAccountResourceAccountsError>>;
        async fn list_cross_account_resources(&self, builder: ListCrossAccountResourcesInputBuilder) -> Result<ListCrossAccountResourcesOutput, SdkError<ListCrossAccountResourcesError>>;
        async fn list_custom_routing_accelerators(&self, builder: ListCustomRoutingAcceleratorsInputBuilder) -> Result<ListCustomRoutingAcceleratorsOutput, SdkError<ListCustomRoutingAcceleratorsError>>;
        async fn list_custom_routing_endpoint_groups(&self, builder: ListCustomRoutingEndpointGroupsInputBuilder) -> Result<ListCustomRoutingEndpointGroupsOutput, SdkError<ListCustomRoutingEndpointGroupsError>>;
        async fn list_custom_routing_listeners(&self, builder: ListCustomRoutingListenersInputBuilder) -> Result<ListCustomRoutingListenersOutput, SdkError<ListCustomRoutingListenersError>>;
        async fn list_custom_routing_port_mappings(&self, builder: ListCustomRoutingPortMappingsInputBuilder) -> Result<ListCustomRoutingPortMappingsOutput, SdkError<ListCustomRoutingPortMappingsError>>;
        async fn list_custom_routing_port_mappings_by_destination(&self, builder: ListCustomRoutingPortMappingsByDestinationInputBuilder) -> Result<ListCustomRoutingPortMappingsByDestinationOutput, SdkError<ListCustomRoutingPortMappingsByDestinationError>>;
        async fn list_endpoint_groups(&self, builder: ListEndpointGroupsInputBuilder) -> Result<ListEndpointGroupsOutput, SdkError<ListEndpointGroupsError>>;
        async fn list_listeners(&self, builder: ListListenersInputBuilder) -> Result<ListListenersOutput, SdkError<ListListenersError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>;
        async fn remove_custom_routing_endpoints(&self, builder: RemoveCustomRoutingEndpointsInputBuilder) -> Result<RemoveCustomRoutingEndpointsOutput, SdkError<RemoveCustomRoutingEndpointsError>>;
        async fn remove_endpoints(&self, builder: RemoveEndpointsInputBuilder) -> Result<RemoveEndpointsOutput, SdkError<RemoveEndpointsError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_accelerator(&self, builder: UpdateAcceleratorInputBuilder) -> Result<UpdateAcceleratorOutput, SdkError<UpdateAcceleratorError>>;
        async fn update_accelerator_attributes(&self, builder: UpdateAcceleratorAttributesInputBuilder) -> Result<UpdateAcceleratorAttributesOutput, SdkError<UpdateAcceleratorAttributesError>>;
        async fn update_cross_account_attachment(&self, builder: UpdateCrossAccountAttachmentInputBuilder) -> Result<UpdateCrossAccountAttachmentOutput, SdkError<UpdateCrossAccountAttachmentError>>;
        async fn update_custom_routing_accelerator(&self, builder: UpdateCustomRoutingAcceleratorInputBuilder) -> Result<UpdateCustomRoutingAcceleratorOutput, SdkError<UpdateCustomRoutingAcceleratorError>>;
        async fn update_custom_routing_accelerator_attributes(&self, builder: UpdateCustomRoutingAcceleratorAttributesInputBuilder) -> Result<UpdateCustomRoutingAcceleratorAttributesOutput, SdkError<UpdateCustomRoutingAcceleratorAttributesError>>;
        async fn update_custom_routing_listener(&self, builder: UpdateCustomRoutingListenerInputBuilder) -> Result<UpdateCustomRoutingListenerOutput, SdkError<UpdateCustomRoutingListenerError>>;
        async fn update_endpoint_group(&self, builder: UpdateEndpointGroupInputBuilder) -> Result<UpdateEndpointGroupOutput, SdkError<UpdateEndpointGroupError>>;
        async fn update_listener(&self, builder: UpdateListenerInputBuilder) -> Result<UpdateListenerOutput, SdkError<UpdateListenerError>>;
        async fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>;
    }
}
