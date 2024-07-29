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
use aws_sdk_redshift::operation::accept_reserved_node_exchange::{builders::*, *};
use aws_sdk_redshift::operation::add_partner::{builders::*, *};
use aws_sdk_redshift::operation::associate_data_share_consumer::{builders::*, *};
use aws_sdk_redshift::operation::authorize_cluster_security_group_ingress::{builders::*, *};
use aws_sdk_redshift::operation::authorize_data_share::{builders::*, *};
use aws_sdk_redshift::operation::authorize_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::authorize_snapshot_access::{builders::*, *};
use aws_sdk_redshift::operation::batch_delete_cluster_snapshots::{builders::*, *};
use aws_sdk_redshift::operation::batch_modify_cluster_snapshots::{builders::*, *};
use aws_sdk_redshift::operation::cancel_resize::{builders::*, *};
use aws_sdk_redshift::operation::copy_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::create_authentication_profile::{builders::*, *};
use aws_sdk_redshift::operation::create_cluster::{builders::*, *};
use aws_sdk_redshift::operation::create_cluster_parameter_group::{builders::*, *};
use aws_sdk_redshift::operation::create_cluster_security_group::{builders::*, *};
use aws_sdk_redshift::operation::create_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::create_cluster_subnet_group::{builders::*, *};
use aws_sdk_redshift::operation::create_custom_domain_association::{builders::*, *};
use aws_sdk_redshift::operation::create_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::create_event_subscription::{builders::*, *};
use aws_sdk_redshift::operation::create_hsm_client_certificate::{builders::*, *};
use aws_sdk_redshift::operation::create_hsm_configuration::{builders::*, *};
use aws_sdk_redshift::operation::create_redshift_idc_application::{builders::*, *};
use aws_sdk_redshift::operation::create_scheduled_action::{builders::*, *};
use aws_sdk_redshift::operation::create_snapshot_copy_grant::{builders::*, *};
use aws_sdk_redshift::operation::create_snapshot_schedule::{builders::*, *};
use aws_sdk_redshift::operation::create_tags::{builders::*, *};
use aws_sdk_redshift::operation::create_usage_limit::{builders::*, *};
use aws_sdk_redshift::operation::deauthorize_data_share::{builders::*, *};
use aws_sdk_redshift::operation::delete_authentication_profile::{builders::*, *};
use aws_sdk_redshift::operation::delete_cluster::{builders::*, *};
use aws_sdk_redshift::operation::delete_cluster_parameter_group::{builders::*, *};
use aws_sdk_redshift::operation::delete_cluster_security_group::{builders::*, *};
use aws_sdk_redshift::operation::delete_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::delete_cluster_subnet_group::{builders::*, *};
use aws_sdk_redshift::operation::delete_custom_domain_association::{builders::*, *};
use aws_sdk_redshift::operation::delete_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::delete_event_subscription::{builders::*, *};
use aws_sdk_redshift::operation::delete_hsm_client_certificate::{builders::*, *};
use aws_sdk_redshift::operation::delete_hsm_configuration::{builders::*, *};
use aws_sdk_redshift::operation::delete_partner::{builders::*, *};
use aws_sdk_redshift::operation::delete_redshift_idc_application::{builders::*, *};
use aws_sdk_redshift::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_redshift::operation::delete_scheduled_action::{builders::*, *};
use aws_sdk_redshift::operation::delete_snapshot_copy_grant::{builders::*, *};
use aws_sdk_redshift::operation::delete_snapshot_schedule::{builders::*, *};
use aws_sdk_redshift::operation::delete_tags::{builders::*, *};
use aws_sdk_redshift::operation::delete_usage_limit::{builders::*, *};
use aws_sdk_redshift::operation::describe_account_attributes::{builders::*, *};
use aws_sdk_redshift::operation::describe_authentication_profiles::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_db_revisions::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_parameter_groups::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_parameters::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_security_groups::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_snapshots::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_subnet_groups::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_tracks::{builders::*, *};
use aws_sdk_redshift::operation::describe_cluster_versions::{builders::*, *};
use aws_sdk_redshift::operation::describe_clusters::{builders::*, *};
use aws_sdk_redshift::operation::describe_custom_domain_associations::{builders::*, *};
use aws_sdk_redshift::operation::describe_data_shares::{builders::*, *};
use aws_sdk_redshift::operation::describe_data_shares_for_consumer::{builders::*, *};
use aws_sdk_redshift::operation::describe_data_shares_for_producer::{builders::*, *};
use aws_sdk_redshift::operation::describe_default_cluster_parameters::{builders::*, *};
use aws_sdk_redshift::operation::describe_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::describe_endpoint_authorization::{builders::*, *};
use aws_sdk_redshift::operation::describe_event_categories::{builders::*, *};
use aws_sdk_redshift::operation::describe_event_subscriptions::{builders::*, *};
use aws_sdk_redshift::operation::describe_events::{builders::*, *};
use aws_sdk_redshift::operation::describe_hsm_client_certificates::{builders::*, *};
use aws_sdk_redshift::operation::describe_hsm_configurations::{builders::*, *};
use aws_sdk_redshift::operation::describe_inbound_integrations::{builders::*, *};
use aws_sdk_redshift::operation::describe_logging_status::{builders::*, *};
use aws_sdk_redshift::operation::describe_node_configuration_options::{builders::*, *};
use aws_sdk_redshift::operation::describe_orderable_cluster_options::{builders::*, *};
use aws_sdk_redshift::operation::describe_partners::{builders::*, *};
use aws_sdk_redshift::operation::describe_redshift_idc_applications::{builders::*, *};
use aws_sdk_redshift::operation::describe_reserved_node_exchange_status::{builders::*, *};
use aws_sdk_redshift::operation::describe_reserved_node_offerings::{builders::*, *};
use aws_sdk_redshift::operation::describe_reserved_nodes::{builders::*, *};
use aws_sdk_redshift::operation::describe_resize::{builders::*, *};
use aws_sdk_redshift::operation::describe_scheduled_actions::{builders::*, *};
use aws_sdk_redshift::operation::describe_snapshot_copy_grants::{builders::*, *};
use aws_sdk_redshift::operation::describe_snapshot_schedules::{builders::*, *};
use aws_sdk_redshift::operation::describe_storage::{builders::*, *};
use aws_sdk_redshift::operation::describe_table_restore_status::{builders::*, *};
use aws_sdk_redshift::operation::describe_tags::{builders::*, *};
use aws_sdk_redshift::operation::describe_usage_limits::{builders::*, *};
use aws_sdk_redshift::operation::disable_logging::{builders::*, *};
use aws_sdk_redshift::operation::disable_snapshot_copy::{builders::*, *};
use aws_sdk_redshift::operation::disassociate_data_share_consumer::{builders::*, *};
use aws_sdk_redshift::operation::enable_logging::{builders::*, *};
use aws_sdk_redshift::operation::enable_snapshot_copy::{builders::*, *};
use aws_sdk_redshift::operation::failover_primary_compute::{builders::*, *};
use aws_sdk_redshift::operation::get_cluster_credentials::{builders::*, *};
use aws_sdk_redshift::operation::get_cluster_credentials_with_iam::{builders::*, *};
use aws_sdk_redshift::operation::get_reserved_node_exchange_configuration_options::{builders::*, *};
use aws_sdk_redshift::operation::get_reserved_node_exchange_offerings::{builders::*, *};
use aws_sdk_redshift::operation::get_resource_policy::{builders::*, *};
use aws_sdk_redshift::operation::list_recommendations::{builders::*, *};
use aws_sdk_redshift::operation::modify_aqua_configuration::{builders::*, *};
use aws_sdk_redshift::operation::modify_authentication_profile::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_db_revision::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_iam_roles::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_maintenance::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_parameter_group::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_snapshot_schedule::{builders::*, *};
use aws_sdk_redshift::operation::modify_cluster_subnet_group::{builders::*, *};
use aws_sdk_redshift::operation::modify_custom_domain_association::{builders::*, *};
use aws_sdk_redshift::operation::modify_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::modify_event_subscription::{builders::*, *};
use aws_sdk_redshift::operation::modify_redshift_idc_application::{builders::*, *};
use aws_sdk_redshift::operation::modify_scheduled_action::{builders::*, *};
use aws_sdk_redshift::operation::modify_snapshot_copy_retention_period::{builders::*, *};
use aws_sdk_redshift::operation::modify_snapshot_schedule::{builders::*, *};
use aws_sdk_redshift::operation::modify_usage_limit::{builders::*, *};
use aws_sdk_redshift::operation::pause_cluster::{builders::*, *};
use aws_sdk_redshift::operation::purchase_reserved_node_offering::{builders::*, *};
use aws_sdk_redshift::operation::put_resource_policy::{builders::*, *};
use aws_sdk_redshift::operation::reboot_cluster::{builders::*, *};
use aws_sdk_redshift::operation::reject_data_share::{builders::*, *};
use aws_sdk_redshift::operation::reset_cluster_parameter_group::{builders::*, *};
use aws_sdk_redshift::operation::resize_cluster::{builders::*, *};
use aws_sdk_redshift::operation::restore_from_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::restore_table_from_cluster_snapshot::{builders::*, *};
use aws_sdk_redshift::operation::resume_cluster::{builders::*, *};
use aws_sdk_redshift::operation::revoke_cluster_security_group_ingress::{builders::*, *};
use aws_sdk_redshift::operation::revoke_endpoint_access::{builders::*, *};
use aws_sdk_redshift::operation::revoke_snapshot_access::{builders::*, *};
use aws_sdk_redshift::operation::rotate_encryption_key::{builders::*, *};
use aws_sdk_redshift::operation::update_partner_status::{builders::*, *};
use aws_sdk_redshift::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_redshift::Client;
use std::ops::Deref;

pub use aws_sdk_redshift::*;

pub struct RedshiftClientImpl(Client);
impl RedshiftClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait RedshiftClient {
    fn accept_reserved_node_exchange(&self, builder: AcceptReservedNodeExchangeInputBuilder) -> impl Future<Output = Result<AcceptReservedNodeExchangeOutput, SdkError<AcceptReservedNodeExchangeError>>>;
    fn add_partner(&self, builder: AddPartnerInputBuilder) -> impl Future<Output = Result<AddPartnerOutput, SdkError<AddPartnerError>>>;
    fn associate_data_share_consumer(&self, builder: AssociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<AssociateDataShareConsumerOutput, SdkError<AssociateDataShareConsumerError>>>;
    fn authorize_cluster_security_group_ingress(&self, builder: AuthorizeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClusterSecurityGroupIngressOutput, SdkError<AuthorizeClusterSecurityGroupIngressError>>>;
    fn authorize_data_share(&self, builder: AuthorizeDataShareInputBuilder) -> impl Future<Output = Result<AuthorizeDataShareOutput, SdkError<AuthorizeDataShareError>>>;
    fn authorize_endpoint_access(&self, builder: AuthorizeEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeEndpointAccessOutput, SdkError<AuthorizeEndpointAccessError>>>;
    fn authorize_snapshot_access(&self, builder: AuthorizeSnapshotAccessInputBuilder) -> impl Future<Output = Result<AuthorizeSnapshotAccessOutput, SdkError<AuthorizeSnapshotAccessError>>>;
    fn batch_delete_cluster_snapshots(&self, builder: BatchDeleteClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchDeleteClusterSnapshotsOutput, SdkError<BatchDeleteClusterSnapshotsError>>>;
    fn batch_modify_cluster_snapshots(&self, builder: BatchModifyClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchModifyClusterSnapshotsOutput, SdkError<BatchModifyClusterSnapshotsError>>>;
    fn cancel_resize(&self, builder: CancelResizeInputBuilder) -> impl Future<Output = Result<CancelResizeOutput, SdkError<CancelResizeError>>>;
    fn copy_cluster_snapshot(&self, builder: CopyClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyClusterSnapshotOutput, SdkError<CopyClusterSnapshotError>>>;
    fn create_authentication_profile(&self, builder: CreateAuthenticationProfileInputBuilder) -> impl Future<Output = Result<CreateAuthenticationProfileOutput, SdkError<CreateAuthenticationProfileError>>>;
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>>;
    fn create_cluster_parameter_group(&self, builder: CreateClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateClusterParameterGroupOutput, SdkError<CreateClusterParameterGroupError>>>;
    fn create_cluster_security_group(&self, builder: CreateClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSecurityGroupOutput, SdkError<CreateClusterSecurityGroupError>>>;
    fn create_cluster_snapshot(&self, builder: CreateClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateClusterSnapshotOutput, SdkError<CreateClusterSnapshotError>>>;
    fn create_cluster_subnet_group(&self, builder: CreateClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSubnetGroupOutput, SdkError<CreateClusterSubnetGroupError>>>;
    fn create_custom_domain_association(&self, builder: CreateCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateCustomDomainAssociationOutput, SdkError<CreateCustomDomainAssociationError>>>;
    fn create_endpoint_access(&self, builder: CreateEndpointAccessInputBuilder) -> impl Future<Output = Result<CreateEndpointAccessOutput, SdkError<CreateEndpointAccessError>>>;
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>>;
    fn create_hsm_client_certificate(&self, builder: CreateHsmClientCertificateInputBuilder) -> impl Future<Output = Result<CreateHsmClientCertificateOutput, SdkError<CreateHsmClientCertificateError>>>;
    fn create_hsm_configuration(&self, builder: CreateHsmConfigurationInputBuilder) -> impl Future<Output = Result<CreateHsmConfigurationOutput, SdkError<CreateHsmConfigurationError>>>;
    fn create_redshift_idc_application(&self, builder: CreateRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<CreateRedshiftIdcApplicationOutput, SdkError<CreateRedshiftIdcApplicationError>>>;
    fn create_scheduled_action(&self, builder: CreateScheduledActionInputBuilder) -> impl Future<Output = Result<CreateScheduledActionOutput, SdkError<CreateScheduledActionError>>>;
    fn create_snapshot_copy_grant(&self, builder: CreateSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<CreateSnapshotCopyGrantOutput, SdkError<CreateSnapshotCopyGrantError>>>;
    fn create_snapshot_schedule(&self, builder: CreateSnapshotScheduleInputBuilder) -> impl Future<Output = Result<CreateSnapshotScheduleOutput, SdkError<CreateSnapshotScheduleError>>>;
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>>;
    fn create_usage_limit(&self, builder: CreateUsageLimitInputBuilder) -> impl Future<Output = Result<CreateUsageLimitOutput, SdkError<CreateUsageLimitError>>>;
    fn deauthorize_data_share(&self, builder: DeauthorizeDataShareInputBuilder) -> impl Future<Output = Result<DeauthorizeDataShareOutput, SdkError<DeauthorizeDataShareError>>>;
    fn delete_authentication_profile(&self, builder: DeleteAuthenticationProfileInputBuilder) -> impl Future<Output = Result<DeleteAuthenticationProfileOutput, SdkError<DeleteAuthenticationProfileError>>>;
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>>;
    fn delete_cluster_parameter_group(&self, builder: DeleteClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterParameterGroupOutput, SdkError<DeleteClusterParameterGroupError>>>;
    fn delete_cluster_security_group(&self, builder: DeleteClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSecurityGroupOutput, SdkError<DeleteClusterSecurityGroupError>>>;
    fn delete_cluster_snapshot(&self, builder: DeleteClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteClusterSnapshotOutput, SdkError<DeleteClusterSnapshotError>>>;
    fn delete_cluster_subnet_group(&self, builder: DeleteClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSubnetGroupOutput, SdkError<DeleteClusterSubnetGroupError>>>;
    fn delete_custom_domain_association(&self, builder: DeleteCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteCustomDomainAssociationOutput, SdkError<DeleteCustomDomainAssociationError>>>;
    fn delete_endpoint_access(&self, builder: DeleteEndpointAccessInputBuilder) -> impl Future<Output = Result<DeleteEndpointAccessOutput, SdkError<DeleteEndpointAccessError>>>;
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>>;
    fn delete_hsm_client_certificate(&self, builder: DeleteHsmClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteHsmClientCertificateOutput, SdkError<DeleteHsmClientCertificateError>>>;
    fn delete_hsm_configuration(&self, builder: DeleteHsmConfigurationInputBuilder) -> impl Future<Output = Result<DeleteHsmConfigurationOutput, SdkError<DeleteHsmConfigurationError>>>;
    fn delete_partner(&self, builder: DeletePartnerInputBuilder) -> impl Future<Output = Result<DeletePartnerOutput, SdkError<DeletePartnerError>>>;
    fn delete_redshift_idc_application(&self, builder: DeleteRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<DeleteRedshiftIdcApplicationOutput, SdkError<DeleteRedshiftIdcApplicationError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn delete_scheduled_action(&self, builder: DeleteScheduledActionInputBuilder) -> impl Future<Output = Result<DeleteScheduledActionOutput, SdkError<DeleteScheduledActionError>>>;
    fn delete_snapshot_copy_grant(&self, builder: DeleteSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<DeleteSnapshotCopyGrantOutput, SdkError<DeleteSnapshotCopyGrantError>>>;
    fn delete_snapshot_schedule(&self, builder: DeleteSnapshotScheduleInputBuilder) -> impl Future<Output = Result<DeleteSnapshotScheduleOutput, SdkError<DeleteSnapshotScheduleError>>>;
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>>;
    fn delete_usage_limit(&self, builder: DeleteUsageLimitInputBuilder) -> impl Future<Output = Result<DeleteUsageLimitOutput, SdkError<DeleteUsageLimitError>>>;
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>>;
    fn describe_authentication_profiles(&self, builder: DescribeAuthenticationProfilesInputBuilder) -> impl Future<Output = Result<DescribeAuthenticationProfilesOutput, SdkError<DescribeAuthenticationProfilesError>>>;
    fn describe_cluster_db_revisions(&self, builder: DescribeClusterDbRevisionsInputBuilder) -> impl Future<Output = Result<DescribeClusterDbRevisionsOutput, SdkError<DescribeClusterDbRevisionsError>>>;
    fn describe_cluster_parameter_groups(&self, builder: DescribeClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterParameterGroupsOutput, SdkError<DescribeClusterParameterGroupsError>>>;
    fn describe_cluster_parameters(&self, builder: DescribeClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeClusterParametersOutput, SdkError<DescribeClusterParametersError>>>;
    fn describe_cluster_security_groups(&self, builder: DescribeClusterSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSecurityGroupsOutput, SdkError<DescribeClusterSecurityGroupsError>>>;
    fn describe_cluster_snapshots(&self, builder: DescribeClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeClusterSnapshotsOutput, SdkError<DescribeClusterSnapshotsError>>>;
    fn describe_cluster_subnet_groups(&self, builder: DescribeClusterSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSubnetGroupsOutput, SdkError<DescribeClusterSubnetGroupsError>>>;
    fn describe_cluster_tracks(&self, builder: DescribeClusterTracksInputBuilder) -> impl Future<Output = Result<DescribeClusterTracksOutput, SdkError<DescribeClusterTracksError>>>;
    fn describe_cluster_versions(&self, builder: DescribeClusterVersionsInputBuilder) -> impl Future<Output = Result<DescribeClusterVersionsOutput, SdkError<DescribeClusterVersionsError>>>;
    fn describe_clusters(&self, builder: DescribeClustersInputBuilder) -> impl Future<Output = Result<DescribeClustersOutput, SdkError<DescribeClustersError>>>;
    fn describe_custom_domain_associations(&self, builder: DescribeCustomDomainAssociationsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainAssociationsOutput, SdkError<DescribeCustomDomainAssociationsError>>>;
    fn describe_data_shares(&self, builder: DescribeDataSharesInputBuilder) -> impl Future<Output = Result<DescribeDataSharesOutput, SdkError<DescribeDataSharesError>>>;
    fn describe_data_shares_for_consumer(&self, builder: DescribeDataSharesForConsumerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForConsumerOutput, SdkError<DescribeDataSharesForConsumerError>>>;
    fn describe_data_shares_for_producer(&self, builder: DescribeDataSharesForProducerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForProducerOutput, SdkError<DescribeDataSharesForProducerError>>>;
    fn describe_default_cluster_parameters(&self, builder: DescribeDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDefaultClusterParametersOutput, SdkError<DescribeDefaultClusterParametersError>>>;
    fn describe_endpoint_access(&self, builder: DescribeEndpointAccessInputBuilder) -> impl Future<Output = Result<DescribeEndpointAccessOutput, SdkError<DescribeEndpointAccessError>>>;
    fn describe_endpoint_authorization(&self, builder: DescribeEndpointAuthorizationInputBuilder) -> impl Future<Output = Result<DescribeEndpointAuthorizationOutput, SdkError<DescribeEndpointAuthorizationError>>>;
    fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> impl Future<Output = Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>>;
    fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>>;
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>>;
    fn describe_hsm_client_certificates(&self, builder: DescribeHsmClientCertificatesInputBuilder) -> impl Future<Output = Result<DescribeHsmClientCertificatesOutput, SdkError<DescribeHsmClientCertificatesError>>>;
    fn describe_hsm_configurations(&self, builder: DescribeHsmConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeHsmConfigurationsOutput, SdkError<DescribeHsmConfigurationsError>>>;
    fn describe_inbound_integrations(&self, builder: DescribeInboundIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeInboundIntegrationsOutput, SdkError<DescribeInboundIntegrationsError>>>;
    fn describe_logging_status(&self, builder: DescribeLoggingStatusInputBuilder) -> impl Future<Output = Result<DescribeLoggingStatusOutput, SdkError<DescribeLoggingStatusError>>>;
    fn describe_node_configuration_options(&self, builder: DescribeNodeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeNodeConfigurationOptionsOutput, SdkError<DescribeNodeConfigurationOptionsError>>>;
    fn describe_orderable_cluster_options(&self, builder: DescribeOrderableClusterOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableClusterOptionsOutput, SdkError<DescribeOrderableClusterOptionsError>>>;
    fn describe_partners(&self, builder: DescribePartnersInputBuilder) -> impl Future<Output = Result<DescribePartnersOutput, SdkError<DescribePartnersError>>>;
    fn describe_redshift_idc_applications(&self, builder: DescribeRedshiftIdcApplicationsInputBuilder) -> impl Future<Output = Result<DescribeRedshiftIdcApplicationsOutput, SdkError<DescribeRedshiftIdcApplicationsError>>>;
    fn describe_reserved_node_exchange_status(&self, builder: DescribeReservedNodeExchangeStatusInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeExchangeStatusOutput, SdkError<DescribeReservedNodeExchangeStatusError>>>;
    fn describe_reserved_node_offerings(&self, builder: DescribeReservedNodeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeOfferingsOutput, SdkError<DescribeReservedNodeOfferingsError>>>;
    fn describe_reserved_nodes(&self, builder: DescribeReservedNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedNodesOutput, SdkError<DescribeReservedNodesError>>>;
    fn describe_resize(&self, builder: DescribeResizeInputBuilder) -> impl Future<Output = Result<DescribeResizeOutput, SdkError<DescribeResizeError>>>;
    fn describe_scheduled_actions(&self, builder: DescribeScheduledActionsInputBuilder) -> impl Future<Output = Result<DescribeScheduledActionsOutput, SdkError<DescribeScheduledActionsError>>>;
    fn describe_snapshot_copy_grants(&self, builder: DescribeSnapshotCopyGrantsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotCopyGrantsOutput, SdkError<DescribeSnapshotCopyGrantsError>>>;
    fn describe_snapshot_schedules(&self, builder: DescribeSnapshotSchedulesInputBuilder) -> impl Future<Output = Result<DescribeSnapshotSchedulesOutput, SdkError<DescribeSnapshotSchedulesError>>>;
    fn describe_storage(&self, builder: DescribeStorageInputBuilder) -> impl Future<Output = Result<DescribeStorageOutput, SdkError<DescribeStorageError>>>;
    fn describe_table_restore_status(&self, builder: DescribeTableRestoreStatusInputBuilder) -> impl Future<Output = Result<DescribeTableRestoreStatusOutput, SdkError<DescribeTableRestoreStatusError>>>;
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>>;
    fn describe_usage_limits(&self, builder: DescribeUsageLimitsInputBuilder) -> impl Future<Output = Result<DescribeUsageLimitsOutput, SdkError<DescribeUsageLimitsError>>>;
    fn disable_logging(&self, builder: DisableLoggingInputBuilder) -> impl Future<Output = Result<DisableLoggingOutput, SdkError<DisableLoggingError>>>;
    fn disable_snapshot_copy(&self, builder: DisableSnapshotCopyInputBuilder) -> impl Future<Output = Result<DisableSnapshotCopyOutput, SdkError<DisableSnapshotCopyError>>>;
    fn disassociate_data_share_consumer(&self, builder: DisassociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<DisassociateDataShareConsumerOutput, SdkError<DisassociateDataShareConsumerError>>>;
    fn enable_logging(&self, builder: EnableLoggingInputBuilder) -> impl Future<Output = Result<EnableLoggingOutput, SdkError<EnableLoggingError>>>;
    fn enable_snapshot_copy(&self, builder: EnableSnapshotCopyInputBuilder) -> impl Future<Output = Result<EnableSnapshotCopyOutput, SdkError<EnableSnapshotCopyError>>>;
    fn failover_primary_compute(&self, builder: FailoverPrimaryComputeInputBuilder) -> impl Future<Output = Result<FailoverPrimaryComputeOutput, SdkError<FailoverPrimaryComputeError>>>;
    fn get_cluster_credentials(&self, builder: GetClusterCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsOutput, SdkError<GetClusterCredentialsError>>>;
    fn get_cluster_credentials_with_iam(&self, builder: GetClusterCredentialsWithIamInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsWithIamOutput, SdkError<GetClusterCredentialsWithIAMError>>>;
    fn get_reserved_node_exchange_configuration_options(&self, builder: GetReservedNodeExchangeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeConfigurationOptionsOutput, SdkError<GetReservedNodeExchangeConfigurationOptionsError>>>;
    fn get_reserved_node_exchange_offerings(&self, builder: GetReservedNodeExchangeOfferingsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeOfferingsOutput, SdkError<GetReservedNodeExchangeOfferingsError>>>;
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>>;
    fn list_recommendations(&self, builder: ListRecommendationsInputBuilder) -> impl Future<Output = Result<ListRecommendationsOutput, SdkError<ListRecommendationsError>>>;
    fn modify_aqua_configuration(&self, builder: ModifyAquaConfigurationInputBuilder) -> impl Future<Output = Result<ModifyAquaConfigurationOutput, SdkError<ModifyAquaConfigurationError>>>;
    fn modify_authentication_profile(&self, builder: ModifyAuthenticationProfileInputBuilder) -> impl Future<Output = Result<ModifyAuthenticationProfileOutput, SdkError<ModifyAuthenticationProfileError>>>;
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>>;
    fn modify_cluster_db_revision(&self, builder: ModifyClusterDbRevisionInputBuilder) -> impl Future<Output = Result<ModifyClusterDbRevisionOutput, SdkError<ModifyClusterDbRevisionError>>>;
    fn modify_cluster_iam_roles(&self, builder: ModifyClusterIamRolesInputBuilder) -> impl Future<Output = Result<ModifyClusterIamRolesOutput, SdkError<ModifyClusterIamRolesError>>>;
    fn modify_cluster_maintenance(&self, builder: ModifyClusterMaintenanceInputBuilder) -> impl Future<Output = Result<ModifyClusterMaintenanceOutput, SdkError<ModifyClusterMaintenanceError>>>;
    fn modify_cluster_parameter_group(&self, builder: ModifyClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterParameterGroupOutput, SdkError<ModifyClusterParameterGroupError>>>;
    fn modify_cluster_snapshot(&self, builder: ModifyClusterSnapshotInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotOutput, SdkError<ModifyClusterSnapshotError>>>;
    fn modify_cluster_snapshot_schedule(&self, builder: ModifyClusterSnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotScheduleOutput, SdkError<ModifyClusterSnapshotScheduleError>>>;
    fn modify_cluster_subnet_group(&self, builder: ModifyClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterSubnetGroupOutput, SdkError<ModifyClusterSubnetGroupError>>>;
    fn modify_custom_domain_association(&self, builder: ModifyCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<ModifyCustomDomainAssociationOutput, SdkError<ModifyCustomDomainAssociationError>>>;
    fn modify_endpoint_access(&self, builder: ModifyEndpointAccessInputBuilder) -> impl Future<Output = Result<ModifyEndpointAccessOutput, SdkError<ModifyEndpointAccessError>>>;
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>>;
    fn modify_redshift_idc_application(&self, builder: ModifyRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<ModifyRedshiftIdcApplicationOutput, SdkError<ModifyRedshiftIdcApplicationError>>>;
    fn modify_scheduled_action(&self, builder: ModifyScheduledActionInputBuilder) -> impl Future<Output = Result<ModifyScheduledActionOutput, SdkError<ModifyScheduledActionError>>>;
    fn modify_snapshot_copy_retention_period(&self, builder: ModifySnapshotCopyRetentionPeriodInputBuilder) -> impl Future<Output = Result<ModifySnapshotCopyRetentionPeriodOutput, SdkError<ModifySnapshotCopyRetentionPeriodError>>>;
    fn modify_snapshot_schedule(&self, builder: ModifySnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifySnapshotScheduleOutput, SdkError<ModifySnapshotScheduleError>>>;
    fn modify_usage_limit(&self, builder: ModifyUsageLimitInputBuilder) -> impl Future<Output = Result<ModifyUsageLimitOutput, SdkError<ModifyUsageLimitError>>>;
    fn pause_cluster(&self, builder: PauseClusterInputBuilder) -> impl Future<Output = Result<PauseClusterOutput, SdkError<PauseClusterError>>>;
    fn purchase_reserved_node_offering(&self, builder: PurchaseReservedNodeOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedNodeOfferingOutput, SdkError<PurchaseReservedNodeOfferingError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn reboot_cluster(&self, builder: RebootClusterInputBuilder) -> impl Future<Output = Result<RebootClusterOutput, SdkError<RebootClusterError>>>;
    fn reject_data_share(&self, builder: RejectDataShareInputBuilder) -> impl Future<Output = Result<RejectDataShareOutput, SdkError<RejectDataShareError>>>;
    fn reset_cluster_parameter_group(&self, builder: ResetClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetClusterParameterGroupOutput, SdkError<ResetClusterParameterGroupError>>>;
    fn resize_cluster(&self, builder: ResizeClusterInputBuilder) -> impl Future<Output = Result<ResizeClusterOutput, SdkError<ResizeClusterError>>>;
    fn restore_from_cluster_snapshot(&self, builder: RestoreFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreFromClusterSnapshotOutput, SdkError<RestoreFromClusterSnapshotError>>>;
    fn restore_table_from_cluster_snapshot(&self, builder: RestoreTableFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreTableFromClusterSnapshotOutput, SdkError<RestoreTableFromClusterSnapshotError>>>;
    fn resume_cluster(&self, builder: ResumeClusterInputBuilder) -> impl Future<Output = Result<ResumeClusterOutput, SdkError<ResumeClusterError>>>;
    fn revoke_cluster_security_group_ingress(&self, builder: RevokeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeClusterSecurityGroupIngressOutput, SdkError<RevokeClusterSecurityGroupIngressError>>>;
    fn revoke_endpoint_access(&self, builder: RevokeEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeEndpointAccessOutput, SdkError<RevokeEndpointAccessError>>>;
    fn revoke_snapshot_access(&self, builder: RevokeSnapshotAccessInputBuilder) -> impl Future<Output = Result<RevokeSnapshotAccessOutput, SdkError<RevokeSnapshotAccessError>>>;
    fn rotate_encryption_key(&self, builder: RotateEncryptionKeyInputBuilder) -> impl Future<Output = Result<RotateEncryptionKeyOutput, SdkError<RotateEncryptionKeyError>>>;
    fn update_partner_status(&self, builder: UpdatePartnerStatusInputBuilder) -> impl Future<Output = Result<UpdatePartnerStatusOutput, SdkError<UpdatePartnerStatusError>>>;
}
impl RedshiftClient for RedshiftClientImpl {
    fn accept_reserved_node_exchange(&self, builder: AcceptReservedNodeExchangeInputBuilder) -> impl Future<Output = Result<AcceptReservedNodeExchangeOutput, SdkError<AcceptReservedNodeExchangeError>>> {
        builder.send_with(&self.0)
    }
    fn add_partner(&self, builder: AddPartnerInputBuilder) -> impl Future<Output = Result<AddPartnerOutput, SdkError<AddPartnerError>>> {
        builder.send_with(&self.0)
    }
    fn associate_data_share_consumer(&self, builder: AssociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<AssociateDataShareConsumerOutput, SdkError<AssociateDataShareConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_cluster_security_group_ingress(&self, builder: AuthorizeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClusterSecurityGroupIngressOutput, SdkError<AuthorizeClusterSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_data_share(&self, builder: AuthorizeDataShareInputBuilder) -> impl Future<Output = Result<AuthorizeDataShareOutput, SdkError<AuthorizeDataShareError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_endpoint_access(&self, builder: AuthorizeEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeEndpointAccessOutput, SdkError<AuthorizeEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_snapshot_access(&self, builder: AuthorizeSnapshotAccessInputBuilder) -> impl Future<Output = Result<AuthorizeSnapshotAccessOutput, SdkError<AuthorizeSnapshotAccessError>>> {
        builder.send_with(&self.0)
    }
    fn batch_delete_cluster_snapshots(&self, builder: BatchDeleteClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchDeleteClusterSnapshotsOutput, SdkError<BatchDeleteClusterSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_modify_cluster_snapshots(&self, builder: BatchModifyClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchModifyClusterSnapshotsOutput, SdkError<BatchModifyClusterSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_resize(&self, builder: CancelResizeInputBuilder) -> impl Future<Output = Result<CancelResizeOutput, SdkError<CancelResizeError>>> {
        builder.send_with(&self.0)
    }
    fn copy_cluster_snapshot(&self, builder: CopyClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyClusterSnapshotOutput, SdkError<CopyClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_authentication_profile(&self, builder: CreateAuthenticationProfileInputBuilder) -> impl Future<Output = Result<CreateAuthenticationProfileOutput, SdkError<CreateAuthenticationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster_parameter_group(&self, builder: CreateClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateClusterParameterGroupOutput, SdkError<CreateClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster_security_group(&self, builder: CreateClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSecurityGroupOutput, SdkError<CreateClusterSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster_snapshot(&self, builder: CreateClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateClusterSnapshotOutput, SdkError<CreateClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_cluster_subnet_group(&self, builder: CreateClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSubnetGroupOutput, SdkError<CreateClusterSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_domain_association(&self, builder: CreateCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateCustomDomainAssociationOutput, SdkError<CreateCustomDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_endpoint_access(&self, builder: CreateEndpointAccessInputBuilder) -> impl Future<Output = Result<CreateEndpointAccessOutput, SdkError<CreateEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_hsm_client_certificate(&self, builder: CreateHsmClientCertificateInputBuilder) -> impl Future<Output = Result<CreateHsmClientCertificateOutput, SdkError<CreateHsmClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn create_hsm_configuration(&self, builder: CreateHsmConfigurationInputBuilder) -> impl Future<Output = Result<CreateHsmConfigurationOutput, SdkError<CreateHsmConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_redshift_idc_application(&self, builder: CreateRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<CreateRedshiftIdcApplicationOutput, SdkError<CreateRedshiftIdcApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_scheduled_action(&self, builder: CreateScheduledActionInputBuilder) -> impl Future<Output = Result<CreateScheduledActionOutput, SdkError<CreateScheduledActionError>>> {
        builder.send_with(&self.0)
    }
    fn create_snapshot_copy_grant(&self, builder: CreateSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<CreateSnapshotCopyGrantOutput, SdkError<CreateSnapshotCopyGrantError>>> {
        builder.send_with(&self.0)
    }
    fn create_snapshot_schedule(&self, builder: CreateSnapshotScheduleInputBuilder) -> impl Future<Output = Result<CreateSnapshotScheduleOutput, SdkError<CreateSnapshotScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        builder.send_with(&self.0)
    }
    fn create_usage_limit(&self, builder: CreateUsageLimitInputBuilder) -> impl Future<Output = Result<CreateUsageLimitOutput, SdkError<CreateUsageLimitError>>> {
        builder.send_with(&self.0)
    }
    fn deauthorize_data_share(&self, builder: DeauthorizeDataShareInputBuilder) -> impl Future<Output = Result<DeauthorizeDataShareOutput, SdkError<DeauthorizeDataShareError>>> {
        builder.send_with(&self.0)
    }
    fn delete_authentication_profile(&self, builder: DeleteAuthenticationProfileInputBuilder) -> impl Future<Output = Result<DeleteAuthenticationProfileOutput, SdkError<DeleteAuthenticationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster_parameter_group(&self, builder: DeleteClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterParameterGroupOutput, SdkError<DeleteClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster_security_group(&self, builder: DeleteClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSecurityGroupOutput, SdkError<DeleteClusterSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster_snapshot(&self, builder: DeleteClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteClusterSnapshotOutput, SdkError<DeleteClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cluster_subnet_group(&self, builder: DeleteClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSubnetGroupOutput, SdkError<DeleteClusterSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_domain_association(&self, builder: DeleteCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteCustomDomainAssociationOutput, SdkError<DeleteCustomDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_endpoint_access(&self, builder: DeleteEndpointAccessInputBuilder) -> impl Future<Output = Result<DeleteEndpointAccessOutput, SdkError<DeleteEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hsm_client_certificate(&self, builder: DeleteHsmClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteHsmClientCertificateOutput, SdkError<DeleteHsmClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hsm_configuration(&self, builder: DeleteHsmConfigurationInputBuilder) -> impl Future<Output = Result<DeleteHsmConfigurationOutput, SdkError<DeleteHsmConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_partner(&self, builder: DeletePartnerInputBuilder) -> impl Future<Output = Result<DeletePartnerOutput, SdkError<DeletePartnerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_redshift_idc_application(&self, builder: DeleteRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<DeleteRedshiftIdcApplicationOutput, SdkError<DeleteRedshiftIdcApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_scheduled_action(&self, builder: DeleteScheduledActionInputBuilder) -> impl Future<Output = Result<DeleteScheduledActionOutput, SdkError<DeleteScheduledActionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_snapshot_copy_grant(&self, builder: DeleteSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<DeleteSnapshotCopyGrantOutput, SdkError<DeleteSnapshotCopyGrantError>>> {
        builder.send_with(&self.0)
    }
    fn delete_snapshot_schedule(&self, builder: DeleteSnapshotScheduleInputBuilder) -> impl Future<Output = Result<DeleteSnapshotScheduleOutput, SdkError<DeleteSnapshotScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_usage_limit(&self, builder: DeleteUsageLimitInputBuilder) -> impl Future<Output = Result<DeleteUsageLimitOutput, SdkError<DeleteUsageLimitError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_authentication_profiles(&self, builder: DescribeAuthenticationProfilesInputBuilder) -> impl Future<Output = Result<DescribeAuthenticationProfilesOutput, SdkError<DescribeAuthenticationProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_db_revisions(&self, builder: DescribeClusterDbRevisionsInputBuilder) -> impl Future<Output = Result<DescribeClusterDbRevisionsOutput, SdkError<DescribeClusterDbRevisionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_parameter_groups(&self, builder: DescribeClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterParameterGroupsOutput, SdkError<DescribeClusterParameterGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_parameters(&self, builder: DescribeClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeClusterParametersOutput, SdkError<DescribeClusterParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_security_groups(&self, builder: DescribeClusterSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSecurityGroupsOutput, SdkError<DescribeClusterSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_snapshots(&self, builder: DescribeClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeClusterSnapshotsOutput, SdkError<DescribeClusterSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_subnet_groups(&self, builder: DescribeClusterSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSubnetGroupsOutput, SdkError<DescribeClusterSubnetGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_tracks(&self, builder: DescribeClusterTracksInputBuilder) -> impl Future<Output = Result<DescribeClusterTracksOutput, SdkError<DescribeClusterTracksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster_versions(&self, builder: DescribeClusterVersionsInputBuilder) -> impl Future<Output = Result<DescribeClusterVersionsOutput, SdkError<DescribeClusterVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_clusters(&self, builder: DescribeClustersInputBuilder) -> impl Future<Output = Result<DescribeClustersOutput, SdkError<DescribeClustersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_domain_associations(&self, builder: DescribeCustomDomainAssociationsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainAssociationsOutput, SdkError<DescribeCustomDomainAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_shares(&self, builder: DescribeDataSharesInputBuilder) -> impl Future<Output = Result<DescribeDataSharesOutput, SdkError<DescribeDataSharesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_shares_for_consumer(&self, builder: DescribeDataSharesForConsumerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForConsumerOutput, SdkError<DescribeDataSharesForConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_data_shares_for_producer(&self, builder: DescribeDataSharesForProducerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForProducerOutput, SdkError<DescribeDataSharesForProducerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_default_cluster_parameters(&self, builder: DescribeDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDefaultClusterParametersOutput, SdkError<DescribeDefaultClusterParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoint_access(&self, builder: DescribeEndpointAccessInputBuilder) -> impl Future<Output = Result<DescribeEndpointAccessOutput, SdkError<DescribeEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn describe_endpoint_authorization(&self, builder: DescribeEndpointAuthorizationInputBuilder) -> impl Future<Output = Result<DescribeEndpointAuthorizationOutput, SdkError<DescribeEndpointAuthorizationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> impl Future<Output = Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hsm_client_certificates(&self, builder: DescribeHsmClientCertificatesInputBuilder) -> impl Future<Output = Result<DescribeHsmClientCertificatesOutput, SdkError<DescribeHsmClientCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_hsm_configurations(&self, builder: DescribeHsmConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeHsmConfigurationsOutput, SdkError<DescribeHsmConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inbound_integrations(&self, builder: DescribeInboundIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeInboundIntegrationsOutput, SdkError<DescribeInboundIntegrationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_logging_status(&self, builder: DescribeLoggingStatusInputBuilder) -> impl Future<Output = Result<DescribeLoggingStatusOutput, SdkError<DescribeLoggingStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_node_configuration_options(&self, builder: DescribeNodeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeNodeConfigurationOptionsOutput, SdkError<DescribeNodeConfigurationOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_orderable_cluster_options(&self, builder: DescribeOrderableClusterOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableClusterOptionsOutput, SdkError<DescribeOrderableClusterOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_partners(&self, builder: DescribePartnersInputBuilder) -> impl Future<Output = Result<DescribePartnersOutput, SdkError<DescribePartnersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_redshift_idc_applications(&self, builder: DescribeRedshiftIdcApplicationsInputBuilder) -> impl Future<Output = Result<DescribeRedshiftIdcApplicationsOutput, SdkError<DescribeRedshiftIdcApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_node_exchange_status(&self, builder: DescribeReservedNodeExchangeStatusInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeExchangeStatusOutput, SdkError<DescribeReservedNodeExchangeStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_node_offerings(&self, builder: DescribeReservedNodeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeOfferingsOutput, SdkError<DescribeReservedNodeOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_nodes(&self, builder: DescribeReservedNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedNodesOutput, SdkError<DescribeReservedNodesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_resize(&self, builder: DescribeResizeInputBuilder) -> impl Future<Output = Result<DescribeResizeOutput, SdkError<DescribeResizeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_scheduled_actions(&self, builder: DescribeScheduledActionsInputBuilder) -> impl Future<Output = Result<DescribeScheduledActionsOutput, SdkError<DescribeScheduledActionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshot_copy_grants(&self, builder: DescribeSnapshotCopyGrantsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotCopyGrantsOutput, SdkError<DescribeSnapshotCopyGrantsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshot_schedules(&self, builder: DescribeSnapshotSchedulesInputBuilder) -> impl Future<Output = Result<DescribeSnapshotSchedulesOutput, SdkError<DescribeSnapshotSchedulesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_storage(&self, builder: DescribeStorageInputBuilder) -> impl Future<Output = Result<DescribeStorageOutput, SdkError<DescribeStorageError>>> {
        builder.send_with(&self.0)
    }
    fn describe_table_restore_status(&self, builder: DescribeTableRestoreStatusInputBuilder) -> impl Future<Output = Result<DescribeTableRestoreStatusOutput, SdkError<DescribeTableRestoreStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_usage_limits(&self, builder: DescribeUsageLimitsInputBuilder) -> impl Future<Output = Result<DescribeUsageLimitsOutput, SdkError<DescribeUsageLimitsError>>> {
        builder.send_with(&self.0)
    }
    fn disable_logging(&self, builder: DisableLoggingInputBuilder) -> impl Future<Output = Result<DisableLoggingOutput, SdkError<DisableLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn disable_snapshot_copy(&self, builder: DisableSnapshotCopyInputBuilder) -> impl Future<Output = Result<DisableSnapshotCopyOutput, SdkError<DisableSnapshotCopyError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_data_share_consumer(&self, builder: DisassociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<DisassociateDataShareConsumerOutput, SdkError<DisassociateDataShareConsumerError>>> {
        builder.send_with(&self.0)
    }
    fn enable_logging(&self, builder: EnableLoggingInputBuilder) -> impl Future<Output = Result<EnableLoggingOutput, SdkError<EnableLoggingError>>> {
        builder.send_with(&self.0)
    }
    fn enable_snapshot_copy(&self, builder: EnableSnapshotCopyInputBuilder) -> impl Future<Output = Result<EnableSnapshotCopyOutput, SdkError<EnableSnapshotCopyError>>> {
        builder.send_with(&self.0)
    }
    fn failover_primary_compute(&self, builder: FailoverPrimaryComputeInputBuilder) -> impl Future<Output = Result<FailoverPrimaryComputeOutput, SdkError<FailoverPrimaryComputeError>>> {
        builder.send_with(&self.0)
    }
    fn get_cluster_credentials(&self, builder: GetClusterCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsOutput, SdkError<GetClusterCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn get_cluster_credentials_with_iam(&self, builder: GetClusterCredentialsWithIamInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsWithIamOutput, SdkError<GetClusterCredentialsWithIAMError>>> {
        builder.send_with(&self.0)
    }
    fn get_reserved_node_exchange_configuration_options(&self, builder: GetReservedNodeExchangeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeConfigurationOptionsOutput, SdkError<GetReservedNodeExchangeConfigurationOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_reserved_node_exchange_offerings(&self, builder: GetReservedNodeExchangeOfferingsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeOfferingsOutput, SdkError<GetReservedNodeExchangeOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn list_recommendations(&self, builder: ListRecommendationsInputBuilder) -> impl Future<Output = Result<ListRecommendationsOutput, SdkError<ListRecommendationsError>>> {
        builder.send_with(&self.0)
    }
    fn modify_aqua_configuration(&self, builder: ModifyAquaConfigurationInputBuilder) -> impl Future<Output = Result<ModifyAquaConfigurationOutput, SdkError<ModifyAquaConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_authentication_profile(&self, builder: ModifyAuthenticationProfileInputBuilder) -> impl Future<Output = Result<ModifyAuthenticationProfileOutput, SdkError<ModifyAuthenticationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_db_revision(&self, builder: ModifyClusterDbRevisionInputBuilder) -> impl Future<Output = Result<ModifyClusterDbRevisionOutput, SdkError<ModifyClusterDbRevisionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_iam_roles(&self, builder: ModifyClusterIamRolesInputBuilder) -> impl Future<Output = Result<ModifyClusterIamRolesOutput, SdkError<ModifyClusterIamRolesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_maintenance(&self, builder: ModifyClusterMaintenanceInputBuilder) -> impl Future<Output = Result<ModifyClusterMaintenanceOutput, SdkError<ModifyClusterMaintenanceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_parameter_group(&self, builder: ModifyClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterParameterGroupOutput, SdkError<ModifyClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_snapshot(&self, builder: ModifyClusterSnapshotInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotOutput, SdkError<ModifyClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_snapshot_schedule(&self, builder: ModifyClusterSnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotScheduleOutput, SdkError<ModifyClusterSnapshotScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster_subnet_group(&self, builder: ModifyClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterSubnetGroupOutput, SdkError<ModifyClusterSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_custom_domain_association(&self, builder: ModifyCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<ModifyCustomDomainAssociationOutput, SdkError<ModifyCustomDomainAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_endpoint_access(&self, builder: ModifyEndpointAccessInputBuilder) -> impl Future<Output = Result<ModifyEndpointAccessOutput, SdkError<ModifyEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_redshift_idc_application(&self, builder: ModifyRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<ModifyRedshiftIdcApplicationOutput, SdkError<ModifyRedshiftIdcApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_scheduled_action(&self, builder: ModifyScheduledActionInputBuilder) -> impl Future<Output = Result<ModifyScheduledActionOutput, SdkError<ModifyScheduledActionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_snapshot_copy_retention_period(&self, builder: ModifySnapshotCopyRetentionPeriodInputBuilder) -> impl Future<Output = Result<ModifySnapshotCopyRetentionPeriodOutput, SdkError<ModifySnapshotCopyRetentionPeriodError>>> {
        builder.send_with(&self.0)
    }
    fn modify_snapshot_schedule(&self, builder: ModifySnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifySnapshotScheduleOutput, SdkError<ModifySnapshotScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn modify_usage_limit(&self, builder: ModifyUsageLimitInputBuilder) -> impl Future<Output = Result<ModifyUsageLimitOutput, SdkError<ModifyUsageLimitError>>> {
        builder.send_with(&self.0)
    }
    fn pause_cluster(&self, builder: PauseClusterInputBuilder) -> impl Future<Output = Result<PauseClusterOutput, SdkError<PauseClusterError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_reserved_node_offering(&self, builder: PurchaseReservedNodeOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedNodeOfferingOutput, SdkError<PurchaseReservedNodeOfferingError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_cluster(&self, builder: RebootClusterInputBuilder) -> impl Future<Output = Result<RebootClusterOutput, SdkError<RebootClusterError>>> {
        builder.send_with(&self.0)
    }
    fn reject_data_share(&self, builder: RejectDataShareInputBuilder) -> impl Future<Output = Result<RejectDataShareOutput, SdkError<RejectDataShareError>>> {
        builder.send_with(&self.0)
    }
    fn reset_cluster_parameter_group(&self, builder: ResetClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetClusterParameterGroupOutput, SdkError<ResetClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn resize_cluster(&self, builder: ResizeClusterInputBuilder) -> impl Future<Output = Result<ResizeClusterOutput, SdkError<ResizeClusterError>>> {
        builder.send_with(&self.0)
    }
    fn restore_from_cluster_snapshot(&self, builder: RestoreFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreFromClusterSnapshotOutput, SdkError<RestoreFromClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn restore_table_from_cluster_snapshot(&self, builder: RestoreTableFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreTableFromClusterSnapshotOutput, SdkError<RestoreTableFromClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn resume_cluster(&self, builder: ResumeClusterInputBuilder) -> impl Future<Output = Result<ResumeClusterOutput, SdkError<ResumeClusterError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_cluster_security_group_ingress(&self, builder: RevokeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeClusterSecurityGroupIngressOutput, SdkError<RevokeClusterSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_endpoint_access(&self, builder: RevokeEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeEndpointAccessOutput, SdkError<RevokeEndpointAccessError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_snapshot_access(&self, builder: RevokeSnapshotAccessInputBuilder) -> impl Future<Output = Result<RevokeSnapshotAccessOutput, SdkError<RevokeSnapshotAccessError>>> {
        builder.send_with(&self.0)
    }
    fn rotate_encryption_key(&self, builder: RotateEncryptionKeyInputBuilder) -> impl Future<Output = Result<RotateEncryptionKeyOutput, SdkError<RotateEncryptionKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_partner_status(&self, builder: UpdatePartnerStatusInputBuilder) -> impl Future<Output = Result<UpdatePartnerStatusOutput, SdkError<UpdatePartnerStatusError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> RedshiftClient for T
where T: Deref,
      T::Target: RedshiftClient {
    fn accept_reserved_node_exchange(&self, builder: AcceptReservedNodeExchangeInputBuilder) -> impl Future<Output = Result<AcceptReservedNodeExchangeOutput, SdkError<AcceptReservedNodeExchangeError>>> {
        self.deref().accept_reserved_node_exchange(builder)
    }
    fn add_partner(&self, builder: AddPartnerInputBuilder) -> impl Future<Output = Result<AddPartnerOutput, SdkError<AddPartnerError>>> {
        self.deref().add_partner(builder)
    }
    fn associate_data_share_consumer(&self, builder: AssociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<AssociateDataShareConsumerOutput, SdkError<AssociateDataShareConsumerError>>> {
        self.deref().associate_data_share_consumer(builder)
    }
    fn authorize_cluster_security_group_ingress(&self, builder: AuthorizeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeClusterSecurityGroupIngressOutput, SdkError<AuthorizeClusterSecurityGroupIngressError>>> {
        self.deref().authorize_cluster_security_group_ingress(builder)
    }
    fn authorize_data_share(&self, builder: AuthorizeDataShareInputBuilder) -> impl Future<Output = Result<AuthorizeDataShareOutput, SdkError<AuthorizeDataShareError>>> {
        self.deref().authorize_data_share(builder)
    }
    fn authorize_endpoint_access(&self, builder: AuthorizeEndpointAccessInputBuilder) -> impl Future<Output = Result<AuthorizeEndpointAccessOutput, SdkError<AuthorizeEndpointAccessError>>> {
        self.deref().authorize_endpoint_access(builder)
    }
    fn authorize_snapshot_access(&self, builder: AuthorizeSnapshotAccessInputBuilder) -> impl Future<Output = Result<AuthorizeSnapshotAccessOutput, SdkError<AuthorizeSnapshotAccessError>>> {
        self.deref().authorize_snapshot_access(builder)
    }
    fn batch_delete_cluster_snapshots(&self, builder: BatchDeleteClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchDeleteClusterSnapshotsOutput, SdkError<BatchDeleteClusterSnapshotsError>>> {
        self.deref().batch_delete_cluster_snapshots(builder)
    }
    fn batch_modify_cluster_snapshots(&self, builder: BatchModifyClusterSnapshotsInputBuilder) -> impl Future<Output = Result<BatchModifyClusterSnapshotsOutput, SdkError<BatchModifyClusterSnapshotsError>>> {
        self.deref().batch_modify_cluster_snapshots(builder)
    }
    fn cancel_resize(&self, builder: CancelResizeInputBuilder) -> impl Future<Output = Result<CancelResizeOutput, SdkError<CancelResizeError>>> {
        self.deref().cancel_resize(builder)
    }
    fn copy_cluster_snapshot(&self, builder: CopyClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyClusterSnapshotOutput, SdkError<CopyClusterSnapshotError>>> {
        self.deref().copy_cluster_snapshot(builder)
    }
    fn create_authentication_profile(&self, builder: CreateAuthenticationProfileInputBuilder) -> impl Future<Output = Result<CreateAuthenticationProfileOutput, SdkError<CreateAuthenticationProfileError>>> {
        self.deref().create_authentication_profile(builder)
    }
    fn create_cluster(&self, builder: CreateClusterInputBuilder) -> impl Future<Output = Result<CreateClusterOutput, SdkError<CreateClusterError>>> {
        self.deref().create_cluster(builder)
    }
    fn create_cluster_parameter_group(&self, builder: CreateClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateClusterParameterGroupOutput, SdkError<CreateClusterParameterGroupError>>> {
        self.deref().create_cluster_parameter_group(builder)
    }
    fn create_cluster_security_group(&self, builder: CreateClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSecurityGroupOutput, SdkError<CreateClusterSecurityGroupError>>> {
        self.deref().create_cluster_security_group(builder)
    }
    fn create_cluster_snapshot(&self, builder: CreateClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateClusterSnapshotOutput, SdkError<CreateClusterSnapshotError>>> {
        self.deref().create_cluster_snapshot(builder)
    }
    fn create_cluster_subnet_group(&self, builder: CreateClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateClusterSubnetGroupOutput, SdkError<CreateClusterSubnetGroupError>>> {
        self.deref().create_cluster_subnet_group(builder)
    }
    fn create_custom_domain_association(&self, builder: CreateCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<CreateCustomDomainAssociationOutput, SdkError<CreateCustomDomainAssociationError>>> {
        self.deref().create_custom_domain_association(builder)
    }
    fn create_endpoint_access(&self, builder: CreateEndpointAccessInputBuilder) -> impl Future<Output = Result<CreateEndpointAccessOutput, SdkError<CreateEndpointAccessError>>> {
        self.deref().create_endpoint_access(builder)
    }
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>> {
        self.deref().create_event_subscription(builder)
    }
    fn create_hsm_client_certificate(&self, builder: CreateHsmClientCertificateInputBuilder) -> impl Future<Output = Result<CreateHsmClientCertificateOutput, SdkError<CreateHsmClientCertificateError>>> {
        self.deref().create_hsm_client_certificate(builder)
    }
    fn create_hsm_configuration(&self, builder: CreateHsmConfigurationInputBuilder) -> impl Future<Output = Result<CreateHsmConfigurationOutput, SdkError<CreateHsmConfigurationError>>> {
        self.deref().create_hsm_configuration(builder)
    }
    fn create_redshift_idc_application(&self, builder: CreateRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<CreateRedshiftIdcApplicationOutput, SdkError<CreateRedshiftIdcApplicationError>>> {
        self.deref().create_redshift_idc_application(builder)
    }
    fn create_scheduled_action(&self, builder: CreateScheduledActionInputBuilder) -> impl Future<Output = Result<CreateScheduledActionOutput, SdkError<CreateScheduledActionError>>> {
        self.deref().create_scheduled_action(builder)
    }
    fn create_snapshot_copy_grant(&self, builder: CreateSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<CreateSnapshotCopyGrantOutput, SdkError<CreateSnapshotCopyGrantError>>> {
        self.deref().create_snapshot_copy_grant(builder)
    }
    fn create_snapshot_schedule(&self, builder: CreateSnapshotScheduleInputBuilder) -> impl Future<Output = Result<CreateSnapshotScheduleOutput, SdkError<CreateSnapshotScheduleError>>> {
        self.deref().create_snapshot_schedule(builder)
    }
    fn create_tags(&self, builder: CreateTagsInputBuilder) -> impl Future<Output = Result<CreateTagsOutput, SdkError<CreateTagsError>>> {
        self.deref().create_tags(builder)
    }
    fn create_usage_limit(&self, builder: CreateUsageLimitInputBuilder) -> impl Future<Output = Result<CreateUsageLimitOutput, SdkError<CreateUsageLimitError>>> {
        self.deref().create_usage_limit(builder)
    }
    fn deauthorize_data_share(&self, builder: DeauthorizeDataShareInputBuilder) -> impl Future<Output = Result<DeauthorizeDataShareOutput, SdkError<DeauthorizeDataShareError>>> {
        self.deref().deauthorize_data_share(builder)
    }
    fn delete_authentication_profile(&self, builder: DeleteAuthenticationProfileInputBuilder) -> impl Future<Output = Result<DeleteAuthenticationProfileOutput, SdkError<DeleteAuthenticationProfileError>>> {
        self.deref().delete_authentication_profile(builder)
    }
    fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> impl Future<Output = Result<DeleteClusterOutput, SdkError<DeleteClusterError>>> {
        self.deref().delete_cluster(builder)
    }
    fn delete_cluster_parameter_group(&self, builder: DeleteClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterParameterGroupOutput, SdkError<DeleteClusterParameterGroupError>>> {
        self.deref().delete_cluster_parameter_group(builder)
    }
    fn delete_cluster_security_group(&self, builder: DeleteClusterSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSecurityGroupOutput, SdkError<DeleteClusterSecurityGroupError>>> {
        self.deref().delete_cluster_security_group(builder)
    }
    fn delete_cluster_snapshot(&self, builder: DeleteClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteClusterSnapshotOutput, SdkError<DeleteClusterSnapshotError>>> {
        self.deref().delete_cluster_snapshot(builder)
    }
    fn delete_cluster_subnet_group(&self, builder: DeleteClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteClusterSubnetGroupOutput, SdkError<DeleteClusterSubnetGroupError>>> {
        self.deref().delete_cluster_subnet_group(builder)
    }
    fn delete_custom_domain_association(&self, builder: DeleteCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<DeleteCustomDomainAssociationOutput, SdkError<DeleteCustomDomainAssociationError>>> {
        self.deref().delete_custom_domain_association(builder)
    }
    fn delete_endpoint_access(&self, builder: DeleteEndpointAccessInputBuilder) -> impl Future<Output = Result<DeleteEndpointAccessOutput, SdkError<DeleteEndpointAccessError>>> {
        self.deref().delete_endpoint_access(builder)
    }
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>> {
        self.deref().delete_event_subscription(builder)
    }
    fn delete_hsm_client_certificate(&self, builder: DeleteHsmClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteHsmClientCertificateOutput, SdkError<DeleteHsmClientCertificateError>>> {
        self.deref().delete_hsm_client_certificate(builder)
    }
    fn delete_hsm_configuration(&self, builder: DeleteHsmConfigurationInputBuilder) -> impl Future<Output = Result<DeleteHsmConfigurationOutput, SdkError<DeleteHsmConfigurationError>>> {
        self.deref().delete_hsm_configuration(builder)
    }
    fn delete_partner(&self, builder: DeletePartnerInputBuilder) -> impl Future<Output = Result<DeletePartnerOutput, SdkError<DeletePartnerError>>> {
        self.deref().delete_partner(builder)
    }
    fn delete_redshift_idc_application(&self, builder: DeleteRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<DeleteRedshiftIdcApplicationOutput, SdkError<DeleteRedshiftIdcApplicationError>>> {
        self.deref().delete_redshift_idc_application(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        self.deref().delete_resource_policy(builder)
    }
    fn delete_scheduled_action(&self, builder: DeleteScheduledActionInputBuilder) -> impl Future<Output = Result<DeleteScheduledActionOutput, SdkError<DeleteScheduledActionError>>> {
        self.deref().delete_scheduled_action(builder)
    }
    fn delete_snapshot_copy_grant(&self, builder: DeleteSnapshotCopyGrantInputBuilder) -> impl Future<Output = Result<DeleteSnapshotCopyGrantOutput, SdkError<DeleteSnapshotCopyGrantError>>> {
        self.deref().delete_snapshot_copy_grant(builder)
    }
    fn delete_snapshot_schedule(&self, builder: DeleteSnapshotScheduleInputBuilder) -> impl Future<Output = Result<DeleteSnapshotScheduleOutput, SdkError<DeleteSnapshotScheduleError>>> {
        self.deref().delete_snapshot_schedule(builder)
    }
    fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> impl Future<Output = Result<DeleteTagsOutput, SdkError<DeleteTagsError>>> {
        self.deref().delete_tags(builder)
    }
    fn delete_usage_limit(&self, builder: DeleteUsageLimitInputBuilder) -> impl Future<Output = Result<DeleteUsageLimitOutput, SdkError<DeleteUsageLimitError>>> {
        self.deref().delete_usage_limit(builder)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        self.deref().describe_account_attributes(builder)
    }
    fn describe_authentication_profiles(&self, builder: DescribeAuthenticationProfilesInputBuilder) -> impl Future<Output = Result<DescribeAuthenticationProfilesOutput, SdkError<DescribeAuthenticationProfilesError>>> {
        self.deref().describe_authentication_profiles(builder)
    }
    fn describe_cluster_db_revisions(&self, builder: DescribeClusterDbRevisionsInputBuilder) -> impl Future<Output = Result<DescribeClusterDbRevisionsOutput, SdkError<DescribeClusterDbRevisionsError>>> {
        self.deref().describe_cluster_db_revisions(builder)
    }
    fn describe_cluster_parameter_groups(&self, builder: DescribeClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterParameterGroupsOutput, SdkError<DescribeClusterParameterGroupsError>>> {
        self.deref().describe_cluster_parameter_groups(builder)
    }
    fn describe_cluster_parameters(&self, builder: DescribeClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeClusterParametersOutput, SdkError<DescribeClusterParametersError>>> {
        self.deref().describe_cluster_parameters(builder)
    }
    fn describe_cluster_security_groups(&self, builder: DescribeClusterSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSecurityGroupsOutput, SdkError<DescribeClusterSecurityGroupsError>>> {
        self.deref().describe_cluster_security_groups(builder)
    }
    fn describe_cluster_snapshots(&self, builder: DescribeClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeClusterSnapshotsOutput, SdkError<DescribeClusterSnapshotsError>>> {
        self.deref().describe_cluster_snapshots(builder)
    }
    fn describe_cluster_subnet_groups(&self, builder: DescribeClusterSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeClusterSubnetGroupsOutput, SdkError<DescribeClusterSubnetGroupsError>>> {
        self.deref().describe_cluster_subnet_groups(builder)
    }
    fn describe_cluster_tracks(&self, builder: DescribeClusterTracksInputBuilder) -> impl Future<Output = Result<DescribeClusterTracksOutput, SdkError<DescribeClusterTracksError>>> {
        self.deref().describe_cluster_tracks(builder)
    }
    fn describe_cluster_versions(&self, builder: DescribeClusterVersionsInputBuilder) -> impl Future<Output = Result<DescribeClusterVersionsOutput, SdkError<DescribeClusterVersionsError>>> {
        self.deref().describe_cluster_versions(builder)
    }
    fn describe_clusters(&self, builder: DescribeClustersInputBuilder) -> impl Future<Output = Result<DescribeClustersOutput, SdkError<DescribeClustersError>>> {
        self.deref().describe_clusters(builder)
    }
    fn describe_custom_domain_associations(&self, builder: DescribeCustomDomainAssociationsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainAssociationsOutput, SdkError<DescribeCustomDomainAssociationsError>>> {
        self.deref().describe_custom_domain_associations(builder)
    }
    fn describe_data_shares(&self, builder: DescribeDataSharesInputBuilder) -> impl Future<Output = Result<DescribeDataSharesOutput, SdkError<DescribeDataSharesError>>> {
        self.deref().describe_data_shares(builder)
    }
    fn describe_data_shares_for_consumer(&self, builder: DescribeDataSharesForConsumerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForConsumerOutput, SdkError<DescribeDataSharesForConsumerError>>> {
        self.deref().describe_data_shares_for_consumer(builder)
    }
    fn describe_data_shares_for_producer(&self, builder: DescribeDataSharesForProducerInputBuilder) -> impl Future<Output = Result<DescribeDataSharesForProducerOutput, SdkError<DescribeDataSharesForProducerError>>> {
        self.deref().describe_data_shares_for_producer(builder)
    }
    fn describe_default_cluster_parameters(&self, builder: DescribeDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDefaultClusterParametersOutput, SdkError<DescribeDefaultClusterParametersError>>> {
        self.deref().describe_default_cluster_parameters(builder)
    }
    fn describe_endpoint_access(&self, builder: DescribeEndpointAccessInputBuilder) -> impl Future<Output = Result<DescribeEndpointAccessOutput, SdkError<DescribeEndpointAccessError>>> {
        self.deref().describe_endpoint_access(builder)
    }
    fn describe_endpoint_authorization(&self, builder: DescribeEndpointAuthorizationInputBuilder) -> impl Future<Output = Result<DescribeEndpointAuthorizationOutput, SdkError<DescribeEndpointAuthorizationError>>> {
        self.deref().describe_endpoint_authorization(builder)
    }
    fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> impl Future<Output = Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>> {
        self.deref().describe_event_categories(builder)
    }
    fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>> {
        self.deref().describe_event_subscriptions(builder)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        self.deref().describe_events(builder)
    }
    fn describe_hsm_client_certificates(&self, builder: DescribeHsmClientCertificatesInputBuilder) -> impl Future<Output = Result<DescribeHsmClientCertificatesOutput, SdkError<DescribeHsmClientCertificatesError>>> {
        self.deref().describe_hsm_client_certificates(builder)
    }
    fn describe_hsm_configurations(&self, builder: DescribeHsmConfigurationsInputBuilder) -> impl Future<Output = Result<DescribeHsmConfigurationsOutput, SdkError<DescribeHsmConfigurationsError>>> {
        self.deref().describe_hsm_configurations(builder)
    }
    fn describe_inbound_integrations(&self, builder: DescribeInboundIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeInboundIntegrationsOutput, SdkError<DescribeInboundIntegrationsError>>> {
        self.deref().describe_inbound_integrations(builder)
    }
    fn describe_logging_status(&self, builder: DescribeLoggingStatusInputBuilder) -> impl Future<Output = Result<DescribeLoggingStatusOutput, SdkError<DescribeLoggingStatusError>>> {
        self.deref().describe_logging_status(builder)
    }
    fn describe_node_configuration_options(&self, builder: DescribeNodeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeNodeConfigurationOptionsOutput, SdkError<DescribeNodeConfigurationOptionsError>>> {
        self.deref().describe_node_configuration_options(builder)
    }
    fn describe_orderable_cluster_options(&self, builder: DescribeOrderableClusterOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableClusterOptionsOutput, SdkError<DescribeOrderableClusterOptionsError>>> {
        self.deref().describe_orderable_cluster_options(builder)
    }
    fn describe_partners(&self, builder: DescribePartnersInputBuilder) -> impl Future<Output = Result<DescribePartnersOutput, SdkError<DescribePartnersError>>> {
        self.deref().describe_partners(builder)
    }
    fn describe_redshift_idc_applications(&self, builder: DescribeRedshiftIdcApplicationsInputBuilder) -> impl Future<Output = Result<DescribeRedshiftIdcApplicationsOutput, SdkError<DescribeRedshiftIdcApplicationsError>>> {
        self.deref().describe_redshift_idc_applications(builder)
    }
    fn describe_reserved_node_exchange_status(&self, builder: DescribeReservedNodeExchangeStatusInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeExchangeStatusOutput, SdkError<DescribeReservedNodeExchangeStatusError>>> {
        self.deref().describe_reserved_node_exchange_status(builder)
    }
    fn describe_reserved_node_offerings(&self, builder: DescribeReservedNodeOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedNodeOfferingsOutput, SdkError<DescribeReservedNodeOfferingsError>>> {
        self.deref().describe_reserved_node_offerings(builder)
    }
    fn describe_reserved_nodes(&self, builder: DescribeReservedNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedNodesOutput, SdkError<DescribeReservedNodesError>>> {
        self.deref().describe_reserved_nodes(builder)
    }
    fn describe_resize(&self, builder: DescribeResizeInputBuilder) -> impl Future<Output = Result<DescribeResizeOutput, SdkError<DescribeResizeError>>> {
        self.deref().describe_resize(builder)
    }
    fn describe_scheduled_actions(&self, builder: DescribeScheduledActionsInputBuilder) -> impl Future<Output = Result<DescribeScheduledActionsOutput, SdkError<DescribeScheduledActionsError>>> {
        self.deref().describe_scheduled_actions(builder)
    }
    fn describe_snapshot_copy_grants(&self, builder: DescribeSnapshotCopyGrantsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotCopyGrantsOutput, SdkError<DescribeSnapshotCopyGrantsError>>> {
        self.deref().describe_snapshot_copy_grants(builder)
    }
    fn describe_snapshot_schedules(&self, builder: DescribeSnapshotSchedulesInputBuilder) -> impl Future<Output = Result<DescribeSnapshotSchedulesOutput, SdkError<DescribeSnapshotSchedulesError>>> {
        self.deref().describe_snapshot_schedules(builder)
    }
    fn describe_storage(&self, builder: DescribeStorageInputBuilder) -> impl Future<Output = Result<DescribeStorageOutput, SdkError<DescribeStorageError>>> {
        self.deref().describe_storage(builder)
    }
    fn describe_table_restore_status(&self, builder: DescribeTableRestoreStatusInputBuilder) -> impl Future<Output = Result<DescribeTableRestoreStatusOutput, SdkError<DescribeTableRestoreStatusError>>> {
        self.deref().describe_table_restore_status(builder)
    }
    fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> impl Future<Output = Result<DescribeTagsOutput, SdkError<DescribeTagsError>>> {
        self.deref().describe_tags(builder)
    }
    fn describe_usage_limits(&self, builder: DescribeUsageLimitsInputBuilder) -> impl Future<Output = Result<DescribeUsageLimitsOutput, SdkError<DescribeUsageLimitsError>>> {
        self.deref().describe_usage_limits(builder)
    }
    fn disable_logging(&self, builder: DisableLoggingInputBuilder) -> impl Future<Output = Result<DisableLoggingOutput, SdkError<DisableLoggingError>>> {
        self.deref().disable_logging(builder)
    }
    fn disable_snapshot_copy(&self, builder: DisableSnapshotCopyInputBuilder) -> impl Future<Output = Result<DisableSnapshotCopyOutput, SdkError<DisableSnapshotCopyError>>> {
        self.deref().disable_snapshot_copy(builder)
    }
    fn disassociate_data_share_consumer(&self, builder: DisassociateDataShareConsumerInputBuilder) -> impl Future<Output = Result<DisassociateDataShareConsumerOutput, SdkError<DisassociateDataShareConsumerError>>> {
        self.deref().disassociate_data_share_consumer(builder)
    }
    fn enable_logging(&self, builder: EnableLoggingInputBuilder) -> impl Future<Output = Result<EnableLoggingOutput, SdkError<EnableLoggingError>>> {
        self.deref().enable_logging(builder)
    }
    fn enable_snapshot_copy(&self, builder: EnableSnapshotCopyInputBuilder) -> impl Future<Output = Result<EnableSnapshotCopyOutput, SdkError<EnableSnapshotCopyError>>> {
        self.deref().enable_snapshot_copy(builder)
    }
    fn failover_primary_compute(&self, builder: FailoverPrimaryComputeInputBuilder) -> impl Future<Output = Result<FailoverPrimaryComputeOutput, SdkError<FailoverPrimaryComputeError>>> {
        self.deref().failover_primary_compute(builder)
    }
    fn get_cluster_credentials(&self, builder: GetClusterCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsOutput, SdkError<GetClusterCredentialsError>>> {
        self.deref().get_cluster_credentials(builder)
    }
    fn get_cluster_credentials_with_iam(&self, builder: GetClusterCredentialsWithIamInputBuilder) -> impl Future<Output = Result<GetClusterCredentialsWithIamOutput, SdkError<GetClusterCredentialsWithIAMError>>> {
        self.deref().get_cluster_credentials_with_iam(builder)
    }
    fn get_reserved_node_exchange_configuration_options(&self, builder: GetReservedNodeExchangeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeConfigurationOptionsOutput, SdkError<GetReservedNodeExchangeConfigurationOptionsError>>> {
        self.deref().get_reserved_node_exchange_configuration_options(builder)
    }
    fn get_reserved_node_exchange_offerings(&self, builder: GetReservedNodeExchangeOfferingsInputBuilder) -> impl Future<Output = Result<GetReservedNodeExchangeOfferingsOutput, SdkError<GetReservedNodeExchangeOfferingsError>>> {
        self.deref().get_reserved_node_exchange_offerings(builder)
    }
    fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> impl Future<Output = Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>> {
        self.deref().get_resource_policy(builder)
    }
    fn list_recommendations(&self, builder: ListRecommendationsInputBuilder) -> impl Future<Output = Result<ListRecommendationsOutput, SdkError<ListRecommendationsError>>> {
        self.deref().list_recommendations(builder)
    }
    fn modify_aqua_configuration(&self, builder: ModifyAquaConfigurationInputBuilder) -> impl Future<Output = Result<ModifyAquaConfigurationOutput, SdkError<ModifyAquaConfigurationError>>> {
        self.deref().modify_aqua_configuration(builder)
    }
    fn modify_authentication_profile(&self, builder: ModifyAuthenticationProfileInputBuilder) -> impl Future<Output = Result<ModifyAuthenticationProfileOutput, SdkError<ModifyAuthenticationProfileError>>> {
        self.deref().modify_authentication_profile(builder)
    }
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>> {
        self.deref().modify_cluster(builder)
    }
    fn modify_cluster_db_revision(&self, builder: ModifyClusterDbRevisionInputBuilder) -> impl Future<Output = Result<ModifyClusterDbRevisionOutput, SdkError<ModifyClusterDbRevisionError>>> {
        self.deref().modify_cluster_db_revision(builder)
    }
    fn modify_cluster_iam_roles(&self, builder: ModifyClusterIamRolesInputBuilder) -> impl Future<Output = Result<ModifyClusterIamRolesOutput, SdkError<ModifyClusterIamRolesError>>> {
        self.deref().modify_cluster_iam_roles(builder)
    }
    fn modify_cluster_maintenance(&self, builder: ModifyClusterMaintenanceInputBuilder) -> impl Future<Output = Result<ModifyClusterMaintenanceOutput, SdkError<ModifyClusterMaintenanceError>>> {
        self.deref().modify_cluster_maintenance(builder)
    }
    fn modify_cluster_parameter_group(&self, builder: ModifyClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterParameterGroupOutput, SdkError<ModifyClusterParameterGroupError>>> {
        self.deref().modify_cluster_parameter_group(builder)
    }
    fn modify_cluster_snapshot(&self, builder: ModifyClusterSnapshotInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotOutput, SdkError<ModifyClusterSnapshotError>>> {
        self.deref().modify_cluster_snapshot(builder)
    }
    fn modify_cluster_snapshot_schedule(&self, builder: ModifyClusterSnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifyClusterSnapshotScheduleOutput, SdkError<ModifyClusterSnapshotScheduleError>>> {
        self.deref().modify_cluster_snapshot_schedule(builder)
    }
    fn modify_cluster_subnet_group(&self, builder: ModifyClusterSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyClusterSubnetGroupOutput, SdkError<ModifyClusterSubnetGroupError>>> {
        self.deref().modify_cluster_subnet_group(builder)
    }
    fn modify_custom_domain_association(&self, builder: ModifyCustomDomainAssociationInputBuilder) -> impl Future<Output = Result<ModifyCustomDomainAssociationOutput, SdkError<ModifyCustomDomainAssociationError>>> {
        self.deref().modify_custom_domain_association(builder)
    }
    fn modify_endpoint_access(&self, builder: ModifyEndpointAccessInputBuilder) -> impl Future<Output = Result<ModifyEndpointAccessOutput, SdkError<ModifyEndpointAccessError>>> {
        self.deref().modify_endpoint_access(builder)
    }
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>> {
        self.deref().modify_event_subscription(builder)
    }
    fn modify_redshift_idc_application(&self, builder: ModifyRedshiftIdcApplicationInputBuilder) -> impl Future<Output = Result<ModifyRedshiftIdcApplicationOutput, SdkError<ModifyRedshiftIdcApplicationError>>> {
        self.deref().modify_redshift_idc_application(builder)
    }
    fn modify_scheduled_action(&self, builder: ModifyScheduledActionInputBuilder) -> impl Future<Output = Result<ModifyScheduledActionOutput, SdkError<ModifyScheduledActionError>>> {
        self.deref().modify_scheduled_action(builder)
    }
    fn modify_snapshot_copy_retention_period(&self, builder: ModifySnapshotCopyRetentionPeriodInputBuilder) -> impl Future<Output = Result<ModifySnapshotCopyRetentionPeriodOutput, SdkError<ModifySnapshotCopyRetentionPeriodError>>> {
        self.deref().modify_snapshot_copy_retention_period(builder)
    }
    fn modify_snapshot_schedule(&self, builder: ModifySnapshotScheduleInputBuilder) -> impl Future<Output = Result<ModifySnapshotScheduleOutput, SdkError<ModifySnapshotScheduleError>>> {
        self.deref().modify_snapshot_schedule(builder)
    }
    fn modify_usage_limit(&self, builder: ModifyUsageLimitInputBuilder) -> impl Future<Output = Result<ModifyUsageLimitOutput, SdkError<ModifyUsageLimitError>>> {
        self.deref().modify_usage_limit(builder)
    }
    fn pause_cluster(&self, builder: PauseClusterInputBuilder) -> impl Future<Output = Result<PauseClusterOutput, SdkError<PauseClusterError>>> {
        self.deref().pause_cluster(builder)
    }
    fn purchase_reserved_node_offering(&self, builder: PurchaseReservedNodeOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedNodeOfferingOutput, SdkError<PurchaseReservedNodeOfferingError>>> {
        self.deref().purchase_reserved_node_offering(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        self.deref().put_resource_policy(builder)
    }
    fn reboot_cluster(&self, builder: RebootClusterInputBuilder) -> impl Future<Output = Result<RebootClusterOutput, SdkError<RebootClusterError>>> {
        self.deref().reboot_cluster(builder)
    }
    fn reject_data_share(&self, builder: RejectDataShareInputBuilder) -> impl Future<Output = Result<RejectDataShareOutput, SdkError<RejectDataShareError>>> {
        self.deref().reject_data_share(builder)
    }
    fn reset_cluster_parameter_group(&self, builder: ResetClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetClusterParameterGroupOutput, SdkError<ResetClusterParameterGroupError>>> {
        self.deref().reset_cluster_parameter_group(builder)
    }
    fn resize_cluster(&self, builder: ResizeClusterInputBuilder) -> impl Future<Output = Result<ResizeClusterOutput, SdkError<ResizeClusterError>>> {
        self.deref().resize_cluster(builder)
    }
    fn restore_from_cluster_snapshot(&self, builder: RestoreFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreFromClusterSnapshotOutput, SdkError<RestoreFromClusterSnapshotError>>> {
        self.deref().restore_from_cluster_snapshot(builder)
    }
    fn restore_table_from_cluster_snapshot(&self, builder: RestoreTableFromClusterSnapshotInputBuilder) -> impl Future<Output = Result<RestoreTableFromClusterSnapshotOutput, SdkError<RestoreTableFromClusterSnapshotError>>> {
        self.deref().restore_table_from_cluster_snapshot(builder)
    }
    fn resume_cluster(&self, builder: ResumeClusterInputBuilder) -> impl Future<Output = Result<ResumeClusterOutput, SdkError<ResumeClusterError>>> {
        self.deref().resume_cluster(builder)
    }
    fn revoke_cluster_security_group_ingress(&self, builder: RevokeClusterSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeClusterSecurityGroupIngressOutput, SdkError<RevokeClusterSecurityGroupIngressError>>> {
        self.deref().revoke_cluster_security_group_ingress(builder)
    }
    fn revoke_endpoint_access(&self, builder: RevokeEndpointAccessInputBuilder) -> impl Future<Output = Result<RevokeEndpointAccessOutput, SdkError<RevokeEndpointAccessError>>> {
        self.deref().revoke_endpoint_access(builder)
    }
    fn revoke_snapshot_access(&self, builder: RevokeSnapshotAccessInputBuilder) -> impl Future<Output = Result<RevokeSnapshotAccessOutput, SdkError<RevokeSnapshotAccessError>>> {
        self.deref().revoke_snapshot_access(builder)
    }
    fn rotate_encryption_key(&self, builder: RotateEncryptionKeyInputBuilder) -> impl Future<Output = Result<RotateEncryptionKeyOutput, SdkError<RotateEncryptionKeyError>>> {
        self.deref().rotate_encryption_key(builder)
    }
    fn update_partner_status(&self, builder: UpdatePartnerStatusInputBuilder) -> impl Future<Output = Result<UpdatePartnerStatusOutput, SdkError<UpdatePartnerStatusError>>> {
        self.deref().update_partner_status(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edRedshiftClient {}
    impl RedshiftClient for edRedshiftClient {
        async fn accept_reserved_node_exchange(&self, builder: AcceptReservedNodeExchangeInputBuilder) -> Result<AcceptReservedNodeExchangeOutput, SdkError<AcceptReservedNodeExchangeError>>;
        async fn add_partner(&self, builder: AddPartnerInputBuilder) -> Result<AddPartnerOutput, SdkError<AddPartnerError>>;
        async fn associate_data_share_consumer(&self, builder: AssociateDataShareConsumerInputBuilder) -> Result<AssociateDataShareConsumerOutput, SdkError<AssociateDataShareConsumerError>>;
        async fn authorize_cluster_security_group_ingress(&self, builder: AuthorizeClusterSecurityGroupIngressInputBuilder) -> Result<AuthorizeClusterSecurityGroupIngressOutput, SdkError<AuthorizeClusterSecurityGroupIngressError>>;
        async fn authorize_data_share(&self, builder: AuthorizeDataShareInputBuilder) -> Result<AuthorizeDataShareOutput, SdkError<AuthorizeDataShareError>>;
        async fn authorize_endpoint_access(&self, builder: AuthorizeEndpointAccessInputBuilder) -> Result<AuthorizeEndpointAccessOutput, SdkError<AuthorizeEndpointAccessError>>;
        async fn authorize_snapshot_access(&self, builder: AuthorizeSnapshotAccessInputBuilder) -> Result<AuthorizeSnapshotAccessOutput, SdkError<AuthorizeSnapshotAccessError>>;
        async fn batch_delete_cluster_snapshots(&self, builder: BatchDeleteClusterSnapshotsInputBuilder) -> Result<BatchDeleteClusterSnapshotsOutput, SdkError<BatchDeleteClusterSnapshotsError>>;
        async fn batch_modify_cluster_snapshots(&self, builder: BatchModifyClusterSnapshotsInputBuilder) -> Result<BatchModifyClusterSnapshotsOutput, SdkError<BatchModifyClusterSnapshotsError>>;
        async fn cancel_resize(&self, builder: CancelResizeInputBuilder) -> Result<CancelResizeOutput, SdkError<CancelResizeError>>;
        async fn copy_cluster_snapshot(&self, builder: CopyClusterSnapshotInputBuilder) -> Result<CopyClusterSnapshotOutput, SdkError<CopyClusterSnapshotError>>;
        async fn create_authentication_profile(&self, builder: CreateAuthenticationProfileInputBuilder) -> Result<CreateAuthenticationProfileOutput, SdkError<CreateAuthenticationProfileError>>;
        async fn create_cluster(&self, builder: CreateClusterInputBuilder) -> Result<CreateClusterOutput, SdkError<CreateClusterError>>;
        async fn create_cluster_parameter_group(&self, builder: CreateClusterParameterGroupInputBuilder) -> Result<CreateClusterParameterGroupOutput, SdkError<CreateClusterParameterGroupError>>;
        async fn create_cluster_security_group(&self, builder: CreateClusterSecurityGroupInputBuilder) -> Result<CreateClusterSecurityGroupOutput, SdkError<CreateClusterSecurityGroupError>>;
        async fn create_cluster_snapshot(&self, builder: CreateClusterSnapshotInputBuilder) -> Result<CreateClusterSnapshotOutput, SdkError<CreateClusterSnapshotError>>;
        async fn create_cluster_subnet_group(&self, builder: CreateClusterSubnetGroupInputBuilder) -> Result<CreateClusterSubnetGroupOutput, SdkError<CreateClusterSubnetGroupError>>;
        async fn create_custom_domain_association(&self, builder: CreateCustomDomainAssociationInputBuilder) -> Result<CreateCustomDomainAssociationOutput, SdkError<CreateCustomDomainAssociationError>>;
        async fn create_endpoint_access(&self, builder: CreateEndpointAccessInputBuilder) -> Result<CreateEndpointAccessOutput, SdkError<CreateEndpointAccessError>>;
        async fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>;
        async fn create_hsm_client_certificate(&self, builder: CreateHsmClientCertificateInputBuilder) -> Result<CreateHsmClientCertificateOutput, SdkError<CreateHsmClientCertificateError>>;
        async fn create_hsm_configuration(&self, builder: CreateHsmConfigurationInputBuilder) -> Result<CreateHsmConfigurationOutput, SdkError<CreateHsmConfigurationError>>;
        async fn create_redshift_idc_application(&self, builder: CreateRedshiftIdcApplicationInputBuilder) -> Result<CreateRedshiftIdcApplicationOutput, SdkError<CreateRedshiftIdcApplicationError>>;
        async fn create_scheduled_action(&self, builder: CreateScheduledActionInputBuilder) -> Result<CreateScheduledActionOutput, SdkError<CreateScheduledActionError>>;
        async fn create_snapshot_copy_grant(&self, builder: CreateSnapshotCopyGrantInputBuilder) -> Result<CreateSnapshotCopyGrantOutput, SdkError<CreateSnapshotCopyGrantError>>;
        async fn create_snapshot_schedule(&self, builder: CreateSnapshotScheduleInputBuilder) -> Result<CreateSnapshotScheduleOutput, SdkError<CreateSnapshotScheduleError>>;
        async fn create_tags(&self, builder: CreateTagsInputBuilder) -> Result<CreateTagsOutput, SdkError<CreateTagsError>>;
        async fn create_usage_limit(&self, builder: CreateUsageLimitInputBuilder) -> Result<CreateUsageLimitOutput, SdkError<CreateUsageLimitError>>;
        async fn deauthorize_data_share(&self, builder: DeauthorizeDataShareInputBuilder) -> Result<DeauthorizeDataShareOutput, SdkError<DeauthorizeDataShareError>>;
        async fn delete_authentication_profile(&self, builder: DeleteAuthenticationProfileInputBuilder) -> Result<DeleteAuthenticationProfileOutput, SdkError<DeleteAuthenticationProfileError>>;
        async fn delete_cluster(&self, builder: DeleteClusterInputBuilder) -> Result<DeleteClusterOutput, SdkError<DeleteClusterError>>;
        async fn delete_cluster_parameter_group(&self, builder: DeleteClusterParameterGroupInputBuilder) -> Result<DeleteClusterParameterGroupOutput, SdkError<DeleteClusterParameterGroupError>>;
        async fn delete_cluster_security_group(&self, builder: DeleteClusterSecurityGroupInputBuilder) -> Result<DeleteClusterSecurityGroupOutput, SdkError<DeleteClusterSecurityGroupError>>;
        async fn delete_cluster_snapshot(&self, builder: DeleteClusterSnapshotInputBuilder) -> Result<DeleteClusterSnapshotOutput, SdkError<DeleteClusterSnapshotError>>;
        async fn delete_cluster_subnet_group(&self, builder: DeleteClusterSubnetGroupInputBuilder) -> Result<DeleteClusterSubnetGroupOutput, SdkError<DeleteClusterSubnetGroupError>>;
        async fn delete_custom_domain_association(&self, builder: DeleteCustomDomainAssociationInputBuilder) -> Result<DeleteCustomDomainAssociationOutput, SdkError<DeleteCustomDomainAssociationError>>;
        async fn delete_endpoint_access(&self, builder: DeleteEndpointAccessInputBuilder) -> Result<DeleteEndpointAccessOutput, SdkError<DeleteEndpointAccessError>>;
        async fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>;
        async fn delete_hsm_client_certificate(&self, builder: DeleteHsmClientCertificateInputBuilder) -> Result<DeleteHsmClientCertificateOutput, SdkError<DeleteHsmClientCertificateError>>;
        async fn delete_hsm_configuration(&self, builder: DeleteHsmConfigurationInputBuilder) -> Result<DeleteHsmConfigurationOutput, SdkError<DeleteHsmConfigurationError>>;
        async fn delete_partner(&self, builder: DeletePartnerInputBuilder) -> Result<DeletePartnerOutput, SdkError<DeletePartnerError>>;
        async fn delete_redshift_idc_application(&self, builder: DeleteRedshiftIdcApplicationInputBuilder) -> Result<DeleteRedshiftIdcApplicationOutput, SdkError<DeleteRedshiftIdcApplicationError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn delete_scheduled_action(&self, builder: DeleteScheduledActionInputBuilder) -> Result<DeleteScheduledActionOutput, SdkError<DeleteScheduledActionError>>;
        async fn delete_snapshot_copy_grant(&self, builder: DeleteSnapshotCopyGrantInputBuilder) -> Result<DeleteSnapshotCopyGrantOutput, SdkError<DeleteSnapshotCopyGrantError>>;
        async fn delete_snapshot_schedule(&self, builder: DeleteSnapshotScheduleInputBuilder) -> Result<DeleteSnapshotScheduleOutput, SdkError<DeleteSnapshotScheduleError>>;
        async fn delete_tags(&self, builder: DeleteTagsInputBuilder) -> Result<DeleteTagsOutput, SdkError<DeleteTagsError>>;
        async fn delete_usage_limit(&self, builder: DeleteUsageLimitInputBuilder) -> Result<DeleteUsageLimitOutput, SdkError<DeleteUsageLimitError>>;
        async fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>;
        async fn describe_authentication_profiles(&self, builder: DescribeAuthenticationProfilesInputBuilder) -> Result<DescribeAuthenticationProfilesOutput, SdkError<DescribeAuthenticationProfilesError>>;
        async fn describe_cluster_db_revisions(&self, builder: DescribeClusterDbRevisionsInputBuilder) -> Result<DescribeClusterDbRevisionsOutput, SdkError<DescribeClusterDbRevisionsError>>;
        async fn describe_cluster_parameter_groups(&self, builder: DescribeClusterParameterGroupsInputBuilder) -> Result<DescribeClusterParameterGroupsOutput, SdkError<DescribeClusterParameterGroupsError>>;
        async fn describe_cluster_parameters(&self, builder: DescribeClusterParametersInputBuilder) -> Result<DescribeClusterParametersOutput, SdkError<DescribeClusterParametersError>>;
        async fn describe_cluster_security_groups(&self, builder: DescribeClusterSecurityGroupsInputBuilder) -> Result<DescribeClusterSecurityGroupsOutput, SdkError<DescribeClusterSecurityGroupsError>>;
        async fn describe_cluster_snapshots(&self, builder: DescribeClusterSnapshotsInputBuilder) -> Result<DescribeClusterSnapshotsOutput, SdkError<DescribeClusterSnapshotsError>>;
        async fn describe_cluster_subnet_groups(&self, builder: DescribeClusterSubnetGroupsInputBuilder) -> Result<DescribeClusterSubnetGroupsOutput, SdkError<DescribeClusterSubnetGroupsError>>;
        async fn describe_cluster_tracks(&self, builder: DescribeClusterTracksInputBuilder) -> Result<DescribeClusterTracksOutput, SdkError<DescribeClusterTracksError>>;
        async fn describe_cluster_versions(&self, builder: DescribeClusterVersionsInputBuilder) -> Result<DescribeClusterVersionsOutput, SdkError<DescribeClusterVersionsError>>;
        async fn describe_clusters(&self, builder: DescribeClustersInputBuilder) -> Result<DescribeClustersOutput, SdkError<DescribeClustersError>>;
        async fn describe_custom_domain_associations(&self, builder: DescribeCustomDomainAssociationsInputBuilder) -> Result<DescribeCustomDomainAssociationsOutput, SdkError<DescribeCustomDomainAssociationsError>>;
        async fn describe_data_shares(&self, builder: DescribeDataSharesInputBuilder) -> Result<DescribeDataSharesOutput, SdkError<DescribeDataSharesError>>;
        async fn describe_data_shares_for_consumer(&self, builder: DescribeDataSharesForConsumerInputBuilder) -> Result<DescribeDataSharesForConsumerOutput, SdkError<DescribeDataSharesForConsumerError>>;
        async fn describe_data_shares_for_producer(&self, builder: DescribeDataSharesForProducerInputBuilder) -> Result<DescribeDataSharesForProducerOutput, SdkError<DescribeDataSharesForProducerError>>;
        async fn describe_default_cluster_parameters(&self, builder: DescribeDefaultClusterParametersInputBuilder) -> Result<DescribeDefaultClusterParametersOutput, SdkError<DescribeDefaultClusterParametersError>>;
        async fn describe_endpoint_access(&self, builder: DescribeEndpointAccessInputBuilder) -> Result<DescribeEndpointAccessOutput, SdkError<DescribeEndpointAccessError>>;
        async fn describe_endpoint_authorization(&self, builder: DescribeEndpointAuthorizationInputBuilder) -> Result<DescribeEndpointAuthorizationOutput, SdkError<DescribeEndpointAuthorizationError>>;
        async fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>;
        async fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>;
        async fn describe_events(&self, builder: DescribeEventsInputBuilder) -> Result<DescribeEventsOutput, SdkError<DescribeEventsError>>;
        async fn describe_hsm_client_certificates(&self, builder: DescribeHsmClientCertificatesInputBuilder) -> Result<DescribeHsmClientCertificatesOutput, SdkError<DescribeHsmClientCertificatesError>>;
        async fn describe_hsm_configurations(&self, builder: DescribeHsmConfigurationsInputBuilder) -> Result<DescribeHsmConfigurationsOutput, SdkError<DescribeHsmConfigurationsError>>;
        async fn describe_inbound_integrations(&self, builder: DescribeInboundIntegrationsInputBuilder) -> Result<DescribeInboundIntegrationsOutput, SdkError<DescribeInboundIntegrationsError>>;
        async fn describe_logging_status(&self, builder: DescribeLoggingStatusInputBuilder) -> Result<DescribeLoggingStatusOutput, SdkError<DescribeLoggingStatusError>>;
        async fn describe_node_configuration_options(&self, builder: DescribeNodeConfigurationOptionsInputBuilder) -> Result<DescribeNodeConfigurationOptionsOutput, SdkError<DescribeNodeConfigurationOptionsError>>;
        async fn describe_orderable_cluster_options(&self, builder: DescribeOrderableClusterOptionsInputBuilder) -> Result<DescribeOrderableClusterOptionsOutput, SdkError<DescribeOrderableClusterOptionsError>>;
        async fn describe_partners(&self, builder: DescribePartnersInputBuilder) -> Result<DescribePartnersOutput, SdkError<DescribePartnersError>>;
        async fn describe_redshift_idc_applications(&self, builder: DescribeRedshiftIdcApplicationsInputBuilder) -> Result<DescribeRedshiftIdcApplicationsOutput, SdkError<DescribeRedshiftIdcApplicationsError>>;
        async fn describe_reserved_node_exchange_status(&self, builder: DescribeReservedNodeExchangeStatusInputBuilder) -> Result<DescribeReservedNodeExchangeStatusOutput, SdkError<DescribeReservedNodeExchangeStatusError>>;
        async fn describe_reserved_node_offerings(&self, builder: DescribeReservedNodeOfferingsInputBuilder) -> Result<DescribeReservedNodeOfferingsOutput, SdkError<DescribeReservedNodeOfferingsError>>;
        async fn describe_reserved_nodes(&self, builder: DescribeReservedNodesInputBuilder) -> Result<DescribeReservedNodesOutput, SdkError<DescribeReservedNodesError>>;
        async fn describe_resize(&self, builder: DescribeResizeInputBuilder) -> Result<DescribeResizeOutput, SdkError<DescribeResizeError>>;
        async fn describe_scheduled_actions(&self, builder: DescribeScheduledActionsInputBuilder) -> Result<DescribeScheduledActionsOutput, SdkError<DescribeScheduledActionsError>>;
        async fn describe_snapshot_copy_grants(&self, builder: DescribeSnapshotCopyGrantsInputBuilder) -> Result<DescribeSnapshotCopyGrantsOutput, SdkError<DescribeSnapshotCopyGrantsError>>;
        async fn describe_snapshot_schedules(&self, builder: DescribeSnapshotSchedulesInputBuilder) -> Result<DescribeSnapshotSchedulesOutput, SdkError<DescribeSnapshotSchedulesError>>;
        async fn describe_storage(&self, builder: DescribeStorageInputBuilder) -> Result<DescribeStorageOutput, SdkError<DescribeStorageError>>;
        async fn describe_table_restore_status(&self, builder: DescribeTableRestoreStatusInputBuilder) -> Result<DescribeTableRestoreStatusOutput, SdkError<DescribeTableRestoreStatusError>>;
        async fn describe_tags(&self, builder: DescribeTagsInputBuilder) -> Result<DescribeTagsOutput, SdkError<DescribeTagsError>>;
        async fn describe_usage_limits(&self, builder: DescribeUsageLimitsInputBuilder) -> Result<DescribeUsageLimitsOutput, SdkError<DescribeUsageLimitsError>>;
        async fn disable_logging(&self, builder: DisableLoggingInputBuilder) -> Result<DisableLoggingOutput, SdkError<DisableLoggingError>>;
        async fn disable_snapshot_copy(&self, builder: DisableSnapshotCopyInputBuilder) -> Result<DisableSnapshotCopyOutput, SdkError<DisableSnapshotCopyError>>;
        async fn disassociate_data_share_consumer(&self, builder: DisassociateDataShareConsumerInputBuilder) -> Result<DisassociateDataShareConsumerOutput, SdkError<DisassociateDataShareConsumerError>>;
        async fn enable_logging(&self, builder: EnableLoggingInputBuilder) -> Result<EnableLoggingOutput, SdkError<EnableLoggingError>>;
        async fn enable_snapshot_copy(&self, builder: EnableSnapshotCopyInputBuilder) -> Result<EnableSnapshotCopyOutput, SdkError<EnableSnapshotCopyError>>;
        async fn failover_primary_compute(&self, builder: FailoverPrimaryComputeInputBuilder) -> Result<FailoverPrimaryComputeOutput, SdkError<FailoverPrimaryComputeError>>;
        async fn get_cluster_credentials(&self, builder: GetClusterCredentialsInputBuilder) -> Result<GetClusterCredentialsOutput, SdkError<GetClusterCredentialsError>>;
        async fn get_cluster_credentials_with_iam(&self, builder: GetClusterCredentialsWithIamInputBuilder) -> Result<GetClusterCredentialsWithIamOutput, SdkError<GetClusterCredentialsWithIAMError>>;
        async fn get_reserved_node_exchange_configuration_options(&self, builder: GetReservedNodeExchangeConfigurationOptionsInputBuilder) -> Result<GetReservedNodeExchangeConfigurationOptionsOutput, SdkError<GetReservedNodeExchangeConfigurationOptionsError>>;
        async fn get_reserved_node_exchange_offerings(&self, builder: GetReservedNodeExchangeOfferingsInputBuilder) -> Result<GetReservedNodeExchangeOfferingsOutput, SdkError<GetReservedNodeExchangeOfferingsError>>;
        async fn get_resource_policy(&self, builder: GetResourcePolicyInputBuilder) -> Result<GetResourcePolicyOutput, SdkError<GetResourcePolicyError>>;
        async fn list_recommendations(&self, builder: ListRecommendationsInputBuilder) -> Result<ListRecommendationsOutput, SdkError<ListRecommendationsError>>;
        async fn modify_aqua_configuration(&self, builder: ModifyAquaConfigurationInputBuilder) -> Result<ModifyAquaConfigurationOutput, SdkError<ModifyAquaConfigurationError>>;
        async fn modify_authentication_profile(&self, builder: ModifyAuthenticationProfileInputBuilder) -> Result<ModifyAuthenticationProfileOutput, SdkError<ModifyAuthenticationProfileError>>;
        async fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> Result<ModifyClusterOutput, SdkError<ModifyClusterError>>;
        async fn modify_cluster_db_revision(&self, builder: ModifyClusterDbRevisionInputBuilder) -> Result<ModifyClusterDbRevisionOutput, SdkError<ModifyClusterDbRevisionError>>;
        async fn modify_cluster_iam_roles(&self, builder: ModifyClusterIamRolesInputBuilder) -> Result<ModifyClusterIamRolesOutput, SdkError<ModifyClusterIamRolesError>>;
        async fn modify_cluster_maintenance(&self, builder: ModifyClusterMaintenanceInputBuilder) -> Result<ModifyClusterMaintenanceOutput, SdkError<ModifyClusterMaintenanceError>>;
        async fn modify_cluster_parameter_group(&self, builder: ModifyClusterParameterGroupInputBuilder) -> Result<ModifyClusterParameterGroupOutput, SdkError<ModifyClusterParameterGroupError>>;
        async fn modify_cluster_snapshot(&self, builder: ModifyClusterSnapshotInputBuilder) -> Result<ModifyClusterSnapshotOutput, SdkError<ModifyClusterSnapshotError>>;
        async fn modify_cluster_snapshot_schedule(&self, builder: ModifyClusterSnapshotScheduleInputBuilder) -> Result<ModifyClusterSnapshotScheduleOutput, SdkError<ModifyClusterSnapshotScheduleError>>;
        async fn modify_cluster_subnet_group(&self, builder: ModifyClusterSubnetGroupInputBuilder) -> Result<ModifyClusterSubnetGroupOutput, SdkError<ModifyClusterSubnetGroupError>>;
        async fn modify_custom_domain_association(&self, builder: ModifyCustomDomainAssociationInputBuilder) -> Result<ModifyCustomDomainAssociationOutput, SdkError<ModifyCustomDomainAssociationError>>;
        async fn modify_endpoint_access(&self, builder: ModifyEndpointAccessInputBuilder) -> Result<ModifyEndpointAccessOutput, SdkError<ModifyEndpointAccessError>>;
        async fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>;
        async fn modify_redshift_idc_application(&self, builder: ModifyRedshiftIdcApplicationInputBuilder) -> Result<ModifyRedshiftIdcApplicationOutput, SdkError<ModifyRedshiftIdcApplicationError>>;
        async fn modify_scheduled_action(&self, builder: ModifyScheduledActionInputBuilder) -> Result<ModifyScheduledActionOutput, SdkError<ModifyScheduledActionError>>;
        async fn modify_snapshot_copy_retention_period(&self, builder: ModifySnapshotCopyRetentionPeriodInputBuilder) -> Result<ModifySnapshotCopyRetentionPeriodOutput, SdkError<ModifySnapshotCopyRetentionPeriodError>>;
        async fn modify_snapshot_schedule(&self, builder: ModifySnapshotScheduleInputBuilder) -> Result<ModifySnapshotScheduleOutput, SdkError<ModifySnapshotScheduleError>>;
        async fn modify_usage_limit(&self, builder: ModifyUsageLimitInputBuilder) -> Result<ModifyUsageLimitOutput, SdkError<ModifyUsageLimitError>>;
        async fn pause_cluster(&self, builder: PauseClusterInputBuilder) -> Result<PauseClusterOutput, SdkError<PauseClusterError>>;
        async fn purchase_reserved_node_offering(&self, builder: PurchaseReservedNodeOfferingInputBuilder) -> Result<PurchaseReservedNodeOfferingOutput, SdkError<PurchaseReservedNodeOfferingError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn reboot_cluster(&self, builder: RebootClusterInputBuilder) -> Result<RebootClusterOutput, SdkError<RebootClusterError>>;
        async fn reject_data_share(&self, builder: RejectDataShareInputBuilder) -> Result<RejectDataShareOutput, SdkError<RejectDataShareError>>;
        async fn reset_cluster_parameter_group(&self, builder: ResetClusterParameterGroupInputBuilder) -> Result<ResetClusterParameterGroupOutput, SdkError<ResetClusterParameterGroupError>>;
        async fn resize_cluster(&self, builder: ResizeClusterInputBuilder) -> Result<ResizeClusterOutput, SdkError<ResizeClusterError>>;
        async fn restore_from_cluster_snapshot(&self, builder: RestoreFromClusterSnapshotInputBuilder) -> Result<RestoreFromClusterSnapshotOutput, SdkError<RestoreFromClusterSnapshotError>>;
        async fn restore_table_from_cluster_snapshot(&self, builder: RestoreTableFromClusterSnapshotInputBuilder) -> Result<RestoreTableFromClusterSnapshotOutput, SdkError<RestoreTableFromClusterSnapshotError>>;
        async fn resume_cluster(&self, builder: ResumeClusterInputBuilder) -> Result<ResumeClusterOutput, SdkError<ResumeClusterError>>;
        async fn revoke_cluster_security_group_ingress(&self, builder: RevokeClusterSecurityGroupIngressInputBuilder) -> Result<RevokeClusterSecurityGroupIngressOutput, SdkError<RevokeClusterSecurityGroupIngressError>>;
        async fn revoke_endpoint_access(&self, builder: RevokeEndpointAccessInputBuilder) -> Result<RevokeEndpointAccessOutput, SdkError<RevokeEndpointAccessError>>;
        async fn revoke_snapshot_access(&self, builder: RevokeSnapshotAccessInputBuilder) -> Result<RevokeSnapshotAccessOutput, SdkError<RevokeSnapshotAccessError>>;
        async fn rotate_encryption_key(&self, builder: RotateEncryptionKeyInputBuilder) -> Result<RotateEncryptionKeyOutput, SdkError<RotateEncryptionKeyError>>;
        async fn update_partner_status(&self, builder: UpdatePartnerStatusInputBuilder) -> Result<UpdatePartnerStatusOutput, SdkError<UpdatePartnerStatusError>>;
    }
}
