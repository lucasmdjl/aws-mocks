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
use aws_sdk_opensearch::operation::accept_inbound_connection::{builders::*, *};
use aws_sdk_opensearch::operation::add_data_source::{builders::*, *};
use aws_sdk_opensearch::operation::add_tags::{builders::*, *};
use aws_sdk_opensearch::operation::associate_package::{builders::*, *};
use aws_sdk_opensearch::operation::authorize_vpc_endpoint_access::{builders::*, *};
use aws_sdk_opensearch::operation::cancel_domain_config_change::{builders::*, *};
use aws_sdk_opensearch::operation::cancel_service_software_update::{builders::*, *};
use aws_sdk_opensearch::operation::create_domain::{builders::*, *};
use aws_sdk_opensearch::operation::create_outbound_connection::{builders::*, *};
use aws_sdk_opensearch::operation::create_package::{builders::*, *};
use aws_sdk_opensearch::operation::create_vpc_endpoint::{builders::*, *};
use aws_sdk_opensearch::operation::delete_data_source::{builders::*, *};
use aws_sdk_opensearch::operation::delete_domain::{builders::*, *};
use aws_sdk_opensearch::operation::delete_inbound_connection::{builders::*, *};
use aws_sdk_opensearch::operation::delete_outbound_connection::{builders::*, *};
use aws_sdk_opensearch::operation::delete_package::{builders::*, *};
use aws_sdk_opensearch::operation::delete_vpc_endpoint::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain_auto_tunes::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain_change_progress::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain_config::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain_health::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domain_nodes::{builders::*, *};
use aws_sdk_opensearch::operation::describe_domains::{builders::*, *};
use aws_sdk_opensearch::operation::describe_dry_run_progress::{builders::*, *};
use aws_sdk_opensearch::operation::describe_inbound_connections::{builders::*, *};
use aws_sdk_opensearch::operation::describe_instance_type_limits::{builders::*, *};
use aws_sdk_opensearch::operation::describe_outbound_connections::{builders::*, *};
use aws_sdk_opensearch::operation::describe_packages::{builders::*, *};
use aws_sdk_opensearch::operation::describe_reserved_instance_offerings::{builders::*, *};
use aws_sdk_opensearch::operation::describe_reserved_instances::{builders::*, *};
use aws_sdk_opensearch::operation::describe_vpc_endpoints::{builders::*, *};
use aws_sdk_opensearch::operation::dissociate_package::{builders::*, *};
use aws_sdk_opensearch::operation::get_compatible_versions::{builders::*, *};
use aws_sdk_opensearch::operation::get_data_source::{builders::*, *};
use aws_sdk_opensearch::operation::get_domain_maintenance_status::{builders::*, *};
use aws_sdk_opensearch::operation::get_package_version_history::{builders::*, *};
use aws_sdk_opensearch::operation::get_upgrade_history::{builders::*, *};
use aws_sdk_opensearch::operation::get_upgrade_status::{builders::*, *};
use aws_sdk_opensearch::operation::list_data_sources::{builders::*, *};
use aws_sdk_opensearch::operation::list_domain_maintenances::{builders::*, *};
use aws_sdk_opensearch::operation::list_domain_names::{builders::*, *};
use aws_sdk_opensearch::operation::list_domains_for_package::{builders::*, *};
use aws_sdk_opensearch::operation::list_instance_type_details::{builders::*, *};
use aws_sdk_opensearch::operation::list_packages_for_domain::{builders::*, *};
use aws_sdk_opensearch::operation::list_scheduled_actions::{builders::*, *};
use aws_sdk_opensearch::operation::list_tags::{builders::*, *};
use aws_sdk_opensearch::operation::list_versions::{builders::*, *};
use aws_sdk_opensearch::operation::list_vpc_endpoint_access::{builders::*, *};
use aws_sdk_opensearch::operation::list_vpc_endpoints::{builders::*, *};
use aws_sdk_opensearch::operation::list_vpc_endpoints_for_domain::{builders::*, *};
use aws_sdk_opensearch::operation::purchase_reserved_instance_offering::{builders::*, *};
use aws_sdk_opensearch::operation::reject_inbound_connection::{builders::*, *};
use aws_sdk_opensearch::operation::remove_tags::{builders::*, *};
use aws_sdk_opensearch::operation::revoke_vpc_endpoint_access::{builders::*, *};
use aws_sdk_opensearch::operation::start_domain_maintenance::{builders::*, *};
use aws_sdk_opensearch::operation::start_service_software_update::{builders::*, *};
use aws_sdk_opensearch::operation::update_data_source::{builders::*, *};
use aws_sdk_opensearch::operation::update_domain_config::{builders::*, *};
use aws_sdk_opensearch::operation::update_package::{builders::*, *};
use aws_sdk_opensearch::operation::update_scheduled_action::{builders::*, *};
use aws_sdk_opensearch::operation::update_vpc_endpoint::{builders::*, *};
use aws_sdk_opensearch::operation::upgrade_domain::{builders::*, *};
use aws_sdk_opensearch::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_opensearch::Client;

pub use aws_sdk_opensearch::*;

pub struct OpenSearchClientImpl(Client);
impl OpenSearchClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait OpenSearchClient {
    fn accept_inbound_connection(&self, builder: AcceptInboundConnectionInputBuilder) -> impl Future<Output = Result<AcceptInboundConnectionOutput, SdkError<AcceptInboundConnectionError>>>;
    fn add_data_source(&self, builder: AddDataSourceInputBuilder) -> impl Future<Output = Result<AddDataSourceOutput, SdkError<AddDataSourceError>>>;
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>>;
    fn associate_package(&self, builder: AssociatePackageInputBuilder) -> impl Future<Output = Result<AssociatePackageOutput, SdkError<AssociatePackageError>>>;
    fn authorize_vpc_endpoint_access(&self, builder: AuthorizeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeVpcEndpointAccessOutput, SdkError<AuthorizeVpcEndpointAccessError>>>;
    fn cancel_domain_config_change(&self, builder: CancelDomainConfigChangeInputBuilder) -> impl Future<Output = Result<CancelDomainConfigChangeOutput, SdkError<CancelDomainConfigChangeError>>>;
    fn cancel_service_software_update(&self, builder: CancelServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<CancelServiceSoftwareUpdateOutput, SdkError<CancelServiceSoftwareUpdateError>>>;
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>>;
    fn create_outbound_connection(&self, builder: CreateOutboundConnectionInputBuilder) -> impl Future<Output = Result<CreateOutboundConnectionOutput, SdkError<CreateOutboundConnectionError>>>;
    fn create_package(&self, builder: CreatePackageInputBuilder) -> impl Future<Output = Result<CreatePackageOutput, SdkError<CreatePackageError>>>;
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>>;
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>>;
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>>;
    fn delete_inbound_connection(&self, builder: DeleteInboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteInboundConnectionOutput, SdkError<DeleteInboundConnectionError>>>;
    fn delete_outbound_connection(&self, builder: DeleteOutboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteOutboundConnectionOutput, SdkError<DeleteOutboundConnectionError>>>;
    fn delete_package(&self, builder: DeletePackageInputBuilder) -> impl Future<Output = Result<DeletePackageOutput, SdkError<DeletePackageError>>>;
    fn delete_vpc_endpoint(&self, builder: DeleteVpcEndpointInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointOutput, SdkError<DeleteVpcEndpointError>>>;
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>>;
    fn describe_domain_auto_tunes(&self, builder: DescribeDomainAutoTunesInputBuilder) -> impl Future<Output = Result<DescribeDomainAutoTunesOutput, SdkError<DescribeDomainAutoTunesError>>>;
    fn describe_domain_change_progress(&self, builder: DescribeDomainChangeProgressInputBuilder) -> impl Future<Output = Result<DescribeDomainChangeProgressOutput, SdkError<DescribeDomainChangeProgressError>>>;
    fn describe_domain_config(&self, builder: DescribeDomainConfigInputBuilder) -> impl Future<Output = Result<DescribeDomainConfigOutput, SdkError<DescribeDomainConfigError>>>;
    fn describe_domain_health(&self, builder: DescribeDomainHealthInputBuilder) -> impl Future<Output = Result<DescribeDomainHealthOutput, SdkError<DescribeDomainHealthError>>>;
    fn describe_domain_nodes(&self, builder: DescribeDomainNodesInputBuilder) -> impl Future<Output = Result<DescribeDomainNodesOutput, SdkError<DescribeDomainNodesError>>>;
    fn describe_domains(&self, builder: DescribeDomainsInputBuilder) -> impl Future<Output = Result<DescribeDomainsOutput, SdkError<DescribeDomainsError>>>;
    fn describe_dry_run_progress(&self, builder: DescribeDryRunProgressInputBuilder) -> impl Future<Output = Result<DescribeDryRunProgressOutput, SdkError<DescribeDryRunProgressError>>>;
    fn describe_inbound_connections(&self, builder: DescribeInboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeInboundConnectionsOutput, SdkError<DescribeInboundConnectionsError>>>;
    fn describe_instance_type_limits(&self, builder: DescribeInstanceTypeLimitsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeLimitsOutput, SdkError<DescribeInstanceTypeLimitsError>>>;
    fn describe_outbound_connections(&self, builder: DescribeOutboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeOutboundConnectionsOutput, SdkError<DescribeOutboundConnectionsError>>>;
    fn describe_packages(&self, builder: DescribePackagesInputBuilder) -> impl Future<Output = Result<DescribePackagesOutput, SdkError<DescribePackagesError>>>;
    fn describe_reserved_instance_offerings(&self, builder: DescribeReservedInstanceOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstanceOfferingsOutput, SdkError<DescribeReservedInstanceOfferingsError>>>;
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>>;
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>>;
    fn dissociate_package(&self, builder: DissociatePackageInputBuilder) -> impl Future<Output = Result<DissociatePackageOutput, SdkError<DissociatePackageError>>>;
    fn get_compatible_versions(&self, builder: GetCompatibleVersionsInputBuilder) -> impl Future<Output = Result<GetCompatibleVersionsOutput, SdkError<GetCompatibleVersionsError>>>;
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>>;
    fn get_domain_maintenance_status(&self, builder: GetDomainMaintenanceStatusInputBuilder) -> impl Future<Output = Result<GetDomainMaintenanceStatusOutput, SdkError<GetDomainMaintenanceStatusError>>>;
    fn get_package_version_history(&self, builder: GetPackageVersionHistoryInputBuilder) -> impl Future<Output = Result<GetPackageVersionHistoryOutput, SdkError<GetPackageVersionHistoryError>>>;
    fn get_upgrade_history(&self, builder: GetUpgradeHistoryInputBuilder) -> impl Future<Output = Result<GetUpgradeHistoryOutput, SdkError<GetUpgradeHistoryError>>>;
    fn get_upgrade_status(&self, builder: GetUpgradeStatusInputBuilder) -> impl Future<Output = Result<GetUpgradeStatusOutput, SdkError<GetUpgradeStatusError>>>;
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>>;
    fn list_domain_maintenances(&self, builder: ListDomainMaintenancesInputBuilder) -> impl Future<Output = Result<ListDomainMaintenancesOutput, SdkError<ListDomainMaintenancesError>>>;
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>>;
    fn list_domains_for_package(&self, builder: ListDomainsForPackageInputBuilder) -> impl Future<Output = Result<ListDomainsForPackageOutput, SdkError<ListDomainsForPackageError>>>;
    fn list_instance_type_details(&self, builder: ListInstanceTypeDetailsInputBuilder) -> impl Future<Output = Result<ListInstanceTypeDetailsOutput, SdkError<ListInstanceTypeDetailsError>>>;
    fn list_packages_for_domain(&self, builder: ListPackagesForDomainInputBuilder) -> impl Future<Output = Result<ListPackagesForDomainOutput, SdkError<ListPackagesForDomainError>>>;
    fn list_scheduled_actions(&self, builder: ListScheduledActionsInputBuilder) -> impl Future<Output = Result<ListScheduledActionsOutput, SdkError<ListScheduledActionsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn list_versions(&self, builder: ListVersionsInputBuilder) -> impl Future<Output = Result<ListVersionsOutput, SdkError<ListVersionsError>>>;
    fn list_vpc_endpoint_access(&self, builder: ListVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<ListVpcEndpointAccessOutput, SdkError<ListVpcEndpointAccessError>>>;
    fn list_vpc_endpoints(&self, builder: ListVpcEndpointsInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsOutput, SdkError<ListVpcEndpointsError>>>;
    fn list_vpc_endpoints_for_domain(&self, builder: ListVpcEndpointsForDomainInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsForDomainOutput, SdkError<ListVpcEndpointsForDomainError>>>;
    fn purchase_reserved_instance_offering(&self, builder: PurchaseReservedInstanceOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstanceOfferingOutput, SdkError<PurchaseReservedInstanceOfferingError>>>;
    fn reject_inbound_connection(&self, builder: RejectInboundConnectionInputBuilder) -> impl Future<Output = Result<RejectInboundConnectionOutput, SdkError<RejectInboundConnectionError>>>;
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>>;
    fn revoke_vpc_endpoint_access(&self, builder: RevokeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeVpcEndpointAccessOutput, SdkError<RevokeVpcEndpointAccessError>>>;
    fn start_domain_maintenance(&self, builder: StartDomainMaintenanceInputBuilder) -> impl Future<Output = Result<StartDomainMaintenanceOutput, SdkError<StartDomainMaintenanceError>>>;
    fn start_service_software_update(&self, builder: StartServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<StartServiceSoftwareUpdateOutput, SdkError<StartServiceSoftwareUpdateError>>>;
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>>;
    fn update_domain_config(&self, builder: UpdateDomainConfigInputBuilder) -> impl Future<Output = Result<UpdateDomainConfigOutput, SdkError<UpdateDomainConfigError>>>;
    fn update_package(&self, builder: UpdatePackageInputBuilder) -> impl Future<Output = Result<UpdatePackageOutput, SdkError<UpdatePackageError>>>;
    fn update_scheduled_action(&self, builder: UpdateScheduledActionInputBuilder) -> impl Future<Output = Result<UpdateScheduledActionOutput, SdkError<UpdateScheduledActionError>>>;
    fn update_vpc_endpoint(&self, builder: UpdateVpcEndpointInputBuilder) -> impl Future<Output = Result<UpdateVpcEndpointOutput, SdkError<UpdateVpcEndpointError>>>;
    fn upgrade_domain(&self, builder: UpgradeDomainInputBuilder) -> impl Future<Output = Result<UpgradeDomainOutput, SdkError<UpgradeDomainError>>>;
}
impl OpenSearchClient for OpenSearchClientImpl {
    fn accept_inbound_connection(&self, builder: AcceptInboundConnectionInputBuilder) -> impl Future<Output = Result<AcceptInboundConnectionOutput, SdkError<AcceptInboundConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn add_data_source(&self, builder: AddDataSourceInputBuilder) -> impl Future<Output = Result<AddDataSourceOutput, SdkError<AddDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        builder.send_with(&self.0)
    }
    fn associate_package(&self, builder: AssociatePackageInputBuilder) -> impl Future<Output = Result<AssociatePackageOutput, SdkError<AssociatePackageError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_vpc_endpoint_access(&self, builder: AuthorizeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeVpcEndpointAccessOutput, SdkError<AuthorizeVpcEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_domain_config_change(&self, builder: CancelDomainConfigChangeInputBuilder) -> impl Future<Output = Result<CancelDomainConfigChangeOutput, SdkError<CancelDomainConfigChangeError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_service_software_update(&self, builder: CancelServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<CancelServiceSoftwareUpdateOutput, SdkError<CancelServiceSoftwareUpdateError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>> {
        builder.send_with(&self.0)
    }
    fn create_outbound_connection(&self, builder: CreateOutboundConnectionInputBuilder) -> impl Future<Output = Result<CreateOutboundConnectionOutput, SdkError<CreateOutboundConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_package(&self, builder: CreatePackageInputBuilder) -> impl Future<Output = Result<CreatePackageOutput, SdkError<CreatePackageError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>> {
        builder.send_with(&self.0)
    }
    fn delete_inbound_connection(&self, builder: DeleteInboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteInboundConnectionOutput, SdkError<DeleteInboundConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_outbound_connection(&self, builder: DeleteOutboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteOutboundConnectionOutput, SdkError<DeleteOutboundConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_package(&self, builder: DeletePackageInputBuilder) -> impl Future<Output = Result<DeletePackageOutput, SdkError<DeletePackageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_endpoint(&self, builder: DeleteVpcEndpointInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointOutput, SdkError<DeleteVpcEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain_auto_tunes(&self, builder: DescribeDomainAutoTunesInputBuilder) -> impl Future<Output = Result<DescribeDomainAutoTunesOutput, SdkError<DescribeDomainAutoTunesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain_change_progress(&self, builder: DescribeDomainChangeProgressInputBuilder) -> impl Future<Output = Result<DescribeDomainChangeProgressOutput, SdkError<DescribeDomainChangeProgressError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain_config(&self, builder: DescribeDomainConfigInputBuilder) -> impl Future<Output = Result<DescribeDomainConfigOutput, SdkError<DescribeDomainConfigError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain_health(&self, builder: DescribeDomainHealthInputBuilder) -> impl Future<Output = Result<DescribeDomainHealthOutput, SdkError<DescribeDomainHealthError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domain_nodes(&self, builder: DescribeDomainNodesInputBuilder) -> impl Future<Output = Result<DescribeDomainNodesOutput, SdkError<DescribeDomainNodesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_domains(&self, builder: DescribeDomainsInputBuilder) -> impl Future<Output = Result<DescribeDomainsOutput, SdkError<DescribeDomainsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dry_run_progress(&self, builder: DescribeDryRunProgressInputBuilder) -> impl Future<Output = Result<DescribeDryRunProgressOutput, SdkError<DescribeDryRunProgressError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inbound_connections(&self, builder: DescribeInboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeInboundConnectionsOutput, SdkError<DescribeInboundConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_type_limits(&self, builder: DescribeInstanceTypeLimitsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeLimitsOutput, SdkError<DescribeInstanceTypeLimitsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_outbound_connections(&self, builder: DescribeOutboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeOutboundConnectionsOutput, SdkError<DescribeOutboundConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_packages(&self, builder: DescribePackagesInputBuilder) -> impl Future<Output = Result<DescribePackagesOutput, SdkError<DescribePackagesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instance_offerings(&self, builder: DescribeReservedInstanceOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstanceOfferingsOutput, SdkError<DescribeReservedInstanceOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn dissociate_package(&self, builder: DissociatePackageInputBuilder) -> impl Future<Output = Result<DissociatePackageOutput, SdkError<DissociatePackageError>>> {
        builder.send_with(&self.0)
    }
    fn get_compatible_versions(&self, builder: GetCompatibleVersionsInputBuilder) -> impl Future<Output = Result<GetCompatibleVersionsOutput, SdkError<GetCompatibleVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_maintenance_status(&self, builder: GetDomainMaintenanceStatusInputBuilder) -> impl Future<Output = Result<GetDomainMaintenanceStatusOutput, SdkError<GetDomainMaintenanceStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_package_version_history(&self, builder: GetPackageVersionHistoryInputBuilder) -> impl Future<Output = Result<GetPackageVersionHistoryOutput, SdkError<GetPackageVersionHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_upgrade_history(&self, builder: GetUpgradeHistoryInputBuilder) -> impl Future<Output = Result<GetUpgradeHistoryOutput, SdkError<GetUpgradeHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_upgrade_status(&self, builder: GetUpgradeStatusInputBuilder) -> impl Future<Output = Result<GetUpgradeStatusOutput, SdkError<GetUpgradeStatusError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domain_maintenances(&self, builder: ListDomainMaintenancesInputBuilder) -> impl Future<Output = Result<ListDomainMaintenancesOutput, SdkError<ListDomainMaintenancesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domains_for_package(&self, builder: ListDomainsForPackageInputBuilder) -> impl Future<Output = Result<ListDomainsForPackageOutput, SdkError<ListDomainsForPackageError>>> {
        builder.send_with(&self.0)
    }
    fn list_instance_type_details(&self, builder: ListInstanceTypeDetailsInputBuilder) -> impl Future<Output = Result<ListInstanceTypeDetailsOutput, SdkError<ListInstanceTypeDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn list_packages_for_domain(&self, builder: ListPackagesForDomainInputBuilder) -> impl Future<Output = Result<ListPackagesForDomainOutput, SdkError<ListPackagesForDomainError>>> {
        builder.send_with(&self.0)
    }
    fn list_scheduled_actions(&self, builder: ListScheduledActionsInputBuilder) -> impl Future<Output = Result<ListScheduledActionsOutput, SdkError<ListScheduledActionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_versions(&self, builder: ListVersionsInputBuilder) -> impl Future<Output = Result<ListVersionsOutput, SdkError<ListVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_endpoint_access(&self, builder: ListVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<ListVpcEndpointAccessOutput, SdkError<ListVpcEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_endpoints(&self, builder: ListVpcEndpointsInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsOutput, SdkError<ListVpcEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_endpoints_for_domain(&self, builder: ListVpcEndpointsForDomainInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsForDomainOutput, SdkError<ListVpcEndpointsForDomainError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_reserved_instance_offering(&self, builder: PurchaseReservedInstanceOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstanceOfferingOutput, SdkError<PurchaseReservedInstanceOfferingError>>> {
        builder.send_with(&self.0)
    }
    fn reject_inbound_connection(&self, builder: RejectInboundConnectionInputBuilder) -> impl Future<Output = Result<RejectInboundConnectionOutput, SdkError<RejectInboundConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_vpc_endpoint_access(&self, builder: RevokeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeVpcEndpointAccessOutput, SdkError<RevokeVpcEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn start_domain_maintenance(&self, builder: StartDomainMaintenanceInputBuilder) -> impl Future<Output = Result<StartDomainMaintenanceOutput, SdkError<StartDomainMaintenanceError>>> {
        builder.send_with(&self.0)
    }
    fn start_service_software_update(&self, builder: StartServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<StartServiceSoftwareUpdateOutput, SdkError<StartServiceSoftwareUpdateError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain_config(&self, builder: UpdateDomainConfigInputBuilder) -> impl Future<Output = Result<UpdateDomainConfigOutput, SdkError<UpdateDomainConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_package(&self, builder: UpdatePackageInputBuilder) -> impl Future<Output = Result<UpdatePackageOutput, SdkError<UpdatePackageError>>> {
        builder.send_with(&self.0)
    }
    fn update_scheduled_action(&self, builder: UpdateScheduledActionInputBuilder) -> impl Future<Output = Result<UpdateScheduledActionOutput, SdkError<UpdateScheduledActionError>>> {
        builder.send_with(&self.0)
    }
    fn update_vpc_endpoint(&self, builder: UpdateVpcEndpointInputBuilder) -> impl Future<Output = Result<UpdateVpcEndpointOutput, SdkError<UpdateVpcEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn upgrade_domain(&self, builder: UpgradeDomainInputBuilder) -> impl Future<Output = Result<UpgradeDomainOutput, SdkError<UpgradeDomainError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: OpenSearchClient> OpenSearchClient for &T {
    fn accept_inbound_connection(&self, builder: AcceptInboundConnectionInputBuilder) -> impl Future<Output = Result<AcceptInboundConnectionOutput, SdkError<AcceptInboundConnectionError>>> {
        (*self).accept_inbound_connection(builder)
    }
    fn add_data_source(&self, builder: AddDataSourceInputBuilder) -> impl Future<Output = Result<AddDataSourceOutput, SdkError<AddDataSourceError>>> {
        (*self).add_data_source(builder)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        (*self).add_tags(builder)
    }
    fn associate_package(&self, builder: AssociatePackageInputBuilder) -> impl Future<Output = Result<AssociatePackageOutput, SdkError<AssociatePackageError>>> {
        (*self).associate_package(builder)
    }
    fn authorize_vpc_endpoint_access(&self, builder: AuthorizeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeVpcEndpointAccessOutput, SdkError<AuthorizeVpcEndpointAccessError>>> {
        (*self).authorize_vpc_endpoint_access(builder)
    }
    fn cancel_domain_config_change(&self, builder: CancelDomainConfigChangeInputBuilder) -> impl Future<Output = Result<CancelDomainConfigChangeOutput, SdkError<CancelDomainConfigChangeError>>> {
        (*self).cancel_domain_config_change(builder)
    }
    fn cancel_service_software_update(&self, builder: CancelServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<CancelServiceSoftwareUpdateOutput, SdkError<CancelServiceSoftwareUpdateError>>> {
        (*self).cancel_service_software_update(builder)
    }
    fn create_domain(&self, builder: CreateDomainInputBuilder) -> impl Future<Output = Result<CreateDomainOutput, SdkError<CreateDomainError>>> {
        (*self).create_domain(builder)
    }
    fn create_outbound_connection(&self, builder: CreateOutboundConnectionInputBuilder) -> impl Future<Output = Result<CreateOutboundConnectionOutput, SdkError<CreateOutboundConnectionError>>> {
        (*self).create_outbound_connection(builder)
    }
    fn create_package(&self, builder: CreatePackageInputBuilder) -> impl Future<Output = Result<CreatePackageOutput, SdkError<CreatePackageError>>> {
        (*self).create_package(builder)
    }
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>> {
        (*self).create_vpc_endpoint(builder)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        (*self).delete_data_source(builder)
    }
    fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> impl Future<Output = Result<DeleteDomainOutput, SdkError<DeleteDomainError>>> {
        (*self).delete_domain(builder)
    }
    fn delete_inbound_connection(&self, builder: DeleteInboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteInboundConnectionOutput, SdkError<DeleteInboundConnectionError>>> {
        (*self).delete_inbound_connection(builder)
    }
    fn delete_outbound_connection(&self, builder: DeleteOutboundConnectionInputBuilder) -> impl Future<Output = Result<DeleteOutboundConnectionOutput, SdkError<DeleteOutboundConnectionError>>> {
        (*self).delete_outbound_connection(builder)
    }
    fn delete_package(&self, builder: DeletePackageInputBuilder) -> impl Future<Output = Result<DeletePackageOutput, SdkError<DeletePackageError>>> {
        (*self).delete_package(builder)
    }
    fn delete_vpc_endpoint(&self, builder: DeleteVpcEndpointInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointOutput, SdkError<DeleteVpcEndpointError>>> {
        (*self).delete_vpc_endpoint(builder)
    }
    fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> impl Future<Output = Result<DescribeDomainOutput, SdkError<DescribeDomainError>>> {
        (*self).describe_domain(builder)
    }
    fn describe_domain_auto_tunes(&self, builder: DescribeDomainAutoTunesInputBuilder) -> impl Future<Output = Result<DescribeDomainAutoTunesOutput, SdkError<DescribeDomainAutoTunesError>>> {
        (*self).describe_domain_auto_tunes(builder)
    }
    fn describe_domain_change_progress(&self, builder: DescribeDomainChangeProgressInputBuilder) -> impl Future<Output = Result<DescribeDomainChangeProgressOutput, SdkError<DescribeDomainChangeProgressError>>> {
        (*self).describe_domain_change_progress(builder)
    }
    fn describe_domain_config(&self, builder: DescribeDomainConfigInputBuilder) -> impl Future<Output = Result<DescribeDomainConfigOutput, SdkError<DescribeDomainConfigError>>> {
        (*self).describe_domain_config(builder)
    }
    fn describe_domain_health(&self, builder: DescribeDomainHealthInputBuilder) -> impl Future<Output = Result<DescribeDomainHealthOutput, SdkError<DescribeDomainHealthError>>> {
        (*self).describe_domain_health(builder)
    }
    fn describe_domain_nodes(&self, builder: DescribeDomainNodesInputBuilder) -> impl Future<Output = Result<DescribeDomainNodesOutput, SdkError<DescribeDomainNodesError>>> {
        (*self).describe_domain_nodes(builder)
    }
    fn describe_domains(&self, builder: DescribeDomainsInputBuilder) -> impl Future<Output = Result<DescribeDomainsOutput, SdkError<DescribeDomainsError>>> {
        (*self).describe_domains(builder)
    }
    fn describe_dry_run_progress(&self, builder: DescribeDryRunProgressInputBuilder) -> impl Future<Output = Result<DescribeDryRunProgressOutput, SdkError<DescribeDryRunProgressError>>> {
        (*self).describe_dry_run_progress(builder)
    }
    fn describe_inbound_connections(&self, builder: DescribeInboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeInboundConnectionsOutput, SdkError<DescribeInboundConnectionsError>>> {
        (*self).describe_inbound_connections(builder)
    }
    fn describe_instance_type_limits(&self, builder: DescribeInstanceTypeLimitsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeLimitsOutput, SdkError<DescribeInstanceTypeLimitsError>>> {
        (*self).describe_instance_type_limits(builder)
    }
    fn describe_outbound_connections(&self, builder: DescribeOutboundConnectionsInputBuilder) -> impl Future<Output = Result<DescribeOutboundConnectionsOutput, SdkError<DescribeOutboundConnectionsError>>> {
        (*self).describe_outbound_connections(builder)
    }
    fn describe_packages(&self, builder: DescribePackagesInputBuilder) -> impl Future<Output = Result<DescribePackagesOutput, SdkError<DescribePackagesError>>> {
        (*self).describe_packages(builder)
    }
    fn describe_reserved_instance_offerings(&self, builder: DescribeReservedInstanceOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstanceOfferingsOutput, SdkError<DescribeReservedInstanceOfferingsError>>> {
        (*self).describe_reserved_instance_offerings(builder)
    }
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>> {
        (*self).describe_reserved_instances(builder)
    }
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>> {
        (*self).describe_vpc_endpoints(builder)
    }
    fn dissociate_package(&self, builder: DissociatePackageInputBuilder) -> impl Future<Output = Result<DissociatePackageOutput, SdkError<DissociatePackageError>>> {
        (*self).dissociate_package(builder)
    }
    fn get_compatible_versions(&self, builder: GetCompatibleVersionsInputBuilder) -> impl Future<Output = Result<GetCompatibleVersionsOutput, SdkError<GetCompatibleVersionsError>>> {
        (*self).get_compatible_versions(builder)
    }
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>> {
        (*self).get_data_source(builder)
    }
    fn get_domain_maintenance_status(&self, builder: GetDomainMaintenanceStatusInputBuilder) -> impl Future<Output = Result<GetDomainMaintenanceStatusOutput, SdkError<GetDomainMaintenanceStatusError>>> {
        (*self).get_domain_maintenance_status(builder)
    }
    fn get_package_version_history(&self, builder: GetPackageVersionHistoryInputBuilder) -> impl Future<Output = Result<GetPackageVersionHistoryOutput, SdkError<GetPackageVersionHistoryError>>> {
        (*self).get_package_version_history(builder)
    }
    fn get_upgrade_history(&self, builder: GetUpgradeHistoryInputBuilder) -> impl Future<Output = Result<GetUpgradeHistoryOutput, SdkError<GetUpgradeHistoryError>>> {
        (*self).get_upgrade_history(builder)
    }
    fn get_upgrade_status(&self, builder: GetUpgradeStatusInputBuilder) -> impl Future<Output = Result<GetUpgradeStatusOutput, SdkError<GetUpgradeStatusError>>> {
        (*self).get_upgrade_status(builder)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        (*self).list_data_sources(builder)
    }
    fn list_domain_maintenances(&self, builder: ListDomainMaintenancesInputBuilder) -> impl Future<Output = Result<ListDomainMaintenancesOutput, SdkError<ListDomainMaintenancesError>>> {
        (*self).list_domain_maintenances(builder)
    }
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>> {
        (*self).list_domain_names(builder)
    }
    fn list_domains_for_package(&self, builder: ListDomainsForPackageInputBuilder) -> impl Future<Output = Result<ListDomainsForPackageOutput, SdkError<ListDomainsForPackageError>>> {
        (*self).list_domains_for_package(builder)
    }
    fn list_instance_type_details(&self, builder: ListInstanceTypeDetailsInputBuilder) -> impl Future<Output = Result<ListInstanceTypeDetailsOutput, SdkError<ListInstanceTypeDetailsError>>> {
        (*self).list_instance_type_details(builder)
    }
    fn list_packages_for_domain(&self, builder: ListPackagesForDomainInputBuilder) -> impl Future<Output = Result<ListPackagesForDomainOutput, SdkError<ListPackagesForDomainError>>> {
        (*self).list_packages_for_domain(builder)
    }
    fn list_scheduled_actions(&self, builder: ListScheduledActionsInputBuilder) -> impl Future<Output = Result<ListScheduledActionsOutput, SdkError<ListScheduledActionsError>>> {
        (*self).list_scheduled_actions(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        (*self).list_tags(builder)
    }
    fn list_versions(&self, builder: ListVersionsInputBuilder) -> impl Future<Output = Result<ListVersionsOutput, SdkError<ListVersionsError>>> {
        (*self).list_versions(builder)
    }
    fn list_vpc_endpoint_access(&self, builder: ListVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<ListVpcEndpointAccessOutput, SdkError<ListVpcEndpointAccessError>>> {
        (*self).list_vpc_endpoint_access(builder)
    }
    fn list_vpc_endpoints(&self, builder: ListVpcEndpointsInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsOutput, SdkError<ListVpcEndpointsError>>> {
        (*self).list_vpc_endpoints(builder)
    }
    fn list_vpc_endpoints_for_domain(&self, builder: ListVpcEndpointsForDomainInputBuilder) -> impl Future<Output = Result<ListVpcEndpointsForDomainOutput, SdkError<ListVpcEndpointsForDomainError>>> {
        (*self).list_vpc_endpoints_for_domain(builder)
    }
    fn purchase_reserved_instance_offering(&self, builder: PurchaseReservedInstanceOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstanceOfferingOutput, SdkError<PurchaseReservedInstanceOfferingError>>> {
        (*self).purchase_reserved_instance_offering(builder)
    }
    fn reject_inbound_connection(&self, builder: RejectInboundConnectionInputBuilder) -> impl Future<Output = Result<RejectInboundConnectionOutput, SdkError<RejectInboundConnectionError>>> {
        (*self).reject_inbound_connection(builder)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        (*self).remove_tags(builder)
    }
    fn revoke_vpc_endpoint_access(&self, builder: RevokeVpcEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeVpcEndpointAccessOutput, SdkError<RevokeVpcEndpointAccessError>>> {
        (*self).revoke_vpc_endpoint_access(builder)
    }
    fn start_domain_maintenance(&self, builder: StartDomainMaintenanceInputBuilder) -> impl Future<Output = Result<StartDomainMaintenanceOutput, SdkError<StartDomainMaintenanceError>>> {
        (*self).start_domain_maintenance(builder)
    }
    fn start_service_software_update(&self, builder: StartServiceSoftwareUpdateInputBuilder) -> impl Future<Output = Result<StartServiceSoftwareUpdateOutput, SdkError<StartServiceSoftwareUpdateError>>> {
        (*self).start_service_software_update(builder)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        (*self).update_data_source(builder)
    }
    fn update_domain_config(&self, builder: UpdateDomainConfigInputBuilder) -> impl Future<Output = Result<UpdateDomainConfigOutput, SdkError<UpdateDomainConfigError>>> {
        (*self).update_domain_config(builder)
    }
    fn update_package(&self, builder: UpdatePackageInputBuilder) -> impl Future<Output = Result<UpdatePackageOutput, SdkError<UpdatePackageError>>> {
        (*self).update_package(builder)
    }
    fn update_scheduled_action(&self, builder: UpdateScheduledActionInputBuilder) -> impl Future<Output = Result<UpdateScheduledActionOutput, SdkError<UpdateScheduledActionError>>> {
        (*self).update_scheduled_action(builder)
    }
    fn update_vpc_endpoint(&self, builder: UpdateVpcEndpointInputBuilder) -> impl Future<Output = Result<UpdateVpcEndpointOutput, SdkError<UpdateVpcEndpointError>>> {
        (*self).update_vpc_endpoint(builder)
    }
    fn upgrade_domain(&self, builder: UpgradeDomainInputBuilder) -> impl Future<Output = Result<UpgradeDomainOutput, SdkError<UpgradeDomainError>>> {
        (*self).upgrade_domain(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edOpenSearchClient {}
    impl OpenSearchClient for edOpenSearchClient {
        async fn accept_inbound_connection(&self, builder: AcceptInboundConnectionInputBuilder) -> Result<AcceptInboundConnectionOutput, SdkError<AcceptInboundConnectionError>>;
        async fn add_data_source(&self, builder: AddDataSourceInputBuilder) -> Result<AddDataSourceOutput, SdkError<AddDataSourceError>>;
        async fn add_tags(&self, builder: AddTagsInputBuilder) -> Result<AddTagsOutput, SdkError<AddTagsError>>;
        async fn associate_package(&self, builder: AssociatePackageInputBuilder) -> Result<AssociatePackageOutput, SdkError<AssociatePackageError>>;
        async fn authorize_vpc_endpoint_access(&self, builder: AuthorizeVpcEndpointAccessInputBuilder) -> Result<AuthorizeVpcEndpointAccessOutput, SdkError<AuthorizeVpcEndpointAccessError>>;
        async fn cancel_domain_config_change(&self, builder: CancelDomainConfigChangeInputBuilder) -> Result<CancelDomainConfigChangeOutput, SdkError<CancelDomainConfigChangeError>>;
        async fn cancel_service_software_update(&self, builder: CancelServiceSoftwareUpdateInputBuilder) -> Result<CancelServiceSoftwareUpdateOutput, SdkError<CancelServiceSoftwareUpdateError>>;
        async fn create_domain(&self, builder: CreateDomainInputBuilder) -> Result<CreateDomainOutput, SdkError<CreateDomainError>>;
        async fn create_outbound_connection(&self, builder: CreateOutboundConnectionInputBuilder) -> Result<CreateOutboundConnectionOutput, SdkError<CreateOutboundConnectionError>>;
        async fn create_package(&self, builder: CreatePackageInputBuilder) -> Result<CreatePackageOutput, SdkError<CreatePackageError>>;
        async fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>;
        async fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>;
        async fn delete_domain(&self, builder: DeleteDomainInputBuilder) -> Result<DeleteDomainOutput, SdkError<DeleteDomainError>>;
        async fn delete_inbound_connection(&self, builder: DeleteInboundConnectionInputBuilder) -> Result<DeleteInboundConnectionOutput, SdkError<DeleteInboundConnectionError>>;
        async fn delete_outbound_connection(&self, builder: DeleteOutboundConnectionInputBuilder) -> Result<DeleteOutboundConnectionOutput, SdkError<DeleteOutboundConnectionError>>;
        async fn delete_package(&self, builder: DeletePackageInputBuilder) -> Result<DeletePackageOutput, SdkError<DeletePackageError>>;
        async fn delete_vpc_endpoint(&self, builder: DeleteVpcEndpointInputBuilder) -> Result<DeleteVpcEndpointOutput, SdkError<DeleteVpcEndpointError>>;
        async fn describe_domain(&self, builder: DescribeDomainInputBuilder) -> Result<DescribeDomainOutput, SdkError<DescribeDomainError>>;
        async fn describe_domain_auto_tunes(&self, builder: DescribeDomainAutoTunesInputBuilder) -> Result<DescribeDomainAutoTunesOutput, SdkError<DescribeDomainAutoTunesError>>;
        async fn describe_domain_change_progress(&self, builder: DescribeDomainChangeProgressInputBuilder) -> Result<DescribeDomainChangeProgressOutput, SdkError<DescribeDomainChangeProgressError>>;
        async fn describe_domain_config(&self, builder: DescribeDomainConfigInputBuilder) -> Result<DescribeDomainConfigOutput, SdkError<DescribeDomainConfigError>>;
        async fn describe_domain_health(&self, builder: DescribeDomainHealthInputBuilder) -> Result<DescribeDomainHealthOutput, SdkError<DescribeDomainHealthError>>;
        async fn describe_domain_nodes(&self, builder: DescribeDomainNodesInputBuilder) -> Result<DescribeDomainNodesOutput, SdkError<DescribeDomainNodesError>>;
        async fn describe_domains(&self, builder: DescribeDomainsInputBuilder) -> Result<DescribeDomainsOutput, SdkError<DescribeDomainsError>>;
        async fn describe_dry_run_progress(&self, builder: DescribeDryRunProgressInputBuilder) -> Result<DescribeDryRunProgressOutput, SdkError<DescribeDryRunProgressError>>;
        async fn describe_inbound_connections(&self, builder: DescribeInboundConnectionsInputBuilder) -> Result<DescribeInboundConnectionsOutput, SdkError<DescribeInboundConnectionsError>>;
        async fn describe_instance_type_limits(&self, builder: DescribeInstanceTypeLimitsInputBuilder) -> Result<DescribeInstanceTypeLimitsOutput, SdkError<DescribeInstanceTypeLimitsError>>;
        async fn describe_outbound_connections(&self, builder: DescribeOutboundConnectionsInputBuilder) -> Result<DescribeOutboundConnectionsOutput, SdkError<DescribeOutboundConnectionsError>>;
        async fn describe_packages(&self, builder: DescribePackagesInputBuilder) -> Result<DescribePackagesOutput, SdkError<DescribePackagesError>>;
        async fn describe_reserved_instance_offerings(&self, builder: DescribeReservedInstanceOfferingsInputBuilder) -> Result<DescribeReservedInstanceOfferingsOutput, SdkError<DescribeReservedInstanceOfferingsError>>;
        async fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>;
        async fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>;
        async fn dissociate_package(&self, builder: DissociatePackageInputBuilder) -> Result<DissociatePackageOutput, SdkError<DissociatePackageError>>;
        async fn get_compatible_versions(&self, builder: GetCompatibleVersionsInputBuilder) -> Result<GetCompatibleVersionsOutput, SdkError<GetCompatibleVersionsError>>;
        async fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> Result<GetDataSourceOutput, SdkError<GetDataSourceError>>;
        async fn get_domain_maintenance_status(&self, builder: GetDomainMaintenanceStatusInputBuilder) -> Result<GetDomainMaintenanceStatusOutput, SdkError<GetDomainMaintenanceStatusError>>;
        async fn get_package_version_history(&self, builder: GetPackageVersionHistoryInputBuilder) -> Result<GetPackageVersionHistoryOutput, SdkError<GetPackageVersionHistoryError>>;
        async fn get_upgrade_history(&self, builder: GetUpgradeHistoryInputBuilder) -> Result<GetUpgradeHistoryOutput, SdkError<GetUpgradeHistoryError>>;
        async fn get_upgrade_status(&self, builder: GetUpgradeStatusInputBuilder) -> Result<GetUpgradeStatusOutput, SdkError<GetUpgradeStatusError>>;
        async fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>;
        async fn list_domain_maintenances(&self, builder: ListDomainMaintenancesInputBuilder) -> Result<ListDomainMaintenancesOutput, SdkError<ListDomainMaintenancesError>>;
        async fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>;
        async fn list_domains_for_package(&self, builder: ListDomainsForPackageInputBuilder) -> Result<ListDomainsForPackageOutput, SdkError<ListDomainsForPackageError>>;
        async fn list_instance_type_details(&self, builder: ListInstanceTypeDetailsInputBuilder) -> Result<ListInstanceTypeDetailsOutput, SdkError<ListInstanceTypeDetailsError>>;
        async fn list_packages_for_domain(&self, builder: ListPackagesForDomainInputBuilder) -> Result<ListPackagesForDomainOutput, SdkError<ListPackagesForDomainError>>;
        async fn list_scheduled_actions(&self, builder: ListScheduledActionsInputBuilder) -> Result<ListScheduledActionsOutput, SdkError<ListScheduledActionsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn list_versions(&self, builder: ListVersionsInputBuilder) -> Result<ListVersionsOutput, SdkError<ListVersionsError>>;
        async fn list_vpc_endpoint_access(&self, builder: ListVpcEndpointAccessInputBuilder) -> Result<ListVpcEndpointAccessOutput, SdkError<ListVpcEndpointAccessError>>;
        async fn list_vpc_endpoints(&self, builder: ListVpcEndpointsInputBuilder) -> Result<ListVpcEndpointsOutput, SdkError<ListVpcEndpointsError>>;
        async fn list_vpc_endpoints_for_domain(&self, builder: ListVpcEndpointsForDomainInputBuilder) -> Result<ListVpcEndpointsForDomainOutput, SdkError<ListVpcEndpointsForDomainError>>;
        async fn purchase_reserved_instance_offering(&self, builder: PurchaseReservedInstanceOfferingInputBuilder) -> Result<PurchaseReservedInstanceOfferingOutput, SdkError<PurchaseReservedInstanceOfferingError>>;
        async fn reject_inbound_connection(&self, builder: RejectInboundConnectionInputBuilder) -> Result<RejectInboundConnectionOutput, SdkError<RejectInboundConnectionError>>;
        async fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> Result<RemoveTagsOutput, SdkError<RemoveTagsError>>;
        async fn revoke_vpc_endpoint_access(&self, builder: RevokeVpcEndpointAccessInputBuilder) -> Result<RevokeVpcEndpointAccessOutput, SdkError<RevokeVpcEndpointAccessError>>;
        async fn start_domain_maintenance(&self, builder: StartDomainMaintenanceInputBuilder) -> Result<StartDomainMaintenanceOutput, SdkError<StartDomainMaintenanceError>>;
        async fn start_service_software_update(&self, builder: StartServiceSoftwareUpdateInputBuilder) -> Result<StartServiceSoftwareUpdateOutput, SdkError<StartServiceSoftwareUpdateError>>;
        async fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>;
        async fn update_domain_config(&self, builder: UpdateDomainConfigInputBuilder) -> Result<UpdateDomainConfigOutput, SdkError<UpdateDomainConfigError>>;
        async fn update_package(&self, builder: UpdatePackageInputBuilder) -> Result<UpdatePackageOutput, SdkError<UpdatePackageError>>;
        async fn update_scheduled_action(&self, builder: UpdateScheduledActionInputBuilder) -> Result<UpdateScheduledActionOutput, SdkError<UpdateScheduledActionError>>;
        async fn update_vpc_endpoint(&self, builder: UpdateVpcEndpointInputBuilder) -> Result<UpdateVpcEndpointOutput, SdkError<UpdateVpcEndpointError>>;
        async fn upgrade_domain(&self, builder: UpgradeDomainInputBuilder) -> Result<UpgradeDomainOutput, SdkError<UpgradeDomainError>>;
    }
}
