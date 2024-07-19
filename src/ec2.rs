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
use aws_sdk_ec2::operation::accept_address_transfer::{builders::*, *};
use aws_sdk_ec2::operation::accept_reserved_instances_exchange_quote::{builders::*, *};
use aws_sdk_ec2::operation::accept_transit_gateway_multicast_domain_associations::{builders::*, *};
use aws_sdk_ec2::operation::accept_transit_gateway_peering_attachment::{builders::*, *};
use aws_sdk_ec2::operation::accept_transit_gateway_vpc_attachment::{builders::*, *};
use aws_sdk_ec2::operation::accept_vpc_endpoint_connections::{builders::*, *};
use aws_sdk_ec2::operation::accept_vpc_peering_connection::{builders::*, *};
use aws_sdk_ec2::operation::advertise_byoip_cidr::{builders::*, *};
use aws_sdk_ec2::operation::allocate_address::{builders::*, *};
use aws_sdk_ec2::operation::allocate_hosts::{builders::*, *};
use aws_sdk_ec2::operation::allocate_ipam_pool_cidr::{builders::*, *};
use aws_sdk_ec2::operation::apply_security_groups_to_client_vpn_target_network::{builders::*, *};
use aws_sdk_ec2::operation::assign_ipv6_addresses::{builders::*, *};
use aws_sdk_ec2::operation::assign_private_ip_addresses::{builders::*, *};
use aws_sdk_ec2::operation::assign_private_nat_gateway_address::{builders::*, *};
use aws_sdk_ec2::operation::associate_address::{builders::*, *};
use aws_sdk_ec2::operation::associate_client_vpn_target_network::{builders::*, *};
use aws_sdk_ec2::operation::associate_dhcp_options::{builders::*, *};
use aws_sdk_ec2::operation::associate_enclave_certificate_iam_role::{builders::*, *};
use aws_sdk_ec2::operation::associate_iam_instance_profile::{builders::*, *};
use aws_sdk_ec2::operation::associate_instance_event_window::{builders::*, *};
use aws_sdk_ec2::operation::associate_ipam_byoasn::{builders::*, *};
use aws_sdk_ec2::operation::associate_ipam_resource_discovery::{builders::*, *};
use aws_sdk_ec2::operation::associate_nat_gateway_address::{builders::*, *};
use aws_sdk_ec2::operation::associate_route_table::{builders::*, *};
use aws_sdk_ec2::operation::associate_subnet_cidr_block::{builders::*, *};
use aws_sdk_ec2::operation::associate_transit_gateway_multicast_domain::{builders::*, *};
use aws_sdk_ec2::operation::associate_transit_gateway_policy_table::{builders::*, *};
use aws_sdk_ec2::operation::associate_transit_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::associate_trunk_interface::{builders::*, *};
use aws_sdk_ec2::operation::associate_vpc_cidr_block::{builders::*, *};
use aws_sdk_ec2::operation::attach_classic_link_vpc::{builders::*, *};
use aws_sdk_ec2::operation::attach_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::attach_network_interface::{builders::*, *};
use aws_sdk_ec2::operation::attach_verified_access_trust_provider::{builders::*, *};
use aws_sdk_ec2::operation::attach_volume::{builders::*, *};
use aws_sdk_ec2::operation::attach_vpn_gateway::{builders::*, *};
use aws_sdk_ec2::operation::authorize_client_vpn_ingress::{builders::*, *};
use aws_sdk_ec2::operation::authorize_security_group_egress::{builders::*, *};
use aws_sdk_ec2::operation::authorize_security_group_ingress::{builders::*, *};
use aws_sdk_ec2::operation::bundle_instance::{builders::*, *};
use aws_sdk_ec2::operation::cancel_bundle_task::{builders::*, *};
use aws_sdk_ec2::operation::cancel_capacity_reservation::{builders::*, *};
use aws_sdk_ec2::operation::cancel_capacity_reservation_fleets::{builders::*, *};
use aws_sdk_ec2::operation::cancel_conversion_task::{builders::*, *};
use aws_sdk_ec2::operation::cancel_export_task::{builders::*, *};
use aws_sdk_ec2::operation::cancel_image_launch_permission::{builders::*, *};
use aws_sdk_ec2::operation::cancel_import_task::{builders::*, *};
use aws_sdk_ec2::operation::cancel_reserved_instances_listing::{builders::*, *};
use aws_sdk_ec2::operation::cancel_spot_fleet_requests::{builders::*, *};
use aws_sdk_ec2::operation::cancel_spot_instance_requests::{builders::*, *};
use aws_sdk_ec2::operation::confirm_product_instance::{builders::*, *};
use aws_sdk_ec2::operation::copy_fpga_image::{builders::*, *};
use aws_sdk_ec2::operation::copy_image::{builders::*, *};
use aws_sdk_ec2::operation::copy_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::create_capacity_reservation::{builders::*, *};
use aws_sdk_ec2::operation::create_capacity_reservation_fleet::{builders::*, *};
use aws_sdk_ec2::operation::create_carrier_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_client_vpn_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::create_client_vpn_route::{builders::*, *};
use aws_sdk_ec2::operation::create_coip_cidr::{builders::*, *};
use aws_sdk_ec2::operation::create_coip_pool::{builders::*, *};
use aws_sdk_ec2::operation::create_customer_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_default_subnet::{builders::*, *};
use aws_sdk_ec2::operation::create_default_vpc::{builders::*, *};
use aws_sdk_ec2::operation::create_dhcp_options::{builders::*, *};
use aws_sdk_ec2::operation::create_egress_only_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_fleet::{builders::*, *};
use aws_sdk_ec2::operation::create_flow_logs::{builders::*, *};
use aws_sdk_ec2::operation::create_fpga_image::{builders::*, *};
use aws_sdk_ec2::operation::create_image::{builders::*, *};
use aws_sdk_ec2::operation::create_instance_connect_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::create_instance_event_window::{builders::*, *};
use aws_sdk_ec2::operation::create_instance_export_task::{builders::*, *};
use aws_sdk_ec2::operation::create_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_ipam::{builders::*, *};
use aws_sdk_ec2::operation::create_ipam_external_resource_verification_token::{builders::*, *};
use aws_sdk_ec2::operation::create_ipam_pool::{builders::*, *};
use aws_sdk_ec2::operation::create_ipam_resource_discovery::{builders::*, *};
use aws_sdk_ec2::operation::create_ipam_scope::{builders::*, *};
use aws_sdk_ec2::operation::create_key_pair::{builders::*, *};
use aws_sdk_ec2::operation::create_launch_template::{builders::*, *};
use aws_sdk_ec2::operation::create_launch_template_version::{builders::*, *};
use aws_sdk_ec2::operation::create_local_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::create_local_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::create_local_gateway_route_table_virtual_interface_group_association::{builders::*, *};
use aws_sdk_ec2::operation::create_local_gateway_route_table_vpc_association::{builders::*, *};
use aws_sdk_ec2::operation::create_managed_prefix_list::{builders::*, *};
use aws_sdk_ec2::operation::create_nat_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_network_acl::{builders::*, *};
use aws_sdk_ec2::operation::create_network_acl_entry::{builders::*, *};
use aws_sdk_ec2::operation::create_network_insights_access_scope::{builders::*, *};
use aws_sdk_ec2::operation::create_network_insights_path::{builders::*, *};
use aws_sdk_ec2::operation::create_network_interface::{builders::*, *};
use aws_sdk_ec2::operation::create_network_interface_permission::{builders::*, *};
use aws_sdk_ec2::operation::create_placement_group::{builders::*, *};
use aws_sdk_ec2::operation::create_public_ipv4_pool::{builders::*, *};
use aws_sdk_ec2::operation::create_replace_root_volume_task::{builders::*, *};
use aws_sdk_ec2::operation::create_reserved_instances_listing::{builders::*, *};
use aws_sdk_ec2::operation::create_restore_image_task::{builders::*, *};
use aws_sdk_ec2::operation::create_route::{builders::*, *};
use aws_sdk_ec2::operation::create_route_table::{builders::*, *};
use aws_sdk_ec2::operation::create_security_group::{builders::*, *};
use aws_sdk_ec2::operation::create_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::create_snapshots::{builders::*, *};
use aws_sdk_ec2::operation::create_spot_datafeed_subscription::{builders::*, *};
use aws_sdk_ec2::operation::create_store_image_task::{builders::*, *};
use aws_sdk_ec2::operation::create_subnet::{builders::*, *};
use aws_sdk_ec2::operation::create_subnet_cidr_reservation::{builders::*, *};
use aws_sdk_ec2::operation::create_tags::{builders::*, *};
use aws_sdk_ec2::operation::create_traffic_mirror_filter::{builders::*, *};
use aws_sdk_ec2::operation::create_traffic_mirror_filter_rule::{builders::*, *};
use aws_sdk_ec2::operation::create_traffic_mirror_session::{builders::*, *};
use aws_sdk_ec2::operation::create_traffic_mirror_target::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_connect::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_connect_peer::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_multicast_domain::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_peering_attachment::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_policy_table::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_prefix_list_reference::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_route_table_announcement::{builders::*, *};
use aws_sdk_ec2::operation::create_transit_gateway_vpc_attachment::{builders::*, *};
use aws_sdk_ec2::operation::create_verified_access_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::create_verified_access_group::{builders::*, *};
use aws_sdk_ec2::operation::create_verified_access_instance::{builders::*, *};
use aws_sdk_ec2::operation::create_verified_access_trust_provider::{builders::*, *};
use aws_sdk_ec2::operation::create_volume::{builders::*, *};
use aws_sdk_ec2::operation::create_vpc::{builders::*, *};
use aws_sdk_ec2::operation::create_vpc_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::create_vpc_endpoint_connection_notification::{builders::*, *};
use aws_sdk_ec2::operation::create_vpc_endpoint_service_configuration::{builders::*, *};
use aws_sdk_ec2::operation::create_vpc_peering_connection::{builders::*, *};
use aws_sdk_ec2::operation::create_vpn_connection::{builders::*, *};
use aws_sdk_ec2::operation::create_vpn_connection_route::{builders::*, *};
use aws_sdk_ec2::operation::create_vpn_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_carrier_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_client_vpn_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::delete_client_vpn_route::{builders::*, *};
use aws_sdk_ec2::operation::delete_coip_cidr::{builders::*, *};
use aws_sdk_ec2::operation::delete_coip_pool::{builders::*, *};
use aws_sdk_ec2::operation::delete_customer_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_dhcp_options::{builders::*, *};
use aws_sdk_ec2::operation::delete_egress_only_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_fleets::{builders::*, *};
use aws_sdk_ec2::operation::delete_flow_logs::{builders::*, *};
use aws_sdk_ec2::operation::delete_fpga_image::{builders::*, *};
use aws_sdk_ec2::operation::delete_instance_connect_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::delete_instance_event_window::{builders::*, *};
use aws_sdk_ec2::operation::delete_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_ipam::{builders::*, *};
use aws_sdk_ec2::operation::delete_ipam_external_resource_verification_token::{builders::*, *};
use aws_sdk_ec2::operation::delete_ipam_pool::{builders::*, *};
use aws_sdk_ec2::operation::delete_ipam_resource_discovery::{builders::*, *};
use aws_sdk_ec2::operation::delete_ipam_scope::{builders::*, *};
use aws_sdk_ec2::operation::delete_key_pair::{builders::*, *};
use aws_sdk_ec2::operation::delete_launch_template::{builders::*, *};
use aws_sdk_ec2::operation::delete_launch_template_versions::{builders::*, *};
use aws_sdk_ec2::operation::delete_local_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::delete_local_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::delete_local_gateway_route_table_virtual_interface_group_association::{builders::*, *};
use aws_sdk_ec2::operation::delete_local_gateway_route_table_vpc_association::{builders::*, *};
use aws_sdk_ec2::operation::delete_managed_prefix_list::{builders::*, *};
use aws_sdk_ec2::operation::delete_nat_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_acl::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_acl_entry::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_insights_access_scope::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_insights_access_scope_analysis::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_insights_analysis::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_insights_path::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_interface::{builders::*, *};
use aws_sdk_ec2::operation::delete_network_interface_permission::{builders::*, *};
use aws_sdk_ec2::operation::delete_placement_group::{builders::*, *};
use aws_sdk_ec2::operation::delete_public_ipv4_pool::{builders::*, *};
use aws_sdk_ec2::operation::delete_queued_reserved_instances::{builders::*, *};
use aws_sdk_ec2::operation::delete_route::{builders::*, *};
use aws_sdk_ec2::operation::delete_route_table::{builders::*, *};
use aws_sdk_ec2::operation::delete_security_group::{builders::*, *};
use aws_sdk_ec2::operation::delete_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::delete_spot_datafeed_subscription::{builders::*, *};
use aws_sdk_ec2::operation::delete_subnet::{builders::*, *};
use aws_sdk_ec2::operation::delete_subnet_cidr_reservation::{builders::*, *};
use aws_sdk_ec2::operation::delete_tags::{builders::*, *};
use aws_sdk_ec2::operation::delete_traffic_mirror_filter::{builders::*, *};
use aws_sdk_ec2::operation::delete_traffic_mirror_filter_rule::{builders::*, *};
use aws_sdk_ec2::operation::delete_traffic_mirror_session::{builders::*, *};
use aws_sdk_ec2::operation::delete_traffic_mirror_target::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_connect::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_connect_peer::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_multicast_domain::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_peering_attachment::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_policy_table::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_prefix_list_reference::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_route_table_announcement::{builders::*, *};
use aws_sdk_ec2::operation::delete_transit_gateway_vpc_attachment::{builders::*, *};
use aws_sdk_ec2::operation::delete_verified_access_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::delete_verified_access_group::{builders::*, *};
use aws_sdk_ec2::operation::delete_verified_access_instance::{builders::*, *};
use aws_sdk_ec2::operation::delete_verified_access_trust_provider::{builders::*, *};
use aws_sdk_ec2::operation::delete_volume::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpc::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpc_endpoint_connection_notifications::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpc_endpoint_service_configurations::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpc_endpoints::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpc_peering_connection::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpn_connection::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpn_connection_route::{builders::*, *};
use aws_sdk_ec2::operation::delete_vpn_gateway::{builders::*, *};
use aws_sdk_ec2::operation::deprovision_byoip_cidr::{builders::*, *};
use aws_sdk_ec2::operation::deprovision_ipam_byoasn::{builders::*, *};
use aws_sdk_ec2::operation::deprovision_ipam_pool_cidr::{builders::*, *};
use aws_sdk_ec2::operation::deprovision_public_ipv4_pool_cidr::{builders::*, *};
use aws_sdk_ec2::operation::deregister_image::{builders::*, *};
use aws_sdk_ec2::operation::deregister_instance_event_notification_attributes::{builders::*, *};
use aws_sdk_ec2::operation::deregister_transit_gateway_multicast_group_members::{builders::*, *};
use aws_sdk_ec2::operation::deregister_transit_gateway_multicast_group_sources::{builders::*, *};
use aws_sdk_ec2::operation::describe_account_attributes::{builders::*, *};
use aws_sdk_ec2::operation::describe_address_transfers::{builders::*, *};
use aws_sdk_ec2::operation::describe_addresses::{builders::*, *};
use aws_sdk_ec2::operation::describe_addresses_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_aggregate_id_format::{builders::*, *};
use aws_sdk_ec2::operation::describe_availability_zones::{builders::*, *};
use aws_sdk_ec2::operation::describe_aws_network_performance_metric_subscriptions::{builders::*, *};
use aws_sdk_ec2::operation::describe_bundle_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_byoip_cidrs::{builders::*, *};
use aws_sdk_ec2::operation::describe_capacity_block_offerings::{builders::*, *};
use aws_sdk_ec2::operation::describe_capacity_reservation_fleets::{builders::*, *};
use aws_sdk_ec2::operation::describe_capacity_reservations::{builders::*, *};
use aws_sdk_ec2::operation::describe_carrier_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_classic_link_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_client_vpn_authorization_rules::{builders::*, *};
use aws_sdk_ec2::operation::describe_client_vpn_connections::{builders::*, *};
use aws_sdk_ec2::operation::describe_client_vpn_endpoints::{builders::*, *};
use aws_sdk_ec2::operation::describe_client_vpn_routes::{builders::*, *};
use aws_sdk_ec2::operation::describe_client_vpn_target_networks::{builders::*, *};
use aws_sdk_ec2::operation::describe_coip_pools::{builders::*, *};
use aws_sdk_ec2::operation::describe_conversion_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_customer_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_dhcp_options::{builders::*, *};
use aws_sdk_ec2::operation::describe_egress_only_internet_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_elastic_gpus::{builders::*, *};
use aws_sdk_ec2::operation::describe_export_image_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_export_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_fast_launch_images::{builders::*, *};
use aws_sdk_ec2::operation::describe_fast_snapshot_restores::{builders::*, *};
use aws_sdk_ec2::operation::describe_fleet_history::{builders::*, *};
use aws_sdk_ec2::operation::describe_fleet_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_fleets::{builders::*, *};
use aws_sdk_ec2::operation::describe_flow_logs::{builders::*, *};
use aws_sdk_ec2::operation::describe_fpga_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_fpga_images::{builders::*, *};
use aws_sdk_ec2::operation::describe_host_reservation_offerings::{builders::*, *};
use aws_sdk_ec2::operation::describe_host_reservations::{builders::*, *};
use aws_sdk_ec2::operation::describe_hosts::{builders::*, *};
use aws_sdk_ec2::operation::describe_iam_instance_profile_associations::{builders::*, *};
use aws_sdk_ec2::operation::describe_id_format::{builders::*, *};
use aws_sdk_ec2::operation::describe_identity_id_format::{builders::*, *};
use aws_sdk_ec2::operation::describe_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_images::{builders::*, *};
use aws_sdk_ec2::operation::describe_import_image_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_import_snapshot_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_connect_endpoints::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_credit_specifications::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_event_notification_attributes::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_event_windows::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_status::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_topology::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_type_offerings::{builders::*, *};
use aws_sdk_ec2::operation::describe_instance_types::{builders::*, *};
use aws_sdk_ec2::operation::describe_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_internet_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_byoasn::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_external_resource_verification_tokens::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_pools::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_resource_discoveries::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_resource_discovery_associations::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipam_scopes::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipams::{builders::*, *};
use aws_sdk_ec2::operation::describe_ipv6_pools::{builders::*, *};
use aws_sdk_ec2::operation::describe_key_pairs::{builders::*, *};
use aws_sdk_ec2::operation::describe_launch_template_versions::{builders::*, *};
use aws_sdk_ec2::operation::describe_launch_templates::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateway_route_table_virtual_interface_group_associations::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateway_route_table_vpc_associations::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateway_route_tables::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateway_virtual_interface_groups::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateway_virtual_interfaces::{builders::*, *};
use aws_sdk_ec2::operation::describe_local_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_locked_snapshots::{builders::*, *};
use aws_sdk_ec2::operation::describe_mac_hosts::{builders::*, *};
use aws_sdk_ec2::operation::describe_managed_prefix_lists::{builders::*, *};
use aws_sdk_ec2::operation::describe_moving_addresses::{builders::*, *};
use aws_sdk_ec2::operation::describe_nat_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_acls::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_insights_access_scope_analyses::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_insights_access_scopes::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_insights_analyses::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_insights_paths::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_interface_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_interface_permissions::{builders::*, *};
use aws_sdk_ec2::operation::describe_network_interfaces::{builders::*, *};
use aws_sdk_ec2::operation::describe_placement_groups::{builders::*, *};
use aws_sdk_ec2::operation::describe_prefix_lists::{builders::*, *};
use aws_sdk_ec2::operation::describe_principal_id_format::{builders::*, *};
use aws_sdk_ec2::operation::describe_public_ipv4_pools::{builders::*, *};
use aws_sdk_ec2::operation::describe_regions::{builders::*, *};
use aws_sdk_ec2::operation::describe_replace_root_volume_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_reserved_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_reserved_instances_listings::{builders::*, *};
use aws_sdk_ec2::operation::describe_reserved_instances_modifications::{builders::*, *};
use aws_sdk_ec2::operation::describe_reserved_instances_offerings::{builders::*, *};
use aws_sdk_ec2::operation::describe_route_tables::{builders::*, *};
use aws_sdk_ec2::operation::describe_scheduled_instance_availability::{builders::*, *};
use aws_sdk_ec2::operation::describe_scheduled_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_security_group_references::{builders::*, *};
use aws_sdk_ec2::operation::describe_security_group_rules::{builders::*, *};
use aws_sdk_ec2::operation::describe_security_groups::{builders::*, *};
use aws_sdk_ec2::operation::describe_snapshot_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_snapshot_tier_status::{builders::*, *};
use aws_sdk_ec2::operation::describe_snapshots::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_datafeed_subscription::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_fleet_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_fleet_request_history::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_fleet_requests::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_instance_requests::{builders::*, *};
use aws_sdk_ec2::operation::describe_spot_price_history::{builders::*, *};
use aws_sdk_ec2::operation::describe_stale_security_groups::{builders::*, *};
use aws_sdk_ec2::operation::describe_store_image_tasks::{builders::*, *};
use aws_sdk_ec2::operation::describe_subnets::{builders::*, *};
use aws_sdk_ec2::operation::describe_tags::{builders::*, *};
use aws_sdk_ec2::operation::describe_traffic_mirror_filter_rules::{builders::*, *};
use aws_sdk_ec2::operation::describe_traffic_mirror_filters::{builders::*, *};
use aws_sdk_ec2::operation::describe_traffic_mirror_sessions::{builders::*, *};
use aws_sdk_ec2::operation::describe_traffic_mirror_targets::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_attachments::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_connect_peers::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_connects::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_multicast_domains::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_peering_attachments::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_policy_tables::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_route_table_announcements::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_route_tables::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateway_vpc_attachments::{builders::*, *};
use aws_sdk_ec2::operation::describe_transit_gateways::{builders::*, *};
use aws_sdk_ec2::operation::describe_trunk_interface_associations::{builders::*, *};
use aws_sdk_ec2::operation::describe_verified_access_endpoints::{builders::*, *};
use aws_sdk_ec2::operation::describe_verified_access_groups::{builders::*, *};
use aws_sdk_ec2::operation::describe_verified_access_instance_logging_configurations::{builders::*, *};
use aws_sdk_ec2::operation::describe_verified_access_instances::{builders::*, *};
use aws_sdk_ec2::operation::describe_verified_access_trust_providers::{builders::*, *};
use aws_sdk_ec2::operation::describe_volume_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_volume_status::{builders::*, *};
use aws_sdk_ec2::operation::describe_volumes::{builders::*, *};
use aws_sdk_ec2::operation::describe_volumes_modifications::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_attribute::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_classic_link::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_classic_link_dns_support::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoint_connection_notifications::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoint_connections::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoint_service_configurations::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoint_service_permissions::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoint_services::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_endpoints::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpc_peering_connections::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpcs::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpn_connections::{builders::*, *};
use aws_sdk_ec2::operation::describe_vpn_gateways::{builders::*, *};
use aws_sdk_ec2::operation::detach_classic_link_vpc::{builders::*, *};
use aws_sdk_ec2::operation::detach_internet_gateway::{builders::*, *};
use aws_sdk_ec2::operation::detach_network_interface::{builders::*, *};
use aws_sdk_ec2::operation::detach_verified_access_trust_provider::{builders::*, *};
use aws_sdk_ec2::operation::detach_volume::{builders::*, *};
use aws_sdk_ec2::operation::detach_vpn_gateway::{builders::*, *};
use aws_sdk_ec2::operation::disable_address_transfer::{builders::*, *};
use aws_sdk_ec2::operation::disable_aws_network_performance_metric_subscription::{builders::*, *};
use aws_sdk_ec2::operation::disable_ebs_encryption_by_default::{builders::*, *};
use aws_sdk_ec2::operation::disable_fast_launch::{builders::*, *};
use aws_sdk_ec2::operation::disable_fast_snapshot_restores::{builders::*, *};
use aws_sdk_ec2::operation::disable_image::{builders::*, *};
use aws_sdk_ec2::operation::disable_image_block_public_access::{builders::*, *};
use aws_sdk_ec2::operation::disable_image_deprecation::{builders::*, *};
use aws_sdk_ec2::operation::disable_image_deregistration_protection::{builders::*, *};
use aws_sdk_ec2::operation::disable_ipam_organization_admin_account::{builders::*, *};
use aws_sdk_ec2::operation::disable_serial_console_access::{builders::*, *};
use aws_sdk_ec2::operation::disable_snapshot_block_public_access::{builders::*, *};
use aws_sdk_ec2::operation::disable_transit_gateway_route_table_propagation::{builders::*, *};
use aws_sdk_ec2::operation::disable_vgw_route_propagation::{builders::*, *};
use aws_sdk_ec2::operation::disable_vpc_classic_link::{builders::*, *};
use aws_sdk_ec2::operation::disable_vpc_classic_link_dns_support::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_address::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_client_vpn_target_network::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_enclave_certificate_iam_role::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_iam_instance_profile::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_instance_event_window::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_ipam_byoasn::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_ipam_resource_discovery::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_nat_gateway_address::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_route_table::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_subnet_cidr_block::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_transit_gateway_multicast_domain::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_transit_gateway_policy_table::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_transit_gateway_route_table::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_trunk_interface::{builders::*, *};
use aws_sdk_ec2::operation::disassociate_vpc_cidr_block::{builders::*, *};
use aws_sdk_ec2::operation::enable_address_transfer::{builders::*, *};
use aws_sdk_ec2::operation::enable_aws_network_performance_metric_subscription::{builders::*, *};
use aws_sdk_ec2::operation::enable_ebs_encryption_by_default::{builders::*, *};
use aws_sdk_ec2::operation::enable_fast_launch::{builders::*, *};
use aws_sdk_ec2::operation::enable_fast_snapshot_restores::{builders::*, *};
use aws_sdk_ec2::operation::enable_image::{builders::*, *};
use aws_sdk_ec2::operation::enable_image_block_public_access::{builders::*, *};
use aws_sdk_ec2::operation::enable_image_deprecation::{builders::*, *};
use aws_sdk_ec2::operation::enable_image_deregistration_protection::{builders::*, *};
use aws_sdk_ec2::operation::enable_ipam_organization_admin_account::{builders::*, *};
use aws_sdk_ec2::operation::enable_reachability_analyzer_organization_sharing::{builders::*, *};
use aws_sdk_ec2::operation::enable_serial_console_access::{builders::*, *};
use aws_sdk_ec2::operation::enable_snapshot_block_public_access::{builders::*, *};
use aws_sdk_ec2::operation::enable_transit_gateway_route_table_propagation::{builders::*, *};
use aws_sdk_ec2::operation::enable_vgw_route_propagation::{builders::*, *};
use aws_sdk_ec2::operation::enable_volume_io::{builders::*, *};
use aws_sdk_ec2::operation::enable_vpc_classic_link::{builders::*, *};
use aws_sdk_ec2::operation::enable_vpc_classic_link_dns_support::{builders::*, *};
use aws_sdk_ec2::operation::export_client_vpn_client_certificate_revocation_list::{builders::*, *};
use aws_sdk_ec2::operation::export_client_vpn_client_configuration::{builders::*, *};
use aws_sdk_ec2::operation::export_image::{builders::*, *};
use aws_sdk_ec2::operation::export_transit_gateway_routes::{builders::*, *};
use aws_sdk_ec2::operation::get_associated_enclave_certificate_iam_roles::{builders::*, *};
use aws_sdk_ec2::operation::get_associated_ipv6_pool_cidrs::{builders::*, *};
use aws_sdk_ec2::operation::get_aws_network_performance_data::{builders::*, *};
use aws_sdk_ec2::operation::get_capacity_reservation_usage::{builders::*, *};
use aws_sdk_ec2::operation::get_coip_pool_usage::{builders::*, *};
use aws_sdk_ec2::operation::get_console_output::{builders::*, *};
use aws_sdk_ec2::operation::get_console_screenshot::{builders::*, *};
use aws_sdk_ec2::operation::get_default_credit_specification::{builders::*, *};
use aws_sdk_ec2::operation::get_ebs_default_kms_key_id::{builders::*, *};
use aws_sdk_ec2::operation::get_ebs_encryption_by_default::{builders::*, *};
use aws_sdk_ec2::operation::get_flow_logs_integration_template::{builders::*, *};
use aws_sdk_ec2::operation::get_groups_for_capacity_reservation::{builders::*, *};
use aws_sdk_ec2::operation::get_host_reservation_purchase_preview::{builders::*, *};
use aws_sdk_ec2::operation::get_image_block_public_access_state::{builders::*, *};
use aws_sdk_ec2::operation::get_instance_metadata_defaults::{builders::*, *};
use aws_sdk_ec2::operation::get_instance_tpm_ek_pub::{builders::*, *};
use aws_sdk_ec2::operation::get_instance_types_from_instance_requirements::{builders::*, *};
use aws_sdk_ec2::operation::get_instance_uefi_data::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_address_history::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_discovered_accounts::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_discovered_public_addresses::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_discovered_resource_cidrs::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_pool_allocations::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_pool_cidrs::{builders::*, *};
use aws_sdk_ec2::operation::get_ipam_resource_cidrs::{builders::*, *};
use aws_sdk_ec2::operation::get_launch_template_data::{builders::*, *};
use aws_sdk_ec2::operation::get_managed_prefix_list_associations::{builders::*, *};
use aws_sdk_ec2::operation::get_managed_prefix_list_entries::{builders::*, *};
use aws_sdk_ec2::operation::get_network_insights_access_scope_analysis_findings::{builders::*, *};
use aws_sdk_ec2::operation::get_network_insights_access_scope_content::{builders::*, *};
use aws_sdk_ec2::operation::get_password_data::{builders::*, *};
use aws_sdk_ec2::operation::get_reserved_instances_exchange_quote::{builders::*, *};
use aws_sdk_ec2::operation::get_security_groups_for_vpc::{builders::*, *};
use aws_sdk_ec2::operation::get_serial_console_access_status::{builders::*, *};
use aws_sdk_ec2::operation::get_snapshot_block_public_access_state::{builders::*, *};
use aws_sdk_ec2::operation::get_spot_placement_scores::{builders::*, *};
use aws_sdk_ec2::operation::get_subnet_cidr_reservations::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_attachment_propagations::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_multicast_domain_associations::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_policy_table_associations::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_policy_table_entries::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_prefix_list_references::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_route_table_associations::{builders::*, *};
use aws_sdk_ec2::operation::get_transit_gateway_route_table_propagations::{builders::*, *};
use aws_sdk_ec2::operation::get_verified_access_endpoint_policy::{builders::*, *};
use aws_sdk_ec2::operation::get_verified_access_group_policy::{builders::*, *};
use aws_sdk_ec2::operation::get_vpn_connection_device_sample_configuration::{builders::*, *};
use aws_sdk_ec2::operation::get_vpn_connection_device_types::{builders::*, *};
use aws_sdk_ec2::operation::get_vpn_tunnel_replacement_status::{builders::*, *};
use aws_sdk_ec2::operation::import_client_vpn_client_certificate_revocation_list::{builders::*, *};
use aws_sdk_ec2::operation::import_image::{builders::*, *};
use aws_sdk_ec2::operation::import_key_pair::{builders::*, *};
use aws_sdk_ec2::operation::import_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::list_images_in_recycle_bin::{builders::*, *};
use aws_sdk_ec2::operation::list_snapshots_in_recycle_bin::{builders::*, *};
use aws_sdk_ec2::operation::lock_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::modify_address_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_availability_zone_group::{builders::*, *};
use aws_sdk_ec2::operation::modify_capacity_reservation::{builders::*, *};
use aws_sdk_ec2::operation::modify_capacity_reservation_fleet::{builders::*, *};
use aws_sdk_ec2::operation::modify_client_vpn_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::modify_default_credit_specification::{builders::*, *};
use aws_sdk_ec2::operation::modify_ebs_default_kms_key_id::{builders::*, *};
use aws_sdk_ec2::operation::modify_fleet::{builders::*, *};
use aws_sdk_ec2::operation::modify_fpga_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_hosts::{builders::*, *};
use aws_sdk_ec2::operation::modify_id_format::{builders::*, *};
use aws_sdk_ec2::operation::modify_identity_id_format::{builders::*, *};
use aws_sdk_ec2::operation::modify_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_capacity_reservation_attributes::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_credit_specification::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_event_start_time::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_event_window::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_maintenance_options::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_metadata_defaults::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_metadata_options::{builders::*, *};
use aws_sdk_ec2::operation::modify_instance_placement::{builders::*, *};
use aws_sdk_ec2::operation::modify_ipam::{builders::*, *};
use aws_sdk_ec2::operation::modify_ipam_pool::{builders::*, *};
use aws_sdk_ec2::operation::modify_ipam_resource_cidr::{builders::*, *};
use aws_sdk_ec2::operation::modify_ipam_resource_discovery::{builders::*, *};
use aws_sdk_ec2::operation::modify_ipam_scope::{builders::*, *};
use aws_sdk_ec2::operation::modify_launch_template::{builders::*, *};
use aws_sdk_ec2::operation::modify_local_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::modify_managed_prefix_list::{builders::*, *};
use aws_sdk_ec2::operation::modify_network_interface_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_private_dns_name_options::{builders::*, *};
use aws_sdk_ec2::operation::modify_reserved_instances::{builders::*, *};
use aws_sdk_ec2::operation::modify_security_group_rules::{builders::*, *};
use aws_sdk_ec2::operation::modify_snapshot_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_snapshot_tier::{builders::*, *};
use aws_sdk_ec2::operation::modify_spot_fleet_request::{builders::*, *};
use aws_sdk_ec2::operation::modify_subnet_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_traffic_mirror_filter_network_services::{builders::*, *};
use aws_sdk_ec2::operation::modify_traffic_mirror_filter_rule::{builders::*, *};
use aws_sdk_ec2::operation::modify_traffic_mirror_session::{builders::*, *};
use aws_sdk_ec2::operation::modify_transit_gateway::{builders::*, *};
use aws_sdk_ec2::operation::modify_transit_gateway_prefix_list_reference::{builders::*, *};
use aws_sdk_ec2::operation::modify_transit_gateway_vpc_attachment::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_endpoint_policy::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_group::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_group_policy::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_instance::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_instance_logging_configuration::{builders::*, *};
use aws_sdk_ec2::operation::modify_verified_access_trust_provider::{builders::*, *};
use aws_sdk_ec2::operation::modify_volume::{builders::*, *};
use aws_sdk_ec2::operation::modify_volume_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_attribute::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_endpoint::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_endpoint_connection_notification::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_endpoint_service_configuration::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_endpoint_service_payer_responsibility::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_endpoint_service_permissions::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_peering_connection_options::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpc_tenancy::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpn_connection::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpn_connection_options::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpn_tunnel_certificate::{builders::*, *};
use aws_sdk_ec2::operation::modify_vpn_tunnel_options::{builders::*, *};
use aws_sdk_ec2::operation::monitor_instances::{builders::*, *};
use aws_sdk_ec2::operation::move_address_to_vpc::{builders::*, *};
use aws_sdk_ec2::operation::move_byoip_cidr_to_ipam::{builders::*, *};
use aws_sdk_ec2::operation::provision_byoip_cidr::{builders::*, *};
use aws_sdk_ec2::operation::provision_ipam_byoasn::{builders::*, *};
use aws_sdk_ec2::operation::provision_ipam_pool_cidr::{builders::*, *};
use aws_sdk_ec2::operation::provision_public_ipv4_pool_cidr::{builders::*, *};
use aws_sdk_ec2::operation::purchase_capacity_block::{builders::*, *};
use aws_sdk_ec2::operation::purchase_host_reservation::{builders::*, *};
use aws_sdk_ec2::operation::purchase_reserved_instances_offering::{builders::*, *};
use aws_sdk_ec2::operation::purchase_scheduled_instances::{builders::*, *};
use aws_sdk_ec2::operation::reboot_instances::{builders::*, *};
use aws_sdk_ec2::operation::register_image::{builders::*, *};
use aws_sdk_ec2::operation::register_instance_event_notification_attributes::{builders::*, *};
use aws_sdk_ec2::operation::register_transit_gateway_multicast_group_members::{builders::*, *};
use aws_sdk_ec2::operation::register_transit_gateway_multicast_group_sources::{builders::*, *};
use aws_sdk_ec2::operation::reject_transit_gateway_multicast_domain_associations::{builders::*, *};
use aws_sdk_ec2::operation::reject_transit_gateway_peering_attachment::{builders::*, *};
use aws_sdk_ec2::operation::reject_transit_gateway_vpc_attachment::{builders::*, *};
use aws_sdk_ec2::operation::reject_vpc_endpoint_connections::{builders::*, *};
use aws_sdk_ec2::operation::reject_vpc_peering_connection::{builders::*, *};
use aws_sdk_ec2::operation::release_address::{builders::*, *};
use aws_sdk_ec2::operation::release_hosts::{builders::*, *};
use aws_sdk_ec2::operation::release_ipam_pool_allocation::{builders::*, *};
use aws_sdk_ec2::operation::replace_iam_instance_profile_association::{builders::*, *};
use aws_sdk_ec2::operation::replace_network_acl_association::{builders::*, *};
use aws_sdk_ec2::operation::replace_network_acl_entry::{builders::*, *};
use aws_sdk_ec2::operation::replace_route::{builders::*, *};
use aws_sdk_ec2::operation::replace_route_table_association::{builders::*, *};
use aws_sdk_ec2::operation::replace_transit_gateway_route::{builders::*, *};
use aws_sdk_ec2::operation::replace_vpn_tunnel::{builders::*, *};
use aws_sdk_ec2::operation::report_instance_status::{builders::*, *};
use aws_sdk_ec2::operation::request_spot_fleet::{builders::*, *};
use aws_sdk_ec2::operation::request_spot_instances::{builders::*, *};
use aws_sdk_ec2::operation::reset_address_attribute::{builders::*, *};
use aws_sdk_ec2::operation::reset_ebs_default_kms_key_id::{builders::*, *};
use aws_sdk_ec2::operation::reset_fpga_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::reset_image_attribute::{builders::*, *};
use aws_sdk_ec2::operation::reset_instance_attribute::{builders::*, *};
use aws_sdk_ec2::operation::reset_network_interface_attribute::{builders::*, *};
use aws_sdk_ec2::operation::reset_snapshot_attribute::{builders::*, *};
use aws_sdk_ec2::operation::restore_address_to_classic::{builders::*, *};
use aws_sdk_ec2::operation::restore_image_from_recycle_bin::{builders::*, *};
use aws_sdk_ec2::operation::restore_managed_prefix_list_version::{builders::*, *};
use aws_sdk_ec2::operation::restore_snapshot_from_recycle_bin::{builders::*, *};
use aws_sdk_ec2::operation::restore_snapshot_tier::{builders::*, *};
use aws_sdk_ec2::operation::revoke_client_vpn_ingress::{builders::*, *};
use aws_sdk_ec2::operation::revoke_security_group_egress::{builders::*, *};
use aws_sdk_ec2::operation::revoke_security_group_ingress::{builders::*, *};
use aws_sdk_ec2::operation::run_instances::{builders::*, *};
use aws_sdk_ec2::operation::run_scheduled_instances::{builders::*, *};
use aws_sdk_ec2::operation::search_local_gateway_routes::{builders::*, *};
use aws_sdk_ec2::operation::search_transit_gateway_multicast_groups::{builders::*, *};
use aws_sdk_ec2::operation::search_transit_gateway_routes::{builders::*, *};
use aws_sdk_ec2::operation::send_diagnostic_interrupt::{builders::*, *};
use aws_sdk_ec2::operation::start_instances::{builders::*, *};
use aws_sdk_ec2::operation::start_network_insights_access_scope_analysis::{builders::*, *};
use aws_sdk_ec2::operation::start_network_insights_analysis::{builders::*, *};
use aws_sdk_ec2::operation::start_vpc_endpoint_service_private_dns_verification::{builders::*, *};
use aws_sdk_ec2::operation::stop_instances::{builders::*, *};
use aws_sdk_ec2::operation::terminate_client_vpn_connections::{builders::*, *};
use aws_sdk_ec2::operation::terminate_instances::{builders::*, *};
use aws_sdk_ec2::operation::unassign_ipv6_addresses::{builders::*, *};
use aws_sdk_ec2::operation::unassign_private_ip_addresses::{builders::*, *};
use aws_sdk_ec2::operation::unassign_private_nat_gateway_address::{builders::*, *};
use aws_sdk_ec2::operation::unlock_snapshot::{builders::*, *};
use aws_sdk_ec2::operation::unmonitor_instances::{builders::*, *};
use aws_sdk_ec2::operation::update_security_group_rule_descriptions_egress::{builders::*, *};
use aws_sdk_ec2::operation::update_security_group_rule_descriptions_ingress::{builders::*, *};
use aws_sdk_ec2::operation::withdraw_byoip_cidr::{builders::*, *};
use aws_sdk_ec2::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_ec2::Client;

pub use aws_sdk_ec2::*;

pub struct EC2ClientImpl(Client);
impl EC2ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait EC2Client {
    fn accept_address_transfer(&self, builder: AcceptAddressTransferInputBuilder) -> impl Future<Output = Result<AcceptAddressTransferOutput, SdkError<AcceptAddressTransferError>>>;
    fn accept_reserved_instances_exchange_quote(&self, builder: AcceptReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<AcceptReservedInstancesExchangeQuoteOutput, SdkError<AcceptReservedInstancesExchangeQuoteError>>>;
    fn accept_transit_gateway_multicast_domain_associations(&self, builder: AcceptTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayMulticastDomainAssociationsOutput, SdkError<AcceptTransitGatewayMulticastDomainAssociationsError>>>;
    fn accept_transit_gateway_peering_attachment(&self, builder: AcceptTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayPeeringAttachmentOutput, SdkError<AcceptTransitGatewayPeeringAttachmentError>>>;
    fn accept_transit_gateway_vpc_attachment(&self, builder: AcceptTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayVpcAttachmentOutput, SdkError<AcceptTransitGatewayVpcAttachmentError>>>;
    fn accept_vpc_endpoint_connections(&self, builder: AcceptVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<AcceptVpcEndpointConnectionsOutput, SdkError<AcceptVpcEndpointConnectionsError>>>;
    fn accept_vpc_peering_connection(&self, builder: AcceptVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<AcceptVpcPeeringConnectionOutput, SdkError<AcceptVpcPeeringConnectionError>>>;
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>>;
    fn allocate_address(&self, builder: AllocateAddressInputBuilder) -> impl Future<Output = Result<AllocateAddressOutput, SdkError<AllocateAddressError>>>;
    fn allocate_hosts(&self, builder: AllocateHostsInputBuilder) -> impl Future<Output = Result<AllocateHostsOutput, SdkError<AllocateHostsError>>>;
    fn allocate_ipam_pool_cidr(&self, builder: AllocateIpamPoolCidrInputBuilder) -> impl Future<Output = Result<AllocateIpamPoolCidrOutput, SdkError<AllocateIpamPoolCidrError>>>;
    fn apply_security_groups_to_client_vpn_target_network(&self, builder: ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<ApplySecurityGroupsToClientVpnTargetNetworkOutput, SdkError<ApplySecurityGroupsToClientVpnTargetNetworkError>>>;
    fn assign_ipv6_addresses(&self, builder: AssignIpv6AddressesInputBuilder) -> impl Future<Output = Result<AssignIpv6AddressesOutput, SdkError<AssignIpv6AddressesError>>>;
    fn assign_private_ip_addresses(&self, builder: AssignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<AssignPrivateIpAddressesOutput, SdkError<AssignPrivateIpAddressesError>>>;
    fn assign_private_nat_gateway_address(&self, builder: AssignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssignPrivateNatGatewayAddressOutput, SdkError<AssignPrivateNatGatewayAddressError>>>;
    fn associate_address(&self, builder: AssociateAddressInputBuilder) -> impl Future<Output = Result<AssociateAddressOutput, SdkError<AssociateAddressError>>>;
    fn associate_client_vpn_target_network(&self, builder: AssociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<AssociateClientVpnTargetNetworkOutput, SdkError<AssociateClientVpnTargetNetworkError>>>;
    fn associate_dhcp_options(&self, builder: AssociateDhcpOptionsInputBuilder) -> impl Future<Output = Result<AssociateDhcpOptionsOutput, SdkError<AssociateDhcpOptionsError>>>;
    fn associate_enclave_certificate_iam_role(&self, builder: AssociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<AssociateEnclaveCertificateIamRoleOutput, SdkError<AssociateEnclaveCertificateIamRoleError>>>;
    fn associate_iam_instance_profile(&self, builder: AssociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<AssociateIamInstanceProfileOutput, SdkError<AssociateIamInstanceProfileError>>>;
    fn associate_instance_event_window(&self, builder: AssociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<AssociateInstanceEventWindowOutput, SdkError<AssociateInstanceEventWindowError>>>;
    fn associate_ipam_byoasn(&self, builder: AssociateIpamByoasnInputBuilder) -> impl Future<Output = Result<AssociateIpamByoasnOutput, SdkError<AssociateIpamByoasnError>>>;
    fn associate_ipam_resource_discovery(&self, builder: AssociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<AssociateIpamResourceDiscoveryOutput, SdkError<AssociateIpamResourceDiscoveryError>>>;
    fn associate_nat_gateway_address(&self, builder: AssociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssociateNatGatewayAddressOutput, SdkError<AssociateNatGatewayAddressError>>>;
    fn associate_route_table(&self, builder: AssociateRouteTableInputBuilder) -> impl Future<Output = Result<AssociateRouteTableOutput, SdkError<AssociateRouteTableError>>>;
    fn associate_subnet_cidr_block(&self, builder: AssociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateSubnetCidrBlockOutput, SdkError<AssociateSubnetCidrBlockError>>>;
    fn associate_transit_gateway_multicast_domain(&self, builder: AssociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayMulticastDomainOutput, SdkError<AssociateTransitGatewayMulticastDomainError>>>;
    fn associate_transit_gateway_policy_table(&self, builder: AssociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayPolicyTableOutput, SdkError<AssociateTransitGatewayPolicyTableError>>>;
    fn associate_transit_gateway_route_table(&self, builder: AssociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayRouteTableOutput, SdkError<AssociateTransitGatewayRouteTableError>>>;
    fn associate_trunk_interface(&self, builder: AssociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<AssociateTrunkInterfaceOutput, SdkError<AssociateTrunkInterfaceError>>>;
    fn associate_vpc_cidr_block(&self, builder: AssociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateVpcCidrBlockOutput, SdkError<AssociateVpcCidrBlockError>>>;
    fn attach_classic_link_vpc(&self, builder: AttachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<AttachClassicLinkVpcOutput, SdkError<AttachClassicLinkVpcError>>>;
    fn attach_internet_gateway(&self, builder: AttachInternetGatewayInputBuilder) -> impl Future<Output = Result<AttachInternetGatewayOutput, SdkError<AttachInternetGatewayError>>>;
    fn attach_network_interface(&self, builder: AttachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<AttachNetworkInterfaceOutput, SdkError<AttachNetworkInterfaceError>>>;
    fn attach_verified_access_trust_provider(&self, builder: AttachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<AttachVerifiedAccessTrustProviderOutput, SdkError<AttachVerifiedAccessTrustProviderError>>>;
    fn attach_volume(&self, builder: AttachVolumeInputBuilder) -> impl Future<Output = Result<AttachVolumeOutput, SdkError<AttachVolumeError>>>;
    fn attach_vpn_gateway(&self, builder: AttachVpnGatewayInputBuilder) -> impl Future<Output = Result<AttachVpnGatewayOutput, SdkError<AttachVpnGatewayError>>>;
    fn authorize_client_vpn_ingress(&self, builder: AuthorizeClientVpnIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClientVpnIngressOutput, SdkError<AuthorizeClientVpnIngressError>>>;
    fn authorize_security_group_egress(&self, builder: AuthorizeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupEgressOutput, SdkError<AuthorizeSecurityGroupEgressError>>>;
    fn authorize_security_group_ingress(&self, builder: AuthorizeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupIngressOutput, SdkError<AuthorizeSecurityGroupIngressError>>>;
    fn bundle_instance(&self, builder: BundleInstanceInputBuilder) -> impl Future<Output = Result<BundleInstanceOutput, SdkError<BundleInstanceError>>>;
    fn cancel_bundle_task(&self, builder: CancelBundleTaskInputBuilder) -> impl Future<Output = Result<CancelBundleTaskOutput, SdkError<CancelBundleTaskError>>>;
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>>;
    fn cancel_capacity_reservation_fleets(&self, builder: CancelCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationFleetsOutput, SdkError<CancelCapacityReservationFleetsError>>>;
    fn cancel_conversion_task(&self, builder: CancelConversionTaskInputBuilder) -> impl Future<Output = Result<CancelConversionTaskOutput, SdkError<CancelConversionTaskError>>>;
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>>;
    fn cancel_image_launch_permission(&self, builder: CancelImageLaunchPermissionInputBuilder) -> impl Future<Output = Result<CancelImageLaunchPermissionOutput, SdkError<CancelImageLaunchPermissionError>>>;
    fn cancel_import_task(&self, builder: CancelImportTaskInputBuilder) -> impl Future<Output = Result<CancelImportTaskOutput, SdkError<CancelImportTaskError>>>;
    fn cancel_reserved_instances_listing(&self, builder: CancelReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CancelReservedInstancesListingOutput, SdkError<CancelReservedInstancesListingError>>>;
    fn cancel_spot_fleet_requests(&self, builder: CancelSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotFleetRequestsOutput, SdkError<CancelSpotFleetRequestsError>>>;
    fn cancel_spot_instance_requests(&self, builder: CancelSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotInstanceRequestsOutput, SdkError<CancelSpotInstanceRequestsError>>>;
    fn confirm_product_instance(&self, builder: ConfirmProductInstanceInputBuilder) -> impl Future<Output = Result<ConfirmProductInstanceOutput, SdkError<ConfirmProductInstanceError>>>;
    fn copy_fpga_image(&self, builder: CopyFpgaImageInputBuilder) -> impl Future<Output = Result<CopyFpgaImageOutput, SdkError<CopyFpgaImageError>>>;
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>>;
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>>;
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>>;
    fn create_capacity_reservation_fleet(&self, builder: CreateCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationFleetOutput, SdkError<CreateCapacityReservationFleetError>>>;
    fn create_carrier_gateway(&self, builder: CreateCarrierGatewayInputBuilder) -> impl Future<Output = Result<CreateCarrierGatewayOutput, SdkError<CreateCarrierGatewayError>>>;
    fn create_client_vpn_endpoint(&self, builder: CreateClientVpnEndpointInputBuilder) -> impl Future<Output = Result<CreateClientVpnEndpointOutput, SdkError<CreateClientVpnEndpointError>>>;
    fn create_client_vpn_route(&self, builder: CreateClientVpnRouteInputBuilder) -> impl Future<Output = Result<CreateClientVpnRouteOutput, SdkError<CreateClientVpnRouteError>>>;
    fn create_coip_cidr(&self, builder: CreateCoipCidrInputBuilder) -> impl Future<Output = Result<CreateCoipCidrOutput, SdkError<CreateCoipCidrError>>>;
    fn create_coip_pool(&self, builder: CreateCoipPoolInputBuilder) -> impl Future<Output = Result<CreateCoipPoolOutput, SdkError<CreateCoipPoolError>>>;
    fn create_customer_gateway(&self, builder: CreateCustomerGatewayInputBuilder) -> impl Future<Output = Result<CreateCustomerGatewayOutput, SdkError<CreateCustomerGatewayError>>>;
    fn create_default_subnet(&self, builder: CreateDefaultSubnetInputBuilder) -> impl Future<Output = Result<CreateDefaultSubnetOutput, SdkError<CreateDefaultSubnetError>>>;
    fn create_default_vpc(&self, builder: CreateDefaultVpcInputBuilder) -> impl Future<Output = Result<CreateDefaultVpcOutput, SdkError<CreateDefaultVpcError>>>;
    fn create_dhcp_options(&self, builder: CreateDhcpOptionsInputBuilder) -> impl Future<Output = Result<CreateDhcpOptionsOutput, SdkError<CreateDhcpOptionsError>>>;
    fn create_egress_only_internet_gateway(&self, builder: CreateEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateEgressOnlyInternetGatewayOutput, SdkError<CreateEgressOnlyInternetGatewayError>>>;
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>>;
    fn create_flow_logs(&self, builder: CreateFlowLogsInputBuilder) -> impl Future<Output = Result<CreateFlowLogsOutput, SdkError<CreateFlowLogsError>>>;
    fn create_fpga_image(&self, builder: CreateFpgaImageInputBuilder) -> impl Future<Output = Result<CreateFpgaImageOutput, SdkError<CreateFpgaImageError>>>;
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>>;
    fn create_instance_connect_endpoint(&self, builder: CreateInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<CreateInstanceConnectEndpointOutput, SdkError<CreateInstanceConnectEndpointError>>>;
    fn create_instance_event_window(&self, builder: CreateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<CreateInstanceEventWindowOutput, SdkError<CreateInstanceEventWindowError>>>;
    fn create_instance_export_task(&self, builder: CreateInstanceExportTaskInputBuilder) -> impl Future<Output = Result<CreateInstanceExportTaskOutput, SdkError<CreateInstanceExportTaskError>>>;
    fn create_internet_gateway(&self, builder: CreateInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateInternetGatewayOutput, SdkError<CreateInternetGatewayError>>>;
    fn create_ipam(&self, builder: CreateIpamInputBuilder) -> impl Future<Output = Result<CreateIpamOutput, SdkError<CreateIpamError>>>;
    fn create_ipam_external_resource_verification_token(&self, builder: CreateIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<CreateIpamExternalResourceVerificationTokenOutput, SdkError<CreateIpamExternalResourceVerificationTokenError>>>;
    fn create_ipam_pool(&self, builder: CreateIpamPoolInputBuilder) -> impl Future<Output = Result<CreateIpamPoolOutput, SdkError<CreateIpamPoolError>>>;
    fn create_ipam_resource_discovery(&self, builder: CreateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<CreateIpamResourceDiscoveryOutput, SdkError<CreateIpamResourceDiscoveryError>>>;
    fn create_ipam_scope(&self, builder: CreateIpamScopeInputBuilder) -> impl Future<Output = Result<CreateIpamScopeOutput, SdkError<CreateIpamScopeError>>>;
    fn create_key_pair(&self, builder: CreateKeyPairInputBuilder) -> impl Future<Output = Result<CreateKeyPairOutput, SdkError<CreateKeyPairError>>>;
    fn create_launch_template(&self, builder: CreateLaunchTemplateInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateOutput, SdkError<CreateLaunchTemplateError>>>;
    fn create_launch_template_version(&self, builder: CreateLaunchTemplateVersionInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateVersionOutput, SdkError<CreateLaunchTemplateVersionError>>>;
    fn create_local_gateway_route(&self, builder: CreateLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteOutput, SdkError<CreateLocalGatewayRouteError>>>;
    fn create_local_gateway_route_table(&self, builder: CreateLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableOutput, SdkError<CreateLocalGatewayRouteTableError>>>;
    fn create_local_gateway_route_table_virtual_interface_group_association(&self, builder: CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>>;
    fn create_local_gateway_route_table_vpc_association(&self, builder: CreateLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVpcAssociationOutput, SdkError<CreateLocalGatewayRouteTableVpcAssociationError>>>;
    fn create_managed_prefix_list(&self, builder: CreateManagedPrefixListInputBuilder) -> impl Future<Output = Result<CreateManagedPrefixListOutput, SdkError<CreateManagedPrefixListError>>>;
    fn create_nat_gateway(&self, builder: CreateNatGatewayInputBuilder) -> impl Future<Output = Result<CreateNatGatewayOutput, SdkError<CreateNatGatewayError>>>;
    fn create_network_acl(&self, builder: CreateNetworkAclInputBuilder) -> impl Future<Output = Result<CreateNetworkAclOutput, SdkError<CreateNetworkAclError>>>;
    fn create_network_acl_entry(&self, builder: CreateNetworkAclEntryInputBuilder) -> impl Future<Output = Result<CreateNetworkAclEntryOutput, SdkError<CreateNetworkAclEntryError>>>;
    fn create_network_insights_access_scope(&self, builder: CreateNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsAccessScopeOutput, SdkError<CreateNetworkInsightsAccessScopeError>>>;
    fn create_network_insights_path(&self, builder: CreateNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsPathOutput, SdkError<CreateNetworkInsightsPathError>>>;
    fn create_network_interface(&self, builder: CreateNetworkInterfaceInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfaceOutput, SdkError<CreateNetworkInterfaceError>>>;
    fn create_network_interface_permission(&self, builder: CreateNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfacePermissionOutput, SdkError<CreateNetworkInterfacePermissionError>>>;
    fn create_placement_group(&self, builder: CreatePlacementGroupInputBuilder) -> impl Future<Output = Result<CreatePlacementGroupOutput, SdkError<CreatePlacementGroupError>>>;
    fn create_public_ipv4_pool(&self, builder: CreatePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<CreatePublicIpv4PoolOutput, SdkError<CreatePublicIpv4PoolError>>>;
    fn create_replace_root_volume_task(&self, builder: CreateReplaceRootVolumeTaskInputBuilder) -> impl Future<Output = Result<CreateReplaceRootVolumeTaskOutput, SdkError<CreateReplaceRootVolumeTaskError>>>;
    fn create_reserved_instances_listing(&self, builder: CreateReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CreateReservedInstancesListingOutput, SdkError<CreateReservedInstancesListingError>>>;
    fn create_restore_image_task(&self, builder: CreateRestoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateRestoreImageTaskOutput, SdkError<CreateRestoreImageTaskError>>>;
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>>;
    fn create_route_table(&self, builder: CreateRouteTableInputBuilder) -> impl Future<Output = Result<CreateRouteTableOutput, SdkError<CreateRouteTableError>>>;
    fn create_security_group(&self, builder: CreateSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateSecurityGroupOutput, SdkError<CreateSecurityGroupError>>>;
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>>;
    fn create_snapshots(&self, builder: CreateSnapshotsInputBuilder) -> impl Future<Output = Result<CreateSnapshotsOutput, SdkError<CreateSnapshotsError>>>;
    fn create_spot_datafeed_subscription(&self, builder: CreateSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<CreateSpotDatafeedSubscriptionOutput, SdkError<CreateSpotDatafeedSubscriptionError>>>;
    fn create_store_image_task(&self, builder: CreateStoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateStoreImageTaskOutput, SdkError<CreateStoreImageTaskError>>>;
    fn create_subnet(&self, builder: CreateSubnetInputBuilder) -> impl Future<Output = Result<CreateSubnetOutput, SdkError<CreateSubnetError>>>;
    fn create_subnet_cidr_reservation(&self, builder: CreateSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<CreateSubnetCidrReservationOutput, SdkError<CreateSubnetCidrReservationError>>>;
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>>;
    fn create_traffic_mirror_filter(&self, builder: CreateTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterOutput, SdkError<CreateTrafficMirrorFilterError>>>;
    fn create_traffic_mirror_filter_rule(&self, builder: CreateTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterRuleOutput, SdkError<CreateTrafficMirrorFilterRuleError>>>;
    fn create_traffic_mirror_session(&self, builder: CreateTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorSessionOutput, SdkError<CreateTrafficMirrorSessionError>>>;
    fn create_traffic_mirror_target(&self, builder: CreateTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorTargetOutput, SdkError<CreateTrafficMirrorTargetError>>>;
    fn create_transit_gateway(&self, builder: CreateTransitGatewayInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayOutput, SdkError<CreateTransitGatewayError>>>;
    fn create_transit_gateway_connect(&self, builder: CreateTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectOutput, SdkError<CreateTransitGatewayConnectError>>>;
    fn create_transit_gateway_connect_peer(&self, builder: CreateTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectPeerOutput, SdkError<CreateTransitGatewayConnectPeerError>>>;
    fn create_transit_gateway_multicast_domain(&self, builder: CreateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayMulticastDomainOutput, SdkError<CreateTransitGatewayMulticastDomainError>>>;
    fn create_transit_gateway_peering_attachment(&self, builder: CreateTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPeeringAttachmentOutput, SdkError<CreateTransitGatewayPeeringAttachmentError>>>;
    fn create_transit_gateway_policy_table(&self, builder: CreateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPolicyTableOutput, SdkError<CreateTransitGatewayPolicyTableError>>>;
    fn create_transit_gateway_prefix_list_reference(&self, builder: CreateTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPrefixListReferenceOutput, SdkError<CreateTransitGatewayPrefixListReferenceError>>>;
    fn create_transit_gateway_route(&self, builder: CreateTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteOutput, SdkError<CreateTransitGatewayRouteError>>>;
    fn create_transit_gateway_route_table(&self, builder: CreateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableOutput, SdkError<CreateTransitGatewayRouteTableError>>>;
    fn create_transit_gateway_route_table_announcement(&self, builder: CreateTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableAnnouncementOutput, SdkError<CreateTransitGatewayRouteTableAnnouncementError>>>;
    fn create_transit_gateway_vpc_attachment(&self, builder: CreateTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayVpcAttachmentOutput, SdkError<CreateTransitGatewayVpcAttachmentError>>>;
    fn create_verified_access_endpoint(&self, builder: CreateVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessEndpointOutput, SdkError<CreateVerifiedAccessEndpointError>>>;
    fn create_verified_access_group(&self, builder: CreateVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessGroupOutput, SdkError<CreateVerifiedAccessGroupError>>>;
    fn create_verified_access_instance(&self, builder: CreateVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessInstanceOutput, SdkError<CreateVerifiedAccessInstanceError>>>;
    fn create_verified_access_trust_provider(&self, builder: CreateVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessTrustProviderOutput, SdkError<CreateVerifiedAccessTrustProviderError>>>;
    fn create_volume(&self, builder: CreateVolumeInputBuilder) -> impl Future<Output = Result<CreateVolumeOutput, SdkError<CreateVolumeError>>>;
    fn create_vpc(&self, builder: CreateVpcInputBuilder) -> impl Future<Output = Result<CreateVpcOutput, SdkError<CreateVpcError>>>;
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>>;
    fn create_vpc_endpoint_connection_notification(&self, builder: CreateVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointConnectionNotificationOutput, SdkError<CreateVpcEndpointConnectionNotificationError>>>;
    fn create_vpc_endpoint_service_configuration(&self, builder: CreateVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointServiceConfigurationOutput, SdkError<CreateVpcEndpointServiceConfigurationError>>>;
    fn create_vpc_peering_connection(&self, builder: CreateVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcPeeringConnectionOutput, SdkError<CreateVpcPeeringConnectionError>>>;
    fn create_vpn_connection(&self, builder: CreateVpnConnectionInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionOutput, SdkError<CreateVpnConnectionError>>>;
    fn create_vpn_connection_route(&self, builder: CreateVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionRouteOutput, SdkError<CreateVpnConnectionRouteError>>>;
    fn create_vpn_gateway(&self, builder: CreateVpnGatewayInputBuilder) -> impl Future<Output = Result<CreateVpnGatewayOutput, SdkError<CreateVpnGatewayError>>>;
    fn delete_carrier_gateway(&self, builder: DeleteCarrierGatewayInputBuilder) -> impl Future<Output = Result<DeleteCarrierGatewayOutput, SdkError<DeleteCarrierGatewayError>>>;
    fn delete_client_vpn_endpoint(&self, builder: DeleteClientVpnEndpointInputBuilder) -> impl Future<Output = Result<DeleteClientVpnEndpointOutput, SdkError<DeleteClientVpnEndpointError>>>;
    fn delete_client_vpn_route(&self, builder: DeleteClientVpnRouteInputBuilder) -> impl Future<Output = Result<DeleteClientVpnRouteOutput, SdkError<DeleteClientVpnRouteError>>>;
    fn delete_coip_cidr(&self, builder: DeleteCoipCidrInputBuilder) -> impl Future<Output = Result<DeleteCoipCidrOutput, SdkError<DeleteCoipCidrError>>>;
    fn delete_coip_pool(&self, builder: DeleteCoipPoolInputBuilder) -> impl Future<Output = Result<DeleteCoipPoolOutput, SdkError<DeleteCoipPoolError>>>;
    fn delete_customer_gateway(&self, builder: DeleteCustomerGatewayInputBuilder) -> impl Future<Output = Result<DeleteCustomerGatewayOutput, SdkError<DeleteCustomerGatewayError>>>;
    fn delete_dhcp_options(&self, builder: DeleteDhcpOptionsInputBuilder) -> impl Future<Output = Result<DeleteDhcpOptionsOutput, SdkError<DeleteDhcpOptionsError>>>;
    fn delete_egress_only_internet_gateway(&self, builder: DeleteEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteEgressOnlyInternetGatewayOutput, SdkError<DeleteEgressOnlyInternetGatewayError>>>;
    fn delete_fleets(&self, builder: DeleteFleetsInputBuilder) -> impl Future<Output = Result<DeleteFleetsOutput, SdkError<DeleteFleetsError>>>;
    fn delete_flow_logs(&self, builder: DeleteFlowLogsInputBuilder) -> impl Future<Output = Result<DeleteFlowLogsOutput, SdkError<DeleteFlowLogsError>>>;
    fn delete_fpga_image(&self, builder: DeleteFpgaImageInputBuilder) -> impl Future<Output = Result<DeleteFpgaImageOutput, SdkError<DeleteFpgaImageError>>>;
    fn delete_instance_connect_endpoint(&self, builder: DeleteInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<DeleteInstanceConnectEndpointOutput, SdkError<DeleteInstanceConnectEndpointError>>>;
    fn delete_instance_event_window(&self, builder: DeleteInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DeleteInstanceEventWindowOutput, SdkError<DeleteInstanceEventWindowError>>>;
    fn delete_internet_gateway(&self, builder: DeleteInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteInternetGatewayOutput, SdkError<DeleteInternetGatewayError>>>;
    fn delete_ipam(&self, builder: DeleteIpamInputBuilder) -> impl Future<Output = Result<DeleteIpamOutput, SdkError<DeleteIpamError>>>;
    fn delete_ipam_external_resource_verification_token(&self, builder: DeleteIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<DeleteIpamExternalResourceVerificationTokenOutput, SdkError<DeleteIpamExternalResourceVerificationTokenError>>>;
    fn delete_ipam_pool(&self, builder: DeleteIpamPoolInputBuilder) -> impl Future<Output = Result<DeleteIpamPoolOutput, SdkError<DeleteIpamPoolError>>>;
    fn delete_ipam_resource_discovery(&self, builder: DeleteIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DeleteIpamResourceDiscoveryOutput, SdkError<DeleteIpamResourceDiscoveryError>>>;
    fn delete_ipam_scope(&self, builder: DeleteIpamScopeInputBuilder) -> impl Future<Output = Result<DeleteIpamScopeOutput, SdkError<DeleteIpamScopeError>>>;
    fn delete_key_pair(&self, builder: DeleteKeyPairInputBuilder) -> impl Future<Output = Result<DeleteKeyPairOutput, SdkError<DeleteKeyPairError>>>;
    fn delete_launch_template(&self, builder: DeleteLaunchTemplateInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateOutput, SdkError<DeleteLaunchTemplateError>>>;
    fn delete_launch_template_versions(&self, builder: DeleteLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateVersionsOutput, SdkError<DeleteLaunchTemplateVersionsError>>>;
    fn delete_local_gateway_route(&self, builder: DeleteLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteOutput, SdkError<DeleteLocalGatewayRouteError>>>;
    fn delete_local_gateway_route_table(&self, builder: DeleteLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableOutput, SdkError<DeleteLocalGatewayRouteTableError>>>;
    fn delete_local_gateway_route_table_virtual_interface_group_association(&self, builder: DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>>;
    fn delete_local_gateway_route_table_vpc_association(&self, builder: DeleteLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVpcAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVpcAssociationError>>>;
    fn delete_managed_prefix_list(&self, builder: DeleteManagedPrefixListInputBuilder) -> impl Future<Output = Result<DeleteManagedPrefixListOutput, SdkError<DeleteManagedPrefixListError>>>;
    fn delete_nat_gateway(&self, builder: DeleteNatGatewayInputBuilder) -> impl Future<Output = Result<DeleteNatGatewayOutput, SdkError<DeleteNatGatewayError>>>;
    fn delete_network_acl(&self, builder: DeleteNetworkAclInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclOutput, SdkError<DeleteNetworkAclError>>>;
    fn delete_network_acl_entry(&self, builder: DeleteNetworkAclEntryInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclEntryOutput, SdkError<DeleteNetworkAclEntryError>>>;
    fn delete_network_insights_access_scope(&self, builder: DeleteNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeOutput, SdkError<DeleteNetworkInsightsAccessScopeError>>>;
    fn delete_network_insights_access_scope_analysis(&self, builder: DeleteNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeAnalysisOutput, SdkError<DeleteNetworkInsightsAccessScopeAnalysisError>>>;
    fn delete_network_insights_analysis(&self, builder: DeleteNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAnalysisOutput, SdkError<DeleteNetworkInsightsAnalysisError>>>;
    fn delete_network_insights_path(&self, builder: DeleteNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsPathOutput, SdkError<DeleteNetworkInsightsPathError>>>;
    fn delete_network_interface(&self, builder: DeleteNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfaceOutput, SdkError<DeleteNetworkInterfaceError>>>;
    fn delete_network_interface_permission(&self, builder: DeleteNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfacePermissionOutput, SdkError<DeleteNetworkInterfacePermissionError>>>;
    fn delete_placement_group(&self, builder: DeletePlacementGroupInputBuilder) -> impl Future<Output = Result<DeletePlacementGroupOutput, SdkError<DeletePlacementGroupError>>>;
    fn delete_public_ipv4_pool(&self, builder: DeletePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<DeletePublicIpv4PoolOutput, SdkError<DeletePublicIpv4PoolError>>>;
    fn delete_queued_reserved_instances(&self, builder: DeleteQueuedReservedInstancesInputBuilder) -> impl Future<Output = Result<DeleteQueuedReservedInstancesOutput, SdkError<DeleteQueuedReservedInstancesError>>>;
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>>;
    fn delete_route_table(&self, builder: DeleteRouteTableInputBuilder) -> impl Future<Output = Result<DeleteRouteTableOutput, SdkError<DeleteRouteTableError>>>;
    fn delete_security_group(&self, builder: DeleteSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteSecurityGroupOutput, SdkError<DeleteSecurityGroupError>>>;
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>>;
    fn delete_spot_datafeed_subscription(&self, builder: DeleteSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteSpotDatafeedSubscriptionOutput, SdkError<DeleteSpotDatafeedSubscriptionError>>>;
    fn delete_subnet(&self, builder: DeleteSubnetInputBuilder) -> impl Future<Output = Result<DeleteSubnetOutput, SdkError<DeleteSubnetError>>>;
    fn delete_subnet_cidr_reservation(&self, builder: DeleteSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<DeleteSubnetCidrReservationOutput, SdkError<DeleteSubnetCidrReservationError>>>;
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>>;
    fn delete_traffic_mirror_filter(&self, builder: DeleteTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterOutput, SdkError<DeleteTrafficMirrorFilterError>>>;
    fn delete_traffic_mirror_filter_rule(&self, builder: DeleteTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterRuleOutput, SdkError<DeleteTrafficMirrorFilterRuleError>>>;
    fn delete_traffic_mirror_session(&self, builder: DeleteTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorSessionOutput, SdkError<DeleteTrafficMirrorSessionError>>>;
    fn delete_traffic_mirror_target(&self, builder: DeleteTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorTargetOutput, SdkError<DeleteTrafficMirrorTargetError>>>;
    fn delete_transit_gateway(&self, builder: DeleteTransitGatewayInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayOutput, SdkError<DeleteTransitGatewayError>>>;
    fn delete_transit_gateway_connect(&self, builder: DeleteTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectOutput, SdkError<DeleteTransitGatewayConnectError>>>;
    fn delete_transit_gateway_connect_peer(&self, builder: DeleteTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectPeerOutput, SdkError<DeleteTransitGatewayConnectPeerError>>>;
    fn delete_transit_gateway_multicast_domain(&self, builder: DeleteTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayMulticastDomainOutput, SdkError<DeleteTransitGatewayMulticastDomainError>>>;
    fn delete_transit_gateway_peering_attachment(&self, builder: DeleteTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPeeringAttachmentOutput, SdkError<DeleteTransitGatewayPeeringAttachmentError>>>;
    fn delete_transit_gateway_policy_table(&self, builder: DeleteTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPolicyTableOutput, SdkError<DeleteTransitGatewayPolicyTableError>>>;
    fn delete_transit_gateway_prefix_list_reference(&self, builder: DeleteTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPrefixListReferenceOutput, SdkError<DeleteTransitGatewayPrefixListReferenceError>>>;
    fn delete_transit_gateway_route(&self, builder: DeleteTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteOutput, SdkError<DeleteTransitGatewayRouteError>>>;
    fn delete_transit_gateway_route_table(&self, builder: DeleteTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableOutput, SdkError<DeleteTransitGatewayRouteTableError>>>;
    fn delete_transit_gateway_route_table_announcement(&self, builder: DeleteTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableAnnouncementOutput, SdkError<DeleteTransitGatewayRouteTableAnnouncementError>>>;
    fn delete_transit_gateway_vpc_attachment(&self, builder: DeleteTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayVpcAttachmentOutput, SdkError<DeleteTransitGatewayVpcAttachmentError>>>;
    fn delete_verified_access_endpoint(&self, builder: DeleteVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessEndpointOutput, SdkError<DeleteVerifiedAccessEndpointError>>>;
    fn delete_verified_access_group(&self, builder: DeleteVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessGroupOutput, SdkError<DeleteVerifiedAccessGroupError>>>;
    fn delete_verified_access_instance(&self, builder: DeleteVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessInstanceOutput, SdkError<DeleteVerifiedAccessInstanceError>>>;
    fn delete_verified_access_trust_provider(&self, builder: DeleteVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessTrustProviderOutput, SdkError<DeleteVerifiedAccessTrustProviderError>>>;
    fn delete_volume(&self, builder: DeleteVolumeInputBuilder) -> impl Future<Output = Result<DeleteVolumeOutput, SdkError<DeleteVolumeError>>>;
    fn delete_vpc(&self, builder: DeleteVpcInputBuilder) -> impl Future<Output = Result<DeleteVpcOutput, SdkError<DeleteVpcError>>>;
    fn delete_vpc_endpoint_connection_notifications(&self, builder: DeleteVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointConnectionNotificationsOutput, SdkError<DeleteVpcEndpointConnectionNotificationsError>>>;
    fn delete_vpc_endpoint_service_configurations(&self, builder: DeleteVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointServiceConfigurationsOutput, SdkError<DeleteVpcEndpointServiceConfigurationsError>>>;
    fn delete_vpc_endpoints(&self, builder: DeleteVpcEndpointsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointsOutput, SdkError<DeleteVpcEndpointsError>>>;
    fn delete_vpc_peering_connection(&self, builder: DeleteVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcPeeringConnectionOutput, SdkError<DeleteVpcPeeringConnectionError>>>;
    fn delete_vpn_connection(&self, builder: DeleteVpnConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionOutput, SdkError<DeleteVpnConnectionError>>>;
    fn delete_vpn_connection_route(&self, builder: DeleteVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionRouteOutput, SdkError<DeleteVpnConnectionRouteError>>>;
    fn delete_vpn_gateway(&self, builder: DeleteVpnGatewayInputBuilder) -> impl Future<Output = Result<DeleteVpnGatewayOutput, SdkError<DeleteVpnGatewayError>>>;
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>>;
    fn deprovision_ipam_byoasn(&self, builder: DeprovisionIpamByoasnInputBuilder) -> impl Future<Output = Result<DeprovisionIpamByoasnOutput, SdkError<DeprovisionIpamByoasnError>>>;
    fn deprovision_ipam_pool_cidr(&self, builder: DeprovisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionIpamPoolCidrOutput, SdkError<DeprovisionIpamPoolCidrError>>>;
    fn deprovision_public_ipv4_pool_cidr(&self, builder: DeprovisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionPublicIpv4PoolCidrOutput, SdkError<DeprovisionPublicIpv4PoolCidrError>>>;
    fn deregister_image(&self, builder: DeregisterImageInputBuilder) -> impl Future<Output = Result<DeregisterImageOutput, SdkError<DeregisterImageError>>>;
    fn deregister_instance_event_notification_attributes(&self, builder: DeregisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DeregisterInstanceEventNotificationAttributesOutput, SdkError<DeregisterInstanceEventNotificationAttributesError>>>;
    fn deregister_transit_gateway_multicast_group_members(&self, builder: DeregisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupMembersOutput, SdkError<DeregisterTransitGatewayMulticastGroupMembersError>>>;
    fn deregister_transit_gateway_multicast_group_sources(&self, builder: DeregisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupSourcesOutput, SdkError<DeregisterTransitGatewayMulticastGroupSourcesError>>>;
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>>;
    fn describe_address_transfers(&self, builder: DescribeAddressTransfersInputBuilder) -> impl Future<Output = Result<DescribeAddressTransfersOutput, SdkError<DescribeAddressTransfersError>>>;
    fn describe_addresses(&self, builder: DescribeAddressesInputBuilder) -> impl Future<Output = Result<DescribeAddressesOutput, SdkError<DescribeAddressesError>>>;
    fn describe_addresses_attribute(&self, builder: DescribeAddressesAttributeInputBuilder) -> impl Future<Output = Result<DescribeAddressesAttributeOutput, SdkError<DescribeAddressesAttributeError>>>;
    fn describe_aggregate_id_format(&self, builder: DescribeAggregateIdFormatInputBuilder) -> impl Future<Output = Result<DescribeAggregateIdFormatOutput, SdkError<DescribeAggregateIdFormatError>>>;
    fn describe_availability_zones(&self, builder: DescribeAvailabilityZonesInputBuilder) -> impl Future<Output = Result<DescribeAvailabilityZonesOutput, SdkError<DescribeAvailabilityZonesError>>>;
    fn describe_aws_network_performance_metric_subscriptions(&self, builder: DescribeAwsNetworkPerformanceMetricSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeAwsNetworkPerformanceMetricSubscriptionsOutput, SdkError<DescribeAwsNetworkPerformanceMetricSubscriptionsError>>>;
    fn describe_bundle_tasks(&self, builder: DescribeBundleTasksInputBuilder) -> impl Future<Output = Result<DescribeBundleTasksOutput, SdkError<DescribeBundleTasksError>>>;
    fn describe_byoip_cidrs(&self, builder: DescribeByoipCidrsInputBuilder) -> impl Future<Output = Result<DescribeByoipCidrsOutput, SdkError<DescribeByoipCidrsError>>>;
    fn describe_capacity_block_offerings(&self, builder: DescribeCapacityBlockOfferingsInputBuilder) -> impl Future<Output = Result<DescribeCapacityBlockOfferingsOutput, SdkError<DescribeCapacityBlockOfferingsError>>>;
    fn describe_capacity_reservation_fleets(&self, builder: DescribeCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationFleetsOutput, SdkError<DescribeCapacityReservationFleetsError>>>;
    fn describe_capacity_reservations(&self, builder: DescribeCapacityReservationsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationsOutput, SdkError<DescribeCapacityReservationsError>>>;
    fn describe_carrier_gateways(&self, builder: DescribeCarrierGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCarrierGatewaysOutput, SdkError<DescribeCarrierGatewaysError>>>;
    fn describe_classic_link_instances(&self, builder: DescribeClassicLinkInstancesInputBuilder) -> impl Future<Output = Result<DescribeClassicLinkInstancesOutput, SdkError<DescribeClassicLinkInstancesError>>>;
    fn describe_client_vpn_authorization_rules(&self, builder: DescribeClientVpnAuthorizationRulesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnAuthorizationRulesOutput, SdkError<DescribeClientVpnAuthorizationRulesError>>>;
    fn describe_client_vpn_connections(&self, builder: DescribeClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnConnectionsOutput, SdkError<DescribeClientVpnConnectionsError>>>;
    fn describe_client_vpn_endpoints(&self, builder: DescribeClientVpnEndpointsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnEndpointsOutput, SdkError<DescribeClientVpnEndpointsError>>>;
    fn describe_client_vpn_routes(&self, builder: DescribeClientVpnRoutesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnRoutesOutput, SdkError<DescribeClientVpnRoutesError>>>;
    fn describe_client_vpn_target_networks(&self, builder: DescribeClientVpnTargetNetworksInputBuilder) -> impl Future<Output = Result<DescribeClientVpnTargetNetworksOutput, SdkError<DescribeClientVpnTargetNetworksError>>>;
    fn describe_coip_pools(&self, builder: DescribeCoipPoolsInputBuilder) -> impl Future<Output = Result<DescribeCoipPoolsOutput, SdkError<DescribeCoipPoolsError>>>;
    fn describe_conversion_tasks(&self, builder: DescribeConversionTasksInputBuilder) -> impl Future<Output = Result<DescribeConversionTasksOutput, SdkError<DescribeConversionTasksError>>>;
    fn describe_customer_gateways(&self, builder: DescribeCustomerGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCustomerGatewaysOutput, SdkError<DescribeCustomerGatewaysError>>>;
    fn describe_dhcp_options(&self, builder: DescribeDhcpOptionsInputBuilder) -> impl Future<Output = Result<DescribeDhcpOptionsOutput, SdkError<DescribeDhcpOptionsError>>>;
    fn describe_egress_only_internet_gateways(&self, builder: DescribeEgressOnlyInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeEgressOnlyInternetGatewaysOutput, SdkError<DescribeEgressOnlyInternetGatewaysError>>>;
    fn describe_elastic_gpus(&self, builder: DescribeElasticGpusInputBuilder) -> impl Future<Output = Result<DescribeElasticGpusOutput, SdkError<DescribeElasticGpusError>>>;
    fn describe_export_image_tasks(&self, builder: DescribeExportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeExportImageTasksOutput, SdkError<DescribeExportImageTasksError>>>;
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>>;
    fn describe_fast_launch_images(&self, builder: DescribeFastLaunchImagesInputBuilder) -> impl Future<Output = Result<DescribeFastLaunchImagesOutput, SdkError<DescribeFastLaunchImagesError>>>;
    fn describe_fast_snapshot_restores(&self, builder: DescribeFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DescribeFastSnapshotRestoresOutput, SdkError<DescribeFastSnapshotRestoresError>>>;
    fn describe_fleet_history(&self, builder: DescribeFleetHistoryInputBuilder) -> impl Future<Output = Result<DescribeFleetHistoryOutput, SdkError<DescribeFleetHistoryError>>>;
    fn describe_fleet_instances(&self, builder: DescribeFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeFleetInstancesOutput, SdkError<DescribeFleetInstancesError>>>;
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>>;
    fn describe_flow_logs(&self, builder: DescribeFlowLogsInputBuilder) -> impl Future<Output = Result<DescribeFlowLogsOutput, SdkError<DescribeFlowLogsError>>>;
    fn describe_fpga_image_attribute(&self, builder: DescribeFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeFpgaImageAttributeOutput, SdkError<DescribeFpgaImageAttributeError>>>;
    fn describe_fpga_images(&self, builder: DescribeFpgaImagesInputBuilder) -> impl Future<Output = Result<DescribeFpgaImagesOutput, SdkError<DescribeFpgaImagesError>>>;
    fn describe_host_reservation_offerings(&self, builder: DescribeHostReservationOfferingsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationOfferingsOutput, SdkError<DescribeHostReservationOfferingsError>>>;
    fn describe_host_reservations(&self, builder: DescribeHostReservationsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationsOutput, SdkError<DescribeHostReservationsError>>>;
    fn describe_hosts(&self, builder: DescribeHostsInputBuilder) -> impl Future<Output = Result<DescribeHostsOutput, SdkError<DescribeHostsError>>>;
    fn describe_iam_instance_profile_associations(&self, builder: DescribeIamInstanceProfileAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIamInstanceProfileAssociationsOutput, SdkError<DescribeIamInstanceProfileAssociationsError>>>;
    fn describe_id_format(&self, builder: DescribeIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdFormatOutput, SdkError<DescribeIdFormatError>>>;
    fn describe_identity_id_format(&self, builder: DescribeIdentityIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdentityIdFormatOutput, SdkError<DescribeIdentityIdFormatError>>>;
    fn describe_image_attribute(&self, builder: DescribeImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeImageAttributeOutput, SdkError<DescribeImageAttributeError>>>;
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>>;
    fn describe_import_image_tasks(&self, builder: DescribeImportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeImportImageTasksOutput, SdkError<DescribeImportImageTasksError>>>;
    fn describe_import_snapshot_tasks(&self, builder: DescribeImportSnapshotTasksInputBuilder) -> impl Future<Output = Result<DescribeImportSnapshotTasksOutput, SdkError<DescribeImportSnapshotTasksError>>>;
    fn describe_instance_attribute(&self, builder: DescribeInstanceAttributeInputBuilder) -> impl Future<Output = Result<DescribeInstanceAttributeOutput, SdkError<DescribeInstanceAttributeError>>>;
    fn describe_instance_connect_endpoints(&self, builder: DescribeInstanceConnectEndpointsInputBuilder) -> impl Future<Output = Result<DescribeInstanceConnectEndpointsOutput, SdkError<DescribeInstanceConnectEndpointsError>>>;
    fn describe_instance_credit_specifications(&self, builder: DescribeInstanceCreditSpecificationsInputBuilder) -> impl Future<Output = Result<DescribeInstanceCreditSpecificationsOutput, SdkError<DescribeInstanceCreditSpecificationsError>>>;
    fn describe_instance_event_notification_attributes(&self, builder: DescribeInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventNotificationAttributesOutput, SdkError<DescribeInstanceEventNotificationAttributesError>>>;
    fn describe_instance_event_windows(&self, builder: DescribeInstanceEventWindowsInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventWindowsOutput, SdkError<DescribeInstanceEventWindowsError>>>;
    fn describe_instance_status(&self, builder: DescribeInstanceStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceStatusOutput, SdkError<DescribeInstanceStatusError>>>;
    fn describe_instance_topology(&self, builder: DescribeInstanceTopologyInputBuilder) -> impl Future<Output = Result<DescribeInstanceTopologyOutput, SdkError<DescribeInstanceTopologyError>>>;
    fn describe_instance_type_offerings(&self, builder: DescribeInstanceTypeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeOfferingsOutput, SdkError<DescribeInstanceTypeOfferingsError>>>;
    fn describe_instance_types(&self, builder: DescribeInstanceTypesInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypesOutput, SdkError<DescribeInstanceTypesError>>>;
    fn describe_instances(&self, builder: DescribeInstancesInputBuilder) -> impl Future<Output = Result<DescribeInstancesOutput, SdkError<DescribeInstancesError>>>;
    fn describe_internet_gateways(&self, builder: DescribeInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeInternetGatewaysOutput, SdkError<DescribeInternetGatewaysError>>>;
    fn describe_ipam_byoasn(&self, builder: DescribeIpamByoasnInputBuilder) -> impl Future<Output = Result<DescribeIpamByoasnOutput, SdkError<DescribeIpamByoasnError>>>;
    fn describe_ipam_external_resource_verification_tokens(&self, builder: DescribeIpamExternalResourceVerificationTokensInputBuilder) -> impl Future<Output = Result<DescribeIpamExternalResourceVerificationTokensOutput, SdkError<DescribeIpamExternalResourceVerificationTokensError>>>;
    fn describe_ipam_pools(&self, builder: DescribeIpamPoolsInputBuilder) -> impl Future<Output = Result<DescribeIpamPoolsOutput, SdkError<DescribeIpamPoolsError>>>;
    fn describe_ipam_resource_discoveries(&self, builder: DescribeIpamResourceDiscoveriesInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveriesOutput, SdkError<DescribeIpamResourceDiscoveriesError>>>;
    fn describe_ipam_resource_discovery_associations(&self, builder: DescribeIpamResourceDiscoveryAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveryAssociationsOutput, SdkError<DescribeIpamResourceDiscoveryAssociationsError>>>;
    fn describe_ipam_scopes(&self, builder: DescribeIpamScopesInputBuilder) -> impl Future<Output = Result<DescribeIpamScopesOutput, SdkError<DescribeIpamScopesError>>>;
    fn describe_ipams(&self, builder: DescribeIpamsInputBuilder) -> impl Future<Output = Result<DescribeIpamsOutput, SdkError<DescribeIpamsError>>>;
    fn describe_ipv6_pools(&self, builder: DescribeIpv6PoolsInputBuilder) -> impl Future<Output = Result<DescribeIpv6PoolsOutput, SdkError<DescribeIpv6PoolsError>>>;
    fn describe_key_pairs(&self, builder: DescribeKeyPairsInputBuilder) -> impl Future<Output = Result<DescribeKeyPairsOutput, SdkError<DescribeKeyPairsError>>>;
    fn describe_launch_template_versions(&self, builder: DescribeLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplateVersionsOutput, SdkError<DescribeLaunchTemplateVersionsError>>>;
    fn describe_launch_templates(&self, builder: DescribeLaunchTemplatesInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplatesOutput, SdkError<DescribeLaunchTemplatesError>>>;
    fn describe_local_gateway_route_table_virtual_interface_group_associations(&self, builder: DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsError>>>;
    fn describe_local_gateway_route_table_vpc_associations(&self, builder: DescribeLocalGatewayRouteTableVpcAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVpcAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVpcAssociationsError>>>;
    fn describe_local_gateway_route_tables(&self, builder: DescribeLocalGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTablesOutput, SdkError<DescribeLocalGatewayRouteTablesError>>>;
    fn describe_local_gateway_virtual_interface_groups(&self, builder: DescribeLocalGatewayVirtualInterfaceGroupsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfaceGroupsOutput, SdkError<DescribeLocalGatewayVirtualInterfaceGroupsError>>>;
    fn describe_local_gateway_virtual_interfaces(&self, builder: DescribeLocalGatewayVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfacesOutput, SdkError<DescribeLocalGatewayVirtualInterfacesError>>>;
    fn describe_local_gateways(&self, builder: DescribeLocalGatewaysInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewaysOutput, SdkError<DescribeLocalGatewaysError>>>;
    fn describe_locked_snapshots(&self, builder: DescribeLockedSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeLockedSnapshotsOutput, SdkError<DescribeLockedSnapshotsError>>>;
    fn describe_mac_hosts(&self, builder: DescribeMacHostsInputBuilder) -> impl Future<Output = Result<DescribeMacHostsOutput, SdkError<DescribeMacHostsError>>>;
    fn describe_managed_prefix_lists(&self, builder: DescribeManagedPrefixListsInputBuilder) -> impl Future<Output = Result<DescribeManagedPrefixListsOutput, SdkError<DescribeManagedPrefixListsError>>>;
    fn describe_moving_addresses(&self, builder: DescribeMovingAddressesInputBuilder) -> impl Future<Output = Result<DescribeMovingAddressesOutput, SdkError<DescribeMovingAddressesError>>>;
    fn describe_nat_gateways(&self, builder: DescribeNatGatewaysInputBuilder) -> impl Future<Output = Result<DescribeNatGatewaysOutput, SdkError<DescribeNatGatewaysError>>>;
    fn describe_network_acls(&self, builder: DescribeNetworkAclsInputBuilder) -> impl Future<Output = Result<DescribeNetworkAclsOutput, SdkError<DescribeNetworkAclsError>>>;
    fn describe_network_insights_access_scope_analyses(&self, builder: DescribeNetworkInsightsAccessScopeAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopeAnalysesOutput, SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>>>;
    fn describe_network_insights_access_scopes(&self, builder: DescribeNetworkInsightsAccessScopesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopesOutput, SdkError<DescribeNetworkInsightsAccessScopesError>>>;
    fn describe_network_insights_analyses(&self, builder: DescribeNetworkInsightsAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAnalysesOutput, SdkError<DescribeNetworkInsightsAnalysesError>>>;
    fn describe_network_insights_paths(&self, builder: DescribeNetworkInsightsPathsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsPathsOutput, SdkError<DescribeNetworkInsightsPathsError>>>;
    fn describe_network_interface_attribute(&self, builder: DescribeNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfaceAttributeOutput, SdkError<DescribeNetworkInterfaceAttributeError>>>;
    fn describe_network_interface_permissions(&self, builder: DescribeNetworkInterfacePermissionsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacePermissionsOutput, SdkError<DescribeNetworkInterfacePermissionsError>>>;
    fn describe_network_interfaces(&self, builder: DescribeNetworkInterfacesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacesOutput, SdkError<DescribeNetworkInterfacesError>>>;
    fn describe_placement_groups(&self, builder: DescribePlacementGroupsInputBuilder) -> impl Future<Output = Result<DescribePlacementGroupsOutput, SdkError<DescribePlacementGroupsError>>>;
    fn describe_prefix_lists(&self, builder: DescribePrefixListsInputBuilder) -> impl Future<Output = Result<DescribePrefixListsOutput, SdkError<DescribePrefixListsError>>>;
    fn describe_principal_id_format(&self, builder: DescribePrincipalIdFormatInputBuilder) -> impl Future<Output = Result<DescribePrincipalIdFormatOutput, SdkError<DescribePrincipalIdFormatError>>>;
    fn describe_public_ipv4_pools(&self, builder: DescribePublicIpv4PoolsInputBuilder) -> impl Future<Output = Result<DescribePublicIpv4PoolsOutput, SdkError<DescribePublicIpv4PoolsError>>>;
    fn describe_regions(&self, builder: DescribeRegionsInputBuilder) -> impl Future<Output = Result<DescribeRegionsOutput, SdkError<DescribeRegionsError>>>;
    fn describe_replace_root_volume_tasks(&self, builder: DescribeReplaceRootVolumeTasksInputBuilder) -> impl Future<Output = Result<DescribeReplaceRootVolumeTasksOutput, SdkError<DescribeReplaceRootVolumeTasksError>>>;
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>>;
    fn describe_reserved_instances_listings(&self, builder: DescribeReservedInstancesListingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesListingsOutput, SdkError<DescribeReservedInstancesListingsError>>>;
    fn describe_reserved_instances_modifications(&self, builder: DescribeReservedInstancesModificationsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesModificationsOutput, SdkError<DescribeReservedInstancesModificationsError>>>;
    fn describe_reserved_instances_offerings(&self, builder: DescribeReservedInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOfferingsOutput, SdkError<DescribeReservedInstancesOfferingsError>>>;
    fn describe_route_tables(&self, builder: DescribeRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeRouteTablesOutput, SdkError<DescribeRouteTablesError>>>;
    fn describe_scheduled_instance_availability(&self, builder: DescribeScheduledInstanceAvailabilityInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstanceAvailabilityOutput, SdkError<DescribeScheduledInstanceAvailabilityError>>>;
    fn describe_scheduled_instances(&self, builder: DescribeScheduledInstancesInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstancesOutput, SdkError<DescribeScheduledInstancesError>>>;
    fn describe_security_group_references(&self, builder: DescribeSecurityGroupReferencesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupReferencesOutput, SdkError<DescribeSecurityGroupReferencesError>>>;
    fn describe_security_group_rules(&self, builder: DescribeSecurityGroupRulesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupRulesOutput, SdkError<DescribeSecurityGroupRulesError>>>;
    fn describe_security_groups(&self, builder: DescribeSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupsOutput, SdkError<DescribeSecurityGroupsError>>>;
    fn describe_snapshot_attribute(&self, builder: DescribeSnapshotAttributeInputBuilder) -> impl Future<Output = Result<DescribeSnapshotAttributeOutput, SdkError<DescribeSnapshotAttributeError>>>;
    fn describe_snapshot_tier_status(&self, builder: DescribeSnapshotTierStatusInputBuilder) -> impl Future<Output = Result<DescribeSnapshotTierStatusOutput, SdkError<DescribeSnapshotTierStatusError>>>;
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>>;
    fn describe_spot_datafeed_subscription(&self, builder: DescribeSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeSpotDatafeedSubscriptionOutput, SdkError<DescribeSpotDatafeedSubscriptionError>>>;
    fn describe_spot_fleet_instances(&self, builder: DescribeSpotFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetInstancesOutput, SdkError<DescribeSpotFleetInstancesError>>>;
    fn describe_spot_fleet_request_history(&self, builder: DescribeSpotFleetRequestHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestHistoryOutput, SdkError<DescribeSpotFleetRequestHistoryError>>>;
    fn describe_spot_fleet_requests(&self, builder: DescribeSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestsOutput, SdkError<DescribeSpotFleetRequestsError>>>;
    fn describe_spot_instance_requests(&self, builder: DescribeSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotInstanceRequestsOutput, SdkError<DescribeSpotInstanceRequestsError>>>;
    fn describe_spot_price_history(&self, builder: DescribeSpotPriceHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotPriceHistoryOutput, SdkError<DescribeSpotPriceHistoryError>>>;
    fn describe_stale_security_groups(&self, builder: DescribeStaleSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeStaleSecurityGroupsOutput, SdkError<DescribeStaleSecurityGroupsError>>>;
    fn describe_store_image_tasks(&self, builder: DescribeStoreImageTasksInputBuilder) -> impl Future<Output = Result<DescribeStoreImageTasksOutput, SdkError<DescribeStoreImageTasksError>>>;
    fn describe_subnets(&self, builder: DescribeSubnetsInputBuilder) -> impl Future<Output = Result<DescribeSubnetsOutput, SdkError<DescribeSubnetsError>>>;
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>>;
    fn describe_traffic_mirror_filter_rules(&self, builder: DescribeTrafficMirrorFilterRulesInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFilterRulesOutput, SdkError<DescribeTrafficMirrorFilterRulesError>>>;
    fn describe_traffic_mirror_filters(&self, builder: DescribeTrafficMirrorFiltersInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFiltersOutput, SdkError<DescribeTrafficMirrorFiltersError>>>;
    fn describe_traffic_mirror_sessions(&self, builder: DescribeTrafficMirrorSessionsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorSessionsOutput, SdkError<DescribeTrafficMirrorSessionsError>>>;
    fn describe_traffic_mirror_targets(&self, builder: DescribeTrafficMirrorTargetsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorTargetsOutput, SdkError<DescribeTrafficMirrorTargetsError>>>;
    fn describe_transit_gateway_attachments(&self, builder: DescribeTransitGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayAttachmentsOutput, SdkError<DescribeTransitGatewayAttachmentsError>>>;
    fn describe_transit_gateway_connect_peers(&self, builder: DescribeTransitGatewayConnectPeersInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectPeersOutput, SdkError<DescribeTransitGatewayConnectPeersError>>>;
    fn describe_transit_gateway_connects(&self, builder: DescribeTransitGatewayConnectsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectsOutput, SdkError<DescribeTransitGatewayConnectsError>>>;
    fn describe_transit_gateway_multicast_domains(&self, builder: DescribeTransitGatewayMulticastDomainsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayMulticastDomainsOutput, SdkError<DescribeTransitGatewayMulticastDomainsError>>>;
    fn describe_transit_gateway_peering_attachments(&self, builder: DescribeTransitGatewayPeeringAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPeeringAttachmentsOutput, SdkError<DescribeTransitGatewayPeeringAttachmentsError>>>;
    fn describe_transit_gateway_policy_tables(&self, builder: DescribeTransitGatewayPolicyTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPolicyTablesOutput, SdkError<DescribeTransitGatewayPolicyTablesError>>>;
    fn describe_transit_gateway_route_table_announcements(&self, builder: DescribeTransitGatewayRouteTableAnnouncementsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTableAnnouncementsOutput, SdkError<DescribeTransitGatewayRouteTableAnnouncementsError>>>;
    fn describe_transit_gateway_route_tables(&self, builder: DescribeTransitGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTablesOutput, SdkError<DescribeTransitGatewayRouteTablesError>>>;
    fn describe_transit_gateway_vpc_attachments(&self, builder: DescribeTransitGatewayVpcAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayVpcAttachmentsOutput, SdkError<DescribeTransitGatewayVpcAttachmentsError>>>;
    fn describe_transit_gateways(&self, builder: DescribeTransitGatewaysInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewaysOutput, SdkError<DescribeTransitGatewaysError>>>;
    fn describe_trunk_interface_associations(&self, builder: DescribeTrunkInterfaceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeTrunkInterfaceAssociationsOutput, SdkError<DescribeTrunkInterfaceAssociationsError>>>;
    fn describe_verified_access_endpoints(&self, builder: DescribeVerifiedAccessEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessEndpointsOutput, SdkError<DescribeVerifiedAccessEndpointsError>>>;
    fn describe_verified_access_groups(&self, builder: DescribeVerifiedAccessGroupsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessGroupsOutput, SdkError<DescribeVerifiedAccessGroupsError>>>;
    fn describe_verified_access_instance_logging_configurations(&self, builder: DescribeVerifiedAccessInstanceLoggingConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstanceLoggingConfigurationsOutput, SdkError<DescribeVerifiedAccessInstanceLoggingConfigurationsError>>>;
    fn describe_verified_access_instances(&self, builder: DescribeVerifiedAccessInstancesInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstancesOutput, SdkError<DescribeVerifiedAccessInstancesError>>>;
    fn describe_verified_access_trust_providers(&self, builder: DescribeVerifiedAccessTrustProvidersInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessTrustProvidersOutput, SdkError<DescribeVerifiedAccessTrustProvidersError>>>;
    fn describe_volume_attribute(&self, builder: DescribeVolumeAttributeInputBuilder) -> impl Future<Output = Result<DescribeVolumeAttributeOutput, SdkError<DescribeVolumeAttributeError>>>;
    fn describe_volume_status(&self, builder: DescribeVolumeStatusInputBuilder) -> impl Future<Output = Result<DescribeVolumeStatusOutput, SdkError<DescribeVolumeStatusError>>>;
    fn describe_volumes(&self, builder: DescribeVolumesInputBuilder) -> impl Future<Output = Result<DescribeVolumesOutput, SdkError<DescribeVolumesError>>>;
    fn describe_volumes_modifications(&self, builder: DescribeVolumesModificationsInputBuilder) -> impl Future<Output = Result<DescribeVolumesModificationsOutput, SdkError<DescribeVolumesModificationsError>>>;
    fn describe_vpc_attribute(&self, builder: DescribeVpcAttributeInputBuilder) -> impl Future<Output = Result<DescribeVpcAttributeOutput, SdkError<DescribeVpcAttributeError>>>;
    fn describe_vpc_classic_link(&self, builder: DescribeVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkOutput, SdkError<DescribeVpcClassicLinkError>>>;
    fn describe_vpc_classic_link_dns_support(&self, builder: DescribeVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkDnsSupportOutput, SdkError<DescribeVpcClassicLinkDnsSupportError>>>;
    fn describe_vpc_endpoint_connection_notifications(&self, builder: DescribeVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionNotificationsOutput, SdkError<DescribeVpcEndpointConnectionNotificationsError>>>;
    fn describe_vpc_endpoint_connections(&self, builder: DescribeVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionsOutput, SdkError<DescribeVpcEndpointConnectionsError>>>;
    fn describe_vpc_endpoint_service_configurations(&self, builder: DescribeVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServiceConfigurationsOutput, SdkError<DescribeVpcEndpointServiceConfigurationsError>>>;
    fn describe_vpc_endpoint_service_permissions(&self, builder: DescribeVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicePermissionsOutput, SdkError<DescribeVpcEndpointServicePermissionsError>>>;
    fn describe_vpc_endpoint_services(&self, builder: DescribeVpcEndpointServicesInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicesOutput, SdkError<DescribeVpcEndpointServicesError>>>;
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>>;
    fn describe_vpc_peering_connections(&self, builder: DescribeVpcPeeringConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcPeeringConnectionsOutput, SdkError<DescribeVpcPeeringConnectionsError>>>;
    fn describe_vpcs(&self, builder: DescribeVpcsInputBuilder) -> impl Future<Output = Result<DescribeVpcsOutput, SdkError<DescribeVpcsError>>>;
    fn describe_vpn_connections(&self, builder: DescribeVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpnConnectionsOutput, SdkError<DescribeVpnConnectionsError>>>;
    fn describe_vpn_gateways(&self, builder: DescribeVpnGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVpnGatewaysOutput, SdkError<DescribeVpnGatewaysError>>>;
    fn detach_classic_link_vpc(&self, builder: DetachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<DetachClassicLinkVpcOutput, SdkError<DetachClassicLinkVpcError>>>;
    fn detach_internet_gateway(&self, builder: DetachInternetGatewayInputBuilder) -> impl Future<Output = Result<DetachInternetGatewayOutput, SdkError<DetachInternetGatewayError>>>;
    fn detach_network_interface(&self, builder: DetachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DetachNetworkInterfaceOutput, SdkError<DetachNetworkInterfaceError>>>;
    fn detach_verified_access_trust_provider(&self, builder: DetachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DetachVerifiedAccessTrustProviderOutput, SdkError<DetachVerifiedAccessTrustProviderError>>>;
    fn detach_volume(&self, builder: DetachVolumeInputBuilder) -> impl Future<Output = Result<DetachVolumeOutput, SdkError<DetachVolumeError>>>;
    fn detach_vpn_gateway(&self, builder: DetachVpnGatewayInputBuilder) -> impl Future<Output = Result<DetachVpnGatewayOutput, SdkError<DetachVpnGatewayError>>>;
    fn disable_address_transfer(&self, builder: DisableAddressTransferInputBuilder) -> impl Future<Output = Result<DisableAddressTransferOutput, SdkError<DisableAddressTransferError>>>;
    fn disable_aws_network_performance_metric_subscription(&self, builder: DisableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<DisableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<DisableAwsNetworkPerformanceMetricSubscriptionError>>>;
    fn disable_ebs_encryption_by_default(&self, builder: DisableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<DisableEbsEncryptionByDefaultOutput, SdkError<DisableEbsEncryptionByDefaultError>>>;
    fn disable_fast_launch(&self, builder: DisableFastLaunchInputBuilder) -> impl Future<Output = Result<DisableFastLaunchOutput, SdkError<DisableFastLaunchError>>>;
    fn disable_fast_snapshot_restores(&self, builder: DisableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DisableFastSnapshotRestoresOutput, SdkError<DisableFastSnapshotRestoresError>>>;
    fn disable_image(&self, builder: DisableImageInputBuilder) -> impl Future<Output = Result<DisableImageOutput, SdkError<DisableImageError>>>;
    fn disable_image_block_public_access(&self, builder: DisableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableImageBlockPublicAccessOutput, SdkError<DisableImageBlockPublicAccessError>>>;
    fn disable_image_deprecation(&self, builder: DisableImageDeprecationInputBuilder) -> impl Future<Output = Result<DisableImageDeprecationOutput, SdkError<DisableImageDeprecationError>>>;
    fn disable_image_deregistration_protection(&self, builder: DisableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<DisableImageDeregistrationProtectionOutput, SdkError<DisableImageDeregistrationProtectionError>>>;
    fn disable_ipam_organization_admin_account(&self, builder: DisableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableIpamOrganizationAdminAccountOutput, SdkError<DisableIpamOrganizationAdminAccountError>>>;
    fn disable_serial_console_access(&self, builder: DisableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<DisableSerialConsoleAccessOutput, SdkError<DisableSerialConsoleAccessError>>>;
    fn disable_snapshot_block_public_access(&self, builder: DisableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableSnapshotBlockPublicAccessOutput, SdkError<DisableSnapshotBlockPublicAccessError>>>;
    fn disable_transit_gateway_route_table_propagation(&self, builder: DisableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<DisableTransitGatewayRouteTablePropagationOutput, SdkError<DisableTransitGatewayRouteTablePropagationError>>>;
    fn disable_vgw_route_propagation(&self, builder: DisableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<DisableVgwRoutePropagationOutput, SdkError<DisableVgwRoutePropagationError>>>;
    fn disable_vpc_classic_link(&self, builder: DisableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkOutput, SdkError<DisableVpcClassicLinkError>>>;
    fn disable_vpc_classic_link_dns_support(&self, builder: DisableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkDnsSupportOutput, SdkError<DisableVpcClassicLinkDnsSupportError>>>;
    fn disassociate_address(&self, builder: DisassociateAddressInputBuilder) -> impl Future<Output = Result<DisassociateAddressOutput, SdkError<DisassociateAddressError>>>;
    fn disassociate_client_vpn_target_network(&self, builder: DisassociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<DisassociateClientVpnTargetNetworkOutput, SdkError<DisassociateClientVpnTargetNetworkError>>>;
    fn disassociate_enclave_certificate_iam_role(&self, builder: DisassociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnclaveCertificateIamRoleOutput, SdkError<DisassociateEnclaveCertificateIamRoleError>>>;
    fn disassociate_iam_instance_profile(&self, builder: DisassociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<DisassociateIamInstanceProfileOutput, SdkError<DisassociateIamInstanceProfileError>>>;
    fn disassociate_instance_event_window(&self, builder: DisassociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DisassociateInstanceEventWindowOutput, SdkError<DisassociateInstanceEventWindowError>>>;
    fn disassociate_ipam_byoasn(&self, builder: DisassociateIpamByoasnInputBuilder) -> impl Future<Output = Result<DisassociateIpamByoasnOutput, SdkError<DisassociateIpamByoasnError>>>;
    fn disassociate_ipam_resource_discovery(&self, builder: DisassociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DisassociateIpamResourceDiscoveryOutput, SdkError<DisassociateIpamResourceDiscoveryError>>>;
    fn disassociate_nat_gateway_address(&self, builder: DisassociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<DisassociateNatGatewayAddressOutput, SdkError<DisassociateNatGatewayAddressError>>>;
    fn disassociate_route_table(&self, builder: DisassociateRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateRouteTableOutput, SdkError<DisassociateRouteTableError>>>;
    fn disassociate_subnet_cidr_block(&self, builder: DisassociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateSubnetCidrBlockOutput, SdkError<DisassociateSubnetCidrBlockError>>>;
    fn disassociate_transit_gateway_multicast_domain(&self, builder: DisassociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayMulticastDomainOutput, SdkError<DisassociateTransitGatewayMulticastDomainError>>>;
    fn disassociate_transit_gateway_policy_table(&self, builder: DisassociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayPolicyTableOutput, SdkError<DisassociateTransitGatewayPolicyTableError>>>;
    fn disassociate_transit_gateway_route_table(&self, builder: DisassociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayRouteTableOutput, SdkError<DisassociateTransitGatewayRouteTableError>>>;
    fn disassociate_trunk_interface(&self, builder: DisassociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<DisassociateTrunkInterfaceOutput, SdkError<DisassociateTrunkInterfaceError>>>;
    fn disassociate_vpc_cidr_block(&self, builder: DisassociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateVpcCidrBlockOutput, SdkError<DisassociateVpcCidrBlockError>>>;
    fn enable_address_transfer(&self, builder: EnableAddressTransferInputBuilder) -> impl Future<Output = Result<EnableAddressTransferOutput, SdkError<EnableAddressTransferError>>>;
    fn enable_aws_network_performance_metric_subscription(&self, builder: EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<EnableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<EnableAwsNetworkPerformanceMetricSubscriptionError>>>;
    fn enable_ebs_encryption_by_default(&self, builder: EnableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<EnableEbsEncryptionByDefaultOutput, SdkError<EnableEbsEncryptionByDefaultError>>>;
    fn enable_fast_launch(&self, builder: EnableFastLaunchInputBuilder) -> impl Future<Output = Result<EnableFastLaunchOutput, SdkError<EnableFastLaunchError>>>;
    fn enable_fast_snapshot_restores(&self, builder: EnableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<EnableFastSnapshotRestoresOutput, SdkError<EnableFastSnapshotRestoresError>>>;
    fn enable_image(&self, builder: EnableImageInputBuilder) -> impl Future<Output = Result<EnableImageOutput, SdkError<EnableImageError>>>;
    fn enable_image_block_public_access(&self, builder: EnableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableImageBlockPublicAccessOutput, SdkError<EnableImageBlockPublicAccessError>>>;
    fn enable_image_deprecation(&self, builder: EnableImageDeprecationInputBuilder) -> impl Future<Output = Result<EnableImageDeprecationOutput, SdkError<EnableImageDeprecationError>>>;
    fn enable_image_deregistration_protection(&self, builder: EnableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<EnableImageDeregistrationProtectionOutput, SdkError<EnableImageDeregistrationProtectionError>>>;
    fn enable_ipam_organization_admin_account(&self, builder: EnableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableIpamOrganizationAdminAccountOutput, SdkError<EnableIpamOrganizationAdminAccountError>>>;
    fn enable_reachability_analyzer_organization_sharing(&self, builder: EnableReachabilityAnalyzerOrganizationSharingInputBuilder) -> impl Future<Output = Result<EnableReachabilityAnalyzerOrganizationSharingOutput, SdkError<EnableReachabilityAnalyzerOrganizationSharingError>>>;
    fn enable_serial_console_access(&self, builder: EnableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<EnableSerialConsoleAccessOutput, SdkError<EnableSerialConsoleAccessError>>>;
    fn enable_snapshot_block_public_access(&self, builder: EnableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableSnapshotBlockPublicAccessOutput, SdkError<EnableSnapshotBlockPublicAccessError>>>;
    fn enable_transit_gateway_route_table_propagation(&self, builder: EnableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<EnableTransitGatewayRouteTablePropagationOutput, SdkError<EnableTransitGatewayRouteTablePropagationError>>>;
    fn enable_vgw_route_propagation(&self, builder: EnableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<EnableVgwRoutePropagationOutput, SdkError<EnableVgwRoutePropagationError>>>;
    fn enable_volume_io(&self, builder: EnableVolumeIoInputBuilder) -> impl Future<Output = Result<EnableVolumeIoOutput, SdkError<EnableVolumeIOError>>>;
    fn enable_vpc_classic_link(&self, builder: EnableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkOutput, SdkError<EnableVpcClassicLinkError>>>;
    fn enable_vpc_classic_link_dns_support(&self, builder: EnableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkDnsSupportOutput, SdkError<EnableVpcClassicLinkDnsSupportError>>>;
    fn export_client_vpn_client_certificate_revocation_list(&self, builder: ExportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientCertificateRevocationListOutput, SdkError<ExportClientVpnClientCertificateRevocationListError>>>;
    fn export_client_vpn_client_configuration(&self, builder: ExportClientVpnClientConfigurationInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientConfigurationOutput, SdkError<ExportClientVpnClientConfigurationError>>>;
    fn export_image(&self, builder: ExportImageInputBuilder) -> impl Future<Output = Result<ExportImageOutput, SdkError<ExportImageError>>>;
    fn export_transit_gateway_routes(&self, builder: ExportTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<ExportTransitGatewayRoutesOutput, SdkError<ExportTransitGatewayRoutesError>>>;
    fn get_associated_enclave_certificate_iam_roles(&self, builder: GetAssociatedEnclaveCertificateIamRolesInputBuilder) -> impl Future<Output = Result<GetAssociatedEnclaveCertificateIamRolesOutput, SdkError<GetAssociatedEnclaveCertificateIamRolesError>>>;
    fn get_associated_ipv6_pool_cidrs(&self, builder: GetAssociatedIpv6PoolCidrsInputBuilder) -> impl Future<Output = Result<GetAssociatedIpv6PoolCidrsOutput, SdkError<GetAssociatedIpv6PoolCidrsError>>>;
    fn get_aws_network_performance_data(&self, builder: GetAwsNetworkPerformanceDataInputBuilder) -> impl Future<Output = Result<GetAwsNetworkPerformanceDataOutput, SdkError<GetAwsNetworkPerformanceDataError>>>;
    fn get_capacity_reservation_usage(&self, builder: GetCapacityReservationUsageInputBuilder) -> impl Future<Output = Result<GetCapacityReservationUsageOutput, SdkError<GetCapacityReservationUsageError>>>;
    fn get_coip_pool_usage(&self, builder: GetCoipPoolUsageInputBuilder) -> impl Future<Output = Result<GetCoipPoolUsageOutput, SdkError<GetCoipPoolUsageError>>>;
    fn get_console_output(&self, builder: GetConsoleOutputInputBuilder) -> impl Future<Output = Result<GetConsoleOutputOutput, SdkError<GetConsoleOutputError>>>;
    fn get_console_screenshot(&self, builder: GetConsoleScreenshotInputBuilder) -> impl Future<Output = Result<GetConsoleScreenshotOutput, SdkError<GetConsoleScreenshotError>>>;
    fn get_default_credit_specification(&self, builder: GetDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<GetDefaultCreditSpecificationOutput, SdkError<GetDefaultCreditSpecificationError>>>;
    fn get_ebs_default_kms_key_id(&self, builder: GetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<GetEbsDefaultKmsKeyIdOutput, SdkError<GetEbsDefaultKmsKeyIdError>>>;
    fn get_ebs_encryption_by_default(&self, builder: GetEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<GetEbsEncryptionByDefaultOutput, SdkError<GetEbsEncryptionByDefaultError>>>;
    fn get_flow_logs_integration_template(&self, builder: GetFlowLogsIntegrationTemplateInputBuilder) -> impl Future<Output = Result<GetFlowLogsIntegrationTemplateOutput, SdkError<GetFlowLogsIntegrationTemplateError>>>;
    fn get_groups_for_capacity_reservation(&self, builder: GetGroupsForCapacityReservationInputBuilder) -> impl Future<Output = Result<GetGroupsForCapacityReservationOutput, SdkError<GetGroupsForCapacityReservationError>>>;
    fn get_host_reservation_purchase_preview(&self, builder: GetHostReservationPurchasePreviewInputBuilder) -> impl Future<Output = Result<GetHostReservationPurchasePreviewOutput, SdkError<GetHostReservationPurchasePreviewError>>>;
    fn get_image_block_public_access_state(&self, builder: GetImageBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetImageBlockPublicAccessStateOutput, SdkError<GetImageBlockPublicAccessStateError>>>;
    fn get_instance_metadata_defaults(&self, builder: GetInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<GetInstanceMetadataDefaultsOutput, SdkError<GetInstanceMetadataDefaultsError>>>;
    fn get_instance_tpm_ek_pub(&self, builder: GetInstanceTpmEkPubInputBuilder) -> impl Future<Output = Result<GetInstanceTpmEkPubOutput, SdkError<GetInstanceTpmEkPubError>>>;
    fn get_instance_types_from_instance_requirements(&self, builder: GetInstanceTypesFromInstanceRequirementsInputBuilder) -> impl Future<Output = Result<GetInstanceTypesFromInstanceRequirementsOutput, SdkError<GetInstanceTypesFromInstanceRequirementsError>>>;
    fn get_instance_uefi_data(&self, builder: GetInstanceUefiDataInputBuilder) -> impl Future<Output = Result<GetInstanceUefiDataOutput, SdkError<GetInstanceUefiDataError>>>;
    fn get_ipam_address_history(&self, builder: GetIpamAddressHistoryInputBuilder) -> impl Future<Output = Result<GetIpamAddressHistoryOutput, SdkError<GetIpamAddressHistoryError>>>;
    fn get_ipam_discovered_accounts(&self, builder: GetIpamDiscoveredAccountsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredAccountsOutput, SdkError<GetIpamDiscoveredAccountsError>>>;
    fn get_ipam_discovered_public_addresses(&self, builder: GetIpamDiscoveredPublicAddressesInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredPublicAddressesOutput, SdkError<GetIpamDiscoveredPublicAddressesError>>>;
    fn get_ipam_discovered_resource_cidrs(&self, builder: GetIpamDiscoveredResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredResourceCidrsOutput, SdkError<GetIpamDiscoveredResourceCidrsError>>>;
    fn get_ipam_pool_allocations(&self, builder: GetIpamPoolAllocationsInputBuilder) -> impl Future<Output = Result<GetIpamPoolAllocationsOutput, SdkError<GetIpamPoolAllocationsError>>>;
    fn get_ipam_pool_cidrs(&self, builder: GetIpamPoolCidrsInputBuilder) -> impl Future<Output = Result<GetIpamPoolCidrsOutput, SdkError<GetIpamPoolCidrsError>>>;
    fn get_ipam_resource_cidrs(&self, builder: GetIpamResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamResourceCidrsOutput, SdkError<GetIpamResourceCidrsError>>>;
    fn get_launch_template_data(&self, builder: GetLaunchTemplateDataInputBuilder) -> impl Future<Output = Result<GetLaunchTemplateDataOutput, SdkError<GetLaunchTemplateDataError>>>;
    fn get_managed_prefix_list_associations(&self, builder: GetManagedPrefixListAssociationsInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListAssociationsOutput, SdkError<GetManagedPrefixListAssociationsError>>>;
    fn get_managed_prefix_list_entries(&self, builder: GetManagedPrefixListEntriesInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListEntriesOutput, SdkError<GetManagedPrefixListEntriesError>>>;
    fn get_network_insights_access_scope_analysis_findings(&self, builder: GetNetworkInsightsAccessScopeAnalysisFindingsInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeAnalysisFindingsOutput, SdkError<GetNetworkInsightsAccessScopeAnalysisFindingsError>>>;
    fn get_network_insights_access_scope_content(&self, builder: GetNetworkInsightsAccessScopeContentInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeContentOutput, SdkError<GetNetworkInsightsAccessScopeContentError>>>;
    fn get_password_data(&self, builder: GetPasswordDataInputBuilder) -> impl Future<Output = Result<GetPasswordDataOutput, SdkError<GetPasswordDataError>>>;
    fn get_reserved_instances_exchange_quote(&self, builder: GetReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<GetReservedInstancesExchangeQuoteOutput, SdkError<GetReservedInstancesExchangeQuoteError>>>;
    fn get_security_groups_for_vpc(&self, builder: GetSecurityGroupsForVpcInputBuilder) -> impl Future<Output = Result<GetSecurityGroupsForVpcOutput, SdkError<GetSecurityGroupsForVpcError>>>;
    fn get_serial_console_access_status(&self, builder: GetSerialConsoleAccessStatusInputBuilder) -> impl Future<Output = Result<GetSerialConsoleAccessStatusOutput, SdkError<GetSerialConsoleAccessStatusError>>>;
    fn get_snapshot_block_public_access_state(&self, builder: GetSnapshotBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetSnapshotBlockPublicAccessStateOutput, SdkError<GetSnapshotBlockPublicAccessStateError>>>;
    fn get_spot_placement_scores(&self, builder: GetSpotPlacementScoresInputBuilder) -> impl Future<Output = Result<GetSpotPlacementScoresOutput, SdkError<GetSpotPlacementScoresError>>>;
    fn get_subnet_cidr_reservations(&self, builder: GetSubnetCidrReservationsInputBuilder) -> impl Future<Output = Result<GetSubnetCidrReservationsOutput, SdkError<GetSubnetCidrReservationsError>>>;
    fn get_transit_gateway_attachment_propagations(&self, builder: GetTransitGatewayAttachmentPropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayAttachmentPropagationsOutput, SdkError<GetTransitGatewayAttachmentPropagationsError>>>;
    fn get_transit_gateway_multicast_domain_associations(&self, builder: GetTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayMulticastDomainAssociationsOutput, SdkError<GetTransitGatewayMulticastDomainAssociationsError>>>;
    fn get_transit_gateway_policy_table_associations(&self, builder: GetTransitGatewayPolicyTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableAssociationsOutput, SdkError<GetTransitGatewayPolicyTableAssociationsError>>>;
    fn get_transit_gateway_policy_table_entries(&self, builder: GetTransitGatewayPolicyTableEntriesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableEntriesOutput, SdkError<GetTransitGatewayPolicyTableEntriesError>>>;
    fn get_transit_gateway_prefix_list_references(&self, builder: GetTransitGatewayPrefixListReferencesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPrefixListReferencesOutput, SdkError<GetTransitGatewayPrefixListReferencesError>>>;
    fn get_transit_gateway_route_table_associations(&self, builder: GetTransitGatewayRouteTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTableAssociationsOutput, SdkError<GetTransitGatewayRouteTableAssociationsError>>>;
    fn get_transit_gateway_route_table_propagations(&self, builder: GetTransitGatewayRouteTablePropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTablePropagationsOutput, SdkError<GetTransitGatewayRouteTablePropagationsError>>>;
    fn get_verified_access_endpoint_policy(&self, builder: GetVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessEndpointPolicyOutput, SdkError<GetVerifiedAccessEndpointPolicyError>>>;
    fn get_verified_access_group_policy(&self, builder: GetVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessGroupPolicyOutput, SdkError<GetVerifiedAccessGroupPolicyError>>>;
    fn get_vpn_connection_device_sample_configuration(&self, builder: GetVpnConnectionDeviceSampleConfigurationInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceSampleConfigurationOutput, SdkError<GetVpnConnectionDeviceSampleConfigurationError>>>;
    fn get_vpn_connection_device_types(&self, builder: GetVpnConnectionDeviceTypesInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceTypesOutput, SdkError<GetVpnConnectionDeviceTypesError>>>;
    fn get_vpn_tunnel_replacement_status(&self, builder: GetVpnTunnelReplacementStatusInputBuilder) -> impl Future<Output = Result<GetVpnTunnelReplacementStatusOutput, SdkError<GetVpnTunnelReplacementStatusError>>>;
    fn import_client_vpn_client_certificate_revocation_list(&self, builder: ImportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ImportClientVpnClientCertificateRevocationListOutput, SdkError<ImportClientVpnClientCertificateRevocationListError>>>;
    fn import_image(&self, builder: ImportImageInputBuilder) -> impl Future<Output = Result<ImportImageOutput, SdkError<ImportImageError>>>;
    fn import_key_pair(&self, builder: ImportKeyPairInputBuilder) -> impl Future<Output = Result<ImportKeyPairOutput, SdkError<ImportKeyPairError>>>;
    fn import_snapshot(&self, builder: ImportSnapshotInputBuilder) -> impl Future<Output = Result<ImportSnapshotOutput, SdkError<ImportSnapshotError>>>;
    fn list_images_in_recycle_bin(&self, builder: ListImagesInRecycleBinInputBuilder) -> impl Future<Output = Result<ListImagesInRecycleBinOutput, SdkError<ListImagesInRecycleBinError>>>;
    fn list_snapshots_in_recycle_bin(&self, builder: ListSnapshotsInRecycleBinInputBuilder) -> impl Future<Output = Result<ListSnapshotsInRecycleBinOutput, SdkError<ListSnapshotsInRecycleBinError>>>;
    fn lock_snapshot(&self, builder: LockSnapshotInputBuilder) -> impl Future<Output = Result<LockSnapshotOutput, SdkError<LockSnapshotError>>>;
    fn modify_address_attribute(&self, builder: ModifyAddressAttributeInputBuilder) -> impl Future<Output = Result<ModifyAddressAttributeOutput, SdkError<ModifyAddressAttributeError>>>;
    fn modify_availability_zone_group(&self, builder: ModifyAvailabilityZoneGroupInputBuilder) -> impl Future<Output = Result<ModifyAvailabilityZoneGroupOutput, SdkError<ModifyAvailabilityZoneGroupError>>>;
    fn modify_capacity_reservation(&self, builder: ModifyCapacityReservationInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationOutput, SdkError<ModifyCapacityReservationError>>>;
    fn modify_capacity_reservation_fleet(&self, builder: ModifyCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationFleetOutput, SdkError<ModifyCapacityReservationFleetError>>>;
    fn modify_client_vpn_endpoint(&self, builder: ModifyClientVpnEndpointInputBuilder) -> impl Future<Output = Result<ModifyClientVpnEndpointOutput, SdkError<ModifyClientVpnEndpointError>>>;
    fn modify_default_credit_specification(&self, builder: ModifyDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyDefaultCreditSpecificationOutput, SdkError<ModifyDefaultCreditSpecificationError>>>;
    fn modify_ebs_default_kms_key_id(&self, builder: ModifyEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ModifyEbsDefaultKmsKeyIdOutput, SdkError<ModifyEbsDefaultKmsKeyIdError>>>;
    fn modify_fleet(&self, builder: ModifyFleetInputBuilder) -> impl Future<Output = Result<ModifyFleetOutput, SdkError<ModifyFleetError>>>;
    fn modify_fpga_image_attribute(&self, builder: ModifyFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyFpgaImageAttributeOutput, SdkError<ModifyFpgaImageAttributeError>>>;
    fn modify_hosts(&self, builder: ModifyHostsInputBuilder) -> impl Future<Output = Result<ModifyHostsOutput, SdkError<ModifyHostsError>>>;
    fn modify_id_format(&self, builder: ModifyIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdFormatOutput, SdkError<ModifyIdFormatError>>>;
    fn modify_identity_id_format(&self, builder: ModifyIdentityIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdentityIdFormatOutput, SdkError<ModifyIdentityIdFormatError>>>;
    fn modify_image_attribute(&self, builder: ModifyImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyImageAttributeOutput, SdkError<ModifyImageAttributeError>>>;
    fn modify_instance_attribute(&self, builder: ModifyInstanceAttributeInputBuilder) -> impl Future<Output = Result<ModifyInstanceAttributeOutput, SdkError<ModifyInstanceAttributeError>>>;
    fn modify_instance_capacity_reservation_attributes(&self, builder: ModifyInstanceCapacityReservationAttributesInputBuilder) -> impl Future<Output = Result<ModifyInstanceCapacityReservationAttributesOutput, SdkError<ModifyInstanceCapacityReservationAttributesError>>>;
    fn modify_instance_credit_specification(&self, builder: ModifyInstanceCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyInstanceCreditSpecificationOutput, SdkError<ModifyInstanceCreditSpecificationError>>>;
    fn modify_instance_event_start_time(&self, builder: ModifyInstanceEventStartTimeInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventStartTimeOutput, SdkError<ModifyInstanceEventStartTimeError>>>;
    fn modify_instance_event_window(&self, builder: ModifyInstanceEventWindowInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventWindowOutput, SdkError<ModifyInstanceEventWindowError>>>;
    fn modify_instance_maintenance_options(&self, builder: ModifyInstanceMaintenanceOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMaintenanceOptionsOutput, SdkError<ModifyInstanceMaintenanceOptionsError>>>;
    fn modify_instance_metadata_defaults(&self, builder: ModifyInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataDefaultsOutput, SdkError<ModifyInstanceMetadataDefaultsError>>>;
    fn modify_instance_metadata_options(&self, builder: ModifyInstanceMetadataOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataOptionsOutput, SdkError<ModifyInstanceMetadataOptionsError>>>;
    fn modify_instance_placement(&self, builder: ModifyInstancePlacementInputBuilder) -> impl Future<Output = Result<ModifyInstancePlacementOutput, SdkError<ModifyInstancePlacementError>>>;
    fn modify_ipam(&self, builder: ModifyIpamInputBuilder) -> impl Future<Output = Result<ModifyIpamOutput, SdkError<ModifyIpamError>>>;
    fn modify_ipam_pool(&self, builder: ModifyIpamPoolInputBuilder) -> impl Future<Output = Result<ModifyIpamPoolOutput, SdkError<ModifyIpamPoolError>>>;
    fn modify_ipam_resource_cidr(&self, builder: ModifyIpamResourceCidrInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceCidrOutput, SdkError<ModifyIpamResourceCidrError>>>;
    fn modify_ipam_resource_discovery(&self, builder: ModifyIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceDiscoveryOutput, SdkError<ModifyIpamResourceDiscoveryError>>>;
    fn modify_ipam_scope(&self, builder: ModifyIpamScopeInputBuilder) -> impl Future<Output = Result<ModifyIpamScopeOutput, SdkError<ModifyIpamScopeError>>>;
    fn modify_launch_template(&self, builder: ModifyLaunchTemplateInputBuilder) -> impl Future<Output = Result<ModifyLaunchTemplateOutput, SdkError<ModifyLaunchTemplateError>>>;
    fn modify_local_gateway_route(&self, builder: ModifyLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<ModifyLocalGatewayRouteOutput, SdkError<ModifyLocalGatewayRouteError>>>;
    fn modify_managed_prefix_list(&self, builder: ModifyManagedPrefixListInputBuilder) -> impl Future<Output = Result<ModifyManagedPrefixListOutput, SdkError<ModifyManagedPrefixListError>>>;
    fn modify_network_interface_attribute(&self, builder: ModifyNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ModifyNetworkInterfaceAttributeOutput, SdkError<ModifyNetworkInterfaceAttributeError>>>;
    fn modify_private_dns_name_options(&self, builder: ModifyPrivateDnsNameOptionsInputBuilder) -> impl Future<Output = Result<ModifyPrivateDnsNameOptionsOutput, SdkError<ModifyPrivateDnsNameOptionsError>>>;
    fn modify_reserved_instances(&self, builder: ModifyReservedInstancesInputBuilder) -> impl Future<Output = Result<ModifyReservedInstancesOutput, SdkError<ModifyReservedInstancesError>>>;
    fn modify_security_group_rules(&self, builder: ModifySecurityGroupRulesInputBuilder) -> impl Future<Output = Result<ModifySecurityGroupRulesOutput, SdkError<ModifySecurityGroupRulesError>>>;
    fn modify_snapshot_attribute(&self, builder: ModifySnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifySnapshotAttributeOutput, SdkError<ModifySnapshotAttributeError>>>;
    fn modify_snapshot_tier(&self, builder: ModifySnapshotTierInputBuilder) -> impl Future<Output = Result<ModifySnapshotTierOutput, SdkError<ModifySnapshotTierError>>>;
    fn modify_spot_fleet_request(&self, builder: ModifySpotFleetRequestInputBuilder) -> impl Future<Output = Result<ModifySpotFleetRequestOutput, SdkError<ModifySpotFleetRequestError>>>;
    fn modify_subnet_attribute(&self, builder: ModifySubnetAttributeInputBuilder) -> impl Future<Output = Result<ModifySubnetAttributeOutput, SdkError<ModifySubnetAttributeError>>>;
    fn modify_traffic_mirror_filter_network_services(&self, builder: ModifyTrafficMirrorFilterNetworkServicesInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterNetworkServicesOutput, SdkError<ModifyTrafficMirrorFilterNetworkServicesError>>>;
    fn modify_traffic_mirror_filter_rule(&self, builder: ModifyTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterRuleOutput, SdkError<ModifyTrafficMirrorFilterRuleError>>>;
    fn modify_traffic_mirror_session(&self, builder: ModifyTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorSessionOutput, SdkError<ModifyTrafficMirrorSessionError>>>;
    fn modify_transit_gateway(&self, builder: ModifyTransitGatewayInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayOutput, SdkError<ModifyTransitGatewayError>>>;
    fn modify_transit_gateway_prefix_list_reference(&self, builder: ModifyTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayPrefixListReferenceOutput, SdkError<ModifyTransitGatewayPrefixListReferenceError>>>;
    fn modify_transit_gateway_vpc_attachment(&self, builder: ModifyTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayVpcAttachmentOutput, SdkError<ModifyTransitGatewayVpcAttachmentError>>>;
    fn modify_verified_access_endpoint(&self, builder: ModifyVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointOutput, SdkError<ModifyVerifiedAccessEndpointError>>>;
    fn modify_verified_access_endpoint_policy(&self, builder: ModifyVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointPolicyOutput, SdkError<ModifyVerifiedAccessEndpointPolicyError>>>;
    fn modify_verified_access_group(&self, builder: ModifyVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupOutput, SdkError<ModifyVerifiedAccessGroupError>>>;
    fn modify_verified_access_group_policy(&self, builder: ModifyVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupPolicyOutput, SdkError<ModifyVerifiedAccessGroupPolicyError>>>;
    fn modify_verified_access_instance(&self, builder: ModifyVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceOutput, SdkError<ModifyVerifiedAccessInstanceError>>>;
    fn modify_verified_access_instance_logging_configuration(&self, builder: ModifyVerifiedAccessInstanceLoggingConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceLoggingConfigurationOutput, SdkError<ModifyVerifiedAccessInstanceLoggingConfigurationError>>>;
    fn modify_verified_access_trust_provider(&self, builder: ModifyVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessTrustProviderOutput, SdkError<ModifyVerifiedAccessTrustProviderError>>>;
    fn modify_volume(&self, builder: ModifyVolumeInputBuilder) -> impl Future<Output = Result<ModifyVolumeOutput, SdkError<ModifyVolumeError>>>;
    fn modify_volume_attribute(&self, builder: ModifyVolumeAttributeInputBuilder) -> impl Future<Output = Result<ModifyVolumeAttributeOutput, SdkError<ModifyVolumeAttributeError>>>;
    fn modify_vpc_attribute(&self, builder: ModifyVpcAttributeInputBuilder) -> impl Future<Output = Result<ModifyVpcAttributeOutput, SdkError<ModifyVpcAttributeError>>>;
    fn modify_vpc_endpoint(&self, builder: ModifyVpcEndpointInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointOutput, SdkError<ModifyVpcEndpointError>>>;
    fn modify_vpc_endpoint_connection_notification(&self, builder: ModifyVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointConnectionNotificationOutput, SdkError<ModifyVpcEndpointConnectionNotificationError>>>;
    fn modify_vpc_endpoint_service_configuration(&self, builder: ModifyVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServiceConfigurationOutput, SdkError<ModifyVpcEndpointServiceConfigurationError>>>;
    fn modify_vpc_endpoint_service_payer_responsibility(&self, builder: ModifyVpcEndpointServicePayerResponsibilityInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePayerResponsibilityOutput, SdkError<ModifyVpcEndpointServicePayerResponsibilityError>>>;
    fn modify_vpc_endpoint_service_permissions(&self, builder: ModifyVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePermissionsOutput, SdkError<ModifyVpcEndpointServicePermissionsError>>>;
    fn modify_vpc_peering_connection_options(&self, builder: ModifyVpcPeeringConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpcPeeringConnectionOptionsOutput, SdkError<ModifyVpcPeeringConnectionOptionsError>>>;
    fn modify_vpc_tenancy(&self, builder: ModifyVpcTenancyInputBuilder) -> impl Future<Output = Result<ModifyVpcTenancyOutput, SdkError<ModifyVpcTenancyError>>>;
    fn modify_vpn_connection(&self, builder: ModifyVpnConnectionInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOutput, SdkError<ModifyVpnConnectionError>>>;
    fn modify_vpn_connection_options(&self, builder: ModifyVpnConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOptionsOutput, SdkError<ModifyVpnConnectionOptionsError>>>;
    fn modify_vpn_tunnel_certificate(&self, builder: ModifyVpnTunnelCertificateInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelCertificateOutput, SdkError<ModifyVpnTunnelCertificateError>>>;
    fn modify_vpn_tunnel_options(&self, builder: ModifyVpnTunnelOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelOptionsOutput, SdkError<ModifyVpnTunnelOptionsError>>>;
    fn monitor_instances(&self, builder: MonitorInstancesInputBuilder) -> impl Future<Output = Result<MonitorInstancesOutput, SdkError<MonitorInstancesError>>>;
    fn move_address_to_vpc(&self, builder: MoveAddressToVpcInputBuilder) -> impl Future<Output = Result<MoveAddressToVpcOutput, SdkError<MoveAddressToVpcError>>>;
    fn move_byoip_cidr_to_ipam(&self, builder: MoveByoipCidrToIpamInputBuilder) -> impl Future<Output = Result<MoveByoipCidrToIpamOutput, SdkError<MoveByoipCidrToIpamError>>>;
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>>;
    fn provision_ipam_byoasn(&self, builder: ProvisionIpamByoasnInputBuilder) -> impl Future<Output = Result<ProvisionIpamByoasnOutput, SdkError<ProvisionIpamByoasnError>>>;
    fn provision_ipam_pool_cidr(&self, builder: ProvisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionIpamPoolCidrOutput, SdkError<ProvisionIpamPoolCidrError>>>;
    fn provision_public_ipv4_pool_cidr(&self, builder: ProvisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionPublicIpv4PoolCidrOutput, SdkError<ProvisionPublicIpv4PoolCidrError>>>;
    fn purchase_capacity_block(&self, builder: PurchaseCapacityBlockInputBuilder) -> impl Future<Output = Result<PurchaseCapacityBlockOutput, SdkError<PurchaseCapacityBlockError>>>;
    fn purchase_host_reservation(&self, builder: PurchaseHostReservationInputBuilder) -> impl Future<Output = Result<PurchaseHostReservationOutput, SdkError<PurchaseHostReservationError>>>;
    fn purchase_reserved_instances_offering(&self, builder: PurchaseReservedInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstancesOfferingOutput, SdkError<PurchaseReservedInstancesOfferingError>>>;
    fn purchase_scheduled_instances(&self, builder: PurchaseScheduledInstancesInputBuilder) -> impl Future<Output = Result<PurchaseScheduledInstancesOutput, SdkError<PurchaseScheduledInstancesError>>>;
    fn reboot_instances(&self, builder: RebootInstancesInputBuilder) -> impl Future<Output = Result<RebootInstancesOutput, SdkError<RebootInstancesError>>>;
    fn register_image(&self, builder: RegisterImageInputBuilder) -> impl Future<Output = Result<RegisterImageOutput, SdkError<RegisterImageError>>>;
    fn register_instance_event_notification_attributes(&self, builder: RegisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<RegisterInstanceEventNotificationAttributesOutput, SdkError<RegisterInstanceEventNotificationAttributesError>>>;
    fn register_transit_gateway_multicast_group_members(&self, builder: RegisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupMembersOutput, SdkError<RegisterTransitGatewayMulticastGroupMembersError>>>;
    fn register_transit_gateway_multicast_group_sources(&self, builder: RegisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupSourcesOutput, SdkError<RegisterTransitGatewayMulticastGroupSourcesError>>>;
    fn reject_transit_gateway_multicast_domain_associations(&self, builder: RejectTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayMulticastDomainAssociationsOutput, SdkError<RejectTransitGatewayMulticastDomainAssociationsError>>>;
    fn reject_transit_gateway_peering_attachment(&self, builder: RejectTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayPeeringAttachmentOutput, SdkError<RejectTransitGatewayPeeringAttachmentError>>>;
    fn reject_transit_gateway_vpc_attachment(&self, builder: RejectTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayVpcAttachmentOutput, SdkError<RejectTransitGatewayVpcAttachmentError>>>;
    fn reject_vpc_endpoint_connections(&self, builder: RejectVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<RejectVpcEndpointConnectionsOutput, SdkError<RejectVpcEndpointConnectionsError>>>;
    fn reject_vpc_peering_connection(&self, builder: RejectVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<RejectVpcPeeringConnectionOutput, SdkError<RejectVpcPeeringConnectionError>>>;
    fn release_address(&self, builder: ReleaseAddressInputBuilder) -> impl Future<Output = Result<ReleaseAddressOutput, SdkError<ReleaseAddressError>>>;
    fn release_hosts(&self, builder: ReleaseHostsInputBuilder) -> impl Future<Output = Result<ReleaseHostsOutput, SdkError<ReleaseHostsError>>>;
    fn release_ipam_pool_allocation(&self, builder: ReleaseIpamPoolAllocationInputBuilder) -> impl Future<Output = Result<ReleaseIpamPoolAllocationOutput, SdkError<ReleaseIpamPoolAllocationError>>>;
    fn replace_iam_instance_profile_association(&self, builder: ReplaceIamInstanceProfileAssociationInputBuilder) -> impl Future<Output = Result<ReplaceIamInstanceProfileAssociationOutput, SdkError<ReplaceIamInstanceProfileAssociationError>>>;
    fn replace_network_acl_association(&self, builder: ReplaceNetworkAclAssociationInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclAssociationOutput, SdkError<ReplaceNetworkAclAssociationError>>>;
    fn replace_network_acl_entry(&self, builder: ReplaceNetworkAclEntryInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclEntryOutput, SdkError<ReplaceNetworkAclEntryError>>>;
    fn replace_route(&self, builder: ReplaceRouteInputBuilder) -> impl Future<Output = Result<ReplaceRouteOutput, SdkError<ReplaceRouteError>>>;
    fn replace_route_table_association(&self, builder: ReplaceRouteTableAssociationInputBuilder) -> impl Future<Output = Result<ReplaceRouteTableAssociationOutput, SdkError<ReplaceRouteTableAssociationError>>>;
    fn replace_transit_gateway_route(&self, builder: ReplaceTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<ReplaceTransitGatewayRouteOutput, SdkError<ReplaceTransitGatewayRouteError>>>;
    fn replace_vpn_tunnel(&self, builder: ReplaceVpnTunnelInputBuilder) -> impl Future<Output = Result<ReplaceVpnTunnelOutput, SdkError<ReplaceVpnTunnelError>>>;
    fn report_instance_status(&self, builder: ReportInstanceStatusInputBuilder) -> impl Future<Output = Result<ReportInstanceStatusOutput, SdkError<ReportInstanceStatusError>>>;
    fn request_spot_fleet(&self, builder: RequestSpotFleetInputBuilder) -> impl Future<Output = Result<RequestSpotFleetOutput, SdkError<RequestSpotFleetError>>>;
    fn request_spot_instances(&self, builder: RequestSpotInstancesInputBuilder) -> impl Future<Output = Result<RequestSpotInstancesOutput, SdkError<RequestSpotInstancesError>>>;
    fn reset_address_attribute(&self, builder: ResetAddressAttributeInputBuilder) -> impl Future<Output = Result<ResetAddressAttributeOutput, SdkError<ResetAddressAttributeError>>>;
    fn reset_ebs_default_kms_key_id(&self, builder: ResetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ResetEbsDefaultKmsKeyIdOutput, SdkError<ResetEbsDefaultKmsKeyIdError>>>;
    fn reset_fpga_image_attribute(&self, builder: ResetFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ResetFpgaImageAttributeOutput, SdkError<ResetFpgaImageAttributeError>>>;
    fn reset_image_attribute(&self, builder: ResetImageAttributeInputBuilder) -> impl Future<Output = Result<ResetImageAttributeOutput, SdkError<ResetImageAttributeError>>>;
    fn reset_instance_attribute(&self, builder: ResetInstanceAttributeInputBuilder) -> impl Future<Output = Result<ResetInstanceAttributeOutput, SdkError<ResetInstanceAttributeError>>>;
    fn reset_network_interface_attribute(&self, builder: ResetNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ResetNetworkInterfaceAttributeOutput, SdkError<ResetNetworkInterfaceAttributeError>>>;
    fn reset_snapshot_attribute(&self, builder: ResetSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ResetSnapshotAttributeOutput, SdkError<ResetSnapshotAttributeError>>>;
    fn restore_address_to_classic(&self, builder: RestoreAddressToClassicInputBuilder) -> impl Future<Output = Result<RestoreAddressToClassicOutput, SdkError<RestoreAddressToClassicError>>>;
    fn restore_image_from_recycle_bin(&self, builder: RestoreImageFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreImageFromRecycleBinOutput, SdkError<RestoreImageFromRecycleBinError>>>;
    fn restore_managed_prefix_list_version(&self, builder: RestoreManagedPrefixListVersionInputBuilder) -> impl Future<Output = Result<RestoreManagedPrefixListVersionOutput, SdkError<RestoreManagedPrefixListVersionError>>>;
    fn restore_snapshot_from_recycle_bin(&self, builder: RestoreSnapshotFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreSnapshotFromRecycleBinOutput, SdkError<RestoreSnapshotFromRecycleBinError>>>;
    fn restore_snapshot_tier(&self, builder: RestoreSnapshotTierInputBuilder) -> impl Future<Output = Result<RestoreSnapshotTierOutput, SdkError<RestoreSnapshotTierError>>>;
    fn revoke_client_vpn_ingress(&self, builder: RevokeClientVpnIngressInputBuilder) -> impl Future<Output = Result<RevokeClientVpnIngressOutput, SdkError<RevokeClientVpnIngressError>>>;
    fn revoke_security_group_egress(&self, builder: RevokeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupEgressOutput, SdkError<RevokeSecurityGroupEgressError>>>;
    fn revoke_security_group_ingress(&self, builder: RevokeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupIngressOutput, SdkError<RevokeSecurityGroupIngressError>>>;
    fn run_instances(&self, builder: RunInstancesInputBuilder) -> impl Future<Output = Result<RunInstancesOutput, SdkError<RunInstancesError>>>;
    fn run_scheduled_instances(&self, builder: RunScheduledInstancesInputBuilder) -> impl Future<Output = Result<RunScheduledInstancesOutput, SdkError<RunScheduledInstancesError>>>;
    fn search_local_gateway_routes(&self, builder: SearchLocalGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchLocalGatewayRoutesOutput, SdkError<SearchLocalGatewayRoutesError>>>;
    fn search_transit_gateway_multicast_groups(&self, builder: SearchTransitGatewayMulticastGroupsInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayMulticastGroupsOutput, SdkError<SearchTransitGatewayMulticastGroupsError>>>;
    fn search_transit_gateway_routes(&self, builder: SearchTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayRoutesOutput, SdkError<SearchTransitGatewayRoutesError>>>;
    fn send_diagnostic_interrupt(&self, builder: SendDiagnosticInterruptInputBuilder) -> impl Future<Output = Result<SendDiagnosticInterruptOutput, SdkError<SendDiagnosticInterruptError>>>;
    fn start_instances(&self, builder: StartInstancesInputBuilder) -> impl Future<Output = Result<StartInstancesOutput, SdkError<StartInstancesError>>>;
    fn start_network_insights_access_scope_analysis(&self, builder: StartNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAccessScopeAnalysisOutput, SdkError<StartNetworkInsightsAccessScopeAnalysisError>>>;
    fn start_network_insights_analysis(&self, builder: StartNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAnalysisOutput, SdkError<StartNetworkInsightsAnalysisError>>>;
    fn start_vpc_endpoint_service_private_dns_verification(&self, builder: StartVpcEndpointServicePrivateDnsVerificationInputBuilder) -> impl Future<Output = Result<StartVpcEndpointServicePrivateDnsVerificationOutput, SdkError<StartVpcEndpointServicePrivateDnsVerificationError>>>;
    fn stop_instances(&self, builder: StopInstancesInputBuilder) -> impl Future<Output = Result<StopInstancesOutput, SdkError<StopInstancesError>>>;
    fn terminate_client_vpn_connections(&self, builder: TerminateClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<TerminateClientVpnConnectionsOutput, SdkError<TerminateClientVpnConnectionsError>>>;
    fn terminate_instances(&self, builder: TerminateInstancesInputBuilder) -> impl Future<Output = Result<TerminateInstancesOutput, SdkError<TerminateInstancesError>>>;
    fn unassign_ipv6_addresses(&self, builder: UnassignIpv6AddressesInputBuilder) -> impl Future<Output = Result<UnassignIpv6AddressesOutput, SdkError<UnassignIpv6AddressesError>>>;
    fn unassign_private_ip_addresses(&self, builder: UnassignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<UnassignPrivateIpAddressesOutput, SdkError<UnassignPrivateIpAddressesError>>>;
    fn unassign_private_nat_gateway_address(&self, builder: UnassignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<UnassignPrivateNatGatewayAddressOutput, SdkError<UnassignPrivateNatGatewayAddressError>>>;
    fn unlock_snapshot(&self, builder: UnlockSnapshotInputBuilder) -> impl Future<Output = Result<UnlockSnapshotOutput, SdkError<UnlockSnapshotError>>>;
    fn unmonitor_instances(&self, builder: UnmonitorInstancesInputBuilder) -> impl Future<Output = Result<UnmonitorInstancesOutput, SdkError<UnmonitorInstancesError>>>;
    fn update_security_group_rule_descriptions_egress(&self, builder: UpdateSecurityGroupRuleDescriptionsEgressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsEgressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsEgressError>>>;
    fn update_security_group_rule_descriptions_ingress(&self, builder: UpdateSecurityGroupRuleDescriptionsIngressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsIngressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsIngressError>>>;
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>>;
}
impl EC2Client for EC2ClientImpl {
    fn accept_address_transfer(&self, builder: AcceptAddressTransferInputBuilder) -> impl Future<Output = Result<AcceptAddressTransferOutput, SdkError<AcceptAddressTransferError>>> {
        builder.send_with(&self.0)
    }
    fn accept_reserved_instances_exchange_quote(&self, builder: AcceptReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<AcceptReservedInstancesExchangeQuoteOutput, SdkError<AcceptReservedInstancesExchangeQuoteError>>> {
        builder.send_with(&self.0)
    }
    fn accept_transit_gateway_multicast_domain_associations(&self, builder: AcceptTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayMulticastDomainAssociationsOutput, SdkError<AcceptTransitGatewayMulticastDomainAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn accept_transit_gateway_peering_attachment(&self, builder: AcceptTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayPeeringAttachmentOutput, SdkError<AcceptTransitGatewayPeeringAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn accept_transit_gateway_vpc_attachment(&self, builder: AcceptTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayVpcAttachmentOutput, SdkError<AcceptTransitGatewayVpcAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn accept_vpc_endpoint_connections(&self, builder: AcceptVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<AcceptVpcEndpointConnectionsOutput, SdkError<AcceptVpcEndpointConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn accept_vpc_peering_connection(&self, builder: AcceptVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<AcceptVpcPeeringConnectionOutput, SdkError<AcceptVpcPeeringConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_address(&self, builder: AllocateAddressInputBuilder) -> impl Future<Output = Result<AllocateAddressOutput, SdkError<AllocateAddressError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_hosts(&self, builder: AllocateHostsInputBuilder) -> impl Future<Output = Result<AllocateHostsOutput, SdkError<AllocateHostsError>>> {
        builder.send_with(&self.0)
    }
    fn allocate_ipam_pool_cidr(&self, builder: AllocateIpamPoolCidrInputBuilder) -> impl Future<Output = Result<AllocateIpamPoolCidrOutput, SdkError<AllocateIpamPoolCidrError>>> {
        builder.send_with(&self.0)
    }
    fn apply_security_groups_to_client_vpn_target_network(&self, builder: ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<ApplySecurityGroupsToClientVpnTargetNetworkOutput, SdkError<ApplySecurityGroupsToClientVpnTargetNetworkError>>> {
        builder.send_with(&self.0)
    }
    fn assign_ipv6_addresses(&self, builder: AssignIpv6AddressesInputBuilder) -> impl Future<Output = Result<AssignIpv6AddressesOutput, SdkError<AssignIpv6AddressesError>>> {
        builder.send_with(&self.0)
    }
    fn assign_private_ip_addresses(&self, builder: AssignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<AssignPrivateIpAddressesOutput, SdkError<AssignPrivateIpAddressesError>>> {
        builder.send_with(&self.0)
    }
    fn assign_private_nat_gateway_address(&self, builder: AssignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssignPrivateNatGatewayAddressOutput, SdkError<AssignPrivateNatGatewayAddressError>>> {
        builder.send_with(&self.0)
    }
    fn associate_address(&self, builder: AssociateAddressInputBuilder) -> impl Future<Output = Result<AssociateAddressOutput, SdkError<AssociateAddressError>>> {
        builder.send_with(&self.0)
    }
    fn associate_client_vpn_target_network(&self, builder: AssociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<AssociateClientVpnTargetNetworkOutput, SdkError<AssociateClientVpnTargetNetworkError>>> {
        builder.send_with(&self.0)
    }
    fn associate_dhcp_options(&self, builder: AssociateDhcpOptionsInputBuilder) -> impl Future<Output = Result<AssociateDhcpOptionsOutput, SdkError<AssociateDhcpOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn associate_enclave_certificate_iam_role(&self, builder: AssociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<AssociateEnclaveCertificateIamRoleOutput, SdkError<AssociateEnclaveCertificateIamRoleError>>> {
        builder.send_with(&self.0)
    }
    fn associate_iam_instance_profile(&self, builder: AssociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<AssociateIamInstanceProfileOutput, SdkError<AssociateIamInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn associate_instance_event_window(&self, builder: AssociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<AssociateInstanceEventWindowOutput, SdkError<AssociateInstanceEventWindowError>>> {
        builder.send_with(&self.0)
    }
    fn associate_ipam_byoasn(&self, builder: AssociateIpamByoasnInputBuilder) -> impl Future<Output = Result<AssociateIpamByoasnOutput, SdkError<AssociateIpamByoasnError>>> {
        builder.send_with(&self.0)
    }
    fn associate_ipam_resource_discovery(&self, builder: AssociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<AssociateIpamResourceDiscoveryOutput, SdkError<AssociateIpamResourceDiscoveryError>>> {
        builder.send_with(&self.0)
    }
    fn associate_nat_gateway_address(&self, builder: AssociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssociateNatGatewayAddressOutput, SdkError<AssociateNatGatewayAddressError>>> {
        builder.send_with(&self.0)
    }
    fn associate_route_table(&self, builder: AssociateRouteTableInputBuilder) -> impl Future<Output = Result<AssociateRouteTableOutput, SdkError<AssociateRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn associate_subnet_cidr_block(&self, builder: AssociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateSubnetCidrBlockOutput, SdkError<AssociateSubnetCidrBlockError>>> {
        builder.send_with(&self.0)
    }
    fn associate_transit_gateway_multicast_domain(&self, builder: AssociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayMulticastDomainOutput, SdkError<AssociateTransitGatewayMulticastDomainError>>> {
        builder.send_with(&self.0)
    }
    fn associate_transit_gateway_policy_table(&self, builder: AssociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayPolicyTableOutput, SdkError<AssociateTransitGatewayPolicyTableError>>> {
        builder.send_with(&self.0)
    }
    fn associate_transit_gateway_route_table(&self, builder: AssociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayRouteTableOutput, SdkError<AssociateTransitGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn associate_trunk_interface(&self, builder: AssociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<AssociateTrunkInterfaceOutput, SdkError<AssociateTrunkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn associate_vpc_cidr_block(&self, builder: AssociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateVpcCidrBlockOutput, SdkError<AssociateVpcCidrBlockError>>> {
        builder.send_with(&self.0)
    }
    fn attach_classic_link_vpc(&self, builder: AttachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<AttachClassicLinkVpcOutput, SdkError<AttachClassicLinkVpcError>>> {
        builder.send_with(&self.0)
    }
    fn attach_internet_gateway(&self, builder: AttachInternetGatewayInputBuilder) -> impl Future<Output = Result<AttachInternetGatewayOutput, SdkError<AttachInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn attach_network_interface(&self, builder: AttachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<AttachNetworkInterfaceOutput, SdkError<AttachNetworkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn attach_verified_access_trust_provider(&self, builder: AttachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<AttachVerifiedAccessTrustProviderOutput, SdkError<AttachVerifiedAccessTrustProviderError>>> {
        builder.send_with(&self.0)
    }
    fn attach_volume(&self, builder: AttachVolumeInputBuilder) -> impl Future<Output = Result<AttachVolumeOutput, SdkError<AttachVolumeError>>> {
        builder.send_with(&self.0)
    }
    fn attach_vpn_gateway(&self, builder: AttachVpnGatewayInputBuilder) -> impl Future<Output = Result<AttachVpnGatewayOutput, SdkError<AttachVpnGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_client_vpn_ingress(&self, builder: AuthorizeClientVpnIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClientVpnIngressOutput, SdkError<AuthorizeClientVpnIngressError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_security_group_egress(&self, builder: AuthorizeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupEgressOutput, SdkError<AuthorizeSecurityGroupEgressError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_security_group_ingress(&self, builder: AuthorizeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupIngressOutput, SdkError<AuthorizeSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn bundle_instance(&self, builder: BundleInstanceInputBuilder) -> impl Future<Output = Result<BundleInstanceOutput, SdkError<BundleInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_bundle_task(&self, builder: CancelBundleTaskInputBuilder) -> impl Future<Output = Result<CancelBundleTaskOutput, SdkError<CancelBundleTaskError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_capacity_reservation_fleets(&self, builder: CancelCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationFleetsOutput, SdkError<CancelCapacityReservationFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_conversion_task(&self, builder: CancelConversionTaskInputBuilder) -> impl Future<Output = Result<CancelConversionTaskOutput, SdkError<CancelConversionTaskError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_image_launch_permission(&self, builder: CancelImageLaunchPermissionInputBuilder) -> impl Future<Output = Result<CancelImageLaunchPermissionOutput, SdkError<CancelImageLaunchPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_import_task(&self, builder: CancelImportTaskInputBuilder) -> impl Future<Output = Result<CancelImportTaskOutput, SdkError<CancelImportTaskError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_reserved_instances_listing(&self, builder: CancelReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CancelReservedInstancesListingOutput, SdkError<CancelReservedInstancesListingError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_spot_fleet_requests(&self, builder: CancelSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotFleetRequestsOutput, SdkError<CancelSpotFleetRequestsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_spot_instance_requests(&self, builder: CancelSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotInstanceRequestsOutput, SdkError<CancelSpotInstanceRequestsError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_product_instance(&self, builder: ConfirmProductInstanceInputBuilder) -> impl Future<Output = Result<ConfirmProductInstanceOutput, SdkError<ConfirmProductInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn copy_fpga_image(&self, builder: CopyFpgaImageInputBuilder) -> impl Future<Output = Result<CopyFpgaImageOutput, SdkError<CopyFpgaImageError>>> {
        builder.send_with(&self.0)
    }
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>> {
        builder.send_with(&self.0)
    }
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn create_capacity_reservation_fleet(&self, builder: CreateCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationFleetOutput, SdkError<CreateCapacityReservationFleetError>>> {
        builder.send_with(&self.0)
    }
    fn create_carrier_gateway(&self, builder: CreateCarrierGatewayInputBuilder) -> impl Future<Output = Result<CreateCarrierGatewayOutput, SdkError<CreateCarrierGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_client_vpn_endpoint(&self, builder: CreateClientVpnEndpointInputBuilder) -> impl Future<Output = Result<CreateClientVpnEndpointOutput, SdkError<CreateClientVpnEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_client_vpn_route(&self, builder: CreateClientVpnRouteInputBuilder) -> impl Future<Output = Result<CreateClientVpnRouteOutput, SdkError<CreateClientVpnRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_coip_cidr(&self, builder: CreateCoipCidrInputBuilder) -> impl Future<Output = Result<CreateCoipCidrOutput, SdkError<CreateCoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn create_coip_pool(&self, builder: CreateCoipPoolInputBuilder) -> impl Future<Output = Result<CreateCoipPoolOutput, SdkError<CreateCoipPoolError>>> {
        builder.send_with(&self.0)
    }
    fn create_customer_gateway(&self, builder: CreateCustomerGatewayInputBuilder) -> impl Future<Output = Result<CreateCustomerGatewayOutput, SdkError<CreateCustomerGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_default_subnet(&self, builder: CreateDefaultSubnetInputBuilder) -> impl Future<Output = Result<CreateDefaultSubnetOutput, SdkError<CreateDefaultSubnetError>>> {
        builder.send_with(&self.0)
    }
    fn create_default_vpc(&self, builder: CreateDefaultVpcInputBuilder) -> impl Future<Output = Result<CreateDefaultVpcOutput, SdkError<CreateDefaultVpcError>>> {
        builder.send_with(&self.0)
    }
    fn create_dhcp_options(&self, builder: CreateDhcpOptionsInputBuilder) -> impl Future<Output = Result<CreateDhcpOptionsOutput, SdkError<CreateDhcpOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn create_egress_only_internet_gateway(&self, builder: CreateEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateEgressOnlyInternetGatewayOutput, SdkError<CreateEgressOnlyInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        builder.send_with(&self.0)
    }
    fn create_flow_logs(&self, builder: CreateFlowLogsInputBuilder) -> impl Future<Output = Result<CreateFlowLogsOutput, SdkError<CreateFlowLogsError>>> {
        builder.send_with(&self.0)
    }
    fn create_fpga_image(&self, builder: CreateFpgaImageInputBuilder) -> impl Future<Output = Result<CreateFpgaImageOutput, SdkError<CreateFpgaImageError>>> {
        builder.send_with(&self.0)
    }
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>> {
        builder.send_with(&self.0)
    }
    fn create_instance_connect_endpoint(&self, builder: CreateInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<CreateInstanceConnectEndpointOutput, SdkError<CreateInstanceConnectEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_instance_event_window(&self, builder: CreateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<CreateInstanceEventWindowOutput, SdkError<CreateInstanceEventWindowError>>> {
        builder.send_with(&self.0)
    }
    fn create_instance_export_task(&self, builder: CreateInstanceExportTaskInputBuilder) -> impl Future<Output = Result<CreateInstanceExportTaskOutput, SdkError<CreateInstanceExportTaskError>>> {
        builder.send_with(&self.0)
    }
    fn create_internet_gateway(&self, builder: CreateInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateInternetGatewayOutput, SdkError<CreateInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_ipam(&self, builder: CreateIpamInputBuilder) -> impl Future<Output = Result<CreateIpamOutput, SdkError<CreateIpamError>>> {
        builder.send_with(&self.0)
    }
    fn create_ipam_external_resource_verification_token(&self, builder: CreateIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<CreateIpamExternalResourceVerificationTokenOutput, SdkError<CreateIpamExternalResourceVerificationTokenError>>> {
        builder.send_with(&self.0)
    }
    fn create_ipam_pool(&self, builder: CreateIpamPoolInputBuilder) -> impl Future<Output = Result<CreateIpamPoolOutput, SdkError<CreateIpamPoolError>>> {
        builder.send_with(&self.0)
    }
    fn create_ipam_resource_discovery(&self, builder: CreateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<CreateIpamResourceDiscoveryOutput, SdkError<CreateIpamResourceDiscoveryError>>> {
        builder.send_with(&self.0)
    }
    fn create_ipam_scope(&self, builder: CreateIpamScopeInputBuilder) -> impl Future<Output = Result<CreateIpamScopeOutput, SdkError<CreateIpamScopeError>>> {
        builder.send_with(&self.0)
    }
    fn create_key_pair(&self, builder: CreateKeyPairInputBuilder) -> impl Future<Output = Result<CreateKeyPairOutput, SdkError<CreateKeyPairError>>> {
        builder.send_with(&self.0)
    }
    fn create_launch_template(&self, builder: CreateLaunchTemplateInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateOutput, SdkError<CreateLaunchTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn create_launch_template_version(&self, builder: CreateLaunchTemplateVersionInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateVersionOutput, SdkError<CreateLaunchTemplateVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_local_gateway_route(&self, builder: CreateLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteOutput, SdkError<CreateLocalGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_local_gateway_route_table(&self, builder: CreateLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableOutput, SdkError<CreateLocalGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_local_gateway_route_table_virtual_interface_group_association(&self, builder: CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_local_gateway_route_table_vpc_association(&self, builder: CreateLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVpcAssociationOutput, SdkError<CreateLocalGatewayRouteTableVpcAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_managed_prefix_list(&self, builder: CreateManagedPrefixListInputBuilder) -> impl Future<Output = Result<CreateManagedPrefixListOutput, SdkError<CreateManagedPrefixListError>>> {
        builder.send_with(&self.0)
    }
    fn create_nat_gateway(&self, builder: CreateNatGatewayInputBuilder) -> impl Future<Output = Result<CreateNatGatewayOutput, SdkError<CreateNatGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_acl(&self, builder: CreateNetworkAclInputBuilder) -> impl Future<Output = Result<CreateNetworkAclOutput, SdkError<CreateNetworkAclError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_acl_entry(&self, builder: CreateNetworkAclEntryInputBuilder) -> impl Future<Output = Result<CreateNetworkAclEntryOutput, SdkError<CreateNetworkAclEntryError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_insights_access_scope(&self, builder: CreateNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsAccessScopeOutput, SdkError<CreateNetworkInsightsAccessScopeError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_insights_path(&self, builder: CreateNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsPathOutput, SdkError<CreateNetworkInsightsPathError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_interface(&self, builder: CreateNetworkInterfaceInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfaceOutput, SdkError<CreateNetworkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn create_network_interface_permission(&self, builder: CreateNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfacePermissionOutput, SdkError<CreateNetworkInterfacePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn create_placement_group(&self, builder: CreatePlacementGroupInputBuilder) -> impl Future<Output = Result<CreatePlacementGroupOutput, SdkError<CreatePlacementGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_public_ipv4_pool(&self, builder: CreatePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<CreatePublicIpv4PoolOutput, SdkError<CreatePublicIpv4PoolError>>> {
        builder.send_with(&self.0)
    }
    fn create_replace_root_volume_task(&self, builder: CreateReplaceRootVolumeTaskInputBuilder) -> impl Future<Output = Result<CreateReplaceRootVolumeTaskOutput, SdkError<CreateReplaceRootVolumeTaskError>>> {
        builder.send_with(&self.0)
    }
    fn create_reserved_instances_listing(&self, builder: CreateReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CreateReservedInstancesListingOutput, SdkError<CreateReservedInstancesListingError>>> {
        builder.send_with(&self.0)
    }
    fn create_restore_image_task(&self, builder: CreateRestoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateRestoreImageTaskOutput, SdkError<CreateRestoreImageTaskError>>> {
        builder.send_with(&self.0)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_route_table(&self, builder: CreateRouteTableInputBuilder) -> impl Future<Output = Result<CreateRouteTableOutput, SdkError<CreateRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_security_group(&self, builder: CreateSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateSecurityGroupOutput, SdkError<CreateSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_snapshots(&self, builder: CreateSnapshotsInputBuilder) -> impl Future<Output = Result<CreateSnapshotsOutput, SdkError<CreateSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn create_spot_datafeed_subscription(&self, builder: CreateSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<CreateSpotDatafeedSubscriptionOutput, SdkError<CreateSpotDatafeedSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_store_image_task(&self, builder: CreateStoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateStoreImageTaskOutput, SdkError<CreateStoreImageTaskError>>> {
        builder.send_with(&self.0)
    }
    fn create_subnet(&self, builder: CreateSubnetInputBuilder) -> impl Future<Output = Result<CreateSubnetOutput, SdkError<CreateSubnetError>>> {
        builder.send_with(&self.0)
    }
    fn create_subnet_cidr_reservation(&self, builder: CreateSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<CreateSubnetCidrReservationOutput, SdkError<CreateSubnetCidrReservationError>>> {
        builder.send_with(&self.0)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        builder.send_with(&self.0)
    }
    fn create_traffic_mirror_filter(&self, builder: CreateTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterOutput, SdkError<CreateTrafficMirrorFilterError>>> {
        builder.send_with(&self.0)
    }
    fn create_traffic_mirror_filter_rule(&self, builder: CreateTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterRuleOutput, SdkError<CreateTrafficMirrorFilterRuleError>>> {
        builder.send_with(&self.0)
    }
    fn create_traffic_mirror_session(&self, builder: CreateTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorSessionOutput, SdkError<CreateTrafficMirrorSessionError>>> {
        builder.send_with(&self.0)
    }
    fn create_traffic_mirror_target(&self, builder: CreateTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorTargetOutput, SdkError<CreateTrafficMirrorTargetError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway(&self, builder: CreateTransitGatewayInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayOutput, SdkError<CreateTransitGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_connect(&self, builder: CreateTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectOutput, SdkError<CreateTransitGatewayConnectError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_connect_peer(&self, builder: CreateTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectPeerOutput, SdkError<CreateTransitGatewayConnectPeerError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_multicast_domain(&self, builder: CreateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayMulticastDomainOutput, SdkError<CreateTransitGatewayMulticastDomainError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_peering_attachment(&self, builder: CreateTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPeeringAttachmentOutput, SdkError<CreateTransitGatewayPeeringAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_policy_table(&self, builder: CreateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPolicyTableOutput, SdkError<CreateTransitGatewayPolicyTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_prefix_list_reference(&self, builder: CreateTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPrefixListReferenceOutput, SdkError<CreateTransitGatewayPrefixListReferenceError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_route(&self, builder: CreateTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteOutput, SdkError<CreateTransitGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_route_table(&self, builder: CreateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableOutput, SdkError<CreateTransitGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_route_table_announcement(&self, builder: CreateTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableAnnouncementOutput, SdkError<CreateTransitGatewayRouteTableAnnouncementError>>> {
        builder.send_with(&self.0)
    }
    fn create_transit_gateway_vpc_attachment(&self, builder: CreateTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayVpcAttachmentOutput, SdkError<CreateTransitGatewayVpcAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_verified_access_endpoint(&self, builder: CreateVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessEndpointOutput, SdkError<CreateVerifiedAccessEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_verified_access_group(&self, builder: CreateVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessGroupOutput, SdkError<CreateVerifiedAccessGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_verified_access_instance(&self, builder: CreateVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessInstanceOutput, SdkError<CreateVerifiedAccessInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn create_verified_access_trust_provider(&self, builder: CreateVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessTrustProviderOutput, SdkError<CreateVerifiedAccessTrustProviderError>>> {
        builder.send_with(&self.0)
    }
    fn create_volume(&self, builder: CreateVolumeInputBuilder) -> impl Future<Output = Result<CreateVolumeOutput, SdkError<CreateVolumeError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc(&self, builder: CreateVpcInputBuilder) -> impl Future<Output = Result<CreateVpcOutput, SdkError<CreateVpcError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_endpoint_connection_notification(&self, builder: CreateVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointConnectionNotificationOutput, SdkError<CreateVpcEndpointConnectionNotificationError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_endpoint_service_configuration(&self, builder: CreateVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointServiceConfigurationOutput, SdkError<CreateVpcEndpointServiceConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_peering_connection(&self, builder: CreateVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcPeeringConnectionOutput, SdkError<CreateVpcPeeringConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpn_connection(&self, builder: CreateVpnConnectionInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionOutput, SdkError<CreateVpnConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpn_connection_route(&self, builder: CreateVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionRouteOutput, SdkError<CreateVpnConnectionRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpn_gateway(&self, builder: CreateVpnGatewayInputBuilder) -> impl Future<Output = Result<CreateVpnGatewayOutput, SdkError<CreateVpnGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_carrier_gateway(&self, builder: DeleteCarrierGatewayInputBuilder) -> impl Future<Output = Result<DeleteCarrierGatewayOutput, SdkError<DeleteCarrierGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_client_vpn_endpoint(&self, builder: DeleteClientVpnEndpointInputBuilder) -> impl Future<Output = Result<DeleteClientVpnEndpointOutput, SdkError<DeleteClientVpnEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_client_vpn_route(&self, builder: DeleteClientVpnRouteInputBuilder) -> impl Future<Output = Result<DeleteClientVpnRouteOutput, SdkError<DeleteClientVpnRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_coip_cidr(&self, builder: DeleteCoipCidrInputBuilder) -> impl Future<Output = Result<DeleteCoipCidrOutput, SdkError<DeleteCoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn delete_coip_pool(&self, builder: DeleteCoipPoolInputBuilder) -> impl Future<Output = Result<DeleteCoipPoolOutput, SdkError<DeleteCoipPoolError>>> {
        builder.send_with(&self.0)
    }
    fn delete_customer_gateway(&self, builder: DeleteCustomerGatewayInputBuilder) -> impl Future<Output = Result<DeleteCustomerGatewayOutput, SdkError<DeleteCustomerGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dhcp_options(&self, builder: DeleteDhcpOptionsInputBuilder) -> impl Future<Output = Result<DeleteDhcpOptionsOutput, SdkError<DeleteDhcpOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_egress_only_internet_gateway(&self, builder: DeleteEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteEgressOnlyInternetGatewayOutput, SdkError<DeleteEgressOnlyInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_fleets(&self, builder: DeleteFleetsInputBuilder) -> impl Future<Output = Result<DeleteFleetsOutput, SdkError<DeleteFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_flow_logs(&self, builder: DeleteFlowLogsInputBuilder) -> impl Future<Output = Result<DeleteFlowLogsOutput, SdkError<DeleteFlowLogsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_fpga_image(&self, builder: DeleteFpgaImageInputBuilder) -> impl Future<Output = Result<DeleteFpgaImageOutput, SdkError<DeleteFpgaImageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_instance_connect_endpoint(&self, builder: DeleteInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<DeleteInstanceConnectEndpointOutput, SdkError<DeleteInstanceConnectEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_instance_event_window(&self, builder: DeleteInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DeleteInstanceEventWindowOutput, SdkError<DeleteInstanceEventWindowError>>> {
        builder.send_with(&self.0)
    }
    fn delete_internet_gateway(&self, builder: DeleteInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteInternetGatewayOutput, SdkError<DeleteInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ipam(&self, builder: DeleteIpamInputBuilder) -> impl Future<Output = Result<DeleteIpamOutput, SdkError<DeleteIpamError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ipam_external_resource_verification_token(&self, builder: DeleteIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<DeleteIpamExternalResourceVerificationTokenOutput, SdkError<DeleteIpamExternalResourceVerificationTokenError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ipam_pool(&self, builder: DeleteIpamPoolInputBuilder) -> impl Future<Output = Result<DeleteIpamPoolOutput, SdkError<DeleteIpamPoolError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ipam_resource_discovery(&self, builder: DeleteIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DeleteIpamResourceDiscoveryOutput, SdkError<DeleteIpamResourceDiscoveryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ipam_scope(&self, builder: DeleteIpamScopeInputBuilder) -> impl Future<Output = Result<DeleteIpamScopeOutput, SdkError<DeleteIpamScopeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_key_pair(&self, builder: DeleteKeyPairInputBuilder) -> impl Future<Output = Result<DeleteKeyPairOutput, SdkError<DeleteKeyPairError>>> {
        builder.send_with(&self.0)
    }
    fn delete_launch_template(&self, builder: DeleteLaunchTemplateInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateOutput, SdkError<DeleteLaunchTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_launch_template_versions(&self, builder: DeleteLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateVersionsOutput, SdkError<DeleteLaunchTemplateVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_local_gateway_route(&self, builder: DeleteLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteOutput, SdkError<DeleteLocalGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_local_gateway_route_table(&self, builder: DeleteLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableOutput, SdkError<DeleteLocalGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_local_gateway_route_table_virtual_interface_group_association(&self, builder: DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_local_gateway_route_table_vpc_association(&self, builder: DeleteLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVpcAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVpcAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_managed_prefix_list(&self, builder: DeleteManagedPrefixListInputBuilder) -> impl Future<Output = Result<DeleteManagedPrefixListOutput, SdkError<DeleteManagedPrefixListError>>> {
        builder.send_with(&self.0)
    }
    fn delete_nat_gateway(&self, builder: DeleteNatGatewayInputBuilder) -> impl Future<Output = Result<DeleteNatGatewayOutput, SdkError<DeleteNatGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_acl(&self, builder: DeleteNetworkAclInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclOutput, SdkError<DeleteNetworkAclError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_acl_entry(&self, builder: DeleteNetworkAclEntryInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclEntryOutput, SdkError<DeleteNetworkAclEntryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_insights_access_scope(&self, builder: DeleteNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeOutput, SdkError<DeleteNetworkInsightsAccessScopeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_insights_access_scope_analysis(&self, builder: DeleteNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeAnalysisOutput, SdkError<DeleteNetworkInsightsAccessScopeAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_insights_analysis(&self, builder: DeleteNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAnalysisOutput, SdkError<DeleteNetworkInsightsAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_insights_path(&self, builder: DeleteNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsPathOutput, SdkError<DeleteNetworkInsightsPathError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_interface(&self, builder: DeleteNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfaceOutput, SdkError<DeleteNetworkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_network_interface_permission(&self, builder: DeleteNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfacePermissionOutput, SdkError<DeleteNetworkInterfacePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_placement_group(&self, builder: DeletePlacementGroupInputBuilder) -> impl Future<Output = Result<DeletePlacementGroupOutput, SdkError<DeletePlacementGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_public_ipv4_pool(&self, builder: DeletePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<DeletePublicIpv4PoolOutput, SdkError<DeletePublicIpv4PoolError>>> {
        builder.send_with(&self.0)
    }
    fn delete_queued_reserved_instances(&self, builder: DeleteQueuedReservedInstancesInputBuilder) -> impl Future<Output = Result<DeleteQueuedReservedInstancesOutput, SdkError<DeleteQueuedReservedInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route_table(&self, builder: DeleteRouteTableInputBuilder) -> impl Future<Output = Result<DeleteRouteTableOutput, SdkError<DeleteRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_security_group(&self, builder: DeleteSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteSecurityGroupOutput, SdkError<DeleteSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_spot_datafeed_subscription(&self, builder: DeleteSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteSpotDatafeedSubscriptionOutput, SdkError<DeleteSpotDatafeedSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_subnet(&self, builder: DeleteSubnetInputBuilder) -> impl Future<Output = Result<DeleteSubnetOutput, SdkError<DeleteSubnetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_subnet_cidr_reservation(&self, builder: DeleteSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<DeleteSubnetCidrReservationOutput, SdkError<DeleteSubnetCidrReservationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_traffic_mirror_filter(&self, builder: DeleteTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterOutput, SdkError<DeleteTrafficMirrorFilterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_traffic_mirror_filter_rule(&self, builder: DeleteTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterRuleOutput, SdkError<DeleteTrafficMirrorFilterRuleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_traffic_mirror_session(&self, builder: DeleteTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorSessionOutput, SdkError<DeleteTrafficMirrorSessionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_traffic_mirror_target(&self, builder: DeleteTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorTargetOutput, SdkError<DeleteTrafficMirrorTargetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway(&self, builder: DeleteTransitGatewayInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayOutput, SdkError<DeleteTransitGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_connect(&self, builder: DeleteTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectOutput, SdkError<DeleteTransitGatewayConnectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_connect_peer(&self, builder: DeleteTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectPeerOutput, SdkError<DeleteTransitGatewayConnectPeerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_multicast_domain(&self, builder: DeleteTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayMulticastDomainOutput, SdkError<DeleteTransitGatewayMulticastDomainError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_peering_attachment(&self, builder: DeleteTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPeeringAttachmentOutput, SdkError<DeleteTransitGatewayPeeringAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_policy_table(&self, builder: DeleteTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPolicyTableOutput, SdkError<DeleteTransitGatewayPolicyTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_prefix_list_reference(&self, builder: DeleteTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPrefixListReferenceOutput, SdkError<DeleteTransitGatewayPrefixListReferenceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_route(&self, builder: DeleteTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteOutput, SdkError<DeleteTransitGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_route_table(&self, builder: DeleteTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableOutput, SdkError<DeleteTransitGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_route_table_announcement(&self, builder: DeleteTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableAnnouncementOutput, SdkError<DeleteTransitGatewayRouteTableAnnouncementError>>> {
        builder.send_with(&self.0)
    }
    fn delete_transit_gateway_vpc_attachment(&self, builder: DeleteTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayVpcAttachmentOutput, SdkError<DeleteTransitGatewayVpcAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_verified_access_endpoint(&self, builder: DeleteVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessEndpointOutput, SdkError<DeleteVerifiedAccessEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_verified_access_group(&self, builder: DeleteVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessGroupOutput, SdkError<DeleteVerifiedAccessGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_verified_access_instance(&self, builder: DeleteVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessInstanceOutput, SdkError<DeleteVerifiedAccessInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_verified_access_trust_provider(&self, builder: DeleteVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessTrustProviderOutput, SdkError<DeleteVerifiedAccessTrustProviderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_volume(&self, builder: DeleteVolumeInputBuilder) -> impl Future<Output = Result<DeleteVolumeOutput, SdkError<DeleteVolumeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc(&self, builder: DeleteVpcInputBuilder) -> impl Future<Output = Result<DeleteVpcOutput, SdkError<DeleteVpcError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_endpoint_connection_notifications(&self, builder: DeleteVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointConnectionNotificationsOutput, SdkError<DeleteVpcEndpointConnectionNotificationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_endpoint_service_configurations(&self, builder: DeleteVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointServiceConfigurationsOutput, SdkError<DeleteVpcEndpointServiceConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_endpoints(&self, builder: DeleteVpcEndpointsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointsOutput, SdkError<DeleteVpcEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_peering_connection(&self, builder: DeleteVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcPeeringConnectionOutput, SdkError<DeleteVpcPeeringConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpn_connection(&self, builder: DeleteVpnConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionOutput, SdkError<DeleteVpnConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpn_connection_route(&self, builder: DeleteVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionRouteOutput, SdkError<DeleteVpnConnectionRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpn_gateway(&self, builder: DeleteVpnGatewayInputBuilder) -> impl Future<Output = Result<DeleteVpnGatewayOutput, SdkError<DeleteVpnGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn deprovision_ipam_byoasn(&self, builder: DeprovisionIpamByoasnInputBuilder) -> impl Future<Output = Result<DeprovisionIpamByoasnOutput, SdkError<DeprovisionIpamByoasnError>>> {
        builder.send_with(&self.0)
    }
    fn deprovision_ipam_pool_cidr(&self, builder: DeprovisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionIpamPoolCidrOutput, SdkError<DeprovisionIpamPoolCidrError>>> {
        builder.send_with(&self.0)
    }
    fn deprovision_public_ipv4_pool_cidr(&self, builder: DeprovisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionPublicIpv4PoolCidrOutput, SdkError<DeprovisionPublicIpv4PoolCidrError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_image(&self, builder: DeregisterImageInputBuilder) -> impl Future<Output = Result<DeregisterImageOutput, SdkError<DeregisterImageError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_instance_event_notification_attributes(&self, builder: DeregisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DeregisterInstanceEventNotificationAttributesOutput, SdkError<DeregisterInstanceEventNotificationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_transit_gateway_multicast_group_members(&self, builder: DeregisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupMembersOutput, SdkError<DeregisterTransitGatewayMulticastGroupMembersError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_transit_gateway_multicast_group_sources(&self, builder: DeregisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupSourcesOutput, SdkError<DeregisterTransitGatewayMulticastGroupSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_address_transfers(&self, builder: DescribeAddressTransfersInputBuilder) -> impl Future<Output = Result<DescribeAddressTransfersOutput, SdkError<DescribeAddressTransfersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_addresses(&self, builder: DescribeAddressesInputBuilder) -> impl Future<Output = Result<DescribeAddressesOutput, SdkError<DescribeAddressesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_addresses_attribute(&self, builder: DescribeAddressesAttributeInputBuilder) -> impl Future<Output = Result<DescribeAddressesAttributeOutput, SdkError<DescribeAddressesAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_aggregate_id_format(&self, builder: DescribeAggregateIdFormatInputBuilder) -> impl Future<Output = Result<DescribeAggregateIdFormatOutput, SdkError<DescribeAggregateIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn describe_availability_zones(&self, builder: DescribeAvailabilityZonesInputBuilder) -> impl Future<Output = Result<DescribeAvailabilityZonesOutput, SdkError<DescribeAvailabilityZonesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_aws_network_performance_metric_subscriptions(&self, builder: DescribeAwsNetworkPerformanceMetricSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeAwsNetworkPerformanceMetricSubscriptionsOutput, SdkError<DescribeAwsNetworkPerformanceMetricSubscriptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_bundle_tasks(&self, builder: DescribeBundleTasksInputBuilder) -> impl Future<Output = Result<DescribeBundleTasksOutput, SdkError<DescribeBundleTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_byoip_cidrs(&self, builder: DescribeByoipCidrsInputBuilder) -> impl Future<Output = Result<DescribeByoipCidrsOutput, SdkError<DescribeByoipCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_capacity_block_offerings(&self, builder: DescribeCapacityBlockOfferingsInputBuilder) -> impl Future<Output = Result<DescribeCapacityBlockOfferingsOutput, SdkError<DescribeCapacityBlockOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_capacity_reservation_fleets(&self, builder: DescribeCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationFleetsOutput, SdkError<DescribeCapacityReservationFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_capacity_reservations(&self, builder: DescribeCapacityReservationsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationsOutput, SdkError<DescribeCapacityReservationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_carrier_gateways(&self, builder: DescribeCarrierGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCarrierGatewaysOutput, SdkError<DescribeCarrierGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_classic_link_instances(&self, builder: DescribeClassicLinkInstancesInputBuilder) -> impl Future<Output = Result<DescribeClassicLinkInstancesOutput, SdkError<DescribeClassicLinkInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_client_vpn_authorization_rules(&self, builder: DescribeClientVpnAuthorizationRulesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnAuthorizationRulesOutput, SdkError<DescribeClientVpnAuthorizationRulesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_client_vpn_connections(&self, builder: DescribeClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnConnectionsOutput, SdkError<DescribeClientVpnConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_client_vpn_endpoints(&self, builder: DescribeClientVpnEndpointsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnEndpointsOutput, SdkError<DescribeClientVpnEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_client_vpn_routes(&self, builder: DescribeClientVpnRoutesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnRoutesOutput, SdkError<DescribeClientVpnRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_client_vpn_target_networks(&self, builder: DescribeClientVpnTargetNetworksInputBuilder) -> impl Future<Output = Result<DescribeClientVpnTargetNetworksOutput, SdkError<DescribeClientVpnTargetNetworksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_coip_pools(&self, builder: DescribeCoipPoolsInputBuilder) -> impl Future<Output = Result<DescribeCoipPoolsOutput, SdkError<DescribeCoipPoolsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_conversion_tasks(&self, builder: DescribeConversionTasksInputBuilder) -> impl Future<Output = Result<DescribeConversionTasksOutput, SdkError<DescribeConversionTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_customer_gateways(&self, builder: DescribeCustomerGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCustomerGatewaysOutput, SdkError<DescribeCustomerGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dhcp_options(&self, builder: DescribeDhcpOptionsInputBuilder) -> impl Future<Output = Result<DescribeDhcpOptionsOutput, SdkError<DescribeDhcpOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_egress_only_internet_gateways(&self, builder: DescribeEgressOnlyInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeEgressOnlyInternetGatewaysOutput, SdkError<DescribeEgressOnlyInternetGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_elastic_gpus(&self, builder: DescribeElasticGpusInputBuilder) -> impl Future<Output = Result<DescribeElasticGpusOutput, SdkError<DescribeElasticGpusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_export_image_tasks(&self, builder: DescribeExportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeExportImageTasksOutput, SdkError<DescribeExportImageTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fast_launch_images(&self, builder: DescribeFastLaunchImagesInputBuilder) -> impl Future<Output = Result<DescribeFastLaunchImagesOutput, SdkError<DescribeFastLaunchImagesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fast_snapshot_restores(&self, builder: DescribeFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DescribeFastSnapshotRestoresOutput, SdkError<DescribeFastSnapshotRestoresError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fleet_history(&self, builder: DescribeFleetHistoryInputBuilder) -> impl Future<Output = Result<DescribeFleetHistoryOutput, SdkError<DescribeFleetHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fleet_instances(&self, builder: DescribeFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeFleetInstancesOutput, SdkError<DescribeFleetInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_flow_logs(&self, builder: DescribeFlowLogsInputBuilder) -> impl Future<Output = Result<DescribeFlowLogsOutput, SdkError<DescribeFlowLogsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fpga_image_attribute(&self, builder: DescribeFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeFpgaImageAttributeOutput, SdkError<DescribeFpgaImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_fpga_images(&self, builder: DescribeFpgaImagesInputBuilder) -> impl Future<Output = Result<DescribeFpgaImagesOutput, SdkError<DescribeFpgaImagesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_host_reservation_offerings(&self, builder: DescribeHostReservationOfferingsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationOfferingsOutput, SdkError<DescribeHostReservationOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_host_reservations(&self, builder: DescribeHostReservationsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationsOutput, SdkError<DescribeHostReservationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hosts(&self, builder: DescribeHostsInputBuilder) -> impl Future<Output = Result<DescribeHostsOutput, SdkError<DescribeHostsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_iam_instance_profile_associations(&self, builder: DescribeIamInstanceProfileAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIamInstanceProfileAssociationsOutput, SdkError<DescribeIamInstanceProfileAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_id_format(&self, builder: DescribeIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdFormatOutput, SdkError<DescribeIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn describe_identity_id_format(&self, builder: DescribeIdentityIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdentityIdFormatOutput, SdkError<DescribeIdentityIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn describe_image_attribute(&self, builder: DescribeImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeImageAttributeOutput, SdkError<DescribeImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_import_image_tasks(&self, builder: DescribeImportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeImportImageTasksOutput, SdkError<DescribeImportImageTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_import_snapshot_tasks(&self, builder: DescribeImportSnapshotTasksInputBuilder) -> impl Future<Output = Result<DescribeImportSnapshotTasksOutput, SdkError<DescribeImportSnapshotTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_attribute(&self, builder: DescribeInstanceAttributeInputBuilder) -> impl Future<Output = Result<DescribeInstanceAttributeOutput, SdkError<DescribeInstanceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_connect_endpoints(&self, builder: DescribeInstanceConnectEndpointsInputBuilder) -> impl Future<Output = Result<DescribeInstanceConnectEndpointsOutput, SdkError<DescribeInstanceConnectEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_credit_specifications(&self, builder: DescribeInstanceCreditSpecificationsInputBuilder) -> impl Future<Output = Result<DescribeInstanceCreditSpecificationsOutput, SdkError<DescribeInstanceCreditSpecificationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_event_notification_attributes(&self, builder: DescribeInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventNotificationAttributesOutput, SdkError<DescribeInstanceEventNotificationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_event_windows(&self, builder: DescribeInstanceEventWindowsInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventWindowsOutput, SdkError<DescribeInstanceEventWindowsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_status(&self, builder: DescribeInstanceStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceStatusOutput, SdkError<DescribeInstanceStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_topology(&self, builder: DescribeInstanceTopologyInputBuilder) -> impl Future<Output = Result<DescribeInstanceTopologyOutput, SdkError<DescribeInstanceTopologyError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_type_offerings(&self, builder: DescribeInstanceTypeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeOfferingsOutput, SdkError<DescribeInstanceTypeOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_types(&self, builder: DescribeInstanceTypesInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypesOutput, SdkError<DescribeInstanceTypesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instances(&self, builder: DescribeInstancesInputBuilder) -> impl Future<Output = Result<DescribeInstancesOutput, SdkError<DescribeInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_internet_gateways(&self, builder: DescribeInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeInternetGatewaysOutput, SdkError<DescribeInternetGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_byoasn(&self, builder: DescribeIpamByoasnInputBuilder) -> impl Future<Output = Result<DescribeIpamByoasnOutput, SdkError<DescribeIpamByoasnError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_external_resource_verification_tokens(&self, builder: DescribeIpamExternalResourceVerificationTokensInputBuilder) -> impl Future<Output = Result<DescribeIpamExternalResourceVerificationTokensOutput, SdkError<DescribeIpamExternalResourceVerificationTokensError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_pools(&self, builder: DescribeIpamPoolsInputBuilder) -> impl Future<Output = Result<DescribeIpamPoolsOutput, SdkError<DescribeIpamPoolsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_resource_discoveries(&self, builder: DescribeIpamResourceDiscoveriesInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveriesOutput, SdkError<DescribeIpamResourceDiscoveriesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_resource_discovery_associations(&self, builder: DescribeIpamResourceDiscoveryAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveryAssociationsOutput, SdkError<DescribeIpamResourceDiscoveryAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipam_scopes(&self, builder: DescribeIpamScopesInputBuilder) -> impl Future<Output = Result<DescribeIpamScopesOutput, SdkError<DescribeIpamScopesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipams(&self, builder: DescribeIpamsInputBuilder) -> impl Future<Output = Result<DescribeIpamsOutput, SdkError<DescribeIpamsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ipv6_pools(&self, builder: DescribeIpv6PoolsInputBuilder) -> impl Future<Output = Result<DescribeIpv6PoolsOutput, SdkError<DescribeIpv6PoolsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_key_pairs(&self, builder: DescribeKeyPairsInputBuilder) -> impl Future<Output = Result<DescribeKeyPairsOutput, SdkError<DescribeKeyPairsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_launch_template_versions(&self, builder: DescribeLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplateVersionsOutput, SdkError<DescribeLaunchTemplateVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_launch_templates(&self, builder: DescribeLaunchTemplatesInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplatesOutput, SdkError<DescribeLaunchTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateway_route_table_virtual_interface_group_associations(&self, builder: DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateway_route_table_vpc_associations(&self, builder: DescribeLocalGatewayRouteTableVpcAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVpcAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVpcAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateway_route_tables(&self, builder: DescribeLocalGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTablesOutput, SdkError<DescribeLocalGatewayRouteTablesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateway_virtual_interface_groups(&self, builder: DescribeLocalGatewayVirtualInterfaceGroupsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfaceGroupsOutput, SdkError<DescribeLocalGatewayVirtualInterfaceGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateway_virtual_interfaces(&self, builder: DescribeLocalGatewayVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfacesOutput, SdkError<DescribeLocalGatewayVirtualInterfacesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_local_gateways(&self, builder: DescribeLocalGatewaysInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewaysOutput, SdkError<DescribeLocalGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_locked_snapshots(&self, builder: DescribeLockedSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeLockedSnapshotsOutput, SdkError<DescribeLockedSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_mac_hosts(&self, builder: DescribeMacHostsInputBuilder) -> impl Future<Output = Result<DescribeMacHostsOutput, SdkError<DescribeMacHostsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_managed_prefix_lists(&self, builder: DescribeManagedPrefixListsInputBuilder) -> impl Future<Output = Result<DescribeManagedPrefixListsOutput, SdkError<DescribeManagedPrefixListsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_moving_addresses(&self, builder: DescribeMovingAddressesInputBuilder) -> impl Future<Output = Result<DescribeMovingAddressesOutput, SdkError<DescribeMovingAddressesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_nat_gateways(&self, builder: DescribeNatGatewaysInputBuilder) -> impl Future<Output = Result<DescribeNatGatewaysOutput, SdkError<DescribeNatGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_acls(&self, builder: DescribeNetworkAclsInputBuilder) -> impl Future<Output = Result<DescribeNetworkAclsOutput, SdkError<DescribeNetworkAclsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_insights_access_scope_analyses(&self, builder: DescribeNetworkInsightsAccessScopeAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopeAnalysesOutput, SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_insights_access_scopes(&self, builder: DescribeNetworkInsightsAccessScopesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopesOutput, SdkError<DescribeNetworkInsightsAccessScopesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_insights_analyses(&self, builder: DescribeNetworkInsightsAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAnalysesOutput, SdkError<DescribeNetworkInsightsAnalysesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_insights_paths(&self, builder: DescribeNetworkInsightsPathsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsPathsOutput, SdkError<DescribeNetworkInsightsPathsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_interface_attribute(&self, builder: DescribeNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfaceAttributeOutput, SdkError<DescribeNetworkInterfaceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_interface_permissions(&self, builder: DescribeNetworkInterfacePermissionsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacePermissionsOutput, SdkError<DescribeNetworkInterfacePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_network_interfaces(&self, builder: DescribeNetworkInterfacesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacesOutput, SdkError<DescribeNetworkInterfacesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_placement_groups(&self, builder: DescribePlacementGroupsInputBuilder) -> impl Future<Output = Result<DescribePlacementGroupsOutput, SdkError<DescribePlacementGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_prefix_lists(&self, builder: DescribePrefixListsInputBuilder) -> impl Future<Output = Result<DescribePrefixListsOutput, SdkError<DescribePrefixListsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_principal_id_format(&self, builder: DescribePrincipalIdFormatInputBuilder) -> impl Future<Output = Result<DescribePrincipalIdFormatOutput, SdkError<DescribePrincipalIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn describe_public_ipv4_pools(&self, builder: DescribePublicIpv4PoolsInputBuilder) -> impl Future<Output = Result<DescribePublicIpv4PoolsOutput, SdkError<DescribePublicIpv4PoolsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_regions(&self, builder: DescribeRegionsInputBuilder) -> impl Future<Output = Result<DescribeRegionsOutput, SdkError<DescribeRegionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_replace_root_volume_tasks(&self, builder: DescribeReplaceRootVolumeTasksInputBuilder) -> impl Future<Output = Result<DescribeReplaceRootVolumeTasksOutput, SdkError<DescribeReplaceRootVolumeTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instances_listings(&self, builder: DescribeReservedInstancesListingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesListingsOutput, SdkError<DescribeReservedInstancesListingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instances_modifications(&self, builder: DescribeReservedInstancesModificationsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesModificationsOutput, SdkError<DescribeReservedInstancesModificationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_instances_offerings(&self, builder: DescribeReservedInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOfferingsOutput, SdkError<DescribeReservedInstancesOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_route_tables(&self, builder: DescribeRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeRouteTablesOutput, SdkError<DescribeRouteTablesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_scheduled_instance_availability(&self, builder: DescribeScheduledInstanceAvailabilityInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstanceAvailabilityOutput, SdkError<DescribeScheduledInstanceAvailabilityError>>> {
        builder.send_with(&self.0)
    }
    fn describe_scheduled_instances(&self, builder: DescribeScheduledInstancesInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstancesOutput, SdkError<DescribeScheduledInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_security_group_references(&self, builder: DescribeSecurityGroupReferencesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupReferencesOutput, SdkError<DescribeSecurityGroupReferencesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_security_group_rules(&self, builder: DescribeSecurityGroupRulesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupRulesOutput, SdkError<DescribeSecurityGroupRulesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_security_groups(&self, builder: DescribeSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupsOutput, SdkError<DescribeSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshot_attribute(&self, builder: DescribeSnapshotAttributeInputBuilder) -> impl Future<Output = Result<DescribeSnapshotAttributeOutput, SdkError<DescribeSnapshotAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshot_tier_status(&self, builder: DescribeSnapshotTierStatusInputBuilder) -> impl Future<Output = Result<DescribeSnapshotTierStatusOutput, SdkError<DescribeSnapshotTierStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_datafeed_subscription(&self, builder: DescribeSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeSpotDatafeedSubscriptionOutput, SdkError<DescribeSpotDatafeedSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_fleet_instances(&self, builder: DescribeSpotFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetInstancesOutput, SdkError<DescribeSpotFleetInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_fleet_request_history(&self, builder: DescribeSpotFleetRequestHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestHistoryOutput, SdkError<DescribeSpotFleetRequestHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_fleet_requests(&self, builder: DescribeSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestsOutput, SdkError<DescribeSpotFleetRequestsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_instance_requests(&self, builder: DescribeSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotInstanceRequestsOutput, SdkError<DescribeSpotInstanceRequestsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_spot_price_history(&self, builder: DescribeSpotPriceHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotPriceHistoryOutput, SdkError<DescribeSpotPriceHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stale_security_groups(&self, builder: DescribeStaleSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeStaleSecurityGroupsOutput, SdkError<DescribeStaleSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_store_image_tasks(&self, builder: DescribeStoreImageTasksInputBuilder) -> impl Future<Output = Result<DescribeStoreImageTasksOutput, SdkError<DescribeStoreImageTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_subnets(&self, builder: DescribeSubnetsInputBuilder) -> impl Future<Output = Result<DescribeSubnetsOutput, SdkError<DescribeSubnetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_traffic_mirror_filter_rules(&self, builder: DescribeTrafficMirrorFilterRulesInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFilterRulesOutput, SdkError<DescribeTrafficMirrorFilterRulesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_traffic_mirror_filters(&self, builder: DescribeTrafficMirrorFiltersInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFiltersOutput, SdkError<DescribeTrafficMirrorFiltersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_traffic_mirror_sessions(&self, builder: DescribeTrafficMirrorSessionsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorSessionsOutput, SdkError<DescribeTrafficMirrorSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_traffic_mirror_targets(&self, builder: DescribeTrafficMirrorTargetsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorTargetsOutput, SdkError<DescribeTrafficMirrorTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_attachments(&self, builder: DescribeTransitGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayAttachmentsOutput, SdkError<DescribeTransitGatewayAttachmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_connect_peers(&self, builder: DescribeTransitGatewayConnectPeersInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectPeersOutput, SdkError<DescribeTransitGatewayConnectPeersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_connects(&self, builder: DescribeTransitGatewayConnectsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectsOutput, SdkError<DescribeTransitGatewayConnectsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_multicast_domains(&self, builder: DescribeTransitGatewayMulticastDomainsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayMulticastDomainsOutput, SdkError<DescribeTransitGatewayMulticastDomainsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_peering_attachments(&self, builder: DescribeTransitGatewayPeeringAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPeeringAttachmentsOutput, SdkError<DescribeTransitGatewayPeeringAttachmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_policy_tables(&self, builder: DescribeTransitGatewayPolicyTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPolicyTablesOutput, SdkError<DescribeTransitGatewayPolicyTablesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_route_table_announcements(&self, builder: DescribeTransitGatewayRouteTableAnnouncementsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTableAnnouncementsOutput, SdkError<DescribeTransitGatewayRouteTableAnnouncementsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_route_tables(&self, builder: DescribeTransitGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTablesOutput, SdkError<DescribeTransitGatewayRouteTablesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateway_vpc_attachments(&self, builder: DescribeTransitGatewayVpcAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayVpcAttachmentsOutput, SdkError<DescribeTransitGatewayVpcAttachmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_transit_gateways(&self, builder: DescribeTransitGatewaysInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewaysOutput, SdkError<DescribeTransitGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn describe_trunk_interface_associations(&self, builder: DescribeTrunkInterfaceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeTrunkInterfaceAssociationsOutput, SdkError<DescribeTrunkInterfaceAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_verified_access_endpoints(&self, builder: DescribeVerifiedAccessEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessEndpointsOutput, SdkError<DescribeVerifiedAccessEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_verified_access_groups(&self, builder: DescribeVerifiedAccessGroupsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessGroupsOutput, SdkError<DescribeVerifiedAccessGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_verified_access_instance_logging_configurations(&self, builder: DescribeVerifiedAccessInstanceLoggingConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstanceLoggingConfigurationsOutput, SdkError<DescribeVerifiedAccessInstanceLoggingConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_verified_access_instances(&self, builder: DescribeVerifiedAccessInstancesInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstancesOutput, SdkError<DescribeVerifiedAccessInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_verified_access_trust_providers(&self, builder: DescribeVerifiedAccessTrustProvidersInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessTrustProvidersOutput, SdkError<DescribeVerifiedAccessTrustProvidersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_volume_attribute(&self, builder: DescribeVolumeAttributeInputBuilder) -> impl Future<Output = Result<DescribeVolumeAttributeOutput, SdkError<DescribeVolumeAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_volume_status(&self, builder: DescribeVolumeStatusInputBuilder) -> impl Future<Output = Result<DescribeVolumeStatusOutput, SdkError<DescribeVolumeStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_volumes(&self, builder: DescribeVolumesInputBuilder) -> impl Future<Output = Result<DescribeVolumesOutput, SdkError<DescribeVolumesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_volumes_modifications(&self, builder: DescribeVolumesModificationsInputBuilder) -> impl Future<Output = Result<DescribeVolumesModificationsOutput, SdkError<DescribeVolumesModificationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_attribute(&self, builder: DescribeVpcAttributeInputBuilder) -> impl Future<Output = Result<DescribeVpcAttributeOutput, SdkError<DescribeVpcAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_classic_link(&self, builder: DescribeVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkOutput, SdkError<DescribeVpcClassicLinkError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_classic_link_dns_support(&self, builder: DescribeVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkDnsSupportOutput, SdkError<DescribeVpcClassicLinkDnsSupportError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoint_connection_notifications(&self, builder: DescribeVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionNotificationsOutput, SdkError<DescribeVpcEndpointConnectionNotificationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoint_connections(&self, builder: DescribeVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionsOutput, SdkError<DescribeVpcEndpointConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoint_service_configurations(&self, builder: DescribeVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServiceConfigurationsOutput, SdkError<DescribeVpcEndpointServiceConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoint_service_permissions(&self, builder: DescribeVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicePermissionsOutput, SdkError<DescribeVpcEndpointServicePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoint_services(&self, builder: DescribeVpcEndpointServicesInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicesOutput, SdkError<DescribeVpcEndpointServicesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_peering_connections(&self, builder: DescribeVpcPeeringConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcPeeringConnectionsOutput, SdkError<DescribeVpcPeeringConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpcs(&self, builder: DescribeVpcsInputBuilder) -> impl Future<Output = Result<DescribeVpcsOutput, SdkError<DescribeVpcsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpn_connections(&self, builder: DescribeVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpnConnectionsOutput, SdkError<DescribeVpnConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpn_gateways(&self, builder: DescribeVpnGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVpnGatewaysOutput, SdkError<DescribeVpnGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn detach_classic_link_vpc(&self, builder: DetachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<DetachClassicLinkVpcOutput, SdkError<DetachClassicLinkVpcError>>> {
        builder.send_with(&self.0)
    }
    fn detach_internet_gateway(&self, builder: DetachInternetGatewayInputBuilder) -> impl Future<Output = Result<DetachInternetGatewayOutput, SdkError<DetachInternetGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn detach_network_interface(&self, builder: DetachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DetachNetworkInterfaceOutput, SdkError<DetachNetworkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn detach_verified_access_trust_provider(&self, builder: DetachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DetachVerifiedAccessTrustProviderOutput, SdkError<DetachVerifiedAccessTrustProviderError>>> {
        builder.send_with(&self.0)
    }
    fn detach_volume(&self, builder: DetachVolumeInputBuilder) -> impl Future<Output = Result<DetachVolumeOutput, SdkError<DetachVolumeError>>> {
        builder.send_with(&self.0)
    }
    fn detach_vpn_gateway(&self, builder: DetachVpnGatewayInputBuilder) -> impl Future<Output = Result<DetachVpnGatewayOutput, SdkError<DetachVpnGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn disable_address_transfer(&self, builder: DisableAddressTransferInputBuilder) -> impl Future<Output = Result<DisableAddressTransferOutput, SdkError<DisableAddressTransferError>>> {
        builder.send_with(&self.0)
    }
    fn disable_aws_network_performance_metric_subscription(&self, builder: DisableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<DisableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<DisableAwsNetworkPerformanceMetricSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn disable_ebs_encryption_by_default(&self, builder: DisableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<DisableEbsEncryptionByDefaultOutput, SdkError<DisableEbsEncryptionByDefaultError>>> {
        builder.send_with(&self.0)
    }
    fn disable_fast_launch(&self, builder: DisableFastLaunchInputBuilder) -> impl Future<Output = Result<DisableFastLaunchOutput, SdkError<DisableFastLaunchError>>> {
        builder.send_with(&self.0)
    }
    fn disable_fast_snapshot_restores(&self, builder: DisableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DisableFastSnapshotRestoresOutput, SdkError<DisableFastSnapshotRestoresError>>> {
        builder.send_with(&self.0)
    }
    fn disable_image(&self, builder: DisableImageInputBuilder) -> impl Future<Output = Result<DisableImageOutput, SdkError<DisableImageError>>> {
        builder.send_with(&self.0)
    }
    fn disable_image_block_public_access(&self, builder: DisableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableImageBlockPublicAccessOutput, SdkError<DisableImageBlockPublicAccessError>>> {
        builder.send_with(&self.0)
    }
    fn disable_image_deprecation(&self, builder: DisableImageDeprecationInputBuilder) -> impl Future<Output = Result<DisableImageDeprecationOutput, SdkError<DisableImageDeprecationError>>> {
        builder.send_with(&self.0)
    }
    fn disable_image_deregistration_protection(&self, builder: DisableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<DisableImageDeregistrationProtectionOutput, SdkError<DisableImageDeregistrationProtectionError>>> {
        builder.send_with(&self.0)
    }
    fn disable_ipam_organization_admin_account(&self, builder: DisableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableIpamOrganizationAdminAccountOutput, SdkError<DisableIpamOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disable_serial_console_access(&self, builder: DisableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<DisableSerialConsoleAccessOutput, SdkError<DisableSerialConsoleAccessError>>> {
        builder.send_with(&self.0)
    }
    fn disable_snapshot_block_public_access(&self, builder: DisableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableSnapshotBlockPublicAccessOutput, SdkError<DisableSnapshotBlockPublicAccessError>>> {
        builder.send_with(&self.0)
    }
    fn disable_transit_gateway_route_table_propagation(&self, builder: DisableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<DisableTransitGatewayRouteTablePropagationOutput, SdkError<DisableTransitGatewayRouteTablePropagationError>>> {
        builder.send_with(&self.0)
    }
    fn disable_vgw_route_propagation(&self, builder: DisableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<DisableVgwRoutePropagationOutput, SdkError<DisableVgwRoutePropagationError>>> {
        builder.send_with(&self.0)
    }
    fn disable_vpc_classic_link(&self, builder: DisableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkOutput, SdkError<DisableVpcClassicLinkError>>> {
        builder.send_with(&self.0)
    }
    fn disable_vpc_classic_link_dns_support(&self, builder: DisableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkDnsSupportOutput, SdkError<DisableVpcClassicLinkDnsSupportError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_address(&self, builder: DisassociateAddressInputBuilder) -> impl Future<Output = Result<DisassociateAddressOutput, SdkError<DisassociateAddressError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_client_vpn_target_network(&self, builder: DisassociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<DisassociateClientVpnTargetNetworkOutput, SdkError<DisassociateClientVpnTargetNetworkError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_enclave_certificate_iam_role(&self, builder: DisassociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnclaveCertificateIamRoleOutput, SdkError<DisassociateEnclaveCertificateIamRoleError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_iam_instance_profile(&self, builder: DisassociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<DisassociateIamInstanceProfileOutput, SdkError<DisassociateIamInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_instance_event_window(&self, builder: DisassociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DisassociateInstanceEventWindowOutput, SdkError<DisassociateInstanceEventWindowError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_ipam_byoasn(&self, builder: DisassociateIpamByoasnInputBuilder) -> impl Future<Output = Result<DisassociateIpamByoasnOutput, SdkError<DisassociateIpamByoasnError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_ipam_resource_discovery(&self, builder: DisassociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DisassociateIpamResourceDiscoveryOutput, SdkError<DisassociateIpamResourceDiscoveryError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_nat_gateway_address(&self, builder: DisassociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<DisassociateNatGatewayAddressOutput, SdkError<DisassociateNatGatewayAddressError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_route_table(&self, builder: DisassociateRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateRouteTableOutput, SdkError<DisassociateRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_subnet_cidr_block(&self, builder: DisassociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateSubnetCidrBlockOutput, SdkError<DisassociateSubnetCidrBlockError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_transit_gateway_multicast_domain(&self, builder: DisassociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayMulticastDomainOutput, SdkError<DisassociateTransitGatewayMulticastDomainError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_transit_gateway_policy_table(&self, builder: DisassociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayPolicyTableOutput, SdkError<DisassociateTransitGatewayPolicyTableError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_transit_gateway_route_table(&self, builder: DisassociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayRouteTableOutput, SdkError<DisassociateTransitGatewayRouteTableError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_trunk_interface(&self, builder: DisassociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<DisassociateTrunkInterfaceOutput, SdkError<DisassociateTrunkInterfaceError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_vpc_cidr_block(&self, builder: DisassociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateVpcCidrBlockOutput, SdkError<DisassociateVpcCidrBlockError>>> {
        builder.send_with(&self.0)
    }
    fn enable_address_transfer(&self, builder: EnableAddressTransferInputBuilder) -> impl Future<Output = Result<EnableAddressTransferOutput, SdkError<EnableAddressTransferError>>> {
        builder.send_with(&self.0)
    }
    fn enable_aws_network_performance_metric_subscription(&self, builder: EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<EnableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<EnableAwsNetworkPerformanceMetricSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn enable_ebs_encryption_by_default(&self, builder: EnableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<EnableEbsEncryptionByDefaultOutput, SdkError<EnableEbsEncryptionByDefaultError>>> {
        builder.send_with(&self.0)
    }
    fn enable_fast_launch(&self, builder: EnableFastLaunchInputBuilder) -> impl Future<Output = Result<EnableFastLaunchOutput, SdkError<EnableFastLaunchError>>> {
        builder.send_with(&self.0)
    }
    fn enable_fast_snapshot_restores(&self, builder: EnableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<EnableFastSnapshotRestoresOutput, SdkError<EnableFastSnapshotRestoresError>>> {
        builder.send_with(&self.0)
    }
    fn enable_image(&self, builder: EnableImageInputBuilder) -> impl Future<Output = Result<EnableImageOutput, SdkError<EnableImageError>>> {
        builder.send_with(&self.0)
    }
    fn enable_image_block_public_access(&self, builder: EnableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableImageBlockPublicAccessOutput, SdkError<EnableImageBlockPublicAccessError>>> {
        builder.send_with(&self.0)
    }
    fn enable_image_deprecation(&self, builder: EnableImageDeprecationInputBuilder) -> impl Future<Output = Result<EnableImageDeprecationOutput, SdkError<EnableImageDeprecationError>>> {
        builder.send_with(&self.0)
    }
    fn enable_image_deregistration_protection(&self, builder: EnableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<EnableImageDeregistrationProtectionOutput, SdkError<EnableImageDeregistrationProtectionError>>> {
        builder.send_with(&self.0)
    }
    fn enable_ipam_organization_admin_account(&self, builder: EnableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableIpamOrganizationAdminAccountOutput, SdkError<EnableIpamOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn enable_reachability_analyzer_organization_sharing(&self, builder: EnableReachabilityAnalyzerOrganizationSharingInputBuilder) -> impl Future<Output = Result<EnableReachabilityAnalyzerOrganizationSharingOutput, SdkError<EnableReachabilityAnalyzerOrganizationSharingError>>> {
        builder.send_with(&self.0)
    }
    fn enable_serial_console_access(&self, builder: EnableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<EnableSerialConsoleAccessOutput, SdkError<EnableSerialConsoleAccessError>>> {
        builder.send_with(&self.0)
    }
    fn enable_snapshot_block_public_access(&self, builder: EnableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableSnapshotBlockPublicAccessOutput, SdkError<EnableSnapshotBlockPublicAccessError>>> {
        builder.send_with(&self.0)
    }
    fn enable_transit_gateway_route_table_propagation(&self, builder: EnableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<EnableTransitGatewayRouteTablePropagationOutput, SdkError<EnableTransitGatewayRouteTablePropagationError>>> {
        builder.send_with(&self.0)
    }
    fn enable_vgw_route_propagation(&self, builder: EnableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<EnableVgwRoutePropagationOutput, SdkError<EnableVgwRoutePropagationError>>> {
        builder.send_with(&self.0)
    }
    fn enable_volume_io(&self, builder: EnableVolumeIoInputBuilder) -> impl Future<Output = Result<EnableVolumeIoOutput, SdkError<EnableVolumeIOError>>> {
        builder.send_with(&self.0)
    }
    fn enable_vpc_classic_link(&self, builder: EnableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkOutput, SdkError<EnableVpcClassicLinkError>>> {
        builder.send_with(&self.0)
    }
    fn enable_vpc_classic_link_dns_support(&self, builder: EnableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkDnsSupportOutput, SdkError<EnableVpcClassicLinkDnsSupportError>>> {
        builder.send_with(&self.0)
    }
    fn export_client_vpn_client_certificate_revocation_list(&self, builder: ExportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientCertificateRevocationListOutput, SdkError<ExportClientVpnClientCertificateRevocationListError>>> {
        builder.send_with(&self.0)
    }
    fn export_client_vpn_client_configuration(&self, builder: ExportClientVpnClientConfigurationInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientConfigurationOutput, SdkError<ExportClientVpnClientConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn export_image(&self, builder: ExportImageInputBuilder) -> impl Future<Output = Result<ExportImageOutput, SdkError<ExportImageError>>> {
        builder.send_with(&self.0)
    }
    fn export_transit_gateway_routes(&self, builder: ExportTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<ExportTransitGatewayRoutesOutput, SdkError<ExportTransitGatewayRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn get_associated_enclave_certificate_iam_roles(&self, builder: GetAssociatedEnclaveCertificateIamRolesInputBuilder) -> impl Future<Output = Result<GetAssociatedEnclaveCertificateIamRolesOutput, SdkError<GetAssociatedEnclaveCertificateIamRolesError>>> {
        builder.send_with(&self.0)
    }
    fn get_associated_ipv6_pool_cidrs(&self, builder: GetAssociatedIpv6PoolCidrsInputBuilder) -> impl Future<Output = Result<GetAssociatedIpv6PoolCidrsOutput, SdkError<GetAssociatedIpv6PoolCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn get_aws_network_performance_data(&self, builder: GetAwsNetworkPerformanceDataInputBuilder) -> impl Future<Output = Result<GetAwsNetworkPerformanceDataOutput, SdkError<GetAwsNetworkPerformanceDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_capacity_reservation_usage(&self, builder: GetCapacityReservationUsageInputBuilder) -> impl Future<Output = Result<GetCapacityReservationUsageOutput, SdkError<GetCapacityReservationUsageError>>> {
        builder.send_with(&self.0)
    }
    fn get_coip_pool_usage(&self, builder: GetCoipPoolUsageInputBuilder) -> impl Future<Output = Result<GetCoipPoolUsageOutput, SdkError<GetCoipPoolUsageError>>> {
        builder.send_with(&self.0)
    }
    fn get_console_output(&self, builder: GetConsoleOutputInputBuilder) -> impl Future<Output = Result<GetConsoleOutputOutput, SdkError<GetConsoleOutputError>>> {
        builder.send_with(&self.0)
    }
    fn get_console_screenshot(&self, builder: GetConsoleScreenshotInputBuilder) -> impl Future<Output = Result<GetConsoleScreenshotOutput, SdkError<GetConsoleScreenshotError>>> {
        builder.send_with(&self.0)
    }
    fn get_default_credit_specification(&self, builder: GetDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<GetDefaultCreditSpecificationOutput, SdkError<GetDefaultCreditSpecificationError>>> {
        builder.send_with(&self.0)
    }
    fn get_ebs_default_kms_key_id(&self, builder: GetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<GetEbsDefaultKmsKeyIdOutput, SdkError<GetEbsDefaultKmsKeyIdError>>> {
        builder.send_with(&self.0)
    }
    fn get_ebs_encryption_by_default(&self, builder: GetEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<GetEbsEncryptionByDefaultOutput, SdkError<GetEbsEncryptionByDefaultError>>> {
        builder.send_with(&self.0)
    }
    fn get_flow_logs_integration_template(&self, builder: GetFlowLogsIntegrationTemplateInputBuilder) -> impl Future<Output = Result<GetFlowLogsIntegrationTemplateOutput, SdkError<GetFlowLogsIntegrationTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_groups_for_capacity_reservation(&self, builder: GetGroupsForCapacityReservationInputBuilder) -> impl Future<Output = Result<GetGroupsForCapacityReservationOutput, SdkError<GetGroupsForCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn get_host_reservation_purchase_preview(&self, builder: GetHostReservationPurchasePreviewInputBuilder) -> impl Future<Output = Result<GetHostReservationPurchasePreviewOutput, SdkError<GetHostReservationPurchasePreviewError>>> {
        builder.send_with(&self.0)
    }
    fn get_image_block_public_access_state(&self, builder: GetImageBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetImageBlockPublicAccessStateOutput, SdkError<GetImageBlockPublicAccessStateError>>> {
        builder.send_with(&self.0)
    }
    fn get_instance_metadata_defaults(&self, builder: GetInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<GetInstanceMetadataDefaultsOutput, SdkError<GetInstanceMetadataDefaultsError>>> {
        builder.send_with(&self.0)
    }
    fn get_instance_tpm_ek_pub(&self, builder: GetInstanceTpmEkPubInputBuilder) -> impl Future<Output = Result<GetInstanceTpmEkPubOutput, SdkError<GetInstanceTpmEkPubError>>> {
        builder.send_with(&self.0)
    }
    fn get_instance_types_from_instance_requirements(&self, builder: GetInstanceTypesFromInstanceRequirementsInputBuilder) -> impl Future<Output = Result<GetInstanceTypesFromInstanceRequirementsOutput, SdkError<GetInstanceTypesFromInstanceRequirementsError>>> {
        builder.send_with(&self.0)
    }
    fn get_instance_uefi_data(&self, builder: GetInstanceUefiDataInputBuilder) -> impl Future<Output = Result<GetInstanceUefiDataOutput, SdkError<GetInstanceUefiDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_address_history(&self, builder: GetIpamAddressHistoryInputBuilder) -> impl Future<Output = Result<GetIpamAddressHistoryOutput, SdkError<GetIpamAddressHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_discovered_accounts(&self, builder: GetIpamDiscoveredAccountsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredAccountsOutput, SdkError<GetIpamDiscoveredAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_discovered_public_addresses(&self, builder: GetIpamDiscoveredPublicAddressesInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredPublicAddressesOutput, SdkError<GetIpamDiscoveredPublicAddressesError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_discovered_resource_cidrs(&self, builder: GetIpamDiscoveredResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredResourceCidrsOutput, SdkError<GetIpamDiscoveredResourceCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_pool_allocations(&self, builder: GetIpamPoolAllocationsInputBuilder) -> impl Future<Output = Result<GetIpamPoolAllocationsOutput, SdkError<GetIpamPoolAllocationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_pool_cidrs(&self, builder: GetIpamPoolCidrsInputBuilder) -> impl Future<Output = Result<GetIpamPoolCidrsOutput, SdkError<GetIpamPoolCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn get_ipam_resource_cidrs(&self, builder: GetIpamResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamResourceCidrsOutput, SdkError<GetIpamResourceCidrsError>>> {
        builder.send_with(&self.0)
    }
    fn get_launch_template_data(&self, builder: GetLaunchTemplateDataInputBuilder) -> impl Future<Output = Result<GetLaunchTemplateDataOutput, SdkError<GetLaunchTemplateDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_managed_prefix_list_associations(&self, builder: GetManagedPrefixListAssociationsInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListAssociationsOutput, SdkError<GetManagedPrefixListAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_managed_prefix_list_entries(&self, builder: GetManagedPrefixListEntriesInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListEntriesOutput, SdkError<GetManagedPrefixListEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn get_network_insights_access_scope_analysis_findings(&self, builder: GetNetworkInsightsAccessScopeAnalysisFindingsInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeAnalysisFindingsOutput, SdkError<GetNetworkInsightsAccessScopeAnalysisFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_network_insights_access_scope_content(&self, builder: GetNetworkInsightsAccessScopeContentInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeContentOutput, SdkError<GetNetworkInsightsAccessScopeContentError>>> {
        builder.send_with(&self.0)
    }
    fn get_password_data(&self, builder: GetPasswordDataInputBuilder) -> impl Future<Output = Result<GetPasswordDataOutput, SdkError<GetPasswordDataError>>> {
        builder.send_with(&self.0)
    }
    fn get_reserved_instances_exchange_quote(&self, builder: GetReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<GetReservedInstancesExchangeQuoteOutput, SdkError<GetReservedInstancesExchangeQuoteError>>> {
        builder.send_with(&self.0)
    }
    fn get_security_groups_for_vpc(&self, builder: GetSecurityGroupsForVpcInputBuilder) -> impl Future<Output = Result<GetSecurityGroupsForVpcOutput, SdkError<GetSecurityGroupsForVpcError>>> {
        builder.send_with(&self.0)
    }
    fn get_serial_console_access_status(&self, builder: GetSerialConsoleAccessStatusInputBuilder) -> impl Future<Output = Result<GetSerialConsoleAccessStatusOutput, SdkError<GetSerialConsoleAccessStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_snapshot_block_public_access_state(&self, builder: GetSnapshotBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetSnapshotBlockPublicAccessStateOutput, SdkError<GetSnapshotBlockPublicAccessStateError>>> {
        builder.send_with(&self.0)
    }
    fn get_spot_placement_scores(&self, builder: GetSpotPlacementScoresInputBuilder) -> impl Future<Output = Result<GetSpotPlacementScoresOutput, SdkError<GetSpotPlacementScoresError>>> {
        builder.send_with(&self.0)
    }
    fn get_subnet_cidr_reservations(&self, builder: GetSubnetCidrReservationsInputBuilder) -> impl Future<Output = Result<GetSubnetCidrReservationsOutput, SdkError<GetSubnetCidrReservationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_attachment_propagations(&self, builder: GetTransitGatewayAttachmentPropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayAttachmentPropagationsOutput, SdkError<GetTransitGatewayAttachmentPropagationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_multicast_domain_associations(&self, builder: GetTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayMulticastDomainAssociationsOutput, SdkError<GetTransitGatewayMulticastDomainAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_policy_table_associations(&self, builder: GetTransitGatewayPolicyTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableAssociationsOutput, SdkError<GetTransitGatewayPolicyTableAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_policy_table_entries(&self, builder: GetTransitGatewayPolicyTableEntriesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableEntriesOutput, SdkError<GetTransitGatewayPolicyTableEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_prefix_list_references(&self, builder: GetTransitGatewayPrefixListReferencesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPrefixListReferencesOutput, SdkError<GetTransitGatewayPrefixListReferencesError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_route_table_associations(&self, builder: GetTransitGatewayRouteTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTableAssociationsOutput, SdkError<GetTransitGatewayRouteTableAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_transit_gateway_route_table_propagations(&self, builder: GetTransitGatewayRouteTablePropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTablePropagationsOutput, SdkError<GetTransitGatewayRouteTablePropagationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_verified_access_endpoint_policy(&self, builder: GetVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessEndpointPolicyOutput, SdkError<GetVerifiedAccessEndpointPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_verified_access_group_policy(&self, builder: GetVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessGroupPolicyOutput, SdkError<GetVerifiedAccessGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpn_connection_device_sample_configuration(&self, builder: GetVpnConnectionDeviceSampleConfigurationInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceSampleConfigurationOutput, SdkError<GetVpnConnectionDeviceSampleConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpn_connection_device_types(&self, builder: GetVpnConnectionDeviceTypesInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceTypesOutput, SdkError<GetVpnConnectionDeviceTypesError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpn_tunnel_replacement_status(&self, builder: GetVpnTunnelReplacementStatusInputBuilder) -> impl Future<Output = Result<GetVpnTunnelReplacementStatusOutput, SdkError<GetVpnTunnelReplacementStatusError>>> {
        builder.send_with(&self.0)
    }
    fn import_client_vpn_client_certificate_revocation_list(&self, builder: ImportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ImportClientVpnClientCertificateRevocationListOutput, SdkError<ImportClientVpnClientCertificateRevocationListError>>> {
        builder.send_with(&self.0)
    }
    fn import_image(&self, builder: ImportImageInputBuilder) -> impl Future<Output = Result<ImportImageOutput, SdkError<ImportImageError>>> {
        builder.send_with(&self.0)
    }
    fn import_key_pair(&self, builder: ImportKeyPairInputBuilder) -> impl Future<Output = Result<ImportKeyPairOutput, SdkError<ImportKeyPairError>>> {
        builder.send_with(&self.0)
    }
    fn import_snapshot(&self, builder: ImportSnapshotInputBuilder) -> impl Future<Output = Result<ImportSnapshotOutput, SdkError<ImportSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn list_images_in_recycle_bin(&self, builder: ListImagesInRecycleBinInputBuilder) -> impl Future<Output = Result<ListImagesInRecycleBinOutput, SdkError<ListImagesInRecycleBinError>>> {
        builder.send_with(&self.0)
    }
    fn list_snapshots_in_recycle_bin(&self, builder: ListSnapshotsInRecycleBinInputBuilder) -> impl Future<Output = Result<ListSnapshotsInRecycleBinOutput, SdkError<ListSnapshotsInRecycleBinError>>> {
        builder.send_with(&self.0)
    }
    fn lock_snapshot(&self, builder: LockSnapshotInputBuilder) -> impl Future<Output = Result<LockSnapshotOutput, SdkError<LockSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn modify_address_attribute(&self, builder: ModifyAddressAttributeInputBuilder) -> impl Future<Output = Result<ModifyAddressAttributeOutput, SdkError<ModifyAddressAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_availability_zone_group(&self, builder: ModifyAvailabilityZoneGroupInputBuilder) -> impl Future<Output = Result<ModifyAvailabilityZoneGroupOutput, SdkError<ModifyAvailabilityZoneGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_capacity_reservation(&self, builder: ModifyCapacityReservationInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationOutput, SdkError<ModifyCapacityReservationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_capacity_reservation_fleet(&self, builder: ModifyCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationFleetOutput, SdkError<ModifyCapacityReservationFleetError>>> {
        builder.send_with(&self.0)
    }
    fn modify_client_vpn_endpoint(&self, builder: ModifyClientVpnEndpointInputBuilder) -> impl Future<Output = Result<ModifyClientVpnEndpointOutput, SdkError<ModifyClientVpnEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn modify_default_credit_specification(&self, builder: ModifyDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyDefaultCreditSpecificationOutput, SdkError<ModifyDefaultCreditSpecificationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ebs_default_kms_key_id(&self, builder: ModifyEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ModifyEbsDefaultKmsKeyIdOutput, SdkError<ModifyEbsDefaultKmsKeyIdError>>> {
        builder.send_with(&self.0)
    }
    fn modify_fleet(&self, builder: ModifyFleetInputBuilder) -> impl Future<Output = Result<ModifyFleetOutput, SdkError<ModifyFleetError>>> {
        builder.send_with(&self.0)
    }
    fn modify_fpga_image_attribute(&self, builder: ModifyFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyFpgaImageAttributeOutput, SdkError<ModifyFpgaImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_hosts(&self, builder: ModifyHostsInputBuilder) -> impl Future<Output = Result<ModifyHostsOutput, SdkError<ModifyHostsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_id_format(&self, builder: ModifyIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdFormatOutput, SdkError<ModifyIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn modify_identity_id_format(&self, builder: ModifyIdentityIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdentityIdFormatOutput, SdkError<ModifyIdentityIdFormatError>>> {
        builder.send_with(&self.0)
    }
    fn modify_image_attribute(&self, builder: ModifyImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyImageAttributeOutput, SdkError<ModifyImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_attribute(&self, builder: ModifyInstanceAttributeInputBuilder) -> impl Future<Output = Result<ModifyInstanceAttributeOutput, SdkError<ModifyInstanceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_capacity_reservation_attributes(&self, builder: ModifyInstanceCapacityReservationAttributesInputBuilder) -> impl Future<Output = Result<ModifyInstanceCapacityReservationAttributesOutput, SdkError<ModifyInstanceCapacityReservationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_credit_specification(&self, builder: ModifyInstanceCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyInstanceCreditSpecificationOutput, SdkError<ModifyInstanceCreditSpecificationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_event_start_time(&self, builder: ModifyInstanceEventStartTimeInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventStartTimeOutput, SdkError<ModifyInstanceEventStartTimeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_event_window(&self, builder: ModifyInstanceEventWindowInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventWindowOutput, SdkError<ModifyInstanceEventWindowError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_maintenance_options(&self, builder: ModifyInstanceMaintenanceOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMaintenanceOptionsOutput, SdkError<ModifyInstanceMaintenanceOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_metadata_defaults(&self, builder: ModifyInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataDefaultsOutput, SdkError<ModifyInstanceMetadataDefaultsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_metadata_options(&self, builder: ModifyInstanceMetadataOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataOptionsOutput, SdkError<ModifyInstanceMetadataOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_placement(&self, builder: ModifyInstancePlacementInputBuilder) -> impl Future<Output = Result<ModifyInstancePlacementOutput, SdkError<ModifyInstancePlacementError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ipam(&self, builder: ModifyIpamInputBuilder) -> impl Future<Output = Result<ModifyIpamOutput, SdkError<ModifyIpamError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ipam_pool(&self, builder: ModifyIpamPoolInputBuilder) -> impl Future<Output = Result<ModifyIpamPoolOutput, SdkError<ModifyIpamPoolError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ipam_resource_cidr(&self, builder: ModifyIpamResourceCidrInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceCidrOutput, SdkError<ModifyIpamResourceCidrError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ipam_resource_discovery(&self, builder: ModifyIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceDiscoveryOutput, SdkError<ModifyIpamResourceDiscoveryError>>> {
        builder.send_with(&self.0)
    }
    fn modify_ipam_scope(&self, builder: ModifyIpamScopeInputBuilder) -> impl Future<Output = Result<ModifyIpamScopeOutput, SdkError<ModifyIpamScopeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_launch_template(&self, builder: ModifyLaunchTemplateInputBuilder) -> impl Future<Output = Result<ModifyLaunchTemplateOutput, SdkError<ModifyLaunchTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn modify_local_gateway_route(&self, builder: ModifyLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<ModifyLocalGatewayRouteOutput, SdkError<ModifyLocalGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn modify_managed_prefix_list(&self, builder: ModifyManagedPrefixListInputBuilder) -> impl Future<Output = Result<ModifyManagedPrefixListOutput, SdkError<ModifyManagedPrefixListError>>> {
        builder.send_with(&self.0)
    }
    fn modify_network_interface_attribute(&self, builder: ModifyNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ModifyNetworkInterfaceAttributeOutput, SdkError<ModifyNetworkInterfaceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_private_dns_name_options(&self, builder: ModifyPrivateDnsNameOptionsInputBuilder) -> impl Future<Output = Result<ModifyPrivateDnsNameOptionsOutput, SdkError<ModifyPrivateDnsNameOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_reserved_instances(&self, builder: ModifyReservedInstancesInputBuilder) -> impl Future<Output = Result<ModifyReservedInstancesOutput, SdkError<ModifyReservedInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_security_group_rules(&self, builder: ModifySecurityGroupRulesInputBuilder) -> impl Future<Output = Result<ModifySecurityGroupRulesOutput, SdkError<ModifySecurityGroupRulesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_snapshot_attribute(&self, builder: ModifySnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifySnapshotAttributeOutput, SdkError<ModifySnapshotAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_snapshot_tier(&self, builder: ModifySnapshotTierInputBuilder) -> impl Future<Output = Result<ModifySnapshotTierOutput, SdkError<ModifySnapshotTierError>>> {
        builder.send_with(&self.0)
    }
    fn modify_spot_fleet_request(&self, builder: ModifySpotFleetRequestInputBuilder) -> impl Future<Output = Result<ModifySpotFleetRequestOutput, SdkError<ModifySpotFleetRequestError>>> {
        builder.send_with(&self.0)
    }
    fn modify_subnet_attribute(&self, builder: ModifySubnetAttributeInputBuilder) -> impl Future<Output = Result<ModifySubnetAttributeOutput, SdkError<ModifySubnetAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_traffic_mirror_filter_network_services(&self, builder: ModifyTrafficMirrorFilterNetworkServicesInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterNetworkServicesOutput, SdkError<ModifyTrafficMirrorFilterNetworkServicesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_traffic_mirror_filter_rule(&self, builder: ModifyTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterRuleOutput, SdkError<ModifyTrafficMirrorFilterRuleError>>> {
        builder.send_with(&self.0)
    }
    fn modify_traffic_mirror_session(&self, builder: ModifyTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorSessionOutput, SdkError<ModifyTrafficMirrorSessionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_transit_gateway(&self, builder: ModifyTransitGatewayInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayOutput, SdkError<ModifyTransitGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn modify_transit_gateway_prefix_list_reference(&self, builder: ModifyTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayPrefixListReferenceOutput, SdkError<ModifyTransitGatewayPrefixListReferenceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_transit_gateway_vpc_attachment(&self, builder: ModifyTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayVpcAttachmentOutput, SdkError<ModifyTransitGatewayVpcAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_endpoint(&self, builder: ModifyVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointOutput, SdkError<ModifyVerifiedAccessEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_endpoint_policy(&self, builder: ModifyVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointPolicyOutput, SdkError<ModifyVerifiedAccessEndpointPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_group(&self, builder: ModifyVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupOutput, SdkError<ModifyVerifiedAccessGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_group_policy(&self, builder: ModifyVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupPolicyOutput, SdkError<ModifyVerifiedAccessGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_instance(&self, builder: ModifyVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceOutput, SdkError<ModifyVerifiedAccessInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_instance_logging_configuration(&self, builder: ModifyVerifiedAccessInstanceLoggingConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceLoggingConfigurationOutput, SdkError<ModifyVerifiedAccessInstanceLoggingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_verified_access_trust_provider(&self, builder: ModifyVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessTrustProviderOutput, SdkError<ModifyVerifiedAccessTrustProviderError>>> {
        builder.send_with(&self.0)
    }
    fn modify_volume(&self, builder: ModifyVolumeInputBuilder) -> impl Future<Output = Result<ModifyVolumeOutput, SdkError<ModifyVolumeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_volume_attribute(&self, builder: ModifyVolumeAttributeInputBuilder) -> impl Future<Output = Result<ModifyVolumeAttributeOutput, SdkError<ModifyVolumeAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_attribute(&self, builder: ModifyVpcAttributeInputBuilder) -> impl Future<Output = Result<ModifyVpcAttributeOutput, SdkError<ModifyVpcAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_endpoint(&self, builder: ModifyVpcEndpointInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointOutput, SdkError<ModifyVpcEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_endpoint_connection_notification(&self, builder: ModifyVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointConnectionNotificationOutput, SdkError<ModifyVpcEndpointConnectionNotificationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_endpoint_service_configuration(&self, builder: ModifyVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServiceConfigurationOutput, SdkError<ModifyVpcEndpointServiceConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_endpoint_service_payer_responsibility(&self, builder: ModifyVpcEndpointServicePayerResponsibilityInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePayerResponsibilityOutput, SdkError<ModifyVpcEndpointServicePayerResponsibilityError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_endpoint_service_permissions(&self, builder: ModifyVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePermissionsOutput, SdkError<ModifyVpcEndpointServicePermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_peering_connection_options(&self, builder: ModifyVpcPeeringConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpcPeeringConnectionOptionsOutput, SdkError<ModifyVpcPeeringConnectionOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpc_tenancy(&self, builder: ModifyVpcTenancyInputBuilder) -> impl Future<Output = Result<ModifyVpcTenancyOutput, SdkError<ModifyVpcTenancyError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpn_connection(&self, builder: ModifyVpnConnectionInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOutput, SdkError<ModifyVpnConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpn_connection_options(&self, builder: ModifyVpnConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOptionsOutput, SdkError<ModifyVpnConnectionOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpn_tunnel_certificate(&self, builder: ModifyVpnTunnelCertificateInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelCertificateOutput, SdkError<ModifyVpnTunnelCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn modify_vpn_tunnel_options(&self, builder: ModifyVpnTunnelOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelOptionsOutput, SdkError<ModifyVpnTunnelOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn monitor_instances(&self, builder: MonitorInstancesInputBuilder) -> impl Future<Output = Result<MonitorInstancesOutput, SdkError<MonitorInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn move_address_to_vpc(&self, builder: MoveAddressToVpcInputBuilder) -> impl Future<Output = Result<MoveAddressToVpcOutput, SdkError<MoveAddressToVpcError>>> {
        builder.send_with(&self.0)
    }
    fn move_byoip_cidr_to_ipam(&self, builder: MoveByoipCidrToIpamInputBuilder) -> impl Future<Output = Result<MoveByoipCidrToIpamOutput, SdkError<MoveByoipCidrToIpamError>>> {
        builder.send_with(&self.0)
    }
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>> {
        builder.send_with(&self.0)
    }
    fn provision_ipam_byoasn(&self, builder: ProvisionIpamByoasnInputBuilder) -> impl Future<Output = Result<ProvisionIpamByoasnOutput, SdkError<ProvisionIpamByoasnError>>> {
        builder.send_with(&self.0)
    }
    fn provision_ipam_pool_cidr(&self, builder: ProvisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionIpamPoolCidrOutput, SdkError<ProvisionIpamPoolCidrError>>> {
        builder.send_with(&self.0)
    }
    fn provision_public_ipv4_pool_cidr(&self, builder: ProvisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionPublicIpv4PoolCidrOutput, SdkError<ProvisionPublicIpv4PoolCidrError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_capacity_block(&self, builder: PurchaseCapacityBlockInputBuilder) -> impl Future<Output = Result<PurchaseCapacityBlockOutput, SdkError<PurchaseCapacityBlockError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_host_reservation(&self, builder: PurchaseHostReservationInputBuilder) -> impl Future<Output = Result<PurchaseHostReservationOutput, SdkError<PurchaseHostReservationError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_reserved_instances_offering(&self, builder: PurchaseReservedInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstancesOfferingOutput, SdkError<PurchaseReservedInstancesOfferingError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_scheduled_instances(&self, builder: PurchaseScheduledInstancesInputBuilder) -> impl Future<Output = Result<PurchaseScheduledInstancesOutput, SdkError<PurchaseScheduledInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_instances(&self, builder: RebootInstancesInputBuilder) -> impl Future<Output = Result<RebootInstancesOutput, SdkError<RebootInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn register_image(&self, builder: RegisterImageInputBuilder) -> impl Future<Output = Result<RegisterImageOutput, SdkError<RegisterImageError>>> {
        builder.send_with(&self.0)
    }
    fn register_instance_event_notification_attributes(&self, builder: RegisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<RegisterInstanceEventNotificationAttributesOutput, SdkError<RegisterInstanceEventNotificationAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn register_transit_gateway_multicast_group_members(&self, builder: RegisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupMembersOutput, SdkError<RegisterTransitGatewayMulticastGroupMembersError>>> {
        builder.send_with(&self.0)
    }
    fn register_transit_gateway_multicast_group_sources(&self, builder: RegisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupSourcesOutput, SdkError<RegisterTransitGatewayMulticastGroupSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn reject_transit_gateway_multicast_domain_associations(&self, builder: RejectTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayMulticastDomainAssociationsOutput, SdkError<RejectTransitGatewayMulticastDomainAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn reject_transit_gateway_peering_attachment(&self, builder: RejectTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayPeeringAttachmentOutput, SdkError<RejectTransitGatewayPeeringAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn reject_transit_gateway_vpc_attachment(&self, builder: RejectTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayVpcAttachmentOutput, SdkError<RejectTransitGatewayVpcAttachmentError>>> {
        builder.send_with(&self.0)
    }
    fn reject_vpc_endpoint_connections(&self, builder: RejectVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<RejectVpcEndpointConnectionsOutput, SdkError<RejectVpcEndpointConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn reject_vpc_peering_connection(&self, builder: RejectVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<RejectVpcPeeringConnectionOutput, SdkError<RejectVpcPeeringConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn release_address(&self, builder: ReleaseAddressInputBuilder) -> impl Future<Output = Result<ReleaseAddressOutput, SdkError<ReleaseAddressError>>> {
        builder.send_with(&self.0)
    }
    fn release_hosts(&self, builder: ReleaseHostsInputBuilder) -> impl Future<Output = Result<ReleaseHostsOutput, SdkError<ReleaseHostsError>>> {
        builder.send_with(&self.0)
    }
    fn release_ipam_pool_allocation(&self, builder: ReleaseIpamPoolAllocationInputBuilder) -> impl Future<Output = Result<ReleaseIpamPoolAllocationOutput, SdkError<ReleaseIpamPoolAllocationError>>> {
        builder.send_with(&self.0)
    }
    fn replace_iam_instance_profile_association(&self, builder: ReplaceIamInstanceProfileAssociationInputBuilder) -> impl Future<Output = Result<ReplaceIamInstanceProfileAssociationOutput, SdkError<ReplaceIamInstanceProfileAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn replace_network_acl_association(&self, builder: ReplaceNetworkAclAssociationInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclAssociationOutput, SdkError<ReplaceNetworkAclAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn replace_network_acl_entry(&self, builder: ReplaceNetworkAclEntryInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclEntryOutput, SdkError<ReplaceNetworkAclEntryError>>> {
        builder.send_with(&self.0)
    }
    fn replace_route(&self, builder: ReplaceRouteInputBuilder) -> impl Future<Output = Result<ReplaceRouteOutput, SdkError<ReplaceRouteError>>> {
        builder.send_with(&self.0)
    }
    fn replace_route_table_association(&self, builder: ReplaceRouteTableAssociationInputBuilder) -> impl Future<Output = Result<ReplaceRouteTableAssociationOutput, SdkError<ReplaceRouteTableAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn replace_transit_gateway_route(&self, builder: ReplaceTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<ReplaceTransitGatewayRouteOutput, SdkError<ReplaceTransitGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn replace_vpn_tunnel(&self, builder: ReplaceVpnTunnelInputBuilder) -> impl Future<Output = Result<ReplaceVpnTunnelOutput, SdkError<ReplaceVpnTunnelError>>> {
        builder.send_with(&self.0)
    }
    fn report_instance_status(&self, builder: ReportInstanceStatusInputBuilder) -> impl Future<Output = Result<ReportInstanceStatusOutput, SdkError<ReportInstanceStatusError>>> {
        builder.send_with(&self.0)
    }
    fn request_spot_fleet(&self, builder: RequestSpotFleetInputBuilder) -> impl Future<Output = Result<RequestSpotFleetOutput, SdkError<RequestSpotFleetError>>> {
        builder.send_with(&self.0)
    }
    fn request_spot_instances(&self, builder: RequestSpotInstancesInputBuilder) -> impl Future<Output = Result<RequestSpotInstancesOutput, SdkError<RequestSpotInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn reset_address_attribute(&self, builder: ResetAddressAttributeInputBuilder) -> impl Future<Output = Result<ResetAddressAttributeOutput, SdkError<ResetAddressAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn reset_ebs_default_kms_key_id(&self, builder: ResetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ResetEbsDefaultKmsKeyIdOutput, SdkError<ResetEbsDefaultKmsKeyIdError>>> {
        builder.send_with(&self.0)
    }
    fn reset_fpga_image_attribute(&self, builder: ResetFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ResetFpgaImageAttributeOutput, SdkError<ResetFpgaImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn reset_image_attribute(&self, builder: ResetImageAttributeInputBuilder) -> impl Future<Output = Result<ResetImageAttributeOutput, SdkError<ResetImageAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn reset_instance_attribute(&self, builder: ResetInstanceAttributeInputBuilder) -> impl Future<Output = Result<ResetInstanceAttributeOutput, SdkError<ResetInstanceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn reset_network_interface_attribute(&self, builder: ResetNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ResetNetworkInterfaceAttributeOutput, SdkError<ResetNetworkInterfaceAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn reset_snapshot_attribute(&self, builder: ResetSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ResetSnapshotAttributeOutput, SdkError<ResetSnapshotAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn restore_address_to_classic(&self, builder: RestoreAddressToClassicInputBuilder) -> impl Future<Output = Result<RestoreAddressToClassicOutput, SdkError<RestoreAddressToClassicError>>> {
        builder.send_with(&self.0)
    }
    fn restore_image_from_recycle_bin(&self, builder: RestoreImageFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreImageFromRecycleBinOutput, SdkError<RestoreImageFromRecycleBinError>>> {
        builder.send_with(&self.0)
    }
    fn restore_managed_prefix_list_version(&self, builder: RestoreManagedPrefixListVersionInputBuilder) -> impl Future<Output = Result<RestoreManagedPrefixListVersionOutput, SdkError<RestoreManagedPrefixListVersionError>>> {
        builder.send_with(&self.0)
    }
    fn restore_snapshot_from_recycle_bin(&self, builder: RestoreSnapshotFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreSnapshotFromRecycleBinOutput, SdkError<RestoreSnapshotFromRecycleBinError>>> {
        builder.send_with(&self.0)
    }
    fn restore_snapshot_tier(&self, builder: RestoreSnapshotTierInputBuilder) -> impl Future<Output = Result<RestoreSnapshotTierOutput, SdkError<RestoreSnapshotTierError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_client_vpn_ingress(&self, builder: RevokeClientVpnIngressInputBuilder) -> impl Future<Output = Result<RevokeClientVpnIngressOutput, SdkError<RevokeClientVpnIngressError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_security_group_egress(&self, builder: RevokeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupEgressOutput, SdkError<RevokeSecurityGroupEgressError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_security_group_ingress(&self, builder: RevokeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupIngressOutput, SdkError<RevokeSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn run_instances(&self, builder: RunInstancesInputBuilder) -> impl Future<Output = Result<RunInstancesOutput, SdkError<RunInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn run_scheduled_instances(&self, builder: RunScheduledInstancesInputBuilder) -> impl Future<Output = Result<RunScheduledInstancesOutput, SdkError<RunScheduledInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn search_local_gateway_routes(&self, builder: SearchLocalGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchLocalGatewayRoutesOutput, SdkError<SearchLocalGatewayRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn search_transit_gateway_multicast_groups(&self, builder: SearchTransitGatewayMulticastGroupsInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayMulticastGroupsOutput, SdkError<SearchTransitGatewayMulticastGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn search_transit_gateway_routes(&self, builder: SearchTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayRoutesOutput, SdkError<SearchTransitGatewayRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn send_diagnostic_interrupt(&self, builder: SendDiagnosticInterruptInputBuilder) -> impl Future<Output = Result<SendDiagnosticInterruptOutput, SdkError<SendDiagnosticInterruptError>>> {
        builder.send_with(&self.0)
    }
    fn start_instances(&self, builder: StartInstancesInputBuilder) -> impl Future<Output = Result<StartInstancesOutput, SdkError<StartInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn start_network_insights_access_scope_analysis(&self, builder: StartNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAccessScopeAnalysisOutput, SdkError<StartNetworkInsightsAccessScopeAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn start_network_insights_analysis(&self, builder: StartNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAnalysisOutput, SdkError<StartNetworkInsightsAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn start_vpc_endpoint_service_private_dns_verification(&self, builder: StartVpcEndpointServicePrivateDnsVerificationInputBuilder) -> impl Future<Output = Result<StartVpcEndpointServicePrivateDnsVerificationOutput, SdkError<StartVpcEndpointServicePrivateDnsVerificationError>>> {
        builder.send_with(&self.0)
    }
    fn stop_instances(&self, builder: StopInstancesInputBuilder) -> impl Future<Output = Result<StopInstancesOutput, SdkError<StopInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_client_vpn_connections(&self, builder: TerminateClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<TerminateClientVpnConnectionsOutput, SdkError<TerminateClientVpnConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_instances(&self, builder: TerminateInstancesInputBuilder) -> impl Future<Output = Result<TerminateInstancesOutput, SdkError<TerminateInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn unassign_ipv6_addresses(&self, builder: UnassignIpv6AddressesInputBuilder) -> impl Future<Output = Result<UnassignIpv6AddressesOutput, SdkError<UnassignIpv6AddressesError>>> {
        builder.send_with(&self.0)
    }
    fn unassign_private_ip_addresses(&self, builder: UnassignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<UnassignPrivateIpAddressesOutput, SdkError<UnassignPrivateIpAddressesError>>> {
        builder.send_with(&self.0)
    }
    fn unassign_private_nat_gateway_address(&self, builder: UnassignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<UnassignPrivateNatGatewayAddressOutput, SdkError<UnassignPrivateNatGatewayAddressError>>> {
        builder.send_with(&self.0)
    }
    fn unlock_snapshot(&self, builder: UnlockSnapshotInputBuilder) -> impl Future<Output = Result<UnlockSnapshotOutput, SdkError<UnlockSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn unmonitor_instances(&self, builder: UnmonitorInstancesInputBuilder) -> impl Future<Output = Result<UnmonitorInstancesOutput, SdkError<UnmonitorInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn update_security_group_rule_descriptions_egress(&self, builder: UpdateSecurityGroupRuleDescriptionsEgressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsEgressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsEgressError>>> {
        builder.send_with(&self.0)
    }
    fn update_security_group_rule_descriptions_ingress(&self, builder: UpdateSecurityGroupRuleDescriptionsIngressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsIngressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsIngressError>>> {
        builder.send_with(&self.0)
    }
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: EC2Client> EC2Client for &T {
    fn accept_address_transfer(&self, builder: AcceptAddressTransferInputBuilder) -> impl Future<Output = Result<AcceptAddressTransferOutput, SdkError<AcceptAddressTransferError>>> {
        (*self).accept_address_transfer(builder)
    }
    fn accept_reserved_instances_exchange_quote(&self, builder: AcceptReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<AcceptReservedInstancesExchangeQuoteOutput, SdkError<AcceptReservedInstancesExchangeQuoteError>>> {
        (*self).accept_reserved_instances_exchange_quote(builder)
    }
    fn accept_transit_gateway_multicast_domain_associations(&self, builder: AcceptTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayMulticastDomainAssociationsOutput, SdkError<AcceptTransitGatewayMulticastDomainAssociationsError>>> {
        (*self).accept_transit_gateway_multicast_domain_associations(builder)
    }
    fn accept_transit_gateway_peering_attachment(&self, builder: AcceptTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayPeeringAttachmentOutput, SdkError<AcceptTransitGatewayPeeringAttachmentError>>> {
        (*self).accept_transit_gateway_peering_attachment(builder)
    }
    fn accept_transit_gateway_vpc_attachment(&self, builder: AcceptTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<AcceptTransitGatewayVpcAttachmentOutput, SdkError<AcceptTransitGatewayVpcAttachmentError>>> {
        (*self).accept_transit_gateway_vpc_attachment(builder)
    }
    fn accept_vpc_endpoint_connections(&self, builder: AcceptVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<AcceptVpcEndpointConnectionsOutput, SdkError<AcceptVpcEndpointConnectionsError>>> {
        (*self).accept_vpc_endpoint_connections(builder)
    }
    fn accept_vpc_peering_connection(&self, builder: AcceptVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<AcceptVpcPeeringConnectionOutput, SdkError<AcceptVpcPeeringConnectionError>>> {
        (*self).accept_vpc_peering_connection(builder)
    }
    fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> impl Future<Output = Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>> {
        (*self).advertise_byoip_cidr(builder)
    }
    fn allocate_address(&self, builder: AllocateAddressInputBuilder) -> impl Future<Output = Result<AllocateAddressOutput, SdkError<AllocateAddressError>>> {
        (*self).allocate_address(builder)
    }
    fn allocate_hosts(&self, builder: AllocateHostsInputBuilder) -> impl Future<Output = Result<AllocateHostsOutput, SdkError<AllocateHostsError>>> {
        (*self).allocate_hosts(builder)
    }
    fn allocate_ipam_pool_cidr(&self, builder: AllocateIpamPoolCidrInputBuilder) -> impl Future<Output = Result<AllocateIpamPoolCidrOutput, SdkError<AllocateIpamPoolCidrError>>> {
        (*self).allocate_ipam_pool_cidr(builder)
    }
    fn apply_security_groups_to_client_vpn_target_network(&self, builder: ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<ApplySecurityGroupsToClientVpnTargetNetworkOutput, SdkError<ApplySecurityGroupsToClientVpnTargetNetworkError>>> {
        (*self).apply_security_groups_to_client_vpn_target_network(builder)
    }
    fn assign_ipv6_addresses(&self, builder: AssignIpv6AddressesInputBuilder) -> impl Future<Output = Result<AssignIpv6AddressesOutput, SdkError<AssignIpv6AddressesError>>> {
        (*self).assign_ipv6_addresses(builder)
    }
    fn assign_private_ip_addresses(&self, builder: AssignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<AssignPrivateIpAddressesOutput, SdkError<AssignPrivateIpAddressesError>>> {
        (*self).assign_private_ip_addresses(builder)
    }
    fn assign_private_nat_gateway_address(&self, builder: AssignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssignPrivateNatGatewayAddressOutput, SdkError<AssignPrivateNatGatewayAddressError>>> {
        (*self).assign_private_nat_gateway_address(builder)
    }
    fn associate_address(&self, builder: AssociateAddressInputBuilder) -> impl Future<Output = Result<AssociateAddressOutput, SdkError<AssociateAddressError>>> {
        (*self).associate_address(builder)
    }
    fn associate_client_vpn_target_network(&self, builder: AssociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<AssociateClientVpnTargetNetworkOutput, SdkError<AssociateClientVpnTargetNetworkError>>> {
        (*self).associate_client_vpn_target_network(builder)
    }
    fn associate_dhcp_options(&self, builder: AssociateDhcpOptionsInputBuilder) -> impl Future<Output = Result<AssociateDhcpOptionsOutput, SdkError<AssociateDhcpOptionsError>>> {
        (*self).associate_dhcp_options(builder)
    }
    fn associate_enclave_certificate_iam_role(&self, builder: AssociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<AssociateEnclaveCertificateIamRoleOutput, SdkError<AssociateEnclaveCertificateIamRoleError>>> {
        (*self).associate_enclave_certificate_iam_role(builder)
    }
    fn associate_iam_instance_profile(&self, builder: AssociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<AssociateIamInstanceProfileOutput, SdkError<AssociateIamInstanceProfileError>>> {
        (*self).associate_iam_instance_profile(builder)
    }
    fn associate_instance_event_window(&self, builder: AssociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<AssociateInstanceEventWindowOutput, SdkError<AssociateInstanceEventWindowError>>> {
        (*self).associate_instance_event_window(builder)
    }
    fn associate_ipam_byoasn(&self, builder: AssociateIpamByoasnInputBuilder) -> impl Future<Output = Result<AssociateIpamByoasnOutput, SdkError<AssociateIpamByoasnError>>> {
        (*self).associate_ipam_byoasn(builder)
    }
    fn associate_ipam_resource_discovery(&self, builder: AssociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<AssociateIpamResourceDiscoveryOutput, SdkError<AssociateIpamResourceDiscoveryError>>> {
        (*self).associate_ipam_resource_discovery(builder)
    }
    fn associate_nat_gateway_address(&self, builder: AssociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<AssociateNatGatewayAddressOutput, SdkError<AssociateNatGatewayAddressError>>> {
        (*self).associate_nat_gateway_address(builder)
    }
    fn associate_route_table(&self, builder: AssociateRouteTableInputBuilder) -> impl Future<Output = Result<AssociateRouteTableOutput, SdkError<AssociateRouteTableError>>> {
        (*self).associate_route_table(builder)
    }
    fn associate_subnet_cidr_block(&self, builder: AssociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateSubnetCidrBlockOutput, SdkError<AssociateSubnetCidrBlockError>>> {
        (*self).associate_subnet_cidr_block(builder)
    }
    fn associate_transit_gateway_multicast_domain(&self, builder: AssociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayMulticastDomainOutput, SdkError<AssociateTransitGatewayMulticastDomainError>>> {
        (*self).associate_transit_gateway_multicast_domain(builder)
    }
    fn associate_transit_gateway_policy_table(&self, builder: AssociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayPolicyTableOutput, SdkError<AssociateTransitGatewayPolicyTableError>>> {
        (*self).associate_transit_gateway_policy_table(builder)
    }
    fn associate_transit_gateway_route_table(&self, builder: AssociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<AssociateTransitGatewayRouteTableOutput, SdkError<AssociateTransitGatewayRouteTableError>>> {
        (*self).associate_transit_gateway_route_table(builder)
    }
    fn associate_trunk_interface(&self, builder: AssociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<AssociateTrunkInterfaceOutput, SdkError<AssociateTrunkInterfaceError>>> {
        (*self).associate_trunk_interface(builder)
    }
    fn associate_vpc_cidr_block(&self, builder: AssociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<AssociateVpcCidrBlockOutput, SdkError<AssociateVpcCidrBlockError>>> {
        (*self).associate_vpc_cidr_block(builder)
    }
    fn attach_classic_link_vpc(&self, builder: AttachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<AttachClassicLinkVpcOutput, SdkError<AttachClassicLinkVpcError>>> {
        (*self).attach_classic_link_vpc(builder)
    }
    fn attach_internet_gateway(&self, builder: AttachInternetGatewayInputBuilder) -> impl Future<Output = Result<AttachInternetGatewayOutput, SdkError<AttachInternetGatewayError>>> {
        (*self).attach_internet_gateway(builder)
    }
    fn attach_network_interface(&self, builder: AttachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<AttachNetworkInterfaceOutput, SdkError<AttachNetworkInterfaceError>>> {
        (*self).attach_network_interface(builder)
    }
    fn attach_verified_access_trust_provider(&self, builder: AttachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<AttachVerifiedAccessTrustProviderOutput, SdkError<AttachVerifiedAccessTrustProviderError>>> {
        (*self).attach_verified_access_trust_provider(builder)
    }
    fn attach_volume(&self, builder: AttachVolumeInputBuilder) -> impl Future<Output = Result<AttachVolumeOutput, SdkError<AttachVolumeError>>> {
        (*self).attach_volume(builder)
    }
    fn attach_vpn_gateway(&self, builder: AttachVpnGatewayInputBuilder) -> impl Future<Output = Result<AttachVpnGatewayOutput, SdkError<AttachVpnGatewayError>>> {
        (*self).attach_vpn_gateway(builder)
    }
    fn authorize_client_vpn_ingress(&self, builder: AuthorizeClientVpnIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClientVpnIngressOutput, SdkError<AuthorizeClientVpnIngressError>>> {
        (*self).authorize_client_vpn_ingress(builder)
    }
    fn authorize_security_group_egress(&self, builder: AuthorizeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupEgressOutput, SdkError<AuthorizeSecurityGroupEgressError>>> {
        (*self).authorize_security_group_egress(builder)
    }
    fn authorize_security_group_ingress(&self, builder: AuthorizeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeSecurityGroupIngressOutput, SdkError<AuthorizeSecurityGroupIngressError>>> {
        (*self).authorize_security_group_ingress(builder)
    }
    fn bundle_instance(&self, builder: BundleInstanceInputBuilder) -> impl Future<Output = Result<BundleInstanceOutput, SdkError<BundleInstanceError>>> {
        (*self).bundle_instance(builder)
    }
    fn cancel_bundle_task(&self, builder: CancelBundleTaskInputBuilder) -> impl Future<Output = Result<CancelBundleTaskOutput, SdkError<CancelBundleTaskError>>> {
        (*self).cancel_bundle_task(builder)
    }
    fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>> {
        (*self).cancel_capacity_reservation(builder)
    }
    fn cancel_capacity_reservation_fleets(&self, builder: CancelCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<CancelCapacityReservationFleetsOutput, SdkError<CancelCapacityReservationFleetsError>>> {
        (*self).cancel_capacity_reservation_fleets(builder)
    }
    fn cancel_conversion_task(&self, builder: CancelConversionTaskInputBuilder) -> impl Future<Output = Result<CancelConversionTaskOutput, SdkError<CancelConversionTaskError>>> {
        (*self).cancel_conversion_task(builder)
    }
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>> {
        (*self).cancel_export_task(builder)
    }
    fn cancel_image_launch_permission(&self, builder: CancelImageLaunchPermissionInputBuilder) -> impl Future<Output = Result<CancelImageLaunchPermissionOutput, SdkError<CancelImageLaunchPermissionError>>> {
        (*self).cancel_image_launch_permission(builder)
    }
    fn cancel_import_task(&self, builder: CancelImportTaskInputBuilder) -> impl Future<Output = Result<CancelImportTaskOutput, SdkError<CancelImportTaskError>>> {
        (*self).cancel_import_task(builder)
    }
    fn cancel_reserved_instances_listing(&self, builder: CancelReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CancelReservedInstancesListingOutput, SdkError<CancelReservedInstancesListingError>>> {
        (*self).cancel_reserved_instances_listing(builder)
    }
    fn cancel_spot_fleet_requests(&self, builder: CancelSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotFleetRequestsOutput, SdkError<CancelSpotFleetRequestsError>>> {
        (*self).cancel_spot_fleet_requests(builder)
    }
    fn cancel_spot_instance_requests(&self, builder: CancelSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<CancelSpotInstanceRequestsOutput, SdkError<CancelSpotInstanceRequestsError>>> {
        (*self).cancel_spot_instance_requests(builder)
    }
    fn confirm_product_instance(&self, builder: ConfirmProductInstanceInputBuilder) -> impl Future<Output = Result<ConfirmProductInstanceOutput, SdkError<ConfirmProductInstanceError>>> {
        (*self).confirm_product_instance(builder)
    }
    fn copy_fpga_image(&self, builder: CopyFpgaImageInputBuilder) -> impl Future<Output = Result<CopyFpgaImageOutput, SdkError<CopyFpgaImageError>>> {
        (*self).copy_fpga_image(builder)
    }
    fn copy_image(&self, builder: CopyImageInputBuilder) -> impl Future<Output = Result<CopyImageOutput, SdkError<CopyImageError>>> {
        (*self).copy_image(builder)
    }
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>> {
        (*self).copy_snapshot(builder)
    }
    fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>> {
        (*self).create_capacity_reservation(builder)
    }
    fn create_capacity_reservation_fleet(&self, builder: CreateCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<CreateCapacityReservationFleetOutput, SdkError<CreateCapacityReservationFleetError>>> {
        (*self).create_capacity_reservation_fleet(builder)
    }
    fn create_carrier_gateway(&self, builder: CreateCarrierGatewayInputBuilder) -> impl Future<Output = Result<CreateCarrierGatewayOutput, SdkError<CreateCarrierGatewayError>>> {
        (*self).create_carrier_gateway(builder)
    }
    fn create_client_vpn_endpoint(&self, builder: CreateClientVpnEndpointInputBuilder) -> impl Future<Output = Result<CreateClientVpnEndpointOutput, SdkError<CreateClientVpnEndpointError>>> {
        (*self).create_client_vpn_endpoint(builder)
    }
    fn create_client_vpn_route(&self, builder: CreateClientVpnRouteInputBuilder) -> impl Future<Output = Result<CreateClientVpnRouteOutput, SdkError<CreateClientVpnRouteError>>> {
        (*self).create_client_vpn_route(builder)
    }
    fn create_coip_cidr(&self, builder: CreateCoipCidrInputBuilder) -> impl Future<Output = Result<CreateCoipCidrOutput, SdkError<CreateCoipCidrError>>> {
        (*self).create_coip_cidr(builder)
    }
    fn create_coip_pool(&self, builder: CreateCoipPoolInputBuilder) -> impl Future<Output = Result<CreateCoipPoolOutput, SdkError<CreateCoipPoolError>>> {
        (*self).create_coip_pool(builder)
    }
    fn create_customer_gateway(&self, builder: CreateCustomerGatewayInputBuilder) -> impl Future<Output = Result<CreateCustomerGatewayOutput, SdkError<CreateCustomerGatewayError>>> {
        (*self).create_customer_gateway(builder)
    }
    fn create_default_subnet(&self, builder: CreateDefaultSubnetInputBuilder) -> impl Future<Output = Result<CreateDefaultSubnetOutput, SdkError<CreateDefaultSubnetError>>> {
        (*self).create_default_subnet(builder)
    }
    fn create_default_vpc(&self, builder: CreateDefaultVpcInputBuilder) -> impl Future<Output = Result<CreateDefaultVpcOutput, SdkError<CreateDefaultVpcError>>> {
        (*self).create_default_vpc(builder)
    }
    fn create_dhcp_options(&self, builder: CreateDhcpOptionsInputBuilder) -> impl Future<Output = Result<CreateDhcpOptionsOutput, SdkError<CreateDhcpOptionsError>>> {
        (*self).create_dhcp_options(builder)
    }
    fn create_egress_only_internet_gateway(&self, builder: CreateEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateEgressOnlyInternetGatewayOutput, SdkError<CreateEgressOnlyInternetGatewayError>>> {
        (*self).create_egress_only_internet_gateway(builder)
    }
    fn create_fleet(&self, builder: CreateFleetInputBuilder) -> impl Future<Output = Result<CreateFleetOutput, SdkError<CreateFleetError>>> {
        (*self).create_fleet(builder)
    }
    fn create_flow_logs(&self, builder: CreateFlowLogsInputBuilder) -> impl Future<Output = Result<CreateFlowLogsOutput, SdkError<CreateFlowLogsError>>> {
        (*self).create_flow_logs(builder)
    }
    fn create_fpga_image(&self, builder: CreateFpgaImageInputBuilder) -> impl Future<Output = Result<CreateFpgaImageOutput, SdkError<CreateFpgaImageError>>> {
        (*self).create_fpga_image(builder)
    }
    fn create_image(&self, builder: CreateImageInputBuilder) -> impl Future<Output = Result<CreateImageOutput, SdkError<CreateImageError>>> {
        (*self).create_image(builder)
    }
    fn create_instance_connect_endpoint(&self, builder: CreateInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<CreateInstanceConnectEndpointOutput, SdkError<CreateInstanceConnectEndpointError>>> {
        (*self).create_instance_connect_endpoint(builder)
    }
    fn create_instance_event_window(&self, builder: CreateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<CreateInstanceEventWindowOutput, SdkError<CreateInstanceEventWindowError>>> {
        (*self).create_instance_event_window(builder)
    }
    fn create_instance_export_task(&self, builder: CreateInstanceExportTaskInputBuilder) -> impl Future<Output = Result<CreateInstanceExportTaskOutput, SdkError<CreateInstanceExportTaskError>>> {
        (*self).create_instance_export_task(builder)
    }
    fn create_internet_gateway(&self, builder: CreateInternetGatewayInputBuilder) -> impl Future<Output = Result<CreateInternetGatewayOutput, SdkError<CreateInternetGatewayError>>> {
        (*self).create_internet_gateway(builder)
    }
    fn create_ipam(&self, builder: CreateIpamInputBuilder) -> impl Future<Output = Result<CreateIpamOutput, SdkError<CreateIpamError>>> {
        (*self).create_ipam(builder)
    }
    fn create_ipam_external_resource_verification_token(&self, builder: CreateIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<CreateIpamExternalResourceVerificationTokenOutput, SdkError<CreateIpamExternalResourceVerificationTokenError>>> {
        (*self).create_ipam_external_resource_verification_token(builder)
    }
    fn create_ipam_pool(&self, builder: CreateIpamPoolInputBuilder) -> impl Future<Output = Result<CreateIpamPoolOutput, SdkError<CreateIpamPoolError>>> {
        (*self).create_ipam_pool(builder)
    }
    fn create_ipam_resource_discovery(&self, builder: CreateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<CreateIpamResourceDiscoveryOutput, SdkError<CreateIpamResourceDiscoveryError>>> {
        (*self).create_ipam_resource_discovery(builder)
    }
    fn create_ipam_scope(&self, builder: CreateIpamScopeInputBuilder) -> impl Future<Output = Result<CreateIpamScopeOutput, SdkError<CreateIpamScopeError>>> {
        (*self).create_ipam_scope(builder)
    }
    fn create_key_pair(&self, builder: CreateKeyPairInputBuilder) -> impl Future<Output = Result<CreateKeyPairOutput, SdkError<CreateKeyPairError>>> {
        (*self).create_key_pair(builder)
    }
    fn create_launch_template(&self, builder: CreateLaunchTemplateInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateOutput, SdkError<CreateLaunchTemplateError>>> {
        (*self).create_launch_template(builder)
    }
    fn create_launch_template_version(&self, builder: CreateLaunchTemplateVersionInputBuilder) -> impl Future<Output = Result<CreateLaunchTemplateVersionOutput, SdkError<CreateLaunchTemplateVersionError>>> {
        (*self).create_launch_template_version(builder)
    }
    fn create_local_gateway_route(&self, builder: CreateLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteOutput, SdkError<CreateLocalGatewayRouteError>>> {
        (*self).create_local_gateway_route(builder)
    }
    fn create_local_gateway_route_table(&self, builder: CreateLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableOutput, SdkError<CreateLocalGatewayRouteTableError>>> {
        (*self).create_local_gateway_route_table(builder)
    }
    fn create_local_gateway_route_table_virtual_interface_group_association(&self, builder: CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>> {
        (*self).create_local_gateway_route_table_virtual_interface_group_association(builder)
    }
    fn create_local_gateway_route_table_vpc_association(&self, builder: CreateLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<CreateLocalGatewayRouteTableVpcAssociationOutput, SdkError<CreateLocalGatewayRouteTableVpcAssociationError>>> {
        (*self).create_local_gateway_route_table_vpc_association(builder)
    }
    fn create_managed_prefix_list(&self, builder: CreateManagedPrefixListInputBuilder) -> impl Future<Output = Result<CreateManagedPrefixListOutput, SdkError<CreateManagedPrefixListError>>> {
        (*self).create_managed_prefix_list(builder)
    }
    fn create_nat_gateway(&self, builder: CreateNatGatewayInputBuilder) -> impl Future<Output = Result<CreateNatGatewayOutput, SdkError<CreateNatGatewayError>>> {
        (*self).create_nat_gateway(builder)
    }
    fn create_network_acl(&self, builder: CreateNetworkAclInputBuilder) -> impl Future<Output = Result<CreateNetworkAclOutput, SdkError<CreateNetworkAclError>>> {
        (*self).create_network_acl(builder)
    }
    fn create_network_acl_entry(&self, builder: CreateNetworkAclEntryInputBuilder) -> impl Future<Output = Result<CreateNetworkAclEntryOutput, SdkError<CreateNetworkAclEntryError>>> {
        (*self).create_network_acl_entry(builder)
    }
    fn create_network_insights_access_scope(&self, builder: CreateNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsAccessScopeOutput, SdkError<CreateNetworkInsightsAccessScopeError>>> {
        (*self).create_network_insights_access_scope(builder)
    }
    fn create_network_insights_path(&self, builder: CreateNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<CreateNetworkInsightsPathOutput, SdkError<CreateNetworkInsightsPathError>>> {
        (*self).create_network_insights_path(builder)
    }
    fn create_network_interface(&self, builder: CreateNetworkInterfaceInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfaceOutput, SdkError<CreateNetworkInterfaceError>>> {
        (*self).create_network_interface(builder)
    }
    fn create_network_interface_permission(&self, builder: CreateNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<CreateNetworkInterfacePermissionOutput, SdkError<CreateNetworkInterfacePermissionError>>> {
        (*self).create_network_interface_permission(builder)
    }
    fn create_placement_group(&self, builder: CreatePlacementGroupInputBuilder) -> impl Future<Output = Result<CreatePlacementGroupOutput, SdkError<CreatePlacementGroupError>>> {
        (*self).create_placement_group(builder)
    }
    fn create_public_ipv4_pool(&self, builder: CreatePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<CreatePublicIpv4PoolOutput, SdkError<CreatePublicIpv4PoolError>>> {
        (*self).create_public_ipv4_pool(builder)
    }
    fn create_replace_root_volume_task(&self, builder: CreateReplaceRootVolumeTaskInputBuilder) -> impl Future<Output = Result<CreateReplaceRootVolumeTaskOutput, SdkError<CreateReplaceRootVolumeTaskError>>> {
        (*self).create_replace_root_volume_task(builder)
    }
    fn create_reserved_instances_listing(&self, builder: CreateReservedInstancesListingInputBuilder) -> impl Future<Output = Result<CreateReservedInstancesListingOutput, SdkError<CreateReservedInstancesListingError>>> {
        (*self).create_reserved_instances_listing(builder)
    }
    fn create_restore_image_task(&self, builder: CreateRestoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateRestoreImageTaskOutput, SdkError<CreateRestoreImageTaskError>>> {
        (*self).create_restore_image_task(builder)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        (*self).create_route(builder)
    }
    fn create_route_table(&self, builder: CreateRouteTableInputBuilder) -> impl Future<Output = Result<CreateRouteTableOutput, SdkError<CreateRouteTableError>>> {
        (*self).create_route_table(builder)
    }
    fn create_security_group(&self, builder: CreateSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateSecurityGroupOutput, SdkError<CreateSecurityGroupError>>> {
        (*self).create_security_group(builder)
    }
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>> {
        (*self).create_snapshot(builder)
    }
    fn create_snapshots(&self, builder: CreateSnapshotsInputBuilder) -> impl Future<Output = Result<CreateSnapshotsOutput, SdkError<CreateSnapshotsError>>> {
        (*self).create_snapshots(builder)
    }
    fn create_spot_datafeed_subscription(&self, builder: CreateSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<CreateSpotDatafeedSubscriptionOutput, SdkError<CreateSpotDatafeedSubscriptionError>>> {
        (*self).create_spot_datafeed_subscription(builder)
    }
    fn create_store_image_task(&self, builder: CreateStoreImageTaskInputBuilder) -> impl Future<Output = Result<CreateStoreImageTaskOutput, SdkError<CreateStoreImageTaskError>>> {
        (*self).create_store_image_task(builder)
    }
    fn create_subnet(&self, builder: CreateSubnetInputBuilder) -> impl Future<Output = Result<CreateSubnetOutput, SdkError<CreateSubnetError>>> {
        (*self).create_subnet(builder)
    }
    fn create_subnet_cidr_reservation(&self, builder: CreateSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<CreateSubnetCidrReservationOutput, SdkError<CreateSubnetCidrReservationError>>> {
        (*self).create_subnet_cidr_reservation(builder)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        (*self).create_tags(builder)
    }
    fn create_traffic_mirror_filter(&self, builder: CreateTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterOutput, SdkError<CreateTrafficMirrorFilterError>>> {
        (*self).create_traffic_mirror_filter(builder)
    }
    fn create_traffic_mirror_filter_rule(&self, builder: CreateTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorFilterRuleOutput, SdkError<CreateTrafficMirrorFilterRuleError>>> {
        (*self).create_traffic_mirror_filter_rule(builder)
    }
    fn create_traffic_mirror_session(&self, builder: CreateTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorSessionOutput, SdkError<CreateTrafficMirrorSessionError>>> {
        (*self).create_traffic_mirror_session(builder)
    }
    fn create_traffic_mirror_target(&self, builder: CreateTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<CreateTrafficMirrorTargetOutput, SdkError<CreateTrafficMirrorTargetError>>> {
        (*self).create_traffic_mirror_target(builder)
    }
    fn create_transit_gateway(&self, builder: CreateTransitGatewayInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayOutput, SdkError<CreateTransitGatewayError>>> {
        (*self).create_transit_gateway(builder)
    }
    fn create_transit_gateway_connect(&self, builder: CreateTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectOutput, SdkError<CreateTransitGatewayConnectError>>> {
        (*self).create_transit_gateway_connect(builder)
    }
    fn create_transit_gateway_connect_peer(&self, builder: CreateTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayConnectPeerOutput, SdkError<CreateTransitGatewayConnectPeerError>>> {
        (*self).create_transit_gateway_connect_peer(builder)
    }
    fn create_transit_gateway_multicast_domain(&self, builder: CreateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayMulticastDomainOutput, SdkError<CreateTransitGatewayMulticastDomainError>>> {
        (*self).create_transit_gateway_multicast_domain(builder)
    }
    fn create_transit_gateway_peering_attachment(&self, builder: CreateTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPeeringAttachmentOutput, SdkError<CreateTransitGatewayPeeringAttachmentError>>> {
        (*self).create_transit_gateway_peering_attachment(builder)
    }
    fn create_transit_gateway_policy_table(&self, builder: CreateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPolicyTableOutput, SdkError<CreateTransitGatewayPolicyTableError>>> {
        (*self).create_transit_gateway_policy_table(builder)
    }
    fn create_transit_gateway_prefix_list_reference(&self, builder: CreateTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayPrefixListReferenceOutput, SdkError<CreateTransitGatewayPrefixListReferenceError>>> {
        (*self).create_transit_gateway_prefix_list_reference(builder)
    }
    fn create_transit_gateway_route(&self, builder: CreateTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteOutput, SdkError<CreateTransitGatewayRouteError>>> {
        (*self).create_transit_gateway_route(builder)
    }
    fn create_transit_gateway_route_table(&self, builder: CreateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableOutput, SdkError<CreateTransitGatewayRouteTableError>>> {
        (*self).create_transit_gateway_route_table(builder)
    }
    fn create_transit_gateway_route_table_announcement(&self, builder: CreateTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayRouteTableAnnouncementOutput, SdkError<CreateTransitGatewayRouteTableAnnouncementError>>> {
        (*self).create_transit_gateway_route_table_announcement(builder)
    }
    fn create_transit_gateway_vpc_attachment(&self, builder: CreateTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<CreateTransitGatewayVpcAttachmentOutput, SdkError<CreateTransitGatewayVpcAttachmentError>>> {
        (*self).create_transit_gateway_vpc_attachment(builder)
    }
    fn create_verified_access_endpoint(&self, builder: CreateVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessEndpointOutput, SdkError<CreateVerifiedAccessEndpointError>>> {
        (*self).create_verified_access_endpoint(builder)
    }
    fn create_verified_access_group(&self, builder: CreateVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessGroupOutput, SdkError<CreateVerifiedAccessGroupError>>> {
        (*self).create_verified_access_group(builder)
    }
    fn create_verified_access_instance(&self, builder: CreateVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessInstanceOutput, SdkError<CreateVerifiedAccessInstanceError>>> {
        (*self).create_verified_access_instance(builder)
    }
    fn create_verified_access_trust_provider(&self, builder: CreateVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<CreateVerifiedAccessTrustProviderOutput, SdkError<CreateVerifiedAccessTrustProviderError>>> {
        (*self).create_verified_access_trust_provider(builder)
    }
    fn create_volume(&self, builder: CreateVolumeInputBuilder) -> impl Future<Output = Result<CreateVolumeOutput, SdkError<CreateVolumeError>>> {
        (*self).create_volume(builder)
    }
    fn create_vpc(&self, builder: CreateVpcInputBuilder) -> impl Future<Output = Result<CreateVpcOutput, SdkError<CreateVpcError>>> {
        (*self).create_vpc(builder)
    }
    fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>> {
        (*self).create_vpc_endpoint(builder)
    }
    fn create_vpc_endpoint_connection_notification(&self, builder: CreateVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointConnectionNotificationOutput, SdkError<CreateVpcEndpointConnectionNotificationError>>> {
        (*self).create_vpc_endpoint_connection_notification(builder)
    }
    fn create_vpc_endpoint_service_configuration(&self, builder: CreateVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<CreateVpcEndpointServiceConfigurationOutput, SdkError<CreateVpcEndpointServiceConfigurationError>>> {
        (*self).create_vpc_endpoint_service_configuration(builder)
    }
    fn create_vpc_peering_connection(&self, builder: CreateVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcPeeringConnectionOutput, SdkError<CreateVpcPeeringConnectionError>>> {
        (*self).create_vpc_peering_connection(builder)
    }
    fn create_vpn_connection(&self, builder: CreateVpnConnectionInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionOutput, SdkError<CreateVpnConnectionError>>> {
        (*self).create_vpn_connection(builder)
    }
    fn create_vpn_connection_route(&self, builder: CreateVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<CreateVpnConnectionRouteOutput, SdkError<CreateVpnConnectionRouteError>>> {
        (*self).create_vpn_connection_route(builder)
    }
    fn create_vpn_gateway(&self, builder: CreateVpnGatewayInputBuilder) -> impl Future<Output = Result<CreateVpnGatewayOutput, SdkError<CreateVpnGatewayError>>> {
        (*self).create_vpn_gateway(builder)
    }
    fn delete_carrier_gateway(&self, builder: DeleteCarrierGatewayInputBuilder) -> impl Future<Output = Result<DeleteCarrierGatewayOutput, SdkError<DeleteCarrierGatewayError>>> {
        (*self).delete_carrier_gateway(builder)
    }
    fn delete_client_vpn_endpoint(&self, builder: DeleteClientVpnEndpointInputBuilder) -> impl Future<Output = Result<DeleteClientVpnEndpointOutput, SdkError<DeleteClientVpnEndpointError>>> {
        (*self).delete_client_vpn_endpoint(builder)
    }
    fn delete_client_vpn_route(&self, builder: DeleteClientVpnRouteInputBuilder) -> impl Future<Output = Result<DeleteClientVpnRouteOutput, SdkError<DeleteClientVpnRouteError>>> {
        (*self).delete_client_vpn_route(builder)
    }
    fn delete_coip_cidr(&self, builder: DeleteCoipCidrInputBuilder) -> impl Future<Output = Result<DeleteCoipCidrOutput, SdkError<DeleteCoipCidrError>>> {
        (*self).delete_coip_cidr(builder)
    }
    fn delete_coip_pool(&self, builder: DeleteCoipPoolInputBuilder) -> impl Future<Output = Result<DeleteCoipPoolOutput, SdkError<DeleteCoipPoolError>>> {
        (*self).delete_coip_pool(builder)
    }
    fn delete_customer_gateway(&self, builder: DeleteCustomerGatewayInputBuilder) -> impl Future<Output = Result<DeleteCustomerGatewayOutput, SdkError<DeleteCustomerGatewayError>>> {
        (*self).delete_customer_gateway(builder)
    }
    fn delete_dhcp_options(&self, builder: DeleteDhcpOptionsInputBuilder) -> impl Future<Output = Result<DeleteDhcpOptionsOutput, SdkError<DeleteDhcpOptionsError>>> {
        (*self).delete_dhcp_options(builder)
    }
    fn delete_egress_only_internet_gateway(&self, builder: DeleteEgressOnlyInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteEgressOnlyInternetGatewayOutput, SdkError<DeleteEgressOnlyInternetGatewayError>>> {
        (*self).delete_egress_only_internet_gateway(builder)
    }
    fn delete_fleets(&self, builder: DeleteFleetsInputBuilder) -> impl Future<Output = Result<DeleteFleetsOutput, SdkError<DeleteFleetsError>>> {
        (*self).delete_fleets(builder)
    }
    fn delete_flow_logs(&self, builder: DeleteFlowLogsInputBuilder) -> impl Future<Output = Result<DeleteFlowLogsOutput, SdkError<DeleteFlowLogsError>>> {
        (*self).delete_flow_logs(builder)
    }
    fn delete_fpga_image(&self, builder: DeleteFpgaImageInputBuilder) -> impl Future<Output = Result<DeleteFpgaImageOutput, SdkError<DeleteFpgaImageError>>> {
        (*self).delete_fpga_image(builder)
    }
    fn delete_instance_connect_endpoint(&self, builder: DeleteInstanceConnectEndpointInputBuilder) -> impl Future<Output = Result<DeleteInstanceConnectEndpointOutput, SdkError<DeleteInstanceConnectEndpointError>>> {
        (*self).delete_instance_connect_endpoint(builder)
    }
    fn delete_instance_event_window(&self, builder: DeleteInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DeleteInstanceEventWindowOutput, SdkError<DeleteInstanceEventWindowError>>> {
        (*self).delete_instance_event_window(builder)
    }
    fn delete_internet_gateway(&self, builder: DeleteInternetGatewayInputBuilder) -> impl Future<Output = Result<DeleteInternetGatewayOutput, SdkError<DeleteInternetGatewayError>>> {
        (*self).delete_internet_gateway(builder)
    }
    fn delete_ipam(&self, builder: DeleteIpamInputBuilder) -> impl Future<Output = Result<DeleteIpamOutput, SdkError<DeleteIpamError>>> {
        (*self).delete_ipam(builder)
    }
    fn delete_ipam_external_resource_verification_token(&self, builder: DeleteIpamExternalResourceVerificationTokenInputBuilder) -> impl Future<Output = Result<DeleteIpamExternalResourceVerificationTokenOutput, SdkError<DeleteIpamExternalResourceVerificationTokenError>>> {
        (*self).delete_ipam_external_resource_verification_token(builder)
    }
    fn delete_ipam_pool(&self, builder: DeleteIpamPoolInputBuilder) -> impl Future<Output = Result<DeleteIpamPoolOutput, SdkError<DeleteIpamPoolError>>> {
        (*self).delete_ipam_pool(builder)
    }
    fn delete_ipam_resource_discovery(&self, builder: DeleteIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DeleteIpamResourceDiscoveryOutput, SdkError<DeleteIpamResourceDiscoveryError>>> {
        (*self).delete_ipam_resource_discovery(builder)
    }
    fn delete_ipam_scope(&self, builder: DeleteIpamScopeInputBuilder) -> impl Future<Output = Result<DeleteIpamScopeOutput, SdkError<DeleteIpamScopeError>>> {
        (*self).delete_ipam_scope(builder)
    }
    fn delete_key_pair(&self, builder: DeleteKeyPairInputBuilder) -> impl Future<Output = Result<DeleteKeyPairOutput, SdkError<DeleteKeyPairError>>> {
        (*self).delete_key_pair(builder)
    }
    fn delete_launch_template(&self, builder: DeleteLaunchTemplateInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateOutput, SdkError<DeleteLaunchTemplateError>>> {
        (*self).delete_launch_template(builder)
    }
    fn delete_launch_template_versions(&self, builder: DeleteLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DeleteLaunchTemplateVersionsOutput, SdkError<DeleteLaunchTemplateVersionsError>>> {
        (*self).delete_launch_template_versions(builder)
    }
    fn delete_local_gateway_route(&self, builder: DeleteLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteOutput, SdkError<DeleteLocalGatewayRouteError>>> {
        (*self).delete_local_gateway_route(builder)
    }
    fn delete_local_gateway_route_table(&self, builder: DeleteLocalGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableOutput, SdkError<DeleteLocalGatewayRouteTableError>>> {
        (*self).delete_local_gateway_route_table(builder)
    }
    fn delete_local_gateway_route_table_virtual_interface_group_association(&self, builder: DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>> {
        (*self).delete_local_gateway_route_table_virtual_interface_group_association(builder)
    }
    fn delete_local_gateway_route_table_vpc_association(&self, builder: DeleteLocalGatewayRouteTableVpcAssociationInputBuilder) -> impl Future<Output = Result<DeleteLocalGatewayRouteTableVpcAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVpcAssociationError>>> {
        (*self).delete_local_gateway_route_table_vpc_association(builder)
    }
    fn delete_managed_prefix_list(&self, builder: DeleteManagedPrefixListInputBuilder) -> impl Future<Output = Result<DeleteManagedPrefixListOutput, SdkError<DeleteManagedPrefixListError>>> {
        (*self).delete_managed_prefix_list(builder)
    }
    fn delete_nat_gateway(&self, builder: DeleteNatGatewayInputBuilder) -> impl Future<Output = Result<DeleteNatGatewayOutput, SdkError<DeleteNatGatewayError>>> {
        (*self).delete_nat_gateway(builder)
    }
    fn delete_network_acl(&self, builder: DeleteNetworkAclInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclOutput, SdkError<DeleteNetworkAclError>>> {
        (*self).delete_network_acl(builder)
    }
    fn delete_network_acl_entry(&self, builder: DeleteNetworkAclEntryInputBuilder) -> impl Future<Output = Result<DeleteNetworkAclEntryOutput, SdkError<DeleteNetworkAclEntryError>>> {
        (*self).delete_network_acl_entry(builder)
    }
    fn delete_network_insights_access_scope(&self, builder: DeleteNetworkInsightsAccessScopeInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeOutput, SdkError<DeleteNetworkInsightsAccessScopeError>>> {
        (*self).delete_network_insights_access_scope(builder)
    }
    fn delete_network_insights_access_scope_analysis(&self, builder: DeleteNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAccessScopeAnalysisOutput, SdkError<DeleteNetworkInsightsAccessScopeAnalysisError>>> {
        (*self).delete_network_insights_access_scope_analysis(builder)
    }
    fn delete_network_insights_analysis(&self, builder: DeleteNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsAnalysisOutput, SdkError<DeleteNetworkInsightsAnalysisError>>> {
        (*self).delete_network_insights_analysis(builder)
    }
    fn delete_network_insights_path(&self, builder: DeleteNetworkInsightsPathInputBuilder) -> impl Future<Output = Result<DeleteNetworkInsightsPathOutput, SdkError<DeleteNetworkInsightsPathError>>> {
        (*self).delete_network_insights_path(builder)
    }
    fn delete_network_interface(&self, builder: DeleteNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfaceOutput, SdkError<DeleteNetworkInterfaceError>>> {
        (*self).delete_network_interface(builder)
    }
    fn delete_network_interface_permission(&self, builder: DeleteNetworkInterfacePermissionInputBuilder) -> impl Future<Output = Result<DeleteNetworkInterfacePermissionOutput, SdkError<DeleteNetworkInterfacePermissionError>>> {
        (*self).delete_network_interface_permission(builder)
    }
    fn delete_placement_group(&self, builder: DeletePlacementGroupInputBuilder) -> impl Future<Output = Result<DeletePlacementGroupOutput, SdkError<DeletePlacementGroupError>>> {
        (*self).delete_placement_group(builder)
    }
    fn delete_public_ipv4_pool(&self, builder: DeletePublicIpv4PoolInputBuilder) -> impl Future<Output = Result<DeletePublicIpv4PoolOutput, SdkError<DeletePublicIpv4PoolError>>> {
        (*self).delete_public_ipv4_pool(builder)
    }
    fn delete_queued_reserved_instances(&self, builder: DeleteQueuedReservedInstancesInputBuilder) -> impl Future<Output = Result<DeleteQueuedReservedInstancesOutput, SdkError<DeleteQueuedReservedInstancesError>>> {
        (*self).delete_queued_reserved_instances(builder)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        (*self).delete_route(builder)
    }
    fn delete_route_table(&self, builder: DeleteRouteTableInputBuilder) -> impl Future<Output = Result<DeleteRouteTableOutput, SdkError<DeleteRouteTableError>>> {
        (*self).delete_route_table(builder)
    }
    fn delete_security_group(&self, builder: DeleteSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteSecurityGroupOutput, SdkError<DeleteSecurityGroupError>>> {
        (*self).delete_security_group(builder)
    }
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>> {
        (*self).delete_snapshot(builder)
    }
    fn delete_spot_datafeed_subscription(&self, builder: DeleteSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteSpotDatafeedSubscriptionOutput, SdkError<DeleteSpotDatafeedSubscriptionError>>> {
        (*self).delete_spot_datafeed_subscription(builder)
    }
    fn delete_subnet(&self, builder: DeleteSubnetInputBuilder) -> impl Future<Output = Result<DeleteSubnetOutput, SdkError<DeleteSubnetError>>> {
        (*self).delete_subnet(builder)
    }
    fn delete_subnet_cidr_reservation(&self, builder: DeleteSubnetCidrReservationInputBuilder) -> impl Future<Output = Result<DeleteSubnetCidrReservationOutput, SdkError<DeleteSubnetCidrReservationError>>> {
        (*self).delete_subnet_cidr_reservation(builder)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        (*self).delete_tags(builder)
    }
    fn delete_traffic_mirror_filter(&self, builder: DeleteTrafficMirrorFilterInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterOutput, SdkError<DeleteTrafficMirrorFilterError>>> {
        (*self).delete_traffic_mirror_filter(builder)
    }
    fn delete_traffic_mirror_filter_rule(&self, builder: DeleteTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorFilterRuleOutput, SdkError<DeleteTrafficMirrorFilterRuleError>>> {
        (*self).delete_traffic_mirror_filter_rule(builder)
    }
    fn delete_traffic_mirror_session(&self, builder: DeleteTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorSessionOutput, SdkError<DeleteTrafficMirrorSessionError>>> {
        (*self).delete_traffic_mirror_session(builder)
    }
    fn delete_traffic_mirror_target(&self, builder: DeleteTrafficMirrorTargetInputBuilder) -> impl Future<Output = Result<DeleteTrafficMirrorTargetOutput, SdkError<DeleteTrafficMirrorTargetError>>> {
        (*self).delete_traffic_mirror_target(builder)
    }
    fn delete_transit_gateway(&self, builder: DeleteTransitGatewayInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayOutput, SdkError<DeleteTransitGatewayError>>> {
        (*self).delete_transit_gateway(builder)
    }
    fn delete_transit_gateway_connect(&self, builder: DeleteTransitGatewayConnectInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectOutput, SdkError<DeleteTransitGatewayConnectError>>> {
        (*self).delete_transit_gateway_connect(builder)
    }
    fn delete_transit_gateway_connect_peer(&self, builder: DeleteTransitGatewayConnectPeerInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayConnectPeerOutput, SdkError<DeleteTransitGatewayConnectPeerError>>> {
        (*self).delete_transit_gateway_connect_peer(builder)
    }
    fn delete_transit_gateway_multicast_domain(&self, builder: DeleteTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayMulticastDomainOutput, SdkError<DeleteTransitGatewayMulticastDomainError>>> {
        (*self).delete_transit_gateway_multicast_domain(builder)
    }
    fn delete_transit_gateway_peering_attachment(&self, builder: DeleteTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPeeringAttachmentOutput, SdkError<DeleteTransitGatewayPeeringAttachmentError>>> {
        (*self).delete_transit_gateway_peering_attachment(builder)
    }
    fn delete_transit_gateway_policy_table(&self, builder: DeleteTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPolicyTableOutput, SdkError<DeleteTransitGatewayPolicyTableError>>> {
        (*self).delete_transit_gateway_policy_table(builder)
    }
    fn delete_transit_gateway_prefix_list_reference(&self, builder: DeleteTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayPrefixListReferenceOutput, SdkError<DeleteTransitGatewayPrefixListReferenceError>>> {
        (*self).delete_transit_gateway_prefix_list_reference(builder)
    }
    fn delete_transit_gateway_route(&self, builder: DeleteTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteOutput, SdkError<DeleteTransitGatewayRouteError>>> {
        (*self).delete_transit_gateway_route(builder)
    }
    fn delete_transit_gateway_route_table(&self, builder: DeleteTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableOutput, SdkError<DeleteTransitGatewayRouteTableError>>> {
        (*self).delete_transit_gateway_route_table(builder)
    }
    fn delete_transit_gateway_route_table_announcement(&self, builder: DeleteTransitGatewayRouteTableAnnouncementInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayRouteTableAnnouncementOutput, SdkError<DeleteTransitGatewayRouteTableAnnouncementError>>> {
        (*self).delete_transit_gateway_route_table_announcement(builder)
    }
    fn delete_transit_gateway_vpc_attachment(&self, builder: DeleteTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<DeleteTransitGatewayVpcAttachmentOutput, SdkError<DeleteTransitGatewayVpcAttachmentError>>> {
        (*self).delete_transit_gateway_vpc_attachment(builder)
    }
    fn delete_verified_access_endpoint(&self, builder: DeleteVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessEndpointOutput, SdkError<DeleteVerifiedAccessEndpointError>>> {
        (*self).delete_verified_access_endpoint(builder)
    }
    fn delete_verified_access_group(&self, builder: DeleteVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessGroupOutput, SdkError<DeleteVerifiedAccessGroupError>>> {
        (*self).delete_verified_access_group(builder)
    }
    fn delete_verified_access_instance(&self, builder: DeleteVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessInstanceOutput, SdkError<DeleteVerifiedAccessInstanceError>>> {
        (*self).delete_verified_access_instance(builder)
    }
    fn delete_verified_access_trust_provider(&self, builder: DeleteVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DeleteVerifiedAccessTrustProviderOutput, SdkError<DeleteVerifiedAccessTrustProviderError>>> {
        (*self).delete_verified_access_trust_provider(builder)
    }
    fn delete_volume(&self, builder: DeleteVolumeInputBuilder) -> impl Future<Output = Result<DeleteVolumeOutput, SdkError<DeleteVolumeError>>> {
        (*self).delete_volume(builder)
    }
    fn delete_vpc(&self, builder: DeleteVpcInputBuilder) -> impl Future<Output = Result<DeleteVpcOutput, SdkError<DeleteVpcError>>> {
        (*self).delete_vpc(builder)
    }
    fn delete_vpc_endpoint_connection_notifications(&self, builder: DeleteVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointConnectionNotificationsOutput, SdkError<DeleteVpcEndpointConnectionNotificationsError>>> {
        (*self).delete_vpc_endpoint_connection_notifications(builder)
    }
    fn delete_vpc_endpoint_service_configurations(&self, builder: DeleteVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointServiceConfigurationsOutput, SdkError<DeleteVpcEndpointServiceConfigurationsError>>> {
        (*self).delete_vpc_endpoint_service_configurations(builder)
    }
    fn delete_vpc_endpoints(&self, builder: DeleteVpcEndpointsInputBuilder) -> impl Future<Output = Result<DeleteVpcEndpointsOutput, SdkError<DeleteVpcEndpointsError>>> {
        (*self).delete_vpc_endpoints(builder)
    }
    fn delete_vpc_peering_connection(&self, builder: DeleteVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcPeeringConnectionOutput, SdkError<DeleteVpcPeeringConnectionError>>> {
        (*self).delete_vpc_peering_connection(builder)
    }
    fn delete_vpn_connection(&self, builder: DeleteVpnConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionOutput, SdkError<DeleteVpnConnectionError>>> {
        (*self).delete_vpn_connection(builder)
    }
    fn delete_vpn_connection_route(&self, builder: DeleteVpnConnectionRouteInputBuilder) -> impl Future<Output = Result<DeleteVpnConnectionRouteOutput, SdkError<DeleteVpnConnectionRouteError>>> {
        (*self).delete_vpn_connection_route(builder)
    }
    fn delete_vpn_gateway(&self, builder: DeleteVpnGatewayInputBuilder) -> impl Future<Output = Result<DeleteVpnGatewayOutput, SdkError<DeleteVpnGatewayError>>> {
        (*self).delete_vpn_gateway(builder)
    }
    fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> impl Future<Output = Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>> {
        (*self).deprovision_byoip_cidr(builder)
    }
    fn deprovision_ipam_byoasn(&self, builder: DeprovisionIpamByoasnInputBuilder) -> impl Future<Output = Result<DeprovisionIpamByoasnOutput, SdkError<DeprovisionIpamByoasnError>>> {
        (*self).deprovision_ipam_byoasn(builder)
    }
    fn deprovision_ipam_pool_cidr(&self, builder: DeprovisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionIpamPoolCidrOutput, SdkError<DeprovisionIpamPoolCidrError>>> {
        (*self).deprovision_ipam_pool_cidr(builder)
    }
    fn deprovision_public_ipv4_pool_cidr(&self, builder: DeprovisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<DeprovisionPublicIpv4PoolCidrOutput, SdkError<DeprovisionPublicIpv4PoolCidrError>>> {
        (*self).deprovision_public_ipv4_pool_cidr(builder)
    }
    fn deregister_image(&self, builder: DeregisterImageInputBuilder) -> impl Future<Output = Result<DeregisterImageOutput, SdkError<DeregisterImageError>>> {
        (*self).deregister_image(builder)
    }
    fn deregister_instance_event_notification_attributes(&self, builder: DeregisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DeregisterInstanceEventNotificationAttributesOutput, SdkError<DeregisterInstanceEventNotificationAttributesError>>> {
        (*self).deregister_instance_event_notification_attributes(builder)
    }
    fn deregister_transit_gateway_multicast_group_members(&self, builder: DeregisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupMembersOutput, SdkError<DeregisterTransitGatewayMulticastGroupMembersError>>> {
        (*self).deregister_transit_gateway_multicast_group_members(builder)
    }
    fn deregister_transit_gateway_multicast_group_sources(&self, builder: DeregisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<DeregisterTransitGatewayMulticastGroupSourcesOutput, SdkError<DeregisterTransitGatewayMulticastGroupSourcesError>>> {
        (*self).deregister_transit_gateway_multicast_group_sources(builder)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        (*self).describe_account_attributes(builder)
    }
    fn describe_address_transfers(&self, builder: DescribeAddressTransfersInputBuilder) -> impl Future<Output = Result<DescribeAddressTransfersOutput, SdkError<DescribeAddressTransfersError>>> {
        (*self).describe_address_transfers(builder)
    }
    fn describe_addresses(&self, builder: DescribeAddressesInputBuilder) -> impl Future<Output = Result<DescribeAddressesOutput, SdkError<DescribeAddressesError>>> {
        (*self).describe_addresses(builder)
    }
    fn describe_addresses_attribute(&self, builder: DescribeAddressesAttributeInputBuilder) -> impl Future<Output = Result<DescribeAddressesAttributeOutput, SdkError<DescribeAddressesAttributeError>>> {
        (*self).describe_addresses_attribute(builder)
    }
    fn describe_aggregate_id_format(&self, builder: DescribeAggregateIdFormatInputBuilder) -> impl Future<Output = Result<DescribeAggregateIdFormatOutput, SdkError<DescribeAggregateIdFormatError>>> {
        (*self).describe_aggregate_id_format(builder)
    }
    fn describe_availability_zones(&self, builder: DescribeAvailabilityZonesInputBuilder) -> impl Future<Output = Result<DescribeAvailabilityZonesOutput, SdkError<DescribeAvailabilityZonesError>>> {
        (*self).describe_availability_zones(builder)
    }
    fn describe_aws_network_performance_metric_subscriptions(&self, builder: DescribeAwsNetworkPerformanceMetricSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeAwsNetworkPerformanceMetricSubscriptionsOutput, SdkError<DescribeAwsNetworkPerformanceMetricSubscriptionsError>>> {
        (*self).describe_aws_network_performance_metric_subscriptions(builder)
    }
    fn describe_bundle_tasks(&self, builder: DescribeBundleTasksInputBuilder) -> impl Future<Output = Result<DescribeBundleTasksOutput, SdkError<DescribeBundleTasksError>>> {
        (*self).describe_bundle_tasks(builder)
    }
    fn describe_byoip_cidrs(&self, builder: DescribeByoipCidrsInputBuilder) -> impl Future<Output = Result<DescribeByoipCidrsOutput, SdkError<DescribeByoipCidrsError>>> {
        (*self).describe_byoip_cidrs(builder)
    }
    fn describe_capacity_block_offerings(&self, builder: DescribeCapacityBlockOfferingsInputBuilder) -> impl Future<Output = Result<DescribeCapacityBlockOfferingsOutput, SdkError<DescribeCapacityBlockOfferingsError>>> {
        (*self).describe_capacity_block_offerings(builder)
    }
    fn describe_capacity_reservation_fleets(&self, builder: DescribeCapacityReservationFleetsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationFleetsOutput, SdkError<DescribeCapacityReservationFleetsError>>> {
        (*self).describe_capacity_reservation_fleets(builder)
    }
    fn describe_capacity_reservations(&self, builder: DescribeCapacityReservationsInputBuilder) -> impl Future<Output = Result<DescribeCapacityReservationsOutput, SdkError<DescribeCapacityReservationsError>>> {
        (*self).describe_capacity_reservations(builder)
    }
    fn describe_carrier_gateways(&self, builder: DescribeCarrierGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCarrierGatewaysOutput, SdkError<DescribeCarrierGatewaysError>>> {
        (*self).describe_carrier_gateways(builder)
    }
    fn describe_classic_link_instances(&self, builder: DescribeClassicLinkInstancesInputBuilder) -> impl Future<Output = Result<DescribeClassicLinkInstancesOutput, SdkError<DescribeClassicLinkInstancesError>>> {
        (*self).describe_classic_link_instances(builder)
    }
    fn describe_client_vpn_authorization_rules(&self, builder: DescribeClientVpnAuthorizationRulesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnAuthorizationRulesOutput, SdkError<DescribeClientVpnAuthorizationRulesError>>> {
        (*self).describe_client_vpn_authorization_rules(builder)
    }
    fn describe_client_vpn_connections(&self, builder: DescribeClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnConnectionsOutput, SdkError<DescribeClientVpnConnectionsError>>> {
        (*self).describe_client_vpn_connections(builder)
    }
    fn describe_client_vpn_endpoints(&self, builder: DescribeClientVpnEndpointsInputBuilder) -> impl Future<Output = Result<DescribeClientVpnEndpointsOutput, SdkError<DescribeClientVpnEndpointsError>>> {
        (*self).describe_client_vpn_endpoints(builder)
    }
    fn describe_client_vpn_routes(&self, builder: DescribeClientVpnRoutesInputBuilder) -> impl Future<Output = Result<DescribeClientVpnRoutesOutput, SdkError<DescribeClientVpnRoutesError>>> {
        (*self).describe_client_vpn_routes(builder)
    }
    fn describe_client_vpn_target_networks(&self, builder: DescribeClientVpnTargetNetworksInputBuilder) -> impl Future<Output = Result<DescribeClientVpnTargetNetworksOutput, SdkError<DescribeClientVpnTargetNetworksError>>> {
        (*self).describe_client_vpn_target_networks(builder)
    }
    fn describe_coip_pools(&self, builder: DescribeCoipPoolsInputBuilder) -> impl Future<Output = Result<DescribeCoipPoolsOutput, SdkError<DescribeCoipPoolsError>>> {
        (*self).describe_coip_pools(builder)
    }
    fn describe_conversion_tasks(&self, builder: DescribeConversionTasksInputBuilder) -> impl Future<Output = Result<DescribeConversionTasksOutput, SdkError<DescribeConversionTasksError>>> {
        (*self).describe_conversion_tasks(builder)
    }
    fn describe_customer_gateways(&self, builder: DescribeCustomerGatewaysInputBuilder) -> impl Future<Output = Result<DescribeCustomerGatewaysOutput, SdkError<DescribeCustomerGatewaysError>>> {
        (*self).describe_customer_gateways(builder)
    }
    fn describe_dhcp_options(&self, builder: DescribeDhcpOptionsInputBuilder) -> impl Future<Output = Result<DescribeDhcpOptionsOutput, SdkError<DescribeDhcpOptionsError>>> {
        (*self).describe_dhcp_options(builder)
    }
    fn describe_egress_only_internet_gateways(&self, builder: DescribeEgressOnlyInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeEgressOnlyInternetGatewaysOutput, SdkError<DescribeEgressOnlyInternetGatewaysError>>> {
        (*self).describe_egress_only_internet_gateways(builder)
    }
    fn describe_elastic_gpus(&self, builder: DescribeElasticGpusInputBuilder) -> impl Future<Output = Result<DescribeElasticGpusOutput, SdkError<DescribeElasticGpusError>>> {
        (*self).describe_elastic_gpus(builder)
    }
    fn describe_export_image_tasks(&self, builder: DescribeExportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeExportImageTasksOutput, SdkError<DescribeExportImageTasksError>>> {
        (*self).describe_export_image_tasks(builder)
    }
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>> {
        (*self).describe_export_tasks(builder)
    }
    fn describe_fast_launch_images(&self, builder: DescribeFastLaunchImagesInputBuilder) -> impl Future<Output = Result<DescribeFastLaunchImagesOutput, SdkError<DescribeFastLaunchImagesError>>> {
        (*self).describe_fast_launch_images(builder)
    }
    fn describe_fast_snapshot_restores(&self, builder: DescribeFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DescribeFastSnapshotRestoresOutput, SdkError<DescribeFastSnapshotRestoresError>>> {
        (*self).describe_fast_snapshot_restores(builder)
    }
    fn describe_fleet_history(&self, builder: DescribeFleetHistoryInputBuilder) -> impl Future<Output = Result<DescribeFleetHistoryOutput, SdkError<DescribeFleetHistoryError>>> {
        (*self).describe_fleet_history(builder)
    }
    fn describe_fleet_instances(&self, builder: DescribeFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeFleetInstancesOutput, SdkError<DescribeFleetInstancesError>>> {
        (*self).describe_fleet_instances(builder)
    }
    fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> impl Future<Output = Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>> {
        (*self).describe_fleets(builder)
    }
    fn describe_flow_logs(&self, builder: DescribeFlowLogsInputBuilder) -> impl Future<Output = Result<DescribeFlowLogsOutput, SdkError<DescribeFlowLogsError>>> {
        (*self).describe_flow_logs(builder)
    }
    fn describe_fpga_image_attribute(&self, builder: DescribeFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeFpgaImageAttributeOutput, SdkError<DescribeFpgaImageAttributeError>>> {
        (*self).describe_fpga_image_attribute(builder)
    }
    fn describe_fpga_images(&self, builder: DescribeFpgaImagesInputBuilder) -> impl Future<Output = Result<DescribeFpgaImagesOutput, SdkError<DescribeFpgaImagesError>>> {
        (*self).describe_fpga_images(builder)
    }
    fn describe_host_reservation_offerings(&self, builder: DescribeHostReservationOfferingsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationOfferingsOutput, SdkError<DescribeHostReservationOfferingsError>>> {
        (*self).describe_host_reservation_offerings(builder)
    }
    fn describe_host_reservations(&self, builder: DescribeHostReservationsInputBuilder) -> impl Future<Output = Result<DescribeHostReservationsOutput, SdkError<DescribeHostReservationsError>>> {
        (*self).describe_host_reservations(builder)
    }
    fn describe_hosts(&self, builder: DescribeHostsInputBuilder) -> impl Future<Output = Result<DescribeHostsOutput, SdkError<DescribeHostsError>>> {
        (*self).describe_hosts(builder)
    }
    fn describe_iam_instance_profile_associations(&self, builder: DescribeIamInstanceProfileAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIamInstanceProfileAssociationsOutput, SdkError<DescribeIamInstanceProfileAssociationsError>>> {
        (*self).describe_iam_instance_profile_associations(builder)
    }
    fn describe_id_format(&self, builder: DescribeIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdFormatOutput, SdkError<DescribeIdFormatError>>> {
        (*self).describe_id_format(builder)
    }
    fn describe_identity_id_format(&self, builder: DescribeIdentityIdFormatInputBuilder) -> impl Future<Output = Result<DescribeIdentityIdFormatOutput, SdkError<DescribeIdentityIdFormatError>>> {
        (*self).describe_identity_id_format(builder)
    }
    fn describe_image_attribute(&self, builder: DescribeImageAttributeInputBuilder) -> impl Future<Output = Result<DescribeImageAttributeOutput, SdkError<DescribeImageAttributeError>>> {
        (*self).describe_image_attribute(builder)
    }
    fn describe_images(&self, builder: DescribeImagesInputBuilder) -> impl Future<Output = Result<DescribeImagesOutput, SdkError<DescribeImagesError>>> {
        (*self).describe_images(builder)
    }
    fn describe_import_image_tasks(&self, builder: DescribeImportImageTasksInputBuilder) -> impl Future<Output = Result<DescribeImportImageTasksOutput, SdkError<DescribeImportImageTasksError>>> {
        (*self).describe_import_image_tasks(builder)
    }
    fn describe_import_snapshot_tasks(&self, builder: DescribeImportSnapshotTasksInputBuilder) -> impl Future<Output = Result<DescribeImportSnapshotTasksOutput, SdkError<DescribeImportSnapshotTasksError>>> {
        (*self).describe_import_snapshot_tasks(builder)
    }
    fn describe_instance_attribute(&self, builder: DescribeInstanceAttributeInputBuilder) -> impl Future<Output = Result<DescribeInstanceAttributeOutput, SdkError<DescribeInstanceAttributeError>>> {
        (*self).describe_instance_attribute(builder)
    }
    fn describe_instance_connect_endpoints(&self, builder: DescribeInstanceConnectEndpointsInputBuilder) -> impl Future<Output = Result<DescribeInstanceConnectEndpointsOutput, SdkError<DescribeInstanceConnectEndpointsError>>> {
        (*self).describe_instance_connect_endpoints(builder)
    }
    fn describe_instance_credit_specifications(&self, builder: DescribeInstanceCreditSpecificationsInputBuilder) -> impl Future<Output = Result<DescribeInstanceCreditSpecificationsOutput, SdkError<DescribeInstanceCreditSpecificationsError>>> {
        (*self).describe_instance_credit_specifications(builder)
    }
    fn describe_instance_event_notification_attributes(&self, builder: DescribeInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventNotificationAttributesOutput, SdkError<DescribeInstanceEventNotificationAttributesError>>> {
        (*self).describe_instance_event_notification_attributes(builder)
    }
    fn describe_instance_event_windows(&self, builder: DescribeInstanceEventWindowsInputBuilder) -> impl Future<Output = Result<DescribeInstanceEventWindowsOutput, SdkError<DescribeInstanceEventWindowsError>>> {
        (*self).describe_instance_event_windows(builder)
    }
    fn describe_instance_status(&self, builder: DescribeInstanceStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceStatusOutput, SdkError<DescribeInstanceStatusError>>> {
        (*self).describe_instance_status(builder)
    }
    fn describe_instance_topology(&self, builder: DescribeInstanceTopologyInputBuilder) -> impl Future<Output = Result<DescribeInstanceTopologyOutput, SdkError<DescribeInstanceTopologyError>>> {
        (*self).describe_instance_topology(builder)
    }
    fn describe_instance_type_offerings(&self, builder: DescribeInstanceTypeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypeOfferingsOutput, SdkError<DescribeInstanceTypeOfferingsError>>> {
        (*self).describe_instance_type_offerings(builder)
    }
    fn describe_instance_types(&self, builder: DescribeInstanceTypesInputBuilder) -> impl Future<Output = Result<DescribeInstanceTypesOutput, SdkError<DescribeInstanceTypesError>>> {
        (*self).describe_instance_types(builder)
    }
    fn describe_instances(&self, builder: DescribeInstancesInputBuilder) -> impl Future<Output = Result<DescribeInstancesOutput, SdkError<DescribeInstancesError>>> {
        (*self).describe_instances(builder)
    }
    fn describe_internet_gateways(&self, builder: DescribeInternetGatewaysInputBuilder) -> impl Future<Output = Result<DescribeInternetGatewaysOutput, SdkError<DescribeInternetGatewaysError>>> {
        (*self).describe_internet_gateways(builder)
    }
    fn describe_ipam_byoasn(&self, builder: DescribeIpamByoasnInputBuilder) -> impl Future<Output = Result<DescribeIpamByoasnOutput, SdkError<DescribeIpamByoasnError>>> {
        (*self).describe_ipam_byoasn(builder)
    }
    fn describe_ipam_external_resource_verification_tokens(&self, builder: DescribeIpamExternalResourceVerificationTokensInputBuilder) -> impl Future<Output = Result<DescribeIpamExternalResourceVerificationTokensOutput, SdkError<DescribeIpamExternalResourceVerificationTokensError>>> {
        (*self).describe_ipam_external_resource_verification_tokens(builder)
    }
    fn describe_ipam_pools(&self, builder: DescribeIpamPoolsInputBuilder) -> impl Future<Output = Result<DescribeIpamPoolsOutput, SdkError<DescribeIpamPoolsError>>> {
        (*self).describe_ipam_pools(builder)
    }
    fn describe_ipam_resource_discoveries(&self, builder: DescribeIpamResourceDiscoveriesInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveriesOutput, SdkError<DescribeIpamResourceDiscoveriesError>>> {
        (*self).describe_ipam_resource_discoveries(builder)
    }
    fn describe_ipam_resource_discovery_associations(&self, builder: DescribeIpamResourceDiscoveryAssociationsInputBuilder) -> impl Future<Output = Result<DescribeIpamResourceDiscoveryAssociationsOutput, SdkError<DescribeIpamResourceDiscoveryAssociationsError>>> {
        (*self).describe_ipam_resource_discovery_associations(builder)
    }
    fn describe_ipam_scopes(&self, builder: DescribeIpamScopesInputBuilder) -> impl Future<Output = Result<DescribeIpamScopesOutput, SdkError<DescribeIpamScopesError>>> {
        (*self).describe_ipam_scopes(builder)
    }
    fn describe_ipams(&self, builder: DescribeIpamsInputBuilder) -> impl Future<Output = Result<DescribeIpamsOutput, SdkError<DescribeIpamsError>>> {
        (*self).describe_ipams(builder)
    }
    fn describe_ipv6_pools(&self, builder: DescribeIpv6PoolsInputBuilder) -> impl Future<Output = Result<DescribeIpv6PoolsOutput, SdkError<DescribeIpv6PoolsError>>> {
        (*self).describe_ipv6_pools(builder)
    }
    fn describe_key_pairs(&self, builder: DescribeKeyPairsInputBuilder) -> impl Future<Output = Result<DescribeKeyPairsOutput, SdkError<DescribeKeyPairsError>>> {
        (*self).describe_key_pairs(builder)
    }
    fn describe_launch_template_versions(&self, builder: DescribeLaunchTemplateVersionsInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplateVersionsOutput, SdkError<DescribeLaunchTemplateVersionsError>>> {
        (*self).describe_launch_template_versions(builder)
    }
    fn describe_launch_templates(&self, builder: DescribeLaunchTemplatesInputBuilder) -> impl Future<Output = Result<DescribeLaunchTemplatesOutput, SdkError<DescribeLaunchTemplatesError>>> {
        (*self).describe_launch_templates(builder)
    }
    fn describe_local_gateway_route_table_virtual_interface_group_associations(&self, builder: DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsError>>> {
        (*self).describe_local_gateway_route_table_virtual_interface_group_associations(builder)
    }
    fn describe_local_gateway_route_table_vpc_associations(&self, builder: DescribeLocalGatewayRouteTableVpcAssociationsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTableVpcAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVpcAssociationsError>>> {
        (*self).describe_local_gateway_route_table_vpc_associations(builder)
    }
    fn describe_local_gateway_route_tables(&self, builder: DescribeLocalGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayRouteTablesOutput, SdkError<DescribeLocalGatewayRouteTablesError>>> {
        (*self).describe_local_gateway_route_tables(builder)
    }
    fn describe_local_gateway_virtual_interface_groups(&self, builder: DescribeLocalGatewayVirtualInterfaceGroupsInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfaceGroupsOutput, SdkError<DescribeLocalGatewayVirtualInterfaceGroupsError>>> {
        (*self).describe_local_gateway_virtual_interface_groups(builder)
    }
    fn describe_local_gateway_virtual_interfaces(&self, builder: DescribeLocalGatewayVirtualInterfacesInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewayVirtualInterfacesOutput, SdkError<DescribeLocalGatewayVirtualInterfacesError>>> {
        (*self).describe_local_gateway_virtual_interfaces(builder)
    }
    fn describe_local_gateways(&self, builder: DescribeLocalGatewaysInputBuilder) -> impl Future<Output = Result<DescribeLocalGatewaysOutput, SdkError<DescribeLocalGatewaysError>>> {
        (*self).describe_local_gateways(builder)
    }
    fn describe_locked_snapshots(&self, builder: DescribeLockedSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeLockedSnapshotsOutput, SdkError<DescribeLockedSnapshotsError>>> {
        (*self).describe_locked_snapshots(builder)
    }
    fn describe_mac_hosts(&self, builder: DescribeMacHostsInputBuilder) -> impl Future<Output = Result<DescribeMacHostsOutput, SdkError<DescribeMacHostsError>>> {
        (*self).describe_mac_hosts(builder)
    }
    fn describe_managed_prefix_lists(&self, builder: DescribeManagedPrefixListsInputBuilder) -> impl Future<Output = Result<DescribeManagedPrefixListsOutput, SdkError<DescribeManagedPrefixListsError>>> {
        (*self).describe_managed_prefix_lists(builder)
    }
    fn describe_moving_addresses(&self, builder: DescribeMovingAddressesInputBuilder) -> impl Future<Output = Result<DescribeMovingAddressesOutput, SdkError<DescribeMovingAddressesError>>> {
        (*self).describe_moving_addresses(builder)
    }
    fn describe_nat_gateways(&self, builder: DescribeNatGatewaysInputBuilder) -> impl Future<Output = Result<DescribeNatGatewaysOutput, SdkError<DescribeNatGatewaysError>>> {
        (*self).describe_nat_gateways(builder)
    }
    fn describe_network_acls(&self, builder: DescribeNetworkAclsInputBuilder) -> impl Future<Output = Result<DescribeNetworkAclsOutput, SdkError<DescribeNetworkAclsError>>> {
        (*self).describe_network_acls(builder)
    }
    fn describe_network_insights_access_scope_analyses(&self, builder: DescribeNetworkInsightsAccessScopeAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopeAnalysesOutput, SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>>> {
        (*self).describe_network_insights_access_scope_analyses(builder)
    }
    fn describe_network_insights_access_scopes(&self, builder: DescribeNetworkInsightsAccessScopesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAccessScopesOutput, SdkError<DescribeNetworkInsightsAccessScopesError>>> {
        (*self).describe_network_insights_access_scopes(builder)
    }
    fn describe_network_insights_analyses(&self, builder: DescribeNetworkInsightsAnalysesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsAnalysesOutput, SdkError<DescribeNetworkInsightsAnalysesError>>> {
        (*self).describe_network_insights_analyses(builder)
    }
    fn describe_network_insights_paths(&self, builder: DescribeNetworkInsightsPathsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInsightsPathsOutput, SdkError<DescribeNetworkInsightsPathsError>>> {
        (*self).describe_network_insights_paths(builder)
    }
    fn describe_network_interface_attribute(&self, builder: DescribeNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfaceAttributeOutput, SdkError<DescribeNetworkInterfaceAttributeError>>> {
        (*self).describe_network_interface_attribute(builder)
    }
    fn describe_network_interface_permissions(&self, builder: DescribeNetworkInterfacePermissionsInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacePermissionsOutput, SdkError<DescribeNetworkInterfacePermissionsError>>> {
        (*self).describe_network_interface_permissions(builder)
    }
    fn describe_network_interfaces(&self, builder: DescribeNetworkInterfacesInputBuilder) -> impl Future<Output = Result<DescribeNetworkInterfacesOutput, SdkError<DescribeNetworkInterfacesError>>> {
        (*self).describe_network_interfaces(builder)
    }
    fn describe_placement_groups(&self, builder: DescribePlacementGroupsInputBuilder) -> impl Future<Output = Result<DescribePlacementGroupsOutput, SdkError<DescribePlacementGroupsError>>> {
        (*self).describe_placement_groups(builder)
    }
    fn describe_prefix_lists(&self, builder: DescribePrefixListsInputBuilder) -> impl Future<Output = Result<DescribePrefixListsOutput, SdkError<DescribePrefixListsError>>> {
        (*self).describe_prefix_lists(builder)
    }
    fn describe_principal_id_format(&self, builder: DescribePrincipalIdFormatInputBuilder) -> impl Future<Output = Result<DescribePrincipalIdFormatOutput, SdkError<DescribePrincipalIdFormatError>>> {
        (*self).describe_principal_id_format(builder)
    }
    fn describe_public_ipv4_pools(&self, builder: DescribePublicIpv4PoolsInputBuilder) -> impl Future<Output = Result<DescribePublicIpv4PoolsOutput, SdkError<DescribePublicIpv4PoolsError>>> {
        (*self).describe_public_ipv4_pools(builder)
    }
    fn describe_regions(&self, builder: DescribeRegionsInputBuilder) -> impl Future<Output = Result<DescribeRegionsOutput, SdkError<DescribeRegionsError>>> {
        (*self).describe_regions(builder)
    }
    fn describe_replace_root_volume_tasks(&self, builder: DescribeReplaceRootVolumeTasksInputBuilder) -> impl Future<Output = Result<DescribeReplaceRootVolumeTasksOutput, SdkError<DescribeReplaceRootVolumeTasksError>>> {
        (*self).describe_replace_root_volume_tasks(builder)
    }
    fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>> {
        (*self).describe_reserved_instances(builder)
    }
    fn describe_reserved_instances_listings(&self, builder: DescribeReservedInstancesListingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesListingsOutput, SdkError<DescribeReservedInstancesListingsError>>> {
        (*self).describe_reserved_instances_listings(builder)
    }
    fn describe_reserved_instances_modifications(&self, builder: DescribeReservedInstancesModificationsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesModificationsOutput, SdkError<DescribeReservedInstancesModificationsError>>> {
        (*self).describe_reserved_instances_modifications(builder)
    }
    fn describe_reserved_instances_offerings(&self, builder: DescribeReservedInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedInstancesOfferingsOutput, SdkError<DescribeReservedInstancesOfferingsError>>> {
        (*self).describe_reserved_instances_offerings(builder)
    }
    fn describe_route_tables(&self, builder: DescribeRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeRouteTablesOutput, SdkError<DescribeRouteTablesError>>> {
        (*self).describe_route_tables(builder)
    }
    fn describe_scheduled_instance_availability(&self, builder: DescribeScheduledInstanceAvailabilityInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstanceAvailabilityOutput, SdkError<DescribeScheduledInstanceAvailabilityError>>> {
        (*self).describe_scheduled_instance_availability(builder)
    }
    fn describe_scheduled_instances(&self, builder: DescribeScheduledInstancesInputBuilder) -> impl Future<Output = Result<DescribeScheduledInstancesOutput, SdkError<DescribeScheduledInstancesError>>> {
        (*self).describe_scheduled_instances(builder)
    }
    fn describe_security_group_references(&self, builder: DescribeSecurityGroupReferencesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupReferencesOutput, SdkError<DescribeSecurityGroupReferencesError>>> {
        (*self).describe_security_group_references(builder)
    }
    fn describe_security_group_rules(&self, builder: DescribeSecurityGroupRulesInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupRulesOutput, SdkError<DescribeSecurityGroupRulesError>>> {
        (*self).describe_security_group_rules(builder)
    }
    fn describe_security_groups(&self, builder: DescribeSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeSecurityGroupsOutput, SdkError<DescribeSecurityGroupsError>>> {
        (*self).describe_security_groups(builder)
    }
    fn describe_snapshot_attribute(&self, builder: DescribeSnapshotAttributeInputBuilder) -> impl Future<Output = Result<DescribeSnapshotAttributeOutput, SdkError<DescribeSnapshotAttributeError>>> {
        (*self).describe_snapshot_attribute(builder)
    }
    fn describe_snapshot_tier_status(&self, builder: DescribeSnapshotTierStatusInputBuilder) -> impl Future<Output = Result<DescribeSnapshotTierStatusOutput, SdkError<DescribeSnapshotTierStatusError>>> {
        (*self).describe_snapshot_tier_status(builder)
    }
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>> {
        (*self).describe_snapshots(builder)
    }
    fn describe_spot_datafeed_subscription(&self, builder: DescribeSpotDatafeedSubscriptionInputBuilder) -> impl Future<Output = Result<DescribeSpotDatafeedSubscriptionOutput, SdkError<DescribeSpotDatafeedSubscriptionError>>> {
        (*self).describe_spot_datafeed_subscription(builder)
    }
    fn describe_spot_fleet_instances(&self, builder: DescribeSpotFleetInstancesInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetInstancesOutput, SdkError<DescribeSpotFleetInstancesError>>> {
        (*self).describe_spot_fleet_instances(builder)
    }
    fn describe_spot_fleet_request_history(&self, builder: DescribeSpotFleetRequestHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestHistoryOutput, SdkError<DescribeSpotFleetRequestHistoryError>>> {
        (*self).describe_spot_fleet_request_history(builder)
    }
    fn describe_spot_fleet_requests(&self, builder: DescribeSpotFleetRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotFleetRequestsOutput, SdkError<DescribeSpotFleetRequestsError>>> {
        (*self).describe_spot_fleet_requests(builder)
    }
    fn describe_spot_instance_requests(&self, builder: DescribeSpotInstanceRequestsInputBuilder) -> impl Future<Output = Result<DescribeSpotInstanceRequestsOutput, SdkError<DescribeSpotInstanceRequestsError>>> {
        (*self).describe_spot_instance_requests(builder)
    }
    fn describe_spot_price_history(&self, builder: DescribeSpotPriceHistoryInputBuilder) -> impl Future<Output = Result<DescribeSpotPriceHistoryOutput, SdkError<DescribeSpotPriceHistoryError>>> {
        (*self).describe_spot_price_history(builder)
    }
    fn describe_stale_security_groups(&self, builder: DescribeStaleSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeStaleSecurityGroupsOutput, SdkError<DescribeStaleSecurityGroupsError>>> {
        (*self).describe_stale_security_groups(builder)
    }
    fn describe_store_image_tasks(&self, builder: DescribeStoreImageTasksInputBuilder) -> impl Future<Output = Result<DescribeStoreImageTasksOutput, SdkError<DescribeStoreImageTasksError>>> {
        (*self).describe_store_image_tasks(builder)
    }
    fn describe_subnets(&self, builder: DescribeSubnetsInputBuilder) -> impl Future<Output = Result<DescribeSubnetsOutput, SdkError<DescribeSubnetsError>>> {
        (*self).describe_subnets(builder)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        (*self).describe_tags(builder)
    }
    fn describe_traffic_mirror_filter_rules(&self, builder: DescribeTrafficMirrorFilterRulesInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFilterRulesOutput, SdkError<DescribeTrafficMirrorFilterRulesError>>> {
        (*self).describe_traffic_mirror_filter_rules(builder)
    }
    fn describe_traffic_mirror_filters(&self, builder: DescribeTrafficMirrorFiltersInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorFiltersOutput, SdkError<DescribeTrafficMirrorFiltersError>>> {
        (*self).describe_traffic_mirror_filters(builder)
    }
    fn describe_traffic_mirror_sessions(&self, builder: DescribeTrafficMirrorSessionsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorSessionsOutput, SdkError<DescribeTrafficMirrorSessionsError>>> {
        (*self).describe_traffic_mirror_sessions(builder)
    }
    fn describe_traffic_mirror_targets(&self, builder: DescribeTrafficMirrorTargetsInputBuilder) -> impl Future<Output = Result<DescribeTrafficMirrorTargetsOutput, SdkError<DescribeTrafficMirrorTargetsError>>> {
        (*self).describe_traffic_mirror_targets(builder)
    }
    fn describe_transit_gateway_attachments(&self, builder: DescribeTransitGatewayAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayAttachmentsOutput, SdkError<DescribeTransitGatewayAttachmentsError>>> {
        (*self).describe_transit_gateway_attachments(builder)
    }
    fn describe_transit_gateway_connect_peers(&self, builder: DescribeTransitGatewayConnectPeersInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectPeersOutput, SdkError<DescribeTransitGatewayConnectPeersError>>> {
        (*self).describe_transit_gateway_connect_peers(builder)
    }
    fn describe_transit_gateway_connects(&self, builder: DescribeTransitGatewayConnectsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayConnectsOutput, SdkError<DescribeTransitGatewayConnectsError>>> {
        (*self).describe_transit_gateway_connects(builder)
    }
    fn describe_transit_gateway_multicast_domains(&self, builder: DescribeTransitGatewayMulticastDomainsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayMulticastDomainsOutput, SdkError<DescribeTransitGatewayMulticastDomainsError>>> {
        (*self).describe_transit_gateway_multicast_domains(builder)
    }
    fn describe_transit_gateway_peering_attachments(&self, builder: DescribeTransitGatewayPeeringAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPeeringAttachmentsOutput, SdkError<DescribeTransitGatewayPeeringAttachmentsError>>> {
        (*self).describe_transit_gateway_peering_attachments(builder)
    }
    fn describe_transit_gateway_policy_tables(&self, builder: DescribeTransitGatewayPolicyTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayPolicyTablesOutput, SdkError<DescribeTransitGatewayPolicyTablesError>>> {
        (*self).describe_transit_gateway_policy_tables(builder)
    }
    fn describe_transit_gateway_route_table_announcements(&self, builder: DescribeTransitGatewayRouteTableAnnouncementsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTableAnnouncementsOutput, SdkError<DescribeTransitGatewayRouteTableAnnouncementsError>>> {
        (*self).describe_transit_gateway_route_table_announcements(builder)
    }
    fn describe_transit_gateway_route_tables(&self, builder: DescribeTransitGatewayRouteTablesInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayRouteTablesOutput, SdkError<DescribeTransitGatewayRouteTablesError>>> {
        (*self).describe_transit_gateway_route_tables(builder)
    }
    fn describe_transit_gateway_vpc_attachments(&self, builder: DescribeTransitGatewayVpcAttachmentsInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewayVpcAttachmentsOutput, SdkError<DescribeTransitGatewayVpcAttachmentsError>>> {
        (*self).describe_transit_gateway_vpc_attachments(builder)
    }
    fn describe_transit_gateways(&self, builder: DescribeTransitGatewaysInputBuilder) -> impl Future<Output = Result<DescribeTransitGatewaysOutput, SdkError<DescribeTransitGatewaysError>>> {
        (*self).describe_transit_gateways(builder)
    }
    fn describe_trunk_interface_associations(&self, builder: DescribeTrunkInterfaceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeTrunkInterfaceAssociationsOutput, SdkError<DescribeTrunkInterfaceAssociationsError>>> {
        (*self).describe_trunk_interface_associations(builder)
    }
    fn describe_verified_access_endpoints(&self, builder: DescribeVerifiedAccessEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessEndpointsOutput, SdkError<DescribeVerifiedAccessEndpointsError>>> {
        (*self).describe_verified_access_endpoints(builder)
    }
    fn describe_verified_access_groups(&self, builder: DescribeVerifiedAccessGroupsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessGroupsOutput, SdkError<DescribeVerifiedAccessGroupsError>>> {
        (*self).describe_verified_access_groups(builder)
    }
    fn describe_verified_access_instance_logging_configurations(&self, builder: DescribeVerifiedAccessInstanceLoggingConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstanceLoggingConfigurationsOutput, SdkError<DescribeVerifiedAccessInstanceLoggingConfigurationsError>>> {
        (*self).describe_verified_access_instance_logging_configurations(builder)
    }
    fn describe_verified_access_instances(&self, builder: DescribeVerifiedAccessInstancesInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessInstancesOutput, SdkError<DescribeVerifiedAccessInstancesError>>> {
        (*self).describe_verified_access_instances(builder)
    }
    fn describe_verified_access_trust_providers(&self, builder: DescribeVerifiedAccessTrustProvidersInputBuilder) -> impl Future<Output = Result<DescribeVerifiedAccessTrustProvidersOutput, SdkError<DescribeVerifiedAccessTrustProvidersError>>> {
        (*self).describe_verified_access_trust_providers(builder)
    }
    fn describe_volume_attribute(&self, builder: DescribeVolumeAttributeInputBuilder) -> impl Future<Output = Result<DescribeVolumeAttributeOutput, SdkError<DescribeVolumeAttributeError>>> {
        (*self).describe_volume_attribute(builder)
    }
    fn describe_volume_status(&self, builder: DescribeVolumeStatusInputBuilder) -> impl Future<Output = Result<DescribeVolumeStatusOutput, SdkError<DescribeVolumeStatusError>>> {
        (*self).describe_volume_status(builder)
    }
    fn describe_volumes(&self, builder: DescribeVolumesInputBuilder) -> impl Future<Output = Result<DescribeVolumesOutput, SdkError<DescribeVolumesError>>> {
        (*self).describe_volumes(builder)
    }
    fn describe_volumes_modifications(&self, builder: DescribeVolumesModificationsInputBuilder) -> impl Future<Output = Result<DescribeVolumesModificationsOutput, SdkError<DescribeVolumesModificationsError>>> {
        (*self).describe_volumes_modifications(builder)
    }
    fn describe_vpc_attribute(&self, builder: DescribeVpcAttributeInputBuilder) -> impl Future<Output = Result<DescribeVpcAttributeOutput, SdkError<DescribeVpcAttributeError>>> {
        (*self).describe_vpc_attribute(builder)
    }
    fn describe_vpc_classic_link(&self, builder: DescribeVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkOutput, SdkError<DescribeVpcClassicLinkError>>> {
        (*self).describe_vpc_classic_link(builder)
    }
    fn describe_vpc_classic_link_dns_support(&self, builder: DescribeVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DescribeVpcClassicLinkDnsSupportOutput, SdkError<DescribeVpcClassicLinkDnsSupportError>>> {
        (*self).describe_vpc_classic_link_dns_support(builder)
    }
    fn describe_vpc_endpoint_connection_notifications(&self, builder: DescribeVpcEndpointConnectionNotificationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionNotificationsOutput, SdkError<DescribeVpcEndpointConnectionNotificationsError>>> {
        (*self).describe_vpc_endpoint_connection_notifications(builder)
    }
    fn describe_vpc_endpoint_connections(&self, builder: DescribeVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointConnectionsOutput, SdkError<DescribeVpcEndpointConnectionsError>>> {
        (*self).describe_vpc_endpoint_connections(builder)
    }
    fn describe_vpc_endpoint_service_configurations(&self, builder: DescribeVpcEndpointServiceConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServiceConfigurationsOutput, SdkError<DescribeVpcEndpointServiceConfigurationsError>>> {
        (*self).describe_vpc_endpoint_service_configurations(builder)
    }
    fn describe_vpc_endpoint_service_permissions(&self, builder: DescribeVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicePermissionsOutput, SdkError<DescribeVpcEndpointServicePermissionsError>>> {
        (*self).describe_vpc_endpoint_service_permissions(builder)
    }
    fn describe_vpc_endpoint_services(&self, builder: DescribeVpcEndpointServicesInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointServicesOutput, SdkError<DescribeVpcEndpointServicesError>>> {
        (*self).describe_vpc_endpoint_services(builder)
    }
    fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> impl Future<Output = Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>> {
        (*self).describe_vpc_endpoints(builder)
    }
    fn describe_vpc_peering_connections(&self, builder: DescribeVpcPeeringConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpcPeeringConnectionsOutput, SdkError<DescribeVpcPeeringConnectionsError>>> {
        (*self).describe_vpc_peering_connections(builder)
    }
    fn describe_vpcs(&self, builder: DescribeVpcsInputBuilder) -> impl Future<Output = Result<DescribeVpcsOutput, SdkError<DescribeVpcsError>>> {
        (*self).describe_vpcs(builder)
    }
    fn describe_vpn_connections(&self, builder: DescribeVpnConnectionsInputBuilder) -> impl Future<Output = Result<DescribeVpnConnectionsOutput, SdkError<DescribeVpnConnectionsError>>> {
        (*self).describe_vpn_connections(builder)
    }
    fn describe_vpn_gateways(&self, builder: DescribeVpnGatewaysInputBuilder) -> impl Future<Output = Result<DescribeVpnGatewaysOutput, SdkError<DescribeVpnGatewaysError>>> {
        (*self).describe_vpn_gateways(builder)
    }
    fn detach_classic_link_vpc(&self, builder: DetachClassicLinkVpcInputBuilder) -> impl Future<Output = Result<DetachClassicLinkVpcOutput, SdkError<DetachClassicLinkVpcError>>> {
        (*self).detach_classic_link_vpc(builder)
    }
    fn detach_internet_gateway(&self, builder: DetachInternetGatewayInputBuilder) -> impl Future<Output = Result<DetachInternetGatewayOutput, SdkError<DetachInternetGatewayError>>> {
        (*self).detach_internet_gateway(builder)
    }
    fn detach_network_interface(&self, builder: DetachNetworkInterfaceInputBuilder) -> impl Future<Output = Result<DetachNetworkInterfaceOutput, SdkError<DetachNetworkInterfaceError>>> {
        (*self).detach_network_interface(builder)
    }
    fn detach_verified_access_trust_provider(&self, builder: DetachVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<DetachVerifiedAccessTrustProviderOutput, SdkError<DetachVerifiedAccessTrustProviderError>>> {
        (*self).detach_verified_access_trust_provider(builder)
    }
    fn detach_volume(&self, builder: DetachVolumeInputBuilder) -> impl Future<Output = Result<DetachVolumeOutput, SdkError<DetachVolumeError>>> {
        (*self).detach_volume(builder)
    }
    fn detach_vpn_gateway(&self, builder: DetachVpnGatewayInputBuilder) -> impl Future<Output = Result<DetachVpnGatewayOutput, SdkError<DetachVpnGatewayError>>> {
        (*self).detach_vpn_gateway(builder)
    }
    fn disable_address_transfer(&self, builder: DisableAddressTransferInputBuilder) -> impl Future<Output = Result<DisableAddressTransferOutput, SdkError<DisableAddressTransferError>>> {
        (*self).disable_address_transfer(builder)
    }
    fn disable_aws_network_performance_metric_subscription(&self, builder: DisableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<DisableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<DisableAwsNetworkPerformanceMetricSubscriptionError>>> {
        (*self).disable_aws_network_performance_metric_subscription(builder)
    }
    fn disable_ebs_encryption_by_default(&self, builder: DisableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<DisableEbsEncryptionByDefaultOutput, SdkError<DisableEbsEncryptionByDefaultError>>> {
        (*self).disable_ebs_encryption_by_default(builder)
    }
    fn disable_fast_launch(&self, builder: DisableFastLaunchInputBuilder) -> impl Future<Output = Result<DisableFastLaunchOutput, SdkError<DisableFastLaunchError>>> {
        (*self).disable_fast_launch(builder)
    }
    fn disable_fast_snapshot_restores(&self, builder: DisableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<DisableFastSnapshotRestoresOutput, SdkError<DisableFastSnapshotRestoresError>>> {
        (*self).disable_fast_snapshot_restores(builder)
    }
    fn disable_image(&self, builder: DisableImageInputBuilder) -> impl Future<Output = Result<DisableImageOutput, SdkError<DisableImageError>>> {
        (*self).disable_image(builder)
    }
    fn disable_image_block_public_access(&self, builder: DisableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableImageBlockPublicAccessOutput, SdkError<DisableImageBlockPublicAccessError>>> {
        (*self).disable_image_block_public_access(builder)
    }
    fn disable_image_deprecation(&self, builder: DisableImageDeprecationInputBuilder) -> impl Future<Output = Result<DisableImageDeprecationOutput, SdkError<DisableImageDeprecationError>>> {
        (*self).disable_image_deprecation(builder)
    }
    fn disable_image_deregistration_protection(&self, builder: DisableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<DisableImageDeregistrationProtectionOutput, SdkError<DisableImageDeregistrationProtectionError>>> {
        (*self).disable_image_deregistration_protection(builder)
    }
    fn disable_ipam_organization_admin_account(&self, builder: DisableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableIpamOrganizationAdminAccountOutput, SdkError<DisableIpamOrganizationAdminAccountError>>> {
        (*self).disable_ipam_organization_admin_account(builder)
    }
    fn disable_serial_console_access(&self, builder: DisableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<DisableSerialConsoleAccessOutput, SdkError<DisableSerialConsoleAccessError>>> {
        (*self).disable_serial_console_access(builder)
    }
    fn disable_snapshot_block_public_access(&self, builder: DisableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<DisableSnapshotBlockPublicAccessOutput, SdkError<DisableSnapshotBlockPublicAccessError>>> {
        (*self).disable_snapshot_block_public_access(builder)
    }
    fn disable_transit_gateway_route_table_propagation(&self, builder: DisableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<DisableTransitGatewayRouteTablePropagationOutput, SdkError<DisableTransitGatewayRouteTablePropagationError>>> {
        (*self).disable_transit_gateway_route_table_propagation(builder)
    }
    fn disable_vgw_route_propagation(&self, builder: DisableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<DisableVgwRoutePropagationOutput, SdkError<DisableVgwRoutePropagationError>>> {
        (*self).disable_vgw_route_propagation(builder)
    }
    fn disable_vpc_classic_link(&self, builder: DisableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkOutput, SdkError<DisableVpcClassicLinkError>>> {
        (*self).disable_vpc_classic_link(builder)
    }
    fn disable_vpc_classic_link_dns_support(&self, builder: DisableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<DisableVpcClassicLinkDnsSupportOutput, SdkError<DisableVpcClassicLinkDnsSupportError>>> {
        (*self).disable_vpc_classic_link_dns_support(builder)
    }
    fn disassociate_address(&self, builder: DisassociateAddressInputBuilder) -> impl Future<Output = Result<DisassociateAddressOutput, SdkError<DisassociateAddressError>>> {
        (*self).disassociate_address(builder)
    }
    fn disassociate_client_vpn_target_network(&self, builder: DisassociateClientVpnTargetNetworkInputBuilder) -> impl Future<Output = Result<DisassociateClientVpnTargetNetworkOutput, SdkError<DisassociateClientVpnTargetNetworkError>>> {
        (*self).disassociate_client_vpn_target_network(builder)
    }
    fn disassociate_enclave_certificate_iam_role(&self, builder: DisassociateEnclaveCertificateIamRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnclaveCertificateIamRoleOutput, SdkError<DisassociateEnclaveCertificateIamRoleError>>> {
        (*self).disassociate_enclave_certificate_iam_role(builder)
    }
    fn disassociate_iam_instance_profile(&self, builder: DisassociateIamInstanceProfileInputBuilder) -> impl Future<Output = Result<DisassociateIamInstanceProfileOutput, SdkError<DisassociateIamInstanceProfileError>>> {
        (*self).disassociate_iam_instance_profile(builder)
    }
    fn disassociate_instance_event_window(&self, builder: DisassociateInstanceEventWindowInputBuilder) -> impl Future<Output = Result<DisassociateInstanceEventWindowOutput, SdkError<DisassociateInstanceEventWindowError>>> {
        (*self).disassociate_instance_event_window(builder)
    }
    fn disassociate_ipam_byoasn(&self, builder: DisassociateIpamByoasnInputBuilder) -> impl Future<Output = Result<DisassociateIpamByoasnOutput, SdkError<DisassociateIpamByoasnError>>> {
        (*self).disassociate_ipam_byoasn(builder)
    }
    fn disassociate_ipam_resource_discovery(&self, builder: DisassociateIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<DisassociateIpamResourceDiscoveryOutput, SdkError<DisassociateIpamResourceDiscoveryError>>> {
        (*self).disassociate_ipam_resource_discovery(builder)
    }
    fn disassociate_nat_gateway_address(&self, builder: DisassociateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<DisassociateNatGatewayAddressOutput, SdkError<DisassociateNatGatewayAddressError>>> {
        (*self).disassociate_nat_gateway_address(builder)
    }
    fn disassociate_route_table(&self, builder: DisassociateRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateRouteTableOutput, SdkError<DisassociateRouteTableError>>> {
        (*self).disassociate_route_table(builder)
    }
    fn disassociate_subnet_cidr_block(&self, builder: DisassociateSubnetCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateSubnetCidrBlockOutput, SdkError<DisassociateSubnetCidrBlockError>>> {
        (*self).disassociate_subnet_cidr_block(builder)
    }
    fn disassociate_transit_gateway_multicast_domain(&self, builder: DisassociateTransitGatewayMulticastDomainInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayMulticastDomainOutput, SdkError<DisassociateTransitGatewayMulticastDomainError>>> {
        (*self).disassociate_transit_gateway_multicast_domain(builder)
    }
    fn disassociate_transit_gateway_policy_table(&self, builder: DisassociateTransitGatewayPolicyTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayPolicyTableOutput, SdkError<DisassociateTransitGatewayPolicyTableError>>> {
        (*self).disassociate_transit_gateway_policy_table(builder)
    }
    fn disassociate_transit_gateway_route_table(&self, builder: DisassociateTransitGatewayRouteTableInputBuilder) -> impl Future<Output = Result<DisassociateTransitGatewayRouteTableOutput, SdkError<DisassociateTransitGatewayRouteTableError>>> {
        (*self).disassociate_transit_gateway_route_table(builder)
    }
    fn disassociate_trunk_interface(&self, builder: DisassociateTrunkInterfaceInputBuilder) -> impl Future<Output = Result<DisassociateTrunkInterfaceOutput, SdkError<DisassociateTrunkInterfaceError>>> {
        (*self).disassociate_trunk_interface(builder)
    }
    fn disassociate_vpc_cidr_block(&self, builder: DisassociateVpcCidrBlockInputBuilder) -> impl Future<Output = Result<DisassociateVpcCidrBlockOutput, SdkError<DisassociateVpcCidrBlockError>>> {
        (*self).disassociate_vpc_cidr_block(builder)
    }
    fn enable_address_transfer(&self, builder: EnableAddressTransferInputBuilder) -> impl Future<Output = Result<EnableAddressTransferOutput, SdkError<EnableAddressTransferError>>> {
        (*self).enable_address_transfer(builder)
    }
    fn enable_aws_network_performance_metric_subscription(&self, builder: EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> impl Future<Output = Result<EnableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<EnableAwsNetworkPerformanceMetricSubscriptionError>>> {
        (*self).enable_aws_network_performance_metric_subscription(builder)
    }
    fn enable_ebs_encryption_by_default(&self, builder: EnableEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<EnableEbsEncryptionByDefaultOutput, SdkError<EnableEbsEncryptionByDefaultError>>> {
        (*self).enable_ebs_encryption_by_default(builder)
    }
    fn enable_fast_launch(&self, builder: EnableFastLaunchInputBuilder) -> impl Future<Output = Result<EnableFastLaunchOutput, SdkError<EnableFastLaunchError>>> {
        (*self).enable_fast_launch(builder)
    }
    fn enable_fast_snapshot_restores(&self, builder: EnableFastSnapshotRestoresInputBuilder) -> impl Future<Output = Result<EnableFastSnapshotRestoresOutput, SdkError<EnableFastSnapshotRestoresError>>> {
        (*self).enable_fast_snapshot_restores(builder)
    }
    fn enable_image(&self, builder: EnableImageInputBuilder) -> impl Future<Output = Result<EnableImageOutput, SdkError<EnableImageError>>> {
        (*self).enable_image(builder)
    }
    fn enable_image_block_public_access(&self, builder: EnableImageBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableImageBlockPublicAccessOutput, SdkError<EnableImageBlockPublicAccessError>>> {
        (*self).enable_image_block_public_access(builder)
    }
    fn enable_image_deprecation(&self, builder: EnableImageDeprecationInputBuilder) -> impl Future<Output = Result<EnableImageDeprecationOutput, SdkError<EnableImageDeprecationError>>> {
        (*self).enable_image_deprecation(builder)
    }
    fn enable_image_deregistration_protection(&self, builder: EnableImageDeregistrationProtectionInputBuilder) -> impl Future<Output = Result<EnableImageDeregistrationProtectionOutput, SdkError<EnableImageDeregistrationProtectionError>>> {
        (*self).enable_image_deregistration_protection(builder)
    }
    fn enable_ipam_organization_admin_account(&self, builder: EnableIpamOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableIpamOrganizationAdminAccountOutput, SdkError<EnableIpamOrganizationAdminAccountError>>> {
        (*self).enable_ipam_organization_admin_account(builder)
    }
    fn enable_reachability_analyzer_organization_sharing(&self, builder: EnableReachabilityAnalyzerOrganizationSharingInputBuilder) -> impl Future<Output = Result<EnableReachabilityAnalyzerOrganizationSharingOutput, SdkError<EnableReachabilityAnalyzerOrganizationSharingError>>> {
        (*self).enable_reachability_analyzer_organization_sharing(builder)
    }
    fn enable_serial_console_access(&self, builder: EnableSerialConsoleAccessInputBuilder) -> impl Future<Output = Result<EnableSerialConsoleAccessOutput, SdkError<EnableSerialConsoleAccessError>>> {
        (*self).enable_serial_console_access(builder)
    }
    fn enable_snapshot_block_public_access(&self, builder: EnableSnapshotBlockPublicAccessInputBuilder) -> impl Future<Output = Result<EnableSnapshotBlockPublicAccessOutput, SdkError<EnableSnapshotBlockPublicAccessError>>> {
        (*self).enable_snapshot_block_public_access(builder)
    }
    fn enable_transit_gateway_route_table_propagation(&self, builder: EnableTransitGatewayRouteTablePropagationInputBuilder) -> impl Future<Output = Result<EnableTransitGatewayRouteTablePropagationOutput, SdkError<EnableTransitGatewayRouteTablePropagationError>>> {
        (*self).enable_transit_gateway_route_table_propagation(builder)
    }
    fn enable_vgw_route_propagation(&self, builder: EnableVgwRoutePropagationInputBuilder) -> impl Future<Output = Result<EnableVgwRoutePropagationOutput, SdkError<EnableVgwRoutePropagationError>>> {
        (*self).enable_vgw_route_propagation(builder)
    }
    fn enable_volume_io(&self, builder: EnableVolumeIoInputBuilder) -> impl Future<Output = Result<EnableVolumeIoOutput, SdkError<EnableVolumeIOError>>> {
        (*self).enable_volume_io(builder)
    }
    fn enable_vpc_classic_link(&self, builder: EnableVpcClassicLinkInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkOutput, SdkError<EnableVpcClassicLinkError>>> {
        (*self).enable_vpc_classic_link(builder)
    }
    fn enable_vpc_classic_link_dns_support(&self, builder: EnableVpcClassicLinkDnsSupportInputBuilder) -> impl Future<Output = Result<EnableVpcClassicLinkDnsSupportOutput, SdkError<EnableVpcClassicLinkDnsSupportError>>> {
        (*self).enable_vpc_classic_link_dns_support(builder)
    }
    fn export_client_vpn_client_certificate_revocation_list(&self, builder: ExportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientCertificateRevocationListOutput, SdkError<ExportClientVpnClientCertificateRevocationListError>>> {
        (*self).export_client_vpn_client_certificate_revocation_list(builder)
    }
    fn export_client_vpn_client_configuration(&self, builder: ExportClientVpnClientConfigurationInputBuilder) -> impl Future<Output = Result<ExportClientVpnClientConfigurationOutput, SdkError<ExportClientVpnClientConfigurationError>>> {
        (*self).export_client_vpn_client_configuration(builder)
    }
    fn export_image(&self, builder: ExportImageInputBuilder) -> impl Future<Output = Result<ExportImageOutput, SdkError<ExportImageError>>> {
        (*self).export_image(builder)
    }
    fn export_transit_gateway_routes(&self, builder: ExportTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<ExportTransitGatewayRoutesOutput, SdkError<ExportTransitGatewayRoutesError>>> {
        (*self).export_transit_gateway_routes(builder)
    }
    fn get_associated_enclave_certificate_iam_roles(&self, builder: GetAssociatedEnclaveCertificateIamRolesInputBuilder) -> impl Future<Output = Result<GetAssociatedEnclaveCertificateIamRolesOutput, SdkError<GetAssociatedEnclaveCertificateIamRolesError>>> {
        (*self).get_associated_enclave_certificate_iam_roles(builder)
    }
    fn get_associated_ipv6_pool_cidrs(&self, builder: GetAssociatedIpv6PoolCidrsInputBuilder) -> impl Future<Output = Result<GetAssociatedIpv6PoolCidrsOutput, SdkError<GetAssociatedIpv6PoolCidrsError>>> {
        (*self).get_associated_ipv6_pool_cidrs(builder)
    }
    fn get_aws_network_performance_data(&self, builder: GetAwsNetworkPerformanceDataInputBuilder) -> impl Future<Output = Result<GetAwsNetworkPerformanceDataOutput, SdkError<GetAwsNetworkPerformanceDataError>>> {
        (*self).get_aws_network_performance_data(builder)
    }
    fn get_capacity_reservation_usage(&self, builder: GetCapacityReservationUsageInputBuilder) -> impl Future<Output = Result<GetCapacityReservationUsageOutput, SdkError<GetCapacityReservationUsageError>>> {
        (*self).get_capacity_reservation_usage(builder)
    }
    fn get_coip_pool_usage(&self, builder: GetCoipPoolUsageInputBuilder) -> impl Future<Output = Result<GetCoipPoolUsageOutput, SdkError<GetCoipPoolUsageError>>> {
        (*self).get_coip_pool_usage(builder)
    }
    fn get_console_output(&self, builder: GetConsoleOutputInputBuilder) -> impl Future<Output = Result<GetConsoleOutputOutput, SdkError<GetConsoleOutputError>>> {
        (*self).get_console_output(builder)
    }
    fn get_console_screenshot(&self, builder: GetConsoleScreenshotInputBuilder) -> impl Future<Output = Result<GetConsoleScreenshotOutput, SdkError<GetConsoleScreenshotError>>> {
        (*self).get_console_screenshot(builder)
    }
    fn get_default_credit_specification(&self, builder: GetDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<GetDefaultCreditSpecificationOutput, SdkError<GetDefaultCreditSpecificationError>>> {
        (*self).get_default_credit_specification(builder)
    }
    fn get_ebs_default_kms_key_id(&self, builder: GetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<GetEbsDefaultKmsKeyIdOutput, SdkError<GetEbsDefaultKmsKeyIdError>>> {
        (*self).get_ebs_default_kms_key_id(builder)
    }
    fn get_ebs_encryption_by_default(&self, builder: GetEbsEncryptionByDefaultInputBuilder) -> impl Future<Output = Result<GetEbsEncryptionByDefaultOutput, SdkError<GetEbsEncryptionByDefaultError>>> {
        (*self).get_ebs_encryption_by_default(builder)
    }
    fn get_flow_logs_integration_template(&self, builder: GetFlowLogsIntegrationTemplateInputBuilder) -> impl Future<Output = Result<GetFlowLogsIntegrationTemplateOutput, SdkError<GetFlowLogsIntegrationTemplateError>>> {
        (*self).get_flow_logs_integration_template(builder)
    }
    fn get_groups_for_capacity_reservation(&self, builder: GetGroupsForCapacityReservationInputBuilder) -> impl Future<Output = Result<GetGroupsForCapacityReservationOutput, SdkError<GetGroupsForCapacityReservationError>>> {
        (*self).get_groups_for_capacity_reservation(builder)
    }
    fn get_host_reservation_purchase_preview(&self, builder: GetHostReservationPurchasePreviewInputBuilder) -> impl Future<Output = Result<GetHostReservationPurchasePreviewOutput, SdkError<GetHostReservationPurchasePreviewError>>> {
        (*self).get_host_reservation_purchase_preview(builder)
    }
    fn get_image_block_public_access_state(&self, builder: GetImageBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetImageBlockPublicAccessStateOutput, SdkError<GetImageBlockPublicAccessStateError>>> {
        (*self).get_image_block_public_access_state(builder)
    }
    fn get_instance_metadata_defaults(&self, builder: GetInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<GetInstanceMetadataDefaultsOutput, SdkError<GetInstanceMetadataDefaultsError>>> {
        (*self).get_instance_metadata_defaults(builder)
    }
    fn get_instance_tpm_ek_pub(&self, builder: GetInstanceTpmEkPubInputBuilder) -> impl Future<Output = Result<GetInstanceTpmEkPubOutput, SdkError<GetInstanceTpmEkPubError>>> {
        (*self).get_instance_tpm_ek_pub(builder)
    }
    fn get_instance_types_from_instance_requirements(&self, builder: GetInstanceTypesFromInstanceRequirementsInputBuilder) -> impl Future<Output = Result<GetInstanceTypesFromInstanceRequirementsOutput, SdkError<GetInstanceTypesFromInstanceRequirementsError>>> {
        (*self).get_instance_types_from_instance_requirements(builder)
    }
    fn get_instance_uefi_data(&self, builder: GetInstanceUefiDataInputBuilder) -> impl Future<Output = Result<GetInstanceUefiDataOutput, SdkError<GetInstanceUefiDataError>>> {
        (*self).get_instance_uefi_data(builder)
    }
    fn get_ipam_address_history(&self, builder: GetIpamAddressHistoryInputBuilder) -> impl Future<Output = Result<GetIpamAddressHistoryOutput, SdkError<GetIpamAddressHistoryError>>> {
        (*self).get_ipam_address_history(builder)
    }
    fn get_ipam_discovered_accounts(&self, builder: GetIpamDiscoveredAccountsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredAccountsOutput, SdkError<GetIpamDiscoveredAccountsError>>> {
        (*self).get_ipam_discovered_accounts(builder)
    }
    fn get_ipam_discovered_public_addresses(&self, builder: GetIpamDiscoveredPublicAddressesInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredPublicAddressesOutput, SdkError<GetIpamDiscoveredPublicAddressesError>>> {
        (*self).get_ipam_discovered_public_addresses(builder)
    }
    fn get_ipam_discovered_resource_cidrs(&self, builder: GetIpamDiscoveredResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamDiscoveredResourceCidrsOutput, SdkError<GetIpamDiscoveredResourceCidrsError>>> {
        (*self).get_ipam_discovered_resource_cidrs(builder)
    }
    fn get_ipam_pool_allocations(&self, builder: GetIpamPoolAllocationsInputBuilder) -> impl Future<Output = Result<GetIpamPoolAllocationsOutput, SdkError<GetIpamPoolAllocationsError>>> {
        (*self).get_ipam_pool_allocations(builder)
    }
    fn get_ipam_pool_cidrs(&self, builder: GetIpamPoolCidrsInputBuilder) -> impl Future<Output = Result<GetIpamPoolCidrsOutput, SdkError<GetIpamPoolCidrsError>>> {
        (*self).get_ipam_pool_cidrs(builder)
    }
    fn get_ipam_resource_cidrs(&self, builder: GetIpamResourceCidrsInputBuilder) -> impl Future<Output = Result<GetIpamResourceCidrsOutput, SdkError<GetIpamResourceCidrsError>>> {
        (*self).get_ipam_resource_cidrs(builder)
    }
    fn get_launch_template_data(&self, builder: GetLaunchTemplateDataInputBuilder) -> impl Future<Output = Result<GetLaunchTemplateDataOutput, SdkError<GetLaunchTemplateDataError>>> {
        (*self).get_launch_template_data(builder)
    }
    fn get_managed_prefix_list_associations(&self, builder: GetManagedPrefixListAssociationsInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListAssociationsOutput, SdkError<GetManagedPrefixListAssociationsError>>> {
        (*self).get_managed_prefix_list_associations(builder)
    }
    fn get_managed_prefix_list_entries(&self, builder: GetManagedPrefixListEntriesInputBuilder) -> impl Future<Output = Result<GetManagedPrefixListEntriesOutput, SdkError<GetManagedPrefixListEntriesError>>> {
        (*self).get_managed_prefix_list_entries(builder)
    }
    fn get_network_insights_access_scope_analysis_findings(&self, builder: GetNetworkInsightsAccessScopeAnalysisFindingsInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeAnalysisFindingsOutput, SdkError<GetNetworkInsightsAccessScopeAnalysisFindingsError>>> {
        (*self).get_network_insights_access_scope_analysis_findings(builder)
    }
    fn get_network_insights_access_scope_content(&self, builder: GetNetworkInsightsAccessScopeContentInputBuilder) -> impl Future<Output = Result<GetNetworkInsightsAccessScopeContentOutput, SdkError<GetNetworkInsightsAccessScopeContentError>>> {
        (*self).get_network_insights_access_scope_content(builder)
    }
    fn get_password_data(&self, builder: GetPasswordDataInputBuilder) -> impl Future<Output = Result<GetPasswordDataOutput, SdkError<GetPasswordDataError>>> {
        (*self).get_password_data(builder)
    }
    fn get_reserved_instances_exchange_quote(&self, builder: GetReservedInstancesExchangeQuoteInputBuilder) -> impl Future<Output = Result<GetReservedInstancesExchangeQuoteOutput, SdkError<GetReservedInstancesExchangeQuoteError>>> {
        (*self).get_reserved_instances_exchange_quote(builder)
    }
    fn get_security_groups_for_vpc(&self, builder: GetSecurityGroupsForVpcInputBuilder) -> impl Future<Output = Result<GetSecurityGroupsForVpcOutput, SdkError<GetSecurityGroupsForVpcError>>> {
        (*self).get_security_groups_for_vpc(builder)
    }
    fn get_serial_console_access_status(&self, builder: GetSerialConsoleAccessStatusInputBuilder) -> impl Future<Output = Result<GetSerialConsoleAccessStatusOutput, SdkError<GetSerialConsoleAccessStatusError>>> {
        (*self).get_serial_console_access_status(builder)
    }
    fn get_snapshot_block_public_access_state(&self, builder: GetSnapshotBlockPublicAccessStateInputBuilder) -> impl Future<Output = Result<GetSnapshotBlockPublicAccessStateOutput, SdkError<GetSnapshotBlockPublicAccessStateError>>> {
        (*self).get_snapshot_block_public_access_state(builder)
    }
    fn get_spot_placement_scores(&self, builder: GetSpotPlacementScoresInputBuilder) -> impl Future<Output = Result<GetSpotPlacementScoresOutput, SdkError<GetSpotPlacementScoresError>>> {
        (*self).get_spot_placement_scores(builder)
    }
    fn get_subnet_cidr_reservations(&self, builder: GetSubnetCidrReservationsInputBuilder) -> impl Future<Output = Result<GetSubnetCidrReservationsOutput, SdkError<GetSubnetCidrReservationsError>>> {
        (*self).get_subnet_cidr_reservations(builder)
    }
    fn get_transit_gateway_attachment_propagations(&self, builder: GetTransitGatewayAttachmentPropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayAttachmentPropagationsOutput, SdkError<GetTransitGatewayAttachmentPropagationsError>>> {
        (*self).get_transit_gateway_attachment_propagations(builder)
    }
    fn get_transit_gateway_multicast_domain_associations(&self, builder: GetTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayMulticastDomainAssociationsOutput, SdkError<GetTransitGatewayMulticastDomainAssociationsError>>> {
        (*self).get_transit_gateway_multicast_domain_associations(builder)
    }
    fn get_transit_gateway_policy_table_associations(&self, builder: GetTransitGatewayPolicyTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableAssociationsOutput, SdkError<GetTransitGatewayPolicyTableAssociationsError>>> {
        (*self).get_transit_gateway_policy_table_associations(builder)
    }
    fn get_transit_gateway_policy_table_entries(&self, builder: GetTransitGatewayPolicyTableEntriesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPolicyTableEntriesOutput, SdkError<GetTransitGatewayPolicyTableEntriesError>>> {
        (*self).get_transit_gateway_policy_table_entries(builder)
    }
    fn get_transit_gateway_prefix_list_references(&self, builder: GetTransitGatewayPrefixListReferencesInputBuilder) -> impl Future<Output = Result<GetTransitGatewayPrefixListReferencesOutput, SdkError<GetTransitGatewayPrefixListReferencesError>>> {
        (*self).get_transit_gateway_prefix_list_references(builder)
    }
    fn get_transit_gateway_route_table_associations(&self, builder: GetTransitGatewayRouteTableAssociationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTableAssociationsOutput, SdkError<GetTransitGatewayRouteTableAssociationsError>>> {
        (*self).get_transit_gateway_route_table_associations(builder)
    }
    fn get_transit_gateway_route_table_propagations(&self, builder: GetTransitGatewayRouteTablePropagationsInputBuilder) -> impl Future<Output = Result<GetTransitGatewayRouteTablePropagationsOutput, SdkError<GetTransitGatewayRouteTablePropagationsError>>> {
        (*self).get_transit_gateway_route_table_propagations(builder)
    }
    fn get_verified_access_endpoint_policy(&self, builder: GetVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessEndpointPolicyOutput, SdkError<GetVerifiedAccessEndpointPolicyError>>> {
        (*self).get_verified_access_endpoint_policy(builder)
    }
    fn get_verified_access_group_policy(&self, builder: GetVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<GetVerifiedAccessGroupPolicyOutput, SdkError<GetVerifiedAccessGroupPolicyError>>> {
        (*self).get_verified_access_group_policy(builder)
    }
    fn get_vpn_connection_device_sample_configuration(&self, builder: GetVpnConnectionDeviceSampleConfigurationInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceSampleConfigurationOutput, SdkError<GetVpnConnectionDeviceSampleConfigurationError>>> {
        (*self).get_vpn_connection_device_sample_configuration(builder)
    }
    fn get_vpn_connection_device_types(&self, builder: GetVpnConnectionDeviceTypesInputBuilder) -> impl Future<Output = Result<GetVpnConnectionDeviceTypesOutput, SdkError<GetVpnConnectionDeviceTypesError>>> {
        (*self).get_vpn_connection_device_types(builder)
    }
    fn get_vpn_tunnel_replacement_status(&self, builder: GetVpnTunnelReplacementStatusInputBuilder) -> impl Future<Output = Result<GetVpnTunnelReplacementStatusOutput, SdkError<GetVpnTunnelReplacementStatusError>>> {
        (*self).get_vpn_tunnel_replacement_status(builder)
    }
    fn import_client_vpn_client_certificate_revocation_list(&self, builder: ImportClientVpnClientCertificateRevocationListInputBuilder) -> impl Future<Output = Result<ImportClientVpnClientCertificateRevocationListOutput, SdkError<ImportClientVpnClientCertificateRevocationListError>>> {
        (*self).import_client_vpn_client_certificate_revocation_list(builder)
    }
    fn import_image(&self, builder: ImportImageInputBuilder) -> impl Future<Output = Result<ImportImageOutput, SdkError<ImportImageError>>> {
        (*self).import_image(builder)
    }
    fn import_key_pair(&self, builder: ImportKeyPairInputBuilder) -> impl Future<Output = Result<ImportKeyPairOutput, SdkError<ImportKeyPairError>>> {
        (*self).import_key_pair(builder)
    }
    fn import_snapshot(&self, builder: ImportSnapshotInputBuilder) -> impl Future<Output = Result<ImportSnapshotOutput, SdkError<ImportSnapshotError>>> {
        (*self).import_snapshot(builder)
    }
    fn list_images_in_recycle_bin(&self, builder: ListImagesInRecycleBinInputBuilder) -> impl Future<Output = Result<ListImagesInRecycleBinOutput, SdkError<ListImagesInRecycleBinError>>> {
        (*self).list_images_in_recycle_bin(builder)
    }
    fn list_snapshots_in_recycle_bin(&self, builder: ListSnapshotsInRecycleBinInputBuilder) -> impl Future<Output = Result<ListSnapshotsInRecycleBinOutput, SdkError<ListSnapshotsInRecycleBinError>>> {
        (*self).list_snapshots_in_recycle_bin(builder)
    }
    fn lock_snapshot(&self, builder: LockSnapshotInputBuilder) -> impl Future<Output = Result<LockSnapshotOutput, SdkError<LockSnapshotError>>> {
        (*self).lock_snapshot(builder)
    }
    fn modify_address_attribute(&self, builder: ModifyAddressAttributeInputBuilder) -> impl Future<Output = Result<ModifyAddressAttributeOutput, SdkError<ModifyAddressAttributeError>>> {
        (*self).modify_address_attribute(builder)
    }
    fn modify_availability_zone_group(&self, builder: ModifyAvailabilityZoneGroupInputBuilder) -> impl Future<Output = Result<ModifyAvailabilityZoneGroupOutput, SdkError<ModifyAvailabilityZoneGroupError>>> {
        (*self).modify_availability_zone_group(builder)
    }
    fn modify_capacity_reservation(&self, builder: ModifyCapacityReservationInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationOutput, SdkError<ModifyCapacityReservationError>>> {
        (*self).modify_capacity_reservation(builder)
    }
    fn modify_capacity_reservation_fleet(&self, builder: ModifyCapacityReservationFleetInputBuilder) -> impl Future<Output = Result<ModifyCapacityReservationFleetOutput, SdkError<ModifyCapacityReservationFleetError>>> {
        (*self).modify_capacity_reservation_fleet(builder)
    }
    fn modify_client_vpn_endpoint(&self, builder: ModifyClientVpnEndpointInputBuilder) -> impl Future<Output = Result<ModifyClientVpnEndpointOutput, SdkError<ModifyClientVpnEndpointError>>> {
        (*self).modify_client_vpn_endpoint(builder)
    }
    fn modify_default_credit_specification(&self, builder: ModifyDefaultCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyDefaultCreditSpecificationOutput, SdkError<ModifyDefaultCreditSpecificationError>>> {
        (*self).modify_default_credit_specification(builder)
    }
    fn modify_ebs_default_kms_key_id(&self, builder: ModifyEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ModifyEbsDefaultKmsKeyIdOutput, SdkError<ModifyEbsDefaultKmsKeyIdError>>> {
        (*self).modify_ebs_default_kms_key_id(builder)
    }
    fn modify_fleet(&self, builder: ModifyFleetInputBuilder) -> impl Future<Output = Result<ModifyFleetOutput, SdkError<ModifyFleetError>>> {
        (*self).modify_fleet(builder)
    }
    fn modify_fpga_image_attribute(&self, builder: ModifyFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyFpgaImageAttributeOutput, SdkError<ModifyFpgaImageAttributeError>>> {
        (*self).modify_fpga_image_attribute(builder)
    }
    fn modify_hosts(&self, builder: ModifyHostsInputBuilder) -> impl Future<Output = Result<ModifyHostsOutput, SdkError<ModifyHostsError>>> {
        (*self).modify_hosts(builder)
    }
    fn modify_id_format(&self, builder: ModifyIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdFormatOutput, SdkError<ModifyIdFormatError>>> {
        (*self).modify_id_format(builder)
    }
    fn modify_identity_id_format(&self, builder: ModifyIdentityIdFormatInputBuilder) -> impl Future<Output = Result<ModifyIdentityIdFormatOutput, SdkError<ModifyIdentityIdFormatError>>> {
        (*self).modify_identity_id_format(builder)
    }
    fn modify_image_attribute(&self, builder: ModifyImageAttributeInputBuilder) -> impl Future<Output = Result<ModifyImageAttributeOutput, SdkError<ModifyImageAttributeError>>> {
        (*self).modify_image_attribute(builder)
    }
    fn modify_instance_attribute(&self, builder: ModifyInstanceAttributeInputBuilder) -> impl Future<Output = Result<ModifyInstanceAttributeOutput, SdkError<ModifyInstanceAttributeError>>> {
        (*self).modify_instance_attribute(builder)
    }
    fn modify_instance_capacity_reservation_attributes(&self, builder: ModifyInstanceCapacityReservationAttributesInputBuilder) -> impl Future<Output = Result<ModifyInstanceCapacityReservationAttributesOutput, SdkError<ModifyInstanceCapacityReservationAttributesError>>> {
        (*self).modify_instance_capacity_reservation_attributes(builder)
    }
    fn modify_instance_credit_specification(&self, builder: ModifyInstanceCreditSpecificationInputBuilder) -> impl Future<Output = Result<ModifyInstanceCreditSpecificationOutput, SdkError<ModifyInstanceCreditSpecificationError>>> {
        (*self).modify_instance_credit_specification(builder)
    }
    fn modify_instance_event_start_time(&self, builder: ModifyInstanceEventStartTimeInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventStartTimeOutput, SdkError<ModifyInstanceEventStartTimeError>>> {
        (*self).modify_instance_event_start_time(builder)
    }
    fn modify_instance_event_window(&self, builder: ModifyInstanceEventWindowInputBuilder) -> impl Future<Output = Result<ModifyInstanceEventWindowOutput, SdkError<ModifyInstanceEventWindowError>>> {
        (*self).modify_instance_event_window(builder)
    }
    fn modify_instance_maintenance_options(&self, builder: ModifyInstanceMaintenanceOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMaintenanceOptionsOutput, SdkError<ModifyInstanceMaintenanceOptionsError>>> {
        (*self).modify_instance_maintenance_options(builder)
    }
    fn modify_instance_metadata_defaults(&self, builder: ModifyInstanceMetadataDefaultsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataDefaultsOutput, SdkError<ModifyInstanceMetadataDefaultsError>>> {
        (*self).modify_instance_metadata_defaults(builder)
    }
    fn modify_instance_metadata_options(&self, builder: ModifyInstanceMetadataOptionsInputBuilder) -> impl Future<Output = Result<ModifyInstanceMetadataOptionsOutput, SdkError<ModifyInstanceMetadataOptionsError>>> {
        (*self).modify_instance_metadata_options(builder)
    }
    fn modify_instance_placement(&self, builder: ModifyInstancePlacementInputBuilder) -> impl Future<Output = Result<ModifyInstancePlacementOutput, SdkError<ModifyInstancePlacementError>>> {
        (*self).modify_instance_placement(builder)
    }
    fn modify_ipam(&self, builder: ModifyIpamInputBuilder) -> impl Future<Output = Result<ModifyIpamOutput, SdkError<ModifyIpamError>>> {
        (*self).modify_ipam(builder)
    }
    fn modify_ipam_pool(&self, builder: ModifyIpamPoolInputBuilder) -> impl Future<Output = Result<ModifyIpamPoolOutput, SdkError<ModifyIpamPoolError>>> {
        (*self).modify_ipam_pool(builder)
    }
    fn modify_ipam_resource_cidr(&self, builder: ModifyIpamResourceCidrInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceCidrOutput, SdkError<ModifyIpamResourceCidrError>>> {
        (*self).modify_ipam_resource_cidr(builder)
    }
    fn modify_ipam_resource_discovery(&self, builder: ModifyIpamResourceDiscoveryInputBuilder) -> impl Future<Output = Result<ModifyIpamResourceDiscoveryOutput, SdkError<ModifyIpamResourceDiscoveryError>>> {
        (*self).modify_ipam_resource_discovery(builder)
    }
    fn modify_ipam_scope(&self, builder: ModifyIpamScopeInputBuilder) -> impl Future<Output = Result<ModifyIpamScopeOutput, SdkError<ModifyIpamScopeError>>> {
        (*self).modify_ipam_scope(builder)
    }
    fn modify_launch_template(&self, builder: ModifyLaunchTemplateInputBuilder) -> impl Future<Output = Result<ModifyLaunchTemplateOutput, SdkError<ModifyLaunchTemplateError>>> {
        (*self).modify_launch_template(builder)
    }
    fn modify_local_gateway_route(&self, builder: ModifyLocalGatewayRouteInputBuilder) -> impl Future<Output = Result<ModifyLocalGatewayRouteOutput, SdkError<ModifyLocalGatewayRouteError>>> {
        (*self).modify_local_gateway_route(builder)
    }
    fn modify_managed_prefix_list(&self, builder: ModifyManagedPrefixListInputBuilder) -> impl Future<Output = Result<ModifyManagedPrefixListOutput, SdkError<ModifyManagedPrefixListError>>> {
        (*self).modify_managed_prefix_list(builder)
    }
    fn modify_network_interface_attribute(&self, builder: ModifyNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ModifyNetworkInterfaceAttributeOutput, SdkError<ModifyNetworkInterfaceAttributeError>>> {
        (*self).modify_network_interface_attribute(builder)
    }
    fn modify_private_dns_name_options(&self, builder: ModifyPrivateDnsNameOptionsInputBuilder) -> impl Future<Output = Result<ModifyPrivateDnsNameOptionsOutput, SdkError<ModifyPrivateDnsNameOptionsError>>> {
        (*self).modify_private_dns_name_options(builder)
    }
    fn modify_reserved_instances(&self, builder: ModifyReservedInstancesInputBuilder) -> impl Future<Output = Result<ModifyReservedInstancesOutput, SdkError<ModifyReservedInstancesError>>> {
        (*self).modify_reserved_instances(builder)
    }
    fn modify_security_group_rules(&self, builder: ModifySecurityGroupRulesInputBuilder) -> impl Future<Output = Result<ModifySecurityGroupRulesOutput, SdkError<ModifySecurityGroupRulesError>>> {
        (*self).modify_security_group_rules(builder)
    }
    fn modify_snapshot_attribute(&self, builder: ModifySnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifySnapshotAttributeOutput, SdkError<ModifySnapshotAttributeError>>> {
        (*self).modify_snapshot_attribute(builder)
    }
    fn modify_snapshot_tier(&self, builder: ModifySnapshotTierInputBuilder) -> impl Future<Output = Result<ModifySnapshotTierOutput, SdkError<ModifySnapshotTierError>>> {
        (*self).modify_snapshot_tier(builder)
    }
    fn modify_spot_fleet_request(&self, builder: ModifySpotFleetRequestInputBuilder) -> impl Future<Output = Result<ModifySpotFleetRequestOutput, SdkError<ModifySpotFleetRequestError>>> {
        (*self).modify_spot_fleet_request(builder)
    }
    fn modify_subnet_attribute(&self, builder: ModifySubnetAttributeInputBuilder) -> impl Future<Output = Result<ModifySubnetAttributeOutput, SdkError<ModifySubnetAttributeError>>> {
        (*self).modify_subnet_attribute(builder)
    }
    fn modify_traffic_mirror_filter_network_services(&self, builder: ModifyTrafficMirrorFilterNetworkServicesInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterNetworkServicesOutput, SdkError<ModifyTrafficMirrorFilterNetworkServicesError>>> {
        (*self).modify_traffic_mirror_filter_network_services(builder)
    }
    fn modify_traffic_mirror_filter_rule(&self, builder: ModifyTrafficMirrorFilterRuleInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorFilterRuleOutput, SdkError<ModifyTrafficMirrorFilterRuleError>>> {
        (*self).modify_traffic_mirror_filter_rule(builder)
    }
    fn modify_traffic_mirror_session(&self, builder: ModifyTrafficMirrorSessionInputBuilder) -> impl Future<Output = Result<ModifyTrafficMirrorSessionOutput, SdkError<ModifyTrafficMirrorSessionError>>> {
        (*self).modify_traffic_mirror_session(builder)
    }
    fn modify_transit_gateway(&self, builder: ModifyTransitGatewayInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayOutput, SdkError<ModifyTransitGatewayError>>> {
        (*self).modify_transit_gateway(builder)
    }
    fn modify_transit_gateway_prefix_list_reference(&self, builder: ModifyTransitGatewayPrefixListReferenceInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayPrefixListReferenceOutput, SdkError<ModifyTransitGatewayPrefixListReferenceError>>> {
        (*self).modify_transit_gateway_prefix_list_reference(builder)
    }
    fn modify_transit_gateway_vpc_attachment(&self, builder: ModifyTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<ModifyTransitGatewayVpcAttachmentOutput, SdkError<ModifyTransitGatewayVpcAttachmentError>>> {
        (*self).modify_transit_gateway_vpc_attachment(builder)
    }
    fn modify_verified_access_endpoint(&self, builder: ModifyVerifiedAccessEndpointInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointOutput, SdkError<ModifyVerifiedAccessEndpointError>>> {
        (*self).modify_verified_access_endpoint(builder)
    }
    fn modify_verified_access_endpoint_policy(&self, builder: ModifyVerifiedAccessEndpointPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessEndpointPolicyOutput, SdkError<ModifyVerifiedAccessEndpointPolicyError>>> {
        (*self).modify_verified_access_endpoint_policy(builder)
    }
    fn modify_verified_access_group(&self, builder: ModifyVerifiedAccessGroupInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupOutput, SdkError<ModifyVerifiedAccessGroupError>>> {
        (*self).modify_verified_access_group(builder)
    }
    fn modify_verified_access_group_policy(&self, builder: ModifyVerifiedAccessGroupPolicyInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessGroupPolicyOutput, SdkError<ModifyVerifiedAccessGroupPolicyError>>> {
        (*self).modify_verified_access_group_policy(builder)
    }
    fn modify_verified_access_instance(&self, builder: ModifyVerifiedAccessInstanceInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceOutput, SdkError<ModifyVerifiedAccessInstanceError>>> {
        (*self).modify_verified_access_instance(builder)
    }
    fn modify_verified_access_instance_logging_configuration(&self, builder: ModifyVerifiedAccessInstanceLoggingConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessInstanceLoggingConfigurationOutput, SdkError<ModifyVerifiedAccessInstanceLoggingConfigurationError>>> {
        (*self).modify_verified_access_instance_logging_configuration(builder)
    }
    fn modify_verified_access_trust_provider(&self, builder: ModifyVerifiedAccessTrustProviderInputBuilder) -> impl Future<Output = Result<ModifyVerifiedAccessTrustProviderOutput, SdkError<ModifyVerifiedAccessTrustProviderError>>> {
        (*self).modify_verified_access_trust_provider(builder)
    }
    fn modify_volume(&self, builder: ModifyVolumeInputBuilder) -> impl Future<Output = Result<ModifyVolumeOutput, SdkError<ModifyVolumeError>>> {
        (*self).modify_volume(builder)
    }
    fn modify_volume_attribute(&self, builder: ModifyVolumeAttributeInputBuilder) -> impl Future<Output = Result<ModifyVolumeAttributeOutput, SdkError<ModifyVolumeAttributeError>>> {
        (*self).modify_volume_attribute(builder)
    }
    fn modify_vpc_attribute(&self, builder: ModifyVpcAttributeInputBuilder) -> impl Future<Output = Result<ModifyVpcAttributeOutput, SdkError<ModifyVpcAttributeError>>> {
        (*self).modify_vpc_attribute(builder)
    }
    fn modify_vpc_endpoint(&self, builder: ModifyVpcEndpointInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointOutput, SdkError<ModifyVpcEndpointError>>> {
        (*self).modify_vpc_endpoint(builder)
    }
    fn modify_vpc_endpoint_connection_notification(&self, builder: ModifyVpcEndpointConnectionNotificationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointConnectionNotificationOutput, SdkError<ModifyVpcEndpointConnectionNotificationError>>> {
        (*self).modify_vpc_endpoint_connection_notification(builder)
    }
    fn modify_vpc_endpoint_service_configuration(&self, builder: ModifyVpcEndpointServiceConfigurationInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServiceConfigurationOutput, SdkError<ModifyVpcEndpointServiceConfigurationError>>> {
        (*self).modify_vpc_endpoint_service_configuration(builder)
    }
    fn modify_vpc_endpoint_service_payer_responsibility(&self, builder: ModifyVpcEndpointServicePayerResponsibilityInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePayerResponsibilityOutput, SdkError<ModifyVpcEndpointServicePayerResponsibilityError>>> {
        (*self).modify_vpc_endpoint_service_payer_responsibility(builder)
    }
    fn modify_vpc_endpoint_service_permissions(&self, builder: ModifyVpcEndpointServicePermissionsInputBuilder) -> impl Future<Output = Result<ModifyVpcEndpointServicePermissionsOutput, SdkError<ModifyVpcEndpointServicePermissionsError>>> {
        (*self).modify_vpc_endpoint_service_permissions(builder)
    }
    fn modify_vpc_peering_connection_options(&self, builder: ModifyVpcPeeringConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpcPeeringConnectionOptionsOutput, SdkError<ModifyVpcPeeringConnectionOptionsError>>> {
        (*self).modify_vpc_peering_connection_options(builder)
    }
    fn modify_vpc_tenancy(&self, builder: ModifyVpcTenancyInputBuilder) -> impl Future<Output = Result<ModifyVpcTenancyOutput, SdkError<ModifyVpcTenancyError>>> {
        (*self).modify_vpc_tenancy(builder)
    }
    fn modify_vpn_connection(&self, builder: ModifyVpnConnectionInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOutput, SdkError<ModifyVpnConnectionError>>> {
        (*self).modify_vpn_connection(builder)
    }
    fn modify_vpn_connection_options(&self, builder: ModifyVpnConnectionOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnConnectionOptionsOutput, SdkError<ModifyVpnConnectionOptionsError>>> {
        (*self).modify_vpn_connection_options(builder)
    }
    fn modify_vpn_tunnel_certificate(&self, builder: ModifyVpnTunnelCertificateInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelCertificateOutput, SdkError<ModifyVpnTunnelCertificateError>>> {
        (*self).modify_vpn_tunnel_certificate(builder)
    }
    fn modify_vpn_tunnel_options(&self, builder: ModifyVpnTunnelOptionsInputBuilder) -> impl Future<Output = Result<ModifyVpnTunnelOptionsOutput, SdkError<ModifyVpnTunnelOptionsError>>> {
        (*self).modify_vpn_tunnel_options(builder)
    }
    fn monitor_instances(&self, builder: MonitorInstancesInputBuilder) -> impl Future<Output = Result<MonitorInstancesOutput, SdkError<MonitorInstancesError>>> {
        (*self).monitor_instances(builder)
    }
    fn move_address_to_vpc(&self, builder: MoveAddressToVpcInputBuilder) -> impl Future<Output = Result<MoveAddressToVpcOutput, SdkError<MoveAddressToVpcError>>> {
        (*self).move_address_to_vpc(builder)
    }
    fn move_byoip_cidr_to_ipam(&self, builder: MoveByoipCidrToIpamInputBuilder) -> impl Future<Output = Result<MoveByoipCidrToIpamOutput, SdkError<MoveByoipCidrToIpamError>>> {
        (*self).move_byoip_cidr_to_ipam(builder)
    }
    fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> impl Future<Output = Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>> {
        (*self).provision_byoip_cidr(builder)
    }
    fn provision_ipam_byoasn(&self, builder: ProvisionIpamByoasnInputBuilder) -> impl Future<Output = Result<ProvisionIpamByoasnOutput, SdkError<ProvisionIpamByoasnError>>> {
        (*self).provision_ipam_byoasn(builder)
    }
    fn provision_ipam_pool_cidr(&self, builder: ProvisionIpamPoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionIpamPoolCidrOutput, SdkError<ProvisionIpamPoolCidrError>>> {
        (*self).provision_ipam_pool_cidr(builder)
    }
    fn provision_public_ipv4_pool_cidr(&self, builder: ProvisionPublicIpv4PoolCidrInputBuilder) -> impl Future<Output = Result<ProvisionPublicIpv4PoolCidrOutput, SdkError<ProvisionPublicIpv4PoolCidrError>>> {
        (*self).provision_public_ipv4_pool_cidr(builder)
    }
    fn purchase_capacity_block(&self, builder: PurchaseCapacityBlockInputBuilder) -> impl Future<Output = Result<PurchaseCapacityBlockOutput, SdkError<PurchaseCapacityBlockError>>> {
        (*self).purchase_capacity_block(builder)
    }
    fn purchase_host_reservation(&self, builder: PurchaseHostReservationInputBuilder) -> impl Future<Output = Result<PurchaseHostReservationOutput, SdkError<PurchaseHostReservationError>>> {
        (*self).purchase_host_reservation(builder)
    }
    fn purchase_reserved_instances_offering(&self, builder: PurchaseReservedInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedInstancesOfferingOutput, SdkError<PurchaseReservedInstancesOfferingError>>> {
        (*self).purchase_reserved_instances_offering(builder)
    }
    fn purchase_scheduled_instances(&self, builder: PurchaseScheduledInstancesInputBuilder) -> impl Future<Output = Result<PurchaseScheduledInstancesOutput, SdkError<PurchaseScheduledInstancesError>>> {
        (*self).purchase_scheduled_instances(builder)
    }
    fn reboot_instances(&self, builder: RebootInstancesInputBuilder) -> impl Future<Output = Result<RebootInstancesOutput, SdkError<RebootInstancesError>>> {
        (*self).reboot_instances(builder)
    }
    fn register_image(&self, builder: RegisterImageInputBuilder) -> impl Future<Output = Result<RegisterImageOutput, SdkError<RegisterImageError>>> {
        (*self).register_image(builder)
    }
    fn register_instance_event_notification_attributes(&self, builder: RegisterInstanceEventNotificationAttributesInputBuilder) -> impl Future<Output = Result<RegisterInstanceEventNotificationAttributesOutput, SdkError<RegisterInstanceEventNotificationAttributesError>>> {
        (*self).register_instance_event_notification_attributes(builder)
    }
    fn register_transit_gateway_multicast_group_members(&self, builder: RegisterTransitGatewayMulticastGroupMembersInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupMembersOutput, SdkError<RegisterTransitGatewayMulticastGroupMembersError>>> {
        (*self).register_transit_gateway_multicast_group_members(builder)
    }
    fn register_transit_gateway_multicast_group_sources(&self, builder: RegisterTransitGatewayMulticastGroupSourcesInputBuilder) -> impl Future<Output = Result<RegisterTransitGatewayMulticastGroupSourcesOutput, SdkError<RegisterTransitGatewayMulticastGroupSourcesError>>> {
        (*self).register_transit_gateway_multicast_group_sources(builder)
    }
    fn reject_transit_gateway_multicast_domain_associations(&self, builder: RejectTransitGatewayMulticastDomainAssociationsInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayMulticastDomainAssociationsOutput, SdkError<RejectTransitGatewayMulticastDomainAssociationsError>>> {
        (*self).reject_transit_gateway_multicast_domain_associations(builder)
    }
    fn reject_transit_gateway_peering_attachment(&self, builder: RejectTransitGatewayPeeringAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayPeeringAttachmentOutput, SdkError<RejectTransitGatewayPeeringAttachmentError>>> {
        (*self).reject_transit_gateway_peering_attachment(builder)
    }
    fn reject_transit_gateway_vpc_attachment(&self, builder: RejectTransitGatewayVpcAttachmentInputBuilder) -> impl Future<Output = Result<RejectTransitGatewayVpcAttachmentOutput, SdkError<RejectTransitGatewayVpcAttachmentError>>> {
        (*self).reject_transit_gateway_vpc_attachment(builder)
    }
    fn reject_vpc_endpoint_connections(&self, builder: RejectVpcEndpointConnectionsInputBuilder) -> impl Future<Output = Result<RejectVpcEndpointConnectionsOutput, SdkError<RejectVpcEndpointConnectionsError>>> {
        (*self).reject_vpc_endpoint_connections(builder)
    }
    fn reject_vpc_peering_connection(&self, builder: RejectVpcPeeringConnectionInputBuilder) -> impl Future<Output = Result<RejectVpcPeeringConnectionOutput, SdkError<RejectVpcPeeringConnectionError>>> {
        (*self).reject_vpc_peering_connection(builder)
    }
    fn release_address(&self, builder: ReleaseAddressInputBuilder) -> impl Future<Output = Result<ReleaseAddressOutput, SdkError<ReleaseAddressError>>> {
        (*self).release_address(builder)
    }
    fn release_hosts(&self, builder: ReleaseHostsInputBuilder) -> impl Future<Output = Result<ReleaseHostsOutput, SdkError<ReleaseHostsError>>> {
        (*self).release_hosts(builder)
    }
    fn release_ipam_pool_allocation(&self, builder: ReleaseIpamPoolAllocationInputBuilder) -> impl Future<Output = Result<ReleaseIpamPoolAllocationOutput, SdkError<ReleaseIpamPoolAllocationError>>> {
        (*self).release_ipam_pool_allocation(builder)
    }
    fn replace_iam_instance_profile_association(&self, builder: ReplaceIamInstanceProfileAssociationInputBuilder) -> impl Future<Output = Result<ReplaceIamInstanceProfileAssociationOutput, SdkError<ReplaceIamInstanceProfileAssociationError>>> {
        (*self).replace_iam_instance_profile_association(builder)
    }
    fn replace_network_acl_association(&self, builder: ReplaceNetworkAclAssociationInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclAssociationOutput, SdkError<ReplaceNetworkAclAssociationError>>> {
        (*self).replace_network_acl_association(builder)
    }
    fn replace_network_acl_entry(&self, builder: ReplaceNetworkAclEntryInputBuilder) -> impl Future<Output = Result<ReplaceNetworkAclEntryOutput, SdkError<ReplaceNetworkAclEntryError>>> {
        (*self).replace_network_acl_entry(builder)
    }
    fn replace_route(&self, builder: ReplaceRouteInputBuilder) -> impl Future<Output = Result<ReplaceRouteOutput, SdkError<ReplaceRouteError>>> {
        (*self).replace_route(builder)
    }
    fn replace_route_table_association(&self, builder: ReplaceRouteTableAssociationInputBuilder) -> impl Future<Output = Result<ReplaceRouteTableAssociationOutput, SdkError<ReplaceRouteTableAssociationError>>> {
        (*self).replace_route_table_association(builder)
    }
    fn replace_transit_gateway_route(&self, builder: ReplaceTransitGatewayRouteInputBuilder) -> impl Future<Output = Result<ReplaceTransitGatewayRouteOutput, SdkError<ReplaceTransitGatewayRouteError>>> {
        (*self).replace_transit_gateway_route(builder)
    }
    fn replace_vpn_tunnel(&self, builder: ReplaceVpnTunnelInputBuilder) -> impl Future<Output = Result<ReplaceVpnTunnelOutput, SdkError<ReplaceVpnTunnelError>>> {
        (*self).replace_vpn_tunnel(builder)
    }
    fn report_instance_status(&self, builder: ReportInstanceStatusInputBuilder) -> impl Future<Output = Result<ReportInstanceStatusOutput, SdkError<ReportInstanceStatusError>>> {
        (*self).report_instance_status(builder)
    }
    fn request_spot_fleet(&self, builder: RequestSpotFleetInputBuilder) -> impl Future<Output = Result<RequestSpotFleetOutput, SdkError<RequestSpotFleetError>>> {
        (*self).request_spot_fleet(builder)
    }
    fn request_spot_instances(&self, builder: RequestSpotInstancesInputBuilder) -> impl Future<Output = Result<RequestSpotInstancesOutput, SdkError<RequestSpotInstancesError>>> {
        (*self).request_spot_instances(builder)
    }
    fn reset_address_attribute(&self, builder: ResetAddressAttributeInputBuilder) -> impl Future<Output = Result<ResetAddressAttributeOutput, SdkError<ResetAddressAttributeError>>> {
        (*self).reset_address_attribute(builder)
    }
    fn reset_ebs_default_kms_key_id(&self, builder: ResetEbsDefaultKmsKeyIdInputBuilder) -> impl Future<Output = Result<ResetEbsDefaultKmsKeyIdOutput, SdkError<ResetEbsDefaultKmsKeyIdError>>> {
        (*self).reset_ebs_default_kms_key_id(builder)
    }
    fn reset_fpga_image_attribute(&self, builder: ResetFpgaImageAttributeInputBuilder) -> impl Future<Output = Result<ResetFpgaImageAttributeOutput, SdkError<ResetFpgaImageAttributeError>>> {
        (*self).reset_fpga_image_attribute(builder)
    }
    fn reset_image_attribute(&self, builder: ResetImageAttributeInputBuilder) -> impl Future<Output = Result<ResetImageAttributeOutput, SdkError<ResetImageAttributeError>>> {
        (*self).reset_image_attribute(builder)
    }
    fn reset_instance_attribute(&self, builder: ResetInstanceAttributeInputBuilder) -> impl Future<Output = Result<ResetInstanceAttributeOutput, SdkError<ResetInstanceAttributeError>>> {
        (*self).reset_instance_attribute(builder)
    }
    fn reset_network_interface_attribute(&self, builder: ResetNetworkInterfaceAttributeInputBuilder) -> impl Future<Output = Result<ResetNetworkInterfaceAttributeOutput, SdkError<ResetNetworkInterfaceAttributeError>>> {
        (*self).reset_network_interface_attribute(builder)
    }
    fn reset_snapshot_attribute(&self, builder: ResetSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ResetSnapshotAttributeOutput, SdkError<ResetSnapshotAttributeError>>> {
        (*self).reset_snapshot_attribute(builder)
    }
    fn restore_address_to_classic(&self, builder: RestoreAddressToClassicInputBuilder) -> impl Future<Output = Result<RestoreAddressToClassicOutput, SdkError<RestoreAddressToClassicError>>> {
        (*self).restore_address_to_classic(builder)
    }
    fn restore_image_from_recycle_bin(&self, builder: RestoreImageFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreImageFromRecycleBinOutput, SdkError<RestoreImageFromRecycleBinError>>> {
        (*self).restore_image_from_recycle_bin(builder)
    }
    fn restore_managed_prefix_list_version(&self, builder: RestoreManagedPrefixListVersionInputBuilder) -> impl Future<Output = Result<RestoreManagedPrefixListVersionOutput, SdkError<RestoreManagedPrefixListVersionError>>> {
        (*self).restore_managed_prefix_list_version(builder)
    }
    fn restore_snapshot_from_recycle_bin(&self, builder: RestoreSnapshotFromRecycleBinInputBuilder) -> impl Future<Output = Result<RestoreSnapshotFromRecycleBinOutput, SdkError<RestoreSnapshotFromRecycleBinError>>> {
        (*self).restore_snapshot_from_recycle_bin(builder)
    }
    fn restore_snapshot_tier(&self, builder: RestoreSnapshotTierInputBuilder) -> impl Future<Output = Result<RestoreSnapshotTierOutput, SdkError<RestoreSnapshotTierError>>> {
        (*self).restore_snapshot_tier(builder)
    }
    fn revoke_client_vpn_ingress(&self, builder: RevokeClientVpnIngressInputBuilder) -> impl Future<Output = Result<RevokeClientVpnIngressOutput, SdkError<RevokeClientVpnIngressError>>> {
        (*self).revoke_client_vpn_ingress(builder)
    }
    fn revoke_security_group_egress(&self, builder: RevokeSecurityGroupEgressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupEgressOutput, SdkError<RevokeSecurityGroupEgressError>>> {
        (*self).revoke_security_group_egress(builder)
    }
    fn revoke_security_group_ingress(&self, builder: RevokeSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeSecurityGroupIngressOutput, SdkError<RevokeSecurityGroupIngressError>>> {
        (*self).revoke_security_group_ingress(builder)
    }
    fn run_instances(&self, builder: RunInstancesInputBuilder) -> impl Future<Output = Result<RunInstancesOutput, SdkError<RunInstancesError>>> {
        (*self).run_instances(builder)
    }
    fn run_scheduled_instances(&self, builder: RunScheduledInstancesInputBuilder) -> impl Future<Output = Result<RunScheduledInstancesOutput, SdkError<RunScheduledInstancesError>>> {
        (*self).run_scheduled_instances(builder)
    }
    fn search_local_gateway_routes(&self, builder: SearchLocalGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchLocalGatewayRoutesOutput, SdkError<SearchLocalGatewayRoutesError>>> {
        (*self).search_local_gateway_routes(builder)
    }
    fn search_transit_gateway_multicast_groups(&self, builder: SearchTransitGatewayMulticastGroupsInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayMulticastGroupsOutput, SdkError<SearchTransitGatewayMulticastGroupsError>>> {
        (*self).search_transit_gateway_multicast_groups(builder)
    }
    fn search_transit_gateway_routes(&self, builder: SearchTransitGatewayRoutesInputBuilder) -> impl Future<Output = Result<SearchTransitGatewayRoutesOutput, SdkError<SearchTransitGatewayRoutesError>>> {
        (*self).search_transit_gateway_routes(builder)
    }
    fn send_diagnostic_interrupt(&self, builder: SendDiagnosticInterruptInputBuilder) -> impl Future<Output = Result<SendDiagnosticInterruptOutput, SdkError<SendDiagnosticInterruptError>>> {
        (*self).send_diagnostic_interrupt(builder)
    }
    fn start_instances(&self, builder: StartInstancesInputBuilder) -> impl Future<Output = Result<StartInstancesOutput, SdkError<StartInstancesError>>> {
        (*self).start_instances(builder)
    }
    fn start_network_insights_access_scope_analysis(&self, builder: StartNetworkInsightsAccessScopeAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAccessScopeAnalysisOutput, SdkError<StartNetworkInsightsAccessScopeAnalysisError>>> {
        (*self).start_network_insights_access_scope_analysis(builder)
    }
    fn start_network_insights_analysis(&self, builder: StartNetworkInsightsAnalysisInputBuilder) -> impl Future<Output = Result<StartNetworkInsightsAnalysisOutput, SdkError<StartNetworkInsightsAnalysisError>>> {
        (*self).start_network_insights_analysis(builder)
    }
    fn start_vpc_endpoint_service_private_dns_verification(&self, builder: StartVpcEndpointServicePrivateDnsVerificationInputBuilder) -> impl Future<Output = Result<StartVpcEndpointServicePrivateDnsVerificationOutput, SdkError<StartVpcEndpointServicePrivateDnsVerificationError>>> {
        (*self).start_vpc_endpoint_service_private_dns_verification(builder)
    }
    fn stop_instances(&self, builder: StopInstancesInputBuilder) -> impl Future<Output = Result<StopInstancesOutput, SdkError<StopInstancesError>>> {
        (*self).stop_instances(builder)
    }
    fn terminate_client_vpn_connections(&self, builder: TerminateClientVpnConnectionsInputBuilder) -> impl Future<Output = Result<TerminateClientVpnConnectionsOutput, SdkError<TerminateClientVpnConnectionsError>>> {
        (*self).terminate_client_vpn_connections(builder)
    }
    fn terminate_instances(&self, builder: TerminateInstancesInputBuilder) -> impl Future<Output = Result<TerminateInstancesOutput, SdkError<TerminateInstancesError>>> {
        (*self).terminate_instances(builder)
    }
    fn unassign_ipv6_addresses(&self, builder: UnassignIpv6AddressesInputBuilder) -> impl Future<Output = Result<UnassignIpv6AddressesOutput, SdkError<UnassignIpv6AddressesError>>> {
        (*self).unassign_ipv6_addresses(builder)
    }
    fn unassign_private_ip_addresses(&self, builder: UnassignPrivateIpAddressesInputBuilder) -> impl Future<Output = Result<UnassignPrivateIpAddressesOutput, SdkError<UnassignPrivateIpAddressesError>>> {
        (*self).unassign_private_ip_addresses(builder)
    }
    fn unassign_private_nat_gateway_address(&self, builder: UnassignPrivateNatGatewayAddressInputBuilder) -> impl Future<Output = Result<UnassignPrivateNatGatewayAddressOutput, SdkError<UnassignPrivateNatGatewayAddressError>>> {
        (*self).unassign_private_nat_gateway_address(builder)
    }
    fn unlock_snapshot(&self, builder: UnlockSnapshotInputBuilder) -> impl Future<Output = Result<UnlockSnapshotOutput, SdkError<UnlockSnapshotError>>> {
        (*self).unlock_snapshot(builder)
    }
    fn unmonitor_instances(&self, builder: UnmonitorInstancesInputBuilder) -> impl Future<Output = Result<UnmonitorInstancesOutput, SdkError<UnmonitorInstancesError>>> {
        (*self).unmonitor_instances(builder)
    }
    fn update_security_group_rule_descriptions_egress(&self, builder: UpdateSecurityGroupRuleDescriptionsEgressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsEgressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsEgressError>>> {
        (*self).update_security_group_rule_descriptions_egress(builder)
    }
    fn update_security_group_rule_descriptions_ingress(&self, builder: UpdateSecurityGroupRuleDescriptionsIngressInputBuilder) -> impl Future<Output = Result<UpdateSecurityGroupRuleDescriptionsIngressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsIngressError>>> {
        (*self).update_security_group_rule_descriptions_ingress(builder)
    }
    fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> impl Future<Output = Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>> {
        (*self).withdraw_byoip_cidr(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edEC2Client {}
    impl EC2Client for edEC2Client {
        async fn accept_address_transfer(&self, builder: AcceptAddressTransferInputBuilder) -> Result<AcceptAddressTransferOutput, SdkError<AcceptAddressTransferError>>;
        async fn accept_reserved_instances_exchange_quote(&self, builder: AcceptReservedInstancesExchangeQuoteInputBuilder) -> Result<AcceptReservedInstancesExchangeQuoteOutput, SdkError<AcceptReservedInstancesExchangeQuoteError>>;
        async fn accept_transit_gateway_multicast_domain_associations(&self, builder: AcceptTransitGatewayMulticastDomainAssociationsInputBuilder) -> Result<AcceptTransitGatewayMulticastDomainAssociationsOutput, SdkError<AcceptTransitGatewayMulticastDomainAssociationsError>>;
        async fn accept_transit_gateway_peering_attachment(&self, builder: AcceptTransitGatewayPeeringAttachmentInputBuilder) -> Result<AcceptTransitGatewayPeeringAttachmentOutput, SdkError<AcceptTransitGatewayPeeringAttachmentError>>;
        async fn accept_transit_gateway_vpc_attachment(&self, builder: AcceptTransitGatewayVpcAttachmentInputBuilder) -> Result<AcceptTransitGatewayVpcAttachmentOutput, SdkError<AcceptTransitGatewayVpcAttachmentError>>;
        async fn accept_vpc_endpoint_connections(&self, builder: AcceptVpcEndpointConnectionsInputBuilder) -> Result<AcceptVpcEndpointConnectionsOutput, SdkError<AcceptVpcEndpointConnectionsError>>;
        async fn accept_vpc_peering_connection(&self, builder: AcceptVpcPeeringConnectionInputBuilder) -> Result<AcceptVpcPeeringConnectionOutput, SdkError<AcceptVpcPeeringConnectionError>>;
        async fn advertise_byoip_cidr(&self, builder: AdvertiseByoipCidrInputBuilder) -> Result<AdvertiseByoipCidrOutput, SdkError<AdvertiseByoipCidrError>>;
        async fn allocate_address(&self, builder: AllocateAddressInputBuilder) -> Result<AllocateAddressOutput, SdkError<AllocateAddressError>>;
        async fn allocate_hosts(&self, builder: AllocateHostsInputBuilder) -> Result<AllocateHostsOutput, SdkError<AllocateHostsError>>;
        async fn allocate_ipam_pool_cidr(&self, builder: AllocateIpamPoolCidrInputBuilder) -> Result<AllocateIpamPoolCidrOutput, SdkError<AllocateIpamPoolCidrError>>;
        async fn apply_security_groups_to_client_vpn_target_network(&self, builder: ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder) -> Result<ApplySecurityGroupsToClientVpnTargetNetworkOutput, SdkError<ApplySecurityGroupsToClientVpnTargetNetworkError>>;
        async fn assign_ipv6_addresses(&self, builder: AssignIpv6AddressesInputBuilder) -> Result<AssignIpv6AddressesOutput, SdkError<AssignIpv6AddressesError>>;
        async fn assign_private_ip_addresses(&self, builder: AssignPrivateIpAddressesInputBuilder) -> Result<AssignPrivateIpAddressesOutput, SdkError<AssignPrivateIpAddressesError>>;
        async fn assign_private_nat_gateway_address(&self, builder: AssignPrivateNatGatewayAddressInputBuilder) -> Result<AssignPrivateNatGatewayAddressOutput, SdkError<AssignPrivateNatGatewayAddressError>>;
        async fn associate_address(&self, builder: AssociateAddressInputBuilder) -> Result<AssociateAddressOutput, SdkError<AssociateAddressError>>;
        async fn associate_client_vpn_target_network(&self, builder: AssociateClientVpnTargetNetworkInputBuilder) -> Result<AssociateClientVpnTargetNetworkOutput, SdkError<AssociateClientVpnTargetNetworkError>>;
        async fn associate_dhcp_options(&self, builder: AssociateDhcpOptionsInputBuilder) -> Result<AssociateDhcpOptionsOutput, SdkError<AssociateDhcpOptionsError>>;
        async fn associate_enclave_certificate_iam_role(&self, builder: AssociateEnclaveCertificateIamRoleInputBuilder) -> Result<AssociateEnclaveCertificateIamRoleOutput, SdkError<AssociateEnclaveCertificateIamRoleError>>;
        async fn associate_iam_instance_profile(&self, builder: AssociateIamInstanceProfileInputBuilder) -> Result<AssociateIamInstanceProfileOutput, SdkError<AssociateIamInstanceProfileError>>;
        async fn associate_instance_event_window(&self, builder: AssociateInstanceEventWindowInputBuilder) -> Result<AssociateInstanceEventWindowOutput, SdkError<AssociateInstanceEventWindowError>>;
        async fn associate_ipam_byoasn(&self, builder: AssociateIpamByoasnInputBuilder) -> Result<AssociateIpamByoasnOutput, SdkError<AssociateIpamByoasnError>>;
        async fn associate_ipam_resource_discovery(&self, builder: AssociateIpamResourceDiscoveryInputBuilder) -> Result<AssociateIpamResourceDiscoveryOutput, SdkError<AssociateIpamResourceDiscoveryError>>;
        async fn associate_nat_gateway_address(&self, builder: AssociateNatGatewayAddressInputBuilder) -> Result<AssociateNatGatewayAddressOutput, SdkError<AssociateNatGatewayAddressError>>;
        async fn associate_route_table(&self, builder: AssociateRouteTableInputBuilder) -> Result<AssociateRouteTableOutput, SdkError<AssociateRouteTableError>>;
        async fn associate_subnet_cidr_block(&self, builder: AssociateSubnetCidrBlockInputBuilder) -> Result<AssociateSubnetCidrBlockOutput, SdkError<AssociateSubnetCidrBlockError>>;
        async fn associate_transit_gateway_multicast_domain(&self, builder: AssociateTransitGatewayMulticastDomainInputBuilder) -> Result<AssociateTransitGatewayMulticastDomainOutput, SdkError<AssociateTransitGatewayMulticastDomainError>>;
        async fn associate_transit_gateway_policy_table(&self, builder: AssociateTransitGatewayPolicyTableInputBuilder) -> Result<AssociateTransitGatewayPolicyTableOutput, SdkError<AssociateTransitGatewayPolicyTableError>>;
        async fn associate_transit_gateway_route_table(&self, builder: AssociateTransitGatewayRouteTableInputBuilder) -> Result<AssociateTransitGatewayRouteTableOutput, SdkError<AssociateTransitGatewayRouteTableError>>;
        async fn associate_trunk_interface(&self, builder: AssociateTrunkInterfaceInputBuilder) -> Result<AssociateTrunkInterfaceOutput, SdkError<AssociateTrunkInterfaceError>>;
        async fn associate_vpc_cidr_block(&self, builder: AssociateVpcCidrBlockInputBuilder) -> Result<AssociateVpcCidrBlockOutput, SdkError<AssociateVpcCidrBlockError>>;
        async fn attach_classic_link_vpc(&self, builder: AttachClassicLinkVpcInputBuilder) -> Result<AttachClassicLinkVpcOutput, SdkError<AttachClassicLinkVpcError>>;
        async fn attach_internet_gateway(&self, builder: AttachInternetGatewayInputBuilder) -> Result<AttachInternetGatewayOutput, SdkError<AttachInternetGatewayError>>;
        async fn attach_network_interface(&self, builder: AttachNetworkInterfaceInputBuilder) -> Result<AttachNetworkInterfaceOutput, SdkError<AttachNetworkInterfaceError>>;
        async fn attach_verified_access_trust_provider(&self, builder: AttachVerifiedAccessTrustProviderInputBuilder) -> Result<AttachVerifiedAccessTrustProviderOutput, SdkError<AttachVerifiedAccessTrustProviderError>>;
        async fn attach_volume(&self, builder: AttachVolumeInputBuilder) -> Result<AttachVolumeOutput, SdkError<AttachVolumeError>>;
        async fn attach_vpn_gateway(&self, builder: AttachVpnGatewayInputBuilder) -> Result<AttachVpnGatewayOutput, SdkError<AttachVpnGatewayError>>;
        async fn authorize_client_vpn_ingress(&self, builder: AuthorizeClientVpnIngressInputBuilder) -> Result<AuthorizeClientVpnIngressOutput, SdkError<AuthorizeClientVpnIngressError>>;
        async fn authorize_security_group_egress(&self, builder: AuthorizeSecurityGroupEgressInputBuilder) -> Result<AuthorizeSecurityGroupEgressOutput, SdkError<AuthorizeSecurityGroupEgressError>>;
        async fn authorize_security_group_ingress(&self, builder: AuthorizeSecurityGroupIngressInputBuilder) -> Result<AuthorizeSecurityGroupIngressOutput, SdkError<AuthorizeSecurityGroupIngressError>>;
        async fn bundle_instance(&self, builder: BundleInstanceInputBuilder) -> Result<BundleInstanceOutput, SdkError<BundleInstanceError>>;
        async fn cancel_bundle_task(&self, builder: CancelBundleTaskInputBuilder) -> Result<CancelBundleTaskOutput, SdkError<CancelBundleTaskError>>;
        async fn cancel_capacity_reservation(&self, builder: CancelCapacityReservationInputBuilder) -> Result<CancelCapacityReservationOutput, SdkError<CancelCapacityReservationError>>;
        async fn cancel_capacity_reservation_fleets(&self, builder: CancelCapacityReservationFleetsInputBuilder) -> Result<CancelCapacityReservationFleetsOutput, SdkError<CancelCapacityReservationFleetsError>>;
        async fn cancel_conversion_task(&self, builder: CancelConversionTaskInputBuilder) -> Result<CancelConversionTaskOutput, SdkError<CancelConversionTaskError>>;
        async fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>;
        async fn cancel_image_launch_permission(&self, builder: CancelImageLaunchPermissionInputBuilder) -> Result<CancelImageLaunchPermissionOutput, SdkError<CancelImageLaunchPermissionError>>;
        async fn cancel_import_task(&self, builder: CancelImportTaskInputBuilder) -> Result<CancelImportTaskOutput, SdkError<CancelImportTaskError>>;
        async fn cancel_reserved_instances_listing(&self, builder: CancelReservedInstancesListingInputBuilder) -> Result<CancelReservedInstancesListingOutput, SdkError<CancelReservedInstancesListingError>>;
        async fn cancel_spot_fleet_requests(&self, builder: CancelSpotFleetRequestsInputBuilder) -> Result<CancelSpotFleetRequestsOutput, SdkError<CancelSpotFleetRequestsError>>;
        async fn cancel_spot_instance_requests(&self, builder: CancelSpotInstanceRequestsInputBuilder) -> Result<CancelSpotInstanceRequestsOutput, SdkError<CancelSpotInstanceRequestsError>>;
        async fn confirm_product_instance(&self, builder: ConfirmProductInstanceInputBuilder) -> Result<ConfirmProductInstanceOutput, SdkError<ConfirmProductInstanceError>>;
        async fn copy_fpga_image(&self, builder: CopyFpgaImageInputBuilder) -> Result<CopyFpgaImageOutput, SdkError<CopyFpgaImageError>>;
        async fn copy_image(&self, builder: CopyImageInputBuilder) -> Result<CopyImageOutput, SdkError<CopyImageError>>;
        async fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> Result<CopySnapshotOutput, SdkError<CopySnapshotError>>;
        async fn create_capacity_reservation(&self, builder: CreateCapacityReservationInputBuilder) -> Result<CreateCapacityReservationOutput, SdkError<CreateCapacityReservationError>>;
        async fn create_capacity_reservation_fleet(&self, builder: CreateCapacityReservationFleetInputBuilder) -> Result<CreateCapacityReservationFleetOutput, SdkError<CreateCapacityReservationFleetError>>;
        async fn create_carrier_gateway(&self, builder: CreateCarrierGatewayInputBuilder) -> Result<CreateCarrierGatewayOutput, SdkError<CreateCarrierGatewayError>>;
        async fn create_client_vpn_endpoint(&self, builder: CreateClientVpnEndpointInputBuilder) -> Result<CreateClientVpnEndpointOutput, SdkError<CreateClientVpnEndpointError>>;
        async fn create_client_vpn_route(&self, builder: CreateClientVpnRouteInputBuilder) -> Result<CreateClientVpnRouteOutput, SdkError<CreateClientVpnRouteError>>;
        async fn create_coip_cidr(&self, builder: CreateCoipCidrInputBuilder) -> Result<CreateCoipCidrOutput, SdkError<CreateCoipCidrError>>;
        async fn create_coip_pool(&self, builder: CreateCoipPoolInputBuilder) -> Result<CreateCoipPoolOutput, SdkError<CreateCoipPoolError>>;
        async fn create_customer_gateway(&self, builder: CreateCustomerGatewayInputBuilder) -> Result<CreateCustomerGatewayOutput, SdkError<CreateCustomerGatewayError>>;
        async fn create_default_subnet(&self, builder: CreateDefaultSubnetInputBuilder) -> Result<CreateDefaultSubnetOutput, SdkError<CreateDefaultSubnetError>>;
        async fn create_default_vpc(&self, builder: CreateDefaultVpcInputBuilder) -> Result<CreateDefaultVpcOutput, SdkError<CreateDefaultVpcError>>;
        async fn create_dhcp_options(&self, builder: CreateDhcpOptionsInputBuilder) -> Result<CreateDhcpOptionsOutput, SdkError<CreateDhcpOptionsError>>;
        async fn create_egress_only_internet_gateway(&self, builder: CreateEgressOnlyInternetGatewayInputBuilder) -> Result<CreateEgressOnlyInternetGatewayOutput, SdkError<CreateEgressOnlyInternetGatewayError>>;
        async fn create_fleet(&self, builder: CreateFleetInputBuilder) -> Result<CreateFleetOutput, SdkError<CreateFleetError>>;
        async fn create_flow_logs(&self, builder: CreateFlowLogsInputBuilder) -> Result<CreateFlowLogsOutput, SdkError<CreateFlowLogsError>>;
        async fn create_fpga_image(&self, builder: CreateFpgaImageInputBuilder) -> Result<CreateFpgaImageOutput, SdkError<CreateFpgaImageError>>;
        async fn create_image(&self, builder: CreateImageInputBuilder) -> Result<CreateImageOutput, SdkError<CreateImageError>>;
        async fn create_instance_connect_endpoint(&self, builder: CreateInstanceConnectEndpointInputBuilder) -> Result<CreateInstanceConnectEndpointOutput, SdkError<CreateInstanceConnectEndpointError>>;
        async fn create_instance_event_window(&self, builder: CreateInstanceEventWindowInputBuilder) -> Result<CreateInstanceEventWindowOutput, SdkError<CreateInstanceEventWindowError>>;
        async fn create_instance_export_task(&self, builder: CreateInstanceExportTaskInputBuilder) -> Result<CreateInstanceExportTaskOutput, SdkError<CreateInstanceExportTaskError>>;
        async fn create_internet_gateway(&self, builder: CreateInternetGatewayInputBuilder) -> Result<CreateInternetGatewayOutput, SdkError<CreateInternetGatewayError>>;
        async fn create_ipam(&self, builder: CreateIpamInputBuilder) -> Result<CreateIpamOutput, SdkError<CreateIpamError>>;
        async fn create_ipam_external_resource_verification_token(&self, builder: CreateIpamExternalResourceVerificationTokenInputBuilder) -> Result<CreateIpamExternalResourceVerificationTokenOutput, SdkError<CreateIpamExternalResourceVerificationTokenError>>;
        async fn create_ipam_pool(&self, builder: CreateIpamPoolInputBuilder) -> Result<CreateIpamPoolOutput, SdkError<CreateIpamPoolError>>;
        async fn create_ipam_resource_discovery(&self, builder: CreateIpamResourceDiscoveryInputBuilder) -> Result<CreateIpamResourceDiscoveryOutput, SdkError<CreateIpamResourceDiscoveryError>>;
        async fn create_ipam_scope(&self, builder: CreateIpamScopeInputBuilder) -> Result<CreateIpamScopeOutput, SdkError<CreateIpamScopeError>>;
        async fn create_key_pair(&self, builder: CreateKeyPairInputBuilder) -> Result<CreateKeyPairOutput, SdkError<CreateKeyPairError>>;
        async fn create_launch_template(&self, builder: CreateLaunchTemplateInputBuilder) -> Result<CreateLaunchTemplateOutput, SdkError<CreateLaunchTemplateError>>;
        async fn create_launch_template_version(&self, builder: CreateLaunchTemplateVersionInputBuilder) -> Result<CreateLaunchTemplateVersionOutput, SdkError<CreateLaunchTemplateVersionError>>;
        async fn create_local_gateway_route(&self, builder: CreateLocalGatewayRouteInputBuilder) -> Result<CreateLocalGatewayRouteOutput, SdkError<CreateLocalGatewayRouteError>>;
        async fn create_local_gateway_route_table(&self, builder: CreateLocalGatewayRouteTableInputBuilder) -> Result<CreateLocalGatewayRouteTableOutput, SdkError<CreateLocalGatewayRouteTableError>>;
        async fn create_local_gateway_route_table_virtual_interface_group_association(&self, builder: CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> Result<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>;
        async fn create_local_gateway_route_table_vpc_association(&self, builder: CreateLocalGatewayRouteTableVpcAssociationInputBuilder) -> Result<CreateLocalGatewayRouteTableVpcAssociationOutput, SdkError<CreateLocalGatewayRouteTableVpcAssociationError>>;
        async fn create_managed_prefix_list(&self, builder: CreateManagedPrefixListInputBuilder) -> Result<CreateManagedPrefixListOutput, SdkError<CreateManagedPrefixListError>>;
        async fn create_nat_gateway(&self, builder: CreateNatGatewayInputBuilder) -> Result<CreateNatGatewayOutput, SdkError<CreateNatGatewayError>>;
        async fn create_network_acl(&self, builder: CreateNetworkAclInputBuilder) -> Result<CreateNetworkAclOutput, SdkError<CreateNetworkAclError>>;
        async fn create_network_acl_entry(&self, builder: CreateNetworkAclEntryInputBuilder) -> Result<CreateNetworkAclEntryOutput, SdkError<CreateNetworkAclEntryError>>;
        async fn create_network_insights_access_scope(&self, builder: CreateNetworkInsightsAccessScopeInputBuilder) -> Result<CreateNetworkInsightsAccessScopeOutput, SdkError<CreateNetworkInsightsAccessScopeError>>;
        async fn create_network_insights_path(&self, builder: CreateNetworkInsightsPathInputBuilder) -> Result<CreateNetworkInsightsPathOutput, SdkError<CreateNetworkInsightsPathError>>;
        async fn create_network_interface(&self, builder: CreateNetworkInterfaceInputBuilder) -> Result<CreateNetworkInterfaceOutput, SdkError<CreateNetworkInterfaceError>>;
        async fn create_network_interface_permission(&self, builder: CreateNetworkInterfacePermissionInputBuilder) -> Result<CreateNetworkInterfacePermissionOutput, SdkError<CreateNetworkInterfacePermissionError>>;
        async fn create_placement_group(&self, builder: CreatePlacementGroupInputBuilder) -> Result<CreatePlacementGroupOutput, SdkError<CreatePlacementGroupError>>;
        async fn create_public_ipv4_pool(&self, builder: CreatePublicIpv4PoolInputBuilder) -> Result<CreatePublicIpv4PoolOutput, SdkError<CreatePublicIpv4PoolError>>;
        async fn create_replace_root_volume_task(&self, builder: CreateReplaceRootVolumeTaskInputBuilder) -> Result<CreateReplaceRootVolumeTaskOutput, SdkError<CreateReplaceRootVolumeTaskError>>;
        async fn create_reserved_instances_listing(&self, builder: CreateReservedInstancesListingInputBuilder) -> Result<CreateReservedInstancesListingOutput, SdkError<CreateReservedInstancesListingError>>;
        async fn create_restore_image_task(&self, builder: CreateRestoreImageTaskInputBuilder) -> Result<CreateRestoreImageTaskOutput, SdkError<CreateRestoreImageTaskError>>;
        async fn create_route(&self, builder: CreateRouteInputBuilder) -> Result<CreateRouteOutput, SdkError<CreateRouteError>>;
        async fn create_route_table(&self, builder: CreateRouteTableInputBuilder) -> Result<CreateRouteTableOutput, SdkError<CreateRouteTableError>>;
        async fn create_security_group(&self, builder: CreateSecurityGroupInputBuilder) -> Result<CreateSecurityGroupOutput, SdkError<CreateSecurityGroupError>>;
        async fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>;
        async fn create_snapshots(&self, builder: CreateSnapshotsInputBuilder) -> Result<CreateSnapshotsOutput, SdkError<CreateSnapshotsError>>;
        async fn create_spot_datafeed_subscription(&self, builder: CreateSpotDatafeedSubscriptionInputBuilder) -> Result<CreateSpotDatafeedSubscriptionOutput, SdkError<CreateSpotDatafeedSubscriptionError>>;
        async fn create_store_image_task(&self, builder: CreateStoreImageTaskInputBuilder) -> Result<CreateStoreImageTaskOutput, SdkError<CreateStoreImageTaskError>>;
        async fn create_subnet(&self, builder: CreateSubnetInputBuilder) -> Result<CreateSubnetOutput, SdkError<CreateSubnetError>>;
        async fn create_subnet_cidr_reservation(&self, builder: CreateSubnetCidrReservationInputBuilder) -> Result<CreateSubnetCidrReservationOutput, SdkError<CreateSubnetCidrReservationError>>;
        async fn create_tags(&self, builder: CreateTagsInputBuilder) -> Result<CreateTagsOutput, SdkError<CreateTagsError>>;
        async fn create_traffic_mirror_filter(&self, builder: CreateTrafficMirrorFilterInputBuilder) -> Result<CreateTrafficMirrorFilterOutput, SdkError<CreateTrafficMirrorFilterError>>;
        async fn create_traffic_mirror_filter_rule(&self, builder: CreateTrafficMirrorFilterRuleInputBuilder) -> Result<CreateTrafficMirrorFilterRuleOutput, SdkError<CreateTrafficMirrorFilterRuleError>>;
        async fn create_traffic_mirror_session(&self, builder: CreateTrafficMirrorSessionInputBuilder) -> Result<CreateTrafficMirrorSessionOutput, SdkError<CreateTrafficMirrorSessionError>>;
        async fn create_traffic_mirror_target(&self, builder: CreateTrafficMirrorTargetInputBuilder) -> Result<CreateTrafficMirrorTargetOutput, SdkError<CreateTrafficMirrorTargetError>>;
        async fn create_transit_gateway(&self, builder: CreateTransitGatewayInputBuilder) -> Result<CreateTransitGatewayOutput, SdkError<CreateTransitGatewayError>>;
        async fn create_transit_gateway_connect(&self, builder: CreateTransitGatewayConnectInputBuilder) -> Result<CreateTransitGatewayConnectOutput, SdkError<CreateTransitGatewayConnectError>>;
        async fn create_transit_gateway_connect_peer(&self, builder: CreateTransitGatewayConnectPeerInputBuilder) -> Result<CreateTransitGatewayConnectPeerOutput, SdkError<CreateTransitGatewayConnectPeerError>>;
        async fn create_transit_gateway_multicast_domain(&self, builder: CreateTransitGatewayMulticastDomainInputBuilder) -> Result<CreateTransitGatewayMulticastDomainOutput, SdkError<CreateTransitGatewayMulticastDomainError>>;
        async fn create_transit_gateway_peering_attachment(&self, builder: CreateTransitGatewayPeeringAttachmentInputBuilder) -> Result<CreateTransitGatewayPeeringAttachmentOutput, SdkError<CreateTransitGatewayPeeringAttachmentError>>;
        async fn create_transit_gateway_policy_table(&self, builder: CreateTransitGatewayPolicyTableInputBuilder) -> Result<CreateTransitGatewayPolicyTableOutput, SdkError<CreateTransitGatewayPolicyTableError>>;
        async fn create_transit_gateway_prefix_list_reference(&self, builder: CreateTransitGatewayPrefixListReferenceInputBuilder) -> Result<CreateTransitGatewayPrefixListReferenceOutput, SdkError<CreateTransitGatewayPrefixListReferenceError>>;
        async fn create_transit_gateway_route(&self, builder: CreateTransitGatewayRouteInputBuilder) -> Result<CreateTransitGatewayRouteOutput, SdkError<CreateTransitGatewayRouteError>>;
        async fn create_transit_gateway_route_table(&self, builder: CreateTransitGatewayRouteTableInputBuilder) -> Result<CreateTransitGatewayRouteTableOutput, SdkError<CreateTransitGatewayRouteTableError>>;
        async fn create_transit_gateway_route_table_announcement(&self, builder: CreateTransitGatewayRouteTableAnnouncementInputBuilder) -> Result<CreateTransitGatewayRouteTableAnnouncementOutput, SdkError<CreateTransitGatewayRouteTableAnnouncementError>>;
        async fn create_transit_gateway_vpc_attachment(&self, builder: CreateTransitGatewayVpcAttachmentInputBuilder) -> Result<CreateTransitGatewayVpcAttachmentOutput, SdkError<CreateTransitGatewayVpcAttachmentError>>;
        async fn create_verified_access_endpoint(&self, builder: CreateVerifiedAccessEndpointInputBuilder) -> Result<CreateVerifiedAccessEndpointOutput, SdkError<CreateVerifiedAccessEndpointError>>;
        async fn create_verified_access_group(&self, builder: CreateVerifiedAccessGroupInputBuilder) -> Result<CreateVerifiedAccessGroupOutput, SdkError<CreateVerifiedAccessGroupError>>;
        async fn create_verified_access_instance(&self, builder: CreateVerifiedAccessInstanceInputBuilder) -> Result<CreateVerifiedAccessInstanceOutput, SdkError<CreateVerifiedAccessInstanceError>>;
        async fn create_verified_access_trust_provider(&self, builder: CreateVerifiedAccessTrustProviderInputBuilder) -> Result<CreateVerifiedAccessTrustProviderOutput, SdkError<CreateVerifiedAccessTrustProviderError>>;
        async fn create_volume(&self, builder: CreateVolumeInputBuilder) -> Result<CreateVolumeOutput, SdkError<CreateVolumeError>>;
        async fn create_vpc(&self, builder: CreateVpcInputBuilder) -> Result<CreateVpcOutput, SdkError<CreateVpcError>>;
        async fn create_vpc_endpoint(&self, builder: CreateVpcEndpointInputBuilder) -> Result<CreateVpcEndpointOutput, SdkError<CreateVpcEndpointError>>;
        async fn create_vpc_endpoint_connection_notification(&self, builder: CreateVpcEndpointConnectionNotificationInputBuilder) -> Result<CreateVpcEndpointConnectionNotificationOutput, SdkError<CreateVpcEndpointConnectionNotificationError>>;
        async fn create_vpc_endpoint_service_configuration(&self, builder: CreateVpcEndpointServiceConfigurationInputBuilder) -> Result<CreateVpcEndpointServiceConfigurationOutput, SdkError<CreateVpcEndpointServiceConfigurationError>>;
        async fn create_vpc_peering_connection(&self, builder: CreateVpcPeeringConnectionInputBuilder) -> Result<CreateVpcPeeringConnectionOutput, SdkError<CreateVpcPeeringConnectionError>>;
        async fn create_vpn_connection(&self, builder: CreateVpnConnectionInputBuilder) -> Result<CreateVpnConnectionOutput, SdkError<CreateVpnConnectionError>>;
        async fn create_vpn_connection_route(&self, builder: CreateVpnConnectionRouteInputBuilder) -> Result<CreateVpnConnectionRouteOutput, SdkError<CreateVpnConnectionRouteError>>;
        async fn create_vpn_gateway(&self, builder: CreateVpnGatewayInputBuilder) -> Result<CreateVpnGatewayOutput, SdkError<CreateVpnGatewayError>>;
        async fn delete_carrier_gateway(&self, builder: DeleteCarrierGatewayInputBuilder) -> Result<DeleteCarrierGatewayOutput, SdkError<DeleteCarrierGatewayError>>;
        async fn delete_client_vpn_endpoint(&self, builder: DeleteClientVpnEndpointInputBuilder) -> Result<DeleteClientVpnEndpointOutput, SdkError<DeleteClientVpnEndpointError>>;
        async fn delete_client_vpn_route(&self, builder: DeleteClientVpnRouteInputBuilder) -> Result<DeleteClientVpnRouteOutput, SdkError<DeleteClientVpnRouteError>>;
        async fn delete_coip_cidr(&self, builder: DeleteCoipCidrInputBuilder) -> Result<DeleteCoipCidrOutput, SdkError<DeleteCoipCidrError>>;
        async fn delete_coip_pool(&self, builder: DeleteCoipPoolInputBuilder) -> Result<DeleteCoipPoolOutput, SdkError<DeleteCoipPoolError>>;
        async fn delete_customer_gateway(&self, builder: DeleteCustomerGatewayInputBuilder) -> Result<DeleteCustomerGatewayOutput, SdkError<DeleteCustomerGatewayError>>;
        async fn delete_dhcp_options(&self, builder: DeleteDhcpOptionsInputBuilder) -> Result<DeleteDhcpOptionsOutput, SdkError<DeleteDhcpOptionsError>>;
        async fn delete_egress_only_internet_gateway(&self, builder: DeleteEgressOnlyInternetGatewayInputBuilder) -> Result<DeleteEgressOnlyInternetGatewayOutput, SdkError<DeleteEgressOnlyInternetGatewayError>>;
        async fn delete_fleets(&self, builder: DeleteFleetsInputBuilder) -> Result<DeleteFleetsOutput, SdkError<DeleteFleetsError>>;
        async fn delete_flow_logs(&self, builder: DeleteFlowLogsInputBuilder) -> Result<DeleteFlowLogsOutput, SdkError<DeleteFlowLogsError>>;
        async fn delete_fpga_image(&self, builder: DeleteFpgaImageInputBuilder) -> Result<DeleteFpgaImageOutput, SdkError<DeleteFpgaImageError>>;
        async fn delete_instance_connect_endpoint(&self, builder: DeleteInstanceConnectEndpointInputBuilder) -> Result<DeleteInstanceConnectEndpointOutput, SdkError<DeleteInstanceConnectEndpointError>>;
        async fn delete_instance_event_window(&self, builder: DeleteInstanceEventWindowInputBuilder) -> Result<DeleteInstanceEventWindowOutput, SdkError<DeleteInstanceEventWindowError>>;
        async fn delete_internet_gateway(&self, builder: DeleteInternetGatewayInputBuilder) -> Result<DeleteInternetGatewayOutput, SdkError<DeleteInternetGatewayError>>;
        async fn delete_ipam(&self, builder: DeleteIpamInputBuilder) -> Result<DeleteIpamOutput, SdkError<DeleteIpamError>>;
        async fn delete_ipam_external_resource_verification_token(&self, builder: DeleteIpamExternalResourceVerificationTokenInputBuilder) -> Result<DeleteIpamExternalResourceVerificationTokenOutput, SdkError<DeleteIpamExternalResourceVerificationTokenError>>;
        async fn delete_ipam_pool(&self, builder: DeleteIpamPoolInputBuilder) -> Result<DeleteIpamPoolOutput, SdkError<DeleteIpamPoolError>>;
        async fn delete_ipam_resource_discovery(&self, builder: DeleteIpamResourceDiscoveryInputBuilder) -> Result<DeleteIpamResourceDiscoveryOutput, SdkError<DeleteIpamResourceDiscoveryError>>;
        async fn delete_ipam_scope(&self, builder: DeleteIpamScopeInputBuilder) -> Result<DeleteIpamScopeOutput, SdkError<DeleteIpamScopeError>>;
        async fn delete_key_pair(&self, builder: DeleteKeyPairInputBuilder) -> Result<DeleteKeyPairOutput, SdkError<DeleteKeyPairError>>;
        async fn delete_launch_template(&self, builder: DeleteLaunchTemplateInputBuilder) -> Result<DeleteLaunchTemplateOutput, SdkError<DeleteLaunchTemplateError>>;
        async fn delete_launch_template_versions(&self, builder: DeleteLaunchTemplateVersionsInputBuilder) -> Result<DeleteLaunchTemplateVersionsOutput, SdkError<DeleteLaunchTemplateVersionsError>>;
        async fn delete_local_gateway_route(&self, builder: DeleteLocalGatewayRouteInputBuilder) -> Result<DeleteLocalGatewayRouteOutput, SdkError<DeleteLocalGatewayRouteError>>;
        async fn delete_local_gateway_route_table(&self, builder: DeleteLocalGatewayRouteTableInputBuilder) -> Result<DeleteLocalGatewayRouteTableOutput, SdkError<DeleteLocalGatewayRouteTableError>>;
        async fn delete_local_gateway_route_table_virtual_interface_group_association(&self, builder: DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder) -> Result<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>>;
        async fn delete_local_gateway_route_table_vpc_association(&self, builder: DeleteLocalGatewayRouteTableVpcAssociationInputBuilder) -> Result<DeleteLocalGatewayRouteTableVpcAssociationOutput, SdkError<DeleteLocalGatewayRouteTableVpcAssociationError>>;
        async fn delete_managed_prefix_list(&self, builder: DeleteManagedPrefixListInputBuilder) -> Result<DeleteManagedPrefixListOutput, SdkError<DeleteManagedPrefixListError>>;
        async fn delete_nat_gateway(&self, builder: DeleteNatGatewayInputBuilder) -> Result<DeleteNatGatewayOutput, SdkError<DeleteNatGatewayError>>;
        async fn delete_network_acl(&self, builder: DeleteNetworkAclInputBuilder) -> Result<DeleteNetworkAclOutput, SdkError<DeleteNetworkAclError>>;
        async fn delete_network_acl_entry(&self, builder: DeleteNetworkAclEntryInputBuilder) -> Result<DeleteNetworkAclEntryOutput, SdkError<DeleteNetworkAclEntryError>>;
        async fn delete_network_insights_access_scope(&self, builder: DeleteNetworkInsightsAccessScopeInputBuilder) -> Result<DeleteNetworkInsightsAccessScopeOutput, SdkError<DeleteNetworkInsightsAccessScopeError>>;
        async fn delete_network_insights_access_scope_analysis(&self, builder: DeleteNetworkInsightsAccessScopeAnalysisInputBuilder) -> Result<DeleteNetworkInsightsAccessScopeAnalysisOutput, SdkError<DeleteNetworkInsightsAccessScopeAnalysisError>>;
        async fn delete_network_insights_analysis(&self, builder: DeleteNetworkInsightsAnalysisInputBuilder) -> Result<DeleteNetworkInsightsAnalysisOutput, SdkError<DeleteNetworkInsightsAnalysisError>>;
        async fn delete_network_insights_path(&self, builder: DeleteNetworkInsightsPathInputBuilder) -> Result<DeleteNetworkInsightsPathOutput, SdkError<DeleteNetworkInsightsPathError>>;
        async fn delete_network_interface(&self, builder: DeleteNetworkInterfaceInputBuilder) -> Result<DeleteNetworkInterfaceOutput, SdkError<DeleteNetworkInterfaceError>>;
        async fn delete_network_interface_permission(&self, builder: DeleteNetworkInterfacePermissionInputBuilder) -> Result<DeleteNetworkInterfacePermissionOutput, SdkError<DeleteNetworkInterfacePermissionError>>;
        async fn delete_placement_group(&self, builder: DeletePlacementGroupInputBuilder) -> Result<DeletePlacementGroupOutput, SdkError<DeletePlacementGroupError>>;
        async fn delete_public_ipv4_pool(&self, builder: DeletePublicIpv4PoolInputBuilder) -> Result<DeletePublicIpv4PoolOutput, SdkError<DeletePublicIpv4PoolError>>;
        async fn delete_queued_reserved_instances(&self, builder: DeleteQueuedReservedInstancesInputBuilder) -> Result<DeleteQueuedReservedInstancesOutput, SdkError<DeleteQueuedReservedInstancesError>>;
        async fn delete_route(&self, builder: DeleteRouteInputBuilder) -> Result<DeleteRouteOutput, SdkError<DeleteRouteError>>;
        async fn delete_route_table(&self, builder: DeleteRouteTableInputBuilder) -> Result<DeleteRouteTableOutput, SdkError<DeleteRouteTableError>>;
        async fn delete_security_group(&self, builder: DeleteSecurityGroupInputBuilder) -> Result<DeleteSecurityGroupOutput, SdkError<DeleteSecurityGroupError>>;
        async fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>;
        async fn delete_spot_datafeed_subscription(&self, builder: DeleteSpotDatafeedSubscriptionInputBuilder) -> Result<DeleteSpotDatafeedSubscriptionOutput, SdkError<DeleteSpotDatafeedSubscriptionError>>;
        async fn delete_subnet(&self, builder: DeleteSubnetInputBuilder) -> Result<DeleteSubnetOutput, SdkError<DeleteSubnetError>>;
        async fn delete_subnet_cidr_reservation(&self, builder: DeleteSubnetCidrReservationInputBuilder) -> Result<DeleteSubnetCidrReservationOutput, SdkError<DeleteSubnetCidrReservationError>>;
        async fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> Result<DeleteTagsOutput, SdkError<DeleteTagsError>>;
        async fn delete_traffic_mirror_filter(&self, builder: DeleteTrafficMirrorFilterInputBuilder) -> Result<DeleteTrafficMirrorFilterOutput, SdkError<DeleteTrafficMirrorFilterError>>;
        async fn delete_traffic_mirror_filter_rule(&self, builder: DeleteTrafficMirrorFilterRuleInputBuilder) -> Result<DeleteTrafficMirrorFilterRuleOutput, SdkError<DeleteTrafficMirrorFilterRuleError>>;
        async fn delete_traffic_mirror_session(&self, builder: DeleteTrafficMirrorSessionInputBuilder) -> Result<DeleteTrafficMirrorSessionOutput, SdkError<DeleteTrafficMirrorSessionError>>;
        async fn delete_traffic_mirror_target(&self, builder: DeleteTrafficMirrorTargetInputBuilder) -> Result<DeleteTrafficMirrorTargetOutput, SdkError<DeleteTrafficMirrorTargetError>>;
        async fn delete_transit_gateway(&self, builder: DeleteTransitGatewayInputBuilder) -> Result<DeleteTransitGatewayOutput, SdkError<DeleteTransitGatewayError>>;
        async fn delete_transit_gateway_connect(&self, builder: DeleteTransitGatewayConnectInputBuilder) -> Result<DeleteTransitGatewayConnectOutput, SdkError<DeleteTransitGatewayConnectError>>;
        async fn delete_transit_gateway_connect_peer(&self, builder: DeleteTransitGatewayConnectPeerInputBuilder) -> Result<DeleteTransitGatewayConnectPeerOutput, SdkError<DeleteTransitGatewayConnectPeerError>>;
        async fn delete_transit_gateway_multicast_domain(&self, builder: DeleteTransitGatewayMulticastDomainInputBuilder) -> Result<DeleteTransitGatewayMulticastDomainOutput, SdkError<DeleteTransitGatewayMulticastDomainError>>;
        async fn delete_transit_gateway_peering_attachment(&self, builder: DeleteTransitGatewayPeeringAttachmentInputBuilder) -> Result<DeleteTransitGatewayPeeringAttachmentOutput, SdkError<DeleteTransitGatewayPeeringAttachmentError>>;
        async fn delete_transit_gateway_policy_table(&self, builder: DeleteTransitGatewayPolicyTableInputBuilder) -> Result<DeleteTransitGatewayPolicyTableOutput, SdkError<DeleteTransitGatewayPolicyTableError>>;
        async fn delete_transit_gateway_prefix_list_reference(&self, builder: DeleteTransitGatewayPrefixListReferenceInputBuilder) -> Result<DeleteTransitGatewayPrefixListReferenceOutput, SdkError<DeleteTransitGatewayPrefixListReferenceError>>;
        async fn delete_transit_gateway_route(&self, builder: DeleteTransitGatewayRouteInputBuilder) -> Result<DeleteTransitGatewayRouteOutput, SdkError<DeleteTransitGatewayRouteError>>;
        async fn delete_transit_gateway_route_table(&self, builder: DeleteTransitGatewayRouteTableInputBuilder) -> Result<DeleteTransitGatewayRouteTableOutput, SdkError<DeleteTransitGatewayRouteTableError>>;
        async fn delete_transit_gateway_route_table_announcement(&self, builder: DeleteTransitGatewayRouteTableAnnouncementInputBuilder) -> Result<DeleteTransitGatewayRouteTableAnnouncementOutput, SdkError<DeleteTransitGatewayRouteTableAnnouncementError>>;
        async fn delete_transit_gateway_vpc_attachment(&self, builder: DeleteTransitGatewayVpcAttachmentInputBuilder) -> Result<DeleteTransitGatewayVpcAttachmentOutput, SdkError<DeleteTransitGatewayVpcAttachmentError>>;
        async fn delete_verified_access_endpoint(&self, builder: DeleteVerifiedAccessEndpointInputBuilder) -> Result<DeleteVerifiedAccessEndpointOutput, SdkError<DeleteVerifiedAccessEndpointError>>;
        async fn delete_verified_access_group(&self, builder: DeleteVerifiedAccessGroupInputBuilder) -> Result<DeleteVerifiedAccessGroupOutput, SdkError<DeleteVerifiedAccessGroupError>>;
        async fn delete_verified_access_instance(&self, builder: DeleteVerifiedAccessInstanceInputBuilder) -> Result<DeleteVerifiedAccessInstanceOutput, SdkError<DeleteVerifiedAccessInstanceError>>;
        async fn delete_verified_access_trust_provider(&self, builder: DeleteVerifiedAccessTrustProviderInputBuilder) -> Result<DeleteVerifiedAccessTrustProviderOutput, SdkError<DeleteVerifiedAccessTrustProviderError>>;
        async fn delete_volume(&self, builder: DeleteVolumeInputBuilder) -> Result<DeleteVolumeOutput, SdkError<DeleteVolumeError>>;
        async fn delete_vpc(&self, builder: DeleteVpcInputBuilder) -> Result<DeleteVpcOutput, SdkError<DeleteVpcError>>;
        async fn delete_vpc_endpoint_connection_notifications(&self, builder: DeleteVpcEndpointConnectionNotificationsInputBuilder) -> Result<DeleteVpcEndpointConnectionNotificationsOutput, SdkError<DeleteVpcEndpointConnectionNotificationsError>>;
        async fn delete_vpc_endpoint_service_configurations(&self, builder: DeleteVpcEndpointServiceConfigurationsInputBuilder) -> Result<DeleteVpcEndpointServiceConfigurationsOutput, SdkError<DeleteVpcEndpointServiceConfigurationsError>>;
        async fn delete_vpc_endpoints(&self, builder: DeleteVpcEndpointsInputBuilder) -> Result<DeleteVpcEndpointsOutput, SdkError<DeleteVpcEndpointsError>>;
        async fn delete_vpc_peering_connection(&self, builder: DeleteVpcPeeringConnectionInputBuilder) -> Result<DeleteVpcPeeringConnectionOutput, SdkError<DeleteVpcPeeringConnectionError>>;
        async fn delete_vpn_connection(&self, builder: DeleteVpnConnectionInputBuilder) -> Result<DeleteVpnConnectionOutput, SdkError<DeleteVpnConnectionError>>;
        async fn delete_vpn_connection_route(&self, builder: DeleteVpnConnectionRouteInputBuilder) -> Result<DeleteVpnConnectionRouteOutput, SdkError<DeleteVpnConnectionRouteError>>;
        async fn delete_vpn_gateway(&self, builder: DeleteVpnGatewayInputBuilder) -> Result<DeleteVpnGatewayOutput, SdkError<DeleteVpnGatewayError>>;
        async fn deprovision_byoip_cidr(&self, builder: DeprovisionByoipCidrInputBuilder) -> Result<DeprovisionByoipCidrOutput, SdkError<DeprovisionByoipCidrError>>;
        async fn deprovision_ipam_byoasn(&self, builder: DeprovisionIpamByoasnInputBuilder) -> Result<DeprovisionIpamByoasnOutput, SdkError<DeprovisionIpamByoasnError>>;
        async fn deprovision_ipam_pool_cidr(&self, builder: DeprovisionIpamPoolCidrInputBuilder) -> Result<DeprovisionIpamPoolCidrOutput, SdkError<DeprovisionIpamPoolCidrError>>;
        async fn deprovision_public_ipv4_pool_cidr(&self, builder: DeprovisionPublicIpv4PoolCidrInputBuilder) -> Result<DeprovisionPublicIpv4PoolCidrOutput, SdkError<DeprovisionPublicIpv4PoolCidrError>>;
        async fn deregister_image(&self, builder: DeregisterImageInputBuilder) -> Result<DeregisterImageOutput, SdkError<DeregisterImageError>>;
        async fn deregister_instance_event_notification_attributes(&self, builder: DeregisterInstanceEventNotificationAttributesInputBuilder) -> Result<DeregisterInstanceEventNotificationAttributesOutput, SdkError<DeregisterInstanceEventNotificationAttributesError>>;
        async fn deregister_transit_gateway_multicast_group_members(&self, builder: DeregisterTransitGatewayMulticastGroupMembersInputBuilder) -> Result<DeregisterTransitGatewayMulticastGroupMembersOutput, SdkError<DeregisterTransitGatewayMulticastGroupMembersError>>;
        async fn deregister_transit_gateway_multicast_group_sources(&self, builder: DeregisterTransitGatewayMulticastGroupSourcesInputBuilder) -> Result<DeregisterTransitGatewayMulticastGroupSourcesOutput, SdkError<DeregisterTransitGatewayMulticastGroupSourcesError>>;
        async fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>;
        async fn describe_address_transfers(&self, builder: DescribeAddressTransfersInputBuilder) -> Result<DescribeAddressTransfersOutput, SdkError<DescribeAddressTransfersError>>;
        async fn describe_addresses(&self, builder: DescribeAddressesInputBuilder) -> Result<DescribeAddressesOutput, SdkError<DescribeAddressesError>>;
        async fn describe_addresses_attribute(&self, builder: DescribeAddressesAttributeInputBuilder) -> Result<DescribeAddressesAttributeOutput, SdkError<DescribeAddressesAttributeError>>;
        async fn describe_aggregate_id_format(&self, builder: DescribeAggregateIdFormatInputBuilder) -> Result<DescribeAggregateIdFormatOutput, SdkError<DescribeAggregateIdFormatError>>;
        async fn describe_availability_zones(&self, builder: DescribeAvailabilityZonesInputBuilder) -> Result<DescribeAvailabilityZonesOutput, SdkError<DescribeAvailabilityZonesError>>;
        async fn describe_aws_network_performance_metric_subscriptions(&self, builder: DescribeAwsNetworkPerformanceMetricSubscriptionsInputBuilder) -> Result<DescribeAwsNetworkPerformanceMetricSubscriptionsOutput, SdkError<DescribeAwsNetworkPerformanceMetricSubscriptionsError>>;
        async fn describe_bundle_tasks(&self, builder: DescribeBundleTasksInputBuilder) -> Result<DescribeBundleTasksOutput, SdkError<DescribeBundleTasksError>>;
        async fn describe_byoip_cidrs(&self, builder: DescribeByoipCidrsInputBuilder) -> Result<DescribeByoipCidrsOutput, SdkError<DescribeByoipCidrsError>>;
        async fn describe_capacity_block_offerings(&self, builder: DescribeCapacityBlockOfferingsInputBuilder) -> Result<DescribeCapacityBlockOfferingsOutput, SdkError<DescribeCapacityBlockOfferingsError>>;
        async fn describe_capacity_reservation_fleets(&self, builder: DescribeCapacityReservationFleetsInputBuilder) -> Result<DescribeCapacityReservationFleetsOutput, SdkError<DescribeCapacityReservationFleetsError>>;
        async fn describe_capacity_reservations(&self, builder: DescribeCapacityReservationsInputBuilder) -> Result<DescribeCapacityReservationsOutput, SdkError<DescribeCapacityReservationsError>>;
        async fn describe_carrier_gateways(&self, builder: DescribeCarrierGatewaysInputBuilder) -> Result<DescribeCarrierGatewaysOutput, SdkError<DescribeCarrierGatewaysError>>;
        async fn describe_classic_link_instances(&self, builder: DescribeClassicLinkInstancesInputBuilder) -> Result<DescribeClassicLinkInstancesOutput, SdkError<DescribeClassicLinkInstancesError>>;
        async fn describe_client_vpn_authorization_rules(&self, builder: DescribeClientVpnAuthorizationRulesInputBuilder) -> Result<DescribeClientVpnAuthorizationRulesOutput, SdkError<DescribeClientVpnAuthorizationRulesError>>;
        async fn describe_client_vpn_connections(&self, builder: DescribeClientVpnConnectionsInputBuilder) -> Result<DescribeClientVpnConnectionsOutput, SdkError<DescribeClientVpnConnectionsError>>;
        async fn describe_client_vpn_endpoints(&self, builder: DescribeClientVpnEndpointsInputBuilder) -> Result<DescribeClientVpnEndpointsOutput, SdkError<DescribeClientVpnEndpointsError>>;
        async fn describe_client_vpn_routes(&self, builder: DescribeClientVpnRoutesInputBuilder) -> Result<DescribeClientVpnRoutesOutput, SdkError<DescribeClientVpnRoutesError>>;
        async fn describe_client_vpn_target_networks(&self, builder: DescribeClientVpnTargetNetworksInputBuilder) -> Result<DescribeClientVpnTargetNetworksOutput, SdkError<DescribeClientVpnTargetNetworksError>>;
        async fn describe_coip_pools(&self, builder: DescribeCoipPoolsInputBuilder) -> Result<DescribeCoipPoolsOutput, SdkError<DescribeCoipPoolsError>>;
        async fn describe_conversion_tasks(&self, builder: DescribeConversionTasksInputBuilder) -> Result<DescribeConversionTasksOutput, SdkError<DescribeConversionTasksError>>;
        async fn describe_customer_gateways(&self, builder: DescribeCustomerGatewaysInputBuilder) -> Result<DescribeCustomerGatewaysOutput, SdkError<DescribeCustomerGatewaysError>>;
        async fn describe_dhcp_options(&self, builder: DescribeDhcpOptionsInputBuilder) -> Result<DescribeDhcpOptionsOutput, SdkError<DescribeDhcpOptionsError>>;
        async fn describe_egress_only_internet_gateways(&self, builder: DescribeEgressOnlyInternetGatewaysInputBuilder) -> Result<DescribeEgressOnlyInternetGatewaysOutput, SdkError<DescribeEgressOnlyInternetGatewaysError>>;
        async fn describe_elastic_gpus(&self, builder: DescribeElasticGpusInputBuilder) -> Result<DescribeElasticGpusOutput, SdkError<DescribeElasticGpusError>>;
        async fn describe_export_image_tasks(&self, builder: DescribeExportImageTasksInputBuilder) -> Result<DescribeExportImageTasksOutput, SdkError<DescribeExportImageTasksError>>;
        async fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>;
        async fn describe_fast_launch_images(&self, builder: DescribeFastLaunchImagesInputBuilder) -> Result<DescribeFastLaunchImagesOutput, SdkError<DescribeFastLaunchImagesError>>;
        async fn describe_fast_snapshot_restores(&self, builder: DescribeFastSnapshotRestoresInputBuilder) -> Result<DescribeFastSnapshotRestoresOutput, SdkError<DescribeFastSnapshotRestoresError>>;
        async fn describe_fleet_history(&self, builder: DescribeFleetHistoryInputBuilder) -> Result<DescribeFleetHistoryOutput, SdkError<DescribeFleetHistoryError>>;
        async fn describe_fleet_instances(&self, builder: DescribeFleetInstancesInputBuilder) -> Result<DescribeFleetInstancesOutput, SdkError<DescribeFleetInstancesError>>;
        async fn describe_fleets(&self, builder: DescribeFleetsInputBuilder) -> Result<DescribeFleetsOutput, SdkError<DescribeFleetsError>>;
        async fn describe_flow_logs(&self, builder: DescribeFlowLogsInputBuilder) -> Result<DescribeFlowLogsOutput, SdkError<DescribeFlowLogsError>>;
        async fn describe_fpga_image_attribute(&self, builder: DescribeFpgaImageAttributeInputBuilder) -> Result<DescribeFpgaImageAttributeOutput, SdkError<DescribeFpgaImageAttributeError>>;
        async fn describe_fpga_images(&self, builder: DescribeFpgaImagesInputBuilder) -> Result<DescribeFpgaImagesOutput, SdkError<DescribeFpgaImagesError>>;
        async fn describe_host_reservation_offerings(&self, builder: DescribeHostReservationOfferingsInputBuilder) -> Result<DescribeHostReservationOfferingsOutput, SdkError<DescribeHostReservationOfferingsError>>;
        async fn describe_host_reservations(&self, builder: DescribeHostReservationsInputBuilder) -> Result<DescribeHostReservationsOutput, SdkError<DescribeHostReservationsError>>;
        async fn describe_hosts(&self, builder: DescribeHostsInputBuilder) -> Result<DescribeHostsOutput, SdkError<DescribeHostsError>>;
        async fn describe_iam_instance_profile_associations(&self, builder: DescribeIamInstanceProfileAssociationsInputBuilder) -> Result<DescribeIamInstanceProfileAssociationsOutput, SdkError<DescribeIamInstanceProfileAssociationsError>>;
        async fn describe_id_format(&self, builder: DescribeIdFormatInputBuilder) -> Result<DescribeIdFormatOutput, SdkError<DescribeIdFormatError>>;
        async fn describe_identity_id_format(&self, builder: DescribeIdentityIdFormatInputBuilder) -> Result<DescribeIdentityIdFormatOutput, SdkError<DescribeIdentityIdFormatError>>;
        async fn describe_image_attribute(&self, builder: DescribeImageAttributeInputBuilder) -> Result<DescribeImageAttributeOutput, SdkError<DescribeImageAttributeError>>;
        async fn describe_images(&self, builder: DescribeImagesInputBuilder) -> Result<DescribeImagesOutput, SdkError<DescribeImagesError>>;
        async fn describe_import_image_tasks(&self, builder: DescribeImportImageTasksInputBuilder) -> Result<DescribeImportImageTasksOutput, SdkError<DescribeImportImageTasksError>>;
        async fn describe_import_snapshot_tasks(&self, builder: DescribeImportSnapshotTasksInputBuilder) -> Result<DescribeImportSnapshotTasksOutput, SdkError<DescribeImportSnapshotTasksError>>;
        async fn describe_instance_attribute(&self, builder: DescribeInstanceAttributeInputBuilder) -> Result<DescribeInstanceAttributeOutput, SdkError<DescribeInstanceAttributeError>>;
        async fn describe_instance_connect_endpoints(&self, builder: DescribeInstanceConnectEndpointsInputBuilder) -> Result<DescribeInstanceConnectEndpointsOutput, SdkError<DescribeInstanceConnectEndpointsError>>;
        async fn describe_instance_credit_specifications(&self, builder: DescribeInstanceCreditSpecificationsInputBuilder) -> Result<DescribeInstanceCreditSpecificationsOutput, SdkError<DescribeInstanceCreditSpecificationsError>>;
        async fn describe_instance_event_notification_attributes(&self, builder: DescribeInstanceEventNotificationAttributesInputBuilder) -> Result<DescribeInstanceEventNotificationAttributesOutput, SdkError<DescribeInstanceEventNotificationAttributesError>>;
        async fn describe_instance_event_windows(&self, builder: DescribeInstanceEventWindowsInputBuilder) -> Result<DescribeInstanceEventWindowsOutput, SdkError<DescribeInstanceEventWindowsError>>;
        async fn describe_instance_status(&self, builder: DescribeInstanceStatusInputBuilder) -> Result<DescribeInstanceStatusOutput, SdkError<DescribeInstanceStatusError>>;
        async fn describe_instance_topology(&self, builder: DescribeInstanceTopologyInputBuilder) -> Result<DescribeInstanceTopologyOutput, SdkError<DescribeInstanceTopologyError>>;
        async fn describe_instance_type_offerings(&self, builder: DescribeInstanceTypeOfferingsInputBuilder) -> Result<DescribeInstanceTypeOfferingsOutput, SdkError<DescribeInstanceTypeOfferingsError>>;
        async fn describe_instance_types(&self, builder: DescribeInstanceTypesInputBuilder) -> Result<DescribeInstanceTypesOutput, SdkError<DescribeInstanceTypesError>>;
        async fn describe_instances(&self, builder: DescribeInstancesInputBuilder) -> Result<DescribeInstancesOutput, SdkError<DescribeInstancesError>>;
        async fn describe_internet_gateways(&self, builder: DescribeInternetGatewaysInputBuilder) -> Result<DescribeInternetGatewaysOutput, SdkError<DescribeInternetGatewaysError>>;
        async fn describe_ipam_byoasn(&self, builder: DescribeIpamByoasnInputBuilder) -> Result<DescribeIpamByoasnOutput, SdkError<DescribeIpamByoasnError>>;
        async fn describe_ipam_external_resource_verification_tokens(&self, builder: DescribeIpamExternalResourceVerificationTokensInputBuilder) -> Result<DescribeIpamExternalResourceVerificationTokensOutput, SdkError<DescribeIpamExternalResourceVerificationTokensError>>;
        async fn describe_ipam_pools(&self, builder: DescribeIpamPoolsInputBuilder) -> Result<DescribeIpamPoolsOutput, SdkError<DescribeIpamPoolsError>>;
        async fn describe_ipam_resource_discoveries(&self, builder: DescribeIpamResourceDiscoveriesInputBuilder) -> Result<DescribeIpamResourceDiscoveriesOutput, SdkError<DescribeIpamResourceDiscoveriesError>>;
        async fn describe_ipam_resource_discovery_associations(&self, builder: DescribeIpamResourceDiscoveryAssociationsInputBuilder) -> Result<DescribeIpamResourceDiscoveryAssociationsOutput, SdkError<DescribeIpamResourceDiscoveryAssociationsError>>;
        async fn describe_ipam_scopes(&self, builder: DescribeIpamScopesInputBuilder) -> Result<DescribeIpamScopesOutput, SdkError<DescribeIpamScopesError>>;
        async fn describe_ipams(&self, builder: DescribeIpamsInputBuilder) -> Result<DescribeIpamsOutput, SdkError<DescribeIpamsError>>;
        async fn describe_ipv6_pools(&self, builder: DescribeIpv6PoolsInputBuilder) -> Result<DescribeIpv6PoolsOutput, SdkError<DescribeIpv6PoolsError>>;
        async fn describe_key_pairs(&self, builder: DescribeKeyPairsInputBuilder) -> Result<DescribeKeyPairsOutput, SdkError<DescribeKeyPairsError>>;
        async fn describe_launch_template_versions(&self, builder: DescribeLaunchTemplateVersionsInputBuilder) -> Result<DescribeLaunchTemplateVersionsOutput, SdkError<DescribeLaunchTemplateVersionsError>>;
        async fn describe_launch_templates(&self, builder: DescribeLaunchTemplatesInputBuilder) -> Result<DescribeLaunchTemplatesOutput, SdkError<DescribeLaunchTemplatesError>>;
        async fn describe_local_gateway_route_table_virtual_interface_group_associations(&self, builder: DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder) -> Result<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsError>>;
        async fn describe_local_gateway_route_table_vpc_associations(&self, builder: DescribeLocalGatewayRouteTableVpcAssociationsInputBuilder) -> Result<DescribeLocalGatewayRouteTableVpcAssociationsOutput, SdkError<DescribeLocalGatewayRouteTableVpcAssociationsError>>;
        async fn describe_local_gateway_route_tables(&self, builder: DescribeLocalGatewayRouteTablesInputBuilder) -> Result<DescribeLocalGatewayRouteTablesOutput, SdkError<DescribeLocalGatewayRouteTablesError>>;
        async fn describe_local_gateway_virtual_interface_groups(&self, builder: DescribeLocalGatewayVirtualInterfaceGroupsInputBuilder) -> Result<DescribeLocalGatewayVirtualInterfaceGroupsOutput, SdkError<DescribeLocalGatewayVirtualInterfaceGroupsError>>;
        async fn describe_local_gateway_virtual_interfaces(&self, builder: DescribeLocalGatewayVirtualInterfacesInputBuilder) -> Result<DescribeLocalGatewayVirtualInterfacesOutput, SdkError<DescribeLocalGatewayVirtualInterfacesError>>;
        async fn describe_local_gateways(&self, builder: DescribeLocalGatewaysInputBuilder) -> Result<DescribeLocalGatewaysOutput, SdkError<DescribeLocalGatewaysError>>;
        async fn describe_locked_snapshots(&self, builder: DescribeLockedSnapshotsInputBuilder) -> Result<DescribeLockedSnapshotsOutput, SdkError<DescribeLockedSnapshotsError>>;
        async fn describe_mac_hosts(&self, builder: DescribeMacHostsInputBuilder) -> Result<DescribeMacHostsOutput, SdkError<DescribeMacHostsError>>;
        async fn describe_managed_prefix_lists(&self, builder: DescribeManagedPrefixListsInputBuilder) -> Result<DescribeManagedPrefixListsOutput, SdkError<DescribeManagedPrefixListsError>>;
        async fn describe_moving_addresses(&self, builder: DescribeMovingAddressesInputBuilder) -> Result<DescribeMovingAddressesOutput, SdkError<DescribeMovingAddressesError>>;
        async fn describe_nat_gateways(&self, builder: DescribeNatGatewaysInputBuilder) -> Result<DescribeNatGatewaysOutput, SdkError<DescribeNatGatewaysError>>;
        async fn describe_network_acls(&self, builder: DescribeNetworkAclsInputBuilder) -> Result<DescribeNetworkAclsOutput, SdkError<DescribeNetworkAclsError>>;
        async fn describe_network_insights_access_scope_analyses(&self, builder: DescribeNetworkInsightsAccessScopeAnalysesInputBuilder) -> Result<DescribeNetworkInsightsAccessScopeAnalysesOutput, SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>>;
        async fn describe_network_insights_access_scopes(&self, builder: DescribeNetworkInsightsAccessScopesInputBuilder) -> Result<DescribeNetworkInsightsAccessScopesOutput, SdkError<DescribeNetworkInsightsAccessScopesError>>;
        async fn describe_network_insights_analyses(&self, builder: DescribeNetworkInsightsAnalysesInputBuilder) -> Result<DescribeNetworkInsightsAnalysesOutput, SdkError<DescribeNetworkInsightsAnalysesError>>;
        async fn describe_network_insights_paths(&self, builder: DescribeNetworkInsightsPathsInputBuilder) -> Result<DescribeNetworkInsightsPathsOutput, SdkError<DescribeNetworkInsightsPathsError>>;
        async fn describe_network_interface_attribute(&self, builder: DescribeNetworkInterfaceAttributeInputBuilder) -> Result<DescribeNetworkInterfaceAttributeOutput, SdkError<DescribeNetworkInterfaceAttributeError>>;
        async fn describe_network_interface_permissions(&self, builder: DescribeNetworkInterfacePermissionsInputBuilder) -> Result<DescribeNetworkInterfacePermissionsOutput, SdkError<DescribeNetworkInterfacePermissionsError>>;
        async fn describe_network_interfaces(&self, builder: DescribeNetworkInterfacesInputBuilder) -> Result<DescribeNetworkInterfacesOutput, SdkError<DescribeNetworkInterfacesError>>;
        async fn describe_placement_groups(&self, builder: DescribePlacementGroupsInputBuilder) -> Result<DescribePlacementGroupsOutput, SdkError<DescribePlacementGroupsError>>;
        async fn describe_prefix_lists(&self, builder: DescribePrefixListsInputBuilder) -> Result<DescribePrefixListsOutput, SdkError<DescribePrefixListsError>>;
        async fn describe_principal_id_format(&self, builder: DescribePrincipalIdFormatInputBuilder) -> Result<DescribePrincipalIdFormatOutput, SdkError<DescribePrincipalIdFormatError>>;
        async fn describe_public_ipv4_pools(&self, builder: DescribePublicIpv4PoolsInputBuilder) -> Result<DescribePublicIpv4PoolsOutput, SdkError<DescribePublicIpv4PoolsError>>;
        async fn describe_regions(&self, builder: DescribeRegionsInputBuilder) -> Result<DescribeRegionsOutput, SdkError<DescribeRegionsError>>;
        async fn describe_replace_root_volume_tasks(&self, builder: DescribeReplaceRootVolumeTasksInputBuilder) -> Result<DescribeReplaceRootVolumeTasksOutput, SdkError<DescribeReplaceRootVolumeTasksError>>;
        async fn describe_reserved_instances(&self, builder: DescribeReservedInstancesInputBuilder) -> Result<DescribeReservedInstancesOutput, SdkError<DescribeReservedInstancesError>>;
        async fn describe_reserved_instances_listings(&self, builder: DescribeReservedInstancesListingsInputBuilder) -> Result<DescribeReservedInstancesListingsOutput, SdkError<DescribeReservedInstancesListingsError>>;
        async fn describe_reserved_instances_modifications(&self, builder: DescribeReservedInstancesModificationsInputBuilder) -> Result<DescribeReservedInstancesModificationsOutput, SdkError<DescribeReservedInstancesModificationsError>>;
        async fn describe_reserved_instances_offerings(&self, builder: DescribeReservedInstancesOfferingsInputBuilder) -> Result<DescribeReservedInstancesOfferingsOutput, SdkError<DescribeReservedInstancesOfferingsError>>;
        async fn describe_route_tables(&self, builder: DescribeRouteTablesInputBuilder) -> Result<DescribeRouteTablesOutput, SdkError<DescribeRouteTablesError>>;
        async fn describe_scheduled_instance_availability(&self, builder: DescribeScheduledInstanceAvailabilityInputBuilder) -> Result<DescribeScheduledInstanceAvailabilityOutput, SdkError<DescribeScheduledInstanceAvailabilityError>>;
        async fn describe_scheduled_instances(&self, builder: DescribeScheduledInstancesInputBuilder) -> Result<DescribeScheduledInstancesOutput, SdkError<DescribeScheduledInstancesError>>;
        async fn describe_security_group_references(&self, builder: DescribeSecurityGroupReferencesInputBuilder) -> Result<DescribeSecurityGroupReferencesOutput, SdkError<DescribeSecurityGroupReferencesError>>;
        async fn describe_security_group_rules(&self, builder: DescribeSecurityGroupRulesInputBuilder) -> Result<DescribeSecurityGroupRulesOutput, SdkError<DescribeSecurityGroupRulesError>>;
        async fn describe_security_groups(&self, builder: DescribeSecurityGroupsInputBuilder) -> Result<DescribeSecurityGroupsOutput, SdkError<DescribeSecurityGroupsError>>;
        async fn describe_snapshot_attribute(&self, builder: DescribeSnapshotAttributeInputBuilder) -> Result<DescribeSnapshotAttributeOutput, SdkError<DescribeSnapshotAttributeError>>;
        async fn describe_snapshot_tier_status(&self, builder: DescribeSnapshotTierStatusInputBuilder) -> Result<DescribeSnapshotTierStatusOutput, SdkError<DescribeSnapshotTierStatusError>>;
        async fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>;
        async fn describe_spot_datafeed_subscription(&self, builder: DescribeSpotDatafeedSubscriptionInputBuilder) -> Result<DescribeSpotDatafeedSubscriptionOutput, SdkError<DescribeSpotDatafeedSubscriptionError>>;
        async fn describe_spot_fleet_instances(&self, builder: DescribeSpotFleetInstancesInputBuilder) -> Result<DescribeSpotFleetInstancesOutput, SdkError<DescribeSpotFleetInstancesError>>;
        async fn describe_spot_fleet_request_history(&self, builder: DescribeSpotFleetRequestHistoryInputBuilder) -> Result<DescribeSpotFleetRequestHistoryOutput, SdkError<DescribeSpotFleetRequestHistoryError>>;
        async fn describe_spot_fleet_requests(&self, builder: DescribeSpotFleetRequestsInputBuilder) -> Result<DescribeSpotFleetRequestsOutput, SdkError<DescribeSpotFleetRequestsError>>;
        async fn describe_spot_instance_requests(&self, builder: DescribeSpotInstanceRequestsInputBuilder) -> Result<DescribeSpotInstanceRequestsOutput, SdkError<DescribeSpotInstanceRequestsError>>;
        async fn describe_spot_price_history(&self, builder: DescribeSpotPriceHistoryInputBuilder) -> Result<DescribeSpotPriceHistoryOutput, SdkError<DescribeSpotPriceHistoryError>>;
        async fn describe_stale_security_groups(&self, builder: DescribeStaleSecurityGroupsInputBuilder) -> Result<DescribeStaleSecurityGroupsOutput, SdkError<DescribeStaleSecurityGroupsError>>;
        async fn describe_store_image_tasks(&self, builder: DescribeStoreImageTasksInputBuilder) -> Result<DescribeStoreImageTasksOutput, SdkError<DescribeStoreImageTasksError>>;
        async fn describe_subnets(&self, builder: DescribeSubnetsInputBuilder) -> Result<DescribeSubnetsOutput, SdkError<DescribeSubnetsError>>;
        async fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> Result<DescribeTagsOutput, SdkError<DescribeTagsError>>;
        async fn describe_traffic_mirror_filter_rules(&self, builder: DescribeTrafficMirrorFilterRulesInputBuilder) -> Result<DescribeTrafficMirrorFilterRulesOutput, SdkError<DescribeTrafficMirrorFilterRulesError>>;
        async fn describe_traffic_mirror_filters(&self, builder: DescribeTrafficMirrorFiltersInputBuilder) -> Result<DescribeTrafficMirrorFiltersOutput, SdkError<DescribeTrafficMirrorFiltersError>>;
        async fn describe_traffic_mirror_sessions(&self, builder: DescribeTrafficMirrorSessionsInputBuilder) -> Result<DescribeTrafficMirrorSessionsOutput, SdkError<DescribeTrafficMirrorSessionsError>>;
        async fn describe_traffic_mirror_targets(&self, builder: DescribeTrafficMirrorTargetsInputBuilder) -> Result<DescribeTrafficMirrorTargetsOutput, SdkError<DescribeTrafficMirrorTargetsError>>;
        async fn describe_transit_gateway_attachments(&self, builder: DescribeTransitGatewayAttachmentsInputBuilder) -> Result<DescribeTransitGatewayAttachmentsOutput, SdkError<DescribeTransitGatewayAttachmentsError>>;
        async fn describe_transit_gateway_connect_peers(&self, builder: DescribeTransitGatewayConnectPeersInputBuilder) -> Result<DescribeTransitGatewayConnectPeersOutput, SdkError<DescribeTransitGatewayConnectPeersError>>;
        async fn describe_transit_gateway_connects(&self, builder: DescribeTransitGatewayConnectsInputBuilder) -> Result<DescribeTransitGatewayConnectsOutput, SdkError<DescribeTransitGatewayConnectsError>>;
        async fn describe_transit_gateway_multicast_domains(&self, builder: DescribeTransitGatewayMulticastDomainsInputBuilder) -> Result<DescribeTransitGatewayMulticastDomainsOutput, SdkError<DescribeTransitGatewayMulticastDomainsError>>;
        async fn describe_transit_gateway_peering_attachments(&self, builder: DescribeTransitGatewayPeeringAttachmentsInputBuilder) -> Result<DescribeTransitGatewayPeeringAttachmentsOutput, SdkError<DescribeTransitGatewayPeeringAttachmentsError>>;
        async fn describe_transit_gateway_policy_tables(&self, builder: DescribeTransitGatewayPolicyTablesInputBuilder) -> Result<DescribeTransitGatewayPolicyTablesOutput, SdkError<DescribeTransitGatewayPolicyTablesError>>;
        async fn describe_transit_gateway_route_table_announcements(&self, builder: DescribeTransitGatewayRouteTableAnnouncementsInputBuilder) -> Result<DescribeTransitGatewayRouteTableAnnouncementsOutput, SdkError<DescribeTransitGatewayRouteTableAnnouncementsError>>;
        async fn describe_transit_gateway_route_tables(&self, builder: DescribeTransitGatewayRouteTablesInputBuilder) -> Result<DescribeTransitGatewayRouteTablesOutput, SdkError<DescribeTransitGatewayRouteTablesError>>;
        async fn describe_transit_gateway_vpc_attachments(&self, builder: DescribeTransitGatewayVpcAttachmentsInputBuilder) -> Result<DescribeTransitGatewayVpcAttachmentsOutput, SdkError<DescribeTransitGatewayVpcAttachmentsError>>;
        async fn describe_transit_gateways(&self, builder: DescribeTransitGatewaysInputBuilder) -> Result<DescribeTransitGatewaysOutput, SdkError<DescribeTransitGatewaysError>>;
        async fn describe_trunk_interface_associations(&self, builder: DescribeTrunkInterfaceAssociationsInputBuilder) -> Result<DescribeTrunkInterfaceAssociationsOutput, SdkError<DescribeTrunkInterfaceAssociationsError>>;
        async fn describe_verified_access_endpoints(&self, builder: DescribeVerifiedAccessEndpointsInputBuilder) -> Result<DescribeVerifiedAccessEndpointsOutput, SdkError<DescribeVerifiedAccessEndpointsError>>;
        async fn describe_verified_access_groups(&self, builder: DescribeVerifiedAccessGroupsInputBuilder) -> Result<DescribeVerifiedAccessGroupsOutput, SdkError<DescribeVerifiedAccessGroupsError>>;
        async fn describe_verified_access_instance_logging_configurations(&self, builder: DescribeVerifiedAccessInstanceLoggingConfigurationsInputBuilder) -> Result<DescribeVerifiedAccessInstanceLoggingConfigurationsOutput, SdkError<DescribeVerifiedAccessInstanceLoggingConfigurationsError>>;
        async fn describe_verified_access_instances(&self, builder: DescribeVerifiedAccessInstancesInputBuilder) -> Result<DescribeVerifiedAccessInstancesOutput, SdkError<DescribeVerifiedAccessInstancesError>>;
        async fn describe_verified_access_trust_providers(&self, builder: DescribeVerifiedAccessTrustProvidersInputBuilder) -> Result<DescribeVerifiedAccessTrustProvidersOutput, SdkError<DescribeVerifiedAccessTrustProvidersError>>;
        async fn describe_volume_attribute(&self, builder: DescribeVolumeAttributeInputBuilder) -> Result<DescribeVolumeAttributeOutput, SdkError<DescribeVolumeAttributeError>>;
        async fn describe_volume_status(&self, builder: DescribeVolumeStatusInputBuilder) -> Result<DescribeVolumeStatusOutput, SdkError<DescribeVolumeStatusError>>;
        async fn describe_volumes(&self, builder: DescribeVolumesInputBuilder) -> Result<DescribeVolumesOutput, SdkError<DescribeVolumesError>>;
        async fn describe_volumes_modifications(&self, builder: DescribeVolumesModificationsInputBuilder) -> Result<DescribeVolumesModificationsOutput, SdkError<DescribeVolumesModificationsError>>;
        async fn describe_vpc_attribute(&self, builder: DescribeVpcAttributeInputBuilder) -> Result<DescribeVpcAttributeOutput, SdkError<DescribeVpcAttributeError>>;
        async fn describe_vpc_classic_link(&self, builder: DescribeVpcClassicLinkInputBuilder) -> Result<DescribeVpcClassicLinkOutput, SdkError<DescribeVpcClassicLinkError>>;
        async fn describe_vpc_classic_link_dns_support(&self, builder: DescribeVpcClassicLinkDnsSupportInputBuilder) -> Result<DescribeVpcClassicLinkDnsSupportOutput, SdkError<DescribeVpcClassicLinkDnsSupportError>>;
        async fn describe_vpc_endpoint_connection_notifications(&self, builder: DescribeVpcEndpointConnectionNotificationsInputBuilder) -> Result<DescribeVpcEndpointConnectionNotificationsOutput, SdkError<DescribeVpcEndpointConnectionNotificationsError>>;
        async fn describe_vpc_endpoint_connections(&self, builder: DescribeVpcEndpointConnectionsInputBuilder) -> Result<DescribeVpcEndpointConnectionsOutput, SdkError<DescribeVpcEndpointConnectionsError>>;
        async fn describe_vpc_endpoint_service_configurations(&self, builder: DescribeVpcEndpointServiceConfigurationsInputBuilder) -> Result<DescribeVpcEndpointServiceConfigurationsOutput, SdkError<DescribeVpcEndpointServiceConfigurationsError>>;
        async fn describe_vpc_endpoint_service_permissions(&self, builder: DescribeVpcEndpointServicePermissionsInputBuilder) -> Result<DescribeVpcEndpointServicePermissionsOutput, SdkError<DescribeVpcEndpointServicePermissionsError>>;
        async fn describe_vpc_endpoint_services(&self, builder: DescribeVpcEndpointServicesInputBuilder) -> Result<DescribeVpcEndpointServicesOutput, SdkError<DescribeVpcEndpointServicesError>>;
        async fn describe_vpc_endpoints(&self, builder: DescribeVpcEndpointsInputBuilder) -> Result<DescribeVpcEndpointsOutput, SdkError<DescribeVpcEndpointsError>>;
        async fn describe_vpc_peering_connections(&self, builder: DescribeVpcPeeringConnectionsInputBuilder) -> Result<DescribeVpcPeeringConnectionsOutput, SdkError<DescribeVpcPeeringConnectionsError>>;
        async fn describe_vpcs(&self, builder: DescribeVpcsInputBuilder) -> Result<DescribeVpcsOutput, SdkError<DescribeVpcsError>>;
        async fn describe_vpn_connections(&self, builder: DescribeVpnConnectionsInputBuilder) -> Result<DescribeVpnConnectionsOutput, SdkError<DescribeVpnConnectionsError>>;
        async fn describe_vpn_gateways(&self, builder: DescribeVpnGatewaysInputBuilder) -> Result<DescribeVpnGatewaysOutput, SdkError<DescribeVpnGatewaysError>>;
        async fn detach_classic_link_vpc(&self, builder: DetachClassicLinkVpcInputBuilder) -> Result<DetachClassicLinkVpcOutput, SdkError<DetachClassicLinkVpcError>>;
        async fn detach_internet_gateway(&self, builder: DetachInternetGatewayInputBuilder) -> Result<DetachInternetGatewayOutput, SdkError<DetachInternetGatewayError>>;
        async fn detach_network_interface(&self, builder: DetachNetworkInterfaceInputBuilder) -> Result<DetachNetworkInterfaceOutput, SdkError<DetachNetworkInterfaceError>>;
        async fn detach_verified_access_trust_provider(&self, builder: DetachVerifiedAccessTrustProviderInputBuilder) -> Result<DetachVerifiedAccessTrustProviderOutput, SdkError<DetachVerifiedAccessTrustProviderError>>;
        async fn detach_volume(&self, builder: DetachVolumeInputBuilder) -> Result<DetachVolumeOutput, SdkError<DetachVolumeError>>;
        async fn detach_vpn_gateway(&self, builder: DetachVpnGatewayInputBuilder) -> Result<DetachVpnGatewayOutput, SdkError<DetachVpnGatewayError>>;
        async fn disable_address_transfer(&self, builder: DisableAddressTransferInputBuilder) -> Result<DisableAddressTransferOutput, SdkError<DisableAddressTransferError>>;
        async fn disable_aws_network_performance_metric_subscription(&self, builder: DisableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> Result<DisableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<DisableAwsNetworkPerformanceMetricSubscriptionError>>;
        async fn disable_ebs_encryption_by_default(&self, builder: DisableEbsEncryptionByDefaultInputBuilder) -> Result<DisableEbsEncryptionByDefaultOutput, SdkError<DisableEbsEncryptionByDefaultError>>;
        async fn disable_fast_launch(&self, builder: DisableFastLaunchInputBuilder) -> Result<DisableFastLaunchOutput, SdkError<DisableFastLaunchError>>;
        async fn disable_fast_snapshot_restores(&self, builder: DisableFastSnapshotRestoresInputBuilder) -> Result<DisableFastSnapshotRestoresOutput, SdkError<DisableFastSnapshotRestoresError>>;
        async fn disable_image(&self, builder: DisableImageInputBuilder) -> Result<DisableImageOutput, SdkError<DisableImageError>>;
        async fn disable_image_block_public_access(&self, builder: DisableImageBlockPublicAccessInputBuilder) -> Result<DisableImageBlockPublicAccessOutput, SdkError<DisableImageBlockPublicAccessError>>;
        async fn disable_image_deprecation(&self, builder: DisableImageDeprecationInputBuilder) -> Result<DisableImageDeprecationOutput, SdkError<DisableImageDeprecationError>>;
        async fn disable_image_deregistration_protection(&self, builder: DisableImageDeregistrationProtectionInputBuilder) -> Result<DisableImageDeregistrationProtectionOutput, SdkError<DisableImageDeregistrationProtectionError>>;
        async fn disable_ipam_organization_admin_account(&self, builder: DisableIpamOrganizationAdminAccountInputBuilder) -> Result<DisableIpamOrganizationAdminAccountOutput, SdkError<DisableIpamOrganizationAdminAccountError>>;
        async fn disable_serial_console_access(&self, builder: DisableSerialConsoleAccessInputBuilder) -> Result<DisableSerialConsoleAccessOutput, SdkError<DisableSerialConsoleAccessError>>;
        async fn disable_snapshot_block_public_access(&self, builder: DisableSnapshotBlockPublicAccessInputBuilder) -> Result<DisableSnapshotBlockPublicAccessOutput, SdkError<DisableSnapshotBlockPublicAccessError>>;
        async fn disable_transit_gateway_route_table_propagation(&self, builder: DisableTransitGatewayRouteTablePropagationInputBuilder) -> Result<DisableTransitGatewayRouteTablePropagationOutput, SdkError<DisableTransitGatewayRouteTablePropagationError>>;
        async fn disable_vgw_route_propagation(&self, builder: DisableVgwRoutePropagationInputBuilder) -> Result<DisableVgwRoutePropagationOutput, SdkError<DisableVgwRoutePropagationError>>;
        async fn disable_vpc_classic_link(&self, builder: DisableVpcClassicLinkInputBuilder) -> Result<DisableVpcClassicLinkOutput, SdkError<DisableVpcClassicLinkError>>;
        async fn disable_vpc_classic_link_dns_support(&self, builder: DisableVpcClassicLinkDnsSupportInputBuilder) -> Result<DisableVpcClassicLinkDnsSupportOutput, SdkError<DisableVpcClassicLinkDnsSupportError>>;
        async fn disassociate_address(&self, builder: DisassociateAddressInputBuilder) -> Result<DisassociateAddressOutput, SdkError<DisassociateAddressError>>;
        async fn disassociate_client_vpn_target_network(&self, builder: DisassociateClientVpnTargetNetworkInputBuilder) -> Result<DisassociateClientVpnTargetNetworkOutput, SdkError<DisassociateClientVpnTargetNetworkError>>;
        async fn disassociate_enclave_certificate_iam_role(&self, builder: DisassociateEnclaveCertificateIamRoleInputBuilder) -> Result<DisassociateEnclaveCertificateIamRoleOutput, SdkError<DisassociateEnclaveCertificateIamRoleError>>;
        async fn disassociate_iam_instance_profile(&self, builder: DisassociateIamInstanceProfileInputBuilder) -> Result<DisassociateIamInstanceProfileOutput, SdkError<DisassociateIamInstanceProfileError>>;
        async fn disassociate_instance_event_window(&self, builder: DisassociateInstanceEventWindowInputBuilder) -> Result<DisassociateInstanceEventWindowOutput, SdkError<DisassociateInstanceEventWindowError>>;
        async fn disassociate_ipam_byoasn(&self, builder: DisassociateIpamByoasnInputBuilder) -> Result<DisassociateIpamByoasnOutput, SdkError<DisassociateIpamByoasnError>>;
        async fn disassociate_ipam_resource_discovery(&self, builder: DisassociateIpamResourceDiscoveryInputBuilder) -> Result<DisassociateIpamResourceDiscoveryOutput, SdkError<DisassociateIpamResourceDiscoveryError>>;
        async fn disassociate_nat_gateway_address(&self, builder: DisassociateNatGatewayAddressInputBuilder) -> Result<DisassociateNatGatewayAddressOutput, SdkError<DisassociateNatGatewayAddressError>>;
        async fn disassociate_route_table(&self, builder: DisassociateRouteTableInputBuilder) -> Result<DisassociateRouteTableOutput, SdkError<DisassociateRouteTableError>>;
        async fn disassociate_subnet_cidr_block(&self, builder: DisassociateSubnetCidrBlockInputBuilder) -> Result<DisassociateSubnetCidrBlockOutput, SdkError<DisassociateSubnetCidrBlockError>>;
        async fn disassociate_transit_gateway_multicast_domain(&self, builder: DisassociateTransitGatewayMulticastDomainInputBuilder) -> Result<DisassociateTransitGatewayMulticastDomainOutput, SdkError<DisassociateTransitGatewayMulticastDomainError>>;
        async fn disassociate_transit_gateway_policy_table(&self, builder: DisassociateTransitGatewayPolicyTableInputBuilder) -> Result<DisassociateTransitGatewayPolicyTableOutput, SdkError<DisassociateTransitGatewayPolicyTableError>>;
        async fn disassociate_transit_gateway_route_table(&self, builder: DisassociateTransitGatewayRouteTableInputBuilder) -> Result<DisassociateTransitGatewayRouteTableOutput, SdkError<DisassociateTransitGatewayRouteTableError>>;
        async fn disassociate_trunk_interface(&self, builder: DisassociateTrunkInterfaceInputBuilder) -> Result<DisassociateTrunkInterfaceOutput, SdkError<DisassociateTrunkInterfaceError>>;
        async fn disassociate_vpc_cidr_block(&self, builder: DisassociateVpcCidrBlockInputBuilder) -> Result<DisassociateVpcCidrBlockOutput, SdkError<DisassociateVpcCidrBlockError>>;
        async fn enable_address_transfer(&self, builder: EnableAddressTransferInputBuilder) -> Result<EnableAddressTransferOutput, SdkError<EnableAddressTransferError>>;
        async fn enable_aws_network_performance_metric_subscription(&self, builder: EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder) -> Result<EnableAwsNetworkPerformanceMetricSubscriptionOutput, SdkError<EnableAwsNetworkPerformanceMetricSubscriptionError>>;
        async fn enable_ebs_encryption_by_default(&self, builder: EnableEbsEncryptionByDefaultInputBuilder) -> Result<EnableEbsEncryptionByDefaultOutput, SdkError<EnableEbsEncryptionByDefaultError>>;
        async fn enable_fast_launch(&self, builder: EnableFastLaunchInputBuilder) -> Result<EnableFastLaunchOutput, SdkError<EnableFastLaunchError>>;
        async fn enable_fast_snapshot_restores(&self, builder: EnableFastSnapshotRestoresInputBuilder) -> Result<EnableFastSnapshotRestoresOutput, SdkError<EnableFastSnapshotRestoresError>>;
        async fn enable_image(&self, builder: EnableImageInputBuilder) -> Result<EnableImageOutput, SdkError<EnableImageError>>;
        async fn enable_image_block_public_access(&self, builder: EnableImageBlockPublicAccessInputBuilder) -> Result<EnableImageBlockPublicAccessOutput, SdkError<EnableImageBlockPublicAccessError>>;
        async fn enable_image_deprecation(&self, builder: EnableImageDeprecationInputBuilder) -> Result<EnableImageDeprecationOutput, SdkError<EnableImageDeprecationError>>;
        async fn enable_image_deregistration_protection(&self, builder: EnableImageDeregistrationProtectionInputBuilder) -> Result<EnableImageDeregistrationProtectionOutput, SdkError<EnableImageDeregistrationProtectionError>>;
        async fn enable_ipam_organization_admin_account(&self, builder: EnableIpamOrganizationAdminAccountInputBuilder) -> Result<EnableIpamOrganizationAdminAccountOutput, SdkError<EnableIpamOrganizationAdminAccountError>>;
        async fn enable_reachability_analyzer_organization_sharing(&self, builder: EnableReachabilityAnalyzerOrganizationSharingInputBuilder) -> Result<EnableReachabilityAnalyzerOrganizationSharingOutput, SdkError<EnableReachabilityAnalyzerOrganizationSharingError>>;
        async fn enable_serial_console_access(&self, builder: EnableSerialConsoleAccessInputBuilder) -> Result<EnableSerialConsoleAccessOutput, SdkError<EnableSerialConsoleAccessError>>;
        async fn enable_snapshot_block_public_access(&self, builder: EnableSnapshotBlockPublicAccessInputBuilder) -> Result<EnableSnapshotBlockPublicAccessOutput, SdkError<EnableSnapshotBlockPublicAccessError>>;
        async fn enable_transit_gateway_route_table_propagation(&self, builder: EnableTransitGatewayRouteTablePropagationInputBuilder) -> Result<EnableTransitGatewayRouteTablePropagationOutput, SdkError<EnableTransitGatewayRouteTablePropagationError>>;
        async fn enable_vgw_route_propagation(&self, builder: EnableVgwRoutePropagationInputBuilder) -> Result<EnableVgwRoutePropagationOutput, SdkError<EnableVgwRoutePropagationError>>;
        async fn enable_volume_io(&self, builder: EnableVolumeIoInputBuilder) -> Result<EnableVolumeIoOutput, SdkError<EnableVolumeIOError>>;
        async fn enable_vpc_classic_link(&self, builder: EnableVpcClassicLinkInputBuilder) -> Result<EnableVpcClassicLinkOutput, SdkError<EnableVpcClassicLinkError>>;
        async fn enable_vpc_classic_link_dns_support(&self, builder: EnableVpcClassicLinkDnsSupportInputBuilder) -> Result<EnableVpcClassicLinkDnsSupportOutput, SdkError<EnableVpcClassicLinkDnsSupportError>>;
        async fn export_client_vpn_client_certificate_revocation_list(&self, builder: ExportClientVpnClientCertificateRevocationListInputBuilder) -> Result<ExportClientVpnClientCertificateRevocationListOutput, SdkError<ExportClientVpnClientCertificateRevocationListError>>;
        async fn export_client_vpn_client_configuration(&self, builder: ExportClientVpnClientConfigurationInputBuilder) -> Result<ExportClientVpnClientConfigurationOutput, SdkError<ExportClientVpnClientConfigurationError>>;
        async fn export_image(&self, builder: ExportImageInputBuilder) -> Result<ExportImageOutput, SdkError<ExportImageError>>;
        async fn export_transit_gateway_routes(&self, builder: ExportTransitGatewayRoutesInputBuilder) -> Result<ExportTransitGatewayRoutesOutput, SdkError<ExportTransitGatewayRoutesError>>;
        async fn get_associated_enclave_certificate_iam_roles(&self, builder: GetAssociatedEnclaveCertificateIamRolesInputBuilder) -> Result<GetAssociatedEnclaveCertificateIamRolesOutput, SdkError<GetAssociatedEnclaveCertificateIamRolesError>>;
        async fn get_associated_ipv6_pool_cidrs(&self, builder: GetAssociatedIpv6PoolCidrsInputBuilder) -> Result<GetAssociatedIpv6PoolCidrsOutput, SdkError<GetAssociatedIpv6PoolCidrsError>>;
        async fn get_aws_network_performance_data(&self, builder: GetAwsNetworkPerformanceDataInputBuilder) -> Result<GetAwsNetworkPerformanceDataOutput, SdkError<GetAwsNetworkPerformanceDataError>>;
        async fn get_capacity_reservation_usage(&self, builder: GetCapacityReservationUsageInputBuilder) -> Result<GetCapacityReservationUsageOutput, SdkError<GetCapacityReservationUsageError>>;
        async fn get_coip_pool_usage(&self, builder: GetCoipPoolUsageInputBuilder) -> Result<GetCoipPoolUsageOutput, SdkError<GetCoipPoolUsageError>>;
        async fn get_console_output(&self, builder: GetConsoleOutputInputBuilder) -> Result<GetConsoleOutputOutput, SdkError<GetConsoleOutputError>>;
        async fn get_console_screenshot(&self, builder: GetConsoleScreenshotInputBuilder) -> Result<GetConsoleScreenshotOutput, SdkError<GetConsoleScreenshotError>>;
        async fn get_default_credit_specification(&self, builder: GetDefaultCreditSpecificationInputBuilder) -> Result<GetDefaultCreditSpecificationOutput, SdkError<GetDefaultCreditSpecificationError>>;
        async fn get_ebs_default_kms_key_id(&self, builder: GetEbsDefaultKmsKeyIdInputBuilder) -> Result<GetEbsDefaultKmsKeyIdOutput, SdkError<GetEbsDefaultKmsKeyIdError>>;
        async fn get_ebs_encryption_by_default(&self, builder: GetEbsEncryptionByDefaultInputBuilder) -> Result<GetEbsEncryptionByDefaultOutput, SdkError<GetEbsEncryptionByDefaultError>>;
        async fn get_flow_logs_integration_template(&self, builder: GetFlowLogsIntegrationTemplateInputBuilder) -> Result<GetFlowLogsIntegrationTemplateOutput, SdkError<GetFlowLogsIntegrationTemplateError>>;
        async fn get_groups_for_capacity_reservation(&self, builder: GetGroupsForCapacityReservationInputBuilder) -> Result<GetGroupsForCapacityReservationOutput, SdkError<GetGroupsForCapacityReservationError>>;
        async fn get_host_reservation_purchase_preview(&self, builder: GetHostReservationPurchasePreviewInputBuilder) -> Result<GetHostReservationPurchasePreviewOutput, SdkError<GetHostReservationPurchasePreviewError>>;
        async fn get_image_block_public_access_state(&self, builder: GetImageBlockPublicAccessStateInputBuilder) -> Result<GetImageBlockPublicAccessStateOutput, SdkError<GetImageBlockPublicAccessStateError>>;
        async fn get_instance_metadata_defaults(&self, builder: GetInstanceMetadataDefaultsInputBuilder) -> Result<GetInstanceMetadataDefaultsOutput, SdkError<GetInstanceMetadataDefaultsError>>;
        async fn get_instance_tpm_ek_pub(&self, builder: GetInstanceTpmEkPubInputBuilder) -> Result<GetInstanceTpmEkPubOutput, SdkError<GetInstanceTpmEkPubError>>;
        async fn get_instance_types_from_instance_requirements(&self, builder: GetInstanceTypesFromInstanceRequirementsInputBuilder) -> Result<GetInstanceTypesFromInstanceRequirementsOutput, SdkError<GetInstanceTypesFromInstanceRequirementsError>>;
        async fn get_instance_uefi_data(&self, builder: GetInstanceUefiDataInputBuilder) -> Result<GetInstanceUefiDataOutput, SdkError<GetInstanceUefiDataError>>;
        async fn get_ipam_address_history(&self, builder: GetIpamAddressHistoryInputBuilder) -> Result<GetIpamAddressHistoryOutput, SdkError<GetIpamAddressHistoryError>>;
        async fn get_ipam_discovered_accounts(&self, builder: GetIpamDiscoveredAccountsInputBuilder) -> Result<GetIpamDiscoveredAccountsOutput, SdkError<GetIpamDiscoveredAccountsError>>;
        async fn get_ipam_discovered_public_addresses(&self, builder: GetIpamDiscoveredPublicAddressesInputBuilder) -> Result<GetIpamDiscoveredPublicAddressesOutput, SdkError<GetIpamDiscoveredPublicAddressesError>>;
        async fn get_ipam_discovered_resource_cidrs(&self, builder: GetIpamDiscoveredResourceCidrsInputBuilder) -> Result<GetIpamDiscoveredResourceCidrsOutput, SdkError<GetIpamDiscoveredResourceCidrsError>>;
        async fn get_ipam_pool_allocations(&self, builder: GetIpamPoolAllocationsInputBuilder) -> Result<GetIpamPoolAllocationsOutput, SdkError<GetIpamPoolAllocationsError>>;
        async fn get_ipam_pool_cidrs(&self, builder: GetIpamPoolCidrsInputBuilder) -> Result<GetIpamPoolCidrsOutput, SdkError<GetIpamPoolCidrsError>>;
        async fn get_ipam_resource_cidrs(&self, builder: GetIpamResourceCidrsInputBuilder) -> Result<GetIpamResourceCidrsOutput, SdkError<GetIpamResourceCidrsError>>;
        async fn get_launch_template_data(&self, builder: GetLaunchTemplateDataInputBuilder) -> Result<GetLaunchTemplateDataOutput, SdkError<GetLaunchTemplateDataError>>;
        async fn get_managed_prefix_list_associations(&self, builder: GetManagedPrefixListAssociationsInputBuilder) -> Result<GetManagedPrefixListAssociationsOutput, SdkError<GetManagedPrefixListAssociationsError>>;
        async fn get_managed_prefix_list_entries(&self, builder: GetManagedPrefixListEntriesInputBuilder) -> Result<GetManagedPrefixListEntriesOutput, SdkError<GetManagedPrefixListEntriesError>>;
        async fn get_network_insights_access_scope_analysis_findings(&self, builder: GetNetworkInsightsAccessScopeAnalysisFindingsInputBuilder) -> Result<GetNetworkInsightsAccessScopeAnalysisFindingsOutput, SdkError<GetNetworkInsightsAccessScopeAnalysisFindingsError>>;
        async fn get_network_insights_access_scope_content(&self, builder: GetNetworkInsightsAccessScopeContentInputBuilder) -> Result<GetNetworkInsightsAccessScopeContentOutput, SdkError<GetNetworkInsightsAccessScopeContentError>>;
        async fn get_password_data(&self, builder: GetPasswordDataInputBuilder) -> Result<GetPasswordDataOutput, SdkError<GetPasswordDataError>>;
        async fn get_reserved_instances_exchange_quote(&self, builder: GetReservedInstancesExchangeQuoteInputBuilder) -> Result<GetReservedInstancesExchangeQuoteOutput, SdkError<GetReservedInstancesExchangeQuoteError>>;
        async fn get_security_groups_for_vpc(&self, builder: GetSecurityGroupsForVpcInputBuilder) -> Result<GetSecurityGroupsForVpcOutput, SdkError<GetSecurityGroupsForVpcError>>;
        async fn get_serial_console_access_status(&self, builder: GetSerialConsoleAccessStatusInputBuilder) -> Result<GetSerialConsoleAccessStatusOutput, SdkError<GetSerialConsoleAccessStatusError>>;
        async fn get_snapshot_block_public_access_state(&self, builder: GetSnapshotBlockPublicAccessStateInputBuilder) -> Result<GetSnapshotBlockPublicAccessStateOutput, SdkError<GetSnapshotBlockPublicAccessStateError>>;
        async fn get_spot_placement_scores(&self, builder: GetSpotPlacementScoresInputBuilder) -> Result<GetSpotPlacementScoresOutput, SdkError<GetSpotPlacementScoresError>>;
        async fn get_subnet_cidr_reservations(&self, builder: GetSubnetCidrReservationsInputBuilder) -> Result<GetSubnetCidrReservationsOutput, SdkError<GetSubnetCidrReservationsError>>;
        async fn get_transit_gateway_attachment_propagations(&self, builder: GetTransitGatewayAttachmentPropagationsInputBuilder) -> Result<GetTransitGatewayAttachmentPropagationsOutput, SdkError<GetTransitGatewayAttachmentPropagationsError>>;
        async fn get_transit_gateway_multicast_domain_associations(&self, builder: GetTransitGatewayMulticastDomainAssociationsInputBuilder) -> Result<GetTransitGatewayMulticastDomainAssociationsOutput, SdkError<GetTransitGatewayMulticastDomainAssociationsError>>;
        async fn get_transit_gateway_policy_table_associations(&self, builder: GetTransitGatewayPolicyTableAssociationsInputBuilder) -> Result<GetTransitGatewayPolicyTableAssociationsOutput, SdkError<GetTransitGatewayPolicyTableAssociationsError>>;
        async fn get_transit_gateway_policy_table_entries(&self, builder: GetTransitGatewayPolicyTableEntriesInputBuilder) -> Result<GetTransitGatewayPolicyTableEntriesOutput, SdkError<GetTransitGatewayPolicyTableEntriesError>>;
        async fn get_transit_gateway_prefix_list_references(&self, builder: GetTransitGatewayPrefixListReferencesInputBuilder) -> Result<GetTransitGatewayPrefixListReferencesOutput, SdkError<GetTransitGatewayPrefixListReferencesError>>;
        async fn get_transit_gateway_route_table_associations(&self, builder: GetTransitGatewayRouteTableAssociationsInputBuilder) -> Result<GetTransitGatewayRouteTableAssociationsOutput, SdkError<GetTransitGatewayRouteTableAssociationsError>>;
        async fn get_transit_gateway_route_table_propagations(&self, builder: GetTransitGatewayRouteTablePropagationsInputBuilder) -> Result<GetTransitGatewayRouteTablePropagationsOutput, SdkError<GetTransitGatewayRouteTablePropagationsError>>;
        async fn get_verified_access_endpoint_policy(&self, builder: GetVerifiedAccessEndpointPolicyInputBuilder) -> Result<GetVerifiedAccessEndpointPolicyOutput, SdkError<GetVerifiedAccessEndpointPolicyError>>;
        async fn get_verified_access_group_policy(&self, builder: GetVerifiedAccessGroupPolicyInputBuilder) -> Result<GetVerifiedAccessGroupPolicyOutput, SdkError<GetVerifiedAccessGroupPolicyError>>;
        async fn get_vpn_connection_device_sample_configuration(&self, builder: GetVpnConnectionDeviceSampleConfigurationInputBuilder) -> Result<GetVpnConnectionDeviceSampleConfigurationOutput, SdkError<GetVpnConnectionDeviceSampleConfigurationError>>;
        async fn get_vpn_connection_device_types(&self, builder: GetVpnConnectionDeviceTypesInputBuilder) -> Result<GetVpnConnectionDeviceTypesOutput, SdkError<GetVpnConnectionDeviceTypesError>>;
        async fn get_vpn_tunnel_replacement_status(&self, builder: GetVpnTunnelReplacementStatusInputBuilder) -> Result<GetVpnTunnelReplacementStatusOutput, SdkError<GetVpnTunnelReplacementStatusError>>;
        async fn import_client_vpn_client_certificate_revocation_list(&self, builder: ImportClientVpnClientCertificateRevocationListInputBuilder) -> Result<ImportClientVpnClientCertificateRevocationListOutput, SdkError<ImportClientVpnClientCertificateRevocationListError>>;
        async fn import_image(&self, builder: ImportImageInputBuilder) -> Result<ImportImageOutput, SdkError<ImportImageError>>;
        async fn import_key_pair(&self, builder: ImportKeyPairInputBuilder) -> Result<ImportKeyPairOutput, SdkError<ImportKeyPairError>>;
        async fn import_snapshot(&self, builder: ImportSnapshotInputBuilder) -> Result<ImportSnapshotOutput, SdkError<ImportSnapshotError>>;
        async fn list_images_in_recycle_bin(&self, builder: ListImagesInRecycleBinInputBuilder) -> Result<ListImagesInRecycleBinOutput, SdkError<ListImagesInRecycleBinError>>;
        async fn list_snapshots_in_recycle_bin(&self, builder: ListSnapshotsInRecycleBinInputBuilder) -> Result<ListSnapshotsInRecycleBinOutput, SdkError<ListSnapshotsInRecycleBinError>>;
        async fn lock_snapshot(&self, builder: LockSnapshotInputBuilder) -> Result<LockSnapshotOutput, SdkError<LockSnapshotError>>;
        async fn modify_address_attribute(&self, builder: ModifyAddressAttributeInputBuilder) -> Result<ModifyAddressAttributeOutput, SdkError<ModifyAddressAttributeError>>;
        async fn modify_availability_zone_group(&self, builder: ModifyAvailabilityZoneGroupInputBuilder) -> Result<ModifyAvailabilityZoneGroupOutput, SdkError<ModifyAvailabilityZoneGroupError>>;
        async fn modify_capacity_reservation(&self, builder: ModifyCapacityReservationInputBuilder) -> Result<ModifyCapacityReservationOutput, SdkError<ModifyCapacityReservationError>>;
        async fn modify_capacity_reservation_fleet(&self, builder: ModifyCapacityReservationFleetInputBuilder) -> Result<ModifyCapacityReservationFleetOutput, SdkError<ModifyCapacityReservationFleetError>>;
        async fn modify_client_vpn_endpoint(&self, builder: ModifyClientVpnEndpointInputBuilder) -> Result<ModifyClientVpnEndpointOutput, SdkError<ModifyClientVpnEndpointError>>;
        async fn modify_default_credit_specification(&self, builder: ModifyDefaultCreditSpecificationInputBuilder) -> Result<ModifyDefaultCreditSpecificationOutput, SdkError<ModifyDefaultCreditSpecificationError>>;
        async fn modify_ebs_default_kms_key_id(&self, builder: ModifyEbsDefaultKmsKeyIdInputBuilder) -> Result<ModifyEbsDefaultKmsKeyIdOutput, SdkError<ModifyEbsDefaultKmsKeyIdError>>;
        async fn modify_fleet(&self, builder: ModifyFleetInputBuilder) -> Result<ModifyFleetOutput, SdkError<ModifyFleetError>>;
        async fn modify_fpga_image_attribute(&self, builder: ModifyFpgaImageAttributeInputBuilder) -> Result<ModifyFpgaImageAttributeOutput, SdkError<ModifyFpgaImageAttributeError>>;
        async fn modify_hosts(&self, builder: ModifyHostsInputBuilder) -> Result<ModifyHostsOutput, SdkError<ModifyHostsError>>;
        async fn modify_id_format(&self, builder: ModifyIdFormatInputBuilder) -> Result<ModifyIdFormatOutput, SdkError<ModifyIdFormatError>>;
        async fn modify_identity_id_format(&self, builder: ModifyIdentityIdFormatInputBuilder) -> Result<ModifyIdentityIdFormatOutput, SdkError<ModifyIdentityIdFormatError>>;
        async fn modify_image_attribute(&self, builder: ModifyImageAttributeInputBuilder) -> Result<ModifyImageAttributeOutput, SdkError<ModifyImageAttributeError>>;
        async fn modify_instance_attribute(&self, builder: ModifyInstanceAttributeInputBuilder) -> Result<ModifyInstanceAttributeOutput, SdkError<ModifyInstanceAttributeError>>;
        async fn modify_instance_capacity_reservation_attributes(&self, builder: ModifyInstanceCapacityReservationAttributesInputBuilder) -> Result<ModifyInstanceCapacityReservationAttributesOutput, SdkError<ModifyInstanceCapacityReservationAttributesError>>;
        async fn modify_instance_credit_specification(&self, builder: ModifyInstanceCreditSpecificationInputBuilder) -> Result<ModifyInstanceCreditSpecificationOutput, SdkError<ModifyInstanceCreditSpecificationError>>;
        async fn modify_instance_event_start_time(&self, builder: ModifyInstanceEventStartTimeInputBuilder) -> Result<ModifyInstanceEventStartTimeOutput, SdkError<ModifyInstanceEventStartTimeError>>;
        async fn modify_instance_event_window(&self, builder: ModifyInstanceEventWindowInputBuilder) -> Result<ModifyInstanceEventWindowOutput, SdkError<ModifyInstanceEventWindowError>>;
        async fn modify_instance_maintenance_options(&self, builder: ModifyInstanceMaintenanceOptionsInputBuilder) -> Result<ModifyInstanceMaintenanceOptionsOutput, SdkError<ModifyInstanceMaintenanceOptionsError>>;
        async fn modify_instance_metadata_defaults(&self, builder: ModifyInstanceMetadataDefaultsInputBuilder) -> Result<ModifyInstanceMetadataDefaultsOutput, SdkError<ModifyInstanceMetadataDefaultsError>>;
        async fn modify_instance_metadata_options(&self, builder: ModifyInstanceMetadataOptionsInputBuilder) -> Result<ModifyInstanceMetadataOptionsOutput, SdkError<ModifyInstanceMetadataOptionsError>>;
        async fn modify_instance_placement(&self, builder: ModifyInstancePlacementInputBuilder) -> Result<ModifyInstancePlacementOutput, SdkError<ModifyInstancePlacementError>>;
        async fn modify_ipam(&self, builder: ModifyIpamInputBuilder) -> Result<ModifyIpamOutput, SdkError<ModifyIpamError>>;
        async fn modify_ipam_pool(&self, builder: ModifyIpamPoolInputBuilder) -> Result<ModifyIpamPoolOutput, SdkError<ModifyIpamPoolError>>;
        async fn modify_ipam_resource_cidr(&self, builder: ModifyIpamResourceCidrInputBuilder) -> Result<ModifyIpamResourceCidrOutput, SdkError<ModifyIpamResourceCidrError>>;
        async fn modify_ipam_resource_discovery(&self, builder: ModifyIpamResourceDiscoveryInputBuilder) -> Result<ModifyIpamResourceDiscoveryOutput, SdkError<ModifyIpamResourceDiscoveryError>>;
        async fn modify_ipam_scope(&self, builder: ModifyIpamScopeInputBuilder) -> Result<ModifyIpamScopeOutput, SdkError<ModifyIpamScopeError>>;
        async fn modify_launch_template(&self, builder: ModifyLaunchTemplateInputBuilder) -> Result<ModifyLaunchTemplateOutput, SdkError<ModifyLaunchTemplateError>>;
        async fn modify_local_gateway_route(&self, builder: ModifyLocalGatewayRouteInputBuilder) -> Result<ModifyLocalGatewayRouteOutput, SdkError<ModifyLocalGatewayRouteError>>;
        async fn modify_managed_prefix_list(&self, builder: ModifyManagedPrefixListInputBuilder) -> Result<ModifyManagedPrefixListOutput, SdkError<ModifyManagedPrefixListError>>;
        async fn modify_network_interface_attribute(&self, builder: ModifyNetworkInterfaceAttributeInputBuilder) -> Result<ModifyNetworkInterfaceAttributeOutput, SdkError<ModifyNetworkInterfaceAttributeError>>;
        async fn modify_private_dns_name_options(&self, builder: ModifyPrivateDnsNameOptionsInputBuilder) -> Result<ModifyPrivateDnsNameOptionsOutput, SdkError<ModifyPrivateDnsNameOptionsError>>;
        async fn modify_reserved_instances(&self, builder: ModifyReservedInstancesInputBuilder) -> Result<ModifyReservedInstancesOutput, SdkError<ModifyReservedInstancesError>>;
        async fn modify_security_group_rules(&self, builder: ModifySecurityGroupRulesInputBuilder) -> Result<ModifySecurityGroupRulesOutput, SdkError<ModifySecurityGroupRulesError>>;
        async fn modify_snapshot_attribute(&self, builder: ModifySnapshotAttributeInputBuilder) -> Result<ModifySnapshotAttributeOutput, SdkError<ModifySnapshotAttributeError>>;
        async fn modify_snapshot_tier(&self, builder: ModifySnapshotTierInputBuilder) -> Result<ModifySnapshotTierOutput, SdkError<ModifySnapshotTierError>>;
        async fn modify_spot_fleet_request(&self, builder: ModifySpotFleetRequestInputBuilder) -> Result<ModifySpotFleetRequestOutput, SdkError<ModifySpotFleetRequestError>>;
        async fn modify_subnet_attribute(&self, builder: ModifySubnetAttributeInputBuilder) -> Result<ModifySubnetAttributeOutput, SdkError<ModifySubnetAttributeError>>;
        async fn modify_traffic_mirror_filter_network_services(&self, builder: ModifyTrafficMirrorFilterNetworkServicesInputBuilder) -> Result<ModifyTrafficMirrorFilterNetworkServicesOutput, SdkError<ModifyTrafficMirrorFilterNetworkServicesError>>;
        async fn modify_traffic_mirror_filter_rule(&self, builder: ModifyTrafficMirrorFilterRuleInputBuilder) -> Result<ModifyTrafficMirrorFilterRuleOutput, SdkError<ModifyTrafficMirrorFilterRuleError>>;
        async fn modify_traffic_mirror_session(&self, builder: ModifyTrafficMirrorSessionInputBuilder) -> Result<ModifyTrafficMirrorSessionOutput, SdkError<ModifyTrafficMirrorSessionError>>;
        async fn modify_transit_gateway(&self, builder: ModifyTransitGatewayInputBuilder) -> Result<ModifyTransitGatewayOutput, SdkError<ModifyTransitGatewayError>>;
        async fn modify_transit_gateway_prefix_list_reference(&self, builder: ModifyTransitGatewayPrefixListReferenceInputBuilder) -> Result<ModifyTransitGatewayPrefixListReferenceOutput, SdkError<ModifyTransitGatewayPrefixListReferenceError>>;
        async fn modify_transit_gateway_vpc_attachment(&self, builder: ModifyTransitGatewayVpcAttachmentInputBuilder) -> Result<ModifyTransitGatewayVpcAttachmentOutput, SdkError<ModifyTransitGatewayVpcAttachmentError>>;
        async fn modify_verified_access_endpoint(&self, builder: ModifyVerifiedAccessEndpointInputBuilder) -> Result<ModifyVerifiedAccessEndpointOutput, SdkError<ModifyVerifiedAccessEndpointError>>;
        async fn modify_verified_access_endpoint_policy(&self, builder: ModifyVerifiedAccessEndpointPolicyInputBuilder) -> Result<ModifyVerifiedAccessEndpointPolicyOutput, SdkError<ModifyVerifiedAccessEndpointPolicyError>>;
        async fn modify_verified_access_group(&self, builder: ModifyVerifiedAccessGroupInputBuilder) -> Result<ModifyVerifiedAccessGroupOutput, SdkError<ModifyVerifiedAccessGroupError>>;
        async fn modify_verified_access_group_policy(&self, builder: ModifyVerifiedAccessGroupPolicyInputBuilder) -> Result<ModifyVerifiedAccessGroupPolicyOutput, SdkError<ModifyVerifiedAccessGroupPolicyError>>;
        async fn modify_verified_access_instance(&self, builder: ModifyVerifiedAccessInstanceInputBuilder) -> Result<ModifyVerifiedAccessInstanceOutput, SdkError<ModifyVerifiedAccessInstanceError>>;
        async fn modify_verified_access_instance_logging_configuration(&self, builder: ModifyVerifiedAccessInstanceLoggingConfigurationInputBuilder) -> Result<ModifyVerifiedAccessInstanceLoggingConfigurationOutput, SdkError<ModifyVerifiedAccessInstanceLoggingConfigurationError>>;
        async fn modify_verified_access_trust_provider(&self, builder: ModifyVerifiedAccessTrustProviderInputBuilder) -> Result<ModifyVerifiedAccessTrustProviderOutput, SdkError<ModifyVerifiedAccessTrustProviderError>>;
        async fn modify_volume(&self, builder: ModifyVolumeInputBuilder) -> Result<ModifyVolumeOutput, SdkError<ModifyVolumeError>>;
        async fn modify_volume_attribute(&self, builder: ModifyVolumeAttributeInputBuilder) -> Result<ModifyVolumeAttributeOutput, SdkError<ModifyVolumeAttributeError>>;
        async fn modify_vpc_attribute(&self, builder: ModifyVpcAttributeInputBuilder) -> Result<ModifyVpcAttributeOutput, SdkError<ModifyVpcAttributeError>>;
        async fn modify_vpc_endpoint(&self, builder: ModifyVpcEndpointInputBuilder) -> Result<ModifyVpcEndpointOutput, SdkError<ModifyVpcEndpointError>>;
        async fn modify_vpc_endpoint_connection_notification(&self, builder: ModifyVpcEndpointConnectionNotificationInputBuilder) -> Result<ModifyVpcEndpointConnectionNotificationOutput, SdkError<ModifyVpcEndpointConnectionNotificationError>>;
        async fn modify_vpc_endpoint_service_configuration(&self, builder: ModifyVpcEndpointServiceConfigurationInputBuilder) -> Result<ModifyVpcEndpointServiceConfigurationOutput, SdkError<ModifyVpcEndpointServiceConfigurationError>>;
        async fn modify_vpc_endpoint_service_payer_responsibility(&self, builder: ModifyVpcEndpointServicePayerResponsibilityInputBuilder) -> Result<ModifyVpcEndpointServicePayerResponsibilityOutput, SdkError<ModifyVpcEndpointServicePayerResponsibilityError>>;
        async fn modify_vpc_endpoint_service_permissions(&self, builder: ModifyVpcEndpointServicePermissionsInputBuilder) -> Result<ModifyVpcEndpointServicePermissionsOutput, SdkError<ModifyVpcEndpointServicePermissionsError>>;
        async fn modify_vpc_peering_connection_options(&self, builder: ModifyVpcPeeringConnectionOptionsInputBuilder) -> Result<ModifyVpcPeeringConnectionOptionsOutput, SdkError<ModifyVpcPeeringConnectionOptionsError>>;
        async fn modify_vpc_tenancy(&self, builder: ModifyVpcTenancyInputBuilder) -> Result<ModifyVpcTenancyOutput, SdkError<ModifyVpcTenancyError>>;
        async fn modify_vpn_connection(&self, builder: ModifyVpnConnectionInputBuilder) -> Result<ModifyVpnConnectionOutput, SdkError<ModifyVpnConnectionError>>;
        async fn modify_vpn_connection_options(&self, builder: ModifyVpnConnectionOptionsInputBuilder) -> Result<ModifyVpnConnectionOptionsOutput, SdkError<ModifyVpnConnectionOptionsError>>;
        async fn modify_vpn_tunnel_certificate(&self, builder: ModifyVpnTunnelCertificateInputBuilder) -> Result<ModifyVpnTunnelCertificateOutput, SdkError<ModifyVpnTunnelCertificateError>>;
        async fn modify_vpn_tunnel_options(&self, builder: ModifyVpnTunnelOptionsInputBuilder) -> Result<ModifyVpnTunnelOptionsOutput, SdkError<ModifyVpnTunnelOptionsError>>;
        async fn monitor_instances(&self, builder: MonitorInstancesInputBuilder) -> Result<MonitorInstancesOutput, SdkError<MonitorInstancesError>>;
        async fn move_address_to_vpc(&self, builder: MoveAddressToVpcInputBuilder) -> Result<MoveAddressToVpcOutput, SdkError<MoveAddressToVpcError>>;
        async fn move_byoip_cidr_to_ipam(&self, builder: MoveByoipCidrToIpamInputBuilder) -> Result<MoveByoipCidrToIpamOutput, SdkError<MoveByoipCidrToIpamError>>;
        async fn provision_byoip_cidr(&self, builder: ProvisionByoipCidrInputBuilder) -> Result<ProvisionByoipCidrOutput, SdkError<ProvisionByoipCidrError>>;
        async fn provision_ipam_byoasn(&self, builder: ProvisionIpamByoasnInputBuilder) -> Result<ProvisionIpamByoasnOutput, SdkError<ProvisionIpamByoasnError>>;
        async fn provision_ipam_pool_cidr(&self, builder: ProvisionIpamPoolCidrInputBuilder) -> Result<ProvisionIpamPoolCidrOutput, SdkError<ProvisionIpamPoolCidrError>>;
        async fn provision_public_ipv4_pool_cidr(&self, builder: ProvisionPublicIpv4PoolCidrInputBuilder) -> Result<ProvisionPublicIpv4PoolCidrOutput, SdkError<ProvisionPublicIpv4PoolCidrError>>;
        async fn purchase_capacity_block(&self, builder: PurchaseCapacityBlockInputBuilder) -> Result<PurchaseCapacityBlockOutput, SdkError<PurchaseCapacityBlockError>>;
        async fn purchase_host_reservation(&self, builder: PurchaseHostReservationInputBuilder) -> Result<PurchaseHostReservationOutput, SdkError<PurchaseHostReservationError>>;
        async fn purchase_reserved_instances_offering(&self, builder: PurchaseReservedInstancesOfferingInputBuilder) -> Result<PurchaseReservedInstancesOfferingOutput, SdkError<PurchaseReservedInstancesOfferingError>>;
        async fn purchase_scheduled_instances(&self, builder: PurchaseScheduledInstancesInputBuilder) -> Result<PurchaseScheduledInstancesOutput, SdkError<PurchaseScheduledInstancesError>>;
        async fn reboot_instances(&self, builder: RebootInstancesInputBuilder) -> Result<RebootInstancesOutput, SdkError<RebootInstancesError>>;
        async fn register_image(&self, builder: RegisterImageInputBuilder) -> Result<RegisterImageOutput, SdkError<RegisterImageError>>;
        async fn register_instance_event_notification_attributes(&self, builder: RegisterInstanceEventNotificationAttributesInputBuilder) -> Result<RegisterInstanceEventNotificationAttributesOutput, SdkError<RegisterInstanceEventNotificationAttributesError>>;
        async fn register_transit_gateway_multicast_group_members(&self, builder: RegisterTransitGatewayMulticastGroupMembersInputBuilder) -> Result<RegisterTransitGatewayMulticastGroupMembersOutput, SdkError<RegisterTransitGatewayMulticastGroupMembersError>>;
        async fn register_transit_gateway_multicast_group_sources(&self, builder: RegisterTransitGatewayMulticastGroupSourcesInputBuilder) -> Result<RegisterTransitGatewayMulticastGroupSourcesOutput, SdkError<RegisterTransitGatewayMulticastGroupSourcesError>>;
        async fn reject_transit_gateway_multicast_domain_associations(&self, builder: RejectTransitGatewayMulticastDomainAssociationsInputBuilder) -> Result<RejectTransitGatewayMulticastDomainAssociationsOutput, SdkError<RejectTransitGatewayMulticastDomainAssociationsError>>;
        async fn reject_transit_gateway_peering_attachment(&self, builder: RejectTransitGatewayPeeringAttachmentInputBuilder) -> Result<RejectTransitGatewayPeeringAttachmentOutput, SdkError<RejectTransitGatewayPeeringAttachmentError>>;
        async fn reject_transit_gateway_vpc_attachment(&self, builder: RejectTransitGatewayVpcAttachmentInputBuilder) -> Result<RejectTransitGatewayVpcAttachmentOutput, SdkError<RejectTransitGatewayVpcAttachmentError>>;
        async fn reject_vpc_endpoint_connections(&self, builder: RejectVpcEndpointConnectionsInputBuilder) -> Result<RejectVpcEndpointConnectionsOutput, SdkError<RejectVpcEndpointConnectionsError>>;
        async fn reject_vpc_peering_connection(&self, builder: RejectVpcPeeringConnectionInputBuilder) -> Result<RejectVpcPeeringConnectionOutput, SdkError<RejectVpcPeeringConnectionError>>;
        async fn release_address(&self, builder: ReleaseAddressInputBuilder) -> Result<ReleaseAddressOutput, SdkError<ReleaseAddressError>>;
        async fn release_hosts(&self, builder: ReleaseHostsInputBuilder) -> Result<ReleaseHostsOutput, SdkError<ReleaseHostsError>>;
        async fn release_ipam_pool_allocation(&self, builder: ReleaseIpamPoolAllocationInputBuilder) -> Result<ReleaseIpamPoolAllocationOutput, SdkError<ReleaseIpamPoolAllocationError>>;
        async fn replace_iam_instance_profile_association(&self, builder: ReplaceIamInstanceProfileAssociationInputBuilder) -> Result<ReplaceIamInstanceProfileAssociationOutput, SdkError<ReplaceIamInstanceProfileAssociationError>>;
        async fn replace_network_acl_association(&self, builder: ReplaceNetworkAclAssociationInputBuilder) -> Result<ReplaceNetworkAclAssociationOutput, SdkError<ReplaceNetworkAclAssociationError>>;
        async fn replace_network_acl_entry(&self, builder: ReplaceNetworkAclEntryInputBuilder) -> Result<ReplaceNetworkAclEntryOutput, SdkError<ReplaceNetworkAclEntryError>>;
        async fn replace_route(&self, builder: ReplaceRouteInputBuilder) -> Result<ReplaceRouteOutput, SdkError<ReplaceRouteError>>;
        async fn replace_route_table_association(&self, builder: ReplaceRouteTableAssociationInputBuilder) -> Result<ReplaceRouteTableAssociationOutput, SdkError<ReplaceRouteTableAssociationError>>;
        async fn replace_transit_gateway_route(&self, builder: ReplaceTransitGatewayRouteInputBuilder) -> Result<ReplaceTransitGatewayRouteOutput, SdkError<ReplaceTransitGatewayRouteError>>;
        async fn replace_vpn_tunnel(&self, builder: ReplaceVpnTunnelInputBuilder) -> Result<ReplaceVpnTunnelOutput, SdkError<ReplaceVpnTunnelError>>;
        async fn report_instance_status(&self, builder: ReportInstanceStatusInputBuilder) -> Result<ReportInstanceStatusOutput, SdkError<ReportInstanceStatusError>>;
        async fn request_spot_fleet(&self, builder: RequestSpotFleetInputBuilder) -> Result<RequestSpotFleetOutput, SdkError<RequestSpotFleetError>>;
        async fn request_spot_instances(&self, builder: RequestSpotInstancesInputBuilder) -> Result<RequestSpotInstancesOutput, SdkError<RequestSpotInstancesError>>;
        async fn reset_address_attribute(&self, builder: ResetAddressAttributeInputBuilder) -> Result<ResetAddressAttributeOutput, SdkError<ResetAddressAttributeError>>;
        async fn reset_ebs_default_kms_key_id(&self, builder: ResetEbsDefaultKmsKeyIdInputBuilder) -> Result<ResetEbsDefaultKmsKeyIdOutput, SdkError<ResetEbsDefaultKmsKeyIdError>>;
        async fn reset_fpga_image_attribute(&self, builder: ResetFpgaImageAttributeInputBuilder) -> Result<ResetFpgaImageAttributeOutput, SdkError<ResetFpgaImageAttributeError>>;
        async fn reset_image_attribute(&self, builder: ResetImageAttributeInputBuilder) -> Result<ResetImageAttributeOutput, SdkError<ResetImageAttributeError>>;
        async fn reset_instance_attribute(&self, builder: ResetInstanceAttributeInputBuilder) -> Result<ResetInstanceAttributeOutput, SdkError<ResetInstanceAttributeError>>;
        async fn reset_network_interface_attribute(&self, builder: ResetNetworkInterfaceAttributeInputBuilder) -> Result<ResetNetworkInterfaceAttributeOutput, SdkError<ResetNetworkInterfaceAttributeError>>;
        async fn reset_snapshot_attribute(&self, builder: ResetSnapshotAttributeInputBuilder) -> Result<ResetSnapshotAttributeOutput, SdkError<ResetSnapshotAttributeError>>;
        async fn restore_address_to_classic(&self, builder: RestoreAddressToClassicInputBuilder) -> Result<RestoreAddressToClassicOutput, SdkError<RestoreAddressToClassicError>>;
        async fn restore_image_from_recycle_bin(&self, builder: RestoreImageFromRecycleBinInputBuilder) -> Result<RestoreImageFromRecycleBinOutput, SdkError<RestoreImageFromRecycleBinError>>;
        async fn restore_managed_prefix_list_version(&self, builder: RestoreManagedPrefixListVersionInputBuilder) -> Result<RestoreManagedPrefixListVersionOutput, SdkError<RestoreManagedPrefixListVersionError>>;
        async fn restore_snapshot_from_recycle_bin(&self, builder: RestoreSnapshotFromRecycleBinInputBuilder) -> Result<RestoreSnapshotFromRecycleBinOutput, SdkError<RestoreSnapshotFromRecycleBinError>>;
        async fn restore_snapshot_tier(&self, builder: RestoreSnapshotTierInputBuilder) -> Result<RestoreSnapshotTierOutput, SdkError<RestoreSnapshotTierError>>;
        async fn revoke_client_vpn_ingress(&self, builder: RevokeClientVpnIngressInputBuilder) -> Result<RevokeClientVpnIngressOutput, SdkError<RevokeClientVpnIngressError>>;
        async fn revoke_security_group_egress(&self, builder: RevokeSecurityGroupEgressInputBuilder) -> Result<RevokeSecurityGroupEgressOutput, SdkError<RevokeSecurityGroupEgressError>>;
        async fn revoke_security_group_ingress(&self, builder: RevokeSecurityGroupIngressInputBuilder) -> Result<RevokeSecurityGroupIngressOutput, SdkError<RevokeSecurityGroupIngressError>>;
        async fn run_instances(&self, builder: RunInstancesInputBuilder) -> Result<RunInstancesOutput, SdkError<RunInstancesError>>;
        async fn run_scheduled_instances(&self, builder: RunScheduledInstancesInputBuilder) -> Result<RunScheduledInstancesOutput, SdkError<RunScheduledInstancesError>>;
        async fn search_local_gateway_routes(&self, builder: SearchLocalGatewayRoutesInputBuilder) -> Result<SearchLocalGatewayRoutesOutput, SdkError<SearchLocalGatewayRoutesError>>;
        async fn search_transit_gateway_multicast_groups(&self, builder: SearchTransitGatewayMulticastGroupsInputBuilder) -> Result<SearchTransitGatewayMulticastGroupsOutput, SdkError<SearchTransitGatewayMulticastGroupsError>>;
        async fn search_transit_gateway_routes(&self, builder: SearchTransitGatewayRoutesInputBuilder) -> Result<SearchTransitGatewayRoutesOutput, SdkError<SearchTransitGatewayRoutesError>>;
        async fn send_diagnostic_interrupt(&self, builder: SendDiagnosticInterruptInputBuilder) -> Result<SendDiagnosticInterruptOutput, SdkError<SendDiagnosticInterruptError>>;
        async fn start_instances(&self, builder: StartInstancesInputBuilder) -> Result<StartInstancesOutput, SdkError<StartInstancesError>>;
        async fn start_network_insights_access_scope_analysis(&self, builder: StartNetworkInsightsAccessScopeAnalysisInputBuilder) -> Result<StartNetworkInsightsAccessScopeAnalysisOutput, SdkError<StartNetworkInsightsAccessScopeAnalysisError>>;
        async fn start_network_insights_analysis(&self, builder: StartNetworkInsightsAnalysisInputBuilder) -> Result<StartNetworkInsightsAnalysisOutput, SdkError<StartNetworkInsightsAnalysisError>>;
        async fn start_vpc_endpoint_service_private_dns_verification(&self, builder: StartVpcEndpointServicePrivateDnsVerificationInputBuilder) -> Result<StartVpcEndpointServicePrivateDnsVerificationOutput, SdkError<StartVpcEndpointServicePrivateDnsVerificationError>>;
        async fn stop_instances(&self, builder: StopInstancesInputBuilder) -> Result<StopInstancesOutput, SdkError<StopInstancesError>>;
        async fn terminate_client_vpn_connections(&self, builder: TerminateClientVpnConnectionsInputBuilder) -> Result<TerminateClientVpnConnectionsOutput, SdkError<TerminateClientVpnConnectionsError>>;
        async fn terminate_instances(&self, builder: TerminateInstancesInputBuilder) -> Result<TerminateInstancesOutput, SdkError<TerminateInstancesError>>;
        async fn unassign_ipv6_addresses(&self, builder: UnassignIpv6AddressesInputBuilder) -> Result<UnassignIpv6AddressesOutput, SdkError<UnassignIpv6AddressesError>>;
        async fn unassign_private_ip_addresses(&self, builder: UnassignPrivateIpAddressesInputBuilder) -> Result<UnassignPrivateIpAddressesOutput, SdkError<UnassignPrivateIpAddressesError>>;
        async fn unassign_private_nat_gateway_address(&self, builder: UnassignPrivateNatGatewayAddressInputBuilder) -> Result<UnassignPrivateNatGatewayAddressOutput, SdkError<UnassignPrivateNatGatewayAddressError>>;
        async fn unlock_snapshot(&self, builder: UnlockSnapshotInputBuilder) -> Result<UnlockSnapshotOutput, SdkError<UnlockSnapshotError>>;
        async fn unmonitor_instances(&self, builder: UnmonitorInstancesInputBuilder) -> Result<UnmonitorInstancesOutput, SdkError<UnmonitorInstancesError>>;
        async fn update_security_group_rule_descriptions_egress(&self, builder: UpdateSecurityGroupRuleDescriptionsEgressInputBuilder) -> Result<UpdateSecurityGroupRuleDescriptionsEgressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsEgressError>>;
        async fn update_security_group_rule_descriptions_ingress(&self, builder: UpdateSecurityGroupRuleDescriptionsIngressInputBuilder) -> Result<UpdateSecurityGroupRuleDescriptionsIngressOutput, SdkError<UpdateSecurityGroupRuleDescriptionsIngressError>>;
        async fn withdraw_byoip_cidr(&self, builder: WithdrawByoipCidrInputBuilder) -> Result<WithdrawByoipCidrOutput, SdkError<WithdrawByoipCidrError>>;
    }
}
