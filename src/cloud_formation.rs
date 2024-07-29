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
use aws_sdk_cloudformation::operation::activate_organizations_access::{builders::*, *};
use aws_sdk_cloudformation::operation::activate_type::{builders::*, *};
use aws_sdk_cloudformation::operation::batch_describe_type_configurations::{builders::*, *};
use aws_sdk_cloudformation::operation::cancel_update_stack::{builders::*, *};
use aws_sdk_cloudformation::operation::continue_update_rollback::{builders::*, *};
use aws_sdk_cloudformation::operation::create_change_set::{builders::*, *};
use aws_sdk_cloudformation::operation::create_generated_template::{builders::*, *};
use aws_sdk_cloudformation::operation::create_stack::{builders::*, *};
use aws_sdk_cloudformation::operation::create_stack_instances::{builders::*, *};
use aws_sdk_cloudformation::operation::create_stack_set::{builders::*, *};
use aws_sdk_cloudformation::operation::deactivate_organizations_access::{builders::*, *};
use aws_sdk_cloudformation::operation::deactivate_type::{builders::*, *};
use aws_sdk_cloudformation::operation::delete_change_set::{builders::*, *};
use aws_sdk_cloudformation::operation::delete_generated_template::{builders::*, *};
use aws_sdk_cloudformation::operation::delete_stack::{builders::*, *};
use aws_sdk_cloudformation::operation::delete_stack_instances::{builders::*, *};
use aws_sdk_cloudformation::operation::delete_stack_set::{builders::*, *};
use aws_sdk_cloudformation::operation::deregister_type::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_account_limits::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_change_set::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_change_set_hooks::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_generated_template::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_organizations_access::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_publisher::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_resource_scan::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_drift_detection_status::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_events::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_instance::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_resource::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_resource_drifts::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_resources::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_set::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stack_set_operation::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_stacks::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_type::{builders::*, *};
use aws_sdk_cloudformation::operation::describe_type_registration::{builders::*, *};
use aws_sdk_cloudformation::operation::detect_stack_drift::{builders::*, *};
use aws_sdk_cloudformation::operation::detect_stack_resource_drift::{builders::*, *};
use aws_sdk_cloudformation::operation::detect_stack_set_drift::{builders::*, *};
use aws_sdk_cloudformation::operation::estimate_template_cost::{builders::*, *};
use aws_sdk_cloudformation::operation::execute_change_set::{builders::*, *};
use aws_sdk_cloudformation::operation::get_generated_template::{builders::*, *};
use aws_sdk_cloudformation::operation::get_stack_policy::{builders::*, *};
use aws_sdk_cloudformation::operation::get_template::{builders::*, *};
use aws_sdk_cloudformation::operation::get_template_summary::{builders::*, *};
use aws_sdk_cloudformation::operation::import_stacks_to_stack_set::{builders::*, *};
use aws_sdk_cloudformation::operation::list_change_sets::{builders::*, *};
use aws_sdk_cloudformation::operation::list_exports::{builders::*, *};
use aws_sdk_cloudformation::operation::list_generated_templates::{builders::*, *};
use aws_sdk_cloudformation::operation::list_imports::{builders::*, *};
use aws_sdk_cloudformation::operation::list_resource_scan_related_resources::{builders::*, *};
use aws_sdk_cloudformation::operation::list_resource_scan_resources::{builders::*, *};
use aws_sdk_cloudformation::operation::list_resource_scans::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_instance_resource_drifts::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_instances::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_resources::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_set_auto_deployment_targets::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_set_operation_results::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_set_operations::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stack_sets::{builders::*, *};
use aws_sdk_cloudformation::operation::list_stacks::{builders::*, *};
use aws_sdk_cloudformation::operation::list_type_registrations::{builders::*, *};
use aws_sdk_cloudformation::operation::list_type_versions::{builders::*, *};
use aws_sdk_cloudformation::operation::list_types::{builders::*, *};
use aws_sdk_cloudformation::operation::publish_type::{builders::*, *};
use aws_sdk_cloudformation::operation::record_handler_progress::{builders::*, *};
use aws_sdk_cloudformation::operation::register_publisher::{builders::*, *};
use aws_sdk_cloudformation::operation::register_type::{builders::*, *};
use aws_sdk_cloudformation::operation::rollback_stack::{builders::*, *};
use aws_sdk_cloudformation::operation::set_stack_policy::{builders::*, *};
use aws_sdk_cloudformation::operation::set_type_configuration::{builders::*, *};
use aws_sdk_cloudformation::operation::set_type_default_version::{builders::*, *};
use aws_sdk_cloudformation::operation::signal_resource::{builders::*, *};
use aws_sdk_cloudformation::operation::start_resource_scan::{builders::*, *};
use aws_sdk_cloudformation::operation::stop_stack_set_operation::{builders::*, *};
use aws_sdk_cloudformation::operation::test_type::{builders::*, *};
use aws_sdk_cloudformation::operation::update_generated_template::{builders::*, *};
use aws_sdk_cloudformation::operation::update_stack::{builders::*, *};
use aws_sdk_cloudformation::operation::update_stack_instances::{builders::*, *};
use aws_sdk_cloudformation::operation::update_stack_set::{builders::*, *};
use aws_sdk_cloudformation::operation::update_termination_protection::{builders::*, *};
use aws_sdk_cloudformation::operation::validate_template::{builders::*, *};
use aws_sdk_cloudformation::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_cloudformation::Client;
use std::ops::Deref;

pub use aws_sdk_cloudformation::*;

pub struct CloudFormationClientImpl(Client);
impl CloudFormationClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CloudFormationClient {
    fn activate_organizations_access(&self, builder: ActivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<ActivateOrganizationsAccessOutput, SdkError<ActivateOrganizationsAccessError>>>;
    fn activate_type(&self, builder: ActivateTypeInputBuilder) -> impl Future<Output = Result<ActivateTypeOutput, SdkError<ActivateTypeError>>>;
    fn batch_describe_type_configurations(&self, builder: BatchDescribeTypeConfigurationsInputBuilder) -> impl Future<Output = Result<BatchDescribeTypeConfigurationsOutput, SdkError<BatchDescribeTypeConfigurationsError>>>;
    fn cancel_update_stack(&self, builder: CancelUpdateStackInputBuilder) -> impl Future<Output = Result<CancelUpdateStackOutput, SdkError<CancelUpdateStackError>>>;
    fn continue_update_rollback(&self, builder: ContinueUpdateRollbackInputBuilder) -> impl Future<Output = Result<ContinueUpdateRollbackOutput, SdkError<ContinueUpdateRollbackError>>>;
    fn create_change_set(&self, builder: CreateChangeSetInputBuilder) -> impl Future<Output = Result<CreateChangeSetOutput, SdkError<CreateChangeSetError>>>;
    fn create_generated_template(&self, builder: CreateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<CreateGeneratedTemplateOutput, SdkError<CreateGeneratedTemplateError>>>;
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>>;
    fn create_stack_instances(&self, builder: CreateStackInstancesInputBuilder) -> impl Future<Output = Result<CreateStackInstancesOutput, SdkError<CreateStackInstancesError>>>;
    fn create_stack_set(&self, builder: CreateStackSetInputBuilder) -> impl Future<Output = Result<CreateStackSetOutput, SdkError<CreateStackSetError>>>;
    fn deactivate_organizations_access(&self, builder: DeactivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DeactivateOrganizationsAccessOutput, SdkError<DeactivateOrganizationsAccessError>>>;
    fn deactivate_type(&self, builder: DeactivateTypeInputBuilder) -> impl Future<Output = Result<DeactivateTypeOutput, SdkError<DeactivateTypeError>>>;
    fn delete_change_set(&self, builder: DeleteChangeSetInputBuilder) -> impl Future<Output = Result<DeleteChangeSetOutput, SdkError<DeleteChangeSetError>>>;
    fn delete_generated_template(&self, builder: DeleteGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DeleteGeneratedTemplateOutput, SdkError<DeleteGeneratedTemplateError>>>;
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>>;
    fn delete_stack_instances(&self, builder: DeleteStackInstancesInputBuilder) -> impl Future<Output = Result<DeleteStackInstancesOutput, SdkError<DeleteStackInstancesError>>>;
    fn delete_stack_set(&self, builder: DeleteStackSetInputBuilder) -> impl Future<Output = Result<DeleteStackSetOutput, SdkError<DeleteStackSetError>>>;
    fn deregister_type(&self, builder: DeregisterTypeInputBuilder) -> impl Future<Output = Result<DeregisterTypeOutput, SdkError<DeregisterTypeError>>>;
    fn describe_account_limits(&self, builder: DescribeAccountLimitsInputBuilder) -> impl Future<Output = Result<DescribeAccountLimitsOutput, SdkError<DescribeAccountLimitsError>>>;
    fn describe_change_set(&self, builder: DescribeChangeSetInputBuilder) -> impl Future<Output = Result<DescribeChangeSetOutput, SdkError<DescribeChangeSetError>>>;
    fn describe_change_set_hooks(&self, builder: DescribeChangeSetHooksInputBuilder) -> impl Future<Output = Result<DescribeChangeSetHooksOutput, SdkError<DescribeChangeSetHooksError>>>;
    fn describe_generated_template(&self, builder: DescribeGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DescribeGeneratedTemplateOutput, SdkError<DescribeGeneratedTemplateError>>>;
    fn describe_organizations_access(&self, builder: DescribeOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DescribeOrganizationsAccessOutput, SdkError<DescribeOrganizationsAccessError>>>;
    fn describe_publisher(&self, builder: DescribePublisherInputBuilder) -> impl Future<Output = Result<DescribePublisherOutput, SdkError<DescribePublisherError>>>;
    fn describe_resource_scan(&self, builder: DescribeResourceScanInputBuilder) -> impl Future<Output = Result<DescribeResourceScanOutput, SdkError<DescribeResourceScanError>>>;
    fn describe_stack_drift_detection_status(&self, builder: DescribeStackDriftDetectionStatusInputBuilder) -> impl Future<Output = Result<DescribeStackDriftDetectionStatusOutput, SdkError<DescribeStackDriftDetectionStatusError>>>;
    fn describe_stack_events(&self, builder: DescribeStackEventsInputBuilder) -> impl Future<Output = Result<DescribeStackEventsOutput, SdkError<DescribeStackEventsError>>>;
    fn describe_stack_instance(&self, builder: DescribeStackInstanceInputBuilder) -> impl Future<Output = Result<DescribeStackInstanceOutput, SdkError<DescribeStackInstanceError>>>;
    fn describe_stack_resource(&self, builder: DescribeStackResourceInputBuilder) -> impl Future<Output = Result<DescribeStackResourceOutput, SdkError<DescribeStackResourceError>>>;
    fn describe_stack_resource_drifts(&self, builder: DescribeStackResourceDriftsInputBuilder) -> impl Future<Output = Result<DescribeStackResourceDriftsOutput, SdkError<DescribeStackResourceDriftsError>>>;
    fn describe_stack_resources(&self, builder: DescribeStackResourcesInputBuilder) -> impl Future<Output = Result<DescribeStackResourcesOutput, SdkError<DescribeStackResourcesError>>>;
    fn describe_stack_set(&self, builder: DescribeStackSetInputBuilder) -> impl Future<Output = Result<DescribeStackSetOutput, SdkError<DescribeStackSetError>>>;
    fn describe_stack_set_operation(&self, builder: DescribeStackSetOperationInputBuilder) -> impl Future<Output = Result<DescribeStackSetOperationOutput, SdkError<DescribeStackSetOperationError>>>;
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>>;
    fn describe_type(&self, builder: DescribeTypeInputBuilder) -> impl Future<Output = Result<DescribeTypeOutput, SdkError<DescribeTypeError>>>;
    fn describe_type_registration(&self, builder: DescribeTypeRegistrationInputBuilder) -> impl Future<Output = Result<DescribeTypeRegistrationOutput, SdkError<DescribeTypeRegistrationError>>>;
    fn detect_stack_drift(&self, builder: DetectStackDriftInputBuilder) -> impl Future<Output = Result<DetectStackDriftOutput, SdkError<DetectStackDriftError>>>;
    fn detect_stack_resource_drift(&self, builder: DetectStackResourceDriftInputBuilder) -> impl Future<Output = Result<DetectStackResourceDriftOutput, SdkError<DetectStackResourceDriftError>>>;
    fn detect_stack_set_drift(&self, builder: DetectStackSetDriftInputBuilder) -> impl Future<Output = Result<DetectStackSetDriftOutput, SdkError<DetectStackSetDriftError>>>;
    fn estimate_template_cost(&self, builder: EstimateTemplateCostInputBuilder) -> impl Future<Output = Result<EstimateTemplateCostOutput, SdkError<EstimateTemplateCostError>>>;
    fn execute_change_set(&self, builder: ExecuteChangeSetInputBuilder) -> impl Future<Output = Result<ExecuteChangeSetOutput, SdkError<ExecuteChangeSetError>>>;
    fn get_generated_template(&self, builder: GetGeneratedTemplateInputBuilder) -> impl Future<Output = Result<GetGeneratedTemplateOutput, SdkError<GetGeneratedTemplateError>>>;
    fn get_stack_policy(&self, builder: GetStackPolicyInputBuilder) -> impl Future<Output = Result<GetStackPolicyOutput, SdkError<GetStackPolicyError>>>;
    fn get_template(&self, builder: GetTemplateInputBuilder) -> impl Future<Output = Result<GetTemplateOutput, SdkError<GetTemplateError>>>;
    fn get_template_summary(&self, builder: GetTemplateSummaryInputBuilder) -> impl Future<Output = Result<GetTemplateSummaryOutput, SdkError<GetTemplateSummaryError>>>;
    fn import_stacks_to_stack_set(&self, builder: ImportStacksToStackSetInputBuilder) -> impl Future<Output = Result<ImportStacksToStackSetOutput, SdkError<ImportStacksToStackSetError>>>;
    fn list_change_sets(&self, builder: ListChangeSetsInputBuilder) -> impl Future<Output = Result<ListChangeSetsOutput, SdkError<ListChangeSetsError>>>;
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>>;
    fn list_generated_templates(&self, builder: ListGeneratedTemplatesInputBuilder) -> impl Future<Output = Result<ListGeneratedTemplatesOutput, SdkError<ListGeneratedTemplatesError>>>;
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>>;
    fn list_resource_scan_related_resources(&self, builder: ListResourceScanRelatedResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanRelatedResourcesOutput, SdkError<ListResourceScanRelatedResourcesError>>>;
    fn list_resource_scan_resources(&self, builder: ListResourceScanResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanResourcesOutput, SdkError<ListResourceScanResourcesError>>>;
    fn list_resource_scans(&self, builder: ListResourceScansInputBuilder) -> impl Future<Output = Result<ListResourceScansOutput, SdkError<ListResourceScansError>>>;
    fn list_stack_instance_resource_drifts(&self, builder: ListStackInstanceResourceDriftsInputBuilder) -> impl Future<Output = Result<ListStackInstanceResourceDriftsOutput, SdkError<ListStackInstanceResourceDriftsError>>>;
    fn list_stack_instances(&self, builder: ListStackInstancesInputBuilder) -> impl Future<Output = Result<ListStackInstancesOutput, SdkError<ListStackInstancesError>>>;
    fn list_stack_resources(&self, builder: ListStackResourcesInputBuilder) -> impl Future<Output = Result<ListStackResourcesOutput, SdkError<ListStackResourcesError>>>;
    fn list_stack_set_auto_deployment_targets(&self, builder: ListStackSetAutoDeploymentTargetsInputBuilder) -> impl Future<Output = Result<ListStackSetAutoDeploymentTargetsOutput, SdkError<ListStackSetAutoDeploymentTargetsError>>>;
    fn list_stack_set_operation_results(&self, builder: ListStackSetOperationResultsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationResultsOutput, SdkError<ListStackSetOperationResultsError>>>;
    fn list_stack_set_operations(&self, builder: ListStackSetOperationsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationsOutput, SdkError<ListStackSetOperationsError>>>;
    fn list_stack_sets(&self, builder: ListStackSetsInputBuilder) -> impl Future<Output = Result<ListStackSetsOutput, SdkError<ListStackSetsError>>>;
    fn list_stacks(&self, builder: ListStacksInputBuilder) -> impl Future<Output = Result<ListStacksOutput, SdkError<ListStacksError>>>;
    fn list_type_registrations(&self, builder: ListTypeRegistrationsInputBuilder) -> impl Future<Output = Result<ListTypeRegistrationsOutput, SdkError<ListTypeRegistrationsError>>>;
    fn list_type_versions(&self, builder: ListTypeVersionsInputBuilder) -> impl Future<Output = Result<ListTypeVersionsOutput, SdkError<ListTypeVersionsError>>>;
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>>;
    fn publish_type(&self, builder: PublishTypeInputBuilder) -> impl Future<Output = Result<PublishTypeOutput, SdkError<PublishTypeError>>>;
    fn record_handler_progress(&self, builder: RecordHandlerProgressInputBuilder) -> impl Future<Output = Result<RecordHandlerProgressOutput, SdkError<RecordHandlerProgressError>>>;
    fn register_publisher(&self, builder: RegisterPublisherInputBuilder) -> impl Future<Output = Result<RegisterPublisherOutput, SdkError<RegisterPublisherError>>>;
    fn register_type(&self, builder: RegisterTypeInputBuilder) -> impl Future<Output = Result<RegisterTypeOutput, SdkError<RegisterTypeError>>>;
    fn rollback_stack(&self, builder: RollbackStackInputBuilder) -> impl Future<Output = Result<RollbackStackOutput, SdkError<RollbackStackError>>>;
    fn set_stack_policy(&self, builder: SetStackPolicyInputBuilder) -> impl Future<Output = Result<SetStackPolicyOutput, SdkError<SetStackPolicyError>>>;
    fn set_type_configuration(&self, builder: SetTypeConfigurationInputBuilder) -> impl Future<Output = Result<SetTypeConfigurationOutput, SdkError<SetTypeConfigurationError>>>;
    fn set_type_default_version(&self, builder: SetTypeDefaultVersionInputBuilder) -> impl Future<Output = Result<SetTypeDefaultVersionOutput, SdkError<SetTypeDefaultVersionError>>>;
    fn signal_resource(&self, builder: SignalResourceInputBuilder) -> impl Future<Output = Result<SignalResourceOutput, SdkError<SignalResourceError>>>;
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>>;
    fn stop_stack_set_operation(&self, builder: StopStackSetOperationInputBuilder) -> impl Future<Output = Result<StopStackSetOperationOutput, SdkError<StopStackSetOperationError>>>;
    fn test_type(&self, builder: TestTypeInputBuilder) -> impl Future<Output = Result<TestTypeOutput, SdkError<TestTypeError>>>;
    fn update_generated_template(&self, builder: UpdateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<UpdateGeneratedTemplateOutput, SdkError<UpdateGeneratedTemplateError>>>;
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>>;
    fn update_stack_instances(&self, builder: UpdateStackInstancesInputBuilder) -> impl Future<Output = Result<UpdateStackInstancesOutput, SdkError<UpdateStackInstancesError>>>;
    fn update_stack_set(&self, builder: UpdateStackSetInputBuilder) -> impl Future<Output = Result<UpdateStackSetOutput, SdkError<UpdateStackSetError>>>;
    fn update_termination_protection(&self, builder: UpdateTerminationProtectionInputBuilder) -> impl Future<Output = Result<UpdateTerminationProtectionOutput, SdkError<UpdateTerminationProtectionError>>>;
    fn validate_template(&self, builder: ValidateTemplateInputBuilder) -> impl Future<Output = Result<ValidateTemplateOutput, SdkError<ValidateTemplateError>>>;
}
impl CloudFormationClient for CloudFormationClientImpl {
    fn activate_organizations_access(&self, builder: ActivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<ActivateOrganizationsAccessOutput, SdkError<ActivateOrganizationsAccessError>>> {
        builder.send_with(&self.0)
    }
    fn activate_type(&self, builder: ActivateTypeInputBuilder) -> impl Future<Output = Result<ActivateTypeOutput, SdkError<ActivateTypeError>>> {
        builder.send_with(&self.0)
    }
    fn batch_describe_type_configurations(&self, builder: BatchDescribeTypeConfigurationsInputBuilder) -> impl Future<Output = Result<BatchDescribeTypeConfigurationsOutput, SdkError<BatchDescribeTypeConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn cancel_update_stack(&self, builder: CancelUpdateStackInputBuilder) -> impl Future<Output = Result<CancelUpdateStackOutput, SdkError<CancelUpdateStackError>>> {
        builder.send_with(&self.0)
    }
    fn continue_update_rollback(&self, builder: ContinueUpdateRollbackInputBuilder) -> impl Future<Output = Result<ContinueUpdateRollbackOutput, SdkError<ContinueUpdateRollbackError>>> {
        builder.send_with(&self.0)
    }
    fn create_change_set(&self, builder: CreateChangeSetInputBuilder) -> impl Future<Output = Result<CreateChangeSetOutput, SdkError<CreateChangeSetError>>> {
        builder.send_with(&self.0)
    }
    fn create_generated_template(&self, builder: CreateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<CreateGeneratedTemplateOutput, SdkError<CreateGeneratedTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>> {
        builder.send_with(&self.0)
    }
    fn create_stack_instances(&self, builder: CreateStackInstancesInputBuilder) -> impl Future<Output = Result<CreateStackInstancesOutput, SdkError<CreateStackInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn create_stack_set(&self, builder: CreateStackSetInputBuilder) -> impl Future<Output = Result<CreateStackSetOutput, SdkError<CreateStackSetError>>> {
        builder.send_with(&self.0)
    }
    fn deactivate_organizations_access(&self, builder: DeactivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DeactivateOrganizationsAccessOutput, SdkError<DeactivateOrganizationsAccessError>>> {
        builder.send_with(&self.0)
    }
    fn deactivate_type(&self, builder: DeactivateTypeInputBuilder) -> impl Future<Output = Result<DeactivateTypeOutput, SdkError<DeactivateTypeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_change_set(&self, builder: DeleteChangeSetInputBuilder) -> impl Future<Output = Result<DeleteChangeSetOutput, SdkError<DeleteChangeSetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_generated_template(&self, builder: DeleteGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DeleteGeneratedTemplateOutput, SdkError<DeleteGeneratedTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stack_instances(&self, builder: DeleteStackInstancesInputBuilder) -> impl Future<Output = Result<DeleteStackInstancesOutput, SdkError<DeleteStackInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stack_set(&self, builder: DeleteStackSetInputBuilder) -> impl Future<Output = Result<DeleteStackSetOutput, SdkError<DeleteStackSetError>>> {
        builder.send_with(&self.0)
    }
    fn deregister_type(&self, builder: DeregisterTypeInputBuilder) -> impl Future<Output = Result<DeregisterTypeOutput, SdkError<DeregisterTypeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_limits(&self, builder: DescribeAccountLimitsInputBuilder) -> impl Future<Output = Result<DescribeAccountLimitsOutput, SdkError<DescribeAccountLimitsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_change_set(&self, builder: DescribeChangeSetInputBuilder) -> impl Future<Output = Result<DescribeChangeSetOutput, SdkError<DescribeChangeSetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_change_set_hooks(&self, builder: DescribeChangeSetHooksInputBuilder) -> impl Future<Output = Result<DescribeChangeSetHooksOutput, SdkError<DescribeChangeSetHooksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_generated_template(&self, builder: DescribeGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DescribeGeneratedTemplateOutput, SdkError<DescribeGeneratedTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn describe_organizations_access(&self, builder: DescribeOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DescribeOrganizationsAccessOutput, SdkError<DescribeOrganizationsAccessError>>> {
        builder.send_with(&self.0)
    }
    fn describe_publisher(&self, builder: DescribePublisherInputBuilder) -> impl Future<Output = Result<DescribePublisherOutput, SdkError<DescribePublisherError>>> {
        builder.send_with(&self.0)
    }
    fn describe_resource_scan(&self, builder: DescribeResourceScanInputBuilder) -> impl Future<Output = Result<DescribeResourceScanOutput, SdkError<DescribeResourceScanError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_drift_detection_status(&self, builder: DescribeStackDriftDetectionStatusInputBuilder) -> impl Future<Output = Result<DescribeStackDriftDetectionStatusOutput, SdkError<DescribeStackDriftDetectionStatusError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_events(&self, builder: DescribeStackEventsInputBuilder) -> impl Future<Output = Result<DescribeStackEventsOutput, SdkError<DescribeStackEventsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_instance(&self, builder: DescribeStackInstanceInputBuilder) -> impl Future<Output = Result<DescribeStackInstanceOutput, SdkError<DescribeStackInstanceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_resource(&self, builder: DescribeStackResourceInputBuilder) -> impl Future<Output = Result<DescribeStackResourceOutput, SdkError<DescribeStackResourceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_resource_drifts(&self, builder: DescribeStackResourceDriftsInputBuilder) -> impl Future<Output = Result<DescribeStackResourceDriftsOutput, SdkError<DescribeStackResourceDriftsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_resources(&self, builder: DescribeStackResourcesInputBuilder) -> impl Future<Output = Result<DescribeStackResourcesOutput, SdkError<DescribeStackResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_set(&self, builder: DescribeStackSetInputBuilder) -> impl Future<Output = Result<DescribeStackSetOutput, SdkError<DescribeStackSetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stack_set_operation(&self, builder: DescribeStackSetOperationInputBuilder) -> impl Future<Output = Result<DescribeStackSetOperationOutput, SdkError<DescribeStackSetOperationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>> {
        builder.send_with(&self.0)
    }
    fn describe_type(&self, builder: DescribeTypeInputBuilder) -> impl Future<Output = Result<DescribeTypeOutput, SdkError<DescribeTypeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_type_registration(&self, builder: DescribeTypeRegistrationInputBuilder) -> impl Future<Output = Result<DescribeTypeRegistrationOutput, SdkError<DescribeTypeRegistrationError>>> {
        builder.send_with(&self.0)
    }
    fn detect_stack_drift(&self, builder: DetectStackDriftInputBuilder) -> impl Future<Output = Result<DetectStackDriftOutput, SdkError<DetectStackDriftError>>> {
        builder.send_with(&self.0)
    }
    fn detect_stack_resource_drift(&self, builder: DetectStackResourceDriftInputBuilder) -> impl Future<Output = Result<DetectStackResourceDriftOutput, SdkError<DetectStackResourceDriftError>>> {
        builder.send_with(&self.0)
    }
    fn detect_stack_set_drift(&self, builder: DetectStackSetDriftInputBuilder) -> impl Future<Output = Result<DetectStackSetDriftOutput, SdkError<DetectStackSetDriftError>>> {
        builder.send_with(&self.0)
    }
    fn estimate_template_cost(&self, builder: EstimateTemplateCostInputBuilder) -> impl Future<Output = Result<EstimateTemplateCostOutput, SdkError<EstimateTemplateCostError>>> {
        builder.send_with(&self.0)
    }
    fn execute_change_set(&self, builder: ExecuteChangeSetInputBuilder) -> impl Future<Output = Result<ExecuteChangeSetOutput, SdkError<ExecuteChangeSetError>>> {
        builder.send_with(&self.0)
    }
    fn get_generated_template(&self, builder: GetGeneratedTemplateInputBuilder) -> impl Future<Output = Result<GetGeneratedTemplateOutput, SdkError<GetGeneratedTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_stack_policy(&self, builder: GetStackPolicyInputBuilder) -> impl Future<Output = Result<GetStackPolicyOutput, SdkError<GetStackPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_template(&self, builder: GetTemplateInputBuilder) -> impl Future<Output = Result<GetTemplateOutput, SdkError<GetTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_template_summary(&self, builder: GetTemplateSummaryInputBuilder) -> impl Future<Output = Result<GetTemplateSummaryOutput, SdkError<GetTemplateSummaryError>>> {
        builder.send_with(&self.0)
    }
    fn import_stacks_to_stack_set(&self, builder: ImportStacksToStackSetInputBuilder) -> impl Future<Output = Result<ImportStacksToStackSetOutput, SdkError<ImportStacksToStackSetError>>> {
        builder.send_with(&self.0)
    }
    fn list_change_sets(&self, builder: ListChangeSetsInputBuilder) -> impl Future<Output = Result<ListChangeSetsOutput, SdkError<ListChangeSetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_generated_templates(&self, builder: ListGeneratedTemplatesInputBuilder) -> impl Future<Output = Result<ListGeneratedTemplatesOutput, SdkError<ListGeneratedTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_scan_related_resources(&self, builder: ListResourceScanRelatedResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanRelatedResourcesOutput, SdkError<ListResourceScanRelatedResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_scan_resources(&self, builder: ListResourceScanResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanResourcesOutput, SdkError<ListResourceScanResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_scans(&self, builder: ListResourceScansInputBuilder) -> impl Future<Output = Result<ListResourceScansOutput, SdkError<ListResourceScansError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_instance_resource_drifts(&self, builder: ListStackInstanceResourceDriftsInputBuilder) -> impl Future<Output = Result<ListStackInstanceResourceDriftsOutput, SdkError<ListStackInstanceResourceDriftsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_instances(&self, builder: ListStackInstancesInputBuilder) -> impl Future<Output = Result<ListStackInstancesOutput, SdkError<ListStackInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_resources(&self, builder: ListStackResourcesInputBuilder) -> impl Future<Output = Result<ListStackResourcesOutput, SdkError<ListStackResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_set_auto_deployment_targets(&self, builder: ListStackSetAutoDeploymentTargetsInputBuilder) -> impl Future<Output = Result<ListStackSetAutoDeploymentTargetsOutput, SdkError<ListStackSetAutoDeploymentTargetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_set_operation_results(&self, builder: ListStackSetOperationResultsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationResultsOutput, SdkError<ListStackSetOperationResultsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_set_operations(&self, builder: ListStackSetOperationsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationsOutput, SdkError<ListStackSetOperationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stack_sets(&self, builder: ListStackSetsInputBuilder) -> impl Future<Output = Result<ListStackSetsOutput, SdkError<ListStackSetsError>>> {
        builder.send_with(&self.0)
    }
    fn list_stacks(&self, builder: ListStacksInputBuilder) -> impl Future<Output = Result<ListStacksOutput, SdkError<ListStacksError>>> {
        builder.send_with(&self.0)
    }
    fn list_type_registrations(&self, builder: ListTypeRegistrationsInputBuilder) -> impl Future<Output = Result<ListTypeRegistrationsOutput, SdkError<ListTypeRegistrationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_type_versions(&self, builder: ListTypeVersionsInputBuilder) -> impl Future<Output = Result<ListTypeVersionsOutput, SdkError<ListTypeVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>> {
        builder.send_with(&self.0)
    }
    fn publish_type(&self, builder: PublishTypeInputBuilder) -> impl Future<Output = Result<PublishTypeOutput, SdkError<PublishTypeError>>> {
        builder.send_with(&self.0)
    }
    fn record_handler_progress(&self, builder: RecordHandlerProgressInputBuilder) -> impl Future<Output = Result<RecordHandlerProgressOutput, SdkError<RecordHandlerProgressError>>> {
        builder.send_with(&self.0)
    }
    fn register_publisher(&self, builder: RegisterPublisherInputBuilder) -> impl Future<Output = Result<RegisterPublisherOutput, SdkError<RegisterPublisherError>>> {
        builder.send_with(&self.0)
    }
    fn register_type(&self, builder: RegisterTypeInputBuilder) -> impl Future<Output = Result<RegisterTypeOutput, SdkError<RegisterTypeError>>> {
        builder.send_with(&self.0)
    }
    fn rollback_stack(&self, builder: RollbackStackInputBuilder) -> impl Future<Output = Result<RollbackStackOutput, SdkError<RollbackStackError>>> {
        builder.send_with(&self.0)
    }
    fn set_stack_policy(&self, builder: SetStackPolicyInputBuilder) -> impl Future<Output = Result<SetStackPolicyOutput, SdkError<SetStackPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn set_type_configuration(&self, builder: SetTypeConfigurationInputBuilder) -> impl Future<Output = Result<SetTypeConfigurationOutput, SdkError<SetTypeConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn set_type_default_version(&self, builder: SetTypeDefaultVersionInputBuilder) -> impl Future<Output = Result<SetTypeDefaultVersionOutput, SdkError<SetTypeDefaultVersionError>>> {
        builder.send_with(&self.0)
    }
    fn signal_resource(&self, builder: SignalResourceInputBuilder) -> impl Future<Output = Result<SignalResourceOutput, SdkError<SignalResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>> {
        builder.send_with(&self.0)
    }
    fn stop_stack_set_operation(&self, builder: StopStackSetOperationInputBuilder) -> impl Future<Output = Result<StopStackSetOperationOutput, SdkError<StopStackSetOperationError>>> {
        builder.send_with(&self.0)
    }
    fn test_type(&self, builder: TestTypeInputBuilder) -> impl Future<Output = Result<TestTypeOutput, SdkError<TestTypeError>>> {
        builder.send_with(&self.0)
    }
    fn update_generated_template(&self, builder: UpdateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<UpdateGeneratedTemplateOutput, SdkError<UpdateGeneratedTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>> {
        builder.send_with(&self.0)
    }
    fn update_stack_instances(&self, builder: UpdateStackInstancesInputBuilder) -> impl Future<Output = Result<UpdateStackInstancesOutput, SdkError<UpdateStackInstancesError>>> {
        builder.send_with(&self.0)
    }
    fn update_stack_set(&self, builder: UpdateStackSetInputBuilder) -> impl Future<Output = Result<UpdateStackSetOutput, SdkError<UpdateStackSetError>>> {
        builder.send_with(&self.0)
    }
    fn update_termination_protection(&self, builder: UpdateTerminationProtectionInputBuilder) -> impl Future<Output = Result<UpdateTerminationProtectionOutput, SdkError<UpdateTerminationProtectionError>>> {
        builder.send_with(&self.0)
    }
    fn validate_template(&self, builder: ValidateTemplateInputBuilder) -> impl Future<Output = Result<ValidateTemplateOutput, SdkError<ValidateTemplateError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> CloudFormationClient for T
where T: Deref,
      T::Target: CloudFormationClient {
    fn activate_organizations_access(&self, builder: ActivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<ActivateOrganizationsAccessOutput, SdkError<ActivateOrganizationsAccessError>>> {
        self.deref().activate_organizations_access(builder)
    }
    fn activate_type(&self, builder: ActivateTypeInputBuilder) -> impl Future<Output = Result<ActivateTypeOutput, SdkError<ActivateTypeError>>> {
        self.deref().activate_type(builder)
    }
    fn batch_describe_type_configurations(&self, builder: BatchDescribeTypeConfigurationsInputBuilder) -> impl Future<Output = Result<BatchDescribeTypeConfigurationsOutput, SdkError<BatchDescribeTypeConfigurationsError>>> {
        self.deref().batch_describe_type_configurations(builder)
    }
    fn cancel_update_stack(&self, builder: CancelUpdateStackInputBuilder) -> impl Future<Output = Result<CancelUpdateStackOutput, SdkError<CancelUpdateStackError>>> {
        self.deref().cancel_update_stack(builder)
    }
    fn continue_update_rollback(&self, builder: ContinueUpdateRollbackInputBuilder) -> impl Future<Output = Result<ContinueUpdateRollbackOutput, SdkError<ContinueUpdateRollbackError>>> {
        self.deref().continue_update_rollback(builder)
    }
    fn create_change_set(&self, builder: CreateChangeSetInputBuilder) -> impl Future<Output = Result<CreateChangeSetOutput, SdkError<CreateChangeSetError>>> {
        self.deref().create_change_set(builder)
    }
    fn create_generated_template(&self, builder: CreateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<CreateGeneratedTemplateOutput, SdkError<CreateGeneratedTemplateError>>> {
        self.deref().create_generated_template(builder)
    }
    fn create_stack(&self, builder: CreateStackInputBuilder) -> impl Future<Output = Result<CreateStackOutput, SdkError<CreateStackError>>> {
        self.deref().create_stack(builder)
    }
    fn create_stack_instances(&self, builder: CreateStackInstancesInputBuilder) -> impl Future<Output = Result<CreateStackInstancesOutput, SdkError<CreateStackInstancesError>>> {
        self.deref().create_stack_instances(builder)
    }
    fn create_stack_set(&self, builder: CreateStackSetInputBuilder) -> impl Future<Output = Result<CreateStackSetOutput, SdkError<CreateStackSetError>>> {
        self.deref().create_stack_set(builder)
    }
    fn deactivate_organizations_access(&self, builder: DeactivateOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DeactivateOrganizationsAccessOutput, SdkError<DeactivateOrganizationsAccessError>>> {
        self.deref().deactivate_organizations_access(builder)
    }
    fn deactivate_type(&self, builder: DeactivateTypeInputBuilder) -> impl Future<Output = Result<DeactivateTypeOutput, SdkError<DeactivateTypeError>>> {
        self.deref().deactivate_type(builder)
    }
    fn delete_change_set(&self, builder: DeleteChangeSetInputBuilder) -> impl Future<Output = Result<DeleteChangeSetOutput, SdkError<DeleteChangeSetError>>> {
        self.deref().delete_change_set(builder)
    }
    fn delete_generated_template(&self, builder: DeleteGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DeleteGeneratedTemplateOutput, SdkError<DeleteGeneratedTemplateError>>> {
        self.deref().delete_generated_template(builder)
    }
    fn delete_stack(&self, builder: DeleteStackInputBuilder) -> impl Future<Output = Result<DeleteStackOutput, SdkError<DeleteStackError>>> {
        self.deref().delete_stack(builder)
    }
    fn delete_stack_instances(&self, builder: DeleteStackInstancesInputBuilder) -> impl Future<Output = Result<DeleteStackInstancesOutput, SdkError<DeleteStackInstancesError>>> {
        self.deref().delete_stack_instances(builder)
    }
    fn delete_stack_set(&self, builder: DeleteStackSetInputBuilder) -> impl Future<Output = Result<DeleteStackSetOutput, SdkError<DeleteStackSetError>>> {
        self.deref().delete_stack_set(builder)
    }
    fn deregister_type(&self, builder: DeregisterTypeInputBuilder) -> impl Future<Output = Result<DeregisterTypeOutput, SdkError<DeregisterTypeError>>> {
        self.deref().deregister_type(builder)
    }
    fn describe_account_limits(&self, builder: DescribeAccountLimitsInputBuilder) -> impl Future<Output = Result<DescribeAccountLimitsOutput, SdkError<DescribeAccountLimitsError>>> {
        self.deref().describe_account_limits(builder)
    }
    fn describe_change_set(&self, builder: DescribeChangeSetInputBuilder) -> impl Future<Output = Result<DescribeChangeSetOutput, SdkError<DescribeChangeSetError>>> {
        self.deref().describe_change_set(builder)
    }
    fn describe_change_set_hooks(&self, builder: DescribeChangeSetHooksInputBuilder) -> impl Future<Output = Result<DescribeChangeSetHooksOutput, SdkError<DescribeChangeSetHooksError>>> {
        self.deref().describe_change_set_hooks(builder)
    }
    fn describe_generated_template(&self, builder: DescribeGeneratedTemplateInputBuilder) -> impl Future<Output = Result<DescribeGeneratedTemplateOutput, SdkError<DescribeGeneratedTemplateError>>> {
        self.deref().describe_generated_template(builder)
    }
    fn describe_organizations_access(&self, builder: DescribeOrganizationsAccessInputBuilder) -> impl Future<Output = Result<DescribeOrganizationsAccessOutput, SdkError<DescribeOrganizationsAccessError>>> {
        self.deref().describe_organizations_access(builder)
    }
    fn describe_publisher(&self, builder: DescribePublisherInputBuilder) -> impl Future<Output = Result<DescribePublisherOutput, SdkError<DescribePublisherError>>> {
        self.deref().describe_publisher(builder)
    }
    fn describe_resource_scan(&self, builder: DescribeResourceScanInputBuilder) -> impl Future<Output = Result<DescribeResourceScanOutput, SdkError<DescribeResourceScanError>>> {
        self.deref().describe_resource_scan(builder)
    }
    fn describe_stack_drift_detection_status(&self, builder: DescribeStackDriftDetectionStatusInputBuilder) -> impl Future<Output = Result<DescribeStackDriftDetectionStatusOutput, SdkError<DescribeStackDriftDetectionStatusError>>> {
        self.deref().describe_stack_drift_detection_status(builder)
    }
    fn describe_stack_events(&self, builder: DescribeStackEventsInputBuilder) -> impl Future<Output = Result<DescribeStackEventsOutput, SdkError<DescribeStackEventsError>>> {
        self.deref().describe_stack_events(builder)
    }
    fn describe_stack_instance(&self, builder: DescribeStackInstanceInputBuilder) -> impl Future<Output = Result<DescribeStackInstanceOutput, SdkError<DescribeStackInstanceError>>> {
        self.deref().describe_stack_instance(builder)
    }
    fn describe_stack_resource(&self, builder: DescribeStackResourceInputBuilder) -> impl Future<Output = Result<DescribeStackResourceOutput, SdkError<DescribeStackResourceError>>> {
        self.deref().describe_stack_resource(builder)
    }
    fn describe_stack_resource_drifts(&self, builder: DescribeStackResourceDriftsInputBuilder) -> impl Future<Output = Result<DescribeStackResourceDriftsOutput, SdkError<DescribeStackResourceDriftsError>>> {
        self.deref().describe_stack_resource_drifts(builder)
    }
    fn describe_stack_resources(&self, builder: DescribeStackResourcesInputBuilder) -> impl Future<Output = Result<DescribeStackResourcesOutput, SdkError<DescribeStackResourcesError>>> {
        self.deref().describe_stack_resources(builder)
    }
    fn describe_stack_set(&self, builder: DescribeStackSetInputBuilder) -> impl Future<Output = Result<DescribeStackSetOutput, SdkError<DescribeStackSetError>>> {
        self.deref().describe_stack_set(builder)
    }
    fn describe_stack_set_operation(&self, builder: DescribeStackSetOperationInputBuilder) -> impl Future<Output = Result<DescribeStackSetOperationOutput, SdkError<DescribeStackSetOperationError>>> {
        self.deref().describe_stack_set_operation(builder)
    }
    fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> impl Future<Output = Result<DescribeStacksOutput, SdkError<DescribeStacksError>>> {
        self.deref().describe_stacks(builder)
    }
    fn describe_type(&self, builder: DescribeTypeInputBuilder) -> impl Future<Output = Result<DescribeTypeOutput, SdkError<DescribeTypeError>>> {
        self.deref().describe_type(builder)
    }
    fn describe_type_registration(&self, builder: DescribeTypeRegistrationInputBuilder) -> impl Future<Output = Result<DescribeTypeRegistrationOutput, SdkError<DescribeTypeRegistrationError>>> {
        self.deref().describe_type_registration(builder)
    }
    fn detect_stack_drift(&self, builder: DetectStackDriftInputBuilder) -> impl Future<Output = Result<DetectStackDriftOutput, SdkError<DetectStackDriftError>>> {
        self.deref().detect_stack_drift(builder)
    }
    fn detect_stack_resource_drift(&self, builder: DetectStackResourceDriftInputBuilder) -> impl Future<Output = Result<DetectStackResourceDriftOutput, SdkError<DetectStackResourceDriftError>>> {
        self.deref().detect_stack_resource_drift(builder)
    }
    fn detect_stack_set_drift(&self, builder: DetectStackSetDriftInputBuilder) -> impl Future<Output = Result<DetectStackSetDriftOutput, SdkError<DetectStackSetDriftError>>> {
        self.deref().detect_stack_set_drift(builder)
    }
    fn estimate_template_cost(&self, builder: EstimateTemplateCostInputBuilder) -> impl Future<Output = Result<EstimateTemplateCostOutput, SdkError<EstimateTemplateCostError>>> {
        self.deref().estimate_template_cost(builder)
    }
    fn execute_change_set(&self, builder: ExecuteChangeSetInputBuilder) -> impl Future<Output = Result<ExecuteChangeSetOutput, SdkError<ExecuteChangeSetError>>> {
        self.deref().execute_change_set(builder)
    }
    fn get_generated_template(&self, builder: GetGeneratedTemplateInputBuilder) -> impl Future<Output = Result<GetGeneratedTemplateOutput, SdkError<GetGeneratedTemplateError>>> {
        self.deref().get_generated_template(builder)
    }
    fn get_stack_policy(&self, builder: GetStackPolicyInputBuilder) -> impl Future<Output = Result<GetStackPolicyOutput, SdkError<GetStackPolicyError>>> {
        self.deref().get_stack_policy(builder)
    }
    fn get_template(&self, builder: GetTemplateInputBuilder) -> impl Future<Output = Result<GetTemplateOutput, SdkError<GetTemplateError>>> {
        self.deref().get_template(builder)
    }
    fn get_template_summary(&self, builder: GetTemplateSummaryInputBuilder) -> impl Future<Output = Result<GetTemplateSummaryOutput, SdkError<GetTemplateSummaryError>>> {
        self.deref().get_template_summary(builder)
    }
    fn import_stacks_to_stack_set(&self, builder: ImportStacksToStackSetInputBuilder) -> impl Future<Output = Result<ImportStacksToStackSetOutput, SdkError<ImportStacksToStackSetError>>> {
        self.deref().import_stacks_to_stack_set(builder)
    }
    fn list_change_sets(&self, builder: ListChangeSetsInputBuilder) -> impl Future<Output = Result<ListChangeSetsOutput, SdkError<ListChangeSetsError>>> {
        self.deref().list_change_sets(builder)
    }
    fn list_exports(&self, builder: ListExportsInputBuilder) -> impl Future<Output = Result<ListExportsOutput, SdkError<ListExportsError>>> {
        self.deref().list_exports(builder)
    }
    fn list_generated_templates(&self, builder: ListGeneratedTemplatesInputBuilder) -> impl Future<Output = Result<ListGeneratedTemplatesOutput, SdkError<ListGeneratedTemplatesError>>> {
        self.deref().list_generated_templates(builder)
    }
    fn list_imports(&self, builder: ListImportsInputBuilder) -> impl Future<Output = Result<ListImportsOutput, SdkError<ListImportsError>>> {
        self.deref().list_imports(builder)
    }
    fn list_resource_scan_related_resources(&self, builder: ListResourceScanRelatedResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanRelatedResourcesOutput, SdkError<ListResourceScanRelatedResourcesError>>> {
        self.deref().list_resource_scan_related_resources(builder)
    }
    fn list_resource_scan_resources(&self, builder: ListResourceScanResourcesInputBuilder) -> impl Future<Output = Result<ListResourceScanResourcesOutput, SdkError<ListResourceScanResourcesError>>> {
        self.deref().list_resource_scan_resources(builder)
    }
    fn list_resource_scans(&self, builder: ListResourceScansInputBuilder) -> impl Future<Output = Result<ListResourceScansOutput, SdkError<ListResourceScansError>>> {
        self.deref().list_resource_scans(builder)
    }
    fn list_stack_instance_resource_drifts(&self, builder: ListStackInstanceResourceDriftsInputBuilder) -> impl Future<Output = Result<ListStackInstanceResourceDriftsOutput, SdkError<ListStackInstanceResourceDriftsError>>> {
        self.deref().list_stack_instance_resource_drifts(builder)
    }
    fn list_stack_instances(&self, builder: ListStackInstancesInputBuilder) -> impl Future<Output = Result<ListStackInstancesOutput, SdkError<ListStackInstancesError>>> {
        self.deref().list_stack_instances(builder)
    }
    fn list_stack_resources(&self, builder: ListStackResourcesInputBuilder) -> impl Future<Output = Result<ListStackResourcesOutput, SdkError<ListStackResourcesError>>> {
        self.deref().list_stack_resources(builder)
    }
    fn list_stack_set_auto_deployment_targets(&self, builder: ListStackSetAutoDeploymentTargetsInputBuilder) -> impl Future<Output = Result<ListStackSetAutoDeploymentTargetsOutput, SdkError<ListStackSetAutoDeploymentTargetsError>>> {
        self.deref().list_stack_set_auto_deployment_targets(builder)
    }
    fn list_stack_set_operation_results(&self, builder: ListStackSetOperationResultsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationResultsOutput, SdkError<ListStackSetOperationResultsError>>> {
        self.deref().list_stack_set_operation_results(builder)
    }
    fn list_stack_set_operations(&self, builder: ListStackSetOperationsInputBuilder) -> impl Future<Output = Result<ListStackSetOperationsOutput, SdkError<ListStackSetOperationsError>>> {
        self.deref().list_stack_set_operations(builder)
    }
    fn list_stack_sets(&self, builder: ListStackSetsInputBuilder) -> impl Future<Output = Result<ListStackSetsOutput, SdkError<ListStackSetsError>>> {
        self.deref().list_stack_sets(builder)
    }
    fn list_stacks(&self, builder: ListStacksInputBuilder) -> impl Future<Output = Result<ListStacksOutput, SdkError<ListStacksError>>> {
        self.deref().list_stacks(builder)
    }
    fn list_type_registrations(&self, builder: ListTypeRegistrationsInputBuilder) -> impl Future<Output = Result<ListTypeRegistrationsOutput, SdkError<ListTypeRegistrationsError>>> {
        self.deref().list_type_registrations(builder)
    }
    fn list_type_versions(&self, builder: ListTypeVersionsInputBuilder) -> impl Future<Output = Result<ListTypeVersionsOutput, SdkError<ListTypeVersionsError>>> {
        self.deref().list_type_versions(builder)
    }
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>> {
        self.deref().list_types(builder)
    }
    fn publish_type(&self, builder: PublishTypeInputBuilder) -> impl Future<Output = Result<PublishTypeOutput, SdkError<PublishTypeError>>> {
        self.deref().publish_type(builder)
    }
    fn record_handler_progress(&self, builder: RecordHandlerProgressInputBuilder) -> impl Future<Output = Result<RecordHandlerProgressOutput, SdkError<RecordHandlerProgressError>>> {
        self.deref().record_handler_progress(builder)
    }
    fn register_publisher(&self, builder: RegisterPublisherInputBuilder) -> impl Future<Output = Result<RegisterPublisherOutput, SdkError<RegisterPublisherError>>> {
        self.deref().register_publisher(builder)
    }
    fn register_type(&self, builder: RegisterTypeInputBuilder) -> impl Future<Output = Result<RegisterTypeOutput, SdkError<RegisterTypeError>>> {
        self.deref().register_type(builder)
    }
    fn rollback_stack(&self, builder: RollbackStackInputBuilder) -> impl Future<Output = Result<RollbackStackOutput, SdkError<RollbackStackError>>> {
        self.deref().rollback_stack(builder)
    }
    fn set_stack_policy(&self, builder: SetStackPolicyInputBuilder) -> impl Future<Output = Result<SetStackPolicyOutput, SdkError<SetStackPolicyError>>> {
        self.deref().set_stack_policy(builder)
    }
    fn set_type_configuration(&self, builder: SetTypeConfigurationInputBuilder) -> impl Future<Output = Result<SetTypeConfigurationOutput, SdkError<SetTypeConfigurationError>>> {
        self.deref().set_type_configuration(builder)
    }
    fn set_type_default_version(&self, builder: SetTypeDefaultVersionInputBuilder) -> impl Future<Output = Result<SetTypeDefaultVersionOutput, SdkError<SetTypeDefaultVersionError>>> {
        self.deref().set_type_default_version(builder)
    }
    fn signal_resource(&self, builder: SignalResourceInputBuilder) -> impl Future<Output = Result<SignalResourceOutput, SdkError<SignalResourceError>>> {
        self.deref().signal_resource(builder)
    }
    fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> impl Future<Output = Result<StartResourceScanOutput, SdkError<StartResourceScanError>>> {
        self.deref().start_resource_scan(builder)
    }
    fn stop_stack_set_operation(&self, builder: StopStackSetOperationInputBuilder) -> impl Future<Output = Result<StopStackSetOperationOutput, SdkError<StopStackSetOperationError>>> {
        self.deref().stop_stack_set_operation(builder)
    }
    fn test_type(&self, builder: TestTypeInputBuilder) -> impl Future<Output = Result<TestTypeOutput, SdkError<TestTypeError>>> {
        self.deref().test_type(builder)
    }
    fn update_generated_template(&self, builder: UpdateGeneratedTemplateInputBuilder) -> impl Future<Output = Result<UpdateGeneratedTemplateOutput, SdkError<UpdateGeneratedTemplateError>>> {
        self.deref().update_generated_template(builder)
    }
    fn update_stack(&self, builder: UpdateStackInputBuilder) -> impl Future<Output = Result<UpdateStackOutput, SdkError<UpdateStackError>>> {
        self.deref().update_stack(builder)
    }
    fn update_stack_instances(&self, builder: UpdateStackInstancesInputBuilder) -> impl Future<Output = Result<UpdateStackInstancesOutput, SdkError<UpdateStackInstancesError>>> {
        self.deref().update_stack_instances(builder)
    }
    fn update_stack_set(&self, builder: UpdateStackSetInputBuilder) -> impl Future<Output = Result<UpdateStackSetOutput, SdkError<UpdateStackSetError>>> {
        self.deref().update_stack_set(builder)
    }
    fn update_termination_protection(&self, builder: UpdateTerminationProtectionInputBuilder) -> impl Future<Output = Result<UpdateTerminationProtectionOutput, SdkError<UpdateTerminationProtectionError>>> {
        self.deref().update_termination_protection(builder)
    }
    fn validate_template(&self, builder: ValidateTemplateInputBuilder) -> impl Future<Output = Result<ValidateTemplateOutput, SdkError<ValidateTemplateError>>> {
        self.deref().validate_template(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCloudFormationClient {}
    impl CloudFormationClient for edCloudFormationClient {
        async fn activate_organizations_access(&self, builder: ActivateOrganizationsAccessInputBuilder) -> Result<ActivateOrganizationsAccessOutput, SdkError<ActivateOrganizationsAccessError>>;
        async fn activate_type(&self, builder: ActivateTypeInputBuilder) -> Result<ActivateTypeOutput, SdkError<ActivateTypeError>>;
        async fn batch_describe_type_configurations(&self, builder: BatchDescribeTypeConfigurationsInputBuilder) -> Result<BatchDescribeTypeConfigurationsOutput, SdkError<BatchDescribeTypeConfigurationsError>>;
        async fn cancel_update_stack(&self, builder: CancelUpdateStackInputBuilder) -> Result<CancelUpdateStackOutput, SdkError<CancelUpdateStackError>>;
        async fn continue_update_rollback(&self, builder: ContinueUpdateRollbackInputBuilder) -> Result<ContinueUpdateRollbackOutput, SdkError<ContinueUpdateRollbackError>>;
        async fn create_change_set(&self, builder: CreateChangeSetInputBuilder) -> Result<CreateChangeSetOutput, SdkError<CreateChangeSetError>>;
        async fn create_generated_template(&self, builder: CreateGeneratedTemplateInputBuilder) -> Result<CreateGeneratedTemplateOutput, SdkError<CreateGeneratedTemplateError>>;
        async fn create_stack(&self, builder: CreateStackInputBuilder) -> Result<CreateStackOutput, SdkError<CreateStackError>>;
        async fn create_stack_instances(&self, builder: CreateStackInstancesInputBuilder) -> Result<CreateStackInstancesOutput, SdkError<CreateStackInstancesError>>;
        async fn create_stack_set(&self, builder: CreateStackSetInputBuilder) -> Result<CreateStackSetOutput, SdkError<CreateStackSetError>>;
        async fn deactivate_organizations_access(&self, builder: DeactivateOrganizationsAccessInputBuilder) -> Result<DeactivateOrganizationsAccessOutput, SdkError<DeactivateOrganizationsAccessError>>;
        async fn deactivate_type(&self, builder: DeactivateTypeInputBuilder) -> Result<DeactivateTypeOutput, SdkError<DeactivateTypeError>>;
        async fn delete_change_set(&self, builder: DeleteChangeSetInputBuilder) -> Result<DeleteChangeSetOutput, SdkError<DeleteChangeSetError>>;
        async fn delete_generated_template(&self, builder: DeleteGeneratedTemplateInputBuilder) -> Result<DeleteGeneratedTemplateOutput, SdkError<DeleteGeneratedTemplateError>>;
        async fn delete_stack(&self, builder: DeleteStackInputBuilder) -> Result<DeleteStackOutput, SdkError<DeleteStackError>>;
        async fn delete_stack_instances(&self, builder: DeleteStackInstancesInputBuilder) -> Result<DeleteStackInstancesOutput, SdkError<DeleteStackInstancesError>>;
        async fn delete_stack_set(&self, builder: DeleteStackSetInputBuilder) -> Result<DeleteStackSetOutput, SdkError<DeleteStackSetError>>;
        async fn deregister_type(&self, builder: DeregisterTypeInputBuilder) -> Result<DeregisterTypeOutput, SdkError<DeregisterTypeError>>;
        async fn describe_account_limits(&self, builder: DescribeAccountLimitsInputBuilder) -> Result<DescribeAccountLimitsOutput, SdkError<DescribeAccountLimitsError>>;
        async fn describe_change_set(&self, builder: DescribeChangeSetInputBuilder) -> Result<DescribeChangeSetOutput, SdkError<DescribeChangeSetError>>;
        async fn describe_change_set_hooks(&self, builder: DescribeChangeSetHooksInputBuilder) -> Result<DescribeChangeSetHooksOutput, SdkError<DescribeChangeSetHooksError>>;
        async fn describe_generated_template(&self, builder: DescribeGeneratedTemplateInputBuilder) -> Result<DescribeGeneratedTemplateOutput, SdkError<DescribeGeneratedTemplateError>>;
        async fn describe_organizations_access(&self, builder: DescribeOrganizationsAccessInputBuilder) -> Result<DescribeOrganizationsAccessOutput, SdkError<DescribeOrganizationsAccessError>>;
        async fn describe_publisher(&self, builder: DescribePublisherInputBuilder) -> Result<DescribePublisherOutput, SdkError<DescribePublisherError>>;
        async fn describe_resource_scan(&self, builder: DescribeResourceScanInputBuilder) -> Result<DescribeResourceScanOutput, SdkError<DescribeResourceScanError>>;
        async fn describe_stack_drift_detection_status(&self, builder: DescribeStackDriftDetectionStatusInputBuilder) -> Result<DescribeStackDriftDetectionStatusOutput, SdkError<DescribeStackDriftDetectionStatusError>>;
        async fn describe_stack_events(&self, builder: DescribeStackEventsInputBuilder) -> Result<DescribeStackEventsOutput, SdkError<DescribeStackEventsError>>;
        async fn describe_stack_instance(&self, builder: DescribeStackInstanceInputBuilder) -> Result<DescribeStackInstanceOutput, SdkError<DescribeStackInstanceError>>;
        async fn describe_stack_resource(&self, builder: DescribeStackResourceInputBuilder) -> Result<DescribeStackResourceOutput, SdkError<DescribeStackResourceError>>;
        async fn describe_stack_resource_drifts(&self, builder: DescribeStackResourceDriftsInputBuilder) -> Result<DescribeStackResourceDriftsOutput, SdkError<DescribeStackResourceDriftsError>>;
        async fn describe_stack_resources(&self, builder: DescribeStackResourcesInputBuilder) -> Result<DescribeStackResourcesOutput, SdkError<DescribeStackResourcesError>>;
        async fn describe_stack_set(&self, builder: DescribeStackSetInputBuilder) -> Result<DescribeStackSetOutput, SdkError<DescribeStackSetError>>;
        async fn describe_stack_set_operation(&self, builder: DescribeStackSetOperationInputBuilder) -> Result<DescribeStackSetOperationOutput, SdkError<DescribeStackSetOperationError>>;
        async fn describe_stacks(&self, builder: DescribeStacksInputBuilder) -> Result<DescribeStacksOutput, SdkError<DescribeStacksError>>;
        async fn describe_type(&self, builder: DescribeTypeInputBuilder) -> Result<DescribeTypeOutput, SdkError<DescribeTypeError>>;
        async fn describe_type_registration(&self, builder: DescribeTypeRegistrationInputBuilder) -> Result<DescribeTypeRegistrationOutput, SdkError<DescribeTypeRegistrationError>>;
        async fn detect_stack_drift(&self, builder: DetectStackDriftInputBuilder) -> Result<DetectStackDriftOutput, SdkError<DetectStackDriftError>>;
        async fn detect_stack_resource_drift(&self, builder: DetectStackResourceDriftInputBuilder) -> Result<DetectStackResourceDriftOutput, SdkError<DetectStackResourceDriftError>>;
        async fn detect_stack_set_drift(&self, builder: DetectStackSetDriftInputBuilder) -> Result<DetectStackSetDriftOutput, SdkError<DetectStackSetDriftError>>;
        async fn estimate_template_cost(&self, builder: EstimateTemplateCostInputBuilder) -> Result<EstimateTemplateCostOutput, SdkError<EstimateTemplateCostError>>;
        async fn execute_change_set(&self, builder: ExecuteChangeSetInputBuilder) -> Result<ExecuteChangeSetOutput, SdkError<ExecuteChangeSetError>>;
        async fn get_generated_template(&self, builder: GetGeneratedTemplateInputBuilder) -> Result<GetGeneratedTemplateOutput, SdkError<GetGeneratedTemplateError>>;
        async fn get_stack_policy(&self, builder: GetStackPolicyInputBuilder) -> Result<GetStackPolicyOutput, SdkError<GetStackPolicyError>>;
        async fn get_template(&self, builder: GetTemplateInputBuilder) -> Result<GetTemplateOutput, SdkError<GetTemplateError>>;
        async fn get_template_summary(&self, builder: GetTemplateSummaryInputBuilder) -> Result<GetTemplateSummaryOutput, SdkError<GetTemplateSummaryError>>;
        async fn import_stacks_to_stack_set(&self, builder: ImportStacksToStackSetInputBuilder) -> Result<ImportStacksToStackSetOutput, SdkError<ImportStacksToStackSetError>>;
        async fn list_change_sets(&self, builder: ListChangeSetsInputBuilder) -> Result<ListChangeSetsOutput, SdkError<ListChangeSetsError>>;
        async fn list_exports(&self, builder: ListExportsInputBuilder) -> Result<ListExportsOutput, SdkError<ListExportsError>>;
        async fn list_generated_templates(&self, builder: ListGeneratedTemplatesInputBuilder) -> Result<ListGeneratedTemplatesOutput, SdkError<ListGeneratedTemplatesError>>;
        async fn list_imports(&self, builder: ListImportsInputBuilder) -> Result<ListImportsOutput, SdkError<ListImportsError>>;
        async fn list_resource_scan_related_resources(&self, builder: ListResourceScanRelatedResourcesInputBuilder) -> Result<ListResourceScanRelatedResourcesOutput, SdkError<ListResourceScanRelatedResourcesError>>;
        async fn list_resource_scan_resources(&self, builder: ListResourceScanResourcesInputBuilder) -> Result<ListResourceScanResourcesOutput, SdkError<ListResourceScanResourcesError>>;
        async fn list_resource_scans(&self, builder: ListResourceScansInputBuilder) -> Result<ListResourceScansOutput, SdkError<ListResourceScansError>>;
        async fn list_stack_instance_resource_drifts(&self, builder: ListStackInstanceResourceDriftsInputBuilder) -> Result<ListStackInstanceResourceDriftsOutput, SdkError<ListStackInstanceResourceDriftsError>>;
        async fn list_stack_instances(&self, builder: ListStackInstancesInputBuilder) -> Result<ListStackInstancesOutput, SdkError<ListStackInstancesError>>;
        async fn list_stack_resources(&self, builder: ListStackResourcesInputBuilder) -> Result<ListStackResourcesOutput, SdkError<ListStackResourcesError>>;
        async fn list_stack_set_auto_deployment_targets(&self, builder: ListStackSetAutoDeploymentTargetsInputBuilder) -> Result<ListStackSetAutoDeploymentTargetsOutput, SdkError<ListStackSetAutoDeploymentTargetsError>>;
        async fn list_stack_set_operation_results(&self, builder: ListStackSetOperationResultsInputBuilder) -> Result<ListStackSetOperationResultsOutput, SdkError<ListStackSetOperationResultsError>>;
        async fn list_stack_set_operations(&self, builder: ListStackSetOperationsInputBuilder) -> Result<ListStackSetOperationsOutput, SdkError<ListStackSetOperationsError>>;
        async fn list_stack_sets(&self, builder: ListStackSetsInputBuilder) -> Result<ListStackSetsOutput, SdkError<ListStackSetsError>>;
        async fn list_stacks(&self, builder: ListStacksInputBuilder) -> Result<ListStacksOutput, SdkError<ListStacksError>>;
        async fn list_type_registrations(&self, builder: ListTypeRegistrationsInputBuilder) -> Result<ListTypeRegistrationsOutput, SdkError<ListTypeRegistrationsError>>;
        async fn list_type_versions(&self, builder: ListTypeVersionsInputBuilder) -> Result<ListTypeVersionsOutput, SdkError<ListTypeVersionsError>>;
        async fn list_types(&self, builder: ListTypesInputBuilder) -> Result<ListTypesOutput, SdkError<ListTypesError>>;
        async fn publish_type(&self, builder: PublishTypeInputBuilder) -> Result<PublishTypeOutput, SdkError<PublishTypeError>>;
        async fn record_handler_progress(&self, builder: RecordHandlerProgressInputBuilder) -> Result<RecordHandlerProgressOutput, SdkError<RecordHandlerProgressError>>;
        async fn register_publisher(&self, builder: RegisterPublisherInputBuilder) -> Result<RegisterPublisherOutput, SdkError<RegisterPublisherError>>;
        async fn register_type(&self, builder: RegisterTypeInputBuilder) -> Result<RegisterTypeOutput, SdkError<RegisterTypeError>>;
        async fn rollback_stack(&self, builder: RollbackStackInputBuilder) -> Result<RollbackStackOutput, SdkError<RollbackStackError>>;
        async fn set_stack_policy(&self, builder: SetStackPolicyInputBuilder) -> Result<SetStackPolicyOutput, SdkError<SetStackPolicyError>>;
        async fn set_type_configuration(&self, builder: SetTypeConfigurationInputBuilder) -> Result<SetTypeConfigurationOutput, SdkError<SetTypeConfigurationError>>;
        async fn set_type_default_version(&self, builder: SetTypeDefaultVersionInputBuilder) -> Result<SetTypeDefaultVersionOutput, SdkError<SetTypeDefaultVersionError>>;
        async fn signal_resource(&self, builder: SignalResourceInputBuilder) -> Result<SignalResourceOutput, SdkError<SignalResourceError>>;
        async fn start_resource_scan(&self, builder: StartResourceScanInputBuilder) -> Result<StartResourceScanOutput, SdkError<StartResourceScanError>>;
        async fn stop_stack_set_operation(&self, builder: StopStackSetOperationInputBuilder) -> Result<StopStackSetOperationOutput, SdkError<StopStackSetOperationError>>;
        async fn test_type(&self, builder: TestTypeInputBuilder) -> Result<TestTypeOutput, SdkError<TestTypeError>>;
        async fn update_generated_template(&self, builder: UpdateGeneratedTemplateInputBuilder) -> Result<UpdateGeneratedTemplateOutput, SdkError<UpdateGeneratedTemplateError>>;
        async fn update_stack(&self, builder: UpdateStackInputBuilder) -> Result<UpdateStackOutput, SdkError<UpdateStackError>>;
        async fn update_stack_instances(&self, builder: UpdateStackInstancesInputBuilder) -> Result<UpdateStackInstancesOutput, SdkError<UpdateStackInstancesError>>;
        async fn update_stack_set(&self, builder: UpdateStackSetInputBuilder) -> Result<UpdateStackSetOutput, SdkError<UpdateStackSetError>>;
        async fn update_termination_protection(&self, builder: UpdateTerminationProtectionInputBuilder) -> Result<UpdateTerminationProtectionOutput, SdkError<UpdateTerminationProtectionError>>;
        async fn validate_template(&self, builder: ValidateTemplateInputBuilder) -> Result<ValidateTemplateOutput, SdkError<ValidateTemplateError>>;
    }
}
