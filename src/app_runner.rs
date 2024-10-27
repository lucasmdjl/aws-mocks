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
use aws_sdk_apprunner::operation::associate_custom_domain::{builders::*, *};
use aws_sdk_apprunner::operation::create_auto_scaling_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::create_connection::{builders::*, *};
use aws_sdk_apprunner::operation::create_observability_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::create_service::{builders::*, *};
use aws_sdk_apprunner::operation::create_vpc_connector::{builders::*, *};
use aws_sdk_apprunner::operation::create_vpc_ingress_connection::{builders::*, *};
use aws_sdk_apprunner::operation::delete_auto_scaling_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::delete_connection::{builders::*, *};
use aws_sdk_apprunner::operation::delete_observability_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::delete_service::{builders::*, *};
use aws_sdk_apprunner::operation::delete_vpc_connector::{builders::*, *};
use aws_sdk_apprunner::operation::delete_vpc_ingress_connection::{builders::*, *};
use aws_sdk_apprunner::operation::describe_auto_scaling_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::describe_custom_domains::{builders::*, *};
use aws_sdk_apprunner::operation::describe_observability_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::describe_service::{builders::*, *};
use aws_sdk_apprunner::operation::describe_vpc_connector::{builders::*, *};
use aws_sdk_apprunner::operation::describe_vpc_ingress_connection::{builders::*, *};
use aws_sdk_apprunner::operation::disassociate_custom_domain::{builders::*, *};
use aws_sdk_apprunner::operation::list_auto_scaling_configurations::{builders::*, *};
use aws_sdk_apprunner::operation::list_connections::{builders::*, *};
use aws_sdk_apprunner::operation::list_observability_configurations::{builders::*, *};
use aws_sdk_apprunner::operation::list_operations::{builders::*, *};
use aws_sdk_apprunner::operation::list_services::{builders::*, *};
use aws_sdk_apprunner::operation::list_services_for_auto_scaling_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_apprunner::operation::list_vpc_connectors::{builders::*, *};
use aws_sdk_apprunner::operation::list_vpc_ingress_connections::{builders::*, *};
use aws_sdk_apprunner::operation::pause_service::{builders::*, *};
use aws_sdk_apprunner::operation::resume_service::{builders::*, *};
use aws_sdk_apprunner::operation::start_deployment::{builders::*, *};
use aws_sdk_apprunner::operation::tag_resource::{builders::*, *};
use aws_sdk_apprunner::operation::untag_resource::{builders::*, *};
use aws_sdk_apprunner::operation::update_default_auto_scaling_configuration::{builders::*, *};
use aws_sdk_apprunner::operation::update_service::{builders::*, *};
use aws_sdk_apprunner::operation::update_vpc_ingress_connection::{builders::*, *};
use aws_sdk_apprunner::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_apprunner::Client;
use std::ops::Deref;

pub use aws_sdk_apprunner::*;

pub struct AppRunnerClientImpl(Client);
impl AppRunnerClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppRunnerClient {
    fn associate_custom_domain(&self, builder: AssociateCustomDomainInputBuilder) -> impl Future<Output = Result<AssociateCustomDomainOutput, SdkError<AssociateCustomDomainError>>> + Send;
    fn create_auto_scaling_configuration(&self, builder: CreateAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<CreateAutoScalingConfigurationOutput, SdkError<CreateAutoScalingConfigurationError>>> + Send;
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> + Send;
    fn create_observability_configuration(&self, builder: CreateObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<CreateObservabilityConfigurationOutput, SdkError<CreateObservabilityConfigurationError>>> + Send;
    fn create_service(&self, builder: CreateServiceInputBuilder) -> impl Future<Output = Result<CreateServiceOutput, SdkError<CreateServiceError>>> + Send;
    fn create_vpc_connector(&self, builder: CreateVpcConnectorInputBuilder) -> impl Future<Output = Result<CreateVpcConnectorOutput, SdkError<CreateVpcConnectorError>>> + Send;
    fn create_vpc_ingress_connection(&self, builder: CreateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcIngressConnectionOutput, SdkError<CreateVpcIngressConnectionError>>> + Send;
    fn delete_auto_scaling_configuration(&self, builder: DeleteAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteAutoScalingConfigurationOutput, SdkError<DeleteAutoScalingConfigurationError>>> + Send;
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> + Send;
    fn delete_observability_configuration(&self, builder: DeleteObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteObservabilityConfigurationOutput, SdkError<DeleteObservabilityConfigurationError>>> + Send;
    fn delete_service(&self, builder: DeleteServiceInputBuilder) -> impl Future<Output = Result<DeleteServiceOutput, SdkError<DeleteServiceError>>> + Send;
    fn delete_vpc_connector(&self, builder: DeleteVpcConnectorInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectorOutput, SdkError<DeleteVpcConnectorError>>> + Send;
    fn delete_vpc_ingress_connection(&self, builder: DeleteVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcIngressConnectionOutput, SdkError<DeleteVpcIngressConnectionError>>> + Send;
    fn describe_auto_scaling_configuration(&self, builder: DescribeAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAutoScalingConfigurationOutput, SdkError<DescribeAutoScalingConfigurationError>>> + Send;
    fn describe_custom_domains(&self, builder: DescribeCustomDomainsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainsOutput, SdkError<DescribeCustomDomainsError>>> + Send;
    fn describe_observability_configuration(&self, builder: DescribeObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeObservabilityConfigurationOutput, SdkError<DescribeObservabilityConfigurationError>>> + Send;
    fn describe_service(&self, builder: DescribeServiceInputBuilder) -> impl Future<Output = Result<DescribeServiceOutput, SdkError<DescribeServiceError>>> + Send;
    fn describe_vpc_connector(&self, builder: DescribeVpcConnectorInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectorOutput, SdkError<DescribeVpcConnectorError>>> + Send;
    fn describe_vpc_ingress_connection(&self, builder: DescribeVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcIngressConnectionOutput, SdkError<DescribeVpcIngressConnectionError>>> + Send;
    fn disassociate_custom_domain(&self, builder: DisassociateCustomDomainInputBuilder) -> impl Future<Output = Result<DisassociateCustomDomainOutput, SdkError<DisassociateCustomDomainError>>> + Send;
    fn list_auto_scaling_configurations(&self, builder: ListAutoScalingConfigurationsInputBuilder) -> impl Future<Output = Result<ListAutoScalingConfigurationsOutput, SdkError<ListAutoScalingConfigurationsError>>> + Send;
    fn list_connections(&self, builder: ListConnectionsInputBuilder) -> impl Future<Output = Result<ListConnectionsOutput, SdkError<ListConnectionsError>>> + Send;
    fn list_observability_configurations(&self, builder: ListObservabilityConfigurationsInputBuilder) -> impl Future<Output = Result<ListObservabilityConfigurationsOutput, SdkError<ListObservabilityConfigurationsError>>> + Send;
    fn list_operations(&self, builder: ListOperationsInputBuilder) -> impl Future<Output = Result<ListOperationsOutput, SdkError<ListOperationsError>>> + Send;
    fn list_services(&self, builder: ListServicesInputBuilder) -> impl Future<Output = Result<ListServicesOutput, SdkError<ListServicesError>>> + Send;
    fn list_services_for_auto_scaling_configuration(&self, builder: ListServicesForAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<ListServicesForAutoScalingConfigurationOutput, SdkError<ListServicesForAutoScalingConfigurationError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_vpc_connectors(&self, builder: ListVpcConnectorsInputBuilder) -> impl Future<Output = Result<ListVpcConnectorsOutput, SdkError<ListVpcConnectorsError>>> + Send;
    fn list_vpc_ingress_connections(&self, builder: ListVpcIngressConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcIngressConnectionsOutput, SdkError<ListVpcIngressConnectionsError>>> + Send;
    fn pause_service(&self, builder: PauseServiceInputBuilder) -> impl Future<Output = Result<PauseServiceOutput, SdkError<PauseServiceError>>> + Send;
    fn resume_service(&self, builder: ResumeServiceInputBuilder) -> impl Future<Output = Result<ResumeServiceOutput, SdkError<ResumeServiceError>>> + Send;
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_default_auto_scaling_configuration(&self, builder: UpdateDefaultAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateDefaultAutoScalingConfigurationOutput, SdkError<UpdateDefaultAutoScalingConfigurationError>>> + Send;
    fn update_service(&self, builder: UpdateServiceInputBuilder) -> impl Future<Output = Result<UpdateServiceOutput, SdkError<UpdateServiceError>>> + Send;
    fn update_vpc_ingress_connection(&self, builder: UpdateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcIngressConnectionOutput, SdkError<UpdateVpcIngressConnectionError>>> + Send;
}
impl AppRunnerClient for AppRunnerClientImpl {
    fn associate_custom_domain(&self, builder: AssociateCustomDomainInputBuilder) -> impl Future<Output = Result<AssociateCustomDomainOutput, SdkError<AssociateCustomDomainError>>> {
        builder.send_with(&self.0)
    }
    fn create_auto_scaling_configuration(&self, builder: CreateAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<CreateAutoScalingConfigurationOutput, SdkError<CreateAutoScalingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_observability_configuration(&self, builder: CreateObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<CreateObservabilityConfigurationOutput, SdkError<CreateObservabilityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_service(&self, builder: CreateServiceInputBuilder) -> impl Future<Output = Result<CreateServiceOutput, SdkError<CreateServiceError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_connector(&self, builder: CreateVpcConnectorInputBuilder) -> impl Future<Output = Result<CreateVpcConnectorOutput, SdkError<CreateVpcConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_ingress_connection(&self, builder: CreateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcIngressConnectionOutput, SdkError<CreateVpcIngressConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_auto_scaling_configuration(&self, builder: DeleteAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteAutoScalingConfigurationOutput, SdkError<DeleteAutoScalingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_observability_configuration(&self, builder: DeleteObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteObservabilityConfigurationOutput, SdkError<DeleteObservabilityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_service(&self, builder: DeleteServiceInputBuilder) -> impl Future<Output = Result<DeleteServiceOutput, SdkError<DeleteServiceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_connector(&self, builder: DeleteVpcConnectorInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectorOutput, SdkError<DeleteVpcConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_ingress_connection(&self, builder: DeleteVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcIngressConnectionOutput, SdkError<DeleteVpcIngressConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_auto_scaling_configuration(&self, builder: DescribeAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAutoScalingConfigurationOutput, SdkError<DescribeAutoScalingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_custom_domains(&self, builder: DescribeCustomDomainsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainsOutput, SdkError<DescribeCustomDomainsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_observability_configuration(&self, builder: DescribeObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeObservabilityConfigurationOutput, SdkError<DescribeObservabilityConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_service(&self, builder: DescribeServiceInputBuilder) -> impl Future<Output = Result<DescribeServiceOutput, SdkError<DescribeServiceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_connector(&self, builder: DescribeVpcConnectorInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectorOutput, SdkError<DescribeVpcConnectorError>>> {
        builder.send_with(&self.0)
    }
    fn describe_vpc_ingress_connection(&self, builder: DescribeVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcIngressConnectionOutput, SdkError<DescribeVpcIngressConnectionError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_custom_domain(&self, builder: DisassociateCustomDomainInputBuilder) -> impl Future<Output = Result<DisassociateCustomDomainOutput, SdkError<DisassociateCustomDomainError>>> {
        builder.send_with(&self.0)
    }
    fn list_auto_scaling_configurations(&self, builder: ListAutoScalingConfigurationsInputBuilder) -> impl Future<Output = Result<ListAutoScalingConfigurationsOutput, SdkError<ListAutoScalingConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_connections(&self, builder: ListConnectionsInputBuilder) -> impl Future<Output = Result<ListConnectionsOutput, SdkError<ListConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_observability_configurations(&self, builder: ListObservabilityConfigurationsInputBuilder) -> impl Future<Output = Result<ListObservabilityConfigurationsOutput, SdkError<ListObservabilityConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_operations(&self, builder: ListOperationsInputBuilder) -> impl Future<Output = Result<ListOperationsOutput, SdkError<ListOperationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_services(&self, builder: ListServicesInputBuilder) -> impl Future<Output = Result<ListServicesOutput, SdkError<ListServicesError>>> {
        builder.send_with(&self.0)
    }
    fn list_services_for_auto_scaling_configuration(&self, builder: ListServicesForAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<ListServicesForAutoScalingConfigurationOutput, SdkError<ListServicesForAutoScalingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_connectors(&self, builder: ListVpcConnectorsInputBuilder) -> impl Future<Output = Result<ListVpcConnectorsOutput, SdkError<ListVpcConnectorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_vpc_ingress_connections(&self, builder: ListVpcIngressConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcIngressConnectionsOutput, SdkError<ListVpcIngressConnectionsError>>> {
        builder.send_with(&self.0)
    }
    fn pause_service(&self, builder: PauseServiceInputBuilder) -> impl Future<Output = Result<PauseServiceOutput, SdkError<PauseServiceError>>> {
        builder.send_with(&self.0)
    }
    fn resume_service(&self, builder: ResumeServiceInputBuilder) -> impl Future<Output = Result<ResumeServiceOutput, SdkError<ResumeServiceError>>> {
        builder.send_with(&self.0)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_default_auto_scaling_configuration(&self, builder: UpdateDefaultAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateDefaultAutoScalingConfigurationOutput, SdkError<UpdateDefaultAutoScalingConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_service(&self, builder: UpdateServiceInputBuilder) -> impl Future<Output = Result<UpdateServiceOutput, SdkError<UpdateServiceError>>> {
        builder.send_with(&self.0)
    }
    fn update_vpc_ingress_connection(&self, builder: UpdateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcIngressConnectionOutput, SdkError<UpdateVpcIngressConnectionError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppRunnerClient for T
where T: Deref,
      T::Target: AppRunnerClient {
    fn associate_custom_domain(&self, builder: AssociateCustomDomainInputBuilder) -> impl Future<Output = Result<AssociateCustomDomainOutput, SdkError<AssociateCustomDomainError>>> {
        self.deref().associate_custom_domain(builder)
    }
    fn create_auto_scaling_configuration(&self, builder: CreateAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<CreateAutoScalingConfigurationOutput, SdkError<CreateAutoScalingConfigurationError>>> {
        self.deref().create_auto_scaling_configuration(builder)
    }
    fn create_connection(&self, builder: CreateConnectionInputBuilder) -> impl Future<Output = Result<CreateConnectionOutput, SdkError<CreateConnectionError>>> {
        self.deref().create_connection(builder)
    }
    fn create_observability_configuration(&self, builder: CreateObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<CreateObservabilityConfigurationOutput, SdkError<CreateObservabilityConfigurationError>>> {
        self.deref().create_observability_configuration(builder)
    }
    fn create_service(&self, builder: CreateServiceInputBuilder) -> impl Future<Output = Result<CreateServiceOutput, SdkError<CreateServiceError>>> {
        self.deref().create_service(builder)
    }
    fn create_vpc_connector(&self, builder: CreateVpcConnectorInputBuilder) -> impl Future<Output = Result<CreateVpcConnectorOutput, SdkError<CreateVpcConnectorError>>> {
        self.deref().create_vpc_connector(builder)
    }
    fn create_vpc_ingress_connection(&self, builder: CreateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<CreateVpcIngressConnectionOutput, SdkError<CreateVpcIngressConnectionError>>> {
        self.deref().create_vpc_ingress_connection(builder)
    }
    fn delete_auto_scaling_configuration(&self, builder: DeleteAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DeleteAutoScalingConfigurationOutput, SdkError<DeleteAutoScalingConfigurationError>>> {
        self.deref().delete_auto_scaling_configuration(builder)
    }
    fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> impl Future<Output = Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>> {
        self.deref().delete_connection(builder)
    }
    fn delete_observability_configuration(&self, builder: DeleteObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DeleteObservabilityConfigurationOutput, SdkError<DeleteObservabilityConfigurationError>>> {
        self.deref().delete_observability_configuration(builder)
    }
    fn delete_service(&self, builder: DeleteServiceInputBuilder) -> impl Future<Output = Result<DeleteServiceOutput, SdkError<DeleteServiceError>>> {
        self.deref().delete_service(builder)
    }
    fn delete_vpc_connector(&self, builder: DeleteVpcConnectorInputBuilder) -> impl Future<Output = Result<DeleteVpcConnectorOutput, SdkError<DeleteVpcConnectorError>>> {
        self.deref().delete_vpc_connector(builder)
    }
    fn delete_vpc_ingress_connection(&self, builder: DeleteVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DeleteVpcIngressConnectionOutput, SdkError<DeleteVpcIngressConnectionError>>> {
        self.deref().delete_vpc_ingress_connection(builder)
    }
    fn describe_auto_scaling_configuration(&self, builder: DescribeAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<DescribeAutoScalingConfigurationOutput, SdkError<DescribeAutoScalingConfigurationError>>> {
        self.deref().describe_auto_scaling_configuration(builder)
    }
    fn describe_custom_domains(&self, builder: DescribeCustomDomainsInputBuilder) -> impl Future<Output = Result<DescribeCustomDomainsOutput, SdkError<DescribeCustomDomainsError>>> {
        self.deref().describe_custom_domains(builder)
    }
    fn describe_observability_configuration(&self, builder: DescribeObservabilityConfigurationInputBuilder) -> impl Future<Output = Result<DescribeObservabilityConfigurationOutput, SdkError<DescribeObservabilityConfigurationError>>> {
        self.deref().describe_observability_configuration(builder)
    }
    fn describe_service(&self, builder: DescribeServiceInputBuilder) -> impl Future<Output = Result<DescribeServiceOutput, SdkError<DescribeServiceError>>> {
        self.deref().describe_service(builder)
    }
    fn describe_vpc_connector(&self, builder: DescribeVpcConnectorInputBuilder) -> impl Future<Output = Result<DescribeVpcConnectorOutput, SdkError<DescribeVpcConnectorError>>> {
        self.deref().describe_vpc_connector(builder)
    }
    fn describe_vpc_ingress_connection(&self, builder: DescribeVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<DescribeVpcIngressConnectionOutput, SdkError<DescribeVpcIngressConnectionError>>> {
        self.deref().describe_vpc_ingress_connection(builder)
    }
    fn disassociate_custom_domain(&self, builder: DisassociateCustomDomainInputBuilder) -> impl Future<Output = Result<DisassociateCustomDomainOutput, SdkError<DisassociateCustomDomainError>>> {
        self.deref().disassociate_custom_domain(builder)
    }
    fn list_auto_scaling_configurations(&self, builder: ListAutoScalingConfigurationsInputBuilder) -> impl Future<Output = Result<ListAutoScalingConfigurationsOutput, SdkError<ListAutoScalingConfigurationsError>>> {
        self.deref().list_auto_scaling_configurations(builder)
    }
    fn list_connections(&self, builder: ListConnectionsInputBuilder) -> impl Future<Output = Result<ListConnectionsOutput, SdkError<ListConnectionsError>>> {
        self.deref().list_connections(builder)
    }
    fn list_observability_configurations(&self, builder: ListObservabilityConfigurationsInputBuilder) -> impl Future<Output = Result<ListObservabilityConfigurationsOutput, SdkError<ListObservabilityConfigurationsError>>> {
        self.deref().list_observability_configurations(builder)
    }
    fn list_operations(&self, builder: ListOperationsInputBuilder) -> impl Future<Output = Result<ListOperationsOutput, SdkError<ListOperationsError>>> {
        self.deref().list_operations(builder)
    }
    fn list_services(&self, builder: ListServicesInputBuilder) -> impl Future<Output = Result<ListServicesOutput, SdkError<ListServicesError>>> {
        self.deref().list_services(builder)
    }
    fn list_services_for_auto_scaling_configuration(&self, builder: ListServicesForAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<ListServicesForAutoScalingConfigurationOutput, SdkError<ListServicesForAutoScalingConfigurationError>>> {
        self.deref().list_services_for_auto_scaling_configuration(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_vpc_connectors(&self, builder: ListVpcConnectorsInputBuilder) -> impl Future<Output = Result<ListVpcConnectorsOutput, SdkError<ListVpcConnectorsError>>> {
        self.deref().list_vpc_connectors(builder)
    }
    fn list_vpc_ingress_connections(&self, builder: ListVpcIngressConnectionsInputBuilder) -> impl Future<Output = Result<ListVpcIngressConnectionsOutput, SdkError<ListVpcIngressConnectionsError>>> {
        self.deref().list_vpc_ingress_connections(builder)
    }
    fn pause_service(&self, builder: PauseServiceInputBuilder) -> impl Future<Output = Result<PauseServiceOutput, SdkError<PauseServiceError>>> {
        self.deref().pause_service(builder)
    }
    fn resume_service(&self, builder: ResumeServiceInputBuilder) -> impl Future<Output = Result<ResumeServiceOutput, SdkError<ResumeServiceError>>> {
        self.deref().resume_service(builder)
    }
    fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> impl Future<Output = Result<StartDeploymentOutput, SdkError<StartDeploymentError>>> {
        self.deref().start_deployment(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_default_auto_scaling_configuration(&self, builder: UpdateDefaultAutoScalingConfigurationInputBuilder) -> impl Future<Output = Result<UpdateDefaultAutoScalingConfigurationOutput, SdkError<UpdateDefaultAutoScalingConfigurationError>>> {
        self.deref().update_default_auto_scaling_configuration(builder)
    }
    fn update_service(&self, builder: UpdateServiceInputBuilder) -> impl Future<Output = Result<UpdateServiceOutput, SdkError<UpdateServiceError>>> {
        self.deref().update_service(builder)
    }
    fn update_vpc_ingress_connection(&self, builder: UpdateVpcIngressConnectionInputBuilder) -> impl Future<Output = Result<UpdateVpcIngressConnectionOutput, SdkError<UpdateVpcIngressConnectionError>>> {
        self.deref().update_vpc_ingress_connection(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppRunnerClient {}
    impl AppRunnerClient for edAppRunnerClient {
        async fn associate_custom_domain(&self, builder: AssociateCustomDomainInputBuilder) -> Result<AssociateCustomDomainOutput, SdkError<AssociateCustomDomainError>>;
        async fn create_auto_scaling_configuration(&self, builder: CreateAutoScalingConfigurationInputBuilder) -> Result<CreateAutoScalingConfigurationOutput, SdkError<CreateAutoScalingConfigurationError>>;
        async fn create_connection(&self, builder: CreateConnectionInputBuilder) -> Result<CreateConnectionOutput, SdkError<CreateConnectionError>>;
        async fn create_observability_configuration(&self, builder: CreateObservabilityConfigurationInputBuilder) -> Result<CreateObservabilityConfigurationOutput, SdkError<CreateObservabilityConfigurationError>>;
        async fn create_service(&self, builder: CreateServiceInputBuilder) -> Result<CreateServiceOutput, SdkError<CreateServiceError>>;
        async fn create_vpc_connector(&self, builder: CreateVpcConnectorInputBuilder) -> Result<CreateVpcConnectorOutput, SdkError<CreateVpcConnectorError>>;
        async fn create_vpc_ingress_connection(&self, builder: CreateVpcIngressConnectionInputBuilder) -> Result<CreateVpcIngressConnectionOutput, SdkError<CreateVpcIngressConnectionError>>;
        async fn delete_auto_scaling_configuration(&self, builder: DeleteAutoScalingConfigurationInputBuilder) -> Result<DeleteAutoScalingConfigurationOutput, SdkError<DeleteAutoScalingConfigurationError>>;
        async fn delete_connection(&self, builder: DeleteConnectionInputBuilder) -> Result<DeleteConnectionOutput, SdkError<DeleteConnectionError>>;
        async fn delete_observability_configuration(&self, builder: DeleteObservabilityConfigurationInputBuilder) -> Result<DeleteObservabilityConfigurationOutput, SdkError<DeleteObservabilityConfigurationError>>;
        async fn delete_service(&self, builder: DeleteServiceInputBuilder) -> Result<DeleteServiceOutput, SdkError<DeleteServiceError>>;
        async fn delete_vpc_connector(&self, builder: DeleteVpcConnectorInputBuilder) -> Result<DeleteVpcConnectorOutput, SdkError<DeleteVpcConnectorError>>;
        async fn delete_vpc_ingress_connection(&self, builder: DeleteVpcIngressConnectionInputBuilder) -> Result<DeleteVpcIngressConnectionOutput, SdkError<DeleteVpcIngressConnectionError>>;
        async fn describe_auto_scaling_configuration(&self, builder: DescribeAutoScalingConfigurationInputBuilder) -> Result<DescribeAutoScalingConfigurationOutput, SdkError<DescribeAutoScalingConfigurationError>>;
        async fn describe_custom_domains(&self, builder: DescribeCustomDomainsInputBuilder) -> Result<DescribeCustomDomainsOutput, SdkError<DescribeCustomDomainsError>>;
        async fn describe_observability_configuration(&self, builder: DescribeObservabilityConfigurationInputBuilder) -> Result<DescribeObservabilityConfigurationOutput, SdkError<DescribeObservabilityConfigurationError>>;
        async fn describe_service(&self, builder: DescribeServiceInputBuilder) -> Result<DescribeServiceOutput, SdkError<DescribeServiceError>>;
        async fn describe_vpc_connector(&self, builder: DescribeVpcConnectorInputBuilder) -> Result<DescribeVpcConnectorOutput, SdkError<DescribeVpcConnectorError>>;
        async fn describe_vpc_ingress_connection(&self, builder: DescribeVpcIngressConnectionInputBuilder) -> Result<DescribeVpcIngressConnectionOutput, SdkError<DescribeVpcIngressConnectionError>>;
        async fn disassociate_custom_domain(&self, builder: DisassociateCustomDomainInputBuilder) -> Result<DisassociateCustomDomainOutput, SdkError<DisassociateCustomDomainError>>;
        async fn list_auto_scaling_configurations(&self, builder: ListAutoScalingConfigurationsInputBuilder) -> Result<ListAutoScalingConfigurationsOutput, SdkError<ListAutoScalingConfigurationsError>>;
        async fn list_connections(&self, builder: ListConnectionsInputBuilder) -> Result<ListConnectionsOutput, SdkError<ListConnectionsError>>;
        async fn list_observability_configurations(&self, builder: ListObservabilityConfigurationsInputBuilder) -> Result<ListObservabilityConfigurationsOutput, SdkError<ListObservabilityConfigurationsError>>;
        async fn list_operations(&self, builder: ListOperationsInputBuilder) -> Result<ListOperationsOutput, SdkError<ListOperationsError>>;
        async fn list_services(&self, builder: ListServicesInputBuilder) -> Result<ListServicesOutput, SdkError<ListServicesError>>;
        async fn list_services_for_auto_scaling_configuration(&self, builder: ListServicesForAutoScalingConfigurationInputBuilder) -> Result<ListServicesForAutoScalingConfigurationOutput, SdkError<ListServicesForAutoScalingConfigurationError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_vpc_connectors(&self, builder: ListVpcConnectorsInputBuilder) -> Result<ListVpcConnectorsOutput, SdkError<ListVpcConnectorsError>>;
        async fn list_vpc_ingress_connections(&self, builder: ListVpcIngressConnectionsInputBuilder) -> Result<ListVpcIngressConnectionsOutput, SdkError<ListVpcIngressConnectionsError>>;
        async fn pause_service(&self, builder: PauseServiceInputBuilder) -> Result<PauseServiceOutput, SdkError<PauseServiceError>>;
        async fn resume_service(&self, builder: ResumeServiceInputBuilder) -> Result<ResumeServiceOutput, SdkError<ResumeServiceError>>;
        async fn start_deployment(&self, builder: StartDeploymentInputBuilder) -> Result<StartDeploymentOutput, SdkError<StartDeploymentError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_default_auto_scaling_configuration(&self, builder: UpdateDefaultAutoScalingConfigurationInputBuilder) -> Result<UpdateDefaultAutoScalingConfigurationOutput, SdkError<UpdateDefaultAutoScalingConfigurationError>>;
        async fn update_service(&self, builder: UpdateServiceInputBuilder) -> Result<UpdateServiceOutput, SdkError<UpdateServiceError>>;
        async fn update_vpc_ingress_connection(&self, builder: UpdateVpcIngressConnectionInputBuilder) -> Result<UpdateVpcIngressConnectionOutput, SdkError<UpdateVpcIngressConnectionError>>;
    }
}
