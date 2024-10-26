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
use aws_sdk_iam::operation::add_client_id_to_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::add_role_to_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::add_user_to_group::{builders::*, *};
use aws_sdk_iam::operation::attach_group_policy::{builders::*, *};
use aws_sdk_iam::operation::attach_role_policy::{builders::*, *};
use aws_sdk_iam::operation::attach_user_policy::{builders::*, *};
use aws_sdk_iam::operation::change_password::{builders::*, *};
use aws_sdk_iam::operation::create_access_key::{builders::*, *};
use aws_sdk_iam::operation::create_account_alias::{builders::*, *};
use aws_sdk_iam::operation::create_group::{builders::*, *};
use aws_sdk_iam::operation::create_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::create_login_profile::{builders::*, *};
use aws_sdk_iam::operation::create_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::create_policy::{builders::*, *};
use aws_sdk_iam::operation::create_policy_version::{builders::*, *};
use aws_sdk_iam::operation::create_role::{builders::*, *};
use aws_sdk_iam::operation::create_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::create_service_linked_role::{builders::*, *};
use aws_sdk_iam::operation::create_service_specific_credential::{builders::*, *};
use aws_sdk_iam::operation::create_user::{builders::*, *};
use aws_sdk_iam::operation::create_virtual_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::deactivate_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::delete_access_key::{builders::*, *};
use aws_sdk_iam::operation::delete_account_alias::{builders::*, *};
use aws_sdk_iam::operation::delete_account_password_policy::{builders::*, *};
use aws_sdk_iam::operation::delete_group::{builders::*, *};
use aws_sdk_iam::operation::delete_group_policy::{builders::*, *};
use aws_sdk_iam::operation::delete_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::delete_login_profile::{builders::*, *};
use aws_sdk_iam::operation::delete_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::delete_policy::{builders::*, *};
use aws_sdk_iam::operation::delete_policy_version::{builders::*, *};
use aws_sdk_iam::operation::delete_role::{builders::*, *};
use aws_sdk_iam::operation::delete_role_permissions_boundary::{builders::*, *};
use aws_sdk_iam::operation::delete_role_policy::{builders::*, *};
use aws_sdk_iam::operation::delete_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::delete_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::delete_service_linked_role::{builders::*, *};
use aws_sdk_iam::operation::delete_service_specific_credential::{builders::*, *};
use aws_sdk_iam::operation::delete_signing_certificate::{builders::*, *};
use aws_sdk_iam::operation::delete_ssh_public_key::{builders::*, *};
use aws_sdk_iam::operation::delete_user::{builders::*, *};
use aws_sdk_iam::operation::delete_user_permissions_boundary::{builders::*, *};
use aws_sdk_iam::operation::delete_user_policy::{builders::*, *};
use aws_sdk_iam::operation::delete_virtual_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::detach_group_policy::{builders::*, *};
use aws_sdk_iam::operation::detach_role_policy::{builders::*, *};
use aws_sdk_iam::operation::detach_user_policy::{builders::*, *};
use aws_sdk_iam::operation::enable_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::generate_credential_report::{builders::*, *};
use aws_sdk_iam::operation::generate_organizations_access_report::{builders::*, *};
use aws_sdk_iam::operation::generate_service_last_accessed_details::{builders::*, *};
use aws_sdk_iam::operation::get_access_key_last_used::{builders::*, *};
use aws_sdk_iam::operation::get_account_authorization_details::{builders::*, *};
use aws_sdk_iam::operation::get_account_password_policy::{builders::*, *};
use aws_sdk_iam::operation::get_account_summary::{builders::*, *};
use aws_sdk_iam::operation::get_context_keys_for_custom_policy::{builders::*, *};
use aws_sdk_iam::operation::get_context_keys_for_principal_policy::{builders::*, *};
use aws_sdk_iam::operation::get_credential_report::{builders::*, *};
use aws_sdk_iam::operation::get_group::{builders::*, *};
use aws_sdk_iam::operation::get_group_policy::{builders::*, *};
use aws_sdk_iam::operation::get_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::get_login_profile::{builders::*, *};
use aws_sdk_iam::operation::get_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::get_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::get_organizations_access_report::{builders::*, *};
use aws_sdk_iam::operation::get_policy::{builders::*, *};
use aws_sdk_iam::operation::get_policy_version::{builders::*, *};
use aws_sdk_iam::operation::get_role::{builders::*, *};
use aws_sdk_iam::operation::get_role_policy::{builders::*, *};
use aws_sdk_iam::operation::get_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::get_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::get_service_last_accessed_details::{builders::*, *};
use aws_sdk_iam::operation::get_service_last_accessed_details_with_entities::{builders::*, *};
use aws_sdk_iam::operation::get_service_linked_role_deletion_status::{builders::*, *};
use aws_sdk_iam::operation::get_ssh_public_key::{builders::*, *};
use aws_sdk_iam::operation::get_user::{builders::*, *};
use aws_sdk_iam::operation::get_user_policy::{builders::*, *};
use aws_sdk_iam::operation::list_access_keys::{builders::*, *};
use aws_sdk_iam::operation::list_account_aliases::{builders::*, *};
use aws_sdk_iam::operation::list_attached_group_policies::{builders::*, *};
use aws_sdk_iam::operation::list_attached_role_policies::{builders::*, *};
use aws_sdk_iam::operation::list_attached_user_policies::{builders::*, *};
use aws_sdk_iam::operation::list_entities_for_policy::{builders::*, *};
use aws_sdk_iam::operation::list_group_policies::{builders::*, *};
use aws_sdk_iam::operation::list_groups::{builders::*, *};
use aws_sdk_iam::operation::list_groups_for_user::{builders::*, *};
use aws_sdk_iam::operation::list_instance_profile_tags::{builders::*, *};
use aws_sdk_iam::operation::list_instance_profiles::{builders::*, *};
use aws_sdk_iam::operation::list_instance_profiles_for_role::{builders::*, *};
use aws_sdk_iam::operation::list_mfa_device_tags::{builders::*, *};
use aws_sdk_iam::operation::list_mfa_devices::{builders::*, *};
use aws_sdk_iam::operation::list_open_id_connect_provider_tags::{builders::*, *};
use aws_sdk_iam::operation::list_open_id_connect_providers::{builders::*, *};
use aws_sdk_iam::operation::list_policies::{builders::*, *};
use aws_sdk_iam::operation::list_policies_granting_service_access::{builders::*, *};
use aws_sdk_iam::operation::list_policy_tags::{builders::*, *};
use aws_sdk_iam::operation::list_policy_versions::{builders::*, *};
use aws_sdk_iam::operation::list_role_policies::{builders::*, *};
use aws_sdk_iam::operation::list_role_tags::{builders::*, *};
use aws_sdk_iam::operation::list_roles::{builders::*, *};
use aws_sdk_iam::operation::list_saml_provider_tags::{builders::*, *};
use aws_sdk_iam::operation::list_saml_providers::{builders::*, *};
use aws_sdk_iam::operation::list_server_certificate_tags::{builders::*, *};
use aws_sdk_iam::operation::list_server_certificates::{builders::*, *};
use aws_sdk_iam::operation::list_service_specific_credentials::{builders::*, *};
use aws_sdk_iam::operation::list_signing_certificates::{builders::*, *};
use aws_sdk_iam::operation::list_ssh_public_keys::{builders::*, *};
use aws_sdk_iam::operation::list_user_policies::{builders::*, *};
use aws_sdk_iam::operation::list_user_tags::{builders::*, *};
use aws_sdk_iam::operation::list_users::{builders::*, *};
use aws_sdk_iam::operation::list_virtual_mfa_devices::{builders::*, *};
use aws_sdk_iam::operation::put_group_policy::{builders::*, *};
use aws_sdk_iam::operation::put_role_permissions_boundary::{builders::*, *};
use aws_sdk_iam::operation::put_role_policy::{builders::*, *};
use aws_sdk_iam::operation::put_user_permissions_boundary::{builders::*, *};
use aws_sdk_iam::operation::put_user_policy::{builders::*, *};
use aws_sdk_iam::operation::remove_client_id_from_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::remove_role_from_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::remove_user_from_group::{builders::*, *};
use aws_sdk_iam::operation::reset_service_specific_credential::{builders::*, *};
use aws_sdk_iam::operation::resync_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::set_default_policy_version::{builders::*, *};
use aws_sdk_iam::operation::set_security_token_service_preferences::{builders::*, *};
use aws_sdk_iam::operation::simulate_custom_policy::{builders::*, *};
use aws_sdk_iam::operation::simulate_principal_policy::{builders::*, *};
use aws_sdk_iam::operation::tag_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::tag_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::tag_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::tag_policy::{builders::*, *};
use aws_sdk_iam::operation::tag_role::{builders::*, *};
use aws_sdk_iam::operation::tag_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::tag_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::tag_user::{builders::*, *};
use aws_sdk_iam::operation::untag_instance_profile::{builders::*, *};
use aws_sdk_iam::operation::untag_mfa_device::{builders::*, *};
use aws_sdk_iam::operation::untag_open_id_connect_provider::{builders::*, *};
use aws_sdk_iam::operation::untag_policy::{builders::*, *};
use aws_sdk_iam::operation::untag_role::{builders::*, *};
use aws_sdk_iam::operation::untag_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::untag_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::untag_user::{builders::*, *};
use aws_sdk_iam::operation::update_access_key::{builders::*, *};
use aws_sdk_iam::operation::update_account_password_policy::{builders::*, *};
use aws_sdk_iam::operation::update_assume_role_policy::{builders::*, *};
use aws_sdk_iam::operation::update_group::{builders::*, *};
use aws_sdk_iam::operation::update_login_profile::{builders::*, *};
use aws_sdk_iam::operation::update_open_id_connect_provider_thumbprint::{builders::*, *};
use aws_sdk_iam::operation::update_role::{builders::*, *};
use aws_sdk_iam::operation::update_role_description::{builders::*, *};
use aws_sdk_iam::operation::update_saml_provider::{builders::*, *};
use aws_sdk_iam::operation::update_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::update_service_specific_credential::{builders::*, *};
use aws_sdk_iam::operation::update_signing_certificate::{builders::*, *};
use aws_sdk_iam::operation::update_ssh_public_key::{builders::*, *};
use aws_sdk_iam::operation::update_user::{builders::*, *};
use aws_sdk_iam::operation::upload_server_certificate::{builders::*, *};
use aws_sdk_iam::operation::upload_signing_certificate::{builders::*, *};
use aws_sdk_iam::operation::upload_ssh_public_key::{builders::*, *};
use aws_sdk_iam::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_iam::Client;
use std::ops::Deref;

pub use aws_sdk_iam::*;

pub struct IAMClientImpl(Client);
impl IAMClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait IAMClient {
    fn add_client_id_to_open_id_connect_provider(&self, builder: AddClientIdToOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<AddClientIdToOpenIdConnectProviderOutput, SdkError<AddClientIDToOpenIDConnectProviderError>>> + Send;
    fn add_role_to_instance_profile(&self, builder: AddRoleToInstanceProfileInputBuilder) -> impl Future<Output = Result<AddRoleToInstanceProfileOutput, SdkError<AddRoleToInstanceProfileError>>> + Send;
    fn add_user_to_group(&self, builder: AddUserToGroupInputBuilder) -> impl Future<Output = Result<AddUserToGroupOutput, SdkError<AddUserToGroupError>>> + Send;
    fn attach_group_policy(&self, builder: AttachGroupPolicyInputBuilder) -> impl Future<Output = Result<AttachGroupPolicyOutput, SdkError<AttachGroupPolicyError>>> + Send;
    fn attach_role_policy(&self, builder: AttachRolePolicyInputBuilder) -> impl Future<Output = Result<AttachRolePolicyOutput, SdkError<AttachRolePolicyError>>> + Send;
    fn attach_user_policy(&self, builder: AttachUserPolicyInputBuilder) -> impl Future<Output = Result<AttachUserPolicyOutput, SdkError<AttachUserPolicyError>>> + Send;
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>> + Send;
    fn create_access_key(&self, builder: CreateAccessKeyInputBuilder) -> impl Future<Output = Result<CreateAccessKeyOutput, SdkError<CreateAccessKeyError>>> + Send;
    fn create_account_alias(&self, builder: CreateAccountAliasInputBuilder) -> impl Future<Output = Result<CreateAccountAliasOutput, SdkError<CreateAccountAliasError>>> + Send;
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> + Send;
    fn create_instance_profile(&self, builder: CreateInstanceProfileInputBuilder) -> impl Future<Output = Result<CreateInstanceProfileOutput, SdkError<CreateInstanceProfileError>>> + Send;
    fn create_login_profile(&self, builder: CreateLoginProfileInputBuilder) -> impl Future<Output = Result<CreateLoginProfileOutput, SdkError<CreateLoginProfileError>>> + Send;
    fn create_open_id_connect_provider(&self, builder: CreateOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<CreateOpenIdConnectProviderOutput, SdkError<CreateOpenIDConnectProviderError>>> + Send;
    fn create_policy(&self, builder: CreatePolicyInputBuilder) -> impl Future<Output = Result<CreatePolicyOutput, SdkError<CreatePolicyError>>> + Send;
    fn create_policy_version(&self, builder: CreatePolicyVersionInputBuilder) -> impl Future<Output = Result<CreatePolicyVersionOutput, SdkError<CreatePolicyVersionError>>> + Send;
    fn create_role(&self, builder: CreateRoleInputBuilder) -> impl Future<Output = Result<CreateRoleOutput, SdkError<CreateRoleError>>> + Send;
    fn create_saml_provider(&self, builder: CreateSamlProviderInputBuilder) -> impl Future<Output = Result<CreateSamlProviderOutput, SdkError<CreateSAMLProviderError>>> + Send;
    fn create_service_linked_role(&self, builder: CreateServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<CreateServiceLinkedRoleOutput, SdkError<CreateServiceLinkedRoleError>>> + Send;
    fn create_service_specific_credential(&self, builder: CreateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<CreateServiceSpecificCredentialOutput, SdkError<CreateServiceSpecificCredentialError>>> + Send;
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> + Send;
    fn create_virtual_mfa_device(&self, builder: CreateVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<CreateVirtualMfaDeviceOutput, SdkError<CreateVirtualMFADeviceError>>> + Send;
    fn deactivate_mfa_device(&self, builder: DeactivateMfaDeviceInputBuilder) -> impl Future<Output = Result<DeactivateMfaDeviceOutput, SdkError<DeactivateMFADeviceError>>> + Send;
    fn delete_access_key(&self, builder: DeleteAccessKeyInputBuilder) -> impl Future<Output = Result<DeleteAccessKeyOutput, SdkError<DeleteAccessKeyError>>> + Send;
    fn delete_account_alias(&self, builder: DeleteAccountAliasInputBuilder) -> impl Future<Output = Result<DeleteAccountAliasOutput, SdkError<DeleteAccountAliasError>>> + Send;
    fn delete_account_password_policy(&self, builder: DeleteAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<DeleteAccountPasswordPolicyOutput, SdkError<DeleteAccountPasswordPolicyError>>> + Send;
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> + Send;
    fn delete_group_policy(&self, builder: DeleteGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteGroupPolicyOutput, SdkError<DeleteGroupPolicyError>>> + Send;
    fn delete_instance_profile(&self, builder: DeleteInstanceProfileInputBuilder) -> impl Future<Output = Result<DeleteInstanceProfileOutput, SdkError<DeleteInstanceProfileError>>> + Send;
    fn delete_login_profile(&self, builder: DeleteLoginProfileInputBuilder) -> impl Future<Output = Result<DeleteLoginProfileOutput, SdkError<DeleteLoginProfileError>>> + Send;
    fn delete_open_id_connect_provider(&self, builder: DeleteOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<DeleteOpenIdConnectProviderOutput, SdkError<DeleteOpenIDConnectProviderError>>> + Send;
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>> + Send;
    fn delete_policy_version(&self, builder: DeletePolicyVersionInputBuilder) -> impl Future<Output = Result<DeletePolicyVersionOutput, SdkError<DeletePolicyVersionError>>> + Send;
    fn delete_role(&self, builder: DeleteRoleInputBuilder) -> impl Future<Output = Result<DeleteRoleOutput, SdkError<DeleteRoleError>>> + Send;
    fn delete_role_permissions_boundary(&self, builder: DeleteRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteRolePermissionsBoundaryOutput, SdkError<DeleteRolePermissionsBoundaryError>>> + Send;
    fn delete_role_policy(&self, builder: DeleteRolePolicyInputBuilder) -> impl Future<Output = Result<DeleteRolePolicyOutput, SdkError<DeleteRolePolicyError>>> + Send;
    fn delete_saml_provider(&self, builder: DeleteSamlProviderInputBuilder) -> impl Future<Output = Result<DeleteSamlProviderOutput, SdkError<DeleteSAMLProviderError>>> + Send;
    fn delete_server_certificate(&self, builder: DeleteServerCertificateInputBuilder) -> impl Future<Output = Result<DeleteServerCertificateOutput, SdkError<DeleteServerCertificateError>>> + Send;
    fn delete_service_linked_role(&self, builder: DeleteServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<DeleteServiceLinkedRoleOutput, SdkError<DeleteServiceLinkedRoleError>>> + Send;
    fn delete_service_specific_credential(&self, builder: DeleteServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<DeleteServiceSpecificCredentialOutput, SdkError<DeleteServiceSpecificCredentialError>>> + Send;
    fn delete_signing_certificate(&self, builder: DeleteSigningCertificateInputBuilder) -> impl Future<Output = Result<DeleteSigningCertificateOutput, SdkError<DeleteSigningCertificateError>>> + Send;
    fn delete_ssh_public_key(&self, builder: DeleteSshPublicKeyInputBuilder) -> impl Future<Output = Result<DeleteSshPublicKeyOutput, SdkError<DeleteSSHPublicKeyError>>> + Send;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> + Send;
    fn delete_user_permissions_boundary(&self, builder: DeleteUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteUserPermissionsBoundaryOutput, SdkError<DeleteUserPermissionsBoundaryError>>> + Send;
    fn delete_user_policy(&self, builder: DeleteUserPolicyInputBuilder) -> impl Future<Output = Result<DeleteUserPolicyOutput, SdkError<DeleteUserPolicyError>>> + Send;
    fn delete_virtual_mfa_device(&self, builder: DeleteVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<DeleteVirtualMfaDeviceOutput, SdkError<DeleteVirtualMFADeviceError>>> + Send;
    fn detach_group_policy(&self, builder: DetachGroupPolicyInputBuilder) -> impl Future<Output = Result<DetachGroupPolicyOutput, SdkError<DetachGroupPolicyError>>> + Send;
    fn detach_role_policy(&self, builder: DetachRolePolicyInputBuilder) -> impl Future<Output = Result<DetachRolePolicyOutput, SdkError<DetachRolePolicyError>>> + Send;
    fn detach_user_policy(&self, builder: DetachUserPolicyInputBuilder) -> impl Future<Output = Result<DetachUserPolicyOutput, SdkError<DetachUserPolicyError>>> + Send;
    fn enable_mfa_device(&self, builder: EnableMfaDeviceInputBuilder) -> impl Future<Output = Result<EnableMfaDeviceOutput, SdkError<EnableMFADeviceError>>> + Send;
    fn generate_credential_report(&self, builder: GenerateCredentialReportInputBuilder) -> impl Future<Output = Result<GenerateCredentialReportOutput, SdkError<GenerateCredentialReportError>>> + Send;
    fn generate_organizations_access_report(&self, builder: GenerateOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GenerateOrganizationsAccessReportOutput, SdkError<GenerateOrganizationsAccessReportError>>> + Send;
    fn generate_service_last_accessed_details(&self, builder: GenerateServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GenerateServiceLastAccessedDetailsOutput, SdkError<GenerateServiceLastAccessedDetailsError>>> + Send;
    fn get_access_key_last_used(&self, builder: GetAccessKeyLastUsedInputBuilder) -> impl Future<Output = Result<GetAccessKeyLastUsedOutput, SdkError<GetAccessKeyLastUsedError>>> + Send;
    fn get_account_authorization_details(&self, builder: GetAccountAuthorizationDetailsInputBuilder) -> impl Future<Output = Result<GetAccountAuthorizationDetailsOutput, SdkError<GetAccountAuthorizationDetailsError>>> + Send;
    fn get_account_password_policy(&self, builder: GetAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<GetAccountPasswordPolicyOutput, SdkError<GetAccountPasswordPolicyError>>> + Send;
    fn get_account_summary(&self, builder: GetAccountSummaryInputBuilder) -> impl Future<Output = Result<GetAccountSummaryOutput, SdkError<GetAccountSummaryError>>> + Send;
    fn get_context_keys_for_custom_policy(&self, builder: GetContextKeysForCustomPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForCustomPolicyOutput, SdkError<GetContextKeysForCustomPolicyError>>> + Send;
    fn get_context_keys_for_principal_policy(&self, builder: GetContextKeysForPrincipalPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForPrincipalPolicyOutput, SdkError<GetContextKeysForPrincipalPolicyError>>> + Send;
    fn get_credential_report(&self, builder: GetCredentialReportInputBuilder) -> impl Future<Output = Result<GetCredentialReportOutput, SdkError<GetCredentialReportError>>> + Send;
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> + Send;
    fn get_group_policy(&self, builder: GetGroupPolicyInputBuilder) -> impl Future<Output = Result<GetGroupPolicyOutput, SdkError<GetGroupPolicyError>>> + Send;
    fn get_instance_profile(&self, builder: GetInstanceProfileInputBuilder) -> impl Future<Output = Result<GetInstanceProfileOutput, SdkError<GetInstanceProfileError>>> + Send;
    fn get_login_profile(&self, builder: GetLoginProfileInputBuilder) -> impl Future<Output = Result<GetLoginProfileOutput, SdkError<GetLoginProfileError>>> + Send;
    fn get_mfa_device(&self, builder: GetMfaDeviceInputBuilder) -> impl Future<Output = Result<GetMfaDeviceOutput, SdkError<GetMFADeviceError>>> + Send;
    fn get_open_id_connect_provider(&self, builder: GetOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<GetOpenIdConnectProviderOutput, SdkError<GetOpenIDConnectProviderError>>> + Send;
    fn get_organizations_access_report(&self, builder: GetOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GetOrganizationsAccessReportOutput, SdkError<GetOrganizationsAccessReportError>>> + Send;
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> + Send;
    fn get_policy_version(&self, builder: GetPolicyVersionInputBuilder) -> impl Future<Output = Result<GetPolicyVersionOutput, SdkError<GetPolicyVersionError>>> + Send;
    fn get_role(&self, builder: GetRoleInputBuilder) -> impl Future<Output = Result<GetRoleOutput, SdkError<GetRoleError>>> + Send;
    fn get_role_policy(&self, builder: GetRolePolicyInputBuilder) -> impl Future<Output = Result<GetRolePolicyOutput, SdkError<GetRolePolicyError>>> + Send;
    fn get_saml_provider(&self, builder: GetSamlProviderInputBuilder) -> impl Future<Output = Result<GetSamlProviderOutput, SdkError<GetSAMLProviderError>>> + Send;
    fn get_server_certificate(&self, builder: GetServerCertificateInputBuilder) -> impl Future<Output = Result<GetServerCertificateOutput, SdkError<GetServerCertificateError>>> + Send;
    fn get_service_last_accessed_details(&self, builder: GetServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsOutput, SdkError<GetServiceLastAccessedDetailsError>>> + Send;
    fn get_service_last_accessed_details_with_entities(&self, builder: GetServiceLastAccessedDetailsWithEntitiesInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsWithEntitiesOutput, SdkError<GetServiceLastAccessedDetailsWithEntitiesError>>> + Send;
    fn get_service_linked_role_deletion_status(&self, builder: GetServiceLinkedRoleDeletionStatusInputBuilder) -> impl Future<Output = Result<GetServiceLinkedRoleDeletionStatusOutput, SdkError<GetServiceLinkedRoleDeletionStatusError>>> + Send;
    fn get_ssh_public_key(&self, builder: GetSshPublicKeyInputBuilder) -> impl Future<Output = Result<GetSshPublicKeyOutput, SdkError<GetSSHPublicKeyError>>> + Send;
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>> + Send;
    fn get_user_policy(&self, builder: GetUserPolicyInputBuilder) -> impl Future<Output = Result<GetUserPolicyOutput, SdkError<GetUserPolicyError>>> + Send;
    fn list_access_keys(&self, builder: ListAccessKeysInputBuilder) -> impl Future<Output = Result<ListAccessKeysOutput, SdkError<ListAccessKeysError>>> + Send;
    fn list_account_aliases(&self, builder: ListAccountAliasesInputBuilder) -> impl Future<Output = Result<ListAccountAliasesOutput, SdkError<ListAccountAliasesError>>> + Send;
    fn list_attached_group_policies(&self, builder: ListAttachedGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedGroupPoliciesOutput, SdkError<ListAttachedGroupPoliciesError>>> + Send;
    fn list_attached_role_policies(&self, builder: ListAttachedRolePoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedRolePoliciesOutput, SdkError<ListAttachedRolePoliciesError>>> + Send;
    fn list_attached_user_policies(&self, builder: ListAttachedUserPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedUserPoliciesOutput, SdkError<ListAttachedUserPoliciesError>>> + Send;
    fn list_entities_for_policy(&self, builder: ListEntitiesForPolicyInputBuilder) -> impl Future<Output = Result<ListEntitiesForPolicyOutput, SdkError<ListEntitiesForPolicyError>>> + Send;
    fn list_group_policies(&self, builder: ListGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListGroupPoliciesOutput, SdkError<ListGroupPoliciesError>>> + Send;
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> + Send;
    fn list_groups_for_user(&self, builder: ListGroupsForUserInputBuilder) -> impl Future<Output = Result<ListGroupsForUserOutput, SdkError<ListGroupsForUserError>>> + Send;
    fn list_instance_profile_tags(&self, builder: ListInstanceProfileTagsInputBuilder) -> impl Future<Output = Result<ListInstanceProfileTagsOutput, SdkError<ListInstanceProfileTagsError>>> + Send;
    fn list_instance_profiles(&self, builder: ListInstanceProfilesInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesOutput, SdkError<ListInstanceProfilesError>>> + Send;
    fn list_instance_profiles_for_role(&self, builder: ListInstanceProfilesForRoleInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesForRoleOutput, SdkError<ListInstanceProfilesForRoleError>>> + Send;
    fn list_mfa_device_tags(&self, builder: ListMfaDeviceTagsInputBuilder) -> impl Future<Output = Result<ListMfaDeviceTagsOutput, SdkError<ListMFADeviceTagsError>>> + Send;
    fn list_mfa_devices(&self, builder: ListMfaDevicesInputBuilder) -> impl Future<Output = Result<ListMfaDevicesOutput, SdkError<ListMFADevicesError>>> + Send;
    fn list_open_id_connect_provider_tags(&self, builder: ListOpenIdConnectProviderTagsInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProviderTagsOutput, SdkError<ListOpenIDConnectProviderTagsError>>> + Send;
    fn list_open_id_connect_providers(&self, builder: ListOpenIdConnectProvidersInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProvidersOutput, SdkError<ListOpenIDConnectProvidersError>>> + Send;
    fn list_policies(&self, builder: ListPoliciesInputBuilder) -> impl Future<Output = Result<ListPoliciesOutput, SdkError<ListPoliciesError>>> + Send;
    fn list_policies_granting_service_access(&self, builder: ListPoliciesGrantingServiceAccessInputBuilder) -> impl Future<Output = Result<ListPoliciesGrantingServiceAccessOutput, SdkError<ListPoliciesGrantingServiceAccessError>>> + Send;
    fn list_policy_tags(&self, builder: ListPolicyTagsInputBuilder) -> impl Future<Output = Result<ListPolicyTagsOutput, SdkError<ListPolicyTagsError>>> + Send;
    fn list_policy_versions(&self, builder: ListPolicyVersionsInputBuilder) -> impl Future<Output = Result<ListPolicyVersionsOutput, SdkError<ListPolicyVersionsError>>> + Send;
    fn list_role_policies(&self, builder: ListRolePoliciesInputBuilder) -> impl Future<Output = Result<ListRolePoliciesOutput, SdkError<ListRolePoliciesError>>> + Send;
    fn list_role_tags(&self, builder: ListRoleTagsInputBuilder) -> impl Future<Output = Result<ListRoleTagsOutput, SdkError<ListRoleTagsError>>> + Send;
    fn list_roles(&self, builder: ListRolesInputBuilder) -> impl Future<Output = Result<ListRolesOutput, SdkError<ListRolesError>>> + Send;
    fn list_saml_provider_tags(&self, builder: ListSamlProviderTagsInputBuilder) -> impl Future<Output = Result<ListSamlProviderTagsOutput, SdkError<ListSAMLProviderTagsError>>> + Send;
    fn list_saml_providers(&self, builder: ListSamlProvidersInputBuilder) -> impl Future<Output = Result<ListSamlProvidersOutput, SdkError<ListSAMLProvidersError>>> + Send;
    fn list_server_certificate_tags(&self, builder: ListServerCertificateTagsInputBuilder) -> impl Future<Output = Result<ListServerCertificateTagsOutput, SdkError<ListServerCertificateTagsError>>> + Send;
    fn list_server_certificates(&self, builder: ListServerCertificatesInputBuilder) -> impl Future<Output = Result<ListServerCertificatesOutput, SdkError<ListServerCertificatesError>>> + Send;
    fn list_service_specific_credentials(&self, builder: ListServiceSpecificCredentialsInputBuilder) -> impl Future<Output = Result<ListServiceSpecificCredentialsOutput, SdkError<ListServiceSpecificCredentialsError>>> + Send;
    fn list_signing_certificates(&self, builder: ListSigningCertificatesInputBuilder) -> impl Future<Output = Result<ListSigningCertificatesOutput, SdkError<ListSigningCertificatesError>>> + Send;
    fn list_ssh_public_keys(&self, builder: ListSshPublicKeysInputBuilder) -> impl Future<Output = Result<ListSshPublicKeysOutput, SdkError<ListSSHPublicKeysError>>> + Send;
    fn list_user_policies(&self, builder: ListUserPoliciesInputBuilder) -> impl Future<Output = Result<ListUserPoliciesOutput, SdkError<ListUserPoliciesError>>> + Send;
    fn list_user_tags(&self, builder: ListUserTagsInputBuilder) -> impl Future<Output = Result<ListUserTagsOutput, SdkError<ListUserTagsError>>> + Send;
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> + Send;
    fn list_virtual_mfa_devices(&self, builder: ListVirtualMfaDevicesInputBuilder) -> impl Future<Output = Result<ListVirtualMfaDevicesOutput, SdkError<ListVirtualMFADevicesError>>> + Send;
    fn put_group_policy(&self, builder: PutGroupPolicyInputBuilder) -> impl Future<Output = Result<PutGroupPolicyOutput, SdkError<PutGroupPolicyError>>> + Send;
    fn put_role_permissions_boundary(&self, builder: PutRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutRolePermissionsBoundaryOutput, SdkError<PutRolePermissionsBoundaryError>>> + Send;
    fn put_role_policy(&self, builder: PutRolePolicyInputBuilder) -> impl Future<Output = Result<PutRolePolicyOutput, SdkError<PutRolePolicyError>>> + Send;
    fn put_user_permissions_boundary(&self, builder: PutUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutUserPermissionsBoundaryOutput, SdkError<PutUserPermissionsBoundaryError>>> + Send;
    fn put_user_policy(&self, builder: PutUserPolicyInputBuilder) -> impl Future<Output = Result<PutUserPolicyOutput, SdkError<PutUserPolicyError>>> + Send;
    fn remove_client_id_from_open_id_connect_provider(&self, builder: RemoveClientIdFromOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<RemoveClientIdFromOpenIdConnectProviderOutput, SdkError<RemoveClientIDFromOpenIDConnectProviderError>>> + Send;
    fn remove_role_from_instance_profile(&self, builder: RemoveRoleFromInstanceProfileInputBuilder) -> impl Future<Output = Result<RemoveRoleFromInstanceProfileOutput, SdkError<RemoveRoleFromInstanceProfileError>>> + Send;
    fn remove_user_from_group(&self, builder: RemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<RemoveUserFromGroupOutput, SdkError<RemoveUserFromGroupError>>> + Send;
    fn reset_service_specific_credential(&self, builder: ResetServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<ResetServiceSpecificCredentialOutput, SdkError<ResetServiceSpecificCredentialError>>> + Send;
    fn resync_mfa_device(&self, builder: ResyncMfaDeviceInputBuilder) -> impl Future<Output = Result<ResyncMfaDeviceOutput, SdkError<ResyncMFADeviceError>>> + Send;
    fn set_default_policy_version(&self, builder: SetDefaultPolicyVersionInputBuilder) -> impl Future<Output = Result<SetDefaultPolicyVersionOutput, SdkError<SetDefaultPolicyVersionError>>> + Send;
    fn set_security_token_service_preferences(&self, builder: SetSecurityTokenServicePreferencesInputBuilder) -> impl Future<Output = Result<SetSecurityTokenServicePreferencesOutput, SdkError<SetSecurityTokenServicePreferencesError>>> + Send;
    fn simulate_custom_policy(&self, builder: SimulateCustomPolicyInputBuilder) -> impl Future<Output = Result<SimulateCustomPolicyOutput, SdkError<SimulateCustomPolicyError>>> + Send;
    fn simulate_principal_policy(&self, builder: SimulatePrincipalPolicyInputBuilder) -> impl Future<Output = Result<SimulatePrincipalPolicyOutput, SdkError<SimulatePrincipalPolicyError>>> + Send;
    fn tag_instance_profile(&self, builder: TagInstanceProfileInputBuilder) -> impl Future<Output = Result<TagInstanceProfileOutput, SdkError<TagInstanceProfileError>>> + Send;
    fn tag_mfa_device(&self, builder: TagMfaDeviceInputBuilder) -> impl Future<Output = Result<TagMfaDeviceOutput, SdkError<TagMFADeviceError>>> + Send;
    fn tag_open_id_connect_provider(&self, builder: TagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<TagOpenIdConnectProviderOutput, SdkError<TagOpenIDConnectProviderError>>> + Send;
    fn tag_policy(&self, builder: TagPolicyInputBuilder) -> impl Future<Output = Result<TagPolicyOutput, SdkError<TagPolicyError>>> + Send;
    fn tag_role(&self, builder: TagRoleInputBuilder) -> impl Future<Output = Result<TagRoleOutput, SdkError<TagRoleError>>> + Send;
    fn tag_saml_provider(&self, builder: TagSamlProviderInputBuilder) -> impl Future<Output = Result<TagSamlProviderOutput, SdkError<TagSAMLProviderError>>> + Send;
    fn tag_server_certificate(&self, builder: TagServerCertificateInputBuilder) -> impl Future<Output = Result<TagServerCertificateOutput, SdkError<TagServerCertificateError>>> + Send;
    fn tag_user(&self, builder: TagUserInputBuilder) -> impl Future<Output = Result<TagUserOutput, SdkError<TagUserError>>> + Send;
    fn untag_instance_profile(&self, builder: UntagInstanceProfileInputBuilder) -> impl Future<Output = Result<UntagInstanceProfileOutput, SdkError<UntagInstanceProfileError>>> + Send;
    fn untag_mfa_device(&self, builder: UntagMfaDeviceInputBuilder) -> impl Future<Output = Result<UntagMfaDeviceOutput, SdkError<UntagMFADeviceError>>> + Send;
    fn untag_open_id_connect_provider(&self, builder: UntagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<UntagOpenIdConnectProviderOutput, SdkError<UntagOpenIDConnectProviderError>>> + Send;
    fn untag_policy(&self, builder: UntagPolicyInputBuilder) -> impl Future<Output = Result<UntagPolicyOutput, SdkError<UntagPolicyError>>> + Send;
    fn untag_role(&self, builder: UntagRoleInputBuilder) -> impl Future<Output = Result<UntagRoleOutput, SdkError<UntagRoleError>>> + Send;
    fn untag_saml_provider(&self, builder: UntagSamlProviderInputBuilder) -> impl Future<Output = Result<UntagSamlProviderOutput, SdkError<UntagSAMLProviderError>>> + Send;
    fn untag_server_certificate(&self, builder: UntagServerCertificateInputBuilder) -> impl Future<Output = Result<UntagServerCertificateOutput, SdkError<UntagServerCertificateError>>> + Send;
    fn untag_user(&self, builder: UntagUserInputBuilder) -> impl Future<Output = Result<UntagUserOutput, SdkError<UntagUserError>>> + Send;
    fn update_access_key(&self, builder: UpdateAccessKeyInputBuilder) -> impl Future<Output = Result<UpdateAccessKeyOutput, SdkError<UpdateAccessKeyError>>> + Send;
    fn update_account_password_policy(&self, builder: UpdateAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<UpdateAccountPasswordPolicyOutput, SdkError<UpdateAccountPasswordPolicyError>>> + Send;
    fn update_assume_role_policy(&self, builder: UpdateAssumeRolePolicyInputBuilder) -> impl Future<Output = Result<UpdateAssumeRolePolicyOutput, SdkError<UpdateAssumeRolePolicyError>>> + Send;
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> + Send;
    fn update_login_profile(&self, builder: UpdateLoginProfileInputBuilder) -> impl Future<Output = Result<UpdateLoginProfileOutput, SdkError<UpdateLoginProfileError>>> + Send;
    fn update_open_id_connect_provider_thumbprint(&self, builder: UpdateOpenIdConnectProviderThumbprintInputBuilder) -> impl Future<Output = Result<UpdateOpenIdConnectProviderThumbprintOutput, SdkError<UpdateOpenIDConnectProviderThumbprintError>>> + Send;
    fn update_role(&self, builder: UpdateRoleInputBuilder) -> impl Future<Output = Result<UpdateRoleOutput, SdkError<UpdateRoleError>>> + Send;
    fn update_role_description(&self, builder: UpdateRoleDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRoleDescriptionOutput, SdkError<UpdateRoleDescriptionError>>> + Send;
    fn update_saml_provider(&self, builder: UpdateSamlProviderInputBuilder) -> impl Future<Output = Result<UpdateSamlProviderOutput, SdkError<UpdateSAMLProviderError>>> + Send;
    fn update_server_certificate(&self, builder: UpdateServerCertificateInputBuilder) -> impl Future<Output = Result<UpdateServerCertificateOutput, SdkError<UpdateServerCertificateError>>> + Send;
    fn update_service_specific_credential(&self, builder: UpdateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<UpdateServiceSpecificCredentialOutput, SdkError<UpdateServiceSpecificCredentialError>>> + Send;
    fn update_signing_certificate(&self, builder: UpdateSigningCertificateInputBuilder) -> impl Future<Output = Result<UpdateSigningCertificateOutput, SdkError<UpdateSigningCertificateError>>> + Send;
    fn update_ssh_public_key(&self, builder: UpdateSshPublicKeyInputBuilder) -> impl Future<Output = Result<UpdateSshPublicKeyOutput, SdkError<UpdateSSHPublicKeyError>>> + Send;
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> + Send;
    fn upload_server_certificate(&self, builder: UploadServerCertificateInputBuilder) -> impl Future<Output = Result<UploadServerCertificateOutput, SdkError<UploadServerCertificateError>>> + Send;
    fn upload_signing_certificate(&self, builder: UploadSigningCertificateInputBuilder) -> impl Future<Output = Result<UploadSigningCertificateOutput, SdkError<UploadSigningCertificateError>>> + Send;
    fn upload_ssh_public_key(&self, builder: UploadSshPublicKeyInputBuilder) -> impl Future<Output = Result<UploadSshPublicKeyOutput, SdkError<UploadSSHPublicKeyError>>> + Send;
}
impl IAMClient for IAMClientImpl {
    fn add_client_id_to_open_id_connect_provider(&self, builder: AddClientIdToOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<AddClientIdToOpenIdConnectProviderOutput, SdkError<AddClientIDToOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn add_role_to_instance_profile(&self, builder: AddRoleToInstanceProfileInputBuilder) -> impl Future<Output = Result<AddRoleToInstanceProfileOutput, SdkError<AddRoleToInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn add_user_to_group(&self, builder: AddUserToGroupInputBuilder) -> impl Future<Output = Result<AddUserToGroupOutput, SdkError<AddUserToGroupError>>> {
        builder.send_with(&self.0)
    }
    fn attach_group_policy(&self, builder: AttachGroupPolicyInputBuilder) -> impl Future<Output = Result<AttachGroupPolicyOutput, SdkError<AttachGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn attach_role_policy(&self, builder: AttachRolePolicyInputBuilder) -> impl Future<Output = Result<AttachRolePolicyOutput, SdkError<AttachRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn attach_user_policy(&self, builder: AttachUserPolicyInputBuilder) -> impl Future<Output = Result<AttachUserPolicyOutput, SdkError<AttachUserPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>> {
        builder.send_with(&self.0)
    }
    fn create_access_key(&self, builder: CreateAccessKeyInputBuilder) -> impl Future<Output = Result<CreateAccessKeyOutput, SdkError<CreateAccessKeyError>>> {
        builder.send_with(&self.0)
    }
    fn create_account_alias(&self, builder: CreateAccountAliasInputBuilder) -> impl Future<Output = Result<CreateAccountAliasOutput, SdkError<CreateAccountAliasError>>> {
        builder.send_with(&self.0)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_instance_profile(&self, builder: CreateInstanceProfileInputBuilder) -> impl Future<Output = Result<CreateInstanceProfileOutput, SdkError<CreateInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_login_profile(&self, builder: CreateLoginProfileInputBuilder) -> impl Future<Output = Result<CreateLoginProfileOutput, SdkError<CreateLoginProfileError>>> {
        builder.send_with(&self.0)
    }
    fn create_open_id_connect_provider(&self, builder: CreateOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<CreateOpenIdConnectProviderOutput, SdkError<CreateOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn create_policy(&self, builder: CreatePolicyInputBuilder) -> impl Future<Output = Result<CreatePolicyOutput, SdkError<CreatePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn create_policy_version(&self, builder: CreatePolicyVersionInputBuilder) -> impl Future<Output = Result<CreatePolicyVersionOutput, SdkError<CreatePolicyVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_role(&self, builder: CreateRoleInputBuilder) -> impl Future<Output = Result<CreateRoleOutput, SdkError<CreateRoleError>>> {
        builder.send_with(&self.0)
    }
    fn create_saml_provider(&self, builder: CreateSamlProviderInputBuilder) -> impl Future<Output = Result<CreateSamlProviderOutput, SdkError<CreateSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn create_service_linked_role(&self, builder: CreateServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<CreateServiceLinkedRoleOutput, SdkError<CreateServiceLinkedRoleError>>> {
        builder.send_with(&self.0)
    }
    fn create_service_specific_credential(&self, builder: CreateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<CreateServiceSpecificCredentialOutput, SdkError<CreateServiceSpecificCredentialError>>> {
        builder.send_with(&self.0)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn create_virtual_mfa_device(&self, builder: CreateVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<CreateVirtualMfaDeviceOutput, SdkError<CreateVirtualMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn deactivate_mfa_device(&self, builder: DeactivateMfaDeviceInputBuilder) -> impl Future<Output = Result<DeactivateMfaDeviceOutput, SdkError<DeactivateMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_access_key(&self, builder: DeleteAccessKeyInputBuilder) -> impl Future<Output = Result<DeleteAccessKeyOutput, SdkError<DeleteAccessKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_account_alias(&self, builder: DeleteAccountAliasInputBuilder) -> impl Future<Output = Result<DeleteAccountAliasOutput, SdkError<DeleteAccountAliasError>>> {
        builder.send_with(&self.0)
    }
    fn delete_account_password_policy(&self, builder: DeleteAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<DeleteAccountPasswordPolicyOutput, SdkError<DeleteAccountPasswordPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group_policy(&self, builder: DeleteGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteGroupPolicyOutput, SdkError<DeleteGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_instance_profile(&self, builder: DeleteInstanceProfileInputBuilder) -> impl Future<Output = Result<DeleteInstanceProfileOutput, SdkError<DeleteInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_login_profile(&self, builder: DeleteLoginProfileInputBuilder) -> impl Future<Output = Result<DeleteLoginProfileOutput, SdkError<DeleteLoginProfileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_open_id_connect_provider(&self, builder: DeleteOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<DeleteOpenIdConnectProviderOutput, SdkError<DeleteOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_policy_version(&self, builder: DeletePolicyVersionInputBuilder) -> impl Future<Output = Result<DeletePolicyVersionOutput, SdkError<DeletePolicyVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_role(&self, builder: DeleteRoleInputBuilder) -> impl Future<Output = Result<DeleteRoleOutput, SdkError<DeleteRoleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_role_permissions_boundary(&self, builder: DeleteRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteRolePermissionsBoundaryOutput, SdkError<DeleteRolePermissionsBoundaryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_role_policy(&self, builder: DeleteRolePolicyInputBuilder) -> impl Future<Output = Result<DeleteRolePolicyOutput, SdkError<DeleteRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_saml_provider(&self, builder: DeleteSamlProviderInputBuilder) -> impl Future<Output = Result<DeleteSamlProviderOutput, SdkError<DeleteSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_server_certificate(&self, builder: DeleteServerCertificateInputBuilder) -> impl Future<Output = Result<DeleteServerCertificateOutput, SdkError<DeleteServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_service_linked_role(&self, builder: DeleteServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<DeleteServiceLinkedRoleOutput, SdkError<DeleteServiceLinkedRoleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_service_specific_credential(&self, builder: DeleteServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<DeleteServiceSpecificCredentialOutput, SdkError<DeleteServiceSpecificCredentialError>>> {
        builder.send_with(&self.0)
    }
    fn delete_signing_certificate(&self, builder: DeleteSigningCertificateInputBuilder) -> impl Future<Output = Result<DeleteSigningCertificateOutput, SdkError<DeleteSigningCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_ssh_public_key(&self, builder: DeleteSshPublicKeyInputBuilder) -> impl Future<Output = Result<DeleteSshPublicKeyOutput, SdkError<DeleteSSHPublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_permissions_boundary(&self, builder: DeleteUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteUserPermissionsBoundaryOutput, SdkError<DeleteUserPermissionsBoundaryError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_policy(&self, builder: DeleteUserPolicyInputBuilder) -> impl Future<Output = Result<DeleteUserPolicyOutput, SdkError<DeleteUserPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_mfa_device(&self, builder: DeleteVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<DeleteVirtualMfaDeviceOutput, SdkError<DeleteVirtualMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn detach_group_policy(&self, builder: DetachGroupPolicyInputBuilder) -> impl Future<Output = Result<DetachGroupPolicyOutput, SdkError<DetachGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn detach_role_policy(&self, builder: DetachRolePolicyInputBuilder) -> impl Future<Output = Result<DetachRolePolicyOutput, SdkError<DetachRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn detach_user_policy(&self, builder: DetachUserPolicyInputBuilder) -> impl Future<Output = Result<DetachUserPolicyOutput, SdkError<DetachUserPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn enable_mfa_device(&self, builder: EnableMfaDeviceInputBuilder) -> impl Future<Output = Result<EnableMfaDeviceOutput, SdkError<EnableMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn generate_credential_report(&self, builder: GenerateCredentialReportInputBuilder) -> impl Future<Output = Result<GenerateCredentialReportOutput, SdkError<GenerateCredentialReportError>>> {
        builder.send_with(&self.0)
    }
    fn generate_organizations_access_report(&self, builder: GenerateOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GenerateOrganizationsAccessReportOutput, SdkError<GenerateOrganizationsAccessReportError>>> {
        builder.send_with(&self.0)
    }
    fn generate_service_last_accessed_details(&self, builder: GenerateServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GenerateServiceLastAccessedDetailsOutput, SdkError<GenerateServiceLastAccessedDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn get_access_key_last_used(&self, builder: GetAccessKeyLastUsedInputBuilder) -> impl Future<Output = Result<GetAccessKeyLastUsedOutput, SdkError<GetAccessKeyLastUsedError>>> {
        builder.send_with(&self.0)
    }
    fn get_account_authorization_details(&self, builder: GetAccountAuthorizationDetailsInputBuilder) -> impl Future<Output = Result<GetAccountAuthorizationDetailsOutput, SdkError<GetAccountAuthorizationDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn get_account_password_policy(&self, builder: GetAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<GetAccountPasswordPolicyOutput, SdkError<GetAccountPasswordPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_account_summary(&self, builder: GetAccountSummaryInputBuilder) -> impl Future<Output = Result<GetAccountSummaryOutput, SdkError<GetAccountSummaryError>>> {
        builder.send_with(&self.0)
    }
    fn get_context_keys_for_custom_policy(&self, builder: GetContextKeysForCustomPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForCustomPolicyOutput, SdkError<GetContextKeysForCustomPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_context_keys_for_principal_policy(&self, builder: GetContextKeysForPrincipalPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForPrincipalPolicyOutput, SdkError<GetContextKeysForPrincipalPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_credential_report(&self, builder: GetCredentialReportInputBuilder) -> impl Future<Output = Result<GetCredentialReportOutput, SdkError<GetCredentialReportError>>> {
        builder.send_with(&self.0)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn get_group_policy(&self, builder: GetGroupPolicyInputBuilder) -> impl Future<Output = Result<GetGroupPolicyOutput, SdkError<GetGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_instance_profile(&self, builder: GetInstanceProfileInputBuilder) -> impl Future<Output = Result<GetInstanceProfileOutput, SdkError<GetInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_login_profile(&self, builder: GetLoginProfileInputBuilder) -> impl Future<Output = Result<GetLoginProfileOutput, SdkError<GetLoginProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_mfa_device(&self, builder: GetMfaDeviceInputBuilder) -> impl Future<Output = Result<GetMfaDeviceOutput, SdkError<GetMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn get_open_id_connect_provider(&self, builder: GetOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<GetOpenIdConnectProviderOutput, SdkError<GetOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn get_organizations_access_report(&self, builder: GetOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GetOrganizationsAccessReportOutput, SdkError<GetOrganizationsAccessReportError>>> {
        builder.send_with(&self.0)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_policy_version(&self, builder: GetPolicyVersionInputBuilder) -> impl Future<Output = Result<GetPolicyVersionOutput, SdkError<GetPolicyVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_role(&self, builder: GetRoleInputBuilder) -> impl Future<Output = Result<GetRoleOutput, SdkError<GetRoleError>>> {
        builder.send_with(&self.0)
    }
    fn get_role_policy(&self, builder: GetRolePolicyInputBuilder) -> impl Future<Output = Result<GetRolePolicyOutput, SdkError<GetRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn get_saml_provider(&self, builder: GetSamlProviderInputBuilder) -> impl Future<Output = Result<GetSamlProviderOutput, SdkError<GetSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn get_server_certificate(&self, builder: GetServerCertificateInputBuilder) -> impl Future<Output = Result<GetServerCertificateOutput, SdkError<GetServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_service_last_accessed_details(&self, builder: GetServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsOutput, SdkError<GetServiceLastAccessedDetailsError>>> {
        builder.send_with(&self.0)
    }
    fn get_service_last_accessed_details_with_entities(&self, builder: GetServiceLastAccessedDetailsWithEntitiesInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsWithEntitiesOutput, SdkError<GetServiceLastAccessedDetailsWithEntitiesError>>> {
        builder.send_with(&self.0)
    }
    fn get_service_linked_role_deletion_status(&self, builder: GetServiceLinkedRoleDeletionStatusInputBuilder) -> impl Future<Output = Result<GetServiceLinkedRoleDeletionStatusOutput, SdkError<GetServiceLinkedRoleDeletionStatusError>>> {
        builder.send_with(&self.0)
    }
    fn get_ssh_public_key(&self, builder: GetSshPublicKeyInputBuilder) -> impl Future<Output = Result<GetSshPublicKeyOutput, SdkError<GetSSHPublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>> {
        builder.send_with(&self.0)
    }
    fn get_user_policy(&self, builder: GetUserPolicyInputBuilder) -> impl Future<Output = Result<GetUserPolicyOutput, SdkError<GetUserPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn list_access_keys(&self, builder: ListAccessKeysInputBuilder) -> impl Future<Output = Result<ListAccessKeysOutput, SdkError<ListAccessKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_account_aliases(&self, builder: ListAccountAliasesInputBuilder) -> impl Future<Output = Result<ListAccountAliasesOutput, SdkError<ListAccountAliasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_attached_group_policies(&self, builder: ListAttachedGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedGroupPoliciesOutput, SdkError<ListAttachedGroupPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_attached_role_policies(&self, builder: ListAttachedRolePoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedRolePoliciesOutput, SdkError<ListAttachedRolePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_attached_user_policies(&self, builder: ListAttachedUserPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedUserPoliciesOutput, SdkError<ListAttachedUserPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_entities_for_policy(&self, builder: ListEntitiesForPolicyInputBuilder) -> impl Future<Output = Result<ListEntitiesForPolicyOutput, SdkError<ListEntitiesForPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn list_group_policies(&self, builder: ListGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListGroupPoliciesOutput, SdkError<ListGroupPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_groups_for_user(&self, builder: ListGroupsForUserInputBuilder) -> impl Future<Output = Result<ListGroupsForUserOutput, SdkError<ListGroupsForUserError>>> {
        builder.send_with(&self.0)
    }
    fn list_instance_profile_tags(&self, builder: ListInstanceProfileTagsInputBuilder) -> impl Future<Output = Result<ListInstanceProfileTagsOutput, SdkError<ListInstanceProfileTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_instance_profiles(&self, builder: ListInstanceProfilesInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesOutput, SdkError<ListInstanceProfilesError>>> {
        builder.send_with(&self.0)
    }
    fn list_instance_profiles_for_role(&self, builder: ListInstanceProfilesForRoleInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesForRoleOutput, SdkError<ListInstanceProfilesForRoleError>>> {
        builder.send_with(&self.0)
    }
    fn list_mfa_device_tags(&self, builder: ListMfaDeviceTagsInputBuilder) -> impl Future<Output = Result<ListMfaDeviceTagsOutput, SdkError<ListMFADeviceTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_mfa_devices(&self, builder: ListMfaDevicesInputBuilder) -> impl Future<Output = Result<ListMfaDevicesOutput, SdkError<ListMFADevicesError>>> {
        builder.send_with(&self.0)
    }
    fn list_open_id_connect_provider_tags(&self, builder: ListOpenIdConnectProviderTagsInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProviderTagsOutput, SdkError<ListOpenIDConnectProviderTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_open_id_connect_providers(&self, builder: ListOpenIdConnectProvidersInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProvidersOutput, SdkError<ListOpenIDConnectProvidersError>>> {
        builder.send_with(&self.0)
    }
    fn list_policies(&self, builder: ListPoliciesInputBuilder) -> impl Future<Output = Result<ListPoliciesOutput, SdkError<ListPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_policies_granting_service_access(&self, builder: ListPoliciesGrantingServiceAccessInputBuilder) -> impl Future<Output = Result<ListPoliciesGrantingServiceAccessOutput, SdkError<ListPoliciesGrantingServiceAccessError>>> {
        builder.send_with(&self.0)
    }
    fn list_policy_tags(&self, builder: ListPolicyTagsInputBuilder) -> impl Future<Output = Result<ListPolicyTagsOutput, SdkError<ListPolicyTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_policy_versions(&self, builder: ListPolicyVersionsInputBuilder) -> impl Future<Output = Result<ListPolicyVersionsOutput, SdkError<ListPolicyVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_role_policies(&self, builder: ListRolePoliciesInputBuilder) -> impl Future<Output = Result<ListRolePoliciesOutput, SdkError<ListRolePoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_role_tags(&self, builder: ListRoleTagsInputBuilder) -> impl Future<Output = Result<ListRoleTagsOutput, SdkError<ListRoleTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_roles(&self, builder: ListRolesInputBuilder) -> impl Future<Output = Result<ListRolesOutput, SdkError<ListRolesError>>> {
        builder.send_with(&self.0)
    }
    fn list_saml_provider_tags(&self, builder: ListSamlProviderTagsInputBuilder) -> impl Future<Output = Result<ListSamlProviderTagsOutput, SdkError<ListSAMLProviderTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_saml_providers(&self, builder: ListSamlProvidersInputBuilder) -> impl Future<Output = Result<ListSamlProvidersOutput, SdkError<ListSAMLProvidersError>>> {
        builder.send_with(&self.0)
    }
    fn list_server_certificate_tags(&self, builder: ListServerCertificateTagsInputBuilder) -> impl Future<Output = Result<ListServerCertificateTagsOutput, SdkError<ListServerCertificateTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_server_certificates(&self, builder: ListServerCertificatesInputBuilder) -> impl Future<Output = Result<ListServerCertificatesOutput, SdkError<ListServerCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_service_specific_credentials(&self, builder: ListServiceSpecificCredentialsInputBuilder) -> impl Future<Output = Result<ListServiceSpecificCredentialsOutput, SdkError<ListServiceSpecificCredentialsError>>> {
        builder.send_with(&self.0)
    }
    fn list_signing_certificates(&self, builder: ListSigningCertificatesInputBuilder) -> impl Future<Output = Result<ListSigningCertificatesOutput, SdkError<ListSigningCertificatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_ssh_public_keys(&self, builder: ListSshPublicKeysInputBuilder) -> impl Future<Output = Result<ListSshPublicKeysOutput, SdkError<ListSSHPublicKeysError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_policies(&self, builder: ListUserPoliciesInputBuilder) -> impl Future<Output = Result<ListUserPoliciesOutput, SdkError<ListUserPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_tags(&self, builder: ListUserTagsInputBuilder) -> impl Future<Output = Result<ListUserTagsOutput, SdkError<ListUserTagsError>>> {
        builder.send_with(&self.0)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_mfa_devices(&self, builder: ListVirtualMfaDevicesInputBuilder) -> impl Future<Output = Result<ListVirtualMfaDevicesOutput, SdkError<ListVirtualMFADevicesError>>> {
        builder.send_with(&self.0)
    }
    fn put_group_policy(&self, builder: PutGroupPolicyInputBuilder) -> impl Future<Output = Result<PutGroupPolicyOutput, SdkError<PutGroupPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_role_permissions_boundary(&self, builder: PutRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutRolePermissionsBoundaryOutput, SdkError<PutRolePermissionsBoundaryError>>> {
        builder.send_with(&self.0)
    }
    fn put_role_policy(&self, builder: PutRolePolicyInputBuilder) -> impl Future<Output = Result<PutRolePolicyOutput, SdkError<PutRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn put_user_permissions_boundary(&self, builder: PutUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutUserPermissionsBoundaryOutput, SdkError<PutUserPermissionsBoundaryError>>> {
        builder.send_with(&self.0)
    }
    fn put_user_policy(&self, builder: PutUserPolicyInputBuilder) -> impl Future<Output = Result<PutUserPolicyOutput, SdkError<PutUserPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn remove_client_id_from_open_id_connect_provider(&self, builder: RemoveClientIdFromOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<RemoveClientIdFromOpenIdConnectProviderOutput, SdkError<RemoveClientIDFromOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn remove_role_from_instance_profile(&self, builder: RemoveRoleFromInstanceProfileInputBuilder) -> impl Future<Output = Result<RemoveRoleFromInstanceProfileOutput, SdkError<RemoveRoleFromInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn remove_user_from_group(&self, builder: RemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<RemoveUserFromGroupOutput, SdkError<RemoveUserFromGroupError>>> {
        builder.send_with(&self.0)
    }
    fn reset_service_specific_credential(&self, builder: ResetServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<ResetServiceSpecificCredentialOutput, SdkError<ResetServiceSpecificCredentialError>>> {
        builder.send_with(&self.0)
    }
    fn resync_mfa_device(&self, builder: ResyncMfaDeviceInputBuilder) -> impl Future<Output = Result<ResyncMfaDeviceOutput, SdkError<ResyncMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn set_default_policy_version(&self, builder: SetDefaultPolicyVersionInputBuilder) -> impl Future<Output = Result<SetDefaultPolicyVersionOutput, SdkError<SetDefaultPolicyVersionError>>> {
        builder.send_with(&self.0)
    }
    fn set_security_token_service_preferences(&self, builder: SetSecurityTokenServicePreferencesInputBuilder) -> impl Future<Output = Result<SetSecurityTokenServicePreferencesOutput, SdkError<SetSecurityTokenServicePreferencesError>>> {
        builder.send_with(&self.0)
    }
    fn simulate_custom_policy(&self, builder: SimulateCustomPolicyInputBuilder) -> impl Future<Output = Result<SimulateCustomPolicyOutput, SdkError<SimulateCustomPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn simulate_principal_policy(&self, builder: SimulatePrincipalPolicyInputBuilder) -> impl Future<Output = Result<SimulatePrincipalPolicyOutput, SdkError<SimulatePrincipalPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn tag_instance_profile(&self, builder: TagInstanceProfileInputBuilder) -> impl Future<Output = Result<TagInstanceProfileOutput, SdkError<TagInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn tag_mfa_device(&self, builder: TagMfaDeviceInputBuilder) -> impl Future<Output = Result<TagMfaDeviceOutput, SdkError<TagMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn tag_open_id_connect_provider(&self, builder: TagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<TagOpenIdConnectProviderOutput, SdkError<TagOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn tag_policy(&self, builder: TagPolicyInputBuilder) -> impl Future<Output = Result<TagPolicyOutput, SdkError<TagPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn tag_role(&self, builder: TagRoleInputBuilder) -> impl Future<Output = Result<TagRoleOutput, SdkError<TagRoleError>>> {
        builder.send_with(&self.0)
    }
    fn tag_saml_provider(&self, builder: TagSamlProviderInputBuilder) -> impl Future<Output = Result<TagSamlProviderOutput, SdkError<TagSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn tag_server_certificate(&self, builder: TagServerCertificateInputBuilder) -> impl Future<Output = Result<TagServerCertificateOutput, SdkError<TagServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn tag_user(&self, builder: TagUserInputBuilder) -> impl Future<Output = Result<TagUserOutput, SdkError<TagUserError>>> {
        builder.send_with(&self.0)
    }
    fn untag_instance_profile(&self, builder: UntagInstanceProfileInputBuilder) -> impl Future<Output = Result<UntagInstanceProfileOutput, SdkError<UntagInstanceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn untag_mfa_device(&self, builder: UntagMfaDeviceInputBuilder) -> impl Future<Output = Result<UntagMfaDeviceOutput, SdkError<UntagMFADeviceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_open_id_connect_provider(&self, builder: UntagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<UntagOpenIdConnectProviderOutput, SdkError<UntagOpenIDConnectProviderError>>> {
        builder.send_with(&self.0)
    }
    fn untag_policy(&self, builder: UntagPolicyInputBuilder) -> impl Future<Output = Result<UntagPolicyOutput, SdkError<UntagPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn untag_role(&self, builder: UntagRoleInputBuilder) -> impl Future<Output = Result<UntagRoleOutput, SdkError<UntagRoleError>>> {
        builder.send_with(&self.0)
    }
    fn untag_saml_provider(&self, builder: UntagSamlProviderInputBuilder) -> impl Future<Output = Result<UntagSamlProviderOutput, SdkError<UntagSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn untag_server_certificate(&self, builder: UntagServerCertificateInputBuilder) -> impl Future<Output = Result<UntagServerCertificateOutput, SdkError<UntagServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn untag_user(&self, builder: UntagUserInputBuilder) -> impl Future<Output = Result<UntagUserOutput, SdkError<UntagUserError>>> {
        builder.send_with(&self.0)
    }
    fn update_access_key(&self, builder: UpdateAccessKeyInputBuilder) -> impl Future<Output = Result<UpdateAccessKeyOutput, SdkError<UpdateAccessKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_account_password_policy(&self, builder: UpdateAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<UpdateAccountPasswordPolicyOutput, SdkError<UpdateAccountPasswordPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_assume_role_policy(&self, builder: UpdateAssumeRolePolicyInputBuilder) -> impl Future<Output = Result<UpdateAssumeRolePolicyOutput, SdkError<UpdateAssumeRolePolicyError>>> {
        builder.send_with(&self.0)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_login_profile(&self, builder: UpdateLoginProfileInputBuilder) -> impl Future<Output = Result<UpdateLoginProfileOutput, SdkError<UpdateLoginProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_open_id_connect_provider_thumbprint(&self, builder: UpdateOpenIdConnectProviderThumbprintInputBuilder) -> impl Future<Output = Result<UpdateOpenIdConnectProviderThumbprintOutput, SdkError<UpdateOpenIDConnectProviderThumbprintError>>> {
        builder.send_with(&self.0)
    }
    fn update_role(&self, builder: UpdateRoleInputBuilder) -> impl Future<Output = Result<UpdateRoleOutput, SdkError<UpdateRoleError>>> {
        builder.send_with(&self.0)
    }
    fn update_role_description(&self, builder: UpdateRoleDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRoleDescriptionOutput, SdkError<UpdateRoleDescriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_saml_provider(&self, builder: UpdateSamlProviderInputBuilder) -> impl Future<Output = Result<UpdateSamlProviderOutput, SdkError<UpdateSAMLProviderError>>> {
        builder.send_with(&self.0)
    }
    fn update_server_certificate(&self, builder: UpdateServerCertificateInputBuilder) -> impl Future<Output = Result<UpdateServerCertificateOutput, SdkError<UpdateServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn update_service_specific_credential(&self, builder: UpdateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<UpdateServiceSpecificCredentialOutput, SdkError<UpdateServiceSpecificCredentialError>>> {
        builder.send_with(&self.0)
    }
    fn update_signing_certificate(&self, builder: UpdateSigningCertificateInputBuilder) -> impl Future<Output = Result<UpdateSigningCertificateOutput, SdkError<UpdateSigningCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn update_ssh_public_key(&self, builder: UpdateSshPublicKeyInputBuilder) -> impl Future<Output = Result<UpdateSshPublicKeyOutput, SdkError<UpdateSSHPublicKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        builder.send_with(&self.0)
    }
    fn upload_server_certificate(&self, builder: UploadServerCertificateInputBuilder) -> impl Future<Output = Result<UploadServerCertificateOutput, SdkError<UploadServerCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn upload_signing_certificate(&self, builder: UploadSigningCertificateInputBuilder) -> impl Future<Output = Result<UploadSigningCertificateOutput, SdkError<UploadSigningCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn upload_ssh_public_key(&self, builder: UploadSshPublicKeyInputBuilder) -> impl Future<Output = Result<UploadSshPublicKeyOutput, SdkError<UploadSSHPublicKeyError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> IAMClient for T
where T: Deref,
      T::Target: IAMClient {
    fn add_client_id_to_open_id_connect_provider(&self, builder: AddClientIdToOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<AddClientIdToOpenIdConnectProviderOutput, SdkError<AddClientIDToOpenIDConnectProviderError>>> {
        self.deref().add_client_id_to_open_id_connect_provider(builder)
    }
    fn add_role_to_instance_profile(&self, builder: AddRoleToInstanceProfileInputBuilder) -> impl Future<Output = Result<AddRoleToInstanceProfileOutput, SdkError<AddRoleToInstanceProfileError>>> {
        self.deref().add_role_to_instance_profile(builder)
    }
    fn add_user_to_group(&self, builder: AddUserToGroupInputBuilder) -> impl Future<Output = Result<AddUserToGroupOutput, SdkError<AddUserToGroupError>>> {
        self.deref().add_user_to_group(builder)
    }
    fn attach_group_policy(&self, builder: AttachGroupPolicyInputBuilder) -> impl Future<Output = Result<AttachGroupPolicyOutput, SdkError<AttachGroupPolicyError>>> {
        self.deref().attach_group_policy(builder)
    }
    fn attach_role_policy(&self, builder: AttachRolePolicyInputBuilder) -> impl Future<Output = Result<AttachRolePolicyOutput, SdkError<AttachRolePolicyError>>> {
        self.deref().attach_role_policy(builder)
    }
    fn attach_user_policy(&self, builder: AttachUserPolicyInputBuilder) -> impl Future<Output = Result<AttachUserPolicyOutput, SdkError<AttachUserPolicyError>>> {
        self.deref().attach_user_policy(builder)
    }
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>> {
        self.deref().change_password(builder)
    }
    fn create_access_key(&self, builder: CreateAccessKeyInputBuilder) -> impl Future<Output = Result<CreateAccessKeyOutput, SdkError<CreateAccessKeyError>>> {
        self.deref().create_access_key(builder)
    }
    fn create_account_alias(&self, builder: CreateAccountAliasInputBuilder) -> impl Future<Output = Result<CreateAccountAliasOutput, SdkError<CreateAccountAliasError>>> {
        self.deref().create_account_alias(builder)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        self.deref().create_group(builder)
    }
    fn create_instance_profile(&self, builder: CreateInstanceProfileInputBuilder) -> impl Future<Output = Result<CreateInstanceProfileOutput, SdkError<CreateInstanceProfileError>>> {
        self.deref().create_instance_profile(builder)
    }
    fn create_login_profile(&self, builder: CreateLoginProfileInputBuilder) -> impl Future<Output = Result<CreateLoginProfileOutput, SdkError<CreateLoginProfileError>>> {
        self.deref().create_login_profile(builder)
    }
    fn create_open_id_connect_provider(&self, builder: CreateOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<CreateOpenIdConnectProviderOutput, SdkError<CreateOpenIDConnectProviderError>>> {
        self.deref().create_open_id_connect_provider(builder)
    }
    fn create_policy(&self, builder: CreatePolicyInputBuilder) -> impl Future<Output = Result<CreatePolicyOutput, SdkError<CreatePolicyError>>> {
        self.deref().create_policy(builder)
    }
    fn create_policy_version(&self, builder: CreatePolicyVersionInputBuilder) -> impl Future<Output = Result<CreatePolicyVersionOutput, SdkError<CreatePolicyVersionError>>> {
        self.deref().create_policy_version(builder)
    }
    fn create_role(&self, builder: CreateRoleInputBuilder) -> impl Future<Output = Result<CreateRoleOutput, SdkError<CreateRoleError>>> {
        self.deref().create_role(builder)
    }
    fn create_saml_provider(&self, builder: CreateSamlProviderInputBuilder) -> impl Future<Output = Result<CreateSamlProviderOutput, SdkError<CreateSAMLProviderError>>> {
        self.deref().create_saml_provider(builder)
    }
    fn create_service_linked_role(&self, builder: CreateServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<CreateServiceLinkedRoleOutput, SdkError<CreateServiceLinkedRoleError>>> {
        self.deref().create_service_linked_role(builder)
    }
    fn create_service_specific_credential(&self, builder: CreateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<CreateServiceSpecificCredentialOutput, SdkError<CreateServiceSpecificCredentialError>>> {
        self.deref().create_service_specific_credential(builder)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        self.deref().create_user(builder)
    }
    fn create_virtual_mfa_device(&self, builder: CreateVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<CreateVirtualMfaDeviceOutput, SdkError<CreateVirtualMFADeviceError>>> {
        self.deref().create_virtual_mfa_device(builder)
    }
    fn deactivate_mfa_device(&self, builder: DeactivateMfaDeviceInputBuilder) -> impl Future<Output = Result<DeactivateMfaDeviceOutput, SdkError<DeactivateMFADeviceError>>> {
        self.deref().deactivate_mfa_device(builder)
    }
    fn delete_access_key(&self, builder: DeleteAccessKeyInputBuilder) -> impl Future<Output = Result<DeleteAccessKeyOutput, SdkError<DeleteAccessKeyError>>> {
        self.deref().delete_access_key(builder)
    }
    fn delete_account_alias(&self, builder: DeleteAccountAliasInputBuilder) -> impl Future<Output = Result<DeleteAccountAliasOutput, SdkError<DeleteAccountAliasError>>> {
        self.deref().delete_account_alias(builder)
    }
    fn delete_account_password_policy(&self, builder: DeleteAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<DeleteAccountPasswordPolicyOutput, SdkError<DeleteAccountPasswordPolicyError>>> {
        self.deref().delete_account_password_policy(builder)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        self.deref().delete_group(builder)
    }
    fn delete_group_policy(&self, builder: DeleteGroupPolicyInputBuilder) -> impl Future<Output = Result<DeleteGroupPolicyOutput, SdkError<DeleteGroupPolicyError>>> {
        self.deref().delete_group_policy(builder)
    }
    fn delete_instance_profile(&self, builder: DeleteInstanceProfileInputBuilder) -> impl Future<Output = Result<DeleteInstanceProfileOutput, SdkError<DeleteInstanceProfileError>>> {
        self.deref().delete_instance_profile(builder)
    }
    fn delete_login_profile(&self, builder: DeleteLoginProfileInputBuilder) -> impl Future<Output = Result<DeleteLoginProfileOutput, SdkError<DeleteLoginProfileError>>> {
        self.deref().delete_login_profile(builder)
    }
    fn delete_open_id_connect_provider(&self, builder: DeleteOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<DeleteOpenIdConnectProviderOutput, SdkError<DeleteOpenIDConnectProviderError>>> {
        self.deref().delete_open_id_connect_provider(builder)
    }
    fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> impl Future<Output = Result<DeletePolicyOutput, SdkError<DeletePolicyError>>> {
        self.deref().delete_policy(builder)
    }
    fn delete_policy_version(&self, builder: DeletePolicyVersionInputBuilder) -> impl Future<Output = Result<DeletePolicyVersionOutput, SdkError<DeletePolicyVersionError>>> {
        self.deref().delete_policy_version(builder)
    }
    fn delete_role(&self, builder: DeleteRoleInputBuilder) -> impl Future<Output = Result<DeleteRoleOutput, SdkError<DeleteRoleError>>> {
        self.deref().delete_role(builder)
    }
    fn delete_role_permissions_boundary(&self, builder: DeleteRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteRolePermissionsBoundaryOutput, SdkError<DeleteRolePermissionsBoundaryError>>> {
        self.deref().delete_role_permissions_boundary(builder)
    }
    fn delete_role_policy(&self, builder: DeleteRolePolicyInputBuilder) -> impl Future<Output = Result<DeleteRolePolicyOutput, SdkError<DeleteRolePolicyError>>> {
        self.deref().delete_role_policy(builder)
    }
    fn delete_saml_provider(&self, builder: DeleteSamlProviderInputBuilder) -> impl Future<Output = Result<DeleteSamlProviderOutput, SdkError<DeleteSAMLProviderError>>> {
        self.deref().delete_saml_provider(builder)
    }
    fn delete_server_certificate(&self, builder: DeleteServerCertificateInputBuilder) -> impl Future<Output = Result<DeleteServerCertificateOutput, SdkError<DeleteServerCertificateError>>> {
        self.deref().delete_server_certificate(builder)
    }
    fn delete_service_linked_role(&self, builder: DeleteServiceLinkedRoleInputBuilder) -> impl Future<Output = Result<DeleteServiceLinkedRoleOutput, SdkError<DeleteServiceLinkedRoleError>>> {
        self.deref().delete_service_linked_role(builder)
    }
    fn delete_service_specific_credential(&self, builder: DeleteServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<DeleteServiceSpecificCredentialOutput, SdkError<DeleteServiceSpecificCredentialError>>> {
        self.deref().delete_service_specific_credential(builder)
    }
    fn delete_signing_certificate(&self, builder: DeleteSigningCertificateInputBuilder) -> impl Future<Output = Result<DeleteSigningCertificateOutput, SdkError<DeleteSigningCertificateError>>> {
        self.deref().delete_signing_certificate(builder)
    }
    fn delete_ssh_public_key(&self, builder: DeleteSshPublicKeyInputBuilder) -> impl Future<Output = Result<DeleteSshPublicKeyOutput, SdkError<DeleteSSHPublicKeyError>>> {
        self.deref().delete_ssh_public_key(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        self.deref().delete_user(builder)
    }
    fn delete_user_permissions_boundary(&self, builder: DeleteUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<DeleteUserPermissionsBoundaryOutput, SdkError<DeleteUserPermissionsBoundaryError>>> {
        self.deref().delete_user_permissions_boundary(builder)
    }
    fn delete_user_policy(&self, builder: DeleteUserPolicyInputBuilder) -> impl Future<Output = Result<DeleteUserPolicyOutput, SdkError<DeleteUserPolicyError>>> {
        self.deref().delete_user_policy(builder)
    }
    fn delete_virtual_mfa_device(&self, builder: DeleteVirtualMfaDeviceInputBuilder) -> impl Future<Output = Result<DeleteVirtualMfaDeviceOutput, SdkError<DeleteVirtualMFADeviceError>>> {
        self.deref().delete_virtual_mfa_device(builder)
    }
    fn detach_group_policy(&self, builder: DetachGroupPolicyInputBuilder) -> impl Future<Output = Result<DetachGroupPolicyOutput, SdkError<DetachGroupPolicyError>>> {
        self.deref().detach_group_policy(builder)
    }
    fn detach_role_policy(&self, builder: DetachRolePolicyInputBuilder) -> impl Future<Output = Result<DetachRolePolicyOutput, SdkError<DetachRolePolicyError>>> {
        self.deref().detach_role_policy(builder)
    }
    fn detach_user_policy(&self, builder: DetachUserPolicyInputBuilder) -> impl Future<Output = Result<DetachUserPolicyOutput, SdkError<DetachUserPolicyError>>> {
        self.deref().detach_user_policy(builder)
    }
    fn enable_mfa_device(&self, builder: EnableMfaDeviceInputBuilder) -> impl Future<Output = Result<EnableMfaDeviceOutput, SdkError<EnableMFADeviceError>>> {
        self.deref().enable_mfa_device(builder)
    }
    fn generate_credential_report(&self, builder: GenerateCredentialReportInputBuilder) -> impl Future<Output = Result<GenerateCredentialReportOutput, SdkError<GenerateCredentialReportError>>> {
        self.deref().generate_credential_report(builder)
    }
    fn generate_organizations_access_report(&self, builder: GenerateOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GenerateOrganizationsAccessReportOutput, SdkError<GenerateOrganizationsAccessReportError>>> {
        self.deref().generate_organizations_access_report(builder)
    }
    fn generate_service_last_accessed_details(&self, builder: GenerateServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GenerateServiceLastAccessedDetailsOutput, SdkError<GenerateServiceLastAccessedDetailsError>>> {
        self.deref().generate_service_last_accessed_details(builder)
    }
    fn get_access_key_last_used(&self, builder: GetAccessKeyLastUsedInputBuilder) -> impl Future<Output = Result<GetAccessKeyLastUsedOutput, SdkError<GetAccessKeyLastUsedError>>> {
        self.deref().get_access_key_last_used(builder)
    }
    fn get_account_authorization_details(&self, builder: GetAccountAuthorizationDetailsInputBuilder) -> impl Future<Output = Result<GetAccountAuthorizationDetailsOutput, SdkError<GetAccountAuthorizationDetailsError>>> {
        self.deref().get_account_authorization_details(builder)
    }
    fn get_account_password_policy(&self, builder: GetAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<GetAccountPasswordPolicyOutput, SdkError<GetAccountPasswordPolicyError>>> {
        self.deref().get_account_password_policy(builder)
    }
    fn get_account_summary(&self, builder: GetAccountSummaryInputBuilder) -> impl Future<Output = Result<GetAccountSummaryOutput, SdkError<GetAccountSummaryError>>> {
        self.deref().get_account_summary(builder)
    }
    fn get_context_keys_for_custom_policy(&self, builder: GetContextKeysForCustomPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForCustomPolicyOutput, SdkError<GetContextKeysForCustomPolicyError>>> {
        self.deref().get_context_keys_for_custom_policy(builder)
    }
    fn get_context_keys_for_principal_policy(&self, builder: GetContextKeysForPrincipalPolicyInputBuilder) -> impl Future<Output = Result<GetContextKeysForPrincipalPolicyOutput, SdkError<GetContextKeysForPrincipalPolicyError>>> {
        self.deref().get_context_keys_for_principal_policy(builder)
    }
    fn get_credential_report(&self, builder: GetCredentialReportInputBuilder) -> impl Future<Output = Result<GetCredentialReportOutput, SdkError<GetCredentialReportError>>> {
        self.deref().get_credential_report(builder)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        self.deref().get_group(builder)
    }
    fn get_group_policy(&self, builder: GetGroupPolicyInputBuilder) -> impl Future<Output = Result<GetGroupPolicyOutput, SdkError<GetGroupPolicyError>>> {
        self.deref().get_group_policy(builder)
    }
    fn get_instance_profile(&self, builder: GetInstanceProfileInputBuilder) -> impl Future<Output = Result<GetInstanceProfileOutput, SdkError<GetInstanceProfileError>>> {
        self.deref().get_instance_profile(builder)
    }
    fn get_login_profile(&self, builder: GetLoginProfileInputBuilder) -> impl Future<Output = Result<GetLoginProfileOutput, SdkError<GetLoginProfileError>>> {
        self.deref().get_login_profile(builder)
    }
    fn get_mfa_device(&self, builder: GetMfaDeviceInputBuilder) -> impl Future<Output = Result<GetMfaDeviceOutput, SdkError<GetMFADeviceError>>> {
        self.deref().get_mfa_device(builder)
    }
    fn get_open_id_connect_provider(&self, builder: GetOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<GetOpenIdConnectProviderOutput, SdkError<GetOpenIDConnectProviderError>>> {
        self.deref().get_open_id_connect_provider(builder)
    }
    fn get_organizations_access_report(&self, builder: GetOrganizationsAccessReportInputBuilder) -> impl Future<Output = Result<GetOrganizationsAccessReportOutput, SdkError<GetOrganizationsAccessReportError>>> {
        self.deref().get_organizations_access_report(builder)
    }
    fn get_policy(&self, builder: GetPolicyInputBuilder) -> impl Future<Output = Result<GetPolicyOutput, SdkError<GetPolicyError>>> {
        self.deref().get_policy(builder)
    }
    fn get_policy_version(&self, builder: GetPolicyVersionInputBuilder) -> impl Future<Output = Result<GetPolicyVersionOutput, SdkError<GetPolicyVersionError>>> {
        self.deref().get_policy_version(builder)
    }
    fn get_role(&self, builder: GetRoleInputBuilder) -> impl Future<Output = Result<GetRoleOutput, SdkError<GetRoleError>>> {
        self.deref().get_role(builder)
    }
    fn get_role_policy(&self, builder: GetRolePolicyInputBuilder) -> impl Future<Output = Result<GetRolePolicyOutput, SdkError<GetRolePolicyError>>> {
        self.deref().get_role_policy(builder)
    }
    fn get_saml_provider(&self, builder: GetSamlProviderInputBuilder) -> impl Future<Output = Result<GetSamlProviderOutput, SdkError<GetSAMLProviderError>>> {
        self.deref().get_saml_provider(builder)
    }
    fn get_server_certificate(&self, builder: GetServerCertificateInputBuilder) -> impl Future<Output = Result<GetServerCertificateOutput, SdkError<GetServerCertificateError>>> {
        self.deref().get_server_certificate(builder)
    }
    fn get_service_last_accessed_details(&self, builder: GetServiceLastAccessedDetailsInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsOutput, SdkError<GetServiceLastAccessedDetailsError>>> {
        self.deref().get_service_last_accessed_details(builder)
    }
    fn get_service_last_accessed_details_with_entities(&self, builder: GetServiceLastAccessedDetailsWithEntitiesInputBuilder) -> impl Future<Output = Result<GetServiceLastAccessedDetailsWithEntitiesOutput, SdkError<GetServiceLastAccessedDetailsWithEntitiesError>>> {
        self.deref().get_service_last_accessed_details_with_entities(builder)
    }
    fn get_service_linked_role_deletion_status(&self, builder: GetServiceLinkedRoleDeletionStatusInputBuilder) -> impl Future<Output = Result<GetServiceLinkedRoleDeletionStatusOutput, SdkError<GetServiceLinkedRoleDeletionStatusError>>> {
        self.deref().get_service_linked_role_deletion_status(builder)
    }
    fn get_ssh_public_key(&self, builder: GetSshPublicKeyInputBuilder) -> impl Future<Output = Result<GetSshPublicKeyOutput, SdkError<GetSSHPublicKeyError>>> {
        self.deref().get_ssh_public_key(builder)
    }
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>> {
        self.deref().get_user(builder)
    }
    fn get_user_policy(&self, builder: GetUserPolicyInputBuilder) -> impl Future<Output = Result<GetUserPolicyOutput, SdkError<GetUserPolicyError>>> {
        self.deref().get_user_policy(builder)
    }
    fn list_access_keys(&self, builder: ListAccessKeysInputBuilder) -> impl Future<Output = Result<ListAccessKeysOutput, SdkError<ListAccessKeysError>>> {
        self.deref().list_access_keys(builder)
    }
    fn list_account_aliases(&self, builder: ListAccountAliasesInputBuilder) -> impl Future<Output = Result<ListAccountAliasesOutput, SdkError<ListAccountAliasesError>>> {
        self.deref().list_account_aliases(builder)
    }
    fn list_attached_group_policies(&self, builder: ListAttachedGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedGroupPoliciesOutput, SdkError<ListAttachedGroupPoliciesError>>> {
        self.deref().list_attached_group_policies(builder)
    }
    fn list_attached_role_policies(&self, builder: ListAttachedRolePoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedRolePoliciesOutput, SdkError<ListAttachedRolePoliciesError>>> {
        self.deref().list_attached_role_policies(builder)
    }
    fn list_attached_user_policies(&self, builder: ListAttachedUserPoliciesInputBuilder) -> impl Future<Output = Result<ListAttachedUserPoliciesOutput, SdkError<ListAttachedUserPoliciesError>>> {
        self.deref().list_attached_user_policies(builder)
    }
    fn list_entities_for_policy(&self, builder: ListEntitiesForPolicyInputBuilder) -> impl Future<Output = Result<ListEntitiesForPolicyOutput, SdkError<ListEntitiesForPolicyError>>> {
        self.deref().list_entities_for_policy(builder)
    }
    fn list_group_policies(&self, builder: ListGroupPoliciesInputBuilder) -> impl Future<Output = Result<ListGroupPoliciesOutput, SdkError<ListGroupPoliciesError>>> {
        self.deref().list_group_policies(builder)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        self.deref().list_groups(builder)
    }
    fn list_groups_for_user(&self, builder: ListGroupsForUserInputBuilder) -> impl Future<Output = Result<ListGroupsForUserOutput, SdkError<ListGroupsForUserError>>> {
        self.deref().list_groups_for_user(builder)
    }
    fn list_instance_profile_tags(&self, builder: ListInstanceProfileTagsInputBuilder) -> impl Future<Output = Result<ListInstanceProfileTagsOutput, SdkError<ListInstanceProfileTagsError>>> {
        self.deref().list_instance_profile_tags(builder)
    }
    fn list_instance_profiles(&self, builder: ListInstanceProfilesInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesOutput, SdkError<ListInstanceProfilesError>>> {
        self.deref().list_instance_profiles(builder)
    }
    fn list_instance_profiles_for_role(&self, builder: ListInstanceProfilesForRoleInputBuilder) -> impl Future<Output = Result<ListInstanceProfilesForRoleOutput, SdkError<ListInstanceProfilesForRoleError>>> {
        self.deref().list_instance_profiles_for_role(builder)
    }
    fn list_mfa_device_tags(&self, builder: ListMfaDeviceTagsInputBuilder) -> impl Future<Output = Result<ListMfaDeviceTagsOutput, SdkError<ListMFADeviceTagsError>>> {
        self.deref().list_mfa_device_tags(builder)
    }
    fn list_mfa_devices(&self, builder: ListMfaDevicesInputBuilder) -> impl Future<Output = Result<ListMfaDevicesOutput, SdkError<ListMFADevicesError>>> {
        self.deref().list_mfa_devices(builder)
    }
    fn list_open_id_connect_provider_tags(&self, builder: ListOpenIdConnectProviderTagsInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProviderTagsOutput, SdkError<ListOpenIDConnectProviderTagsError>>> {
        self.deref().list_open_id_connect_provider_tags(builder)
    }
    fn list_open_id_connect_providers(&self, builder: ListOpenIdConnectProvidersInputBuilder) -> impl Future<Output = Result<ListOpenIdConnectProvidersOutput, SdkError<ListOpenIDConnectProvidersError>>> {
        self.deref().list_open_id_connect_providers(builder)
    }
    fn list_policies(&self, builder: ListPoliciesInputBuilder) -> impl Future<Output = Result<ListPoliciesOutput, SdkError<ListPoliciesError>>> {
        self.deref().list_policies(builder)
    }
    fn list_policies_granting_service_access(&self, builder: ListPoliciesGrantingServiceAccessInputBuilder) -> impl Future<Output = Result<ListPoliciesGrantingServiceAccessOutput, SdkError<ListPoliciesGrantingServiceAccessError>>> {
        self.deref().list_policies_granting_service_access(builder)
    }
    fn list_policy_tags(&self, builder: ListPolicyTagsInputBuilder) -> impl Future<Output = Result<ListPolicyTagsOutput, SdkError<ListPolicyTagsError>>> {
        self.deref().list_policy_tags(builder)
    }
    fn list_policy_versions(&self, builder: ListPolicyVersionsInputBuilder) -> impl Future<Output = Result<ListPolicyVersionsOutput, SdkError<ListPolicyVersionsError>>> {
        self.deref().list_policy_versions(builder)
    }
    fn list_role_policies(&self, builder: ListRolePoliciesInputBuilder) -> impl Future<Output = Result<ListRolePoliciesOutput, SdkError<ListRolePoliciesError>>> {
        self.deref().list_role_policies(builder)
    }
    fn list_role_tags(&self, builder: ListRoleTagsInputBuilder) -> impl Future<Output = Result<ListRoleTagsOutput, SdkError<ListRoleTagsError>>> {
        self.deref().list_role_tags(builder)
    }
    fn list_roles(&self, builder: ListRolesInputBuilder) -> impl Future<Output = Result<ListRolesOutput, SdkError<ListRolesError>>> {
        self.deref().list_roles(builder)
    }
    fn list_saml_provider_tags(&self, builder: ListSamlProviderTagsInputBuilder) -> impl Future<Output = Result<ListSamlProviderTagsOutput, SdkError<ListSAMLProviderTagsError>>> {
        self.deref().list_saml_provider_tags(builder)
    }
    fn list_saml_providers(&self, builder: ListSamlProvidersInputBuilder) -> impl Future<Output = Result<ListSamlProvidersOutput, SdkError<ListSAMLProvidersError>>> {
        self.deref().list_saml_providers(builder)
    }
    fn list_server_certificate_tags(&self, builder: ListServerCertificateTagsInputBuilder) -> impl Future<Output = Result<ListServerCertificateTagsOutput, SdkError<ListServerCertificateTagsError>>> {
        self.deref().list_server_certificate_tags(builder)
    }
    fn list_server_certificates(&self, builder: ListServerCertificatesInputBuilder) -> impl Future<Output = Result<ListServerCertificatesOutput, SdkError<ListServerCertificatesError>>> {
        self.deref().list_server_certificates(builder)
    }
    fn list_service_specific_credentials(&self, builder: ListServiceSpecificCredentialsInputBuilder) -> impl Future<Output = Result<ListServiceSpecificCredentialsOutput, SdkError<ListServiceSpecificCredentialsError>>> {
        self.deref().list_service_specific_credentials(builder)
    }
    fn list_signing_certificates(&self, builder: ListSigningCertificatesInputBuilder) -> impl Future<Output = Result<ListSigningCertificatesOutput, SdkError<ListSigningCertificatesError>>> {
        self.deref().list_signing_certificates(builder)
    }
    fn list_ssh_public_keys(&self, builder: ListSshPublicKeysInputBuilder) -> impl Future<Output = Result<ListSshPublicKeysOutput, SdkError<ListSSHPublicKeysError>>> {
        self.deref().list_ssh_public_keys(builder)
    }
    fn list_user_policies(&self, builder: ListUserPoliciesInputBuilder) -> impl Future<Output = Result<ListUserPoliciesOutput, SdkError<ListUserPoliciesError>>> {
        self.deref().list_user_policies(builder)
    }
    fn list_user_tags(&self, builder: ListUserTagsInputBuilder) -> impl Future<Output = Result<ListUserTagsOutput, SdkError<ListUserTagsError>>> {
        self.deref().list_user_tags(builder)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        self.deref().list_users(builder)
    }
    fn list_virtual_mfa_devices(&self, builder: ListVirtualMfaDevicesInputBuilder) -> impl Future<Output = Result<ListVirtualMfaDevicesOutput, SdkError<ListVirtualMFADevicesError>>> {
        self.deref().list_virtual_mfa_devices(builder)
    }
    fn put_group_policy(&self, builder: PutGroupPolicyInputBuilder) -> impl Future<Output = Result<PutGroupPolicyOutput, SdkError<PutGroupPolicyError>>> {
        self.deref().put_group_policy(builder)
    }
    fn put_role_permissions_boundary(&self, builder: PutRolePermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutRolePermissionsBoundaryOutput, SdkError<PutRolePermissionsBoundaryError>>> {
        self.deref().put_role_permissions_boundary(builder)
    }
    fn put_role_policy(&self, builder: PutRolePolicyInputBuilder) -> impl Future<Output = Result<PutRolePolicyOutput, SdkError<PutRolePolicyError>>> {
        self.deref().put_role_policy(builder)
    }
    fn put_user_permissions_boundary(&self, builder: PutUserPermissionsBoundaryInputBuilder) -> impl Future<Output = Result<PutUserPermissionsBoundaryOutput, SdkError<PutUserPermissionsBoundaryError>>> {
        self.deref().put_user_permissions_boundary(builder)
    }
    fn put_user_policy(&self, builder: PutUserPolicyInputBuilder) -> impl Future<Output = Result<PutUserPolicyOutput, SdkError<PutUserPolicyError>>> {
        self.deref().put_user_policy(builder)
    }
    fn remove_client_id_from_open_id_connect_provider(&self, builder: RemoveClientIdFromOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<RemoveClientIdFromOpenIdConnectProviderOutput, SdkError<RemoveClientIDFromOpenIDConnectProviderError>>> {
        self.deref().remove_client_id_from_open_id_connect_provider(builder)
    }
    fn remove_role_from_instance_profile(&self, builder: RemoveRoleFromInstanceProfileInputBuilder) -> impl Future<Output = Result<RemoveRoleFromInstanceProfileOutput, SdkError<RemoveRoleFromInstanceProfileError>>> {
        self.deref().remove_role_from_instance_profile(builder)
    }
    fn remove_user_from_group(&self, builder: RemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<RemoveUserFromGroupOutput, SdkError<RemoveUserFromGroupError>>> {
        self.deref().remove_user_from_group(builder)
    }
    fn reset_service_specific_credential(&self, builder: ResetServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<ResetServiceSpecificCredentialOutput, SdkError<ResetServiceSpecificCredentialError>>> {
        self.deref().reset_service_specific_credential(builder)
    }
    fn resync_mfa_device(&self, builder: ResyncMfaDeviceInputBuilder) -> impl Future<Output = Result<ResyncMfaDeviceOutput, SdkError<ResyncMFADeviceError>>> {
        self.deref().resync_mfa_device(builder)
    }
    fn set_default_policy_version(&self, builder: SetDefaultPolicyVersionInputBuilder) -> impl Future<Output = Result<SetDefaultPolicyVersionOutput, SdkError<SetDefaultPolicyVersionError>>> {
        self.deref().set_default_policy_version(builder)
    }
    fn set_security_token_service_preferences(&self, builder: SetSecurityTokenServicePreferencesInputBuilder) -> impl Future<Output = Result<SetSecurityTokenServicePreferencesOutput, SdkError<SetSecurityTokenServicePreferencesError>>> {
        self.deref().set_security_token_service_preferences(builder)
    }
    fn simulate_custom_policy(&self, builder: SimulateCustomPolicyInputBuilder) -> impl Future<Output = Result<SimulateCustomPolicyOutput, SdkError<SimulateCustomPolicyError>>> {
        self.deref().simulate_custom_policy(builder)
    }
    fn simulate_principal_policy(&self, builder: SimulatePrincipalPolicyInputBuilder) -> impl Future<Output = Result<SimulatePrincipalPolicyOutput, SdkError<SimulatePrincipalPolicyError>>> {
        self.deref().simulate_principal_policy(builder)
    }
    fn tag_instance_profile(&self, builder: TagInstanceProfileInputBuilder) -> impl Future<Output = Result<TagInstanceProfileOutput, SdkError<TagInstanceProfileError>>> {
        self.deref().tag_instance_profile(builder)
    }
    fn tag_mfa_device(&self, builder: TagMfaDeviceInputBuilder) -> impl Future<Output = Result<TagMfaDeviceOutput, SdkError<TagMFADeviceError>>> {
        self.deref().tag_mfa_device(builder)
    }
    fn tag_open_id_connect_provider(&self, builder: TagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<TagOpenIdConnectProviderOutput, SdkError<TagOpenIDConnectProviderError>>> {
        self.deref().tag_open_id_connect_provider(builder)
    }
    fn tag_policy(&self, builder: TagPolicyInputBuilder) -> impl Future<Output = Result<TagPolicyOutput, SdkError<TagPolicyError>>> {
        self.deref().tag_policy(builder)
    }
    fn tag_role(&self, builder: TagRoleInputBuilder) -> impl Future<Output = Result<TagRoleOutput, SdkError<TagRoleError>>> {
        self.deref().tag_role(builder)
    }
    fn tag_saml_provider(&self, builder: TagSamlProviderInputBuilder) -> impl Future<Output = Result<TagSamlProviderOutput, SdkError<TagSAMLProviderError>>> {
        self.deref().tag_saml_provider(builder)
    }
    fn tag_server_certificate(&self, builder: TagServerCertificateInputBuilder) -> impl Future<Output = Result<TagServerCertificateOutput, SdkError<TagServerCertificateError>>> {
        self.deref().tag_server_certificate(builder)
    }
    fn tag_user(&self, builder: TagUserInputBuilder) -> impl Future<Output = Result<TagUserOutput, SdkError<TagUserError>>> {
        self.deref().tag_user(builder)
    }
    fn untag_instance_profile(&self, builder: UntagInstanceProfileInputBuilder) -> impl Future<Output = Result<UntagInstanceProfileOutput, SdkError<UntagInstanceProfileError>>> {
        self.deref().untag_instance_profile(builder)
    }
    fn untag_mfa_device(&self, builder: UntagMfaDeviceInputBuilder) -> impl Future<Output = Result<UntagMfaDeviceOutput, SdkError<UntagMFADeviceError>>> {
        self.deref().untag_mfa_device(builder)
    }
    fn untag_open_id_connect_provider(&self, builder: UntagOpenIdConnectProviderInputBuilder) -> impl Future<Output = Result<UntagOpenIdConnectProviderOutput, SdkError<UntagOpenIDConnectProviderError>>> {
        self.deref().untag_open_id_connect_provider(builder)
    }
    fn untag_policy(&self, builder: UntagPolicyInputBuilder) -> impl Future<Output = Result<UntagPolicyOutput, SdkError<UntagPolicyError>>> {
        self.deref().untag_policy(builder)
    }
    fn untag_role(&self, builder: UntagRoleInputBuilder) -> impl Future<Output = Result<UntagRoleOutput, SdkError<UntagRoleError>>> {
        self.deref().untag_role(builder)
    }
    fn untag_saml_provider(&self, builder: UntagSamlProviderInputBuilder) -> impl Future<Output = Result<UntagSamlProviderOutput, SdkError<UntagSAMLProviderError>>> {
        self.deref().untag_saml_provider(builder)
    }
    fn untag_server_certificate(&self, builder: UntagServerCertificateInputBuilder) -> impl Future<Output = Result<UntagServerCertificateOutput, SdkError<UntagServerCertificateError>>> {
        self.deref().untag_server_certificate(builder)
    }
    fn untag_user(&self, builder: UntagUserInputBuilder) -> impl Future<Output = Result<UntagUserOutput, SdkError<UntagUserError>>> {
        self.deref().untag_user(builder)
    }
    fn update_access_key(&self, builder: UpdateAccessKeyInputBuilder) -> impl Future<Output = Result<UpdateAccessKeyOutput, SdkError<UpdateAccessKeyError>>> {
        self.deref().update_access_key(builder)
    }
    fn update_account_password_policy(&self, builder: UpdateAccountPasswordPolicyInputBuilder) -> impl Future<Output = Result<UpdateAccountPasswordPolicyOutput, SdkError<UpdateAccountPasswordPolicyError>>> {
        self.deref().update_account_password_policy(builder)
    }
    fn update_assume_role_policy(&self, builder: UpdateAssumeRolePolicyInputBuilder) -> impl Future<Output = Result<UpdateAssumeRolePolicyOutput, SdkError<UpdateAssumeRolePolicyError>>> {
        self.deref().update_assume_role_policy(builder)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        self.deref().update_group(builder)
    }
    fn update_login_profile(&self, builder: UpdateLoginProfileInputBuilder) -> impl Future<Output = Result<UpdateLoginProfileOutput, SdkError<UpdateLoginProfileError>>> {
        self.deref().update_login_profile(builder)
    }
    fn update_open_id_connect_provider_thumbprint(&self, builder: UpdateOpenIdConnectProviderThumbprintInputBuilder) -> impl Future<Output = Result<UpdateOpenIdConnectProviderThumbprintOutput, SdkError<UpdateOpenIDConnectProviderThumbprintError>>> {
        self.deref().update_open_id_connect_provider_thumbprint(builder)
    }
    fn update_role(&self, builder: UpdateRoleInputBuilder) -> impl Future<Output = Result<UpdateRoleOutput, SdkError<UpdateRoleError>>> {
        self.deref().update_role(builder)
    }
    fn update_role_description(&self, builder: UpdateRoleDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRoleDescriptionOutput, SdkError<UpdateRoleDescriptionError>>> {
        self.deref().update_role_description(builder)
    }
    fn update_saml_provider(&self, builder: UpdateSamlProviderInputBuilder) -> impl Future<Output = Result<UpdateSamlProviderOutput, SdkError<UpdateSAMLProviderError>>> {
        self.deref().update_saml_provider(builder)
    }
    fn update_server_certificate(&self, builder: UpdateServerCertificateInputBuilder) -> impl Future<Output = Result<UpdateServerCertificateOutput, SdkError<UpdateServerCertificateError>>> {
        self.deref().update_server_certificate(builder)
    }
    fn update_service_specific_credential(&self, builder: UpdateServiceSpecificCredentialInputBuilder) -> impl Future<Output = Result<UpdateServiceSpecificCredentialOutput, SdkError<UpdateServiceSpecificCredentialError>>> {
        self.deref().update_service_specific_credential(builder)
    }
    fn update_signing_certificate(&self, builder: UpdateSigningCertificateInputBuilder) -> impl Future<Output = Result<UpdateSigningCertificateOutput, SdkError<UpdateSigningCertificateError>>> {
        self.deref().update_signing_certificate(builder)
    }
    fn update_ssh_public_key(&self, builder: UpdateSshPublicKeyInputBuilder) -> impl Future<Output = Result<UpdateSshPublicKeyOutput, SdkError<UpdateSSHPublicKeyError>>> {
        self.deref().update_ssh_public_key(builder)
    }
    fn update_user(&self, builder: UpdateUserInputBuilder) -> impl Future<Output = Result<UpdateUserOutput, SdkError<UpdateUserError>>> {
        self.deref().update_user(builder)
    }
    fn upload_server_certificate(&self, builder: UploadServerCertificateInputBuilder) -> impl Future<Output = Result<UploadServerCertificateOutput, SdkError<UploadServerCertificateError>>> {
        self.deref().upload_server_certificate(builder)
    }
    fn upload_signing_certificate(&self, builder: UploadSigningCertificateInputBuilder) -> impl Future<Output = Result<UploadSigningCertificateOutput, SdkError<UploadSigningCertificateError>>> {
        self.deref().upload_signing_certificate(builder)
    }
    fn upload_ssh_public_key(&self, builder: UploadSshPublicKeyInputBuilder) -> impl Future<Output = Result<UploadSshPublicKeyOutput, SdkError<UploadSSHPublicKeyError>>> {
        self.deref().upload_ssh_public_key(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edIAMClient {}
    impl IAMClient for edIAMClient {
        async fn add_client_id_to_open_id_connect_provider(&self, builder: AddClientIdToOpenIdConnectProviderInputBuilder) -> Result<AddClientIdToOpenIdConnectProviderOutput, SdkError<AddClientIDToOpenIDConnectProviderError>>;
        async fn add_role_to_instance_profile(&self, builder: AddRoleToInstanceProfileInputBuilder) -> Result<AddRoleToInstanceProfileOutput, SdkError<AddRoleToInstanceProfileError>>;
        async fn add_user_to_group(&self, builder: AddUserToGroupInputBuilder) -> Result<AddUserToGroupOutput, SdkError<AddUserToGroupError>>;
        async fn attach_group_policy(&self, builder: AttachGroupPolicyInputBuilder) -> Result<AttachGroupPolicyOutput, SdkError<AttachGroupPolicyError>>;
        async fn attach_role_policy(&self, builder: AttachRolePolicyInputBuilder) -> Result<AttachRolePolicyOutput, SdkError<AttachRolePolicyError>>;
        async fn attach_user_policy(&self, builder: AttachUserPolicyInputBuilder) -> Result<AttachUserPolicyOutput, SdkError<AttachUserPolicyError>>;
        async fn change_password(&self, builder: ChangePasswordInputBuilder) -> Result<ChangePasswordOutput, SdkError<ChangePasswordError>>;
        async fn create_access_key(&self, builder: CreateAccessKeyInputBuilder) -> Result<CreateAccessKeyOutput, SdkError<CreateAccessKeyError>>;
        async fn create_account_alias(&self, builder: CreateAccountAliasInputBuilder) -> Result<CreateAccountAliasOutput, SdkError<CreateAccountAliasError>>;
        async fn create_group(&self, builder: CreateGroupInputBuilder) -> Result<CreateGroupOutput, SdkError<CreateGroupError>>;
        async fn create_instance_profile(&self, builder: CreateInstanceProfileInputBuilder) -> Result<CreateInstanceProfileOutput, SdkError<CreateInstanceProfileError>>;
        async fn create_login_profile(&self, builder: CreateLoginProfileInputBuilder) -> Result<CreateLoginProfileOutput, SdkError<CreateLoginProfileError>>;
        async fn create_open_id_connect_provider(&self, builder: CreateOpenIdConnectProviderInputBuilder) -> Result<CreateOpenIdConnectProviderOutput, SdkError<CreateOpenIDConnectProviderError>>;
        async fn create_policy(&self, builder: CreatePolicyInputBuilder) -> Result<CreatePolicyOutput, SdkError<CreatePolicyError>>;
        async fn create_policy_version(&self, builder: CreatePolicyVersionInputBuilder) -> Result<CreatePolicyVersionOutput, SdkError<CreatePolicyVersionError>>;
        async fn create_role(&self, builder: CreateRoleInputBuilder) -> Result<CreateRoleOutput, SdkError<CreateRoleError>>;
        async fn create_saml_provider(&self, builder: CreateSamlProviderInputBuilder) -> Result<CreateSamlProviderOutput, SdkError<CreateSAMLProviderError>>;
        async fn create_service_linked_role(&self, builder: CreateServiceLinkedRoleInputBuilder) -> Result<CreateServiceLinkedRoleOutput, SdkError<CreateServiceLinkedRoleError>>;
        async fn create_service_specific_credential(&self, builder: CreateServiceSpecificCredentialInputBuilder) -> Result<CreateServiceSpecificCredentialOutput, SdkError<CreateServiceSpecificCredentialError>>;
        async fn create_user(&self, builder: CreateUserInputBuilder) -> Result<CreateUserOutput, SdkError<CreateUserError>>;
        async fn create_virtual_mfa_device(&self, builder: CreateVirtualMfaDeviceInputBuilder) -> Result<CreateVirtualMfaDeviceOutput, SdkError<CreateVirtualMFADeviceError>>;
        async fn deactivate_mfa_device(&self, builder: DeactivateMfaDeviceInputBuilder) -> Result<DeactivateMfaDeviceOutput, SdkError<DeactivateMFADeviceError>>;
        async fn delete_access_key(&self, builder: DeleteAccessKeyInputBuilder) -> Result<DeleteAccessKeyOutput, SdkError<DeleteAccessKeyError>>;
        async fn delete_account_alias(&self, builder: DeleteAccountAliasInputBuilder) -> Result<DeleteAccountAliasOutput, SdkError<DeleteAccountAliasError>>;
        async fn delete_account_password_policy(&self, builder: DeleteAccountPasswordPolicyInputBuilder) -> Result<DeleteAccountPasswordPolicyOutput, SdkError<DeleteAccountPasswordPolicyError>>;
        async fn delete_group(&self, builder: DeleteGroupInputBuilder) -> Result<DeleteGroupOutput, SdkError<DeleteGroupError>>;
        async fn delete_group_policy(&self, builder: DeleteGroupPolicyInputBuilder) -> Result<DeleteGroupPolicyOutput, SdkError<DeleteGroupPolicyError>>;
        async fn delete_instance_profile(&self, builder: DeleteInstanceProfileInputBuilder) -> Result<DeleteInstanceProfileOutput, SdkError<DeleteInstanceProfileError>>;
        async fn delete_login_profile(&self, builder: DeleteLoginProfileInputBuilder) -> Result<DeleteLoginProfileOutput, SdkError<DeleteLoginProfileError>>;
        async fn delete_open_id_connect_provider(&self, builder: DeleteOpenIdConnectProviderInputBuilder) -> Result<DeleteOpenIdConnectProviderOutput, SdkError<DeleteOpenIDConnectProviderError>>;
        async fn delete_policy(&self, builder: DeletePolicyInputBuilder) -> Result<DeletePolicyOutput, SdkError<DeletePolicyError>>;
        async fn delete_policy_version(&self, builder: DeletePolicyVersionInputBuilder) -> Result<DeletePolicyVersionOutput, SdkError<DeletePolicyVersionError>>;
        async fn delete_role(&self, builder: DeleteRoleInputBuilder) -> Result<DeleteRoleOutput, SdkError<DeleteRoleError>>;
        async fn delete_role_permissions_boundary(&self, builder: DeleteRolePermissionsBoundaryInputBuilder) -> Result<DeleteRolePermissionsBoundaryOutput, SdkError<DeleteRolePermissionsBoundaryError>>;
        async fn delete_role_policy(&self, builder: DeleteRolePolicyInputBuilder) -> Result<DeleteRolePolicyOutput, SdkError<DeleteRolePolicyError>>;
        async fn delete_saml_provider(&self, builder: DeleteSamlProviderInputBuilder) -> Result<DeleteSamlProviderOutput, SdkError<DeleteSAMLProviderError>>;
        async fn delete_server_certificate(&self, builder: DeleteServerCertificateInputBuilder) -> Result<DeleteServerCertificateOutput, SdkError<DeleteServerCertificateError>>;
        async fn delete_service_linked_role(&self, builder: DeleteServiceLinkedRoleInputBuilder) -> Result<DeleteServiceLinkedRoleOutput, SdkError<DeleteServiceLinkedRoleError>>;
        async fn delete_service_specific_credential(&self, builder: DeleteServiceSpecificCredentialInputBuilder) -> Result<DeleteServiceSpecificCredentialOutput, SdkError<DeleteServiceSpecificCredentialError>>;
        async fn delete_signing_certificate(&self, builder: DeleteSigningCertificateInputBuilder) -> Result<DeleteSigningCertificateOutput, SdkError<DeleteSigningCertificateError>>;
        async fn delete_ssh_public_key(&self, builder: DeleteSshPublicKeyInputBuilder) -> Result<DeleteSshPublicKeyOutput, SdkError<DeleteSSHPublicKeyError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn delete_user_permissions_boundary(&self, builder: DeleteUserPermissionsBoundaryInputBuilder) -> Result<DeleteUserPermissionsBoundaryOutput, SdkError<DeleteUserPermissionsBoundaryError>>;
        async fn delete_user_policy(&self, builder: DeleteUserPolicyInputBuilder) -> Result<DeleteUserPolicyOutput, SdkError<DeleteUserPolicyError>>;
        async fn delete_virtual_mfa_device(&self, builder: DeleteVirtualMfaDeviceInputBuilder) -> Result<DeleteVirtualMfaDeviceOutput, SdkError<DeleteVirtualMFADeviceError>>;
        async fn detach_group_policy(&self, builder: DetachGroupPolicyInputBuilder) -> Result<DetachGroupPolicyOutput, SdkError<DetachGroupPolicyError>>;
        async fn detach_role_policy(&self, builder: DetachRolePolicyInputBuilder) -> Result<DetachRolePolicyOutput, SdkError<DetachRolePolicyError>>;
        async fn detach_user_policy(&self, builder: DetachUserPolicyInputBuilder) -> Result<DetachUserPolicyOutput, SdkError<DetachUserPolicyError>>;
        async fn enable_mfa_device(&self, builder: EnableMfaDeviceInputBuilder) -> Result<EnableMfaDeviceOutput, SdkError<EnableMFADeviceError>>;
        async fn generate_credential_report(&self, builder: GenerateCredentialReportInputBuilder) -> Result<GenerateCredentialReportOutput, SdkError<GenerateCredentialReportError>>;
        async fn generate_organizations_access_report(&self, builder: GenerateOrganizationsAccessReportInputBuilder) -> Result<GenerateOrganizationsAccessReportOutput, SdkError<GenerateOrganizationsAccessReportError>>;
        async fn generate_service_last_accessed_details(&self, builder: GenerateServiceLastAccessedDetailsInputBuilder) -> Result<GenerateServiceLastAccessedDetailsOutput, SdkError<GenerateServiceLastAccessedDetailsError>>;
        async fn get_access_key_last_used(&self, builder: GetAccessKeyLastUsedInputBuilder) -> Result<GetAccessKeyLastUsedOutput, SdkError<GetAccessKeyLastUsedError>>;
        async fn get_account_authorization_details(&self, builder: GetAccountAuthorizationDetailsInputBuilder) -> Result<GetAccountAuthorizationDetailsOutput, SdkError<GetAccountAuthorizationDetailsError>>;
        async fn get_account_password_policy(&self, builder: GetAccountPasswordPolicyInputBuilder) -> Result<GetAccountPasswordPolicyOutput, SdkError<GetAccountPasswordPolicyError>>;
        async fn get_account_summary(&self, builder: GetAccountSummaryInputBuilder) -> Result<GetAccountSummaryOutput, SdkError<GetAccountSummaryError>>;
        async fn get_context_keys_for_custom_policy(&self, builder: GetContextKeysForCustomPolicyInputBuilder) -> Result<GetContextKeysForCustomPolicyOutput, SdkError<GetContextKeysForCustomPolicyError>>;
        async fn get_context_keys_for_principal_policy(&self, builder: GetContextKeysForPrincipalPolicyInputBuilder) -> Result<GetContextKeysForPrincipalPolicyOutput, SdkError<GetContextKeysForPrincipalPolicyError>>;
        async fn get_credential_report(&self, builder: GetCredentialReportInputBuilder) -> Result<GetCredentialReportOutput, SdkError<GetCredentialReportError>>;
        async fn get_group(&self, builder: GetGroupInputBuilder) -> Result<GetGroupOutput, SdkError<GetGroupError>>;
        async fn get_group_policy(&self, builder: GetGroupPolicyInputBuilder) -> Result<GetGroupPolicyOutput, SdkError<GetGroupPolicyError>>;
        async fn get_instance_profile(&self, builder: GetInstanceProfileInputBuilder) -> Result<GetInstanceProfileOutput, SdkError<GetInstanceProfileError>>;
        async fn get_login_profile(&self, builder: GetLoginProfileInputBuilder) -> Result<GetLoginProfileOutput, SdkError<GetLoginProfileError>>;
        async fn get_mfa_device(&self, builder: GetMfaDeviceInputBuilder) -> Result<GetMfaDeviceOutput, SdkError<GetMFADeviceError>>;
        async fn get_open_id_connect_provider(&self, builder: GetOpenIdConnectProviderInputBuilder) -> Result<GetOpenIdConnectProviderOutput, SdkError<GetOpenIDConnectProviderError>>;
        async fn get_organizations_access_report(&self, builder: GetOrganizationsAccessReportInputBuilder) -> Result<GetOrganizationsAccessReportOutput, SdkError<GetOrganizationsAccessReportError>>;
        async fn get_policy(&self, builder: GetPolicyInputBuilder) -> Result<GetPolicyOutput, SdkError<GetPolicyError>>;
        async fn get_policy_version(&self, builder: GetPolicyVersionInputBuilder) -> Result<GetPolicyVersionOutput, SdkError<GetPolicyVersionError>>;
        async fn get_role(&self, builder: GetRoleInputBuilder) -> Result<GetRoleOutput, SdkError<GetRoleError>>;
        async fn get_role_policy(&self, builder: GetRolePolicyInputBuilder) -> Result<GetRolePolicyOutput, SdkError<GetRolePolicyError>>;
        async fn get_saml_provider(&self, builder: GetSamlProviderInputBuilder) -> Result<GetSamlProviderOutput, SdkError<GetSAMLProviderError>>;
        async fn get_server_certificate(&self, builder: GetServerCertificateInputBuilder) -> Result<GetServerCertificateOutput, SdkError<GetServerCertificateError>>;
        async fn get_service_last_accessed_details(&self, builder: GetServiceLastAccessedDetailsInputBuilder) -> Result<GetServiceLastAccessedDetailsOutput, SdkError<GetServiceLastAccessedDetailsError>>;
        async fn get_service_last_accessed_details_with_entities(&self, builder: GetServiceLastAccessedDetailsWithEntitiesInputBuilder) -> Result<GetServiceLastAccessedDetailsWithEntitiesOutput, SdkError<GetServiceLastAccessedDetailsWithEntitiesError>>;
        async fn get_service_linked_role_deletion_status(&self, builder: GetServiceLinkedRoleDeletionStatusInputBuilder) -> Result<GetServiceLinkedRoleDeletionStatusOutput, SdkError<GetServiceLinkedRoleDeletionStatusError>>;
        async fn get_ssh_public_key(&self, builder: GetSshPublicKeyInputBuilder) -> Result<GetSshPublicKeyOutput, SdkError<GetSSHPublicKeyError>>;
        async fn get_user(&self, builder: GetUserInputBuilder) -> Result<GetUserOutput, SdkError<GetUserError>>;
        async fn get_user_policy(&self, builder: GetUserPolicyInputBuilder) -> Result<GetUserPolicyOutput, SdkError<GetUserPolicyError>>;
        async fn list_access_keys(&self, builder: ListAccessKeysInputBuilder) -> Result<ListAccessKeysOutput, SdkError<ListAccessKeysError>>;
        async fn list_account_aliases(&self, builder: ListAccountAliasesInputBuilder) -> Result<ListAccountAliasesOutput, SdkError<ListAccountAliasesError>>;
        async fn list_attached_group_policies(&self, builder: ListAttachedGroupPoliciesInputBuilder) -> Result<ListAttachedGroupPoliciesOutput, SdkError<ListAttachedGroupPoliciesError>>;
        async fn list_attached_role_policies(&self, builder: ListAttachedRolePoliciesInputBuilder) -> Result<ListAttachedRolePoliciesOutput, SdkError<ListAttachedRolePoliciesError>>;
        async fn list_attached_user_policies(&self, builder: ListAttachedUserPoliciesInputBuilder) -> Result<ListAttachedUserPoliciesOutput, SdkError<ListAttachedUserPoliciesError>>;
        async fn list_entities_for_policy(&self, builder: ListEntitiesForPolicyInputBuilder) -> Result<ListEntitiesForPolicyOutput, SdkError<ListEntitiesForPolicyError>>;
        async fn list_group_policies(&self, builder: ListGroupPoliciesInputBuilder) -> Result<ListGroupPoliciesOutput, SdkError<ListGroupPoliciesError>>;
        async fn list_groups(&self, builder: ListGroupsInputBuilder) -> Result<ListGroupsOutput, SdkError<ListGroupsError>>;
        async fn list_groups_for_user(&self, builder: ListGroupsForUserInputBuilder) -> Result<ListGroupsForUserOutput, SdkError<ListGroupsForUserError>>;
        async fn list_instance_profile_tags(&self, builder: ListInstanceProfileTagsInputBuilder) -> Result<ListInstanceProfileTagsOutput, SdkError<ListInstanceProfileTagsError>>;
        async fn list_instance_profiles(&self, builder: ListInstanceProfilesInputBuilder) -> Result<ListInstanceProfilesOutput, SdkError<ListInstanceProfilesError>>;
        async fn list_instance_profiles_for_role(&self, builder: ListInstanceProfilesForRoleInputBuilder) -> Result<ListInstanceProfilesForRoleOutput, SdkError<ListInstanceProfilesForRoleError>>;
        async fn list_mfa_device_tags(&self, builder: ListMfaDeviceTagsInputBuilder) -> Result<ListMfaDeviceTagsOutput, SdkError<ListMFADeviceTagsError>>;
        async fn list_mfa_devices(&self, builder: ListMfaDevicesInputBuilder) -> Result<ListMfaDevicesOutput, SdkError<ListMFADevicesError>>;
        async fn list_open_id_connect_provider_tags(&self, builder: ListOpenIdConnectProviderTagsInputBuilder) -> Result<ListOpenIdConnectProviderTagsOutput, SdkError<ListOpenIDConnectProviderTagsError>>;
        async fn list_open_id_connect_providers(&self, builder: ListOpenIdConnectProvidersInputBuilder) -> Result<ListOpenIdConnectProvidersOutput, SdkError<ListOpenIDConnectProvidersError>>;
        async fn list_policies(&self, builder: ListPoliciesInputBuilder) -> Result<ListPoliciesOutput, SdkError<ListPoliciesError>>;
        async fn list_policies_granting_service_access(&self, builder: ListPoliciesGrantingServiceAccessInputBuilder) -> Result<ListPoliciesGrantingServiceAccessOutput, SdkError<ListPoliciesGrantingServiceAccessError>>;
        async fn list_policy_tags(&self, builder: ListPolicyTagsInputBuilder) -> Result<ListPolicyTagsOutput, SdkError<ListPolicyTagsError>>;
        async fn list_policy_versions(&self, builder: ListPolicyVersionsInputBuilder) -> Result<ListPolicyVersionsOutput, SdkError<ListPolicyVersionsError>>;
        async fn list_role_policies(&self, builder: ListRolePoliciesInputBuilder) -> Result<ListRolePoliciesOutput, SdkError<ListRolePoliciesError>>;
        async fn list_role_tags(&self, builder: ListRoleTagsInputBuilder) -> Result<ListRoleTagsOutput, SdkError<ListRoleTagsError>>;
        async fn list_roles(&self, builder: ListRolesInputBuilder) -> Result<ListRolesOutput, SdkError<ListRolesError>>;
        async fn list_saml_provider_tags(&self, builder: ListSamlProviderTagsInputBuilder) -> Result<ListSamlProviderTagsOutput, SdkError<ListSAMLProviderTagsError>>;
        async fn list_saml_providers(&self, builder: ListSamlProvidersInputBuilder) -> Result<ListSamlProvidersOutput, SdkError<ListSAMLProvidersError>>;
        async fn list_server_certificate_tags(&self, builder: ListServerCertificateTagsInputBuilder) -> Result<ListServerCertificateTagsOutput, SdkError<ListServerCertificateTagsError>>;
        async fn list_server_certificates(&self, builder: ListServerCertificatesInputBuilder) -> Result<ListServerCertificatesOutput, SdkError<ListServerCertificatesError>>;
        async fn list_service_specific_credentials(&self, builder: ListServiceSpecificCredentialsInputBuilder) -> Result<ListServiceSpecificCredentialsOutput, SdkError<ListServiceSpecificCredentialsError>>;
        async fn list_signing_certificates(&self, builder: ListSigningCertificatesInputBuilder) -> Result<ListSigningCertificatesOutput, SdkError<ListSigningCertificatesError>>;
        async fn list_ssh_public_keys(&self, builder: ListSshPublicKeysInputBuilder) -> Result<ListSshPublicKeysOutput, SdkError<ListSSHPublicKeysError>>;
        async fn list_user_policies(&self, builder: ListUserPoliciesInputBuilder) -> Result<ListUserPoliciesOutput, SdkError<ListUserPoliciesError>>;
        async fn list_user_tags(&self, builder: ListUserTagsInputBuilder) -> Result<ListUserTagsOutput, SdkError<ListUserTagsError>>;
        async fn list_users(&self, builder: ListUsersInputBuilder) -> Result<ListUsersOutput, SdkError<ListUsersError>>;
        async fn list_virtual_mfa_devices(&self, builder: ListVirtualMfaDevicesInputBuilder) -> Result<ListVirtualMfaDevicesOutput, SdkError<ListVirtualMFADevicesError>>;
        async fn put_group_policy(&self, builder: PutGroupPolicyInputBuilder) -> Result<PutGroupPolicyOutput, SdkError<PutGroupPolicyError>>;
        async fn put_role_permissions_boundary(&self, builder: PutRolePermissionsBoundaryInputBuilder) -> Result<PutRolePermissionsBoundaryOutput, SdkError<PutRolePermissionsBoundaryError>>;
        async fn put_role_policy(&self, builder: PutRolePolicyInputBuilder) -> Result<PutRolePolicyOutput, SdkError<PutRolePolicyError>>;
        async fn put_user_permissions_boundary(&self, builder: PutUserPermissionsBoundaryInputBuilder) -> Result<PutUserPermissionsBoundaryOutput, SdkError<PutUserPermissionsBoundaryError>>;
        async fn put_user_policy(&self, builder: PutUserPolicyInputBuilder) -> Result<PutUserPolicyOutput, SdkError<PutUserPolicyError>>;
        async fn remove_client_id_from_open_id_connect_provider(&self, builder: RemoveClientIdFromOpenIdConnectProviderInputBuilder) -> Result<RemoveClientIdFromOpenIdConnectProviderOutput, SdkError<RemoveClientIDFromOpenIDConnectProviderError>>;
        async fn remove_role_from_instance_profile(&self, builder: RemoveRoleFromInstanceProfileInputBuilder) -> Result<RemoveRoleFromInstanceProfileOutput, SdkError<RemoveRoleFromInstanceProfileError>>;
        async fn remove_user_from_group(&self, builder: RemoveUserFromGroupInputBuilder) -> Result<RemoveUserFromGroupOutput, SdkError<RemoveUserFromGroupError>>;
        async fn reset_service_specific_credential(&self, builder: ResetServiceSpecificCredentialInputBuilder) -> Result<ResetServiceSpecificCredentialOutput, SdkError<ResetServiceSpecificCredentialError>>;
        async fn resync_mfa_device(&self, builder: ResyncMfaDeviceInputBuilder) -> Result<ResyncMfaDeviceOutput, SdkError<ResyncMFADeviceError>>;
        async fn set_default_policy_version(&self, builder: SetDefaultPolicyVersionInputBuilder) -> Result<SetDefaultPolicyVersionOutput, SdkError<SetDefaultPolicyVersionError>>;
        async fn set_security_token_service_preferences(&self, builder: SetSecurityTokenServicePreferencesInputBuilder) -> Result<SetSecurityTokenServicePreferencesOutput, SdkError<SetSecurityTokenServicePreferencesError>>;
        async fn simulate_custom_policy(&self, builder: SimulateCustomPolicyInputBuilder) -> Result<SimulateCustomPolicyOutput, SdkError<SimulateCustomPolicyError>>;
        async fn simulate_principal_policy(&self, builder: SimulatePrincipalPolicyInputBuilder) -> Result<SimulatePrincipalPolicyOutput, SdkError<SimulatePrincipalPolicyError>>;
        async fn tag_instance_profile(&self, builder: TagInstanceProfileInputBuilder) -> Result<TagInstanceProfileOutput, SdkError<TagInstanceProfileError>>;
        async fn tag_mfa_device(&self, builder: TagMfaDeviceInputBuilder) -> Result<TagMfaDeviceOutput, SdkError<TagMFADeviceError>>;
        async fn tag_open_id_connect_provider(&self, builder: TagOpenIdConnectProviderInputBuilder) -> Result<TagOpenIdConnectProviderOutput, SdkError<TagOpenIDConnectProviderError>>;
        async fn tag_policy(&self, builder: TagPolicyInputBuilder) -> Result<TagPolicyOutput, SdkError<TagPolicyError>>;
        async fn tag_role(&self, builder: TagRoleInputBuilder) -> Result<TagRoleOutput, SdkError<TagRoleError>>;
        async fn tag_saml_provider(&self, builder: TagSamlProviderInputBuilder) -> Result<TagSamlProviderOutput, SdkError<TagSAMLProviderError>>;
        async fn tag_server_certificate(&self, builder: TagServerCertificateInputBuilder) -> Result<TagServerCertificateOutput, SdkError<TagServerCertificateError>>;
        async fn tag_user(&self, builder: TagUserInputBuilder) -> Result<TagUserOutput, SdkError<TagUserError>>;
        async fn untag_instance_profile(&self, builder: UntagInstanceProfileInputBuilder) -> Result<UntagInstanceProfileOutput, SdkError<UntagInstanceProfileError>>;
        async fn untag_mfa_device(&self, builder: UntagMfaDeviceInputBuilder) -> Result<UntagMfaDeviceOutput, SdkError<UntagMFADeviceError>>;
        async fn untag_open_id_connect_provider(&self, builder: UntagOpenIdConnectProviderInputBuilder) -> Result<UntagOpenIdConnectProviderOutput, SdkError<UntagOpenIDConnectProviderError>>;
        async fn untag_policy(&self, builder: UntagPolicyInputBuilder) -> Result<UntagPolicyOutput, SdkError<UntagPolicyError>>;
        async fn untag_role(&self, builder: UntagRoleInputBuilder) -> Result<UntagRoleOutput, SdkError<UntagRoleError>>;
        async fn untag_saml_provider(&self, builder: UntagSamlProviderInputBuilder) -> Result<UntagSamlProviderOutput, SdkError<UntagSAMLProviderError>>;
        async fn untag_server_certificate(&self, builder: UntagServerCertificateInputBuilder) -> Result<UntagServerCertificateOutput, SdkError<UntagServerCertificateError>>;
        async fn untag_user(&self, builder: UntagUserInputBuilder) -> Result<UntagUserOutput, SdkError<UntagUserError>>;
        async fn update_access_key(&self, builder: UpdateAccessKeyInputBuilder) -> Result<UpdateAccessKeyOutput, SdkError<UpdateAccessKeyError>>;
        async fn update_account_password_policy(&self, builder: UpdateAccountPasswordPolicyInputBuilder) -> Result<UpdateAccountPasswordPolicyOutput, SdkError<UpdateAccountPasswordPolicyError>>;
        async fn update_assume_role_policy(&self, builder: UpdateAssumeRolePolicyInputBuilder) -> Result<UpdateAssumeRolePolicyOutput, SdkError<UpdateAssumeRolePolicyError>>;
        async fn update_group(&self, builder: UpdateGroupInputBuilder) -> Result<UpdateGroupOutput, SdkError<UpdateGroupError>>;
        async fn update_login_profile(&self, builder: UpdateLoginProfileInputBuilder) -> Result<UpdateLoginProfileOutput, SdkError<UpdateLoginProfileError>>;
        async fn update_open_id_connect_provider_thumbprint(&self, builder: UpdateOpenIdConnectProviderThumbprintInputBuilder) -> Result<UpdateOpenIdConnectProviderThumbprintOutput, SdkError<UpdateOpenIDConnectProviderThumbprintError>>;
        async fn update_role(&self, builder: UpdateRoleInputBuilder) -> Result<UpdateRoleOutput, SdkError<UpdateRoleError>>;
        async fn update_role_description(&self, builder: UpdateRoleDescriptionInputBuilder) -> Result<UpdateRoleDescriptionOutput, SdkError<UpdateRoleDescriptionError>>;
        async fn update_saml_provider(&self, builder: UpdateSamlProviderInputBuilder) -> Result<UpdateSamlProviderOutput, SdkError<UpdateSAMLProviderError>>;
        async fn update_server_certificate(&self, builder: UpdateServerCertificateInputBuilder) -> Result<UpdateServerCertificateOutput, SdkError<UpdateServerCertificateError>>;
        async fn update_service_specific_credential(&self, builder: UpdateServiceSpecificCredentialInputBuilder) -> Result<UpdateServiceSpecificCredentialOutput, SdkError<UpdateServiceSpecificCredentialError>>;
        async fn update_signing_certificate(&self, builder: UpdateSigningCertificateInputBuilder) -> Result<UpdateSigningCertificateOutput, SdkError<UpdateSigningCertificateError>>;
        async fn update_ssh_public_key(&self, builder: UpdateSshPublicKeyInputBuilder) -> Result<UpdateSshPublicKeyOutput, SdkError<UpdateSSHPublicKeyError>>;
        async fn update_user(&self, builder: UpdateUserInputBuilder) -> Result<UpdateUserOutput, SdkError<UpdateUserError>>;
        async fn upload_server_certificate(&self, builder: UploadServerCertificateInputBuilder) -> Result<UploadServerCertificateOutput, SdkError<UploadServerCertificateError>>;
        async fn upload_signing_certificate(&self, builder: UploadSigningCertificateInputBuilder) -> Result<UploadSigningCertificateOutput, SdkError<UploadSigningCertificateError>>;
        async fn upload_ssh_public_key(&self, builder: UploadSshPublicKeyInputBuilder) -> Result<UploadSshPublicKeyOutput, SdkError<UploadSSHPublicKeyError>>;
    }
}
