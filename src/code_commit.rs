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
use aws_sdk_codecommit::operation::associate_approval_rule_template_with_repository::{builders::*, *};
use aws_sdk_codecommit::operation::batch_associate_approval_rule_template_with_repositories::{builders::*, *};
use aws_sdk_codecommit::operation::batch_describe_merge_conflicts::{builders::*, *};
use aws_sdk_codecommit::operation::batch_disassociate_approval_rule_template_from_repositories::{builders::*, *};
use aws_sdk_codecommit::operation::batch_get_commits::{builders::*, *};
use aws_sdk_codecommit::operation::batch_get_repositories::{builders::*, *};
use aws_sdk_codecommit::operation::create_approval_rule_template::{builders::*, *};
use aws_sdk_codecommit::operation::create_branch::{builders::*, *};
use aws_sdk_codecommit::operation::create_commit::{builders::*, *};
use aws_sdk_codecommit::operation::create_pull_request::{builders::*, *};
use aws_sdk_codecommit::operation::create_pull_request_approval_rule::{builders::*, *};
use aws_sdk_codecommit::operation::create_repository::{builders::*, *};
use aws_sdk_codecommit::operation::create_unreferenced_merge_commit::{builders::*, *};
use aws_sdk_codecommit::operation::delete_approval_rule_template::{builders::*, *};
use aws_sdk_codecommit::operation::delete_branch::{builders::*, *};
use aws_sdk_codecommit::operation::delete_comment_content::{builders::*, *};
use aws_sdk_codecommit::operation::delete_file::{builders::*, *};
use aws_sdk_codecommit::operation::delete_pull_request_approval_rule::{builders::*, *};
use aws_sdk_codecommit::operation::delete_repository::{builders::*, *};
use aws_sdk_codecommit::operation::describe_merge_conflicts::{builders::*, *};
use aws_sdk_codecommit::operation::describe_pull_request_events::{builders::*, *};
use aws_sdk_codecommit::operation::disassociate_approval_rule_template_from_repository::{builders::*, *};
use aws_sdk_codecommit::operation::evaluate_pull_request_approval_rules::{builders::*, *};
use aws_sdk_codecommit::operation::get_approval_rule_template::{builders::*, *};
use aws_sdk_codecommit::operation::get_blob::{builders::*, *};
use aws_sdk_codecommit::operation::get_branch::{builders::*, *};
use aws_sdk_codecommit::operation::get_comment::{builders::*, *};
use aws_sdk_codecommit::operation::get_comment_reactions::{builders::*, *};
use aws_sdk_codecommit::operation::get_comments_for_compared_commit::{builders::*, *};
use aws_sdk_codecommit::operation::get_comments_for_pull_request::{builders::*, *};
use aws_sdk_codecommit::operation::get_commit::{builders::*, *};
use aws_sdk_codecommit::operation::get_differences::{builders::*, *};
use aws_sdk_codecommit::operation::get_file::{builders::*, *};
use aws_sdk_codecommit::operation::get_folder::{builders::*, *};
use aws_sdk_codecommit::operation::get_merge_commit::{builders::*, *};
use aws_sdk_codecommit::operation::get_merge_conflicts::{builders::*, *};
use aws_sdk_codecommit::operation::get_merge_options::{builders::*, *};
use aws_sdk_codecommit::operation::get_pull_request::{builders::*, *};
use aws_sdk_codecommit::operation::get_pull_request_approval_states::{builders::*, *};
use aws_sdk_codecommit::operation::get_pull_request_override_state::{builders::*, *};
use aws_sdk_codecommit::operation::get_repository::{builders::*, *};
use aws_sdk_codecommit::operation::get_repository_triggers::{builders::*, *};
use aws_sdk_codecommit::operation::list_approval_rule_templates::{builders::*, *};
use aws_sdk_codecommit::operation::list_associated_approval_rule_templates_for_repository::{builders::*, *};
use aws_sdk_codecommit::operation::list_branches::{builders::*, *};
use aws_sdk_codecommit::operation::list_file_commit_history::{builders::*, *};
use aws_sdk_codecommit::operation::list_pull_requests::{builders::*, *};
use aws_sdk_codecommit::operation::list_repositories::{builders::*, *};
use aws_sdk_codecommit::operation::list_repositories_for_approval_rule_template::{builders::*, *};
use aws_sdk_codecommit::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_codecommit::operation::merge_branches_by_fast_forward::{builders::*, *};
use aws_sdk_codecommit::operation::merge_branches_by_squash::{builders::*, *};
use aws_sdk_codecommit::operation::merge_branches_by_three_way::{builders::*, *};
use aws_sdk_codecommit::operation::merge_pull_request_by_fast_forward::{builders::*, *};
use aws_sdk_codecommit::operation::merge_pull_request_by_squash::{builders::*, *};
use aws_sdk_codecommit::operation::merge_pull_request_by_three_way::{builders::*, *};
use aws_sdk_codecommit::operation::override_pull_request_approval_rules::{builders::*, *};
use aws_sdk_codecommit::operation::post_comment_for_compared_commit::{builders::*, *};
use aws_sdk_codecommit::operation::post_comment_for_pull_request::{builders::*, *};
use aws_sdk_codecommit::operation::post_comment_reply::{builders::*, *};
use aws_sdk_codecommit::operation::put_comment_reaction::{builders::*, *};
use aws_sdk_codecommit::operation::put_file::{builders::*, *};
use aws_sdk_codecommit::operation::put_repository_triggers::{builders::*, *};
use aws_sdk_codecommit::operation::tag_resource::{builders::*, *};
use aws_sdk_codecommit::operation::test_repository_triggers::{builders::*, *};
use aws_sdk_codecommit::operation::untag_resource::{builders::*, *};
use aws_sdk_codecommit::operation::update_approval_rule_template_content::{builders::*, *};
use aws_sdk_codecommit::operation::update_approval_rule_template_description::{builders::*, *};
use aws_sdk_codecommit::operation::update_approval_rule_template_name::{builders::*, *};
use aws_sdk_codecommit::operation::update_comment::{builders::*, *};
use aws_sdk_codecommit::operation::update_default_branch::{builders::*, *};
use aws_sdk_codecommit::operation::update_pull_request_approval_rule_content::{builders::*, *};
use aws_sdk_codecommit::operation::update_pull_request_approval_state::{builders::*, *};
use aws_sdk_codecommit::operation::update_pull_request_description::{builders::*, *};
use aws_sdk_codecommit::operation::update_pull_request_status::{builders::*, *};
use aws_sdk_codecommit::operation::update_pull_request_title::{builders::*, *};
use aws_sdk_codecommit::operation::update_repository_description::{builders::*, *};
use aws_sdk_codecommit::operation::update_repository_encryption_key::{builders::*, *};
use aws_sdk_codecommit::operation::update_repository_name::{builders::*, *};
use aws_sdk_codecommit::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_codecommit::Client;
use std::ops::Deref;

pub use aws_sdk_codecommit::*;

pub struct CodeCommitClientImpl(Client);
impl CodeCommitClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait CodeCommitClient {
    fn associate_approval_rule_template_with_repository(&self, builder: AssociateApprovalRuleTemplateWithRepositoryInputBuilder) -> impl Future<Output = Result<AssociateApprovalRuleTemplateWithRepositoryOutput, SdkError<AssociateApprovalRuleTemplateWithRepositoryError>>>;
    fn batch_associate_approval_rule_template_with_repositories(&self, builder: BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder) -> impl Future<Output = Result<BatchAssociateApprovalRuleTemplateWithRepositoriesOutput, SdkError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>>>;
    fn batch_describe_merge_conflicts(&self, builder: BatchDescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<BatchDescribeMergeConflictsOutput, SdkError<BatchDescribeMergeConflictsError>>>;
    fn batch_disassociate_approval_rule_template_from_repositories(&self, builder: BatchDisassociateApprovalRuleTemplateFromRepositoriesInputBuilder) -> impl Future<Output = Result<BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput, SdkError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>>>;
    fn batch_get_commits(&self, builder: BatchGetCommitsInputBuilder) -> impl Future<Output = Result<BatchGetCommitsOutput, SdkError<BatchGetCommitsError>>>;
    fn batch_get_repositories(&self, builder: BatchGetRepositoriesInputBuilder) -> impl Future<Output = Result<BatchGetRepositoriesOutput, SdkError<BatchGetRepositoriesError>>>;
    fn create_approval_rule_template(&self, builder: CreateApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<CreateApprovalRuleTemplateOutput, SdkError<CreateApprovalRuleTemplateError>>>;
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>>;
    fn create_commit(&self, builder: CreateCommitInputBuilder) -> impl Future<Output = Result<CreateCommitOutput, SdkError<CreateCommitError>>>;
    fn create_pull_request(&self, builder: CreatePullRequestInputBuilder) -> impl Future<Output = Result<CreatePullRequestOutput, SdkError<CreatePullRequestError>>>;
    fn create_pull_request_approval_rule(&self, builder: CreatePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<CreatePullRequestApprovalRuleOutput, SdkError<CreatePullRequestApprovalRuleError>>>;
    fn create_repository(&self, builder: CreateRepositoryInputBuilder) -> impl Future<Output = Result<CreateRepositoryOutput, SdkError<CreateRepositoryError>>>;
    fn create_unreferenced_merge_commit(&self, builder: CreateUnreferencedMergeCommitInputBuilder) -> impl Future<Output = Result<CreateUnreferencedMergeCommitOutput, SdkError<CreateUnreferencedMergeCommitError>>>;
    fn delete_approval_rule_template(&self, builder: DeleteApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<DeleteApprovalRuleTemplateOutput, SdkError<DeleteApprovalRuleTemplateError>>>;
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>>;
    fn delete_comment_content(&self, builder: DeleteCommentContentInputBuilder) -> impl Future<Output = Result<DeleteCommentContentOutput, SdkError<DeleteCommentContentError>>>;
    fn delete_file(&self, builder: DeleteFileInputBuilder) -> impl Future<Output = Result<DeleteFileOutput, SdkError<DeleteFileError>>>;
    fn delete_pull_request_approval_rule(&self, builder: DeletePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<DeletePullRequestApprovalRuleOutput, SdkError<DeletePullRequestApprovalRuleError>>>;
    fn delete_repository(&self, builder: DeleteRepositoryInputBuilder) -> impl Future<Output = Result<DeleteRepositoryOutput, SdkError<DeleteRepositoryError>>>;
    fn describe_merge_conflicts(&self, builder: DescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<DescribeMergeConflictsOutput, SdkError<DescribeMergeConflictsError>>>;
    fn describe_pull_request_events(&self, builder: DescribePullRequestEventsInputBuilder) -> impl Future<Output = Result<DescribePullRequestEventsOutput, SdkError<DescribePullRequestEventsError>>>;
    fn disassociate_approval_rule_template_from_repository(&self, builder: DisassociateApprovalRuleTemplateFromRepositoryInputBuilder) -> impl Future<Output = Result<DisassociateApprovalRuleTemplateFromRepositoryOutput, SdkError<DisassociateApprovalRuleTemplateFromRepositoryError>>>;
    fn evaluate_pull_request_approval_rules(&self, builder: EvaluatePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<EvaluatePullRequestApprovalRulesOutput, SdkError<EvaluatePullRequestApprovalRulesError>>>;
    fn get_approval_rule_template(&self, builder: GetApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<GetApprovalRuleTemplateOutput, SdkError<GetApprovalRuleTemplateError>>>;
    fn get_blob(&self, builder: GetBlobInputBuilder) -> impl Future<Output = Result<GetBlobOutput, SdkError<GetBlobError>>>;
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>>;
    fn get_comment(&self, builder: GetCommentInputBuilder) -> impl Future<Output = Result<GetCommentOutput, SdkError<GetCommentError>>>;
    fn get_comment_reactions(&self, builder: GetCommentReactionsInputBuilder) -> impl Future<Output = Result<GetCommentReactionsOutput, SdkError<GetCommentReactionsError>>>;
    fn get_comments_for_compared_commit(&self, builder: GetCommentsForComparedCommitInputBuilder) -> impl Future<Output = Result<GetCommentsForComparedCommitOutput, SdkError<GetCommentsForComparedCommitError>>>;
    fn get_comments_for_pull_request(&self, builder: GetCommentsForPullRequestInputBuilder) -> impl Future<Output = Result<GetCommentsForPullRequestOutput, SdkError<GetCommentsForPullRequestError>>>;
    fn get_commit(&self, builder: GetCommitInputBuilder) -> impl Future<Output = Result<GetCommitOutput, SdkError<GetCommitError>>>;
    fn get_differences(&self, builder: GetDifferencesInputBuilder) -> impl Future<Output = Result<GetDifferencesOutput, SdkError<GetDifferencesError>>>;
    fn get_file(&self, builder: GetFileInputBuilder) -> impl Future<Output = Result<GetFileOutput, SdkError<GetFileError>>>;
    fn get_folder(&self, builder: GetFolderInputBuilder) -> impl Future<Output = Result<GetFolderOutput, SdkError<GetFolderError>>>;
    fn get_merge_commit(&self, builder: GetMergeCommitInputBuilder) -> impl Future<Output = Result<GetMergeCommitOutput, SdkError<GetMergeCommitError>>>;
    fn get_merge_conflicts(&self, builder: GetMergeConflictsInputBuilder) -> impl Future<Output = Result<GetMergeConflictsOutput, SdkError<GetMergeConflictsError>>>;
    fn get_merge_options(&self, builder: GetMergeOptionsInputBuilder) -> impl Future<Output = Result<GetMergeOptionsOutput, SdkError<GetMergeOptionsError>>>;
    fn get_pull_request(&self, builder: GetPullRequestInputBuilder) -> impl Future<Output = Result<GetPullRequestOutput, SdkError<GetPullRequestError>>>;
    fn get_pull_request_approval_states(&self, builder: GetPullRequestApprovalStatesInputBuilder) -> impl Future<Output = Result<GetPullRequestApprovalStatesOutput, SdkError<GetPullRequestApprovalStatesError>>>;
    fn get_pull_request_override_state(&self, builder: GetPullRequestOverrideStateInputBuilder) -> impl Future<Output = Result<GetPullRequestOverrideStateOutput, SdkError<GetPullRequestOverrideStateError>>>;
    fn get_repository(&self, builder: GetRepositoryInputBuilder) -> impl Future<Output = Result<GetRepositoryOutput, SdkError<GetRepositoryError>>>;
    fn get_repository_triggers(&self, builder: GetRepositoryTriggersInputBuilder) -> impl Future<Output = Result<GetRepositoryTriggersOutput, SdkError<GetRepositoryTriggersError>>>;
    fn list_approval_rule_templates(&self, builder: ListApprovalRuleTemplatesInputBuilder) -> impl Future<Output = Result<ListApprovalRuleTemplatesOutput, SdkError<ListApprovalRuleTemplatesError>>>;
    fn list_associated_approval_rule_templates_for_repository(&self, builder: ListAssociatedApprovalRuleTemplatesForRepositoryInputBuilder) -> impl Future<Output = Result<ListAssociatedApprovalRuleTemplatesForRepositoryOutput, SdkError<ListAssociatedApprovalRuleTemplatesForRepositoryError>>>;
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>>;
    fn list_file_commit_history(&self, builder: ListFileCommitHistoryInputBuilder) -> impl Future<Output = Result<ListFileCommitHistoryOutput, SdkError<ListFileCommitHistoryError>>>;
    fn list_pull_requests(&self, builder: ListPullRequestsInputBuilder) -> impl Future<Output = Result<ListPullRequestsOutput, SdkError<ListPullRequestsError>>>;
    fn list_repositories(&self, builder: ListRepositoriesInputBuilder) -> impl Future<Output = Result<ListRepositoriesOutput, SdkError<ListRepositoriesError>>>;
    fn list_repositories_for_approval_rule_template(&self, builder: ListRepositoriesForApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<ListRepositoriesForApprovalRuleTemplateOutput, SdkError<ListRepositoriesForApprovalRuleTemplateError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn merge_branches_by_fast_forward(&self, builder: MergeBranchesByFastForwardInputBuilder) -> impl Future<Output = Result<MergeBranchesByFastForwardOutput, SdkError<MergeBranchesByFastForwardError>>>;
    fn merge_branches_by_squash(&self, builder: MergeBranchesBySquashInputBuilder) -> impl Future<Output = Result<MergeBranchesBySquashOutput, SdkError<MergeBranchesBySquashError>>>;
    fn merge_branches_by_three_way(&self, builder: MergeBranchesByThreeWayInputBuilder) -> impl Future<Output = Result<MergeBranchesByThreeWayOutput, SdkError<MergeBranchesByThreeWayError>>>;
    fn merge_pull_request_by_fast_forward(&self, builder: MergePullRequestByFastForwardInputBuilder) -> impl Future<Output = Result<MergePullRequestByFastForwardOutput, SdkError<MergePullRequestByFastForwardError>>>;
    fn merge_pull_request_by_squash(&self, builder: MergePullRequestBySquashInputBuilder) -> impl Future<Output = Result<MergePullRequestBySquashOutput, SdkError<MergePullRequestBySquashError>>>;
    fn merge_pull_request_by_three_way(&self, builder: MergePullRequestByThreeWayInputBuilder) -> impl Future<Output = Result<MergePullRequestByThreeWayOutput, SdkError<MergePullRequestByThreeWayError>>>;
    fn override_pull_request_approval_rules(&self, builder: OverridePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<OverridePullRequestApprovalRulesOutput, SdkError<OverridePullRequestApprovalRulesError>>>;
    fn post_comment_for_compared_commit(&self, builder: PostCommentForComparedCommitInputBuilder) -> impl Future<Output = Result<PostCommentForComparedCommitOutput, SdkError<PostCommentForComparedCommitError>>>;
    fn post_comment_for_pull_request(&self, builder: PostCommentForPullRequestInputBuilder) -> impl Future<Output = Result<PostCommentForPullRequestOutput, SdkError<PostCommentForPullRequestError>>>;
    fn post_comment_reply(&self, builder: PostCommentReplyInputBuilder) -> impl Future<Output = Result<PostCommentReplyOutput, SdkError<PostCommentReplyError>>>;
    fn put_comment_reaction(&self, builder: PutCommentReactionInputBuilder) -> impl Future<Output = Result<PutCommentReactionOutput, SdkError<PutCommentReactionError>>>;
    fn put_file(&self, builder: PutFileInputBuilder) -> impl Future<Output = Result<PutFileOutput, SdkError<PutFileError>>>;
    fn put_repository_triggers(&self, builder: PutRepositoryTriggersInputBuilder) -> impl Future<Output = Result<PutRepositoryTriggersOutput, SdkError<PutRepositoryTriggersError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn test_repository_triggers(&self, builder: TestRepositoryTriggersInputBuilder) -> impl Future<Output = Result<TestRepositoryTriggersOutput, SdkError<TestRepositoryTriggersError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_approval_rule_template_content(&self, builder: UpdateApprovalRuleTemplateContentInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateContentOutput, SdkError<UpdateApprovalRuleTemplateContentError>>>;
    fn update_approval_rule_template_description(&self, builder: UpdateApprovalRuleTemplateDescriptionInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateDescriptionOutput, SdkError<UpdateApprovalRuleTemplateDescriptionError>>>;
    fn update_approval_rule_template_name(&self, builder: UpdateApprovalRuleTemplateNameInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateNameOutput, SdkError<UpdateApprovalRuleTemplateNameError>>>;
    fn update_comment(&self, builder: UpdateCommentInputBuilder) -> impl Future<Output = Result<UpdateCommentOutput, SdkError<UpdateCommentError>>>;
    fn update_default_branch(&self, builder: UpdateDefaultBranchInputBuilder) -> impl Future<Output = Result<UpdateDefaultBranchOutput, SdkError<UpdateDefaultBranchError>>>;
    fn update_pull_request_approval_rule_content(&self, builder: UpdatePullRequestApprovalRuleContentInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalRuleContentOutput, SdkError<UpdatePullRequestApprovalRuleContentError>>>;
    fn update_pull_request_approval_state(&self, builder: UpdatePullRequestApprovalStateInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalStateOutput, SdkError<UpdatePullRequestApprovalStateError>>>;
    fn update_pull_request_description(&self, builder: UpdatePullRequestDescriptionInputBuilder) -> impl Future<Output = Result<UpdatePullRequestDescriptionOutput, SdkError<UpdatePullRequestDescriptionError>>>;
    fn update_pull_request_status(&self, builder: UpdatePullRequestStatusInputBuilder) -> impl Future<Output = Result<UpdatePullRequestStatusOutput, SdkError<UpdatePullRequestStatusError>>>;
    fn update_pull_request_title(&self, builder: UpdatePullRequestTitleInputBuilder) -> impl Future<Output = Result<UpdatePullRequestTitleOutput, SdkError<UpdatePullRequestTitleError>>>;
    fn update_repository_description(&self, builder: UpdateRepositoryDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRepositoryDescriptionOutput, SdkError<UpdateRepositoryDescriptionError>>>;
    fn update_repository_encryption_key(&self, builder: UpdateRepositoryEncryptionKeyInputBuilder) -> impl Future<Output = Result<UpdateRepositoryEncryptionKeyOutput, SdkError<UpdateRepositoryEncryptionKeyError>>>;
    fn update_repository_name(&self, builder: UpdateRepositoryNameInputBuilder) -> impl Future<Output = Result<UpdateRepositoryNameOutput, SdkError<UpdateRepositoryNameError>>>;
}
impl CodeCommitClient for CodeCommitClientImpl {
    fn associate_approval_rule_template_with_repository(&self, builder: AssociateApprovalRuleTemplateWithRepositoryInputBuilder) -> impl Future<Output = Result<AssociateApprovalRuleTemplateWithRepositoryOutput, SdkError<AssociateApprovalRuleTemplateWithRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn batch_associate_approval_rule_template_with_repositories(&self, builder: BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder) -> impl Future<Output = Result<BatchAssociateApprovalRuleTemplateWithRepositoriesOutput, SdkError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>>> {
        builder.send_with(&self.0)
    }
    fn batch_describe_merge_conflicts(&self, builder: BatchDescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<BatchDescribeMergeConflictsOutput, SdkError<BatchDescribeMergeConflictsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_disassociate_approval_rule_template_from_repositories(&self, builder: BatchDisassociateApprovalRuleTemplateFromRepositoriesInputBuilder) -> impl Future<Output = Result<BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput, SdkError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_commits(&self, builder: BatchGetCommitsInputBuilder) -> impl Future<Output = Result<BatchGetCommitsOutput, SdkError<BatchGetCommitsError>>> {
        builder.send_with(&self.0)
    }
    fn batch_get_repositories(&self, builder: BatchGetRepositoriesInputBuilder) -> impl Future<Output = Result<BatchGetRepositoriesOutput, SdkError<BatchGetRepositoriesError>>> {
        builder.send_with(&self.0)
    }
    fn create_approval_rule_template(&self, builder: CreateApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<CreateApprovalRuleTemplateOutput, SdkError<CreateApprovalRuleTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>> {
        builder.send_with(&self.0)
    }
    fn create_commit(&self, builder: CreateCommitInputBuilder) -> impl Future<Output = Result<CreateCommitOutput, SdkError<CreateCommitError>>> {
        builder.send_with(&self.0)
    }
    fn create_pull_request(&self, builder: CreatePullRequestInputBuilder) -> impl Future<Output = Result<CreatePullRequestOutput, SdkError<CreatePullRequestError>>> {
        builder.send_with(&self.0)
    }
    fn create_pull_request_approval_rule(&self, builder: CreatePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<CreatePullRequestApprovalRuleOutput, SdkError<CreatePullRequestApprovalRuleError>>> {
        builder.send_with(&self.0)
    }
    fn create_repository(&self, builder: CreateRepositoryInputBuilder) -> impl Future<Output = Result<CreateRepositoryOutput, SdkError<CreateRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn create_unreferenced_merge_commit(&self, builder: CreateUnreferencedMergeCommitInputBuilder) -> impl Future<Output = Result<CreateUnreferencedMergeCommitOutput, SdkError<CreateUnreferencedMergeCommitError>>> {
        builder.send_with(&self.0)
    }
    fn delete_approval_rule_template(&self, builder: DeleteApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<DeleteApprovalRuleTemplateOutput, SdkError<DeleteApprovalRuleTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>> {
        builder.send_with(&self.0)
    }
    fn delete_comment_content(&self, builder: DeleteCommentContentInputBuilder) -> impl Future<Output = Result<DeleteCommentContentOutput, SdkError<DeleteCommentContentError>>> {
        builder.send_with(&self.0)
    }
    fn delete_file(&self, builder: DeleteFileInputBuilder) -> impl Future<Output = Result<DeleteFileOutput, SdkError<DeleteFileError>>> {
        builder.send_with(&self.0)
    }
    fn delete_pull_request_approval_rule(&self, builder: DeletePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<DeletePullRequestApprovalRuleOutput, SdkError<DeletePullRequestApprovalRuleError>>> {
        builder.send_with(&self.0)
    }
    fn delete_repository(&self, builder: DeleteRepositoryInputBuilder) -> impl Future<Output = Result<DeleteRepositoryOutput, SdkError<DeleteRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn describe_merge_conflicts(&self, builder: DescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<DescribeMergeConflictsOutput, SdkError<DescribeMergeConflictsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_pull_request_events(&self, builder: DescribePullRequestEventsInputBuilder) -> impl Future<Output = Result<DescribePullRequestEventsOutput, SdkError<DescribePullRequestEventsError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_approval_rule_template_from_repository(&self, builder: DisassociateApprovalRuleTemplateFromRepositoryInputBuilder) -> impl Future<Output = Result<DisassociateApprovalRuleTemplateFromRepositoryOutput, SdkError<DisassociateApprovalRuleTemplateFromRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn evaluate_pull_request_approval_rules(&self, builder: EvaluatePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<EvaluatePullRequestApprovalRulesOutput, SdkError<EvaluatePullRequestApprovalRulesError>>> {
        builder.send_with(&self.0)
    }
    fn get_approval_rule_template(&self, builder: GetApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<GetApprovalRuleTemplateOutput, SdkError<GetApprovalRuleTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn get_blob(&self, builder: GetBlobInputBuilder) -> impl Future<Output = Result<GetBlobOutput, SdkError<GetBlobError>>> {
        builder.send_with(&self.0)
    }
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>> {
        builder.send_with(&self.0)
    }
    fn get_comment(&self, builder: GetCommentInputBuilder) -> impl Future<Output = Result<GetCommentOutput, SdkError<GetCommentError>>> {
        builder.send_with(&self.0)
    }
    fn get_comment_reactions(&self, builder: GetCommentReactionsInputBuilder) -> impl Future<Output = Result<GetCommentReactionsOutput, SdkError<GetCommentReactionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_comments_for_compared_commit(&self, builder: GetCommentsForComparedCommitInputBuilder) -> impl Future<Output = Result<GetCommentsForComparedCommitOutput, SdkError<GetCommentsForComparedCommitError>>> {
        builder.send_with(&self.0)
    }
    fn get_comments_for_pull_request(&self, builder: GetCommentsForPullRequestInputBuilder) -> impl Future<Output = Result<GetCommentsForPullRequestOutput, SdkError<GetCommentsForPullRequestError>>> {
        builder.send_with(&self.0)
    }
    fn get_commit(&self, builder: GetCommitInputBuilder) -> impl Future<Output = Result<GetCommitOutput, SdkError<GetCommitError>>> {
        builder.send_with(&self.0)
    }
    fn get_differences(&self, builder: GetDifferencesInputBuilder) -> impl Future<Output = Result<GetDifferencesOutput, SdkError<GetDifferencesError>>> {
        builder.send_with(&self.0)
    }
    fn get_file(&self, builder: GetFileInputBuilder) -> impl Future<Output = Result<GetFileOutput, SdkError<GetFileError>>> {
        builder.send_with(&self.0)
    }
    fn get_folder(&self, builder: GetFolderInputBuilder) -> impl Future<Output = Result<GetFolderOutput, SdkError<GetFolderError>>> {
        builder.send_with(&self.0)
    }
    fn get_merge_commit(&self, builder: GetMergeCommitInputBuilder) -> impl Future<Output = Result<GetMergeCommitOutput, SdkError<GetMergeCommitError>>> {
        builder.send_with(&self.0)
    }
    fn get_merge_conflicts(&self, builder: GetMergeConflictsInputBuilder) -> impl Future<Output = Result<GetMergeConflictsOutput, SdkError<GetMergeConflictsError>>> {
        builder.send_with(&self.0)
    }
    fn get_merge_options(&self, builder: GetMergeOptionsInputBuilder) -> impl Future<Output = Result<GetMergeOptionsOutput, SdkError<GetMergeOptionsError>>> {
        builder.send_with(&self.0)
    }
    fn get_pull_request(&self, builder: GetPullRequestInputBuilder) -> impl Future<Output = Result<GetPullRequestOutput, SdkError<GetPullRequestError>>> {
        builder.send_with(&self.0)
    }
    fn get_pull_request_approval_states(&self, builder: GetPullRequestApprovalStatesInputBuilder) -> impl Future<Output = Result<GetPullRequestApprovalStatesOutput, SdkError<GetPullRequestApprovalStatesError>>> {
        builder.send_with(&self.0)
    }
    fn get_pull_request_override_state(&self, builder: GetPullRequestOverrideStateInputBuilder) -> impl Future<Output = Result<GetPullRequestOverrideStateOutput, SdkError<GetPullRequestOverrideStateError>>> {
        builder.send_with(&self.0)
    }
    fn get_repository(&self, builder: GetRepositoryInputBuilder) -> impl Future<Output = Result<GetRepositoryOutput, SdkError<GetRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn get_repository_triggers(&self, builder: GetRepositoryTriggersInputBuilder) -> impl Future<Output = Result<GetRepositoryTriggersOutput, SdkError<GetRepositoryTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn list_approval_rule_templates(&self, builder: ListApprovalRuleTemplatesInputBuilder) -> impl Future<Output = Result<ListApprovalRuleTemplatesOutput, SdkError<ListApprovalRuleTemplatesError>>> {
        builder.send_with(&self.0)
    }
    fn list_associated_approval_rule_templates_for_repository(&self, builder: ListAssociatedApprovalRuleTemplatesForRepositoryInputBuilder) -> impl Future<Output = Result<ListAssociatedApprovalRuleTemplatesForRepositoryOutput, SdkError<ListAssociatedApprovalRuleTemplatesForRepositoryError>>> {
        builder.send_with(&self.0)
    }
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>> {
        builder.send_with(&self.0)
    }
    fn list_file_commit_history(&self, builder: ListFileCommitHistoryInputBuilder) -> impl Future<Output = Result<ListFileCommitHistoryOutput, SdkError<ListFileCommitHistoryError>>> {
        builder.send_with(&self.0)
    }
    fn list_pull_requests(&self, builder: ListPullRequestsInputBuilder) -> impl Future<Output = Result<ListPullRequestsOutput, SdkError<ListPullRequestsError>>> {
        builder.send_with(&self.0)
    }
    fn list_repositories(&self, builder: ListRepositoriesInputBuilder) -> impl Future<Output = Result<ListRepositoriesOutput, SdkError<ListRepositoriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_repositories_for_approval_rule_template(&self, builder: ListRepositoriesForApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<ListRepositoriesForApprovalRuleTemplateOutput, SdkError<ListRepositoriesForApprovalRuleTemplateError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn merge_branches_by_fast_forward(&self, builder: MergeBranchesByFastForwardInputBuilder) -> impl Future<Output = Result<MergeBranchesByFastForwardOutput, SdkError<MergeBranchesByFastForwardError>>> {
        builder.send_with(&self.0)
    }
    fn merge_branches_by_squash(&self, builder: MergeBranchesBySquashInputBuilder) -> impl Future<Output = Result<MergeBranchesBySquashOutput, SdkError<MergeBranchesBySquashError>>> {
        builder.send_with(&self.0)
    }
    fn merge_branches_by_three_way(&self, builder: MergeBranchesByThreeWayInputBuilder) -> impl Future<Output = Result<MergeBranchesByThreeWayOutput, SdkError<MergeBranchesByThreeWayError>>> {
        builder.send_with(&self.0)
    }
    fn merge_pull_request_by_fast_forward(&self, builder: MergePullRequestByFastForwardInputBuilder) -> impl Future<Output = Result<MergePullRequestByFastForwardOutput, SdkError<MergePullRequestByFastForwardError>>> {
        builder.send_with(&self.0)
    }
    fn merge_pull_request_by_squash(&self, builder: MergePullRequestBySquashInputBuilder) -> impl Future<Output = Result<MergePullRequestBySquashOutput, SdkError<MergePullRequestBySquashError>>> {
        builder.send_with(&self.0)
    }
    fn merge_pull_request_by_three_way(&self, builder: MergePullRequestByThreeWayInputBuilder) -> impl Future<Output = Result<MergePullRequestByThreeWayOutput, SdkError<MergePullRequestByThreeWayError>>> {
        builder.send_with(&self.0)
    }
    fn override_pull_request_approval_rules(&self, builder: OverridePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<OverridePullRequestApprovalRulesOutput, SdkError<OverridePullRequestApprovalRulesError>>> {
        builder.send_with(&self.0)
    }
    fn post_comment_for_compared_commit(&self, builder: PostCommentForComparedCommitInputBuilder) -> impl Future<Output = Result<PostCommentForComparedCommitOutput, SdkError<PostCommentForComparedCommitError>>> {
        builder.send_with(&self.0)
    }
    fn post_comment_for_pull_request(&self, builder: PostCommentForPullRequestInputBuilder) -> impl Future<Output = Result<PostCommentForPullRequestOutput, SdkError<PostCommentForPullRequestError>>> {
        builder.send_with(&self.0)
    }
    fn post_comment_reply(&self, builder: PostCommentReplyInputBuilder) -> impl Future<Output = Result<PostCommentReplyOutput, SdkError<PostCommentReplyError>>> {
        builder.send_with(&self.0)
    }
    fn put_comment_reaction(&self, builder: PutCommentReactionInputBuilder) -> impl Future<Output = Result<PutCommentReactionOutput, SdkError<PutCommentReactionError>>> {
        builder.send_with(&self.0)
    }
    fn put_file(&self, builder: PutFileInputBuilder) -> impl Future<Output = Result<PutFileOutput, SdkError<PutFileError>>> {
        builder.send_with(&self.0)
    }
    fn put_repository_triggers(&self, builder: PutRepositoryTriggersInputBuilder) -> impl Future<Output = Result<PutRepositoryTriggersOutput, SdkError<PutRepositoryTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn test_repository_triggers(&self, builder: TestRepositoryTriggersInputBuilder) -> impl Future<Output = Result<TestRepositoryTriggersOutput, SdkError<TestRepositoryTriggersError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_approval_rule_template_content(&self, builder: UpdateApprovalRuleTemplateContentInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateContentOutput, SdkError<UpdateApprovalRuleTemplateContentError>>> {
        builder.send_with(&self.0)
    }
    fn update_approval_rule_template_description(&self, builder: UpdateApprovalRuleTemplateDescriptionInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateDescriptionOutput, SdkError<UpdateApprovalRuleTemplateDescriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_approval_rule_template_name(&self, builder: UpdateApprovalRuleTemplateNameInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateNameOutput, SdkError<UpdateApprovalRuleTemplateNameError>>> {
        builder.send_with(&self.0)
    }
    fn update_comment(&self, builder: UpdateCommentInputBuilder) -> impl Future<Output = Result<UpdateCommentOutput, SdkError<UpdateCommentError>>> {
        builder.send_with(&self.0)
    }
    fn update_default_branch(&self, builder: UpdateDefaultBranchInputBuilder) -> impl Future<Output = Result<UpdateDefaultBranchOutput, SdkError<UpdateDefaultBranchError>>> {
        builder.send_with(&self.0)
    }
    fn update_pull_request_approval_rule_content(&self, builder: UpdatePullRequestApprovalRuleContentInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalRuleContentOutput, SdkError<UpdatePullRequestApprovalRuleContentError>>> {
        builder.send_with(&self.0)
    }
    fn update_pull_request_approval_state(&self, builder: UpdatePullRequestApprovalStateInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalStateOutput, SdkError<UpdatePullRequestApprovalStateError>>> {
        builder.send_with(&self.0)
    }
    fn update_pull_request_description(&self, builder: UpdatePullRequestDescriptionInputBuilder) -> impl Future<Output = Result<UpdatePullRequestDescriptionOutput, SdkError<UpdatePullRequestDescriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_pull_request_status(&self, builder: UpdatePullRequestStatusInputBuilder) -> impl Future<Output = Result<UpdatePullRequestStatusOutput, SdkError<UpdatePullRequestStatusError>>> {
        builder.send_with(&self.0)
    }
    fn update_pull_request_title(&self, builder: UpdatePullRequestTitleInputBuilder) -> impl Future<Output = Result<UpdatePullRequestTitleOutput, SdkError<UpdatePullRequestTitleError>>> {
        builder.send_with(&self.0)
    }
    fn update_repository_description(&self, builder: UpdateRepositoryDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRepositoryDescriptionOutput, SdkError<UpdateRepositoryDescriptionError>>> {
        builder.send_with(&self.0)
    }
    fn update_repository_encryption_key(&self, builder: UpdateRepositoryEncryptionKeyInputBuilder) -> impl Future<Output = Result<UpdateRepositoryEncryptionKeyOutput, SdkError<UpdateRepositoryEncryptionKeyError>>> {
        builder.send_with(&self.0)
    }
    fn update_repository_name(&self, builder: UpdateRepositoryNameInputBuilder) -> impl Future<Output = Result<UpdateRepositoryNameOutput, SdkError<UpdateRepositoryNameError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> CodeCommitClient for T
where T: Deref,
      T::Target: CodeCommitClient {
    fn associate_approval_rule_template_with_repository(&self, builder: AssociateApprovalRuleTemplateWithRepositoryInputBuilder) -> impl Future<Output = Result<AssociateApprovalRuleTemplateWithRepositoryOutput, SdkError<AssociateApprovalRuleTemplateWithRepositoryError>>> {
        self.deref().associate_approval_rule_template_with_repository(builder)
    }
    fn batch_associate_approval_rule_template_with_repositories(&self, builder: BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder) -> impl Future<Output = Result<BatchAssociateApprovalRuleTemplateWithRepositoriesOutput, SdkError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>>> {
        self.deref().batch_associate_approval_rule_template_with_repositories(builder)
    }
    fn batch_describe_merge_conflicts(&self, builder: BatchDescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<BatchDescribeMergeConflictsOutput, SdkError<BatchDescribeMergeConflictsError>>> {
        self.deref().batch_describe_merge_conflicts(builder)
    }
    fn batch_disassociate_approval_rule_template_from_repositories(&self, builder: BatchDisassociateApprovalRuleTemplateFromRepositoriesInputBuilder) -> impl Future<Output = Result<BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput, SdkError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>>> {
        self.deref().batch_disassociate_approval_rule_template_from_repositories(builder)
    }
    fn batch_get_commits(&self, builder: BatchGetCommitsInputBuilder) -> impl Future<Output = Result<BatchGetCommitsOutput, SdkError<BatchGetCommitsError>>> {
        self.deref().batch_get_commits(builder)
    }
    fn batch_get_repositories(&self, builder: BatchGetRepositoriesInputBuilder) -> impl Future<Output = Result<BatchGetRepositoriesOutput, SdkError<BatchGetRepositoriesError>>> {
        self.deref().batch_get_repositories(builder)
    }
    fn create_approval_rule_template(&self, builder: CreateApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<CreateApprovalRuleTemplateOutput, SdkError<CreateApprovalRuleTemplateError>>> {
        self.deref().create_approval_rule_template(builder)
    }
    fn create_branch(&self, builder: CreateBranchInputBuilder) -> impl Future<Output = Result<CreateBranchOutput, SdkError<CreateBranchError>>> {
        self.deref().create_branch(builder)
    }
    fn create_commit(&self, builder: CreateCommitInputBuilder) -> impl Future<Output = Result<CreateCommitOutput, SdkError<CreateCommitError>>> {
        self.deref().create_commit(builder)
    }
    fn create_pull_request(&self, builder: CreatePullRequestInputBuilder) -> impl Future<Output = Result<CreatePullRequestOutput, SdkError<CreatePullRequestError>>> {
        self.deref().create_pull_request(builder)
    }
    fn create_pull_request_approval_rule(&self, builder: CreatePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<CreatePullRequestApprovalRuleOutput, SdkError<CreatePullRequestApprovalRuleError>>> {
        self.deref().create_pull_request_approval_rule(builder)
    }
    fn create_repository(&self, builder: CreateRepositoryInputBuilder) -> impl Future<Output = Result<CreateRepositoryOutput, SdkError<CreateRepositoryError>>> {
        self.deref().create_repository(builder)
    }
    fn create_unreferenced_merge_commit(&self, builder: CreateUnreferencedMergeCommitInputBuilder) -> impl Future<Output = Result<CreateUnreferencedMergeCommitOutput, SdkError<CreateUnreferencedMergeCommitError>>> {
        self.deref().create_unreferenced_merge_commit(builder)
    }
    fn delete_approval_rule_template(&self, builder: DeleteApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<DeleteApprovalRuleTemplateOutput, SdkError<DeleteApprovalRuleTemplateError>>> {
        self.deref().delete_approval_rule_template(builder)
    }
    fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> impl Future<Output = Result<DeleteBranchOutput, SdkError<DeleteBranchError>>> {
        self.deref().delete_branch(builder)
    }
    fn delete_comment_content(&self, builder: DeleteCommentContentInputBuilder) -> impl Future<Output = Result<DeleteCommentContentOutput, SdkError<DeleteCommentContentError>>> {
        self.deref().delete_comment_content(builder)
    }
    fn delete_file(&self, builder: DeleteFileInputBuilder) -> impl Future<Output = Result<DeleteFileOutput, SdkError<DeleteFileError>>> {
        self.deref().delete_file(builder)
    }
    fn delete_pull_request_approval_rule(&self, builder: DeletePullRequestApprovalRuleInputBuilder) -> impl Future<Output = Result<DeletePullRequestApprovalRuleOutput, SdkError<DeletePullRequestApprovalRuleError>>> {
        self.deref().delete_pull_request_approval_rule(builder)
    }
    fn delete_repository(&self, builder: DeleteRepositoryInputBuilder) -> impl Future<Output = Result<DeleteRepositoryOutput, SdkError<DeleteRepositoryError>>> {
        self.deref().delete_repository(builder)
    }
    fn describe_merge_conflicts(&self, builder: DescribeMergeConflictsInputBuilder) -> impl Future<Output = Result<DescribeMergeConflictsOutput, SdkError<DescribeMergeConflictsError>>> {
        self.deref().describe_merge_conflicts(builder)
    }
    fn describe_pull_request_events(&self, builder: DescribePullRequestEventsInputBuilder) -> impl Future<Output = Result<DescribePullRequestEventsOutput, SdkError<DescribePullRequestEventsError>>> {
        self.deref().describe_pull_request_events(builder)
    }
    fn disassociate_approval_rule_template_from_repository(&self, builder: DisassociateApprovalRuleTemplateFromRepositoryInputBuilder) -> impl Future<Output = Result<DisassociateApprovalRuleTemplateFromRepositoryOutput, SdkError<DisassociateApprovalRuleTemplateFromRepositoryError>>> {
        self.deref().disassociate_approval_rule_template_from_repository(builder)
    }
    fn evaluate_pull_request_approval_rules(&self, builder: EvaluatePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<EvaluatePullRequestApprovalRulesOutput, SdkError<EvaluatePullRequestApprovalRulesError>>> {
        self.deref().evaluate_pull_request_approval_rules(builder)
    }
    fn get_approval_rule_template(&self, builder: GetApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<GetApprovalRuleTemplateOutput, SdkError<GetApprovalRuleTemplateError>>> {
        self.deref().get_approval_rule_template(builder)
    }
    fn get_blob(&self, builder: GetBlobInputBuilder) -> impl Future<Output = Result<GetBlobOutput, SdkError<GetBlobError>>> {
        self.deref().get_blob(builder)
    }
    fn get_branch(&self, builder: GetBranchInputBuilder) -> impl Future<Output = Result<GetBranchOutput, SdkError<GetBranchError>>> {
        self.deref().get_branch(builder)
    }
    fn get_comment(&self, builder: GetCommentInputBuilder) -> impl Future<Output = Result<GetCommentOutput, SdkError<GetCommentError>>> {
        self.deref().get_comment(builder)
    }
    fn get_comment_reactions(&self, builder: GetCommentReactionsInputBuilder) -> impl Future<Output = Result<GetCommentReactionsOutput, SdkError<GetCommentReactionsError>>> {
        self.deref().get_comment_reactions(builder)
    }
    fn get_comments_for_compared_commit(&self, builder: GetCommentsForComparedCommitInputBuilder) -> impl Future<Output = Result<GetCommentsForComparedCommitOutput, SdkError<GetCommentsForComparedCommitError>>> {
        self.deref().get_comments_for_compared_commit(builder)
    }
    fn get_comments_for_pull_request(&self, builder: GetCommentsForPullRequestInputBuilder) -> impl Future<Output = Result<GetCommentsForPullRequestOutput, SdkError<GetCommentsForPullRequestError>>> {
        self.deref().get_comments_for_pull_request(builder)
    }
    fn get_commit(&self, builder: GetCommitInputBuilder) -> impl Future<Output = Result<GetCommitOutput, SdkError<GetCommitError>>> {
        self.deref().get_commit(builder)
    }
    fn get_differences(&self, builder: GetDifferencesInputBuilder) -> impl Future<Output = Result<GetDifferencesOutput, SdkError<GetDifferencesError>>> {
        self.deref().get_differences(builder)
    }
    fn get_file(&self, builder: GetFileInputBuilder) -> impl Future<Output = Result<GetFileOutput, SdkError<GetFileError>>> {
        self.deref().get_file(builder)
    }
    fn get_folder(&self, builder: GetFolderInputBuilder) -> impl Future<Output = Result<GetFolderOutput, SdkError<GetFolderError>>> {
        self.deref().get_folder(builder)
    }
    fn get_merge_commit(&self, builder: GetMergeCommitInputBuilder) -> impl Future<Output = Result<GetMergeCommitOutput, SdkError<GetMergeCommitError>>> {
        self.deref().get_merge_commit(builder)
    }
    fn get_merge_conflicts(&self, builder: GetMergeConflictsInputBuilder) -> impl Future<Output = Result<GetMergeConflictsOutput, SdkError<GetMergeConflictsError>>> {
        self.deref().get_merge_conflicts(builder)
    }
    fn get_merge_options(&self, builder: GetMergeOptionsInputBuilder) -> impl Future<Output = Result<GetMergeOptionsOutput, SdkError<GetMergeOptionsError>>> {
        self.deref().get_merge_options(builder)
    }
    fn get_pull_request(&self, builder: GetPullRequestInputBuilder) -> impl Future<Output = Result<GetPullRequestOutput, SdkError<GetPullRequestError>>> {
        self.deref().get_pull_request(builder)
    }
    fn get_pull_request_approval_states(&self, builder: GetPullRequestApprovalStatesInputBuilder) -> impl Future<Output = Result<GetPullRequestApprovalStatesOutput, SdkError<GetPullRequestApprovalStatesError>>> {
        self.deref().get_pull_request_approval_states(builder)
    }
    fn get_pull_request_override_state(&self, builder: GetPullRequestOverrideStateInputBuilder) -> impl Future<Output = Result<GetPullRequestOverrideStateOutput, SdkError<GetPullRequestOverrideStateError>>> {
        self.deref().get_pull_request_override_state(builder)
    }
    fn get_repository(&self, builder: GetRepositoryInputBuilder) -> impl Future<Output = Result<GetRepositoryOutput, SdkError<GetRepositoryError>>> {
        self.deref().get_repository(builder)
    }
    fn get_repository_triggers(&self, builder: GetRepositoryTriggersInputBuilder) -> impl Future<Output = Result<GetRepositoryTriggersOutput, SdkError<GetRepositoryTriggersError>>> {
        self.deref().get_repository_triggers(builder)
    }
    fn list_approval_rule_templates(&self, builder: ListApprovalRuleTemplatesInputBuilder) -> impl Future<Output = Result<ListApprovalRuleTemplatesOutput, SdkError<ListApprovalRuleTemplatesError>>> {
        self.deref().list_approval_rule_templates(builder)
    }
    fn list_associated_approval_rule_templates_for_repository(&self, builder: ListAssociatedApprovalRuleTemplatesForRepositoryInputBuilder) -> impl Future<Output = Result<ListAssociatedApprovalRuleTemplatesForRepositoryOutput, SdkError<ListAssociatedApprovalRuleTemplatesForRepositoryError>>> {
        self.deref().list_associated_approval_rule_templates_for_repository(builder)
    }
    fn list_branches(&self, builder: ListBranchesInputBuilder) -> impl Future<Output = Result<ListBranchesOutput, SdkError<ListBranchesError>>> {
        self.deref().list_branches(builder)
    }
    fn list_file_commit_history(&self, builder: ListFileCommitHistoryInputBuilder) -> impl Future<Output = Result<ListFileCommitHistoryOutput, SdkError<ListFileCommitHistoryError>>> {
        self.deref().list_file_commit_history(builder)
    }
    fn list_pull_requests(&self, builder: ListPullRequestsInputBuilder) -> impl Future<Output = Result<ListPullRequestsOutput, SdkError<ListPullRequestsError>>> {
        self.deref().list_pull_requests(builder)
    }
    fn list_repositories(&self, builder: ListRepositoriesInputBuilder) -> impl Future<Output = Result<ListRepositoriesOutput, SdkError<ListRepositoriesError>>> {
        self.deref().list_repositories(builder)
    }
    fn list_repositories_for_approval_rule_template(&self, builder: ListRepositoriesForApprovalRuleTemplateInputBuilder) -> impl Future<Output = Result<ListRepositoriesForApprovalRuleTemplateOutput, SdkError<ListRepositoriesForApprovalRuleTemplateError>>> {
        self.deref().list_repositories_for_approval_rule_template(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn merge_branches_by_fast_forward(&self, builder: MergeBranchesByFastForwardInputBuilder) -> impl Future<Output = Result<MergeBranchesByFastForwardOutput, SdkError<MergeBranchesByFastForwardError>>> {
        self.deref().merge_branches_by_fast_forward(builder)
    }
    fn merge_branches_by_squash(&self, builder: MergeBranchesBySquashInputBuilder) -> impl Future<Output = Result<MergeBranchesBySquashOutput, SdkError<MergeBranchesBySquashError>>> {
        self.deref().merge_branches_by_squash(builder)
    }
    fn merge_branches_by_three_way(&self, builder: MergeBranchesByThreeWayInputBuilder) -> impl Future<Output = Result<MergeBranchesByThreeWayOutput, SdkError<MergeBranchesByThreeWayError>>> {
        self.deref().merge_branches_by_three_way(builder)
    }
    fn merge_pull_request_by_fast_forward(&self, builder: MergePullRequestByFastForwardInputBuilder) -> impl Future<Output = Result<MergePullRequestByFastForwardOutput, SdkError<MergePullRequestByFastForwardError>>> {
        self.deref().merge_pull_request_by_fast_forward(builder)
    }
    fn merge_pull_request_by_squash(&self, builder: MergePullRequestBySquashInputBuilder) -> impl Future<Output = Result<MergePullRequestBySquashOutput, SdkError<MergePullRequestBySquashError>>> {
        self.deref().merge_pull_request_by_squash(builder)
    }
    fn merge_pull_request_by_three_way(&self, builder: MergePullRequestByThreeWayInputBuilder) -> impl Future<Output = Result<MergePullRequestByThreeWayOutput, SdkError<MergePullRequestByThreeWayError>>> {
        self.deref().merge_pull_request_by_three_way(builder)
    }
    fn override_pull_request_approval_rules(&self, builder: OverridePullRequestApprovalRulesInputBuilder) -> impl Future<Output = Result<OverridePullRequestApprovalRulesOutput, SdkError<OverridePullRequestApprovalRulesError>>> {
        self.deref().override_pull_request_approval_rules(builder)
    }
    fn post_comment_for_compared_commit(&self, builder: PostCommentForComparedCommitInputBuilder) -> impl Future<Output = Result<PostCommentForComparedCommitOutput, SdkError<PostCommentForComparedCommitError>>> {
        self.deref().post_comment_for_compared_commit(builder)
    }
    fn post_comment_for_pull_request(&self, builder: PostCommentForPullRequestInputBuilder) -> impl Future<Output = Result<PostCommentForPullRequestOutput, SdkError<PostCommentForPullRequestError>>> {
        self.deref().post_comment_for_pull_request(builder)
    }
    fn post_comment_reply(&self, builder: PostCommentReplyInputBuilder) -> impl Future<Output = Result<PostCommentReplyOutput, SdkError<PostCommentReplyError>>> {
        self.deref().post_comment_reply(builder)
    }
    fn put_comment_reaction(&self, builder: PutCommentReactionInputBuilder) -> impl Future<Output = Result<PutCommentReactionOutput, SdkError<PutCommentReactionError>>> {
        self.deref().put_comment_reaction(builder)
    }
    fn put_file(&self, builder: PutFileInputBuilder) -> impl Future<Output = Result<PutFileOutput, SdkError<PutFileError>>> {
        self.deref().put_file(builder)
    }
    fn put_repository_triggers(&self, builder: PutRepositoryTriggersInputBuilder) -> impl Future<Output = Result<PutRepositoryTriggersOutput, SdkError<PutRepositoryTriggersError>>> {
        self.deref().put_repository_triggers(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn test_repository_triggers(&self, builder: TestRepositoryTriggersInputBuilder) -> impl Future<Output = Result<TestRepositoryTriggersOutput, SdkError<TestRepositoryTriggersError>>> {
        self.deref().test_repository_triggers(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_approval_rule_template_content(&self, builder: UpdateApprovalRuleTemplateContentInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateContentOutput, SdkError<UpdateApprovalRuleTemplateContentError>>> {
        self.deref().update_approval_rule_template_content(builder)
    }
    fn update_approval_rule_template_description(&self, builder: UpdateApprovalRuleTemplateDescriptionInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateDescriptionOutput, SdkError<UpdateApprovalRuleTemplateDescriptionError>>> {
        self.deref().update_approval_rule_template_description(builder)
    }
    fn update_approval_rule_template_name(&self, builder: UpdateApprovalRuleTemplateNameInputBuilder) -> impl Future<Output = Result<UpdateApprovalRuleTemplateNameOutput, SdkError<UpdateApprovalRuleTemplateNameError>>> {
        self.deref().update_approval_rule_template_name(builder)
    }
    fn update_comment(&self, builder: UpdateCommentInputBuilder) -> impl Future<Output = Result<UpdateCommentOutput, SdkError<UpdateCommentError>>> {
        self.deref().update_comment(builder)
    }
    fn update_default_branch(&self, builder: UpdateDefaultBranchInputBuilder) -> impl Future<Output = Result<UpdateDefaultBranchOutput, SdkError<UpdateDefaultBranchError>>> {
        self.deref().update_default_branch(builder)
    }
    fn update_pull_request_approval_rule_content(&self, builder: UpdatePullRequestApprovalRuleContentInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalRuleContentOutput, SdkError<UpdatePullRequestApprovalRuleContentError>>> {
        self.deref().update_pull_request_approval_rule_content(builder)
    }
    fn update_pull_request_approval_state(&self, builder: UpdatePullRequestApprovalStateInputBuilder) -> impl Future<Output = Result<UpdatePullRequestApprovalStateOutput, SdkError<UpdatePullRequestApprovalStateError>>> {
        self.deref().update_pull_request_approval_state(builder)
    }
    fn update_pull_request_description(&self, builder: UpdatePullRequestDescriptionInputBuilder) -> impl Future<Output = Result<UpdatePullRequestDescriptionOutput, SdkError<UpdatePullRequestDescriptionError>>> {
        self.deref().update_pull_request_description(builder)
    }
    fn update_pull_request_status(&self, builder: UpdatePullRequestStatusInputBuilder) -> impl Future<Output = Result<UpdatePullRequestStatusOutput, SdkError<UpdatePullRequestStatusError>>> {
        self.deref().update_pull_request_status(builder)
    }
    fn update_pull_request_title(&self, builder: UpdatePullRequestTitleInputBuilder) -> impl Future<Output = Result<UpdatePullRequestTitleOutput, SdkError<UpdatePullRequestTitleError>>> {
        self.deref().update_pull_request_title(builder)
    }
    fn update_repository_description(&self, builder: UpdateRepositoryDescriptionInputBuilder) -> impl Future<Output = Result<UpdateRepositoryDescriptionOutput, SdkError<UpdateRepositoryDescriptionError>>> {
        self.deref().update_repository_description(builder)
    }
    fn update_repository_encryption_key(&self, builder: UpdateRepositoryEncryptionKeyInputBuilder) -> impl Future<Output = Result<UpdateRepositoryEncryptionKeyOutput, SdkError<UpdateRepositoryEncryptionKeyError>>> {
        self.deref().update_repository_encryption_key(builder)
    }
    fn update_repository_name(&self, builder: UpdateRepositoryNameInputBuilder) -> impl Future<Output = Result<UpdateRepositoryNameOutput, SdkError<UpdateRepositoryNameError>>> {
        self.deref().update_repository_name(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edCodeCommitClient {}
    impl CodeCommitClient for edCodeCommitClient {
        async fn associate_approval_rule_template_with_repository(&self, builder: AssociateApprovalRuleTemplateWithRepositoryInputBuilder) -> Result<AssociateApprovalRuleTemplateWithRepositoryOutput, SdkError<AssociateApprovalRuleTemplateWithRepositoryError>>;
        async fn batch_associate_approval_rule_template_with_repositories(&self, builder: BatchAssociateApprovalRuleTemplateWithRepositoriesInputBuilder) -> Result<BatchAssociateApprovalRuleTemplateWithRepositoriesOutput, SdkError<BatchAssociateApprovalRuleTemplateWithRepositoriesError>>;
        async fn batch_describe_merge_conflicts(&self, builder: BatchDescribeMergeConflictsInputBuilder) -> Result<BatchDescribeMergeConflictsOutput, SdkError<BatchDescribeMergeConflictsError>>;
        async fn batch_disassociate_approval_rule_template_from_repositories(&self, builder: BatchDisassociateApprovalRuleTemplateFromRepositoriesInputBuilder) -> Result<BatchDisassociateApprovalRuleTemplateFromRepositoriesOutput, SdkError<BatchDisassociateApprovalRuleTemplateFromRepositoriesError>>;
        async fn batch_get_commits(&self, builder: BatchGetCommitsInputBuilder) -> Result<BatchGetCommitsOutput, SdkError<BatchGetCommitsError>>;
        async fn batch_get_repositories(&self, builder: BatchGetRepositoriesInputBuilder) -> Result<BatchGetRepositoriesOutput, SdkError<BatchGetRepositoriesError>>;
        async fn create_approval_rule_template(&self, builder: CreateApprovalRuleTemplateInputBuilder) -> Result<CreateApprovalRuleTemplateOutput, SdkError<CreateApprovalRuleTemplateError>>;
        async fn create_branch(&self, builder: CreateBranchInputBuilder) -> Result<CreateBranchOutput, SdkError<CreateBranchError>>;
        async fn create_commit(&self, builder: CreateCommitInputBuilder) -> Result<CreateCommitOutput, SdkError<CreateCommitError>>;
        async fn create_pull_request(&self, builder: CreatePullRequestInputBuilder) -> Result<CreatePullRequestOutput, SdkError<CreatePullRequestError>>;
        async fn create_pull_request_approval_rule(&self, builder: CreatePullRequestApprovalRuleInputBuilder) -> Result<CreatePullRequestApprovalRuleOutput, SdkError<CreatePullRequestApprovalRuleError>>;
        async fn create_repository(&self, builder: CreateRepositoryInputBuilder) -> Result<CreateRepositoryOutput, SdkError<CreateRepositoryError>>;
        async fn create_unreferenced_merge_commit(&self, builder: CreateUnreferencedMergeCommitInputBuilder) -> Result<CreateUnreferencedMergeCommitOutput, SdkError<CreateUnreferencedMergeCommitError>>;
        async fn delete_approval_rule_template(&self, builder: DeleteApprovalRuleTemplateInputBuilder) -> Result<DeleteApprovalRuleTemplateOutput, SdkError<DeleteApprovalRuleTemplateError>>;
        async fn delete_branch(&self, builder: DeleteBranchInputBuilder) -> Result<DeleteBranchOutput, SdkError<DeleteBranchError>>;
        async fn delete_comment_content(&self, builder: DeleteCommentContentInputBuilder) -> Result<DeleteCommentContentOutput, SdkError<DeleteCommentContentError>>;
        async fn delete_file(&self, builder: DeleteFileInputBuilder) -> Result<DeleteFileOutput, SdkError<DeleteFileError>>;
        async fn delete_pull_request_approval_rule(&self, builder: DeletePullRequestApprovalRuleInputBuilder) -> Result<DeletePullRequestApprovalRuleOutput, SdkError<DeletePullRequestApprovalRuleError>>;
        async fn delete_repository(&self, builder: DeleteRepositoryInputBuilder) -> Result<DeleteRepositoryOutput, SdkError<DeleteRepositoryError>>;
        async fn describe_merge_conflicts(&self, builder: DescribeMergeConflictsInputBuilder) -> Result<DescribeMergeConflictsOutput, SdkError<DescribeMergeConflictsError>>;
        async fn describe_pull_request_events(&self, builder: DescribePullRequestEventsInputBuilder) -> Result<DescribePullRequestEventsOutput, SdkError<DescribePullRequestEventsError>>;
        async fn disassociate_approval_rule_template_from_repository(&self, builder: DisassociateApprovalRuleTemplateFromRepositoryInputBuilder) -> Result<DisassociateApprovalRuleTemplateFromRepositoryOutput, SdkError<DisassociateApprovalRuleTemplateFromRepositoryError>>;
        async fn evaluate_pull_request_approval_rules(&self, builder: EvaluatePullRequestApprovalRulesInputBuilder) -> Result<EvaluatePullRequestApprovalRulesOutput, SdkError<EvaluatePullRequestApprovalRulesError>>;
        async fn get_approval_rule_template(&self, builder: GetApprovalRuleTemplateInputBuilder) -> Result<GetApprovalRuleTemplateOutput, SdkError<GetApprovalRuleTemplateError>>;
        async fn get_blob(&self, builder: GetBlobInputBuilder) -> Result<GetBlobOutput, SdkError<GetBlobError>>;
        async fn get_branch(&self, builder: GetBranchInputBuilder) -> Result<GetBranchOutput, SdkError<GetBranchError>>;
        async fn get_comment(&self, builder: GetCommentInputBuilder) -> Result<GetCommentOutput, SdkError<GetCommentError>>;
        async fn get_comment_reactions(&self, builder: GetCommentReactionsInputBuilder) -> Result<GetCommentReactionsOutput, SdkError<GetCommentReactionsError>>;
        async fn get_comments_for_compared_commit(&self, builder: GetCommentsForComparedCommitInputBuilder) -> Result<GetCommentsForComparedCommitOutput, SdkError<GetCommentsForComparedCommitError>>;
        async fn get_comments_for_pull_request(&self, builder: GetCommentsForPullRequestInputBuilder) -> Result<GetCommentsForPullRequestOutput, SdkError<GetCommentsForPullRequestError>>;
        async fn get_commit(&self, builder: GetCommitInputBuilder) -> Result<GetCommitOutput, SdkError<GetCommitError>>;
        async fn get_differences(&self, builder: GetDifferencesInputBuilder) -> Result<GetDifferencesOutput, SdkError<GetDifferencesError>>;
        async fn get_file(&self, builder: GetFileInputBuilder) -> Result<GetFileOutput, SdkError<GetFileError>>;
        async fn get_folder(&self, builder: GetFolderInputBuilder) -> Result<GetFolderOutput, SdkError<GetFolderError>>;
        async fn get_merge_commit(&self, builder: GetMergeCommitInputBuilder) -> Result<GetMergeCommitOutput, SdkError<GetMergeCommitError>>;
        async fn get_merge_conflicts(&self, builder: GetMergeConflictsInputBuilder) -> Result<GetMergeConflictsOutput, SdkError<GetMergeConflictsError>>;
        async fn get_merge_options(&self, builder: GetMergeOptionsInputBuilder) -> Result<GetMergeOptionsOutput, SdkError<GetMergeOptionsError>>;
        async fn get_pull_request(&self, builder: GetPullRequestInputBuilder) -> Result<GetPullRequestOutput, SdkError<GetPullRequestError>>;
        async fn get_pull_request_approval_states(&self, builder: GetPullRequestApprovalStatesInputBuilder) -> Result<GetPullRequestApprovalStatesOutput, SdkError<GetPullRequestApprovalStatesError>>;
        async fn get_pull_request_override_state(&self, builder: GetPullRequestOverrideStateInputBuilder) -> Result<GetPullRequestOverrideStateOutput, SdkError<GetPullRequestOverrideStateError>>;
        async fn get_repository(&self, builder: GetRepositoryInputBuilder) -> Result<GetRepositoryOutput, SdkError<GetRepositoryError>>;
        async fn get_repository_triggers(&self, builder: GetRepositoryTriggersInputBuilder) -> Result<GetRepositoryTriggersOutput, SdkError<GetRepositoryTriggersError>>;
        async fn list_approval_rule_templates(&self, builder: ListApprovalRuleTemplatesInputBuilder) -> Result<ListApprovalRuleTemplatesOutput, SdkError<ListApprovalRuleTemplatesError>>;
        async fn list_associated_approval_rule_templates_for_repository(&self, builder: ListAssociatedApprovalRuleTemplatesForRepositoryInputBuilder) -> Result<ListAssociatedApprovalRuleTemplatesForRepositoryOutput, SdkError<ListAssociatedApprovalRuleTemplatesForRepositoryError>>;
        async fn list_branches(&self, builder: ListBranchesInputBuilder) -> Result<ListBranchesOutput, SdkError<ListBranchesError>>;
        async fn list_file_commit_history(&self, builder: ListFileCommitHistoryInputBuilder) -> Result<ListFileCommitHistoryOutput, SdkError<ListFileCommitHistoryError>>;
        async fn list_pull_requests(&self, builder: ListPullRequestsInputBuilder) -> Result<ListPullRequestsOutput, SdkError<ListPullRequestsError>>;
        async fn list_repositories(&self, builder: ListRepositoriesInputBuilder) -> Result<ListRepositoriesOutput, SdkError<ListRepositoriesError>>;
        async fn list_repositories_for_approval_rule_template(&self, builder: ListRepositoriesForApprovalRuleTemplateInputBuilder) -> Result<ListRepositoriesForApprovalRuleTemplateOutput, SdkError<ListRepositoriesForApprovalRuleTemplateError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn merge_branches_by_fast_forward(&self, builder: MergeBranchesByFastForwardInputBuilder) -> Result<MergeBranchesByFastForwardOutput, SdkError<MergeBranchesByFastForwardError>>;
        async fn merge_branches_by_squash(&self, builder: MergeBranchesBySquashInputBuilder) -> Result<MergeBranchesBySquashOutput, SdkError<MergeBranchesBySquashError>>;
        async fn merge_branches_by_three_way(&self, builder: MergeBranchesByThreeWayInputBuilder) -> Result<MergeBranchesByThreeWayOutput, SdkError<MergeBranchesByThreeWayError>>;
        async fn merge_pull_request_by_fast_forward(&self, builder: MergePullRequestByFastForwardInputBuilder) -> Result<MergePullRequestByFastForwardOutput, SdkError<MergePullRequestByFastForwardError>>;
        async fn merge_pull_request_by_squash(&self, builder: MergePullRequestBySquashInputBuilder) -> Result<MergePullRequestBySquashOutput, SdkError<MergePullRequestBySquashError>>;
        async fn merge_pull_request_by_three_way(&self, builder: MergePullRequestByThreeWayInputBuilder) -> Result<MergePullRequestByThreeWayOutput, SdkError<MergePullRequestByThreeWayError>>;
        async fn override_pull_request_approval_rules(&self, builder: OverridePullRequestApprovalRulesInputBuilder) -> Result<OverridePullRequestApprovalRulesOutput, SdkError<OverridePullRequestApprovalRulesError>>;
        async fn post_comment_for_compared_commit(&self, builder: PostCommentForComparedCommitInputBuilder) -> Result<PostCommentForComparedCommitOutput, SdkError<PostCommentForComparedCommitError>>;
        async fn post_comment_for_pull_request(&self, builder: PostCommentForPullRequestInputBuilder) -> Result<PostCommentForPullRequestOutput, SdkError<PostCommentForPullRequestError>>;
        async fn post_comment_reply(&self, builder: PostCommentReplyInputBuilder) -> Result<PostCommentReplyOutput, SdkError<PostCommentReplyError>>;
        async fn put_comment_reaction(&self, builder: PutCommentReactionInputBuilder) -> Result<PutCommentReactionOutput, SdkError<PutCommentReactionError>>;
        async fn put_file(&self, builder: PutFileInputBuilder) -> Result<PutFileOutput, SdkError<PutFileError>>;
        async fn put_repository_triggers(&self, builder: PutRepositoryTriggersInputBuilder) -> Result<PutRepositoryTriggersOutput, SdkError<PutRepositoryTriggersError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn test_repository_triggers(&self, builder: TestRepositoryTriggersInputBuilder) -> Result<TestRepositoryTriggersOutput, SdkError<TestRepositoryTriggersError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_approval_rule_template_content(&self, builder: UpdateApprovalRuleTemplateContentInputBuilder) -> Result<UpdateApprovalRuleTemplateContentOutput, SdkError<UpdateApprovalRuleTemplateContentError>>;
        async fn update_approval_rule_template_description(&self, builder: UpdateApprovalRuleTemplateDescriptionInputBuilder) -> Result<UpdateApprovalRuleTemplateDescriptionOutput, SdkError<UpdateApprovalRuleTemplateDescriptionError>>;
        async fn update_approval_rule_template_name(&self, builder: UpdateApprovalRuleTemplateNameInputBuilder) -> Result<UpdateApprovalRuleTemplateNameOutput, SdkError<UpdateApprovalRuleTemplateNameError>>;
        async fn update_comment(&self, builder: UpdateCommentInputBuilder) -> Result<UpdateCommentOutput, SdkError<UpdateCommentError>>;
        async fn update_default_branch(&self, builder: UpdateDefaultBranchInputBuilder) -> Result<UpdateDefaultBranchOutput, SdkError<UpdateDefaultBranchError>>;
        async fn update_pull_request_approval_rule_content(&self, builder: UpdatePullRequestApprovalRuleContentInputBuilder) -> Result<UpdatePullRequestApprovalRuleContentOutput, SdkError<UpdatePullRequestApprovalRuleContentError>>;
        async fn update_pull_request_approval_state(&self, builder: UpdatePullRequestApprovalStateInputBuilder) -> Result<UpdatePullRequestApprovalStateOutput, SdkError<UpdatePullRequestApprovalStateError>>;
        async fn update_pull_request_description(&self, builder: UpdatePullRequestDescriptionInputBuilder) -> Result<UpdatePullRequestDescriptionOutput, SdkError<UpdatePullRequestDescriptionError>>;
        async fn update_pull_request_status(&self, builder: UpdatePullRequestStatusInputBuilder) -> Result<UpdatePullRequestStatusOutput, SdkError<UpdatePullRequestStatusError>>;
        async fn update_pull_request_title(&self, builder: UpdatePullRequestTitleInputBuilder) -> Result<UpdatePullRequestTitleOutput, SdkError<UpdatePullRequestTitleError>>;
        async fn update_repository_description(&self, builder: UpdateRepositoryDescriptionInputBuilder) -> Result<UpdateRepositoryDescriptionOutput, SdkError<UpdateRepositoryDescriptionError>>;
        async fn update_repository_encryption_key(&self, builder: UpdateRepositoryEncryptionKeyInputBuilder) -> Result<UpdateRepositoryEncryptionKeyOutput, SdkError<UpdateRepositoryEncryptionKeyError>>;
        async fn update_repository_name(&self, builder: UpdateRepositoryNameInputBuilder) -> Result<UpdateRepositoryNameOutput, SdkError<UpdateRepositoryNameError>>;
    }
}
