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
use aws_sdk_amplifyuibuilder::operation::create_component::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::create_form::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::create_theme::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::delete_component::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::delete_form::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::delete_theme::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::exchange_code_for_token::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::export_components::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::export_forms::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::export_themes::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::get_codegen_job::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::get_component::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::get_form::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::get_metadata::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::get_theme::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::list_codegen_jobs::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::list_components::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::list_forms::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::list_themes::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::put_metadata_flag::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::refresh_token::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::start_codegen_job::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::tag_resource::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::untag_resource::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::update_component::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::update_form::{builders::*, *};
use aws_sdk_amplifyuibuilder::operation::update_theme::{builders::*, *};
use aws_sdk_amplifyuibuilder::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_amplifyuibuilder::Client;
use std::ops::Deref;

pub use aws_sdk_amplifyuibuilder::*;

pub struct AmplifyUiBuilderClientImpl(Client);
impl AmplifyUiBuilderClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AmplifyUiBuilderClient {
    fn create_component(&self, builder: CreateComponentInputBuilder) -> impl Future<Output = Result<CreateComponentOutput, SdkError<CreateComponentError>>>;
    fn create_form(&self, builder: CreateFormInputBuilder) -> impl Future<Output = Result<CreateFormOutput, SdkError<CreateFormError>>>;
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>>;
    fn delete_component(&self, builder: DeleteComponentInputBuilder) -> impl Future<Output = Result<DeleteComponentOutput, SdkError<DeleteComponentError>>>;
    fn delete_form(&self, builder: DeleteFormInputBuilder) -> impl Future<Output = Result<DeleteFormOutput, SdkError<DeleteFormError>>>;
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>>;
    fn exchange_code_for_token(&self, builder: ExchangeCodeForTokenInputBuilder) -> impl Future<Output = Result<ExchangeCodeForTokenOutput, SdkError<ExchangeCodeForTokenError>>>;
    fn export_components(&self, builder: ExportComponentsInputBuilder) -> impl Future<Output = Result<ExportComponentsOutput, SdkError<ExportComponentsError>>>;
    fn export_forms(&self, builder: ExportFormsInputBuilder) -> impl Future<Output = Result<ExportFormsOutput, SdkError<ExportFormsError>>>;
    fn export_themes(&self, builder: ExportThemesInputBuilder) -> impl Future<Output = Result<ExportThemesOutput, SdkError<ExportThemesError>>>;
    fn get_codegen_job(&self, builder: GetCodegenJobInputBuilder) -> impl Future<Output = Result<GetCodegenJobOutput, SdkError<GetCodegenJobError>>>;
    fn get_component(&self, builder: GetComponentInputBuilder) -> impl Future<Output = Result<GetComponentOutput, SdkError<GetComponentError>>>;
    fn get_form(&self, builder: GetFormInputBuilder) -> impl Future<Output = Result<GetFormOutput, SdkError<GetFormError>>>;
    fn get_metadata(&self, builder: GetMetadataInputBuilder) -> impl Future<Output = Result<GetMetadataOutput, SdkError<GetMetadataError>>>;
    fn get_theme(&self, builder: GetThemeInputBuilder) -> impl Future<Output = Result<GetThemeOutput, SdkError<GetThemeError>>>;
    fn list_codegen_jobs(&self, builder: ListCodegenJobsInputBuilder) -> impl Future<Output = Result<ListCodegenJobsOutput, SdkError<ListCodegenJobsError>>>;
    fn list_components(&self, builder: ListComponentsInputBuilder) -> impl Future<Output = Result<ListComponentsOutput, SdkError<ListComponentsError>>>;
    fn list_forms(&self, builder: ListFormsInputBuilder) -> impl Future<Output = Result<ListFormsOutput, SdkError<ListFormsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>>;
    fn put_metadata_flag(&self, builder: PutMetadataFlagInputBuilder) -> impl Future<Output = Result<PutMetadataFlagOutput, SdkError<PutMetadataFlagError>>>;
    fn refresh_token(&self, builder: RefreshTokenInputBuilder) -> impl Future<Output = Result<RefreshTokenOutput, SdkError<RefreshTokenError>>>;
    fn start_codegen_job(&self, builder: StartCodegenJobInputBuilder) -> impl Future<Output = Result<StartCodegenJobOutput, SdkError<StartCodegenJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_component(&self, builder: UpdateComponentInputBuilder) -> impl Future<Output = Result<UpdateComponentOutput, SdkError<UpdateComponentError>>>;
    fn update_form(&self, builder: UpdateFormInputBuilder) -> impl Future<Output = Result<UpdateFormOutput, SdkError<UpdateFormError>>>;
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>>;
}
impl AmplifyUiBuilderClient for AmplifyUiBuilderClientImpl {
    fn create_component(&self, builder: CreateComponentInputBuilder) -> impl Future<Output = Result<CreateComponentOutput, SdkError<CreateComponentError>>> {
        builder.send_with(&self.0)
    }
    fn create_form(&self, builder: CreateFormInputBuilder) -> impl Future<Output = Result<CreateFormOutput, SdkError<CreateFormError>>> {
        builder.send_with(&self.0)
    }
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_component(&self, builder: DeleteComponentInputBuilder) -> impl Future<Output = Result<DeleteComponentOutput, SdkError<DeleteComponentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_form(&self, builder: DeleteFormInputBuilder) -> impl Future<Output = Result<DeleteFormOutput, SdkError<DeleteFormError>>> {
        builder.send_with(&self.0)
    }
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>> {
        builder.send_with(&self.0)
    }
    fn exchange_code_for_token(&self, builder: ExchangeCodeForTokenInputBuilder) -> impl Future<Output = Result<ExchangeCodeForTokenOutput, SdkError<ExchangeCodeForTokenError>>> {
        builder.send_with(&self.0)
    }
    fn export_components(&self, builder: ExportComponentsInputBuilder) -> impl Future<Output = Result<ExportComponentsOutput, SdkError<ExportComponentsError>>> {
        builder.send_with(&self.0)
    }
    fn export_forms(&self, builder: ExportFormsInputBuilder) -> impl Future<Output = Result<ExportFormsOutput, SdkError<ExportFormsError>>> {
        builder.send_with(&self.0)
    }
    fn export_themes(&self, builder: ExportThemesInputBuilder) -> impl Future<Output = Result<ExportThemesOutput, SdkError<ExportThemesError>>> {
        builder.send_with(&self.0)
    }
    fn get_codegen_job(&self, builder: GetCodegenJobInputBuilder) -> impl Future<Output = Result<GetCodegenJobOutput, SdkError<GetCodegenJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_component(&self, builder: GetComponentInputBuilder) -> impl Future<Output = Result<GetComponentOutput, SdkError<GetComponentError>>> {
        builder.send_with(&self.0)
    }
    fn get_form(&self, builder: GetFormInputBuilder) -> impl Future<Output = Result<GetFormOutput, SdkError<GetFormError>>> {
        builder.send_with(&self.0)
    }
    fn get_metadata(&self, builder: GetMetadataInputBuilder) -> impl Future<Output = Result<GetMetadataOutput, SdkError<GetMetadataError>>> {
        builder.send_with(&self.0)
    }
    fn get_theme(&self, builder: GetThemeInputBuilder) -> impl Future<Output = Result<GetThemeOutput, SdkError<GetThemeError>>> {
        builder.send_with(&self.0)
    }
    fn list_codegen_jobs(&self, builder: ListCodegenJobsInputBuilder) -> impl Future<Output = Result<ListCodegenJobsOutput, SdkError<ListCodegenJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_components(&self, builder: ListComponentsInputBuilder) -> impl Future<Output = Result<ListComponentsOutput, SdkError<ListComponentsError>>> {
        builder.send_with(&self.0)
    }
    fn list_forms(&self, builder: ListFormsInputBuilder) -> impl Future<Output = Result<ListFormsOutput, SdkError<ListFormsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>> {
        builder.send_with(&self.0)
    }
    fn put_metadata_flag(&self, builder: PutMetadataFlagInputBuilder) -> impl Future<Output = Result<PutMetadataFlagOutput, SdkError<PutMetadataFlagError>>> {
        builder.send_with(&self.0)
    }
    fn refresh_token(&self, builder: RefreshTokenInputBuilder) -> impl Future<Output = Result<RefreshTokenOutput, SdkError<RefreshTokenError>>> {
        builder.send_with(&self.0)
    }
    fn start_codegen_job(&self, builder: StartCodegenJobInputBuilder) -> impl Future<Output = Result<StartCodegenJobOutput, SdkError<StartCodegenJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_component(&self, builder: UpdateComponentInputBuilder) -> impl Future<Output = Result<UpdateComponentOutput, SdkError<UpdateComponentError>>> {
        builder.send_with(&self.0)
    }
    fn update_form(&self, builder: UpdateFormInputBuilder) -> impl Future<Output = Result<UpdateFormOutput, SdkError<UpdateFormError>>> {
        builder.send_with(&self.0)
    }
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AmplifyUiBuilderClient for T
where T: Deref,
      T::Target: AmplifyUiBuilderClient {
    fn create_component(&self, builder: CreateComponentInputBuilder) -> impl Future<Output = Result<CreateComponentOutput, SdkError<CreateComponentError>>> {
        self.deref().create_component(builder)
    }
    fn create_form(&self, builder: CreateFormInputBuilder) -> impl Future<Output = Result<CreateFormOutput, SdkError<CreateFormError>>> {
        self.deref().create_form(builder)
    }
    fn create_theme(&self, builder: CreateThemeInputBuilder) -> impl Future<Output = Result<CreateThemeOutput, SdkError<CreateThemeError>>> {
        self.deref().create_theme(builder)
    }
    fn delete_component(&self, builder: DeleteComponentInputBuilder) -> impl Future<Output = Result<DeleteComponentOutput, SdkError<DeleteComponentError>>> {
        self.deref().delete_component(builder)
    }
    fn delete_form(&self, builder: DeleteFormInputBuilder) -> impl Future<Output = Result<DeleteFormOutput, SdkError<DeleteFormError>>> {
        self.deref().delete_form(builder)
    }
    fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> impl Future<Output = Result<DeleteThemeOutput, SdkError<DeleteThemeError>>> {
        self.deref().delete_theme(builder)
    }
    fn exchange_code_for_token(&self, builder: ExchangeCodeForTokenInputBuilder) -> impl Future<Output = Result<ExchangeCodeForTokenOutput, SdkError<ExchangeCodeForTokenError>>> {
        self.deref().exchange_code_for_token(builder)
    }
    fn export_components(&self, builder: ExportComponentsInputBuilder) -> impl Future<Output = Result<ExportComponentsOutput, SdkError<ExportComponentsError>>> {
        self.deref().export_components(builder)
    }
    fn export_forms(&self, builder: ExportFormsInputBuilder) -> impl Future<Output = Result<ExportFormsOutput, SdkError<ExportFormsError>>> {
        self.deref().export_forms(builder)
    }
    fn export_themes(&self, builder: ExportThemesInputBuilder) -> impl Future<Output = Result<ExportThemesOutput, SdkError<ExportThemesError>>> {
        self.deref().export_themes(builder)
    }
    fn get_codegen_job(&self, builder: GetCodegenJobInputBuilder) -> impl Future<Output = Result<GetCodegenJobOutput, SdkError<GetCodegenJobError>>> {
        self.deref().get_codegen_job(builder)
    }
    fn get_component(&self, builder: GetComponentInputBuilder) -> impl Future<Output = Result<GetComponentOutput, SdkError<GetComponentError>>> {
        self.deref().get_component(builder)
    }
    fn get_form(&self, builder: GetFormInputBuilder) -> impl Future<Output = Result<GetFormOutput, SdkError<GetFormError>>> {
        self.deref().get_form(builder)
    }
    fn get_metadata(&self, builder: GetMetadataInputBuilder) -> impl Future<Output = Result<GetMetadataOutput, SdkError<GetMetadataError>>> {
        self.deref().get_metadata(builder)
    }
    fn get_theme(&self, builder: GetThemeInputBuilder) -> impl Future<Output = Result<GetThemeOutput, SdkError<GetThemeError>>> {
        self.deref().get_theme(builder)
    }
    fn list_codegen_jobs(&self, builder: ListCodegenJobsInputBuilder) -> impl Future<Output = Result<ListCodegenJobsOutput, SdkError<ListCodegenJobsError>>> {
        self.deref().list_codegen_jobs(builder)
    }
    fn list_components(&self, builder: ListComponentsInputBuilder) -> impl Future<Output = Result<ListComponentsOutput, SdkError<ListComponentsError>>> {
        self.deref().list_components(builder)
    }
    fn list_forms(&self, builder: ListFormsInputBuilder) -> impl Future<Output = Result<ListFormsOutput, SdkError<ListFormsError>>> {
        self.deref().list_forms(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_themes(&self, builder: ListThemesInputBuilder) -> impl Future<Output = Result<ListThemesOutput, SdkError<ListThemesError>>> {
        self.deref().list_themes(builder)
    }
    fn put_metadata_flag(&self, builder: PutMetadataFlagInputBuilder) -> impl Future<Output = Result<PutMetadataFlagOutput, SdkError<PutMetadataFlagError>>> {
        self.deref().put_metadata_flag(builder)
    }
    fn refresh_token(&self, builder: RefreshTokenInputBuilder) -> impl Future<Output = Result<RefreshTokenOutput, SdkError<RefreshTokenError>>> {
        self.deref().refresh_token(builder)
    }
    fn start_codegen_job(&self, builder: StartCodegenJobInputBuilder) -> impl Future<Output = Result<StartCodegenJobOutput, SdkError<StartCodegenJobError>>> {
        self.deref().start_codegen_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_component(&self, builder: UpdateComponentInputBuilder) -> impl Future<Output = Result<UpdateComponentOutput, SdkError<UpdateComponentError>>> {
        self.deref().update_component(builder)
    }
    fn update_form(&self, builder: UpdateFormInputBuilder) -> impl Future<Output = Result<UpdateFormOutput, SdkError<UpdateFormError>>> {
        self.deref().update_form(builder)
    }
    fn update_theme(&self, builder: UpdateThemeInputBuilder) -> impl Future<Output = Result<UpdateThemeOutput, SdkError<UpdateThemeError>>> {
        self.deref().update_theme(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAmplifyUiBuilderClient {}
    impl AmplifyUiBuilderClient for edAmplifyUiBuilderClient {
        async fn create_component(&self, builder: CreateComponentInputBuilder) -> Result<CreateComponentOutput, SdkError<CreateComponentError>>;
        async fn create_form(&self, builder: CreateFormInputBuilder) -> Result<CreateFormOutput, SdkError<CreateFormError>>;
        async fn create_theme(&self, builder: CreateThemeInputBuilder) -> Result<CreateThemeOutput, SdkError<CreateThemeError>>;
        async fn delete_component(&self, builder: DeleteComponentInputBuilder) -> Result<DeleteComponentOutput, SdkError<DeleteComponentError>>;
        async fn delete_form(&self, builder: DeleteFormInputBuilder) -> Result<DeleteFormOutput, SdkError<DeleteFormError>>;
        async fn delete_theme(&self, builder: DeleteThemeInputBuilder) -> Result<DeleteThemeOutput, SdkError<DeleteThemeError>>;
        async fn exchange_code_for_token(&self, builder: ExchangeCodeForTokenInputBuilder) -> Result<ExchangeCodeForTokenOutput, SdkError<ExchangeCodeForTokenError>>;
        async fn export_components(&self, builder: ExportComponentsInputBuilder) -> Result<ExportComponentsOutput, SdkError<ExportComponentsError>>;
        async fn export_forms(&self, builder: ExportFormsInputBuilder) -> Result<ExportFormsOutput, SdkError<ExportFormsError>>;
        async fn export_themes(&self, builder: ExportThemesInputBuilder) -> Result<ExportThemesOutput, SdkError<ExportThemesError>>;
        async fn get_codegen_job(&self, builder: GetCodegenJobInputBuilder) -> Result<GetCodegenJobOutput, SdkError<GetCodegenJobError>>;
        async fn get_component(&self, builder: GetComponentInputBuilder) -> Result<GetComponentOutput, SdkError<GetComponentError>>;
        async fn get_form(&self, builder: GetFormInputBuilder) -> Result<GetFormOutput, SdkError<GetFormError>>;
        async fn get_metadata(&self, builder: GetMetadataInputBuilder) -> Result<GetMetadataOutput, SdkError<GetMetadataError>>;
        async fn get_theme(&self, builder: GetThemeInputBuilder) -> Result<GetThemeOutput, SdkError<GetThemeError>>;
        async fn list_codegen_jobs(&self, builder: ListCodegenJobsInputBuilder) -> Result<ListCodegenJobsOutput, SdkError<ListCodegenJobsError>>;
        async fn list_components(&self, builder: ListComponentsInputBuilder) -> Result<ListComponentsOutput, SdkError<ListComponentsError>>;
        async fn list_forms(&self, builder: ListFormsInputBuilder) -> Result<ListFormsOutput, SdkError<ListFormsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_themes(&self, builder: ListThemesInputBuilder) -> Result<ListThemesOutput, SdkError<ListThemesError>>;
        async fn put_metadata_flag(&self, builder: PutMetadataFlagInputBuilder) -> Result<PutMetadataFlagOutput, SdkError<PutMetadataFlagError>>;
        async fn refresh_token(&self, builder: RefreshTokenInputBuilder) -> Result<RefreshTokenOutput, SdkError<RefreshTokenError>>;
        async fn start_codegen_job(&self, builder: StartCodegenJobInputBuilder) -> Result<StartCodegenJobOutput, SdkError<StartCodegenJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_component(&self, builder: UpdateComponentInputBuilder) -> Result<UpdateComponentOutput, SdkError<UpdateComponentError>>;
        async fn update_form(&self, builder: UpdateFormInputBuilder) -> Result<UpdateFormOutput, SdkError<UpdateFormError>>;
        async fn update_theme(&self, builder: UpdateThemeInputBuilder) -> Result<UpdateThemeOutput, SdkError<UpdateThemeError>>;
    }
}
