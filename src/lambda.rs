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
use aws_sdk_lambda::operation::add_layer_version_permission::{builders::*, *};
use aws_sdk_lambda::operation::add_permission::{builders::*, *};
use aws_sdk_lambda::operation::create_alias::{builders::*, *};
use aws_sdk_lambda::operation::create_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::create_event_source_mapping::{builders::*, *};
use aws_sdk_lambda::operation::create_function::{builders::*, *};
use aws_sdk_lambda::operation::create_function_url_config::{builders::*, *};
use aws_sdk_lambda::operation::delete_alias::{builders::*, *};
use aws_sdk_lambda::operation::delete_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::delete_event_source_mapping::{builders::*, *};
use aws_sdk_lambda::operation::delete_function::{builders::*, *};
use aws_sdk_lambda::operation::delete_function_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::delete_function_concurrency::{builders::*, *};
use aws_sdk_lambda::operation::delete_function_event_invoke_config::{builders::*, *};
use aws_sdk_lambda::operation::delete_function_url_config::{builders::*, *};
use aws_sdk_lambda::operation::delete_layer_version::{builders::*, *};
use aws_sdk_lambda::operation::delete_provisioned_concurrency_config::{builders::*, *};
use aws_sdk_lambda::operation::get_account_settings::{builders::*, *};
use aws_sdk_lambda::operation::get_alias::{builders::*, *};
use aws_sdk_lambda::operation::get_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::get_event_source_mapping::{builders::*, *};
use aws_sdk_lambda::operation::get_function::{builders::*, *};
use aws_sdk_lambda::operation::get_function_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::get_function_concurrency::{builders::*, *};
use aws_sdk_lambda::operation::get_function_configuration::{builders::*, *};
use aws_sdk_lambda::operation::get_function_event_invoke_config::{builders::*, *};
use aws_sdk_lambda::operation::get_function_url_config::{builders::*, *};
use aws_sdk_lambda::operation::get_layer_version::{builders::*, *};
use aws_sdk_lambda::operation::get_layer_version_by_arn::{builders::*, *};
use aws_sdk_lambda::operation::get_layer_version_policy::{builders::*, *};
use aws_sdk_lambda::operation::get_policy::{builders::*, *};
use aws_sdk_lambda::operation::get_provisioned_concurrency_config::{builders::*, *};
use aws_sdk_lambda::operation::get_runtime_management_config::{builders::*, *};
use aws_sdk_lambda::operation::invoke::{builders::*, *};
use aws_sdk_lambda::operation::list_aliases::{builders::*, *};
use aws_sdk_lambda::operation::list_code_signing_configs::{builders::*, *};
use aws_sdk_lambda::operation::list_event_source_mappings::{builders::*, *};
use aws_sdk_lambda::operation::list_function_event_invoke_configs::{builders::*, *};
use aws_sdk_lambda::operation::list_function_url_configs::{builders::*, *};
use aws_sdk_lambda::operation::list_functions::{builders::*, *};
use aws_sdk_lambda::operation::list_functions_by_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::list_layer_versions::{builders::*, *};
use aws_sdk_lambda::operation::list_layers::{builders::*, *};
use aws_sdk_lambda::operation::list_provisioned_concurrency_configs::{builders::*, *};
use aws_sdk_lambda::operation::list_tags::{builders::*, *};
use aws_sdk_lambda::operation::list_versions_by_function::{builders::*, *};
use aws_sdk_lambda::operation::publish_layer_version::{builders::*, *};
use aws_sdk_lambda::operation::publish_version::{builders::*, *};
use aws_sdk_lambda::operation::put_function_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::put_function_concurrency::{builders::*, *};
use aws_sdk_lambda::operation::put_function_event_invoke_config::{builders::*, *};
use aws_sdk_lambda::operation::put_provisioned_concurrency_config::{builders::*, *};
use aws_sdk_lambda::operation::put_runtime_management_config::{builders::*, *};
use aws_sdk_lambda::operation::remove_layer_version_permission::{builders::*, *};
use aws_sdk_lambda::operation::remove_permission::{builders::*, *};
use aws_sdk_lambda::operation::tag_resource::{builders::*, *};
use aws_sdk_lambda::operation::untag_resource::{builders::*, *};
use aws_sdk_lambda::operation::update_alias::{builders::*, *};
use aws_sdk_lambda::operation::update_code_signing_config::{builders::*, *};
use aws_sdk_lambda::operation::update_event_source_mapping::{builders::*, *};
use aws_sdk_lambda::operation::update_function_code::{builders::*, *};
use aws_sdk_lambda::operation::update_function_configuration::{builders::*, *};
use aws_sdk_lambda::operation::update_function_event_invoke_config::{builders::*, *};
use aws_sdk_lambda::operation::update_function_url_config::{builders::*, *};
use aws_sdk_lambda::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_lambda::Client;

pub use aws_sdk_lambda::*;

pub struct LambdaClientImpl(Client);
impl LambdaClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait LambdaClient {
    fn add_layer_version_permission(&self, builder: AddLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<AddLayerVersionPermissionOutput, SdkError<AddLayerVersionPermissionError>>>;
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>>;
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>>;
    fn create_code_signing_config(&self, builder: CreateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<CreateCodeSigningConfigOutput, SdkError<CreateCodeSigningConfigError>>>;
    fn create_event_source_mapping(&self, builder: CreateEventSourceMappingInputBuilder) -> impl Future<Output = Result<CreateEventSourceMappingOutput, SdkError<CreateEventSourceMappingError>>>;
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>>;
    fn create_function_url_config(&self, builder: CreateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<CreateFunctionUrlConfigOutput, SdkError<CreateFunctionUrlConfigError>>>;
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>>;
    fn delete_code_signing_config(&self, builder: DeleteCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteCodeSigningConfigOutput, SdkError<DeleteCodeSigningConfigError>>>;
    fn delete_event_source_mapping(&self, builder: DeleteEventSourceMappingInputBuilder) -> impl Future<Output = Result<DeleteEventSourceMappingOutput, SdkError<DeleteEventSourceMappingError>>>;
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>>;
    fn delete_function_code_signing_config(&self, builder: DeleteFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionCodeSigningConfigOutput, SdkError<DeleteFunctionCodeSigningConfigError>>>;
    fn delete_function_concurrency(&self, builder: DeleteFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<DeleteFunctionConcurrencyOutput, SdkError<DeleteFunctionConcurrencyError>>>;
    fn delete_function_event_invoke_config(&self, builder: DeleteFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionEventInvokeConfigOutput, SdkError<DeleteFunctionEventInvokeConfigError>>>;
    fn delete_function_url_config(&self, builder: DeleteFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionUrlConfigOutput, SdkError<DeleteFunctionUrlConfigError>>>;
    fn delete_layer_version(&self, builder: DeleteLayerVersionInputBuilder) -> impl Future<Output = Result<DeleteLayerVersionOutput, SdkError<DeleteLayerVersionError>>>;
    fn delete_provisioned_concurrency_config(&self, builder: DeleteProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<DeleteProvisionedConcurrencyConfigOutput, SdkError<DeleteProvisionedConcurrencyConfigError>>>;
    fn get_account_settings(&self, builder: GetAccountSettingsInputBuilder) -> impl Future<Output = Result<GetAccountSettingsOutput, SdkError<GetAccountSettingsError>>>;
    fn get_alias(&self, builder: GetAliasInputBuilder) -> impl Future<Output = Result<GetAliasOutput, SdkError<GetAliasError>>>;
    fn get_code_signing_config(&self, builder: GetCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetCodeSigningConfigOutput, SdkError<GetCodeSigningConfigError>>>;
    fn get_event_source_mapping(&self, builder: GetEventSourceMappingInputBuilder) -> impl Future<Output = Result<GetEventSourceMappingOutput, SdkError<GetEventSourceMappingError>>>;
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>>;
    fn get_function_code_signing_config(&self, builder: GetFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetFunctionCodeSigningConfigOutput, SdkError<GetFunctionCodeSigningConfigError>>>;
    fn get_function_concurrency(&self, builder: GetFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<GetFunctionConcurrencyOutput, SdkError<GetFunctionConcurrencyError>>>;
    fn get_function_configuration(&self, builder: GetFunctionConfigurationInputBuilder) -> impl Future<Output = Result<GetFunctionConfigurationOutput, SdkError<GetFunctionConfigurationError>>>;
    fn get_function_event_invoke_config(&self, builder: GetFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<GetFunctionEventInvokeConfigOutput, SdkError<GetFunctionEventInvokeConfigError>>>;
    fn get_function_url_config(&self, builder: GetFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<GetFunctionUrlConfigOutput, SdkError<GetFunctionUrlConfigError>>>;
    fn get_layer_version(&self, builder: GetLayerVersionInputBuilder) -> impl Future<Output = Result<GetLayerVersionOutput, SdkError<GetLayerVersionError>>>;
    fn get_layer_version_by_arn(&self, builder: GetLayerVersionByArnInputBuilder) -> impl Future<Output = Result<GetLayerVersionByArnOutput, SdkError<GetLayerVersionByArnError>>>;
    fn get_layer_version_policy(&self, builder: GetLayerVersionPolicyInputBuilder) -> impl Future<Output = Result<GetLayerVersionPolicyOutput, SdkError<GetLayerVersionPolicyError>>>;
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>>;
    fn get_provisioned_concurrency_config(&self, builder: GetProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<GetProvisionedConcurrencyConfigOutput, SdkError<GetProvisionedConcurrencyConfigError>>>;
    fn get_runtime_management_config(&self, builder: GetRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<GetRuntimeManagementConfigOutput, SdkError<GetRuntimeManagementConfigError>>>;
    fn invoke(&self, builder: InvokeInputBuilder) -> impl Future<Output = Result<InvokeOutput, SdkError<InvokeError>>>;
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>>;
    fn list_code_signing_configs(&self, builder: ListCodeSigningConfigsInputBuilder) -> impl Future<Output = Result<ListCodeSigningConfigsOutput, SdkError<ListCodeSigningConfigsError>>>;
    fn list_event_source_mappings(&self, builder: ListEventSourceMappingsInputBuilder) -> impl Future<Output = Result<ListEventSourceMappingsOutput, SdkError<ListEventSourceMappingsError>>>;
    fn list_function_event_invoke_configs(&self, builder: ListFunctionEventInvokeConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionEventInvokeConfigsOutput, SdkError<ListFunctionEventInvokeConfigsError>>>;
    fn list_function_url_configs(&self, builder: ListFunctionUrlConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionUrlConfigsOutput, SdkError<ListFunctionUrlConfigsError>>>;
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>>;
    fn list_functions_by_code_signing_config(&self, builder: ListFunctionsByCodeSigningConfigInputBuilder) -> impl Future<Output = Result<ListFunctionsByCodeSigningConfigOutput, SdkError<ListFunctionsByCodeSigningConfigError>>>;
    fn list_layer_versions(&self, builder: ListLayerVersionsInputBuilder) -> impl Future<Output = Result<ListLayerVersionsOutput, SdkError<ListLayerVersionsError>>>;
    fn list_layers(&self, builder: ListLayersInputBuilder) -> impl Future<Output = Result<ListLayersOutput, SdkError<ListLayersError>>>;
    fn list_provisioned_concurrency_configs(&self, builder: ListProvisionedConcurrencyConfigsInputBuilder) -> impl Future<Output = Result<ListProvisionedConcurrencyConfigsOutput, SdkError<ListProvisionedConcurrencyConfigsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn list_versions_by_function(&self, builder: ListVersionsByFunctionInputBuilder) -> impl Future<Output = Result<ListVersionsByFunctionOutput, SdkError<ListVersionsByFunctionError>>>;
    fn publish_layer_version(&self, builder: PublishLayerVersionInputBuilder) -> impl Future<Output = Result<PublishLayerVersionOutput, SdkError<PublishLayerVersionError>>>;
    fn publish_version(&self, builder: PublishVersionInputBuilder) -> impl Future<Output = Result<PublishVersionOutput, SdkError<PublishVersionError>>>;
    fn put_function_code_signing_config(&self, builder: PutFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<PutFunctionCodeSigningConfigOutput, SdkError<PutFunctionCodeSigningConfigError>>>;
    fn put_function_concurrency(&self, builder: PutFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<PutFunctionConcurrencyOutput, SdkError<PutFunctionConcurrencyError>>>;
    fn put_function_event_invoke_config(&self, builder: PutFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<PutFunctionEventInvokeConfigOutput, SdkError<PutFunctionEventInvokeConfigError>>>;
    fn put_provisioned_concurrency_config(&self, builder: PutProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<PutProvisionedConcurrencyConfigOutput, SdkError<PutProvisionedConcurrencyConfigError>>>;
    fn put_runtime_management_config(&self, builder: PutRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<PutRuntimeManagementConfigOutput, SdkError<PutRuntimeManagementConfigError>>>;
    fn remove_layer_version_permission(&self, builder: RemoveLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<RemoveLayerVersionPermissionOutput, SdkError<RemoveLayerVersionPermissionError>>>;
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>>;
    fn update_code_signing_config(&self, builder: UpdateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<UpdateCodeSigningConfigOutput, SdkError<UpdateCodeSigningConfigError>>>;
    fn update_event_source_mapping(&self, builder: UpdateEventSourceMappingInputBuilder) -> impl Future<Output = Result<UpdateEventSourceMappingOutput, SdkError<UpdateEventSourceMappingError>>>;
    fn update_function_code(&self, builder: UpdateFunctionCodeInputBuilder) -> impl Future<Output = Result<UpdateFunctionCodeOutput, SdkError<UpdateFunctionCodeError>>>;
    fn update_function_configuration(&self, builder: UpdateFunctionConfigurationInputBuilder) -> impl Future<Output = Result<UpdateFunctionConfigurationOutput, SdkError<UpdateFunctionConfigurationError>>>;
    fn update_function_event_invoke_config(&self, builder: UpdateFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionEventInvokeConfigOutput, SdkError<UpdateFunctionEventInvokeConfigError>>>;
    fn update_function_url_config(&self, builder: UpdateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionUrlConfigOutput, SdkError<UpdateFunctionUrlConfigError>>>;
}
impl LambdaClient for LambdaClientImpl {
    fn add_layer_version_permission(&self, builder: AddLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<AddLayerVersionPermissionOutput, SdkError<AddLayerVersionPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn create_code_signing_config(&self, builder: CreateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<CreateCodeSigningConfigOutput, SdkError<CreateCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn create_event_source_mapping(&self, builder: CreateEventSourceMappingInputBuilder) -> impl Future<Output = Result<CreateEventSourceMappingOutput, SdkError<CreateEventSourceMappingError>>> {
        builder.send_with(&self.0)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn create_function_url_config(&self, builder: CreateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<CreateFunctionUrlConfigOutput, SdkError<CreateFunctionUrlConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>> {
        builder.send_with(&self.0)
    }
    fn delete_code_signing_config(&self, builder: DeleteCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteCodeSigningConfigOutput, SdkError<DeleteCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_event_source_mapping(&self, builder: DeleteEventSourceMappingInputBuilder) -> impl Future<Output = Result<DeleteEventSourceMappingOutput, SdkError<DeleteEventSourceMappingError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function_code_signing_config(&self, builder: DeleteFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionCodeSigningConfigOutput, SdkError<DeleteFunctionCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function_concurrency(&self, builder: DeleteFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<DeleteFunctionConcurrencyOutput, SdkError<DeleteFunctionConcurrencyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function_event_invoke_config(&self, builder: DeleteFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionEventInvokeConfigOutput, SdkError<DeleteFunctionEventInvokeConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_function_url_config(&self, builder: DeleteFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionUrlConfigOutput, SdkError<DeleteFunctionUrlConfigError>>> {
        builder.send_with(&self.0)
    }
    fn delete_layer_version(&self, builder: DeleteLayerVersionInputBuilder) -> impl Future<Output = Result<DeleteLayerVersionOutput, SdkError<DeleteLayerVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_provisioned_concurrency_config(&self, builder: DeleteProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<DeleteProvisionedConcurrencyConfigOutput, SdkError<DeleteProvisionedConcurrencyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_account_settings(&self, builder: GetAccountSettingsInputBuilder) -> impl Future<Output = Result<GetAccountSettingsOutput, SdkError<GetAccountSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_alias(&self, builder: GetAliasInputBuilder) -> impl Future<Output = Result<GetAliasOutput, SdkError<GetAliasError>>> {
        builder.send_with(&self.0)
    }
    fn get_code_signing_config(&self, builder: GetCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetCodeSigningConfigOutput, SdkError<GetCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_event_source_mapping(&self, builder: GetEventSourceMappingInputBuilder) -> impl Future<Output = Result<GetEventSourceMappingOutput, SdkError<GetEventSourceMappingError>>> {
        builder.send_with(&self.0)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn get_function_code_signing_config(&self, builder: GetFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetFunctionCodeSigningConfigOutput, SdkError<GetFunctionCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_function_concurrency(&self, builder: GetFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<GetFunctionConcurrencyOutput, SdkError<GetFunctionConcurrencyError>>> {
        builder.send_with(&self.0)
    }
    fn get_function_configuration(&self, builder: GetFunctionConfigurationInputBuilder) -> impl Future<Output = Result<GetFunctionConfigurationOutput, SdkError<GetFunctionConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_function_event_invoke_config(&self, builder: GetFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<GetFunctionEventInvokeConfigOutput, SdkError<GetFunctionEventInvokeConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_function_url_config(&self, builder: GetFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<GetFunctionUrlConfigOutput, SdkError<GetFunctionUrlConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_layer_version(&self, builder: GetLayerVersionInputBuilder) -> impl Future<Output = Result<GetLayerVersionOutput, SdkError<GetLayerVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_layer_version_by_arn(&self, builder: GetLayerVersionByArnInputBuilder) -> impl Future<Output = Result<GetLayerVersionByArnOutput, SdkError<GetLayerVersionByArnError>>> {
        builder.send_with(&self.0)
    }
    fn get_layer_version_policy(&self, builder: GetLayerVersionPolicyInputBuilder) -> impl Future<Output = Result<GetLayerVersionPolicyOutput, SdkError<GetLayerVersionPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_provisioned_concurrency_config(&self, builder: GetProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<GetProvisionedConcurrencyConfigOutput, SdkError<GetProvisionedConcurrencyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn get_runtime_management_config(&self, builder: GetRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<GetRuntimeManagementConfigOutput, SdkError<GetRuntimeManagementConfigError>>> {
        builder.send_with(&self.0)
    }
    fn invoke(&self, builder: InvokeInputBuilder) -> impl Future<Output = Result<InvokeOutput, SdkError<InvokeError>>> {
        builder.send_with(&self.0)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_code_signing_configs(&self, builder: ListCodeSigningConfigsInputBuilder) -> impl Future<Output = Result<ListCodeSigningConfigsOutput, SdkError<ListCodeSigningConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_event_source_mappings(&self, builder: ListEventSourceMappingsInputBuilder) -> impl Future<Output = Result<ListEventSourceMappingsOutput, SdkError<ListEventSourceMappingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_function_event_invoke_configs(&self, builder: ListFunctionEventInvokeConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionEventInvokeConfigsOutput, SdkError<ListFunctionEventInvokeConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_function_url_configs(&self, builder: ListFunctionUrlConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionUrlConfigsOutput, SdkError<ListFunctionUrlConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_functions_by_code_signing_config(&self, builder: ListFunctionsByCodeSigningConfigInputBuilder) -> impl Future<Output = Result<ListFunctionsByCodeSigningConfigOutput, SdkError<ListFunctionsByCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn list_layer_versions(&self, builder: ListLayerVersionsInputBuilder) -> impl Future<Output = Result<ListLayerVersionsOutput, SdkError<ListLayerVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_layers(&self, builder: ListLayersInputBuilder) -> impl Future<Output = Result<ListLayersOutput, SdkError<ListLayersError>>> {
        builder.send_with(&self.0)
    }
    fn list_provisioned_concurrency_configs(&self, builder: ListProvisionedConcurrencyConfigsInputBuilder) -> impl Future<Output = Result<ListProvisionedConcurrencyConfigsOutput, SdkError<ListProvisionedConcurrencyConfigsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_versions_by_function(&self, builder: ListVersionsByFunctionInputBuilder) -> impl Future<Output = Result<ListVersionsByFunctionOutput, SdkError<ListVersionsByFunctionError>>> {
        builder.send_with(&self.0)
    }
    fn publish_layer_version(&self, builder: PublishLayerVersionInputBuilder) -> impl Future<Output = Result<PublishLayerVersionOutput, SdkError<PublishLayerVersionError>>> {
        builder.send_with(&self.0)
    }
    fn publish_version(&self, builder: PublishVersionInputBuilder) -> impl Future<Output = Result<PublishVersionOutput, SdkError<PublishVersionError>>> {
        builder.send_with(&self.0)
    }
    fn put_function_code_signing_config(&self, builder: PutFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<PutFunctionCodeSigningConfigOutput, SdkError<PutFunctionCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn put_function_concurrency(&self, builder: PutFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<PutFunctionConcurrencyOutput, SdkError<PutFunctionConcurrencyError>>> {
        builder.send_with(&self.0)
    }
    fn put_function_event_invoke_config(&self, builder: PutFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<PutFunctionEventInvokeConfigOutput, SdkError<PutFunctionEventInvokeConfigError>>> {
        builder.send_with(&self.0)
    }
    fn put_provisioned_concurrency_config(&self, builder: PutProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<PutProvisionedConcurrencyConfigOutput, SdkError<PutProvisionedConcurrencyConfigError>>> {
        builder.send_with(&self.0)
    }
    fn put_runtime_management_config(&self, builder: PutRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<PutRuntimeManagementConfigOutput, SdkError<PutRuntimeManagementConfigError>>> {
        builder.send_with(&self.0)
    }
    fn remove_layer_version_permission(&self, builder: RemoveLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<RemoveLayerVersionPermissionOutput, SdkError<RemoveLayerVersionPermissionError>>> {
        builder.send_with(&self.0)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>> {
        builder.send_with(&self.0)
    }
    fn update_code_signing_config(&self, builder: UpdateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<UpdateCodeSigningConfigOutput, SdkError<UpdateCodeSigningConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_event_source_mapping(&self, builder: UpdateEventSourceMappingInputBuilder) -> impl Future<Output = Result<UpdateEventSourceMappingOutput, SdkError<UpdateEventSourceMappingError>>> {
        builder.send_with(&self.0)
    }
    fn update_function_code(&self, builder: UpdateFunctionCodeInputBuilder) -> impl Future<Output = Result<UpdateFunctionCodeOutput, SdkError<UpdateFunctionCodeError>>> {
        builder.send_with(&self.0)
    }
    fn update_function_configuration(&self, builder: UpdateFunctionConfigurationInputBuilder) -> impl Future<Output = Result<UpdateFunctionConfigurationOutput, SdkError<UpdateFunctionConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_function_event_invoke_config(&self, builder: UpdateFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionEventInvokeConfigOutput, SdkError<UpdateFunctionEventInvokeConfigError>>> {
        builder.send_with(&self.0)
    }
    fn update_function_url_config(&self, builder: UpdateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionUrlConfigOutput, SdkError<UpdateFunctionUrlConfigError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: LambdaClient> LambdaClient for &T {
    fn add_layer_version_permission(&self, builder: AddLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<AddLayerVersionPermissionOutput, SdkError<AddLayerVersionPermissionError>>> {
        (*self).add_layer_version_permission(builder)
    }
    fn add_permission(&self, builder: AddPermissionInputBuilder) -> impl Future<Output = Result<AddPermissionOutput, SdkError<AddPermissionError>>> {
        (*self).add_permission(builder)
    }
    fn create_alias(&self, builder: CreateAliasInputBuilder) -> impl Future<Output = Result<CreateAliasOutput, SdkError<CreateAliasError>>> {
        (*self).create_alias(builder)
    }
    fn create_code_signing_config(&self, builder: CreateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<CreateCodeSigningConfigOutput, SdkError<CreateCodeSigningConfigError>>> {
        (*self).create_code_signing_config(builder)
    }
    fn create_event_source_mapping(&self, builder: CreateEventSourceMappingInputBuilder) -> impl Future<Output = Result<CreateEventSourceMappingOutput, SdkError<CreateEventSourceMappingError>>> {
        (*self).create_event_source_mapping(builder)
    }
    fn create_function(&self, builder: CreateFunctionInputBuilder) -> impl Future<Output = Result<CreateFunctionOutput, SdkError<CreateFunctionError>>> {
        (*self).create_function(builder)
    }
    fn create_function_url_config(&self, builder: CreateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<CreateFunctionUrlConfigOutput, SdkError<CreateFunctionUrlConfigError>>> {
        (*self).create_function_url_config(builder)
    }
    fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> impl Future<Output = Result<DeleteAliasOutput, SdkError<DeleteAliasError>>> {
        (*self).delete_alias(builder)
    }
    fn delete_code_signing_config(&self, builder: DeleteCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteCodeSigningConfigOutput, SdkError<DeleteCodeSigningConfigError>>> {
        (*self).delete_code_signing_config(builder)
    }
    fn delete_event_source_mapping(&self, builder: DeleteEventSourceMappingInputBuilder) -> impl Future<Output = Result<DeleteEventSourceMappingOutput, SdkError<DeleteEventSourceMappingError>>> {
        (*self).delete_event_source_mapping(builder)
    }
    fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> impl Future<Output = Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>> {
        (*self).delete_function(builder)
    }
    fn delete_function_code_signing_config(&self, builder: DeleteFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionCodeSigningConfigOutput, SdkError<DeleteFunctionCodeSigningConfigError>>> {
        (*self).delete_function_code_signing_config(builder)
    }
    fn delete_function_concurrency(&self, builder: DeleteFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<DeleteFunctionConcurrencyOutput, SdkError<DeleteFunctionConcurrencyError>>> {
        (*self).delete_function_concurrency(builder)
    }
    fn delete_function_event_invoke_config(&self, builder: DeleteFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionEventInvokeConfigOutput, SdkError<DeleteFunctionEventInvokeConfigError>>> {
        (*self).delete_function_event_invoke_config(builder)
    }
    fn delete_function_url_config(&self, builder: DeleteFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<DeleteFunctionUrlConfigOutput, SdkError<DeleteFunctionUrlConfigError>>> {
        (*self).delete_function_url_config(builder)
    }
    fn delete_layer_version(&self, builder: DeleteLayerVersionInputBuilder) -> impl Future<Output = Result<DeleteLayerVersionOutput, SdkError<DeleteLayerVersionError>>> {
        (*self).delete_layer_version(builder)
    }
    fn delete_provisioned_concurrency_config(&self, builder: DeleteProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<DeleteProvisionedConcurrencyConfigOutput, SdkError<DeleteProvisionedConcurrencyConfigError>>> {
        (*self).delete_provisioned_concurrency_config(builder)
    }
    fn get_account_settings(&self, builder: GetAccountSettingsInputBuilder) -> impl Future<Output = Result<GetAccountSettingsOutput, SdkError<GetAccountSettingsError>>> {
        (*self).get_account_settings(builder)
    }
    fn get_alias(&self, builder: GetAliasInputBuilder) -> impl Future<Output = Result<GetAliasOutput, SdkError<GetAliasError>>> {
        (*self).get_alias(builder)
    }
    fn get_code_signing_config(&self, builder: GetCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetCodeSigningConfigOutput, SdkError<GetCodeSigningConfigError>>> {
        (*self).get_code_signing_config(builder)
    }
    fn get_event_source_mapping(&self, builder: GetEventSourceMappingInputBuilder) -> impl Future<Output = Result<GetEventSourceMappingOutput, SdkError<GetEventSourceMappingError>>> {
        (*self).get_event_source_mapping(builder)
    }
    fn get_function(&self, builder: GetFunctionInputBuilder) -> impl Future<Output = Result<GetFunctionOutput, SdkError<GetFunctionError>>> {
        (*self).get_function(builder)
    }
    fn get_function_code_signing_config(&self, builder: GetFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<GetFunctionCodeSigningConfigOutput, SdkError<GetFunctionCodeSigningConfigError>>> {
        (*self).get_function_code_signing_config(builder)
    }
    fn get_function_concurrency(&self, builder: GetFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<GetFunctionConcurrencyOutput, SdkError<GetFunctionConcurrencyError>>> {
        (*self).get_function_concurrency(builder)
    }
    fn get_function_configuration(&self, builder: GetFunctionConfigurationInputBuilder) -> impl Future<Output = Result<GetFunctionConfigurationOutput, SdkError<GetFunctionConfigurationError>>> {
        (*self).get_function_configuration(builder)
    }
    fn get_function_event_invoke_config(&self, builder: GetFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<GetFunctionEventInvokeConfigOutput, SdkError<GetFunctionEventInvokeConfigError>>> {
        (*self).get_function_event_invoke_config(builder)
    }
    fn get_function_url_config(&self, builder: GetFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<GetFunctionUrlConfigOutput, SdkError<GetFunctionUrlConfigError>>> {
        (*self).get_function_url_config(builder)
    }
    fn get_layer_version(&self, builder: GetLayerVersionInputBuilder) -> impl Future<Output = Result<GetLayerVersionOutput, SdkError<GetLayerVersionError>>> {
        (*self).get_layer_version(builder)
    }
    fn get_layer_version_by_arn(&self, builder: GetLayerVersionByArnInputBuilder) -> impl Future<Output = Result<GetLayerVersionByArnOutput, SdkError<GetLayerVersionByArnError>>> {
        (*self).get_layer_version_by_arn(builder)
    }
    fn get_layer_version_policy(&self, builder: GetLayerVersionPolicyInputBuilder) -> impl Future<Output = Result<GetLayerVersionPolicyOutput, SdkError<GetLayerVersionPolicyError>>> {
        (*self).get_layer_version_policy(builder)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        (*self).get_policy(builder)
    }
    fn get_provisioned_concurrency_config(&self, builder: GetProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<GetProvisionedConcurrencyConfigOutput, SdkError<GetProvisionedConcurrencyConfigError>>> {
        (*self).get_provisioned_concurrency_config(builder)
    }
    fn get_runtime_management_config(&self, builder: GetRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<GetRuntimeManagementConfigOutput, SdkError<GetRuntimeManagementConfigError>>> {
        (*self).get_runtime_management_config(builder)
    }
    fn invoke(&self, builder: InvokeInputBuilder) -> impl Future<Output = Result<InvokeOutput, SdkError<InvokeError>>> {
        (*self).invoke(builder)
    }
    fn list_aliases(&self, builder: ListAliasesInputBuilder) -> impl Future<Output = Result<ListAliasesOutput, SdkError<ListAliasesError>>> {
        (*self).list_aliases(builder)
    }
    fn list_code_signing_configs(&self, builder: ListCodeSigningConfigsInputBuilder) -> impl Future<Output = Result<ListCodeSigningConfigsOutput, SdkError<ListCodeSigningConfigsError>>> {
        (*self).list_code_signing_configs(builder)
    }
    fn list_event_source_mappings(&self, builder: ListEventSourceMappingsInputBuilder) -> impl Future<Output = Result<ListEventSourceMappingsOutput, SdkError<ListEventSourceMappingsError>>> {
        (*self).list_event_source_mappings(builder)
    }
    fn list_function_event_invoke_configs(&self, builder: ListFunctionEventInvokeConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionEventInvokeConfigsOutput, SdkError<ListFunctionEventInvokeConfigsError>>> {
        (*self).list_function_event_invoke_configs(builder)
    }
    fn list_function_url_configs(&self, builder: ListFunctionUrlConfigsInputBuilder) -> impl Future<Output = Result<ListFunctionUrlConfigsOutput, SdkError<ListFunctionUrlConfigsError>>> {
        (*self).list_function_url_configs(builder)
    }
    fn list_functions(&self, builder: ListFunctionsInputBuilder) -> impl Future<Output = Result<ListFunctionsOutput, SdkError<ListFunctionsError>>> {
        (*self).list_functions(builder)
    }
    fn list_functions_by_code_signing_config(&self, builder: ListFunctionsByCodeSigningConfigInputBuilder) -> impl Future<Output = Result<ListFunctionsByCodeSigningConfigOutput, SdkError<ListFunctionsByCodeSigningConfigError>>> {
        (*self).list_functions_by_code_signing_config(builder)
    }
    fn list_layer_versions(&self, builder: ListLayerVersionsInputBuilder) -> impl Future<Output = Result<ListLayerVersionsOutput, SdkError<ListLayerVersionsError>>> {
        (*self).list_layer_versions(builder)
    }
    fn list_layers(&self, builder: ListLayersInputBuilder) -> impl Future<Output = Result<ListLayersOutput, SdkError<ListLayersError>>> {
        (*self).list_layers(builder)
    }
    fn list_provisioned_concurrency_configs(&self, builder: ListProvisionedConcurrencyConfigsInputBuilder) -> impl Future<Output = Result<ListProvisionedConcurrencyConfigsOutput, SdkError<ListProvisionedConcurrencyConfigsError>>> {
        (*self).list_provisioned_concurrency_configs(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        (*self).list_tags(builder)
    }
    fn list_versions_by_function(&self, builder: ListVersionsByFunctionInputBuilder) -> impl Future<Output = Result<ListVersionsByFunctionOutput, SdkError<ListVersionsByFunctionError>>> {
        (*self).list_versions_by_function(builder)
    }
    fn publish_layer_version(&self, builder: PublishLayerVersionInputBuilder) -> impl Future<Output = Result<PublishLayerVersionOutput, SdkError<PublishLayerVersionError>>> {
        (*self).publish_layer_version(builder)
    }
    fn publish_version(&self, builder: PublishVersionInputBuilder) -> impl Future<Output = Result<PublishVersionOutput, SdkError<PublishVersionError>>> {
        (*self).publish_version(builder)
    }
    fn put_function_code_signing_config(&self, builder: PutFunctionCodeSigningConfigInputBuilder) -> impl Future<Output = Result<PutFunctionCodeSigningConfigOutput, SdkError<PutFunctionCodeSigningConfigError>>> {
        (*self).put_function_code_signing_config(builder)
    }
    fn put_function_concurrency(&self, builder: PutFunctionConcurrencyInputBuilder) -> impl Future<Output = Result<PutFunctionConcurrencyOutput, SdkError<PutFunctionConcurrencyError>>> {
        (*self).put_function_concurrency(builder)
    }
    fn put_function_event_invoke_config(&self, builder: PutFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<PutFunctionEventInvokeConfigOutput, SdkError<PutFunctionEventInvokeConfigError>>> {
        (*self).put_function_event_invoke_config(builder)
    }
    fn put_provisioned_concurrency_config(&self, builder: PutProvisionedConcurrencyConfigInputBuilder) -> impl Future<Output = Result<PutProvisionedConcurrencyConfigOutput, SdkError<PutProvisionedConcurrencyConfigError>>> {
        (*self).put_provisioned_concurrency_config(builder)
    }
    fn put_runtime_management_config(&self, builder: PutRuntimeManagementConfigInputBuilder) -> impl Future<Output = Result<PutRuntimeManagementConfigOutput, SdkError<PutRuntimeManagementConfigError>>> {
        (*self).put_runtime_management_config(builder)
    }
    fn remove_layer_version_permission(&self, builder: RemoveLayerVersionPermissionInputBuilder) -> impl Future<Output = Result<RemoveLayerVersionPermissionOutput, SdkError<RemoveLayerVersionPermissionError>>> {
        (*self).remove_layer_version_permission(builder)
    }
    fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> impl Future<Output = Result<RemovePermissionOutput, SdkError<RemovePermissionError>>> {
        (*self).remove_permission(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_alias(&self, builder: UpdateAliasInputBuilder) -> impl Future<Output = Result<UpdateAliasOutput, SdkError<UpdateAliasError>>> {
        (*self).update_alias(builder)
    }
    fn update_code_signing_config(&self, builder: UpdateCodeSigningConfigInputBuilder) -> impl Future<Output = Result<UpdateCodeSigningConfigOutput, SdkError<UpdateCodeSigningConfigError>>> {
        (*self).update_code_signing_config(builder)
    }
    fn update_event_source_mapping(&self, builder: UpdateEventSourceMappingInputBuilder) -> impl Future<Output = Result<UpdateEventSourceMappingOutput, SdkError<UpdateEventSourceMappingError>>> {
        (*self).update_event_source_mapping(builder)
    }
    fn update_function_code(&self, builder: UpdateFunctionCodeInputBuilder) -> impl Future<Output = Result<UpdateFunctionCodeOutput, SdkError<UpdateFunctionCodeError>>> {
        (*self).update_function_code(builder)
    }
    fn update_function_configuration(&self, builder: UpdateFunctionConfigurationInputBuilder) -> impl Future<Output = Result<UpdateFunctionConfigurationOutput, SdkError<UpdateFunctionConfigurationError>>> {
        (*self).update_function_configuration(builder)
    }
    fn update_function_event_invoke_config(&self, builder: UpdateFunctionEventInvokeConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionEventInvokeConfigOutput, SdkError<UpdateFunctionEventInvokeConfigError>>> {
        (*self).update_function_event_invoke_config(builder)
    }
    fn update_function_url_config(&self, builder: UpdateFunctionUrlConfigInputBuilder) -> impl Future<Output = Result<UpdateFunctionUrlConfigOutput, SdkError<UpdateFunctionUrlConfigError>>> {
        (*self).update_function_url_config(builder)
    }
}
#[cfg(feature = "mocks")]
mockall::mock! {
    pub edLambdaClient {}
    impl LambdaClient for edLambdaClient {
        async fn add_layer_version_permission(&self, builder: AddLayerVersionPermissionInputBuilder) -> Result<AddLayerVersionPermissionOutput, SdkError<AddLayerVersionPermissionError>>;
        async fn add_permission(&self, builder: AddPermissionInputBuilder) -> Result<AddPermissionOutput, SdkError<AddPermissionError>>;
        async fn create_alias(&self, builder: CreateAliasInputBuilder) -> Result<CreateAliasOutput, SdkError<CreateAliasError>>;
        async fn create_code_signing_config(&self, builder: CreateCodeSigningConfigInputBuilder) -> Result<CreateCodeSigningConfigOutput, SdkError<CreateCodeSigningConfigError>>;
        async fn create_event_source_mapping(&self, builder: CreateEventSourceMappingInputBuilder) -> Result<CreateEventSourceMappingOutput, SdkError<CreateEventSourceMappingError>>;
        async fn create_function(&self, builder: CreateFunctionInputBuilder) -> Result<CreateFunctionOutput, SdkError<CreateFunctionError>>;
        async fn create_function_url_config(&self, builder: CreateFunctionUrlConfigInputBuilder) -> Result<CreateFunctionUrlConfigOutput, SdkError<CreateFunctionUrlConfigError>>;
        async fn delete_alias(&self, builder: DeleteAliasInputBuilder) -> Result<DeleteAliasOutput, SdkError<DeleteAliasError>>;
        async fn delete_code_signing_config(&self, builder: DeleteCodeSigningConfigInputBuilder) -> Result<DeleteCodeSigningConfigOutput, SdkError<DeleteCodeSigningConfigError>>;
        async fn delete_event_source_mapping(&self, builder: DeleteEventSourceMappingInputBuilder) -> Result<DeleteEventSourceMappingOutput, SdkError<DeleteEventSourceMappingError>>;
        async fn delete_function(&self, builder: DeleteFunctionInputBuilder) -> Result<DeleteFunctionOutput, SdkError<DeleteFunctionError>>;
        async fn delete_function_code_signing_config(&self, builder: DeleteFunctionCodeSigningConfigInputBuilder) -> Result<DeleteFunctionCodeSigningConfigOutput, SdkError<DeleteFunctionCodeSigningConfigError>>;
        async fn delete_function_concurrency(&self, builder: DeleteFunctionConcurrencyInputBuilder) -> Result<DeleteFunctionConcurrencyOutput, SdkError<DeleteFunctionConcurrencyError>>;
        async fn delete_function_event_invoke_config(&self, builder: DeleteFunctionEventInvokeConfigInputBuilder) -> Result<DeleteFunctionEventInvokeConfigOutput, SdkError<DeleteFunctionEventInvokeConfigError>>;
        async fn delete_function_url_config(&self, builder: DeleteFunctionUrlConfigInputBuilder) -> Result<DeleteFunctionUrlConfigOutput, SdkError<DeleteFunctionUrlConfigError>>;
        async fn delete_layer_version(&self, builder: DeleteLayerVersionInputBuilder) -> Result<DeleteLayerVersionOutput, SdkError<DeleteLayerVersionError>>;
        async fn delete_provisioned_concurrency_config(&self, builder: DeleteProvisionedConcurrencyConfigInputBuilder) -> Result<DeleteProvisionedConcurrencyConfigOutput, SdkError<DeleteProvisionedConcurrencyConfigError>>;
        async fn get_account_settings(&self, builder: GetAccountSettingsInputBuilder) -> Result<GetAccountSettingsOutput, SdkError<GetAccountSettingsError>>;
        async fn get_alias(&self, builder: GetAliasInputBuilder) -> Result<GetAliasOutput, SdkError<GetAliasError>>;
        async fn get_code_signing_config(&self, builder: GetCodeSigningConfigInputBuilder) -> Result<GetCodeSigningConfigOutput, SdkError<GetCodeSigningConfigError>>;
        async fn get_event_source_mapping(&self, builder: GetEventSourceMappingInputBuilder) -> Result<GetEventSourceMappingOutput, SdkError<GetEventSourceMappingError>>;
        async fn get_function(&self, builder: GetFunctionInputBuilder) -> Result<GetFunctionOutput, SdkError<GetFunctionError>>;
        async fn get_function_code_signing_config(&self, builder: GetFunctionCodeSigningConfigInputBuilder) -> Result<GetFunctionCodeSigningConfigOutput, SdkError<GetFunctionCodeSigningConfigError>>;
        async fn get_function_concurrency(&self, builder: GetFunctionConcurrencyInputBuilder) -> Result<GetFunctionConcurrencyOutput, SdkError<GetFunctionConcurrencyError>>;
        async fn get_function_configuration(&self, builder: GetFunctionConfigurationInputBuilder) -> Result<GetFunctionConfigurationOutput, SdkError<GetFunctionConfigurationError>>;
        async fn get_function_event_invoke_config(&self, builder: GetFunctionEventInvokeConfigInputBuilder) -> Result<GetFunctionEventInvokeConfigOutput, SdkError<GetFunctionEventInvokeConfigError>>;
        async fn get_function_url_config(&self, builder: GetFunctionUrlConfigInputBuilder) -> Result<GetFunctionUrlConfigOutput, SdkError<GetFunctionUrlConfigError>>;
        async fn get_layer_version(&self, builder: GetLayerVersionInputBuilder) -> Result<GetLayerVersionOutput, SdkError<GetLayerVersionError>>;
        async fn get_layer_version_by_arn(&self, builder: GetLayerVersionByArnInputBuilder) -> Result<GetLayerVersionByArnOutput, SdkError<GetLayerVersionByArnError>>;
        async fn get_layer_version_policy(&self, builder: GetLayerVersionPolicyInputBuilder) -> Result<GetLayerVersionPolicyOutput, SdkError<GetLayerVersionPolicyError>>;
        async fn get_policy(&self, builder: GetPolicyInputBuilder) -> Result<GetPolicyOutput, SdkError<GetPolicyError>>;
        async fn get_provisioned_concurrency_config(&self, builder: GetProvisionedConcurrencyConfigInputBuilder) -> Result<GetProvisionedConcurrencyConfigOutput, SdkError<GetProvisionedConcurrencyConfigError>>;
        async fn get_runtime_management_config(&self, builder: GetRuntimeManagementConfigInputBuilder) -> Result<GetRuntimeManagementConfigOutput, SdkError<GetRuntimeManagementConfigError>>;
        async fn invoke(&self, builder: InvokeInputBuilder) -> Result<InvokeOutput, SdkError<InvokeError>>;
        async fn list_aliases(&self, builder: ListAliasesInputBuilder) -> Result<ListAliasesOutput, SdkError<ListAliasesError>>;
        async fn list_code_signing_configs(&self, builder: ListCodeSigningConfigsInputBuilder) -> Result<ListCodeSigningConfigsOutput, SdkError<ListCodeSigningConfigsError>>;
        async fn list_event_source_mappings(&self, builder: ListEventSourceMappingsInputBuilder) -> Result<ListEventSourceMappingsOutput, SdkError<ListEventSourceMappingsError>>;
        async fn list_function_event_invoke_configs(&self, builder: ListFunctionEventInvokeConfigsInputBuilder) -> Result<ListFunctionEventInvokeConfigsOutput, SdkError<ListFunctionEventInvokeConfigsError>>;
        async fn list_function_url_configs(&self, builder: ListFunctionUrlConfigsInputBuilder) -> Result<ListFunctionUrlConfigsOutput, SdkError<ListFunctionUrlConfigsError>>;
        async fn list_functions(&self, builder: ListFunctionsInputBuilder) -> Result<ListFunctionsOutput, SdkError<ListFunctionsError>>;
        async fn list_functions_by_code_signing_config(&self, builder: ListFunctionsByCodeSigningConfigInputBuilder) -> Result<ListFunctionsByCodeSigningConfigOutput, SdkError<ListFunctionsByCodeSigningConfigError>>;
        async fn list_layer_versions(&self, builder: ListLayerVersionsInputBuilder) -> Result<ListLayerVersionsOutput, SdkError<ListLayerVersionsError>>;
        async fn list_layers(&self, builder: ListLayersInputBuilder) -> Result<ListLayersOutput, SdkError<ListLayersError>>;
        async fn list_provisioned_concurrency_configs(&self, builder: ListProvisionedConcurrencyConfigsInputBuilder) -> Result<ListProvisionedConcurrencyConfigsOutput, SdkError<ListProvisionedConcurrencyConfigsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn list_versions_by_function(&self, builder: ListVersionsByFunctionInputBuilder) -> Result<ListVersionsByFunctionOutput, SdkError<ListVersionsByFunctionError>>;
        async fn publish_layer_version(&self, builder: PublishLayerVersionInputBuilder) -> Result<PublishLayerVersionOutput, SdkError<PublishLayerVersionError>>;
        async fn publish_version(&self, builder: PublishVersionInputBuilder) -> Result<PublishVersionOutput, SdkError<PublishVersionError>>;
        async fn put_function_code_signing_config(&self, builder: PutFunctionCodeSigningConfigInputBuilder) -> Result<PutFunctionCodeSigningConfigOutput, SdkError<PutFunctionCodeSigningConfigError>>;
        async fn put_function_concurrency(&self, builder: PutFunctionConcurrencyInputBuilder) -> Result<PutFunctionConcurrencyOutput, SdkError<PutFunctionConcurrencyError>>;
        async fn put_function_event_invoke_config(&self, builder: PutFunctionEventInvokeConfigInputBuilder) -> Result<PutFunctionEventInvokeConfigOutput, SdkError<PutFunctionEventInvokeConfigError>>;
        async fn put_provisioned_concurrency_config(&self, builder: PutProvisionedConcurrencyConfigInputBuilder) -> Result<PutProvisionedConcurrencyConfigOutput, SdkError<PutProvisionedConcurrencyConfigError>>;
        async fn put_runtime_management_config(&self, builder: PutRuntimeManagementConfigInputBuilder) -> Result<PutRuntimeManagementConfigOutput, SdkError<PutRuntimeManagementConfigError>>;
        async fn remove_layer_version_permission(&self, builder: RemoveLayerVersionPermissionInputBuilder) -> Result<RemoveLayerVersionPermissionOutput, SdkError<RemoveLayerVersionPermissionError>>;
        async fn remove_permission(&self, builder: RemovePermissionInputBuilder) -> Result<RemovePermissionOutput, SdkError<RemovePermissionError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_alias(&self, builder: UpdateAliasInputBuilder) -> Result<UpdateAliasOutput, SdkError<UpdateAliasError>>;
        async fn update_code_signing_config(&self, builder: UpdateCodeSigningConfigInputBuilder) -> Result<UpdateCodeSigningConfigOutput, SdkError<UpdateCodeSigningConfigError>>;
        async fn update_event_source_mapping(&self, builder: UpdateEventSourceMappingInputBuilder) -> Result<UpdateEventSourceMappingOutput, SdkError<UpdateEventSourceMappingError>>;
        async fn update_function_code(&self, builder: UpdateFunctionCodeInputBuilder) -> Result<UpdateFunctionCodeOutput, SdkError<UpdateFunctionCodeError>>;
        async fn update_function_configuration(&self, builder: UpdateFunctionConfigurationInputBuilder) -> Result<UpdateFunctionConfigurationOutput, SdkError<UpdateFunctionConfigurationError>>;
        async fn update_function_event_invoke_config(&self, builder: UpdateFunctionEventInvokeConfigInputBuilder) -> Result<UpdateFunctionEventInvokeConfigOutput, SdkError<UpdateFunctionEventInvokeConfigError>>;
        async fn update_function_url_config(&self, builder: UpdateFunctionUrlConfigInputBuilder) -> Result<UpdateFunctionUrlConfigOutput, SdkError<UpdateFunctionUrlConfigError>>;
    }
}
