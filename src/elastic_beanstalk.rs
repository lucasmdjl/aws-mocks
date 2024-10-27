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
use aws_sdk_elasticbeanstalk::operation::abort_environment_update::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::apply_environment_managed_action::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::associate_environment_operations_role::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::check_dns_availability::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::compose_environments::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_application::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_application_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_configuration_template::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_environment::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_platform_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::create_storage_location::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::delete_application::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::delete_application_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::delete_configuration_template::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::delete_environment_configuration::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::delete_platform_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_account_attributes::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_application_versions::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_applications::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_configuration_options::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_configuration_settings::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_environment_health::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_environment_managed_action_history::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_environment_managed_actions::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_environment_resources::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_environments::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_events::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_instances_health::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::describe_platform_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::disassociate_environment_operations_role::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::list_available_solution_stacks::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::list_platform_branches::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::list_platform_versions::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::rebuild_environment::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::request_environment_info::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::restart_app_server::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::retrieve_environment_info::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::swap_environment_cnames::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::terminate_environment::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_application::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_application_resource_lifecycle::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_application_version::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_configuration_template::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_environment::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::update_tags_for_resource::{builders::*, *};
use aws_sdk_elasticbeanstalk::operation::validate_configuration_settings::{builders::*, *};
use aws_sdk_elasticbeanstalk::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_elasticbeanstalk::Client;
use std::ops::Deref;

pub use aws_sdk_elasticbeanstalk::*;

pub struct ElasticBeanstalkClientImpl(Client);
impl ElasticBeanstalkClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ElasticBeanstalkClient {
    fn abort_environment_update(&self, builder: AbortEnvironmentUpdateInputBuilder) -> impl Future<Output = Result<AbortEnvironmentUpdateOutput, SdkError<AbortEnvironmentUpdateError>>> + Send;
    fn apply_environment_managed_action(&self, builder: ApplyEnvironmentManagedActionInputBuilder) -> impl Future<Output = Result<ApplyEnvironmentManagedActionOutput, SdkError<ApplyEnvironmentManagedActionError>>> + Send;
    fn associate_environment_operations_role(&self, builder: AssociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<AssociateEnvironmentOperationsRoleOutput, SdkError<AssociateEnvironmentOperationsRoleError>>> + Send;
    fn check_dns_availability(&self, builder: CheckDnsAvailabilityInputBuilder) -> impl Future<Output = Result<CheckDnsAvailabilityOutput, SdkError<CheckDNSAvailabilityError>>> + Send;
    fn compose_environments(&self, builder: ComposeEnvironmentsInputBuilder) -> impl Future<Output = Result<ComposeEnvironmentsOutput, SdkError<ComposeEnvironmentsError>>> + Send;
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> + Send;
    fn create_application_version(&self, builder: CreateApplicationVersionInputBuilder) -> impl Future<Output = Result<CreateApplicationVersionOutput, SdkError<CreateApplicationVersionError>>> + Send;
    fn create_configuration_template(&self, builder: CreateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<CreateConfigurationTemplateOutput, SdkError<CreateConfigurationTemplateError>>> + Send;
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> + Send;
    fn create_platform_version(&self, builder: CreatePlatformVersionInputBuilder) -> impl Future<Output = Result<CreatePlatformVersionOutput, SdkError<CreatePlatformVersionError>>> + Send;
    fn create_storage_location(&self, builder: CreateStorageLocationInputBuilder) -> impl Future<Output = Result<CreateStorageLocationOutput, SdkError<CreateStorageLocationError>>> + Send;
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> + Send;
    fn delete_application_version(&self, builder: DeleteApplicationVersionInputBuilder) -> impl Future<Output = Result<DeleteApplicationVersionOutput, SdkError<DeleteApplicationVersionError>>> + Send;
    fn delete_configuration_template(&self, builder: DeleteConfigurationTemplateInputBuilder) -> impl Future<Output = Result<DeleteConfigurationTemplateOutput, SdkError<DeleteConfigurationTemplateError>>> + Send;
    fn delete_environment_configuration(&self, builder: DeleteEnvironmentConfigurationInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentConfigurationOutput, SdkError<DeleteEnvironmentConfigurationError>>> + Send;
    fn delete_platform_version(&self, builder: DeletePlatformVersionInputBuilder) -> impl Future<Output = Result<DeletePlatformVersionOutput, SdkError<DeletePlatformVersionError>>> + Send;
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> + Send;
    fn describe_application_versions(&self, builder: DescribeApplicationVersionsInputBuilder) -> impl Future<Output = Result<DescribeApplicationVersionsOutput, SdkError<DescribeApplicationVersionsError>>> + Send;
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>> + Send;
    fn describe_configuration_options(&self, builder: DescribeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOptionsOutput, SdkError<DescribeConfigurationOptionsError>>> + Send;
    fn describe_configuration_settings(&self, builder: DescribeConfigurationSettingsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationSettingsOutput, SdkError<DescribeConfigurationSettingsError>>> + Send;
    fn describe_environment_health(&self, builder: DescribeEnvironmentHealthInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentHealthOutput, SdkError<DescribeEnvironmentHealthError>>> + Send;
    fn describe_environment_managed_action_history(&self, builder: DescribeEnvironmentManagedActionHistoryInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionHistoryOutput, SdkError<DescribeEnvironmentManagedActionHistoryError>>> + Send;
    fn describe_environment_managed_actions(&self, builder: DescribeEnvironmentManagedActionsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionsOutput, SdkError<DescribeEnvironmentManagedActionsError>>> + Send;
    fn describe_environment_resources(&self, builder: DescribeEnvironmentResourcesInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentResourcesOutput, SdkError<DescribeEnvironmentResourcesError>>> + Send;
    fn describe_environments(&self, builder: DescribeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentsOutput, SdkError<DescribeEnvironmentsError>>> + Send;
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> + Send;
    fn describe_instances_health(&self, builder: DescribeInstancesHealthInputBuilder) -> impl Future<Output = Result<DescribeInstancesHealthOutput, SdkError<DescribeInstancesHealthError>>> + Send;
    fn describe_platform_version(&self, builder: DescribePlatformVersionInputBuilder) -> impl Future<Output = Result<DescribePlatformVersionOutput, SdkError<DescribePlatformVersionError>>> + Send;
    fn disassociate_environment_operations_role(&self, builder: DisassociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnvironmentOperationsRoleOutput, SdkError<DisassociateEnvironmentOperationsRoleError>>> + Send;
    fn list_available_solution_stacks(&self, builder: ListAvailableSolutionStacksInputBuilder) -> impl Future<Output = Result<ListAvailableSolutionStacksOutput, SdkError<ListAvailableSolutionStacksError>>> + Send;
    fn list_platform_branches(&self, builder: ListPlatformBranchesInputBuilder) -> impl Future<Output = Result<ListPlatformBranchesOutput, SdkError<ListPlatformBranchesError>>> + Send;
    fn list_platform_versions(&self, builder: ListPlatformVersionsInputBuilder) -> impl Future<Output = Result<ListPlatformVersionsOutput, SdkError<ListPlatformVersionsError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn rebuild_environment(&self, builder: RebuildEnvironmentInputBuilder) -> impl Future<Output = Result<RebuildEnvironmentOutput, SdkError<RebuildEnvironmentError>>> + Send;
    fn request_environment_info(&self, builder: RequestEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RequestEnvironmentInfoOutput, SdkError<RequestEnvironmentInfoError>>> + Send;
    fn restart_app_server(&self, builder: RestartAppServerInputBuilder) -> impl Future<Output = Result<RestartAppServerOutput, SdkError<RestartAppServerError>>> + Send;
    fn retrieve_environment_info(&self, builder: RetrieveEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RetrieveEnvironmentInfoOutput, SdkError<RetrieveEnvironmentInfoError>>> + Send;
    fn swap_environment_cnames(&self, builder: SwapEnvironmentCnamEsInputBuilder) -> impl Future<Output = Result<SwapEnvironmentCnamEsOutput, SdkError<SwapEnvironmentCNAMEsError>>> + Send;
    fn terminate_environment(&self, builder: TerminateEnvironmentInputBuilder) -> impl Future<Output = Result<TerminateEnvironmentOutput, SdkError<TerminateEnvironmentError>>> + Send;
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> + Send;
    fn update_application_resource_lifecycle(&self, builder: UpdateApplicationResourceLifecycleInputBuilder) -> impl Future<Output = Result<UpdateApplicationResourceLifecycleOutput, SdkError<UpdateApplicationResourceLifecycleError>>> + Send;
    fn update_application_version(&self, builder: UpdateApplicationVersionInputBuilder) -> impl Future<Output = Result<UpdateApplicationVersionOutput, SdkError<UpdateApplicationVersionError>>> + Send;
    fn update_configuration_template(&self, builder: UpdateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<UpdateConfigurationTemplateOutput, SdkError<UpdateConfigurationTemplateError>>> + Send;
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> + Send;
    fn update_tags_for_resource(&self, builder: UpdateTagsForResourceInputBuilder) -> impl Future<Output = Result<UpdateTagsForResourceOutput, SdkError<UpdateTagsForResourceError>>> + Send;
    fn validate_configuration_settings(&self, builder: ValidateConfigurationSettingsInputBuilder) -> impl Future<Output = Result<ValidateConfigurationSettingsOutput, SdkError<ValidateConfigurationSettingsError>>> + Send;
}
impl ElasticBeanstalkClient for ElasticBeanstalkClientImpl {
    fn abort_environment_update(&self, builder: AbortEnvironmentUpdateInputBuilder) -> impl Future<Output = Result<AbortEnvironmentUpdateOutput, SdkError<AbortEnvironmentUpdateError>>> {
        builder.send_with(&self.0)
    }
    fn apply_environment_managed_action(&self, builder: ApplyEnvironmentManagedActionInputBuilder) -> impl Future<Output = Result<ApplyEnvironmentManagedActionOutput, SdkError<ApplyEnvironmentManagedActionError>>> {
        builder.send_with(&self.0)
    }
    fn associate_environment_operations_role(&self, builder: AssociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<AssociateEnvironmentOperationsRoleOutput, SdkError<AssociateEnvironmentOperationsRoleError>>> {
        builder.send_with(&self.0)
    }
    fn check_dns_availability(&self, builder: CheckDnsAvailabilityInputBuilder) -> impl Future<Output = Result<CheckDnsAvailabilityOutput, SdkError<CheckDNSAvailabilityError>>> {
        builder.send_with(&self.0)
    }
    fn compose_environments(&self, builder: ComposeEnvironmentsInputBuilder) -> impl Future<Output = Result<ComposeEnvironmentsOutput, SdkError<ComposeEnvironmentsError>>> {
        builder.send_with(&self.0)
    }
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_application_version(&self, builder: CreateApplicationVersionInputBuilder) -> impl Future<Output = Result<CreateApplicationVersionOutput, SdkError<CreateApplicationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_configuration_template(&self, builder: CreateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<CreateConfigurationTemplateOutput, SdkError<CreateConfigurationTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_platform_version(&self, builder: CreatePlatformVersionInputBuilder) -> impl Future<Output = Result<CreatePlatformVersionOutput, SdkError<CreatePlatformVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_storage_location(&self, builder: CreateStorageLocationInputBuilder) -> impl Future<Output = Result<CreateStorageLocationOutput, SdkError<CreateStorageLocationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_application_version(&self, builder: DeleteApplicationVersionInputBuilder) -> impl Future<Output = Result<DeleteApplicationVersionOutput, SdkError<DeleteApplicationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_configuration_template(&self, builder: DeleteConfigurationTemplateInputBuilder) -> impl Future<Output = Result<DeleteConfigurationTemplateOutput, SdkError<DeleteConfigurationTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_environment_configuration(&self, builder: DeleteEnvironmentConfigurationInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentConfigurationOutput, SdkError<DeleteEnvironmentConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_platform_version(&self, builder: DeletePlatformVersionInputBuilder) -> impl Future<Output = Result<DeletePlatformVersionOutput, SdkError<DeletePlatformVersionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_application_versions(&self, builder: DescribeApplicationVersionsInputBuilder) -> impl Future<Output = Result<DescribeApplicationVersionsOutput, SdkError<DescribeApplicationVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_configuration_options(&self, builder: DescribeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOptionsOutput, SdkError<DescribeConfigurationOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_configuration_settings(&self, builder: DescribeConfigurationSettingsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationSettingsOutput, SdkError<DescribeConfigurationSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_environment_health(&self, builder: DescribeEnvironmentHealthInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentHealthOutput, SdkError<DescribeEnvironmentHealthError>>> {
        builder.send_with(&self.0)
    }
    fn describe_environment_managed_action_history(&self, builder: DescribeEnvironmentManagedActionHistoryInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionHistoryOutput, SdkError<DescribeEnvironmentManagedActionHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_environment_managed_actions(&self, builder: DescribeEnvironmentManagedActionsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionsOutput, SdkError<DescribeEnvironmentManagedActionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_environment_resources(&self, builder: DescribeEnvironmentResourcesInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentResourcesOutput, SdkError<DescribeEnvironmentResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn describe_environments(&self, builder: DescribeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentsOutput, SdkError<DescribeEnvironmentsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_instances_health(&self, builder: DescribeInstancesHealthInputBuilder) -> impl Future<Output = Result<DescribeInstancesHealthOutput, SdkError<DescribeInstancesHealthError>>> {
        builder.send_with(&self.0)
    }
    fn describe_platform_version(&self, builder: DescribePlatformVersionInputBuilder) -> impl Future<Output = Result<DescribePlatformVersionOutput, SdkError<DescribePlatformVersionError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_environment_operations_role(&self, builder: DisassociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnvironmentOperationsRoleOutput, SdkError<DisassociateEnvironmentOperationsRoleError>>> {
        builder.send_with(&self.0)
    }
    fn list_available_solution_stacks(&self, builder: ListAvailableSolutionStacksInputBuilder) -> impl Future<Output = Result<ListAvailableSolutionStacksOutput, SdkError<ListAvailableSolutionStacksError>>> {
        builder.send_with(&self.0)
    }
    fn list_platform_branches(&self, builder: ListPlatformBranchesInputBuilder) -> impl Future<Output = Result<ListPlatformBranchesOutput, SdkError<ListPlatformBranchesError>>> {
        builder.send_with(&self.0)
    }
    fn list_platform_versions(&self, builder: ListPlatformVersionsInputBuilder) -> impl Future<Output = Result<ListPlatformVersionsOutput, SdkError<ListPlatformVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn rebuild_environment(&self, builder: RebuildEnvironmentInputBuilder) -> impl Future<Output = Result<RebuildEnvironmentOutput, SdkError<RebuildEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn request_environment_info(&self, builder: RequestEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RequestEnvironmentInfoOutput, SdkError<RequestEnvironmentInfoError>>> {
        builder.send_with(&self.0)
    }
    fn restart_app_server(&self, builder: RestartAppServerInputBuilder) -> impl Future<Output = Result<RestartAppServerOutput, SdkError<RestartAppServerError>>> {
        builder.send_with(&self.0)
    }
    fn retrieve_environment_info(&self, builder: RetrieveEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RetrieveEnvironmentInfoOutput, SdkError<RetrieveEnvironmentInfoError>>> {
        builder.send_with(&self.0)
    }
    fn swap_environment_cnames(&self, builder: SwapEnvironmentCnamEsInputBuilder) -> impl Future<Output = Result<SwapEnvironmentCnamEsOutput, SdkError<SwapEnvironmentCNAMEsError>>> {
        builder.send_with(&self.0)
    }
    fn terminate_environment(&self, builder: TerminateEnvironmentInputBuilder) -> impl Future<Output = Result<TerminateEnvironmentOutput, SdkError<TerminateEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn update_application_resource_lifecycle(&self, builder: UpdateApplicationResourceLifecycleInputBuilder) -> impl Future<Output = Result<UpdateApplicationResourceLifecycleOutput, SdkError<UpdateApplicationResourceLifecycleError>>> {
        builder.send_with(&self.0)
    }
    fn update_application_version(&self, builder: UpdateApplicationVersionInputBuilder) -> impl Future<Output = Result<UpdateApplicationVersionOutput, SdkError<UpdateApplicationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_configuration_template(&self, builder: UpdateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<UpdateConfigurationTemplateOutput, SdkError<UpdateConfigurationTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_tags_for_resource(&self, builder: UpdateTagsForResourceInputBuilder) -> impl Future<Output = Result<UpdateTagsForResourceOutput, SdkError<UpdateTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn validate_configuration_settings(&self, builder: ValidateConfigurationSettingsInputBuilder) -> impl Future<Output = Result<ValidateConfigurationSettingsOutput, SdkError<ValidateConfigurationSettingsError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> ElasticBeanstalkClient for T
where T: Deref,
      T::Target: ElasticBeanstalkClient {
    fn abort_environment_update(&self, builder: AbortEnvironmentUpdateInputBuilder) -> impl Future<Output = Result<AbortEnvironmentUpdateOutput, SdkError<AbortEnvironmentUpdateError>>> {
        self.deref().abort_environment_update(builder)
    }
    fn apply_environment_managed_action(&self, builder: ApplyEnvironmentManagedActionInputBuilder) -> impl Future<Output = Result<ApplyEnvironmentManagedActionOutput, SdkError<ApplyEnvironmentManagedActionError>>> {
        self.deref().apply_environment_managed_action(builder)
    }
    fn associate_environment_operations_role(&self, builder: AssociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<AssociateEnvironmentOperationsRoleOutput, SdkError<AssociateEnvironmentOperationsRoleError>>> {
        self.deref().associate_environment_operations_role(builder)
    }
    fn check_dns_availability(&self, builder: CheckDnsAvailabilityInputBuilder) -> impl Future<Output = Result<CheckDnsAvailabilityOutput, SdkError<CheckDNSAvailabilityError>>> {
        self.deref().check_dns_availability(builder)
    }
    fn compose_environments(&self, builder: ComposeEnvironmentsInputBuilder) -> impl Future<Output = Result<ComposeEnvironmentsOutput, SdkError<ComposeEnvironmentsError>>> {
        self.deref().compose_environments(builder)
    }
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        self.deref().create_application(builder)
    }
    fn create_application_version(&self, builder: CreateApplicationVersionInputBuilder) -> impl Future<Output = Result<CreateApplicationVersionOutput, SdkError<CreateApplicationVersionError>>> {
        self.deref().create_application_version(builder)
    }
    fn create_configuration_template(&self, builder: CreateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<CreateConfigurationTemplateOutput, SdkError<CreateConfigurationTemplateError>>> {
        self.deref().create_configuration_template(builder)
    }
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> {
        self.deref().create_environment(builder)
    }
    fn create_platform_version(&self, builder: CreatePlatformVersionInputBuilder) -> impl Future<Output = Result<CreatePlatformVersionOutput, SdkError<CreatePlatformVersionError>>> {
        self.deref().create_platform_version(builder)
    }
    fn create_storage_location(&self, builder: CreateStorageLocationInputBuilder) -> impl Future<Output = Result<CreateStorageLocationOutput, SdkError<CreateStorageLocationError>>> {
        self.deref().create_storage_location(builder)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        self.deref().delete_application(builder)
    }
    fn delete_application_version(&self, builder: DeleteApplicationVersionInputBuilder) -> impl Future<Output = Result<DeleteApplicationVersionOutput, SdkError<DeleteApplicationVersionError>>> {
        self.deref().delete_application_version(builder)
    }
    fn delete_configuration_template(&self, builder: DeleteConfigurationTemplateInputBuilder) -> impl Future<Output = Result<DeleteConfigurationTemplateOutput, SdkError<DeleteConfigurationTemplateError>>> {
        self.deref().delete_configuration_template(builder)
    }
    fn delete_environment_configuration(&self, builder: DeleteEnvironmentConfigurationInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentConfigurationOutput, SdkError<DeleteEnvironmentConfigurationError>>> {
        self.deref().delete_environment_configuration(builder)
    }
    fn delete_platform_version(&self, builder: DeletePlatformVersionInputBuilder) -> impl Future<Output = Result<DeletePlatformVersionOutput, SdkError<DeletePlatformVersionError>>> {
        self.deref().delete_platform_version(builder)
    }
    fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> impl Future<Output = Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>> {
        self.deref().describe_account_attributes(builder)
    }
    fn describe_application_versions(&self, builder: DescribeApplicationVersionsInputBuilder) -> impl Future<Output = Result<DescribeApplicationVersionsOutput, SdkError<DescribeApplicationVersionsError>>> {
        self.deref().describe_application_versions(builder)
    }
    fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> impl Future<Output = Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>> {
        self.deref().describe_applications(builder)
    }
    fn describe_configuration_options(&self, builder: DescribeConfigurationOptionsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationOptionsOutput, SdkError<DescribeConfigurationOptionsError>>> {
        self.deref().describe_configuration_options(builder)
    }
    fn describe_configuration_settings(&self, builder: DescribeConfigurationSettingsInputBuilder) -> impl Future<Output = Result<DescribeConfigurationSettingsOutput, SdkError<DescribeConfigurationSettingsError>>> {
        self.deref().describe_configuration_settings(builder)
    }
    fn describe_environment_health(&self, builder: DescribeEnvironmentHealthInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentHealthOutput, SdkError<DescribeEnvironmentHealthError>>> {
        self.deref().describe_environment_health(builder)
    }
    fn describe_environment_managed_action_history(&self, builder: DescribeEnvironmentManagedActionHistoryInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionHistoryOutput, SdkError<DescribeEnvironmentManagedActionHistoryError>>> {
        self.deref().describe_environment_managed_action_history(builder)
    }
    fn describe_environment_managed_actions(&self, builder: DescribeEnvironmentManagedActionsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentManagedActionsOutput, SdkError<DescribeEnvironmentManagedActionsError>>> {
        self.deref().describe_environment_managed_actions(builder)
    }
    fn describe_environment_resources(&self, builder: DescribeEnvironmentResourcesInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentResourcesOutput, SdkError<DescribeEnvironmentResourcesError>>> {
        self.deref().describe_environment_resources(builder)
    }
    fn describe_environments(&self, builder: DescribeEnvironmentsInputBuilder) -> impl Future<Output = Result<DescribeEnvironmentsOutput, SdkError<DescribeEnvironmentsError>>> {
        self.deref().describe_environments(builder)
    }
    fn describe_events(&self, builder: DescribeEventsInputBuilder) -> impl Future<Output = Result<DescribeEventsOutput, SdkError<DescribeEventsError>>> {
        self.deref().describe_events(builder)
    }
    fn describe_instances_health(&self, builder: DescribeInstancesHealthInputBuilder) -> impl Future<Output = Result<DescribeInstancesHealthOutput, SdkError<DescribeInstancesHealthError>>> {
        self.deref().describe_instances_health(builder)
    }
    fn describe_platform_version(&self, builder: DescribePlatformVersionInputBuilder) -> impl Future<Output = Result<DescribePlatformVersionOutput, SdkError<DescribePlatformVersionError>>> {
        self.deref().describe_platform_version(builder)
    }
    fn disassociate_environment_operations_role(&self, builder: DisassociateEnvironmentOperationsRoleInputBuilder) -> impl Future<Output = Result<DisassociateEnvironmentOperationsRoleOutput, SdkError<DisassociateEnvironmentOperationsRoleError>>> {
        self.deref().disassociate_environment_operations_role(builder)
    }
    fn list_available_solution_stacks(&self, builder: ListAvailableSolutionStacksInputBuilder) -> impl Future<Output = Result<ListAvailableSolutionStacksOutput, SdkError<ListAvailableSolutionStacksError>>> {
        self.deref().list_available_solution_stacks(builder)
    }
    fn list_platform_branches(&self, builder: ListPlatformBranchesInputBuilder) -> impl Future<Output = Result<ListPlatformBranchesOutput, SdkError<ListPlatformBranchesError>>> {
        self.deref().list_platform_branches(builder)
    }
    fn list_platform_versions(&self, builder: ListPlatformVersionsInputBuilder) -> impl Future<Output = Result<ListPlatformVersionsOutput, SdkError<ListPlatformVersionsError>>> {
        self.deref().list_platform_versions(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn rebuild_environment(&self, builder: RebuildEnvironmentInputBuilder) -> impl Future<Output = Result<RebuildEnvironmentOutput, SdkError<RebuildEnvironmentError>>> {
        self.deref().rebuild_environment(builder)
    }
    fn request_environment_info(&self, builder: RequestEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RequestEnvironmentInfoOutput, SdkError<RequestEnvironmentInfoError>>> {
        self.deref().request_environment_info(builder)
    }
    fn restart_app_server(&self, builder: RestartAppServerInputBuilder) -> impl Future<Output = Result<RestartAppServerOutput, SdkError<RestartAppServerError>>> {
        self.deref().restart_app_server(builder)
    }
    fn retrieve_environment_info(&self, builder: RetrieveEnvironmentInfoInputBuilder) -> impl Future<Output = Result<RetrieveEnvironmentInfoOutput, SdkError<RetrieveEnvironmentInfoError>>> {
        self.deref().retrieve_environment_info(builder)
    }
    fn swap_environment_cnames(&self, builder: SwapEnvironmentCnamEsInputBuilder) -> impl Future<Output = Result<SwapEnvironmentCnamEsOutput, SdkError<SwapEnvironmentCNAMEsError>>> {
        self.deref().swap_environment_cnames(builder)
    }
    fn terminate_environment(&self, builder: TerminateEnvironmentInputBuilder) -> impl Future<Output = Result<TerminateEnvironmentOutput, SdkError<TerminateEnvironmentError>>> {
        self.deref().terminate_environment(builder)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        self.deref().update_application(builder)
    }
    fn update_application_resource_lifecycle(&self, builder: UpdateApplicationResourceLifecycleInputBuilder) -> impl Future<Output = Result<UpdateApplicationResourceLifecycleOutput, SdkError<UpdateApplicationResourceLifecycleError>>> {
        self.deref().update_application_resource_lifecycle(builder)
    }
    fn update_application_version(&self, builder: UpdateApplicationVersionInputBuilder) -> impl Future<Output = Result<UpdateApplicationVersionOutput, SdkError<UpdateApplicationVersionError>>> {
        self.deref().update_application_version(builder)
    }
    fn update_configuration_template(&self, builder: UpdateConfigurationTemplateInputBuilder) -> impl Future<Output = Result<UpdateConfigurationTemplateOutput, SdkError<UpdateConfigurationTemplateError>>> {
        self.deref().update_configuration_template(builder)
    }
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> {
        self.deref().update_environment(builder)
    }
    fn update_tags_for_resource(&self, builder: UpdateTagsForResourceInputBuilder) -> impl Future<Output = Result<UpdateTagsForResourceOutput, SdkError<UpdateTagsForResourceError>>> {
        self.deref().update_tags_for_resource(builder)
    }
    fn validate_configuration_settings(&self, builder: ValidateConfigurationSettingsInputBuilder) -> impl Future<Output = Result<ValidateConfigurationSettingsOutput, SdkError<ValidateConfigurationSettingsError>>> {
        self.deref().validate_configuration_settings(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edElasticBeanstalkClient {}
    impl ElasticBeanstalkClient for edElasticBeanstalkClient {
        async fn abort_environment_update(&self, builder: AbortEnvironmentUpdateInputBuilder) -> Result<AbortEnvironmentUpdateOutput, SdkError<AbortEnvironmentUpdateError>>;
        async fn apply_environment_managed_action(&self, builder: ApplyEnvironmentManagedActionInputBuilder) -> Result<ApplyEnvironmentManagedActionOutput, SdkError<ApplyEnvironmentManagedActionError>>;
        async fn associate_environment_operations_role(&self, builder: AssociateEnvironmentOperationsRoleInputBuilder) -> Result<AssociateEnvironmentOperationsRoleOutput, SdkError<AssociateEnvironmentOperationsRoleError>>;
        async fn check_dns_availability(&self, builder: CheckDnsAvailabilityInputBuilder) -> Result<CheckDnsAvailabilityOutput, SdkError<CheckDNSAvailabilityError>>;
        async fn compose_environments(&self, builder: ComposeEnvironmentsInputBuilder) -> Result<ComposeEnvironmentsOutput, SdkError<ComposeEnvironmentsError>>;
        async fn create_application(&self, builder: CreateApplicationInputBuilder) -> Result<CreateApplicationOutput, SdkError<CreateApplicationError>>;
        async fn create_application_version(&self, builder: CreateApplicationVersionInputBuilder) -> Result<CreateApplicationVersionOutput, SdkError<CreateApplicationVersionError>>;
        async fn create_configuration_template(&self, builder: CreateConfigurationTemplateInputBuilder) -> Result<CreateConfigurationTemplateOutput, SdkError<CreateConfigurationTemplateError>>;
        async fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>;
        async fn create_platform_version(&self, builder: CreatePlatformVersionInputBuilder) -> Result<CreatePlatformVersionOutput, SdkError<CreatePlatformVersionError>>;
        async fn create_storage_location(&self, builder: CreateStorageLocationInputBuilder) -> Result<CreateStorageLocationOutput, SdkError<CreateStorageLocationError>>;
        async fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>;
        async fn delete_application_version(&self, builder: DeleteApplicationVersionInputBuilder) -> Result<DeleteApplicationVersionOutput, SdkError<DeleteApplicationVersionError>>;
        async fn delete_configuration_template(&self, builder: DeleteConfigurationTemplateInputBuilder) -> Result<DeleteConfigurationTemplateOutput, SdkError<DeleteConfigurationTemplateError>>;
        async fn delete_environment_configuration(&self, builder: DeleteEnvironmentConfigurationInputBuilder) -> Result<DeleteEnvironmentConfigurationOutput, SdkError<DeleteEnvironmentConfigurationError>>;
        async fn delete_platform_version(&self, builder: DeletePlatformVersionInputBuilder) -> Result<DeletePlatformVersionOutput, SdkError<DeletePlatformVersionError>>;
        async fn describe_account_attributes(&self, builder: DescribeAccountAttributesInputBuilder) -> Result<DescribeAccountAttributesOutput, SdkError<DescribeAccountAttributesError>>;
        async fn describe_application_versions(&self, builder: DescribeApplicationVersionsInputBuilder) -> Result<DescribeApplicationVersionsOutput, SdkError<DescribeApplicationVersionsError>>;
        async fn describe_applications(&self, builder: DescribeApplicationsInputBuilder) -> Result<DescribeApplicationsOutput, SdkError<DescribeApplicationsError>>;
        async fn describe_configuration_options(&self, builder: DescribeConfigurationOptionsInputBuilder) -> Result<DescribeConfigurationOptionsOutput, SdkError<DescribeConfigurationOptionsError>>;
        async fn describe_configuration_settings(&self, builder: DescribeConfigurationSettingsInputBuilder) -> Result<DescribeConfigurationSettingsOutput, SdkError<DescribeConfigurationSettingsError>>;
        async fn describe_environment_health(&self, builder: DescribeEnvironmentHealthInputBuilder) -> Result<DescribeEnvironmentHealthOutput, SdkError<DescribeEnvironmentHealthError>>;
        async fn describe_environment_managed_action_history(&self, builder: DescribeEnvironmentManagedActionHistoryInputBuilder) -> Result<DescribeEnvironmentManagedActionHistoryOutput, SdkError<DescribeEnvironmentManagedActionHistoryError>>;
        async fn describe_environment_managed_actions(&self, builder: DescribeEnvironmentManagedActionsInputBuilder) -> Result<DescribeEnvironmentManagedActionsOutput, SdkError<DescribeEnvironmentManagedActionsError>>;
        async fn describe_environment_resources(&self, builder: DescribeEnvironmentResourcesInputBuilder) -> Result<DescribeEnvironmentResourcesOutput, SdkError<DescribeEnvironmentResourcesError>>;
        async fn describe_environments(&self, builder: DescribeEnvironmentsInputBuilder) -> Result<DescribeEnvironmentsOutput, SdkError<DescribeEnvironmentsError>>;
        async fn describe_events(&self, builder: DescribeEventsInputBuilder) -> Result<DescribeEventsOutput, SdkError<DescribeEventsError>>;
        async fn describe_instances_health(&self, builder: DescribeInstancesHealthInputBuilder) -> Result<DescribeInstancesHealthOutput, SdkError<DescribeInstancesHealthError>>;
        async fn describe_platform_version(&self, builder: DescribePlatformVersionInputBuilder) -> Result<DescribePlatformVersionOutput, SdkError<DescribePlatformVersionError>>;
        async fn disassociate_environment_operations_role(&self, builder: DisassociateEnvironmentOperationsRoleInputBuilder) -> Result<DisassociateEnvironmentOperationsRoleOutput, SdkError<DisassociateEnvironmentOperationsRoleError>>;
        async fn list_available_solution_stacks(&self, builder: ListAvailableSolutionStacksInputBuilder) -> Result<ListAvailableSolutionStacksOutput, SdkError<ListAvailableSolutionStacksError>>;
        async fn list_platform_branches(&self, builder: ListPlatformBranchesInputBuilder) -> Result<ListPlatformBranchesOutput, SdkError<ListPlatformBranchesError>>;
        async fn list_platform_versions(&self, builder: ListPlatformVersionsInputBuilder) -> Result<ListPlatformVersionsOutput, SdkError<ListPlatformVersionsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn rebuild_environment(&self, builder: RebuildEnvironmentInputBuilder) -> Result<RebuildEnvironmentOutput, SdkError<RebuildEnvironmentError>>;
        async fn request_environment_info(&self, builder: RequestEnvironmentInfoInputBuilder) -> Result<RequestEnvironmentInfoOutput, SdkError<RequestEnvironmentInfoError>>;
        async fn restart_app_server(&self, builder: RestartAppServerInputBuilder) -> Result<RestartAppServerOutput, SdkError<RestartAppServerError>>;
        async fn retrieve_environment_info(&self, builder: RetrieveEnvironmentInfoInputBuilder) -> Result<RetrieveEnvironmentInfoOutput, SdkError<RetrieveEnvironmentInfoError>>;
        async fn swap_environment_cnames(&self, builder: SwapEnvironmentCnamEsInputBuilder) -> Result<SwapEnvironmentCnamEsOutput, SdkError<SwapEnvironmentCNAMEsError>>;
        async fn terminate_environment(&self, builder: TerminateEnvironmentInputBuilder) -> Result<TerminateEnvironmentOutput, SdkError<TerminateEnvironmentError>>;
        async fn update_application(&self, builder: UpdateApplicationInputBuilder) -> Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>;
        async fn update_application_resource_lifecycle(&self, builder: UpdateApplicationResourceLifecycleInputBuilder) -> Result<UpdateApplicationResourceLifecycleOutput, SdkError<UpdateApplicationResourceLifecycleError>>;
        async fn update_application_version(&self, builder: UpdateApplicationVersionInputBuilder) -> Result<UpdateApplicationVersionOutput, SdkError<UpdateApplicationVersionError>>;
        async fn update_configuration_template(&self, builder: UpdateConfigurationTemplateInputBuilder) -> Result<UpdateConfigurationTemplateOutput, SdkError<UpdateConfigurationTemplateError>>;
        async fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>;
        async fn update_tags_for_resource(&self, builder: UpdateTagsForResourceInputBuilder) -> Result<UpdateTagsForResourceOutput, SdkError<UpdateTagsForResourceError>>;
        async fn validate_configuration_settings(&self, builder: ValidateConfigurationSettingsInputBuilder) -> Result<ValidateConfigurationSettingsOutput, SdkError<ValidateConfigurationSettingsError>>;
    }
}
