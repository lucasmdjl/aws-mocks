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
use aws_sdk_directconnect::operation::accept_direct_connect_gateway_association_proposal::{builders::*, *};
use aws_sdk_directconnect::operation::allocate_hosted_connection::{builders::*, *};
use aws_sdk_directconnect::operation::allocate_private_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::allocate_public_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::allocate_transit_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::associate_connection_with_lag::{builders::*, *};
use aws_sdk_directconnect::operation::associate_hosted_connection::{builders::*, *};
use aws_sdk_directconnect::operation::associate_mac_sec_key::{builders::*, *};
use aws_sdk_directconnect::operation::associate_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::confirm_connection::{builders::*, *};
use aws_sdk_directconnect::operation::confirm_customer_agreement::{builders::*, *};
use aws_sdk_directconnect::operation::confirm_private_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::confirm_public_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::confirm_transit_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::create_bgp_peer::{builders::*, *};
use aws_sdk_directconnect::operation::create_connection::{builders::*, *};
use aws_sdk_directconnect::operation::create_direct_connect_gateway::{builders::*, *};
use aws_sdk_directconnect::operation::create_direct_connect_gateway_association::{builders::*, *};
use aws_sdk_directconnect::operation::create_direct_connect_gateway_association_proposal::{builders::*, *};
use aws_sdk_directconnect::operation::create_interconnect::{builders::*, *};
use aws_sdk_directconnect::operation::create_lag::{builders::*, *};
use aws_sdk_directconnect::operation::create_private_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::create_public_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::create_transit_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::delete_bgp_peer::{builders::*, *};
use aws_sdk_directconnect::operation::delete_connection::{builders::*, *};
use aws_sdk_directconnect::operation::delete_direct_connect_gateway::{builders::*, *};
use aws_sdk_directconnect::operation::delete_direct_connect_gateway_association::{builders::*, *};
use aws_sdk_directconnect::operation::delete_direct_connect_gateway_association_proposal::{builders::*, *};
use aws_sdk_directconnect::operation::delete_interconnect::{builders::*, *};
use aws_sdk_directconnect::operation::delete_lag::{builders::*, *};
use aws_sdk_directconnect::operation::delete_virtual_interface::{builders::*, *};
use aws_sdk_directconnect::operation::describe_connections::{builders::*, *};
use aws_sdk_directconnect::operation::describe_customer_metadata::{builders::*, *};
use aws_sdk_directconnect::operation::describe_direct_connect_gateway_association_proposals::{builders::*, *};
use aws_sdk_directconnect::operation::describe_direct_connect_gateway_associations::{builders::*, *};
use aws_sdk_directconnect::operation::describe_direct_connect_gateway_attachments::{builders::*, *};
use aws_sdk_directconnect::operation::describe_direct_connect_gateways::{builders::*, *};
use aws_sdk_directconnect::operation::describe_hosted_connections::{builders::*, *};
use aws_sdk_directconnect::operation::describe_interconnects::{builders::*, *};
use aws_sdk_directconnect::operation::describe_lags::{builders::*, *};
use aws_sdk_directconnect::operation::describe_loa::{builders::*, *};
use aws_sdk_directconnect::operation::describe_locations::{builders::*, *};
use aws_sdk_directconnect::operation::describe_router_configuration::{builders::*, *};
use aws_sdk_directconnect::operation::describe_tags::{builders::*, *};
use aws_sdk_directconnect::operation::describe_virtual_gateways::{builders::*, *};
use aws_sdk_directconnect::operation::describe_virtual_interfaces::{builders::*, *};
use aws_sdk_directconnect::operation::disassociate_connection_from_lag::{builders::*, *};
use aws_sdk_directconnect::operation::disassociate_mac_sec_key::{builders::*, *};
use aws_sdk_directconnect::operation::list_virtual_interface_test_history::{builders::*, *};
use aws_sdk_directconnect::operation::start_bgp_failover_test::{builders::*, *};
use aws_sdk_directconnect::operation::stop_bgp_failover_test::{builders::*, *};
use aws_sdk_directconnect::operation::tag_resource::{builders::*, *};
use aws_sdk_directconnect::operation::untag_resource::{builders::*, *};
use aws_sdk_directconnect::operation::update_connection::{builders::*, *};
use aws_sdk_directconnect::operation::update_direct_connect_gateway::{builders::*, *};
use aws_sdk_directconnect::operation::update_direct_connect_gateway_association::{builders::*, *};
use aws_sdk_directconnect::operation::update_lag::{builders::*, *};
use aws_sdk_directconnect::operation::update_virtual_interface_attributes::{builders::*, *};
use aws_sdk_directconnect::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_directconnect::Client;
use std::ops::Deref;

pub use aws_sdk_directconnect::*;

pub struct DirectConnectClientImpl(Client);
impl DirectConnectClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait DirectConnectClient {
    fn accept_direct_connect_gateway_association_proposal(&self, builder: AcceptDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<AcceptDirectConnectGatewayAssociationProposalOutput, SdkError<AcceptDirectConnectGatewayAssociationProposalError>>> + Send;
    fn allocate_hosted_connection(&self, builder: AllocateHostedConnectionInputBuilder) -> impl Future<Output = Result<AllocateHostedConnectionOutput, SdkError<AllocateHostedConnectionError>>> + Send;
    fn allocate_private_virtual_interface(&self, builder: AllocatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePrivateVirtualInterfaceOutput, SdkError<AllocatePrivateVirtualInterfaceError>>> + Send;
    fn allocate_public_virtual_interface(&self, builder: AllocatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePublicVirtualInterfaceOutput, SdkError<AllocatePublicVirtualInterfaceError>>> + Send;
    fn allocate_transit_virtual_interface(&self, builder: AllocateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocateTransitVirtualInterfaceOutput, SdkError<AllocateTransitVirtualInterfaceError>>> + Send;
    fn associate_connection_with_lag(&self, builder: AssociateConnectionWithLagInputBuilder) -> impl Future<Output = Result<AssociateConnectionWithLagOutput, SdkError<AssociateConnectionWithLagError>>> + Send;
    fn associate_hosted_connection(&self, builder: AssociateHostedConnectionInputBuilder) -> impl Future<Output = Result<AssociateHostedConnectionOutput, SdkError<AssociateHostedConnectionError>>> + Send;
    fn associate_mac_sec_key(&self, builder: AssociateMacSecKeyInputBuilder) -> impl Future<Output = Result<AssociateMacSecKeyOutput, SdkError<AssociateMacSecKeyError>>> + Send;
    fn associate_virtual_interface(&self, builder: AssociateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AssociateVirtualInterfaceOutput, SdkError<AssociateVirtualInterfaceError>>> + Send;
    fn confirm_connection(&self, builder: ConfirmConnectionInputBuilder) -> impl Future<Output = Result<ConfirmConnectionOutput, SdkError<ConfirmConnectionError>>> + Send;
    fn confirm_customer_agreement(&self, builder: ConfirmCustomerAgreementInputBuilder) -> impl Future<Output = Result<ConfirmCustomerAgreementOutput, SdkError<ConfirmCustomerAgreementError>>> + Send;
    fn confirm_private_virtual_interface(&self, builder: ConfirmPrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPrivateVirtualInterfaceOutput, SdkError<ConfirmPrivateVirtualInterfaceError>>> + Send;
    fn confirm_public_virtual_interface(&self, builder: ConfirmPublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPublicVirtualInterfaceOutput, SdkError<ConfirmPublicVirtualInterfaceError>>> + Send;
    fn confirm_transit_virtual_interface(&self, builder: ConfirmTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmTransitVirtualInterfaceOutput, SdkError<ConfirmTransitVirtualInterfaceError>>> + Send;
    fn create_bgp_peer(&self, builder: CreateBgpPeerInputBuilder) -> impl Future<Output = Result<CreateBgpPeerOutput, SdkError<CreateBGPPeerError>>> + Send;
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> + Send;
    fn create_direct_connect_gateway(&self, builder: CreateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayOutput, SdkError<CreateDirectConnectGatewayError>>> + Send;
    fn create_direct_connect_gateway_association(&self, builder: CreateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationOutput, SdkError<CreateDirectConnectGatewayAssociationError>>> + Send;
    fn create_direct_connect_gateway_association_proposal(&self, builder: CreateDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationProposalOutput, SdkError<CreateDirectConnectGatewayAssociationProposalError>>> + Send;
    fn create_interconnect(&self, builder: CreateInterconnectInputBuilder) -> impl Future<Output = Result<CreateInterconnectOutput, SdkError<CreateInterconnectError>>> + Send;
    fn create_lag(&self, builder: CreateLagInputBuilder) -> impl Future<Output = Result<CreateLagOutput, SdkError<CreateLagError>>> + Send;
    fn create_private_virtual_interface(&self, builder: CreatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePrivateVirtualInterfaceOutput, SdkError<CreatePrivateVirtualInterfaceError>>> + Send;
    fn create_public_virtual_interface(&self, builder: CreatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePublicVirtualInterfaceOutput, SdkError<CreatePublicVirtualInterfaceError>>> + Send;
    fn create_transit_virtual_interface(&self, builder: CreateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreateTransitVirtualInterfaceOutput, SdkError<CreateTransitVirtualInterfaceError>>> + Send;
    fn delete_bgp_peer(&self, builder: DeleteBgpPeerInputBuilder) -> impl Future<Output = Result<DeleteBgpPeerOutput, SdkError<DeleteBGPPeerError>>> + Send;
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> + Send;
    fn delete_direct_connect_gateway(&self, builder: DeleteDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayOutput, SdkError<DeleteDirectConnectGatewayError>>> + Send;
    fn delete_direct_connect_gateway_association(&self, builder: DeleteDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationOutput, SdkError<DeleteDirectConnectGatewayAssociationError>>> + Send;
    fn delete_direct_connect_gateway_association_proposal(&self, builder: DeleteDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationProposalOutput, SdkError<DeleteDirectConnectGatewayAssociationProposalError>>> + Send;
    fn delete_interconnect(&self, builder: DeleteInterconnectInputBuilder) -> impl Future<Output = Result<DeleteInterconnectOutput, SdkError<DeleteInterconnectError>>> + Send;
    fn delete_lag(&self, builder: DeleteLagInputBuilder) -> impl Future<Output = Result<DeleteLagOutput, SdkError<DeleteLagError>>> + Send;
    fn delete_virtual_interface(&self, builder: DeleteVirtualInterfaceInputBuilder) -> impl Future<Output = Result<DeleteVirtualInterfaceOutput, SdkError<DeleteVirtualInterfaceError>>> + Send;
    fn describe_connections(&self, builder: DescribeConnectionsInputBuilder) -> impl Future<Output = Result<DescribeConnectionsOutput, SdkError<DescribeConnectionsError>>> + Send;
    fn describe_customer_metadata(&self, builder: DescribeCustomerMetadataInputBuilder) -> impl Future<Output = Result<DescribeCustomerMetadataOutput, SdkError<DescribeCustomerMetadataError>>> + Send;
    fn describe_direct_connect_gateway_association_proposals(&self, builder: DescribeDirectConnectGatewayAssociationProposalsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationProposalsOutput, SdkError<DescribeDirectConnectGatewayAssociationProposalsError>>> + Send;
    fn describe_direct_connect_gateway_associations(&self, builder: DescribeDirectConnectGatewayAssociationsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationsOutput, SdkError<DescribeDirectConnectGatewayAssociationsError>>> + Send;
    fn describe_direct_connect_gateway_attachments(&self, builder: DescribeDirectConnectGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAttachmentsOutput, SdkError<DescribeDirectConnectGatewayAttachmentsError>>> + Send;
    fn describe_direct_connect_gateways(&self, builder: DescribeDirectConnectGatewaysInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewaysOutput, SdkError<DescribeDirectConnectGatewaysError>>> + Send;
    fn describe_hosted_connections(&self, builder: DescribeHostedConnectionsInputBuilder) -> impl Future<Output = Result<DescribeHostedConnectionsOutput, SdkError<DescribeHostedConnectionsError>>> + Send;
    fn describe_interconnects(&self, builder: DescribeInterconnectsInputBuilder) -> impl Future<Output = Result<DescribeInterconnectsOutput, SdkError<DescribeInterconnectsError>>> + Send;
    fn describe_lags(&self, builder: DescribeLagsInputBuilder) -> impl Future<Output = Result<DescribeLagsOutput, SdkError<DescribeLagsError>>> + Send;
    fn describe_loa(&self, builder: DescribeLoaInputBuilder) -> impl Future<Output = Result<DescribeLoaOutput, SdkError<DescribeLoaError>>> + Send;
    fn describe_locations(&self, builder: DescribeLocationsInputBuilder) -> impl Future<Output = Result<DescribeLocationsOutput, SdkError<DescribeLocationsError>>> + Send;
    fn describe_router_configuration(&self, builder: DescribeRouterConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRouterConfigurationOutput, SdkError<DescribeRouterConfigurationError>>> + Send;
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> + Send;
    fn describe_virtual_gateways(&self, builder: DescribeVirtualGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewaysOutput, SdkError<DescribeVirtualGatewaysError>>> + Send;
    fn describe_virtual_interfaces(&self, builder: DescribeVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeVirtualInterfacesOutput, SdkError<DescribeVirtualInterfacesError>>> + Send;
    fn disassociate_connection_from_lag(&self, builder: DisassociateConnectionFromLagInputBuilder) -> impl Future<Output = Result<DisassociateConnectionFromLagOutput, SdkError<DisassociateConnectionFromLagError>>> + Send;
    fn disassociate_mac_sec_key(&self, builder: DisassociateMacSecKeyInputBuilder) -> impl Future<Output = Result<DisassociateMacSecKeyOutput, SdkError<DisassociateMacSecKeyError>>> + Send;
    fn list_virtual_interface_test_history(&self, builder: ListVirtualInterfaceTestHistoryInputBuilder) -> impl Future<Output = Result<ListVirtualInterfaceTestHistoryOutput, SdkError<ListVirtualInterfaceTestHistoryError>>> + Send;
    fn start_bgp_failover_test(&self, builder: StartBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StartBgpFailoverTestOutput, SdkError<StartBgpFailoverTestError>>> + Send;
    fn stop_bgp_failover_test(&self, builder: StopBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StopBgpFailoverTestOutput, SdkError<StopBgpFailoverTestError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>> + Send;
    fn update_direct_connect_gateway(&self, builder: UpdateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayOutput, SdkError<UpdateDirectConnectGatewayError>>> + Send;
    fn update_direct_connect_gateway_association(&self, builder: UpdateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayAssociationOutput, SdkError<UpdateDirectConnectGatewayAssociationError>>> + Send;
    fn update_lag(&self, builder: UpdateLagInputBuilder) -> impl Future<Output = Result<UpdateLagOutput, SdkError<UpdateLagError>>> + Send;
    fn update_virtual_interface_attributes(&self, builder: UpdateVirtualInterfaceAttributesInputBuilder) -> impl Future<Output = Result<UpdateVirtualInterfaceAttributesOutput, SdkError<UpdateVirtualInterfaceAttributesError>>> + Send;
}
impl DirectConnectClient for DirectConnectClientImpl {
    fn accept_direct_connect_gateway_association_proposal(&self, builder: AcceptDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<AcceptDirectConnectGatewayAssociationProposalOutput, SdkError<AcceptDirectConnectGatewayAssociationProposalError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_hosted_connection(&self, builder: AllocateHostedConnectionInputBuilder) -> impl Future<Output = Result<AllocateHostedConnectionOutput, SdkError<AllocateHostedConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_private_virtual_interface(&self, builder: AllocatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePrivateVirtualInterfaceOutput, SdkError<AllocatePrivateVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_public_virtual_interface(&self, builder: AllocatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePublicVirtualInterfaceOutput, SdkError<AllocatePublicVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_transit_virtual_interface(&self, builder: AllocateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocateTransitVirtualInterfaceOutput, SdkError<AllocateTransitVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn associate_connection_with_lag(&self, builder: AssociateConnectionWithLagInputBuilder) -> impl Future<Output = Result<AssociateConnectionWithLagOutput, SdkError<AssociateConnectionWithLagError>>> {
        builder.send_with(&self.0)
    }
    fn associate_hosted_connection(&self, builder: AssociateHostedConnectionInputBuilder) -> impl Future<Output = Result<AssociateHostedConnectionOutput, SdkError<AssociateHostedConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn associate_mac_sec_key(&self, builder: AssociateMacSecKeyInputBuilder) -> impl Future<Output = Result<AssociateMacSecKeyOutput, SdkError<AssociateMacSecKeyError>>> {
        builder.send_with(&self.0)
    }
    fn associate_virtual_interface(&self, builder: AssociateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AssociateVirtualInterfaceOutput, SdkError<AssociateVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_connection(&self, builder: ConfirmConnectionInputBuilder) -> impl Future<Output = Result<ConfirmConnectionOutput, SdkError<ConfirmConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_customer_agreement(&self, builder: ConfirmCustomerAgreementInputBuilder) -> impl Future<Output = Result<ConfirmCustomerAgreementOutput, SdkError<ConfirmCustomerAgreementError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_private_virtual_interface(&self, builder: ConfirmPrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPrivateVirtualInterfaceOutput, SdkError<ConfirmPrivateVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_public_virtual_interface(&self, builder: ConfirmPublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPublicVirtualInterfaceOutput, SdkError<ConfirmPublicVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_transit_virtual_interface(&self, builder: ConfirmTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmTransitVirtualInterfaceOutput, SdkError<ConfirmTransitVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_bgp_peer(&self, builder: CreateBgpPeerInputBuilder) -> impl Future<Output = Result<CreateBgpPeerOutput, SdkError<CreateBGPPeerError>>> {
        builder.send_with(&self.0)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_direct_connect_gateway(&self, builder: CreateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayOutput, SdkError<CreateDirectConnectGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_direct_connect_gateway_association(&self, builder: CreateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationOutput, SdkError<CreateDirectConnectGatewayAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_direct_connect_gateway_association_proposal(&self, builder: CreateDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationProposalOutput, SdkError<CreateDirectConnectGatewayAssociationProposalError>>> {
        builder.send_with(&self.0)
    }
    fn create_interconnect(&self, builder: CreateInterconnectInputBuilder) -> impl Future<Output = Result<CreateInterconnectOutput, SdkError<CreateInterconnectError>>> {
        builder.send_with(&self.0)
    }
    fn create_lag(&self, builder: CreateLagInputBuilder) -> impl Future<Output = Result<CreateLagOutput, SdkError<CreateLagError>>> {
        builder.send_with(&self.0)
    }
    fn create_private_virtual_interface(&self, builder: CreatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePrivateVirtualInterfaceOutput, SdkError<CreatePrivateVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_public_virtual_interface(&self, builder: CreatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePublicVirtualInterfaceOutput, SdkError<CreatePublicVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_virtual_interface(&self, builder: CreateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreateTransitVirtualInterfaceOutput, SdkError<CreateTransitVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_bgp_peer(&self, builder: DeleteBgpPeerInputBuilder) -> impl Future<Output = Result<DeleteBgpPeerOutput, SdkError<DeleteBGPPeerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_direct_connect_gateway(&self, builder: DeleteDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayOutput, SdkError<DeleteDirectConnectGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_direct_connect_gateway_association(&self, builder: DeleteDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationOutput, SdkError<DeleteDirectConnectGatewayAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_direct_connect_gateway_association_proposal(&self, builder: DeleteDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationProposalOutput, SdkError<DeleteDirectConnectGatewayAssociationProposalError>>> {
        builder.send_with(&self.0)
    }
    fn delete_interconnect(&self, builder: DeleteInterconnectInputBuilder) -> impl Future<Output = Result<DeleteInterconnectOutput, SdkError<DeleteInterconnectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_lag(&self, builder: DeleteLagInputBuilder) -> impl Future<Output = Result<DeleteLagOutput, SdkError<DeleteLagError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_interface(&self, builder: DeleteVirtualInterfaceInputBuilder) -> impl Future<Output = Result<DeleteVirtualInterfaceOutput, SdkError<DeleteVirtualInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_connections(&self, builder: DescribeConnectionsInputBuilder) -> impl Future<Output = Result<DescribeConnectionsOutput, SdkError<DescribeConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_customer_metadata(&self, builder: DescribeCustomerMetadataInputBuilder) -> impl Future<Output = Result<DescribeCustomerMetadataOutput, SdkError<DescribeCustomerMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn describe_direct_connect_gateway_association_proposals(&self, builder: DescribeDirectConnectGatewayAssociationProposalsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationProposalsOutput, SdkError<DescribeDirectConnectGatewayAssociationProposalsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_direct_connect_gateway_associations(&self, builder: DescribeDirectConnectGatewayAssociationsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationsOutput, SdkError<DescribeDirectConnectGatewayAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_direct_connect_gateway_attachments(&self, builder: DescribeDirectConnectGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAttachmentsOutput, SdkError<DescribeDirectConnectGatewayAttachmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_direct_connect_gateways(&self, builder: DescribeDirectConnectGatewaysInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewaysOutput, SdkError<DescribeDirectConnectGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hosted_connections(&self, builder: DescribeHostedConnectionsInputBuilder) -> impl Future<Output = Result<DescribeHostedConnectionsOutput, SdkError<DescribeHostedConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_interconnects(&self, builder: DescribeInterconnectsInputBuilder) -> impl Future<Output = Result<DescribeInterconnectsOutput, SdkError<DescribeInterconnectsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_lags(&self, builder: DescribeLagsInputBuilder) -> impl Future<Output = Result<DescribeLagsOutput, SdkError<DescribeLagsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_loa(&self, builder: DescribeLoaInputBuilder) -> impl Future<Output = Result<DescribeLoaOutput, SdkError<DescribeLoaError>>> {
        builder.send_with(&self.0)
    }
    fn describe_locations(&self, builder: DescribeLocationsInputBuilder) -> impl Future<Output = Result<DescribeLocationsOutput, SdkError<DescribeLocationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_router_configuration(&self, builder: DescribeRouterConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRouterConfigurationOutput, SdkError<DescribeRouterConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_gateways(&self, builder: DescribeVirtualGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewaysOutput, SdkError<DescribeVirtualGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_interfaces(&self, builder: DescribeVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeVirtualInterfacesOutput, SdkError<DescribeVirtualInterfacesError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_connection_from_lag(&self, builder: DisassociateConnectionFromLagInputBuilder) -> impl Future<Output = Result<DisassociateConnectionFromLagOutput, SdkError<DisassociateConnectionFromLagError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_mac_sec_key(&self, builder: DisassociateMacSecKeyInputBuilder) -> impl Future<Output = Result<DisassociateMacSecKeyOutput, SdkError<DisassociateMacSecKeyError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_interface_test_history(&self, builder: ListVirtualInterfaceTestHistoryInputBuilder) -> impl Future<Output = Result<ListVirtualInterfaceTestHistoryOutput, SdkError<ListVirtualInterfaceTestHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn start_bgp_failover_test(&self, builder: StartBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StartBgpFailoverTestOutput, SdkError<StartBgpFailoverTestError>>> {
        builder.send_with(&self.0)
    }
    fn stop_bgp_failover_test(&self, builder: StopBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StopBgpFailoverTestOutput, SdkError<StopBgpFailoverTestError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn update_direct_connect_gateway(&self, builder: UpdateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayOutput, SdkError<UpdateDirectConnectGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn update_direct_connect_gateway_association(&self, builder: UpdateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayAssociationOutput, SdkError<UpdateDirectConnectGatewayAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn update_lag(&self, builder: UpdateLagInputBuilder) -> impl Future<Output = Result<UpdateLagOutput, SdkError<UpdateLagError>>> {
        builder.send_with(&self.0)
    }
    fn update_virtual_interface_attributes(&self, builder: UpdateVirtualInterfaceAttributesInputBuilder) -> impl Future<Output = Result<UpdateVirtualInterfaceAttributesOutput, SdkError<UpdateVirtualInterfaceAttributesError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> DirectConnectClient for T
where T: Deref,
      T::Target: DirectConnectClient {
    fn accept_direct_connect_gateway_association_proposal(&self, builder: AcceptDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<AcceptDirectConnectGatewayAssociationProposalOutput, SdkError<AcceptDirectConnectGatewayAssociationProposalError>>> {
        self.deref().accept_direct_connect_gateway_association_proposal(builder)
    }
    fn allocate_hosted_connection(&self, builder: AllocateHostedConnectionInputBuilder) -> impl Future<Output = Result<AllocateHostedConnectionOutput, SdkError<AllocateHostedConnectionError>>> {
        self.deref().allocate_hosted_connection(builder)
    }
    fn allocate_private_virtual_interface(&self, builder: AllocatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePrivateVirtualInterfaceOutput, SdkError<AllocatePrivateVirtualInterfaceError>>> {
        self.deref().allocate_private_virtual_interface(builder)
    }
    fn allocate_public_virtual_interface(&self, builder: AllocatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocatePublicVirtualInterfaceOutput, SdkError<AllocatePublicVirtualInterfaceError>>> {
        self.deref().allocate_public_virtual_interface(builder)
    }
    fn allocate_transit_virtual_interface(&self, builder: AllocateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AllocateTransitVirtualInterfaceOutput, SdkError<AllocateTransitVirtualInterfaceError>>> {
        self.deref().allocate_transit_virtual_interface(builder)
    }
    fn associate_connection_with_lag(&self, builder: AssociateConnectionWithLagInputBuilder) -> impl Future<Output = Result<AssociateConnectionWithLagOutput, SdkError<AssociateConnectionWithLagError>>> {
        self.deref().associate_connection_with_lag(builder)
    }
    fn associate_hosted_connection(&self, builder: AssociateHostedConnectionInputBuilder) -> impl Future<Output = Result<AssociateHostedConnectionOutput, SdkError<AssociateHostedConnectionError>>> {
        self.deref().associate_hosted_connection(builder)
    }
    fn associate_mac_sec_key(&self, builder: AssociateMacSecKeyInputBuilder) -> impl Future<Output = Result<AssociateMacSecKeyOutput, SdkError<AssociateMacSecKeyError>>> {
        self.deref().associate_mac_sec_key(builder)
    }
    fn associate_virtual_interface(&self, builder: AssociateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<AssociateVirtualInterfaceOutput, SdkError<AssociateVirtualInterfaceError>>> {
        self.deref().associate_virtual_interface(builder)
    }
    fn confirm_connection(&self, builder: ConfirmConnectionInputBuilder) -> impl Future<Output = Result<ConfirmConnectionOutput, SdkError<ConfirmConnectionError>>> {
        self.deref().confirm_connection(builder)
    }
    fn confirm_customer_agreement(&self, builder: ConfirmCustomerAgreementInputBuilder) -> impl Future<Output = Result<ConfirmCustomerAgreementOutput, SdkError<ConfirmCustomerAgreementError>>> {
        self.deref().confirm_customer_agreement(builder)
    }
    fn confirm_private_virtual_interface(&self, builder: ConfirmPrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPrivateVirtualInterfaceOutput, SdkError<ConfirmPrivateVirtualInterfaceError>>> {
        self.deref().confirm_private_virtual_interface(builder)
    }
    fn confirm_public_virtual_interface(&self, builder: ConfirmPublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmPublicVirtualInterfaceOutput, SdkError<ConfirmPublicVirtualInterfaceError>>> {
        self.deref().confirm_public_virtual_interface(builder)
    }
    fn confirm_transit_virtual_interface(&self, builder: ConfirmTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<ConfirmTransitVirtualInterfaceOutput, SdkError<ConfirmTransitVirtualInterfaceError>>> {
        self.deref().confirm_transit_virtual_interface(builder)
    }
    fn create_bgp_peer(&self, builder: CreateBgpPeerInputBuilder) -> impl Future<Output = Result<CreateBgpPeerOutput, SdkError<CreateBGPPeerError>>> {
        self.deref().create_bgp_peer(builder)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        self.deref().create_connection(builder)
    }
    fn create_direct_connect_gateway(&self, builder: CreateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayOutput, SdkError<CreateDirectConnectGatewayError>>> {
        self.deref().create_direct_connect_gateway(builder)
    }
    fn create_direct_connect_gateway_association(&self, builder: CreateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationOutput, SdkError<CreateDirectConnectGatewayAssociationError>>> {
        self.deref().create_direct_connect_gateway_association(builder)
    }
    fn create_direct_connect_gateway_association_proposal(&self, builder: CreateDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<CreateDirectConnectGatewayAssociationProposalOutput, SdkError<CreateDirectConnectGatewayAssociationProposalError>>> {
        self.deref().create_direct_connect_gateway_association_proposal(builder)
    }
    fn create_interconnect(&self, builder: CreateInterconnectInputBuilder) -> impl Future<Output = Result<CreateInterconnectOutput, SdkError<CreateInterconnectError>>> {
        self.deref().create_interconnect(builder)
    }
    fn create_lag(&self, builder: CreateLagInputBuilder) -> impl Future<Output = Result<CreateLagOutput, SdkError<CreateLagError>>> {
        self.deref().create_lag(builder)
    }
    fn create_private_virtual_interface(&self, builder: CreatePrivateVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePrivateVirtualInterfaceOutput, SdkError<CreatePrivateVirtualInterfaceError>>> {
        self.deref().create_private_virtual_interface(builder)
    }
    fn create_public_virtual_interface(&self, builder: CreatePublicVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreatePublicVirtualInterfaceOutput, SdkError<CreatePublicVirtualInterfaceError>>> {
        self.deref().create_public_virtual_interface(builder)
    }
    fn create_transit_virtual_interface(&self, builder: CreateTransitVirtualInterfaceInputBuilder) -> impl Future<Output = Result<CreateTransitVirtualInterfaceOutput, SdkError<CreateTransitVirtualInterfaceError>>> {
        self.deref().create_transit_virtual_interface(builder)
    }
    fn delete_bgp_peer(&self, builder: DeleteBgpPeerInputBuilder) -> impl Future<Output = Result<DeleteBgpPeerOutput, SdkError<DeleteBGPPeerError>>> {
        self.deref().delete_bgp_peer(builder)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        self.deref().delete_connection(builder)
    }
    fn delete_direct_connect_gateway(&self, builder: DeleteDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayOutput, SdkError<DeleteDirectConnectGatewayError>>> {
        self.deref().delete_direct_connect_gateway(builder)
    }
    fn delete_direct_connect_gateway_association(&self, builder: DeleteDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationOutput, SdkError<DeleteDirectConnectGatewayAssociationError>>> {
        self.deref().delete_direct_connect_gateway_association(builder)
    }
    fn delete_direct_connect_gateway_association_proposal(&self, builder: DeleteDirectConnectGatewayAssociationProposalInputBuilder) -> impl Future<Output = Result<DeleteDirectConnectGatewayAssociationProposalOutput, SdkError<DeleteDirectConnectGatewayAssociationProposalError>>> {
        self.deref().delete_direct_connect_gateway_association_proposal(builder)
    }
    fn delete_interconnect(&self, builder: DeleteInterconnectInputBuilder) -> impl Future<Output = Result<DeleteInterconnectOutput, SdkError<DeleteInterconnectError>>> {
        self.deref().delete_interconnect(builder)
    }
    fn delete_lag(&self, builder: DeleteLagInputBuilder) -> impl Future<Output = Result<DeleteLagOutput, SdkError<DeleteLagError>>> {
        self.deref().delete_lag(builder)
    }
    fn delete_virtual_interface(&self, builder: DeleteVirtualInterfaceInputBuilder) -> impl Future<Output = Result<DeleteVirtualInterfaceOutput, SdkError<DeleteVirtualInterfaceError>>> {
        self.deref().delete_virtual_interface(builder)
    }
    fn describe_connections(&self, builder: DescribeConnectionsInputBuilder) -> impl Future<Output = Result<DescribeConnectionsOutput, SdkError<DescribeConnectionsError>>> {
        self.deref().describe_connections(builder)
    }
    fn describe_customer_metadata(&self, builder: DescribeCustomerMetadataInputBuilder) -> impl Future<Output = Result<DescribeCustomerMetadataOutput, SdkError<DescribeCustomerMetadataError>>> {
        self.deref().describe_customer_metadata(builder)
    }
    fn describe_direct_connect_gateway_association_proposals(&self, builder: DescribeDirectConnectGatewayAssociationProposalsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationProposalsOutput, SdkError<DescribeDirectConnectGatewayAssociationProposalsError>>> {
        self.deref().describe_direct_connect_gateway_association_proposals(builder)
    }
    fn describe_direct_connect_gateway_associations(&self, builder: DescribeDirectConnectGatewayAssociationsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAssociationsOutput, SdkError<DescribeDirectConnectGatewayAssociationsError>>> {
        self.deref().describe_direct_connect_gateway_associations(builder)
    }
    fn describe_direct_connect_gateway_attachments(&self, builder: DescribeDirectConnectGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewayAttachmentsOutput, SdkError<DescribeDirectConnectGatewayAttachmentsError>>> {
        self.deref().describe_direct_connect_gateway_attachments(builder)
    }
    fn describe_direct_connect_gateways(&self, builder: DescribeDirectConnectGatewaysInputBuilder) -> impl Future<Output = Result<DescribeDirectConnectGatewaysOutput, SdkError<DescribeDirectConnectGatewaysError>>> {
        self.deref().describe_direct_connect_gateways(builder)
    }
    fn describe_hosted_connections(&self, builder: DescribeHostedConnectionsInputBuilder) -> impl Future<Output = Result<DescribeHostedConnectionsOutput, SdkError<DescribeHostedConnectionsError>>> {
        self.deref().describe_hosted_connections(builder)
    }
    fn describe_interconnects(&self, builder: DescribeInterconnectsInputBuilder) -> impl Future<Output = Result<DescribeInterconnectsOutput, SdkError<DescribeInterconnectsError>>> {
        self.deref().describe_interconnects(builder)
    }
    fn describe_lags(&self, builder: DescribeLagsInputBuilder) -> impl Future<Output = Result<DescribeLagsOutput, SdkError<DescribeLagsError>>> {
        self.deref().describe_lags(builder)
    }
    fn describe_loa(&self, builder: DescribeLoaInputBuilder) -> impl Future<Output = Result<DescribeLoaOutput, SdkError<DescribeLoaError>>> {
        self.deref().describe_loa(builder)
    }
    fn describe_locations(&self, builder: DescribeLocationsInputBuilder) -> impl Future<Output = Result<DescribeLocationsOutput, SdkError<DescribeLocationsError>>> {
        self.deref().describe_locations(builder)
    }
    fn describe_router_configuration(&self, builder: DescribeRouterConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRouterConfigurationOutput, SdkError<DescribeRouterConfigurationError>>> {
        self.deref().describe_router_configuration(builder)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        self.deref().describe_tags(builder)
    }
    fn describe_virtual_gateways(&self, builder: DescribeVirtualGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewaysOutput, SdkError<DescribeVirtualGatewaysError>>> {
        self.deref().describe_virtual_gateways(builder)
    }
    fn describe_virtual_interfaces(&self, builder: DescribeVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeVirtualInterfacesOutput, SdkError<DescribeVirtualInterfacesError>>> {
        self.deref().describe_virtual_interfaces(builder)
    }
    fn disassociate_connection_from_lag(&self, builder: DisassociateConnectionFromLagInputBuilder) -> impl Future<Output = Result<DisassociateConnectionFromLagOutput, SdkError<DisassociateConnectionFromLagError>>> {
        self.deref().disassociate_connection_from_lag(builder)
    }
    fn disassociate_mac_sec_key(&self, builder: DisassociateMacSecKeyInputBuilder) -> impl Future<Output = Result<DisassociateMacSecKeyOutput, SdkError<DisassociateMacSecKeyError>>> {
        self.deref().disassociate_mac_sec_key(builder)
    }
    fn list_virtual_interface_test_history(&self, builder: ListVirtualInterfaceTestHistoryInputBuilder) -> impl Future<Output = Result<ListVirtualInterfaceTestHistoryOutput, SdkError<ListVirtualInterfaceTestHistoryError>>> {
        self.deref().list_virtual_interface_test_history(builder)
    }
    fn start_bgp_failover_test(&self, builder: StartBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StartBgpFailoverTestOutput, SdkError<StartBgpFailoverTestError>>> {
        self.deref().start_bgp_failover_test(builder)
    }
    fn stop_bgp_failover_test(&self, builder: StopBgpFailoverTestInputBuilder) -> impl Future<Output = Result<StopBgpFailoverTestOutput, SdkError<StopBgpFailoverTestError>>> {
        self.deref().stop_bgp_failover_test(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> impl Future<Output = Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>> {
        self.deref().update_connection(builder)
    }
    fn update_direct_connect_gateway(&self, builder: UpdateDirectConnectGatewayInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayOutput, SdkError<UpdateDirectConnectGatewayError>>> {
        self.deref().update_direct_connect_gateway(builder)
    }
    fn update_direct_connect_gateway_association(&self, builder: UpdateDirectConnectGatewayAssociationInputBuilder) -> impl Future<Output = Result<UpdateDirectConnectGatewayAssociationOutput, SdkError<UpdateDirectConnectGatewayAssociationError>>> {
        self.deref().update_direct_connect_gateway_association(builder)
    }
    fn update_lag(&self, builder: UpdateLagInputBuilder) -> impl Future<Output = Result<UpdateLagOutput, SdkError<UpdateLagError>>> {
        self.deref().update_lag(builder)
    }
    fn update_virtual_interface_attributes(&self, builder: UpdateVirtualInterfaceAttributesInputBuilder) -> impl Future<Output = Result<UpdateVirtualInterfaceAttributesOutput, SdkError<UpdateVirtualInterfaceAttributesError>>> {
        self.deref().update_virtual_interface_attributes(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edDirectConnectClient {}
    impl DirectConnectClient for edDirectConnectClient {
        async fn accept_direct_connect_gateway_association_proposal(&self, builder: AcceptDirectConnectGatewayAssociationProposalInputBuilder) -> Result<AcceptDirectConnectGatewayAssociationProposalOutput, SdkError<AcceptDirectConnectGatewayAssociationProposalError>>;
        async fn allocate_hosted_connection(&self, builder: AllocateHostedConnectionInputBuilder) -> Result<AllocateHostedConnectionOutput, SdkError<AllocateHostedConnectionError>>;
        async fn allocate_private_virtual_interface(&self, builder: AllocatePrivateVirtualInterfaceInputBuilder) -> Result<AllocatePrivateVirtualInterfaceOutput, SdkError<AllocatePrivateVirtualInterfaceError>>;
        async fn allocate_public_virtual_interface(&self, builder: AllocatePublicVirtualInterfaceInputBuilder) -> Result<AllocatePublicVirtualInterfaceOutput, SdkError<AllocatePublicVirtualInterfaceError>>;
        async fn allocate_transit_virtual_interface(&self, builder: AllocateTransitVirtualInterfaceInputBuilder) -> Result<AllocateTransitVirtualInterfaceOutput, SdkError<AllocateTransitVirtualInterfaceError>>;
        async fn associate_connection_with_lag(&self, builder: AssociateConnectionWithLagInputBuilder) -> Result<AssociateConnectionWithLagOutput, SdkError<AssociateConnectionWithLagError>>;
        async fn associate_hosted_connection(&self, builder: AssociateHostedConnectionInputBuilder) -> Result<AssociateHostedConnectionOutput, SdkError<AssociateHostedConnectionError>>;
        async fn associate_mac_sec_key(&self, builder: AssociateMacSecKeyInputBuilder) -> Result<AssociateMacSecKeyOutput, SdkError<AssociateMacSecKeyError>>;
        async fn associate_virtual_interface(&self, builder: AssociateVirtualInterfaceInputBuilder) -> Result<AssociateVirtualInterfaceOutput, SdkError<AssociateVirtualInterfaceError>>;
        async fn confirm_connection(&self, builder: ConfirmConnectionInputBuilder) -> Result<ConfirmConnectionOutput, SdkError<ConfirmConnectionError>>;
        async fn confirm_customer_agreement(&self, builder: ConfirmCustomerAgreementInputBuilder) -> Result<ConfirmCustomerAgreementOutput, SdkError<ConfirmCustomerAgreementError>>;
        async fn confirm_private_virtual_interface(&self, builder: ConfirmPrivateVirtualInterfaceInputBuilder) -> Result<ConfirmPrivateVirtualInterfaceOutput, SdkError<ConfirmPrivateVirtualInterfaceError>>;
        async fn confirm_public_virtual_interface(&self, builder: ConfirmPublicVirtualInterfaceInputBuilder) -> Result<ConfirmPublicVirtualInterfaceOutput, SdkError<ConfirmPublicVirtualInterfaceError>>;
        async fn confirm_transit_virtual_interface(&self, builder: ConfirmTransitVirtualInterfaceInputBuilder) -> Result<ConfirmTransitVirtualInterfaceOutput, SdkError<ConfirmTransitVirtualInterfaceError>>;
        async fn create_bgp_peer(&self, builder: CreateBgpPeerInputBuilder) -> Result<CreateBgpPeerOutput, SdkError<CreateBGPPeerError>>;
        async fn create_connection(&self, builder: CreateConnectionInputBuilder) -> Result<CreateConnectionOutput, SdkError<CreateConnectionError>>;
        async fn create_direct_connect_gateway(&self, builder: CreateDirectConnectGatewayInputBuilder) -> Result<CreateDirectConnectGatewayOutput, SdkError<CreateDirectConnectGatewayError>>;
        async fn create_direct_connect_gateway_association(&self, builder: CreateDirectConnectGatewayAssociationInputBuilder) -> Result<CreateDirectConnectGatewayAssociationOutput, SdkError<CreateDirectConnectGatewayAssociationError>>;
        async fn create_direct_connect_gateway_association_proposal(&self, builder: CreateDirectConnectGatewayAssociationProposalInputBuilder) -> Result<CreateDirectConnectGatewayAssociationProposalOutput, SdkError<CreateDirectConnectGatewayAssociationProposalError>>;
        async fn create_interconnect(&self, builder: CreateInterconnectInputBuilder) -> Result<CreateInterconnectOutput, SdkError<CreateInterconnectError>>;
        async fn create_lag(&self, builder: CreateLagInputBuilder) -> Result<CreateLagOutput, SdkError<CreateLagError>>;
        async fn create_private_virtual_interface(&self, builder: CreatePrivateVirtualInterfaceInputBuilder) -> Result<CreatePrivateVirtualInterfaceOutput, SdkError<CreatePrivateVirtualInterfaceError>>;
        async fn create_public_virtual_interface(&self, builder: CreatePublicVirtualInterfaceInputBuilder) -> Result<CreatePublicVirtualInterfaceOutput, SdkError<CreatePublicVirtualInterfaceError>>;
        async fn create_transit_virtual_interface(&self, builder: CreateTransitVirtualInterfaceInputBuilder) -> Result<CreateTransitVirtualInterfaceOutput, SdkError<CreateTransitVirtualInterfaceError>>;
        async fn delete_bgp_peer(&self, builder: DeleteBgpPeerInputBuilder) -> Result<DeleteBgpPeerOutput, SdkError<DeleteBGPPeerError>>;
        async fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>;
        async fn delete_direct_connect_gateway(&self, builder: DeleteDirectConnectGatewayInputBuilder) -> Result<DeleteDirectConnectGatewayOutput, SdkError<DeleteDirectConnectGatewayError>>;
        async fn delete_direct_connect_gateway_association(&self, builder: DeleteDirectConnectGatewayAssociationInputBuilder) -> Result<DeleteDirectConnectGatewayAssociationOutput, SdkError<DeleteDirectConnectGatewayAssociationError>>;
        async fn delete_direct_connect_gateway_association_proposal(&self, builder: DeleteDirectConnectGatewayAssociationProposalInputBuilder) -> Result<DeleteDirectConnectGatewayAssociationProposalOutput, SdkError<DeleteDirectConnectGatewayAssociationProposalError>>;
        async fn delete_interconnect(&self, builder: DeleteInterconnectInputBuilder) -> Result<DeleteInterconnectOutput, SdkError<DeleteInterconnectError>>;
        async fn delete_lag(&self, builder: DeleteLagInputBuilder) -> Result<DeleteLagOutput, SdkError<DeleteLagError>>;
        async fn delete_virtual_interface(&self, builder: DeleteVirtualInterfaceInputBuilder) -> Result<DeleteVirtualInterfaceOutput, SdkError<DeleteVirtualInterfaceError>>;
        async fn describe_connections(&self, builder: DescribeConnectionsInputBuilder) -> Result<DescribeConnectionsOutput, SdkError<DescribeConnectionsError>>;
        async fn describe_customer_metadata(&self, builder: DescribeCustomerMetadataInputBuilder) -> Result<DescribeCustomerMetadataOutput, SdkError<DescribeCustomerMetadataError>>;
        async fn describe_direct_connect_gateway_association_proposals(&self, builder: DescribeDirectConnectGatewayAssociationProposalsInputBuilder) -> Result<DescribeDirectConnectGatewayAssociationProposalsOutput, SdkError<DescribeDirectConnectGatewayAssociationProposalsError>>;
        async fn describe_direct_connect_gateway_associations(&self, builder: DescribeDirectConnectGatewayAssociationsInputBuilder) -> Result<DescribeDirectConnectGatewayAssociationsOutput, SdkError<DescribeDirectConnectGatewayAssociationsError>>;
        async fn describe_direct_connect_gateway_attachments(&self, builder: DescribeDirectConnectGatewayAttachmentsInputBuilder) -> Result<DescribeDirectConnectGatewayAttachmentsOutput, SdkError<DescribeDirectConnectGatewayAttachmentsError>>;
        async fn describe_direct_connect_gateways(&self, builder: DescribeDirectConnectGatewaysInputBuilder) -> Result<DescribeDirectConnectGatewaysOutput, SdkError<DescribeDirectConnectGatewaysError>>;
        async fn describe_hosted_connections(&self, builder: DescribeHostedConnectionsInputBuilder) -> Result<DescribeHostedConnectionsOutput, SdkError<DescribeHostedConnectionsError>>;
        async fn describe_interconnects(&self, builder: DescribeInterconnectsInputBuilder) -> Result<DescribeInterconnectsOutput, SdkError<DescribeInterconnectsError>>;
        async fn describe_lags(&self, builder: DescribeLagsInputBuilder) -> Result<DescribeLagsOutput, SdkError<DescribeLagsError>>;
        async fn describe_loa(&self, builder: DescribeLoaInputBuilder) -> Result<DescribeLoaOutput, SdkError<DescribeLoaError>>;
        async fn describe_locations(&self, builder: DescribeLocationsInputBuilder) -> Result<DescribeLocationsOutput, SdkError<DescribeLocationsError>>;
        async fn describe_router_configuration(&self, builder: DescribeRouterConfigurationInputBuilder) -> Result<DescribeRouterConfigurationOutput, SdkError<DescribeRouterConfigurationError>>;
        async fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> Result<DescribeTagsOutput, SdkError<DescribeTagsError>>;
        async fn describe_virtual_gateways(&self, builder: DescribeVirtualGatewaysInputBuilder) -> Result<DescribeVirtualGatewaysOutput, SdkError<DescribeVirtualGatewaysError>>;
        async fn describe_virtual_interfaces(&self, builder: DescribeVirtualInterfacesInputBuilder) -> Result<DescribeVirtualInterfacesOutput, SdkError<DescribeVirtualInterfacesError>>;
        async fn disassociate_connection_from_lag(&self, builder: DisassociateConnectionFromLagInputBuilder) -> Result<DisassociateConnectionFromLagOutput, SdkError<DisassociateConnectionFromLagError>>;
        async fn disassociate_mac_sec_key(&self, builder: DisassociateMacSecKeyInputBuilder) -> Result<DisassociateMacSecKeyOutput, SdkError<DisassociateMacSecKeyError>>;
        async fn list_virtual_interface_test_history(&self, builder: ListVirtualInterfaceTestHistoryInputBuilder) -> Result<ListVirtualInterfaceTestHistoryOutput, SdkError<ListVirtualInterfaceTestHistoryError>>;
        async fn start_bgp_failover_test(&self, builder: StartBgpFailoverTestInputBuilder) -> Result<StartBgpFailoverTestOutput, SdkError<StartBgpFailoverTestError>>;
        async fn stop_bgp_failover_test(&self, builder: StopBgpFailoverTestInputBuilder) -> Result<StopBgpFailoverTestOutput, SdkError<StopBgpFailoverTestError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_connection(&self, builder: UpdateConnectionInputBuilder) -> Result<UpdateConnectionOutput, SdkError<UpdateConnectionError>>;
        async fn update_direct_connect_gateway(&self, builder: UpdateDirectConnectGatewayInputBuilder) -> Result<UpdateDirectConnectGatewayOutput, SdkError<UpdateDirectConnectGatewayError>>;
        async fn update_direct_connect_gateway_association(&self, builder: UpdateDirectConnectGatewayAssociationInputBuilder) -> Result<UpdateDirectConnectGatewayAssociationOutput, SdkError<UpdateDirectConnectGatewayAssociationError>>;
        async fn update_lag(&self, builder: UpdateLagInputBuilder) -> Result<UpdateLagOutput, SdkError<UpdateLagError>>;
        async fn update_virtual_interface_attributes(&self, builder: UpdateVirtualInterfaceAttributesInputBuilder) -> Result<UpdateVirtualInterfaceAttributesOutput, SdkError<UpdateVirtualInterfaceAttributesError>>;
    }
}
