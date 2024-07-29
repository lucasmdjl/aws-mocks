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
use aws_sdk_appsync::operation::associate_api::{builders::*, *};
use aws_sdk_appsync::operation::associate_merged_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::associate_source_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::create_api_cache::{builders::*, *};
use aws_sdk_appsync::operation::create_api_key::{builders::*, *};
use aws_sdk_appsync::operation::create_data_source::{builders::*, *};
use aws_sdk_appsync::operation::create_domain_name::{builders::*, *};
use aws_sdk_appsync::operation::create_function::{builders::*, *};
use aws_sdk_appsync::operation::create_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::create_resolver::{builders::*, *};
use aws_sdk_appsync::operation::create_type::{builders::*, *};
use aws_sdk_appsync::operation::delete_api_cache::{builders::*, *};
use aws_sdk_appsync::operation::delete_api_key::{builders::*, *};
use aws_sdk_appsync::operation::delete_data_source::{builders::*, *};
use aws_sdk_appsync::operation::delete_domain_name::{builders::*, *};
use aws_sdk_appsync::operation::delete_function::{builders::*, *};
use aws_sdk_appsync::operation::delete_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::delete_resolver::{builders::*, *};
use aws_sdk_appsync::operation::delete_type::{builders::*, *};
use aws_sdk_appsync::operation::disassociate_api::{builders::*, *};
use aws_sdk_appsync::operation::disassociate_merged_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::disassociate_source_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::evaluate_code::{builders::*, *};
use aws_sdk_appsync::operation::evaluate_mapping_template::{builders::*, *};
use aws_sdk_appsync::operation::flush_api_cache::{builders::*, *};
use aws_sdk_appsync::operation::get_api_association::{builders::*, *};
use aws_sdk_appsync::operation::get_api_cache::{builders::*, *};
use aws_sdk_appsync::operation::get_data_source::{builders::*, *};
use aws_sdk_appsync::operation::get_data_source_introspection::{builders::*, *};
use aws_sdk_appsync::operation::get_domain_name::{builders::*, *};
use aws_sdk_appsync::operation::get_function::{builders::*, *};
use aws_sdk_appsync::operation::get_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::get_graphql_api_environment_variables::{builders::*, *};
use aws_sdk_appsync::operation::get_introspection_schema::{builders::*, *};
use aws_sdk_appsync::operation::get_resolver::{builders::*, *};
use aws_sdk_appsync::operation::get_schema_creation_status::{builders::*, *};
use aws_sdk_appsync::operation::get_source_api_association::{builders::*, *};
use aws_sdk_appsync::operation::get_type::{builders::*, *};
use aws_sdk_appsync::operation::list_api_keys::{builders::*, *};
use aws_sdk_appsync::operation::list_data_sources::{builders::*, *};
use aws_sdk_appsync::operation::list_domain_names::{builders::*, *};
use aws_sdk_appsync::operation::list_functions::{builders::*, *};
use aws_sdk_appsync::operation::list_graphql_apis::{builders::*, *};
use aws_sdk_appsync::operation::list_resolvers::{builders::*, *};
use aws_sdk_appsync::operation::list_resolvers_by_function::{builders::*, *};
use aws_sdk_appsync::operation::list_source_api_associations::{builders::*, *};
use aws_sdk_appsync::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appsync::operation::list_types::{builders::*, *};
use aws_sdk_appsync::operation::list_types_by_association::{builders::*, *};
use aws_sdk_appsync::operation::put_graphql_api_environment_variables::{builders::*, *};
use aws_sdk_appsync::operation::start_data_source_introspection::{builders::*, *};
use aws_sdk_appsync::operation::start_schema_creation::{builders::*, *};
use aws_sdk_appsync::operation::start_schema_merge::{builders::*, *};
use aws_sdk_appsync::operation::tag_resource::{builders::*, *};
use aws_sdk_appsync::operation::untag_resource::{builders::*, *};
use aws_sdk_appsync::operation::update_api_cache::{builders::*, *};
use aws_sdk_appsync::operation::update_api_key::{builders::*, *};
use aws_sdk_appsync::operation::update_data_source::{builders::*, *};
use aws_sdk_appsync::operation::update_domain_name::{builders::*, *};
use aws_sdk_appsync::operation::update_function::{builders::*, *};
use aws_sdk_appsync::operation::update_graphql_api::{builders::*, *};
use aws_sdk_appsync::operation::update_resolver::{builders::*, *};
use aws_sdk_appsync::operation::update_source_api_association::{builders::*, *};
use aws_sdk_appsync::operation::update_type::{builders::*, *};
use aws_sdk_appsync::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_appsync::Client;
use std::ops::Deref;

pub use aws_sdk_appsync::*;

pub struct AppSyncClientImpl(Client);
impl AppSyncClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppSyncClient {
    fn associate_api(&self, builder: AssociateApiInputBuilder) -> impl Future<Output = Result<AssociateApiOutput, SdkError<AssociateApiError>>>;
    fn associate_merged_graphql_api(&self, builder: AssociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateMergedGraphqlApiOutput, SdkError<AssociateMergedGraphqlApiError>>>;
    fn associate_source_graphql_api(&self, builder: AssociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateSourceGraphqlApiOutput, SdkError<AssociateSourceGraphqlApiError>>>;
    fn create_api_cache(&self, builder: CreateApiCacheInputBuilder) -> impl Future<Output = Result<CreateApiCacheOutput, SdkError<CreateApiCacheError>>>;
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>>;
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>>;
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>>;
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>>;
    fn create_graphql_api(&self, builder: CreateGraphqlApiInputBuilder) -> impl Future<Output = Result<CreateGraphqlApiOutput, SdkError<CreateGraphqlApiError>>>;
    fn create_resolver(&self, builder: CreateResolverInputBuilder) -> impl Future<Output = Result<CreateResolverOutput, SdkError<CreateResolverError>>>;
    fn create_type(&self, builder: CreateTypeInputBuilder) -> impl Future<Output = Result<CreateTypeOutput, SdkError<CreateTypeError>>>;
    fn delete_api_cache(&self, builder: DeleteApiCacheInputBuilder) -> impl Future<Output = Result<DeleteApiCacheOutput, SdkError<DeleteApiCacheError>>>;
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>>;
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>>;
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>>;
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>>;
    fn delete_graphql_api(&self, builder: DeleteGraphqlApiInputBuilder) -> impl Future<Output = Result<DeleteGraphqlApiOutput, SdkError<DeleteGraphqlApiError>>>;
    fn delete_resolver(&self, builder: DeleteResolverInputBuilder) -> impl Future<Output = Result<DeleteResolverOutput, SdkError<DeleteResolverError>>>;
    fn delete_type(&self, builder: DeleteTypeInputBuilder) -> impl Future<Output = Result<DeleteTypeOutput, SdkError<DeleteTypeError>>>;
    fn disassociate_api(&self, builder: DisassociateApiInputBuilder) -> impl Future<Output = Result<DisassociateApiOutput, SdkError<DisassociateApiError>>>;
    fn disassociate_merged_graphql_api(&self, builder: DisassociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateMergedGraphqlApiOutput, SdkError<DisassociateMergedGraphqlApiError>>>;
    fn disassociate_source_graphql_api(&self, builder: DisassociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateSourceGraphqlApiOutput, SdkError<DisassociateSourceGraphqlApiError>>>;
    fn evaluate_code(&self, builder: EvaluateCodeInputBuilder) -> impl Future<Output = Result<EvaluateCodeOutput, SdkError<EvaluateCodeError>>>;
    fn evaluate_mapping_template(&self, builder: EvaluateMappingTemplateInputBuilder) -> impl Future<Output = Result<EvaluateMappingTemplateOutput, SdkError<EvaluateMappingTemplateError>>>;
    fn flush_api_cache(&self, builder: FlushApiCacheInputBuilder) -> impl Future<Output = Result<FlushApiCacheOutput, SdkError<FlushApiCacheError>>>;
    fn get_api_association(&self, builder: GetApiAssociationInputBuilder) -> impl Future<Output = Result<GetApiAssociationOutput, SdkError<GetApiAssociationError>>>;
    fn get_api_cache(&self, builder: GetApiCacheInputBuilder) -> impl Future<Output = Result<GetApiCacheOutput, SdkError<GetApiCacheError>>>;
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>>;
    fn get_data_source_introspection(&self, builder: GetDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<GetDataSourceIntrospectionOutput, SdkError<GetDataSourceIntrospectionError>>>;
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>>;
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>>;
    fn get_graphql_api(&self, builder: GetGraphqlApiInputBuilder) -> impl Future<Output = Result<GetGraphqlApiOutput, SdkError<GetGraphqlApiError>>>;
    fn get_graphql_api_environment_variables(&self, builder: GetGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<GetGraphqlApiEnvironmentVariablesOutput, SdkError<GetGraphqlApiEnvironmentVariablesError>>>;
    fn get_introspection_schema(&self, builder: GetIntrospectionSchemaInputBuilder) -> impl Future<Output = Result<GetIntrospectionSchemaOutput, SdkError<GetIntrospectionSchemaError>>>;
    fn get_resolver(&self, builder: GetResolverInputBuilder) -> impl Future<Output = Result<GetResolverOutput, SdkError<GetResolverError>>>;
    fn get_schema_creation_status(&self, builder: GetSchemaCreationStatusInputBuilder) -> impl Future<Output = Result<GetSchemaCreationStatusOutput, SdkError<GetSchemaCreationStatusError>>>;
    fn get_source_api_association(&self, builder: GetSourceApiAssociationInputBuilder) -> impl Future<Output = Result<GetSourceApiAssociationOutput, SdkError<GetSourceApiAssociationError>>>;
    fn get_type(&self, builder: GetTypeInputBuilder) -> impl Future<Output = Result<GetTypeOutput, SdkError<GetTypeError>>>;
    fn list_api_keys(&self, builder: ListApiKeysInputBuilder) -> impl Future<Output = Result<ListApiKeysOutput, SdkError<ListApiKeysError>>>;
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>>;
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>>;
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>>;
    fn list_graphql_apis(&self, builder: ListGraphqlApisInputBuilder) -> impl Future<Output = Result<ListGraphqlApisOutput, SdkError<ListGraphqlApisError>>>;
    fn list_resolvers(&self, builder: ListResolversInputBuilder) -> impl Future<Output = Result<ListResolversOutput, SdkError<ListResolversError>>>;
    fn list_resolvers_by_function(&self, builder: ListResolversByFunctionInputBuilder) -> impl Future<Output = Result<ListResolversByFunctionOutput, SdkError<ListResolversByFunctionError>>>;
    fn list_source_api_associations(&self, builder: ListSourceApiAssociationsInputBuilder) -> impl Future<Output = Result<ListSourceApiAssociationsOutput, SdkError<ListSourceApiAssociationsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>>;
    fn list_types_by_association(&self, builder: ListTypesByAssociationInputBuilder) -> impl Future<Output = Result<ListTypesByAssociationOutput, SdkError<ListTypesByAssociationError>>>;
    fn put_graphql_api_environment_variables(&self, builder: PutGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<PutGraphqlApiEnvironmentVariablesOutput, SdkError<PutGraphqlApiEnvironmentVariablesError>>>;
    fn start_data_source_introspection(&self, builder: StartDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<StartDataSourceIntrospectionOutput, SdkError<StartDataSourceIntrospectionError>>>;
    fn start_schema_creation(&self, builder: StartSchemaCreationInputBuilder) -> impl Future<Output = Result<StartSchemaCreationOutput, SdkError<StartSchemaCreationError>>>;
    fn start_schema_merge(&self, builder: StartSchemaMergeInputBuilder) -> impl Future<Output = Result<StartSchemaMergeOutput, SdkError<StartSchemaMergeError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_api_cache(&self, builder: UpdateApiCacheInputBuilder) -> impl Future<Output = Result<UpdateApiCacheOutput, SdkError<UpdateApiCacheError>>>;
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>>;
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>>;
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>>;
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>>;
    fn update_graphql_api(&self, builder: UpdateGraphqlApiInputBuilder) -> impl Future<Output = Result<UpdateGraphqlApiOutput, SdkError<UpdateGraphqlApiError>>>;
    fn update_resolver(&self, builder: UpdateResolverInputBuilder) -> impl Future<Output = Result<UpdateResolverOutput, SdkError<UpdateResolverError>>>;
    fn update_source_api_association(&self, builder: UpdateSourceApiAssociationInputBuilder) -> impl Future<Output = Result<UpdateSourceApiAssociationOutput, SdkError<UpdateSourceApiAssociationError>>>;
    fn update_type(&self, builder: UpdateTypeInputBuilder) -> impl Future<Output = Result<UpdateTypeOutput, SdkError<UpdateTypeError>>>;
}
impl AppSyncClient for AppSyncClientImpl {
    fn associate_api(&self, builder: AssociateApiInputBuilder) -> impl Future<Output = Result<AssociateApiOutput, SdkError<AssociateApiError>>> {
        builder.send_with(&self.0)
    }
    fn associate_merged_graphql_api(&self, builder: AssociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateMergedGraphqlApiOutput, SdkError<AssociateMergedGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn associate_source_graphql_api(&self, builder: AssociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateSourceGraphqlApiOutput, SdkError<AssociateSourceGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn create_api_cache(&self, builder: CreateApiCacheInputBuilder) -> impl Future<Output = Result<CreateApiCacheOutput, SdkError<CreateApiCacheError>>> {
        builder.send_with(&self.0)
    }
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn create_graphql_api(&self, builder: CreateGraphqlApiInputBuilder) -> impl Future<Output = Result<CreateGraphqlApiOutput, SdkError<CreateGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn create_resolver(&self, builder: CreateResolverInputBuilder) -> impl Future<Output = Result<CreateResolverOutput, SdkError<CreateResolverError>>> {
        builder.send_with(&self.0)
    }
    fn create_type(&self, builder: CreateTypeInputBuilder) -> impl Future<Output = Result<CreateTypeOutput, SdkError<CreateTypeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_api_cache(&self, builder: DeleteApiCacheInputBuilder) -> impl Future<Output = Result<DeleteApiCacheOutput, SdkError<DeleteApiCacheError>>> {
        builder.send_with(&self.0)
    }
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_graphql_api(&self, builder: DeleteGraphqlApiInputBuilder) -> impl Future<Output = Result<DeleteGraphqlApiOutput, SdkError<DeleteGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resolver(&self, builder: DeleteResolverInputBuilder) -> impl Future<Output = Result<DeleteResolverOutput, SdkError<DeleteResolverError>>> {
        builder.send_with(&self.0)
    }
    fn delete_type(&self, builder: DeleteTypeInputBuilder) -> impl Future<Output = Result<DeleteTypeOutput, SdkError<DeleteTypeError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_api(&self, builder: DisassociateApiInputBuilder) -> impl Future<Output = Result<DisassociateApiOutput, SdkError<DisassociateApiError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_merged_graphql_api(&self, builder: DisassociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateMergedGraphqlApiOutput, SdkError<DisassociateMergedGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_source_graphql_api(&self, builder: DisassociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateSourceGraphqlApiOutput, SdkError<DisassociateSourceGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn evaluate_code(&self, builder: EvaluateCodeInputBuilder) -> impl Future<Output = Result<EvaluateCodeOutput, SdkError<EvaluateCodeError>>> {
        builder.send_with(&self.0)
    }
    fn evaluate_mapping_template(&self, builder: EvaluateMappingTemplateInputBuilder) -> impl Future<Output = Result<EvaluateMappingTemplateOutput, SdkError<EvaluateMappingTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn flush_api_cache(&self, builder: FlushApiCacheInputBuilder) -> impl Future<Output = Result<FlushApiCacheOutput, SdkError<FlushApiCacheError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_association(&self, builder: GetApiAssociationInputBuilder) -> impl Future<Output = Result<GetApiAssociationOutput, SdkError<GetApiAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn get_api_cache(&self, builder: GetApiCacheInputBuilder) -> impl Future<Output = Result<GetApiCacheOutput, SdkError<GetApiCacheError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn get_data_source_introspection(&self, builder: GetDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<GetDataSourceIntrospectionOutput, SdkError<GetDataSourceIntrospectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn get_graphql_api(&self, builder: GetGraphqlApiInputBuilder) -> impl Future<Output = Result<GetGraphqlApiOutput, SdkError<GetGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn get_graphql_api_environment_variables(&self, builder: GetGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<GetGraphqlApiEnvironmentVariablesOutput, SdkError<GetGraphqlApiEnvironmentVariablesError>>> {
        builder.send_with(&self.0)
    }
    fn get_introspection_schema(&self, builder: GetIntrospectionSchemaInputBuilder) -> impl Future<Output = Result<GetIntrospectionSchemaOutput, SdkError<GetIntrospectionSchemaError>>> {
        builder.send_with(&self.0)
    }
    fn get_resolver(&self, builder: GetResolverInputBuilder) -> impl Future<Output = Result<GetResolverOutput, SdkError<GetResolverError>>> {
        builder.send_with(&self.0)
    }
    fn get_schema_creation_status(&self, builder: GetSchemaCreationStatusInputBuilder) -> impl Future<Output = Result<GetSchemaCreationStatusOutput, SdkError<GetSchemaCreationStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_source_api_association(&self, builder: GetSourceApiAssociationInputBuilder) -> impl Future<Output = Result<GetSourceApiAssociationOutput, SdkError<GetSourceApiAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn get_type(&self, builder: GetTypeInputBuilder) -> impl Future<Output = Result<GetTypeOutput, SdkError<GetTypeError>>> {
        builder.send_with(&self.0)
    }
    fn list_api_keys(&self, builder: ListApiKeysInputBuilder) -> impl Future<Output = Result<ListApiKeysOutput, SdkError<ListApiKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        builder.send_with(&self.0)
    }
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>> {
        builder.send_with(&self.0)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_graphql_apis(&self, builder: ListGraphqlApisInputBuilder) -> impl Future<Output = Result<ListGraphqlApisOutput, SdkError<ListGraphqlApisError>>> {
        builder.send_with(&self.0)
    }
    fn list_resolvers(&self, builder: ListResolversInputBuilder) -> impl Future<Output = Result<ListResolversOutput, SdkError<ListResolversError>>> {
        builder.send_with(&self.0)
    }
    fn list_resolvers_by_function(&self, builder: ListResolversByFunctionInputBuilder) -> impl Future<Output = Result<ListResolversByFunctionOutput, SdkError<ListResolversByFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn list_source_api_associations(&self, builder: ListSourceApiAssociationsInputBuilder) -> impl Future<Output = Result<ListSourceApiAssociationsOutput, SdkError<ListSourceApiAssociationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>> {
        builder.send_with(&self.0)
    }
    fn list_types_by_association(&self, builder: ListTypesByAssociationInputBuilder) -> impl Future<Output = Result<ListTypesByAssociationOutput, SdkError<ListTypesByAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn put_graphql_api_environment_variables(&self, builder: PutGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<PutGraphqlApiEnvironmentVariablesOutput, SdkError<PutGraphqlApiEnvironmentVariablesError>>> {
        builder.send_with(&self.0)
    }
    fn start_data_source_introspection(&self, builder: StartDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<StartDataSourceIntrospectionOutput, SdkError<StartDataSourceIntrospectionError>>> {
        builder.send_with(&self.0)
    }
    fn start_schema_creation(&self, builder: StartSchemaCreationInputBuilder) -> impl Future<Output = Result<StartSchemaCreationOutput, SdkError<StartSchemaCreationError>>> {
        builder.send_with(&self.0)
    }
    fn start_schema_merge(&self, builder: StartSchemaMergeInputBuilder) -> impl Future<Output = Result<StartSchemaMergeOutput, SdkError<StartSchemaMergeError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_api_cache(&self, builder: UpdateApiCacheInputBuilder) -> impl Future<Output = Result<UpdateApiCacheOutput, SdkError<UpdateApiCacheError>>> {
        builder.send_with(&self.0)
    }
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        builder.send_with(&self.0)
    }
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn update_graphql_api(&self, builder: UpdateGraphqlApiInputBuilder) -> impl Future<Output = Result<UpdateGraphqlApiOutput, SdkError<UpdateGraphqlApiError>>> {
        builder.send_with(&self.0)
    }
    fn update_resolver(&self, builder: UpdateResolverInputBuilder) -> impl Future<Output = Result<UpdateResolverOutput, SdkError<UpdateResolverError>>> {
        builder.send_with(&self.0)
    }
    fn update_source_api_association(&self, builder: UpdateSourceApiAssociationInputBuilder) -> impl Future<Output = Result<UpdateSourceApiAssociationOutput, SdkError<UpdateSourceApiAssociationError>>> {
        builder.send_with(&self.0)
    }
    fn update_type(&self, builder: UpdateTypeInputBuilder) -> impl Future<Output = Result<UpdateTypeOutput, SdkError<UpdateTypeError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppSyncClient for T
where T: Deref,
      T::Target: AppSyncClient {
    fn associate_api(&self, builder: AssociateApiInputBuilder) -> impl Future<Output = Result<AssociateApiOutput, SdkError<AssociateApiError>>> {
        self.deref().associate_api(builder)
    }
    fn associate_merged_graphql_api(&self, builder: AssociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateMergedGraphqlApiOutput, SdkError<AssociateMergedGraphqlApiError>>> {
        self.deref().associate_merged_graphql_api(builder)
    }
    fn associate_source_graphql_api(&self, builder: AssociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<AssociateSourceGraphqlApiOutput, SdkError<AssociateSourceGraphqlApiError>>> {
        self.deref().associate_source_graphql_api(builder)
    }
    fn create_api_cache(&self, builder: CreateApiCacheInputBuilder) -> impl Future<Output = Result<CreateApiCacheOutput, SdkError<CreateApiCacheError>>> {
        self.deref().create_api_cache(builder)
    }
    fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> impl Future<Output = Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>> {
        self.deref().create_api_key(builder)
    }
    fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> impl Future<Output = Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>> {
        self.deref().create_data_source(builder)
    }
    fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> impl Future<Output = Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>> {
        self.deref().create_domain_name(builder)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        self.deref().create_function(builder)
    }
    fn create_graphql_api(&self, builder: CreateGraphqlApiInputBuilder) -> impl Future<Output = Result<CreateGraphqlApiOutput, SdkError<CreateGraphqlApiError>>> {
        self.deref().create_graphql_api(builder)
    }
    fn create_resolver(&self, builder: CreateResolverInputBuilder) -> impl Future<Output = Result<CreateResolverOutput, SdkError<CreateResolverError>>> {
        self.deref().create_resolver(builder)
    }
    fn create_type(&self, builder: CreateTypeInputBuilder) -> impl Future<Output = Result<CreateTypeOutput, SdkError<CreateTypeError>>> {
        self.deref().create_type(builder)
    }
    fn delete_api_cache(&self, builder: DeleteApiCacheInputBuilder) -> impl Future<Output = Result<DeleteApiCacheOutput, SdkError<DeleteApiCacheError>>> {
        self.deref().delete_api_cache(builder)
    }
    fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> impl Future<Output = Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>> {
        self.deref().delete_api_key(builder)
    }
    fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> impl Future<Output = Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>> {
        self.deref().delete_data_source(builder)
    }
    fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> impl Future<Output = Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>> {
        self.deref().delete_domain_name(builder)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        self.deref().delete_function(builder)
    }
    fn delete_graphql_api(&self, builder: DeleteGraphqlApiInputBuilder) -> impl Future<Output = Result<DeleteGraphqlApiOutput, SdkError<DeleteGraphqlApiError>>> {
        self.deref().delete_graphql_api(builder)
    }
    fn delete_resolver(&self, builder: DeleteResolverInputBuilder) -> impl Future<Output = Result<DeleteResolverOutput, SdkError<DeleteResolverError>>> {
        self.deref().delete_resolver(builder)
    }
    fn delete_type(&self, builder: DeleteTypeInputBuilder) -> impl Future<Output = Result<DeleteTypeOutput, SdkError<DeleteTypeError>>> {
        self.deref().delete_type(builder)
    }
    fn disassociate_api(&self, builder: DisassociateApiInputBuilder) -> impl Future<Output = Result<DisassociateApiOutput, SdkError<DisassociateApiError>>> {
        self.deref().disassociate_api(builder)
    }
    fn disassociate_merged_graphql_api(&self, builder: DisassociateMergedGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateMergedGraphqlApiOutput, SdkError<DisassociateMergedGraphqlApiError>>> {
        self.deref().disassociate_merged_graphql_api(builder)
    }
    fn disassociate_source_graphql_api(&self, builder: DisassociateSourceGraphqlApiInputBuilder) -> impl Future<Output = Result<DisassociateSourceGraphqlApiOutput, SdkError<DisassociateSourceGraphqlApiError>>> {
        self.deref().disassociate_source_graphql_api(builder)
    }
    fn evaluate_code(&self, builder: EvaluateCodeInputBuilder) -> impl Future<Output = Result<EvaluateCodeOutput, SdkError<EvaluateCodeError>>> {
        self.deref().evaluate_code(builder)
    }
    fn evaluate_mapping_template(&self, builder: EvaluateMappingTemplateInputBuilder) -> impl Future<Output = Result<EvaluateMappingTemplateOutput, SdkError<EvaluateMappingTemplateError>>> {
        self.deref().evaluate_mapping_template(builder)
    }
    fn flush_api_cache(&self, builder: FlushApiCacheInputBuilder) -> impl Future<Output = Result<FlushApiCacheOutput, SdkError<FlushApiCacheError>>> {
        self.deref().flush_api_cache(builder)
    }
    fn get_api_association(&self, builder: GetApiAssociationInputBuilder) -> impl Future<Output = Result<GetApiAssociationOutput, SdkError<GetApiAssociationError>>> {
        self.deref().get_api_association(builder)
    }
    fn get_api_cache(&self, builder: GetApiCacheInputBuilder) -> impl Future<Output = Result<GetApiCacheOutput, SdkError<GetApiCacheError>>> {
        self.deref().get_api_cache(builder)
    }
    fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> impl Future<Output = Result<GetDataSourceOutput, SdkError<GetDataSourceError>>> {
        self.deref().get_data_source(builder)
    }
    fn get_data_source_introspection(&self, builder: GetDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<GetDataSourceIntrospectionOutput, SdkError<GetDataSourceIntrospectionError>>> {
        self.deref().get_data_source_introspection(builder)
    }
    fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> impl Future<Output = Result<GetDomainNameOutput, SdkError<GetDomainNameError>>> {
        self.deref().get_domain_name(builder)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        self.deref().get_function(builder)
    }
    fn get_graphql_api(&self, builder: GetGraphqlApiInputBuilder) -> impl Future<Output = Result<GetGraphqlApiOutput, SdkError<GetGraphqlApiError>>> {
        self.deref().get_graphql_api(builder)
    }
    fn get_graphql_api_environment_variables(&self, builder: GetGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<GetGraphqlApiEnvironmentVariablesOutput, SdkError<GetGraphqlApiEnvironmentVariablesError>>> {
        self.deref().get_graphql_api_environment_variables(builder)
    }
    fn get_introspection_schema(&self, builder: GetIntrospectionSchemaInputBuilder) -> impl Future<Output = Result<GetIntrospectionSchemaOutput, SdkError<GetIntrospectionSchemaError>>> {
        self.deref().get_introspection_schema(builder)
    }
    fn get_resolver(&self, builder: GetResolverInputBuilder) -> impl Future<Output = Result<GetResolverOutput, SdkError<GetResolverError>>> {
        self.deref().get_resolver(builder)
    }
    fn get_schema_creation_status(&self, builder: GetSchemaCreationStatusInputBuilder) -> impl Future<Output = Result<GetSchemaCreationStatusOutput, SdkError<GetSchemaCreationStatusError>>> {
        self.deref().get_schema_creation_status(builder)
    }
    fn get_source_api_association(&self, builder: GetSourceApiAssociationInputBuilder) -> impl Future<Output = Result<GetSourceApiAssociationOutput, SdkError<GetSourceApiAssociationError>>> {
        self.deref().get_source_api_association(builder)
    }
    fn get_type(&self, builder: GetTypeInputBuilder) -> impl Future<Output = Result<GetTypeOutput, SdkError<GetTypeError>>> {
        self.deref().get_type(builder)
    }
    fn list_api_keys(&self, builder: ListApiKeysInputBuilder) -> impl Future<Output = Result<ListApiKeysOutput, SdkError<ListApiKeysError>>> {
        self.deref().list_api_keys(builder)
    }
    fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> impl Future<Output = Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>> {
        self.deref().list_data_sources(builder)
    }
    fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> impl Future<Output = Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>> {
        self.deref().list_domain_names(builder)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        self.deref().list_functions(builder)
    }
    fn list_graphql_apis(&self, builder: ListGraphqlApisInputBuilder) -> impl Future<Output = Result<ListGraphqlApisOutput, SdkError<ListGraphqlApisError>>> {
        self.deref().list_graphql_apis(builder)
    }
    fn list_resolvers(&self, builder: ListResolversInputBuilder) -> impl Future<Output = Result<ListResolversOutput, SdkError<ListResolversError>>> {
        self.deref().list_resolvers(builder)
    }
    fn list_resolvers_by_function(&self, builder: ListResolversByFunctionInputBuilder) -> impl Future<Output = Result<ListResolversByFunctionOutput, SdkError<ListResolversByFunctionError>>> {
        self.deref().list_resolvers_by_function(builder)
    }
    fn list_source_api_associations(&self, builder: ListSourceApiAssociationsInputBuilder) -> impl Future<Output = Result<ListSourceApiAssociationsOutput, SdkError<ListSourceApiAssociationsError>>> {
        self.deref().list_source_api_associations(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_types(&self, builder: ListTypesInputBuilder) -> impl Future<Output = Result<ListTypesOutput, SdkError<ListTypesError>>> {
        self.deref().list_types(builder)
    }
    fn list_types_by_association(&self, builder: ListTypesByAssociationInputBuilder) -> impl Future<Output = Result<ListTypesByAssociationOutput, SdkError<ListTypesByAssociationError>>> {
        self.deref().list_types_by_association(builder)
    }
    fn put_graphql_api_environment_variables(&self, builder: PutGraphqlApiEnvironmentVariablesInputBuilder) -> impl Future<Output = Result<PutGraphqlApiEnvironmentVariablesOutput, SdkError<PutGraphqlApiEnvironmentVariablesError>>> {
        self.deref().put_graphql_api_environment_variables(builder)
    }
    fn start_data_source_introspection(&self, builder: StartDataSourceIntrospectionInputBuilder) -> impl Future<Output = Result<StartDataSourceIntrospectionOutput, SdkError<StartDataSourceIntrospectionError>>> {
        self.deref().start_data_source_introspection(builder)
    }
    fn start_schema_creation(&self, builder: StartSchemaCreationInputBuilder) -> impl Future<Output = Result<StartSchemaCreationOutput, SdkError<StartSchemaCreationError>>> {
        self.deref().start_schema_creation(builder)
    }
    fn start_schema_merge(&self, builder: StartSchemaMergeInputBuilder) -> impl Future<Output = Result<StartSchemaMergeOutput, SdkError<StartSchemaMergeError>>> {
        self.deref().start_schema_merge(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_api_cache(&self, builder: UpdateApiCacheInputBuilder) -> impl Future<Output = Result<UpdateApiCacheOutput, SdkError<UpdateApiCacheError>>> {
        self.deref().update_api_cache(builder)
    }
    fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> impl Future<Output = Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>> {
        self.deref().update_api_key(builder)
    }
    fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> impl Future<Output = Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>> {
        self.deref().update_data_source(builder)
    }
    fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> impl Future<Output = Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>> {
        self.deref().update_domain_name(builder)
    }
    fn update_function(&self, builder: UpdateFunctionInputBuilder) -> impl Future<Output = Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>> {
        self.deref().update_function(builder)
    }
    fn update_graphql_api(&self, builder: UpdateGraphqlApiInputBuilder) -> impl Future<Output = Result<UpdateGraphqlApiOutput, SdkError<UpdateGraphqlApiError>>> {
        self.deref().update_graphql_api(builder)
    }
    fn update_resolver(&self, builder: UpdateResolverInputBuilder) -> impl Future<Output = Result<UpdateResolverOutput, SdkError<UpdateResolverError>>> {
        self.deref().update_resolver(builder)
    }
    fn update_source_api_association(&self, builder: UpdateSourceApiAssociationInputBuilder) -> impl Future<Output = Result<UpdateSourceApiAssociationOutput, SdkError<UpdateSourceApiAssociationError>>> {
        self.deref().update_source_api_association(builder)
    }
    fn update_type(&self, builder: UpdateTypeInputBuilder) -> impl Future<Output = Result<UpdateTypeOutput, SdkError<UpdateTypeError>>> {
        self.deref().update_type(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppSyncClient {}
    impl AppSyncClient for edAppSyncClient {
        async fn associate_api(&self, builder: AssociateApiInputBuilder) -> Result<AssociateApiOutput, SdkError<AssociateApiError>>;
        async fn associate_merged_graphql_api(&self, builder: AssociateMergedGraphqlApiInputBuilder) -> Result<AssociateMergedGraphqlApiOutput, SdkError<AssociateMergedGraphqlApiError>>;
        async fn associate_source_graphql_api(&self, builder: AssociateSourceGraphqlApiInputBuilder) -> Result<AssociateSourceGraphqlApiOutput, SdkError<AssociateSourceGraphqlApiError>>;
        async fn create_api_cache(&self, builder: CreateApiCacheInputBuilder) -> Result<CreateApiCacheOutput, SdkError<CreateApiCacheError>>;
        async fn create_api_key(&self, builder: CreateApiKeyInputBuilder) -> Result<CreateApiKeyOutput, SdkError<CreateApiKeyError>>;
        async fn create_data_source(&self, builder: CreateDataSourceInputBuilder) -> Result<CreateDataSourceOutput, SdkError<CreateDataSourceError>>;
        async fn create_domain_name(&self, builder: CreateDomainNameInputBuilder) -> Result<CreateDomainNameOutput, SdkError<CreateDomainNameError>>;
        async fn create_function(&self, builder: CreateFunctionInputBuilder) -> Result<CreateFunctionOutput, SdkError<CreateFunctionError>>;
        async fn create_graphql_api(&self, builder: CreateGraphqlApiInputBuilder) -> Result<CreateGraphqlApiOutput, SdkError<CreateGraphqlApiError>>;
        async fn create_resolver(&self, builder: CreateResolverInputBuilder) -> Result<CreateResolverOutput, SdkError<CreateResolverError>>;
        async fn create_type(&self, builder: CreateTypeInputBuilder) -> Result<CreateTypeOutput, SdkError<CreateTypeError>>;
        async fn delete_api_cache(&self, builder: DeleteApiCacheInputBuilder) -> Result<DeleteApiCacheOutput, SdkError<DeleteApiCacheError>>;
        async fn delete_api_key(&self, builder: DeleteApiKeyInputBuilder) -> Result<DeleteApiKeyOutput, SdkError<DeleteApiKeyError>>;
        async fn delete_data_source(&self, builder: DeleteDataSourceInputBuilder) -> Result<DeleteDataSourceOutput, SdkError<DeleteDataSourceError>>;
        async fn delete_domain_name(&self, builder: DeleteDomainNameInputBuilder) -> Result<DeleteDomainNameOutput, SdkError<DeleteDomainNameError>>;
        async fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>;
        async fn delete_graphql_api(&self, builder: DeleteGraphqlApiInputBuilder) -> Result<DeleteGraphqlApiOutput, SdkError<DeleteGraphqlApiError>>;
        async fn delete_resolver(&self, builder: DeleteResolverInputBuilder) -> Result<DeleteResolverOutput, SdkError<DeleteResolverError>>;
        async fn delete_type(&self, builder: DeleteTypeInputBuilder) -> Result<DeleteTypeOutput, SdkError<DeleteTypeError>>;
        async fn disassociate_api(&self, builder: DisassociateApiInputBuilder) -> Result<DisassociateApiOutput, SdkError<DisassociateApiError>>;
        async fn disassociate_merged_graphql_api(&self, builder: DisassociateMergedGraphqlApiInputBuilder) -> Result<DisassociateMergedGraphqlApiOutput, SdkError<DisassociateMergedGraphqlApiError>>;
        async fn disassociate_source_graphql_api(&self, builder: DisassociateSourceGraphqlApiInputBuilder) -> Result<DisassociateSourceGraphqlApiOutput, SdkError<DisassociateSourceGraphqlApiError>>;
        async fn evaluate_code(&self, builder: EvaluateCodeInputBuilder) -> Result<EvaluateCodeOutput, SdkError<EvaluateCodeError>>;
        async fn evaluate_mapping_template(&self, builder: EvaluateMappingTemplateInputBuilder) -> Result<EvaluateMappingTemplateOutput, SdkError<EvaluateMappingTemplateError>>;
        async fn flush_api_cache(&self, builder: FlushApiCacheInputBuilder) -> Result<FlushApiCacheOutput, SdkError<FlushApiCacheError>>;
        async fn get_api_association(&self, builder: GetApiAssociationInputBuilder) -> Result<GetApiAssociationOutput, SdkError<GetApiAssociationError>>;
        async fn get_api_cache(&self, builder: GetApiCacheInputBuilder) -> Result<GetApiCacheOutput, SdkError<GetApiCacheError>>;
        async fn get_data_source(&self, builder: GetDataSourceInputBuilder) -> Result<GetDataSourceOutput, SdkError<GetDataSourceError>>;
        async fn get_data_source_introspection(&self, builder: GetDataSourceIntrospectionInputBuilder) -> Result<GetDataSourceIntrospectionOutput, SdkError<GetDataSourceIntrospectionError>>;
        async fn get_domain_name(&self, builder: GetDomainNameInputBuilder) -> Result<GetDomainNameOutput, SdkError<GetDomainNameError>>;
        async fn get_function(&self, builder: GetFunctionInputBuilder) -> Result<GetFunctionOutput, SdkError<GetFunctionError>>;
        async fn get_graphql_api(&self, builder: GetGraphqlApiInputBuilder) -> Result<GetGraphqlApiOutput, SdkError<GetGraphqlApiError>>;
        async fn get_graphql_api_environment_variables(&self, builder: GetGraphqlApiEnvironmentVariablesInputBuilder) -> Result<GetGraphqlApiEnvironmentVariablesOutput, SdkError<GetGraphqlApiEnvironmentVariablesError>>;
        async fn get_introspection_schema(&self, builder: GetIntrospectionSchemaInputBuilder) -> Result<GetIntrospectionSchemaOutput, SdkError<GetIntrospectionSchemaError>>;
        async fn get_resolver(&self, builder: GetResolverInputBuilder) -> Result<GetResolverOutput, SdkError<GetResolverError>>;
        async fn get_schema_creation_status(&self, builder: GetSchemaCreationStatusInputBuilder) -> Result<GetSchemaCreationStatusOutput, SdkError<GetSchemaCreationStatusError>>;
        async fn get_source_api_association(&self, builder: GetSourceApiAssociationInputBuilder) -> Result<GetSourceApiAssociationOutput, SdkError<GetSourceApiAssociationError>>;
        async fn get_type(&self, builder: GetTypeInputBuilder) -> Result<GetTypeOutput, SdkError<GetTypeError>>;
        async fn list_api_keys(&self, builder: ListApiKeysInputBuilder) -> Result<ListApiKeysOutput, SdkError<ListApiKeysError>>;
        async fn list_data_sources(&self, builder: ListDataSourcesInputBuilder) -> Result<ListDataSourcesOutput, SdkError<ListDataSourcesError>>;
        async fn list_domain_names(&self, builder: ListDomainNamesInputBuilder) -> Result<ListDomainNamesOutput, SdkError<ListDomainNamesError>>;
        async fn list_functions(&self, builder: ListFunctionsInputBuilder) -> Result<ListFunctionsOutput, SdkError<ListFunctionsError>>;
        async fn list_graphql_apis(&self, builder: ListGraphqlApisInputBuilder) -> Result<ListGraphqlApisOutput, SdkError<ListGraphqlApisError>>;
        async fn list_resolvers(&self, builder: ListResolversInputBuilder) -> Result<ListResolversOutput, SdkError<ListResolversError>>;
        async fn list_resolvers_by_function(&self, builder: ListResolversByFunctionInputBuilder) -> Result<ListResolversByFunctionOutput, SdkError<ListResolversByFunctionError>>;
        async fn list_source_api_associations(&self, builder: ListSourceApiAssociationsInputBuilder) -> Result<ListSourceApiAssociationsOutput, SdkError<ListSourceApiAssociationsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_types(&self, builder: ListTypesInputBuilder) -> Result<ListTypesOutput, SdkError<ListTypesError>>;
        async fn list_types_by_association(&self, builder: ListTypesByAssociationInputBuilder) -> Result<ListTypesByAssociationOutput, SdkError<ListTypesByAssociationError>>;
        async fn put_graphql_api_environment_variables(&self, builder: PutGraphqlApiEnvironmentVariablesInputBuilder) -> Result<PutGraphqlApiEnvironmentVariablesOutput, SdkError<PutGraphqlApiEnvironmentVariablesError>>;
        async fn start_data_source_introspection(&self, builder: StartDataSourceIntrospectionInputBuilder) -> Result<StartDataSourceIntrospectionOutput, SdkError<StartDataSourceIntrospectionError>>;
        async fn start_schema_creation(&self, builder: StartSchemaCreationInputBuilder) -> Result<StartSchemaCreationOutput, SdkError<StartSchemaCreationError>>;
        async fn start_schema_merge(&self, builder: StartSchemaMergeInputBuilder) -> Result<StartSchemaMergeOutput, SdkError<StartSchemaMergeError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_api_cache(&self, builder: UpdateApiCacheInputBuilder) -> Result<UpdateApiCacheOutput, SdkError<UpdateApiCacheError>>;
        async fn update_api_key(&self, builder: UpdateApiKeyInputBuilder) -> Result<UpdateApiKeyOutput, SdkError<UpdateApiKeyError>>;
        async fn update_data_source(&self, builder: UpdateDataSourceInputBuilder) -> Result<UpdateDataSourceOutput, SdkError<UpdateDataSourceError>>;
        async fn update_domain_name(&self, builder: UpdateDomainNameInputBuilder) -> Result<UpdateDomainNameOutput, SdkError<UpdateDomainNameError>>;
        async fn update_function(&self, builder: UpdateFunctionInputBuilder) -> Result<UpdateFunctionOutput, SdkError<UpdateFunctionError>>;
        async fn update_graphql_api(&self, builder: UpdateGraphqlApiInputBuilder) -> Result<UpdateGraphqlApiOutput, SdkError<UpdateGraphqlApiError>>;
        async fn update_resolver(&self, builder: UpdateResolverInputBuilder) -> Result<UpdateResolverOutput, SdkError<UpdateResolverError>>;
        async fn update_source_api_association(&self, builder: UpdateSourceApiAssociationInputBuilder) -> Result<UpdateSourceApiAssociationOutput, SdkError<UpdateSourceApiAssociationError>>;
        async fn update_type(&self, builder: UpdateTypeInputBuilder) -> Result<UpdateTypeOutput, SdkError<UpdateTypeError>>;
    }
}
