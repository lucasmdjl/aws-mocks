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
use aws_sdk_elasticache::operation::add_tags_to_resource::{builders::*, *};
use aws_sdk_elasticache::operation::authorize_cache_security_group_ingress::{builders::*, *};
use aws_sdk_elasticache::operation::batch_apply_update_action::{builders::*, *};
use aws_sdk_elasticache::operation::batch_stop_update_action::{builders::*, *};
use aws_sdk_elasticache::operation::complete_migration::{builders::*, *};
use aws_sdk_elasticache::operation::copy_serverless_cache_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::copy_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::create_cache_cluster::{builders::*, *};
use aws_sdk_elasticache::operation::create_cache_parameter_group::{builders::*, *};
use aws_sdk_elasticache::operation::create_cache_security_group::{builders::*, *};
use aws_sdk_elasticache::operation::create_cache_subnet_group::{builders::*, *};
use aws_sdk_elasticache::operation::create_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::create_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::create_serverless_cache::{builders::*, *};
use aws_sdk_elasticache::operation::create_serverless_cache_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::create_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::create_user::{builders::*, *};
use aws_sdk_elasticache::operation::create_user_group::{builders::*, *};
use aws_sdk_elasticache::operation::decrease_node_groups_in_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::decrease_replica_count::{builders::*, *};
use aws_sdk_elasticache::operation::delete_cache_cluster::{builders::*, *};
use aws_sdk_elasticache::operation::delete_cache_parameter_group::{builders::*, *};
use aws_sdk_elasticache::operation::delete_cache_security_group::{builders::*, *};
use aws_sdk_elasticache::operation::delete_cache_subnet_group::{builders::*, *};
use aws_sdk_elasticache::operation::delete_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::delete_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::delete_serverless_cache::{builders::*, *};
use aws_sdk_elasticache::operation::delete_serverless_cache_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::delete_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::delete_user::{builders::*, *};
use aws_sdk_elasticache::operation::delete_user_group::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_clusters::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_engine_versions::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_parameter_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_parameters::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_security_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_cache_subnet_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_engine_default_parameters::{builders::*, *};
use aws_sdk_elasticache::operation::describe_events::{builders::*, *};
use aws_sdk_elasticache::operation::describe_global_replication_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_replication_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_reserved_cache_nodes::{builders::*, *};
use aws_sdk_elasticache::operation::describe_reserved_cache_nodes_offerings::{builders::*, *};
use aws_sdk_elasticache::operation::describe_serverless_cache_snapshots::{builders::*, *};
use aws_sdk_elasticache::operation::describe_serverless_caches::{builders::*, *};
use aws_sdk_elasticache::operation::describe_service_updates::{builders::*, *};
use aws_sdk_elasticache::operation::describe_snapshots::{builders::*, *};
use aws_sdk_elasticache::operation::describe_update_actions::{builders::*, *};
use aws_sdk_elasticache::operation::describe_user_groups::{builders::*, *};
use aws_sdk_elasticache::operation::describe_users::{builders::*, *};
use aws_sdk_elasticache::operation::disassociate_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::export_serverless_cache_snapshot::{builders::*, *};
use aws_sdk_elasticache::operation::failover_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::increase_node_groups_in_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::increase_replica_count::{builders::*, *};
use aws_sdk_elasticache::operation::list_allowed_node_type_modifications::{builders::*, *};
use aws_sdk_elasticache::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_elasticache::operation::modify_cache_cluster::{builders::*, *};
use aws_sdk_elasticache::operation::modify_cache_parameter_group::{builders::*, *};
use aws_sdk_elasticache::operation::modify_cache_subnet_group::{builders::*, *};
use aws_sdk_elasticache::operation::modify_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::modify_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::modify_replication_group_shard_configuration::{builders::*, *};
use aws_sdk_elasticache::operation::modify_serverless_cache::{builders::*, *};
use aws_sdk_elasticache::operation::modify_user::{builders::*, *};
use aws_sdk_elasticache::operation::modify_user_group::{builders::*, *};
use aws_sdk_elasticache::operation::purchase_reserved_cache_nodes_offering::{builders::*, *};
use aws_sdk_elasticache::operation::rebalance_slots_in_global_replication_group::{builders::*, *};
use aws_sdk_elasticache::operation::reboot_cache_cluster::{builders::*, *};
use aws_sdk_elasticache::operation::remove_tags_from_resource::{builders::*, *};
use aws_sdk_elasticache::operation::reset_cache_parameter_group::{builders::*, *};
use aws_sdk_elasticache::operation::revoke_cache_security_group_ingress::{builders::*, *};
use aws_sdk_elasticache::operation::start_migration::{builders::*, *};
use aws_sdk_elasticache::operation::test_failover::{builders::*, *};
use aws_sdk_elasticache::operation::test_migration::{builders::*, *};
use aws_sdk_elasticache::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_elasticache::Client;
use std::ops::Deref;

pub use aws_sdk_elasticache::*;

pub struct ElastiCacheClientImpl(Client);
impl ElastiCacheClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ElastiCacheClient {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>>;
    fn authorize_cache_security_group_ingress(&self, builder: AuthorizeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeCacheSecurityGroupIngressOutput, SdkError<AuthorizeCacheSecurityGroupIngressError>>>;
    fn batch_apply_update_action(&self, builder: BatchApplyUpdateActionInputBuilder) -> impl Future<Output = Result<BatchApplyUpdateActionOutput, SdkError<BatchApplyUpdateActionError>>>;
    fn batch_stop_update_action(&self, builder: BatchStopUpdateActionInputBuilder) -> impl Future<Output = Result<BatchStopUpdateActionOutput, SdkError<BatchStopUpdateActionError>>>;
    fn complete_migration(&self, builder: CompleteMigrationInputBuilder) -> impl Future<Output = Result<CompleteMigrationOutput, SdkError<CompleteMigrationError>>>;
    fn copy_serverless_cache_snapshot(&self, builder: CopyServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CopyServerlessCacheSnapshotOutput, SdkError<CopyServerlessCacheSnapshotError>>>;
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>>;
    fn create_cache_cluster(&self, builder: CreateCacheClusterInputBuilder) -> impl Future<Output = Result<CreateCacheClusterOutput, SdkError<CreateCacheClusterError>>>;
    fn create_cache_parameter_group(&self, builder: CreateCacheParameterGroupInputBuilder) -> impl Future<Output = Result<CreateCacheParameterGroupOutput, SdkError<CreateCacheParameterGroupError>>>;
    fn create_cache_security_group(&self, builder: CreateCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSecurityGroupOutput, SdkError<CreateCacheSecurityGroupError>>>;
    fn create_cache_subnet_group(&self, builder: CreateCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSubnetGroupOutput, SdkError<CreateCacheSubnetGroupError>>>;
    fn create_global_replication_group(&self, builder: CreateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateGlobalReplicationGroupOutput, SdkError<CreateGlobalReplicationGroupError>>>;
    fn create_replication_group(&self, builder: CreateReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateReplicationGroupOutput, SdkError<CreateReplicationGroupError>>>;
    fn create_serverless_cache(&self, builder: CreateServerlessCacheInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheOutput, SdkError<CreateServerlessCacheError>>>;
    fn create_serverless_cache_snapshot(&self, builder: CreateServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheSnapshotOutput, SdkError<CreateServerlessCacheSnapshotError>>>;
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>>;
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>>;
    fn create_user_group(&self, builder: CreateUserGroupInputBuilder) -> impl Future<Output = Result<CreateUserGroupOutput, SdkError<CreateUserGroupError>>>;
    fn decrease_node_groups_in_global_replication_group(&self, builder: DecreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DecreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<DecreaseNodeGroupsInGlobalReplicationGroupError>>>;
    fn decrease_replica_count(&self, builder: DecreaseReplicaCountInputBuilder) -> impl Future<Output = Result<DecreaseReplicaCountOutput, SdkError<DecreaseReplicaCountError>>>;
    fn delete_cache_cluster(&self, builder: DeleteCacheClusterInputBuilder) -> impl Future<Output = Result<DeleteCacheClusterOutput, SdkError<DeleteCacheClusterError>>>;
    fn delete_cache_parameter_group(&self, builder: DeleteCacheParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheParameterGroupOutput, SdkError<DeleteCacheParameterGroupError>>>;
    fn delete_cache_security_group(&self, builder: DeleteCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSecurityGroupOutput, SdkError<DeleteCacheSecurityGroupError>>>;
    fn delete_cache_subnet_group(&self, builder: DeleteCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSubnetGroupOutput, SdkError<DeleteCacheSubnetGroupError>>>;
    fn delete_global_replication_group(&self, builder: DeleteGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteGlobalReplicationGroupOutput, SdkError<DeleteGlobalReplicationGroupError>>>;
    fn delete_replication_group(&self, builder: DeleteReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteReplicationGroupOutput, SdkError<DeleteReplicationGroupError>>>;
    fn delete_serverless_cache(&self, builder: DeleteServerlessCacheInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheOutput, SdkError<DeleteServerlessCacheError>>>;
    fn delete_serverless_cache_snapshot(&self, builder: DeleteServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheSnapshotOutput, SdkError<DeleteServerlessCacheSnapshotError>>>;
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn delete_user_group(&self, builder: DeleteUserGroupInputBuilder) -> impl Future<Output = Result<DeleteUserGroupOutput, SdkError<DeleteUserGroupError>>>;
    fn describe_cache_clusters(&self, builder: DescribeCacheClustersInputBuilder) -> impl Future<Output = Result<DescribeCacheClustersOutput, SdkError<DescribeCacheClustersError>>>;
    fn describe_cache_engine_versions(&self, builder: DescribeCacheEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeCacheEngineVersionsOutput, SdkError<DescribeCacheEngineVersionsError>>>;
    fn describe_cache_parameter_groups(&self, builder: DescribeCacheParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheParameterGroupsOutput, SdkError<DescribeCacheParameterGroupsError>>>;
    fn describe_cache_parameters(&self, builder: DescribeCacheParametersInputBuilder) -> impl Future<Output = Result<DescribeCacheParametersOutput, SdkError<DescribeCacheParametersError>>>;
    fn describe_cache_security_groups(&self, builder: DescribeCacheSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSecurityGroupsOutput, SdkError<DescribeCacheSecurityGroupsError>>>;
    fn describe_cache_subnet_groups(&self, builder: DescribeCacheSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSubnetGroupsOutput, SdkError<DescribeCacheSubnetGroupsError>>>;
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>>;
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>>;
    fn describe_global_replication_groups(&self, builder: DescribeGlobalReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeGlobalReplicationGroupsOutput, SdkError<DescribeGlobalReplicationGroupsError>>>;
    fn describe_replication_groups(&self, builder: DescribeReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeReplicationGroupsOutput, SdkError<DescribeReplicationGroupsError>>>;
    fn describe_reserved_cache_nodes(&self, builder: DescribeReservedCacheNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOutput, SdkError<DescribeReservedCacheNodesError>>>;
    fn describe_reserved_cache_nodes_offerings(&self, builder: DescribeReservedCacheNodesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOfferingsOutput, SdkError<DescribeReservedCacheNodesOfferingsError>>>;
    fn describe_serverless_cache_snapshots(&self, builder: DescribeServerlessCacheSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeServerlessCacheSnapshotsOutput, SdkError<DescribeServerlessCacheSnapshotsError>>>;
    fn describe_serverless_caches(&self, builder: DescribeServerlessCachesInputBuilder) -> impl Future<Output = Result<DescribeServerlessCachesOutput, SdkError<DescribeServerlessCachesError>>>;
    fn describe_service_updates(&self, builder: DescribeServiceUpdatesInputBuilder) -> impl Future<Output = Result<DescribeServiceUpdatesOutput, SdkError<DescribeServiceUpdatesError>>>;
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>>;
    fn describe_update_actions(&self, builder: DescribeUpdateActionsInputBuilder) -> impl Future<Output = Result<DescribeUpdateActionsOutput, SdkError<DescribeUpdateActionsError>>>;
    fn describe_user_groups(&self, builder: DescribeUserGroupsInputBuilder) -> impl Future<Output = Result<DescribeUserGroupsOutput, SdkError<DescribeUserGroupsError>>>;
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>>;
    fn disassociate_global_replication_group(&self, builder: DisassociateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DisassociateGlobalReplicationGroupOutput, SdkError<DisassociateGlobalReplicationGroupError>>>;
    fn export_serverless_cache_snapshot(&self, builder: ExportServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<ExportServerlessCacheSnapshotOutput, SdkError<ExportServerlessCacheSnapshotError>>>;
    fn failover_global_replication_group(&self, builder: FailoverGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<FailoverGlobalReplicationGroupOutput, SdkError<FailoverGlobalReplicationGroupError>>>;
    fn increase_node_groups_in_global_replication_group(&self, builder: IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<IncreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<IncreaseNodeGroupsInGlobalReplicationGroupError>>>;
    fn increase_replica_count(&self, builder: IncreaseReplicaCountInputBuilder) -> impl Future<Output = Result<IncreaseReplicaCountOutput, SdkError<IncreaseReplicaCountError>>>;
    fn list_allowed_node_type_modifications(&self, builder: ListAllowedNodeTypeModificationsInputBuilder) -> impl Future<Output = Result<ListAllowedNodeTypeModificationsOutput, SdkError<ListAllowedNodeTypeModificationsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn modify_cache_cluster(&self, builder: ModifyCacheClusterInputBuilder) -> impl Future<Output = Result<ModifyCacheClusterOutput, SdkError<ModifyCacheClusterError>>>;
    fn modify_cache_parameter_group(&self, builder: ModifyCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheParameterGroupOutput, SdkError<ModifyCacheParameterGroupError>>>;
    fn modify_cache_subnet_group(&self, builder: ModifyCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheSubnetGroupOutput, SdkError<ModifyCacheSubnetGroupError>>>;
    fn modify_global_replication_group(&self, builder: ModifyGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyGlobalReplicationGroupOutput, SdkError<ModifyGlobalReplicationGroupError>>>;
    fn modify_replication_group(&self, builder: ModifyReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupOutput, SdkError<ModifyReplicationGroupError>>>;
    fn modify_replication_group_shard_configuration(&self, builder: ModifyReplicationGroupShardConfigurationInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupShardConfigurationOutput, SdkError<ModifyReplicationGroupShardConfigurationError>>>;
    fn modify_serverless_cache(&self, builder: ModifyServerlessCacheInputBuilder) -> impl Future<Output = Result<ModifyServerlessCacheOutput, SdkError<ModifyServerlessCacheError>>>;
    fn modify_user(&self, builder: ModifyUserInputBuilder) -> impl Future<Output = Result<ModifyUserOutput, SdkError<ModifyUserError>>>;
    fn modify_user_group(&self, builder: ModifyUserGroupInputBuilder) -> impl Future<Output = Result<ModifyUserGroupOutput, SdkError<ModifyUserGroupError>>>;
    fn purchase_reserved_cache_nodes_offering(&self, builder: PurchaseReservedCacheNodesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedCacheNodesOfferingOutput, SdkError<PurchaseReservedCacheNodesOfferingError>>>;
    fn rebalance_slots_in_global_replication_group(&self, builder: RebalanceSlotsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<RebalanceSlotsInGlobalReplicationGroupOutput, SdkError<RebalanceSlotsInGlobalReplicationGroupError>>>;
    fn reboot_cache_cluster(&self, builder: RebootCacheClusterInputBuilder) -> impl Future<Output = Result<RebootCacheClusterOutput, SdkError<RebootCacheClusterError>>>;
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>>;
    fn reset_cache_parameter_group(&self, builder: ResetCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ResetCacheParameterGroupOutput, SdkError<ResetCacheParameterGroupError>>>;
    fn revoke_cache_security_group_ingress(&self, builder: RevokeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeCacheSecurityGroupIngressOutput, SdkError<RevokeCacheSecurityGroupIngressError>>>;
    fn start_migration(&self, builder: StartMigrationInputBuilder) -> impl Future<Output = Result<StartMigrationOutput, SdkError<StartMigrationError>>>;
    fn test_failover(&self, builder: TestFailoverInputBuilder) -> impl Future<Output = Result<TestFailoverOutput, SdkError<TestFailoverError>>>;
    fn test_migration(&self, builder: TestMigrationInputBuilder) -> impl Future<Output = Result<TestMigrationOutput, SdkError<TestMigrationError>>>;
}
impl ElastiCacheClient for ElastiCacheClientImpl {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        builder.send_with(&self.0)
    }
    fn authorize_cache_security_group_ingress(&self, builder: AuthorizeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeCacheSecurityGroupIngressOutput, SdkError<AuthorizeCacheSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn batch_apply_update_action(&self, builder: BatchApplyUpdateActionInputBuilder) -> impl Future<Output = Result<BatchApplyUpdateActionOutput, SdkError<BatchApplyUpdateActionError>>> {
        builder.send_with(&self.0)
    }
    fn batch_stop_update_action(&self, builder: BatchStopUpdateActionInputBuilder) -> impl Future<Output = Result<BatchStopUpdateActionOutput, SdkError<BatchStopUpdateActionError>>> {
        builder.send_with(&self.0)
    }
    fn complete_migration(&self, builder: CompleteMigrationInputBuilder) -> impl Future<Output = Result<CompleteMigrationOutput, SdkError<CompleteMigrationError>>> {
        builder.send_with(&self.0)
    }
    fn copy_serverless_cache_snapshot(&self, builder: CopyServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CopyServerlessCacheSnapshotOutput, SdkError<CopyServerlessCacheSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_cache_cluster(&self, builder: CreateCacheClusterInputBuilder) -> impl Future<Output = Result<CreateCacheClusterOutput, SdkError<CreateCacheClusterError>>> {
        builder.send_with(&self.0)
    }
    fn create_cache_parameter_group(&self, builder: CreateCacheParameterGroupInputBuilder) -> impl Future<Output = Result<CreateCacheParameterGroupOutput, SdkError<CreateCacheParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_cache_security_group(&self, builder: CreateCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSecurityGroupOutput, SdkError<CreateCacheSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_cache_subnet_group(&self, builder: CreateCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSubnetGroupOutput, SdkError<CreateCacheSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_global_replication_group(&self, builder: CreateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateGlobalReplicationGroupOutput, SdkError<CreateGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_replication_group(&self, builder: CreateReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateReplicationGroupOutput, SdkError<CreateReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_serverless_cache(&self, builder: CreateServerlessCacheInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheOutput, SdkError<CreateServerlessCacheError>>> {
        builder.send_with(&self.0)
    }
    fn create_serverless_cache_snapshot(&self, builder: CreateServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheSnapshotOutput, SdkError<CreateServerlessCacheSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_group(&self, builder: CreateUserGroupInputBuilder) -> impl Future<Output = Result<CreateUserGroupOutput, SdkError<CreateUserGroupError>>> {
        builder.send_with(&self.0)
    }
    fn decrease_node_groups_in_global_replication_group(&self, builder: DecreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DecreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<DecreaseNodeGroupsInGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn decrease_replica_count(&self, builder: DecreaseReplicaCountInputBuilder) -> impl Future<Output = Result<DecreaseReplicaCountOutput, SdkError<DecreaseReplicaCountError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cache_cluster(&self, builder: DeleteCacheClusterInputBuilder) -> impl Future<Output = Result<DeleteCacheClusterOutput, SdkError<DeleteCacheClusterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cache_parameter_group(&self, builder: DeleteCacheParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheParameterGroupOutput, SdkError<DeleteCacheParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cache_security_group(&self, builder: DeleteCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSecurityGroupOutput, SdkError<DeleteCacheSecurityGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cache_subnet_group(&self, builder: DeleteCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSubnetGroupOutput, SdkError<DeleteCacheSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_global_replication_group(&self, builder: DeleteGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteGlobalReplicationGroupOutput, SdkError<DeleteGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_replication_group(&self, builder: DeleteReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteReplicationGroupOutput, SdkError<DeleteReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_serverless_cache(&self, builder: DeleteServerlessCacheInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheOutput, SdkError<DeleteServerlessCacheError>>> {
        builder.send_with(&self.0)
    }
    fn delete_serverless_cache_snapshot(&self, builder: DeleteServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheSnapshotOutput, SdkError<DeleteServerlessCacheSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_group(&self, builder: DeleteUserGroupInputBuilder) -> impl Future<Output = Result<DeleteUserGroupOutput, SdkError<DeleteUserGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_clusters(&self, builder: DescribeCacheClustersInputBuilder) -> impl Future<Output = Result<DescribeCacheClustersOutput, SdkError<DescribeCacheClustersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_engine_versions(&self, builder: DescribeCacheEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeCacheEngineVersionsOutput, SdkError<DescribeCacheEngineVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_parameter_groups(&self, builder: DescribeCacheParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheParameterGroupsOutput, SdkError<DescribeCacheParameterGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_parameters(&self, builder: DescribeCacheParametersInputBuilder) -> impl Future<Output = Result<DescribeCacheParametersOutput, SdkError<DescribeCacheParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_security_groups(&self, builder: DescribeCacheSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSecurityGroupsOutput, SdkError<DescribeCacheSecurityGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cache_subnet_groups(&self, builder: DescribeCacheSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSubnetGroupsOutput, SdkError<DescribeCacheSubnetGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_global_replication_groups(&self, builder: DescribeGlobalReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeGlobalReplicationGroupsOutput, SdkError<DescribeGlobalReplicationGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_replication_groups(&self, builder: DescribeReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeReplicationGroupsOutput, SdkError<DescribeReplicationGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_cache_nodes(&self, builder: DescribeReservedCacheNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOutput, SdkError<DescribeReservedCacheNodesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_reserved_cache_nodes_offerings(&self, builder: DescribeReservedCacheNodesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOfferingsOutput, SdkError<DescribeReservedCacheNodesOfferingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_serverless_cache_snapshots(&self, builder: DescribeServerlessCacheSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeServerlessCacheSnapshotsOutput, SdkError<DescribeServerlessCacheSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_serverless_caches(&self, builder: DescribeServerlessCachesInputBuilder) -> impl Future<Output = Result<DescribeServerlessCachesOutput, SdkError<DescribeServerlessCachesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_service_updates(&self, builder: DescribeServiceUpdatesInputBuilder) -> impl Future<Output = Result<DescribeServiceUpdatesOutput, SdkError<DescribeServiceUpdatesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_update_actions(&self, builder: DescribeUpdateActionsInputBuilder) -> impl Future<Output = Result<DescribeUpdateActionsOutput, SdkError<DescribeUpdateActionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_groups(&self, builder: DescribeUserGroupsInputBuilder) -> impl Future<Output = Result<DescribeUserGroupsOutput, SdkError<DescribeUserGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_global_replication_group(&self, builder: DisassociateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DisassociateGlobalReplicationGroupOutput, SdkError<DisassociateGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn export_serverless_cache_snapshot(&self, builder: ExportServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<ExportServerlessCacheSnapshotOutput, SdkError<ExportServerlessCacheSnapshotError>>> {
        builder.send_with(&self.0)
    }
    fn failover_global_replication_group(&self, builder: FailoverGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<FailoverGlobalReplicationGroupOutput, SdkError<FailoverGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn increase_node_groups_in_global_replication_group(&self, builder: IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<IncreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<IncreaseNodeGroupsInGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn increase_replica_count(&self, builder: IncreaseReplicaCountInputBuilder) -> impl Future<Output = Result<IncreaseReplicaCountOutput, SdkError<IncreaseReplicaCountError>>> {
        builder.send_with(&self.0)
    }
    fn list_allowed_node_type_modifications(&self, builder: ListAllowedNodeTypeModificationsInputBuilder) -> impl Future<Output = Result<ListAllowedNodeTypeModificationsOutput, SdkError<ListAllowedNodeTypeModificationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cache_cluster(&self, builder: ModifyCacheClusterInputBuilder) -> impl Future<Output = Result<ModifyCacheClusterOutput, SdkError<ModifyCacheClusterError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cache_parameter_group(&self, builder: ModifyCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheParameterGroupOutput, SdkError<ModifyCacheParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cache_subnet_group(&self, builder: ModifyCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheSubnetGroupOutput, SdkError<ModifyCacheSubnetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_global_replication_group(&self, builder: ModifyGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyGlobalReplicationGroupOutput, SdkError<ModifyGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_replication_group(&self, builder: ModifyReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupOutput, SdkError<ModifyReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn modify_replication_group_shard_configuration(&self, builder: ModifyReplicationGroupShardConfigurationInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupShardConfigurationOutput, SdkError<ModifyReplicationGroupShardConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn modify_serverless_cache(&self, builder: ModifyServerlessCacheInputBuilder) -> impl Future<Output = Result<ModifyServerlessCacheOutput, SdkError<ModifyServerlessCacheError>>> {
        builder.send_with(&self.0)
    }
    fn modify_user(&self, builder: ModifyUserInputBuilder) -> impl Future<Output = Result<ModifyUserOutput, SdkError<ModifyUserError>>> {
        builder.send_with(&self.0)
    }
    fn modify_user_group(&self, builder: ModifyUserGroupInputBuilder) -> impl Future<Output = Result<ModifyUserGroupOutput, SdkError<ModifyUserGroupError>>> {
        builder.send_with(&self.0)
    }
    fn purchase_reserved_cache_nodes_offering(&self, builder: PurchaseReservedCacheNodesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedCacheNodesOfferingOutput, SdkError<PurchaseReservedCacheNodesOfferingError>>> {
        builder.send_with(&self.0)
    }
    fn rebalance_slots_in_global_replication_group(&self, builder: RebalanceSlotsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<RebalanceSlotsInGlobalReplicationGroupOutput, SdkError<RebalanceSlotsInGlobalReplicationGroupError>>> {
        builder.send_with(&self.0)
    }
    fn reboot_cache_cluster(&self, builder: RebootCacheClusterInputBuilder) -> impl Future<Output = Result<RebootCacheClusterOutput, SdkError<RebootCacheClusterError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        builder.send_with(&self.0)
    }
    fn reset_cache_parameter_group(&self, builder: ResetCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ResetCacheParameterGroupOutput, SdkError<ResetCacheParameterGroupError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_cache_security_group_ingress(&self, builder: RevokeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeCacheSecurityGroupIngressOutput, SdkError<RevokeCacheSecurityGroupIngressError>>> {
        builder.send_with(&self.0)
    }
    fn start_migration(&self, builder: StartMigrationInputBuilder) -> impl Future<Output = Result<StartMigrationOutput, SdkError<StartMigrationError>>> {
        builder.send_with(&self.0)
    }
    fn test_failover(&self, builder: TestFailoverInputBuilder) -> impl Future<Output = Result<TestFailoverOutput, SdkError<TestFailoverError>>> {
        builder.send_with(&self.0)
    }
    fn test_migration(&self, builder: TestMigrationInputBuilder) -> impl Future<Output = Result<TestMigrationOutput, SdkError<TestMigrationError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> ElastiCacheClient for T
where T: Deref,
      T::Target: ElastiCacheClient {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        self.deref().add_tags_to_resource(builder)
    }
    fn authorize_cache_security_group_ingress(&self, builder: AuthorizeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<AuthorizeCacheSecurityGroupIngressOutput, SdkError<AuthorizeCacheSecurityGroupIngressError>>> {
        self.deref().authorize_cache_security_group_ingress(builder)
    }
    fn batch_apply_update_action(&self, builder: BatchApplyUpdateActionInputBuilder) -> impl Future<Output = Result<BatchApplyUpdateActionOutput, SdkError<BatchApplyUpdateActionError>>> {
        self.deref().batch_apply_update_action(builder)
    }
    fn batch_stop_update_action(&self, builder: BatchStopUpdateActionInputBuilder) -> impl Future<Output = Result<BatchStopUpdateActionOutput, SdkError<BatchStopUpdateActionError>>> {
        self.deref().batch_stop_update_action(builder)
    }
    fn complete_migration(&self, builder: CompleteMigrationInputBuilder) -> impl Future<Output = Result<CompleteMigrationOutput, SdkError<CompleteMigrationError>>> {
        self.deref().complete_migration(builder)
    }
    fn copy_serverless_cache_snapshot(&self, builder: CopyServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CopyServerlessCacheSnapshotOutput, SdkError<CopyServerlessCacheSnapshotError>>> {
        self.deref().copy_serverless_cache_snapshot(builder)
    }
    fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> impl Future<Output = Result<CopySnapshotOutput, SdkError<CopySnapshotError>>> {
        self.deref().copy_snapshot(builder)
    }
    fn create_cache_cluster(&self, builder: CreateCacheClusterInputBuilder) -> impl Future<Output = Result<CreateCacheClusterOutput, SdkError<CreateCacheClusterError>>> {
        self.deref().create_cache_cluster(builder)
    }
    fn create_cache_parameter_group(&self, builder: CreateCacheParameterGroupInputBuilder) -> impl Future<Output = Result<CreateCacheParameterGroupOutput, SdkError<CreateCacheParameterGroupError>>> {
        self.deref().create_cache_parameter_group(builder)
    }
    fn create_cache_security_group(&self, builder: CreateCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSecurityGroupOutput, SdkError<CreateCacheSecurityGroupError>>> {
        self.deref().create_cache_security_group(builder)
    }
    fn create_cache_subnet_group(&self, builder: CreateCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<CreateCacheSubnetGroupOutput, SdkError<CreateCacheSubnetGroupError>>> {
        self.deref().create_cache_subnet_group(builder)
    }
    fn create_global_replication_group(&self, builder: CreateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateGlobalReplicationGroupOutput, SdkError<CreateGlobalReplicationGroupError>>> {
        self.deref().create_global_replication_group(builder)
    }
    fn create_replication_group(&self, builder: CreateReplicationGroupInputBuilder) -> impl Future<Output = Result<CreateReplicationGroupOutput, SdkError<CreateReplicationGroupError>>> {
        self.deref().create_replication_group(builder)
    }
    fn create_serverless_cache(&self, builder: CreateServerlessCacheInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheOutput, SdkError<CreateServerlessCacheError>>> {
        self.deref().create_serverless_cache(builder)
    }
    fn create_serverless_cache_snapshot(&self, builder: CreateServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<CreateServerlessCacheSnapshotOutput, SdkError<CreateServerlessCacheSnapshotError>>> {
        self.deref().create_serverless_cache_snapshot(builder)
    }
    fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> impl Future<Output = Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>> {
        self.deref().create_snapshot(builder)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        self.deref().create_user(builder)
    }
    fn create_user_group(&self, builder: CreateUserGroupInputBuilder) -> impl Future<Output = Result<CreateUserGroupOutput, SdkError<CreateUserGroupError>>> {
        self.deref().create_user_group(builder)
    }
    fn decrease_node_groups_in_global_replication_group(&self, builder: DecreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DecreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<DecreaseNodeGroupsInGlobalReplicationGroupError>>> {
        self.deref().decrease_node_groups_in_global_replication_group(builder)
    }
    fn decrease_replica_count(&self, builder: DecreaseReplicaCountInputBuilder) -> impl Future<Output = Result<DecreaseReplicaCountOutput, SdkError<DecreaseReplicaCountError>>> {
        self.deref().decrease_replica_count(builder)
    }
    fn delete_cache_cluster(&self, builder: DeleteCacheClusterInputBuilder) -> impl Future<Output = Result<DeleteCacheClusterOutput, SdkError<DeleteCacheClusterError>>> {
        self.deref().delete_cache_cluster(builder)
    }
    fn delete_cache_parameter_group(&self, builder: DeleteCacheParameterGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheParameterGroupOutput, SdkError<DeleteCacheParameterGroupError>>> {
        self.deref().delete_cache_parameter_group(builder)
    }
    fn delete_cache_security_group(&self, builder: DeleteCacheSecurityGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSecurityGroupOutput, SdkError<DeleteCacheSecurityGroupError>>> {
        self.deref().delete_cache_security_group(builder)
    }
    fn delete_cache_subnet_group(&self, builder: DeleteCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<DeleteCacheSubnetGroupOutput, SdkError<DeleteCacheSubnetGroupError>>> {
        self.deref().delete_cache_subnet_group(builder)
    }
    fn delete_global_replication_group(&self, builder: DeleteGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteGlobalReplicationGroupOutput, SdkError<DeleteGlobalReplicationGroupError>>> {
        self.deref().delete_global_replication_group(builder)
    }
    fn delete_replication_group(&self, builder: DeleteReplicationGroupInputBuilder) -> impl Future<Output = Result<DeleteReplicationGroupOutput, SdkError<DeleteReplicationGroupError>>> {
        self.deref().delete_replication_group(builder)
    }
    fn delete_serverless_cache(&self, builder: DeleteServerlessCacheInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheOutput, SdkError<DeleteServerlessCacheError>>> {
        self.deref().delete_serverless_cache(builder)
    }
    fn delete_serverless_cache_snapshot(&self, builder: DeleteServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<DeleteServerlessCacheSnapshotOutput, SdkError<DeleteServerlessCacheSnapshotError>>> {
        self.deref().delete_serverless_cache_snapshot(builder)
    }
    fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> impl Future<Output = Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>> {
        self.deref().delete_snapshot(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        self.deref().delete_user(builder)
    }
    fn delete_user_group(&self, builder: DeleteUserGroupInputBuilder) -> impl Future<Output = Result<DeleteUserGroupOutput, SdkError<DeleteUserGroupError>>> {
        self.deref().delete_user_group(builder)
    }
    fn describe_cache_clusters(&self, builder: DescribeCacheClustersInputBuilder) -> impl Future<Output = Result<DescribeCacheClustersOutput, SdkError<DescribeCacheClustersError>>> {
        self.deref().describe_cache_clusters(builder)
    }
    fn describe_cache_engine_versions(&self, builder: DescribeCacheEngineVersionsInputBuilder) -> impl Future<Output = Result<DescribeCacheEngineVersionsOutput, SdkError<DescribeCacheEngineVersionsError>>> {
        self.deref().describe_cache_engine_versions(builder)
    }
    fn describe_cache_parameter_groups(&self, builder: DescribeCacheParameterGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheParameterGroupsOutput, SdkError<DescribeCacheParameterGroupsError>>> {
        self.deref().describe_cache_parameter_groups(builder)
    }
    fn describe_cache_parameters(&self, builder: DescribeCacheParametersInputBuilder) -> impl Future<Output = Result<DescribeCacheParametersOutput, SdkError<DescribeCacheParametersError>>> {
        self.deref().describe_cache_parameters(builder)
    }
    fn describe_cache_security_groups(&self, builder: DescribeCacheSecurityGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSecurityGroupsOutput, SdkError<DescribeCacheSecurityGroupsError>>> {
        self.deref().describe_cache_security_groups(builder)
    }
    fn describe_cache_subnet_groups(&self, builder: DescribeCacheSubnetGroupsInputBuilder) -> impl Future<Output = Result<DescribeCacheSubnetGroupsOutput, SdkError<DescribeCacheSubnetGroupsError>>> {
        self.deref().describe_cache_subnet_groups(builder)
    }
    fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> impl Future<Output = Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>> {
        self.deref().describe_engine_default_parameters(builder)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        self.deref().describe_events(builder)
    }
    fn describe_global_replication_groups(&self, builder: DescribeGlobalReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeGlobalReplicationGroupsOutput, SdkError<DescribeGlobalReplicationGroupsError>>> {
        self.deref().describe_global_replication_groups(builder)
    }
    fn describe_replication_groups(&self, builder: DescribeReplicationGroupsInputBuilder) -> impl Future<Output = Result<DescribeReplicationGroupsOutput, SdkError<DescribeReplicationGroupsError>>> {
        self.deref().describe_replication_groups(builder)
    }
    fn describe_reserved_cache_nodes(&self, builder: DescribeReservedCacheNodesInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOutput, SdkError<DescribeReservedCacheNodesError>>> {
        self.deref().describe_reserved_cache_nodes(builder)
    }
    fn describe_reserved_cache_nodes_offerings(&self, builder: DescribeReservedCacheNodesOfferingsInputBuilder) -> impl Future<Output = Result<DescribeReservedCacheNodesOfferingsOutput, SdkError<DescribeReservedCacheNodesOfferingsError>>> {
        self.deref().describe_reserved_cache_nodes_offerings(builder)
    }
    fn describe_serverless_cache_snapshots(&self, builder: DescribeServerlessCacheSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeServerlessCacheSnapshotsOutput, SdkError<DescribeServerlessCacheSnapshotsError>>> {
        self.deref().describe_serverless_cache_snapshots(builder)
    }
    fn describe_serverless_caches(&self, builder: DescribeServerlessCachesInputBuilder) -> impl Future<Output = Result<DescribeServerlessCachesOutput, SdkError<DescribeServerlessCachesError>>> {
        self.deref().describe_serverless_caches(builder)
    }
    fn describe_service_updates(&self, builder: DescribeServiceUpdatesInputBuilder) -> impl Future<Output = Result<DescribeServiceUpdatesOutput, SdkError<DescribeServiceUpdatesError>>> {
        self.deref().describe_service_updates(builder)
    }
    fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> impl Future<Output = Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>> {
        self.deref().describe_snapshots(builder)
    }
    fn describe_update_actions(&self, builder: DescribeUpdateActionsInputBuilder) -> impl Future<Output = Result<DescribeUpdateActionsOutput, SdkError<DescribeUpdateActionsError>>> {
        self.deref().describe_update_actions(builder)
    }
    fn describe_user_groups(&self, builder: DescribeUserGroupsInputBuilder) -> impl Future<Output = Result<DescribeUserGroupsOutput, SdkError<DescribeUserGroupsError>>> {
        self.deref().describe_user_groups(builder)
    }
    fn describe_users(&self, builder: DescribeUsersInputBuilder) -> impl Future<Output = Result<DescribeUsersOutput, SdkError<DescribeUsersError>>> {
        self.deref().describe_users(builder)
    }
    fn disassociate_global_replication_group(&self, builder: DisassociateGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<DisassociateGlobalReplicationGroupOutput, SdkError<DisassociateGlobalReplicationGroupError>>> {
        self.deref().disassociate_global_replication_group(builder)
    }
    fn export_serverless_cache_snapshot(&self, builder: ExportServerlessCacheSnapshotInputBuilder) -> impl Future<Output = Result<ExportServerlessCacheSnapshotOutput, SdkError<ExportServerlessCacheSnapshotError>>> {
        self.deref().export_serverless_cache_snapshot(builder)
    }
    fn failover_global_replication_group(&self, builder: FailoverGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<FailoverGlobalReplicationGroupOutput, SdkError<FailoverGlobalReplicationGroupError>>> {
        self.deref().failover_global_replication_group(builder)
    }
    fn increase_node_groups_in_global_replication_group(&self, builder: IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<IncreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<IncreaseNodeGroupsInGlobalReplicationGroupError>>> {
        self.deref().increase_node_groups_in_global_replication_group(builder)
    }
    fn increase_replica_count(&self, builder: IncreaseReplicaCountInputBuilder) -> impl Future<Output = Result<IncreaseReplicaCountOutput, SdkError<IncreaseReplicaCountError>>> {
        self.deref().increase_replica_count(builder)
    }
    fn list_allowed_node_type_modifications(&self, builder: ListAllowedNodeTypeModificationsInputBuilder) -> impl Future<Output = Result<ListAllowedNodeTypeModificationsOutput, SdkError<ListAllowedNodeTypeModificationsError>>> {
        self.deref().list_allowed_node_type_modifications(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn modify_cache_cluster(&self, builder: ModifyCacheClusterInputBuilder) -> impl Future<Output = Result<ModifyCacheClusterOutput, SdkError<ModifyCacheClusterError>>> {
        self.deref().modify_cache_cluster(builder)
    }
    fn modify_cache_parameter_group(&self, builder: ModifyCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheParameterGroupOutput, SdkError<ModifyCacheParameterGroupError>>> {
        self.deref().modify_cache_parameter_group(builder)
    }
    fn modify_cache_subnet_group(&self, builder: ModifyCacheSubnetGroupInputBuilder) -> impl Future<Output = Result<ModifyCacheSubnetGroupOutput, SdkError<ModifyCacheSubnetGroupError>>> {
        self.deref().modify_cache_subnet_group(builder)
    }
    fn modify_global_replication_group(&self, builder: ModifyGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyGlobalReplicationGroupOutput, SdkError<ModifyGlobalReplicationGroupError>>> {
        self.deref().modify_global_replication_group(builder)
    }
    fn modify_replication_group(&self, builder: ModifyReplicationGroupInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupOutput, SdkError<ModifyReplicationGroupError>>> {
        self.deref().modify_replication_group(builder)
    }
    fn modify_replication_group_shard_configuration(&self, builder: ModifyReplicationGroupShardConfigurationInputBuilder) -> impl Future<Output = Result<ModifyReplicationGroupShardConfigurationOutput, SdkError<ModifyReplicationGroupShardConfigurationError>>> {
        self.deref().modify_replication_group_shard_configuration(builder)
    }
    fn modify_serverless_cache(&self, builder: ModifyServerlessCacheInputBuilder) -> impl Future<Output = Result<ModifyServerlessCacheOutput, SdkError<ModifyServerlessCacheError>>> {
        self.deref().modify_serverless_cache(builder)
    }
    fn modify_user(&self, builder: ModifyUserInputBuilder) -> impl Future<Output = Result<ModifyUserOutput, SdkError<ModifyUserError>>> {
        self.deref().modify_user(builder)
    }
    fn modify_user_group(&self, builder: ModifyUserGroupInputBuilder) -> impl Future<Output = Result<ModifyUserGroupOutput, SdkError<ModifyUserGroupError>>> {
        self.deref().modify_user_group(builder)
    }
    fn purchase_reserved_cache_nodes_offering(&self, builder: PurchaseReservedCacheNodesOfferingInputBuilder) -> impl Future<Output = Result<PurchaseReservedCacheNodesOfferingOutput, SdkError<PurchaseReservedCacheNodesOfferingError>>> {
        self.deref().purchase_reserved_cache_nodes_offering(builder)
    }
    fn rebalance_slots_in_global_replication_group(&self, builder: RebalanceSlotsInGlobalReplicationGroupInputBuilder) -> impl Future<Output = Result<RebalanceSlotsInGlobalReplicationGroupOutput, SdkError<RebalanceSlotsInGlobalReplicationGroupError>>> {
        self.deref().rebalance_slots_in_global_replication_group(builder)
    }
    fn reboot_cache_cluster(&self, builder: RebootCacheClusterInputBuilder) -> impl Future<Output = Result<RebootCacheClusterOutput, SdkError<RebootCacheClusterError>>> {
        self.deref().reboot_cache_cluster(builder)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        self.deref().remove_tags_from_resource(builder)
    }
    fn reset_cache_parameter_group(&self, builder: ResetCacheParameterGroupInputBuilder) -> impl Future<Output = Result<ResetCacheParameterGroupOutput, SdkError<ResetCacheParameterGroupError>>> {
        self.deref().reset_cache_parameter_group(builder)
    }
    fn revoke_cache_security_group_ingress(&self, builder: RevokeCacheSecurityGroupIngressInputBuilder) -> impl Future<Output = Result<RevokeCacheSecurityGroupIngressOutput, SdkError<RevokeCacheSecurityGroupIngressError>>> {
        self.deref().revoke_cache_security_group_ingress(builder)
    }
    fn start_migration(&self, builder: StartMigrationInputBuilder) -> impl Future<Output = Result<StartMigrationOutput, SdkError<StartMigrationError>>> {
        self.deref().start_migration(builder)
    }
    fn test_failover(&self, builder: TestFailoverInputBuilder) -> impl Future<Output = Result<TestFailoverOutput, SdkError<TestFailoverError>>> {
        self.deref().test_failover(builder)
    }
    fn test_migration(&self, builder: TestMigrationInputBuilder) -> impl Future<Output = Result<TestMigrationOutput, SdkError<TestMigrationError>>> {
        self.deref().test_migration(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edElastiCacheClient {}
    impl ElastiCacheClient for edElastiCacheClient {
        async fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>;
        async fn authorize_cache_security_group_ingress(&self, builder: AuthorizeCacheSecurityGroupIngressInputBuilder) -> Result<AuthorizeCacheSecurityGroupIngressOutput, SdkError<AuthorizeCacheSecurityGroupIngressError>>;
        async fn batch_apply_update_action(&self, builder: BatchApplyUpdateActionInputBuilder) -> Result<BatchApplyUpdateActionOutput, SdkError<BatchApplyUpdateActionError>>;
        async fn batch_stop_update_action(&self, builder: BatchStopUpdateActionInputBuilder) -> Result<BatchStopUpdateActionOutput, SdkError<BatchStopUpdateActionError>>;
        async fn complete_migration(&self, builder: CompleteMigrationInputBuilder) -> Result<CompleteMigrationOutput, SdkError<CompleteMigrationError>>;
        async fn copy_serverless_cache_snapshot(&self, builder: CopyServerlessCacheSnapshotInputBuilder) -> Result<CopyServerlessCacheSnapshotOutput, SdkError<CopyServerlessCacheSnapshotError>>;
        async fn copy_snapshot(&self, builder: CopySnapshotInputBuilder) -> Result<CopySnapshotOutput, SdkError<CopySnapshotError>>;
        async fn create_cache_cluster(&self, builder: CreateCacheClusterInputBuilder) -> Result<CreateCacheClusterOutput, SdkError<CreateCacheClusterError>>;
        async fn create_cache_parameter_group(&self, builder: CreateCacheParameterGroupInputBuilder) -> Result<CreateCacheParameterGroupOutput, SdkError<CreateCacheParameterGroupError>>;
        async fn create_cache_security_group(&self, builder: CreateCacheSecurityGroupInputBuilder) -> Result<CreateCacheSecurityGroupOutput, SdkError<CreateCacheSecurityGroupError>>;
        async fn create_cache_subnet_group(&self, builder: CreateCacheSubnetGroupInputBuilder) -> Result<CreateCacheSubnetGroupOutput, SdkError<CreateCacheSubnetGroupError>>;
        async fn create_global_replication_group(&self, builder: CreateGlobalReplicationGroupInputBuilder) -> Result<CreateGlobalReplicationGroupOutput, SdkError<CreateGlobalReplicationGroupError>>;
        async fn create_replication_group(&self, builder: CreateReplicationGroupInputBuilder) -> Result<CreateReplicationGroupOutput, SdkError<CreateReplicationGroupError>>;
        async fn create_serverless_cache(&self, builder: CreateServerlessCacheInputBuilder) -> Result<CreateServerlessCacheOutput, SdkError<CreateServerlessCacheError>>;
        async fn create_serverless_cache_snapshot(&self, builder: CreateServerlessCacheSnapshotInputBuilder) -> Result<CreateServerlessCacheSnapshotOutput, SdkError<CreateServerlessCacheSnapshotError>>;
        async fn create_snapshot(&self, builder: CreateSnapshotInputBuilder) -> Result<CreateSnapshotOutput, SdkError<CreateSnapshotError>>;
        async fn create_user(&self, builder: CreateUserInputBuilder) -> Result<CreateUserOutput, SdkError<CreateUserError>>;
        async fn create_user_group(&self, builder: CreateUserGroupInputBuilder) -> Result<CreateUserGroupOutput, SdkError<CreateUserGroupError>>;
        async fn decrease_node_groups_in_global_replication_group(&self, builder: DecreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> Result<DecreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<DecreaseNodeGroupsInGlobalReplicationGroupError>>;
        async fn decrease_replica_count(&self, builder: DecreaseReplicaCountInputBuilder) -> Result<DecreaseReplicaCountOutput, SdkError<DecreaseReplicaCountError>>;
        async fn delete_cache_cluster(&self, builder: DeleteCacheClusterInputBuilder) -> Result<DeleteCacheClusterOutput, SdkError<DeleteCacheClusterError>>;
        async fn delete_cache_parameter_group(&self, builder: DeleteCacheParameterGroupInputBuilder) -> Result<DeleteCacheParameterGroupOutput, SdkError<DeleteCacheParameterGroupError>>;
        async fn delete_cache_security_group(&self, builder: DeleteCacheSecurityGroupInputBuilder) -> Result<DeleteCacheSecurityGroupOutput, SdkError<DeleteCacheSecurityGroupError>>;
        async fn delete_cache_subnet_group(&self, builder: DeleteCacheSubnetGroupInputBuilder) -> Result<DeleteCacheSubnetGroupOutput, SdkError<DeleteCacheSubnetGroupError>>;
        async fn delete_global_replication_group(&self, builder: DeleteGlobalReplicationGroupInputBuilder) -> Result<DeleteGlobalReplicationGroupOutput, SdkError<DeleteGlobalReplicationGroupError>>;
        async fn delete_replication_group(&self, builder: DeleteReplicationGroupInputBuilder) -> Result<DeleteReplicationGroupOutput, SdkError<DeleteReplicationGroupError>>;
        async fn delete_serverless_cache(&self, builder: DeleteServerlessCacheInputBuilder) -> Result<DeleteServerlessCacheOutput, SdkError<DeleteServerlessCacheError>>;
        async fn delete_serverless_cache_snapshot(&self, builder: DeleteServerlessCacheSnapshotInputBuilder) -> Result<DeleteServerlessCacheSnapshotOutput, SdkError<DeleteServerlessCacheSnapshotError>>;
        async fn delete_snapshot(&self, builder: DeleteSnapshotInputBuilder) -> Result<DeleteSnapshotOutput, SdkError<DeleteSnapshotError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn delete_user_group(&self, builder: DeleteUserGroupInputBuilder) -> Result<DeleteUserGroupOutput, SdkError<DeleteUserGroupError>>;
        async fn describe_cache_clusters(&self, builder: DescribeCacheClustersInputBuilder) -> Result<DescribeCacheClustersOutput, SdkError<DescribeCacheClustersError>>;
        async fn describe_cache_engine_versions(&self, builder: DescribeCacheEngineVersionsInputBuilder) -> Result<DescribeCacheEngineVersionsOutput, SdkError<DescribeCacheEngineVersionsError>>;
        async fn describe_cache_parameter_groups(&self, builder: DescribeCacheParameterGroupsInputBuilder) -> Result<DescribeCacheParameterGroupsOutput, SdkError<DescribeCacheParameterGroupsError>>;
        async fn describe_cache_parameters(&self, builder: DescribeCacheParametersInputBuilder) -> Result<DescribeCacheParametersOutput, SdkError<DescribeCacheParametersError>>;
        async fn describe_cache_security_groups(&self, builder: DescribeCacheSecurityGroupsInputBuilder) -> Result<DescribeCacheSecurityGroupsOutput, SdkError<DescribeCacheSecurityGroupsError>>;
        async fn describe_cache_subnet_groups(&self, builder: DescribeCacheSubnetGroupsInputBuilder) -> Result<DescribeCacheSubnetGroupsOutput, SdkError<DescribeCacheSubnetGroupsError>>;
        async fn describe_engine_default_parameters(&self, builder: DescribeEngineDefaultParametersInputBuilder) -> Result<DescribeEngineDefaultParametersOutput, SdkError<DescribeEngineDefaultParametersError>>;
        async fn describe_events(&self, builder: DescribeEventsInputBuilder) -> Result<DescribeEventsOutput, SdkError<DescribeEventsError>>;
        async fn describe_global_replication_groups(&self, builder: DescribeGlobalReplicationGroupsInputBuilder) -> Result<DescribeGlobalReplicationGroupsOutput, SdkError<DescribeGlobalReplicationGroupsError>>;
        async fn describe_replication_groups(&self, builder: DescribeReplicationGroupsInputBuilder) -> Result<DescribeReplicationGroupsOutput, SdkError<DescribeReplicationGroupsError>>;
        async fn describe_reserved_cache_nodes(&self, builder: DescribeReservedCacheNodesInputBuilder) -> Result<DescribeReservedCacheNodesOutput, SdkError<DescribeReservedCacheNodesError>>;
        async fn describe_reserved_cache_nodes_offerings(&self, builder: DescribeReservedCacheNodesOfferingsInputBuilder) -> Result<DescribeReservedCacheNodesOfferingsOutput, SdkError<DescribeReservedCacheNodesOfferingsError>>;
        async fn describe_serverless_cache_snapshots(&self, builder: DescribeServerlessCacheSnapshotsInputBuilder) -> Result<DescribeServerlessCacheSnapshotsOutput, SdkError<DescribeServerlessCacheSnapshotsError>>;
        async fn describe_serverless_caches(&self, builder: DescribeServerlessCachesInputBuilder) -> Result<DescribeServerlessCachesOutput, SdkError<DescribeServerlessCachesError>>;
        async fn describe_service_updates(&self, builder: DescribeServiceUpdatesInputBuilder) -> Result<DescribeServiceUpdatesOutput, SdkError<DescribeServiceUpdatesError>>;
        async fn describe_snapshots(&self, builder: DescribeSnapshotsInputBuilder) -> Result<DescribeSnapshotsOutput, SdkError<DescribeSnapshotsError>>;
        async fn describe_update_actions(&self, builder: DescribeUpdateActionsInputBuilder) -> Result<DescribeUpdateActionsOutput, SdkError<DescribeUpdateActionsError>>;
        async fn describe_user_groups(&self, builder: DescribeUserGroupsInputBuilder) -> Result<DescribeUserGroupsOutput, SdkError<DescribeUserGroupsError>>;
        async fn describe_users(&self, builder: DescribeUsersInputBuilder) -> Result<DescribeUsersOutput, SdkError<DescribeUsersError>>;
        async fn disassociate_global_replication_group(&self, builder: DisassociateGlobalReplicationGroupInputBuilder) -> Result<DisassociateGlobalReplicationGroupOutput, SdkError<DisassociateGlobalReplicationGroupError>>;
        async fn export_serverless_cache_snapshot(&self, builder: ExportServerlessCacheSnapshotInputBuilder) -> Result<ExportServerlessCacheSnapshotOutput, SdkError<ExportServerlessCacheSnapshotError>>;
        async fn failover_global_replication_group(&self, builder: FailoverGlobalReplicationGroupInputBuilder) -> Result<FailoverGlobalReplicationGroupOutput, SdkError<FailoverGlobalReplicationGroupError>>;
        async fn increase_node_groups_in_global_replication_group(&self, builder: IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder) -> Result<IncreaseNodeGroupsInGlobalReplicationGroupOutput, SdkError<IncreaseNodeGroupsInGlobalReplicationGroupError>>;
        async fn increase_replica_count(&self, builder: IncreaseReplicaCountInputBuilder) -> Result<IncreaseReplicaCountOutput, SdkError<IncreaseReplicaCountError>>;
        async fn list_allowed_node_type_modifications(&self, builder: ListAllowedNodeTypeModificationsInputBuilder) -> Result<ListAllowedNodeTypeModificationsOutput, SdkError<ListAllowedNodeTypeModificationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn modify_cache_cluster(&self, builder: ModifyCacheClusterInputBuilder) -> Result<ModifyCacheClusterOutput, SdkError<ModifyCacheClusterError>>;
        async fn modify_cache_parameter_group(&self, builder: ModifyCacheParameterGroupInputBuilder) -> Result<ModifyCacheParameterGroupOutput, SdkError<ModifyCacheParameterGroupError>>;
        async fn modify_cache_subnet_group(&self, builder: ModifyCacheSubnetGroupInputBuilder) -> Result<ModifyCacheSubnetGroupOutput, SdkError<ModifyCacheSubnetGroupError>>;
        async fn modify_global_replication_group(&self, builder: ModifyGlobalReplicationGroupInputBuilder) -> Result<ModifyGlobalReplicationGroupOutput, SdkError<ModifyGlobalReplicationGroupError>>;
        async fn modify_replication_group(&self, builder: ModifyReplicationGroupInputBuilder) -> Result<ModifyReplicationGroupOutput, SdkError<ModifyReplicationGroupError>>;
        async fn modify_replication_group_shard_configuration(&self, builder: ModifyReplicationGroupShardConfigurationInputBuilder) -> Result<ModifyReplicationGroupShardConfigurationOutput, SdkError<ModifyReplicationGroupShardConfigurationError>>;
        async fn modify_serverless_cache(&self, builder: ModifyServerlessCacheInputBuilder) -> Result<ModifyServerlessCacheOutput, SdkError<ModifyServerlessCacheError>>;
        async fn modify_user(&self, builder: ModifyUserInputBuilder) -> Result<ModifyUserOutput, SdkError<ModifyUserError>>;
        async fn modify_user_group(&self, builder: ModifyUserGroupInputBuilder) -> Result<ModifyUserGroupOutput, SdkError<ModifyUserGroupError>>;
        async fn purchase_reserved_cache_nodes_offering(&self, builder: PurchaseReservedCacheNodesOfferingInputBuilder) -> Result<PurchaseReservedCacheNodesOfferingOutput, SdkError<PurchaseReservedCacheNodesOfferingError>>;
        async fn rebalance_slots_in_global_replication_group(&self, builder: RebalanceSlotsInGlobalReplicationGroupInputBuilder) -> Result<RebalanceSlotsInGlobalReplicationGroupOutput, SdkError<RebalanceSlotsInGlobalReplicationGroupError>>;
        async fn reboot_cache_cluster(&self, builder: RebootCacheClusterInputBuilder) -> Result<RebootCacheClusterOutput, SdkError<RebootCacheClusterError>>;
        async fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>;
        async fn reset_cache_parameter_group(&self, builder: ResetCacheParameterGroupInputBuilder) -> Result<ResetCacheParameterGroupOutput, SdkError<ResetCacheParameterGroupError>>;
        async fn revoke_cache_security_group_ingress(&self, builder: RevokeCacheSecurityGroupIngressInputBuilder) -> Result<RevokeCacheSecurityGroupIngressOutput, SdkError<RevokeCacheSecurityGroupIngressError>>;
        async fn start_migration(&self, builder: StartMigrationInputBuilder) -> Result<StartMigrationOutput, SdkError<StartMigrationError>>;
        async fn test_failover(&self, builder: TestFailoverInputBuilder) -> Result<TestFailoverOutput, SdkError<TestFailoverError>>;
        async fn test_migration(&self, builder: TestMigrationInputBuilder) -> Result<TestMigrationOutput, SdkError<TestMigrationError>>;
    }
}
