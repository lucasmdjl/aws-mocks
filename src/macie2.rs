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
use aws_sdk_macie2::operation::accept_invitation::{builders::*, *};
use aws_sdk_macie2::operation::batch_get_custom_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::batch_update_automated_discovery_accounts::{builders::*, *};
use aws_sdk_macie2::operation::create_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::create_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::create_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::create_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::create_invitations::{builders::*, *};
use aws_sdk_macie2::operation::create_member::{builders::*, *};
use aws_sdk_macie2::operation::create_sample_findings::{builders::*, *};
use aws_sdk_macie2::operation::decline_invitations::{builders::*, *};
use aws_sdk_macie2::operation::delete_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::delete_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::delete_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::delete_invitations::{builders::*, *};
use aws_sdk_macie2::operation::delete_member::{builders::*, *};
use aws_sdk_macie2::operation::describe_buckets::{builders::*, *};
use aws_sdk_macie2::operation::describe_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::describe_organization_configuration::{builders::*, *};
use aws_sdk_macie2::operation::disable_macie::{builders::*, *};
use aws_sdk_macie2::operation::disable_organization_admin_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_from_administrator_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_from_master_account::{builders::*, *};
use aws_sdk_macie2::operation::disassociate_member::{builders::*, *};
use aws_sdk_macie2::operation::enable_macie::{builders::*, *};
use aws_sdk_macie2::operation::enable_organization_admin_account::{builders::*, *};
use aws_sdk_macie2::operation::get_administrator_account::{builders::*, *};
use aws_sdk_macie2::operation::get_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::get_automated_discovery_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_bucket_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_classification_export_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_classification_scope::{builders::*, *};
use aws_sdk_macie2::operation::get_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::get_finding_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_findings::{builders::*, *};
use aws_sdk_macie2::operation::get_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::get_findings_publication_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_invitations_count::{builders::*, *};
use aws_sdk_macie2::operation::get_macie_session::{builders::*, *};
use aws_sdk_macie2::operation::get_master_account::{builders::*, *};
use aws_sdk_macie2::operation::get_member::{builders::*, *};
use aws_sdk_macie2::operation::get_resource_profile::{builders::*, *};
use aws_sdk_macie2::operation::get_reveal_configuration::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitive_data_occurrences::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitive_data_occurrences_availability::{builders::*, *};
use aws_sdk_macie2::operation::get_sensitivity_inspection_template::{builders::*, *};
use aws_sdk_macie2::operation::get_usage_statistics::{builders::*, *};
use aws_sdk_macie2::operation::get_usage_totals::{builders::*, *};
use aws_sdk_macie2::operation::list_allow_lists::{builders::*, *};
use aws_sdk_macie2::operation::list_automated_discovery_accounts::{builders::*, *};
use aws_sdk_macie2::operation::list_classification_jobs::{builders::*, *};
use aws_sdk_macie2::operation::list_classification_scopes::{builders::*, *};
use aws_sdk_macie2::operation::list_custom_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::list_findings::{builders::*, *};
use aws_sdk_macie2::operation::list_findings_filters::{builders::*, *};
use aws_sdk_macie2::operation::list_invitations::{builders::*, *};
use aws_sdk_macie2::operation::list_managed_data_identifiers::{builders::*, *};
use aws_sdk_macie2::operation::list_members::{builders::*, *};
use aws_sdk_macie2::operation::list_organization_admin_accounts::{builders::*, *};
use aws_sdk_macie2::operation::list_resource_profile_artifacts::{builders::*, *};
use aws_sdk_macie2::operation::list_resource_profile_detections::{builders::*, *};
use aws_sdk_macie2::operation::list_sensitivity_inspection_templates::{builders::*, *};
use aws_sdk_macie2::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_macie2::operation::put_classification_export_configuration::{builders::*, *};
use aws_sdk_macie2::operation::put_findings_publication_configuration::{builders::*, *};
use aws_sdk_macie2::operation::search_resources::{builders::*, *};
use aws_sdk_macie2::operation::tag_resource::{builders::*, *};
use aws_sdk_macie2::operation::test_custom_data_identifier::{builders::*, *};
use aws_sdk_macie2::operation::untag_resource::{builders::*, *};
use aws_sdk_macie2::operation::update_allow_list::{builders::*, *};
use aws_sdk_macie2::operation::update_automated_discovery_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_classification_job::{builders::*, *};
use aws_sdk_macie2::operation::update_classification_scope::{builders::*, *};
use aws_sdk_macie2::operation::update_findings_filter::{builders::*, *};
use aws_sdk_macie2::operation::update_macie_session::{builders::*, *};
use aws_sdk_macie2::operation::update_member_session::{builders::*, *};
use aws_sdk_macie2::operation::update_organization_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_resource_profile::{builders::*, *};
use aws_sdk_macie2::operation::update_resource_profile_detections::{builders::*, *};
use aws_sdk_macie2::operation::update_reveal_configuration::{builders::*, *};
use aws_sdk_macie2::operation::update_sensitivity_inspection_template::{builders::*, *};
use aws_sdk_macie2::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_macie2::Client;

pub use aws_sdk_macie2::*;

pub struct Macie2ClientImpl(Client);
impl Macie2ClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait Macie2Client {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>>;
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>>;
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>>;
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>>;
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>>;
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>>;
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>>;
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>>;
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>>;
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>>;
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>>;
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>>;
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>>;
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>>;
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>>;
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>>;
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>>;
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>>;
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>>;
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>>;
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>>;
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>>;
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>>;
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>>;
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>>;
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>>;
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>>;
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>>;
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>>;
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>>;
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>>;
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>>;
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>>;
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>>;
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>>;
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>>;
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>>;
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>>;
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>>;
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>>;
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>>;
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>>;
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>>;
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>>;
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>>;
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>>;
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>>;
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>>;
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>>;
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>>;
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>>;
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>>;
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>>;
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>>;
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>>;
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>>;
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>>;
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>>;
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>>;
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>>;
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>>;
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>>;
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>>;
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>>;
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>>;
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>>;
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>>;
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>>;
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>>;
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>>;
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>>;
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>>;
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>>;
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>>;
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>>;
}
impl Macie2Client for Macie2ClientImpl {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>> {
        builder.send_with(&self.0)
    }
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>> {
        builder.send_with(&self.0)
    }
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>> {
        builder.send_with(&self.0)
    }
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>> {
        builder.send_with(&self.0)
    }
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>> {
        builder.send_with(&self.0)
    }
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>> {
        builder.send_with(&self.0)
    }
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>> {
        builder.send_with(&self.0)
    }
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>> {
        builder.send_with(&self.0)
    }
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>> {
        builder.send_with(&self.0)
    }
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>> {
        builder.send_with(&self.0)
    }
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>> {
        builder.send_with(&self.0)
    }
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>> {
        builder.send_with(&self.0)
    }
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>> {
        builder.send_with(&self.0)
    }
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>> {
        builder.send_with(&self.0)
    }
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>> {
        builder.send_with(&self.0)
    }
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        builder.send_with(&self.0)
    }
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>> {
        builder.send_with(&self.0)
    }
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>> {
        builder.send_with(&self.0)
    }
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>> {
        builder.send_with(&self.0)
    }
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>> {
        builder.send_with(&self.0)
    }
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>> {
        builder.send_with(&self.0)
    }
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>> {
        builder.send_with(&self.0)
    }
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>> {
        builder.send_with(&self.0)
    }
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>> {
        builder.send_with(&self.0)
    }
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>> {
        builder.send_with(&self.0)
    }
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>> {
        builder.send_with(&self.0)
    }
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>> {
        builder.send_with(&self.0)
    }
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>> {
        builder.send_with(&self.0)
    }
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: Macie2Client> Macie2Client for &T {
    fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> impl Future<Output = Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>> {
        (*self).accept_invitation(builder)
    }
    fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>> {
        (*self).batch_get_custom_data_identifiers(builder)
    }
    fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>> {
        (*self).batch_update_automated_discovery_accounts(builder)
    }
    fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> impl Future<Output = Result<CreateAllowListOutput, SdkError<CreateAllowListError>>> {
        (*self).create_allow_list(builder)
    }
    fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> impl Future<Output = Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>> {
        (*self).create_classification_job(builder)
    }
    fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>> {
        (*self).create_custom_data_identifier(builder)
    }
    fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> impl Future<Output = Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>> {
        (*self).create_findings_filter(builder)
    }
    fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> impl Future<Output = Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>> {
        (*self).create_invitations(builder)
    }
    fn create_member(&self, builder: CreateMemberInputBuilder) -> impl Future<Output = Result<CreateMemberOutput, SdkError<CreateMemberError>>> {
        (*self).create_member(builder)
    }
    fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> impl Future<Output = Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>> {
        (*self).create_sample_findings(builder)
    }
    fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> impl Future<Output = Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>> {
        (*self).decline_invitations(builder)
    }
    fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> impl Future<Output = Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>> {
        (*self).delete_allow_list(builder)
    }
    fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>> {
        (*self).delete_custom_data_identifier(builder)
    }
    fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> impl Future<Output = Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>> {
        (*self).delete_findings_filter(builder)
    }
    fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> impl Future<Output = Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>> {
        (*self).delete_invitations(builder)
    }
    fn delete_member(&self, builder: DeleteMemberInputBuilder) -> impl Future<Output = Result<DeleteMemberOutput, SdkError<DeleteMemberError>>> {
        (*self).delete_member(builder)
    }
    fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> impl Future<Output = Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>> {
        (*self).describe_buckets(builder)
    }
    fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> impl Future<Output = Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>> {
        (*self).describe_classification_job(builder)
    }
    fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>> {
        (*self).describe_organization_configuration(builder)
    }
    fn disable_macie(&self, builder: DisableMacieInputBuilder) -> impl Future<Output = Result<DisableMacieOutput, SdkError<DisableMacieError>>> {
        (*self).disable_macie(builder)
    }
    fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>> {
        (*self).disable_organization_admin_account(builder)
    }
    fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>> {
        (*self).disassociate_from_administrator_account(builder)
    }
    fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> impl Future<Output = Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>> {
        (*self).disassociate_from_master_account(builder)
    }
    fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> impl Future<Output = Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>> {
        (*self).disassociate_member(builder)
    }
    fn enable_macie(&self, builder: EnableMacieInputBuilder) -> impl Future<Output = Result<EnableMacieOutput, SdkError<EnableMacieError>>> {
        (*self).enable_macie(builder)
    }
    fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> impl Future<Output = Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>> {
        (*self).enable_organization_admin_account(builder)
    }
    fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> impl Future<Output = Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>> {
        (*self).get_administrator_account(builder)
    }
    fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> impl Future<Output = Result<GetAllowListOutput, SdkError<GetAllowListError>>> {
        (*self).get_allow_list(builder)
    }
    fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>> {
        (*self).get_automated_discovery_configuration(builder)
    }
    fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> impl Future<Output = Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>> {
        (*self).get_bucket_statistics(builder)
    }
    fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>> {
        (*self).get_classification_export_configuration(builder)
    }
    fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> impl Future<Output = Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>> {
        (*self).get_classification_scope(builder)
    }
    fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>> {
        (*self).get_custom_data_identifier(builder)
    }
    fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> impl Future<Output = Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>> {
        (*self).get_finding_statistics(builder)
    }
    fn get_findings(&self, builder: GetFindingsInputBuilder) -> impl Future<Output = Result<GetFindingsOutput, SdkError<GetFindingsError>>> {
        (*self).get_findings(builder)
    }
    fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> impl Future<Output = Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>> {
        (*self).get_findings_filter(builder)
    }
    fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>> {
        (*self).get_findings_publication_configuration(builder)
    }
    fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> impl Future<Output = Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>> {
        (*self).get_invitations_count(builder)
    }
    fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> impl Future<Output = Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>> {
        (*self).get_macie_session(builder)
    }
    fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> impl Future<Output = Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>> {
        (*self).get_master_account(builder)
    }
    fn get_member(&self, builder: GetMemberInputBuilder) -> impl Future<Output = Result<GetMemberOutput, SdkError<GetMemberError>>> {
        (*self).get_member(builder)
    }
    fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> impl Future<Output = Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>> {
        (*self).get_resource_profile(builder)
    }
    fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> impl Future<Output = Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>> {
        (*self).get_reveal_configuration(builder)
    }
    fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>> {
        (*self).get_sensitive_data_occurrences(builder)
    }
    fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> impl Future<Output = Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>> {
        (*self).get_sensitive_data_occurrences_availability(builder)
    }
    fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>> {
        (*self).get_sensitivity_inspection_template(builder)
    }
    fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> impl Future<Output = Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>> {
        (*self).get_usage_statistics(builder)
    }
    fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> impl Future<Output = Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>> {
        (*self).get_usage_totals(builder)
    }
    fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> impl Future<Output = Result<ListAllowListsOutput, SdkError<ListAllowListsError>>> {
        (*self).list_allow_lists(builder)
    }
    fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> impl Future<Output = Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>> {
        (*self).list_automated_discovery_accounts(builder)
    }
    fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> impl Future<Output = Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>> {
        (*self).list_classification_jobs(builder)
    }
    fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> impl Future<Output = Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>> {
        (*self).list_classification_scopes(builder)
    }
    fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>> {
        (*self).list_custom_data_identifiers(builder)
    }
    fn list_findings(&self, builder: ListFindingsInputBuilder) -> impl Future<Output = Result<ListFindingsOutput, SdkError<ListFindingsError>>> {
        (*self).list_findings(builder)
    }
    fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> impl Future<Output = Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>> {
        (*self).list_findings_filters(builder)
    }
    fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> impl Future<Output = Result<ListInvitationsOutput, SdkError<ListInvitationsError>>> {
        (*self).list_invitations(builder)
    }
    fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> impl Future<Output = Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>> {
        (*self).list_managed_data_identifiers(builder)
    }
    fn list_members(&self, builder: ListMembersInputBuilder) -> impl Future<Output = Result<ListMembersOutput, SdkError<ListMembersError>>> {
        (*self).list_members(builder)
    }
    fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> impl Future<Output = Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>> {
        (*self).list_organization_admin_accounts(builder)
    }
    fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> impl Future<Output = Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>> {
        (*self).list_resource_profile_artifacts(builder)
    }
    fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>> {
        (*self).list_resource_profile_detections(builder)
    }
    fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> impl Future<Output = Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>> {
        (*self).list_sensitivity_inspection_templates(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> impl Future<Output = Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>> {
        (*self).put_classification_export_configuration(builder)
    }
    fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> impl Future<Output = Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>> {
        (*self).put_findings_publication_configuration(builder)
    }
    fn search_resources(&self, builder: SearchResourcesInputBuilder) -> impl Future<Output = Result<SearchResourcesOutput, SdkError<SearchResourcesError>>> {
        (*self).search_resources(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> impl Future<Output = Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>> {
        (*self).test_custom_data_identifier(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> impl Future<Output = Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>> {
        (*self).update_allow_list(builder)
    }
    fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> impl Future<Output = Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>> {
        (*self).update_automated_discovery_configuration(builder)
    }
    fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> impl Future<Output = Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>> {
        (*self).update_classification_job(builder)
    }
    fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> impl Future<Output = Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>> {
        (*self).update_classification_scope(builder)
    }
    fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> impl Future<Output = Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>> {
        (*self).update_findings_filter(builder)
    }
    fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> impl Future<Output = Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>> {
        (*self).update_macie_session(builder)
    }
    fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> impl Future<Output = Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>> {
        (*self).update_member_session(builder)
    }
    fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> impl Future<Output = Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>> {
        (*self).update_organization_configuration(builder)
    }
    fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>> {
        (*self).update_resource_profile(builder)
    }
    fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> impl Future<Output = Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>> {
        (*self).update_resource_profile_detections(builder)
    }
    fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> impl Future<Output = Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>> {
        (*self).update_reveal_configuration(builder)
    }
    fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> impl Future<Output = Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>> {
        (*self).update_sensitivity_inspection_template(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edMacie2Client {}
    impl Macie2Client for edMacie2Client {
        async fn accept_invitation(&self, builder: AcceptInvitationInputBuilder) -> Result<AcceptInvitationOutput, SdkError<AcceptInvitationError>>;
        async fn batch_get_custom_data_identifiers(&self, builder: BatchGetCustomDataIdentifiersInputBuilder) -> Result<BatchGetCustomDataIdentifiersOutput, SdkError<BatchGetCustomDataIdentifiersError>>;
        async fn batch_update_automated_discovery_accounts(&self, builder: BatchUpdateAutomatedDiscoveryAccountsInputBuilder) -> Result<BatchUpdateAutomatedDiscoveryAccountsOutput, SdkError<BatchUpdateAutomatedDiscoveryAccountsError>>;
        async fn create_allow_list(&self, builder: CreateAllowListInputBuilder) -> Result<CreateAllowListOutput, SdkError<CreateAllowListError>>;
        async fn create_classification_job(&self, builder: CreateClassificationJobInputBuilder) -> Result<CreateClassificationJobOutput, SdkError<CreateClassificationJobError>>;
        async fn create_custom_data_identifier(&self, builder: CreateCustomDataIdentifierInputBuilder) -> Result<CreateCustomDataIdentifierOutput, SdkError<CreateCustomDataIdentifierError>>;
        async fn create_findings_filter(&self, builder: CreateFindingsFilterInputBuilder) -> Result<CreateFindingsFilterOutput, SdkError<CreateFindingsFilterError>>;
        async fn create_invitations(&self, builder: CreateInvitationsInputBuilder) -> Result<CreateInvitationsOutput, SdkError<CreateInvitationsError>>;
        async fn create_member(&self, builder: CreateMemberInputBuilder) -> Result<CreateMemberOutput, SdkError<CreateMemberError>>;
        async fn create_sample_findings(&self, builder: CreateSampleFindingsInputBuilder) -> Result<CreateSampleFindingsOutput, SdkError<CreateSampleFindingsError>>;
        async fn decline_invitations(&self, builder: DeclineInvitationsInputBuilder) -> Result<DeclineInvitationsOutput, SdkError<DeclineInvitationsError>>;
        async fn delete_allow_list(&self, builder: DeleteAllowListInputBuilder) -> Result<DeleteAllowListOutput, SdkError<DeleteAllowListError>>;
        async fn delete_custom_data_identifier(&self, builder: DeleteCustomDataIdentifierInputBuilder) -> Result<DeleteCustomDataIdentifierOutput, SdkError<DeleteCustomDataIdentifierError>>;
        async fn delete_findings_filter(&self, builder: DeleteFindingsFilterInputBuilder) -> Result<DeleteFindingsFilterOutput, SdkError<DeleteFindingsFilterError>>;
        async fn delete_invitations(&self, builder: DeleteInvitationsInputBuilder) -> Result<DeleteInvitationsOutput, SdkError<DeleteInvitationsError>>;
        async fn delete_member(&self, builder: DeleteMemberInputBuilder) -> Result<DeleteMemberOutput, SdkError<DeleteMemberError>>;
        async fn describe_buckets(&self, builder: DescribeBucketsInputBuilder) -> Result<DescribeBucketsOutput, SdkError<DescribeBucketsError>>;
        async fn describe_classification_job(&self, builder: DescribeClassificationJobInputBuilder) -> Result<DescribeClassificationJobOutput, SdkError<DescribeClassificationJobError>>;
        async fn describe_organization_configuration(&self, builder: DescribeOrganizationConfigurationInputBuilder) -> Result<DescribeOrganizationConfigurationOutput, SdkError<DescribeOrganizationConfigurationError>>;
        async fn disable_macie(&self, builder: DisableMacieInputBuilder) -> Result<DisableMacieOutput, SdkError<DisableMacieError>>;
        async fn disable_organization_admin_account(&self, builder: DisableOrganizationAdminAccountInputBuilder) -> Result<DisableOrganizationAdminAccountOutput, SdkError<DisableOrganizationAdminAccountError>>;
        async fn disassociate_from_administrator_account(&self, builder: DisassociateFromAdministratorAccountInputBuilder) -> Result<DisassociateFromAdministratorAccountOutput, SdkError<DisassociateFromAdministratorAccountError>>;
        async fn disassociate_from_master_account(&self, builder: DisassociateFromMasterAccountInputBuilder) -> Result<DisassociateFromMasterAccountOutput, SdkError<DisassociateFromMasterAccountError>>;
        async fn disassociate_member(&self, builder: DisassociateMemberInputBuilder) -> Result<DisassociateMemberOutput, SdkError<DisassociateMemberError>>;
        async fn enable_macie(&self, builder: EnableMacieInputBuilder) -> Result<EnableMacieOutput, SdkError<EnableMacieError>>;
        async fn enable_organization_admin_account(&self, builder: EnableOrganizationAdminAccountInputBuilder) -> Result<EnableOrganizationAdminAccountOutput, SdkError<EnableOrganizationAdminAccountError>>;
        async fn get_administrator_account(&self, builder: GetAdministratorAccountInputBuilder) -> Result<GetAdministratorAccountOutput, SdkError<GetAdministratorAccountError>>;
        async fn get_allow_list(&self, builder: GetAllowListInputBuilder) -> Result<GetAllowListOutput, SdkError<GetAllowListError>>;
        async fn get_automated_discovery_configuration(&self, builder: GetAutomatedDiscoveryConfigurationInputBuilder) -> Result<GetAutomatedDiscoveryConfigurationOutput, SdkError<GetAutomatedDiscoveryConfigurationError>>;
        async fn get_bucket_statistics(&self, builder: GetBucketStatisticsInputBuilder) -> Result<GetBucketStatisticsOutput, SdkError<GetBucketStatisticsError>>;
        async fn get_classification_export_configuration(&self, builder: GetClassificationExportConfigurationInputBuilder) -> Result<GetClassificationExportConfigurationOutput, SdkError<GetClassificationExportConfigurationError>>;
        async fn get_classification_scope(&self, builder: GetClassificationScopeInputBuilder) -> Result<GetClassificationScopeOutput, SdkError<GetClassificationScopeError>>;
        async fn get_custom_data_identifier(&self, builder: GetCustomDataIdentifierInputBuilder) -> Result<GetCustomDataIdentifierOutput, SdkError<GetCustomDataIdentifierError>>;
        async fn get_finding_statistics(&self, builder: GetFindingStatisticsInputBuilder) -> Result<GetFindingStatisticsOutput, SdkError<GetFindingStatisticsError>>;
        async fn get_findings(&self, builder: GetFindingsInputBuilder) -> Result<GetFindingsOutput, SdkError<GetFindingsError>>;
        async fn get_findings_filter(&self, builder: GetFindingsFilterInputBuilder) -> Result<GetFindingsFilterOutput, SdkError<GetFindingsFilterError>>;
        async fn get_findings_publication_configuration(&self, builder: GetFindingsPublicationConfigurationInputBuilder) -> Result<GetFindingsPublicationConfigurationOutput, SdkError<GetFindingsPublicationConfigurationError>>;
        async fn get_invitations_count(&self, builder: GetInvitationsCountInputBuilder) -> Result<GetInvitationsCountOutput, SdkError<GetInvitationsCountError>>;
        async fn get_macie_session(&self, builder: GetMacieSessionInputBuilder) -> Result<GetMacieSessionOutput, SdkError<GetMacieSessionError>>;
        async fn get_master_account(&self, builder: GetMasterAccountInputBuilder) -> Result<GetMasterAccountOutput, SdkError<GetMasterAccountError>>;
        async fn get_member(&self, builder: GetMemberInputBuilder) -> Result<GetMemberOutput, SdkError<GetMemberError>>;
        async fn get_resource_profile(&self, builder: GetResourceProfileInputBuilder) -> Result<GetResourceProfileOutput, SdkError<GetResourceProfileError>>;
        async fn get_reveal_configuration(&self, builder: GetRevealConfigurationInputBuilder) -> Result<GetRevealConfigurationOutput, SdkError<GetRevealConfigurationError>>;
        async fn get_sensitive_data_occurrences(&self, builder: GetSensitiveDataOccurrencesInputBuilder) -> Result<GetSensitiveDataOccurrencesOutput, SdkError<GetSensitiveDataOccurrencesError>>;
        async fn get_sensitive_data_occurrences_availability(&self, builder: GetSensitiveDataOccurrencesAvailabilityInputBuilder) -> Result<GetSensitiveDataOccurrencesAvailabilityOutput, SdkError<GetSensitiveDataOccurrencesAvailabilityError>>;
        async fn get_sensitivity_inspection_template(&self, builder: GetSensitivityInspectionTemplateInputBuilder) -> Result<GetSensitivityInspectionTemplateOutput, SdkError<GetSensitivityInspectionTemplateError>>;
        async fn get_usage_statistics(&self, builder: GetUsageStatisticsInputBuilder) -> Result<GetUsageStatisticsOutput, SdkError<GetUsageStatisticsError>>;
        async fn get_usage_totals(&self, builder: GetUsageTotalsInputBuilder) -> Result<GetUsageTotalsOutput, SdkError<GetUsageTotalsError>>;
        async fn list_allow_lists(&self, builder: ListAllowListsInputBuilder) -> Result<ListAllowListsOutput, SdkError<ListAllowListsError>>;
        async fn list_automated_discovery_accounts(&self, builder: ListAutomatedDiscoveryAccountsInputBuilder) -> Result<ListAutomatedDiscoveryAccountsOutput, SdkError<ListAutomatedDiscoveryAccountsError>>;
        async fn list_classification_jobs(&self, builder: ListClassificationJobsInputBuilder) -> Result<ListClassificationJobsOutput, SdkError<ListClassificationJobsError>>;
        async fn list_classification_scopes(&self, builder: ListClassificationScopesInputBuilder) -> Result<ListClassificationScopesOutput, SdkError<ListClassificationScopesError>>;
        async fn list_custom_data_identifiers(&self, builder: ListCustomDataIdentifiersInputBuilder) -> Result<ListCustomDataIdentifiersOutput, SdkError<ListCustomDataIdentifiersError>>;
        async fn list_findings(&self, builder: ListFindingsInputBuilder) -> Result<ListFindingsOutput, SdkError<ListFindingsError>>;
        async fn list_findings_filters(&self, builder: ListFindingsFiltersInputBuilder) -> Result<ListFindingsFiltersOutput, SdkError<ListFindingsFiltersError>>;
        async fn list_invitations(&self, builder: ListInvitationsInputBuilder) -> Result<ListInvitationsOutput, SdkError<ListInvitationsError>>;
        async fn list_managed_data_identifiers(&self, builder: ListManagedDataIdentifiersInputBuilder) -> Result<ListManagedDataIdentifiersOutput, SdkError<ListManagedDataIdentifiersError>>;
        async fn list_members(&self, builder: ListMembersInputBuilder) -> Result<ListMembersOutput, SdkError<ListMembersError>>;
        async fn list_organization_admin_accounts(&self, builder: ListOrganizationAdminAccountsInputBuilder) -> Result<ListOrganizationAdminAccountsOutput, SdkError<ListOrganizationAdminAccountsError>>;
        async fn list_resource_profile_artifacts(&self, builder: ListResourceProfileArtifactsInputBuilder) -> Result<ListResourceProfileArtifactsOutput, SdkError<ListResourceProfileArtifactsError>>;
        async fn list_resource_profile_detections(&self, builder: ListResourceProfileDetectionsInputBuilder) -> Result<ListResourceProfileDetectionsOutput, SdkError<ListResourceProfileDetectionsError>>;
        async fn list_sensitivity_inspection_templates(&self, builder: ListSensitivityInspectionTemplatesInputBuilder) -> Result<ListSensitivityInspectionTemplatesOutput, SdkError<ListSensitivityInspectionTemplatesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn put_classification_export_configuration(&self, builder: PutClassificationExportConfigurationInputBuilder) -> Result<PutClassificationExportConfigurationOutput, SdkError<PutClassificationExportConfigurationError>>;
        async fn put_findings_publication_configuration(&self, builder: PutFindingsPublicationConfigurationInputBuilder) -> Result<PutFindingsPublicationConfigurationOutput, SdkError<PutFindingsPublicationConfigurationError>>;
        async fn search_resources(&self, builder: SearchResourcesInputBuilder) -> Result<SearchResourcesOutput, SdkError<SearchResourcesError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn test_custom_data_identifier(&self, builder: TestCustomDataIdentifierInputBuilder) -> Result<TestCustomDataIdentifierOutput, SdkError<TestCustomDataIdentifierError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_allow_list(&self, builder: UpdateAllowListInputBuilder) -> Result<UpdateAllowListOutput, SdkError<UpdateAllowListError>>;
        async fn update_automated_discovery_configuration(&self, builder: UpdateAutomatedDiscoveryConfigurationInputBuilder) -> Result<UpdateAutomatedDiscoveryConfigurationOutput, SdkError<UpdateAutomatedDiscoveryConfigurationError>>;
        async fn update_classification_job(&self, builder: UpdateClassificationJobInputBuilder) -> Result<UpdateClassificationJobOutput, SdkError<UpdateClassificationJobError>>;
        async fn update_classification_scope(&self, builder: UpdateClassificationScopeInputBuilder) -> Result<UpdateClassificationScopeOutput, SdkError<UpdateClassificationScopeError>>;
        async fn update_findings_filter(&self, builder: UpdateFindingsFilterInputBuilder) -> Result<UpdateFindingsFilterOutput, SdkError<UpdateFindingsFilterError>>;
        async fn update_macie_session(&self, builder: UpdateMacieSessionInputBuilder) -> Result<UpdateMacieSessionOutput, SdkError<UpdateMacieSessionError>>;
        async fn update_member_session(&self, builder: UpdateMemberSessionInputBuilder) -> Result<UpdateMemberSessionOutput, SdkError<UpdateMemberSessionError>>;
        async fn update_organization_configuration(&self, builder: UpdateOrganizationConfigurationInputBuilder) -> Result<UpdateOrganizationConfigurationOutput, SdkError<UpdateOrganizationConfigurationError>>;
        async fn update_resource_profile(&self, builder: UpdateResourceProfileInputBuilder) -> Result<UpdateResourceProfileOutput, SdkError<UpdateResourceProfileError>>;
        async fn update_resource_profile_detections(&self, builder: UpdateResourceProfileDetectionsInputBuilder) -> Result<UpdateResourceProfileDetectionsOutput, SdkError<UpdateResourceProfileDetectionsError>>;
        async fn update_reveal_configuration(&self, builder: UpdateRevealConfigurationInputBuilder) -> Result<UpdateRevealConfigurationOutput, SdkError<UpdateRevealConfigurationError>>;
        async fn update_sensitivity_inspection_template(&self, builder: UpdateSensitivityInspectionTemplateInputBuilder) -> Result<UpdateSensitivityInspectionTemplateOutput, SdkError<UpdateSensitivityInspectionTemplateError>>;
    }
}
