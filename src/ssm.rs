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
use aws_sdk_ssm::operation::add_tags_to_resource::{builders::*, *};
use aws_sdk_ssm::operation::associate_ops_item_related_item::{builders::*, *};
use aws_sdk_ssm::operation::cancel_command::{builders::*, *};
use aws_sdk_ssm::operation::cancel_maintenance_window_execution::{builders::*, *};
use aws_sdk_ssm::operation::create_activation::{builders::*, *};
use aws_sdk_ssm::operation::create_association::{builders::*, *};
use aws_sdk_ssm::operation::create_association_batch::{builders::*, *};
use aws_sdk_ssm::operation::create_document::{builders::*, *};
use aws_sdk_ssm::operation::create_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::create_ops_item::{builders::*, *};
use aws_sdk_ssm::operation::create_ops_metadata::{builders::*, *};
use aws_sdk_ssm::operation::create_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::create_resource_data_sync::{builders::*, *};
use aws_sdk_ssm::operation::delete_activation::{builders::*, *};
use aws_sdk_ssm::operation::delete_association::{builders::*, *};
use aws_sdk_ssm::operation::delete_document::{builders::*, *};
use aws_sdk_ssm::operation::delete_inventory::{builders::*, *};
use aws_sdk_ssm::operation::delete_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::delete_ops_item::{builders::*, *};
use aws_sdk_ssm::operation::delete_ops_metadata::{builders::*, *};
use aws_sdk_ssm::operation::delete_parameter::{builders::*, *};
use aws_sdk_ssm::operation::delete_parameters::{builders::*, *};
use aws_sdk_ssm::operation::delete_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::delete_resource_data_sync::{builders::*, *};
use aws_sdk_ssm::operation::delete_resource_policy::{builders::*, *};
use aws_sdk_ssm::operation::deregister_managed_instance::{builders::*, *};
use aws_sdk_ssm::operation::deregister_patch_baseline_for_patch_group::{builders::*, *};
use aws_sdk_ssm::operation::deregister_target_from_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::deregister_task_from_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::describe_activations::{builders::*, *};
use aws_sdk_ssm::operation::describe_association::{builders::*, *};
use aws_sdk_ssm::operation::describe_association_execution_targets::{builders::*, *};
use aws_sdk_ssm::operation::describe_association_executions::{builders::*, *};
use aws_sdk_ssm::operation::describe_automation_executions::{builders::*, *};
use aws_sdk_ssm::operation::describe_automation_step_executions::{builders::*, *};
use aws_sdk_ssm::operation::describe_available_patches::{builders::*, *};
use aws_sdk_ssm::operation::describe_document::{builders::*, *};
use aws_sdk_ssm::operation::describe_document_permission::{builders::*, *};
use aws_sdk_ssm::operation::describe_effective_instance_associations::{builders::*, *};
use aws_sdk_ssm::operation::describe_effective_patches_for_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_associations_status::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_information::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_patch_states::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_patch_states_for_patch_group::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_patches::{builders::*, *};
use aws_sdk_ssm::operation::describe_instance_properties::{builders::*, *};
use aws_sdk_ssm::operation::describe_inventory_deletions::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_execution_task_invocations::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_execution_tasks::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_executions::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_schedule::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_targets::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_window_tasks::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_windows::{builders::*, *};
use aws_sdk_ssm::operation::describe_maintenance_windows_for_target::{builders::*, *};
use aws_sdk_ssm::operation::describe_ops_items::{builders::*, *};
use aws_sdk_ssm::operation::describe_parameters::{builders::*, *};
use aws_sdk_ssm::operation::describe_patch_baselines::{builders::*, *};
use aws_sdk_ssm::operation::describe_patch_group_state::{builders::*, *};
use aws_sdk_ssm::operation::describe_patch_groups::{builders::*, *};
use aws_sdk_ssm::operation::describe_patch_properties::{builders::*, *};
use aws_sdk_ssm::operation::describe_sessions::{builders::*, *};
use aws_sdk_ssm::operation::disassociate_ops_item_related_item::{builders::*, *};
use aws_sdk_ssm::operation::get_automation_execution::{builders::*, *};
use aws_sdk_ssm::operation::get_calendar_state::{builders::*, *};
use aws_sdk_ssm::operation::get_command_invocation::{builders::*, *};
use aws_sdk_ssm::operation::get_connection_status::{builders::*, *};
use aws_sdk_ssm::operation::get_default_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::get_deployable_patch_snapshot_for_instance::{builders::*, *};
use aws_sdk_ssm::operation::get_document::{builders::*, *};
use aws_sdk_ssm::operation::get_inventory::{builders::*, *};
use aws_sdk_ssm::operation::get_inventory_schema::{builders::*, *};
use aws_sdk_ssm::operation::get_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::get_maintenance_window_execution::{builders::*, *};
use aws_sdk_ssm::operation::get_maintenance_window_execution_task::{builders::*, *};
use aws_sdk_ssm::operation::get_maintenance_window_execution_task_invocation::{builders::*, *};
use aws_sdk_ssm::operation::get_maintenance_window_task::{builders::*, *};
use aws_sdk_ssm::operation::get_ops_item::{builders::*, *};
use aws_sdk_ssm::operation::get_ops_metadata::{builders::*, *};
use aws_sdk_ssm::operation::get_ops_summary::{builders::*, *};
use aws_sdk_ssm::operation::get_parameter::{builders::*, *};
use aws_sdk_ssm::operation::get_parameter_history::{builders::*, *};
use aws_sdk_ssm::operation::get_parameters::{builders::*, *};
use aws_sdk_ssm::operation::get_parameters_by_path::{builders::*, *};
use aws_sdk_ssm::operation::get_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::get_patch_baseline_for_patch_group::{builders::*, *};
use aws_sdk_ssm::operation::get_resource_policies::{builders::*, *};
use aws_sdk_ssm::operation::get_service_setting::{builders::*, *};
use aws_sdk_ssm::operation::label_parameter_version::{builders::*, *};
use aws_sdk_ssm::operation::list_association_versions::{builders::*, *};
use aws_sdk_ssm::operation::list_associations::{builders::*, *};
use aws_sdk_ssm::operation::list_command_invocations::{builders::*, *};
use aws_sdk_ssm::operation::list_commands::{builders::*, *};
use aws_sdk_ssm::operation::list_compliance_items::{builders::*, *};
use aws_sdk_ssm::operation::list_compliance_summaries::{builders::*, *};
use aws_sdk_ssm::operation::list_document_metadata_history::{builders::*, *};
use aws_sdk_ssm::operation::list_document_versions::{builders::*, *};
use aws_sdk_ssm::operation::list_documents::{builders::*, *};
use aws_sdk_ssm::operation::list_inventory_entries::{builders::*, *};
use aws_sdk_ssm::operation::list_ops_item_events::{builders::*, *};
use aws_sdk_ssm::operation::list_ops_item_related_items::{builders::*, *};
use aws_sdk_ssm::operation::list_ops_metadata::{builders::*, *};
use aws_sdk_ssm::operation::list_resource_compliance_summaries::{builders::*, *};
use aws_sdk_ssm::operation::list_resource_data_sync::{builders::*, *};
use aws_sdk_ssm::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_ssm::operation::modify_document_permission::{builders::*, *};
use aws_sdk_ssm::operation::put_compliance_items::{builders::*, *};
use aws_sdk_ssm::operation::put_inventory::{builders::*, *};
use aws_sdk_ssm::operation::put_parameter::{builders::*, *};
use aws_sdk_ssm::operation::put_resource_policy::{builders::*, *};
use aws_sdk_ssm::operation::register_default_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::register_patch_baseline_for_patch_group::{builders::*, *};
use aws_sdk_ssm::operation::register_target_with_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::register_task_with_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::remove_tags_from_resource::{builders::*, *};
use aws_sdk_ssm::operation::reset_service_setting::{builders::*, *};
use aws_sdk_ssm::operation::resume_session::{builders::*, *};
use aws_sdk_ssm::operation::send_automation_signal::{builders::*, *};
use aws_sdk_ssm::operation::send_command::{builders::*, *};
use aws_sdk_ssm::operation::start_associations_once::{builders::*, *};
use aws_sdk_ssm::operation::start_automation_execution::{builders::*, *};
use aws_sdk_ssm::operation::start_change_request_execution::{builders::*, *};
use aws_sdk_ssm::operation::start_session::{builders::*, *};
use aws_sdk_ssm::operation::stop_automation_execution::{builders::*, *};
use aws_sdk_ssm::operation::terminate_session::{builders::*, *};
use aws_sdk_ssm::operation::unlabel_parameter_version::{builders::*, *};
use aws_sdk_ssm::operation::update_association::{builders::*, *};
use aws_sdk_ssm::operation::update_association_status::{builders::*, *};
use aws_sdk_ssm::operation::update_document::{builders::*, *};
use aws_sdk_ssm::operation::update_document_default_version::{builders::*, *};
use aws_sdk_ssm::operation::update_document_metadata::{builders::*, *};
use aws_sdk_ssm::operation::update_maintenance_window::{builders::*, *};
use aws_sdk_ssm::operation::update_maintenance_window_target::{builders::*, *};
use aws_sdk_ssm::operation::update_maintenance_window_task::{builders::*, *};
use aws_sdk_ssm::operation::update_managed_instance_role::{builders::*, *};
use aws_sdk_ssm::operation::update_ops_item::{builders::*, *};
use aws_sdk_ssm::operation::update_ops_metadata::{builders::*, *};
use aws_sdk_ssm::operation::update_patch_baseline::{builders::*, *};
use aws_sdk_ssm::operation::update_resource_data_sync::{builders::*, *};
use aws_sdk_ssm::operation::update_service_setting::{builders::*, *};
use aws_sdk_ssm::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_ssm::Client;

pub use aws_sdk_ssm::*;

pub struct SSMClientImpl(Client);
impl SSMClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait SSMClient {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>>;
    fn associate_ops_item_related_item(&self, builder: AssociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<AssociateOpsItemRelatedItemOutput, SdkError<AssociateOpsItemRelatedItemError>>>;
    fn cancel_command(&self, builder: CancelCommandInputBuilder) -> impl Future<Output = Result<CancelCommandOutput, SdkError<CancelCommandError>>>;
    fn cancel_maintenance_window_execution(&self, builder: CancelMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<CancelMaintenanceWindowExecutionOutput, SdkError<CancelMaintenanceWindowExecutionError>>>;
    fn create_activation(&self, builder: CreateActivationInputBuilder) -> impl Future<Output = Result<CreateActivationOutput, SdkError<CreateActivationError>>>;
    fn create_association(&self, builder: CreateAssociationInputBuilder) -> impl Future<Output = Result<CreateAssociationOutput, SdkError<CreateAssociationError>>>;
    fn create_association_batch(&self, builder: CreateAssociationBatchInputBuilder) -> impl Future<Output = Result<CreateAssociationBatchOutput, SdkError<CreateAssociationBatchError>>>;
    fn create_document(&self, builder: CreateDocumentInputBuilder) -> impl Future<Output = Result<CreateDocumentOutput, SdkError<CreateDocumentError>>>;
    fn create_maintenance_window(&self, builder: CreateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<CreateMaintenanceWindowOutput, SdkError<CreateMaintenanceWindowError>>>;
    fn create_ops_item(&self, builder: CreateOpsItemInputBuilder) -> impl Future<Output = Result<CreateOpsItemOutput, SdkError<CreateOpsItemError>>>;
    fn create_ops_metadata(&self, builder: CreateOpsMetadataInputBuilder) -> impl Future<Output = Result<CreateOpsMetadataOutput, SdkError<CreateOpsMetadataError>>>;
    fn create_patch_baseline(&self, builder: CreatePatchBaselineInputBuilder) -> impl Future<Output = Result<CreatePatchBaselineOutput, SdkError<CreatePatchBaselineError>>>;
    fn create_resource_data_sync(&self, builder: CreateResourceDataSyncInputBuilder) -> impl Future<Output = Result<CreateResourceDataSyncOutput, SdkError<CreateResourceDataSyncError>>>;
    fn delete_activation(&self, builder: DeleteActivationInputBuilder) -> impl Future<Output = Result<DeleteActivationOutput, SdkError<DeleteActivationError>>>;
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>>;
    fn delete_document(&self, builder: DeleteDocumentInputBuilder) -> impl Future<Output = Result<DeleteDocumentOutput, SdkError<DeleteDocumentError>>>;
    fn delete_inventory(&self, builder: DeleteInventoryInputBuilder) -> impl Future<Output = Result<DeleteInventoryOutput, SdkError<DeleteInventoryError>>>;
    fn delete_maintenance_window(&self, builder: DeleteMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeleteMaintenanceWindowOutput, SdkError<DeleteMaintenanceWindowError>>>;
    fn delete_ops_item(&self, builder: DeleteOpsItemInputBuilder) -> impl Future<Output = Result<DeleteOpsItemOutput, SdkError<DeleteOpsItemError>>>;
    fn delete_ops_metadata(&self, builder: DeleteOpsMetadataInputBuilder) -> impl Future<Output = Result<DeleteOpsMetadataOutput, SdkError<DeleteOpsMetadataError>>>;
    fn delete_parameter(&self, builder: DeleteParameterInputBuilder) -> impl Future<Output = Result<DeleteParameterOutput, SdkError<DeleteParameterError>>>;
    fn delete_parameters(&self, builder: DeleteParametersInputBuilder) -> impl Future<Output = Result<DeleteParametersOutput, SdkError<DeleteParametersError>>>;
    fn delete_patch_baseline(&self, builder: DeletePatchBaselineInputBuilder) -> impl Future<Output = Result<DeletePatchBaselineOutput, SdkError<DeletePatchBaselineError>>>;
    fn delete_resource_data_sync(&self, builder: DeleteResourceDataSyncInputBuilder) -> impl Future<Output = Result<DeleteResourceDataSyncOutput, SdkError<DeleteResourceDataSyncError>>>;
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>>;
    fn deregister_managed_instance(&self, builder: DeregisterManagedInstanceInputBuilder) -> impl Future<Output = Result<DeregisterManagedInstanceOutput, SdkError<DeregisterManagedInstanceError>>>;
    fn deregister_patch_baseline_for_patch_group(&self, builder: DeregisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<DeregisterPatchBaselineForPatchGroupOutput, SdkError<DeregisterPatchBaselineForPatchGroupError>>>;
    fn deregister_target_from_maintenance_window(&self, builder: DeregisterTargetFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTargetFromMaintenanceWindowOutput, SdkError<DeregisterTargetFromMaintenanceWindowError>>>;
    fn deregister_task_from_maintenance_window(&self, builder: DeregisterTaskFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTaskFromMaintenanceWindowOutput, SdkError<DeregisterTaskFromMaintenanceWindowError>>>;
    fn describe_activations(&self, builder: DescribeActivationsInputBuilder) -> impl Future<Output = Result<DescribeActivationsOutput, SdkError<DescribeActivationsError>>>;
    fn describe_association(&self, builder: DescribeAssociationInputBuilder) -> impl Future<Output = Result<DescribeAssociationOutput, SdkError<DescribeAssociationError>>>;
    fn describe_association_execution_targets(&self, builder: DescribeAssociationExecutionTargetsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionTargetsOutput, SdkError<DescribeAssociationExecutionTargetsError>>>;
    fn describe_association_executions(&self, builder: DescribeAssociationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionsOutput, SdkError<DescribeAssociationExecutionsError>>>;
    fn describe_automation_executions(&self, builder: DescribeAutomationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationExecutionsOutput, SdkError<DescribeAutomationExecutionsError>>>;
    fn describe_automation_step_executions(&self, builder: DescribeAutomationStepExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationStepExecutionsOutput, SdkError<DescribeAutomationStepExecutionsError>>>;
    fn describe_available_patches(&self, builder: DescribeAvailablePatchesInputBuilder) -> impl Future<Output = Result<DescribeAvailablePatchesOutput, SdkError<DescribeAvailablePatchesError>>>;
    fn describe_document(&self, builder: DescribeDocumentInputBuilder) -> impl Future<Output = Result<DescribeDocumentOutput, SdkError<DescribeDocumentError>>>;
    fn describe_document_permission(&self, builder: DescribeDocumentPermissionInputBuilder) -> impl Future<Output = Result<DescribeDocumentPermissionOutput, SdkError<DescribeDocumentPermissionError>>>;
    fn describe_effective_instance_associations(&self, builder: DescribeEffectiveInstanceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeEffectiveInstanceAssociationsOutput, SdkError<DescribeEffectiveInstanceAssociationsError>>>;
    fn describe_effective_patches_for_patch_baseline(&self, builder: DescribeEffectivePatchesForPatchBaselineInputBuilder) -> impl Future<Output = Result<DescribeEffectivePatchesForPatchBaselineOutput, SdkError<DescribeEffectivePatchesForPatchBaselineError>>>;
    fn describe_instance_associations_status(&self, builder: DescribeInstanceAssociationsStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceAssociationsStatusOutput, SdkError<DescribeInstanceAssociationsStatusError>>>;
    fn describe_instance_information(&self, builder: DescribeInstanceInformationInputBuilder) -> impl Future<Output = Result<DescribeInstanceInformationOutput, SdkError<DescribeInstanceInformationError>>>;
    fn describe_instance_patch_states(&self, builder: DescribeInstancePatchStatesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesOutput, SdkError<DescribeInstancePatchStatesError>>>;
    fn describe_instance_patch_states_for_patch_group(&self, builder: DescribeInstancePatchStatesForPatchGroupInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesForPatchGroupOutput, SdkError<DescribeInstancePatchStatesForPatchGroupError>>>;
    fn describe_instance_patches(&self, builder: DescribeInstancePatchesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchesOutput, SdkError<DescribeInstancePatchesError>>>;
    fn describe_instance_properties(&self, builder: DescribeInstancePropertiesInputBuilder) -> impl Future<Output = Result<DescribeInstancePropertiesOutput, SdkError<DescribeInstancePropertiesError>>>;
    fn describe_inventory_deletions(&self, builder: DescribeInventoryDeletionsInputBuilder) -> impl Future<Output = Result<DescribeInventoryDeletionsOutput, SdkError<DescribeInventoryDeletionsError>>>;
    fn describe_maintenance_window_execution_task_invocations(&self, builder: DescribeMaintenanceWindowExecutionTaskInvocationsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTaskInvocationsOutput, SdkError<DescribeMaintenanceWindowExecutionTaskInvocationsError>>>;
    fn describe_maintenance_window_execution_tasks(&self, builder: DescribeMaintenanceWindowExecutionTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTasksOutput, SdkError<DescribeMaintenanceWindowExecutionTasksError>>>;
    fn describe_maintenance_window_executions(&self, builder: DescribeMaintenanceWindowExecutionsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionsOutput, SdkError<DescribeMaintenanceWindowExecutionsError>>>;
    fn describe_maintenance_window_schedule(&self, builder: DescribeMaintenanceWindowScheduleInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowScheduleOutput, SdkError<DescribeMaintenanceWindowScheduleError>>>;
    fn describe_maintenance_window_targets(&self, builder: DescribeMaintenanceWindowTargetsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTargetsOutput, SdkError<DescribeMaintenanceWindowTargetsError>>>;
    fn describe_maintenance_window_tasks(&self, builder: DescribeMaintenanceWindowTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTasksOutput, SdkError<DescribeMaintenanceWindowTasksError>>>;
    fn describe_maintenance_windows(&self, builder: DescribeMaintenanceWindowsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsOutput, SdkError<DescribeMaintenanceWindowsError>>>;
    fn describe_maintenance_windows_for_target(&self, builder: DescribeMaintenanceWindowsForTargetInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsForTargetOutput, SdkError<DescribeMaintenanceWindowsForTargetError>>>;
    fn describe_ops_items(&self, builder: DescribeOpsItemsInputBuilder) -> impl Future<Output = Result<DescribeOpsItemsOutput, SdkError<DescribeOpsItemsError>>>;
    fn describe_parameters(&self, builder: DescribeParametersInputBuilder) -> impl Future<Output = Result<DescribeParametersOutput, SdkError<DescribeParametersError>>>;
    fn describe_patch_baselines(&self, builder: DescribePatchBaselinesInputBuilder) -> impl Future<Output = Result<DescribePatchBaselinesOutput, SdkError<DescribePatchBaselinesError>>>;
    fn describe_patch_group_state(&self, builder: DescribePatchGroupStateInputBuilder) -> impl Future<Output = Result<DescribePatchGroupStateOutput, SdkError<DescribePatchGroupStateError>>>;
    fn describe_patch_groups(&self, builder: DescribePatchGroupsInputBuilder) -> impl Future<Output = Result<DescribePatchGroupsOutput, SdkError<DescribePatchGroupsError>>>;
    fn describe_patch_properties(&self, builder: DescribePatchPropertiesInputBuilder) -> impl Future<Output = Result<DescribePatchPropertiesOutput, SdkError<DescribePatchPropertiesError>>>;
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>>;
    fn disassociate_ops_item_related_item(&self, builder: DisassociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<DisassociateOpsItemRelatedItemOutput, SdkError<DisassociateOpsItemRelatedItemError>>>;
    fn get_automation_execution(&self, builder: GetAutomationExecutionInputBuilder) -> impl Future<Output = Result<GetAutomationExecutionOutput, SdkError<GetAutomationExecutionError>>>;
    fn get_calendar_state(&self, builder: GetCalendarStateInputBuilder) -> impl Future<Output = Result<GetCalendarStateOutput, SdkError<GetCalendarStateError>>>;
    fn get_command_invocation(&self, builder: GetCommandInvocationInputBuilder) -> impl Future<Output = Result<GetCommandInvocationOutput, SdkError<GetCommandInvocationError>>>;
    fn get_connection_status(&self, builder: GetConnectionStatusInputBuilder) -> impl Future<Output = Result<GetConnectionStatusOutput, SdkError<GetConnectionStatusError>>>;
    fn get_default_patch_baseline(&self, builder: GetDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<GetDefaultPatchBaselineOutput, SdkError<GetDefaultPatchBaselineError>>>;
    fn get_deployable_patch_snapshot_for_instance(&self, builder: GetDeployablePatchSnapshotForInstanceInputBuilder) -> impl Future<Output = Result<GetDeployablePatchSnapshotForInstanceOutput, SdkError<GetDeployablePatchSnapshotForInstanceError>>>;
    fn get_document(&self, builder: GetDocumentInputBuilder) -> impl Future<Output = Result<GetDocumentOutput, SdkError<GetDocumentError>>>;
    fn get_inventory(&self, builder: GetInventoryInputBuilder) -> impl Future<Output = Result<GetInventoryOutput, SdkError<GetInventoryError>>>;
    fn get_inventory_schema(&self, builder: GetInventorySchemaInputBuilder) -> impl Future<Output = Result<GetInventorySchemaOutput, SdkError<GetInventorySchemaError>>>;
    fn get_maintenance_window(&self, builder: GetMaintenanceWindowInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowOutput, SdkError<GetMaintenanceWindowError>>>;
    fn get_maintenance_window_execution(&self, builder: GetMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionOutput, SdkError<GetMaintenanceWindowExecutionError>>>;
    fn get_maintenance_window_execution_task(&self, builder: GetMaintenanceWindowExecutionTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskOutput, SdkError<GetMaintenanceWindowExecutionTaskError>>>;
    fn get_maintenance_window_execution_task_invocation(&self, builder: GetMaintenanceWindowExecutionTaskInvocationInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskInvocationOutput, SdkError<GetMaintenanceWindowExecutionTaskInvocationError>>>;
    fn get_maintenance_window_task(&self, builder: GetMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowTaskOutput, SdkError<GetMaintenanceWindowTaskError>>>;
    fn get_ops_item(&self, builder: GetOpsItemInputBuilder) -> impl Future<Output = Result<GetOpsItemOutput, SdkError<GetOpsItemError>>>;
    fn get_ops_metadata(&self, builder: GetOpsMetadataInputBuilder) -> impl Future<Output = Result<GetOpsMetadataOutput, SdkError<GetOpsMetadataError>>>;
    fn get_ops_summary(&self, builder: GetOpsSummaryInputBuilder) -> impl Future<Output = Result<GetOpsSummaryOutput, SdkError<GetOpsSummaryError>>>;
    fn get_parameter(&self, builder: GetParameterInputBuilder) -> impl Future<Output = Result<GetParameterOutput, SdkError<GetParameterError>>>;
    fn get_parameter_history(&self, builder: GetParameterHistoryInputBuilder) -> impl Future<Output = Result<GetParameterHistoryOutput, SdkError<GetParameterHistoryError>>>;
    fn get_parameters(&self, builder: GetParametersInputBuilder) -> impl Future<Output = Result<GetParametersOutput, SdkError<GetParametersError>>>;
    fn get_parameters_by_path(&self, builder: GetParametersByPathInputBuilder) -> impl Future<Output = Result<GetParametersByPathOutput, SdkError<GetParametersByPathError>>>;
    fn get_patch_baseline(&self, builder: GetPatchBaselineInputBuilder) -> impl Future<Output = Result<GetPatchBaselineOutput, SdkError<GetPatchBaselineError>>>;
    fn get_patch_baseline_for_patch_group(&self, builder: GetPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<GetPatchBaselineForPatchGroupOutput, SdkError<GetPatchBaselineForPatchGroupError>>>;
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>>;
    fn get_service_setting(&self, builder: GetServiceSettingInputBuilder) -> impl Future<Output = Result<GetServiceSettingOutput, SdkError<GetServiceSettingError>>>;
    fn label_parameter_version(&self, builder: LabelParameterVersionInputBuilder) -> impl Future<Output = Result<LabelParameterVersionOutput, SdkError<LabelParameterVersionError>>>;
    fn list_association_versions(&self, builder: ListAssociationVersionsInputBuilder) -> impl Future<Output = Result<ListAssociationVersionsOutput, SdkError<ListAssociationVersionsError>>>;
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>>;
    fn list_command_invocations(&self, builder: ListCommandInvocationsInputBuilder) -> impl Future<Output = Result<ListCommandInvocationsOutput, SdkError<ListCommandInvocationsError>>>;
    fn list_commands(&self, builder: ListCommandsInputBuilder) -> impl Future<Output = Result<ListCommandsOutput, SdkError<ListCommandsError>>>;
    fn list_compliance_items(&self, builder: ListComplianceItemsInputBuilder) -> impl Future<Output = Result<ListComplianceItemsOutput, SdkError<ListComplianceItemsError>>>;
    fn list_compliance_summaries(&self, builder: ListComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListComplianceSummariesOutput, SdkError<ListComplianceSummariesError>>>;
    fn list_document_metadata_history(&self, builder: ListDocumentMetadataHistoryInputBuilder) -> impl Future<Output = Result<ListDocumentMetadataHistoryOutput, SdkError<ListDocumentMetadataHistoryError>>>;
    fn list_document_versions(&self, builder: ListDocumentVersionsInputBuilder) -> impl Future<Output = Result<ListDocumentVersionsOutput, SdkError<ListDocumentVersionsError>>>;
    fn list_documents(&self, builder: ListDocumentsInputBuilder) -> impl Future<Output = Result<ListDocumentsOutput, SdkError<ListDocumentsError>>>;
    fn list_inventory_entries(&self, builder: ListInventoryEntriesInputBuilder) -> impl Future<Output = Result<ListInventoryEntriesOutput, SdkError<ListInventoryEntriesError>>>;
    fn list_ops_item_events(&self, builder: ListOpsItemEventsInputBuilder) -> impl Future<Output = Result<ListOpsItemEventsOutput, SdkError<ListOpsItemEventsError>>>;
    fn list_ops_item_related_items(&self, builder: ListOpsItemRelatedItemsInputBuilder) -> impl Future<Output = Result<ListOpsItemRelatedItemsOutput, SdkError<ListOpsItemRelatedItemsError>>>;
    fn list_ops_metadata(&self, builder: ListOpsMetadataInputBuilder) -> impl Future<Output = Result<ListOpsMetadataOutput, SdkError<ListOpsMetadataError>>>;
    fn list_resource_compliance_summaries(&self, builder: ListResourceComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListResourceComplianceSummariesOutput, SdkError<ListResourceComplianceSummariesError>>>;
    fn list_resource_data_sync(&self, builder: ListResourceDataSyncInputBuilder) -> impl Future<Output = Result<ListResourceDataSyncOutput, SdkError<ListResourceDataSyncError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn modify_document_permission(&self, builder: ModifyDocumentPermissionInputBuilder) -> impl Future<Output = Result<ModifyDocumentPermissionOutput, SdkError<ModifyDocumentPermissionError>>>;
    fn put_compliance_items(&self, builder: PutComplianceItemsInputBuilder) -> impl Future<Output = Result<PutComplianceItemsOutput, SdkError<PutComplianceItemsError>>>;
    fn put_inventory(&self, builder: PutInventoryInputBuilder) -> impl Future<Output = Result<PutInventoryOutput, SdkError<PutInventoryError>>>;
    fn put_parameter(&self, builder: PutParameterInputBuilder) -> impl Future<Output = Result<PutParameterOutput, SdkError<PutParameterError>>>;
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>>;
    fn register_default_patch_baseline(&self, builder: RegisterDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<RegisterDefaultPatchBaselineOutput, SdkError<RegisterDefaultPatchBaselineError>>>;
    fn register_patch_baseline_for_patch_group(&self, builder: RegisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<RegisterPatchBaselineForPatchGroupOutput, SdkError<RegisterPatchBaselineForPatchGroupError>>>;
    fn register_target_with_maintenance_window(&self, builder: RegisterTargetWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTargetWithMaintenanceWindowOutput, SdkError<RegisterTargetWithMaintenanceWindowError>>>;
    fn register_task_with_maintenance_window(&self, builder: RegisterTaskWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTaskWithMaintenanceWindowOutput, SdkError<RegisterTaskWithMaintenanceWindowError>>>;
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>>;
    fn reset_service_setting(&self, builder: ResetServiceSettingInputBuilder) -> impl Future<Output = Result<ResetServiceSettingOutput, SdkError<ResetServiceSettingError>>>;
    fn resume_session(&self, builder: ResumeSessionInputBuilder) -> impl Future<Output = Result<ResumeSessionOutput, SdkError<ResumeSessionError>>>;
    fn send_automation_signal(&self, builder: SendAutomationSignalInputBuilder) -> impl Future<Output = Result<SendAutomationSignalOutput, SdkError<SendAutomationSignalError>>>;
    fn send_command(&self, builder: SendCommandInputBuilder) -> impl Future<Output = Result<SendCommandOutput, SdkError<SendCommandError>>>;
    fn start_associations_once(&self, builder: StartAssociationsOnceInputBuilder) -> impl Future<Output = Result<StartAssociationsOnceOutput, SdkError<StartAssociationsOnceError>>>;
    fn start_automation_execution(&self, builder: StartAutomationExecutionInputBuilder) -> impl Future<Output = Result<StartAutomationExecutionOutput, SdkError<StartAutomationExecutionError>>>;
    fn start_change_request_execution(&self, builder: StartChangeRequestExecutionInputBuilder) -> impl Future<Output = Result<StartChangeRequestExecutionOutput, SdkError<StartChangeRequestExecutionError>>>;
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>>;
    fn stop_automation_execution(&self, builder: StopAutomationExecutionInputBuilder) -> impl Future<Output = Result<StopAutomationExecutionOutput, SdkError<StopAutomationExecutionError>>>;
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>>;
    fn unlabel_parameter_version(&self, builder: UnlabelParameterVersionInputBuilder) -> impl Future<Output = Result<UnlabelParameterVersionOutput, SdkError<UnlabelParameterVersionError>>>;
    fn update_association(&self, builder: UpdateAssociationInputBuilder) -> impl Future<Output = Result<UpdateAssociationOutput, SdkError<UpdateAssociationError>>>;
    fn update_association_status(&self, builder: UpdateAssociationStatusInputBuilder) -> impl Future<Output = Result<UpdateAssociationStatusOutput, SdkError<UpdateAssociationStatusError>>>;
    fn update_document(&self, builder: UpdateDocumentInputBuilder) -> impl Future<Output = Result<UpdateDocumentOutput, SdkError<UpdateDocumentError>>>;
    fn update_document_default_version(&self, builder: UpdateDocumentDefaultVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentDefaultVersionOutput, SdkError<UpdateDocumentDefaultVersionError>>>;
    fn update_document_metadata(&self, builder: UpdateDocumentMetadataInputBuilder) -> impl Future<Output = Result<UpdateDocumentMetadataOutput, SdkError<UpdateDocumentMetadataError>>>;
    fn update_maintenance_window(&self, builder: UpdateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowOutput, SdkError<UpdateMaintenanceWindowError>>>;
    fn update_maintenance_window_target(&self, builder: UpdateMaintenanceWindowTargetInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTargetOutput, SdkError<UpdateMaintenanceWindowTargetError>>>;
    fn update_maintenance_window_task(&self, builder: UpdateMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTaskOutput, SdkError<UpdateMaintenanceWindowTaskError>>>;
    fn update_managed_instance_role(&self, builder: UpdateManagedInstanceRoleInputBuilder) -> impl Future<Output = Result<UpdateManagedInstanceRoleOutput, SdkError<UpdateManagedInstanceRoleError>>>;
    fn update_ops_item(&self, builder: UpdateOpsItemInputBuilder) -> impl Future<Output = Result<UpdateOpsItemOutput, SdkError<UpdateOpsItemError>>>;
    fn update_ops_metadata(&self, builder: UpdateOpsMetadataInputBuilder) -> impl Future<Output = Result<UpdateOpsMetadataOutput, SdkError<UpdateOpsMetadataError>>>;
    fn update_patch_baseline(&self, builder: UpdatePatchBaselineInputBuilder) -> impl Future<Output = Result<UpdatePatchBaselineOutput, SdkError<UpdatePatchBaselineError>>>;
    fn update_resource_data_sync(&self, builder: UpdateResourceDataSyncInputBuilder) -> impl Future<Output = Result<UpdateResourceDataSyncOutput, SdkError<UpdateResourceDataSyncError>>>;
    fn update_service_setting(&self, builder: UpdateServiceSettingInputBuilder) -> impl Future<Output = Result<UpdateServiceSettingOutput, SdkError<UpdateServiceSettingError>>>;
}
impl SSMClient for SSMClientImpl {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        builder.send_with(&self.0)
    }
    fn associate_ops_item_related_item(&self, builder: AssociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<AssociateOpsItemRelatedItemOutput, SdkError<AssociateOpsItemRelatedItemError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_command(&self, builder: CancelCommandInputBuilder) -> impl Future<Output = Result<CancelCommandOutput, SdkError<CancelCommandError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_maintenance_window_execution(&self, builder: CancelMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<CancelMaintenanceWindowExecutionOutput, SdkError<CancelMaintenanceWindowExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn create_activation(&self, builder: CreateActivationInputBuilder) -> impl Future<Output = Result<CreateActivationOutput, SdkError<CreateActivationError>>> {
        builder.send_with(&self.0)
    }
    fn create_association(&self, builder: CreateAssociationInputBuilder) -> impl Future<Output = Result<CreateAssociationOutput, SdkError<CreateAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_association_batch(&self, builder: CreateAssociationBatchInputBuilder) -> impl Future<Output = Result<CreateAssociationBatchOutput, SdkError<CreateAssociationBatchError>>> {
        builder.send_with(&self.0)
    }
    fn create_document(&self, builder: CreateDocumentInputBuilder) -> impl Future<Output = Result<CreateDocumentOutput, SdkError<CreateDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn create_maintenance_window(&self, builder: CreateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<CreateMaintenanceWindowOutput, SdkError<CreateMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn create_ops_item(&self, builder: CreateOpsItemInputBuilder) -> impl Future<Output = Result<CreateOpsItemOutput, SdkError<CreateOpsItemError>>> {
        builder.send_with(&self.0)
    }
    fn create_ops_metadata(&self, builder: CreateOpsMetadataInputBuilder) -> impl Future<Output = Result<CreateOpsMetadataOutput, SdkError<CreateOpsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn create_patch_baseline(&self, builder: CreatePatchBaselineInputBuilder) -> impl Future<Output = Result<CreatePatchBaselineOutput, SdkError<CreatePatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn create_resource_data_sync(&self, builder: CreateResourceDataSyncInputBuilder) -> impl Future<Output = Result<CreateResourceDataSyncOutput, SdkError<CreateResourceDataSyncError>>> {
        builder.send_with(&self.0)
    }
    fn delete_activation(&self, builder: DeleteActivationInputBuilder) -> impl Future<Output = Result<DeleteActivationOutput, SdkError<DeleteActivationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_document(&self, builder: DeleteDocumentInputBuilder) -> impl Future<Output = Result<DeleteDocumentOutput, SdkError<DeleteDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_inventory(&self, builder: DeleteInventoryInputBuilder) -> impl Future<Output = Result<DeleteInventoryOutput, SdkError<DeleteInventoryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_maintenance_window(&self, builder: DeleteMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeleteMaintenanceWindowOutput, SdkError<DeleteMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ops_item(&self, builder: DeleteOpsItemInputBuilder) -> impl Future<Output = Result<DeleteOpsItemOutput, SdkError<DeleteOpsItemError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ops_metadata(&self, builder: DeleteOpsMetadataInputBuilder) -> impl Future<Output = Result<DeleteOpsMetadataOutput, SdkError<DeleteOpsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn delete_parameter(&self, builder: DeleteParameterInputBuilder) -> impl Future<Output = Result<DeleteParameterOutput, SdkError<DeleteParameterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_parameters(&self, builder: DeleteParametersInputBuilder) -> impl Future<Output = Result<DeleteParametersOutput, SdkError<DeleteParametersError>>> {
        builder.send_with(&self.0)
    }
    fn delete_patch_baseline(&self, builder: DeletePatchBaselineInputBuilder) -> impl Future<Output = Result<DeletePatchBaselineOutput, SdkError<DeletePatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_data_sync(&self, builder: DeleteResourceDataSyncInputBuilder) -> impl Future<Output = Result<DeleteResourceDataSyncOutput, SdkError<DeleteResourceDataSyncError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_managed_instance(&self, builder: DeregisterManagedInstanceInputBuilder) -> impl Future<Output = Result<DeregisterManagedInstanceOutput, SdkError<DeregisterManagedInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_patch_baseline_for_patch_group(&self, builder: DeregisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<DeregisterPatchBaselineForPatchGroupOutput, SdkError<DeregisterPatchBaselineForPatchGroupError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_target_from_maintenance_window(&self, builder: DeregisterTargetFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTargetFromMaintenanceWindowOutput, SdkError<DeregisterTargetFromMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_task_from_maintenance_window(&self, builder: DeregisterTaskFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTaskFromMaintenanceWindowOutput, SdkError<DeregisterTaskFromMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn describe_activations(&self, builder: DescribeActivationsInputBuilder) -> impl Future<Output = Result<DescribeActivationsOutput, SdkError<DescribeActivationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_association(&self, builder: DescribeAssociationInputBuilder) -> impl Future<Output = Result<DescribeAssociationOutput, SdkError<DescribeAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_association_execution_targets(&self, builder: DescribeAssociationExecutionTargetsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionTargetsOutput, SdkError<DescribeAssociationExecutionTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_association_executions(&self, builder: DescribeAssociationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionsOutput, SdkError<DescribeAssociationExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_automation_executions(&self, builder: DescribeAutomationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationExecutionsOutput, SdkError<DescribeAutomationExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_automation_step_executions(&self, builder: DescribeAutomationStepExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationStepExecutionsOutput, SdkError<DescribeAutomationStepExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_available_patches(&self, builder: DescribeAvailablePatchesInputBuilder) -> impl Future<Output = Result<DescribeAvailablePatchesOutput, SdkError<DescribeAvailablePatchesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_document(&self, builder: DescribeDocumentInputBuilder) -> impl Future<Output = Result<DescribeDocumentOutput, SdkError<DescribeDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn describe_document_permission(&self, builder: DescribeDocumentPermissionInputBuilder) -> impl Future<Output = Result<DescribeDocumentPermissionOutput, SdkError<DescribeDocumentPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_effective_instance_associations(&self, builder: DescribeEffectiveInstanceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeEffectiveInstanceAssociationsOutput, SdkError<DescribeEffectiveInstanceAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_effective_patches_for_patch_baseline(&self, builder: DescribeEffectivePatchesForPatchBaselineInputBuilder) -> impl Future<Output = Result<DescribeEffectivePatchesForPatchBaselineOutput, SdkError<DescribeEffectivePatchesForPatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_associations_status(&self, builder: DescribeInstanceAssociationsStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceAssociationsStatusOutput, SdkError<DescribeInstanceAssociationsStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_information(&self, builder: DescribeInstanceInformationInputBuilder) -> impl Future<Output = Result<DescribeInstanceInformationOutput, SdkError<DescribeInstanceInformationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_patch_states(&self, builder: DescribeInstancePatchStatesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesOutput, SdkError<DescribeInstancePatchStatesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_patch_states_for_patch_group(&self, builder: DescribeInstancePatchStatesForPatchGroupInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesForPatchGroupOutput, SdkError<DescribeInstancePatchStatesForPatchGroupError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_patches(&self, builder: DescribeInstancePatchesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchesOutput, SdkError<DescribeInstancePatchesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instance_properties(&self, builder: DescribeInstancePropertiesInputBuilder) -> impl Future<Output = Result<DescribeInstancePropertiesOutput, SdkError<DescribeInstancePropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_inventory_deletions(&self, builder: DescribeInventoryDeletionsInputBuilder) -> impl Future<Output = Result<DescribeInventoryDeletionsOutput, SdkError<DescribeInventoryDeletionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_execution_task_invocations(&self, builder: DescribeMaintenanceWindowExecutionTaskInvocationsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTaskInvocationsOutput, SdkError<DescribeMaintenanceWindowExecutionTaskInvocationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_execution_tasks(&self, builder: DescribeMaintenanceWindowExecutionTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTasksOutput, SdkError<DescribeMaintenanceWindowExecutionTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_executions(&self, builder: DescribeMaintenanceWindowExecutionsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionsOutput, SdkError<DescribeMaintenanceWindowExecutionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_schedule(&self, builder: DescribeMaintenanceWindowScheduleInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowScheduleOutput, SdkError<DescribeMaintenanceWindowScheduleError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_targets(&self, builder: DescribeMaintenanceWindowTargetsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTargetsOutput, SdkError<DescribeMaintenanceWindowTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_window_tasks(&self, builder: DescribeMaintenanceWindowTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTasksOutput, SdkError<DescribeMaintenanceWindowTasksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_windows(&self, builder: DescribeMaintenanceWindowsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsOutput, SdkError<DescribeMaintenanceWindowsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_maintenance_windows_for_target(&self, builder: DescribeMaintenanceWindowsForTargetInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsForTargetOutput, SdkError<DescribeMaintenanceWindowsForTargetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_ops_items(&self, builder: DescribeOpsItemsInputBuilder) -> impl Future<Output = Result<DescribeOpsItemsOutput, SdkError<DescribeOpsItemsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_parameters(&self, builder: DescribeParametersInputBuilder) -> impl Future<Output = Result<DescribeParametersOutput, SdkError<DescribeParametersError>>> {
        builder.send_with(&self.0)
    }
    fn describe_patch_baselines(&self, builder: DescribePatchBaselinesInputBuilder) -> impl Future<Output = Result<DescribePatchBaselinesOutput, SdkError<DescribePatchBaselinesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_patch_group_state(&self, builder: DescribePatchGroupStateInputBuilder) -> impl Future<Output = Result<DescribePatchGroupStateOutput, SdkError<DescribePatchGroupStateError>>> {
        builder.send_with(&self.0)
    }
    fn describe_patch_groups(&self, builder: DescribePatchGroupsInputBuilder) -> impl Future<Output = Result<DescribePatchGroupsOutput, SdkError<DescribePatchGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_patch_properties(&self, builder: DescribePatchPropertiesInputBuilder) -> impl Future<Output = Result<DescribePatchPropertiesOutput, SdkError<DescribePatchPropertiesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_ops_item_related_item(&self, builder: DisassociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<DisassociateOpsItemRelatedItemOutput, SdkError<DisassociateOpsItemRelatedItemError>>> {
        builder.send_with(&self.0)
    }
    fn get_automation_execution(&self, builder: GetAutomationExecutionInputBuilder) -> impl Future<Output = Result<GetAutomationExecutionOutput, SdkError<GetAutomationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn get_calendar_state(&self, builder: GetCalendarStateInputBuilder) -> impl Future<Output = Result<GetCalendarStateOutput, SdkError<GetCalendarStateError>>> {
        builder.send_with(&self.0)
    }
    fn get_command_invocation(&self, builder: GetCommandInvocationInputBuilder) -> impl Future<Output = Result<GetCommandInvocationOutput, SdkError<GetCommandInvocationError>>> {
        builder.send_with(&self.0)
    }
    fn get_connection_status(&self, builder: GetConnectionStatusInputBuilder) -> impl Future<Output = Result<GetConnectionStatusOutput, SdkError<GetConnectionStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_default_patch_baseline(&self, builder: GetDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<GetDefaultPatchBaselineOutput, SdkError<GetDefaultPatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployable_patch_snapshot_for_instance(&self, builder: GetDeployablePatchSnapshotForInstanceInputBuilder) -> impl Future<Output = Result<GetDeployablePatchSnapshotForInstanceOutput, SdkError<GetDeployablePatchSnapshotForInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn get_document(&self, builder: GetDocumentInputBuilder) -> impl Future<Output = Result<GetDocumentOutput, SdkError<GetDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn get_inventory(&self, builder: GetInventoryInputBuilder) -> impl Future<Output = Result<GetInventoryOutput, SdkError<GetInventoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_inventory_schema(&self, builder: GetInventorySchemaInputBuilder) -> impl Future<Output = Result<GetInventorySchemaOutput, SdkError<GetInventorySchemaError>>> {
        builder.send_with(&self.0)
    }
    fn get_maintenance_window(&self, builder: GetMaintenanceWindowInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowOutput, SdkError<GetMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn get_maintenance_window_execution(&self, builder: GetMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionOutput, SdkError<GetMaintenanceWindowExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn get_maintenance_window_execution_task(&self, builder: GetMaintenanceWindowExecutionTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskOutput, SdkError<GetMaintenanceWindowExecutionTaskError>>> {
        builder.send_with(&self.0)
    }
    fn get_maintenance_window_execution_task_invocation(&self, builder: GetMaintenanceWindowExecutionTaskInvocationInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskInvocationOutput, SdkError<GetMaintenanceWindowExecutionTaskInvocationError>>> {
        builder.send_with(&self.0)
    }
    fn get_maintenance_window_task(&self, builder: GetMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowTaskOutput, SdkError<GetMaintenanceWindowTaskError>>> {
        builder.send_with(&self.0)
    }
    fn get_ops_item(&self, builder: GetOpsItemInputBuilder) -> impl Future<Output = Result<GetOpsItemOutput, SdkError<GetOpsItemError>>> {
        builder.send_with(&self.0)
    }
    fn get_ops_metadata(&self, builder: GetOpsMetadataInputBuilder) -> impl Future<Output = Result<GetOpsMetadataOutput, SdkError<GetOpsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_ops_summary(&self, builder: GetOpsSummaryInputBuilder) -> impl Future<Output = Result<GetOpsSummaryOutput, SdkError<GetOpsSummaryError>>> {
        builder.send_with(&self.0)
    }
    fn get_parameter(&self, builder: GetParameterInputBuilder) -> impl Future<Output = Result<GetParameterOutput, SdkError<GetParameterError>>> {
        builder.send_with(&self.0)
    }
    fn get_parameter_history(&self, builder: GetParameterHistoryInputBuilder) -> impl Future<Output = Result<GetParameterHistoryOutput, SdkError<GetParameterHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_parameters(&self, builder: GetParametersInputBuilder) -> impl Future<Output = Result<GetParametersOutput, SdkError<GetParametersError>>> {
        builder.send_with(&self.0)
    }
    fn get_parameters_by_path(&self, builder: GetParametersByPathInputBuilder) -> impl Future<Output = Result<GetParametersByPathOutput, SdkError<GetParametersByPathError>>> {
        builder.send_with(&self.0)
    }
    fn get_patch_baseline(&self, builder: GetPatchBaselineInputBuilder) -> impl Future<Output = Result<GetPatchBaselineOutput, SdkError<GetPatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn get_patch_baseline_for_patch_group(&self, builder: GetPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<GetPatchBaselineForPatchGroupOutput, SdkError<GetPatchBaselineForPatchGroupError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn get_service_setting(&self, builder: GetServiceSettingInputBuilder) -> impl Future<Output = Result<GetServiceSettingOutput, SdkError<GetServiceSettingError>>> {
        builder.send_with(&self.0)
    }
    fn label_parameter_version(&self, builder: LabelParameterVersionInputBuilder) -> impl Future<Output = Result<LabelParameterVersionOutput, SdkError<LabelParameterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn list_association_versions(&self, builder: ListAssociationVersionsInputBuilder) -> impl Future<Output = Result<ListAssociationVersionsOutput, SdkError<ListAssociationVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_command_invocations(&self, builder: ListCommandInvocationsInputBuilder) -> impl Future<Output = Result<ListCommandInvocationsOutput, SdkError<ListCommandInvocationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_commands(&self, builder: ListCommandsInputBuilder) -> impl Future<Output = Result<ListCommandsOutput, SdkError<ListCommandsError>>> {
        builder.send_with(&self.0)
    }
    fn list_compliance_items(&self, builder: ListComplianceItemsInputBuilder) -> impl Future<Output = Result<ListComplianceItemsOutput, SdkError<ListComplianceItemsError>>> {
        builder.send_with(&self.0)
    }
    fn list_compliance_summaries(&self, builder: ListComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListComplianceSummariesOutput, SdkError<ListComplianceSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_document_metadata_history(&self, builder: ListDocumentMetadataHistoryInputBuilder) -> impl Future<Output = Result<ListDocumentMetadataHistoryOutput, SdkError<ListDocumentMetadataHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn list_document_versions(&self, builder: ListDocumentVersionsInputBuilder) -> impl Future<Output = Result<ListDocumentVersionsOutput, SdkError<ListDocumentVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_documents(&self, builder: ListDocumentsInputBuilder) -> impl Future<Output = Result<ListDocumentsOutput, SdkError<ListDocumentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_inventory_entries(&self, builder: ListInventoryEntriesInputBuilder) -> impl Future<Output = Result<ListInventoryEntriesOutput, SdkError<ListInventoryEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_ops_item_events(&self, builder: ListOpsItemEventsInputBuilder) -> impl Future<Output = Result<ListOpsItemEventsOutput, SdkError<ListOpsItemEventsError>>> {
        builder.send_with(&self.0)
    }
    fn list_ops_item_related_items(&self, builder: ListOpsItemRelatedItemsInputBuilder) -> impl Future<Output = Result<ListOpsItemRelatedItemsOutput, SdkError<ListOpsItemRelatedItemsError>>> {
        builder.send_with(&self.0)
    }
    fn list_ops_metadata(&self, builder: ListOpsMetadataInputBuilder) -> impl Future<Output = Result<ListOpsMetadataOutput, SdkError<ListOpsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_compliance_summaries(&self, builder: ListResourceComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListResourceComplianceSummariesOutput, SdkError<ListResourceComplianceSummariesError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_data_sync(&self, builder: ListResourceDataSyncInputBuilder) -> impl Future<Output = Result<ListResourceDataSyncOutput, SdkError<ListResourceDataSyncError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn modify_document_permission(&self, builder: ModifyDocumentPermissionInputBuilder) -> impl Future<Output = Result<ModifyDocumentPermissionOutput, SdkError<ModifyDocumentPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn put_compliance_items(&self, builder: PutComplianceItemsInputBuilder) -> impl Future<Output = Result<PutComplianceItemsOutput, SdkError<PutComplianceItemsError>>> {
        builder.send_with(&self.0)
    }
    fn put_inventory(&self, builder: PutInventoryInputBuilder) -> impl Future<Output = Result<PutInventoryOutput, SdkError<PutInventoryError>>> {
        builder.send_with(&self.0)
    }
    fn put_parameter(&self, builder: PutParameterInputBuilder) -> impl Future<Output = Result<PutParameterOutput, SdkError<PutParameterError>>> {
        builder.send_with(&self.0)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn register_default_patch_baseline(&self, builder: RegisterDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<RegisterDefaultPatchBaselineOutput, SdkError<RegisterDefaultPatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn register_patch_baseline_for_patch_group(&self, builder: RegisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<RegisterPatchBaselineForPatchGroupOutput, SdkError<RegisterPatchBaselineForPatchGroupError>>> {
        builder.send_with(&self.0)
    }
    fn register_target_with_maintenance_window(&self, builder: RegisterTargetWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTargetWithMaintenanceWindowOutput, SdkError<RegisterTargetWithMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn register_task_with_maintenance_window(&self, builder: RegisterTaskWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTaskWithMaintenanceWindowOutput, SdkError<RegisterTaskWithMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        builder.send_with(&self.0)
    }
    fn reset_service_setting(&self, builder: ResetServiceSettingInputBuilder) -> impl Future<Output = Result<ResetServiceSettingOutput, SdkError<ResetServiceSettingError>>> {
        builder.send_with(&self.0)
    }
    fn resume_session(&self, builder: ResumeSessionInputBuilder) -> impl Future<Output = Result<ResumeSessionOutput, SdkError<ResumeSessionError>>> {
        builder.send_with(&self.0)
    }
    fn send_automation_signal(&self, builder: SendAutomationSignalInputBuilder) -> impl Future<Output = Result<SendAutomationSignalOutput, SdkError<SendAutomationSignalError>>> {
        builder.send_with(&self.0)
    }
    fn send_command(&self, builder: SendCommandInputBuilder) -> impl Future<Output = Result<SendCommandOutput, SdkError<SendCommandError>>> {
        builder.send_with(&self.0)
    }
    fn start_associations_once(&self, builder: StartAssociationsOnceInputBuilder) -> impl Future<Output = Result<StartAssociationsOnceOutput, SdkError<StartAssociationsOnceError>>> {
        builder.send_with(&self.0)
    }
    fn start_automation_execution(&self, builder: StartAutomationExecutionInputBuilder) -> impl Future<Output = Result<StartAutomationExecutionOutput, SdkError<StartAutomationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn start_change_request_execution(&self, builder: StartChangeRequestExecutionInputBuilder) -> impl Future<Output = Result<StartChangeRequestExecutionOutput, SdkError<StartChangeRequestExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_automation_execution(&self, builder: StopAutomationExecutionInputBuilder) -> impl Future<Output = Result<StopAutomationExecutionOutput, SdkError<StopAutomationExecutionError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>> {
        builder.send_with(&self.0)
    }
    fn unlabel_parameter_version(&self, builder: UnlabelParameterVersionInputBuilder) -> impl Future<Output = Result<UnlabelParameterVersionOutput, SdkError<UnlabelParameterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_association(&self, builder: UpdateAssociationInputBuilder) -> impl Future<Output = Result<UpdateAssociationOutput, SdkError<UpdateAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn update_association_status(&self, builder: UpdateAssociationStatusInputBuilder) -> impl Future<Output = Result<UpdateAssociationStatusOutput, SdkError<UpdateAssociationStatusError>>> {
        builder.send_with(&self.0)
    }
    fn update_document(&self, builder: UpdateDocumentInputBuilder) -> impl Future<Output = Result<UpdateDocumentOutput, SdkError<UpdateDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn update_document_default_version(&self, builder: UpdateDocumentDefaultVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentDefaultVersionOutput, SdkError<UpdateDocumentDefaultVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_document_metadata(&self, builder: UpdateDocumentMetadataInputBuilder) -> impl Future<Output = Result<UpdateDocumentMetadataOutput, SdkError<UpdateDocumentMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn update_maintenance_window(&self, builder: UpdateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowOutput, SdkError<UpdateMaintenanceWindowError>>> {
        builder.send_with(&self.0)
    }
    fn update_maintenance_window_target(&self, builder: UpdateMaintenanceWindowTargetInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTargetOutput, SdkError<UpdateMaintenanceWindowTargetError>>> {
        builder.send_with(&self.0)
    }
    fn update_maintenance_window_task(&self, builder: UpdateMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTaskOutput, SdkError<UpdateMaintenanceWindowTaskError>>> {
        builder.send_with(&self.0)
    }
    fn update_managed_instance_role(&self, builder: UpdateManagedInstanceRoleInputBuilder) -> impl Future<Output = Result<UpdateManagedInstanceRoleOutput, SdkError<UpdateManagedInstanceRoleError>>> {
        builder.send_with(&self.0)
    }
    fn update_ops_item(&self, builder: UpdateOpsItemInputBuilder) -> impl Future<Output = Result<UpdateOpsItemOutput, SdkError<UpdateOpsItemError>>> {
        builder.send_with(&self.0)
    }
    fn update_ops_metadata(&self, builder: UpdateOpsMetadataInputBuilder) -> impl Future<Output = Result<UpdateOpsMetadataOutput, SdkError<UpdateOpsMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn update_patch_baseline(&self, builder: UpdatePatchBaselineInputBuilder) -> impl Future<Output = Result<UpdatePatchBaselineOutput, SdkError<UpdatePatchBaselineError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_data_sync(&self, builder: UpdateResourceDataSyncInputBuilder) -> impl Future<Output = Result<UpdateResourceDataSyncOutput, SdkError<UpdateResourceDataSyncError>>> {
        builder.send_with(&self.0)
    }
    fn update_service_setting(&self, builder: UpdateServiceSettingInputBuilder) -> impl Future<Output = Result<UpdateServiceSettingOutput, SdkError<UpdateServiceSettingError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: SSMClient> SSMClient for &T {
    fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> impl Future<Output = Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>> {
        (*self).add_tags_to_resource(builder)
    }
    fn associate_ops_item_related_item(&self, builder: AssociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<AssociateOpsItemRelatedItemOutput, SdkError<AssociateOpsItemRelatedItemError>>> {
        (*self).associate_ops_item_related_item(builder)
    }
    fn cancel_command(&self, builder: CancelCommandInputBuilder) -> impl Future<Output = Result<CancelCommandOutput, SdkError<CancelCommandError>>> {
        (*self).cancel_command(builder)
    }
    fn cancel_maintenance_window_execution(&self, builder: CancelMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<CancelMaintenanceWindowExecutionOutput, SdkError<CancelMaintenanceWindowExecutionError>>> {
        (*self).cancel_maintenance_window_execution(builder)
    }
    fn create_activation(&self, builder: CreateActivationInputBuilder) -> impl Future<Output = Result<CreateActivationOutput, SdkError<CreateActivationError>>> {
        (*self).create_activation(builder)
    }
    fn create_association(&self, builder: CreateAssociationInputBuilder) -> impl Future<Output = Result<CreateAssociationOutput, SdkError<CreateAssociationError>>> {
        (*self).create_association(builder)
    }
    fn create_association_batch(&self, builder: CreateAssociationBatchInputBuilder) -> impl Future<Output = Result<CreateAssociationBatchOutput, SdkError<CreateAssociationBatchError>>> {
        (*self).create_association_batch(builder)
    }
    fn create_document(&self, builder: CreateDocumentInputBuilder) -> impl Future<Output = Result<CreateDocumentOutput, SdkError<CreateDocumentError>>> {
        (*self).create_document(builder)
    }
    fn create_maintenance_window(&self, builder: CreateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<CreateMaintenanceWindowOutput, SdkError<CreateMaintenanceWindowError>>> {
        (*self).create_maintenance_window(builder)
    }
    fn create_ops_item(&self, builder: CreateOpsItemInputBuilder) -> impl Future<Output = Result<CreateOpsItemOutput, SdkError<CreateOpsItemError>>> {
        (*self).create_ops_item(builder)
    }
    fn create_ops_metadata(&self, builder: CreateOpsMetadataInputBuilder) -> impl Future<Output = Result<CreateOpsMetadataOutput, SdkError<CreateOpsMetadataError>>> {
        (*self).create_ops_metadata(builder)
    }
    fn create_patch_baseline(&self, builder: CreatePatchBaselineInputBuilder) -> impl Future<Output = Result<CreatePatchBaselineOutput, SdkError<CreatePatchBaselineError>>> {
        (*self).create_patch_baseline(builder)
    }
    fn create_resource_data_sync(&self, builder: CreateResourceDataSyncInputBuilder) -> impl Future<Output = Result<CreateResourceDataSyncOutput, SdkError<CreateResourceDataSyncError>>> {
        (*self).create_resource_data_sync(builder)
    }
    fn delete_activation(&self, builder: DeleteActivationInputBuilder) -> impl Future<Output = Result<DeleteActivationOutput, SdkError<DeleteActivationError>>> {
        (*self).delete_activation(builder)
    }
    fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> impl Future<Output = Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>> {
        (*self).delete_association(builder)
    }
    fn delete_document(&self, builder: DeleteDocumentInputBuilder) -> impl Future<Output = Result<DeleteDocumentOutput, SdkError<DeleteDocumentError>>> {
        (*self).delete_document(builder)
    }
    fn delete_inventory(&self, builder: DeleteInventoryInputBuilder) -> impl Future<Output = Result<DeleteInventoryOutput, SdkError<DeleteInventoryError>>> {
        (*self).delete_inventory(builder)
    }
    fn delete_maintenance_window(&self, builder: DeleteMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeleteMaintenanceWindowOutput, SdkError<DeleteMaintenanceWindowError>>> {
        (*self).delete_maintenance_window(builder)
    }
    fn delete_ops_item(&self, builder: DeleteOpsItemInputBuilder) -> impl Future<Output = Result<DeleteOpsItemOutput, SdkError<DeleteOpsItemError>>> {
        (*self).delete_ops_item(builder)
    }
    fn delete_ops_metadata(&self, builder: DeleteOpsMetadataInputBuilder) -> impl Future<Output = Result<DeleteOpsMetadataOutput, SdkError<DeleteOpsMetadataError>>> {
        (*self).delete_ops_metadata(builder)
    }
    fn delete_parameter(&self, builder: DeleteParameterInputBuilder) -> impl Future<Output = Result<DeleteParameterOutput, SdkError<DeleteParameterError>>> {
        (*self).delete_parameter(builder)
    }
    fn delete_parameters(&self, builder: DeleteParametersInputBuilder) -> impl Future<Output = Result<DeleteParametersOutput, SdkError<DeleteParametersError>>> {
        (*self).delete_parameters(builder)
    }
    fn delete_patch_baseline(&self, builder: DeletePatchBaselineInputBuilder) -> impl Future<Output = Result<DeletePatchBaselineOutput, SdkError<DeletePatchBaselineError>>> {
        (*self).delete_patch_baseline(builder)
    }
    fn delete_resource_data_sync(&self, builder: DeleteResourceDataSyncInputBuilder) -> impl Future<Output = Result<DeleteResourceDataSyncOutput, SdkError<DeleteResourceDataSyncError>>> {
        (*self).delete_resource_data_sync(builder)
    }
    fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> impl Future<Output = Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>> {
        (*self).delete_resource_policy(builder)
    }
    fn deregister_managed_instance(&self, builder: DeregisterManagedInstanceInputBuilder) -> impl Future<Output = Result<DeregisterManagedInstanceOutput, SdkError<DeregisterManagedInstanceError>>> {
        (*self).deregister_managed_instance(builder)
    }
    fn deregister_patch_baseline_for_patch_group(&self, builder: DeregisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<DeregisterPatchBaselineForPatchGroupOutput, SdkError<DeregisterPatchBaselineForPatchGroupError>>> {
        (*self).deregister_patch_baseline_for_patch_group(builder)
    }
    fn deregister_target_from_maintenance_window(&self, builder: DeregisterTargetFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTargetFromMaintenanceWindowOutput, SdkError<DeregisterTargetFromMaintenanceWindowError>>> {
        (*self).deregister_target_from_maintenance_window(builder)
    }
    fn deregister_task_from_maintenance_window(&self, builder: DeregisterTaskFromMaintenanceWindowInputBuilder) -> impl Future<Output = Result<DeregisterTaskFromMaintenanceWindowOutput, SdkError<DeregisterTaskFromMaintenanceWindowError>>> {
        (*self).deregister_task_from_maintenance_window(builder)
    }
    fn describe_activations(&self, builder: DescribeActivationsInputBuilder) -> impl Future<Output = Result<DescribeActivationsOutput, SdkError<DescribeActivationsError>>> {
        (*self).describe_activations(builder)
    }
    fn describe_association(&self, builder: DescribeAssociationInputBuilder) -> impl Future<Output = Result<DescribeAssociationOutput, SdkError<DescribeAssociationError>>> {
        (*self).describe_association(builder)
    }
    fn describe_association_execution_targets(&self, builder: DescribeAssociationExecutionTargetsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionTargetsOutput, SdkError<DescribeAssociationExecutionTargetsError>>> {
        (*self).describe_association_execution_targets(builder)
    }
    fn describe_association_executions(&self, builder: DescribeAssociationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAssociationExecutionsOutput, SdkError<DescribeAssociationExecutionsError>>> {
        (*self).describe_association_executions(builder)
    }
    fn describe_automation_executions(&self, builder: DescribeAutomationExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationExecutionsOutput, SdkError<DescribeAutomationExecutionsError>>> {
        (*self).describe_automation_executions(builder)
    }
    fn describe_automation_step_executions(&self, builder: DescribeAutomationStepExecutionsInputBuilder) -> impl Future<Output = Result<DescribeAutomationStepExecutionsOutput, SdkError<DescribeAutomationStepExecutionsError>>> {
        (*self).describe_automation_step_executions(builder)
    }
    fn describe_available_patches(&self, builder: DescribeAvailablePatchesInputBuilder) -> impl Future<Output = Result<DescribeAvailablePatchesOutput, SdkError<DescribeAvailablePatchesError>>> {
        (*self).describe_available_patches(builder)
    }
    fn describe_document(&self, builder: DescribeDocumentInputBuilder) -> impl Future<Output = Result<DescribeDocumentOutput, SdkError<DescribeDocumentError>>> {
        (*self).describe_document(builder)
    }
    fn describe_document_permission(&self, builder: DescribeDocumentPermissionInputBuilder) -> impl Future<Output = Result<DescribeDocumentPermissionOutput, SdkError<DescribeDocumentPermissionError>>> {
        (*self).describe_document_permission(builder)
    }
    fn describe_effective_instance_associations(&self, builder: DescribeEffectiveInstanceAssociationsInputBuilder) -> impl Future<Output = Result<DescribeEffectiveInstanceAssociationsOutput, SdkError<DescribeEffectiveInstanceAssociationsError>>> {
        (*self).describe_effective_instance_associations(builder)
    }
    fn describe_effective_patches_for_patch_baseline(&self, builder: DescribeEffectivePatchesForPatchBaselineInputBuilder) -> impl Future<Output = Result<DescribeEffectivePatchesForPatchBaselineOutput, SdkError<DescribeEffectivePatchesForPatchBaselineError>>> {
        (*self).describe_effective_patches_for_patch_baseline(builder)
    }
    fn describe_instance_associations_status(&self, builder: DescribeInstanceAssociationsStatusInputBuilder) -> impl Future<Output = Result<DescribeInstanceAssociationsStatusOutput, SdkError<DescribeInstanceAssociationsStatusError>>> {
        (*self).describe_instance_associations_status(builder)
    }
    fn describe_instance_information(&self, builder: DescribeInstanceInformationInputBuilder) -> impl Future<Output = Result<DescribeInstanceInformationOutput, SdkError<DescribeInstanceInformationError>>> {
        (*self).describe_instance_information(builder)
    }
    fn describe_instance_patch_states(&self, builder: DescribeInstancePatchStatesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesOutput, SdkError<DescribeInstancePatchStatesError>>> {
        (*self).describe_instance_patch_states(builder)
    }
    fn describe_instance_patch_states_for_patch_group(&self, builder: DescribeInstancePatchStatesForPatchGroupInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchStatesForPatchGroupOutput, SdkError<DescribeInstancePatchStatesForPatchGroupError>>> {
        (*self).describe_instance_patch_states_for_patch_group(builder)
    }
    fn describe_instance_patches(&self, builder: DescribeInstancePatchesInputBuilder) -> impl Future<Output = Result<DescribeInstancePatchesOutput, SdkError<DescribeInstancePatchesError>>> {
        (*self).describe_instance_patches(builder)
    }
    fn describe_instance_properties(&self, builder: DescribeInstancePropertiesInputBuilder) -> impl Future<Output = Result<DescribeInstancePropertiesOutput, SdkError<DescribeInstancePropertiesError>>> {
        (*self).describe_instance_properties(builder)
    }
    fn describe_inventory_deletions(&self, builder: DescribeInventoryDeletionsInputBuilder) -> impl Future<Output = Result<DescribeInventoryDeletionsOutput, SdkError<DescribeInventoryDeletionsError>>> {
        (*self).describe_inventory_deletions(builder)
    }
    fn describe_maintenance_window_execution_task_invocations(&self, builder: DescribeMaintenanceWindowExecutionTaskInvocationsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTaskInvocationsOutput, SdkError<DescribeMaintenanceWindowExecutionTaskInvocationsError>>> {
        (*self).describe_maintenance_window_execution_task_invocations(builder)
    }
    fn describe_maintenance_window_execution_tasks(&self, builder: DescribeMaintenanceWindowExecutionTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionTasksOutput, SdkError<DescribeMaintenanceWindowExecutionTasksError>>> {
        (*self).describe_maintenance_window_execution_tasks(builder)
    }
    fn describe_maintenance_window_executions(&self, builder: DescribeMaintenanceWindowExecutionsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowExecutionsOutput, SdkError<DescribeMaintenanceWindowExecutionsError>>> {
        (*self).describe_maintenance_window_executions(builder)
    }
    fn describe_maintenance_window_schedule(&self, builder: DescribeMaintenanceWindowScheduleInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowScheduleOutput, SdkError<DescribeMaintenanceWindowScheduleError>>> {
        (*self).describe_maintenance_window_schedule(builder)
    }
    fn describe_maintenance_window_targets(&self, builder: DescribeMaintenanceWindowTargetsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTargetsOutput, SdkError<DescribeMaintenanceWindowTargetsError>>> {
        (*self).describe_maintenance_window_targets(builder)
    }
    fn describe_maintenance_window_tasks(&self, builder: DescribeMaintenanceWindowTasksInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowTasksOutput, SdkError<DescribeMaintenanceWindowTasksError>>> {
        (*self).describe_maintenance_window_tasks(builder)
    }
    fn describe_maintenance_windows(&self, builder: DescribeMaintenanceWindowsInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsOutput, SdkError<DescribeMaintenanceWindowsError>>> {
        (*self).describe_maintenance_windows(builder)
    }
    fn describe_maintenance_windows_for_target(&self, builder: DescribeMaintenanceWindowsForTargetInputBuilder) -> impl Future<Output = Result<DescribeMaintenanceWindowsForTargetOutput, SdkError<DescribeMaintenanceWindowsForTargetError>>> {
        (*self).describe_maintenance_windows_for_target(builder)
    }
    fn describe_ops_items(&self, builder: DescribeOpsItemsInputBuilder) -> impl Future<Output = Result<DescribeOpsItemsOutput, SdkError<DescribeOpsItemsError>>> {
        (*self).describe_ops_items(builder)
    }
    fn describe_parameters(&self, builder: DescribeParametersInputBuilder) -> impl Future<Output = Result<DescribeParametersOutput, SdkError<DescribeParametersError>>> {
        (*self).describe_parameters(builder)
    }
    fn describe_patch_baselines(&self, builder: DescribePatchBaselinesInputBuilder) -> impl Future<Output = Result<DescribePatchBaselinesOutput, SdkError<DescribePatchBaselinesError>>> {
        (*self).describe_patch_baselines(builder)
    }
    fn describe_patch_group_state(&self, builder: DescribePatchGroupStateInputBuilder) -> impl Future<Output = Result<DescribePatchGroupStateOutput, SdkError<DescribePatchGroupStateError>>> {
        (*self).describe_patch_group_state(builder)
    }
    fn describe_patch_groups(&self, builder: DescribePatchGroupsInputBuilder) -> impl Future<Output = Result<DescribePatchGroupsOutput, SdkError<DescribePatchGroupsError>>> {
        (*self).describe_patch_groups(builder)
    }
    fn describe_patch_properties(&self, builder: DescribePatchPropertiesInputBuilder) -> impl Future<Output = Result<DescribePatchPropertiesOutput, SdkError<DescribePatchPropertiesError>>> {
        (*self).describe_patch_properties(builder)
    }
    fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> impl Future<Output = Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>> {
        (*self).describe_sessions(builder)
    }
    fn disassociate_ops_item_related_item(&self, builder: DisassociateOpsItemRelatedItemInputBuilder) -> impl Future<Output = Result<DisassociateOpsItemRelatedItemOutput, SdkError<DisassociateOpsItemRelatedItemError>>> {
        (*self).disassociate_ops_item_related_item(builder)
    }
    fn get_automation_execution(&self, builder: GetAutomationExecutionInputBuilder) -> impl Future<Output = Result<GetAutomationExecutionOutput, SdkError<GetAutomationExecutionError>>> {
        (*self).get_automation_execution(builder)
    }
    fn get_calendar_state(&self, builder: GetCalendarStateInputBuilder) -> impl Future<Output = Result<GetCalendarStateOutput, SdkError<GetCalendarStateError>>> {
        (*self).get_calendar_state(builder)
    }
    fn get_command_invocation(&self, builder: GetCommandInvocationInputBuilder) -> impl Future<Output = Result<GetCommandInvocationOutput, SdkError<GetCommandInvocationError>>> {
        (*self).get_command_invocation(builder)
    }
    fn get_connection_status(&self, builder: GetConnectionStatusInputBuilder) -> impl Future<Output = Result<GetConnectionStatusOutput, SdkError<GetConnectionStatusError>>> {
        (*self).get_connection_status(builder)
    }
    fn get_default_patch_baseline(&self, builder: GetDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<GetDefaultPatchBaselineOutput, SdkError<GetDefaultPatchBaselineError>>> {
        (*self).get_default_patch_baseline(builder)
    }
    fn get_deployable_patch_snapshot_for_instance(&self, builder: GetDeployablePatchSnapshotForInstanceInputBuilder) -> impl Future<Output = Result<GetDeployablePatchSnapshotForInstanceOutput, SdkError<GetDeployablePatchSnapshotForInstanceError>>> {
        (*self).get_deployable_patch_snapshot_for_instance(builder)
    }
    fn get_document(&self, builder: GetDocumentInputBuilder) -> impl Future<Output = Result<GetDocumentOutput, SdkError<GetDocumentError>>> {
        (*self).get_document(builder)
    }
    fn get_inventory(&self, builder: GetInventoryInputBuilder) -> impl Future<Output = Result<GetInventoryOutput, SdkError<GetInventoryError>>> {
        (*self).get_inventory(builder)
    }
    fn get_inventory_schema(&self, builder: GetInventorySchemaInputBuilder) -> impl Future<Output = Result<GetInventorySchemaOutput, SdkError<GetInventorySchemaError>>> {
        (*self).get_inventory_schema(builder)
    }
    fn get_maintenance_window(&self, builder: GetMaintenanceWindowInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowOutput, SdkError<GetMaintenanceWindowError>>> {
        (*self).get_maintenance_window(builder)
    }
    fn get_maintenance_window_execution(&self, builder: GetMaintenanceWindowExecutionInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionOutput, SdkError<GetMaintenanceWindowExecutionError>>> {
        (*self).get_maintenance_window_execution(builder)
    }
    fn get_maintenance_window_execution_task(&self, builder: GetMaintenanceWindowExecutionTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskOutput, SdkError<GetMaintenanceWindowExecutionTaskError>>> {
        (*self).get_maintenance_window_execution_task(builder)
    }
    fn get_maintenance_window_execution_task_invocation(&self, builder: GetMaintenanceWindowExecutionTaskInvocationInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowExecutionTaskInvocationOutput, SdkError<GetMaintenanceWindowExecutionTaskInvocationError>>> {
        (*self).get_maintenance_window_execution_task_invocation(builder)
    }
    fn get_maintenance_window_task(&self, builder: GetMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<GetMaintenanceWindowTaskOutput, SdkError<GetMaintenanceWindowTaskError>>> {
        (*self).get_maintenance_window_task(builder)
    }
    fn get_ops_item(&self, builder: GetOpsItemInputBuilder) -> impl Future<Output = Result<GetOpsItemOutput, SdkError<GetOpsItemError>>> {
        (*self).get_ops_item(builder)
    }
    fn get_ops_metadata(&self, builder: GetOpsMetadataInputBuilder) -> impl Future<Output = Result<GetOpsMetadataOutput, SdkError<GetOpsMetadataError>>> {
        (*self).get_ops_metadata(builder)
    }
    fn get_ops_summary(&self, builder: GetOpsSummaryInputBuilder) -> impl Future<Output = Result<GetOpsSummaryOutput, SdkError<GetOpsSummaryError>>> {
        (*self).get_ops_summary(builder)
    }
    fn get_parameter(&self, builder: GetParameterInputBuilder) -> impl Future<Output = Result<GetParameterOutput, SdkError<GetParameterError>>> {
        (*self).get_parameter(builder)
    }
    fn get_parameter_history(&self, builder: GetParameterHistoryInputBuilder) -> impl Future<Output = Result<GetParameterHistoryOutput, SdkError<GetParameterHistoryError>>> {
        (*self).get_parameter_history(builder)
    }
    fn get_parameters(&self, builder: GetParametersInputBuilder) -> impl Future<Output = Result<GetParametersOutput, SdkError<GetParametersError>>> {
        (*self).get_parameters(builder)
    }
    fn get_parameters_by_path(&self, builder: GetParametersByPathInputBuilder) -> impl Future<Output = Result<GetParametersByPathOutput, SdkError<GetParametersByPathError>>> {
        (*self).get_parameters_by_path(builder)
    }
    fn get_patch_baseline(&self, builder: GetPatchBaselineInputBuilder) -> impl Future<Output = Result<GetPatchBaselineOutput, SdkError<GetPatchBaselineError>>> {
        (*self).get_patch_baseline(builder)
    }
    fn get_patch_baseline_for_patch_group(&self, builder: GetPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<GetPatchBaselineForPatchGroupOutput, SdkError<GetPatchBaselineForPatchGroupError>>> {
        (*self).get_patch_baseline_for_patch_group(builder)
    }
    fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> impl Future<Output = Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>> {
        (*self).get_resource_policies(builder)
    }
    fn get_service_setting(&self, builder: GetServiceSettingInputBuilder) -> impl Future<Output = Result<GetServiceSettingOutput, SdkError<GetServiceSettingError>>> {
        (*self).get_service_setting(builder)
    }
    fn label_parameter_version(&self, builder: LabelParameterVersionInputBuilder) -> impl Future<Output = Result<LabelParameterVersionOutput, SdkError<LabelParameterVersionError>>> {
        (*self).label_parameter_version(builder)
    }
    fn list_association_versions(&self, builder: ListAssociationVersionsInputBuilder) -> impl Future<Output = Result<ListAssociationVersionsOutput, SdkError<ListAssociationVersionsError>>> {
        (*self).list_association_versions(builder)
    }
    fn list_associations(&self, builder: ListAssociationsInputBuilder) -> impl Future<Output = Result<ListAssociationsOutput, SdkError<ListAssociationsError>>> {
        (*self).list_associations(builder)
    }
    fn list_command_invocations(&self, builder: ListCommandInvocationsInputBuilder) -> impl Future<Output = Result<ListCommandInvocationsOutput, SdkError<ListCommandInvocationsError>>> {
        (*self).list_command_invocations(builder)
    }
    fn list_commands(&self, builder: ListCommandsInputBuilder) -> impl Future<Output = Result<ListCommandsOutput, SdkError<ListCommandsError>>> {
        (*self).list_commands(builder)
    }
    fn list_compliance_items(&self, builder: ListComplianceItemsInputBuilder) -> impl Future<Output = Result<ListComplianceItemsOutput, SdkError<ListComplianceItemsError>>> {
        (*self).list_compliance_items(builder)
    }
    fn list_compliance_summaries(&self, builder: ListComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListComplianceSummariesOutput, SdkError<ListComplianceSummariesError>>> {
        (*self).list_compliance_summaries(builder)
    }
    fn list_document_metadata_history(&self, builder: ListDocumentMetadataHistoryInputBuilder) -> impl Future<Output = Result<ListDocumentMetadataHistoryOutput, SdkError<ListDocumentMetadataHistoryError>>> {
        (*self).list_document_metadata_history(builder)
    }
    fn list_document_versions(&self, builder: ListDocumentVersionsInputBuilder) -> impl Future<Output = Result<ListDocumentVersionsOutput, SdkError<ListDocumentVersionsError>>> {
        (*self).list_document_versions(builder)
    }
    fn list_documents(&self, builder: ListDocumentsInputBuilder) -> impl Future<Output = Result<ListDocumentsOutput, SdkError<ListDocumentsError>>> {
        (*self).list_documents(builder)
    }
    fn list_inventory_entries(&self, builder: ListInventoryEntriesInputBuilder) -> impl Future<Output = Result<ListInventoryEntriesOutput, SdkError<ListInventoryEntriesError>>> {
        (*self).list_inventory_entries(builder)
    }
    fn list_ops_item_events(&self, builder: ListOpsItemEventsInputBuilder) -> impl Future<Output = Result<ListOpsItemEventsOutput, SdkError<ListOpsItemEventsError>>> {
        (*self).list_ops_item_events(builder)
    }
    fn list_ops_item_related_items(&self, builder: ListOpsItemRelatedItemsInputBuilder) -> impl Future<Output = Result<ListOpsItemRelatedItemsOutput, SdkError<ListOpsItemRelatedItemsError>>> {
        (*self).list_ops_item_related_items(builder)
    }
    fn list_ops_metadata(&self, builder: ListOpsMetadataInputBuilder) -> impl Future<Output = Result<ListOpsMetadataOutput, SdkError<ListOpsMetadataError>>> {
        (*self).list_ops_metadata(builder)
    }
    fn list_resource_compliance_summaries(&self, builder: ListResourceComplianceSummariesInputBuilder) -> impl Future<Output = Result<ListResourceComplianceSummariesOutput, SdkError<ListResourceComplianceSummariesError>>> {
        (*self).list_resource_compliance_summaries(builder)
    }
    fn list_resource_data_sync(&self, builder: ListResourceDataSyncInputBuilder) -> impl Future<Output = Result<ListResourceDataSyncOutput, SdkError<ListResourceDataSyncError>>> {
        (*self).list_resource_data_sync(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn modify_document_permission(&self, builder: ModifyDocumentPermissionInputBuilder) -> impl Future<Output = Result<ModifyDocumentPermissionOutput, SdkError<ModifyDocumentPermissionError>>> {
        (*self).modify_document_permission(builder)
    }
    fn put_compliance_items(&self, builder: PutComplianceItemsInputBuilder) -> impl Future<Output = Result<PutComplianceItemsOutput, SdkError<PutComplianceItemsError>>> {
        (*self).put_compliance_items(builder)
    }
    fn put_inventory(&self, builder: PutInventoryInputBuilder) -> impl Future<Output = Result<PutInventoryOutput, SdkError<PutInventoryError>>> {
        (*self).put_inventory(builder)
    }
    fn put_parameter(&self, builder: PutParameterInputBuilder) -> impl Future<Output = Result<PutParameterOutput, SdkError<PutParameterError>>> {
        (*self).put_parameter(builder)
    }
    fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> impl Future<Output = Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>> {
        (*self).put_resource_policy(builder)
    }
    fn register_default_patch_baseline(&self, builder: RegisterDefaultPatchBaselineInputBuilder) -> impl Future<Output = Result<RegisterDefaultPatchBaselineOutput, SdkError<RegisterDefaultPatchBaselineError>>> {
        (*self).register_default_patch_baseline(builder)
    }
    fn register_patch_baseline_for_patch_group(&self, builder: RegisterPatchBaselineForPatchGroupInputBuilder) -> impl Future<Output = Result<RegisterPatchBaselineForPatchGroupOutput, SdkError<RegisterPatchBaselineForPatchGroupError>>> {
        (*self).register_patch_baseline_for_patch_group(builder)
    }
    fn register_target_with_maintenance_window(&self, builder: RegisterTargetWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTargetWithMaintenanceWindowOutput, SdkError<RegisterTargetWithMaintenanceWindowError>>> {
        (*self).register_target_with_maintenance_window(builder)
    }
    fn register_task_with_maintenance_window(&self, builder: RegisterTaskWithMaintenanceWindowInputBuilder) -> impl Future<Output = Result<RegisterTaskWithMaintenanceWindowOutput, SdkError<RegisterTaskWithMaintenanceWindowError>>> {
        (*self).register_task_with_maintenance_window(builder)
    }
    fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> impl Future<Output = Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>> {
        (*self).remove_tags_from_resource(builder)
    }
    fn reset_service_setting(&self, builder: ResetServiceSettingInputBuilder) -> impl Future<Output = Result<ResetServiceSettingOutput, SdkError<ResetServiceSettingError>>> {
        (*self).reset_service_setting(builder)
    }
    fn resume_session(&self, builder: ResumeSessionInputBuilder) -> impl Future<Output = Result<ResumeSessionOutput, SdkError<ResumeSessionError>>> {
        (*self).resume_session(builder)
    }
    fn send_automation_signal(&self, builder: SendAutomationSignalInputBuilder) -> impl Future<Output = Result<SendAutomationSignalOutput, SdkError<SendAutomationSignalError>>> {
        (*self).send_automation_signal(builder)
    }
    fn send_command(&self, builder: SendCommandInputBuilder) -> impl Future<Output = Result<SendCommandOutput, SdkError<SendCommandError>>> {
        (*self).send_command(builder)
    }
    fn start_associations_once(&self, builder: StartAssociationsOnceInputBuilder) -> impl Future<Output = Result<StartAssociationsOnceOutput, SdkError<StartAssociationsOnceError>>> {
        (*self).start_associations_once(builder)
    }
    fn start_automation_execution(&self, builder: StartAutomationExecutionInputBuilder) -> impl Future<Output = Result<StartAutomationExecutionOutput, SdkError<StartAutomationExecutionError>>> {
        (*self).start_automation_execution(builder)
    }
    fn start_change_request_execution(&self, builder: StartChangeRequestExecutionInputBuilder) -> impl Future<Output = Result<StartChangeRequestExecutionOutput, SdkError<StartChangeRequestExecutionError>>> {
        (*self).start_change_request_execution(builder)
    }
    fn start_session(&self, builder: StartSessionInputBuilder) -> impl Future<Output = Result<StartSessionOutput, SdkError<StartSessionError>>> {
        (*self).start_session(builder)
    }
    fn stop_automation_execution(&self, builder: StopAutomationExecutionInputBuilder) -> impl Future<Output = Result<StopAutomationExecutionOutput, SdkError<StopAutomationExecutionError>>> {
        (*self).stop_automation_execution(builder)
    }
    fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> impl Future<Output = Result<TerminateSessionOutput, SdkError<TerminateSessionError>>> {
        (*self).terminate_session(builder)
    }
    fn unlabel_parameter_version(&self, builder: UnlabelParameterVersionInputBuilder) -> impl Future<Output = Result<UnlabelParameterVersionOutput, SdkError<UnlabelParameterVersionError>>> {
        (*self).unlabel_parameter_version(builder)
    }
    fn update_association(&self, builder: UpdateAssociationInputBuilder) -> impl Future<Output = Result<UpdateAssociationOutput, SdkError<UpdateAssociationError>>> {
        (*self).update_association(builder)
    }
    fn update_association_status(&self, builder: UpdateAssociationStatusInputBuilder) -> impl Future<Output = Result<UpdateAssociationStatusOutput, SdkError<UpdateAssociationStatusError>>> {
        (*self).update_association_status(builder)
    }
    fn update_document(&self, builder: UpdateDocumentInputBuilder) -> impl Future<Output = Result<UpdateDocumentOutput, SdkError<UpdateDocumentError>>> {
        (*self).update_document(builder)
    }
    fn update_document_default_version(&self, builder: UpdateDocumentDefaultVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentDefaultVersionOutput, SdkError<UpdateDocumentDefaultVersionError>>> {
        (*self).update_document_default_version(builder)
    }
    fn update_document_metadata(&self, builder: UpdateDocumentMetadataInputBuilder) -> impl Future<Output = Result<UpdateDocumentMetadataOutput, SdkError<UpdateDocumentMetadataError>>> {
        (*self).update_document_metadata(builder)
    }
    fn update_maintenance_window(&self, builder: UpdateMaintenanceWindowInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowOutput, SdkError<UpdateMaintenanceWindowError>>> {
        (*self).update_maintenance_window(builder)
    }
    fn update_maintenance_window_target(&self, builder: UpdateMaintenanceWindowTargetInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTargetOutput, SdkError<UpdateMaintenanceWindowTargetError>>> {
        (*self).update_maintenance_window_target(builder)
    }
    fn update_maintenance_window_task(&self, builder: UpdateMaintenanceWindowTaskInputBuilder) -> impl Future<Output = Result<UpdateMaintenanceWindowTaskOutput, SdkError<UpdateMaintenanceWindowTaskError>>> {
        (*self).update_maintenance_window_task(builder)
    }
    fn update_managed_instance_role(&self, builder: UpdateManagedInstanceRoleInputBuilder) -> impl Future<Output = Result<UpdateManagedInstanceRoleOutput, SdkError<UpdateManagedInstanceRoleError>>> {
        (*self).update_managed_instance_role(builder)
    }
    fn update_ops_item(&self, builder: UpdateOpsItemInputBuilder) -> impl Future<Output = Result<UpdateOpsItemOutput, SdkError<UpdateOpsItemError>>> {
        (*self).update_ops_item(builder)
    }
    fn update_ops_metadata(&self, builder: UpdateOpsMetadataInputBuilder) -> impl Future<Output = Result<UpdateOpsMetadataOutput, SdkError<UpdateOpsMetadataError>>> {
        (*self).update_ops_metadata(builder)
    }
    fn update_patch_baseline(&self, builder: UpdatePatchBaselineInputBuilder) -> impl Future<Output = Result<UpdatePatchBaselineOutput, SdkError<UpdatePatchBaselineError>>> {
        (*self).update_patch_baseline(builder)
    }
    fn update_resource_data_sync(&self, builder: UpdateResourceDataSyncInputBuilder) -> impl Future<Output = Result<UpdateResourceDataSyncOutput, SdkError<UpdateResourceDataSyncError>>> {
        (*self).update_resource_data_sync(builder)
    }
    fn update_service_setting(&self, builder: UpdateServiceSettingInputBuilder) -> impl Future<Output = Result<UpdateServiceSettingOutput, SdkError<UpdateServiceSettingError>>> {
        (*self).update_service_setting(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edSSMClient {}
    impl SSMClient for edSSMClient {
        async fn add_tags_to_resource(&self, builder: AddTagsToResourceInputBuilder) -> Result<AddTagsToResourceOutput, SdkError<AddTagsToResourceError>>;
        async fn associate_ops_item_related_item(&self, builder: AssociateOpsItemRelatedItemInputBuilder) -> Result<AssociateOpsItemRelatedItemOutput, SdkError<AssociateOpsItemRelatedItemError>>;
        async fn cancel_command(&self, builder: CancelCommandInputBuilder) -> Result<CancelCommandOutput, SdkError<CancelCommandError>>;
        async fn cancel_maintenance_window_execution(&self, builder: CancelMaintenanceWindowExecutionInputBuilder) -> Result<CancelMaintenanceWindowExecutionOutput, SdkError<CancelMaintenanceWindowExecutionError>>;
        async fn create_activation(&self, builder: CreateActivationInputBuilder) -> Result<CreateActivationOutput, SdkError<CreateActivationError>>;
        async fn create_association(&self, builder: CreateAssociationInputBuilder) -> Result<CreateAssociationOutput, SdkError<CreateAssociationError>>;
        async fn create_association_batch(&self, builder: CreateAssociationBatchInputBuilder) -> Result<CreateAssociationBatchOutput, SdkError<CreateAssociationBatchError>>;
        async fn create_document(&self, builder: CreateDocumentInputBuilder) -> Result<CreateDocumentOutput, SdkError<CreateDocumentError>>;
        async fn create_maintenance_window(&self, builder: CreateMaintenanceWindowInputBuilder) -> Result<CreateMaintenanceWindowOutput, SdkError<CreateMaintenanceWindowError>>;
        async fn create_ops_item(&self, builder: CreateOpsItemInputBuilder) -> Result<CreateOpsItemOutput, SdkError<CreateOpsItemError>>;
        async fn create_ops_metadata(&self, builder: CreateOpsMetadataInputBuilder) -> Result<CreateOpsMetadataOutput, SdkError<CreateOpsMetadataError>>;
        async fn create_patch_baseline(&self, builder: CreatePatchBaselineInputBuilder) -> Result<CreatePatchBaselineOutput, SdkError<CreatePatchBaselineError>>;
        async fn create_resource_data_sync(&self, builder: CreateResourceDataSyncInputBuilder) -> Result<CreateResourceDataSyncOutput, SdkError<CreateResourceDataSyncError>>;
        async fn delete_activation(&self, builder: DeleteActivationInputBuilder) -> Result<DeleteActivationOutput, SdkError<DeleteActivationError>>;
        async fn delete_association(&self, builder: DeleteAssociationInputBuilder) -> Result<DeleteAssociationOutput, SdkError<DeleteAssociationError>>;
        async fn delete_document(&self, builder: DeleteDocumentInputBuilder) -> Result<DeleteDocumentOutput, SdkError<DeleteDocumentError>>;
        async fn delete_inventory(&self, builder: DeleteInventoryInputBuilder) -> Result<DeleteInventoryOutput, SdkError<DeleteInventoryError>>;
        async fn delete_maintenance_window(&self, builder: DeleteMaintenanceWindowInputBuilder) -> Result<DeleteMaintenanceWindowOutput, SdkError<DeleteMaintenanceWindowError>>;
        async fn delete_ops_item(&self, builder: DeleteOpsItemInputBuilder) -> Result<DeleteOpsItemOutput, SdkError<DeleteOpsItemError>>;
        async fn delete_ops_metadata(&self, builder: DeleteOpsMetadataInputBuilder) -> Result<DeleteOpsMetadataOutput, SdkError<DeleteOpsMetadataError>>;
        async fn delete_parameter(&self, builder: DeleteParameterInputBuilder) -> Result<DeleteParameterOutput, SdkError<DeleteParameterError>>;
        async fn delete_parameters(&self, builder: DeleteParametersInputBuilder) -> Result<DeleteParametersOutput, SdkError<DeleteParametersError>>;
        async fn delete_patch_baseline(&self, builder: DeletePatchBaselineInputBuilder) -> Result<DeletePatchBaselineOutput, SdkError<DeletePatchBaselineError>>;
        async fn delete_resource_data_sync(&self, builder: DeleteResourceDataSyncInputBuilder) -> Result<DeleteResourceDataSyncOutput, SdkError<DeleteResourceDataSyncError>>;
        async fn delete_resource_policy(&self, builder: DeleteResourcePolicyInputBuilder) -> Result<DeleteResourcePolicyOutput, SdkError<DeleteResourcePolicyError>>;
        async fn deregister_managed_instance(&self, builder: DeregisterManagedInstanceInputBuilder) -> Result<DeregisterManagedInstanceOutput, SdkError<DeregisterManagedInstanceError>>;
        async fn deregister_patch_baseline_for_patch_group(&self, builder: DeregisterPatchBaselineForPatchGroupInputBuilder) -> Result<DeregisterPatchBaselineForPatchGroupOutput, SdkError<DeregisterPatchBaselineForPatchGroupError>>;
        async fn deregister_target_from_maintenance_window(&self, builder: DeregisterTargetFromMaintenanceWindowInputBuilder) -> Result<DeregisterTargetFromMaintenanceWindowOutput, SdkError<DeregisterTargetFromMaintenanceWindowError>>;
        async fn deregister_task_from_maintenance_window(&self, builder: DeregisterTaskFromMaintenanceWindowInputBuilder) -> Result<DeregisterTaskFromMaintenanceWindowOutput, SdkError<DeregisterTaskFromMaintenanceWindowError>>;
        async fn describe_activations(&self, builder: DescribeActivationsInputBuilder) -> Result<DescribeActivationsOutput, SdkError<DescribeActivationsError>>;
        async fn describe_association(&self, builder: DescribeAssociationInputBuilder) -> Result<DescribeAssociationOutput, SdkError<DescribeAssociationError>>;
        async fn describe_association_execution_targets(&self, builder: DescribeAssociationExecutionTargetsInputBuilder) -> Result<DescribeAssociationExecutionTargetsOutput, SdkError<DescribeAssociationExecutionTargetsError>>;
        async fn describe_association_executions(&self, builder: DescribeAssociationExecutionsInputBuilder) -> Result<DescribeAssociationExecutionsOutput, SdkError<DescribeAssociationExecutionsError>>;
        async fn describe_automation_executions(&self, builder: DescribeAutomationExecutionsInputBuilder) -> Result<DescribeAutomationExecutionsOutput, SdkError<DescribeAutomationExecutionsError>>;
        async fn describe_automation_step_executions(&self, builder: DescribeAutomationStepExecutionsInputBuilder) -> Result<DescribeAutomationStepExecutionsOutput, SdkError<DescribeAutomationStepExecutionsError>>;
        async fn describe_available_patches(&self, builder: DescribeAvailablePatchesInputBuilder) -> Result<DescribeAvailablePatchesOutput, SdkError<DescribeAvailablePatchesError>>;
        async fn describe_document(&self, builder: DescribeDocumentInputBuilder) -> Result<DescribeDocumentOutput, SdkError<DescribeDocumentError>>;
        async fn describe_document_permission(&self, builder: DescribeDocumentPermissionInputBuilder) -> Result<DescribeDocumentPermissionOutput, SdkError<DescribeDocumentPermissionError>>;
        async fn describe_effective_instance_associations(&self, builder: DescribeEffectiveInstanceAssociationsInputBuilder) -> Result<DescribeEffectiveInstanceAssociationsOutput, SdkError<DescribeEffectiveInstanceAssociationsError>>;
        async fn describe_effective_patches_for_patch_baseline(&self, builder: DescribeEffectivePatchesForPatchBaselineInputBuilder) -> Result<DescribeEffectivePatchesForPatchBaselineOutput, SdkError<DescribeEffectivePatchesForPatchBaselineError>>;
        async fn describe_instance_associations_status(&self, builder: DescribeInstanceAssociationsStatusInputBuilder) -> Result<DescribeInstanceAssociationsStatusOutput, SdkError<DescribeInstanceAssociationsStatusError>>;
        async fn describe_instance_information(&self, builder: DescribeInstanceInformationInputBuilder) -> Result<DescribeInstanceInformationOutput, SdkError<DescribeInstanceInformationError>>;
        async fn describe_instance_patch_states(&self, builder: DescribeInstancePatchStatesInputBuilder) -> Result<DescribeInstancePatchStatesOutput, SdkError<DescribeInstancePatchStatesError>>;
        async fn describe_instance_patch_states_for_patch_group(&self, builder: DescribeInstancePatchStatesForPatchGroupInputBuilder) -> Result<DescribeInstancePatchStatesForPatchGroupOutput, SdkError<DescribeInstancePatchStatesForPatchGroupError>>;
        async fn describe_instance_patches(&self, builder: DescribeInstancePatchesInputBuilder) -> Result<DescribeInstancePatchesOutput, SdkError<DescribeInstancePatchesError>>;
        async fn describe_instance_properties(&self, builder: DescribeInstancePropertiesInputBuilder) -> Result<DescribeInstancePropertiesOutput, SdkError<DescribeInstancePropertiesError>>;
        async fn describe_inventory_deletions(&self, builder: DescribeInventoryDeletionsInputBuilder) -> Result<DescribeInventoryDeletionsOutput, SdkError<DescribeInventoryDeletionsError>>;
        async fn describe_maintenance_window_execution_task_invocations(&self, builder: DescribeMaintenanceWindowExecutionTaskInvocationsInputBuilder) -> Result<DescribeMaintenanceWindowExecutionTaskInvocationsOutput, SdkError<DescribeMaintenanceWindowExecutionTaskInvocationsError>>;
        async fn describe_maintenance_window_execution_tasks(&self, builder: DescribeMaintenanceWindowExecutionTasksInputBuilder) -> Result<DescribeMaintenanceWindowExecutionTasksOutput, SdkError<DescribeMaintenanceWindowExecutionTasksError>>;
        async fn describe_maintenance_window_executions(&self, builder: DescribeMaintenanceWindowExecutionsInputBuilder) -> Result<DescribeMaintenanceWindowExecutionsOutput, SdkError<DescribeMaintenanceWindowExecutionsError>>;
        async fn describe_maintenance_window_schedule(&self, builder: DescribeMaintenanceWindowScheduleInputBuilder) -> Result<DescribeMaintenanceWindowScheduleOutput, SdkError<DescribeMaintenanceWindowScheduleError>>;
        async fn describe_maintenance_window_targets(&self, builder: DescribeMaintenanceWindowTargetsInputBuilder) -> Result<DescribeMaintenanceWindowTargetsOutput, SdkError<DescribeMaintenanceWindowTargetsError>>;
        async fn describe_maintenance_window_tasks(&self, builder: DescribeMaintenanceWindowTasksInputBuilder) -> Result<DescribeMaintenanceWindowTasksOutput, SdkError<DescribeMaintenanceWindowTasksError>>;
        async fn describe_maintenance_windows(&self, builder: DescribeMaintenanceWindowsInputBuilder) -> Result<DescribeMaintenanceWindowsOutput, SdkError<DescribeMaintenanceWindowsError>>;
        async fn describe_maintenance_windows_for_target(&self, builder: DescribeMaintenanceWindowsForTargetInputBuilder) -> Result<DescribeMaintenanceWindowsForTargetOutput, SdkError<DescribeMaintenanceWindowsForTargetError>>;
        async fn describe_ops_items(&self, builder: DescribeOpsItemsInputBuilder) -> Result<DescribeOpsItemsOutput, SdkError<DescribeOpsItemsError>>;
        async fn describe_parameters(&self, builder: DescribeParametersInputBuilder) -> Result<DescribeParametersOutput, SdkError<DescribeParametersError>>;
        async fn describe_patch_baselines(&self, builder: DescribePatchBaselinesInputBuilder) -> Result<DescribePatchBaselinesOutput, SdkError<DescribePatchBaselinesError>>;
        async fn describe_patch_group_state(&self, builder: DescribePatchGroupStateInputBuilder) -> Result<DescribePatchGroupStateOutput, SdkError<DescribePatchGroupStateError>>;
        async fn describe_patch_groups(&self, builder: DescribePatchGroupsInputBuilder) -> Result<DescribePatchGroupsOutput, SdkError<DescribePatchGroupsError>>;
        async fn describe_patch_properties(&self, builder: DescribePatchPropertiesInputBuilder) -> Result<DescribePatchPropertiesOutput, SdkError<DescribePatchPropertiesError>>;
        async fn describe_sessions(&self, builder: DescribeSessionsInputBuilder) -> Result<DescribeSessionsOutput, SdkError<DescribeSessionsError>>;
        async fn disassociate_ops_item_related_item(&self, builder: DisassociateOpsItemRelatedItemInputBuilder) -> Result<DisassociateOpsItemRelatedItemOutput, SdkError<DisassociateOpsItemRelatedItemError>>;
        async fn get_automation_execution(&self, builder: GetAutomationExecutionInputBuilder) -> Result<GetAutomationExecutionOutput, SdkError<GetAutomationExecutionError>>;
        async fn get_calendar_state(&self, builder: GetCalendarStateInputBuilder) -> Result<GetCalendarStateOutput, SdkError<GetCalendarStateError>>;
        async fn get_command_invocation(&self, builder: GetCommandInvocationInputBuilder) -> Result<GetCommandInvocationOutput, SdkError<GetCommandInvocationError>>;
        async fn get_connection_status(&self, builder: GetConnectionStatusInputBuilder) -> Result<GetConnectionStatusOutput, SdkError<GetConnectionStatusError>>;
        async fn get_default_patch_baseline(&self, builder: GetDefaultPatchBaselineInputBuilder) -> Result<GetDefaultPatchBaselineOutput, SdkError<GetDefaultPatchBaselineError>>;
        async fn get_deployable_patch_snapshot_for_instance(&self, builder: GetDeployablePatchSnapshotForInstanceInputBuilder) -> Result<GetDeployablePatchSnapshotForInstanceOutput, SdkError<GetDeployablePatchSnapshotForInstanceError>>;
        async fn get_document(&self, builder: GetDocumentInputBuilder) -> Result<GetDocumentOutput, SdkError<GetDocumentError>>;
        async fn get_inventory(&self, builder: GetInventoryInputBuilder) -> Result<GetInventoryOutput, SdkError<GetInventoryError>>;
        async fn get_inventory_schema(&self, builder: GetInventorySchemaInputBuilder) -> Result<GetInventorySchemaOutput, SdkError<GetInventorySchemaError>>;
        async fn get_maintenance_window(&self, builder: GetMaintenanceWindowInputBuilder) -> Result<GetMaintenanceWindowOutput, SdkError<GetMaintenanceWindowError>>;
        async fn get_maintenance_window_execution(&self, builder: GetMaintenanceWindowExecutionInputBuilder) -> Result<GetMaintenanceWindowExecutionOutput, SdkError<GetMaintenanceWindowExecutionError>>;
        async fn get_maintenance_window_execution_task(&self, builder: GetMaintenanceWindowExecutionTaskInputBuilder) -> Result<GetMaintenanceWindowExecutionTaskOutput, SdkError<GetMaintenanceWindowExecutionTaskError>>;
        async fn get_maintenance_window_execution_task_invocation(&self, builder: GetMaintenanceWindowExecutionTaskInvocationInputBuilder) -> Result<GetMaintenanceWindowExecutionTaskInvocationOutput, SdkError<GetMaintenanceWindowExecutionTaskInvocationError>>;
        async fn get_maintenance_window_task(&self, builder: GetMaintenanceWindowTaskInputBuilder) -> Result<GetMaintenanceWindowTaskOutput, SdkError<GetMaintenanceWindowTaskError>>;
        async fn get_ops_item(&self, builder: GetOpsItemInputBuilder) -> Result<GetOpsItemOutput, SdkError<GetOpsItemError>>;
        async fn get_ops_metadata(&self, builder: GetOpsMetadataInputBuilder) -> Result<GetOpsMetadataOutput, SdkError<GetOpsMetadataError>>;
        async fn get_ops_summary(&self, builder: GetOpsSummaryInputBuilder) -> Result<GetOpsSummaryOutput, SdkError<GetOpsSummaryError>>;
        async fn get_parameter(&self, builder: GetParameterInputBuilder) -> Result<GetParameterOutput, SdkError<GetParameterError>>;
        async fn get_parameter_history(&self, builder: GetParameterHistoryInputBuilder) -> Result<GetParameterHistoryOutput, SdkError<GetParameterHistoryError>>;
        async fn get_parameters(&self, builder: GetParametersInputBuilder) -> Result<GetParametersOutput, SdkError<GetParametersError>>;
        async fn get_parameters_by_path(&self, builder: GetParametersByPathInputBuilder) -> Result<GetParametersByPathOutput, SdkError<GetParametersByPathError>>;
        async fn get_patch_baseline(&self, builder: GetPatchBaselineInputBuilder) -> Result<GetPatchBaselineOutput, SdkError<GetPatchBaselineError>>;
        async fn get_patch_baseline_for_patch_group(&self, builder: GetPatchBaselineForPatchGroupInputBuilder) -> Result<GetPatchBaselineForPatchGroupOutput, SdkError<GetPatchBaselineForPatchGroupError>>;
        async fn get_resource_policies(&self, builder: GetResourcePoliciesInputBuilder) -> Result<GetResourcePoliciesOutput, SdkError<GetResourcePoliciesError>>;
        async fn get_service_setting(&self, builder: GetServiceSettingInputBuilder) -> Result<GetServiceSettingOutput, SdkError<GetServiceSettingError>>;
        async fn label_parameter_version(&self, builder: LabelParameterVersionInputBuilder) -> Result<LabelParameterVersionOutput, SdkError<LabelParameterVersionError>>;
        async fn list_association_versions(&self, builder: ListAssociationVersionsInputBuilder) -> Result<ListAssociationVersionsOutput, SdkError<ListAssociationVersionsError>>;
        async fn list_associations(&self, builder: ListAssociationsInputBuilder) -> Result<ListAssociationsOutput, SdkError<ListAssociationsError>>;
        async fn list_command_invocations(&self, builder: ListCommandInvocationsInputBuilder) -> Result<ListCommandInvocationsOutput, SdkError<ListCommandInvocationsError>>;
        async fn list_commands(&self, builder: ListCommandsInputBuilder) -> Result<ListCommandsOutput, SdkError<ListCommandsError>>;
        async fn list_compliance_items(&self, builder: ListComplianceItemsInputBuilder) -> Result<ListComplianceItemsOutput, SdkError<ListComplianceItemsError>>;
        async fn list_compliance_summaries(&self, builder: ListComplianceSummariesInputBuilder) -> Result<ListComplianceSummariesOutput, SdkError<ListComplianceSummariesError>>;
        async fn list_document_metadata_history(&self, builder: ListDocumentMetadataHistoryInputBuilder) -> Result<ListDocumentMetadataHistoryOutput, SdkError<ListDocumentMetadataHistoryError>>;
        async fn list_document_versions(&self, builder: ListDocumentVersionsInputBuilder) -> Result<ListDocumentVersionsOutput, SdkError<ListDocumentVersionsError>>;
        async fn list_documents(&self, builder: ListDocumentsInputBuilder) -> Result<ListDocumentsOutput, SdkError<ListDocumentsError>>;
        async fn list_inventory_entries(&self, builder: ListInventoryEntriesInputBuilder) -> Result<ListInventoryEntriesOutput, SdkError<ListInventoryEntriesError>>;
        async fn list_ops_item_events(&self, builder: ListOpsItemEventsInputBuilder) -> Result<ListOpsItemEventsOutput, SdkError<ListOpsItemEventsError>>;
        async fn list_ops_item_related_items(&self, builder: ListOpsItemRelatedItemsInputBuilder) -> Result<ListOpsItemRelatedItemsOutput, SdkError<ListOpsItemRelatedItemsError>>;
        async fn list_ops_metadata(&self, builder: ListOpsMetadataInputBuilder) -> Result<ListOpsMetadataOutput, SdkError<ListOpsMetadataError>>;
        async fn list_resource_compliance_summaries(&self, builder: ListResourceComplianceSummariesInputBuilder) -> Result<ListResourceComplianceSummariesOutput, SdkError<ListResourceComplianceSummariesError>>;
        async fn list_resource_data_sync(&self, builder: ListResourceDataSyncInputBuilder) -> Result<ListResourceDataSyncOutput, SdkError<ListResourceDataSyncError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn modify_document_permission(&self, builder: ModifyDocumentPermissionInputBuilder) -> Result<ModifyDocumentPermissionOutput, SdkError<ModifyDocumentPermissionError>>;
        async fn put_compliance_items(&self, builder: PutComplianceItemsInputBuilder) -> Result<PutComplianceItemsOutput, SdkError<PutComplianceItemsError>>;
        async fn put_inventory(&self, builder: PutInventoryInputBuilder) -> Result<PutInventoryOutput, SdkError<PutInventoryError>>;
        async fn put_parameter(&self, builder: PutParameterInputBuilder) -> Result<PutParameterOutput, SdkError<PutParameterError>>;
        async fn put_resource_policy(&self, builder: PutResourcePolicyInputBuilder) -> Result<PutResourcePolicyOutput, SdkError<PutResourcePolicyError>>;
        async fn register_default_patch_baseline(&self, builder: RegisterDefaultPatchBaselineInputBuilder) -> Result<RegisterDefaultPatchBaselineOutput, SdkError<RegisterDefaultPatchBaselineError>>;
        async fn register_patch_baseline_for_patch_group(&self, builder: RegisterPatchBaselineForPatchGroupInputBuilder) -> Result<RegisterPatchBaselineForPatchGroupOutput, SdkError<RegisterPatchBaselineForPatchGroupError>>;
        async fn register_target_with_maintenance_window(&self, builder: RegisterTargetWithMaintenanceWindowInputBuilder) -> Result<RegisterTargetWithMaintenanceWindowOutput, SdkError<RegisterTargetWithMaintenanceWindowError>>;
        async fn register_task_with_maintenance_window(&self, builder: RegisterTaskWithMaintenanceWindowInputBuilder) -> Result<RegisterTaskWithMaintenanceWindowOutput, SdkError<RegisterTaskWithMaintenanceWindowError>>;
        async fn remove_tags_from_resource(&self, builder: RemoveTagsFromResourceInputBuilder) -> Result<RemoveTagsFromResourceOutput, SdkError<RemoveTagsFromResourceError>>;
        async fn reset_service_setting(&self, builder: ResetServiceSettingInputBuilder) -> Result<ResetServiceSettingOutput, SdkError<ResetServiceSettingError>>;
        async fn resume_session(&self, builder: ResumeSessionInputBuilder) -> Result<ResumeSessionOutput, SdkError<ResumeSessionError>>;
        async fn send_automation_signal(&self, builder: SendAutomationSignalInputBuilder) -> Result<SendAutomationSignalOutput, SdkError<SendAutomationSignalError>>;
        async fn send_command(&self, builder: SendCommandInputBuilder) -> Result<SendCommandOutput, SdkError<SendCommandError>>;
        async fn start_associations_once(&self, builder: StartAssociationsOnceInputBuilder) -> Result<StartAssociationsOnceOutput, SdkError<StartAssociationsOnceError>>;
        async fn start_automation_execution(&self, builder: StartAutomationExecutionInputBuilder) -> Result<StartAutomationExecutionOutput, SdkError<StartAutomationExecutionError>>;
        async fn start_change_request_execution(&self, builder: StartChangeRequestExecutionInputBuilder) -> Result<StartChangeRequestExecutionOutput, SdkError<StartChangeRequestExecutionError>>;
        async fn start_session(&self, builder: StartSessionInputBuilder) -> Result<StartSessionOutput, SdkError<StartSessionError>>;
        async fn stop_automation_execution(&self, builder: StopAutomationExecutionInputBuilder) -> Result<StopAutomationExecutionOutput, SdkError<StopAutomationExecutionError>>;
        async fn terminate_session(&self, builder: TerminateSessionInputBuilder) -> Result<TerminateSessionOutput, SdkError<TerminateSessionError>>;
        async fn unlabel_parameter_version(&self, builder: UnlabelParameterVersionInputBuilder) -> Result<UnlabelParameterVersionOutput, SdkError<UnlabelParameterVersionError>>;
        async fn update_association(&self, builder: UpdateAssociationInputBuilder) -> Result<UpdateAssociationOutput, SdkError<UpdateAssociationError>>;
        async fn update_association_status(&self, builder: UpdateAssociationStatusInputBuilder) -> Result<UpdateAssociationStatusOutput, SdkError<UpdateAssociationStatusError>>;
        async fn update_document(&self, builder: UpdateDocumentInputBuilder) -> Result<UpdateDocumentOutput, SdkError<UpdateDocumentError>>;
        async fn update_document_default_version(&self, builder: UpdateDocumentDefaultVersionInputBuilder) -> Result<UpdateDocumentDefaultVersionOutput, SdkError<UpdateDocumentDefaultVersionError>>;
        async fn update_document_metadata(&self, builder: UpdateDocumentMetadataInputBuilder) -> Result<UpdateDocumentMetadataOutput, SdkError<UpdateDocumentMetadataError>>;
        async fn update_maintenance_window(&self, builder: UpdateMaintenanceWindowInputBuilder) -> Result<UpdateMaintenanceWindowOutput, SdkError<UpdateMaintenanceWindowError>>;
        async fn update_maintenance_window_target(&self, builder: UpdateMaintenanceWindowTargetInputBuilder) -> Result<UpdateMaintenanceWindowTargetOutput, SdkError<UpdateMaintenanceWindowTargetError>>;
        async fn update_maintenance_window_task(&self, builder: UpdateMaintenanceWindowTaskInputBuilder) -> Result<UpdateMaintenanceWindowTaskOutput, SdkError<UpdateMaintenanceWindowTaskError>>;
        async fn update_managed_instance_role(&self, builder: UpdateManagedInstanceRoleInputBuilder) -> Result<UpdateManagedInstanceRoleOutput, SdkError<UpdateManagedInstanceRoleError>>;
        async fn update_ops_item(&self, builder: UpdateOpsItemInputBuilder) -> Result<UpdateOpsItemOutput, SdkError<UpdateOpsItemError>>;
        async fn update_ops_metadata(&self, builder: UpdateOpsMetadataInputBuilder) -> Result<UpdateOpsMetadataOutput, SdkError<UpdateOpsMetadataError>>;
        async fn update_patch_baseline(&self, builder: UpdatePatchBaselineInputBuilder) -> Result<UpdatePatchBaselineOutput, SdkError<UpdatePatchBaselineError>>;
        async fn update_resource_data_sync(&self, builder: UpdateResourceDataSyncInputBuilder) -> Result<UpdateResourceDataSyncOutput, SdkError<UpdateResourceDataSyncError>>;
        async fn update_service_setting(&self, builder: UpdateServiceSettingInputBuilder) -> Result<UpdateServiceSettingOutput, SdkError<UpdateServiceSettingError>>;
    }
}
