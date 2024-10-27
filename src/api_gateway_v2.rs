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
use aws_sdk_apigatewayv2::operation::create_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_api_mapping::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_authorizer::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_deployment::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_domain_name::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_integration::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_integration_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_model::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_route::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_route_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_stage::{builders::*, *};
use aws_sdk_apigatewayv2::operation::create_vpc_link::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_access_log_settings::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_api_mapping::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_authorizer::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_cors_configuration::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_deployment::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_domain_name::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_integration::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_integration_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_model::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_route::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_route_request_parameter::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_route_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_route_settings::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_stage::{builders::*, *};
use aws_sdk_apigatewayv2::operation::delete_vpc_link::{builders::*, *};
use aws_sdk_apigatewayv2::operation::export_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_api_mapping::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_api_mappings::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_apis::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_authorizer::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_authorizers::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_deployment::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_deployments::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_domain_name::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_domain_names::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_integration::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_integration_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_integration_responses::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_integrations::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_model::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_model_template::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_models::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_route::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_route_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_route_responses::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_routes::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_stage::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_stages::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_tags::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_vpc_link::{builders::*, *};
use aws_sdk_apigatewayv2::operation::get_vpc_links::{builders::*, *};
use aws_sdk_apigatewayv2::operation::import_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::reimport_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::reset_authorizers_cache::{builders::*, *};
use aws_sdk_apigatewayv2::operation::tag_resource::{builders::*, *};
use aws_sdk_apigatewayv2::operation::untag_resource::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_api::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_api_mapping::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_authorizer::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_deployment::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_domain_name::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_integration::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_integration_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_model::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_route::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_route_response::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_stage::{builders::*, *};
use aws_sdk_apigatewayv2::operation::update_vpc_link::{builders::*, *};
use aws_sdk_apigatewayv2::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_apigatewayv2::Client;
use std::ops::Deref;

pub use aws_sdk_apigatewayv2::*;

pub struct ApiGatewayV2ClientImpl(Client);
impl ApiGatewayV2ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ApiGatewayV2Client {
    fn create_api(&self, builder: CreateApiInputBuilder) -> impl Future<Output = Result<CreateApiOutput, SdkError<CreateApiError>>> + Send;
    fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> impl Future<Output = Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>> + Send;
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> + Send;
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> + Send;
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> + Send;
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> + Send;
    fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> impl Future<Output = Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>> + Send;
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> + Send;
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> + Send;
    fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> impl Future<Output = Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>> + Send;
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> + Send;
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> + Send;
    fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> impl Future<Output = Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>> + Send;
    fn delete_api(&self, builder: DeleteApiInputBuilder) -> impl Future<Output = Result<DeleteApiOutput, SdkError<DeleteApiError>>> + Send;
    fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> impl Future<Output = Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>> + Send;
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> + Send;
    fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>> + Send;
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> + Send;
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> + Send;
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> + Send;
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> + Send;
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> + Send;
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> + Send;
    fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> impl Future<Output = Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>> + Send;
    fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> impl Future<Output = Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>> + Send;
    fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> impl Future<Output = Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>> + Send;
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> + Send;
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> + Send;
    fn export_api(&self, builder: ExportApiInputBuilder) -> impl Future<Output = Result<ExportApiOutput, SdkError<ExportApiError>>> + Send;
    fn get_api(&self, builder: GetApiInputBuilder) -> impl Future<Output = Result<GetApiOutput, SdkError<GetApiError>>> + Send;
    fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> impl Future<Output = Result<GetApiMappingOutput, SdkError<GetApiMappingError>>> + Send;
    fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> impl Future<Output = Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>> + Send;
    fn get_apis(&self, builder: GetApisInputBuilder) -> impl Future<Output = Result<GetApisOutput, SdkError<GetApisError>>> + Send;
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> + Send;
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> + Send;
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> + Send;
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> + Send;
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> + Send;
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> + Send;
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> + Send;
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> + Send;
    fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> impl Future<Output = Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>> + Send;
    fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> impl Future<Output = Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>> + Send;
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>> + Send;
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>> + Send;
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>> + Send;
    fn get_route(&self, builder: GetRouteInputBuilder) -> impl Future<Output = Result<GetRouteOutput, SdkError<GetRouteError>>> + Send;
    fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> impl Future<Output = Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>> + Send;
    fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> impl Future<Output = Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>> + Send;
    fn get_routes(&self, builder: GetRoutesInputBuilder) -> impl Future<Output = Result<GetRoutesOutput, SdkError<GetRoutesError>>> + Send;
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>> + Send;
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>> + Send;
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> + Send;
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> + Send;
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> + Send;
    fn import_api(&self, builder: ImportApiInputBuilder) -> impl Future<Output = Result<ImportApiOutput, SdkError<ImportApiError>>> + Send;
    fn reimport_api(&self, builder: ReimportApiInputBuilder) -> impl Future<Output = Result<ReimportApiOutput, SdkError<ReimportApiError>>> + Send;
    fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> impl Future<Output = Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_api(&self, builder: UpdateApiInputBuilder) -> impl Future<Output = Result<UpdateApiOutput, SdkError<UpdateApiError>>> + Send;
    fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> impl Future<Output = Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>> + Send;
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> + Send;
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> + Send;
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> + Send;
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> + Send;
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> + Send;
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> + Send;
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> + Send;
    fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> impl Future<Output = Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>> + Send;
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> + Send;
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> + Send;
}
impl ApiGatewayV2Client for ApiGatewayV2ClientImpl {
    fn create_api(&self, builder: CreateApiInputBuilder) -> impl Future<Output = Result<CreateApiOutput, SdkError<CreateApiError>>> {
        builder.send_with(&self.0)
    }
    fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> impl Future<Output = Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>> {
        builder.send_with(&self.0)
    }
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> impl Future<Output = Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        builder.send_with(&self.0)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> impl Future<Output = Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>> {
        builder.send_with(&self.0)
    }
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> impl Future<Output = Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_api(&self, builder: DeleteApiInputBuilder) -> impl Future<Output = Result<DeleteApiOutput, SdkError<DeleteApiError>>> {
        builder.send_with(&self.0)
    }
    fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> impl Future<Output = Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> impl Future<Output = Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> impl Future<Output = Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> impl Future<Output = Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn export_api(&self, builder: ExportApiInputBuilder) -> impl Future<Output = Result<ExportApiOutput, SdkError<ExportApiError>>> {
        builder.send_with(&self.0)
    }
    fn get_api(&self, builder: GetApiInputBuilder) -> impl Future<Output = Result<GetApiOutput, SdkError<GetApiError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> impl Future<Output = Result<GetApiMappingOutput, SdkError<GetApiMappingError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> impl Future<Output = Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_apis(&self, builder: GetApisInputBuilder) -> impl Future<Output = Result<GetApisOutput, SdkError<GetApisError>>> {
        builder.send_with(&self.0)
    }
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> {
        builder.send_with(&self.0)
    }
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> impl Future<Output = Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>> {
        builder.send_with(&self.0)
    }
    fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> impl Future<Output = Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>> {
        builder.send_with(&self.0)
    }
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>> {
        builder.send_with(&self.0)
    }
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>> {
        builder.send_with(&self.0)
    }
    fn get_route(&self, builder: GetRouteInputBuilder) -> impl Future<Output = Result<GetRouteOutput, SdkError<GetRouteError>>> {
        builder.send_with(&self.0)
    }
    fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> impl Future<Output = Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>> {
        builder.send_with(&self.0)
    }
    fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> impl Future<Output = Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>> {
        builder.send_with(&self.0)
    }
    fn get_routes(&self, builder: GetRoutesInputBuilder) -> impl Future<Output = Result<GetRoutesOutput, SdkError<GetRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>> {
        builder.send_with(&self.0)
    }
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>> {
        builder.send_with(&self.0)
    }
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> {
        builder.send_with(&self.0)
    }
    fn import_api(&self, builder: ImportApiInputBuilder) -> impl Future<Output = Result<ImportApiOutput, SdkError<ImportApiError>>> {
        builder.send_with(&self.0)
    }
    fn reimport_api(&self, builder: ReimportApiInputBuilder) -> impl Future<Output = Result<ReimportApiOutput, SdkError<ReimportApiError>>> {
        builder.send_with(&self.0)
    }
    fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> impl Future<Output = Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_api(&self, builder: UpdateApiInputBuilder) -> impl Future<Output = Result<UpdateApiOutput, SdkError<UpdateApiError>>> {
        builder.send_with(&self.0)
    }
    fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> impl Future<Output = Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>> {
        builder.send_with(&self.0)
    }
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> {
        builder.send_with(&self.0)
    }
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> {
        builder.send_with(&self.0)
    }
    fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> impl Future<Output = Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>> {
        builder.send_with(&self.0)
    }
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> {
        builder.send_with(&self.0)
    }
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> ApiGatewayV2Client for T
where T: Deref,
      T::Target: ApiGatewayV2Client {
    fn create_api(&self, builder: CreateApiInputBuilder) -> impl Future<Output = Result<CreateApiOutput, SdkError<CreateApiError>>> {
        self.deref().create_api(builder)
    }
    fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> impl Future<Output = Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>> {
        self.deref().create_api_mapping(builder)
    }
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> {
        self.deref().create_authorizer(builder)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        self.deref().create_deployment(builder)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        self.deref().create_domain_name(builder)
    }
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> {
        self.deref().create_integration(builder)
    }
    fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> impl Future<Output = Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>> {
        self.deref().create_integration_response(builder)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        self.deref().create_model(builder)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        self.deref().create_route(builder)
    }
    fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> impl Future<Output = Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>> {
        self.deref().create_route_response(builder)
    }
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> {
        self.deref().create_stage(builder)
    }
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> {
        self.deref().create_vpc_link(builder)
    }
    fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> impl Future<Output = Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>> {
        self.deref().delete_access_log_settings(builder)
    }
    fn delete_api(&self, builder: DeleteApiInputBuilder) -> impl Future<Output = Result<DeleteApiOutput, SdkError<DeleteApiError>>> {
        self.deref().delete_api(builder)
    }
    fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> impl Future<Output = Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>> {
        self.deref().delete_api_mapping(builder)
    }
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> {
        self.deref().delete_authorizer(builder)
    }
    fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>> {
        self.deref().delete_cors_configuration(builder)
    }
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> {
        self.deref().delete_deployment(builder)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        self.deref().delete_domain_name(builder)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        self.deref().delete_integration(builder)
    }
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> {
        self.deref().delete_integration_response(builder)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        self.deref().delete_model(builder)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        self.deref().delete_route(builder)
    }
    fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> impl Future<Output = Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>> {
        self.deref().delete_route_request_parameter(builder)
    }
    fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> impl Future<Output = Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>> {
        self.deref().delete_route_response(builder)
    }
    fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> impl Future<Output = Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>> {
        self.deref().delete_route_settings(builder)
    }
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> {
        self.deref().delete_stage(builder)
    }
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> {
        self.deref().delete_vpc_link(builder)
    }
    fn export_api(&self, builder: ExportApiInputBuilder) -> impl Future<Output = Result<ExportApiOutput, SdkError<ExportApiError>>> {
        self.deref().export_api(builder)
    }
    fn get_api(&self, builder: GetApiInputBuilder) -> impl Future<Output = Result<GetApiOutput, SdkError<GetApiError>>> {
        self.deref().get_api(builder)
    }
    fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> impl Future<Output = Result<GetApiMappingOutput, SdkError<GetApiMappingError>>> {
        self.deref().get_api_mapping(builder)
    }
    fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> impl Future<Output = Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>> {
        self.deref().get_api_mappings(builder)
    }
    fn get_apis(&self, builder: GetApisInputBuilder) -> impl Future<Output = Result<GetApisOutput, SdkError<GetApisError>>> {
        self.deref().get_apis(builder)
    }
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> {
        self.deref().get_authorizer(builder)
    }
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> {
        self.deref().get_authorizers(builder)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        self.deref().get_deployment(builder)
    }
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> {
        self.deref().get_deployments(builder)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        self.deref().get_domain_name(builder)
    }
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> {
        self.deref().get_domain_names(builder)
    }
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> {
        self.deref().get_integration(builder)
    }
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> {
        self.deref().get_integration_response(builder)
    }
    fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> impl Future<Output = Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>> {
        self.deref().get_integration_responses(builder)
    }
    fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> impl Future<Output = Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>> {
        self.deref().get_integrations(builder)
    }
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>> {
        self.deref().get_model(builder)
    }
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>> {
        self.deref().get_model_template(builder)
    }
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>> {
        self.deref().get_models(builder)
    }
    fn get_route(&self, builder: GetRouteInputBuilder) -> impl Future<Output = Result<GetRouteOutput, SdkError<GetRouteError>>> {
        self.deref().get_route(builder)
    }
    fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> impl Future<Output = Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>> {
        self.deref().get_route_response(builder)
    }
    fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> impl Future<Output = Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>> {
        self.deref().get_route_responses(builder)
    }
    fn get_routes(&self, builder: GetRoutesInputBuilder) -> impl Future<Output = Result<GetRoutesOutput, SdkError<GetRoutesError>>> {
        self.deref().get_routes(builder)
    }
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>> {
        self.deref().get_stage(builder)
    }
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>> {
        self.deref().get_stages(builder)
    }
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> {
        self.deref().get_tags(builder)
    }
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> {
        self.deref().get_vpc_link(builder)
    }
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> {
        self.deref().get_vpc_links(builder)
    }
    fn import_api(&self, builder: ImportApiInputBuilder) -> impl Future<Output = Result<ImportApiOutput, SdkError<ImportApiError>>> {
        self.deref().import_api(builder)
    }
    fn reimport_api(&self, builder: ReimportApiInputBuilder) -> impl Future<Output = Result<ReimportApiOutput, SdkError<ReimportApiError>>> {
        self.deref().reimport_api(builder)
    }
    fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> impl Future<Output = Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>> {
        self.deref().reset_authorizers_cache(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_api(&self, builder: UpdateApiInputBuilder) -> impl Future<Output = Result<UpdateApiOutput, SdkError<UpdateApiError>>> {
        self.deref().update_api(builder)
    }
    fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> impl Future<Output = Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>> {
        self.deref().update_api_mapping(builder)
    }
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> {
        self.deref().update_authorizer(builder)
    }
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> {
        self.deref().update_deployment(builder)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        self.deref().update_domain_name(builder)
    }
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> {
        self.deref().update_integration(builder)
    }
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> {
        self.deref().update_integration_response(builder)
    }
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> {
        self.deref().update_model(builder)
    }
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> {
        self.deref().update_route(builder)
    }
    fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> impl Future<Output = Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>> {
        self.deref().update_route_response(builder)
    }
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> {
        self.deref().update_stage(builder)
    }
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> {
        self.deref().update_vpc_link(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edApiGatewayV2Client {}
    impl ApiGatewayV2Client for edApiGatewayV2Client {
        async fn create_api(&self, builder: CreateApiInputBuilder) -> Result<CreateApiOutput, SdkError<CreateApiError>>;
        async fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>;
        async fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>;
        async fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>;
        async fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>;
        async fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>;
        async fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>;
        async fn create_model(&self, builder: CreateModelInputBuilder) -> Result<CreateModelOutput, SdkError<CreateModelError>>;
        async fn create_route(&self, builder: CreateRouteInputBuilder) -> Result<CreateRouteOutput, SdkError<CreateRouteError>>;
        async fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>;
        async fn create_stage(&self, builder: CreateStageInputBuilder) -> Result<CreateStageOutput, SdkError<CreateStageError>>;
        async fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>;
        async fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>;
        async fn delete_api(&self, builder: DeleteApiInputBuilder) -> Result<DeleteApiOutput, SdkError<DeleteApiError>>;
        async fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>;
        async fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>;
        async fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>;
        async fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>;
        async fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>;
        async fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>;
        async fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>;
        async fn delete_model(&self, builder: DeleteModelInputBuilder) -> Result<DeleteModelOutput, SdkError<DeleteModelError>>;
        async fn delete_route(&self, builder: DeleteRouteInputBuilder) -> Result<DeleteRouteOutput, SdkError<DeleteRouteError>>;
        async fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>;
        async fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>;
        async fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>;
        async fn delete_stage(&self, builder: DeleteStageInputBuilder) -> Result<DeleteStageOutput, SdkError<DeleteStageError>>;
        async fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>;
        async fn export_api(&self, builder: ExportApiInputBuilder) -> Result<ExportApiOutput, SdkError<ExportApiError>>;
        async fn get_api(&self, builder: GetApiInputBuilder) -> Result<GetApiOutput, SdkError<GetApiError>>;
        async fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> Result<GetApiMappingOutput, SdkError<GetApiMappingError>>;
        async fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>;
        async fn get_apis(&self, builder: GetApisInputBuilder) -> Result<GetApisOutput, SdkError<GetApisError>>;
        async fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>;
        async fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>;
        async fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> Result<GetDeploymentOutput, SdkError<GetDeploymentError>>;
        async fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>;
        async fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> Result<GetDomainNameOutput, SdkError<GetDomainNameError>>;
        async fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>;
        async fn get_integration(&self, builder: GetIntegrationInputBuilder) -> Result<GetIntegrationOutput, SdkError<GetIntegrationError>>;
        async fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>;
        async fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>;
        async fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>;
        async fn get_model(&self, builder: GetModelInputBuilder) -> Result<GetModelOutput, SdkError<GetModelError>>;
        async fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>;
        async fn get_models(&self, builder: GetModelsInputBuilder) -> Result<GetModelsOutput, SdkError<GetModelsError>>;
        async fn get_route(&self, builder: GetRouteInputBuilder) -> Result<GetRouteOutput, SdkError<GetRouteError>>;
        async fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>;
        async fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>;
        async fn get_routes(&self, builder: GetRoutesInputBuilder) -> Result<GetRoutesOutput, SdkError<GetRoutesError>>;
        async fn get_stage(&self, builder: GetStageInputBuilder) -> Result<GetStageOutput, SdkError<GetStageError>>;
        async fn get_stages(&self, builder: GetStagesInputBuilder) -> Result<GetStagesOutput, SdkError<GetStagesError>>;
        async fn get_tags(&self, builder: GetTagsInputBuilder) -> Result<GetTagsOutput, SdkError<GetTagsError>>;
        async fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>;
        async fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>;
        async fn import_api(&self, builder: ImportApiInputBuilder) -> Result<ImportApiOutput, SdkError<ImportApiError>>;
        async fn reimport_api(&self, builder: ReimportApiInputBuilder) -> Result<ReimportApiOutput, SdkError<ReimportApiError>>;
        async fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_api(&self, builder: UpdateApiInputBuilder) -> Result<UpdateApiOutput, SdkError<UpdateApiError>>;
        async fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>;
        async fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>;
        async fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>;
        async fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>;
        async fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>;
        async fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>;
        async fn update_model(&self, builder: UpdateModelInputBuilder) -> Result<UpdateModelOutput, SdkError<UpdateModelError>>;
        async fn update_route(&self, builder: UpdateRouteInputBuilder) -> Result<UpdateRouteOutput, SdkError<UpdateRouteError>>;
        async fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>;
        async fn update_stage(&self, builder: UpdateStageInputBuilder) -> Result<UpdateStageOutput, SdkError<UpdateStageError>>;
        async fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>;
    }
}
