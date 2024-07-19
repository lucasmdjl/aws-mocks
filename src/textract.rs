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
use aws_sdk_textract::operation::analyze_document::{builders::*, *};
use aws_sdk_textract::operation::analyze_expense::{builders::*, *};
use aws_sdk_textract::operation::analyze_id::{builders::*, *};
use aws_sdk_textract::operation::create_adapter::{builders::*, *};
use aws_sdk_textract::operation::create_adapter_version::{builders::*, *};
use aws_sdk_textract::operation::delete_adapter::{builders::*, *};
use aws_sdk_textract::operation::delete_adapter_version::{builders::*, *};
use aws_sdk_textract::operation::detect_document_text::{builders::*, *};
use aws_sdk_textract::operation::get_adapter::{builders::*, *};
use aws_sdk_textract::operation::get_adapter_version::{builders::*, *};
use aws_sdk_textract::operation::get_document_analysis::{builders::*, *};
use aws_sdk_textract::operation::get_document_text_detection::{builders::*, *};
use aws_sdk_textract::operation::get_expense_analysis::{builders::*, *};
use aws_sdk_textract::operation::get_lending_analysis::{builders::*, *};
use aws_sdk_textract::operation::get_lending_analysis_summary::{builders::*, *};
use aws_sdk_textract::operation::list_adapter_versions::{builders::*, *};
use aws_sdk_textract::operation::list_adapters::{builders::*, *};
use aws_sdk_textract::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_textract::operation::start_document_analysis::{builders::*, *};
use aws_sdk_textract::operation::start_document_text_detection::{builders::*, *};
use aws_sdk_textract::operation::start_expense_analysis::{builders::*, *};
use aws_sdk_textract::operation::start_lending_analysis::{builders::*, *};
use aws_sdk_textract::operation::tag_resource::{builders::*, *};
use aws_sdk_textract::operation::untag_resource::{builders::*, *};
use aws_sdk_textract::operation::update_adapter::{builders::*, *};
use aws_sdk_textract::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_textract::Client;

pub use aws_sdk_textract::*;

pub struct TextractClientImpl(Client);
impl TextractClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait TextractClient {
    fn analyze_document(&self, builder: AnalyzeDocumentInputBuilder) -> impl Future<Output = Result<AnalyzeDocumentOutput, SdkError<AnalyzeDocumentError>>>;
    fn analyze_expense(&self, builder: AnalyzeExpenseInputBuilder) -> impl Future<Output = Result<AnalyzeExpenseOutput, SdkError<AnalyzeExpenseError>>>;
    fn analyze_id(&self, builder: AnalyzeIdInputBuilder) -> impl Future<Output = Result<AnalyzeIdOutput, SdkError<AnalyzeIDError>>>;
    fn create_adapter(&self, builder: CreateAdapterInputBuilder) -> impl Future<Output = Result<CreateAdapterOutput, SdkError<CreateAdapterError>>>;
    fn create_adapter_version(&self, builder: CreateAdapterVersionInputBuilder) -> impl Future<Output = Result<CreateAdapterVersionOutput, SdkError<CreateAdapterVersionError>>>;
    fn delete_adapter(&self, builder: DeleteAdapterInputBuilder) -> impl Future<Output = Result<DeleteAdapterOutput, SdkError<DeleteAdapterError>>>;
    fn delete_adapter_version(&self, builder: DeleteAdapterVersionInputBuilder) -> impl Future<Output = Result<DeleteAdapterVersionOutput, SdkError<DeleteAdapterVersionError>>>;
    fn detect_document_text(&self, builder: DetectDocumentTextInputBuilder) -> impl Future<Output = Result<DetectDocumentTextOutput, SdkError<DetectDocumentTextError>>>;
    fn get_adapter(&self, builder: GetAdapterInputBuilder) -> impl Future<Output = Result<GetAdapterOutput, SdkError<GetAdapterError>>>;
    fn get_adapter_version(&self, builder: GetAdapterVersionInputBuilder) -> impl Future<Output = Result<GetAdapterVersionOutput, SdkError<GetAdapterVersionError>>>;
    fn get_document_analysis(&self, builder: GetDocumentAnalysisInputBuilder) -> impl Future<Output = Result<GetDocumentAnalysisOutput, SdkError<GetDocumentAnalysisError>>>;
    fn get_document_text_detection(&self, builder: GetDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<GetDocumentTextDetectionOutput, SdkError<GetDocumentTextDetectionError>>>;
    fn get_expense_analysis(&self, builder: GetExpenseAnalysisInputBuilder) -> impl Future<Output = Result<GetExpenseAnalysisOutput, SdkError<GetExpenseAnalysisError>>>;
    fn get_lending_analysis(&self, builder: GetLendingAnalysisInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisOutput, SdkError<GetLendingAnalysisError>>>;
    fn get_lending_analysis_summary(&self, builder: GetLendingAnalysisSummaryInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisSummaryOutput, SdkError<GetLendingAnalysisSummaryError>>>;
    fn list_adapter_versions(&self, builder: ListAdapterVersionsInputBuilder) -> impl Future<Output = Result<ListAdapterVersionsOutput, SdkError<ListAdapterVersionsError>>>;
    fn list_adapters(&self, builder: ListAdaptersInputBuilder) -> impl Future<Output = Result<ListAdaptersOutput, SdkError<ListAdaptersError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn start_document_analysis(&self, builder: StartDocumentAnalysisInputBuilder) -> impl Future<Output = Result<StartDocumentAnalysisOutput, SdkError<StartDocumentAnalysisError>>>;
    fn start_document_text_detection(&self, builder: StartDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<StartDocumentTextDetectionOutput, SdkError<StartDocumentTextDetectionError>>>;
    fn start_expense_analysis(&self, builder: StartExpenseAnalysisInputBuilder) -> impl Future<Output = Result<StartExpenseAnalysisOutput, SdkError<StartExpenseAnalysisError>>>;
    fn start_lending_analysis(&self, builder: StartLendingAnalysisInputBuilder) -> impl Future<Output = Result<StartLendingAnalysisOutput, SdkError<StartLendingAnalysisError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_adapter(&self, builder: UpdateAdapterInputBuilder) -> impl Future<Output = Result<UpdateAdapterOutput, SdkError<UpdateAdapterError>>>;
}
impl TextractClient for TextractClientImpl {
    fn analyze_document(&self, builder: AnalyzeDocumentInputBuilder) -> impl Future<Output = Result<AnalyzeDocumentOutput, SdkError<AnalyzeDocumentError>>> {
        builder.send_with(&self.0)
    }
    fn analyze_expense(&self, builder: AnalyzeExpenseInputBuilder) -> impl Future<Output = Result<AnalyzeExpenseOutput, SdkError<AnalyzeExpenseError>>> {
        builder.send_with(&self.0)
    }
    fn analyze_id(&self, builder: AnalyzeIdInputBuilder) -> impl Future<Output = Result<AnalyzeIdOutput, SdkError<AnalyzeIDError>>> {
        builder.send_with(&self.0)
    }
    fn create_adapter(&self, builder: CreateAdapterInputBuilder) -> impl Future<Output = Result<CreateAdapterOutput, SdkError<CreateAdapterError>>> {
        builder.send_with(&self.0)
    }
    fn create_adapter_version(&self, builder: CreateAdapterVersionInputBuilder) -> impl Future<Output = Result<CreateAdapterVersionOutput, SdkError<CreateAdapterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_adapter(&self, builder: DeleteAdapterInputBuilder) -> impl Future<Output = Result<DeleteAdapterOutput, SdkError<DeleteAdapterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_adapter_version(&self, builder: DeleteAdapterVersionInputBuilder) -> impl Future<Output = Result<DeleteAdapterVersionOutput, SdkError<DeleteAdapterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn detect_document_text(&self, builder: DetectDocumentTextInputBuilder) -> impl Future<Output = Result<DetectDocumentTextOutput, SdkError<DetectDocumentTextError>>> {
        builder.send_with(&self.0)
    }
    fn get_adapter(&self, builder: GetAdapterInputBuilder) -> impl Future<Output = Result<GetAdapterOutput, SdkError<GetAdapterError>>> {
        builder.send_with(&self.0)
    }
    fn get_adapter_version(&self, builder: GetAdapterVersionInputBuilder) -> impl Future<Output = Result<GetAdapterVersionOutput, SdkError<GetAdapterVersionError>>> {
        builder.send_with(&self.0)
    }
    fn get_document_analysis(&self, builder: GetDocumentAnalysisInputBuilder) -> impl Future<Output = Result<GetDocumentAnalysisOutput, SdkError<GetDocumentAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn get_document_text_detection(&self, builder: GetDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<GetDocumentTextDetectionOutput, SdkError<GetDocumentTextDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_expense_analysis(&self, builder: GetExpenseAnalysisInputBuilder) -> impl Future<Output = Result<GetExpenseAnalysisOutput, SdkError<GetExpenseAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn get_lending_analysis(&self, builder: GetLendingAnalysisInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisOutput, SdkError<GetLendingAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn get_lending_analysis_summary(&self, builder: GetLendingAnalysisSummaryInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisSummaryOutput, SdkError<GetLendingAnalysisSummaryError>>> {
        builder.send_with(&self.0)
    }
    fn list_adapter_versions(&self, builder: ListAdapterVersionsInputBuilder) -> impl Future<Output = Result<ListAdapterVersionsOutput, SdkError<ListAdapterVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_adapters(&self, builder: ListAdaptersInputBuilder) -> impl Future<Output = Result<ListAdaptersOutput, SdkError<ListAdaptersError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn start_document_analysis(&self, builder: StartDocumentAnalysisInputBuilder) -> impl Future<Output = Result<StartDocumentAnalysisOutput, SdkError<StartDocumentAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn start_document_text_detection(&self, builder: StartDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<StartDocumentTextDetectionOutput, SdkError<StartDocumentTextDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn start_expense_analysis(&self, builder: StartExpenseAnalysisInputBuilder) -> impl Future<Output = Result<StartExpenseAnalysisOutput, SdkError<StartExpenseAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn start_lending_analysis(&self, builder: StartLendingAnalysisInputBuilder) -> impl Future<Output = Result<StartLendingAnalysisOutput, SdkError<StartLendingAnalysisError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_adapter(&self, builder: UpdateAdapterInputBuilder) -> impl Future<Output = Result<UpdateAdapterOutput, SdkError<UpdateAdapterError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: TextractClient> TextractClient for &T {
    fn analyze_document(&self, builder: AnalyzeDocumentInputBuilder) -> impl Future<Output = Result<AnalyzeDocumentOutput, SdkError<AnalyzeDocumentError>>> {
        (*self).analyze_document(builder)
    }
    fn analyze_expense(&self, builder: AnalyzeExpenseInputBuilder) -> impl Future<Output = Result<AnalyzeExpenseOutput, SdkError<AnalyzeExpenseError>>> {
        (*self).analyze_expense(builder)
    }
    fn analyze_id(&self, builder: AnalyzeIdInputBuilder) -> impl Future<Output = Result<AnalyzeIdOutput, SdkError<AnalyzeIDError>>> {
        (*self).analyze_id(builder)
    }
    fn create_adapter(&self, builder: CreateAdapterInputBuilder) -> impl Future<Output = Result<CreateAdapterOutput, SdkError<CreateAdapterError>>> {
        (*self).create_adapter(builder)
    }
    fn create_adapter_version(&self, builder: CreateAdapterVersionInputBuilder) -> impl Future<Output = Result<CreateAdapterVersionOutput, SdkError<CreateAdapterVersionError>>> {
        (*self).create_adapter_version(builder)
    }
    fn delete_adapter(&self, builder: DeleteAdapterInputBuilder) -> impl Future<Output = Result<DeleteAdapterOutput, SdkError<DeleteAdapterError>>> {
        (*self).delete_adapter(builder)
    }
    fn delete_adapter_version(&self, builder: DeleteAdapterVersionInputBuilder) -> impl Future<Output = Result<DeleteAdapterVersionOutput, SdkError<DeleteAdapterVersionError>>> {
        (*self).delete_adapter_version(builder)
    }
    fn detect_document_text(&self, builder: DetectDocumentTextInputBuilder) -> impl Future<Output = Result<DetectDocumentTextOutput, SdkError<DetectDocumentTextError>>> {
        (*self).detect_document_text(builder)
    }
    fn get_adapter(&self, builder: GetAdapterInputBuilder) -> impl Future<Output = Result<GetAdapterOutput, SdkError<GetAdapterError>>> {
        (*self).get_adapter(builder)
    }
    fn get_adapter_version(&self, builder: GetAdapterVersionInputBuilder) -> impl Future<Output = Result<GetAdapterVersionOutput, SdkError<GetAdapterVersionError>>> {
        (*self).get_adapter_version(builder)
    }
    fn get_document_analysis(&self, builder: GetDocumentAnalysisInputBuilder) -> impl Future<Output = Result<GetDocumentAnalysisOutput, SdkError<GetDocumentAnalysisError>>> {
        (*self).get_document_analysis(builder)
    }
    fn get_document_text_detection(&self, builder: GetDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<GetDocumentTextDetectionOutput, SdkError<GetDocumentTextDetectionError>>> {
        (*self).get_document_text_detection(builder)
    }
    fn get_expense_analysis(&self, builder: GetExpenseAnalysisInputBuilder) -> impl Future<Output = Result<GetExpenseAnalysisOutput, SdkError<GetExpenseAnalysisError>>> {
        (*self).get_expense_analysis(builder)
    }
    fn get_lending_analysis(&self, builder: GetLendingAnalysisInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisOutput, SdkError<GetLendingAnalysisError>>> {
        (*self).get_lending_analysis(builder)
    }
    fn get_lending_analysis_summary(&self, builder: GetLendingAnalysisSummaryInputBuilder) -> impl Future<Output = Result<GetLendingAnalysisSummaryOutput, SdkError<GetLendingAnalysisSummaryError>>> {
        (*self).get_lending_analysis_summary(builder)
    }
    fn list_adapter_versions(&self, builder: ListAdapterVersionsInputBuilder) -> impl Future<Output = Result<ListAdapterVersionsOutput, SdkError<ListAdapterVersionsError>>> {
        (*self).list_adapter_versions(builder)
    }
    fn list_adapters(&self, builder: ListAdaptersInputBuilder) -> impl Future<Output = Result<ListAdaptersOutput, SdkError<ListAdaptersError>>> {
        (*self).list_adapters(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn start_document_analysis(&self, builder: StartDocumentAnalysisInputBuilder) -> impl Future<Output = Result<StartDocumentAnalysisOutput, SdkError<StartDocumentAnalysisError>>> {
        (*self).start_document_analysis(builder)
    }
    fn start_document_text_detection(&self, builder: StartDocumentTextDetectionInputBuilder) -> impl Future<Output = Result<StartDocumentTextDetectionOutput, SdkError<StartDocumentTextDetectionError>>> {
        (*self).start_document_text_detection(builder)
    }
    fn start_expense_analysis(&self, builder: StartExpenseAnalysisInputBuilder) -> impl Future<Output = Result<StartExpenseAnalysisOutput, SdkError<StartExpenseAnalysisError>>> {
        (*self).start_expense_analysis(builder)
    }
    fn start_lending_analysis(&self, builder: StartLendingAnalysisInputBuilder) -> impl Future<Output = Result<StartLendingAnalysisOutput, SdkError<StartLendingAnalysisError>>> {
        (*self).start_lending_analysis(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_adapter(&self, builder: UpdateAdapterInputBuilder) -> impl Future<Output = Result<UpdateAdapterOutput, SdkError<UpdateAdapterError>>> {
        (*self).update_adapter(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edTextractClient {}
    impl TextractClient for edTextractClient {
        async fn analyze_document(&self, builder: AnalyzeDocumentInputBuilder) -> Result<AnalyzeDocumentOutput, SdkError<AnalyzeDocumentError>>;
        async fn analyze_expense(&self, builder: AnalyzeExpenseInputBuilder) -> Result<AnalyzeExpenseOutput, SdkError<AnalyzeExpenseError>>;
        async fn analyze_id(&self, builder: AnalyzeIdInputBuilder) -> Result<AnalyzeIdOutput, SdkError<AnalyzeIDError>>;
        async fn create_adapter(&self, builder: CreateAdapterInputBuilder) -> Result<CreateAdapterOutput, SdkError<CreateAdapterError>>;
        async fn create_adapter_version(&self, builder: CreateAdapterVersionInputBuilder) -> Result<CreateAdapterVersionOutput, SdkError<CreateAdapterVersionError>>;
        async fn delete_adapter(&self, builder: DeleteAdapterInputBuilder) -> Result<DeleteAdapterOutput, SdkError<DeleteAdapterError>>;
        async fn delete_adapter_version(&self, builder: DeleteAdapterVersionInputBuilder) -> Result<DeleteAdapterVersionOutput, SdkError<DeleteAdapterVersionError>>;
        async fn detect_document_text(&self, builder: DetectDocumentTextInputBuilder) -> Result<DetectDocumentTextOutput, SdkError<DetectDocumentTextError>>;
        async fn get_adapter(&self, builder: GetAdapterInputBuilder) -> Result<GetAdapterOutput, SdkError<GetAdapterError>>;
        async fn get_adapter_version(&self, builder: GetAdapterVersionInputBuilder) -> Result<GetAdapterVersionOutput, SdkError<GetAdapterVersionError>>;
        async fn get_document_analysis(&self, builder: GetDocumentAnalysisInputBuilder) -> Result<GetDocumentAnalysisOutput, SdkError<GetDocumentAnalysisError>>;
        async fn get_document_text_detection(&self, builder: GetDocumentTextDetectionInputBuilder) -> Result<GetDocumentTextDetectionOutput, SdkError<GetDocumentTextDetectionError>>;
        async fn get_expense_analysis(&self, builder: GetExpenseAnalysisInputBuilder) -> Result<GetExpenseAnalysisOutput, SdkError<GetExpenseAnalysisError>>;
        async fn get_lending_analysis(&self, builder: GetLendingAnalysisInputBuilder) -> Result<GetLendingAnalysisOutput, SdkError<GetLendingAnalysisError>>;
        async fn get_lending_analysis_summary(&self, builder: GetLendingAnalysisSummaryInputBuilder) -> Result<GetLendingAnalysisSummaryOutput, SdkError<GetLendingAnalysisSummaryError>>;
        async fn list_adapter_versions(&self, builder: ListAdapterVersionsInputBuilder) -> Result<ListAdapterVersionsOutput, SdkError<ListAdapterVersionsError>>;
        async fn list_adapters(&self, builder: ListAdaptersInputBuilder) -> Result<ListAdaptersOutput, SdkError<ListAdaptersError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn start_document_analysis(&self, builder: StartDocumentAnalysisInputBuilder) -> Result<StartDocumentAnalysisOutput, SdkError<StartDocumentAnalysisError>>;
        async fn start_document_text_detection(&self, builder: StartDocumentTextDetectionInputBuilder) -> Result<StartDocumentTextDetectionOutput, SdkError<StartDocumentTextDetectionError>>;
        async fn start_expense_analysis(&self, builder: StartExpenseAnalysisInputBuilder) -> Result<StartExpenseAnalysisOutput, SdkError<StartExpenseAnalysisError>>;
        async fn start_lending_analysis(&self, builder: StartLendingAnalysisInputBuilder) -> Result<StartLendingAnalysisOutput, SdkError<StartLendingAnalysisError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_adapter(&self, builder: UpdateAdapterInputBuilder) -> Result<UpdateAdapterOutput, SdkError<UpdateAdapterError>>;
    }
}
