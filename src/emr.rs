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
use aws_sdk_emr::operation::add_instance_fleet::{builders::*, *};
use aws_sdk_emr::operation::add_instance_groups::{builders::*, *};
use aws_sdk_emr::operation::add_tags::{builders::*, *};
use aws_sdk_emr::operation::cancel_steps::{builders::*, *};
use aws_sdk_emr::operation::create_security_configuration::{builders::*, *};
use aws_sdk_emr::operation::create_studio::{builders::*, *};
use aws_sdk_emr::operation::create_studio_session_mapping::{builders::*, *};
use aws_sdk_emr::operation::delete_security_configuration::{builders::*, *};
use aws_sdk_emr::operation::delete_studio::{builders::*, *};
use aws_sdk_emr::operation::delete_studio_session_mapping::{builders::*, *};
use aws_sdk_emr::operation::describe_cluster::{builders::*, *};
use aws_sdk_emr::operation::describe_notebook_execution::{builders::*, *};
use aws_sdk_emr::operation::describe_release_label::{builders::*, *};
use aws_sdk_emr::operation::describe_security_configuration::{builders::*, *};
use aws_sdk_emr::operation::describe_step::{builders::*, *};
use aws_sdk_emr::operation::describe_studio::{builders::*, *};
use aws_sdk_emr::operation::get_auto_termination_policy::{builders::*, *};
use aws_sdk_emr::operation::get_block_public_access_configuration::{builders::*, *};
use aws_sdk_emr::operation::get_cluster_session_credentials::{builders::*, *};
use aws_sdk_emr::operation::get_managed_scaling_policy::{builders::*, *};
use aws_sdk_emr::operation::get_studio_session_mapping::{builders::*, *};
use aws_sdk_emr::operation::list_clusters::{builders::*, *};
use aws_sdk_emr::operation::list_instance_fleets::{builders::*, *};
use aws_sdk_emr::operation::list_instances::{builders::*, *};
use aws_sdk_emr::operation::list_notebook_executions::{builders::*, *};
use aws_sdk_emr::operation::list_release_labels::{builders::*, *};
use aws_sdk_emr::operation::list_security_configurations::{builders::*, *};
use aws_sdk_emr::operation::list_steps::{builders::*, *};
use aws_sdk_emr::operation::list_studio_session_mappings::{builders::*, *};
use aws_sdk_emr::operation::list_studios::{builders::*, *};
use aws_sdk_emr::operation::list_supported_instance_types::{builders::*, *};
use aws_sdk_emr::operation::modify_cluster::{builders::*, *};
use aws_sdk_emr::operation::modify_instance_fleet::{builders::*, *};
use aws_sdk_emr::operation::modify_instance_groups::{builders::*, *};
use aws_sdk_emr::operation::put_auto_scaling_policy::{builders::*, *};
use aws_sdk_emr::operation::put_auto_termination_policy::{builders::*, *};
use aws_sdk_emr::operation::put_block_public_access_configuration::{builders::*, *};
use aws_sdk_emr::operation::put_managed_scaling_policy::{builders::*, *};
use aws_sdk_emr::operation::remove_auto_scaling_policy::{builders::*, *};
use aws_sdk_emr::operation::remove_auto_termination_policy::{builders::*, *};
use aws_sdk_emr::operation::remove_managed_scaling_policy::{builders::*, *};
use aws_sdk_emr::operation::remove_tags::{builders::*, *};
use aws_sdk_emr::operation::start_notebook_execution::{builders::*, *};
use aws_sdk_emr::operation::stop_notebook_execution::{builders::*, *};
use aws_sdk_emr::operation::update_studio::{builders::*, *};
use aws_sdk_emr::operation::update_studio_session_mapping::{builders::*, *};
use aws_sdk_emr::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_emr::Client;
use std::ops::Deref;

pub use aws_sdk_emr::*;

pub struct EMRClientImpl(Client);
impl EMRClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait EMRClient {
    fn add_instance_fleet(&self, builder: AddInstanceFleetInputBuilder) -> impl Future<Output = Result<AddInstanceFleetOutput, SdkError<AddInstanceFleetError>>> + Send;
    fn add_instance_groups(&self, builder: AddInstanceGroupsInputBuilder) -> impl Future<Output = Result<AddInstanceGroupsOutput, SdkError<AddInstanceGroupsError>>> + Send;
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> + Send;
    fn cancel_steps(&self, builder: CancelStepsInputBuilder) -> impl Future<Output = Result<CancelStepsOutput, SdkError<CancelStepsError>>> + Send;
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>> + Send;
    fn create_studio(&self, builder: CreateStudioInputBuilder) -> impl Future<Output = Result<CreateStudioOutput, SdkError<CreateStudioError>>> + Send;
    fn create_studio_session_mapping(&self, builder: CreateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<CreateStudioSessionMappingOutput, SdkError<CreateStudioSessionMappingError>>> + Send;
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>> + Send;
    fn delete_studio(&self, builder: DeleteStudioInputBuilder) -> impl Future<Output = Result<DeleteStudioOutput, SdkError<DeleteStudioError>>> + Send;
    fn delete_studio_session_mapping(&self, builder: DeleteStudioSessionMappingInputBuilder) -> impl Future<Output = Result<DeleteStudioSessionMappingOutput, SdkError<DeleteStudioSessionMappingError>>> + Send;
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> + Send;
    fn describe_notebook_execution(&self, builder: DescribeNotebookExecutionInputBuilder) -> impl Future<Output = Result<DescribeNotebookExecutionOutput, SdkError<DescribeNotebookExecutionError>>> + Send;
    fn describe_release_label(&self, builder: DescribeReleaseLabelInputBuilder) -> impl Future<Output = Result<DescribeReleaseLabelOutput, SdkError<DescribeReleaseLabelError>>> + Send;
    fn describe_security_configuration(&self, builder: DescribeSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeSecurityConfigurationOutput, SdkError<DescribeSecurityConfigurationError>>> + Send;
    fn describe_step(&self, builder: DescribeStepInputBuilder) -> impl Future<Output = Result<DescribeStepOutput, SdkError<DescribeStepError>>> + Send;
    fn describe_studio(&self, builder: DescribeStudioInputBuilder) -> impl Future<Output = Result<DescribeStudioOutput, SdkError<DescribeStudioError>>> + Send;
    fn get_auto_termination_policy(&self, builder: GetAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<GetAutoTerminationPolicyOutput, SdkError<GetAutoTerminationPolicyError>>> + Send;
    fn get_block_public_access_configuration(&self, builder: GetBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<GetBlockPublicAccessConfigurationOutput, SdkError<GetBlockPublicAccessConfigurationError>>> + Send;
    fn get_cluster_session_credentials(&self, builder: GetClusterSessionCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterSessionCredentialsOutput, SdkError<GetClusterSessionCredentialsError>>> + Send;
    fn get_managed_scaling_policy(&self, builder: GetManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<GetManagedScalingPolicyOutput, SdkError<GetManagedScalingPolicyError>>> + Send;
    fn get_studio_session_mapping(&self, builder: GetStudioSessionMappingInputBuilder) -> impl Future<Output = Result<GetStudioSessionMappingOutput, SdkError<GetStudioSessionMappingError>>> + Send;
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> + Send;
    fn list_instance_fleets(&self, builder: ListInstanceFleetsInputBuilder) -> impl Future<Output = Result<ListInstanceFleetsOutput, SdkError<ListInstanceFleetsError>>> + Send;
    fn list_instances(&self, builder: ListInstancesInputBuilder) -> impl Future<Output = Result<ListInstancesOutput, SdkError<ListInstancesError>>> + Send;
    fn list_notebook_executions(&self, builder: ListNotebookExecutionsInputBuilder) -> impl Future<Output = Result<ListNotebookExecutionsOutput, SdkError<ListNotebookExecutionsError>>> + Send;
    fn list_release_labels(&self, builder: ListReleaseLabelsInputBuilder) -> impl Future<Output = Result<ListReleaseLabelsOutput, SdkError<ListReleaseLabelsError>>> + Send;
    fn list_security_configurations(&self, builder: ListSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<ListSecurityConfigurationsOutput, SdkError<ListSecurityConfigurationsError>>> + Send;
    fn list_steps(&self, builder: ListStepsInputBuilder) -> impl Future<Output = Result<ListStepsOutput, SdkError<ListStepsError>>> + Send;
    fn list_studio_session_mappings(&self, builder: ListStudioSessionMappingsInputBuilder) -> impl Future<Output = Result<ListStudioSessionMappingsOutput, SdkError<ListStudioSessionMappingsError>>> + Send;
    fn list_studios(&self, builder: ListStudiosInputBuilder) -> impl Future<Output = Result<ListStudiosOutput, SdkError<ListStudiosError>>> + Send;
    fn list_supported_instance_types(&self, builder: ListSupportedInstanceTypesInputBuilder) -> impl Future<Output = Result<ListSupportedInstanceTypesOutput, SdkError<ListSupportedInstanceTypesError>>> + Send;
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>> + Send;
    fn modify_instance_fleet(&self, builder: ModifyInstanceFleetInputBuilder) -> impl Future<Output = Result<ModifyInstanceFleetOutput, SdkError<ModifyInstanceFleetError>>> + Send;
    fn modify_instance_groups(&self, builder: ModifyInstanceGroupsInputBuilder) -> impl Future<Output = Result<ModifyInstanceGroupsOutput, SdkError<ModifyInstanceGroupsError>>> + Send;
    fn put_auto_scaling_policy(&self, builder: PutAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<PutAutoScalingPolicyOutput, SdkError<PutAutoScalingPolicyError>>> + Send;
    fn put_auto_termination_policy(&self, builder: PutAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<PutAutoTerminationPolicyOutput, SdkError<PutAutoTerminationPolicyError>>> + Send;
    fn put_block_public_access_configuration(&self, builder: PutBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<PutBlockPublicAccessConfigurationOutput, SdkError<PutBlockPublicAccessConfigurationError>>> + Send;
    fn put_managed_scaling_policy(&self, builder: PutManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<PutManagedScalingPolicyOutput, SdkError<PutManagedScalingPolicyError>>> + Send;
    fn remove_auto_scaling_policy(&self, builder: RemoveAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoScalingPolicyOutput, SdkError<RemoveAutoScalingPolicyError>>> + Send;
    fn remove_auto_termination_policy(&self, builder: RemoveAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoTerminationPolicyOutput, SdkError<RemoveAutoTerminationPolicyError>>> + Send;
    fn remove_managed_scaling_policy(&self, builder: RemoveManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveManagedScalingPolicyOutput, SdkError<RemoveManagedScalingPolicyError>>> + Send;
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> + Send;
    fn start_notebook_execution(&self, builder: StartNotebookExecutionInputBuilder) -> impl Future<Output = Result<StartNotebookExecutionOutput, SdkError<StartNotebookExecutionError>>> + Send;
    fn stop_notebook_execution(&self, builder: StopNotebookExecutionInputBuilder) -> impl Future<Output = Result<StopNotebookExecutionOutput, SdkError<StopNotebookExecutionError>>> + Send;
    fn update_studio(&self, builder: UpdateStudioInputBuilder) -> impl Future<Output = Result<UpdateStudioOutput, SdkError<UpdateStudioError>>> + Send;
    fn update_studio_session_mapping(&self, builder: UpdateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<UpdateStudioSessionMappingOutput, SdkError<UpdateStudioSessionMappingError>>> + Send;
}
impl EMRClient for EMRClientImpl {
    fn add_instance_fleet(&self, builder: AddInstanceFleetInputBuilder) -> impl Future<Output = Result<AddInstanceFleetOutput, SdkError<AddInstanceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn add_instance_groups(&self, builder: AddInstanceGroupsInputBuilder) -> impl Future<Output = Result<AddInstanceGroupsOutput, SdkError<AddInstanceGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_steps(&self, builder: CancelStepsInputBuilder) -> impl Future<Output = Result<CancelStepsOutput, SdkError<CancelStepsError>>> {
        builder.send_with(&self.0)
    }
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_studio(&self, builder: CreateStudioInputBuilder) -> impl Future<Output = Result<CreateStudioOutput, SdkError<CreateStudioError>>> {
        builder.send_with(&self.0)
    }
    fn create_studio_session_mapping(&self, builder: CreateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<CreateStudioSessionMappingOutput, SdkError<CreateStudioSessionMappingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_studio(&self, builder: DeleteStudioInputBuilder) -> impl Future<Output = Result<DeleteStudioOutput, SdkError<DeleteStudioError>>> {
        builder.send_with(&self.0)
    }
    fn delete_studio_session_mapping(&self, builder: DeleteStudioSessionMappingInputBuilder) -> impl Future<Output = Result<DeleteStudioSessionMappingOutput, SdkError<DeleteStudioSessionMappingError>>> {
        builder.send_with(&self.0)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_notebook_execution(&self, builder: DescribeNotebookExecutionInputBuilder) -> impl Future<Output = Result<DescribeNotebookExecutionOutput, SdkError<DescribeNotebookExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_release_label(&self, builder: DescribeReleaseLabelInputBuilder) -> impl Future<Output = Result<DescribeReleaseLabelOutput, SdkError<DescribeReleaseLabelError>>> {
        builder.send_with(&self.0)
    }
    fn describe_security_configuration(&self, builder: DescribeSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeSecurityConfigurationOutput, SdkError<DescribeSecurityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_step(&self, builder: DescribeStepInputBuilder) -> impl Future<Output = Result<DescribeStepOutput, SdkError<DescribeStepError>>> {
        builder.send_with(&self.0)
    }
    fn describe_studio(&self, builder: DescribeStudioInputBuilder) -> impl Future<Output = Result<DescribeStudioOutput, SdkError<DescribeStudioError>>> {
        builder.send_with(&self.0)
    }
    fn get_auto_termination_policy(&self, builder: GetAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<GetAutoTerminationPolicyOutput, SdkError<GetAutoTerminationPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_block_public_access_configuration(&self, builder: GetBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<GetBlockPublicAccessConfigurationOutput, SdkError<GetBlockPublicAccessConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_cluster_session_credentials(&self, builder: GetClusterSessionCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterSessionCredentialsOutput, SdkError<GetClusterSessionCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn get_managed_scaling_policy(&self, builder: GetManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<GetManagedScalingPolicyOutput, SdkError<GetManagedScalingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_studio_session_mapping(&self, builder: GetStudioSessionMappingInputBuilder) -> impl Future<Output = Result<GetStudioSessionMappingOutput, SdkError<GetStudioSessionMappingError>>> {
        builder.send_with(&self.0)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        builder.send_with(&self.0)
    }
    fn list_instance_fleets(&self, builder: ListInstanceFleetsInputBuilder) -> impl Future<Output = Result<ListInstanceFleetsOutput, SdkError<ListInstanceFleetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_instances(&self, builder: ListInstancesInputBuilder) -> impl Future<Output = Result<ListInstancesOutput, SdkError<ListInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn list_notebook_executions(&self, builder: ListNotebookExecutionsInputBuilder) -> impl Future<Output = Result<ListNotebookExecutionsOutput, SdkError<ListNotebookExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_release_labels(&self, builder: ListReleaseLabelsInputBuilder) -> impl Future<Output = Result<ListReleaseLabelsOutput, SdkError<ListReleaseLabelsError>>> {
        builder.send_with(&self.0)
    }
    fn list_security_configurations(&self, builder: ListSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<ListSecurityConfigurationsOutput, SdkError<ListSecurityConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_steps(&self, builder: ListStepsInputBuilder) -> impl Future<Output = Result<ListStepsOutput, SdkError<ListStepsError>>> {
        builder.send_with(&self.0)
    }
    fn list_studio_session_mappings(&self, builder: ListStudioSessionMappingsInputBuilder) -> impl Future<Output = Result<ListStudioSessionMappingsOutput, SdkError<ListStudioSessionMappingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_studios(&self, builder: ListStudiosInputBuilder) -> impl Future<Output = Result<ListStudiosOutput, SdkError<ListStudiosError>>> {
        builder.send_with(&self.0)
    }
    fn list_supported_instance_types(&self, builder: ListSupportedInstanceTypesInputBuilder) -> impl Future<Output = Result<ListSupportedInstanceTypesOutput, SdkError<ListSupportedInstanceTypesError>>> {
        builder.send_with(&self.0)
    }
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_fleet(&self, builder: ModifyInstanceFleetInputBuilder) -> impl Future<Output = Result<ModifyInstanceFleetOutput, SdkError<ModifyInstanceFleetError>>> {
        builder.send_with(&self.0)
    }
    fn modify_instance_groups(&self, builder: ModifyInstanceGroupsInputBuilder) -> impl Future<Output = Result<ModifyInstanceGroupsOutput, SdkError<ModifyInstanceGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn put_auto_scaling_policy(&self, builder: PutAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<PutAutoScalingPolicyOutput, SdkError<PutAutoScalingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_auto_termination_policy(&self, builder: PutAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<PutAutoTerminationPolicyOutput, SdkError<PutAutoTerminationPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_block_public_access_configuration(&self, builder: PutBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<PutBlockPublicAccessConfigurationOutput, SdkError<PutBlockPublicAccessConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_managed_scaling_policy(&self, builder: PutManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<PutManagedScalingPolicyOutput, SdkError<PutManagedScalingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_auto_scaling_policy(&self, builder: RemoveAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoScalingPolicyOutput, SdkError<RemoveAutoScalingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_auto_termination_policy(&self, builder: RemoveAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoTerminationPolicyOutput, SdkError<RemoveAutoTerminationPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_managed_scaling_policy(&self, builder: RemoveManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveManagedScalingPolicyOutput, SdkError<RemoveManagedScalingPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        builder.send_with(&self.0)
    }
    fn start_notebook_execution(&self, builder: StartNotebookExecutionInputBuilder) -> impl Future<Output = Result<StartNotebookExecutionOutput, SdkError<StartNotebookExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_notebook_execution(&self, builder: StopNotebookExecutionInputBuilder) -> impl Future<Output = Result<StopNotebookExecutionOutput, SdkError<StopNotebookExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn update_studio(&self, builder: UpdateStudioInputBuilder) -> impl Future<Output = Result<UpdateStudioOutput, SdkError<UpdateStudioError>>> {
        builder.send_with(&self.0)
    }
    fn update_studio_session_mapping(&self, builder: UpdateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<UpdateStudioSessionMappingOutput, SdkError<UpdateStudioSessionMappingError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> EMRClient for T
where T: Deref,
      T::Target: EMRClient {
    fn add_instance_fleet(&self, builder: AddInstanceFleetInputBuilder) -> impl Future<Output = Result<AddInstanceFleetOutput, SdkError<AddInstanceFleetError>>> {
        self.deref().add_instance_fleet(builder)
    }
    fn add_instance_groups(&self, builder: AddInstanceGroupsInputBuilder) -> impl Future<Output = Result<AddInstanceGroupsOutput, SdkError<AddInstanceGroupsError>>> {
        self.deref().add_instance_groups(builder)
    }
    fn add_tags(&self, builder: AddTagsInputBuilder) -> impl Future<Output = Result<AddTagsOutput, SdkError<AddTagsError>>> {
        self.deref().add_tags(builder)
    }
    fn cancel_steps(&self, builder: CancelStepsInputBuilder) -> impl Future<Output = Result<CancelStepsOutput, SdkError<CancelStepsError>>> {
        self.deref().cancel_steps(builder)
    }
    fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> impl Future<Output = Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>> {
        self.deref().create_security_configuration(builder)
    }
    fn create_studio(&self, builder: CreateStudioInputBuilder) -> impl Future<Output = Result<CreateStudioOutput, SdkError<CreateStudioError>>> {
        self.deref().create_studio(builder)
    }
    fn create_studio_session_mapping(&self, builder: CreateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<CreateStudioSessionMappingOutput, SdkError<CreateStudioSessionMappingError>>> {
        self.deref().create_studio_session_mapping(builder)
    }
    fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>> {
        self.deref().delete_security_configuration(builder)
    }
    fn delete_studio(&self, builder: DeleteStudioInputBuilder) -> impl Future<Output = Result<DeleteStudioOutput, SdkError<DeleteStudioError>>> {
        self.deref().delete_studio(builder)
    }
    fn delete_studio_session_mapping(&self, builder: DeleteStudioSessionMappingInputBuilder) -> impl Future<Output = Result<DeleteStudioSessionMappingOutput, SdkError<DeleteStudioSessionMappingError>>> {
        self.deref().delete_studio_session_mapping(builder)
    }
    fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> impl Future<Output = Result<DescribeClusterOutput, SdkError<DescribeClusterError>>> {
        self.deref().describe_cluster(builder)
    }
    fn describe_notebook_execution(&self, builder: DescribeNotebookExecutionInputBuilder) -> impl Future<Output = Result<DescribeNotebookExecutionOutput, SdkError<DescribeNotebookExecutionError>>> {
        self.deref().describe_notebook_execution(builder)
    }
    fn describe_release_label(&self, builder: DescribeReleaseLabelInputBuilder) -> impl Future<Output = Result<DescribeReleaseLabelOutput, SdkError<DescribeReleaseLabelError>>> {
        self.deref().describe_release_label(builder)
    }
    fn describe_security_configuration(&self, builder: DescribeSecurityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeSecurityConfigurationOutput, SdkError<DescribeSecurityConfigurationError>>> {
        self.deref().describe_security_configuration(builder)
    }
    fn describe_step(&self, builder: DescribeStepInputBuilder) -> impl Future<Output = Result<DescribeStepOutput, SdkError<DescribeStepError>>> {
        self.deref().describe_step(builder)
    }
    fn describe_studio(&self, builder: DescribeStudioInputBuilder) -> impl Future<Output = Result<DescribeStudioOutput, SdkError<DescribeStudioError>>> {
        self.deref().describe_studio(builder)
    }
    fn get_auto_termination_policy(&self, builder: GetAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<GetAutoTerminationPolicyOutput, SdkError<GetAutoTerminationPolicyError>>> {
        self.deref().get_auto_termination_policy(builder)
    }
    fn get_block_public_access_configuration(&self, builder: GetBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<GetBlockPublicAccessConfigurationOutput, SdkError<GetBlockPublicAccessConfigurationError>>> {
        self.deref().get_block_public_access_configuration(builder)
    }
    fn get_cluster_session_credentials(&self, builder: GetClusterSessionCredentialsInputBuilder) -> impl Future<Output = Result<GetClusterSessionCredentialsOutput, SdkError<GetClusterSessionCredentialsError>>> {
        self.deref().get_cluster_session_credentials(builder)
    }
    fn get_managed_scaling_policy(&self, builder: GetManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<GetManagedScalingPolicyOutput, SdkError<GetManagedScalingPolicyError>>> {
        self.deref().get_managed_scaling_policy(builder)
    }
    fn get_studio_session_mapping(&self, builder: GetStudioSessionMappingInputBuilder) -> impl Future<Output = Result<GetStudioSessionMappingOutput, SdkError<GetStudioSessionMappingError>>> {
        self.deref().get_studio_session_mapping(builder)
    }
    fn list_clusters(&self, builder: ListClustersInputBuilder) -> impl Future<Output = Result<ListClustersOutput, SdkError<ListClustersError>>> {
        self.deref().list_clusters(builder)
    }
    fn list_instance_fleets(&self, builder: ListInstanceFleetsInputBuilder) -> impl Future<Output = Result<ListInstanceFleetsOutput, SdkError<ListInstanceFleetsError>>> {
        self.deref().list_instance_fleets(builder)
    }
    fn list_instances(&self, builder: ListInstancesInputBuilder) -> impl Future<Output = Result<ListInstancesOutput, SdkError<ListInstancesError>>> {
        self.deref().list_instances(builder)
    }
    fn list_notebook_executions(&self, builder: ListNotebookExecutionsInputBuilder) -> impl Future<Output = Result<ListNotebookExecutionsOutput, SdkError<ListNotebookExecutionsError>>> {
        self.deref().list_notebook_executions(builder)
    }
    fn list_release_labels(&self, builder: ListReleaseLabelsInputBuilder) -> impl Future<Output = Result<ListReleaseLabelsOutput, SdkError<ListReleaseLabelsError>>> {
        self.deref().list_release_labels(builder)
    }
    fn list_security_configurations(&self, builder: ListSecurityConfigurationsInputBuilder) -> impl Future<Output = Result<ListSecurityConfigurationsOutput, SdkError<ListSecurityConfigurationsError>>> {
        self.deref().list_security_configurations(builder)
    }
    fn list_steps(&self, builder: ListStepsInputBuilder) -> impl Future<Output = Result<ListStepsOutput, SdkError<ListStepsError>>> {
        self.deref().list_steps(builder)
    }
    fn list_studio_session_mappings(&self, builder: ListStudioSessionMappingsInputBuilder) -> impl Future<Output = Result<ListStudioSessionMappingsOutput, SdkError<ListStudioSessionMappingsError>>> {
        self.deref().list_studio_session_mappings(builder)
    }
    fn list_studios(&self, builder: ListStudiosInputBuilder) -> impl Future<Output = Result<ListStudiosOutput, SdkError<ListStudiosError>>> {
        self.deref().list_studios(builder)
    }
    fn list_supported_instance_types(&self, builder: ListSupportedInstanceTypesInputBuilder) -> impl Future<Output = Result<ListSupportedInstanceTypesOutput, SdkError<ListSupportedInstanceTypesError>>> {
        self.deref().list_supported_instance_types(builder)
    }
    fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> impl Future<Output = Result<ModifyClusterOutput, SdkError<ModifyClusterError>>> {
        self.deref().modify_cluster(builder)
    }
    fn modify_instance_fleet(&self, builder: ModifyInstanceFleetInputBuilder) -> impl Future<Output = Result<ModifyInstanceFleetOutput, SdkError<ModifyInstanceFleetError>>> {
        self.deref().modify_instance_fleet(builder)
    }
    fn modify_instance_groups(&self, builder: ModifyInstanceGroupsInputBuilder) -> impl Future<Output = Result<ModifyInstanceGroupsOutput, SdkError<ModifyInstanceGroupsError>>> {
        self.deref().modify_instance_groups(builder)
    }
    fn put_auto_scaling_policy(&self, builder: PutAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<PutAutoScalingPolicyOutput, SdkError<PutAutoScalingPolicyError>>> {
        self.deref().put_auto_scaling_policy(builder)
    }
    fn put_auto_termination_policy(&self, builder: PutAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<PutAutoTerminationPolicyOutput, SdkError<PutAutoTerminationPolicyError>>> {
        self.deref().put_auto_termination_policy(builder)
    }
    fn put_block_public_access_configuration(&self, builder: PutBlockPublicAccessConfigurationInputBuilder) -> impl Future<Output = Result<PutBlockPublicAccessConfigurationOutput, SdkError<PutBlockPublicAccessConfigurationError>>> {
        self.deref().put_block_public_access_configuration(builder)
    }
    fn put_managed_scaling_policy(&self, builder: PutManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<PutManagedScalingPolicyOutput, SdkError<PutManagedScalingPolicyError>>> {
        self.deref().put_managed_scaling_policy(builder)
    }
    fn remove_auto_scaling_policy(&self, builder: RemoveAutoScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoScalingPolicyOutput, SdkError<RemoveAutoScalingPolicyError>>> {
        self.deref().remove_auto_scaling_policy(builder)
    }
    fn remove_auto_termination_policy(&self, builder: RemoveAutoTerminationPolicyInputBuilder) -> impl Future<Output = Result<RemoveAutoTerminationPolicyOutput, SdkError<RemoveAutoTerminationPolicyError>>> {
        self.deref().remove_auto_termination_policy(builder)
    }
    fn remove_managed_scaling_policy(&self, builder: RemoveManagedScalingPolicyInputBuilder) -> impl Future<Output = Result<RemoveManagedScalingPolicyOutput, SdkError<RemoveManagedScalingPolicyError>>> {
        self.deref().remove_managed_scaling_policy(builder)
    }
    fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> impl Future<Output = Result<RemoveTagsOutput, SdkError<RemoveTagsError>>> {
        self.deref().remove_tags(builder)
    }
    fn start_notebook_execution(&self, builder: StartNotebookExecutionInputBuilder) -> impl Future<Output = Result<StartNotebookExecutionOutput, SdkError<StartNotebookExecutionError>>> {
        self.deref().start_notebook_execution(builder)
    }
    fn stop_notebook_execution(&self, builder: StopNotebookExecutionInputBuilder) -> impl Future<Output = Result<StopNotebookExecutionOutput, SdkError<StopNotebookExecutionError>>> {
        self.deref().stop_notebook_execution(builder)
    }
    fn update_studio(&self, builder: UpdateStudioInputBuilder) -> impl Future<Output = Result<UpdateStudioOutput, SdkError<UpdateStudioError>>> {
        self.deref().update_studio(builder)
    }
    fn update_studio_session_mapping(&self, builder: UpdateStudioSessionMappingInputBuilder) -> impl Future<Output = Result<UpdateStudioSessionMappingOutput, SdkError<UpdateStudioSessionMappingError>>> {
        self.deref().update_studio_session_mapping(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edEMRClient {}
    impl EMRClient for edEMRClient {
        async fn add_instance_fleet(&self, builder: AddInstanceFleetInputBuilder) -> Result<AddInstanceFleetOutput, SdkError<AddInstanceFleetError>>;
        async fn add_instance_groups(&self, builder: AddInstanceGroupsInputBuilder) -> Result<AddInstanceGroupsOutput, SdkError<AddInstanceGroupsError>>;
        async fn add_tags(&self, builder: AddTagsInputBuilder) -> Result<AddTagsOutput, SdkError<AddTagsError>>;
        async fn cancel_steps(&self, builder: CancelStepsInputBuilder) -> Result<CancelStepsOutput, SdkError<CancelStepsError>>;
        async fn create_security_configuration(&self, builder: CreateSecurityConfigurationInputBuilder) -> Result<CreateSecurityConfigurationOutput, SdkError<CreateSecurityConfigurationError>>;
        async fn create_studio(&self, builder: CreateStudioInputBuilder) -> Result<CreateStudioOutput, SdkError<CreateStudioError>>;
        async fn create_studio_session_mapping(&self, builder: CreateStudioSessionMappingInputBuilder) -> Result<CreateStudioSessionMappingOutput, SdkError<CreateStudioSessionMappingError>>;
        async fn delete_security_configuration(&self, builder: DeleteSecurityConfigurationInputBuilder) -> Result<DeleteSecurityConfigurationOutput, SdkError<DeleteSecurityConfigurationError>>;
        async fn delete_studio(&self, builder: DeleteStudioInputBuilder) -> Result<DeleteStudioOutput, SdkError<DeleteStudioError>>;
        async fn delete_studio_session_mapping(&self, builder: DeleteStudioSessionMappingInputBuilder) -> Result<DeleteStudioSessionMappingOutput, SdkError<DeleteStudioSessionMappingError>>;
        async fn describe_cluster(&self, builder: DescribeClusterInputBuilder) -> Result<DescribeClusterOutput, SdkError<DescribeClusterError>>;
        async fn describe_notebook_execution(&self, builder: DescribeNotebookExecutionInputBuilder) -> Result<DescribeNotebookExecutionOutput, SdkError<DescribeNotebookExecutionError>>;
        async fn describe_release_label(&self, builder: DescribeReleaseLabelInputBuilder) -> Result<DescribeReleaseLabelOutput, SdkError<DescribeReleaseLabelError>>;
        async fn describe_security_configuration(&self, builder: DescribeSecurityConfigurationInputBuilder) -> Result<DescribeSecurityConfigurationOutput, SdkError<DescribeSecurityConfigurationError>>;
        async fn describe_step(&self, builder: DescribeStepInputBuilder) -> Result<DescribeStepOutput, SdkError<DescribeStepError>>;
        async fn describe_studio(&self, builder: DescribeStudioInputBuilder) -> Result<DescribeStudioOutput, SdkError<DescribeStudioError>>;
        async fn get_auto_termination_policy(&self, builder: GetAutoTerminationPolicyInputBuilder) -> Result<GetAutoTerminationPolicyOutput, SdkError<GetAutoTerminationPolicyError>>;
        async fn get_block_public_access_configuration(&self, builder: GetBlockPublicAccessConfigurationInputBuilder) -> Result<GetBlockPublicAccessConfigurationOutput, SdkError<GetBlockPublicAccessConfigurationError>>;
        async fn get_cluster_session_credentials(&self, builder: GetClusterSessionCredentialsInputBuilder) -> Result<GetClusterSessionCredentialsOutput, SdkError<GetClusterSessionCredentialsError>>;
        async fn get_managed_scaling_policy(&self, builder: GetManagedScalingPolicyInputBuilder) -> Result<GetManagedScalingPolicyOutput, SdkError<GetManagedScalingPolicyError>>;
        async fn get_studio_session_mapping(&self, builder: GetStudioSessionMappingInputBuilder) -> Result<GetStudioSessionMappingOutput, SdkError<GetStudioSessionMappingError>>;
        async fn list_clusters(&self, builder: ListClustersInputBuilder) -> Result<ListClustersOutput, SdkError<ListClustersError>>;
        async fn list_instance_fleets(&self, builder: ListInstanceFleetsInputBuilder) -> Result<ListInstanceFleetsOutput, SdkError<ListInstanceFleetsError>>;
        async fn list_instances(&self, builder: ListInstancesInputBuilder) -> Result<ListInstancesOutput, SdkError<ListInstancesError>>;
        async fn list_notebook_executions(&self, builder: ListNotebookExecutionsInputBuilder) -> Result<ListNotebookExecutionsOutput, SdkError<ListNotebookExecutionsError>>;
        async fn list_release_labels(&self, builder: ListReleaseLabelsInputBuilder) -> Result<ListReleaseLabelsOutput, SdkError<ListReleaseLabelsError>>;
        async fn list_security_configurations(&self, builder: ListSecurityConfigurationsInputBuilder) -> Result<ListSecurityConfigurationsOutput, SdkError<ListSecurityConfigurationsError>>;
        async fn list_steps(&self, builder: ListStepsInputBuilder) -> Result<ListStepsOutput, SdkError<ListStepsError>>;
        async fn list_studio_session_mappings(&self, builder: ListStudioSessionMappingsInputBuilder) -> Result<ListStudioSessionMappingsOutput, SdkError<ListStudioSessionMappingsError>>;
        async fn list_studios(&self, builder: ListStudiosInputBuilder) -> Result<ListStudiosOutput, SdkError<ListStudiosError>>;
        async fn list_supported_instance_types(&self, builder: ListSupportedInstanceTypesInputBuilder) -> Result<ListSupportedInstanceTypesOutput, SdkError<ListSupportedInstanceTypesError>>;
        async fn modify_cluster(&self, builder: ModifyClusterInputBuilder) -> Result<ModifyClusterOutput, SdkError<ModifyClusterError>>;
        async fn modify_instance_fleet(&self, builder: ModifyInstanceFleetInputBuilder) -> Result<ModifyInstanceFleetOutput, SdkError<ModifyInstanceFleetError>>;
        async fn modify_instance_groups(&self, builder: ModifyInstanceGroupsInputBuilder) -> Result<ModifyInstanceGroupsOutput, SdkError<ModifyInstanceGroupsError>>;
        async fn put_auto_scaling_policy(&self, builder: PutAutoScalingPolicyInputBuilder) -> Result<PutAutoScalingPolicyOutput, SdkError<PutAutoScalingPolicyError>>;
        async fn put_auto_termination_policy(&self, builder: PutAutoTerminationPolicyInputBuilder) -> Result<PutAutoTerminationPolicyOutput, SdkError<PutAutoTerminationPolicyError>>;
        async fn put_block_public_access_configuration(&self, builder: PutBlockPublicAccessConfigurationInputBuilder) -> Result<PutBlockPublicAccessConfigurationOutput, SdkError<PutBlockPublicAccessConfigurationError>>;
        async fn put_managed_scaling_policy(&self, builder: PutManagedScalingPolicyInputBuilder) -> Result<PutManagedScalingPolicyOutput, SdkError<PutManagedScalingPolicyError>>;
        async fn remove_auto_scaling_policy(&self, builder: RemoveAutoScalingPolicyInputBuilder) -> Result<RemoveAutoScalingPolicyOutput, SdkError<RemoveAutoScalingPolicyError>>;
        async fn remove_auto_termination_policy(&self, builder: RemoveAutoTerminationPolicyInputBuilder) -> Result<RemoveAutoTerminationPolicyOutput, SdkError<RemoveAutoTerminationPolicyError>>;
        async fn remove_managed_scaling_policy(&self, builder: RemoveManagedScalingPolicyInputBuilder) -> Result<RemoveManagedScalingPolicyOutput, SdkError<RemoveManagedScalingPolicyError>>;
        async fn remove_tags(&self, builder: RemoveTagsInputBuilder) -> Result<RemoveTagsOutput, SdkError<RemoveTagsError>>;
        async fn start_notebook_execution(&self, builder: StartNotebookExecutionInputBuilder) -> Result<StartNotebookExecutionOutput, SdkError<StartNotebookExecutionError>>;
        async fn stop_notebook_execution(&self, builder: StopNotebookExecutionInputBuilder) -> Result<StopNotebookExecutionOutput, SdkError<StopNotebookExecutionError>>;
        async fn update_studio(&self, builder: UpdateStudioInputBuilder) -> Result<UpdateStudioOutput, SdkError<UpdateStudioError>>;
        async fn update_studio_session_mapping(&self, builder: UpdateStudioSessionMappingInputBuilder) -> Result<UpdateStudioSessionMappingOutput, SdkError<UpdateStudioSessionMappingError>>;
    }
}
