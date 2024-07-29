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
use aws_sdk_rds::operation::add_role_to_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::add_role_to_db_instance::{builders::*, *};
use aws_sdk_rds::operation::add_source_identifier_to_subscription::{builders::*, *};
use aws_sdk_rds::operation::add_tags_to_resource::{builders::*, *};
use aws_sdk_rds::operation::apply_pending_maintenance_action::{builders::*, *};
use aws_sdk_rds::operation::authorize_db_security_group_ingress::{builders::*, *};
use aws_sdk_rds::operation::backtrack_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::cancel_export_task::{builders::*, *};
use aws_sdk_rds::operation::copy_db_cluster_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::copy_db_cluster_snapshot::{builders::*, *};
use aws_sdk_rds::operation::copy_db_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::copy_db_snapshot::{builders::*, *};
use aws_sdk_rds::operation::copy_option_group::{builders::*, *};
use aws_sdk_rds::operation::create_blue_green_deployment::{builders::*, *};
use aws_sdk_rds::operation::create_custom_db_engine_version::{builders::*, *};
use aws_sdk_rds::operation::create_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::create_db_cluster_endpoint::{builders::*, *};
use aws_sdk_rds::operation::create_db_cluster_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::create_db_cluster_snapshot::{builders::*, *};
use aws_sdk_rds::operation::create_db_instance::{builders::*, *};
use aws_sdk_rds::operation::create_db_instance_read_replica::{builders::*, *};
use aws_sdk_rds::operation::create_db_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::create_db_proxy::{builders::*, *};
use aws_sdk_rds::operation::create_db_proxy_endpoint::{builders::*, *};
use aws_sdk_rds::operation::create_db_security_group::{builders::*, *};
use aws_sdk_rds::operation::create_db_shard_group::{builders::*, *};
use aws_sdk_rds::operation::create_db_snapshot::{builders::*, *};
use aws_sdk_rds::operation::create_db_subnet_group::{builders::*, *};
use aws_sdk_rds::operation::create_event_subscription::{builders::*, *};
use aws_sdk_rds::operation::create_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::create_integration::{builders::*, *};
use aws_sdk_rds::operation::create_option_group::{builders::*, *};
use aws_sdk_rds::operation::create_tenant_database::{builders::*, *};
use aws_sdk_rds::operation::delete_blue_green_deployment::{builders::*, *};
use aws_sdk_rds::operation::delete_custom_db_engine_version::{builders::*, *};
use aws_sdk_rds::operation::delete_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::delete_db_cluster_automated_backup::{builders::*, *};
use aws_sdk_rds::operation::delete_db_cluster_endpoint::{builders::*, *};
use aws_sdk_rds::operation::delete_db_cluster_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::delete_db_cluster_snapshot::{builders::*, *};
use aws_sdk_rds::operation::delete_db_instance::{builders::*, *};
use aws_sdk_rds::operation::delete_db_instance_automated_backup::{builders::*, *};
use aws_sdk_rds::operation::delete_db_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::delete_db_proxy::{builders::*, *};
use aws_sdk_rds::operation::delete_db_proxy_endpoint::{builders::*, *};
use aws_sdk_rds::operation::delete_db_security_group::{builders::*, *};
use aws_sdk_rds::operation::delete_db_shard_group::{builders::*, *};
use aws_sdk_rds::operation::delete_db_snapshot::{builders::*, *};
use aws_sdk_rds::operation::delete_db_subnet_group::{builders::*, *};
use aws_sdk_rds::operation::delete_event_subscription::{builders::*, *};
use aws_sdk_rds::operation::delete_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::delete_integration::{builders::*, *};
use aws_sdk_rds::operation::delete_option_group::{builders::*, *};
use aws_sdk_rds::operation::delete_tenant_database::{builders::*, *};
use aws_sdk_rds::operation::deregister_db_proxy_targets::{builders::*, *};
use aws_sdk_rds::operation::describe_account_attributes::{builders::*, *};
use aws_sdk_rds::operation::describe_blue_green_deployments::{builders::*, *};
use aws_sdk_rds::operation::describe_certificates::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_automated_backups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_backtracks::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_endpoints::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_parameter_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_parameters::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_snapshot_attributes::{builders::*, *};
use aws_sdk_rds::operation::describe_db_cluster_snapshots::{builders::*, *};
use aws_sdk_rds::operation::describe_db_clusters::{builders::*, *};
use aws_sdk_rds::operation::describe_db_engine_versions::{builders::*, *};
use aws_sdk_rds::operation::describe_db_instance_automated_backups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_instances::{builders::*, *};
use aws_sdk_rds::operation::describe_db_log_files::{builders::*, *};
use aws_sdk_rds::operation::describe_db_parameter_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_parameters::{builders::*, *};
use aws_sdk_rds::operation::describe_db_proxies::{builders::*, *};
use aws_sdk_rds::operation::describe_db_proxy_endpoints::{builders::*, *};
use aws_sdk_rds::operation::describe_db_proxy_target_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_proxy_targets::{builders::*, *};
use aws_sdk_rds::operation::describe_db_recommendations::{builders::*, *};
use aws_sdk_rds::operation::describe_db_security_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_shard_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_db_snapshot_attributes::{builders::*, *};
use aws_sdk_rds::operation::describe_db_snapshot_tenant_databases::{builders::*, *};
use aws_sdk_rds::operation::describe_db_snapshots::{builders::*, *};
use aws_sdk_rds::operation::describe_db_subnet_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_engine_default_cluster_parameters::{builders::*, *};
use aws_sdk_rds::operation::describe_engine_default_parameters::{builders::*, *};
use aws_sdk_rds::operation::describe_event_categories::{builders::*, *};
use aws_sdk_rds::operation::describe_event_subscriptions::{builders::*, *};
use aws_sdk_rds::operation::describe_events::{builders::*, *};
use aws_sdk_rds::operation::describe_export_tasks::{builders::*, *};
use aws_sdk_rds::operation::describe_global_clusters::{builders::*, *};
use aws_sdk_rds::operation::describe_integrations::{builders::*, *};
use aws_sdk_rds::operation::describe_option_group_options::{builders::*, *};
use aws_sdk_rds::operation::describe_option_groups::{builders::*, *};
use aws_sdk_rds::operation::describe_orderable_db_instance_options::{builders::*, *};
use aws_sdk_rds::operation::describe_pending_maintenance_actions::{builders::*, *};
use aws_sdk_rds::operation::describe_reserved_db_instances::{builders::*, *};
use aws_sdk_rds::operation::describe_reserved_db_instances_offerings::{builders::*, *};
use aws_sdk_rds::operation::describe_source_regions::{builders::*, *};
use aws_sdk_rds::operation::describe_tenant_databases::{builders::*, *};
use aws_sdk_rds::operation::describe_valid_db_instance_modifications::{builders::*, *};
use aws_sdk_rds::operation::disable_http_endpoint::{builders::*, *};
use aws_sdk_rds::operation::download_db_log_file_portion::{builders::*, *};
use aws_sdk_rds::operation::enable_http_endpoint::{builders::*, *};
use aws_sdk_rds::operation::failover_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::failover_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_rds::operation::modify_activity_stream::{builders::*, *};
use aws_sdk_rds::operation::modify_certificates::{builders::*, *};
use aws_sdk_rds::operation::modify_current_db_cluster_capacity::{builders::*, *};
use aws_sdk_rds::operation::modify_custom_db_engine_version::{builders::*, *};
use aws_sdk_rds::operation::modify_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::modify_db_cluster_endpoint::{builders::*, *};
use aws_sdk_rds::operation::modify_db_cluster_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::modify_db_cluster_snapshot_attribute::{builders::*, *};
use aws_sdk_rds::operation::modify_db_instance::{builders::*, *};
use aws_sdk_rds::operation::modify_db_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::modify_db_proxy::{builders::*, *};
use aws_sdk_rds::operation::modify_db_proxy_endpoint::{builders::*, *};
use aws_sdk_rds::operation::modify_db_proxy_target_group::{builders::*, *};
use aws_sdk_rds::operation::modify_db_recommendation::{builders::*, *};
use aws_sdk_rds::operation::modify_db_shard_group::{builders::*, *};
use aws_sdk_rds::operation::modify_db_snapshot::{builders::*, *};
use aws_sdk_rds::operation::modify_db_snapshot_attribute::{builders::*, *};
use aws_sdk_rds::operation::modify_db_subnet_group::{builders::*, *};
use aws_sdk_rds::operation::modify_event_subscription::{builders::*, *};
use aws_sdk_rds::operation::modify_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::modify_integration::{builders::*, *};
use aws_sdk_rds::operation::modify_tenant_database::{builders::*, *};
use aws_sdk_rds::operation::promote_read_replica::{builders::*, *};
use aws_sdk_rds::operation::promote_read_replica_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::purchase_reserved_db_instances_offering::{builders::*, *};
use aws_sdk_rds::operation::reboot_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::reboot_db_instance::{builders::*, *};
use aws_sdk_rds::operation::reboot_db_shard_group::{builders::*, *};
use aws_sdk_rds::operation::register_db_proxy_targets::{builders::*, *};
use aws_sdk_rds::operation::remove_from_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::remove_role_from_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::remove_role_from_db_instance::{builders::*, *};
use aws_sdk_rds::operation::remove_source_identifier_from_subscription::{builders::*, *};
use aws_sdk_rds::operation::remove_tags_from_resource::{builders::*, *};
use aws_sdk_rds::operation::reset_db_cluster_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::reset_db_parameter_group::{builders::*, *};
use aws_sdk_rds::operation::restore_db_cluster_from_s3::{builders::*, *};
use aws_sdk_rds::operation::restore_db_cluster_from_snapshot::{builders::*, *};
use aws_sdk_rds::operation::restore_db_cluster_to_point_in_time::{builders::*, *};
use aws_sdk_rds::operation::restore_db_instance_from_db_snapshot::{builders::*, *};
use aws_sdk_rds::operation::restore_db_instance_from_s3::{builders::*, *};
use aws_sdk_rds::operation::restore_db_instance_to_point_in_time::{builders::*, *};
use aws_sdk_rds::operation::revoke_db_security_group_ingress::{builders::*, *};
use aws_sdk_rds::operation::start_activity_stream::{builders::*, *};
use aws_sdk_rds::operation::start_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::start_db_instance::{builders::*, *};
use aws_sdk_rds::operation::start_db_instance_automated_backups_replication::{builders::*, *};
use aws_sdk_rds::operation::start_export_task::{builders::*, *};
use aws_sdk_rds::operation::stop_activity_stream::{builders::*, *};
use aws_sdk_rds::operation::stop_db_cluster::{builders::*, *};
use aws_sdk_rds::operation::stop_db_instance::{builders::*, *};
use aws_sdk_rds::operation::stop_db_instance_automated_backups_replication::{builders::*, *};
use aws_sdk_rds::operation::switchover_blue_green_deployment::{builders::*, *};
use aws_sdk_rds::operation::switchover_global_cluster::{builders::*, *};
use aws_sdk_rds::operation::switchover_read_replica::{builders::*, *};
use aws_sdk_rds::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_rds::Client;
use std::ops::Deref;

pub use aws_sdk_rds::*;

pub struct RDSClientImpl(Client);
impl RDSClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait RDSClient {
    fn add_role_to_db_cluster(&self, builder: AddRoleToDbClusterInputBuilder) -> impl Future<Output = Result<AddRoleToDbClusterOutput, SdkError<AddRoleToDBClusterError>>>;
    fn add_role_to_db_instance(&self, builder: AddRoleToDbInstanceInputBuilder) -> impl Future<Output = Result<AddRoleToDbInstanceOutput, SdkError<AddRoleToDBInstanceError>>>;
    fn add_source_identifier_to_subscription(&self, builder: AddSourceIdentifierToSubscriptionInputBuilder) -> impl Future<Output = Result<AddSourceIdentifierToSubscriptionOutput, SdkError<AddSourceIdentifierToSubscriptionError>>>;
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>>;
    fn apply_pending_maintenance_action(&self, builder: ApplyPendingMaintenanceActionInputBuilder) -> impl Future<Output = Result<ApplyPendingMaintenanceActionOutput, SdkError<ApplyPendingMaintenanceActionError>>>;
    fn authorize_db_security_group_ingress(&self, builder: AuthorizeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeDbSecurityGroupIngressOutput, SdkError<AuthorizeDBSecurityGroupIngressError>>>;
    fn backtrack_db_cluster(&self, builder: BacktrackDbClusterInputBuilder) -> impl Future<Output = Result<BacktrackDbClusterOutput, SdkError<BacktrackDBClusterError>>>;
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>>;
    fn copy_db_cluster_parameter_group(&self, builder: CopyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbClusterParameterGroupOutput, SdkError<CopyDBClusterParameterGroupError>>>;
    fn copy_db_cluster_snapshot(&self, builder: CopyDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbClusterSnapshotOutput, SdkError<CopyDBClusterSnapshotError>>>;
    fn copy_db_parameter_group(&self, builder: CopyDbParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbParameterGroupOutput, SdkError<CopyDBParameterGroupError>>>;
    fn copy_db_snapshot(&self, builder: CopyDbSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbSnapshotOutput, SdkError<CopyDBSnapshotError>>>;
    fn copy_option_group(&self, builder: CopyOptionGroupInputBuilder) -> impl Future<Output = Result<CopyOptionGroupOutput, SdkError<CopyOptionGroupError>>>;
    fn create_blue_green_deployment(&self, builder: CreateBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<CreateBlueGreenDeploymentOutput, SdkError<CreateBlueGreenDeploymentError>>>;
    fn create_custom_db_engine_version(&self, builder: CreateCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<CreateCustomDbEngineVersionOutput, SdkError<CreateCustomDBEngineVersionError>>>;
    fn create_db_cluster(&self, builder: CreateDbClusterInputBuilder) -> impl Future<Output = Result<CreateDbClusterOutput, SdkError<CreateDBClusterError>>>;
    fn create_db_cluster_endpoint(&self, builder: CreateDbClusterEndpointInputBuilder) -> impl Future<Output = Result<CreateDbClusterEndpointOutput, SdkError<CreateDBClusterEndpointError>>>;
    fn create_db_cluster_parameter_group(&self, builder: CreateDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbClusterParameterGroupOutput, SdkError<CreateDBClusterParameterGroupError>>>;
    fn create_db_cluster_snapshot(&self, builder: CreateDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbClusterSnapshotOutput, SdkError<CreateDBClusterSnapshotError>>>;
    fn create_db_instance(&self, builder: CreateDbInstanceInputBuilder) -> impl Future<Output = Result<CreateDbInstanceOutput, SdkError<CreateDBInstanceError>>>;
    fn create_db_instance_read_replica(&self, builder: CreateDbInstanceReadReplicaInputBuilder) -> impl Future<Output = Result<CreateDbInstanceReadReplicaOutput, SdkError<CreateDBInstanceReadReplicaError>>>;
    fn create_db_parameter_group(&self, builder: CreateDbParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbParameterGroupOutput, SdkError<CreateDBParameterGroupError>>>;
    fn create_db_proxy(&self, builder: CreateDbProxyInputBuilder) -> impl Future<Output = Result<CreateDbProxyOutput, SdkError<CreateDBProxyError>>>;
    fn create_db_proxy_endpoint(&self, builder: CreateDbProxyEndpointInputBuilder) -> impl Future<Output = Result<CreateDbProxyEndpointOutput, SdkError<CreateDBProxyEndpointError>>>;
    fn create_db_security_group(&self, builder: CreateDbSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateDbSecurityGroupOutput, SdkError<CreateDBSecurityGroupError>>>;
    fn create_db_shard_group(&self, builder: CreateDbShardGroupInputBuilder) -> impl Future<Output = Result<CreateDbShardGroupOutput, SdkError<CreateDBShardGroupError>>>;
    fn create_db_snapshot(&self, builder: CreateDbSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbSnapshotOutput, SdkError<CreateDBSnapshotError>>>;
    fn create_db_subnet_group(&self, builder: CreateDbSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateDbSubnetGroupOutput, SdkError<CreateDBSubnetGroupError>>>;
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>>;
    fn create_global_cluster(&self, builder: CreateGlobalClusterInputBuilder) -> impl Future<Output = Result<CreateGlobalClusterOutput, SdkError<CreateGlobalClusterError>>>;
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>>;
    fn create_option_group(&self, builder: CreateOptionGroupInputBuilder) -> impl Future<Output = Result<CreateOptionGroupOutput, SdkError<CreateOptionGroupError>>>;
    fn create_tenant_database(&self, builder: CreateTenantDatabaseInputBuilder) -> impl Future<Output = Result<CreateTenantDatabaseOutput, SdkError<CreateTenantDatabaseError>>>;
    fn delete_blue_green_deployment(&self, builder: DeleteBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<DeleteBlueGreenDeploymentOutput, SdkError<DeleteBlueGreenDeploymentError>>>;
    fn delete_custom_db_engine_version(&self, builder: DeleteCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<DeleteCustomDbEngineVersionOutput, SdkError<DeleteCustomDBEngineVersionError>>>;
    fn delete_db_cluster(&self, builder: DeleteDbClusterInputBuilder) -> impl Future<Output = Result<DeleteDbClusterOutput, SdkError<DeleteDBClusterError>>>;
    fn delete_db_cluster_automated_backup(&self, builder: DeleteDbClusterAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterAutomatedBackupOutput, SdkError<DeleteDBClusterAutomatedBackupError>>>;
    fn delete_db_cluster_endpoint(&self, builder: DeleteDbClusterEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbClusterEndpointOutput, SdkError<DeleteDBClusterEndpointError>>>;
    fn delete_db_cluster_parameter_group(&self, builder: DeleteDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterParameterGroupOutput, SdkError<DeleteDBClusterParameterGroupError>>>;
    fn delete_db_cluster_snapshot(&self, builder: DeleteDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbClusterSnapshotOutput, SdkError<DeleteDBClusterSnapshotError>>>;
    fn delete_db_instance(&self, builder: DeleteDbInstanceInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceOutput, SdkError<DeleteDBInstanceError>>>;
    fn delete_db_instance_automated_backup(&self, builder: DeleteDbInstanceAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceAutomatedBackupOutput, SdkError<DeleteDBInstanceAutomatedBackupError>>>;
    fn delete_db_parameter_group(&self, builder: DeleteDbParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbParameterGroupOutput, SdkError<DeleteDBParameterGroupError>>>;
    fn delete_db_proxy(&self, builder: DeleteDbProxyInputBuilder) -> impl Future<Output = Result<DeleteDbProxyOutput, SdkError<DeleteDBProxyError>>>;
    fn delete_db_proxy_endpoint(&self, builder: DeleteDbProxyEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbProxyEndpointOutput, SdkError<DeleteDBProxyEndpointError>>>;
    fn delete_db_security_group(&self, builder: DeleteDbSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSecurityGroupOutput, SdkError<DeleteDBSecurityGroupError>>>;
    fn delete_db_shard_group(&self, builder: DeleteDbShardGroupInputBuilder) -> impl Future<Output = Result<DeleteDbShardGroupOutput, SdkError<DeleteDBShardGroupError>>>;
    fn delete_db_snapshot(&self, builder: DeleteDbSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbSnapshotOutput, SdkError<DeleteDBSnapshotError>>>;
    fn delete_db_subnet_group(&self, builder: DeleteDbSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSubnetGroupOutput, SdkError<DeleteDBSubnetGroupError>>>;
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>>;
    fn delete_global_cluster(&self, builder: DeleteGlobalClusterInputBuilder) -> impl Future<Output = Result<DeleteGlobalClusterOutput, SdkError<DeleteGlobalClusterError>>>;
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>>;
    fn delete_option_group(&self, builder: DeleteOptionGroupInputBuilder) -> impl Future<Output = Result<DeleteOptionGroupOutput, SdkError<DeleteOptionGroupError>>>;
    fn delete_tenant_database(&self, builder: DeleteTenantDatabaseInputBuilder) -> impl Future<Output = Result<DeleteTenantDatabaseOutput, SdkError<DeleteTenantDatabaseError>>>;
    fn deregister_db_proxy_targets(&self, builder: DeregisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DeregisterDbProxyTargetsOutput, SdkError<DeregisterDBProxyTargetsError>>>;
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>>;
    fn describe_blue_green_deployments(&self, builder: DescribeBlueGreenDeploymentsInputBuilder) -> impl Future<Output = Result<DescribeBlueGreenDeploymentsOutput, SdkError<DescribeBlueGreenDeploymentsError>>>;
    fn describe_certificates(&self, builder: DescribeCertificatesInputBuilder) -> impl Future<Output = Result<DescribeCertificatesOutput, SdkError<DescribeCertificatesError>>>;
    fn describe_db_cluster_automated_backups(&self, builder: DescribeDbClusterAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterAutomatedBackupsOutput, SdkError<DescribeDBClusterAutomatedBackupsError>>>;
    fn describe_db_cluster_backtracks(&self, builder: DescribeDbClusterBacktracksInputBuilder) -> impl Future<Output = Result<DescribeDbClusterBacktracksOutput, SdkError<DescribeDBClusterBacktracksError>>>;
    fn describe_db_cluster_endpoints(&self, builder: DescribeDbClusterEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterEndpointsOutput, SdkError<DescribeDBClusterEndpointsError>>>;
    fn describe_db_cluster_parameter_groups(&self, builder: DescribeDbClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParameterGroupsOutput, SdkError<DescribeDBClusterParameterGroupsError>>>;
    fn describe_db_cluster_parameters(&self, builder: DescribeDbClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParametersOutput, SdkError<DescribeDBClusterParametersError>>>;
    fn describe_db_cluster_snapshot_attributes(&self, builder: DescribeDbClusterSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotAttributesOutput, SdkError<DescribeDBClusterSnapshotAttributesError>>>;
    fn describe_db_cluster_snapshots(&self, builder: DescribeDbClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotsOutput, SdkError<DescribeDBClusterSnapshotsError>>>;
    fn describe_db_clusters(&self, builder: DescribeDbClustersInputBuilder) -> impl Future<Output = Result<DescribeDbClustersOutput, SdkError<DescribeDBClustersError>>>;
    fn describe_db_engine_versions(&self, builder: DescribeDbEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeDbEngineVersionsOutput, SdkError<DescribeDBEngineVersionsError>>>;
    fn describe_db_instance_automated_backups(&self, builder: DescribeDbInstanceAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbInstanceAutomatedBackupsOutput, SdkError<DescribeDBInstanceAutomatedBackupsError>>>;
    fn describe_db_instances(&self, builder: DescribeDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeDbInstancesOutput, SdkError<DescribeDBInstancesError>>>;
    fn describe_db_log_files(&self, builder: DescribeDbLogFilesInputBuilder) -> impl Future<Output = Result<DescribeDbLogFilesOutput, SdkError<DescribeDBLogFilesError>>>;
    fn describe_db_parameter_groups(&self, builder: DescribeDbParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbParameterGroupsOutput, SdkError<DescribeDBParameterGroupsError>>>;
    fn describe_db_parameters(&self, builder: DescribeDbParametersInputBuilder) -> impl Future<Output = Result<DescribeDbParametersOutput, SdkError<DescribeDBParametersError>>>;
    fn describe_db_proxies(&self, builder: DescribeDbProxiesInputBuilder) -> impl Future<Output = Result<DescribeDbProxiesOutput, SdkError<DescribeDBProxiesError>>>;
    fn describe_db_proxy_endpoints(&self, builder: DescribeDbProxyEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyEndpointsOutput, SdkError<DescribeDBProxyEndpointsError>>>;
    fn describe_db_proxy_target_groups(&self, builder: DescribeDbProxyTargetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetGroupsOutput, SdkError<DescribeDBProxyTargetGroupsError>>>;
    fn describe_db_proxy_targets(&self, builder: DescribeDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetsOutput, SdkError<DescribeDBProxyTargetsError>>>;
    fn describe_db_recommendations(&self, builder: DescribeDbRecommendationsInputBuilder) -> impl Future<Output = Result<DescribeDbRecommendationsOutput, SdkError<DescribeDBRecommendationsError>>>;
    fn describe_db_security_groups(&self, builder: DescribeDbSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSecurityGroupsOutput, SdkError<DescribeDBSecurityGroupsError>>>;
    fn describe_db_shard_groups(&self, builder: DescribeDbShardGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbShardGroupsOutput, SdkError<DescribeDBShardGroupsError>>>;
    fn describe_db_snapshot_attributes(&self, builder: DescribeDbSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotAttributesOutput, SdkError<DescribeDBSnapshotAttributesError>>>;
    fn describe_db_snapshot_tenant_databases(&self, builder: DescribeDbSnapshotTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotTenantDatabasesOutput, SdkError<DescribeDBSnapshotTenantDatabasesError>>>;
    fn describe_db_snapshots(&self, builder: DescribeDbSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotsOutput, SdkError<DescribeDBSnapshotsError>>>;
    fn describe_db_subnet_groups(&self, builder: DescribeDbSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSubnetGroupsOutput, SdkError<DescribeDBSubnetGroupsError>>>;
    fn describe_engine_default_cluster_parameters(&self, builder: DescribeEngineDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultClusterParametersOutput, SdkError<DescribeEngineDefaultClusterParametersError>>>;
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>>;
    fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> impl Future<Output = Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>>;
    fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> impl Future<Output = Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>>;
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>>;
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>>;
    fn describe_global_clusters(&self, builder: DescribeGlobalClustersInputBuilder) -> impl Future<Output = Result<DescribeGlobalClustersOutput, SdkError<DescribeGlobalClustersError>>>;
    fn describe_integrations(&self, builder: DescribeIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeIntegrationsOutput, SdkError<DescribeIntegrationsError>>>;
    fn describe_option_group_options(&self, builder: DescribeOptionGroupOptionsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupOptionsOutput, SdkError<DescribeOptionGroupOptionsError>>>;
    fn describe_option_groups(&self, builder: DescribeOptionGroupsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupsOutput, SdkError<DescribeOptionGroupsError>>>;
    fn describe_orderable_db_instance_options(&self, builder: DescribeOrderableDbInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableDbInstanceOptionsOutput, SdkError<DescribeOrderableDBInstanceOptionsError>>>;
    fn describe_pending_maintenance_actions(&self, builder: DescribePendingMaintenanceActionsInputBuilder) -> impl Future<Output = Result<DescribePendingMaintenanceActionsOutput, SdkError<DescribePendingMaintenanceActionsError>>>;
    fn describe_reserved_db_instances(&self, builder: DescribeReservedDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOutput, SdkError<DescribeReservedDBInstancesError>>>;
    fn describe_reserved_db_instances_offerings(&self, builder: DescribeReservedDbInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOfferingsOutput, SdkError<DescribeReservedDBInstancesOfferingsError>>>;
    fn describe_source_regions(&self, builder: DescribeSourceRegionsInputBuilder) -> impl Future<Output = Result<DescribeSourceRegionsOutput, SdkError<DescribeSourceRegionsError>>>;
    fn describe_tenant_databases(&self, builder: DescribeTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeTenantDatabasesOutput, SdkError<DescribeTenantDatabasesError>>>;
    fn describe_valid_db_instance_modifications(&self, builder: DescribeValidDbInstanceModificationsInputBuilder) -> impl Future<Output = Result<DescribeValidDbInstanceModificationsOutput, SdkError<DescribeValidDBInstanceModificationsError>>>;
    fn disable_http_endpoint(&self, builder: DisableHttpEndpointInputBuilder) -> impl Future<Output = Result<DisableHttpEndpointOutput, SdkError<DisableHttpEndpointError>>>;
    fn download_db_log_file_portion(&self, builder: DownloadDbLogFilePortionInputBuilder) -> impl Future<Output = Result<DownloadDbLogFilePortionOutput, SdkError<DownloadDBLogFilePortionError>>>;
    fn enable_http_endpoint(&self, builder: EnableHttpEndpointInputBuilder) -> impl Future<Output = Result<EnableHttpEndpointOutput, SdkError<EnableHttpEndpointError>>>;
    fn failover_db_cluster(&self, builder: FailoverDbClusterInputBuilder) -> impl Future<Output = Result<FailoverDbClusterOutput, SdkError<FailoverDBClusterError>>>;
    fn failover_global_cluster(&self, builder: FailoverGlobalClusterInputBuilder) -> impl Future<Output = Result<FailoverGlobalClusterOutput, SdkError<FailoverGlobalClusterError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn modify_activity_stream(&self, builder: ModifyActivityStreamInputBuilder) -> impl Future<Output = Result<ModifyActivityStreamOutput, SdkError<ModifyActivityStreamError>>>;
    fn modify_certificates(&self, builder: ModifyCertificatesInputBuilder) -> impl Future<Output = Result<ModifyCertificatesOutput, SdkError<ModifyCertificatesError>>>;
    fn modify_current_db_cluster_capacity(&self, builder: ModifyCurrentDbClusterCapacityInputBuilder) -> impl Future<Output = Result<ModifyCurrentDbClusterCapacityOutput, SdkError<ModifyCurrentDBClusterCapacityError>>>;
    fn modify_custom_db_engine_version(&self, builder: ModifyCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<ModifyCustomDbEngineVersionOutput, SdkError<ModifyCustomDBEngineVersionError>>>;
    fn modify_db_cluster(&self, builder: ModifyDbClusterInputBuilder) -> impl Future<Output = Result<ModifyDbClusterOutput, SdkError<ModifyDBClusterError>>>;
    fn modify_db_cluster_endpoint(&self, builder: ModifyDbClusterEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbClusterEndpointOutput, SdkError<ModifyDBClusterEndpointError>>>;
    fn modify_db_cluster_parameter_group(&self, builder: ModifyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbClusterParameterGroupOutput, SdkError<ModifyDBClusterParameterGroupError>>>;
    fn modify_db_cluster_snapshot_attribute(&self, builder: ModifyDbClusterSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbClusterSnapshotAttributeOutput, SdkError<ModifyDBClusterSnapshotAttributeError>>>;
    fn modify_db_instance(&self, builder: ModifyDbInstanceInputBuilder) -> impl Future<Output = Result<ModifyDbInstanceOutput, SdkError<ModifyDBInstanceError>>>;
    fn modify_db_parameter_group(&self, builder: ModifyDbParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbParameterGroupOutput, SdkError<ModifyDBParameterGroupError>>>;
    fn modify_db_proxy(&self, builder: ModifyDbProxyInputBuilder) -> impl Future<Output = Result<ModifyDbProxyOutput, SdkError<ModifyDBProxyError>>>;
    fn modify_db_proxy_endpoint(&self, builder: ModifyDbProxyEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbProxyEndpointOutput, SdkError<ModifyDBProxyEndpointError>>>;
    fn modify_db_proxy_target_group(&self, builder: ModifyDbProxyTargetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbProxyTargetGroupOutput, SdkError<ModifyDBProxyTargetGroupError>>>;
    fn modify_db_recommendation(&self, builder: ModifyDbRecommendationInputBuilder) -> impl Future<Output = Result<ModifyDbRecommendationOutput, SdkError<ModifyDBRecommendationError>>>;
    fn modify_db_shard_group(&self, builder: ModifyDbShardGroupInputBuilder) -> impl Future<Output = Result<ModifyDbShardGroupOutput, SdkError<ModifyDBShardGroupError>>>;
    fn modify_db_snapshot(&self, builder: ModifyDbSnapshotInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotOutput, SdkError<ModifyDBSnapshotError>>>;
    fn modify_db_snapshot_attribute(&self, builder: ModifyDbSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotAttributeOutput, SdkError<ModifyDBSnapshotAttributeError>>>;
    fn modify_db_subnet_group(&self, builder: ModifyDbSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbSubnetGroupOutput, SdkError<ModifyDBSubnetGroupError>>>;
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>>;
    fn modify_global_cluster(&self, builder: ModifyGlobalClusterInputBuilder) -> impl Future<Output = Result<ModifyGlobalClusterOutput, SdkError<ModifyGlobalClusterError>>>;
    fn modify_integration(&self, builder: ModifyIntegrationInputBuilder) -> impl Future<Output = Result<ModifyIntegrationOutput, SdkError<ModifyIntegrationError>>>;
    fn modify_tenant_database(&self, builder: ModifyTenantDatabaseInputBuilder) -> impl Future<Output = Result<ModifyTenantDatabaseOutput, SdkError<ModifyTenantDatabaseError>>>;
    fn promote_read_replica(&self, builder: PromoteReadReplicaInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaOutput, SdkError<PromoteReadReplicaError>>>;
    fn promote_read_replica_db_cluster(&self, builder: PromoteReadReplicaDbClusterInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaDbClusterOutput, SdkError<PromoteReadReplicaDBClusterError>>>;
    fn purchase_reserved_db_instances_offering(&self, builder: PurchaseReservedDbInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedDbInstancesOfferingOutput, SdkError<PurchaseReservedDBInstancesOfferingError>>>;
    fn reboot_db_cluster(&self, builder: RebootDbClusterInputBuilder) -> impl Future<Output = Result<RebootDbClusterOutput, SdkError<RebootDBClusterError>>>;
    fn reboot_db_instance(&self, builder: RebootDbInstanceInputBuilder) -> impl Future<Output = Result<RebootDbInstanceOutput, SdkError<RebootDBInstanceError>>>;
    fn reboot_db_shard_group(&self, builder: RebootDbShardGroupInputBuilder) -> impl Future<Output = Result<RebootDbShardGroupOutput, SdkError<RebootDBShardGroupError>>>;
    fn register_db_proxy_targets(&self, builder: RegisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<RegisterDbProxyTargetsOutput, SdkError<RegisterDBProxyTargetsError>>>;
    fn remove_from_global_cluster(&self, builder: RemoveFromGlobalClusterInputBuilder) -> impl Future<Output = Result<RemoveFromGlobalClusterOutput, SdkError<RemoveFromGlobalClusterError>>>;
    fn remove_role_from_db_cluster(&self, builder: RemoveRoleFromDbClusterInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbClusterOutput, SdkError<RemoveRoleFromDBClusterError>>>;
    fn remove_role_from_db_instance(&self, builder: RemoveRoleFromDbInstanceInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbInstanceOutput, SdkError<RemoveRoleFromDBInstanceError>>>;
    fn remove_source_identifier_from_subscription(&self, builder: RemoveSourceIdentifierFromSubscriptionInputBuilder) -> impl Future<Output = Result<RemoveSourceIdentifierFromSubscriptionOutput, SdkError<RemoveSourceIdentifierFromSubscriptionError>>>;
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>>;
    fn reset_db_cluster_parameter_group(&self, builder: ResetDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbClusterParameterGroupOutput, SdkError<ResetDBClusterParameterGroupError>>>;
    fn reset_db_parameter_group(&self, builder: ResetDbParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbParameterGroupOutput, SdkError<ResetDBParameterGroupError>>>;
    fn restore_db_cluster_from_s3(&self, builder: RestoreDbClusterFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromS3Output, SdkError<RestoreDBClusterFromS3Error>>>;
    fn restore_db_cluster_from_snapshot(&self, builder: RestoreDbClusterFromSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromSnapshotOutput, SdkError<RestoreDBClusterFromSnapshotError>>>;
    fn restore_db_cluster_to_point_in_time(&self, builder: RestoreDbClusterToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbClusterToPointInTimeOutput, SdkError<RestoreDBClusterToPointInTimeError>>>;
    fn restore_db_instance_from_db_snapshot(&self, builder: RestoreDbInstanceFromDbSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromDbSnapshotOutput, SdkError<RestoreDBInstanceFromDBSnapshotError>>>;
    fn restore_db_instance_from_s3(&self, builder: RestoreDbInstanceFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromS3Output, SdkError<RestoreDBInstanceFromS3Error>>>;
    fn restore_db_instance_to_point_in_time(&self, builder: RestoreDbInstanceToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceToPointInTimeOutput, SdkError<RestoreDBInstanceToPointInTimeError>>>;
    fn revoke_db_security_group_ingress(&self, builder: RevokeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeDbSecurityGroupIngressOutput, SdkError<RevokeDBSecurityGroupIngressError>>>;
    fn start_activity_stream(&self, builder: StartActivityStreamInputBuilder) -> impl Future<Output = Result<StartActivityStreamOutput, SdkError<StartActivityStreamError>>>;
    fn start_db_cluster(&self, builder: StartDbClusterInputBuilder) -> impl Future<Output = Result<StartDbClusterOutput, SdkError<StartDBClusterError>>>;
    fn start_db_instance(&self, builder: StartDbInstanceInputBuilder) -> impl Future<Output = Result<StartDbInstanceOutput, SdkError<StartDBInstanceError>>>;
    fn start_db_instance_automated_backups_replication(&self, builder: StartDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StartDbInstanceAutomatedBackupsReplicationOutput, SdkError<StartDBInstanceAutomatedBackupsReplicationError>>>;
    fn start_export_task(&self, builder: StartExportTaskInputBuilder) -> impl Future<Output = Result<StartExportTaskOutput, SdkError<StartExportTaskError>>>;
    fn stop_activity_stream(&self, builder: StopActivityStreamInputBuilder) -> impl Future<Output = Result<StopActivityStreamOutput, SdkError<StopActivityStreamError>>>;
    fn stop_db_cluster(&self, builder: StopDbClusterInputBuilder) -> impl Future<Output = Result<StopDbClusterOutput, SdkError<StopDBClusterError>>>;
    fn stop_db_instance(&self, builder: StopDbInstanceInputBuilder) -> impl Future<Output = Result<StopDbInstanceOutput, SdkError<StopDBInstanceError>>>;
    fn stop_db_instance_automated_backups_replication(&self, builder: StopDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StopDbInstanceAutomatedBackupsReplicationOutput, SdkError<StopDBInstanceAutomatedBackupsReplicationError>>>;
    fn switchover_blue_green_deployment(&self, builder: SwitchoverBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<SwitchoverBlueGreenDeploymentOutput, SdkError<SwitchoverBlueGreenDeploymentError>>>;
    fn switchover_global_cluster(&self, builder: SwitchoverGlobalClusterInputBuilder) -> impl Future<Output = Result<SwitchoverGlobalClusterOutput, SdkError<SwitchoverGlobalClusterError>>>;
    fn switchover_read_replica(&self, builder: SwitchoverReadReplicaInputBuilder) -> impl Future<Output = Result<SwitchoverReadReplicaOutput, SdkError<SwitchoverReadReplicaError>>>;
}
impl RDSClient for RDSClientImpl {
    fn add_role_to_db_cluster(&self, builder: AddRoleToDbClusterInputBuilder) -> impl Future<Output = Result<AddRoleToDbClusterOutput, SdkError<AddRoleToDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn add_role_to_db_instance(&self, builder: AddRoleToDbInstanceInputBuilder) -> impl Future<Output = Result<AddRoleToDbInstanceOutput, SdkError<AddRoleToDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn add_source_identifier_to_subscription(&self, builder: AddSourceIdentifierToSubscriptionInputBuilder) -> impl Future<Output = Result<AddSourceIdentifierToSubscriptionOutput, SdkError<AddSourceIdentifierToSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        builder.send_with(&self.0)
    }
    fn apply_pending_maintenance_action(&self, builder: ApplyPendingMaintenanceActionInputBuilder) -> impl Future<Output = Result<ApplyPendingMaintenanceActionOutput, SdkError<ApplyPendingMaintenanceActionError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_db_security_group_ingress(&self, builder: AuthorizeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeDbSecurityGroupIngressOutput, SdkError<AuthorizeDBSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn backtrack_db_cluster(&self, builder: BacktrackDbClusterInputBuilder) -> impl Future<Output = Result<BacktrackDbClusterOutput, SdkError<BacktrackDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>> {
        builder.send_with(&self.0)
    }
    fn copy_db_cluster_parameter_group(&self, builder: CopyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbClusterParameterGroupOutput, SdkError<CopyDBClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn copy_db_cluster_snapshot(&self, builder: CopyDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbClusterSnapshotOutput, SdkError<CopyDBClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn copy_db_parameter_group(&self, builder: CopyDbParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbParameterGroupOutput, SdkError<CopyDBParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn copy_db_snapshot(&self, builder: CopyDbSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbSnapshotOutput, SdkError<CopyDBSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn copy_option_group(&self, builder: CopyOptionGroupInputBuilder) -> impl Future<Output = Result<CopyOptionGroupOutput, SdkError<CopyOptionGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_blue_green_deployment(&self, builder: CreateBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<CreateBlueGreenDeploymentOutput, SdkError<CreateBlueGreenDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_db_engine_version(&self, builder: CreateCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<CreateCustomDbEngineVersionOutput, SdkError<CreateCustomDBEngineVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_cluster(&self, builder: CreateDbClusterInputBuilder) -> impl Future<Output = Result<CreateDbClusterOutput, SdkError<CreateDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_cluster_endpoint(&self, builder: CreateDbClusterEndpointInputBuilder) -> impl Future<Output = Result<CreateDbClusterEndpointOutput, SdkError<CreateDBClusterEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_cluster_parameter_group(&self, builder: CreateDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbClusterParameterGroupOutput, SdkError<CreateDBClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_cluster_snapshot(&self, builder: CreateDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbClusterSnapshotOutput, SdkError<CreateDBClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_instance(&self, builder: CreateDbInstanceInputBuilder) -> impl Future<Output = Result<CreateDbInstanceOutput, SdkError<CreateDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_instance_read_replica(&self, builder: CreateDbInstanceReadReplicaInputBuilder) -> impl Future<Output = Result<CreateDbInstanceReadReplicaOutput, SdkError<CreateDBInstanceReadReplicaError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_parameter_group(&self, builder: CreateDbParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbParameterGroupOutput, SdkError<CreateDBParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_proxy(&self, builder: CreateDbProxyInputBuilder) -> impl Future<Output = Result<CreateDbProxyOutput, SdkError<CreateDBProxyError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_proxy_endpoint(&self, builder: CreateDbProxyEndpointInputBuilder) -> impl Future<Output = Result<CreateDbProxyEndpointOutput, SdkError<CreateDBProxyEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_security_group(&self, builder: CreateDbSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateDbSecurityGroupOutput, SdkError<CreateDBSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_shard_group(&self, builder: CreateDbShardGroupInputBuilder) -> impl Future<Output = Result<CreateDbShardGroupOutput, SdkError<CreateDBShardGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_snapshot(&self, builder: CreateDbSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbSnapshotOutput, SdkError<CreateDBSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_db_subnet_group(&self, builder: CreateDbSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateDbSubnetGroupOutput, SdkError<CreateDBSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn create_global_cluster(&self, builder: CreateGlobalClusterInputBuilder) -> impl Future<Output = Result<CreateGlobalClusterOutput, SdkError<CreateGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn create_option_group(&self, builder: CreateOptionGroupInputBuilder) -> impl Future<Output = Result<CreateOptionGroupOutput, SdkError<CreateOptionGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_tenant_database(&self, builder: CreateTenantDatabaseInputBuilder) -> impl Future<Output = Result<CreateTenantDatabaseOutput, SdkError<CreateTenantDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_blue_green_deployment(&self, builder: DeleteBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<DeleteBlueGreenDeploymentOutput, SdkError<DeleteBlueGreenDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_db_engine_version(&self, builder: DeleteCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<DeleteCustomDbEngineVersionOutput, SdkError<DeleteCustomDBEngineVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_cluster(&self, builder: DeleteDbClusterInputBuilder) -> impl Future<Output = Result<DeleteDbClusterOutput, SdkError<DeleteDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_cluster_automated_backup(&self, builder: DeleteDbClusterAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterAutomatedBackupOutput, SdkError<DeleteDBClusterAutomatedBackupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_cluster_endpoint(&self, builder: DeleteDbClusterEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbClusterEndpointOutput, SdkError<DeleteDBClusterEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_cluster_parameter_group(&self, builder: DeleteDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterParameterGroupOutput, SdkError<DeleteDBClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_cluster_snapshot(&self, builder: DeleteDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbClusterSnapshotOutput, SdkError<DeleteDBClusterSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_instance(&self, builder: DeleteDbInstanceInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceOutput, SdkError<DeleteDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_instance_automated_backup(&self, builder: DeleteDbInstanceAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceAutomatedBackupOutput, SdkError<DeleteDBInstanceAutomatedBackupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_parameter_group(&self, builder: DeleteDbParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbParameterGroupOutput, SdkError<DeleteDBParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_proxy(&self, builder: DeleteDbProxyInputBuilder) -> impl Future<Output = Result<DeleteDbProxyOutput, SdkError<DeleteDBProxyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_proxy_endpoint(&self, builder: DeleteDbProxyEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbProxyEndpointOutput, SdkError<DeleteDBProxyEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_security_group(&self, builder: DeleteDbSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSecurityGroupOutput, SdkError<DeleteDBSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_shard_group(&self, builder: DeleteDbShardGroupInputBuilder) -> impl Future<Output = Result<DeleteDbShardGroupOutput, SdkError<DeleteDBShardGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_snapshot(&self, builder: DeleteDbSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbSnapshotOutput, SdkError<DeleteDBSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_db_subnet_group(&self, builder: DeleteDbSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSubnetGroupOutput, SdkError<DeleteDBSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_global_cluster(&self, builder: DeleteGlobalClusterInputBuilder) -> impl Future<Output = Result<DeleteGlobalClusterOutput, SdkError<DeleteGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_option_group(&self, builder: DeleteOptionGroupInputBuilder) -> impl Future<Output = Result<DeleteOptionGroupOutput, SdkError<DeleteOptionGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_tenant_database(&self, builder: DeleteTenantDatabaseInputBuilder) -> impl Future<Output = Result<DeleteTenantDatabaseOutput, SdkError<DeleteTenantDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_db_proxy_targets(&self, builder: DeregisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DeregisterDbProxyTargetsOutput, SdkError<DeregisterDBProxyTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_blue_green_deployments(&self, builder: DescribeBlueGreenDeploymentsInputBuilder) -> impl Future<Output = Result<DescribeBlueGreenDeploymentsOutput, SdkError<DescribeBlueGreenDeploymentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_certificates(&self, builder: DescribeCertificatesInputBuilder) -> impl Future<Output = Result<DescribeCertificatesOutput, SdkError<DescribeCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_automated_backups(&self, builder: DescribeDbClusterAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterAutomatedBackupsOutput, SdkError<DescribeDBClusterAutomatedBackupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_backtracks(&self, builder: DescribeDbClusterBacktracksInputBuilder) -> impl Future<Output = Result<DescribeDbClusterBacktracksOutput, SdkError<DescribeDBClusterBacktracksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_endpoints(&self, builder: DescribeDbClusterEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterEndpointsOutput, SdkError<DescribeDBClusterEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_parameter_groups(&self, builder: DescribeDbClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParameterGroupsOutput, SdkError<DescribeDBClusterParameterGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_parameters(&self, builder: DescribeDbClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParametersOutput, SdkError<DescribeDBClusterParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_snapshot_attributes(&self, builder: DescribeDbClusterSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotAttributesOutput, SdkError<DescribeDBClusterSnapshotAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_cluster_snapshots(&self, builder: DescribeDbClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotsOutput, SdkError<DescribeDBClusterSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_clusters(&self, builder: DescribeDbClustersInputBuilder) -> impl Future<Output = Result<DescribeDbClustersOutput, SdkError<DescribeDBClustersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_engine_versions(&self, builder: DescribeDbEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeDbEngineVersionsOutput, SdkError<DescribeDBEngineVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_instance_automated_backups(&self, builder: DescribeDbInstanceAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbInstanceAutomatedBackupsOutput, SdkError<DescribeDBInstanceAutomatedBackupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_instances(&self, builder: DescribeDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeDbInstancesOutput, SdkError<DescribeDBInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_log_files(&self, builder: DescribeDbLogFilesInputBuilder) -> impl Future<Output = Result<DescribeDbLogFilesOutput, SdkError<DescribeDBLogFilesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_parameter_groups(&self, builder: DescribeDbParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbParameterGroupsOutput, SdkError<DescribeDBParameterGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_parameters(&self, builder: DescribeDbParametersInputBuilder) -> impl Future<Output = Result<DescribeDbParametersOutput, SdkError<DescribeDBParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_proxies(&self, builder: DescribeDbProxiesInputBuilder) -> impl Future<Output = Result<DescribeDbProxiesOutput, SdkError<DescribeDBProxiesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_proxy_endpoints(&self, builder: DescribeDbProxyEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyEndpointsOutput, SdkError<DescribeDBProxyEndpointsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_proxy_target_groups(&self, builder: DescribeDbProxyTargetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetGroupsOutput, SdkError<DescribeDBProxyTargetGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_proxy_targets(&self, builder: DescribeDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetsOutput, SdkError<DescribeDBProxyTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_recommendations(&self, builder: DescribeDbRecommendationsInputBuilder) -> impl Future<Output = Result<DescribeDbRecommendationsOutput, SdkError<DescribeDBRecommendationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_security_groups(&self, builder: DescribeDbSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSecurityGroupsOutput, SdkError<DescribeDBSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_shard_groups(&self, builder: DescribeDbShardGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbShardGroupsOutput, SdkError<DescribeDBShardGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_snapshot_attributes(&self, builder: DescribeDbSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotAttributesOutput, SdkError<DescribeDBSnapshotAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_snapshot_tenant_databases(&self, builder: DescribeDbSnapshotTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotTenantDatabasesOutput, SdkError<DescribeDBSnapshotTenantDatabasesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_snapshots(&self, builder: DescribeDbSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotsOutput, SdkError<DescribeDBSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_db_subnet_groups(&self, builder: DescribeDbSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSubnetGroupsOutput, SdkError<DescribeDBSubnetGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_engine_default_cluster_parameters(&self, builder: DescribeEngineDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultClusterParametersOutput, SdkError<DescribeEngineDefaultClusterParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>> {
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
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_global_clusters(&self, builder: DescribeGlobalClustersInputBuilder) -> impl Future<Output = Result<DescribeGlobalClustersOutput, SdkError<DescribeGlobalClustersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_integrations(&self, builder: DescribeIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeIntegrationsOutput, SdkError<DescribeIntegrationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_option_group_options(&self, builder: DescribeOptionGroupOptionsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupOptionsOutput, SdkError<DescribeOptionGroupOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_option_groups(&self, builder: DescribeOptionGroupsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupsOutput, SdkError<DescribeOptionGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_orderable_db_instance_options(&self, builder: DescribeOrderableDbInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableDbInstanceOptionsOutput, SdkError<DescribeOrderableDBInstanceOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pending_maintenance_actions(&self, builder: DescribePendingMaintenanceActionsInputBuilder) -> impl Future<Output = Result<DescribePendingMaintenanceActionsOutput, SdkError<DescribePendingMaintenanceActionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_db_instances(&self, builder: DescribeReservedDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOutput, SdkError<DescribeReservedDBInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_db_instances_offerings(&self, builder: DescribeReservedDbInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOfferingsOutput, SdkError<DescribeReservedDBInstancesOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_source_regions(&self, builder: DescribeSourceRegionsInputBuilder) -> impl Future<Output = Result<DescribeSourceRegionsOutput, SdkError<DescribeSourceRegionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_tenant_databases(&self, builder: DescribeTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeTenantDatabasesOutput, SdkError<DescribeTenantDatabasesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_valid_db_instance_modifications(&self, builder: DescribeValidDbInstanceModificationsInputBuilder) -> impl Future<Output = Result<DescribeValidDbInstanceModificationsOutput, SdkError<DescribeValidDBInstanceModificationsError>>> {
        builder.send_with(&self.0)
    }
    fn disable_http_endpoint(&self, builder: DisableHttpEndpointInputBuilder) -> impl Future<Output = Result<DisableHttpEndpointOutput, SdkError<DisableHttpEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn download_db_log_file_portion(&self, builder: DownloadDbLogFilePortionInputBuilder) -> impl Future<Output = Result<DownloadDbLogFilePortionOutput, SdkError<DownloadDBLogFilePortionError>>> {
        builder.send_with(&self.0)
    }
    fn enable_http_endpoint(&self, builder: EnableHttpEndpointInputBuilder) -> impl Future<Output = Result<EnableHttpEndpointOutput, SdkError<EnableHttpEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn failover_db_cluster(&self, builder: FailoverDbClusterInputBuilder) -> impl Future<Output = Result<FailoverDbClusterOutput, SdkError<FailoverDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn failover_global_cluster(&self, builder: FailoverGlobalClusterInputBuilder) -> impl Future<Output = Result<FailoverGlobalClusterOutput, SdkError<FailoverGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_activity_stream(&self, builder: ModifyActivityStreamInputBuilder) -> impl Future<Output = Result<ModifyActivityStreamOutput, SdkError<ModifyActivityStreamError>>> {
        builder.send_with(&self.0)
    }
    fn modify_certificates(&self, builder: ModifyCertificatesInputBuilder) -> impl Future<Output = Result<ModifyCertificatesOutput, SdkError<ModifyCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_current_db_cluster_capacity(&self, builder: ModifyCurrentDbClusterCapacityInputBuilder) -> impl Future<Output = Result<ModifyCurrentDbClusterCapacityOutput, SdkError<ModifyCurrentDBClusterCapacityError>>> {
        builder.send_with(&self.0)
    }
    fn modify_custom_db_engine_version(&self, builder: ModifyCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<ModifyCustomDbEngineVersionOutput, SdkError<ModifyCustomDBEngineVersionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_cluster(&self, builder: ModifyDbClusterInputBuilder) -> impl Future<Output = Result<ModifyDbClusterOutput, SdkError<ModifyDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_cluster_endpoint(&self, builder: ModifyDbClusterEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbClusterEndpointOutput, SdkError<ModifyDBClusterEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_cluster_parameter_group(&self, builder: ModifyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbClusterParameterGroupOutput, SdkError<ModifyDBClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_cluster_snapshot_attribute(&self, builder: ModifyDbClusterSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbClusterSnapshotAttributeOutput, SdkError<ModifyDBClusterSnapshotAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_instance(&self, builder: ModifyDbInstanceInputBuilder) -> impl Future<Output = Result<ModifyDbInstanceOutput, SdkError<ModifyDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_parameter_group(&self, builder: ModifyDbParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbParameterGroupOutput, SdkError<ModifyDBParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_proxy(&self, builder: ModifyDbProxyInputBuilder) -> impl Future<Output = Result<ModifyDbProxyOutput, SdkError<ModifyDBProxyError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_proxy_endpoint(&self, builder: ModifyDbProxyEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbProxyEndpointOutput, SdkError<ModifyDBProxyEndpointError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_proxy_target_group(&self, builder: ModifyDbProxyTargetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbProxyTargetGroupOutput, SdkError<ModifyDBProxyTargetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_recommendation(&self, builder: ModifyDbRecommendationInputBuilder) -> impl Future<Output = Result<ModifyDbRecommendationOutput, SdkError<ModifyDBRecommendationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_shard_group(&self, builder: ModifyDbShardGroupInputBuilder) -> impl Future<Output = Result<ModifyDbShardGroupOutput, SdkError<ModifyDBShardGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_snapshot(&self, builder: ModifyDbSnapshotInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotOutput, SdkError<ModifyDBSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_snapshot_attribute(&self, builder: ModifyDbSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotAttributeOutput, SdkError<ModifyDBSnapshotAttributeError>>> {
        builder.send_with(&self.0)
    }
    fn modify_db_subnet_group(&self, builder: ModifyDbSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbSubnetGroupOutput, SdkError<ModifyDBSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn modify_global_cluster(&self, builder: ModifyGlobalClusterInputBuilder) -> impl Future<Output = Result<ModifyGlobalClusterOutput, SdkError<ModifyGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn modify_integration(&self, builder: ModifyIntegrationInputBuilder) -> impl Future<Output = Result<ModifyIntegrationOutput, SdkError<ModifyIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_tenant_database(&self, builder: ModifyTenantDatabaseInputBuilder) -> impl Future<Output = Result<ModifyTenantDatabaseOutput, SdkError<ModifyTenantDatabaseError>>> {
        builder.send_with(&self.0)
    }
    fn promote_read_replica(&self, builder: PromoteReadReplicaInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaOutput, SdkError<PromoteReadReplicaError>>> {
        builder.send_with(&self.0)
    }
    fn promote_read_replica_db_cluster(&self, builder: PromoteReadReplicaDbClusterInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaDbClusterOutput, SdkError<PromoteReadReplicaDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_reserved_db_instances_offering(&self, builder: PurchaseReservedDbInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedDbInstancesOfferingOutput, SdkError<PurchaseReservedDBInstancesOfferingError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_db_cluster(&self, builder: RebootDbClusterInputBuilder) -> impl Future<Output = Result<RebootDbClusterOutput, SdkError<RebootDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_db_instance(&self, builder: RebootDbInstanceInputBuilder) -> impl Future<Output = Result<RebootDbInstanceOutput, SdkError<RebootDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_db_shard_group(&self, builder: RebootDbShardGroupInputBuilder) -> impl Future<Output = Result<RebootDbShardGroupOutput, SdkError<RebootDBShardGroupError>>> {
        builder.send_with(&self.0)
    }
    fn register_db_proxy_targets(&self, builder: RegisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<RegisterDbProxyTargetsOutput, SdkError<RegisterDBProxyTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn remove_from_global_cluster(&self, builder: RemoveFromGlobalClusterInputBuilder) -> impl Future<Output = Result<RemoveFromGlobalClusterOutput, SdkError<RemoveFromGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn remove_role_from_db_cluster(&self, builder: RemoveRoleFromDbClusterInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbClusterOutput, SdkError<RemoveRoleFromDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn remove_role_from_db_instance(&self, builder: RemoveRoleFromDbInstanceInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbInstanceOutput, SdkError<RemoveRoleFromDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn remove_source_identifier_from_subscription(&self, builder: RemoveSourceIdentifierFromSubscriptionInputBuilder) -> impl Future<Output = Result<RemoveSourceIdentifierFromSubscriptionOutput, SdkError<RemoveSourceIdentifierFromSubscriptionError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        builder.send_with(&self.0)
    }
    fn reset_db_cluster_parameter_group(&self, builder: ResetDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbClusterParameterGroupOutput, SdkError<ResetDBClusterParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn reset_db_parameter_group(&self, builder: ResetDbParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbParameterGroupOutput, SdkError<ResetDBParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_cluster_from_s3(&self, builder: RestoreDbClusterFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromS3Output, SdkError<RestoreDBClusterFromS3Error>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_cluster_from_snapshot(&self, builder: RestoreDbClusterFromSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromSnapshotOutput, SdkError<RestoreDBClusterFromSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_cluster_to_point_in_time(&self, builder: RestoreDbClusterToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbClusterToPointInTimeOutput, SdkError<RestoreDBClusterToPointInTimeError>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_instance_from_db_snapshot(&self, builder: RestoreDbInstanceFromDbSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromDbSnapshotOutput, SdkError<RestoreDBInstanceFromDBSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_instance_from_s3(&self, builder: RestoreDbInstanceFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromS3Output, SdkError<RestoreDBInstanceFromS3Error>>> {
        builder.send_with(&self.0)
    }
    fn restore_db_instance_to_point_in_time(&self, builder: RestoreDbInstanceToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceToPointInTimeOutput, SdkError<RestoreDBInstanceToPointInTimeError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_db_security_group_ingress(&self, builder: RevokeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeDbSecurityGroupIngressOutput, SdkError<RevokeDBSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn start_activity_stream(&self, builder: StartActivityStreamInputBuilder) -> impl Future<Output = Result<StartActivityStreamOutput, SdkError<StartActivityStreamError>>> {
        builder.send_with(&self.0)
    }
    fn start_db_cluster(&self, builder: StartDbClusterInputBuilder) -> impl Future<Output = Result<StartDbClusterOutput, SdkError<StartDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn start_db_instance(&self, builder: StartDbInstanceInputBuilder) -> impl Future<Output = Result<StartDbInstanceOutput, SdkError<StartDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn start_db_instance_automated_backups_replication(&self, builder: StartDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StartDbInstanceAutomatedBackupsReplicationOutput, SdkError<StartDBInstanceAutomatedBackupsReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn start_export_task(&self, builder: StartExportTaskInputBuilder) -> impl Future<Output = Result<StartExportTaskOutput, SdkError<StartExportTaskError>>> {
        builder.send_with(&self.0)
    }
    fn stop_activity_stream(&self, builder: StopActivityStreamInputBuilder) -> impl Future<Output = Result<StopActivityStreamOutput, SdkError<StopActivityStreamError>>> {
        builder.send_with(&self.0)
    }
    fn stop_db_cluster(&self, builder: StopDbClusterInputBuilder) -> impl Future<Output = Result<StopDbClusterOutput, SdkError<StopDBClusterError>>> {
        builder.send_with(&self.0)
    }
    fn stop_db_instance(&self, builder: StopDbInstanceInputBuilder) -> impl Future<Output = Result<StopDbInstanceOutput, SdkError<StopDBInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn stop_db_instance_automated_backups_replication(&self, builder: StopDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StopDbInstanceAutomatedBackupsReplicationOutput, SdkError<StopDBInstanceAutomatedBackupsReplicationError>>> {
        builder.send_with(&self.0)
    }
    fn switchover_blue_green_deployment(&self, builder: SwitchoverBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<SwitchoverBlueGreenDeploymentOutput, SdkError<SwitchoverBlueGreenDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn switchover_global_cluster(&self, builder: SwitchoverGlobalClusterInputBuilder) -> impl Future<Output = Result<SwitchoverGlobalClusterOutput, SdkError<SwitchoverGlobalClusterError>>> {
        builder.send_with(&self.0)
    }
    fn switchover_read_replica(&self, builder: SwitchoverReadReplicaInputBuilder) -> impl Future<Output = Result<SwitchoverReadReplicaOutput, SdkError<SwitchoverReadReplicaError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> RDSClient for T
where T: Deref,
      T::Target: RDSClient {
    fn add_role_to_db_cluster(&self, builder: AddRoleToDbClusterInputBuilder) -> impl Future<Output = Result<AddRoleToDbClusterOutput, SdkError<AddRoleToDBClusterError>>> {
        self.deref().add_role_to_db_cluster(builder)
    }
    fn add_role_to_db_instance(&self, builder: AddRoleToDbInstanceInputBuilder) -> impl Future<Output = Result<AddRoleToDbInstanceOutput, SdkError<AddRoleToDBInstanceError>>> {
        self.deref().add_role_to_db_instance(builder)
    }
    fn add_source_identifier_to_subscription(&self, builder: AddSourceIdentifierToSubscriptionInputBuilder) -> impl Future<Output = Result<AddSourceIdentifierToSubscriptionOutput, SdkError<AddSourceIdentifierToSubscriptionError>>> {
        self.deref().add_source_identifier_to_subscription(builder)
    }
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        self.deref().add_tags_to_resource(builder)
    }
    fn apply_pending_maintenance_action(&self, builder: ApplyPendingMaintenanceActionInputBuilder) -> impl Future<Output = Result<ApplyPendingMaintenanceActionOutput, SdkError<ApplyPendingMaintenanceActionError>>> {
        self.deref().apply_pending_maintenance_action(builder)
    }
    fn authorize_db_security_group_ingress(&self, builder: AuthorizeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeDbSecurityGroupIngressOutput, SdkError<AuthorizeDBSecurityGroupIngressError>>> {
        self.deref().authorize_db_security_group_ingress(builder)
    }
    fn backtrack_db_cluster(&self, builder: BacktrackDbClusterInputBuilder) -> impl Future<Output = Result<BacktrackDbClusterOutput, SdkError<BacktrackDBClusterError>>> {
        self.deref().backtrack_db_cluster(builder)
    }
    fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> impl Future<Output = Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>> {
        self.deref().cancel_export_task(builder)
    }
    fn copy_db_cluster_parameter_group(&self, builder: CopyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbClusterParameterGroupOutput, SdkError<CopyDBClusterParameterGroupError>>> {
        self.deref().copy_db_cluster_parameter_group(builder)
    }
    fn copy_db_cluster_snapshot(&self, builder: CopyDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbClusterSnapshotOutput, SdkError<CopyDBClusterSnapshotError>>> {
        self.deref().copy_db_cluster_snapshot(builder)
    }
    fn copy_db_parameter_group(&self, builder: CopyDbParameterGroupInputBuilder) -> impl Future<Output = Result<CopyDbParameterGroupOutput, SdkError<CopyDBParameterGroupError>>> {
        self.deref().copy_db_parameter_group(builder)
    }
    fn copy_db_snapshot(&self, builder: CopyDbSnapshotInputBuilder) -> impl Future<Output = Result<CopyDbSnapshotOutput, SdkError<CopyDBSnapshotError>>> {
        self.deref().copy_db_snapshot(builder)
    }
    fn copy_option_group(&self, builder: CopyOptionGroupInputBuilder) -> impl Future<Output = Result<CopyOptionGroupOutput, SdkError<CopyOptionGroupError>>> {
        self.deref().copy_option_group(builder)
    }
    fn create_blue_green_deployment(&self, builder: CreateBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<CreateBlueGreenDeploymentOutput, SdkError<CreateBlueGreenDeploymentError>>> {
        self.deref().create_blue_green_deployment(builder)
    }
    fn create_custom_db_engine_version(&self, builder: CreateCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<CreateCustomDbEngineVersionOutput, SdkError<CreateCustomDBEngineVersionError>>> {
        self.deref().create_custom_db_engine_version(builder)
    }
    fn create_db_cluster(&self, builder: CreateDbClusterInputBuilder) -> impl Future<Output = Result<CreateDbClusterOutput, SdkError<CreateDBClusterError>>> {
        self.deref().create_db_cluster(builder)
    }
    fn create_db_cluster_endpoint(&self, builder: CreateDbClusterEndpointInputBuilder) -> impl Future<Output = Result<CreateDbClusterEndpointOutput, SdkError<CreateDBClusterEndpointError>>> {
        self.deref().create_db_cluster_endpoint(builder)
    }
    fn create_db_cluster_parameter_group(&self, builder: CreateDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbClusterParameterGroupOutput, SdkError<CreateDBClusterParameterGroupError>>> {
        self.deref().create_db_cluster_parameter_group(builder)
    }
    fn create_db_cluster_snapshot(&self, builder: CreateDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbClusterSnapshotOutput, SdkError<CreateDBClusterSnapshotError>>> {
        self.deref().create_db_cluster_snapshot(builder)
    }
    fn create_db_instance(&self, builder: CreateDbInstanceInputBuilder) -> impl Future<Output = Result<CreateDbInstanceOutput, SdkError<CreateDBInstanceError>>> {
        self.deref().create_db_instance(builder)
    }
    fn create_db_instance_read_replica(&self, builder: CreateDbInstanceReadReplicaInputBuilder) -> impl Future<Output = Result<CreateDbInstanceReadReplicaOutput, SdkError<CreateDBInstanceReadReplicaError>>> {
        self.deref().create_db_instance_read_replica(builder)
    }
    fn create_db_parameter_group(&self, builder: CreateDbParameterGroupInputBuilder) -> impl Future<Output = Result<CreateDbParameterGroupOutput, SdkError<CreateDBParameterGroupError>>> {
        self.deref().create_db_parameter_group(builder)
    }
    fn create_db_proxy(&self, builder: CreateDbProxyInputBuilder) -> impl Future<Output = Result<CreateDbProxyOutput, SdkError<CreateDBProxyError>>> {
        self.deref().create_db_proxy(builder)
    }
    fn create_db_proxy_endpoint(&self, builder: CreateDbProxyEndpointInputBuilder) -> impl Future<Output = Result<CreateDbProxyEndpointOutput, SdkError<CreateDBProxyEndpointError>>> {
        self.deref().create_db_proxy_endpoint(builder)
    }
    fn create_db_security_group(&self, builder: CreateDbSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateDbSecurityGroupOutput, SdkError<CreateDBSecurityGroupError>>> {
        self.deref().create_db_security_group(builder)
    }
    fn create_db_shard_group(&self, builder: CreateDbShardGroupInputBuilder) -> impl Future<Output = Result<CreateDbShardGroupOutput, SdkError<CreateDBShardGroupError>>> {
        self.deref().create_db_shard_group(builder)
    }
    fn create_db_snapshot(&self, builder: CreateDbSnapshotInputBuilder) -> impl Future<Output = Result<CreateDbSnapshotOutput, SdkError<CreateDBSnapshotError>>> {
        self.deref().create_db_snapshot(builder)
    }
    fn create_db_subnet_group(&self, builder: CreateDbSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateDbSubnetGroupOutput, SdkError<CreateDBSubnetGroupError>>> {
        self.deref().create_db_subnet_group(builder)
    }
    fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> impl Future<Output = Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>> {
        self.deref().create_event_subscription(builder)
    }
    fn create_global_cluster(&self, builder: CreateGlobalClusterInputBuilder) -> impl Future<Output = Result<CreateGlobalClusterOutput, SdkError<CreateGlobalClusterError>>> {
        self.deref().create_global_cluster(builder)
    }
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> {
        self.deref().create_integration(builder)
    }
    fn create_option_group(&self, builder: CreateOptionGroupInputBuilder) -> impl Future<Output = Result<CreateOptionGroupOutput, SdkError<CreateOptionGroupError>>> {
        self.deref().create_option_group(builder)
    }
    fn create_tenant_database(&self, builder: CreateTenantDatabaseInputBuilder) -> impl Future<Output = Result<CreateTenantDatabaseOutput, SdkError<CreateTenantDatabaseError>>> {
        self.deref().create_tenant_database(builder)
    }
    fn delete_blue_green_deployment(&self, builder: DeleteBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<DeleteBlueGreenDeploymentOutput, SdkError<DeleteBlueGreenDeploymentError>>> {
        self.deref().delete_blue_green_deployment(builder)
    }
    fn delete_custom_db_engine_version(&self, builder: DeleteCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<DeleteCustomDbEngineVersionOutput, SdkError<DeleteCustomDBEngineVersionError>>> {
        self.deref().delete_custom_db_engine_version(builder)
    }
    fn delete_db_cluster(&self, builder: DeleteDbClusterInputBuilder) -> impl Future<Output = Result<DeleteDbClusterOutput, SdkError<DeleteDBClusterError>>> {
        self.deref().delete_db_cluster(builder)
    }
    fn delete_db_cluster_automated_backup(&self, builder: DeleteDbClusterAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterAutomatedBackupOutput, SdkError<DeleteDBClusterAutomatedBackupError>>> {
        self.deref().delete_db_cluster_automated_backup(builder)
    }
    fn delete_db_cluster_endpoint(&self, builder: DeleteDbClusterEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbClusterEndpointOutput, SdkError<DeleteDBClusterEndpointError>>> {
        self.deref().delete_db_cluster_endpoint(builder)
    }
    fn delete_db_cluster_parameter_group(&self, builder: DeleteDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbClusterParameterGroupOutput, SdkError<DeleteDBClusterParameterGroupError>>> {
        self.deref().delete_db_cluster_parameter_group(builder)
    }
    fn delete_db_cluster_snapshot(&self, builder: DeleteDbClusterSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbClusterSnapshotOutput, SdkError<DeleteDBClusterSnapshotError>>> {
        self.deref().delete_db_cluster_snapshot(builder)
    }
    fn delete_db_instance(&self, builder: DeleteDbInstanceInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceOutput, SdkError<DeleteDBInstanceError>>> {
        self.deref().delete_db_instance(builder)
    }
    fn delete_db_instance_automated_backup(&self, builder: DeleteDbInstanceAutomatedBackupInputBuilder) -> impl Future<Output = Result<DeleteDbInstanceAutomatedBackupOutput, SdkError<DeleteDBInstanceAutomatedBackupError>>> {
        self.deref().delete_db_instance_automated_backup(builder)
    }
    fn delete_db_parameter_group(&self, builder: DeleteDbParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteDbParameterGroupOutput, SdkError<DeleteDBParameterGroupError>>> {
        self.deref().delete_db_parameter_group(builder)
    }
    fn delete_db_proxy(&self, builder: DeleteDbProxyInputBuilder) -> impl Future<Output = Result<DeleteDbProxyOutput, SdkError<DeleteDBProxyError>>> {
        self.deref().delete_db_proxy(builder)
    }
    fn delete_db_proxy_endpoint(&self, builder: DeleteDbProxyEndpointInputBuilder) -> impl Future<Output = Result<DeleteDbProxyEndpointOutput, SdkError<DeleteDBProxyEndpointError>>> {
        self.deref().delete_db_proxy_endpoint(builder)
    }
    fn delete_db_security_group(&self, builder: DeleteDbSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSecurityGroupOutput, SdkError<DeleteDBSecurityGroupError>>> {
        self.deref().delete_db_security_group(builder)
    }
    fn delete_db_shard_group(&self, builder: DeleteDbShardGroupInputBuilder) -> impl Future<Output = Result<DeleteDbShardGroupOutput, SdkError<DeleteDBShardGroupError>>> {
        self.deref().delete_db_shard_group(builder)
    }
    fn delete_db_snapshot(&self, builder: DeleteDbSnapshotInputBuilder) -> impl Future<Output = Result<DeleteDbSnapshotOutput, SdkError<DeleteDBSnapshotError>>> {
        self.deref().delete_db_snapshot(builder)
    }
    fn delete_db_subnet_group(&self, builder: DeleteDbSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteDbSubnetGroupOutput, SdkError<DeleteDBSubnetGroupError>>> {
        self.deref().delete_db_subnet_group(builder)
    }
    fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> impl Future<Output = Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>> {
        self.deref().delete_event_subscription(builder)
    }
    fn delete_global_cluster(&self, builder: DeleteGlobalClusterInputBuilder) -> impl Future<Output = Result<DeleteGlobalClusterOutput, SdkError<DeleteGlobalClusterError>>> {
        self.deref().delete_global_cluster(builder)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        self.deref().delete_integration(builder)
    }
    fn delete_option_group(&self, builder: DeleteOptionGroupInputBuilder) -> impl Future<Output = Result<DeleteOptionGroupOutput, SdkError<DeleteOptionGroupError>>> {
        self.deref().delete_option_group(builder)
    }
    fn delete_tenant_database(&self, builder: DeleteTenantDatabaseInputBuilder) -> impl Future<Output = Result<DeleteTenantDatabaseOutput, SdkError<DeleteTenantDatabaseError>>> {
        self.deref().delete_tenant_database(builder)
    }
    fn deregister_db_proxy_targets(&self, builder: DeregisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DeregisterDbProxyTargetsOutput, SdkError<DeregisterDBProxyTargetsError>>> {
        self.deref().deregister_db_proxy_targets(builder)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        self.deref().describe_account_attributes(builder)
    }
    fn describe_blue_green_deployments(&self, builder: DescribeBlueGreenDeploymentsInputBuilder) -> impl Future<Output = Result<DescribeBlueGreenDeploymentsOutput, SdkError<DescribeBlueGreenDeploymentsError>>> {
        self.deref().describe_blue_green_deployments(builder)
    }
    fn describe_certificates(&self, builder: DescribeCertificatesInputBuilder) -> impl Future<Output = Result<DescribeCertificatesOutput, SdkError<DescribeCertificatesError>>> {
        self.deref().describe_certificates(builder)
    }
    fn describe_db_cluster_automated_backups(&self, builder: DescribeDbClusterAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterAutomatedBackupsOutput, SdkError<DescribeDBClusterAutomatedBackupsError>>> {
        self.deref().describe_db_cluster_automated_backups(builder)
    }
    fn describe_db_cluster_backtracks(&self, builder: DescribeDbClusterBacktracksInputBuilder) -> impl Future<Output = Result<DescribeDbClusterBacktracksOutput, SdkError<DescribeDBClusterBacktracksError>>> {
        self.deref().describe_db_cluster_backtracks(builder)
    }
    fn describe_db_cluster_endpoints(&self, builder: DescribeDbClusterEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterEndpointsOutput, SdkError<DescribeDBClusterEndpointsError>>> {
        self.deref().describe_db_cluster_endpoints(builder)
    }
    fn describe_db_cluster_parameter_groups(&self, builder: DescribeDbClusterParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParameterGroupsOutput, SdkError<DescribeDBClusterParameterGroupsError>>> {
        self.deref().describe_db_cluster_parameter_groups(builder)
    }
    fn describe_db_cluster_parameters(&self, builder: DescribeDbClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeDbClusterParametersOutput, SdkError<DescribeDBClusterParametersError>>> {
        self.deref().describe_db_cluster_parameters(builder)
    }
    fn describe_db_cluster_snapshot_attributes(&self, builder: DescribeDbClusterSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotAttributesOutput, SdkError<DescribeDBClusterSnapshotAttributesError>>> {
        self.deref().describe_db_cluster_snapshot_attributes(builder)
    }
    fn describe_db_cluster_snapshots(&self, builder: DescribeDbClusterSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbClusterSnapshotsOutput, SdkError<DescribeDBClusterSnapshotsError>>> {
        self.deref().describe_db_cluster_snapshots(builder)
    }
    fn describe_db_clusters(&self, builder: DescribeDbClustersInputBuilder) -> impl Future<Output = Result<DescribeDbClustersOutput, SdkError<DescribeDBClustersError>>> {
        self.deref().describe_db_clusters(builder)
    }
    fn describe_db_engine_versions(&self, builder: DescribeDbEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeDbEngineVersionsOutput, SdkError<DescribeDBEngineVersionsError>>> {
        self.deref().describe_db_engine_versions(builder)
    }
    fn describe_db_instance_automated_backups(&self, builder: DescribeDbInstanceAutomatedBackupsInputBuilder) -> impl Future<Output = Result<DescribeDbInstanceAutomatedBackupsOutput, SdkError<DescribeDBInstanceAutomatedBackupsError>>> {
        self.deref().describe_db_instance_automated_backups(builder)
    }
    fn describe_db_instances(&self, builder: DescribeDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeDbInstancesOutput, SdkError<DescribeDBInstancesError>>> {
        self.deref().describe_db_instances(builder)
    }
    fn describe_db_log_files(&self, builder: DescribeDbLogFilesInputBuilder) -> impl Future<Output = Result<DescribeDbLogFilesOutput, SdkError<DescribeDBLogFilesError>>> {
        self.deref().describe_db_log_files(builder)
    }
    fn describe_db_parameter_groups(&self, builder: DescribeDbParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbParameterGroupsOutput, SdkError<DescribeDBParameterGroupsError>>> {
        self.deref().describe_db_parameter_groups(builder)
    }
    fn describe_db_parameters(&self, builder: DescribeDbParametersInputBuilder) -> impl Future<Output = Result<DescribeDbParametersOutput, SdkError<DescribeDBParametersError>>> {
        self.deref().describe_db_parameters(builder)
    }
    fn describe_db_proxies(&self, builder: DescribeDbProxiesInputBuilder) -> impl Future<Output = Result<DescribeDbProxiesOutput, SdkError<DescribeDBProxiesError>>> {
        self.deref().describe_db_proxies(builder)
    }
    fn describe_db_proxy_endpoints(&self, builder: DescribeDbProxyEndpointsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyEndpointsOutput, SdkError<DescribeDBProxyEndpointsError>>> {
        self.deref().describe_db_proxy_endpoints(builder)
    }
    fn describe_db_proxy_target_groups(&self, builder: DescribeDbProxyTargetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetGroupsOutput, SdkError<DescribeDBProxyTargetGroupsError>>> {
        self.deref().describe_db_proxy_target_groups(builder)
    }
    fn describe_db_proxy_targets(&self, builder: DescribeDbProxyTargetsInputBuilder) -> impl Future<Output = Result<DescribeDbProxyTargetsOutput, SdkError<DescribeDBProxyTargetsError>>> {
        self.deref().describe_db_proxy_targets(builder)
    }
    fn describe_db_recommendations(&self, builder: DescribeDbRecommendationsInputBuilder) -> impl Future<Output = Result<DescribeDbRecommendationsOutput, SdkError<DescribeDBRecommendationsError>>> {
        self.deref().describe_db_recommendations(builder)
    }
    fn describe_db_security_groups(&self, builder: DescribeDbSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSecurityGroupsOutput, SdkError<DescribeDBSecurityGroupsError>>> {
        self.deref().describe_db_security_groups(builder)
    }
    fn describe_db_shard_groups(&self, builder: DescribeDbShardGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbShardGroupsOutput, SdkError<DescribeDBShardGroupsError>>> {
        self.deref().describe_db_shard_groups(builder)
    }
    fn describe_db_snapshot_attributes(&self, builder: DescribeDbSnapshotAttributesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotAttributesOutput, SdkError<DescribeDBSnapshotAttributesError>>> {
        self.deref().describe_db_snapshot_attributes(builder)
    }
    fn describe_db_snapshot_tenant_databases(&self, builder: DescribeDbSnapshotTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotTenantDatabasesOutput, SdkError<DescribeDBSnapshotTenantDatabasesError>>> {
        self.deref().describe_db_snapshot_tenant_databases(builder)
    }
    fn describe_db_snapshots(&self, builder: DescribeDbSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeDbSnapshotsOutput, SdkError<DescribeDBSnapshotsError>>> {
        self.deref().describe_db_snapshots(builder)
    }
    fn describe_db_subnet_groups(&self, builder: DescribeDbSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeDbSubnetGroupsOutput, SdkError<DescribeDBSubnetGroupsError>>> {
        self.deref().describe_db_subnet_groups(builder)
    }
    fn describe_engine_default_cluster_parameters(&self, builder: DescribeEngineDefaultClusterParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultClusterParametersOutput, SdkError<DescribeEngineDefaultClusterParametersError>>> {
        self.deref().describe_engine_default_cluster_parameters(builder)
    }
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>> {
        self.deref().describe_engine_default_parameters(builder)
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
    fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> impl Future<Output = Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>> {
        self.deref().describe_export_tasks(builder)
    }
    fn describe_global_clusters(&self, builder: DescribeGlobalClustersInputBuilder) -> impl Future<Output = Result<DescribeGlobalClustersOutput, SdkError<DescribeGlobalClustersError>>> {
        self.deref().describe_global_clusters(builder)
    }
    fn describe_integrations(&self, builder: DescribeIntegrationsInputBuilder) -> impl Future<Output = Result<DescribeIntegrationsOutput, SdkError<DescribeIntegrationsError>>> {
        self.deref().describe_integrations(builder)
    }
    fn describe_option_group_options(&self, builder: DescribeOptionGroupOptionsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupOptionsOutput, SdkError<DescribeOptionGroupOptionsError>>> {
        self.deref().describe_option_group_options(builder)
    }
    fn describe_option_groups(&self, builder: DescribeOptionGroupsInputBuilder) -> impl Future<Output = Result<DescribeOptionGroupsOutput, SdkError<DescribeOptionGroupsError>>> {
        self.deref().describe_option_groups(builder)
    }
    fn describe_orderable_db_instance_options(&self, builder: DescribeOrderableDbInstanceOptionsInputBuilder) -> impl Future<Output = Result<DescribeOrderableDbInstanceOptionsOutput, SdkError<DescribeOrderableDBInstanceOptionsError>>> {
        self.deref().describe_orderable_db_instance_options(builder)
    }
    fn describe_pending_maintenance_actions(&self, builder: DescribePendingMaintenanceActionsInputBuilder) -> impl Future<Output = Result<DescribePendingMaintenanceActionsOutput, SdkError<DescribePendingMaintenanceActionsError>>> {
        self.deref().describe_pending_maintenance_actions(builder)
    }
    fn describe_reserved_db_instances(&self, builder: DescribeReservedDbInstancesInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOutput, SdkError<DescribeReservedDBInstancesError>>> {
        self.deref().describe_reserved_db_instances(builder)
    }
    fn describe_reserved_db_instances_offerings(&self, builder: DescribeReservedDbInstancesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedDbInstancesOfferingsOutput, SdkError<DescribeReservedDBInstancesOfferingsError>>> {
        self.deref().describe_reserved_db_instances_offerings(builder)
    }
    fn describe_source_regions(&self, builder: DescribeSourceRegionsInputBuilder) -> impl Future<Output = Result<DescribeSourceRegionsOutput, SdkError<DescribeSourceRegionsError>>> {
        self.deref().describe_source_regions(builder)
    }
    fn describe_tenant_databases(&self, builder: DescribeTenantDatabasesInputBuilder) -> impl Future<Output = Result<DescribeTenantDatabasesOutput, SdkError<DescribeTenantDatabasesError>>> {
        self.deref().describe_tenant_databases(builder)
    }
    fn describe_valid_db_instance_modifications(&self, builder: DescribeValidDbInstanceModificationsInputBuilder) -> impl Future<Output = Result<DescribeValidDbInstanceModificationsOutput, SdkError<DescribeValidDBInstanceModificationsError>>> {
        self.deref().describe_valid_db_instance_modifications(builder)
    }
    fn disable_http_endpoint(&self, builder: DisableHttpEndpointInputBuilder) -> impl Future<Output = Result<DisableHttpEndpointOutput, SdkError<DisableHttpEndpointError>>> {
        self.deref().disable_http_endpoint(builder)
    }
    fn download_db_log_file_portion(&self, builder: DownloadDbLogFilePortionInputBuilder) -> impl Future<Output = Result<DownloadDbLogFilePortionOutput, SdkError<DownloadDBLogFilePortionError>>> {
        self.deref().download_db_log_file_portion(builder)
    }
    fn enable_http_endpoint(&self, builder: EnableHttpEndpointInputBuilder) -> impl Future<Output = Result<EnableHttpEndpointOutput, SdkError<EnableHttpEndpointError>>> {
        self.deref().enable_http_endpoint(builder)
    }
    fn failover_db_cluster(&self, builder: FailoverDbClusterInputBuilder) -> impl Future<Output = Result<FailoverDbClusterOutput, SdkError<FailoverDBClusterError>>> {
        self.deref().failover_db_cluster(builder)
    }
    fn failover_global_cluster(&self, builder: FailoverGlobalClusterInputBuilder) -> impl Future<Output = Result<FailoverGlobalClusterOutput, SdkError<FailoverGlobalClusterError>>> {
        self.deref().failover_global_cluster(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn modify_activity_stream(&self, builder: ModifyActivityStreamInputBuilder) -> impl Future<Output = Result<ModifyActivityStreamOutput, SdkError<ModifyActivityStreamError>>> {
        self.deref().modify_activity_stream(builder)
    }
    fn modify_certificates(&self, builder: ModifyCertificatesInputBuilder) -> impl Future<Output = Result<ModifyCertificatesOutput, SdkError<ModifyCertificatesError>>> {
        self.deref().modify_certificates(builder)
    }
    fn modify_current_db_cluster_capacity(&self, builder: ModifyCurrentDbClusterCapacityInputBuilder) -> impl Future<Output = Result<ModifyCurrentDbClusterCapacityOutput, SdkError<ModifyCurrentDBClusterCapacityError>>> {
        self.deref().modify_current_db_cluster_capacity(builder)
    }
    fn modify_custom_db_engine_version(&self, builder: ModifyCustomDbEngineVersionInputBuilder) -> impl Future<Output = Result<ModifyCustomDbEngineVersionOutput, SdkError<ModifyCustomDBEngineVersionError>>> {
        self.deref().modify_custom_db_engine_version(builder)
    }
    fn modify_db_cluster(&self, builder: ModifyDbClusterInputBuilder) -> impl Future<Output = Result<ModifyDbClusterOutput, SdkError<ModifyDBClusterError>>> {
        self.deref().modify_db_cluster(builder)
    }
    fn modify_db_cluster_endpoint(&self, builder: ModifyDbClusterEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbClusterEndpointOutput, SdkError<ModifyDBClusterEndpointError>>> {
        self.deref().modify_db_cluster_endpoint(builder)
    }
    fn modify_db_cluster_parameter_group(&self, builder: ModifyDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbClusterParameterGroupOutput, SdkError<ModifyDBClusterParameterGroupError>>> {
        self.deref().modify_db_cluster_parameter_group(builder)
    }
    fn modify_db_cluster_snapshot_attribute(&self, builder: ModifyDbClusterSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbClusterSnapshotAttributeOutput, SdkError<ModifyDBClusterSnapshotAttributeError>>> {
        self.deref().modify_db_cluster_snapshot_attribute(builder)
    }
    fn modify_db_instance(&self, builder: ModifyDbInstanceInputBuilder) -> impl Future<Output = Result<ModifyDbInstanceOutput, SdkError<ModifyDBInstanceError>>> {
        self.deref().modify_db_instance(builder)
    }
    fn modify_db_parameter_group(&self, builder: ModifyDbParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyDbParameterGroupOutput, SdkError<ModifyDBParameterGroupError>>> {
        self.deref().modify_db_parameter_group(builder)
    }
    fn modify_db_proxy(&self, builder: ModifyDbProxyInputBuilder) -> impl Future<Output = Result<ModifyDbProxyOutput, SdkError<ModifyDBProxyError>>> {
        self.deref().modify_db_proxy(builder)
    }
    fn modify_db_proxy_endpoint(&self, builder: ModifyDbProxyEndpointInputBuilder) -> impl Future<Output = Result<ModifyDbProxyEndpointOutput, SdkError<ModifyDBProxyEndpointError>>> {
        self.deref().modify_db_proxy_endpoint(builder)
    }
    fn modify_db_proxy_target_group(&self, builder: ModifyDbProxyTargetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbProxyTargetGroupOutput, SdkError<ModifyDBProxyTargetGroupError>>> {
        self.deref().modify_db_proxy_target_group(builder)
    }
    fn modify_db_recommendation(&self, builder: ModifyDbRecommendationInputBuilder) -> impl Future<Output = Result<ModifyDbRecommendationOutput, SdkError<ModifyDBRecommendationError>>> {
        self.deref().modify_db_recommendation(builder)
    }
    fn modify_db_shard_group(&self, builder: ModifyDbShardGroupInputBuilder) -> impl Future<Output = Result<ModifyDbShardGroupOutput, SdkError<ModifyDBShardGroupError>>> {
        self.deref().modify_db_shard_group(builder)
    }
    fn modify_db_snapshot(&self, builder: ModifyDbSnapshotInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotOutput, SdkError<ModifyDBSnapshotError>>> {
        self.deref().modify_db_snapshot(builder)
    }
    fn modify_db_snapshot_attribute(&self, builder: ModifyDbSnapshotAttributeInputBuilder) -> impl Future<Output = Result<ModifyDbSnapshotAttributeOutput, SdkError<ModifyDBSnapshotAttributeError>>> {
        self.deref().modify_db_snapshot_attribute(builder)
    }
    fn modify_db_subnet_group(&self, builder: ModifyDbSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyDbSubnetGroupOutput, SdkError<ModifyDBSubnetGroupError>>> {
        self.deref().modify_db_subnet_group(builder)
    }
    fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> impl Future<Output = Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>> {
        self.deref().modify_event_subscription(builder)
    }
    fn modify_global_cluster(&self, builder: ModifyGlobalClusterInputBuilder) -> impl Future<Output = Result<ModifyGlobalClusterOutput, SdkError<ModifyGlobalClusterError>>> {
        self.deref().modify_global_cluster(builder)
    }
    fn modify_integration(&self, builder: ModifyIntegrationInputBuilder) -> impl Future<Output = Result<ModifyIntegrationOutput, SdkError<ModifyIntegrationError>>> {
        self.deref().modify_integration(builder)
    }
    fn modify_tenant_database(&self, builder: ModifyTenantDatabaseInputBuilder) -> impl Future<Output = Result<ModifyTenantDatabaseOutput, SdkError<ModifyTenantDatabaseError>>> {
        self.deref().modify_tenant_database(builder)
    }
    fn promote_read_replica(&self, builder: PromoteReadReplicaInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaOutput, SdkError<PromoteReadReplicaError>>> {
        self.deref().promote_read_replica(builder)
    }
    fn promote_read_replica_db_cluster(&self, builder: PromoteReadReplicaDbClusterInputBuilder) -> impl Future<Output = Result<PromoteReadReplicaDbClusterOutput, SdkError<PromoteReadReplicaDBClusterError>>> {
        self.deref().promote_read_replica_db_cluster(builder)
    }
    fn purchase_reserved_db_instances_offering(&self, builder: PurchaseReservedDbInstancesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedDbInstancesOfferingOutput, SdkError<PurchaseReservedDBInstancesOfferingError>>> {
        self.deref().purchase_reserved_db_instances_offering(builder)
    }
    fn reboot_db_cluster(&self, builder: RebootDbClusterInputBuilder) -> impl Future<Output = Result<RebootDbClusterOutput, SdkError<RebootDBClusterError>>> {
        self.deref().reboot_db_cluster(builder)
    }
    fn reboot_db_instance(&self, builder: RebootDbInstanceInputBuilder) -> impl Future<Output = Result<RebootDbInstanceOutput, SdkError<RebootDBInstanceError>>> {
        self.deref().reboot_db_instance(builder)
    }
    fn reboot_db_shard_group(&self, builder: RebootDbShardGroupInputBuilder) -> impl Future<Output = Result<RebootDbShardGroupOutput, SdkError<RebootDBShardGroupError>>> {
        self.deref().reboot_db_shard_group(builder)
    }
    fn register_db_proxy_targets(&self, builder: RegisterDbProxyTargetsInputBuilder) -> impl Future<Output = Result<RegisterDbProxyTargetsOutput, SdkError<RegisterDBProxyTargetsError>>> {
        self.deref().register_db_proxy_targets(builder)
    }
    fn remove_from_global_cluster(&self, builder: RemoveFromGlobalClusterInputBuilder) -> impl Future<Output = Result<RemoveFromGlobalClusterOutput, SdkError<RemoveFromGlobalClusterError>>> {
        self.deref().remove_from_global_cluster(builder)
    }
    fn remove_role_from_db_cluster(&self, builder: RemoveRoleFromDbClusterInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbClusterOutput, SdkError<RemoveRoleFromDBClusterError>>> {
        self.deref().remove_role_from_db_cluster(builder)
    }
    fn remove_role_from_db_instance(&self, builder: RemoveRoleFromDbInstanceInputBuilder) -> impl Future<Output = Result<RemoveRoleFromDbInstanceOutput, SdkError<RemoveRoleFromDBInstanceError>>> {
        self.deref().remove_role_from_db_instance(builder)
    }
    fn remove_source_identifier_from_subscription(&self, builder: RemoveSourceIdentifierFromSubscriptionInputBuilder) -> impl Future<Output = Result<RemoveSourceIdentifierFromSubscriptionOutput, SdkError<RemoveSourceIdentifierFromSubscriptionError>>> {
        self.deref().remove_source_identifier_from_subscription(builder)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        self.deref().remove_tags_from_resource(builder)
    }
    fn reset_db_cluster_parameter_group(&self, builder: ResetDbClusterParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbClusterParameterGroupOutput, SdkError<ResetDBClusterParameterGroupError>>> {
        self.deref().reset_db_cluster_parameter_group(builder)
    }
    fn reset_db_parameter_group(&self, builder: ResetDbParameterGroupInputBuilder) -> impl Future<Output = Result<ResetDbParameterGroupOutput, SdkError<ResetDBParameterGroupError>>> {
        self.deref().reset_db_parameter_group(builder)
    }
    fn restore_db_cluster_from_s3(&self, builder: RestoreDbClusterFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromS3Output, SdkError<RestoreDBClusterFromS3Error>>> {
        self.deref().restore_db_cluster_from_s3(builder)
    }
    fn restore_db_cluster_from_snapshot(&self, builder: RestoreDbClusterFromSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbClusterFromSnapshotOutput, SdkError<RestoreDBClusterFromSnapshotError>>> {
        self.deref().restore_db_cluster_from_snapshot(builder)
    }
    fn restore_db_cluster_to_point_in_time(&self, builder: RestoreDbClusterToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbClusterToPointInTimeOutput, SdkError<RestoreDBClusterToPointInTimeError>>> {
        self.deref().restore_db_cluster_to_point_in_time(builder)
    }
    fn restore_db_instance_from_db_snapshot(&self, builder: RestoreDbInstanceFromDbSnapshotInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromDbSnapshotOutput, SdkError<RestoreDBInstanceFromDBSnapshotError>>> {
        self.deref().restore_db_instance_from_db_snapshot(builder)
    }
    fn restore_db_instance_from_s3(&self, builder: RestoreDbInstanceFromS3InputBuilder) -> impl Future<Output = Result<RestoreDbInstanceFromS3Output, SdkError<RestoreDBInstanceFromS3Error>>> {
        self.deref().restore_db_instance_from_s3(builder)
    }
    fn restore_db_instance_to_point_in_time(&self, builder: RestoreDbInstanceToPointInTimeInputBuilder) -> impl Future<Output = Result<RestoreDbInstanceToPointInTimeOutput, SdkError<RestoreDBInstanceToPointInTimeError>>> {
        self.deref().restore_db_instance_to_point_in_time(builder)
    }
    fn revoke_db_security_group_ingress(&self, builder: RevokeDbSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeDbSecurityGroupIngressOutput, SdkError<RevokeDBSecurityGroupIngressError>>> {
        self.deref().revoke_db_security_group_ingress(builder)
    }
    fn start_activity_stream(&self, builder: StartActivityStreamInputBuilder) -> impl Future<Output = Result<StartActivityStreamOutput, SdkError<StartActivityStreamError>>> {
        self.deref().start_activity_stream(builder)
    }
    fn start_db_cluster(&self, builder: StartDbClusterInputBuilder) -> impl Future<Output = Result<StartDbClusterOutput, SdkError<StartDBClusterError>>> {
        self.deref().start_db_cluster(builder)
    }
    fn start_db_instance(&self, builder: StartDbInstanceInputBuilder) -> impl Future<Output = Result<StartDbInstanceOutput, SdkError<StartDBInstanceError>>> {
        self.deref().start_db_instance(builder)
    }
    fn start_db_instance_automated_backups_replication(&self, builder: StartDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StartDbInstanceAutomatedBackupsReplicationOutput, SdkError<StartDBInstanceAutomatedBackupsReplicationError>>> {
        self.deref().start_db_instance_automated_backups_replication(builder)
    }
    fn start_export_task(&self, builder: StartExportTaskInputBuilder) -> impl Future<Output = Result<StartExportTaskOutput, SdkError<StartExportTaskError>>> {
        self.deref().start_export_task(builder)
    }
    fn stop_activity_stream(&self, builder: StopActivityStreamInputBuilder) -> impl Future<Output = Result<StopActivityStreamOutput, SdkError<StopActivityStreamError>>> {
        self.deref().stop_activity_stream(builder)
    }
    fn stop_db_cluster(&self, builder: StopDbClusterInputBuilder) -> impl Future<Output = Result<StopDbClusterOutput, SdkError<StopDBClusterError>>> {
        self.deref().stop_db_cluster(builder)
    }
    fn stop_db_instance(&self, builder: StopDbInstanceInputBuilder) -> impl Future<Output = Result<StopDbInstanceOutput, SdkError<StopDBInstanceError>>> {
        self.deref().stop_db_instance(builder)
    }
    fn stop_db_instance_automated_backups_replication(&self, builder: StopDbInstanceAutomatedBackupsReplicationInputBuilder) -> impl Future<Output = Result<StopDbInstanceAutomatedBackupsReplicationOutput, SdkError<StopDBInstanceAutomatedBackupsReplicationError>>> {
        self.deref().stop_db_instance_automated_backups_replication(builder)
    }
    fn switchover_blue_green_deployment(&self, builder: SwitchoverBlueGreenDeploymentInputBuilder) -> impl Future<Output = Result<SwitchoverBlueGreenDeploymentOutput, SdkError<SwitchoverBlueGreenDeploymentError>>> {
        self.deref().switchover_blue_green_deployment(builder)
    }
    fn switchover_global_cluster(&self, builder: SwitchoverGlobalClusterInputBuilder) -> impl Future<Output = Result<SwitchoverGlobalClusterOutput, SdkError<SwitchoverGlobalClusterError>>> {
        self.deref().switchover_global_cluster(builder)
    }
    fn switchover_read_replica(&self, builder: SwitchoverReadReplicaInputBuilder) -> impl Future<Output = Result<SwitchoverReadReplicaOutput, SdkError<SwitchoverReadReplicaError>>> {
        self.deref().switchover_read_replica(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edRDSClient {}
    impl RDSClient for edRDSClient {
        async fn add_role_to_db_cluster(&self, builder: AddRoleToDbClusterInputBuilder) -> Result<AddRoleToDbClusterOutput, SdkError<AddRoleToDBClusterError>>;
        async fn add_role_to_db_instance(&self, builder: AddRoleToDbInstanceInputBuilder) -> Result<AddRoleToDbInstanceOutput, SdkError<AddRoleToDBInstanceError>>;
        async fn add_source_identifier_to_subscription(&self, builder: AddSourceIdentifierToSubscriptionInputBuilder) -> Result<AddSourceIdentifierToSubscriptionOutput, SdkError<AddSourceIdentifierToSubscriptionError>>;
        async fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>;
        async fn apply_pending_maintenance_action(&self, builder: ApplyPendingMaintenanceActionInputBuilder) -> Result<ApplyPendingMaintenanceActionOutput, SdkError<ApplyPendingMaintenanceActionError>>;
        async fn authorize_db_security_group_ingress(&self, builder: AuthorizeDbSecurityGroupIngressInputBuilder) -> Result<AuthorizeDbSecurityGroupIngressOutput, SdkError<AuthorizeDBSecurityGroupIngressError>>;
        async fn backtrack_db_cluster(&self, builder: BacktrackDbClusterInputBuilder) -> Result<BacktrackDbClusterOutput, SdkError<BacktrackDBClusterError>>;
        async fn cancel_export_task(&self, builder: CancelExportTaskInputBuilder) -> Result<CancelExportTaskOutput, SdkError<CancelExportTaskError>>;
        async fn copy_db_cluster_parameter_group(&self, builder: CopyDbClusterParameterGroupInputBuilder) -> Result<CopyDbClusterParameterGroupOutput, SdkError<CopyDBClusterParameterGroupError>>;
        async fn copy_db_cluster_snapshot(&self, builder: CopyDbClusterSnapshotInputBuilder) -> Result<CopyDbClusterSnapshotOutput, SdkError<CopyDBClusterSnapshotError>>;
        async fn copy_db_parameter_group(&self, builder: CopyDbParameterGroupInputBuilder) -> Result<CopyDbParameterGroupOutput, SdkError<CopyDBParameterGroupError>>;
        async fn copy_db_snapshot(&self, builder: CopyDbSnapshotInputBuilder) -> Result<CopyDbSnapshotOutput, SdkError<CopyDBSnapshotError>>;
        async fn copy_option_group(&self, builder: CopyOptionGroupInputBuilder) -> Result<CopyOptionGroupOutput, SdkError<CopyOptionGroupError>>;
        async fn create_blue_green_deployment(&self, builder: CreateBlueGreenDeploymentInputBuilder) -> Result<CreateBlueGreenDeploymentOutput, SdkError<CreateBlueGreenDeploymentError>>;
        async fn create_custom_db_engine_version(&self, builder: CreateCustomDbEngineVersionInputBuilder) -> Result<CreateCustomDbEngineVersionOutput, SdkError<CreateCustomDBEngineVersionError>>;
        async fn create_db_cluster(&self, builder: CreateDbClusterInputBuilder) -> Result<CreateDbClusterOutput, SdkError<CreateDBClusterError>>;
        async fn create_db_cluster_endpoint(&self, builder: CreateDbClusterEndpointInputBuilder) -> Result<CreateDbClusterEndpointOutput, SdkError<CreateDBClusterEndpointError>>;
        async fn create_db_cluster_parameter_group(&self, builder: CreateDbClusterParameterGroupInputBuilder) -> Result<CreateDbClusterParameterGroupOutput, SdkError<CreateDBClusterParameterGroupError>>;
        async fn create_db_cluster_snapshot(&self, builder: CreateDbClusterSnapshotInputBuilder) -> Result<CreateDbClusterSnapshotOutput, SdkError<CreateDBClusterSnapshotError>>;
        async fn create_db_instance(&self, builder: CreateDbInstanceInputBuilder) -> Result<CreateDbInstanceOutput, SdkError<CreateDBInstanceError>>;
        async fn create_db_instance_read_replica(&self, builder: CreateDbInstanceReadReplicaInputBuilder) -> Result<CreateDbInstanceReadReplicaOutput, SdkError<CreateDBInstanceReadReplicaError>>;
        async fn create_db_parameter_group(&self, builder: CreateDbParameterGroupInputBuilder) -> Result<CreateDbParameterGroupOutput, SdkError<CreateDBParameterGroupError>>;
        async fn create_db_proxy(&self, builder: CreateDbProxyInputBuilder) -> Result<CreateDbProxyOutput, SdkError<CreateDBProxyError>>;
        async fn create_db_proxy_endpoint(&self, builder: CreateDbProxyEndpointInputBuilder) -> Result<CreateDbProxyEndpointOutput, SdkError<CreateDBProxyEndpointError>>;
        async fn create_db_security_group(&self, builder: CreateDbSecurityGroupInputBuilder) -> Result<CreateDbSecurityGroupOutput, SdkError<CreateDBSecurityGroupError>>;
        async fn create_db_shard_group(&self, builder: CreateDbShardGroupInputBuilder) -> Result<CreateDbShardGroupOutput, SdkError<CreateDBShardGroupError>>;
        async fn create_db_snapshot(&self, builder: CreateDbSnapshotInputBuilder) -> Result<CreateDbSnapshotOutput, SdkError<CreateDBSnapshotError>>;
        async fn create_db_subnet_group(&self, builder: CreateDbSubnetGroupInputBuilder) -> Result<CreateDbSubnetGroupOutput, SdkError<CreateDBSubnetGroupError>>;
        async fn create_event_subscription(&self, builder: CreateEventSubscriptionInputBuilder) -> Result<CreateEventSubscriptionOutput, SdkError<CreateEventSubscriptionError>>;
        async fn create_global_cluster(&self, builder: CreateGlobalClusterInputBuilder) -> Result<CreateGlobalClusterOutput, SdkError<CreateGlobalClusterError>>;
        async fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>;
        async fn create_option_group(&self, builder: CreateOptionGroupInputBuilder) -> Result<CreateOptionGroupOutput, SdkError<CreateOptionGroupError>>;
        async fn create_tenant_database(&self, builder: CreateTenantDatabaseInputBuilder) -> Result<CreateTenantDatabaseOutput, SdkError<CreateTenantDatabaseError>>;
        async fn delete_blue_green_deployment(&self, builder: DeleteBlueGreenDeploymentInputBuilder) -> Result<DeleteBlueGreenDeploymentOutput, SdkError<DeleteBlueGreenDeploymentError>>;
        async fn delete_custom_db_engine_version(&self, builder: DeleteCustomDbEngineVersionInputBuilder) -> Result<DeleteCustomDbEngineVersionOutput, SdkError<DeleteCustomDBEngineVersionError>>;
        async fn delete_db_cluster(&self, builder: DeleteDbClusterInputBuilder) -> Result<DeleteDbClusterOutput, SdkError<DeleteDBClusterError>>;
        async fn delete_db_cluster_automated_backup(&self, builder: DeleteDbClusterAutomatedBackupInputBuilder) -> Result<DeleteDbClusterAutomatedBackupOutput, SdkError<DeleteDBClusterAutomatedBackupError>>;
        async fn delete_db_cluster_endpoint(&self, builder: DeleteDbClusterEndpointInputBuilder) -> Result<DeleteDbClusterEndpointOutput, SdkError<DeleteDBClusterEndpointError>>;
        async fn delete_db_cluster_parameter_group(&self, builder: DeleteDbClusterParameterGroupInputBuilder) -> Result<DeleteDbClusterParameterGroupOutput, SdkError<DeleteDBClusterParameterGroupError>>;
        async fn delete_db_cluster_snapshot(&self, builder: DeleteDbClusterSnapshotInputBuilder) -> Result<DeleteDbClusterSnapshotOutput, SdkError<DeleteDBClusterSnapshotError>>;
        async fn delete_db_instance(&self, builder: DeleteDbInstanceInputBuilder) -> Result<DeleteDbInstanceOutput, SdkError<DeleteDBInstanceError>>;
        async fn delete_db_instance_automated_backup(&self, builder: DeleteDbInstanceAutomatedBackupInputBuilder) -> Result<DeleteDbInstanceAutomatedBackupOutput, SdkError<DeleteDBInstanceAutomatedBackupError>>;
        async fn delete_db_parameter_group(&self, builder: DeleteDbParameterGroupInputBuilder) -> Result<DeleteDbParameterGroupOutput, SdkError<DeleteDBParameterGroupError>>;
        async fn delete_db_proxy(&self, builder: DeleteDbProxyInputBuilder) -> Result<DeleteDbProxyOutput, SdkError<DeleteDBProxyError>>;
        async fn delete_db_proxy_endpoint(&self, builder: DeleteDbProxyEndpointInputBuilder) -> Result<DeleteDbProxyEndpointOutput, SdkError<DeleteDBProxyEndpointError>>;
        async fn delete_db_security_group(&self, builder: DeleteDbSecurityGroupInputBuilder) -> Result<DeleteDbSecurityGroupOutput, SdkError<DeleteDBSecurityGroupError>>;
        async fn delete_db_shard_group(&self, builder: DeleteDbShardGroupInputBuilder) -> Result<DeleteDbShardGroupOutput, SdkError<DeleteDBShardGroupError>>;
        async fn delete_db_snapshot(&self, builder: DeleteDbSnapshotInputBuilder) -> Result<DeleteDbSnapshotOutput, SdkError<DeleteDBSnapshotError>>;
        async fn delete_db_subnet_group(&self, builder: DeleteDbSubnetGroupInputBuilder) -> Result<DeleteDbSubnetGroupOutput, SdkError<DeleteDBSubnetGroupError>>;
        async fn delete_event_subscription(&self, builder: DeleteEventSubscriptionInputBuilder) -> Result<DeleteEventSubscriptionOutput, SdkError<DeleteEventSubscriptionError>>;
        async fn delete_global_cluster(&self, builder: DeleteGlobalClusterInputBuilder) -> Result<DeleteGlobalClusterOutput, SdkError<DeleteGlobalClusterError>>;
        async fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>;
        async fn delete_option_group(&self, builder: DeleteOptionGroupInputBuilder) -> Result<DeleteOptionGroupOutput, SdkError<DeleteOptionGroupError>>;
        async fn delete_tenant_database(&self, builder: DeleteTenantDatabaseInputBuilder) -> Result<DeleteTenantDatabaseOutput, SdkError<DeleteTenantDatabaseError>>;
        async fn deregister_db_proxy_targets(&self, builder: DeregisterDbProxyTargetsInputBuilder) -> Result<DeregisterDbProxyTargetsOutput, SdkError<DeregisterDBProxyTargetsError>>;
        async fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>;
        async fn describe_blue_green_deployments(&self, builder: DescribeBlueGreenDeploymentsInputBuilder) -> Result<DescribeBlueGreenDeploymentsOutput, SdkError<DescribeBlueGreenDeploymentsError>>;
        async fn describe_certificates(&self, builder: DescribeCertificatesInputBuilder) -> Result<DescribeCertificatesOutput, SdkError<DescribeCertificatesError>>;
        async fn describe_db_cluster_automated_backups(&self, builder: DescribeDbClusterAutomatedBackupsInputBuilder) -> Result<DescribeDbClusterAutomatedBackupsOutput, SdkError<DescribeDBClusterAutomatedBackupsError>>;
        async fn describe_db_cluster_backtracks(&self, builder: DescribeDbClusterBacktracksInputBuilder) -> Result<DescribeDbClusterBacktracksOutput, SdkError<DescribeDBClusterBacktracksError>>;
        async fn describe_db_cluster_endpoints(&self, builder: DescribeDbClusterEndpointsInputBuilder) -> Result<DescribeDbClusterEndpointsOutput, SdkError<DescribeDBClusterEndpointsError>>;
        async fn describe_db_cluster_parameter_groups(&self, builder: DescribeDbClusterParameterGroupsInputBuilder) -> Result<DescribeDbClusterParameterGroupsOutput, SdkError<DescribeDBClusterParameterGroupsError>>;
        async fn describe_db_cluster_parameters(&self, builder: DescribeDbClusterParametersInputBuilder) -> Result<DescribeDbClusterParametersOutput, SdkError<DescribeDBClusterParametersError>>;
        async fn describe_db_cluster_snapshot_attributes(&self, builder: DescribeDbClusterSnapshotAttributesInputBuilder) -> Result<DescribeDbClusterSnapshotAttributesOutput, SdkError<DescribeDBClusterSnapshotAttributesError>>;
        async fn describe_db_cluster_snapshots(&self, builder: DescribeDbClusterSnapshotsInputBuilder) -> Result<DescribeDbClusterSnapshotsOutput, SdkError<DescribeDBClusterSnapshotsError>>;
        async fn describe_db_clusters(&self, builder: DescribeDbClustersInputBuilder) -> Result<DescribeDbClustersOutput, SdkError<DescribeDBClustersError>>;
        async fn describe_db_engine_versions(&self, builder: DescribeDbEngineVersionsInputBuilder) -> Result<DescribeDbEngineVersionsOutput, SdkError<DescribeDBEngineVersionsError>>;
        async fn describe_db_instance_automated_backups(&self, builder: DescribeDbInstanceAutomatedBackupsInputBuilder) -> Result<DescribeDbInstanceAutomatedBackupsOutput, SdkError<DescribeDBInstanceAutomatedBackupsError>>;
        async fn describe_db_instances(&self, builder: DescribeDbInstancesInputBuilder) -> Result<DescribeDbInstancesOutput, SdkError<DescribeDBInstancesError>>;
        async fn describe_db_log_files(&self, builder: DescribeDbLogFilesInputBuilder) -> Result<DescribeDbLogFilesOutput, SdkError<DescribeDBLogFilesError>>;
        async fn describe_db_parameter_groups(&self, builder: DescribeDbParameterGroupsInputBuilder) -> Result<DescribeDbParameterGroupsOutput, SdkError<DescribeDBParameterGroupsError>>;
        async fn describe_db_parameters(&self, builder: DescribeDbParametersInputBuilder) -> Result<DescribeDbParametersOutput, SdkError<DescribeDBParametersError>>;
        async fn describe_db_proxies(&self, builder: DescribeDbProxiesInputBuilder) -> Result<DescribeDbProxiesOutput, SdkError<DescribeDBProxiesError>>;
        async fn describe_db_proxy_endpoints(&self, builder: DescribeDbProxyEndpointsInputBuilder) -> Result<DescribeDbProxyEndpointsOutput, SdkError<DescribeDBProxyEndpointsError>>;
        async fn describe_db_proxy_target_groups(&self, builder: DescribeDbProxyTargetGroupsInputBuilder) -> Result<DescribeDbProxyTargetGroupsOutput, SdkError<DescribeDBProxyTargetGroupsError>>;
        async fn describe_db_proxy_targets(&self, builder: DescribeDbProxyTargetsInputBuilder) -> Result<DescribeDbProxyTargetsOutput, SdkError<DescribeDBProxyTargetsError>>;
        async fn describe_db_recommendations(&self, builder: DescribeDbRecommendationsInputBuilder) -> Result<DescribeDbRecommendationsOutput, SdkError<DescribeDBRecommendationsError>>;
        async fn describe_db_security_groups(&self, builder: DescribeDbSecurityGroupsInputBuilder) -> Result<DescribeDbSecurityGroupsOutput, SdkError<DescribeDBSecurityGroupsError>>;
        async fn describe_db_shard_groups(&self, builder: DescribeDbShardGroupsInputBuilder) -> Result<DescribeDbShardGroupsOutput, SdkError<DescribeDBShardGroupsError>>;
        async fn describe_db_snapshot_attributes(&self, builder: DescribeDbSnapshotAttributesInputBuilder) -> Result<DescribeDbSnapshotAttributesOutput, SdkError<DescribeDBSnapshotAttributesError>>;
        async fn describe_db_snapshot_tenant_databases(&self, builder: DescribeDbSnapshotTenantDatabasesInputBuilder) -> Result<DescribeDbSnapshotTenantDatabasesOutput, SdkError<DescribeDBSnapshotTenantDatabasesError>>;
        async fn describe_db_snapshots(&self, builder: DescribeDbSnapshotsInputBuilder) -> Result<DescribeDbSnapshotsOutput, SdkError<DescribeDBSnapshotsError>>;
        async fn describe_db_subnet_groups(&self, builder: DescribeDbSubnetGroupsInputBuilder) -> Result<DescribeDbSubnetGroupsOutput, SdkError<DescribeDBSubnetGroupsError>>;
        async fn describe_engine_default_cluster_parameters(&self, builder: DescribeEngineDefaultClusterParametersInputBuilder) -> Result<DescribeEngineDefaultClusterParametersOutput, SdkError<DescribeEngineDefaultClusterParametersError>>;
        async fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>;
        async fn describe_event_categories(&self, builder: DescribeEventCategoriesInputBuilder) -> Result<DescribeEventCategoriesOutput, SdkError<DescribeEventCategoriesError>>;
        async fn describe_event_subscriptions(&self, builder: DescribeEventSubscriptionsInputBuilder) -> Result<DescribeEventSubscriptionsOutput, SdkError<DescribeEventSubscriptionsError>>;
        async fn describe_events(&self, builder: DescribeEventsInputBuilder) -> Result<DescribeEventsOutput, SdkError<DescribeEventsError>>;
        async fn describe_export_tasks(&self, builder: DescribeExportTasksInputBuilder) -> Result<DescribeExportTasksOutput, SdkError<DescribeExportTasksError>>;
        async fn describe_global_clusters(&self, builder: DescribeGlobalClustersInputBuilder) -> Result<DescribeGlobalClustersOutput, SdkError<DescribeGlobalClustersError>>;
        async fn describe_integrations(&self, builder: DescribeIntegrationsInputBuilder) -> Result<DescribeIntegrationsOutput, SdkError<DescribeIntegrationsError>>;
        async fn describe_option_group_options(&self, builder: DescribeOptionGroupOptionsInputBuilder) -> Result<DescribeOptionGroupOptionsOutput, SdkError<DescribeOptionGroupOptionsError>>;
        async fn describe_option_groups(&self, builder: DescribeOptionGroupsInputBuilder) -> Result<DescribeOptionGroupsOutput, SdkError<DescribeOptionGroupsError>>;
        async fn describe_orderable_db_instance_options(&self, builder: DescribeOrderableDbInstanceOptionsInputBuilder) -> Result<DescribeOrderableDbInstanceOptionsOutput, SdkError<DescribeOrderableDBInstanceOptionsError>>;
        async fn describe_pending_maintenance_actions(&self, builder: DescribePendingMaintenanceActionsInputBuilder) -> Result<DescribePendingMaintenanceActionsOutput, SdkError<DescribePendingMaintenanceActionsError>>;
        async fn describe_reserved_db_instances(&self, builder: DescribeReservedDbInstancesInputBuilder) -> Result<DescribeReservedDbInstancesOutput, SdkError<DescribeReservedDBInstancesError>>;
        async fn describe_reserved_db_instances_offerings(&self, builder: DescribeReservedDbInstancesOfferingsInputBuilder) -> Result<DescribeReservedDbInstancesOfferingsOutput, SdkError<DescribeReservedDBInstancesOfferingsError>>;
        async fn describe_source_regions(&self, builder: DescribeSourceRegionsInputBuilder) -> Result<DescribeSourceRegionsOutput, SdkError<DescribeSourceRegionsError>>;
        async fn describe_tenant_databases(&self, builder: DescribeTenantDatabasesInputBuilder) -> Result<DescribeTenantDatabasesOutput, SdkError<DescribeTenantDatabasesError>>;
        async fn describe_valid_db_instance_modifications(&self, builder: DescribeValidDbInstanceModificationsInputBuilder) -> Result<DescribeValidDbInstanceModificationsOutput, SdkError<DescribeValidDBInstanceModificationsError>>;
        async fn disable_http_endpoint(&self, builder: DisableHttpEndpointInputBuilder) -> Result<DisableHttpEndpointOutput, SdkError<DisableHttpEndpointError>>;
        async fn download_db_log_file_portion(&self, builder: DownloadDbLogFilePortionInputBuilder) -> Result<DownloadDbLogFilePortionOutput, SdkError<DownloadDBLogFilePortionError>>;
        async fn enable_http_endpoint(&self, builder: EnableHttpEndpointInputBuilder) -> Result<EnableHttpEndpointOutput, SdkError<EnableHttpEndpointError>>;
        async fn failover_db_cluster(&self, builder: FailoverDbClusterInputBuilder) -> Result<FailoverDbClusterOutput, SdkError<FailoverDBClusterError>>;
        async fn failover_global_cluster(&self, builder: FailoverGlobalClusterInputBuilder) -> Result<FailoverGlobalClusterOutput, SdkError<FailoverGlobalClusterError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn modify_activity_stream(&self, builder: ModifyActivityStreamInputBuilder) -> Result<ModifyActivityStreamOutput, SdkError<ModifyActivityStreamError>>;
        async fn modify_certificates(&self, builder: ModifyCertificatesInputBuilder) -> Result<ModifyCertificatesOutput, SdkError<ModifyCertificatesError>>;
        async fn modify_current_db_cluster_capacity(&self, builder: ModifyCurrentDbClusterCapacityInputBuilder) -> Result<ModifyCurrentDbClusterCapacityOutput, SdkError<ModifyCurrentDBClusterCapacityError>>;
        async fn modify_custom_db_engine_version(&self, builder: ModifyCustomDbEngineVersionInputBuilder) -> Result<ModifyCustomDbEngineVersionOutput, SdkError<ModifyCustomDBEngineVersionError>>;
        async fn modify_db_cluster(&self, builder: ModifyDbClusterInputBuilder) -> Result<ModifyDbClusterOutput, SdkError<ModifyDBClusterError>>;
        async fn modify_db_cluster_endpoint(&self, builder: ModifyDbClusterEndpointInputBuilder) -> Result<ModifyDbClusterEndpointOutput, SdkError<ModifyDBClusterEndpointError>>;
        async fn modify_db_cluster_parameter_group(&self, builder: ModifyDbClusterParameterGroupInputBuilder) -> Result<ModifyDbClusterParameterGroupOutput, SdkError<ModifyDBClusterParameterGroupError>>;
        async fn modify_db_cluster_snapshot_attribute(&self, builder: ModifyDbClusterSnapshotAttributeInputBuilder) -> Result<ModifyDbClusterSnapshotAttributeOutput, SdkError<ModifyDBClusterSnapshotAttributeError>>;
        async fn modify_db_instance(&self, builder: ModifyDbInstanceInputBuilder) -> Result<ModifyDbInstanceOutput, SdkError<ModifyDBInstanceError>>;
        async fn modify_db_parameter_group(&self, builder: ModifyDbParameterGroupInputBuilder) -> Result<ModifyDbParameterGroupOutput, SdkError<ModifyDBParameterGroupError>>;
        async fn modify_db_proxy(&self, builder: ModifyDbProxyInputBuilder) -> Result<ModifyDbProxyOutput, SdkError<ModifyDBProxyError>>;
        async fn modify_db_proxy_endpoint(&self, builder: ModifyDbProxyEndpointInputBuilder) -> Result<ModifyDbProxyEndpointOutput, SdkError<ModifyDBProxyEndpointError>>;
        async fn modify_db_proxy_target_group(&self, builder: ModifyDbProxyTargetGroupInputBuilder) -> Result<ModifyDbProxyTargetGroupOutput, SdkError<ModifyDBProxyTargetGroupError>>;
        async fn modify_db_recommendation(&self, builder: ModifyDbRecommendationInputBuilder) -> Result<ModifyDbRecommendationOutput, SdkError<ModifyDBRecommendationError>>;
        async fn modify_db_shard_group(&self, builder: ModifyDbShardGroupInputBuilder) -> Result<ModifyDbShardGroupOutput, SdkError<ModifyDBShardGroupError>>;
        async fn modify_db_snapshot(&self, builder: ModifyDbSnapshotInputBuilder) -> Result<ModifyDbSnapshotOutput, SdkError<ModifyDBSnapshotError>>;
        async fn modify_db_snapshot_attribute(&self, builder: ModifyDbSnapshotAttributeInputBuilder) -> Result<ModifyDbSnapshotAttributeOutput, SdkError<ModifyDBSnapshotAttributeError>>;
        async fn modify_db_subnet_group(&self, builder: ModifyDbSubnetGroupInputBuilder) -> Result<ModifyDbSubnetGroupOutput, SdkError<ModifyDBSubnetGroupError>>;
        async fn modify_event_subscription(&self, builder: ModifyEventSubscriptionInputBuilder) -> Result<ModifyEventSubscriptionOutput, SdkError<ModifyEventSubscriptionError>>;
        async fn modify_global_cluster(&self, builder: ModifyGlobalClusterInputBuilder) -> Result<ModifyGlobalClusterOutput, SdkError<ModifyGlobalClusterError>>;
        async fn modify_integration(&self, builder: ModifyIntegrationInputBuilder) -> Result<ModifyIntegrationOutput, SdkError<ModifyIntegrationError>>;
        async fn modify_tenant_database(&self, builder: ModifyTenantDatabaseInputBuilder) -> Result<ModifyTenantDatabaseOutput, SdkError<ModifyTenantDatabaseError>>;
        async fn promote_read_replica(&self, builder: PromoteReadReplicaInputBuilder) -> Result<PromoteReadReplicaOutput, SdkError<PromoteReadReplicaError>>;
        async fn promote_read_replica_db_cluster(&self, builder: PromoteReadReplicaDbClusterInputBuilder) -> Result<PromoteReadReplicaDbClusterOutput, SdkError<PromoteReadReplicaDBClusterError>>;
        async fn purchase_reserved_db_instances_offering(&self, builder: PurchaseReservedDbInstancesOfferingInputBuilder) -> Result<PurchaseReservedDbInstancesOfferingOutput, SdkError<PurchaseReservedDBInstancesOfferingError>>;
        async fn reboot_db_cluster(&self, builder: RebootDbClusterInputBuilder) -> Result<RebootDbClusterOutput, SdkError<RebootDBClusterError>>;
        async fn reboot_db_instance(&self, builder: RebootDbInstanceInputBuilder) -> Result<RebootDbInstanceOutput, SdkError<RebootDBInstanceError>>;
        async fn reboot_db_shard_group(&self, builder: RebootDbShardGroupInputBuilder) -> Result<RebootDbShardGroupOutput, SdkError<RebootDBShardGroupError>>;
        async fn register_db_proxy_targets(&self, builder: RegisterDbProxyTargetsInputBuilder) -> Result<RegisterDbProxyTargetsOutput, SdkError<RegisterDBProxyTargetsError>>;
        async fn remove_from_global_cluster(&self, builder: RemoveFromGlobalClusterInputBuilder) -> Result<RemoveFromGlobalClusterOutput, SdkError<RemoveFromGlobalClusterError>>;
        async fn remove_role_from_db_cluster(&self, builder: RemoveRoleFromDbClusterInputBuilder) -> Result<RemoveRoleFromDbClusterOutput, SdkError<RemoveRoleFromDBClusterError>>;
        async fn remove_role_from_db_instance(&self, builder: RemoveRoleFromDbInstanceInputBuilder) -> Result<RemoveRoleFromDbInstanceOutput, SdkError<RemoveRoleFromDBInstanceError>>;
        async fn remove_source_identifier_from_subscription(&self, builder: RemoveSourceIdentifierFromSubscriptionInputBuilder) -> Result<RemoveSourceIdentifierFromSubscriptionOutput, SdkError<RemoveSourceIdentifierFromSubscriptionError>>;
        async fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>;
        async fn reset_db_cluster_parameter_group(&self, builder: ResetDbClusterParameterGroupInputBuilder) -> Result<ResetDbClusterParameterGroupOutput, SdkError<ResetDBClusterParameterGroupError>>;
        async fn reset_db_parameter_group(&self, builder: ResetDbParameterGroupInputBuilder) -> Result<ResetDbParameterGroupOutput, SdkError<ResetDBParameterGroupError>>;
        async fn restore_db_cluster_from_s3(&self, builder: RestoreDbClusterFromS3InputBuilder) -> Result<RestoreDbClusterFromS3Output, SdkError<RestoreDBClusterFromS3Error>>;
        async fn restore_db_cluster_from_snapshot(&self, builder: RestoreDbClusterFromSnapshotInputBuilder) -> Result<RestoreDbClusterFromSnapshotOutput, SdkError<RestoreDBClusterFromSnapshotError>>;
        async fn restore_db_cluster_to_point_in_time(&self, builder: RestoreDbClusterToPointInTimeInputBuilder) -> Result<RestoreDbClusterToPointInTimeOutput, SdkError<RestoreDBClusterToPointInTimeError>>;
        async fn restore_db_instance_from_db_snapshot(&self, builder: RestoreDbInstanceFromDbSnapshotInputBuilder) -> Result<RestoreDbInstanceFromDbSnapshotOutput, SdkError<RestoreDBInstanceFromDBSnapshotError>>;
        async fn restore_db_instance_from_s3(&self, builder: RestoreDbInstanceFromS3InputBuilder) -> Result<RestoreDbInstanceFromS3Output, SdkError<RestoreDBInstanceFromS3Error>>;
        async fn restore_db_instance_to_point_in_time(&self, builder: RestoreDbInstanceToPointInTimeInputBuilder) -> Result<RestoreDbInstanceToPointInTimeOutput, SdkError<RestoreDBInstanceToPointInTimeError>>;
        async fn revoke_db_security_group_ingress(&self, builder: RevokeDbSecurityGroupIngressInputBuilder) -> Result<RevokeDbSecurityGroupIngressOutput, SdkError<RevokeDBSecurityGroupIngressError>>;
        async fn start_activity_stream(&self, builder: StartActivityStreamInputBuilder) -> Result<StartActivityStreamOutput, SdkError<StartActivityStreamError>>;
        async fn start_db_cluster(&self, builder: StartDbClusterInputBuilder) -> Result<StartDbClusterOutput, SdkError<StartDBClusterError>>;
        async fn start_db_instance(&self, builder: StartDbInstanceInputBuilder) -> Result<StartDbInstanceOutput, SdkError<StartDBInstanceError>>;
        async fn start_db_instance_automated_backups_replication(&self, builder: StartDbInstanceAutomatedBackupsReplicationInputBuilder) -> Result<StartDbInstanceAutomatedBackupsReplicationOutput, SdkError<StartDBInstanceAutomatedBackupsReplicationError>>;
        async fn start_export_task(&self, builder: StartExportTaskInputBuilder) -> Result<StartExportTaskOutput, SdkError<StartExportTaskError>>;
        async fn stop_activity_stream(&self, builder: StopActivityStreamInputBuilder) -> Result<StopActivityStreamOutput, SdkError<StopActivityStreamError>>;
        async fn stop_db_cluster(&self, builder: StopDbClusterInputBuilder) -> Result<StopDbClusterOutput, SdkError<StopDBClusterError>>;
        async fn stop_db_instance(&self, builder: StopDbInstanceInputBuilder) -> Result<StopDbInstanceOutput, SdkError<StopDBInstanceError>>;
        async fn stop_db_instance_automated_backups_replication(&self, builder: StopDbInstanceAutomatedBackupsReplicationInputBuilder) -> Result<StopDbInstanceAutomatedBackupsReplicationOutput, SdkError<StopDBInstanceAutomatedBackupsReplicationError>>;
        async fn switchover_blue_green_deployment(&self, builder: SwitchoverBlueGreenDeploymentInputBuilder) -> Result<SwitchoverBlueGreenDeploymentOutput, SdkError<SwitchoverBlueGreenDeploymentError>>;
        async fn switchover_global_cluster(&self, builder: SwitchoverGlobalClusterInputBuilder) -> Result<SwitchoverGlobalClusterOutput, SdkError<SwitchoverGlobalClusterError>>;
        async fn switchover_read_replica(&self, builder: SwitchoverReadReplicaInputBuilder) -> Result<SwitchoverReadReplicaOutput, SdkError<SwitchoverReadReplicaError>>;
    }
}
