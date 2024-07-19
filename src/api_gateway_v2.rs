/*
 * aws_mocks - A mocking library for AWS.
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
use aws_sdk_apigatewayv2::Client;

pub use aws_sdk_apigatewayv2::*;

pub struct ApiGatewayV2ClientImpl(Client);
impl ApiGatewayV2ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ApiGatewayV2Client {
    fn create_api(&self, builder: CreateApiInputBuilder) -> impl Future<Output = Result<CreateApiOutput, SdkError<CreateApiError>>>;
    fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> impl Future<Output = Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>>;
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>>;
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>>;
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>>;
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>>;
    fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> impl Future<Output = Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>>;
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>>;
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>>;
    fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> impl Future<Output = Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>>;
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>>;
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>>;
    fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> impl Future<Output = Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>>;
    fn delete_api(&self, builder: DeleteApiInputBuilder) -> impl Future<Output = Result<DeleteApiOutput, SdkError<DeleteApiError>>>;
    fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> impl Future<Output = Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>>;
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>>;
    fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>>;
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>>;
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>>;
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>>;
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>>;
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>>;
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>>;
    fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> impl Future<Output = Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>>;
    fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> impl Future<Output = Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>>;
    fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> impl Future<Output = Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>>;
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>>;
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>>;
    fn export_api(&self, builder: ExportApiInputBuilder) -> impl Future<Output = Result<ExportApiOutput, SdkError<ExportApiError>>>;
    fn get_api(&self, builder: GetApiInputBuilder) -> impl Future<Output = Result<GetApiOutput, SdkError<GetApiError>>>;
    fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> impl Future<Output = Result<GetApiMappingOutput, SdkError<GetApiMappingError>>>;
    fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> impl Future<Output = Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>>;
    fn get_apis(&self, builder: GetApisInputBuilder) -> impl Future<Output = Result<GetApisOutput, SdkError<GetApisError>>>;
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>>;
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>>;
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>>;
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>>;
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>>;
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>>;
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>>;
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>>;
    fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> impl Future<Output = Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>>;
    fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> impl Future<Output = Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>>;
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>>;
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>>;
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>>;
    fn get_route(&self, builder: GetRouteInputBuilder) -> impl Future<Output = Result<GetRouteOutput, SdkError<GetRouteError>>>;
    fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> impl Future<Output = Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>>;
    fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> impl Future<Output = Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>>;
    fn get_routes(&self, builder: GetRoutesInputBuilder) -> impl Future<Output = Result<GetRoutesOutput, SdkError<GetRoutesError>>>;
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>>;
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>>;
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>>;
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>>;
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>>;
    fn import_api(&self, builder: ImportApiInputBuilder) -> impl Future<Output = Result<ImportApiOutput, SdkError<ImportApiError>>>;
    fn reimport_api(&self, builder: ReimportApiInputBuilder) -> impl Future<Output = Result<ReimportApiOutput, SdkError<ReimportApiError>>>;
    fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> impl Future<Output = Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_api(&self, builder: UpdateApiInputBuilder) -> impl Future<Output = Result<UpdateApiOutput, SdkError<UpdateApiError>>>;
    fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> impl Future<Output = Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>>;
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>>;
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>>;
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>>;
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>>;
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>>;
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>>;
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>>;
    fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> impl Future<Output = Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>>;
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>>;
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>>;
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
impl <T: ApiGatewayV2Client> ApiGatewayV2Client for &T {
    fn create_api(&self, builder: CreateApiInputBuilder) -> impl Future<Output = Result<CreateApiOutput, SdkError<CreateApiError>>> {
        (*self).create_api(builder)
    }
    fn create_api_mapping(&self, builder: CreateApiMappingInputBuilder) -> impl Future<Output = Result<CreateApiMappingOutput, SdkError<CreateApiMappingError>>> {
        (*self).create_api_mapping(builder)
    }
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> {
        (*self).create_authorizer(builder)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        (*self).create_deployment(builder)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        (*self).create_domain_name(builder)
    }
    fn create_integration(&self, builder: CreateIntegrationInputBuilder) -> impl Future<Output = Result<CreateIntegrationOutput, SdkError<CreateIntegrationError>>> {
        (*self).create_integration(builder)
    }
    fn create_integration_response(&self, builder: CreateIntegrationResponseInputBuilder) -> impl Future<Output = Result<CreateIntegrationResponseOutput, SdkError<CreateIntegrationResponseError>>> {
        (*self).create_integration_response(builder)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        (*self).create_model(builder)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        (*self).create_route(builder)
    }
    fn create_route_response(&self, builder: CreateRouteResponseInputBuilder) -> impl Future<Output = Result<CreateRouteResponseOutput, SdkError<CreateRouteResponseError>>> {
        (*self).create_route_response(builder)
    }
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> {
        (*self).create_stage(builder)
    }
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> {
        (*self).create_vpc_link(builder)
    }
    fn delete_access_log_settings(&self, builder: DeleteAccessLogSettingsInputBuilder) -> impl Future<Output = Result<DeleteAccessLogSettingsOutput, SdkError<DeleteAccessLogSettingsError>>> {
        (*self).delete_access_log_settings(builder)
    }
    fn delete_api(&self, builder: DeleteApiInputBuilder) -> impl Future<Output = Result<DeleteApiOutput, SdkError<DeleteApiError>>> {
        (*self).delete_api(builder)
    }
    fn delete_api_mapping(&self, builder: DeleteApiMappingInputBuilder) -> impl Future<Output = Result<DeleteApiMappingOutput, SdkError<DeleteApiMappingError>>> {
        (*self).delete_api_mapping(builder)
    }
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> {
        (*self).delete_authorizer(builder)
    }
    fn delete_cors_configuration(&self, builder: DeleteCorsConfigurationInputBuilder) -> impl Future<Output = Result<DeleteCorsConfigurationOutput, SdkError<DeleteCorsConfigurationError>>> {
        (*self).delete_cors_configuration(builder)
    }
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> {
        (*self).delete_deployment(builder)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        (*self).delete_domain_name(builder)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        (*self).delete_integration(builder)
    }
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> {
        (*self).delete_integration_response(builder)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        (*self).delete_model(builder)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        (*self).delete_route(builder)
    }
    fn delete_route_request_parameter(&self, builder: DeleteRouteRequestParameterInputBuilder) -> impl Future<Output = Result<DeleteRouteRequestParameterOutput, SdkError<DeleteRouteRequestParameterError>>> {
        (*self).delete_route_request_parameter(builder)
    }
    fn delete_route_response(&self, builder: DeleteRouteResponseInputBuilder) -> impl Future<Output = Result<DeleteRouteResponseOutput, SdkError<DeleteRouteResponseError>>> {
        (*self).delete_route_response(builder)
    }
    fn delete_route_settings(&self, builder: DeleteRouteSettingsInputBuilder) -> impl Future<Output = Result<DeleteRouteSettingsOutput, SdkError<DeleteRouteSettingsError>>> {
        (*self).delete_route_settings(builder)
    }
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> {
        (*self).delete_stage(builder)
    }
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> {
        (*self).delete_vpc_link(builder)
    }
    fn export_api(&self, builder: ExportApiInputBuilder) -> impl Future<Output = Result<ExportApiOutput, SdkError<ExportApiError>>> {
        (*self).export_api(builder)
    }
    fn get_api(&self, builder: GetApiInputBuilder) -> impl Future<Output = Result<GetApiOutput, SdkError<GetApiError>>> {
        (*self).get_api(builder)
    }
    fn get_api_mapping(&self, builder: GetApiMappingInputBuilder) -> impl Future<Output = Result<GetApiMappingOutput, SdkError<GetApiMappingError>>> {
        (*self).get_api_mapping(builder)
    }
    fn get_api_mappings(&self, builder: GetApiMappingsInputBuilder) -> impl Future<Output = Result<GetApiMappingsOutput, SdkError<GetApiMappingsError>>> {
        (*self).get_api_mappings(builder)
    }
    fn get_apis(&self, builder: GetApisInputBuilder) -> impl Future<Output = Result<GetApisOutput, SdkError<GetApisError>>> {
        (*self).get_apis(builder)
    }
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> {
        (*self).get_authorizer(builder)
    }
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> {
        (*self).get_authorizers(builder)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        (*self).get_deployment(builder)
    }
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> {
        (*self).get_deployments(builder)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        (*self).get_domain_name(builder)
    }
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> {
        (*self).get_domain_names(builder)
    }
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> {
        (*self).get_integration(builder)
    }
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> {
        (*self).get_integration_response(builder)
    }
    fn get_integration_responses(&self, builder: GetIntegrationResponsesInputBuilder) -> impl Future<Output = Result<GetIntegrationResponsesOutput, SdkError<GetIntegrationResponsesError>>> {
        (*self).get_integration_responses(builder)
    }
    fn get_integrations(&self, builder: GetIntegrationsInputBuilder) -> impl Future<Output = Result<GetIntegrationsOutput, SdkError<GetIntegrationsError>>> {
        (*self).get_integrations(builder)
    }
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>> {
        (*self).get_model(builder)
    }
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>> {
        (*self).get_model_template(builder)
    }
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>> {
        (*self).get_models(builder)
    }
    fn get_route(&self, builder: GetRouteInputBuilder) -> impl Future<Output = Result<GetRouteOutput, SdkError<GetRouteError>>> {
        (*self).get_route(builder)
    }
    fn get_route_response(&self, builder: GetRouteResponseInputBuilder) -> impl Future<Output = Result<GetRouteResponseOutput, SdkError<GetRouteResponseError>>> {
        (*self).get_route_response(builder)
    }
    fn get_route_responses(&self, builder: GetRouteResponsesInputBuilder) -> impl Future<Output = Result<GetRouteResponsesOutput, SdkError<GetRouteResponsesError>>> {
        (*self).get_route_responses(builder)
    }
    fn get_routes(&self, builder: GetRoutesInputBuilder) -> impl Future<Output = Result<GetRoutesOutput, SdkError<GetRoutesError>>> {
        (*self).get_routes(builder)
    }
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>> {
        (*self).get_stage(builder)
    }
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>> {
        (*self).get_stages(builder)
    }
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>> {
        (*self).get_tags(builder)
    }
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> {
        (*self).get_vpc_link(builder)
    }
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> {
        (*self).get_vpc_links(builder)
    }
    fn import_api(&self, builder: ImportApiInputBuilder) -> impl Future<Output = Result<ImportApiOutput, SdkError<ImportApiError>>> {
        (*self).import_api(builder)
    }
    fn reimport_api(&self, builder: ReimportApiInputBuilder) -> impl Future<Output = Result<ReimportApiOutput, SdkError<ReimportApiError>>> {
        (*self).reimport_api(builder)
    }
    fn reset_authorizers_cache(&self, builder: ResetAuthorizersCacheInputBuilder) -> impl Future<Output = Result<ResetAuthorizersCacheOutput, SdkError<ResetAuthorizersCacheError>>> {
        (*self).reset_authorizers_cache(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_api(&self, builder: UpdateApiInputBuilder) -> impl Future<Output = Result<UpdateApiOutput, SdkError<UpdateApiError>>> {
        (*self).update_api(builder)
    }
    fn update_api_mapping(&self, builder: UpdateApiMappingInputBuilder) -> impl Future<Output = Result<UpdateApiMappingOutput, SdkError<UpdateApiMappingError>>> {
        (*self).update_api_mapping(builder)
    }
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> {
        (*self).update_authorizer(builder)
    }
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> {
        (*self).update_deployment(builder)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        (*self).update_domain_name(builder)
    }
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> {
        (*self).update_integration(builder)
    }
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> {
        (*self).update_integration_response(builder)
    }
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> {
        (*self).update_model(builder)
    }
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> {
        (*self).update_route(builder)
    }
    fn update_route_response(&self, builder: UpdateRouteResponseInputBuilder) -> impl Future<Output = Result<UpdateRouteResponseOutput, SdkError<UpdateRouteResponseError>>> {
        (*self).update_route_response(builder)
    }
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> {
        (*self).update_stage(builder)
    }
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> {
        (*self).update_vpc_link(builder)
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
