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
use aws_sdk_cognitoidentityprovider::operation::add_custom_attributes::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_add_user_to_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_confirm_sign_up::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_create_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_delete_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_delete_user_attributes::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_disable_provider_for_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_disable_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_enable_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_forget_device::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_get_device::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_get_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_initiate_auth::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_link_provider_for_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_list_devices::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_list_groups_for_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_list_user_auth_events::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_remove_user_from_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_reset_user_password::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_respond_to_auth_challenge::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_set_user_mfa_preference::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_set_user_password::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_set_user_settings::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_update_auth_event_feedback::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_update_device_status::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_update_user_attributes::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::admin_user_global_sign_out::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::associate_software_token::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::change_password::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::confirm_device::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::confirm_forgot_password::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::confirm_sign_up::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_identity_provider::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_resource_server::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_user_import_job::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_user_pool::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_user_pool_client::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::create_user_pool_domain::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_identity_provider::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_resource_server::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_user_attributes::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_user_pool::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_user_pool_client::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::delete_user_pool_domain::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_identity_provider::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_resource_server::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_risk_configuration::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_user_import_job::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_user_pool::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_user_pool_client::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::describe_user_pool_domain::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::forget_device::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::forgot_password::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_csv_header::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_device::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_identity_provider_by_identifier::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_log_delivery_configuration::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_signing_certificate::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_ui_customization::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_user::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_user_attribute_verification_code::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::get_user_pool_mfa_config::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::global_sign_out::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::initiate_auth::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_devices::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_groups::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_identity_providers::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_resource_servers::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_user_import_jobs::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_user_pool_clients::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_user_pools::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_users::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::list_users_in_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::resend_confirmation_code::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::respond_to_auth_challenge::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::revoke_token::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_log_delivery_configuration::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_risk_configuration::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_ui_customization::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_user_mfa_preference::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_user_pool_mfa_config::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::set_user_settings::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::sign_up::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::start_user_import_job::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::stop_user_import_job::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::tag_resource::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::untag_resource::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_auth_event_feedback::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_device_status::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_group::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_identity_provider::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_resource_server::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_user_attributes::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_user_pool::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_user_pool_client::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::update_user_pool_domain::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::verify_software_token::{builders::*, *};
use aws_sdk_cognitoidentityprovider::operation::verify_user_attribute::{builders::*, *};
use aws_sdk_cognitoidentityprovider::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider::Client;

pub use aws_sdk_cognitoidentityprovider::*;

pub struct CognitoIdentityProviderClientImpl(Client);
impl CognitoIdentityProviderClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CognitoIdentityProviderClient {
    fn add_custom_attributes(&self, builder: AddCustomAttributesInputBuilder) -> impl Future<Output = Result<AddCustomAttributesOutput, SdkError<AddCustomAttributesError>>>;
    fn admin_add_user_to_group(&self, builder: AdminAddUserToGroupInputBuilder) -> impl Future<Output = Result<AdminAddUserToGroupOutput, SdkError<AdminAddUserToGroupError>>>;
    fn admin_confirm_sign_up(&self, builder: AdminConfirmSignUpInputBuilder) -> impl Future<Output = Result<AdminConfirmSignUpOutput, SdkError<AdminConfirmSignUpError>>>;
    fn admin_create_user(&self, builder: AdminCreateUserInputBuilder) -> impl Future<Output = Result<AdminCreateUserOutput, SdkError<AdminCreateUserError>>>;
    fn admin_delete_user(&self, builder: AdminDeleteUserInputBuilder) -> impl Future<Output = Result<AdminDeleteUserOutput, SdkError<AdminDeleteUserError>>>;
    fn admin_delete_user_attributes(&self, builder: AdminDeleteUserAttributesInputBuilder) -> impl Future<Output = Result<AdminDeleteUserAttributesOutput, SdkError<AdminDeleteUserAttributesError>>>;
    fn admin_disable_provider_for_user(&self, builder: AdminDisableProviderForUserInputBuilder) -> impl Future<Output = Result<AdminDisableProviderForUserOutput, SdkError<AdminDisableProviderForUserError>>>;
    fn admin_disable_user(&self, builder: AdminDisableUserInputBuilder) -> impl Future<Output = Result<AdminDisableUserOutput, SdkError<AdminDisableUserError>>>;
    fn admin_enable_user(&self, builder: AdminEnableUserInputBuilder) -> impl Future<Output = Result<AdminEnableUserOutput, SdkError<AdminEnableUserError>>>;
    fn admin_forget_device(&self, builder: AdminForgetDeviceInputBuilder) -> impl Future<Output = Result<AdminForgetDeviceOutput, SdkError<AdminForgetDeviceError>>>;
    fn admin_get_device(&self, builder: AdminGetDeviceInputBuilder) -> impl Future<Output = Result<AdminGetDeviceOutput, SdkError<AdminGetDeviceError>>>;
    fn admin_get_user(&self, builder: AdminGetUserInputBuilder) -> impl Future<Output = Result<AdminGetUserOutput, SdkError<AdminGetUserError>>>;
    fn admin_initiate_auth(&self, builder: AdminInitiateAuthInputBuilder) -> impl Future<Output = Result<AdminInitiateAuthOutput, SdkError<AdminInitiateAuthError>>>;
    fn admin_link_provider_for_user(&self, builder: AdminLinkProviderForUserInputBuilder) -> impl Future<Output = Result<AdminLinkProviderForUserOutput, SdkError<AdminLinkProviderForUserError>>>;
    fn admin_list_devices(&self, builder: AdminListDevicesInputBuilder) -> impl Future<Output = Result<AdminListDevicesOutput, SdkError<AdminListDevicesError>>>;
    fn admin_list_groups_for_user(&self, builder: AdminListGroupsForUserInputBuilder) -> impl Future<Output = Result<AdminListGroupsForUserOutput, SdkError<AdminListGroupsForUserError>>>;
    fn admin_list_user_auth_events(&self, builder: AdminListUserAuthEventsInputBuilder) -> impl Future<Output = Result<AdminListUserAuthEventsOutput, SdkError<AdminListUserAuthEventsError>>>;
    fn admin_remove_user_from_group(&self, builder: AdminRemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<AdminRemoveUserFromGroupOutput, SdkError<AdminRemoveUserFromGroupError>>>;
    fn admin_reset_user_password(&self, builder: AdminResetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminResetUserPasswordOutput, SdkError<AdminResetUserPasswordError>>>;
    fn admin_respond_to_auth_challenge(&self, builder: AdminRespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<AdminRespondToAuthChallengeOutput, SdkError<AdminRespondToAuthChallengeError>>>;
    fn admin_set_user_mfa_preference(&self, builder: AdminSetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<AdminSetUserMfaPreferenceOutput, SdkError<AdminSetUserMFAPreferenceError>>>;
    fn admin_set_user_password(&self, builder: AdminSetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminSetUserPasswordOutput, SdkError<AdminSetUserPasswordError>>>;
    fn admin_set_user_settings(&self, builder: AdminSetUserSettingsInputBuilder) -> impl Future<Output = Result<AdminSetUserSettingsOutput, SdkError<AdminSetUserSettingsError>>>;
    fn admin_update_auth_event_feedback(&self, builder: AdminUpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<AdminUpdateAuthEventFeedbackOutput, SdkError<AdminUpdateAuthEventFeedbackError>>>;
    fn admin_update_device_status(&self, builder: AdminUpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<AdminUpdateDeviceStatusOutput, SdkError<AdminUpdateDeviceStatusError>>>;
    fn admin_update_user_attributes(&self, builder: AdminUpdateUserAttributesInputBuilder) -> impl Future<Output = Result<AdminUpdateUserAttributesOutput, SdkError<AdminUpdateUserAttributesError>>>;
    fn admin_user_global_sign_out(&self, builder: AdminUserGlobalSignOutInputBuilder) -> impl Future<Output = Result<AdminUserGlobalSignOutOutput, SdkError<AdminUserGlobalSignOutError>>>;
    fn associate_software_token(&self, builder: AssociateSoftwareTokenInputBuilder) -> impl Future<Output = Result<AssociateSoftwareTokenOutput, SdkError<AssociateSoftwareTokenError>>>;
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>>;
    fn confirm_device(&self, builder: ConfirmDeviceInputBuilder) -> impl Future<Output = Result<ConfirmDeviceOutput, SdkError<ConfirmDeviceError>>>;
    fn confirm_forgot_password(&self, builder: ConfirmForgotPasswordInputBuilder) -> impl Future<Output = Result<ConfirmForgotPasswordOutput, SdkError<ConfirmForgotPasswordError>>>;
    fn confirm_sign_up(&self, builder: ConfirmSignUpInputBuilder) -> impl Future<Output = Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>>>;
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>>;
    fn create_identity_provider(&self, builder: CreateIdentityProviderInputBuilder) -> impl Future<Output = Result<CreateIdentityProviderOutput, SdkError<CreateIdentityProviderError>>>;
    fn create_resource_server(&self, builder: CreateResourceServerInputBuilder) -> impl Future<Output = Result<CreateResourceServerOutput, SdkError<CreateResourceServerError>>>;
    fn create_user_import_job(&self, builder: CreateUserImportJobInputBuilder) -> impl Future<Output = Result<CreateUserImportJobOutput, SdkError<CreateUserImportJobError>>>;
    fn create_user_pool(&self, builder: CreateUserPoolInputBuilder) -> impl Future<Output = Result<CreateUserPoolOutput, SdkError<CreateUserPoolError>>>;
    fn create_user_pool_client(&self, builder: CreateUserPoolClientInputBuilder) -> impl Future<Output = Result<CreateUserPoolClientOutput, SdkError<CreateUserPoolClientError>>>;
    fn create_user_pool_domain(&self, builder: CreateUserPoolDomainInputBuilder) -> impl Future<Output = Result<CreateUserPoolDomainOutput, SdkError<CreateUserPoolDomainError>>>;
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>>;
    fn delete_identity_provider(&self, builder: DeleteIdentityProviderInputBuilder) -> impl Future<Output = Result<DeleteIdentityProviderOutput, SdkError<DeleteIdentityProviderError>>>;
    fn delete_resource_server(&self, builder: DeleteResourceServerInputBuilder) -> impl Future<Output = Result<DeleteResourceServerOutput, SdkError<DeleteResourceServerError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn delete_user_attributes(&self, builder: DeleteUserAttributesInputBuilder) -> impl Future<Output = Result<DeleteUserAttributesOutput, SdkError<DeleteUserAttributesError>>>;
    fn delete_user_pool(&self, builder: DeleteUserPoolInputBuilder) -> impl Future<Output = Result<DeleteUserPoolOutput, SdkError<DeleteUserPoolError>>>;
    fn delete_user_pool_client(&self, builder: DeleteUserPoolClientInputBuilder) -> impl Future<Output = Result<DeleteUserPoolClientOutput, SdkError<DeleteUserPoolClientError>>>;
    fn delete_user_pool_domain(&self, builder: DeleteUserPoolDomainInputBuilder) -> impl Future<Output = Result<DeleteUserPoolDomainOutput, SdkError<DeleteUserPoolDomainError>>>;
    fn describe_identity_provider(&self, builder: DescribeIdentityProviderInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderOutput, SdkError<DescribeIdentityProviderError>>>;
    fn describe_resource_server(&self, builder: DescribeResourceServerInputBuilder) -> impl Future<Output = Result<DescribeResourceServerOutput, SdkError<DescribeResourceServerError>>>;
    fn describe_risk_configuration(&self, builder: DescribeRiskConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRiskConfigurationOutput, SdkError<DescribeRiskConfigurationError>>>;
    fn describe_user_import_job(&self, builder: DescribeUserImportJobInputBuilder) -> impl Future<Output = Result<DescribeUserImportJobOutput, SdkError<DescribeUserImportJobError>>>;
    fn describe_user_pool(&self, builder: DescribeUserPoolInputBuilder) -> impl Future<Output = Result<DescribeUserPoolOutput, SdkError<DescribeUserPoolError>>>;
    fn describe_user_pool_client(&self, builder: DescribeUserPoolClientInputBuilder) -> impl Future<Output = Result<DescribeUserPoolClientOutput, SdkError<DescribeUserPoolClientError>>>;
    fn describe_user_pool_domain(&self, builder: DescribeUserPoolDomainInputBuilder) -> impl Future<Output = Result<DescribeUserPoolDomainOutput, SdkError<DescribeUserPoolDomainError>>>;
    fn forget_device(&self, builder: ForgetDeviceInputBuilder) -> impl Future<Output = Result<ForgetDeviceOutput, SdkError<ForgetDeviceError>>>;
    fn forgot_password(&self, builder: ForgotPasswordInputBuilder) -> impl Future<Output = Result<ForgotPasswordOutput, SdkError<ForgotPasswordError>>>;
    fn get_csv_header(&self, builder: GetCsvHeaderInputBuilder) -> impl Future<Output = Result<GetCsvHeaderOutput, SdkError<GetCSVHeaderError>>>;
    fn get_device(&self, builder: GetDeviceInputBuilder) -> impl Future<Output = Result<GetDeviceOutput, SdkError<GetDeviceError>>>;
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>>;
    fn get_identity_provider_by_identifier(&self, builder: GetIdentityProviderByIdentifierInputBuilder) -> impl Future<Output = Result<GetIdentityProviderByIdentifierOutput, SdkError<GetIdentityProviderByIdentifierError>>>;
    fn get_log_delivery_configuration(&self, builder: GetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<GetLogDeliveryConfigurationOutput, SdkError<GetLogDeliveryConfigurationError>>>;
    fn get_signing_certificate(&self, builder: GetSigningCertificateInputBuilder) -> impl Future<Output = Result<GetSigningCertificateOutput, SdkError<GetSigningCertificateError>>>;
    fn get_ui_customization(&self, builder: GetUiCustomizationInputBuilder) -> impl Future<Output = Result<GetUiCustomizationOutput, SdkError<GetUICustomizationError>>>;
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>>;
    fn get_user_attribute_verification_code(&self, builder: GetUserAttributeVerificationCodeInputBuilder) -> impl Future<Output = Result<GetUserAttributeVerificationCodeOutput, SdkError<GetUserAttributeVerificationCodeError>>>;
    fn get_user_pool_mfa_config(&self, builder: GetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<GetUserPoolMfaConfigOutput, SdkError<GetUserPoolMfaConfigError>>>;
    fn global_sign_out(&self, builder: GlobalSignOutInputBuilder) -> impl Future<Output = Result<GlobalSignOutOutput, SdkError<GlobalSignOutError>>>;
    fn initiate_auth(&self, builder: InitiateAuthInputBuilder) -> impl Future<Output = Result<InitiateAuthOutput, SdkError<InitiateAuthError>>>;
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>>;
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>>;
    fn list_identity_providers(&self, builder: ListIdentityProvidersInputBuilder) -> impl Future<Output = Result<ListIdentityProvidersOutput, SdkError<ListIdentityProvidersError>>>;
    fn list_resource_servers(&self, builder: ListResourceServersInputBuilder) -> impl Future<Output = Result<ListResourceServersOutput, SdkError<ListResourceServersError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_user_import_jobs(&self, builder: ListUserImportJobsInputBuilder) -> impl Future<Output = Result<ListUserImportJobsOutput, SdkError<ListUserImportJobsError>>>;
    fn list_user_pool_clients(&self, builder: ListUserPoolClientsInputBuilder) -> impl Future<Output = Result<ListUserPoolClientsOutput, SdkError<ListUserPoolClientsError>>>;
    fn list_user_pools(&self, builder: ListUserPoolsInputBuilder) -> impl Future<Output = Result<ListUserPoolsOutput, SdkError<ListUserPoolsError>>>;
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>>;
    fn list_users_in_group(&self, builder: ListUsersInGroupInputBuilder) -> impl Future<Output = Result<ListUsersInGroupOutput, SdkError<ListUsersInGroupError>>>;
    fn resend_confirmation_code(&self, builder: ResendConfirmationCodeInputBuilder) -> impl Future<Output = Result<ResendConfirmationCodeOutput, SdkError<ResendConfirmationCodeError>>>;
    fn respond_to_auth_challenge(&self, builder: RespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<RespondToAuthChallengeOutput, SdkError<RespondToAuthChallengeError>>>;
    fn revoke_token(&self, builder: RevokeTokenInputBuilder) -> impl Future<Output = Result<RevokeTokenOutput, SdkError<RevokeTokenError>>>;
    fn set_log_delivery_configuration(&self, builder: SetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<SetLogDeliveryConfigurationOutput, SdkError<SetLogDeliveryConfigurationError>>>;
    fn set_risk_configuration(&self, builder: SetRiskConfigurationInputBuilder) -> impl Future<Output = Result<SetRiskConfigurationOutput, SdkError<SetRiskConfigurationError>>>;
    fn set_ui_customization(&self, builder: SetUiCustomizationInputBuilder) -> impl Future<Output = Result<SetUiCustomizationOutput, SdkError<SetUICustomizationError>>>;
    fn set_user_mfa_preference(&self, builder: SetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<SetUserMfaPreferenceOutput, SdkError<SetUserMFAPreferenceError>>>;
    fn set_user_pool_mfa_config(&self, builder: SetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<SetUserPoolMfaConfigOutput, SdkError<SetUserPoolMfaConfigError>>>;
    fn set_user_settings(&self, builder: SetUserSettingsInputBuilder) -> impl Future<Output = Result<SetUserSettingsOutput, SdkError<SetUserSettingsError>>>;
    fn sign_up(&self, builder: SignUpInputBuilder) -> impl Future<Output = Result<SignUpOutput, SdkError<SignUpError>>>;
    fn start_user_import_job(&self, builder: StartUserImportJobInputBuilder) -> impl Future<Output = Result<StartUserImportJobOutput, SdkError<StartUserImportJobError>>>;
    fn stop_user_import_job(&self, builder: StopUserImportJobInputBuilder) -> impl Future<Output = Result<StopUserImportJobOutput, SdkError<StopUserImportJobError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_auth_event_feedback(&self, builder: UpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<UpdateAuthEventFeedbackOutput, SdkError<UpdateAuthEventFeedbackError>>>;
    fn update_device_status(&self, builder: UpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<UpdateDeviceStatusOutput, SdkError<UpdateDeviceStatusError>>>;
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>>;
    fn update_identity_provider(&self, builder: UpdateIdentityProviderInputBuilder) -> impl Future<Output = Result<UpdateIdentityProviderOutput, SdkError<UpdateIdentityProviderError>>>;
    fn update_resource_server(&self, builder: UpdateResourceServerInputBuilder) -> impl Future<Output = Result<UpdateResourceServerOutput, SdkError<UpdateResourceServerError>>>;
    fn update_user_attributes(&self, builder: UpdateUserAttributesInputBuilder) -> impl Future<Output = Result<UpdateUserAttributesOutput, SdkError<UpdateUserAttributesError>>>;
    fn update_user_pool(&self, builder: UpdateUserPoolInputBuilder) -> impl Future<Output = Result<UpdateUserPoolOutput, SdkError<UpdateUserPoolError>>>;
    fn update_user_pool_client(&self, builder: UpdateUserPoolClientInputBuilder) -> impl Future<Output = Result<UpdateUserPoolClientOutput, SdkError<UpdateUserPoolClientError>>>;
    fn update_user_pool_domain(&self, builder: UpdateUserPoolDomainInputBuilder) -> impl Future<Output = Result<UpdateUserPoolDomainOutput, SdkError<UpdateUserPoolDomainError>>>;
    fn verify_software_token(&self, builder: VerifySoftwareTokenInputBuilder) -> impl Future<Output = Result<VerifySoftwareTokenOutput, SdkError<VerifySoftwareTokenError>>>;
    fn verify_user_attribute(&self, builder: VerifyUserAttributeInputBuilder) -> impl Future<Output = Result<VerifyUserAttributeOutput, SdkError<VerifyUserAttributeError>>>;
}
impl CognitoIdentityProviderClient for CognitoIdentityProviderClientImpl {
    fn add_custom_attributes(&self, builder: AddCustomAttributesInputBuilder) -> impl Future<Output = Result<AddCustomAttributesOutput, SdkError<AddCustomAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn admin_add_user_to_group(&self, builder: AdminAddUserToGroupInputBuilder) -> impl Future<Output = Result<AdminAddUserToGroupOutput, SdkError<AdminAddUserToGroupError>>> {
        builder.send_with(&self.0)
    }
    fn admin_confirm_sign_up(&self, builder: AdminConfirmSignUpInputBuilder) -> impl Future<Output = Result<AdminConfirmSignUpOutput, SdkError<AdminConfirmSignUpError>>> {
        builder.send_with(&self.0)
    }
    fn admin_create_user(&self, builder: AdminCreateUserInputBuilder) -> impl Future<Output = Result<AdminCreateUserOutput, SdkError<AdminCreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_delete_user(&self, builder: AdminDeleteUserInputBuilder) -> impl Future<Output = Result<AdminDeleteUserOutput, SdkError<AdminDeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_delete_user_attributes(&self, builder: AdminDeleteUserAttributesInputBuilder) -> impl Future<Output = Result<AdminDeleteUserAttributesOutput, SdkError<AdminDeleteUserAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn admin_disable_provider_for_user(&self, builder: AdminDisableProviderForUserInputBuilder) -> impl Future<Output = Result<AdminDisableProviderForUserOutput, SdkError<AdminDisableProviderForUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_disable_user(&self, builder: AdminDisableUserInputBuilder) -> impl Future<Output = Result<AdminDisableUserOutput, SdkError<AdminDisableUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_enable_user(&self, builder: AdminEnableUserInputBuilder) -> impl Future<Output = Result<AdminEnableUserOutput, SdkError<AdminEnableUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_forget_device(&self, builder: AdminForgetDeviceInputBuilder) -> impl Future<Output = Result<AdminForgetDeviceOutput, SdkError<AdminForgetDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn admin_get_device(&self, builder: AdminGetDeviceInputBuilder) -> impl Future<Output = Result<AdminGetDeviceOutput, SdkError<AdminGetDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn admin_get_user(&self, builder: AdminGetUserInputBuilder) -> impl Future<Output = Result<AdminGetUserOutput, SdkError<AdminGetUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_initiate_auth(&self, builder: AdminInitiateAuthInputBuilder) -> impl Future<Output = Result<AdminInitiateAuthOutput, SdkError<AdminInitiateAuthError>>> {
        builder.send_with(&self.0)
    }
    fn admin_link_provider_for_user(&self, builder: AdminLinkProviderForUserInputBuilder) -> impl Future<Output = Result<AdminLinkProviderForUserOutput, SdkError<AdminLinkProviderForUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_list_devices(&self, builder: AdminListDevicesInputBuilder) -> impl Future<Output = Result<AdminListDevicesOutput, SdkError<AdminListDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn admin_list_groups_for_user(&self, builder: AdminListGroupsForUserInputBuilder) -> impl Future<Output = Result<AdminListGroupsForUserOutput, SdkError<AdminListGroupsForUserError>>> {
        builder.send_with(&self.0)
    }
    fn admin_list_user_auth_events(&self, builder: AdminListUserAuthEventsInputBuilder) -> impl Future<Output = Result<AdminListUserAuthEventsOutput, SdkError<AdminListUserAuthEventsError>>> {
        builder.send_with(&self.0)
    }
    fn admin_remove_user_from_group(&self, builder: AdminRemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<AdminRemoveUserFromGroupOutput, SdkError<AdminRemoveUserFromGroupError>>> {
        builder.send_with(&self.0)
    }
    fn admin_reset_user_password(&self, builder: AdminResetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminResetUserPasswordOutput, SdkError<AdminResetUserPasswordError>>> {
        builder.send_with(&self.0)
    }
    fn admin_respond_to_auth_challenge(&self, builder: AdminRespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<AdminRespondToAuthChallengeOutput, SdkError<AdminRespondToAuthChallengeError>>> {
        builder.send_with(&self.0)
    }
    fn admin_set_user_mfa_preference(&self, builder: AdminSetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<AdminSetUserMfaPreferenceOutput, SdkError<AdminSetUserMFAPreferenceError>>> {
        builder.send_with(&self.0)
    }
    fn admin_set_user_password(&self, builder: AdminSetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminSetUserPasswordOutput, SdkError<AdminSetUserPasswordError>>> {
        builder.send_with(&self.0)
    }
    fn admin_set_user_settings(&self, builder: AdminSetUserSettingsInputBuilder) -> impl Future<Output = Result<AdminSetUserSettingsOutput, SdkError<AdminSetUserSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn admin_update_auth_event_feedback(&self, builder: AdminUpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<AdminUpdateAuthEventFeedbackOutput, SdkError<AdminUpdateAuthEventFeedbackError>>> {
        builder.send_with(&self.0)
    }
    fn admin_update_device_status(&self, builder: AdminUpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<AdminUpdateDeviceStatusOutput, SdkError<AdminUpdateDeviceStatusError>>> {
        builder.send_with(&self.0)
    }
    fn admin_update_user_attributes(&self, builder: AdminUpdateUserAttributesInputBuilder) -> impl Future<Output = Result<AdminUpdateUserAttributesOutput, SdkError<AdminUpdateUserAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn admin_user_global_sign_out(&self, builder: AdminUserGlobalSignOutInputBuilder) -> impl Future<Output = Result<AdminUserGlobalSignOutOutput, SdkError<AdminUserGlobalSignOutError>>> {
        builder.send_with(&self.0)
    }
    fn associate_software_token(&self, builder: AssociateSoftwareTokenInputBuilder) -> impl Future<Output = Result<AssociateSoftwareTokenOutput, SdkError<AssociateSoftwareTokenError>>> {
        builder.send_with(&self.0)
    }
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_device(&self, builder: ConfirmDeviceInputBuilder) -> impl Future<Output = Result<ConfirmDeviceOutput, SdkError<ConfirmDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_forgot_password(&self, builder: ConfirmForgotPasswordInputBuilder) -> impl Future<Output = Result<ConfirmForgotPasswordOutput, SdkError<ConfirmForgotPasswordError>>> {
        builder.send_with(&self.0)
    }
    fn confirm_sign_up(&self, builder: ConfirmSignUpInputBuilder) -> impl Future<Output = Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>>> {
        builder.send_with(&self.0)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn create_identity_provider(&self, builder: CreateIdentityProviderInputBuilder) -> impl Future<Output = Result<CreateIdentityProviderOutput, SdkError<CreateIdentityProviderError>>> {
        builder.send_with(&self.0)
    }
    fn create_resource_server(&self, builder: CreateResourceServerInputBuilder) -> impl Future<Output = Result<CreateResourceServerOutput, SdkError<CreateResourceServerError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_import_job(&self, builder: CreateUserImportJobInputBuilder) -> impl Future<Output = Result<CreateUserImportJobOutput, SdkError<CreateUserImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_pool(&self, builder: CreateUserPoolInputBuilder) -> impl Future<Output = Result<CreateUserPoolOutput, SdkError<CreateUserPoolError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_pool_client(&self, builder: CreateUserPoolClientInputBuilder) -> impl Future<Output = Result<CreateUserPoolClientOutput, SdkError<CreateUserPoolClientError>>> {
        builder.send_with(&self.0)
    }
    fn create_user_pool_domain(&self, builder: CreateUserPoolDomainInputBuilder) -> impl Future<Output = Result<CreateUserPoolDomainOutput, SdkError<CreateUserPoolDomainError>>> {
        builder.send_with(&self.0)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        builder.send_with(&self.0)
    }
    fn delete_identity_provider(&self, builder: DeleteIdentityProviderInputBuilder) -> impl Future<Output = Result<DeleteIdentityProviderOutput, SdkError<DeleteIdentityProviderError>>> {
        builder.send_with(&self.0)
    }
    fn delete_resource_server(&self, builder: DeleteResourceServerInputBuilder) -> impl Future<Output = Result<DeleteResourceServerOutput, SdkError<DeleteResourceServerError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_attributes(&self, builder: DeleteUserAttributesInputBuilder) -> impl Future<Output = Result<DeleteUserAttributesOutput, SdkError<DeleteUserAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_pool(&self, builder: DeleteUserPoolInputBuilder) -> impl Future<Output = Result<DeleteUserPoolOutput, SdkError<DeleteUserPoolError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_pool_client(&self, builder: DeleteUserPoolClientInputBuilder) -> impl Future<Output = Result<DeleteUserPoolClientOutput, SdkError<DeleteUserPoolClientError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user_pool_domain(&self, builder: DeleteUserPoolDomainInputBuilder) -> impl Future<Output = Result<DeleteUserPoolDomainOutput, SdkError<DeleteUserPoolDomainError>>> {
        builder.send_with(&self.0)
    }
    fn describe_identity_provider(&self, builder: DescribeIdentityProviderInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderOutput, SdkError<DescribeIdentityProviderError>>> {
        builder.send_with(&self.0)
    }
    fn describe_resource_server(&self, builder: DescribeResourceServerInputBuilder) -> impl Future<Output = Result<DescribeResourceServerOutput, SdkError<DescribeResourceServerError>>> {
        builder.send_with(&self.0)
    }
    fn describe_risk_configuration(&self, builder: DescribeRiskConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRiskConfigurationOutput, SdkError<DescribeRiskConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_import_job(&self, builder: DescribeUserImportJobInputBuilder) -> impl Future<Output = Result<DescribeUserImportJobOutput, SdkError<DescribeUserImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_pool(&self, builder: DescribeUserPoolInputBuilder) -> impl Future<Output = Result<DescribeUserPoolOutput, SdkError<DescribeUserPoolError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_pool_client(&self, builder: DescribeUserPoolClientInputBuilder) -> impl Future<Output = Result<DescribeUserPoolClientOutput, SdkError<DescribeUserPoolClientError>>> {
        builder.send_with(&self.0)
    }
    fn describe_user_pool_domain(&self, builder: DescribeUserPoolDomainInputBuilder) -> impl Future<Output = Result<DescribeUserPoolDomainOutput, SdkError<DescribeUserPoolDomainError>>> {
        builder.send_with(&self.0)
    }
    fn forget_device(&self, builder: ForgetDeviceInputBuilder) -> impl Future<Output = Result<ForgetDeviceOutput, SdkError<ForgetDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn forgot_password(&self, builder: ForgotPasswordInputBuilder) -> impl Future<Output = Result<ForgotPasswordOutput, SdkError<ForgotPasswordError>>> {
        builder.send_with(&self.0)
    }
    fn get_csv_header(&self, builder: GetCsvHeaderInputBuilder) -> impl Future<Output = Result<GetCsvHeaderOutput, SdkError<GetCSVHeaderError>>> {
        builder.send_with(&self.0)
    }
    fn get_device(&self, builder: GetDeviceInputBuilder) -> impl Future<Output = Result<GetDeviceOutput, SdkError<GetDeviceError>>> {
        builder.send_with(&self.0)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        builder.send_with(&self.0)
    }
    fn get_identity_provider_by_identifier(&self, builder: GetIdentityProviderByIdentifierInputBuilder) -> impl Future<Output = Result<GetIdentityProviderByIdentifierOutput, SdkError<GetIdentityProviderByIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn get_log_delivery_configuration(&self, builder: GetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<GetLogDeliveryConfigurationOutput, SdkError<GetLogDeliveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_signing_certificate(&self, builder: GetSigningCertificateInputBuilder) -> impl Future<Output = Result<GetSigningCertificateOutput, SdkError<GetSigningCertificateError>>> {
        builder.send_with(&self.0)
    }
    fn get_ui_customization(&self, builder: GetUiCustomizationInputBuilder) -> impl Future<Output = Result<GetUiCustomizationOutput, SdkError<GetUICustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>> {
        builder.send_with(&self.0)
    }
    fn get_user_attribute_verification_code(&self, builder: GetUserAttributeVerificationCodeInputBuilder) -> impl Future<Output = Result<GetUserAttributeVerificationCodeOutput, SdkError<GetUserAttributeVerificationCodeError>>> {
        builder.send_with(&self.0)
    }
    fn get_user_pool_mfa_config(&self, builder: GetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<GetUserPoolMfaConfigOutput, SdkError<GetUserPoolMfaConfigError>>> {
        builder.send_with(&self.0)
    }
    fn global_sign_out(&self, builder: GlobalSignOutInputBuilder) -> impl Future<Output = Result<GlobalSignOutOutput, SdkError<GlobalSignOutError>>> {
        builder.send_with(&self.0)
    }
    fn initiate_auth(&self, builder: InitiateAuthInputBuilder) -> impl Future<Output = Result<InitiateAuthOutput, SdkError<InitiateAuthError>>> {
        builder.send_with(&self.0)
    }
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>> {
        builder.send_with(&self.0)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        builder.send_with(&self.0)
    }
    fn list_identity_providers(&self, builder: ListIdentityProvidersInputBuilder) -> impl Future<Output = Result<ListIdentityProvidersOutput, SdkError<ListIdentityProvidersError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_servers(&self, builder: ListResourceServersInputBuilder) -> impl Future<Output = Result<ListResourceServersOutput, SdkError<ListResourceServersError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_import_jobs(&self, builder: ListUserImportJobsInputBuilder) -> impl Future<Output = Result<ListUserImportJobsOutput, SdkError<ListUserImportJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_pool_clients(&self, builder: ListUserPoolClientsInputBuilder) -> impl Future<Output = Result<ListUserPoolClientsOutput, SdkError<ListUserPoolClientsError>>> {
        builder.send_with(&self.0)
    }
    fn list_user_pools(&self, builder: ListUserPoolsInputBuilder) -> impl Future<Output = Result<ListUserPoolsOutput, SdkError<ListUserPoolsError>>> {
        builder.send_with(&self.0)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        builder.send_with(&self.0)
    }
    fn list_users_in_group(&self, builder: ListUsersInGroupInputBuilder) -> impl Future<Output = Result<ListUsersInGroupOutput, SdkError<ListUsersInGroupError>>> {
        builder.send_with(&self.0)
    }
    fn resend_confirmation_code(&self, builder: ResendConfirmationCodeInputBuilder) -> impl Future<Output = Result<ResendConfirmationCodeOutput, SdkError<ResendConfirmationCodeError>>> {
        builder.send_with(&self.0)
    }
    fn respond_to_auth_challenge(&self, builder: RespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<RespondToAuthChallengeOutput, SdkError<RespondToAuthChallengeError>>> {
        builder.send_with(&self.0)
    }
    fn revoke_token(&self, builder: RevokeTokenInputBuilder) -> impl Future<Output = Result<RevokeTokenOutput, SdkError<RevokeTokenError>>> {
        builder.send_with(&self.0)
    }
    fn set_log_delivery_configuration(&self, builder: SetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<SetLogDeliveryConfigurationOutput, SdkError<SetLogDeliveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn set_risk_configuration(&self, builder: SetRiskConfigurationInputBuilder) -> impl Future<Output = Result<SetRiskConfigurationOutput, SdkError<SetRiskConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn set_ui_customization(&self, builder: SetUiCustomizationInputBuilder) -> impl Future<Output = Result<SetUiCustomizationOutput, SdkError<SetUICustomizationError>>> {
        builder.send_with(&self.0)
    }
    fn set_user_mfa_preference(&self, builder: SetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<SetUserMfaPreferenceOutput, SdkError<SetUserMFAPreferenceError>>> {
        builder.send_with(&self.0)
    }
    fn set_user_pool_mfa_config(&self, builder: SetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<SetUserPoolMfaConfigOutput, SdkError<SetUserPoolMfaConfigError>>> {
        builder.send_with(&self.0)
    }
    fn set_user_settings(&self, builder: SetUserSettingsInputBuilder) -> impl Future<Output = Result<SetUserSettingsOutput, SdkError<SetUserSettingsError>>> {
        builder.send_with(&self.0)
    }
    fn sign_up(&self, builder: SignUpInputBuilder) -> impl Future<Output = Result<SignUpOutput, SdkError<SignUpError>>> {
        builder.send_with(&self.0)
    }
    fn start_user_import_job(&self, builder: StartUserImportJobInputBuilder) -> impl Future<Output = Result<StartUserImportJobOutput, SdkError<StartUserImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn stop_user_import_job(&self, builder: StopUserImportJobInputBuilder) -> impl Future<Output = Result<StopUserImportJobOutput, SdkError<StopUserImportJobError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_auth_event_feedback(&self, builder: UpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<UpdateAuthEventFeedbackOutput, SdkError<UpdateAuthEventFeedbackError>>> {
        builder.send_with(&self.0)
    }
    fn update_device_status(&self, builder: UpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<UpdateDeviceStatusOutput, SdkError<UpdateDeviceStatusError>>> {
        builder.send_with(&self.0)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        builder.send_with(&self.0)
    }
    fn update_identity_provider(&self, builder: UpdateIdentityProviderInputBuilder) -> impl Future<Output = Result<UpdateIdentityProviderOutput, SdkError<UpdateIdentityProviderError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_server(&self, builder: UpdateResourceServerInputBuilder) -> impl Future<Output = Result<UpdateResourceServerOutput, SdkError<UpdateResourceServerError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_attributes(&self, builder: UpdateUserAttributesInputBuilder) -> impl Future<Output = Result<UpdateUserAttributesOutput, SdkError<UpdateUserAttributesError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_pool(&self, builder: UpdateUserPoolInputBuilder) -> impl Future<Output = Result<UpdateUserPoolOutput, SdkError<UpdateUserPoolError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_pool_client(&self, builder: UpdateUserPoolClientInputBuilder) -> impl Future<Output = Result<UpdateUserPoolClientOutput, SdkError<UpdateUserPoolClientError>>> {
        builder.send_with(&self.0)
    }
    fn update_user_pool_domain(&self, builder: UpdateUserPoolDomainInputBuilder) -> impl Future<Output = Result<UpdateUserPoolDomainOutput, SdkError<UpdateUserPoolDomainError>>> {
        builder.send_with(&self.0)
    }
    fn verify_software_token(&self, builder: VerifySoftwareTokenInputBuilder) -> impl Future<Output = Result<VerifySoftwareTokenOutput, SdkError<VerifySoftwareTokenError>>> {
        builder.send_with(&self.0)
    }
    fn verify_user_attribute(&self, builder: VerifyUserAttributeInputBuilder) -> impl Future<Output = Result<VerifyUserAttributeOutput, SdkError<VerifyUserAttributeError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: CognitoIdentityProviderClient> CognitoIdentityProviderClient for &T {
    fn add_custom_attributes(&self, builder: AddCustomAttributesInputBuilder) -> impl Future<Output = Result<AddCustomAttributesOutput, SdkError<AddCustomAttributesError>>> {
        (*self).add_custom_attributes(builder)
    }
    fn admin_add_user_to_group(&self, builder: AdminAddUserToGroupInputBuilder) -> impl Future<Output = Result<AdminAddUserToGroupOutput, SdkError<AdminAddUserToGroupError>>> {
        (*self).admin_add_user_to_group(builder)
    }
    fn admin_confirm_sign_up(&self, builder: AdminConfirmSignUpInputBuilder) -> impl Future<Output = Result<AdminConfirmSignUpOutput, SdkError<AdminConfirmSignUpError>>> {
        (*self).admin_confirm_sign_up(builder)
    }
    fn admin_create_user(&self, builder: AdminCreateUserInputBuilder) -> impl Future<Output = Result<AdminCreateUserOutput, SdkError<AdminCreateUserError>>> {
        (*self).admin_create_user(builder)
    }
    fn admin_delete_user(&self, builder: AdminDeleteUserInputBuilder) -> impl Future<Output = Result<AdminDeleteUserOutput, SdkError<AdminDeleteUserError>>> {
        (*self).admin_delete_user(builder)
    }
    fn admin_delete_user_attributes(&self, builder: AdminDeleteUserAttributesInputBuilder) -> impl Future<Output = Result<AdminDeleteUserAttributesOutput, SdkError<AdminDeleteUserAttributesError>>> {
        (*self).admin_delete_user_attributes(builder)
    }
    fn admin_disable_provider_for_user(&self, builder: AdminDisableProviderForUserInputBuilder) -> impl Future<Output = Result<AdminDisableProviderForUserOutput, SdkError<AdminDisableProviderForUserError>>> {
        (*self).admin_disable_provider_for_user(builder)
    }
    fn admin_disable_user(&self, builder: AdminDisableUserInputBuilder) -> impl Future<Output = Result<AdminDisableUserOutput, SdkError<AdminDisableUserError>>> {
        (*self).admin_disable_user(builder)
    }
    fn admin_enable_user(&self, builder: AdminEnableUserInputBuilder) -> impl Future<Output = Result<AdminEnableUserOutput, SdkError<AdminEnableUserError>>> {
        (*self).admin_enable_user(builder)
    }
    fn admin_forget_device(&self, builder: AdminForgetDeviceInputBuilder) -> impl Future<Output = Result<AdminForgetDeviceOutput, SdkError<AdminForgetDeviceError>>> {
        (*self).admin_forget_device(builder)
    }
    fn admin_get_device(&self, builder: AdminGetDeviceInputBuilder) -> impl Future<Output = Result<AdminGetDeviceOutput, SdkError<AdminGetDeviceError>>> {
        (*self).admin_get_device(builder)
    }
    fn admin_get_user(&self, builder: AdminGetUserInputBuilder) -> impl Future<Output = Result<AdminGetUserOutput, SdkError<AdminGetUserError>>> {
        (*self).admin_get_user(builder)
    }
    fn admin_initiate_auth(&self, builder: AdminInitiateAuthInputBuilder) -> impl Future<Output = Result<AdminInitiateAuthOutput, SdkError<AdminInitiateAuthError>>> {
        (*self).admin_initiate_auth(builder)
    }
    fn admin_link_provider_for_user(&self, builder: AdminLinkProviderForUserInputBuilder) -> impl Future<Output = Result<AdminLinkProviderForUserOutput, SdkError<AdminLinkProviderForUserError>>> {
        (*self).admin_link_provider_for_user(builder)
    }
    fn admin_list_devices(&self, builder: AdminListDevicesInputBuilder) -> impl Future<Output = Result<AdminListDevicesOutput, SdkError<AdminListDevicesError>>> {
        (*self).admin_list_devices(builder)
    }
    fn admin_list_groups_for_user(&self, builder: AdminListGroupsForUserInputBuilder) -> impl Future<Output = Result<AdminListGroupsForUserOutput, SdkError<AdminListGroupsForUserError>>> {
        (*self).admin_list_groups_for_user(builder)
    }
    fn admin_list_user_auth_events(&self, builder: AdminListUserAuthEventsInputBuilder) -> impl Future<Output = Result<AdminListUserAuthEventsOutput, SdkError<AdminListUserAuthEventsError>>> {
        (*self).admin_list_user_auth_events(builder)
    }
    fn admin_remove_user_from_group(&self, builder: AdminRemoveUserFromGroupInputBuilder) -> impl Future<Output = Result<AdminRemoveUserFromGroupOutput, SdkError<AdminRemoveUserFromGroupError>>> {
        (*self).admin_remove_user_from_group(builder)
    }
    fn admin_reset_user_password(&self, builder: AdminResetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminResetUserPasswordOutput, SdkError<AdminResetUserPasswordError>>> {
        (*self).admin_reset_user_password(builder)
    }
    fn admin_respond_to_auth_challenge(&self, builder: AdminRespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<AdminRespondToAuthChallengeOutput, SdkError<AdminRespondToAuthChallengeError>>> {
        (*self).admin_respond_to_auth_challenge(builder)
    }
    fn admin_set_user_mfa_preference(&self, builder: AdminSetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<AdminSetUserMfaPreferenceOutput, SdkError<AdminSetUserMFAPreferenceError>>> {
        (*self).admin_set_user_mfa_preference(builder)
    }
    fn admin_set_user_password(&self, builder: AdminSetUserPasswordInputBuilder) -> impl Future<Output = Result<AdminSetUserPasswordOutput, SdkError<AdminSetUserPasswordError>>> {
        (*self).admin_set_user_password(builder)
    }
    fn admin_set_user_settings(&self, builder: AdminSetUserSettingsInputBuilder) -> impl Future<Output = Result<AdminSetUserSettingsOutput, SdkError<AdminSetUserSettingsError>>> {
        (*self).admin_set_user_settings(builder)
    }
    fn admin_update_auth_event_feedback(&self, builder: AdminUpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<AdminUpdateAuthEventFeedbackOutput, SdkError<AdminUpdateAuthEventFeedbackError>>> {
        (*self).admin_update_auth_event_feedback(builder)
    }
    fn admin_update_device_status(&self, builder: AdminUpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<AdminUpdateDeviceStatusOutput, SdkError<AdminUpdateDeviceStatusError>>> {
        (*self).admin_update_device_status(builder)
    }
    fn admin_update_user_attributes(&self, builder: AdminUpdateUserAttributesInputBuilder) -> impl Future<Output = Result<AdminUpdateUserAttributesOutput, SdkError<AdminUpdateUserAttributesError>>> {
        (*self).admin_update_user_attributes(builder)
    }
    fn admin_user_global_sign_out(&self, builder: AdminUserGlobalSignOutInputBuilder) -> impl Future<Output = Result<AdminUserGlobalSignOutOutput, SdkError<AdminUserGlobalSignOutError>>> {
        (*self).admin_user_global_sign_out(builder)
    }
    fn associate_software_token(&self, builder: AssociateSoftwareTokenInputBuilder) -> impl Future<Output = Result<AssociateSoftwareTokenOutput, SdkError<AssociateSoftwareTokenError>>> {
        (*self).associate_software_token(builder)
    }
    fn change_password(&self, builder: ChangePasswordInputBuilder) -> impl Future<Output = Result<ChangePasswordOutput, SdkError<ChangePasswordError>>> {
        (*self).change_password(builder)
    }
    fn confirm_device(&self, builder: ConfirmDeviceInputBuilder) -> impl Future<Output = Result<ConfirmDeviceOutput, SdkError<ConfirmDeviceError>>> {
        (*self).confirm_device(builder)
    }
    fn confirm_forgot_password(&self, builder: ConfirmForgotPasswordInputBuilder) -> impl Future<Output = Result<ConfirmForgotPasswordOutput, SdkError<ConfirmForgotPasswordError>>> {
        (*self).confirm_forgot_password(builder)
    }
    fn confirm_sign_up(&self, builder: ConfirmSignUpInputBuilder) -> impl Future<Output = Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>>> {
        (*self).confirm_sign_up(builder)
    }
    fn create_group(&self, builder: CreateGroupInputBuilder) -> impl Future<Output = Result<CreateGroupOutput, SdkError<CreateGroupError>>> {
        (*self).create_group(builder)
    }
    fn create_identity_provider(&self, builder: CreateIdentityProviderInputBuilder) -> impl Future<Output = Result<CreateIdentityProviderOutput, SdkError<CreateIdentityProviderError>>> {
        (*self).create_identity_provider(builder)
    }
    fn create_resource_server(&self, builder: CreateResourceServerInputBuilder) -> impl Future<Output = Result<CreateResourceServerOutput, SdkError<CreateResourceServerError>>> {
        (*self).create_resource_server(builder)
    }
    fn create_user_import_job(&self, builder: CreateUserImportJobInputBuilder) -> impl Future<Output = Result<CreateUserImportJobOutput, SdkError<CreateUserImportJobError>>> {
        (*self).create_user_import_job(builder)
    }
    fn create_user_pool(&self, builder: CreateUserPoolInputBuilder) -> impl Future<Output = Result<CreateUserPoolOutput, SdkError<CreateUserPoolError>>> {
        (*self).create_user_pool(builder)
    }
    fn create_user_pool_client(&self, builder: CreateUserPoolClientInputBuilder) -> impl Future<Output = Result<CreateUserPoolClientOutput, SdkError<CreateUserPoolClientError>>> {
        (*self).create_user_pool_client(builder)
    }
    fn create_user_pool_domain(&self, builder: CreateUserPoolDomainInputBuilder) -> impl Future<Output = Result<CreateUserPoolDomainOutput, SdkError<CreateUserPoolDomainError>>> {
        (*self).create_user_pool_domain(builder)
    }
    fn delete_group(&self, builder: DeleteGroupInputBuilder) -> impl Future<Output = Result<DeleteGroupOutput, SdkError<DeleteGroupError>>> {
        (*self).delete_group(builder)
    }
    fn delete_identity_provider(&self, builder: DeleteIdentityProviderInputBuilder) -> impl Future<Output = Result<DeleteIdentityProviderOutput, SdkError<DeleteIdentityProviderError>>> {
        (*self).delete_identity_provider(builder)
    }
    fn delete_resource_server(&self, builder: DeleteResourceServerInputBuilder) -> impl Future<Output = Result<DeleteResourceServerOutput, SdkError<DeleteResourceServerError>>> {
        (*self).delete_resource_server(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        (*self).delete_user(builder)
    }
    fn delete_user_attributes(&self, builder: DeleteUserAttributesInputBuilder) -> impl Future<Output = Result<DeleteUserAttributesOutput, SdkError<DeleteUserAttributesError>>> {
        (*self).delete_user_attributes(builder)
    }
    fn delete_user_pool(&self, builder: DeleteUserPoolInputBuilder) -> impl Future<Output = Result<DeleteUserPoolOutput, SdkError<DeleteUserPoolError>>> {
        (*self).delete_user_pool(builder)
    }
    fn delete_user_pool_client(&self, builder: DeleteUserPoolClientInputBuilder) -> impl Future<Output = Result<DeleteUserPoolClientOutput, SdkError<DeleteUserPoolClientError>>> {
        (*self).delete_user_pool_client(builder)
    }
    fn delete_user_pool_domain(&self, builder: DeleteUserPoolDomainInputBuilder) -> impl Future<Output = Result<DeleteUserPoolDomainOutput, SdkError<DeleteUserPoolDomainError>>> {
        (*self).delete_user_pool_domain(builder)
    }
    fn describe_identity_provider(&self, builder: DescribeIdentityProviderInputBuilder) -> impl Future<Output = Result<DescribeIdentityProviderOutput, SdkError<DescribeIdentityProviderError>>> {
        (*self).describe_identity_provider(builder)
    }
    fn describe_resource_server(&self, builder: DescribeResourceServerInputBuilder) -> impl Future<Output = Result<DescribeResourceServerOutput, SdkError<DescribeResourceServerError>>> {
        (*self).describe_resource_server(builder)
    }
    fn describe_risk_configuration(&self, builder: DescribeRiskConfigurationInputBuilder) -> impl Future<Output = Result<DescribeRiskConfigurationOutput, SdkError<DescribeRiskConfigurationError>>> {
        (*self).describe_risk_configuration(builder)
    }
    fn describe_user_import_job(&self, builder: DescribeUserImportJobInputBuilder) -> impl Future<Output = Result<DescribeUserImportJobOutput, SdkError<DescribeUserImportJobError>>> {
        (*self).describe_user_import_job(builder)
    }
    fn describe_user_pool(&self, builder: DescribeUserPoolInputBuilder) -> impl Future<Output = Result<DescribeUserPoolOutput, SdkError<DescribeUserPoolError>>> {
        (*self).describe_user_pool(builder)
    }
    fn describe_user_pool_client(&self, builder: DescribeUserPoolClientInputBuilder) -> impl Future<Output = Result<DescribeUserPoolClientOutput, SdkError<DescribeUserPoolClientError>>> {
        (*self).describe_user_pool_client(builder)
    }
    fn describe_user_pool_domain(&self, builder: DescribeUserPoolDomainInputBuilder) -> impl Future<Output = Result<DescribeUserPoolDomainOutput, SdkError<DescribeUserPoolDomainError>>> {
        (*self).describe_user_pool_domain(builder)
    }
    fn forget_device(&self, builder: ForgetDeviceInputBuilder) -> impl Future<Output = Result<ForgetDeviceOutput, SdkError<ForgetDeviceError>>> {
        (*self).forget_device(builder)
    }
    fn forgot_password(&self, builder: ForgotPasswordInputBuilder) -> impl Future<Output = Result<ForgotPasswordOutput, SdkError<ForgotPasswordError>>> {
        (*self).forgot_password(builder)
    }
    fn get_csv_header(&self, builder: GetCsvHeaderInputBuilder) -> impl Future<Output = Result<GetCsvHeaderOutput, SdkError<GetCSVHeaderError>>> {
        (*self).get_csv_header(builder)
    }
    fn get_device(&self, builder: GetDeviceInputBuilder) -> impl Future<Output = Result<GetDeviceOutput, SdkError<GetDeviceError>>> {
        (*self).get_device(builder)
    }
    fn get_group(&self, builder: GetGroupInputBuilder) -> impl Future<Output = Result<GetGroupOutput, SdkError<GetGroupError>>> {
        (*self).get_group(builder)
    }
    fn get_identity_provider_by_identifier(&self, builder: GetIdentityProviderByIdentifierInputBuilder) -> impl Future<Output = Result<GetIdentityProviderByIdentifierOutput, SdkError<GetIdentityProviderByIdentifierError>>> {
        (*self).get_identity_provider_by_identifier(builder)
    }
    fn get_log_delivery_configuration(&self, builder: GetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<GetLogDeliveryConfigurationOutput, SdkError<GetLogDeliveryConfigurationError>>> {
        (*self).get_log_delivery_configuration(builder)
    }
    fn get_signing_certificate(&self, builder: GetSigningCertificateInputBuilder) -> impl Future<Output = Result<GetSigningCertificateOutput, SdkError<GetSigningCertificateError>>> {
        (*self).get_signing_certificate(builder)
    }
    fn get_ui_customization(&self, builder: GetUiCustomizationInputBuilder) -> impl Future<Output = Result<GetUiCustomizationOutput, SdkError<GetUICustomizationError>>> {
        (*self).get_ui_customization(builder)
    }
    fn get_user(&self, builder: GetUserInputBuilder) -> impl Future<Output = Result<GetUserOutput, SdkError<GetUserError>>> {
        (*self).get_user(builder)
    }
    fn get_user_attribute_verification_code(&self, builder: GetUserAttributeVerificationCodeInputBuilder) -> impl Future<Output = Result<GetUserAttributeVerificationCodeOutput, SdkError<GetUserAttributeVerificationCodeError>>> {
        (*self).get_user_attribute_verification_code(builder)
    }
    fn get_user_pool_mfa_config(&self, builder: GetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<GetUserPoolMfaConfigOutput, SdkError<GetUserPoolMfaConfigError>>> {
        (*self).get_user_pool_mfa_config(builder)
    }
    fn global_sign_out(&self, builder: GlobalSignOutInputBuilder) -> impl Future<Output = Result<GlobalSignOutOutput, SdkError<GlobalSignOutError>>> {
        (*self).global_sign_out(builder)
    }
    fn initiate_auth(&self, builder: InitiateAuthInputBuilder) -> impl Future<Output = Result<InitiateAuthOutput, SdkError<InitiateAuthError>>> {
        (*self).initiate_auth(builder)
    }
    fn list_devices(&self, builder: ListDevicesInputBuilder) -> impl Future<Output = Result<ListDevicesOutput, SdkError<ListDevicesError>>> {
        (*self).list_devices(builder)
    }
    fn list_groups(&self, builder: ListGroupsInputBuilder) -> impl Future<Output = Result<ListGroupsOutput, SdkError<ListGroupsError>>> {
        (*self).list_groups(builder)
    }
    fn list_identity_providers(&self, builder: ListIdentityProvidersInputBuilder) -> impl Future<Output = Result<ListIdentityProvidersOutput, SdkError<ListIdentityProvidersError>>> {
        (*self).list_identity_providers(builder)
    }
    fn list_resource_servers(&self, builder: ListResourceServersInputBuilder) -> impl Future<Output = Result<ListResourceServersOutput, SdkError<ListResourceServersError>>> {
        (*self).list_resource_servers(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn list_user_import_jobs(&self, builder: ListUserImportJobsInputBuilder) -> impl Future<Output = Result<ListUserImportJobsOutput, SdkError<ListUserImportJobsError>>> {
        (*self).list_user_import_jobs(builder)
    }
    fn list_user_pool_clients(&self, builder: ListUserPoolClientsInputBuilder) -> impl Future<Output = Result<ListUserPoolClientsOutput, SdkError<ListUserPoolClientsError>>> {
        (*self).list_user_pool_clients(builder)
    }
    fn list_user_pools(&self, builder: ListUserPoolsInputBuilder) -> impl Future<Output = Result<ListUserPoolsOutput, SdkError<ListUserPoolsError>>> {
        (*self).list_user_pools(builder)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        (*self).list_users(builder)
    }
    fn list_users_in_group(&self, builder: ListUsersInGroupInputBuilder) -> impl Future<Output = Result<ListUsersInGroupOutput, SdkError<ListUsersInGroupError>>> {
        (*self).list_users_in_group(builder)
    }
    fn resend_confirmation_code(&self, builder: ResendConfirmationCodeInputBuilder) -> impl Future<Output = Result<ResendConfirmationCodeOutput, SdkError<ResendConfirmationCodeError>>> {
        (*self).resend_confirmation_code(builder)
    }
    fn respond_to_auth_challenge(&self, builder: RespondToAuthChallengeInputBuilder) -> impl Future<Output = Result<RespondToAuthChallengeOutput, SdkError<RespondToAuthChallengeError>>> {
        (*self).respond_to_auth_challenge(builder)
    }
    fn revoke_token(&self, builder: RevokeTokenInputBuilder) -> impl Future<Output = Result<RevokeTokenOutput, SdkError<RevokeTokenError>>> {
        (*self).revoke_token(builder)
    }
    fn set_log_delivery_configuration(&self, builder: SetLogDeliveryConfigurationInputBuilder) -> impl Future<Output = Result<SetLogDeliveryConfigurationOutput, SdkError<SetLogDeliveryConfigurationError>>> {
        (*self).set_log_delivery_configuration(builder)
    }
    fn set_risk_configuration(&self, builder: SetRiskConfigurationInputBuilder) -> impl Future<Output = Result<SetRiskConfigurationOutput, SdkError<SetRiskConfigurationError>>> {
        (*self).set_risk_configuration(builder)
    }
    fn set_ui_customization(&self, builder: SetUiCustomizationInputBuilder) -> impl Future<Output = Result<SetUiCustomizationOutput, SdkError<SetUICustomizationError>>> {
        (*self).set_ui_customization(builder)
    }
    fn set_user_mfa_preference(&self, builder: SetUserMfaPreferenceInputBuilder) -> impl Future<Output = Result<SetUserMfaPreferenceOutput, SdkError<SetUserMFAPreferenceError>>> {
        (*self).set_user_mfa_preference(builder)
    }
    fn set_user_pool_mfa_config(&self, builder: SetUserPoolMfaConfigInputBuilder) -> impl Future<Output = Result<SetUserPoolMfaConfigOutput, SdkError<SetUserPoolMfaConfigError>>> {
        (*self).set_user_pool_mfa_config(builder)
    }
    fn set_user_settings(&self, builder: SetUserSettingsInputBuilder) -> impl Future<Output = Result<SetUserSettingsOutput, SdkError<SetUserSettingsError>>> {
        (*self).set_user_settings(builder)
    }
    fn sign_up(&self, builder: SignUpInputBuilder) -> impl Future<Output = Result<SignUpOutput, SdkError<SignUpError>>> {
        (*self).sign_up(builder)
    }
    fn start_user_import_job(&self, builder: StartUserImportJobInputBuilder) -> impl Future<Output = Result<StartUserImportJobOutput, SdkError<StartUserImportJobError>>> {
        (*self).start_user_import_job(builder)
    }
    fn stop_user_import_job(&self, builder: StopUserImportJobInputBuilder) -> impl Future<Output = Result<StopUserImportJobOutput, SdkError<StopUserImportJobError>>> {
        (*self).stop_user_import_job(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_auth_event_feedback(&self, builder: UpdateAuthEventFeedbackInputBuilder) -> impl Future<Output = Result<UpdateAuthEventFeedbackOutput, SdkError<UpdateAuthEventFeedbackError>>> {
        (*self).update_auth_event_feedback(builder)
    }
    fn update_device_status(&self, builder: UpdateDeviceStatusInputBuilder) -> impl Future<Output = Result<UpdateDeviceStatusOutput, SdkError<UpdateDeviceStatusError>>> {
        (*self).update_device_status(builder)
    }
    fn update_group(&self, builder: UpdateGroupInputBuilder) -> impl Future<Output = Result<UpdateGroupOutput, SdkError<UpdateGroupError>>> {
        (*self).update_group(builder)
    }
    fn update_identity_provider(&self, builder: UpdateIdentityProviderInputBuilder) -> impl Future<Output = Result<UpdateIdentityProviderOutput, SdkError<UpdateIdentityProviderError>>> {
        (*self).update_identity_provider(builder)
    }
    fn update_resource_server(&self, builder: UpdateResourceServerInputBuilder) -> impl Future<Output = Result<UpdateResourceServerOutput, SdkError<UpdateResourceServerError>>> {
        (*self).update_resource_server(builder)
    }
    fn update_user_attributes(&self, builder: UpdateUserAttributesInputBuilder) -> impl Future<Output = Result<UpdateUserAttributesOutput, SdkError<UpdateUserAttributesError>>> {
        (*self).update_user_attributes(builder)
    }
    fn update_user_pool(&self, builder: UpdateUserPoolInputBuilder) -> impl Future<Output = Result<UpdateUserPoolOutput, SdkError<UpdateUserPoolError>>> {
        (*self).update_user_pool(builder)
    }
    fn update_user_pool_client(&self, builder: UpdateUserPoolClientInputBuilder) -> impl Future<Output = Result<UpdateUserPoolClientOutput, SdkError<UpdateUserPoolClientError>>> {
        (*self).update_user_pool_client(builder)
    }
    fn update_user_pool_domain(&self, builder: UpdateUserPoolDomainInputBuilder) -> impl Future<Output = Result<UpdateUserPoolDomainOutput, SdkError<UpdateUserPoolDomainError>>> {
        (*self).update_user_pool_domain(builder)
    }
    fn verify_software_token(&self, builder: VerifySoftwareTokenInputBuilder) -> impl Future<Output = Result<VerifySoftwareTokenOutput, SdkError<VerifySoftwareTokenError>>> {
        (*self).verify_software_token(builder)
    }
    fn verify_user_attribute(&self, builder: VerifyUserAttributeInputBuilder) -> impl Future<Output = Result<VerifyUserAttributeOutput, SdkError<VerifyUserAttributeError>>> {
        (*self).verify_user_attribute(builder)
    }
}
#[cfg(feature = "mocks")]
mockall::mock! {
    pub edCognitoIdentityProviderClient {}
    impl CognitoIdentityProviderClient for edCognitoIdentityProviderClient {
        async fn add_custom_attributes(&self, builder: AddCustomAttributesInputBuilder) -> Result<AddCustomAttributesOutput, SdkError<AddCustomAttributesError>>;
        async fn admin_add_user_to_group(&self, builder: AdminAddUserToGroupInputBuilder) -> Result<AdminAddUserToGroupOutput, SdkError<AdminAddUserToGroupError>>;
        async fn admin_confirm_sign_up(&self, builder: AdminConfirmSignUpInputBuilder) -> Result<AdminConfirmSignUpOutput, SdkError<AdminConfirmSignUpError>>;
        async fn admin_create_user(&self, builder: AdminCreateUserInputBuilder) -> Result<AdminCreateUserOutput, SdkError<AdminCreateUserError>>;
        async fn admin_delete_user(&self, builder: AdminDeleteUserInputBuilder) -> Result<AdminDeleteUserOutput, SdkError<AdminDeleteUserError>>;
        async fn admin_delete_user_attributes(&self, builder: AdminDeleteUserAttributesInputBuilder) -> Result<AdminDeleteUserAttributesOutput, SdkError<AdminDeleteUserAttributesError>>;
        async fn admin_disable_provider_for_user(&self, builder: AdminDisableProviderForUserInputBuilder) -> Result<AdminDisableProviderForUserOutput, SdkError<AdminDisableProviderForUserError>>;
        async fn admin_disable_user(&self, builder: AdminDisableUserInputBuilder) -> Result<AdminDisableUserOutput, SdkError<AdminDisableUserError>>;
        async fn admin_enable_user(&self, builder: AdminEnableUserInputBuilder) -> Result<AdminEnableUserOutput, SdkError<AdminEnableUserError>>;
        async fn admin_forget_device(&self, builder: AdminForgetDeviceInputBuilder) -> Result<AdminForgetDeviceOutput, SdkError<AdminForgetDeviceError>>;
        async fn admin_get_device(&self, builder: AdminGetDeviceInputBuilder) -> Result<AdminGetDeviceOutput, SdkError<AdminGetDeviceError>>;
        async fn admin_get_user(&self, builder: AdminGetUserInputBuilder) -> Result<AdminGetUserOutput, SdkError<AdminGetUserError>>;
        async fn admin_initiate_auth(&self, builder: AdminInitiateAuthInputBuilder) -> Result<AdminInitiateAuthOutput, SdkError<AdminInitiateAuthError>>;
        async fn admin_link_provider_for_user(&self, builder: AdminLinkProviderForUserInputBuilder) -> Result<AdminLinkProviderForUserOutput, SdkError<AdminLinkProviderForUserError>>;
        async fn admin_list_devices(&self, builder: AdminListDevicesInputBuilder) -> Result<AdminListDevicesOutput, SdkError<AdminListDevicesError>>;
        async fn admin_list_groups_for_user(&self, builder: AdminListGroupsForUserInputBuilder) -> Result<AdminListGroupsForUserOutput, SdkError<AdminListGroupsForUserError>>;
        async fn admin_list_user_auth_events(&self, builder: AdminListUserAuthEventsInputBuilder) -> Result<AdminListUserAuthEventsOutput, SdkError<AdminListUserAuthEventsError>>;
        async fn admin_remove_user_from_group(&self, builder: AdminRemoveUserFromGroupInputBuilder) -> Result<AdminRemoveUserFromGroupOutput, SdkError<AdminRemoveUserFromGroupError>>;
        async fn admin_reset_user_password(&self, builder: AdminResetUserPasswordInputBuilder) -> Result<AdminResetUserPasswordOutput, SdkError<AdminResetUserPasswordError>>;
        async fn admin_respond_to_auth_challenge(&self, builder: AdminRespondToAuthChallengeInputBuilder) -> Result<AdminRespondToAuthChallengeOutput, SdkError<AdminRespondToAuthChallengeError>>;
        async fn admin_set_user_mfa_preference(&self, builder: AdminSetUserMfaPreferenceInputBuilder) -> Result<AdminSetUserMfaPreferenceOutput, SdkError<AdminSetUserMFAPreferenceError>>;
        async fn admin_set_user_password(&self, builder: AdminSetUserPasswordInputBuilder) -> Result<AdminSetUserPasswordOutput, SdkError<AdminSetUserPasswordError>>;
        async fn admin_set_user_settings(&self, builder: AdminSetUserSettingsInputBuilder) -> Result<AdminSetUserSettingsOutput, SdkError<AdminSetUserSettingsError>>;
        async fn admin_update_auth_event_feedback(&self, builder: AdminUpdateAuthEventFeedbackInputBuilder) -> Result<AdminUpdateAuthEventFeedbackOutput, SdkError<AdminUpdateAuthEventFeedbackError>>;
        async fn admin_update_device_status(&self, builder: AdminUpdateDeviceStatusInputBuilder) -> Result<AdminUpdateDeviceStatusOutput, SdkError<AdminUpdateDeviceStatusError>>;
        async fn admin_update_user_attributes(&self, builder: AdminUpdateUserAttributesInputBuilder) -> Result<AdminUpdateUserAttributesOutput, SdkError<AdminUpdateUserAttributesError>>;
        async fn admin_user_global_sign_out(&self, builder: AdminUserGlobalSignOutInputBuilder) -> Result<AdminUserGlobalSignOutOutput, SdkError<AdminUserGlobalSignOutError>>;
        async fn associate_software_token(&self, builder: AssociateSoftwareTokenInputBuilder) -> Result<AssociateSoftwareTokenOutput, SdkError<AssociateSoftwareTokenError>>;
        async fn change_password(&self, builder: ChangePasswordInputBuilder) -> Result<ChangePasswordOutput, SdkError<ChangePasswordError>>;
        async fn confirm_device(&self, builder: ConfirmDeviceInputBuilder) -> Result<ConfirmDeviceOutput, SdkError<ConfirmDeviceError>>;
        async fn confirm_forgot_password(&self, builder: ConfirmForgotPasswordInputBuilder) -> Result<ConfirmForgotPasswordOutput, SdkError<ConfirmForgotPasswordError>>;
        async fn confirm_sign_up(&self, builder: ConfirmSignUpInputBuilder) -> Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>>;
        async fn create_group(&self, builder: CreateGroupInputBuilder) -> Result<CreateGroupOutput, SdkError<CreateGroupError>>;
        async fn create_identity_provider(&self, builder: CreateIdentityProviderInputBuilder) -> Result<CreateIdentityProviderOutput, SdkError<CreateIdentityProviderError>>;
        async fn create_resource_server(&self, builder: CreateResourceServerInputBuilder) -> Result<CreateResourceServerOutput, SdkError<CreateResourceServerError>>;
        async fn create_user_import_job(&self, builder: CreateUserImportJobInputBuilder) -> Result<CreateUserImportJobOutput, SdkError<CreateUserImportJobError>>;
        async fn create_user_pool(&self, builder: CreateUserPoolInputBuilder) -> Result<CreateUserPoolOutput, SdkError<CreateUserPoolError>>;
        async fn create_user_pool_client(&self, builder: CreateUserPoolClientInputBuilder) -> Result<CreateUserPoolClientOutput, SdkError<CreateUserPoolClientError>>;
        async fn create_user_pool_domain(&self, builder: CreateUserPoolDomainInputBuilder) -> Result<CreateUserPoolDomainOutput, SdkError<CreateUserPoolDomainError>>;
        async fn delete_group(&self, builder: DeleteGroupInputBuilder) -> Result<DeleteGroupOutput, SdkError<DeleteGroupError>>;
        async fn delete_identity_provider(&self, builder: DeleteIdentityProviderInputBuilder) -> Result<DeleteIdentityProviderOutput, SdkError<DeleteIdentityProviderError>>;
        async fn delete_resource_server(&self, builder: DeleteResourceServerInputBuilder) -> Result<DeleteResourceServerOutput, SdkError<DeleteResourceServerError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn delete_user_attributes(&self, builder: DeleteUserAttributesInputBuilder) -> Result<DeleteUserAttributesOutput, SdkError<DeleteUserAttributesError>>;
        async fn delete_user_pool(&self, builder: DeleteUserPoolInputBuilder) -> Result<DeleteUserPoolOutput, SdkError<DeleteUserPoolError>>;
        async fn delete_user_pool_client(&self, builder: DeleteUserPoolClientInputBuilder) -> Result<DeleteUserPoolClientOutput, SdkError<DeleteUserPoolClientError>>;
        async fn delete_user_pool_domain(&self, builder: DeleteUserPoolDomainInputBuilder) -> Result<DeleteUserPoolDomainOutput, SdkError<DeleteUserPoolDomainError>>;
        async fn describe_identity_provider(&self, builder: DescribeIdentityProviderInputBuilder) -> Result<DescribeIdentityProviderOutput, SdkError<DescribeIdentityProviderError>>;
        async fn describe_resource_server(&self, builder: DescribeResourceServerInputBuilder) -> Result<DescribeResourceServerOutput, SdkError<DescribeResourceServerError>>;
        async fn describe_risk_configuration(&self, builder: DescribeRiskConfigurationInputBuilder) -> Result<DescribeRiskConfigurationOutput, SdkError<DescribeRiskConfigurationError>>;
        async fn describe_user_import_job(&self, builder: DescribeUserImportJobInputBuilder) -> Result<DescribeUserImportJobOutput, SdkError<DescribeUserImportJobError>>;
        async fn describe_user_pool(&self, builder: DescribeUserPoolInputBuilder) -> Result<DescribeUserPoolOutput, SdkError<DescribeUserPoolError>>;
        async fn describe_user_pool_client(&self, builder: DescribeUserPoolClientInputBuilder) -> Result<DescribeUserPoolClientOutput, SdkError<DescribeUserPoolClientError>>;
        async fn describe_user_pool_domain(&self, builder: DescribeUserPoolDomainInputBuilder) -> Result<DescribeUserPoolDomainOutput, SdkError<DescribeUserPoolDomainError>>;
        async fn forget_device(&self, builder: ForgetDeviceInputBuilder) -> Result<ForgetDeviceOutput, SdkError<ForgetDeviceError>>;
        async fn forgot_password(&self, builder: ForgotPasswordInputBuilder) -> Result<ForgotPasswordOutput, SdkError<ForgotPasswordError>>;
        async fn get_csv_header(&self, builder: GetCsvHeaderInputBuilder) -> Result<GetCsvHeaderOutput, SdkError<GetCSVHeaderError>>;
        async fn get_device(&self, builder: GetDeviceInputBuilder) -> Result<GetDeviceOutput, SdkError<GetDeviceError>>;
        async fn get_group(&self, builder: GetGroupInputBuilder) -> Result<GetGroupOutput, SdkError<GetGroupError>>;
        async fn get_identity_provider_by_identifier(&self, builder: GetIdentityProviderByIdentifierInputBuilder) -> Result<GetIdentityProviderByIdentifierOutput, SdkError<GetIdentityProviderByIdentifierError>>;
        async fn get_log_delivery_configuration(&self, builder: GetLogDeliveryConfigurationInputBuilder) -> Result<GetLogDeliveryConfigurationOutput, SdkError<GetLogDeliveryConfigurationError>>;
        async fn get_signing_certificate(&self, builder: GetSigningCertificateInputBuilder) -> Result<GetSigningCertificateOutput, SdkError<GetSigningCertificateError>>;
        async fn get_ui_customization(&self, builder: GetUiCustomizationInputBuilder) -> Result<GetUiCustomizationOutput, SdkError<GetUICustomizationError>>;
        async fn get_user(&self, builder: GetUserInputBuilder) -> Result<GetUserOutput, SdkError<GetUserError>>;
        async fn get_user_attribute_verification_code(&self, builder: GetUserAttributeVerificationCodeInputBuilder) -> Result<GetUserAttributeVerificationCodeOutput, SdkError<GetUserAttributeVerificationCodeError>>;
        async fn get_user_pool_mfa_config(&self, builder: GetUserPoolMfaConfigInputBuilder) -> Result<GetUserPoolMfaConfigOutput, SdkError<GetUserPoolMfaConfigError>>;
        async fn global_sign_out(&self, builder: GlobalSignOutInputBuilder) -> Result<GlobalSignOutOutput, SdkError<GlobalSignOutError>>;
        async fn initiate_auth(&self, builder: InitiateAuthInputBuilder) -> Result<InitiateAuthOutput, SdkError<InitiateAuthError>>;
        async fn list_devices(&self, builder: ListDevicesInputBuilder) -> Result<ListDevicesOutput, SdkError<ListDevicesError>>;
        async fn list_groups(&self, builder: ListGroupsInputBuilder) -> Result<ListGroupsOutput, SdkError<ListGroupsError>>;
        async fn list_identity_providers(&self, builder: ListIdentityProvidersInputBuilder) -> Result<ListIdentityProvidersOutput, SdkError<ListIdentityProvidersError>>;
        async fn list_resource_servers(&self, builder: ListResourceServersInputBuilder) -> Result<ListResourceServersOutput, SdkError<ListResourceServersError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_user_import_jobs(&self, builder: ListUserImportJobsInputBuilder) -> Result<ListUserImportJobsOutput, SdkError<ListUserImportJobsError>>;
        async fn list_user_pool_clients(&self, builder: ListUserPoolClientsInputBuilder) -> Result<ListUserPoolClientsOutput, SdkError<ListUserPoolClientsError>>;
        async fn list_user_pools(&self, builder: ListUserPoolsInputBuilder) -> Result<ListUserPoolsOutput, SdkError<ListUserPoolsError>>;
        async fn list_users(&self, builder: ListUsersInputBuilder) -> Result<ListUsersOutput, SdkError<ListUsersError>>;
        async fn list_users_in_group(&self, builder: ListUsersInGroupInputBuilder) -> Result<ListUsersInGroupOutput, SdkError<ListUsersInGroupError>>;
        async fn resend_confirmation_code(&self, builder: ResendConfirmationCodeInputBuilder) -> Result<ResendConfirmationCodeOutput, SdkError<ResendConfirmationCodeError>>;
        async fn respond_to_auth_challenge(&self, builder: RespondToAuthChallengeInputBuilder) -> Result<RespondToAuthChallengeOutput, SdkError<RespondToAuthChallengeError>>;
        async fn revoke_token(&self, builder: RevokeTokenInputBuilder) -> Result<RevokeTokenOutput, SdkError<RevokeTokenError>>;
        async fn set_log_delivery_configuration(&self, builder: SetLogDeliveryConfigurationInputBuilder) -> Result<SetLogDeliveryConfigurationOutput, SdkError<SetLogDeliveryConfigurationError>>;
        async fn set_risk_configuration(&self, builder: SetRiskConfigurationInputBuilder) -> Result<SetRiskConfigurationOutput, SdkError<SetRiskConfigurationError>>;
        async fn set_ui_customization(&self, builder: SetUiCustomizationInputBuilder) -> Result<SetUiCustomizationOutput, SdkError<SetUICustomizationError>>;
        async fn set_user_mfa_preference(&self, builder: SetUserMfaPreferenceInputBuilder) -> Result<SetUserMfaPreferenceOutput, SdkError<SetUserMFAPreferenceError>>;
        async fn set_user_pool_mfa_config(&self, builder: SetUserPoolMfaConfigInputBuilder) -> Result<SetUserPoolMfaConfigOutput, SdkError<SetUserPoolMfaConfigError>>;
        async fn set_user_settings(&self, builder: SetUserSettingsInputBuilder) -> Result<SetUserSettingsOutput, SdkError<SetUserSettingsError>>;
        async fn sign_up(&self, builder: SignUpInputBuilder) -> Result<SignUpOutput, SdkError<SignUpError>>;
        async fn start_user_import_job(&self, builder: StartUserImportJobInputBuilder) -> Result<StartUserImportJobOutput, SdkError<StartUserImportJobError>>;
        async fn stop_user_import_job(&self, builder: StopUserImportJobInputBuilder) -> Result<StopUserImportJobOutput, SdkError<StopUserImportJobError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_auth_event_feedback(&self, builder: UpdateAuthEventFeedbackInputBuilder) -> Result<UpdateAuthEventFeedbackOutput, SdkError<UpdateAuthEventFeedbackError>>;
        async fn update_device_status(&self, builder: UpdateDeviceStatusInputBuilder) -> Result<UpdateDeviceStatusOutput, SdkError<UpdateDeviceStatusError>>;
        async fn update_group(&self, builder: UpdateGroupInputBuilder) -> Result<UpdateGroupOutput, SdkError<UpdateGroupError>>;
        async fn update_identity_provider(&self, builder: UpdateIdentityProviderInputBuilder) -> Result<UpdateIdentityProviderOutput, SdkError<UpdateIdentityProviderError>>;
        async fn update_resource_server(&self, builder: UpdateResourceServerInputBuilder) -> Result<UpdateResourceServerOutput, SdkError<UpdateResourceServerError>>;
        async fn update_user_attributes(&self, builder: UpdateUserAttributesInputBuilder) -> Result<UpdateUserAttributesOutput, SdkError<UpdateUserAttributesError>>;
        async fn update_user_pool(&self, builder: UpdateUserPoolInputBuilder) -> Result<UpdateUserPoolOutput, SdkError<UpdateUserPoolError>>;
        async fn update_user_pool_client(&self, builder: UpdateUserPoolClientInputBuilder) -> Result<UpdateUserPoolClientOutput, SdkError<UpdateUserPoolClientError>>;
        async fn update_user_pool_domain(&self, builder: UpdateUserPoolDomainInputBuilder) -> Result<UpdateUserPoolDomainOutput, SdkError<UpdateUserPoolDomainError>>;
        async fn verify_software_token(&self, builder: VerifySoftwareTokenInputBuilder) -> Result<VerifySoftwareTokenOutput, SdkError<VerifySoftwareTokenError>>;
        async fn verify_user_attribute(&self, builder: VerifyUserAttributeInputBuilder) -> Result<VerifyUserAttributeOutput, SdkError<VerifyUserAttributeError>>;
    }
}
