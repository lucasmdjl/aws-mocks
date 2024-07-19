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
use aws_sdk_apigateway::operation::create_api_key::{builders::*, *};
use aws_sdk_apigateway::operation::create_authorizer::{builders::*, *};
use aws_sdk_apigateway::operation::create_base_path_mapping::{builders::*, *};
use aws_sdk_apigateway::operation::create_deployment::{builders::*, *};
use aws_sdk_apigateway::operation::create_documentation_part::{builders::*, *};
use aws_sdk_apigateway::operation::create_documentation_version::{builders::*, *};
use aws_sdk_apigateway::operation::create_domain_name::{builders::*, *};
use aws_sdk_apigateway::operation::create_model::{builders::*, *};
use aws_sdk_apigateway::operation::create_request_validator::{builders::*, *};
use aws_sdk_apigateway::operation::create_resource::{builders::*, *};
use aws_sdk_apigateway::operation::create_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::create_stage::{builders::*, *};
use aws_sdk_apigateway::operation::create_usage_plan::{builders::*, *};
use aws_sdk_apigateway::operation::create_usage_plan_key::{builders::*, *};
use aws_sdk_apigateway::operation::create_vpc_link::{builders::*, *};
use aws_sdk_apigateway::operation::delete_api_key::{builders::*, *};
use aws_sdk_apigateway::operation::delete_authorizer::{builders::*, *};
use aws_sdk_apigateway::operation::delete_base_path_mapping::{builders::*, *};
use aws_sdk_apigateway::operation::delete_client_certificate::{builders::*, *};
use aws_sdk_apigateway::operation::delete_deployment::{builders::*, *};
use aws_sdk_apigateway::operation::delete_documentation_part::{builders::*, *};
use aws_sdk_apigateway::operation::delete_documentation_version::{builders::*, *};
use aws_sdk_apigateway::operation::delete_domain_name::{builders::*, *};
use aws_sdk_apigateway::operation::delete_gateway_response::{builders::*, *};
use aws_sdk_apigateway::operation::delete_integration::{builders::*, *};
use aws_sdk_apigateway::operation::delete_integration_response::{builders::*, *};
use aws_sdk_apigateway::operation::delete_method::{builders::*, *};
use aws_sdk_apigateway::operation::delete_method_response::{builders::*, *};
use aws_sdk_apigateway::operation::delete_model::{builders::*, *};
use aws_sdk_apigateway::operation::delete_request_validator::{builders::*, *};
use aws_sdk_apigateway::operation::delete_resource::{builders::*, *};
use aws_sdk_apigateway::operation::delete_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::delete_stage::{builders::*, *};
use aws_sdk_apigateway::operation::delete_usage_plan::{builders::*, *};
use aws_sdk_apigateway::operation::delete_usage_plan_key::{builders::*, *};
use aws_sdk_apigateway::operation::delete_vpc_link::{builders::*, *};
use aws_sdk_apigateway::operation::flush_stage_authorizers_cache::{builders::*, *};
use aws_sdk_apigateway::operation::flush_stage_cache::{builders::*, *};
use aws_sdk_apigateway::operation::generate_client_certificate::{builders::*, *};
use aws_sdk_apigateway::operation::get_account::{builders::*, *};
use aws_sdk_apigateway::operation::get_api_key::{builders::*, *};
use aws_sdk_apigateway::operation::get_api_keys::{builders::*, *};
use aws_sdk_apigateway::operation::get_authorizer::{builders::*, *};
use aws_sdk_apigateway::operation::get_authorizers::{builders::*, *};
use aws_sdk_apigateway::operation::get_base_path_mapping::{builders::*, *};
use aws_sdk_apigateway::operation::get_base_path_mappings::{builders::*, *};
use aws_sdk_apigateway::operation::get_client_certificate::{builders::*, *};
use aws_sdk_apigateway::operation::get_client_certificates::{builders::*, *};
use aws_sdk_apigateway::operation::get_deployment::{builders::*, *};
use aws_sdk_apigateway::operation::get_deployments::{builders::*, *};
use aws_sdk_apigateway::operation::get_documentation_part::{builders::*, *};
use aws_sdk_apigateway::operation::get_documentation_parts::{builders::*, *};
use aws_sdk_apigateway::operation::get_documentation_version::{builders::*, *};
use aws_sdk_apigateway::operation::get_documentation_versions::{builders::*, *};
use aws_sdk_apigateway::operation::get_domain_name::{builders::*, *};
use aws_sdk_apigateway::operation::get_domain_names::{builders::*, *};
use aws_sdk_apigateway::operation::get_export::{builders::*, *};
use aws_sdk_apigateway::operation::get_gateway_response::{builders::*, *};
use aws_sdk_apigateway::operation::get_gateway_responses::{builders::*, *};
use aws_sdk_apigateway::operation::get_integration::{builders::*, *};
use aws_sdk_apigateway::operation::get_integration_response::{builders::*, *};
use aws_sdk_apigateway::operation::get_method::{builders::*, *};
use aws_sdk_apigateway::operation::get_method_response::{builders::*, *};
use aws_sdk_apigateway::operation::get_model::{builders::*, *};
use aws_sdk_apigateway::operation::get_model_template::{builders::*, *};
use aws_sdk_apigateway::operation::get_models::{builders::*, *};
use aws_sdk_apigateway::operation::get_request_validator::{builders::*, *};
use aws_sdk_apigateway::operation::get_request_validators::{builders::*, *};
use aws_sdk_apigateway::operation::get_resource::{builders::*, *};
use aws_sdk_apigateway::operation::get_resources::{builders::*, *};
use aws_sdk_apigateway::operation::get_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::get_rest_apis::{builders::*, *};
use aws_sdk_apigateway::operation::get_sdk::{builders::*, *};
use aws_sdk_apigateway::operation::get_sdk_type::{builders::*, *};
use aws_sdk_apigateway::operation::get_sdk_types::{builders::*, *};
use aws_sdk_apigateway::operation::get_stage::{builders::*, *};
use aws_sdk_apigateway::operation::get_stages::{builders::*, *};
use aws_sdk_apigateway::operation::get_tags::{builders::*, *};
use aws_sdk_apigateway::operation::get_usage::{builders::*, *};
use aws_sdk_apigateway::operation::get_usage_plan::{builders::*, *};
use aws_sdk_apigateway::operation::get_usage_plan_key::{builders::*, *};
use aws_sdk_apigateway::operation::get_usage_plan_keys::{builders::*, *};
use aws_sdk_apigateway::operation::get_usage_plans::{builders::*, *};
use aws_sdk_apigateway::operation::get_vpc_link::{builders::*, *};
use aws_sdk_apigateway::operation::get_vpc_links::{builders::*, *};
use aws_sdk_apigateway::operation::import_api_keys::{builders::*, *};
use aws_sdk_apigateway::operation::import_documentation_parts::{builders::*, *};
use aws_sdk_apigateway::operation::import_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::put_gateway_response::{builders::*, *};
use aws_sdk_apigateway::operation::put_integration::{builders::*, *};
use aws_sdk_apigateway::operation::put_integration_response::{builders::*, *};
use aws_sdk_apigateway::operation::put_method::{builders::*, *};
use aws_sdk_apigateway::operation::put_method_response::{builders::*, *};
use aws_sdk_apigateway::operation::put_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::tag_resource::{builders::*, *};
use aws_sdk_apigateway::operation::test_invoke_authorizer::{builders::*, *};
use aws_sdk_apigateway::operation::test_invoke_method::{builders::*, *};
use aws_sdk_apigateway::operation::untag_resource::{builders::*, *};
use aws_sdk_apigateway::operation::update_account::{builders::*, *};
use aws_sdk_apigateway::operation::update_api_key::{builders::*, *};
use aws_sdk_apigateway::operation::update_authorizer::{builders::*, *};
use aws_sdk_apigateway::operation::update_base_path_mapping::{builders::*, *};
use aws_sdk_apigateway::operation::update_client_certificate::{builders::*, *};
use aws_sdk_apigateway::operation::update_deployment::{builders::*, *};
use aws_sdk_apigateway::operation::update_documentation_part::{builders::*, *};
use aws_sdk_apigateway::operation::update_documentation_version::{builders::*, *};
use aws_sdk_apigateway::operation::update_domain_name::{builders::*, *};
use aws_sdk_apigateway::operation::update_gateway_response::{builders::*, *};
use aws_sdk_apigateway::operation::update_integration::{builders::*, *};
use aws_sdk_apigateway::operation::update_integration_response::{builders::*, *};
use aws_sdk_apigateway::operation::update_method::{builders::*, *};
use aws_sdk_apigateway::operation::update_method_response::{builders::*, *};
use aws_sdk_apigateway::operation::update_model::{builders::*, *};
use aws_sdk_apigateway::operation::update_request_validator::{builders::*, *};
use aws_sdk_apigateway::operation::update_resource::{builders::*, *};
use aws_sdk_apigateway::operation::update_rest_api::{builders::*, *};
use aws_sdk_apigateway::operation::update_stage::{builders::*, *};
use aws_sdk_apigateway::operation::update_usage::{builders::*, *};
use aws_sdk_apigateway::operation::update_usage_plan::{builders::*, *};
use aws_sdk_apigateway::operation::update_vpc_link::{builders::*, *};
use aws_sdk_apigateway::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_apigateway::Client;

pub use aws_sdk_apigateway::*;

pub struct ApiGatewayClientImpl(Client);
impl ApiGatewayClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ApiGatewayClient {
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>>;
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>>;
    fn create_base_path_mapping(&self, builder: CreateBasePathMappingInputBuilder) -> impl Future<Output = Result<CreateBasePathMappingOutput, SdkError<CreateBasePathMappingError>>>;
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>>;
    fn create_documentation_part(&self, builder: CreateDocumentationPartInputBuilder) -> impl Future<Output = Result<CreateDocumentationPartOutput, SdkError<CreateDocumentationPartError>>>;
    fn create_documentation_version(&self, builder: CreateDocumentationVersionInputBuilder) -> impl Future<Output = Result<CreateDocumentationVersionOutput, SdkError<CreateDocumentationVersionError>>>;
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>>;
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>>;
    fn create_request_validator(&self, builder: CreateRequestValidatorInputBuilder) -> impl Future<Output = Result<CreateRequestValidatorOutput, SdkError<CreateRequestValidatorError>>>;
    fn create_resource(&self, builder: CreateResourceInputBuilder) -> impl Future<Output = Result<CreateResourceOutput, SdkError<CreateResourceError>>>;
    fn create_rest_api(&self, builder: CreateRestApiInputBuilder) -> impl Future<Output = Result<CreateRestApiOutput, SdkError<CreateRestApiError>>>;
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>>;
    fn create_usage_plan(&self, builder: CreateUsagePlanInputBuilder) -> impl Future<Output = Result<CreateUsagePlanOutput, SdkError<CreateUsagePlanError>>>;
    fn create_usage_plan_key(&self, builder: CreateUsagePlanKeyInputBuilder) -> impl Future<Output = Result<CreateUsagePlanKeyOutput, SdkError<CreateUsagePlanKeyError>>>;
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>>;
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>>;
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>>;
    fn delete_base_path_mapping(&self, builder: DeleteBasePathMappingInputBuilder) -> impl Future<Output = Result<DeleteBasePathMappingOutput, SdkError<DeleteBasePathMappingError>>>;
    fn delete_client_certificate(&self, builder: DeleteClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteClientCertificateOutput, SdkError<DeleteClientCertificateError>>>;
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>>;
    fn delete_documentation_part(&self, builder: DeleteDocumentationPartInputBuilder) -> impl Future<Output = Result<DeleteDocumentationPartOutput, SdkError<DeleteDocumentationPartError>>>;
    fn delete_documentation_version(&self, builder: DeleteDocumentationVersionInputBuilder) -> impl Future<Output = Result<DeleteDocumentationVersionOutput, SdkError<DeleteDocumentationVersionError>>>;
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>>;
    fn delete_gateway_response(&self, builder: DeleteGatewayResponseInputBuilder) -> impl Future<Output = Result<DeleteGatewayResponseOutput, SdkError<DeleteGatewayResponseError>>>;
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>>;
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>>;
    fn delete_method(&self, builder: DeleteMethodInputBuilder) -> impl Future<Output = Result<DeleteMethodOutput, SdkError<DeleteMethodError>>>;
    fn delete_method_response(&self, builder: DeleteMethodResponseInputBuilder) -> impl Future<Output = Result<DeleteMethodResponseOutput, SdkError<DeleteMethodResponseError>>>;
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>>;
    fn delete_request_validator(&self, builder: DeleteRequestValidatorInputBuilder) -> impl Future<Output = Result<DeleteRequestValidatorOutput, SdkError<DeleteRequestValidatorError>>>;
    fn delete_resource(&self, builder: DeleteResourceInputBuilder) -> impl Future<Output = Result<DeleteResourceOutput, SdkError<DeleteResourceError>>>;
    fn delete_rest_api(&self, builder: DeleteRestApiInputBuilder) -> impl Future<Output = Result<DeleteRestApiOutput, SdkError<DeleteRestApiError>>>;
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>>;
    fn delete_usage_plan(&self, builder: DeleteUsagePlanInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanOutput, SdkError<DeleteUsagePlanError>>>;
    fn delete_usage_plan_key(&self, builder: DeleteUsagePlanKeyInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanKeyOutput, SdkError<DeleteUsagePlanKeyError>>>;
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>>;
    fn flush_stage_authorizers_cache(&self, builder: FlushStageAuthorizersCacheInputBuilder) -> impl Future<Output = Result<FlushStageAuthorizersCacheOutput, SdkError<FlushStageAuthorizersCacheError>>>;
    fn flush_stage_cache(&self, builder: FlushStageCacheInputBuilder) -> impl Future<Output = Result<FlushStageCacheOutput, SdkError<FlushStageCacheError>>>;
    fn generate_client_certificate(&self, builder: GenerateClientCertificateInputBuilder) -> impl Future<Output = Result<GenerateClientCertificateOutput, SdkError<GenerateClientCertificateError>>>;
    fn get_account(&self, builder: GetAccountInputBuilder) -> impl Future<Output = Result<GetAccountOutput, SdkError<GetAccountError>>>;
    fn get_api_key(&self, builder: GetApiKeyInputBuilder) -> impl Future<Output = Result<GetApiKeyOutput, SdkError<GetApiKeyError>>>;
    fn get_api_keys(&self, builder: GetApiKeysInputBuilder) -> impl Future<Output = Result<GetApiKeysOutput, SdkError<GetApiKeysError>>>;
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>>;
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>>;
    fn get_base_path_mapping(&self, builder: GetBasePathMappingInputBuilder) -> impl Future<Output = Result<GetBasePathMappingOutput, SdkError<GetBasePathMappingError>>>;
    fn get_base_path_mappings(&self, builder: GetBasePathMappingsInputBuilder) -> impl Future<Output = Result<GetBasePathMappingsOutput, SdkError<GetBasePathMappingsError>>>;
    fn get_client_certificate(&self, builder: GetClientCertificateInputBuilder) -> impl Future<Output = Result<GetClientCertificateOutput, SdkError<GetClientCertificateError>>>;
    fn get_client_certificates(&self, builder: GetClientCertificatesInputBuilder) -> impl Future<Output = Result<GetClientCertificatesOutput, SdkError<GetClientCertificatesError>>>;
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>>;
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>>;
    fn get_documentation_part(&self, builder: GetDocumentationPartInputBuilder) -> impl Future<Output = Result<GetDocumentationPartOutput, SdkError<GetDocumentationPartError>>>;
    fn get_documentation_parts(&self, builder: GetDocumentationPartsInputBuilder) -> impl Future<Output = Result<GetDocumentationPartsOutput, SdkError<GetDocumentationPartsError>>>;
    fn get_documentation_version(&self, builder: GetDocumentationVersionInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionOutput, SdkError<GetDocumentationVersionError>>>;
    fn get_documentation_versions(&self, builder: GetDocumentationVersionsInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionsOutput, SdkError<GetDocumentationVersionsError>>>;
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>>;
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>>;
    fn get_export(&self, builder: GetExportInputBuilder) -> impl Future<Output = Result<GetExportOutput, SdkError<GetExportError>>>;
    fn get_gateway_response(&self, builder: GetGatewayResponseInputBuilder) -> impl Future<Output = Result<GetGatewayResponseOutput, SdkError<GetGatewayResponseError>>>;
    fn get_gateway_responses(&self, builder: GetGatewayResponsesInputBuilder) -> impl Future<Output = Result<GetGatewayResponsesOutput, SdkError<GetGatewayResponsesError>>>;
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>>;
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>>;
    fn get_method(&self, builder: GetMethodInputBuilder) -> impl Future<Output = Result<GetMethodOutput, SdkError<GetMethodError>>>;
    fn get_method_response(&self, builder: GetMethodResponseInputBuilder) -> impl Future<Output = Result<GetMethodResponseOutput, SdkError<GetMethodResponseError>>>;
    fn get_model(&self, builder: GetModelInputBuilder) -> impl Future<Output = Result<GetModelOutput, SdkError<GetModelError>>>;
    fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> impl Future<Output = Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>>;
    fn get_models(&self, builder: GetModelsInputBuilder) -> impl Future<Output = Result<GetModelsOutput, SdkError<GetModelsError>>>;
    fn get_request_validator(&self, builder: GetRequestValidatorInputBuilder) -> impl Future<Output = Result<GetRequestValidatorOutput, SdkError<GetRequestValidatorError>>>;
    fn get_request_validators(&self, builder: GetRequestValidatorsInputBuilder) -> impl Future<Output = Result<GetRequestValidatorsOutput, SdkError<GetRequestValidatorsError>>>;
    fn get_resource(&self, builder: GetResourceInputBuilder) -> impl Future<Output = Result<GetResourceOutput, SdkError<GetResourceError>>>;
    fn get_resources(&self, builder: GetResourcesInputBuilder) -> impl Future<Output = Result<GetResourcesOutput, SdkError<GetResourcesError>>>;
    fn get_rest_api(&self, builder: GetRestApiInputBuilder) -> impl Future<Output = Result<GetRestApiOutput, SdkError<GetRestApiError>>>;
    fn get_rest_apis(&self, builder: GetRestApisInputBuilder) -> impl Future<Output = Result<GetRestApisOutput, SdkError<GetRestApisError>>>;
    fn get_sdk(&self, builder: GetSdkInputBuilder) -> impl Future<Output = Result<GetSdkOutput, SdkError<GetSdkError>>>;
    fn get_sdk_type(&self, builder: GetSdkTypeInputBuilder) -> impl Future<Output = Result<GetSdkTypeOutput, SdkError<GetSdkTypeError>>>;
    fn get_sdk_types(&self, builder: GetSdkTypesInputBuilder) -> impl Future<Output = Result<GetSdkTypesOutput, SdkError<GetSdkTypesError>>>;
    fn get_stage(&self, builder: GetStageInputBuilder) -> impl Future<Output = Result<GetStageOutput, SdkError<GetStageError>>>;
    fn get_stages(&self, builder: GetStagesInputBuilder) -> impl Future<Output = Result<GetStagesOutput, SdkError<GetStagesError>>>;
    fn get_tags(&self, builder: GetTagsInputBuilder) -> impl Future<Output = Result<GetTagsOutput, SdkError<GetTagsError>>>;
    fn get_usage(&self, builder: GetUsageInputBuilder) -> impl Future<Output = Result<GetUsageOutput, SdkError<GetUsageError>>>;
    fn get_usage_plan(&self, builder: GetUsagePlanInputBuilder) -> impl Future<Output = Result<GetUsagePlanOutput, SdkError<GetUsagePlanError>>>;
    fn get_usage_plan_key(&self, builder: GetUsagePlanKeyInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeyOutput, SdkError<GetUsagePlanKeyError>>>;
    fn get_usage_plan_keys(&self, builder: GetUsagePlanKeysInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeysOutput, SdkError<GetUsagePlanKeysError>>>;
    fn get_usage_plans(&self, builder: GetUsagePlansInputBuilder) -> impl Future<Output = Result<GetUsagePlansOutput, SdkError<GetUsagePlansError>>>;
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>>;
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>>;
    fn import_api_keys(&self, builder: ImportApiKeysInputBuilder) -> impl Future<Output = Result<ImportApiKeysOutput, SdkError<ImportApiKeysError>>>;
    fn import_documentation_parts(&self, builder: ImportDocumentationPartsInputBuilder) -> impl Future<Output = Result<ImportDocumentationPartsOutput, SdkError<ImportDocumentationPartsError>>>;
    fn import_rest_api(&self, builder: ImportRestApiInputBuilder) -> impl Future<Output = Result<ImportRestApiOutput, SdkError<ImportRestApiError>>>;
    fn put_gateway_response(&self, builder: PutGatewayResponseInputBuilder) -> impl Future<Output = Result<PutGatewayResponseOutput, SdkError<PutGatewayResponseError>>>;
    fn put_integration(&self, builder: PutIntegrationInputBuilder) -> impl Future<Output = Result<PutIntegrationOutput, SdkError<PutIntegrationError>>>;
    fn put_integration_response(&self, builder: PutIntegrationResponseInputBuilder) -> impl Future<Output = Result<PutIntegrationResponseOutput, SdkError<PutIntegrationResponseError>>>;
    fn put_method(&self, builder: PutMethodInputBuilder) -> impl Future<Output = Result<PutMethodOutput, SdkError<PutMethodError>>>;
    fn put_method_response(&self, builder: PutMethodResponseInputBuilder) -> impl Future<Output = Result<PutMethodResponseOutput, SdkError<PutMethodResponseError>>>;
    fn put_rest_api(&self, builder: PutRestApiInputBuilder) -> impl Future<Output = Result<PutRestApiOutput, SdkError<PutRestApiError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn test_invoke_authorizer(&self, builder: TestInvokeAuthorizerInputBuilder) -> impl Future<Output = Result<TestInvokeAuthorizerOutput, SdkError<TestInvokeAuthorizerError>>>;
    fn test_invoke_method(&self, builder: TestInvokeMethodInputBuilder) -> impl Future<Output = Result<TestInvokeMethodOutput, SdkError<TestInvokeMethodError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_account(&self, builder: UpdateAccountInputBuilder) -> impl Future<Output = Result<UpdateAccountOutput, SdkError<UpdateAccountError>>>;
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>>;
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>>;
    fn update_base_path_mapping(&self, builder: UpdateBasePathMappingInputBuilder) -> impl Future<Output = Result<UpdateBasePathMappingOutput, SdkError<UpdateBasePathMappingError>>>;
    fn update_client_certificate(&self, builder: UpdateClientCertificateInputBuilder) -> impl Future<Output = Result<UpdateClientCertificateOutput, SdkError<UpdateClientCertificateError>>>;
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>>;
    fn update_documentation_part(&self, builder: UpdateDocumentationPartInputBuilder) -> impl Future<Output = Result<UpdateDocumentationPartOutput, SdkError<UpdateDocumentationPartError>>>;
    fn update_documentation_version(&self, builder: UpdateDocumentationVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentationVersionOutput, SdkError<UpdateDocumentationVersionError>>>;
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>>;
    fn update_gateway_response(&self, builder: UpdateGatewayResponseInputBuilder) -> impl Future<Output = Result<UpdateGatewayResponseOutput, SdkError<UpdateGatewayResponseError>>>;
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>>;
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>>;
    fn update_method(&self, builder: UpdateMethodInputBuilder) -> impl Future<Output = Result<UpdateMethodOutput, SdkError<UpdateMethodError>>>;
    fn update_method_response(&self, builder: UpdateMethodResponseInputBuilder) -> impl Future<Output = Result<UpdateMethodResponseOutput, SdkError<UpdateMethodResponseError>>>;
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>>;
    fn update_request_validator(&self, builder: UpdateRequestValidatorInputBuilder) -> impl Future<Output = Result<UpdateRequestValidatorOutput, SdkError<UpdateRequestValidatorError>>>;
    fn update_resource(&self, builder: UpdateResourceInputBuilder) -> impl Future<Output = Result<UpdateResourceOutput, SdkError<UpdateResourceError>>>;
    fn update_rest_api(&self, builder: UpdateRestApiInputBuilder) -> impl Future<Output = Result<UpdateRestApiOutput, SdkError<UpdateRestApiError>>>;
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>>;
    fn update_usage(&self, builder: UpdateUsageInputBuilder) -> impl Future<Output = Result<UpdateUsageOutput, SdkError<UpdateUsageError>>>;
    fn update_usage_plan(&self, builder: UpdateUsagePlanInputBuilder) -> impl Future<Output = Result<UpdateUsagePlanOutput, SdkError<UpdateUsagePlanError>>>;
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>>;
}
impl ApiGatewayClient for ApiGatewayClientImpl {
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn create_base_path_mapping(&self, builder: CreateBasePathMappingInputBuilder) -> impl Future<Output = Result<CreateBasePathMappingOutput, SdkError<CreateBasePathMappingError>>> {
        builder.send_with(&self.0)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn create_documentation_part(&self, builder: CreateDocumentationPartInputBuilder) -> impl Future<Output = Result<CreateDocumentationPartOutput, SdkError<CreateDocumentationPartError>>> {
        builder.send_with(&self.0)
    }
    fn create_documentation_version(&self, builder: CreateDocumentationVersionInputBuilder) -> impl Future<Output = Result<CreateDocumentationVersionOutput, SdkError<CreateDocumentationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        builder.send_with(&self.0)
    }
    fn create_request_validator(&self, builder: CreateRequestValidatorInputBuilder) -> impl Future<Output = Result<CreateRequestValidatorOutput, SdkError<CreateRequestValidatorError>>> {
        builder.send_with(&self.0)
    }
    fn create_resource(&self, builder: CreateResourceInputBuilder) -> impl Future<Output = Result<CreateResourceOutput, SdkError<CreateResourceError>>> {
        builder.send_with(&self.0)
    }
    fn create_rest_api(&self, builder: CreateRestApiInputBuilder) -> impl Future<Output = Result<CreateRestApiOutput, SdkError<CreateRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> {
        builder.send_with(&self.0)
    }
    fn create_usage_plan(&self, builder: CreateUsagePlanInputBuilder) -> impl Future<Output = Result<CreateUsagePlanOutput, SdkError<CreateUsagePlanError>>> {
        builder.send_with(&self.0)
    }
    fn create_usage_plan_key(&self, builder: CreateUsagePlanKeyInputBuilder) -> impl Future<Output = Result<CreateUsagePlanKeyOutput, SdkError<CreateUsagePlanKeyError>>> {
        builder.send_with(&self.0)
    }
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_base_path_mapping(&self, builder: DeleteBasePathMappingInputBuilder) -> impl Future<Output = Result<DeleteBasePathMappingOutput, SdkError<DeleteBasePathMappingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_client_certificate(&self, builder: DeleteClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteClientCertificateOutput, SdkError<DeleteClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_documentation_part(&self, builder: DeleteDocumentationPartInputBuilder) -> impl Future<Output = Result<DeleteDocumentationPartOutput, SdkError<DeleteDocumentationPartError>>> {
        builder.send_with(&self.0)
    }
    fn delete_documentation_version(&self, builder: DeleteDocumentationVersionInputBuilder) -> impl Future<Output = Result<DeleteDocumentationVersionOutput, SdkError<DeleteDocumentationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn delete_gateway_response(&self, builder: DeleteGatewayResponseInputBuilder) -> impl Future<Output = Result<DeleteGatewayResponseOutput, SdkError<DeleteGatewayResponseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_method(&self, builder: DeleteMethodInputBuilder) -> impl Future<Output = Result<DeleteMethodOutput, SdkError<DeleteMethodError>>> {
        builder.send_with(&self.0)
    }
    fn delete_method_response(&self, builder: DeleteMethodResponseInputBuilder) -> impl Future<Output = Result<DeleteMethodResponseOutput, SdkError<DeleteMethodResponseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        builder.send_with(&self.0)
    }
    fn delete_request_validator(&self, builder: DeleteRequestValidatorInputBuilder) -> impl Future<Output = Result<DeleteRequestValidatorOutput, SdkError<DeleteRequestValidatorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource(&self, builder: DeleteResourceInputBuilder) -> impl Future<Output = Result<DeleteResourceOutput, SdkError<DeleteResourceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_rest_api(&self, builder: DeleteRestApiInputBuilder) -> impl Future<Output = Result<DeleteRestApiOutput, SdkError<DeleteRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> {
        builder.send_with(&self.0)
    }
    fn delete_usage_plan(&self, builder: DeleteUsagePlanInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanOutput, SdkError<DeleteUsagePlanError>>> {
        builder.send_with(&self.0)
    }
    fn delete_usage_plan_key(&self, builder: DeleteUsagePlanKeyInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanKeyOutput, SdkError<DeleteUsagePlanKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn flush_stage_authorizers_cache(&self, builder: FlushStageAuthorizersCacheInputBuilder) -> impl Future<Output = Result<FlushStageAuthorizersCacheOutput, SdkError<FlushStageAuthorizersCacheError>>> {
        builder.send_with(&self.0)
    }
    fn flush_stage_cache(&self, builder: FlushStageCacheInputBuilder) -> impl Future<Output = Result<FlushStageCacheOutput, SdkError<FlushStageCacheError>>> {
        builder.send_with(&self.0)
    }
    fn generate_client_certificate(&self, builder: GenerateClientCertificateInputBuilder) -> impl Future<Output = Result<GenerateClientCertificateOutput, SdkError<GenerateClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_account(&self, builder: GetAccountInputBuilder) -> impl Future<Output = Result<GetAccountOutput, SdkError<GetAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_key(&self, builder: GetApiKeyInputBuilder) -> impl Future<Output = Result<GetApiKeyOutput, SdkError<GetApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_keys(&self, builder: GetApiKeysInputBuilder) -> impl Future<Output = Result<GetApiKeysOutput, SdkError<GetApiKeysError>>> {
        builder.send_with(&self.0)
    }
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> {
        builder.send_with(&self.0)
    }
    fn get_base_path_mapping(&self, builder: GetBasePathMappingInputBuilder) -> impl Future<Output = Result<GetBasePathMappingOutput, SdkError<GetBasePathMappingError>>> {
        builder.send_with(&self.0)
    }
    fn get_base_path_mappings(&self, builder: GetBasePathMappingsInputBuilder) -> impl Future<Output = Result<GetBasePathMappingsOutput, SdkError<GetBasePathMappingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_client_certificate(&self, builder: GetClientCertificateInputBuilder) -> impl Future<Output = Result<GetClientCertificateOutput, SdkError<GetClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_client_certificates(&self, builder: GetClientCertificatesInputBuilder) -> impl Future<Output = Result<GetClientCertificatesOutput, SdkError<GetClientCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> {
        builder.send_with(&self.0)
    }
    fn get_documentation_part(&self, builder: GetDocumentationPartInputBuilder) -> impl Future<Output = Result<GetDocumentationPartOutput, SdkError<GetDocumentationPartError>>> {
        builder.send_with(&self.0)
    }
    fn get_documentation_parts(&self, builder: GetDocumentationPartsInputBuilder) -> impl Future<Output = Result<GetDocumentationPartsOutput, SdkError<GetDocumentationPartsError>>> {
        builder.send_with(&self.0)
    }
    fn get_documentation_version(&self, builder: GetDocumentationVersionInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionOutput, SdkError<GetDocumentationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_documentation_versions(&self, builder: GetDocumentationVersionsInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionsOutput, SdkError<GetDocumentationVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> {
        builder.send_with(&self.0)
    }
    fn get_export(&self, builder: GetExportInputBuilder) -> impl Future<Output = Result<GetExportOutput, SdkError<GetExportError>>> {
        builder.send_with(&self.0)
    }
    fn get_gateway_response(&self, builder: GetGatewayResponseInputBuilder) -> impl Future<Output = Result<GetGatewayResponseOutput, SdkError<GetGatewayResponseError>>> {
        builder.send_with(&self.0)
    }
    fn get_gateway_responses(&self, builder: GetGatewayResponsesInputBuilder) -> impl Future<Output = Result<GetGatewayResponsesOutput, SdkError<GetGatewayResponsesError>>> {
        builder.send_with(&self.0)
    }
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn get_method(&self, builder: GetMethodInputBuilder) -> impl Future<Output = Result<GetMethodOutput, SdkError<GetMethodError>>> {
        builder.send_with(&self.0)
    }
    fn get_method_response(&self, builder: GetMethodResponseInputBuilder) -> impl Future<Output = Result<GetMethodResponseOutput, SdkError<GetMethodResponseError>>> {
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
    fn get_request_validator(&self, builder: GetRequestValidatorInputBuilder) -> impl Future<Output = Result<GetRequestValidatorOutput, SdkError<GetRequestValidatorError>>> {
        builder.send_with(&self.0)
    }
    fn get_request_validators(&self, builder: GetRequestValidatorsInputBuilder) -> impl Future<Output = Result<GetRequestValidatorsOutput, SdkError<GetRequestValidatorsError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource(&self, builder: GetResourceInputBuilder) -> impl Future<Output = Result<GetResourceOutput, SdkError<GetResourceError>>> {
        builder.send_with(&self.0)
    }
    fn get_resources(&self, builder: GetResourcesInputBuilder) -> impl Future<Output = Result<GetResourcesOutput, SdkError<GetResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn get_rest_api(&self, builder: GetRestApiInputBuilder) -> impl Future<Output = Result<GetRestApiOutput, SdkError<GetRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn get_rest_apis(&self, builder: GetRestApisInputBuilder) -> impl Future<Output = Result<GetRestApisOutput, SdkError<GetRestApisError>>> {
        builder.send_with(&self.0)
    }
    fn get_sdk(&self, builder: GetSdkInputBuilder) -> impl Future<Output = Result<GetSdkOutput, SdkError<GetSdkError>>> {
        builder.send_with(&self.0)
    }
    fn get_sdk_type(&self, builder: GetSdkTypeInputBuilder) -> impl Future<Output = Result<GetSdkTypeOutput, SdkError<GetSdkTypeError>>> {
        builder.send_with(&self.0)
    }
    fn get_sdk_types(&self, builder: GetSdkTypesInputBuilder) -> impl Future<Output = Result<GetSdkTypesOutput, SdkError<GetSdkTypesError>>> {
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
    fn get_usage(&self, builder: GetUsageInputBuilder) -> impl Future<Output = Result<GetUsageOutput, SdkError<GetUsageError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_plan(&self, builder: GetUsagePlanInputBuilder) -> impl Future<Output = Result<GetUsagePlanOutput, SdkError<GetUsagePlanError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_plan_key(&self, builder: GetUsagePlanKeyInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeyOutput, SdkError<GetUsagePlanKeyError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_plan_keys(&self, builder: GetUsagePlanKeysInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeysOutput, SdkError<GetUsagePlanKeysError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_plans(&self, builder: GetUsagePlansInputBuilder) -> impl Future<Output = Result<GetUsagePlansOutput, SdkError<GetUsagePlansError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> {
        builder.send_with(&self.0)
    }
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> {
        builder.send_with(&self.0)
    }
    fn import_api_keys(&self, builder: ImportApiKeysInputBuilder) -> impl Future<Output = Result<ImportApiKeysOutput, SdkError<ImportApiKeysError>>> {
        builder.send_with(&self.0)
    }
    fn import_documentation_parts(&self, builder: ImportDocumentationPartsInputBuilder) -> impl Future<Output = Result<ImportDocumentationPartsOutput, SdkError<ImportDocumentationPartsError>>> {
        builder.send_with(&self.0)
    }
    fn import_rest_api(&self, builder: ImportRestApiInputBuilder) -> impl Future<Output = Result<ImportRestApiOutput, SdkError<ImportRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn put_gateway_response(&self, builder: PutGatewayResponseInputBuilder) -> impl Future<Output = Result<PutGatewayResponseOutput, SdkError<PutGatewayResponseError>>> {
        builder.send_with(&self.0)
    }
    fn put_integration(&self, builder: PutIntegrationInputBuilder) -> impl Future<Output = Result<PutIntegrationOutput, SdkError<PutIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn put_integration_response(&self, builder: PutIntegrationResponseInputBuilder) -> impl Future<Output = Result<PutIntegrationResponseOutput, SdkError<PutIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn put_method(&self, builder: PutMethodInputBuilder) -> impl Future<Output = Result<PutMethodOutput, SdkError<PutMethodError>>> {
        builder.send_with(&self.0)
    }
    fn put_method_response(&self, builder: PutMethodResponseInputBuilder) -> impl Future<Output = Result<PutMethodResponseOutput, SdkError<PutMethodResponseError>>> {
        builder.send_with(&self.0)
    }
    fn put_rest_api(&self, builder: PutRestApiInputBuilder) -> impl Future<Output = Result<PutRestApiOutput, SdkError<PutRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn test_invoke_authorizer(&self, builder: TestInvokeAuthorizerInputBuilder) -> impl Future<Output = Result<TestInvokeAuthorizerOutput, SdkError<TestInvokeAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn test_invoke_method(&self, builder: TestInvokeMethodInputBuilder) -> impl Future<Output = Result<TestInvokeMethodOutput, SdkError<TestInvokeMethodError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_account(&self, builder: UpdateAccountInputBuilder) -> impl Future<Output = Result<UpdateAccountOutput, SdkError<UpdateAccountError>>> {
        builder.send_with(&self.0)
    }
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> {
        builder.send_with(&self.0)
    }
    fn update_base_path_mapping(&self, builder: UpdateBasePathMappingInputBuilder) -> impl Future<Output = Result<UpdateBasePathMappingOutput, SdkError<UpdateBasePathMappingError>>> {
        builder.send_with(&self.0)
    }
    fn update_client_certificate(&self, builder: UpdateClientCertificateInputBuilder) -> impl Future<Output = Result<UpdateClientCertificateOutput, SdkError<UpdateClientCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> {
        builder.send_with(&self.0)
    }
    fn update_documentation_part(&self, builder: UpdateDocumentationPartInputBuilder) -> impl Future<Output = Result<UpdateDocumentationPartOutput, SdkError<UpdateDocumentationPartError>>> {
        builder.send_with(&self.0)
    }
    fn update_documentation_version(&self, builder: UpdateDocumentationVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentationVersionOutput, SdkError<UpdateDocumentationVersionError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn update_gateway_response(&self, builder: UpdateGatewayResponseInputBuilder) -> impl Future<Output = Result<UpdateGatewayResponseOutput, SdkError<UpdateGatewayResponseError>>> {
        builder.send_with(&self.0)
    }
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> {
        builder.send_with(&self.0)
    }
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> {
        builder.send_with(&self.0)
    }
    fn update_method(&self, builder: UpdateMethodInputBuilder) -> impl Future<Output = Result<UpdateMethodOutput, SdkError<UpdateMethodError>>> {
        builder.send_with(&self.0)
    }
    fn update_method_response(&self, builder: UpdateMethodResponseInputBuilder) -> impl Future<Output = Result<UpdateMethodResponseOutput, SdkError<UpdateMethodResponseError>>> {
        builder.send_with(&self.0)
    }
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> {
        builder.send_with(&self.0)
    }
    fn update_request_validator(&self, builder: UpdateRequestValidatorInputBuilder) -> impl Future<Output = Result<UpdateRequestValidatorOutput, SdkError<UpdateRequestValidatorError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource(&self, builder: UpdateResourceInputBuilder) -> impl Future<Output = Result<UpdateResourceOutput, SdkError<UpdateResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_rest_api(&self, builder: UpdateRestApiInputBuilder) -> impl Future<Output = Result<UpdateRestApiOutput, SdkError<UpdateRestApiError>>> {
        builder.send_with(&self.0)
    }
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> {
        builder.send_with(&self.0)
    }
    fn update_usage(&self, builder: UpdateUsageInputBuilder) -> impl Future<Output = Result<UpdateUsageOutput, SdkError<UpdateUsageError>>> {
        builder.send_with(&self.0)
    }
    fn update_usage_plan(&self, builder: UpdateUsagePlanInputBuilder) -> impl Future<Output = Result<UpdateUsagePlanOutput, SdkError<UpdateUsagePlanError>>> {
        builder.send_with(&self.0)
    }
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: ApiGatewayClient> ApiGatewayClient for &T {
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>> {
        (*self).create_api_key(builder)
    }
    fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> impl Future<Output = Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>> {
        (*self).create_authorizer(builder)
    }
    fn create_base_path_mapping(&self, builder: CreateBasePathMappingInputBuilder) -> impl Future<Output = Result<CreateBasePathMappingOutput, SdkError<CreateBasePathMappingError>>> {
        (*self).create_base_path_mapping(builder)
    }
    fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> impl Future<Output = Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>> {
        (*self).create_deployment(builder)
    }
    fn create_documentation_part(&self, builder: CreateDocumentationPartInputBuilder) -> impl Future<Output = Result<CreateDocumentationPartOutput, SdkError<CreateDocumentationPartError>>> {
        (*self).create_documentation_part(builder)
    }
    fn create_documentation_version(&self, builder: CreateDocumentationVersionInputBuilder) -> impl Future<Output = Result<CreateDocumentationVersionOutput, SdkError<CreateDocumentationVersionError>>> {
        (*self).create_documentation_version(builder)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        (*self).create_domain_name(builder)
    }
    fn create_model(&self, builder: CreateModelInputBuilder) -> impl Future<Output = Result<CreateModelOutput, SdkError<CreateModelError>>> {
        (*self).create_model(builder)
    }
    fn create_request_validator(&self, builder: CreateRequestValidatorInputBuilder) -> impl Future<Output = Result<CreateRequestValidatorOutput, SdkError<CreateRequestValidatorError>>> {
        (*self).create_request_validator(builder)
    }
    fn create_resource(&self, builder: CreateResourceInputBuilder) -> impl Future<Output = Result<CreateResourceOutput, SdkError<CreateResourceError>>> {
        (*self).create_resource(builder)
    }
    fn create_rest_api(&self, builder: CreateRestApiInputBuilder) -> impl Future<Output = Result<CreateRestApiOutput, SdkError<CreateRestApiError>>> {
        (*self).create_rest_api(builder)
    }
    fn create_stage(&self, builder: CreateStageInputBuilder) -> impl Future<Output = Result<CreateStageOutput, SdkError<CreateStageError>>> {
        (*self).create_stage(builder)
    }
    fn create_usage_plan(&self, builder: CreateUsagePlanInputBuilder) -> impl Future<Output = Result<CreateUsagePlanOutput, SdkError<CreateUsagePlanError>>> {
        (*self).create_usage_plan(builder)
    }
    fn create_usage_plan_key(&self, builder: CreateUsagePlanKeyInputBuilder) -> impl Future<Output = Result<CreateUsagePlanKeyOutput, SdkError<CreateUsagePlanKeyError>>> {
        (*self).create_usage_plan_key(builder)
    }
    fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> impl Future<Output = Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>> {
        (*self).create_vpc_link(builder)
    }
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>> {
        (*self).delete_api_key(builder)
    }
    fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> impl Future<Output = Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>> {
        (*self).delete_authorizer(builder)
    }
    fn delete_base_path_mapping(&self, builder: DeleteBasePathMappingInputBuilder) -> impl Future<Output = Result<DeleteBasePathMappingOutput, SdkError<DeleteBasePathMappingError>>> {
        (*self).delete_base_path_mapping(builder)
    }
    fn delete_client_certificate(&self, builder: DeleteClientCertificateInputBuilder) -> impl Future<Output = Result<DeleteClientCertificateOutput, SdkError<DeleteClientCertificateError>>> {
        (*self).delete_client_certificate(builder)
    }
    fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> impl Future<Output = Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>> {
        (*self).delete_deployment(builder)
    }
    fn delete_documentation_part(&self, builder: DeleteDocumentationPartInputBuilder) -> impl Future<Output = Result<DeleteDocumentationPartOutput, SdkError<DeleteDocumentationPartError>>> {
        (*self).delete_documentation_part(builder)
    }
    fn delete_documentation_version(&self, builder: DeleteDocumentationVersionInputBuilder) -> impl Future<Output = Result<DeleteDocumentationVersionOutput, SdkError<DeleteDocumentationVersionError>>> {
        (*self).delete_documentation_version(builder)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        (*self).delete_domain_name(builder)
    }
    fn delete_gateway_response(&self, builder: DeleteGatewayResponseInputBuilder) -> impl Future<Output = Result<DeleteGatewayResponseOutput, SdkError<DeleteGatewayResponseError>>> {
        (*self).delete_gateway_response(builder)
    }
    fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> impl Future<Output = Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>> {
        (*self).delete_integration(builder)
    }
    fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> impl Future<Output = Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>> {
        (*self).delete_integration_response(builder)
    }
    fn delete_method(&self, builder: DeleteMethodInputBuilder) -> impl Future<Output = Result<DeleteMethodOutput, SdkError<DeleteMethodError>>> {
        (*self).delete_method(builder)
    }
    fn delete_method_response(&self, builder: DeleteMethodResponseInputBuilder) -> impl Future<Output = Result<DeleteMethodResponseOutput, SdkError<DeleteMethodResponseError>>> {
        (*self).delete_method_response(builder)
    }
    fn delete_model(&self, builder: DeleteModelInputBuilder) -> impl Future<Output = Result<DeleteModelOutput, SdkError<DeleteModelError>>> {
        (*self).delete_model(builder)
    }
    fn delete_request_validator(&self, builder: DeleteRequestValidatorInputBuilder) -> impl Future<Output = Result<DeleteRequestValidatorOutput, SdkError<DeleteRequestValidatorError>>> {
        (*self).delete_request_validator(builder)
    }
    fn delete_resource(&self, builder: DeleteResourceInputBuilder) -> impl Future<Output = Result<DeleteResourceOutput, SdkError<DeleteResourceError>>> {
        (*self).delete_resource(builder)
    }
    fn delete_rest_api(&self, builder: DeleteRestApiInputBuilder) -> impl Future<Output = Result<DeleteRestApiOutput, SdkError<DeleteRestApiError>>> {
        (*self).delete_rest_api(builder)
    }
    fn delete_stage(&self, builder: DeleteStageInputBuilder) -> impl Future<Output = Result<DeleteStageOutput, SdkError<DeleteStageError>>> {
        (*self).delete_stage(builder)
    }
    fn delete_usage_plan(&self, builder: DeleteUsagePlanInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanOutput, SdkError<DeleteUsagePlanError>>> {
        (*self).delete_usage_plan(builder)
    }
    fn delete_usage_plan_key(&self, builder: DeleteUsagePlanKeyInputBuilder) -> impl Future<Output = Result<DeleteUsagePlanKeyOutput, SdkError<DeleteUsagePlanKeyError>>> {
        (*self).delete_usage_plan_key(builder)
    }
    fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> impl Future<Output = Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>> {
        (*self).delete_vpc_link(builder)
    }
    fn flush_stage_authorizers_cache(&self, builder: FlushStageAuthorizersCacheInputBuilder) -> impl Future<Output = Result<FlushStageAuthorizersCacheOutput, SdkError<FlushStageAuthorizersCacheError>>> {
        (*self).flush_stage_authorizers_cache(builder)
    }
    fn flush_stage_cache(&self, builder: FlushStageCacheInputBuilder) -> impl Future<Output = Result<FlushStageCacheOutput, SdkError<FlushStageCacheError>>> {
        (*self).flush_stage_cache(builder)
    }
    fn generate_client_certificate(&self, builder: GenerateClientCertificateInputBuilder) -> impl Future<Output = Result<GenerateClientCertificateOutput, SdkError<GenerateClientCertificateError>>> {
        (*self).generate_client_certificate(builder)
    }
    fn get_account(&self, builder: GetAccountInputBuilder) -> impl Future<Output = Result<GetAccountOutput, SdkError<GetAccountError>>> {
        (*self).get_account(builder)
    }
    fn get_api_key(&self, builder: GetApiKeyInputBuilder) -> impl Future<Output = Result<GetApiKeyOutput, SdkError<GetApiKeyError>>> {
        (*self).get_api_key(builder)
    }
    fn get_api_keys(&self, builder: GetApiKeysInputBuilder) -> impl Future<Output = Result<GetApiKeysOutput, SdkError<GetApiKeysError>>> {
        (*self).get_api_keys(builder)
    }
    fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> impl Future<Output = Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>> {
        (*self).get_authorizer(builder)
    }
    fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> impl Future<Output = Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>> {
        (*self).get_authorizers(builder)
    }
    fn get_base_path_mapping(&self, builder: GetBasePathMappingInputBuilder) -> impl Future<Output = Result<GetBasePathMappingOutput, SdkError<GetBasePathMappingError>>> {
        (*self).get_base_path_mapping(builder)
    }
    fn get_base_path_mappings(&self, builder: GetBasePathMappingsInputBuilder) -> impl Future<Output = Result<GetBasePathMappingsOutput, SdkError<GetBasePathMappingsError>>> {
        (*self).get_base_path_mappings(builder)
    }
    fn get_client_certificate(&self, builder: GetClientCertificateInputBuilder) -> impl Future<Output = Result<GetClientCertificateOutput, SdkError<GetClientCertificateError>>> {
        (*self).get_client_certificate(builder)
    }
    fn get_client_certificates(&self, builder: GetClientCertificatesInputBuilder) -> impl Future<Output = Result<GetClientCertificatesOutput, SdkError<GetClientCertificatesError>>> {
        (*self).get_client_certificates(builder)
    }
    fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> impl Future<Output = Result<GetDeploymentOutput, SdkError<GetDeploymentError>>> {
        (*self).get_deployment(builder)
    }
    fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> impl Future<Output = Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>> {
        (*self).get_deployments(builder)
    }
    fn get_documentation_part(&self, builder: GetDocumentationPartInputBuilder) -> impl Future<Output = Result<GetDocumentationPartOutput, SdkError<GetDocumentationPartError>>> {
        (*self).get_documentation_part(builder)
    }
    fn get_documentation_parts(&self, builder: GetDocumentationPartsInputBuilder) -> impl Future<Output = Result<GetDocumentationPartsOutput, SdkError<GetDocumentationPartsError>>> {
        (*self).get_documentation_parts(builder)
    }
    fn get_documentation_version(&self, builder: GetDocumentationVersionInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionOutput, SdkError<GetDocumentationVersionError>>> {
        (*self).get_documentation_version(builder)
    }
    fn get_documentation_versions(&self, builder: GetDocumentationVersionsInputBuilder) -> impl Future<Output = Result<GetDocumentationVersionsOutput, SdkError<GetDocumentationVersionsError>>> {
        (*self).get_documentation_versions(builder)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        (*self).get_domain_name(builder)
    }
    fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> impl Future<Output = Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>> {
        (*self).get_domain_names(builder)
    }
    fn get_export(&self, builder: GetExportInputBuilder) -> impl Future<Output = Result<GetExportOutput, SdkError<GetExportError>>> {
        (*self).get_export(builder)
    }
    fn get_gateway_response(&self, builder: GetGatewayResponseInputBuilder) -> impl Future<Output = Result<GetGatewayResponseOutput, SdkError<GetGatewayResponseError>>> {
        (*self).get_gateway_response(builder)
    }
    fn get_gateway_responses(&self, builder: GetGatewayResponsesInputBuilder) -> impl Future<Output = Result<GetGatewayResponsesOutput, SdkError<GetGatewayResponsesError>>> {
        (*self).get_gateway_responses(builder)
    }
    fn get_integration(&self, builder: GetIntegrationInputBuilder) -> impl Future<Output = Result<GetIntegrationOutput, SdkError<GetIntegrationError>>> {
        (*self).get_integration(builder)
    }
    fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> impl Future<Output = Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>> {
        (*self).get_integration_response(builder)
    }
    fn get_method(&self, builder: GetMethodInputBuilder) -> impl Future<Output = Result<GetMethodOutput, SdkError<GetMethodError>>> {
        (*self).get_method(builder)
    }
    fn get_method_response(&self, builder: GetMethodResponseInputBuilder) -> impl Future<Output = Result<GetMethodResponseOutput, SdkError<GetMethodResponseError>>> {
        (*self).get_method_response(builder)
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
    fn get_request_validator(&self, builder: GetRequestValidatorInputBuilder) -> impl Future<Output = Result<GetRequestValidatorOutput, SdkError<GetRequestValidatorError>>> {
        (*self).get_request_validator(builder)
    }
    fn get_request_validators(&self, builder: GetRequestValidatorsInputBuilder) -> impl Future<Output = Result<GetRequestValidatorsOutput, SdkError<GetRequestValidatorsError>>> {
        (*self).get_request_validators(builder)
    }
    fn get_resource(&self, builder: GetResourceInputBuilder) -> impl Future<Output = Result<GetResourceOutput, SdkError<GetResourceError>>> {
        (*self).get_resource(builder)
    }
    fn get_resources(&self, builder: GetResourcesInputBuilder) -> impl Future<Output = Result<GetResourcesOutput, SdkError<GetResourcesError>>> {
        (*self).get_resources(builder)
    }
    fn get_rest_api(&self, builder: GetRestApiInputBuilder) -> impl Future<Output = Result<GetRestApiOutput, SdkError<GetRestApiError>>> {
        (*self).get_rest_api(builder)
    }
    fn get_rest_apis(&self, builder: GetRestApisInputBuilder) -> impl Future<Output = Result<GetRestApisOutput, SdkError<GetRestApisError>>> {
        (*self).get_rest_apis(builder)
    }
    fn get_sdk(&self, builder: GetSdkInputBuilder) -> impl Future<Output = Result<GetSdkOutput, SdkError<GetSdkError>>> {
        (*self).get_sdk(builder)
    }
    fn get_sdk_type(&self, builder: GetSdkTypeInputBuilder) -> impl Future<Output = Result<GetSdkTypeOutput, SdkError<GetSdkTypeError>>> {
        (*self).get_sdk_type(builder)
    }
    fn get_sdk_types(&self, builder: GetSdkTypesInputBuilder) -> impl Future<Output = Result<GetSdkTypesOutput, SdkError<GetSdkTypesError>>> {
        (*self).get_sdk_types(builder)
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
    fn get_usage(&self, builder: GetUsageInputBuilder) -> impl Future<Output = Result<GetUsageOutput, SdkError<GetUsageError>>> {
        (*self).get_usage(builder)
    }
    fn get_usage_plan(&self, builder: GetUsagePlanInputBuilder) -> impl Future<Output = Result<GetUsagePlanOutput, SdkError<GetUsagePlanError>>> {
        (*self).get_usage_plan(builder)
    }
    fn get_usage_plan_key(&self, builder: GetUsagePlanKeyInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeyOutput, SdkError<GetUsagePlanKeyError>>> {
        (*self).get_usage_plan_key(builder)
    }
    fn get_usage_plan_keys(&self, builder: GetUsagePlanKeysInputBuilder) -> impl Future<Output = Result<GetUsagePlanKeysOutput, SdkError<GetUsagePlanKeysError>>> {
        (*self).get_usage_plan_keys(builder)
    }
    fn get_usage_plans(&self, builder: GetUsagePlansInputBuilder) -> impl Future<Output = Result<GetUsagePlansOutput, SdkError<GetUsagePlansError>>> {
        (*self).get_usage_plans(builder)
    }
    fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> impl Future<Output = Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>> {
        (*self).get_vpc_link(builder)
    }
    fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> impl Future<Output = Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>> {
        (*self).get_vpc_links(builder)
    }
    fn import_api_keys(&self, builder: ImportApiKeysInputBuilder) -> impl Future<Output = Result<ImportApiKeysOutput, SdkError<ImportApiKeysError>>> {
        (*self).import_api_keys(builder)
    }
    fn import_documentation_parts(&self, builder: ImportDocumentationPartsInputBuilder) -> impl Future<Output = Result<ImportDocumentationPartsOutput, SdkError<ImportDocumentationPartsError>>> {
        (*self).import_documentation_parts(builder)
    }
    fn import_rest_api(&self, builder: ImportRestApiInputBuilder) -> impl Future<Output = Result<ImportRestApiOutput, SdkError<ImportRestApiError>>> {
        (*self).import_rest_api(builder)
    }
    fn put_gateway_response(&self, builder: PutGatewayResponseInputBuilder) -> impl Future<Output = Result<PutGatewayResponseOutput, SdkError<PutGatewayResponseError>>> {
        (*self).put_gateway_response(builder)
    }
    fn put_integration(&self, builder: PutIntegrationInputBuilder) -> impl Future<Output = Result<PutIntegrationOutput, SdkError<PutIntegrationError>>> {
        (*self).put_integration(builder)
    }
    fn put_integration_response(&self, builder: PutIntegrationResponseInputBuilder) -> impl Future<Output = Result<PutIntegrationResponseOutput, SdkError<PutIntegrationResponseError>>> {
        (*self).put_integration_response(builder)
    }
    fn put_method(&self, builder: PutMethodInputBuilder) -> impl Future<Output = Result<PutMethodOutput, SdkError<PutMethodError>>> {
        (*self).put_method(builder)
    }
    fn put_method_response(&self, builder: PutMethodResponseInputBuilder) -> impl Future<Output = Result<PutMethodResponseOutput, SdkError<PutMethodResponseError>>> {
        (*self).put_method_response(builder)
    }
    fn put_rest_api(&self, builder: PutRestApiInputBuilder) -> impl Future<Output = Result<PutRestApiOutput, SdkError<PutRestApiError>>> {
        (*self).put_rest_api(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn test_invoke_authorizer(&self, builder: TestInvokeAuthorizerInputBuilder) -> impl Future<Output = Result<TestInvokeAuthorizerOutput, SdkError<TestInvokeAuthorizerError>>> {
        (*self).test_invoke_authorizer(builder)
    }
    fn test_invoke_method(&self, builder: TestInvokeMethodInputBuilder) -> impl Future<Output = Result<TestInvokeMethodOutput, SdkError<TestInvokeMethodError>>> {
        (*self).test_invoke_method(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_account(&self, builder: UpdateAccountInputBuilder) -> impl Future<Output = Result<UpdateAccountOutput, SdkError<UpdateAccountError>>> {
        (*self).update_account(builder)
    }
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>> {
        (*self).update_api_key(builder)
    }
    fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> impl Future<Output = Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>> {
        (*self).update_authorizer(builder)
    }
    fn update_base_path_mapping(&self, builder: UpdateBasePathMappingInputBuilder) -> impl Future<Output = Result<UpdateBasePathMappingOutput, SdkError<UpdateBasePathMappingError>>> {
        (*self).update_base_path_mapping(builder)
    }
    fn update_client_certificate(&self, builder: UpdateClientCertificateInputBuilder) -> impl Future<Output = Result<UpdateClientCertificateOutput, SdkError<UpdateClientCertificateError>>> {
        (*self).update_client_certificate(builder)
    }
    fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> impl Future<Output = Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>> {
        (*self).update_deployment(builder)
    }
    fn update_documentation_part(&self, builder: UpdateDocumentationPartInputBuilder) -> impl Future<Output = Result<UpdateDocumentationPartOutput, SdkError<UpdateDocumentationPartError>>> {
        (*self).update_documentation_part(builder)
    }
    fn update_documentation_version(&self, builder: UpdateDocumentationVersionInputBuilder) -> impl Future<Output = Result<UpdateDocumentationVersionOutput, SdkError<UpdateDocumentationVersionError>>> {
        (*self).update_documentation_version(builder)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        (*self).update_domain_name(builder)
    }
    fn update_gateway_response(&self, builder: UpdateGatewayResponseInputBuilder) -> impl Future<Output = Result<UpdateGatewayResponseOutput, SdkError<UpdateGatewayResponseError>>> {
        (*self).update_gateway_response(builder)
    }
    fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> impl Future<Output = Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>> {
        (*self).update_integration(builder)
    }
    fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> impl Future<Output = Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>> {
        (*self).update_integration_response(builder)
    }
    fn update_method(&self, builder: UpdateMethodInputBuilder) -> impl Future<Output = Result<UpdateMethodOutput, SdkError<UpdateMethodError>>> {
        (*self).update_method(builder)
    }
    fn update_method_response(&self, builder: UpdateMethodResponseInputBuilder) -> impl Future<Output = Result<UpdateMethodResponseOutput, SdkError<UpdateMethodResponseError>>> {
        (*self).update_method_response(builder)
    }
    fn update_model(&self, builder: UpdateModelInputBuilder) -> impl Future<Output = Result<UpdateModelOutput, SdkError<UpdateModelError>>> {
        (*self).update_model(builder)
    }
    fn update_request_validator(&self, builder: UpdateRequestValidatorInputBuilder) -> impl Future<Output = Result<UpdateRequestValidatorOutput, SdkError<UpdateRequestValidatorError>>> {
        (*self).update_request_validator(builder)
    }
    fn update_resource(&self, builder: UpdateResourceInputBuilder) -> impl Future<Output = Result<UpdateResourceOutput, SdkError<UpdateResourceError>>> {
        (*self).update_resource(builder)
    }
    fn update_rest_api(&self, builder: UpdateRestApiInputBuilder) -> impl Future<Output = Result<UpdateRestApiOutput, SdkError<UpdateRestApiError>>> {
        (*self).update_rest_api(builder)
    }
    fn update_stage(&self, builder: UpdateStageInputBuilder) -> impl Future<Output = Result<UpdateStageOutput, SdkError<UpdateStageError>>> {
        (*self).update_stage(builder)
    }
    fn update_usage(&self, builder: UpdateUsageInputBuilder) -> impl Future<Output = Result<UpdateUsageOutput, SdkError<UpdateUsageError>>> {
        (*self).update_usage(builder)
    }
    fn update_usage_plan(&self, builder: UpdateUsagePlanInputBuilder) -> impl Future<Output = Result<UpdateUsagePlanOutput, SdkError<UpdateUsagePlanError>>> {
        (*self).update_usage_plan(builder)
    }
    fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> impl Future<Output = Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>> {
        (*self).update_vpc_link(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edApiGatewayClient {}
    impl ApiGatewayClient for edApiGatewayClient {
        async fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>;
        async fn create_authorizer(&self, builder: CreateAuthorizerInputBuilder) -> Result<CreateAuthorizerOutput, SdkError<CreateAuthorizerError>>;
        async fn create_base_path_mapping(&self, builder: CreateBasePathMappingInputBuilder) -> Result<CreateBasePathMappingOutput, SdkError<CreateBasePathMappingError>>;
        async fn create_deployment(&self, builder: CreateDeploymentInputBuilder) -> Result<CreateDeploymentOutput, SdkError<CreateDeploymentError>>;
        async fn create_documentation_part(&self, builder: CreateDocumentationPartInputBuilder) -> Result<CreateDocumentationPartOutput, SdkError<CreateDocumentationPartError>>;
        async fn create_documentation_version(&self, builder: CreateDocumentationVersionInputBuilder) -> Result<CreateDocumentationVersionOutput, SdkError<CreateDocumentationVersionError>>;
        async fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>;
        async fn create_model(&self, builder: CreateModelInputBuilder) -> Result<CreateModelOutput, SdkError<CreateModelError>>;
        async fn create_request_validator(&self, builder: CreateRequestValidatorInputBuilder) -> Result<CreateRequestValidatorOutput, SdkError<CreateRequestValidatorError>>;
        async fn create_resource(&self, builder: CreateResourceInputBuilder) -> Result<CreateResourceOutput, SdkError<CreateResourceError>>;
        async fn create_rest_api(&self, builder: CreateRestApiInputBuilder) -> Result<CreateRestApiOutput, SdkError<CreateRestApiError>>;
        async fn create_stage(&self, builder: CreateStageInputBuilder) -> Result<CreateStageOutput, SdkError<CreateStageError>>;
        async fn create_usage_plan(&self, builder: CreateUsagePlanInputBuilder) -> Result<CreateUsagePlanOutput, SdkError<CreateUsagePlanError>>;
        async fn create_usage_plan_key(&self, builder: CreateUsagePlanKeyInputBuilder) -> Result<CreateUsagePlanKeyOutput, SdkError<CreateUsagePlanKeyError>>;
        async fn create_vpc_link(&self, builder: CreateVpcLinkInputBuilder) -> Result<CreateVpcLinkOutput, SdkError<CreateVpcLinkError>>;
        async fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>;
        async fn delete_authorizer(&self, builder: DeleteAuthorizerInputBuilder) -> Result<DeleteAuthorizerOutput, SdkError<DeleteAuthorizerError>>;
        async fn delete_base_path_mapping(&self, builder: DeleteBasePathMappingInputBuilder) -> Result<DeleteBasePathMappingOutput, SdkError<DeleteBasePathMappingError>>;
        async fn delete_client_certificate(&self, builder: DeleteClientCertificateInputBuilder) -> Result<DeleteClientCertificateOutput, SdkError<DeleteClientCertificateError>>;
        async fn delete_deployment(&self, builder: DeleteDeploymentInputBuilder) -> Result<DeleteDeploymentOutput, SdkError<DeleteDeploymentError>>;
        async fn delete_documentation_part(&self, builder: DeleteDocumentationPartInputBuilder) -> Result<DeleteDocumentationPartOutput, SdkError<DeleteDocumentationPartError>>;
        async fn delete_documentation_version(&self, builder: DeleteDocumentationVersionInputBuilder) -> Result<DeleteDocumentationVersionOutput, SdkError<DeleteDocumentationVersionError>>;
        async fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>;
        async fn delete_gateway_response(&self, builder: DeleteGatewayResponseInputBuilder) -> Result<DeleteGatewayResponseOutput, SdkError<DeleteGatewayResponseError>>;
        async fn delete_integration(&self, builder: DeleteIntegrationInputBuilder) -> Result<DeleteIntegrationOutput, SdkError<DeleteIntegrationError>>;
        async fn delete_integration_response(&self, builder: DeleteIntegrationResponseInputBuilder) -> Result<DeleteIntegrationResponseOutput, SdkError<DeleteIntegrationResponseError>>;
        async fn delete_method(&self, builder: DeleteMethodInputBuilder) -> Result<DeleteMethodOutput, SdkError<DeleteMethodError>>;
        async fn delete_method_response(&self, builder: DeleteMethodResponseInputBuilder) -> Result<DeleteMethodResponseOutput, SdkError<DeleteMethodResponseError>>;
        async fn delete_model(&self, builder: DeleteModelInputBuilder) -> Result<DeleteModelOutput, SdkError<DeleteModelError>>;
        async fn delete_request_validator(&self, builder: DeleteRequestValidatorInputBuilder) -> Result<DeleteRequestValidatorOutput, SdkError<DeleteRequestValidatorError>>;
        async fn delete_resource(&self, builder: DeleteResourceInputBuilder) -> Result<DeleteResourceOutput, SdkError<DeleteResourceError>>;
        async fn delete_rest_api(&self, builder: DeleteRestApiInputBuilder) -> Result<DeleteRestApiOutput, SdkError<DeleteRestApiError>>;
        async fn delete_stage(&self, builder: DeleteStageInputBuilder) -> Result<DeleteStageOutput, SdkError<DeleteStageError>>;
        async fn delete_usage_plan(&self, builder: DeleteUsagePlanInputBuilder) -> Result<DeleteUsagePlanOutput, SdkError<DeleteUsagePlanError>>;
        async fn delete_usage_plan_key(&self, builder: DeleteUsagePlanKeyInputBuilder) -> Result<DeleteUsagePlanKeyOutput, SdkError<DeleteUsagePlanKeyError>>;
        async fn delete_vpc_link(&self, builder: DeleteVpcLinkInputBuilder) -> Result<DeleteVpcLinkOutput, SdkError<DeleteVpcLinkError>>;
        async fn flush_stage_authorizers_cache(&self, builder: FlushStageAuthorizersCacheInputBuilder) -> Result<FlushStageAuthorizersCacheOutput, SdkError<FlushStageAuthorizersCacheError>>;
        async fn flush_stage_cache(&self, builder: FlushStageCacheInputBuilder) -> Result<FlushStageCacheOutput, SdkError<FlushStageCacheError>>;
        async fn generate_client_certificate(&self, builder: GenerateClientCertificateInputBuilder) -> Result<GenerateClientCertificateOutput, SdkError<GenerateClientCertificateError>>;
        async fn get_account(&self, builder: GetAccountInputBuilder) -> Result<GetAccountOutput, SdkError<GetAccountError>>;
        async fn get_api_key(&self, builder: GetApiKeyInputBuilder) -> Result<GetApiKeyOutput, SdkError<GetApiKeyError>>;
        async fn get_api_keys(&self, builder: GetApiKeysInputBuilder) -> Result<GetApiKeysOutput, SdkError<GetApiKeysError>>;
        async fn get_authorizer(&self, builder: GetAuthorizerInputBuilder) -> Result<GetAuthorizerOutput, SdkError<GetAuthorizerError>>;
        async fn get_authorizers(&self, builder: GetAuthorizersInputBuilder) -> Result<GetAuthorizersOutput, SdkError<GetAuthorizersError>>;
        async fn get_base_path_mapping(&self, builder: GetBasePathMappingInputBuilder) -> Result<GetBasePathMappingOutput, SdkError<GetBasePathMappingError>>;
        async fn get_base_path_mappings(&self, builder: GetBasePathMappingsInputBuilder) -> Result<GetBasePathMappingsOutput, SdkError<GetBasePathMappingsError>>;
        async fn get_client_certificate(&self, builder: GetClientCertificateInputBuilder) -> Result<GetClientCertificateOutput, SdkError<GetClientCertificateError>>;
        async fn get_client_certificates(&self, builder: GetClientCertificatesInputBuilder) -> Result<GetClientCertificatesOutput, SdkError<GetClientCertificatesError>>;
        async fn get_deployment(&self, builder: GetDeploymentInputBuilder) -> Result<GetDeploymentOutput, SdkError<GetDeploymentError>>;
        async fn get_deployments(&self, builder: GetDeploymentsInputBuilder) -> Result<GetDeploymentsOutput, SdkError<GetDeploymentsError>>;
        async fn get_documentation_part(&self, builder: GetDocumentationPartInputBuilder) -> Result<GetDocumentationPartOutput, SdkError<GetDocumentationPartError>>;
        async fn get_documentation_parts(&self, builder: GetDocumentationPartsInputBuilder) -> Result<GetDocumentationPartsOutput, SdkError<GetDocumentationPartsError>>;
        async fn get_documentation_version(&self, builder: GetDocumentationVersionInputBuilder) -> Result<GetDocumentationVersionOutput, SdkError<GetDocumentationVersionError>>;
        async fn get_documentation_versions(&self, builder: GetDocumentationVersionsInputBuilder) -> Result<GetDocumentationVersionsOutput, SdkError<GetDocumentationVersionsError>>;
        async fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> Result<GetDomainNameOutput, SdkError<GetDomainNameError>>;
        async fn get_domain_names(&self, builder: GetDomainNamesInputBuilder) -> Result<GetDomainNamesOutput, SdkError<GetDomainNamesError>>;
        async fn get_export(&self, builder: GetExportInputBuilder) -> Result<GetExportOutput, SdkError<GetExportError>>;
        async fn get_gateway_response(&self, builder: GetGatewayResponseInputBuilder) -> Result<GetGatewayResponseOutput, SdkError<GetGatewayResponseError>>;
        async fn get_gateway_responses(&self, builder: GetGatewayResponsesInputBuilder) -> Result<GetGatewayResponsesOutput, SdkError<GetGatewayResponsesError>>;
        async fn get_integration(&self, builder: GetIntegrationInputBuilder) -> Result<GetIntegrationOutput, SdkError<GetIntegrationError>>;
        async fn get_integration_response(&self, builder: GetIntegrationResponseInputBuilder) -> Result<GetIntegrationResponseOutput, SdkError<GetIntegrationResponseError>>;
        async fn get_method(&self, builder: GetMethodInputBuilder) -> Result<GetMethodOutput, SdkError<GetMethodError>>;
        async fn get_method_response(&self, builder: GetMethodResponseInputBuilder) -> Result<GetMethodResponseOutput, SdkError<GetMethodResponseError>>;
        async fn get_model(&self, builder: GetModelInputBuilder) -> Result<GetModelOutput, SdkError<GetModelError>>;
        async fn get_model_template(&self, builder: GetModelTemplateInputBuilder) -> Result<GetModelTemplateOutput, SdkError<GetModelTemplateError>>;
        async fn get_models(&self, builder: GetModelsInputBuilder) -> Result<GetModelsOutput, SdkError<GetModelsError>>;
        async fn get_request_validator(&self, builder: GetRequestValidatorInputBuilder) -> Result<GetRequestValidatorOutput, SdkError<GetRequestValidatorError>>;
        async fn get_request_validators(&self, builder: GetRequestValidatorsInputBuilder) -> Result<GetRequestValidatorsOutput, SdkError<GetRequestValidatorsError>>;
        async fn get_resource(&self, builder: GetResourceInputBuilder) -> Result<GetResourceOutput, SdkError<GetResourceError>>;
        async fn get_resources(&self, builder: GetResourcesInputBuilder) -> Result<GetResourcesOutput, SdkError<GetResourcesError>>;
        async fn get_rest_api(&self, builder: GetRestApiInputBuilder) -> Result<GetRestApiOutput, SdkError<GetRestApiError>>;
        async fn get_rest_apis(&self, builder: GetRestApisInputBuilder) -> Result<GetRestApisOutput, SdkError<GetRestApisError>>;
        async fn get_sdk(&self, builder: GetSdkInputBuilder) -> Result<GetSdkOutput, SdkError<GetSdkError>>;
        async fn get_sdk_type(&self, builder: GetSdkTypeInputBuilder) -> Result<GetSdkTypeOutput, SdkError<GetSdkTypeError>>;
        async fn get_sdk_types(&self, builder: GetSdkTypesInputBuilder) -> Result<GetSdkTypesOutput, SdkError<GetSdkTypesError>>;
        async fn get_stage(&self, builder: GetStageInputBuilder) -> Result<GetStageOutput, SdkError<GetStageError>>;
        async fn get_stages(&self, builder: GetStagesInputBuilder) -> Result<GetStagesOutput, SdkError<GetStagesError>>;
        async fn get_tags(&self, builder: GetTagsInputBuilder) -> Result<GetTagsOutput, SdkError<GetTagsError>>;
        async fn get_usage(&self, builder: GetUsageInputBuilder) -> Result<GetUsageOutput, SdkError<GetUsageError>>;
        async fn get_usage_plan(&self, builder: GetUsagePlanInputBuilder) -> Result<GetUsagePlanOutput, SdkError<GetUsagePlanError>>;
        async fn get_usage_plan_key(&self, builder: GetUsagePlanKeyInputBuilder) -> Result<GetUsagePlanKeyOutput, SdkError<GetUsagePlanKeyError>>;
        async fn get_usage_plan_keys(&self, builder: GetUsagePlanKeysInputBuilder) -> Result<GetUsagePlanKeysOutput, SdkError<GetUsagePlanKeysError>>;
        async fn get_usage_plans(&self, builder: GetUsagePlansInputBuilder) -> Result<GetUsagePlansOutput, SdkError<GetUsagePlansError>>;
        async fn get_vpc_link(&self, builder: GetVpcLinkInputBuilder) -> Result<GetVpcLinkOutput, SdkError<GetVpcLinkError>>;
        async fn get_vpc_links(&self, builder: GetVpcLinksInputBuilder) -> Result<GetVpcLinksOutput, SdkError<GetVpcLinksError>>;
        async fn import_api_keys(&self, builder: ImportApiKeysInputBuilder) -> Result<ImportApiKeysOutput, SdkError<ImportApiKeysError>>;
        async fn import_documentation_parts(&self, builder: ImportDocumentationPartsInputBuilder) -> Result<ImportDocumentationPartsOutput, SdkError<ImportDocumentationPartsError>>;
        async fn import_rest_api(&self, builder: ImportRestApiInputBuilder) -> Result<ImportRestApiOutput, SdkError<ImportRestApiError>>;
        async fn put_gateway_response(&self, builder: PutGatewayResponseInputBuilder) -> Result<PutGatewayResponseOutput, SdkError<PutGatewayResponseError>>;
        async fn put_integration(&self, builder: PutIntegrationInputBuilder) -> Result<PutIntegrationOutput, SdkError<PutIntegrationError>>;
        async fn put_integration_response(&self, builder: PutIntegrationResponseInputBuilder) -> Result<PutIntegrationResponseOutput, SdkError<PutIntegrationResponseError>>;
        async fn put_method(&self, builder: PutMethodInputBuilder) -> Result<PutMethodOutput, SdkError<PutMethodError>>;
        async fn put_method_response(&self, builder: PutMethodResponseInputBuilder) -> Result<PutMethodResponseOutput, SdkError<PutMethodResponseError>>;
        async fn put_rest_api(&self, builder: PutRestApiInputBuilder) -> Result<PutRestApiOutput, SdkError<PutRestApiError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn test_invoke_authorizer(&self, builder: TestInvokeAuthorizerInputBuilder) -> Result<TestInvokeAuthorizerOutput, SdkError<TestInvokeAuthorizerError>>;
        async fn test_invoke_method(&self, builder: TestInvokeMethodInputBuilder) -> Result<TestInvokeMethodOutput, SdkError<TestInvokeMethodError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_account(&self, builder: UpdateAccountInputBuilder) -> Result<UpdateAccountOutput, SdkError<UpdateAccountError>>;
        async fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>;
        async fn update_authorizer(&self, builder: UpdateAuthorizerInputBuilder) -> Result<UpdateAuthorizerOutput, SdkError<UpdateAuthorizerError>>;
        async fn update_base_path_mapping(&self, builder: UpdateBasePathMappingInputBuilder) -> Result<UpdateBasePathMappingOutput, SdkError<UpdateBasePathMappingError>>;
        async fn update_client_certificate(&self, builder: UpdateClientCertificateInputBuilder) -> Result<UpdateClientCertificateOutput, SdkError<UpdateClientCertificateError>>;
        async fn update_deployment(&self, builder: UpdateDeploymentInputBuilder) -> Result<UpdateDeploymentOutput, SdkError<UpdateDeploymentError>>;
        async fn update_documentation_part(&self, builder: UpdateDocumentationPartInputBuilder) -> Result<UpdateDocumentationPartOutput, SdkError<UpdateDocumentationPartError>>;
        async fn update_documentation_version(&self, builder: UpdateDocumentationVersionInputBuilder) -> Result<UpdateDocumentationVersionOutput, SdkError<UpdateDocumentationVersionError>>;
        async fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>;
        async fn update_gateway_response(&self, builder: UpdateGatewayResponseInputBuilder) -> Result<UpdateGatewayResponseOutput, SdkError<UpdateGatewayResponseError>>;
        async fn update_integration(&self, builder: UpdateIntegrationInputBuilder) -> Result<UpdateIntegrationOutput, SdkError<UpdateIntegrationError>>;
        async fn update_integration_response(&self, builder: UpdateIntegrationResponseInputBuilder) -> Result<UpdateIntegrationResponseOutput, SdkError<UpdateIntegrationResponseError>>;
        async fn update_method(&self, builder: UpdateMethodInputBuilder) -> Result<UpdateMethodOutput, SdkError<UpdateMethodError>>;
        async fn update_method_response(&self, builder: UpdateMethodResponseInputBuilder) -> Result<UpdateMethodResponseOutput, SdkError<UpdateMethodResponseError>>;
        async fn update_model(&self, builder: UpdateModelInputBuilder) -> Result<UpdateModelOutput, SdkError<UpdateModelError>>;
        async fn update_request_validator(&self, builder: UpdateRequestValidatorInputBuilder) -> Result<UpdateRequestValidatorOutput, SdkError<UpdateRequestValidatorError>>;
        async fn update_resource(&self, builder: UpdateResourceInputBuilder) -> Result<UpdateResourceOutput, SdkError<UpdateResourceError>>;
        async fn update_rest_api(&self, builder: UpdateRestApiInputBuilder) -> Result<UpdateRestApiOutput, SdkError<UpdateRestApiError>>;
        async fn update_stage(&self, builder: UpdateStageInputBuilder) -> Result<UpdateStageOutput, SdkError<UpdateStageError>>;
        async fn update_usage(&self, builder: UpdateUsageInputBuilder) -> Result<UpdateUsageOutput, SdkError<UpdateUsageError>>;
        async fn update_usage_plan(&self, builder: UpdateUsagePlanInputBuilder) -> Result<UpdateUsagePlanOutput, SdkError<UpdateUsagePlanError>>;
        async fn update_vpc_link(&self, builder: UpdateVpcLinkInputBuilder) -> Result<UpdateVpcLinkOutput, SdkError<UpdateVpcLinkError>>;
    }
}
