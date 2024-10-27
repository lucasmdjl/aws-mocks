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
use aws_sdk_appconfig::operation::create_application::{builders::*, *};
use aws_sdk_appconfig::operation::create_configuration_profile::{builders::*, *};
use aws_sdk_appconfig::operation::create_deployment_strategy::{builders::*, *};
use aws_sdk_appconfig::operation::create_environment::{builders::*, *};
use aws_sdk_appconfig::operation::create_extension::{builders::*, *};
use aws_sdk_appconfig::operation::create_extension_association::{builders::*, *};
use aws_sdk_appconfig::operation::create_hosted_configuration_version::{builders::*, *};
use aws_sdk_appconfig::operation::delete_application::{builders::*, *};
use aws_sdk_appconfig::operation::delete_configuration_profile::{builders::*, *};
use aws_sdk_appconfig::operation::delete_deployment_strategy::{builders::*, *};
use aws_sdk_appconfig::operation::delete_environment::{builders::*, *};
use aws_sdk_appconfig::operation::delete_extension::{builders::*, *};
use aws_sdk_appconfig::operation::delete_extension_association::{builders::*, *};
use aws_sdk_appconfig::operation::delete_hosted_configuration_version::{builders::*, *};
use aws_sdk_appconfig::operation::get_application::{builders::*, *};
use aws_sdk_appconfig::operation::get_configuration_profile::{builders::*, *};
use aws_sdk_appconfig::operation::get_deployment::{builders::*, *};
use aws_sdk_appconfig::operation::get_deployment_strategy::{builders::*, *};
use aws_sdk_appconfig::operation::get_environment::{builders::*, *};
use aws_sdk_appconfig::operation::get_extension::{builders::*, *};
use aws_sdk_appconfig::operation::get_extension_association::{builders::*, *};
use aws_sdk_appconfig::operation::get_hosted_configuration_version::{builders::*, *};
use aws_sdk_appconfig::operation::list_applications::{builders::*, *};
use aws_sdk_appconfig::operation::list_configuration_profiles::{builders::*, *};
use aws_sdk_appconfig::operation::list_deployment_strategies::{builders::*, *};
use aws_sdk_appconfig::operation::list_deployments::{builders::*, *};
use aws_sdk_appconfig::operation::list_environments::{builders::*, *};
use aws_sdk_appconfig::operation::list_extension_associations::{builders::*, *};
use aws_sdk_appconfig::operation::list_extensions::{builders::*, *};
use aws_sdk_appconfig::operation::list_hosted_configuration_versions::{builders::*, *};
use aws_sdk_appconfig::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appconfig::operation::start_deployment::{builders::*, *};
use aws_sdk_appconfig::operation::stop_deployment::{builders::*, *};
use aws_sdk_appconfig::operation::tag_resource::{builders::*, *};
use aws_sdk_appconfig::operation::untag_resource::{builders::*, *};
use aws_sdk_appconfig::operation::update_application::{builders::*, *};
use aws_sdk_appconfig::operation::update_configuration_profile::{builders::*, *};
use aws_sdk_appconfig::operation::update_deployment_strategy::{builders::*, *};
use aws_sdk_appconfig::operation::update_environment::{builders::*, *};
use aws_sdk_appconfig::operation::update_extension::{builders::*, *};
use aws_sdk_appconfig::operation::update_extension_association::{builders::*, *};
use aws_sdk_appconfig::operation::validate_configuration::{builders::*, *};
use aws_sdk_appconfig::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appconfig::Client;
use std::ops::Deref;

pub use aws_sdk_appconfig::*;

pub struct AppConfigClientImpl(Client);
impl AppConfigClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppConfigClient {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> + Send;
    fn create_configuration_profile(&self, builder: CreateConfigurationProfileInputBuilder) -> impl Future<Output = Result<CreateConfigurationProfileOutput, SdkError<CreateConfigurationProfileError>>> + Send;
    fn create_deployment_strategy(&self, builder: CreateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<CreateDeploymentStrategyOutput, SdkError<CreateDeploymentStrategyError>>> + Send;
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> + Send;
    fn create_extension(&self, builder: CreateExtensionInputBuilder) -> impl Future<Output = Result<CreateExtensionOutput, SdkError<CreateExtensionError>>> + Send;
    fn create_extension_association(&self, builder: CreateExtensionAssociationInputBuilder) -> impl Future<Output = Result<CreateExtensionAssociationOutput, SdkError<CreateExtensionAssociationError>>> + Send;
    fn create_hosted_configuration_version(&self, builder: CreateHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<CreateHostedConfigurationVersionOutput, SdkError<CreateHostedConfigurationVersionError>>> + Send;
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> + Send;
    fn delete_configuration_profile(&self, builder: DeleteConfigurationProfileInputBuilder) -> impl Future<Output = Result<DeleteConfigurationProfileOutput, SdkError<DeleteConfigurationProfileError>>> + Send;
    fn delete_deployment_strategy(&self, builder: DeleteDeploymentStrategyInputBuilder) -> impl Future<Output = Result<DeleteDeploymentStrategyOutput, SdkError<DeleteDeploymentStrategyError>>> + Send;
    fn delete_environment(&self, builder: DeleteEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentOutput, SdkError<DeleteEnvironmentError>>> + Send;
    fn delete_extension(&self, builder: DeleteExtensionInputBuilder) -> impl Future<Output = Result<DeleteExtensionOutput, SdkError<DeleteExtensionError>>> + Send;
    fn delete_extension_association(&self, builder: DeleteExtensionAssociationInputBuilder) -> impl Future<Output = Result<DeleteExtensionAssociationOutput, SdkError<DeleteExtensionAssociationError>>> + Send;
    fn delete_hosted_configuration_version(&self, builder: DeleteHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<DeleteHostedConfigurationVersionOutput, SdkError<DeleteHostedConfigurationVersionError>>> + Send;
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>> + Send;
    fn get_configuration_profile(&self, builder: GetConfigurationProfileInputBuilder) -> impl Future<Output = Result<GetConfigurationProfileOutput, SdkError<GetConfigurationProfileError>>> + Send;
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> + Send;
    fn get_deployment_strategy(&self, builder: GetDeploymentStrategyInputBuilder) -> impl Future<Output = Result<GetDeploymentStrategyOutput, SdkError<GetDeploymentStrategyError>>> + Send;
    fn get_environment(&self, builder: GetEnvironmentInputBuilder) -> impl Future<Output = Result<GetEnvironmentOutput, SdkError<GetEnvironmentError>>> + Send;
    fn get_extension(&self, builder: GetExtensionInputBuilder) -> impl Future<Output = Result<GetExtensionOutput, SdkError<GetExtensionError>>> + Send;
    fn get_extension_association(&self, builder: GetExtensionAssociationInputBuilder) -> impl Future<Output = Result<GetExtensionAssociationOutput, SdkError<GetExtensionAssociationError>>> + Send;
    fn get_hosted_configuration_version(&self, builder: GetHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<GetHostedConfigurationVersionOutput, SdkError<GetHostedConfigurationVersionError>>> + Send;
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>> + Send;
    fn list_configuration_profiles(&self, builder: ListConfigurationProfilesInputBuilder) -> impl Future<Output = Result<ListConfigurationProfilesOutput, SdkError<ListConfigurationProfilesError>>> + Send;
    fn list_deployment_strategies(&self, builder: ListDeploymentStrategiesInputBuilder) -> impl Future<Output = Result<ListDeploymentStrategiesOutput, SdkError<ListDeploymentStrategiesError>>> + Send;
    fn list_deployments(&self, builder: ListDeploymentsInputBuilder) -> impl Future<Output = Result<ListDeploymentsOutput, SdkError<ListDeploymentsError>>> + Send;
    fn list_environments(&self, builder: ListEnvironmentsInputBuilder) -> impl Future<Output = Result<ListEnvironmentsOutput, SdkError<ListEnvironmentsError>>> + Send;
    fn list_extension_associations(&self, builder: ListExtensionAssociationsInputBuilder) -> impl Future<Output = Result<ListExtensionAssociationsOutput, SdkError<ListExtensionAssociationsError>>> + Send;
    fn list_extensions(&self, builder: ListExtensionsInputBuilder) -> impl Future<Output = Result<ListExtensionsOutput, SdkError<ListExtensionsError>>> + Send;
    fn list_hosted_configuration_versions(&self, builder: ListHostedConfigurationVersionsInputBuilder) -> impl Future<Output = Result<ListHostedConfigurationVersionsOutput, SdkError<ListHostedConfigurationVersionsError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> + Send;
    fn stop_deployment(&self, builder: StopDeploymentInputBuilder) -> impl Future<Output = Result<StopDeploymentOutput, SdkError<StopDeploymentError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> + Send;
    fn update_configuration_profile(&self, builder: UpdateConfigurationProfileInputBuilder) -> impl Future<Output = Result<UpdateConfigurationProfileOutput, SdkError<UpdateConfigurationProfileError>>> + Send;
    fn update_deployment_strategy(&self, builder: UpdateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<UpdateDeploymentStrategyOutput, SdkError<UpdateDeploymentStrategyError>>> + Send;
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> + Send;
    fn update_extension(&self, builder: UpdateExtensionInputBuilder) -> impl Future<Output = Result<UpdateExtensionOutput, SdkError<UpdateExtensionError>>> + Send;
    fn update_extension_association(&self, builder: UpdateExtensionAssociationInputBuilder) -> impl Future<Output = Result<UpdateExtensionAssociationOutput, SdkError<UpdateExtensionAssociationError>>> + Send;
    fn validate_configuration(&self, builder: ValidateConfigurationInputBuilder) -> impl Future<Output = Result<ValidateConfigurationOutput, SdkError<ValidateConfigurationError>>> + Send;
}
impl AppConfigClient for AppConfigClientImpl {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn create_configuration_profile(&self, builder: CreateConfigurationProfileInputBuilder) -> impl Future<Output = Result<CreateConfigurationProfileOutput, SdkError<CreateConfigurationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_deployment_strategy(&self, builder: CreateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<CreateDeploymentStrategyOutput, SdkError<CreateDeploymentStrategyError>>> {
        builder.send_with(&self.0)
    }
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn create_extension(&self, builder: CreateExtensionInputBuilder) -> impl Future<Output = Result<CreateExtensionOutput, SdkError<CreateExtensionError>>> {
        builder.send_with(&self.0)
    }
    fn create_extension_association(&self, builder: CreateExtensionAssociationInputBuilder) -> impl Future<Output = Result<CreateExtensionAssociationOutput, SdkError<CreateExtensionAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn create_hosted_configuration_version(&self, builder: CreateHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<CreateHostedConfigurationVersionOutput, SdkError<CreateHostedConfigurationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_configuration_profile(&self, builder: DeleteConfigurationProfileInputBuilder) -> impl Future<Output = Result<DeleteConfigurationProfileOutput, SdkError<DeleteConfigurationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_deployment_strategy(&self, builder: DeleteDeploymentStrategyInputBuilder) -> impl Future<Output = Result<DeleteDeploymentStrategyOutput, SdkError<DeleteDeploymentStrategyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_environment(&self, builder: DeleteEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentOutput, SdkError<DeleteEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_extension(&self, builder: DeleteExtensionInputBuilder) -> impl Future<Output = Result<DeleteExtensionOutput, SdkError<DeleteExtensionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_extension_association(&self, builder: DeleteExtensionAssociationInputBuilder) -> impl Future<Output = Result<DeleteExtensionAssociationOutput, SdkError<DeleteExtensionAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_hosted_configuration_version(&self, builder: DeleteHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<DeleteHostedConfigurationVersionOutput, SdkError<DeleteHostedConfigurationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn get_configuration_profile(&self, builder: GetConfigurationProfileInputBuilder) -> impl Future<Output = Result<GetConfigurationProfileOutput, SdkError<GetConfigurationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployment_strategy(&self, builder: GetDeploymentStrategyInputBuilder) -> impl Future<Output = Result<GetDeploymentStrategyOutput, SdkError<GetDeploymentStrategyError>>> {
        builder.send_with(&self.0)
    }
    fn get_environment(&self, builder: GetEnvironmentInputBuilder) -> impl Future<Output = Result<GetEnvironmentOutput, SdkError<GetEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn get_extension(&self, builder: GetExtensionInputBuilder) -> impl Future<Output = Result<GetExtensionOutput, SdkError<GetExtensionError>>> {
        builder.send_with(&self.0)
    }
    fn get_extension_association(&self, builder: GetExtensionAssociationInputBuilder) -> impl Future<Output = Result<GetExtensionAssociationOutput, SdkError<GetExtensionAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn get_hosted_configuration_version(&self, builder: GetHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<GetHostedConfigurationVersionOutput, SdkError<GetHostedConfigurationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_configuration_profiles(&self, builder: ListConfigurationProfilesInputBuilder) -> impl Future<Output = Result<ListConfigurationProfilesOutput, SdkError<ListConfigurationProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_deployment_strategies(&self, builder: ListDeploymentStrategiesInputBuilder) -> impl Future<Output = Result<ListDeploymentStrategiesOutput, SdkError<ListDeploymentStrategiesError>>> {
        builder.send_with(&self.0)
    }
    fn list_deployments(&self, builder: ListDeploymentsInputBuilder) -> impl Future<Output = Result<ListDeploymentsOutput, SdkError<ListDeploymentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_environments(&self, builder: ListEnvironmentsInputBuilder) -> impl Future<Output = Result<ListEnvironmentsOutput, SdkError<ListEnvironmentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_extension_associations(&self, builder: ListExtensionAssociationsInputBuilder) -> impl Future<Output = Result<ListExtensionAssociationsOutput, SdkError<ListExtensionAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_extensions(&self, builder: ListExtensionsInputBuilder) -> impl Future<Output = Result<ListExtensionsOutput, SdkError<ListExtensionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_hosted_configuration_versions(&self, builder: ListHostedConfigurationVersionsInputBuilder) -> impl Future<Output = Result<ListHostedConfigurationVersionsOutput, SdkError<ListHostedConfigurationVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn stop_deployment(&self, builder: StopDeploymentInputBuilder) -> impl Future<Output = Result<StopDeploymentOutput, SdkError<StopDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        builder.send_with(&self.0)
    }
    fn update_configuration_profile(&self, builder: UpdateConfigurationProfileInputBuilder) -> impl Future<Output = Result<UpdateConfigurationProfileOutput, SdkError<UpdateConfigurationProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_deployment_strategy(&self, builder: UpdateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<UpdateDeploymentStrategyOutput, SdkError<UpdateDeploymentStrategyError>>> {
        builder.send_with(&self.0)
    }
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> {
        builder.send_with(&self.0)
    }
    fn update_extension(&self, builder: UpdateExtensionInputBuilder) -> impl Future<Output = Result<UpdateExtensionOutput, SdkError<UpdateExtensionError>>> {
        builder.send_with(&self.0)
    }
    fn update_extension_association(&self, builder: UpdateExtensionAssociationInputBuilder) -> impl Future<Output = Result<UpdateExtensionAssociationOutput, SdkError<UpdateExtensionAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn validate_configuration(&self, builder: ValidateConfigurationInputBuilder) -> impl Future<Output = Result<ValidateConfigurationOutput, SdkError<ValidateConfigurationError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppConfigClient for T
where T: Deref,
      T::Target: AppConfigClient {
    fn create_application(&self, builder: CreateApplicationInputBuilder) -> impl Future<Output = Result<CreateApplicationOutput, SdkError<CreateApplicationError>>> {
        self.deref().create_application(builder)
    }
    fn create_configuration_profile(&self, builder: CreateConfigurationProfileInputBuilder) -> impl Future<Output = Result<CreateConfigurationProfileOutput, SdkError<CreateConfigurationProfileError>>> {
        self.deref().create_configuration_profile(builder)
    }
    fn create_deployment_strategy(&self, builder: CreateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<CreateDeploymentStrategyOutput, SdkError<CreateDeploymentStrategyError>>> {
        self.deref().create_deployment_strategy(builder)
    }
    fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> impl Future<Output = Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>> {
        self.deref().create_environment(builder)
    }
    fn create_extension(&self, builder: CreateExtensionInputBuilder) -> impl Future<Output = Result<CreateExtensionOutput, SdkError<CreateExtensionError>>> {
        self.deref().create_extension(builder)
    }
    fn create_extension_association(&self, builder: CreateExtensionAssociationInputBuilder) -> impl Future<Output = Result<CreateExtensionAssociationOutput, SdkError<CreateExtensionAssociationError>>> {
        self.deref().create_extension_association(builder)
    }
    fn create_hosted_configuration_version(&self, builder: CreateHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<CreateHostedConfigurationVersionOutput, SdkError<CreateHostedConfigurationVersionError>>> {
        self.deref().create_hosted_configuration_version(builder)
    }
    fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> impl Future<Output = Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>> {
        self.deref().delete_application(builder)
    }
    fn delete_configuration_profile(&self, builder: DeleteConfigurationProfileInputBuilder) -> impl Future<Output = Result<DeleteConfigurationProfileOutput, SdkError<DeleteConfigurationProfileError>>> {
        self.deref().delete_configuration_profile(builder)
    }
    fn delete_deployment_strategy(&self, builder: DeleteDeploymentStrategyInputBuilder) -> impl Future<Output = Result<DeleteDeploymentStrategyOutput, SdkError<DeleteDeploymentStrategyError>>> {
        self.deref().delete_deployment_strategy(builder)
    }
    fn delete_environment(&self, builder: DeleteEnvironmentInputBuilder) -> impl Future<Output = Result<DeleteEnvironmentOutput, SdkError<DeleteEnvironmentError>>> {
        self.deref().delete_environment(builder)
    }
    fn delete_extension(&self, builder: DeleteExtensionInputBuilder) -> impl Future<Output = Result<DeleteExtensionOutput, SdkError<DeleteExtensionError>>> {
        self.deref().delete_extension(builder)
    }
    fn delete_extension_association(&self, builder: DeleteExtensionAssociationInputBuilder) -> impl Future<Output = Result<DeleteExtensionAssociationOutput, SdkError<DeleteExtensionAssociationError>>> {
        self.deref().delete_extension_association(builder)
    }
    fn delete_hosted_configuration_version(&self, builder: DeleteHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<DeleteHostedConfigurationVersionOutput, SdkError<DeleteHostedConfigurationVersionError>>> {
        self.deref().delete_hosted_configuration_version(builder)
    }
    fn get_application(&self, builder: GetApplicationInputBuilder) -> impl Future<Output = Result<GetApplicationOutput, SdkError<GetApplicationError>>> {
        self.deref().get_application(builder)
    }
    fn get_configuration_profile(&self, builder: GetConfigurationProfileInputBuilder) -> impl Future<Output = Result<GetConfigurationProfileOutput, SdkError<GetConfigurationProfileError>>> {
        self.deref().get_configuration_profile(builder)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        self.deref().get_deployment(builder)
    }
    fn get_deployment_strategy(&self, builder: GetDeploymentStrategyInputBuilder) -> impl Future<Output = Result<GetDeploymentStrategyOutput, SdkError<GetDeploymentStrategyError>>> {
        self.deref().get_deployment_strategy(builder)
    }
    fn get_environment(&self, builder: GetEnvironmentInputBuilder) -> impl Future<Output = Result<GetEnvironmentOutput, SdkError<GetEnvironmentError>>> {
        self.deref().get_environment(builder)
    }
    fn get_extension(&self, builder: GetExtensionInputBuilder) -> impl Future<Output = Result<GetExtensionOutput, SdkError<GetExtensionError>>> {
        self.deref().get_extension(builder)
    }
    fn get_extension_association(&self, builder: GetExtensionAssociationInputBuilder) -> impl Future<Output = Result<GetExtensionAssociationOutput, SdkError<GetExtensionAssociationError>>> {
        self.deref().get_extension_association(builder)
    }
    fn get_hosted_configuration_version(&self, builder: GetHostedConfigurationVersionInputBuilder) -> impl Future<Output = Result<GetHostedConfigurationVersionOutput, SdkError<GetHostedConfigurationVersionError>>> {
        self.deref().get_hosted_configuration_version(builder)
    }
    fn list_applications(&self, builder: ListApplicationsInputBuilder) -> impl Future<Output = Result<ListApplicationsOutput, SdkError<ListApplicationsError>>> {
        self.deref().list_applications(builder)
    }
    fn list_configuration_profiles(&self, builder: ListConfigurationProfilesInputBuilder) -> impl Future<Output = Result<ListConfigurationProfilesOutput, SdkError<ListConfigurationProfilesError>>> {
        self.deref().list_configuration_profiles(builder)
    }
    fn list_deployment_strategies(&self, builder: ListDeploymentStrategiesInputBuilder) -> impl Future<Output = Result<ListDeploymentStrategiesOutput, SdkError<ListDeploymentStrategiesError>>> {
        self.deref().list_deployment_strategies(builder)
    }
    fn list_deployments(&self, builder: ListDeploymentsInputBuilder) -> impl Future<Output = Result<ListDeploymentsOutput, SdkError<ListDeploymentsError>>> {
        self.deref().list_deployments(builder)
    }
    fn list_environments(&self, builder: ListEnvironmentsInputBuilder) -> impl Future<Output = Result<ListEnvironmentsOutput, SdkError<ListEnvironmentsError>>> {
        self.deref().list_environments(builder)
    }
    fn list_extension_associations(&self, builder: ListExtensionAssociationsInputBuilder) -> impl Future<Output = Result<ListExtensionAssociationsOutput, SdkError<ListExtensionAssociationsError>>> {
        self.deref().list_extension_associations(builder)
    }
    fn list_extensions(&self, builder: ListExtensionsInputBuilder) -> impl Future<Output = Result<ListExtensionsOutput, SdkError<ListExtensionsError>>> {
        self.deref().list_extensions(builder)
    }
    fn list_hosted_configuration_versions(&self, builder: ListHostedConfigurationVersionsInputBuilder) -> impl Future<Output = Result<ListHostedConfigurationVersionsOutput, SdkError<ListHostedConfigurationVersionsError>>> {
        self.deref().list_hosted_configuration_versions(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        self.deref().start_deployment(builder)
    }
    fn stop_deployment(&self, builder: StopDeploymentInputBuilder) -> impl Future<Output = Result<StopDeploymentOutput, SdkError<StopDeploymentError>>> {
        self.deref().stop_deployment(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_application(&self, builder: UpdateApplicationInputBuilder) -> impl Future<Output = Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>> {
        self.deref().update_application(builder)
    }
    fn update_configuration_profile(&self, builder: UpdateConfigurationProfileInputBuilder) -> impl Future<Output = Result<UpdateConfigurationProfileOutput, SdkError<UpdateConfigurationProfileError>>> {
        self.deref().update_configuration_profile(builder)
    }
    fn update_deployment_strategy(&self, builder: UpdateDeploymentStrategyInputBuilder) -> impl Future<Output = Result<UpdateDeploymentStrategyOutput, SdkError<UpdateDeploymentStrategyError>>> {
        self.deref().update_deployment_strategy(builder)
    }
    fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> impl Future<Output = Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>> {
        self.deref().update_environment(builder)
    }
    fn update_extension(&self, builder: UpdateExtensionInputBuilder) -> impl Future<Output = Result<UpdateExtensionOutput, SdkError<UpdateExtensionError>>> {
        self.deref().update_extension(builder)
    }
    fn update_extension_association(&self, builder: UpdateExtensionAssociationInputBuilder) -> impl Future<Output = Result<UpdateExtensionAssociationOutput, SdkError<UpdateExtensionAssociationError>>> {
        self.deref().update_extension_association(builder)
    }
    fn validate_configuration(&self, builder: ValidateConfigurationInputBuilder) -> impl Future<Output = Result<ValidateConfigurationOutput, SdkError<ValidateConfigurationError>>> {
        self.deref().validate_configuration(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppConfigClient {}
    impl AppConfigClient for edAppConfigClient {
        async fn create_application(&self, builder: CreateApplicationInputBuilder) -> Result<CreateApplicationOutput, SdkError<CreateApplicationError>>;
        async fn create_configuration_profile(&self, builder: CreateConfigurationProfileInputBuilder) -> Result<CreateConfigurationProfileOutput, SdkError<CreateConfigurationProfileError>>;
        async fn create_deployment_strategy(&self, builder: CreateDeploymentStrategyInputBuilder) -> Result<CreateDeploymentStrategyOutput, SdkError<CreateDeploymentStrategyError>>;
        async fn create_environment(&self, builder: CreateEnvironmentInputBuilder) -> Result<CreateEnvironmentOutput, SdkError<CreateEnvironmentError>>;
        async fn create_extension(&self, builder: CreateExtensionInputBuilder) -> Result<CreateExtensionOutput, SdkError<CreateExtensionError>>;
        async fn create_extension_association(&self, builder: CreateExtensionAssociationInputBuilder) -> Result<CreateExtensionAssociationOutput, SdkError<CreateExtensionAssociationError>>;
        async fn create_hosted_configuration_version(&self, builder: CreateHostedConfigurationVersionInputBuilder) -> Result<CreateHostedConfigurationVersionOutput, SdkError<CreateHostedConfigurationVersionError>>;
        async fn delete_application(&self, builder: DeleteApplicationInputBuilder) -> Result<DeleteApplicationOutput, SdkError<DeleteApplicationError>>;
        async fn delete_configuration_profile(&self, builder: DeleteConfigurationProfileInputBuilder) -> Result<DeleteConfigurationProfileOutput, SdkError<DeleteConfigurationProfileError>>;
        async fn delete_deployment_strategy(&self, builder: DeleteDeploymentStrategyInputBuilder) -> Result<DeleteDeploymentStrategyOutput, SdkError<DeleteDeploymentStrategyError>>;
        async fn delete_environment(&self, builder: DeleteEnvironmentInputBuilder) -> Result<DeleteEnvironmentOutput, SdkError<DeleteEnvironmentError>>;
        async fn delete_extension(&self, builder: DeleteExtensionInputBuilder) -> Result<DeleteExtensionOutput, SdkError<DeleteExtensionError>>;
        async fn delete_extension_association(&self, builder: DeleteExtensionAssociationInputBuilder) -> Result<DeleteExtensionAssociationOutput, SdkError<DeleteExtensionAssociationError>>;
        async fn delete_hosted_configuration_version(&self, builder: DeleteHostedConfigurationVersionInputBuilder) -> Result<DeleteHostedConfigurationVersionOutput, SdkError<DeleteHostedConfigurationVersionError>>;
        async fn get_application(&self, builder: GetApplicationInputBuilder) -> Result<GetApplicationOutput, SdkError<GetApplicationError>>;
        async fn get_configuration_profile(&self, builder: GetConfigurationProfileInputBuilder) -> Result<GetConfigurationProfileOutput, SdkError<GetConfigurationProfileError>>;
        async fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> Result<GetDeploymentOutput, SdkError<GetDeploymentError>>;
        async fn get_deployment_strategy(&self, builder: GetDeploymentStrategyInputBuilder) -> Result<GetDeploymentStrategyOutput, SdkError<GetDeploymentStrategyError>>;
        async fn get_environment(&self, builder: GetEnvironmentInputBuilder) -> Result<GetEnvironmentOutput, SdkError<GetEnvironmentError>>;
        async fn get_extension(&self, builder: GetExtensionInputBuilder) -> Result<GetExtensionOutput, SdkError<GetExtensionError>>;
        async fn get_extension_association(&self, builder: GetExtensionAssociationInputBuilder) -> Result<GetExtensionAssociationOutput, SdkError<GetExtensionAssociationError>>;
        async fn get_hosted_configuration_version(&self, builder: GetHostedConfigurationVersionInputBuilder) -> Result<GetHostedConfigurationVersionOutput, SdkError<GetHostedConfigurationVersionError>>;
        async fn list_applications(&self, builder: ListApplicationsInputBuilder) -> Result<ListApplicationsOutput, SdkError<ListApplicationsError>>;
        async fn list_configuration_profiles(&self, builder: ListConfigurationProfilesInputBuilder) -> Result<ListConfigurationProfilesOutput, SdkError<ListConfigurationProfilesError>>;
        async fn list_deployment_strategies(&self, builder: ListDeploymentStrategiesInputBuilder) -> Result<ListDeploymentStrategiesOutput, SdkError<ListDeploymentStrategiesError>>;
        async fn list_deployments(&self, builder: ListDeploymentsInputBuilder) -> Result<ListDeploymentsOutput, SdkError<ListDeploymentsError>>;
        async fn list_environments(&self, builder: ListEnvironmentsInputBuilder) -> Result<ListEnvironmentsOutput, SdkError<ListEnvironmentsError>>;
        async fn list_extension_associations(&self, builder: ListExtensionAssociationsInputBuilder) -> Result<ListExtensionAssociationsOutput, SdkError<ListExtensionAssociationsError>>;
        async fn list_extensions(&self, builder: ListExtensionsInputBuilder) -> Result<ListExtensionsOutput, SdkError<ListExtensionsError>>;
        async fn list_hosted_configuration_versions(&self, builder: ListHostedConfigurationVersionsInputBuilder) -> Result<ListHostedConfigurationVersionsOutput, SdkError<ListHostedConfigurationVersionsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> Result<StartDeploymentOutput, SdkError<StartDeploymentError>>;
        async fn stop_deployment(&self, builder: StopDeploymentInputBuilder) -> Result<StopDeploymentOutput, SdkError<StopDeploymentError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_application(&self, builder: UpdateApplicationInputBuilder) -> Result<UpdateApplicationOutput, SdkError<UpdateApplicationError>>;
        async fn update_configuration_profile(&self, builder: UpdateConfigurationProfileInputBuilder) -> Result<UpdateConfigurationProfileOutput, SdkError<UpdateConfigurationProfileError>>;
        async fn update_deployment_strategy(&self, builder: UpdateDeploymentStrategyInputBuilder) -> Result<UpdateDeploymentStrategyOutput, SdkError<UpdateDeploymentStrategyError>>;
        async fn update_environment(&self, builder: UpdateEnvironmentInputBuilder) -> Result<UpdateEnvironmentOutput, SdkError<UpdateEnvironmentError>>;
        async fn update_extension(&self, builder: UpdateExtensionInputBuilder) -> Result<UpdateExtensionOutput, SdkError<UpdateExtensionError>>;
        async fn update_extension_association(&self, builder: UpdateExtensionAssociationInputBuilder) -> Result<UpdateExtensionAssociationOutput, SdkError<UpdateExtensionAssociationError>>;
        async fn validate_configuration(&self, builder: ValidateConfigurationInputBuilder) -> Result<ValidateConfigurationOutput, SdkError<ValidateConfigurationError>>;
    }
}
