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
use aws_sdk_acm::operation::add_tags_to_certificate::{builders::*, *};
use aws_sdk_acm::operation::delete_certificate::{builders::*, *};
use aws_sdk_acm::operation::describe_certificate::{builders::*, *};
use aws_sdk_acm::operation::export_certificate::{builders::*, *};
use aws_sdk_acm::operation::get_account_configuration::{builders::*, *};
use aws_sdk_acm::operation::get_certificate::{builders::*, *};
use aws_sdk_acm::operation::import_certificate::{builders::*, *};
use aws_sdk_acm::operation::list_certificates::{builders::*, *};
use aws_sdk_acm::operation::list_tags_for_certificate::{builders::*, *};
use aws_sdk_acm::operation::put_account_configuration::{builders::*, *};
use aws_sdk_acm::operation::remove_tags_from_certificate::{builders::*, *};
use aws_sdk_acm::operation::renew_certificate::{builders::*, *};
use aws_sdk_acm::operation::request_certificate::{builders::*, *};
use aws_sdk_acm::operation::resend_validation_email::{builders::*, *};
use aws_sdk_acm::operation::update_certificate_options::{builders::*, *};
use aws_sdk_acm::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_acm::Client;

pub use aws_sdk_acm::*;

pub struct ACMClientImpl(Client);
impl ACMClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait ACMClient {
    fn add_tags_to_certificate(&self, builder: AddTagsToCertificateInputBuilder) -> impl Future<Output = Result<AddTagsToCertificateOutput, SdkError<AddTagsToCertificateError>>>;
    fn delete_certificate(&self, builder: DeleteCertificateInputBuilder) -> impl Future<Output = Result<DeleteCertificateOutput, SdkError<DeleteCertificateError>>>;
    fn describe_certificate(&self, builder: DescribeCertificateInputBuilder) -> impl Future<Output = Result<DescribeCertificateOutput, SdkError<DescribeCertificateError>>>;
    fn export_certificate(&self, builder: ExportCertificateInputBuilder) -> impl Future<Output = Result<ExportCertificateOutput, SdkError<ExportCertificateError>>>;
    fn get_account_configuration(&self, builder: GetAccountConfigurationInputBuilder) -> impl Future<Output = Result<GetAccountConfigurationOutput, SdkError<GetAccountConfigurationError>>>;
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>>;
    fn import_certificate(&self, builder: ImportCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateOutput, SdkError<ImportCertificateError>>>;
    fn list_certificates(&self, builder: ListCertificatesInputBuilder) -> impl Future<Output = Result<ListCertificatesOutput, SdkError<ListCertificatesError>>>;
    fn list_tags_for_certificate(&self, builder: ListTagsForCertificateInputBuilder) -> impl Future<Output = Result<ListTagsForCertificateOutput, SdkError<ListTagsForCertificateError>>>;
    fn put_account_configuration(&self, builder: PutAccountConfigurationInputBuilder) -> impl Future<Output = Result<PutAccountConfigurationOutput, SdkError<PutAccountConfigurationError>>>;
    fn remove_tags_from_certificate(&self, builder: RemoveTagsFromCertificateInputBuilder) -> impl Future<Output = Result<RemoveTagsFromCertificateOutput, SdkError<RemoveTagsFromCertificateError>>>;
    fn renew_certificate(&self, builder: RenewCertificateInputBuilder) -> impl Future<Output = Result<RenewCertificateOutput, SdkError<RenewCertificateError>>>;
    fn request_certificate(&self, builder: RequestCertificateInputBuilder) -> impl Future<Output = Result<RequestCertificateOutput, SdkError<RequestCertificateError>>>;
    fn resend_validation_email(&self, builder: ResendValidationEmailInputBuilder) -> impl Future<Output = Result<ResendValidationEmailOutput, SdkError<ResendValidationEmailError>>>;
    fn update_certificate_options(&self, builder: UpdateCertificateOptionsInputBuilder) -> impl Future<Output = Result<UpdateCertificateOptionsOutput, SdkError<UpdateCertificateOptionsError>>>;
}
impl ACMClient for ACMClientImpl {
    fn add_tags_to_certificate(&self, builder: AddTagsToCertificateInputBuilder) -> impl Future<Output = Result<AddTagsToCertificateOutput, SdkError<AddTagsToCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_certificate(&self, builder: DeleteCertificateInputBuilder) -> impl Future<Output = Result<DeleteCertificateOutput, SdkError<DeleteCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn describe_certificate(&self, builder: DescribeCertificateInputBuilder) -> impl Future<Output = Result<DescribeCertificateOutput, SdkError<DescribeCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn export_certificate(&self, builder: ExportCertificateInputBuilder) -> impl Future<Output = Result<ExportCertificateOutput, SdkError<ExportCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_account_configuration(&self, builder: GetAccountConfigurationInputBuilder) -> impl Future<Output = Result<GetAccountConfigurationOutput, SdkError<GetAccountConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn import_certificate(&self, builder: ImportCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateOutput, SdkError<ImportCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn list_certificates(&self, builder: ListCertificatesInputBuilder) -> impl Future<Output = Result<ListCertificatesOutput, SdkError<ListCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_certificate(&self, builder: ListTagsForCertificateInputBuilder) -> impl Future<Output = Result<ListTagsForCertificateOutput, SdkError<ListTagsForCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn put_account_configuration(&self, builder: PutAccountConfigurationInputBuilder) -> impl Future<Output = Result<PutAccountConfigurationOutput, SdkError<PutAccountConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn remove_tags_from_certificate(&self, builder: RemoveTagsFromCertificateInputBuilder) -> impl Future<Output = Result<RemoveTagsFromCertificateOutput, SdkError<RemoveTagsFromCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn renew_certificate(&self, builder: RenewCertificateInputBuilder) -> impl Future<Output = Result<RenewCertificateOutput, SdkError<RenewCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn request_certificate(&self, builder: RequestCertificateInputBuilder) -> impl Future<Output = Result<RequestCertificateOutput, SdkError<RequestCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn resend_validation_email(&self, builder: ResendValidationEmailInputBuilder) -> impl Future<Output = Result<ResendValidationEmailOutput, SdkError<ResendValidationEmailError>>> {
        builder.send_with(&self.0)
    }
    fn update_certificate_options(&self, builder: UpdateCertificateOptionsInputBuilder) -> impl Future<Output = Result<UpdateCertificateOptionsOutput, SdkError<UpdateCertificateOptionsError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: ACMClient> ACMClient for &T {
    fn add_tags_to_certificate(&self, builder: AddTagsToCertificateInputBuilder) -> impl Future<Output = Result<AddTagsToCertificateOutput, SdkError<AddTagsToCertificateError>>> {
        (*self).add_tags_to_certificate(builder)
    }
    fn delete_certificate(&self, builder: DeleteCertificateInputBuilder) -> impl Future<Output = Result<DeleteCertificateOutput, SdkError<DeleteCertificateError>>> {
        (*self).delete_certificate(builder)
    }
    fn describe_certificate(&self, builder: DescribeCertificateInputBuilder) -> impl Future<Output = Result<DescribeCertificateOutput, SdkError<DescribeCertificateError>>> {
        (*self).describe_certificate(builder)
    }
    fn export_certificate(&self, builder: ExportCertificateInputBuilder) -> impl Future<Output = Result<ExportCertificateOutput, SdkError<ExportCertificateError>>> {
        (*self).export_certificate(builder)
    }
    fn get_account_configuration(&self, builder: GetAccountConfigurationInputBuilder) -> impl Future<Output = Result<GetAccountConfigurationOutput, SdkError<GetAccountConfigurationError>>> {
        (*self).get_account_configuration(builder)
    }
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>> {
        (*self).get_certificate(builder)
    }
    fn import_certificate(&self, builder: ImportCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateOutput, SdkError<ImportCertificateError>>> {
        (*self).import_certificate(builder)
    }
    fn list_certificates(&self, builder: ListCertificatesInputBuilder) -> impl Future<Output = Result<ListCertificatesOutput, SdkError<ListCertificatesError>>> {
        (*self).list_certificates(builder)
    }
    fn list_tags_for_certificate(&self, builder: ListTagsForCertificateInputBuilder) -> impl Future<Output = Result<ListTagsForCertificateOutput, SdkError<ListTagsForCertificateError>>> {
        (*self).list_tags_for_certificate(builder)
    }
    fn put_account_configuration(&self, builder: PutAccountConfigurationInputBuilder) -> impl Future<Output = Result<PutAccountConfigurationOutput, SdkError<PutAccountConfigurationError>>> {
        (*self).put_account_configuration(builder)
    }
    fn remove_tags_from_certificate(&self, builder: RemoveTagsFromCertificateInputBuilder) -> impl Future<Output = Result<RemoveTagsFromCertificateOutput, SdkError<RemoveTagsFromCertificateError>>> {
        (*self).remove_tags_from_certificate(builder)
    }
    fn renew_certificate(&self, builder: RenewCertificateInputBuilder) -> impl Future<Output = Result<RenewCertificateOutput, SdkError<RenewCertificateError>>> {
        (*self).renew_certificate(builder)
    }
    fn request_certificate(&self, builder: RequestCertificateInputBuilder) -> impl Future<Output = Result<RequestCertificateOutput, SdkError<RequestCertificateError>>> {
        (*self).request_certificate(builder)
    }
    fn resend_validation_email(&self, builder: ResendValidationEmailInputBuilder) -> impl Future<Output = Result<ResendValidationEmailOutput, SdkError<ResendValidationEmailError>>> {
        (*self).resend_validation_email(builder)
    }
    fn update_certificate_options(&self, builder: UpdateCertificateOptionsInputBuilder) -> impl Future<Output = Result<UpdateCertificateOptionsOutput, SdkError<UpdateCertificateOptionsError>>> {
        (*self).update_certificate_options(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edACMClient {}
    impl ACMClient for edACMClient {
        async fn add_tags_to_certificate(&self, builder: AddTagsToCertificateInputBuilder) -> Result<AddTagsToCertificateOutput, SdkError<AddTagsToCertificateError>>;
        async fn delete_certificate(&self, builder: DeleteCertificateInputBuilder) -> Result<DeleteCertificateOutput, SdkError<DeleteCertificateError>>;
        async fn describe_certificate(&self, builder: DescribeCertificateInputBuilder) -> Result<DescribeCertificateOutput, SdkError<DescribeCertificateError>>;
        async fn export_certificate(&self, builder: ExportCertificateInputBuilder) -> Result<ExportCertificateOutput, SdkError<ExportCertificateError>>;
        async fn get_account_configuration(&self, builder: GetAccountConfigurationInputBuilder) -> Result<GetAccountConfigurationOutput, SdkError<GetAccountConfigurationError>>;
        async fn get_certificate(&self, builder: GetCertificateInputBuilder) -> Result<GetCertificateOutput, SdkError<GetCertificateError>>;
        async fn import_certificate(&self, builder: ImportCertificateInputBuilder) -> Result<ImportCertificateOutput, SdkError<ImportCertificateError>>;
        async fn list_certificates(&self, builder: ListCertificatesInputBuilder) -> Result<ListCertificatesOutput, SdkError<ListCertificatesError>>;
        async fn list_tags_for_certificate(&self, builder: ListTagsForCertificateInputBuilder) -> Result<ListTagsForCertificateOutput, SdkError<ListTagsForCertificateError>>;
        async fn put_account_configuration(&self, builder: PutAccountConfigurationInputBuilder) -> Result<PutAccountConfigurationOutput, SdkError<PutAccountConfigurationError>>;
        async fn remove_tags_from_certificate(&self, builder: RemoveTagsFromCertificateInputBuilder) -> Result<RemoveTagsFromCertificateOutput, SdkError<RemoveTagsFromCertificateError>>;
        async fn renew_certificate(&self, builder: RenewCertificateInputBuilder) -> Result<RenewCertificateOutput, SdkError<RenewCertificateError>>;
        async fn request_certificate(&self, builder: RequestCertificateInputBuilder) -> Result<RequestCertificateOutput, SdkError<RequestCertificateError>>;
        async fn resend_validation_email(&self, builder: ResendValidationEmailInputBuilder) -> Result<ResendValidationEmailOutput, SdkError<ResendValidationEmailError>>;
        async fn update_certificate_options(&self, builder: UpdateCertificateOptionsInputBuilder) -> Result<UpdateCertificateOptionsOutput, SdkError<UpdateCertificateOptionsError>>;
    }
}
