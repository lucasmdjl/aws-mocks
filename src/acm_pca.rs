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
use aws_sdk_acmpca::operation::create_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::create_certificate_authority_audit_report::{builders::*, *};
use aws_sdk_acmpca::operation::create_permission::{builders::*, *};
use aws_sdk_acmpca::operation::delete_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::delete_permission::{builders::*, *};
use aws_sdk_acmpca::operation::delete_policy::{builders::*, *};
use aws_sdk_acmpca::operation::describe_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::describe_certificate_authority_audit_report::{builders::*, *};
use aws_sdk_acmpca::operation::get_certificate::{builders::*, *};
use aws_sdk_acmpca::operation::get_certificate_authority_certificate::{builders::*, *};
use aws_sdk_acmpca::operation::get_certificate_authority_csr::{builders::*, *};
use aws_sdk_acmpca::operation::get_policy::{builders::*, *};
use aws_sdk_acmpca::operation::import_certificate_authority_certificate::{builders::*, *};
use aws_sdk_acmpca::operation::issue_certificate::{builders::*, *};
use aws_sdk_acmpca::operation::list_certificate_authorities::{builders::*, *};
use aws_sdk_acmpca::operation::list_permissions::{builders::*, *};
use aws_sdk_acmpca::operation::list_tags::{builders::*, *};
use aws_sdk_acmpca::operation::put_policy::{builders::*, *};
use aws_sdk_acmpca::operation::restore_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::revoke_certificate::{builders::*, *};
use aws_sdk_acmpca::operation::tag_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::untag_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::operation::update_certificate_authority::{builders::*, *};
use aws_sdk_acmpca::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_acmpca::Client;

pub use aws_sdk_acmpca::*;

pub struct AcmPcaClientImpl(Client);
impl AcmPcaClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AcmPcaClient {
    fn create_certificate_authority(&self, builder: CreateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityOutput, SdkError<CreateCertificateAuthorityError>>>;
    fn create_certificate_authority_audit_report(&self, builder: CreateCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityAuditReportOutput, SdkError<CreateCertificateAuthorityAuditReportError>>>;
    fn create_permission(&self, builder: CreatePermissionInputBuilder) -> impl Future<Output = Result<CreatePermissionOutput, SdkError<CreatePermissionError>>>;
    fn delete_certificate_authority(&self, builder: DeleteCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DeleteCertificateAuthorityOutput, SdkError<DeleteCertificateAuthorityError>>>;
    fn delete_permission(&self, builder: DeletePermissionInputBuilder) -> impl Future<Output = Result<DeletePermissionOutput, SdkError<DeletePermissionError>>>;
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>>;
    fn describe_certificate_authority(&self, builder: DescribeCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityOutput, SdkError<DescribeCertificateAuthorityError>>>;
    fn describe_certificate_authority_audit_report(&self, builder: DescribeCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityAuditReportOutput, SdkError<DescribeCertificateAuthorityAuditReportError>>>;
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>>;
    fn get_certificate_authority_certificate(&self, builder: GetCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCertificateOutput, SdkError<GetCertificateAuthorityCertificateError>>>;
    fn get_certificate_authority_csr(&self, builder: GetCertificateAuthorityCsrInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCsrOutput, SdkError<GetCertificateAuthorityCsrError>>>;
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>>;
    fn import_certificate_authority_certificate(&self, builder: ImportCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateAuthorityCertificateOutput, SdkError<ImportCertificateAuthorityCertificateError>>>;
    fn issue_certificate(&self, builder: IssueCertificateInputBuilder) -> impl Future<Output = Result<IssueCertificateOutput, SdkError<IssueCertificateError>>>;
    fn list_certificate_authorities(&self, builder: ListCertificateAuthoritiesInputBuilder) -> impl Future<Output = Result<ListCertificateAuthoritiesOutput, SdkError<ListCertificateAuthoritiesError>>>;
    fn list_permissions(&self, builder: ListPermissionsInputBuilder) -> impl Future<Output = Result<ListPermissionsOutput, SdkError<ListPermissionsError>>>;
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>>;
    fn put_policy(&self, builder: PutPolicyInputBuilder) -> impl Future<Output = Result<PutPolicyOutput, SdkError<PutPolicyError>>>;
    fn restore_certificate_authority(&self, builder: RestoreCertificateAuthorityInputBuilder) -> impl Future<Output = Result<RestoreCertificateAuthorityOutput, SdkError<RestoreCertificateAuthorityError>>>;
    fn revoke_certificate(&self, builder: RevokeCertificateInputBuilder) -> impl Future<Output = Result<RevokeCertificateOutput, SdkError<RevokeCertificateError>>>;
    fn tag_certificate_authority(&self, builder: TagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<TagCertificateAuthorityOutput, SdkError<TagCertificateAuthorityError>>>;
    fn untag_certificate_authority(&self, builder: UntagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UntagCertificateAuthorityOutput, SdkError<UntagCertificateAuthorityError>>>;
    fn update_certificate_authority(&self, builder: UpdateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UpdateCertificateAuthorityOutput, SdkError<UpdateCertificateAuthorityError>>>;
}
impl AcmPcaClient for AcmPcaClientImpl {
    fn create_certificate_authority(&self, builder: CreateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityOutput, SdkError<CreateCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn create_certificate_authority_audit_report(&self, builder: CreateCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityAuditReportOutput, SdkError<CreateCertificateAuthorityAuditReportError>>> {
        builder.send_with(&self.0)
    }
    fn create_permission(&self, builder: CreatePermissionInputBuilder) -> impl Future<Output = Result<CreatePermissionOutput, SdkError<CreatePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_certificate_authority(&self, builder: DeleteCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DeleteCertificateAuthorityOutput, SdkError<DeleteCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn delete_permission(&self, builder: DeletePermissionInputBuilder) -> impl Future<Output = Result<DeletePermissionOutput, SdkError<DeletePermissionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn describe_certificate_authority(&self, builder: DescribeCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityOutput, SdkError<DescribeCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn describe_certificate_authority_audit_report(&self, builder: DescribeCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityAuditReportOutput, SdkError<DescribeCertificateAuthorityAuditReportError>>> {
        builder.send_with(&self.0)
    }
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_certificate_authority_certificate(&self, builder: GetCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCertificateOutput, SdkError<GetCertificateAuthorityCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_certificate_authority_csr(&self, builder: GetCertificateAuthorityCsrInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCsrOutput, SdkError<GetCertificateAuthorityCsrError>>> {
        builder.send_with(&self.0)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn import_certificate_authority_certificate(&self, builder: ImportCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateAuthorityCertificateOutput, SdkError<ImportCertificateAuthorityCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn issue_certificate(&self, builder: IssueCertificateInputBuilder) -> impl Future<Output = Result<IssueCertificateOutput, SdkError<IssueCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn list_certificate_authorities(&self, builder: ListCertificateAuthoritiesInputBuilder) -> impl Future<Output = Result<ListCertificateAuthoritiesOutput, SdkError<ListCertificateAuthoritiesError>>> {
        builder.send_with(&self.0)
    }
    fn list_permissions(&self, builder: ListPermissionsInputBuilder) -> impl Future<Output = Result<ListPermissionsOutput, SdkError<ListPermissionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        builder.send_with(&self.0)
    }
    fn put_policy(&self, builder: PutPolicyInputBuilder) -> impl Future<Output = Result<PutPolicyOutput, SdkError<PutPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn restore_certificate_authority(&self, builder: RestoreCertificateAuthorityInputBuilder) -> impl Future<Output = Result<RestoreCertificateAuthorityOutput, SdkError<RestoreCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_certificate(&self, builder: RevokeCertificateInputBuilder) -> impl Future<Output = Result<RevokeCertificateOutput, SdkError<RevokeCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn tag_certificate_authority(&self, builder: TagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<TagCertificateAuthorityOutput, SdkError<TagCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn untag_certificate_authority(&self, builder: UntagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UntagCertificateAuthorityOutput, SdkError<UntagCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
    fn update_certificate_authority(&self, builder: UpdateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UpdateCertificateAuthorityOutput, SdkError<UpdateCertificateAuthorityError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: AcmPcaClient> AcmPcaClient for &T {
    fn create_certificate_authority(&self, builder: CreateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityOutput, SdkError<CreateCertificateAuthorityError>>> {
        (*self).create_certificate_authority(builder)
    }
    fn create_certificate_authority_audit_report(&self, builder: CreateCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<CreateCertificateAuthorityAuditReportOutput, SdkError<CreateCertificateAuthorityAuditReportError>>> {
        (*self).create_certificate_authority_audit_report(builder)
    }
    fn create_permission(&self, builder: CreatePermissionInputBuilder) -> impl Future<Output = Result<CreatePermissionOutput, SdkError<CreatePermissionError>>> {
        (*self).create_permission(builder)
    }
    fn delete_certificate_authority(&self, builder: DeleteCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DeleteCertificateAuthorityOutput, SdkError<DeleteCertificateAuthorityError>>> {
        (*self).delete_certificate_authority(builder)
    }
    fn delete_permission(&self, builder: DeletePermissionInputBuilder) -> impl Future<Output = Result<DeletePermissionOutput, SdkError<DeletePermissionError>>> {
        (*self).delete_permission(builder)
    }
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>> {
        (*self).delete_policy(builder)
    }
    fn describe_certificate_authority(&self, builder: DescribeCertificateAuthorityInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityOutput, SdkError<DescribeCertificateAuthorityError>>> {
        (*self).describe_certificate_authority(builder)
    }
    fn describe_certificate_authority_audit_report(&self, builder: DescribeCertificateAuthorityAuditReportInputBuilder) -> impl Future<Output = Result<DescribeCertificateAuthorityAuditReportOutput, SdkError<DescribeCertificateAuthorityAuditReportError>>> {
        (*self).describe_certificate_authority_audit_report(builder)
    }
    fn get_certificate(&self, builder: GetCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateOutput, SdkError<GetCertificateError>>> {
        (*self).get_certificate(builder)
    }
    fn get_certificate_authority_certificate(&self, builder: GetCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCertificateOutput, SdkError<GetCertificateAuthorityCertificateError>>> {
        (*self).get_certificate_authority_certificate(builder)
    }
    fn get_certificate_authority_csr(&self, builder: GetCertificateAuthorityCsrInputBuilder) -> impl Future<Output = Result<GetCertificateAuthorityCsrOutput, SdkError<GetCertificateAuthorityCsrError>>> {
        (*self).get_certificate_authority_csr(builder)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        (*self).get_policy(builder)
    }
    fn import_certificate_authority_certificate(&self, builder: ImportCertificateAuthorityCertificateInputBuilder) -> impl Future<Output = Result<ImportCertificateAuthorityCertificateOutput, SdkError<ImportCertificateAuthorityCertificateError>>> {
        (*self).import_certificate_authority_certificate(builder)
    }
    fn issue_certificate(&self, builder: IssueCertificateInputBuilder) -> impl Future<Output = Result<IssueCertificateOutput, SdkError<IssueCertificateError>>> {
        (*self).issue_certificate(builder)
    }
    fn list_certificate_authorities(&self, builder: ListCertificateAuthoritiesInputBuilder) -> impl Future<Output = Result<ListCertificateAuthoritiesOutput, SdkError<ListCertificateAuthoritiesError>>> {
        (*self).list_certificate_authorities(builder)
    }
    fn list_permissions(&self, builder: ListPermissionsInputBuilder) -> impl Future<Output = Result<ListPermissionsOutput, SdkError<ListPermissionsError>>> {
        (*self).list_permissions(builder)
    }
    fn list_tags(&self, builder: ListTagsInputBuilder) -> impl Future<Output = Result<ListTagsOutput, SdkError<ListTagsError>>> {
        (*self).list_tags(builder)
    }
    fn put_policy(&self, builder: PutPolicyInputBuilder) -> impl Future<Output = Result<PutPolicyOutput, SdkError<PutPolicyError>>> {
        (*self).put_policy(builder)
    }
    fn restore_certificate_authority(&self, builder: RestoreCertificateAuthorityInputBuilder) -> impl Future<Output = Result<RestoreCertificateAuthorityOutput, SdkError<RestoreCertificateAuthorityError>>> {
        (*self).restore_certificate_authority(builder)
    }
    fn revoke_certificate(&self, builder: RevokeCertificateInputBuilder) -> impl Future<Output = Result<RevokeCertificateOutput, SdkError<RevokeCertificateError>>> {
        (*self).revoke_certificate(builder)
    }
    fn tag_certificate_authority(&self, builder: TagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<TagCertificateAuthorityOutput, SdkError<TagCertificateAuthorityError>>> {
        (*self).tag_certificate_authority(builder)
    }
    fn untag_certificate_authority(&self, builder: UntagCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UntagCertificateAuthorityOutput, SdkError<UntagCertificateAuthorityError>>> {
        (*self).untag_certificate_authority(builder)
    }
    fn update_certificate_authority(&self, builder: UpdateCertificateAuthorityInputBuilder) -> impl Future<Output = Result<UpdateCertificateAuthorityOutput, SdkError<UpdateCertificateAuthorityError>>> {
        (*self).update_certificate_authority(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAcmPcaClient {}
    impl AcmPcaClient for edAcmPcaClient {
        async fn create_certificate_authority(&self, builder: CreateCertificateAuthorityInputBuilder) -> Result<CreateCertificateAuthorityOutput, SdkError<CreateCertificateAuthorityError>>;
        async fn create_certificate_authority_audit_report(&self, builder: CreateCertificateAuthorityAuditReportInputBuilder) -> Result<CreateCertificateAuthorityAuditReportOutput, SdkError<CreateCertificateAuthorityAuditReportError>>;
        async fn create_permission(&self, builder: CreatePermissionInputBuilder) -> Result<CreatePermissionOutput, SdkError<CreatePermissionError>>;
        async fn delete_certificate_authority(&self, builder: DeleteCertificateAuthorityInputBuilder) -> Result<DeleteCertificateAuthorityOutput, SdkError<DeleteCertificateAuthorityError>>;
        async fn delete_permission(&self, builder: DeletePermissionInputBuilder) -> Result<DeletePermissionOutput, SdkError<DeletePermissionError>>;
        async fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> Result<DeletePolicyOutput, SdkError<DeletePolicyError>>;
        async fn describe_certificate_authority(&self, builder: DescribeCertificateAuthorityInputBuilder) -> Result<DescribeCertificateAuthorityOutput, SdkError<DescribeCertificateAuthorityError>>;
        async fn describe_certificate_authority_audit_report(&self, builder: DescribeCertificateAuthorityAuditReportInputBuilder) -> Result<DescribeCertificateAuthorityAuditReportOutput, SdkError<DescribeCertificateAuthorityAuditReportError>>;
        async fn get_certificate(&self, builder: GetCertificateInputBuilder) -> Result<GetCertificateOutput, SdkError<GetCertificateError>>;
        async fn get_certificate_authority_certificate(&self, builder: GetCertificateAuthorityCertificateInputBuilder) -> Result<GetCertificateAuthorityCertificateOutput, SdkError<GetCertificateAuthorityCertificateError>>;
        async fn get_certificate_authority_csr(&self, builder: GetCertificateAuthorityCsrInputBuilder) -> Result<GetCertificateAuthorityCsrOutput, SdkError<GetCertificateAuthorityCsrError>>;
        async fn get_policy(&self, builder: GetPolicyInputBuilder) -> Result<GetPolicyOutput, SdkError<GetPolicyError>>;
        async fn import_certificate_authority_certificate(&self, builder: ImportCertificateAuthorityCertificateInputBuilder) -> Result<ImportCertificateAuthorityCertificateOutput, SdkError<ImportCertificateAuthorityCertificateError>>;
        async fn issue_certificate(&self, builder: IssueCertificateInputBuilder) -> Result<IssueCertificateOutput, SdkError<IssueCertificateError>>;
        async fn list_certificate_authorities(&self, builder: ListCertificateAuthoritiesInputBuilder) -> Result<ListCertificateAuthoritiesOutput, SdkError<ListCertificateAuthoritiesError>>;
        async fn list_permissions(&self, builder: ListPermissionsInputBuilder) -> Result<ListPermissionsOutput, SdkError<ListPermissionsError>>;
        async fn list_tags(&self, builder: ListTagsInputBuilder) -> Result<ListTagsOutput, SdkError<ListTagsError>>;
        async fn put_policy(&self, builder: PutPolicyInputBuilder) -> Result<PutPolicyOutput, SdkError<PutPolicyError>>;
        async fn restore_certificate_authority(&self, builder: RestoreCertificateAuthorityInputBuilder) -> Result<RestoreCertificateAuthorityOutput, SdkError<RestoreCertificateAuthorityError>>;
        async fn revoke_certificate(&self, builder: RevokeCertificateInputBuilder) -> Result<RevokeCertificateOutput, SdkError<RevokeCertificateError>>;
        async fn tag_certificate_authority(&self, builder: TagCertificateAuthorityInputBuilder) -> Result<TagCertificateAuthorityOutput, SdkError<TagCertificateAuthorityError>>;
        async fn untag_certificate_authority(&self, builder: UntagCertificateAuthorityInputBuilder) -> Result<UntagCertificateAuthorityOutput, SdkError<UntagCertificateAuthorityError>>;
        async fn update_certificate_authority(&self, builder: UpdateCertificateAuthorityInputBuilder) -> Result<UpdateCertificateAuthorityOutput, SdkError<UpdateCertificateAuthorityError>>;
    }
}
