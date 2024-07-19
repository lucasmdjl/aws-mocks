/*
 * aws_mock - A mocking library for AWS.
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
use aws_sdk_rekognition::operation::associate_faces::{builders::*, *};
use aws_sdk_rekognition::operation::compare_faces::{builders::*, *};
use aws_sdk_rekognition::operation::copy_project_version::{builders::*, *};
use aws_sdk_rekognition::operation::create_collection::{builders::*, *};
use aws_sdk_rekognition::operation::create_dataset::{builders::*, *};
use aws_sdk_rekognition::operation::create_face_liveness_session::{builders::*, *};
use aws_sdk_rekognition::operation::create_project::{builders::*, *};
use aws_sdk_rekognition::operation::create_project_version::{builders::*, *};
use aws_sdk_rekognition::operation::create_stream_processor::{builders::*, *};
use aws_sdk_rekognition::operation::create_user::{builders::*, *};
use aws_sdk_rekognition::operation::delete_collection::{builders::*, *};
use aws_sdk_rekognition::operation::delete_dataset::{builders::*, *};
use aws_sdk_rekognition::operation::delete_faces::{builders::*, *};
use aws_sdk_rekognition::operation::delete_project::{builders::*, *};
use aws_sdk_rekognition::operation::delete_project_policy::{builders::*, *};
use aws_sdk_rekognition::operation::delete_project_version::{builders::*, *};
use aws_sdk_rekognition::operation::delete_stream_processor::{builders::*, *};
use aws_sdk_rekognition::operation::delete_user::{builders::*, *};
use aws_sdk_rekognition::operation::describe_collection::{builders::*, *};
use aws_sdk_rekognition::operation::describe_dataset::{builders::*, *};
use aws_sdk_rekognition::operation::describe_project_versions::{builders::*, *};
use aws_sdk_rekognition::operation::describe_projects::{builders::*, *};
use aws_sdk_rekognition::operation::describe_stream_processor::{builders::*, *};
use aws_sdk_rekognition::operation::detect_custom_labels::{builders::*, *};
use aws_sdk_rekognition::operation::detect_faces::{builders::*, *};
use aws_sdk_rekognition::operation::detect_labels::{builders::*, *};
use aws_sdk_rekognition::operation::detect_moderation_labels::{builders::*, *};
use aws_sdk_rekognition::operation::detect_protective_equipment::{builders::*, *};
use aws_sdk_rekognition::operation::detect_text::{builders::*, *};
use aws_sdk_rekognition::operation::disassociate_faces::{builders::*, *};
use aws_sdk_rekognition::operation::distribute_dataset_entries::{builders::*, *};
use aws_sdk_rekognition::operation::get_celebrity_info::{builders::*, *};
use aws_sdk_rekognition::operation::get_celebrity_recognition::{builders::*, *};
use aws_sdk_rekognition::operation::get_content_moderation::{builders::*, *};
use aws_sdk_rekognition::operation::get_face_detection::{builders::*, *};
use aws_sdk_rekognition::operation::get_face_liveness_session_results::{builders::*, *};
use aws_sdk_rekognition::operation::get_face_search::{builders::*, *};
use aws_sdk_rekognition::operation::get_label_detection::{builders::*, *};
use aws_sdk_rekognition::operation::get_media_analysis_job::{builders::*, *};
use aws_sdk_rekognition::operation::get_person_tracking::{builders::*, *};
use aws_sdk_rekognition::operation::get_segment_detection::{builders::*, *};
use aws_sdk_rekognition::operation::get_text_detection::{builders::*, *};
use aws_sdk_rekognition::operation::index_faces::{builders::*, *};
use aws_sdk_rekognition::operation::list_collections::{builders::*, *};
use aws_sdk_rekognition::operation::list_dataset_entries::{builders::*, *};
use aws_sdk_rekognition::operation::list_dataset_labels::{builders::*, *};
use aws_sdk_rekognition::operation::list_faces::{builders::*, *};
use aws_sdk_rekognition::operation::list_media_analysis_jobs::{builders::*, *};
use aws_sdk_rekognition::operation::list_project_policies::{builders::*, *};
use aws_sdk_rekognition::operation::list_stream_processors::{builders::*, *};
use aws_sdk_rekognition::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_rekognition::operation::list_users::{builders::*, *};
use aws_sdk_rekognition::operation::put_project_policy::{builders::*, *};
use aws_sdk_rekognition::operation::recognize_celebrities::{builders::*, *};
use aws_sdk_rekognition::operation::search_faces::{builders::*, *};
use aws_sdk_rekognition::operation::search_faces_by_image::{builders::*, *};
use aws_sdk_rekognition::operation::search_users::{builders::*, *};
use aws_sdk_rekognition::operation::search_users_by_image::{builders::*, *};
use aws_sdk_rekognition::operation::start_celebrity_recognition::{builders::*, *};
use aws_sdk_rekognition::operation::start_content_moderation::{builders::*, *};
use aws_sdk_rekognition::operation::start_face_detection::{builders::*, *};
use aws_sdk_rekognition::operation::start_face_search::{builders::*, *};
use aws_sdk_rekognition::operation::start_label_detection::{builders::*, *};
use aws_sdk_rekognition::operation::start_media_analysis_job::{builders::*, *};
use aws_sdk_rekognition::operation::start_person_tracking::{builders::*, *};
use aws_sdk_rekognition::operation::start_project_version::{builders::*, *};
use aws_sdk_rekognition::operation::start_segment_detection::{builders::*, *};
use aws_sdk_rekognition::operation::start_stream_processor::{builders::*, *};
use aws_sdk_rekognition::operation::start_text_detection::{builders::*, *};
use aws_sdk_rekognition::operation::stop_project_version::{builders::*, *};
use aws_sdk_rekognition::operation::stop_stream_processor::{builders::*, *};
use aws_sdk_rekognition::operation::tag_resource::{builders::*, *};
use aws_sdk_rekognition::operation::untag_resource::{builders::*, *};
use aws_sdk_rekognition::operation::update_dataset_entries::{builders::*, *};
use aws_sdk_rekognition::operation::update_stream_processor::{builders::*, *};
use aws_sdk_rekognition::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_rekognition::Client;

pub use aws_sdk_rekognition::*;

pub struct RekognitionClientImpl(Client);
impl RekognitionClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait RekognitionClient {
    fn associate_faces(&self, builder: AssociateFacesInputBuilder) -> impl Future<Output = Result<AssociateFacesOutput, SdkError<AssociateFacesError>>>;
    fn compare_faces(&self, builder: CompareFacesInputBuilder) -> impl Future<Output = Result<CompareFacesOutput, SdkError<CompareFacesError>>>;
    fn copy_project_version(&self, builder: CopyProjectVersionInputBuilder) -> impl Future<Output = Result<CopyProjectVersionOutput, SdkError<CopyProjectVersionError>>>;
    fn create_collection(&self, builder: CreateCollectionInputBuilder) -> impl Future<Output = Result<CreateCollectionOutput, SdkError<CreateCollectionError>>>;
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>>;
    fn create_face_liveness_session(&self, builder: CreateFaceLivenessSessionInputBuilder) -> impl Future<Output = Result<CreateFaceLivenessSessionOutput, SdkError<CreateFaceLivenessSessionError>>>;
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>>;
    fn create_project_version(&self, builder: CreateProjectVersionInputBuilder) -> impl Future<Output = Result<CreateProjectVersionOutput, SdkError<CreateProjectVersionError>>>;
    fn create_stream_processor(&self, builder: CreateStreamProcessorInputBuilder) -> impl Future<Output = Result<CreateStreamProcessorOutput, SdkError<CreateStreamProcessorError>>>;
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>>;
    fn delete_collection(&self, builder: DeleteCollectionInputBuilder) -> impl Future<Output = Result<DeleteCollectionOutput, SdkError<DeleteCollectionError>>>;
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>>;
    fn delete_faces(&self, builder: DeleteFacesInputBuilder) -> impl Future<Output = Result<DeleteFacesOutput, SdkError<DeleteFacesError>>>;
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>>;
    fn delete_project_policy(&self, builder: DeleteProjectPolicyInputBuilder) -> impl Future<Output = Result<DeleteProjectPolicyOutput, SdkError<DeleteProjectPolicyError>>>;
    fn delete_project_version(&self, builder: DeleteProjectVersionInputBuilder) -> impl Future<Output = Result<DeleteProjectVersionOutput, SdkError<DeleteProjectVersionError>>>;
    fn delete_stream_processor(&self, builder: DeleteStreamProcessorInputBuilder) -> impl Future<Output = Result<DeleteStreamProcessorOutput, SdkError<DeleteStreamProcessorError>>>;
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>>;
    fn describe_collection(&self, builder: DescribeCollectionInputBuilder) -> impl Future<Output = Result<DescribeCollectionOutput, SdkError<DescribeCollectionError>>>;
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>>;
    fn describe_project_versions(&self, builder: DescribeProjectVersionsInputBuilder) -> impl Future<Output = Result<DescribeProjectVersionsOutput, SdkError<DescribeProjectVersionsError>>>;
    fn describe_projects(&self, builder: DescribeProjectsInputBuilder) -> impl Future<Output = Result<DescribeProjectsOutput, SdkError<DescribeProjectsError>>>;
    fn describe_stream_processor(&self, builder: DescribeStreamProcessorInputBuilder) -> impl Future<Output = Result<DescribeStreamProcessorOutput, SdkError<DescribeStreamProcessorError>>>;
    fn detect_custom_labels(&self, builder: DetectCustomLabelsInputBuilder) -> impl Future<Output = Result<DetectCustomLabelsOutput, SdkError<DetectCustomLabelsError>>>;
    fn detect_faces(&self, builder: DetectFacesInputBuilder) -> impl Future<Output = Result<DetectFacesOutput, SdkError<DetectFacesError>>>;
    fn detect_labels(&self, builder: DetectLabelsInputBuilder) -> impl Future<Output = Result<DetectLabelsOutput, SdkError<DetectLabelsError>>>;
    fn detect_moderation_labels(&self, builder: DetectModerationLabelsInputBuilder) -> impl Future<Output = Result<DetectModerationLabelsOutput, SdkError<DetectModerationLabelsError>>>;
    fn detect_protective_equipment(&self, builder: DetectProtectiveEquipmentInputBuilder) -> impl Future<Output = Result<DetectProtectiveEquipmentOutput, SdkError<DetectProtectiveEquipmentError>>>;
    fn detect_text(&self, builder: DetectTextInputBuilder) -> impl Future<Output = Result<DetectTextOutput, SdkError<DetectTextError>>>;
    fn disassociate_faces(&self, builder: DisassociateFacesInputBuilder) -> impl Future<Output = Result<DisassociateFacesOutput, SdkError<DisassociateFacesError>>>;
    fn distribute_dataset_entries(&self, builder: DistributeDatasetEntriesInputBuilder) -> impl Future<Output = Result<DistributeDatasetEntriesOutput, SdkError<DistributeDatasetEntriesError>>>;
    fn get_celebrity_info(&self, builder: GetCelebrityInfoInputBuilder) -> impl Future<Output = Result<GetCelebrityInfoOutput, SdkError<GetCelebrityInfoError>>>;
    fn get_celebrity_recognition(&self, builder: GetCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<GetCelebrityRecognitionOutput, SdkError<GetCelebrityRecognitionError>>>;
    fn get_content_moderation(&self, builder: GetContentModerationInputBuilder) -> impl Future<Output = Result<GetContentModerationOutput, SdkError<GetContentModerationError>>>;
    fn get_face_detection(&self, builder: GetFaceDetectionInputBuilder) -> impl Future<Output = Result<GetFaceDetectionOutput, SdkError<GetFaceDetectionError>>>;
    fn get_face_liveness_session_results(&self, builder: GetFaceLivenessSessionResultsInputBuilder) -> impl Future<Output = Result<GetFaceLivenessSessionResultsOutput, SdkError<GetFaceLivenessSessionResultsError>>>;
    fn get_face_search(&self, builder: GetFaceSearchInputBuilder) -> impl Future<Output = Result<GetFaceSearchOutput, SdkError<GetFaceSearchError>>>;
    fn get_label_detection(&self, builder: GetLabelDetectionInputBuilder) -> impl Future<Output = Result<GetLabelDetectionOutput, SdkError<GetLabelDetectionError>>>;
    fn get_media_analysis_job(&self, builder: GetMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<GetMediaAnalysisJobOutput, SdkError<GetMediaAnalysisJobError>>>;
    fn get_person_tracking(&self, builder: GetPersonTrackingInputBuilder) -> impl Future<Output = Result<GetPersonTrackingOutput, SdkError<GetPersonTrackingError>>>;
    fn get_segment_detection(&self, builder: GetSegmentDetectionInputBuilder) -> impl Future<Output = Result<GetSegmentDetectionOutput, SdkError<GetSegmentDetectionError>>>;
    fn get_text_detection(&self, builder: GetTextDetectionInputBuilder) -> impl Future<Output = Result<GetTextDetectionOutput, SdkError<GetTextDetectionError>>>;
    fn index_faces(&self, builder: IndexFacesInputBuilder) -> impl Future<Output = Result<IndexFacesOutput, SdkError<IndexFacesError>>>;
    fn list_collections(&self, builder: ListCollectionsInputBuilder) -> impl Future<Output = Result<ListCollectionsOutput, SdkError<ListCollectionsError>>>;
    fn list_dataset_entries(&self, builder: ListDatasetEntriesInputBuilder) -> impl Future<Output = Result<ListDatasetEntriesOutput, SdkError<ListDatasetEntriesError>>>;
    fn list_dataset_labels(&self, builder: ListDatasetLabelsInputBuilder) -> impl Future<Output = Result<ListDatasetLabelsOutput, SdkError<ListDatasetLabelsError>>>;
    fn list_faces(&self, builder: ListFacesInputBuilder) -> impl Future<Output = Result<ListFacesOutput, SdkError<ListFacesError>>>;
    fn list_media_analysis_jobs(&self, builder: ListMediaAnalysisJobsInputBuilder) -> impl Future<Output = Result<ListMediaAnalysisJobsOutput, SdkError<ListMediaAnalysisJobsError>>>;
    fn list_project_policies(&self, builder: ListProjectPoliciesInputBuilder) -> impl Future<Output = Result<ListProjectPoliciesOutput, SdkError<ListProjectPoliciesError>>>;
    fn list_stream_processors(&self, builder: ListStreamProcessorsInputBuilder) -> impl Future<Output = Result<ListStreamProcessorsOutput, SdkError<ListStreamProcessorsError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>>;
    fn put_project_policy(&self, builder: PutProjectPolicyInputBuilder) -> impl Future<Output = Result<PutProjectPolicyOutput, SdkError<PutProjectPolicyError>>>;
    fn recognize_celebrities(&self, builder: RecognizeCelebritiesInputBuilder) -> impl Future<Output = Result<RecognizeCelebritiesOutput, SdkError<RecognizeCelebritiesError>>>;
    fn search_faces(&self, builder: SearchFacesInputBuilder) -> impl Future<Output = Result<SearchFacesOutput, SdkError<SearchFacesError>>>;
    fn search_faces_by_image(&self, builder: SearchFacesByImageInputBuilder) -> impl Future<Output = Result<SearchFacesByImageOutput, SdkError<SearchFacesByImageError>>>;
    fn search_users(&self, builder: SearchUsersInputBuilder) -> impl Future<Output = Result<SearchUsersOutput, SdkError<SearchUsersError>>>;
    fn search_users_by_image(&self, builder: SearchUsersByImageInputBuilder) -> impl Future<Output = Result<SearchUsersByImageOutput, SdkError<SearchUsersByImageError>>>;
    fn start_celebrity_recognition(&self, builder: StartCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<StartCelebrityRecognitionOutput, SdkError<StartCelebrityRecognitionError>>>;
    fn start_content_moderation(&self, builder: StartContentModerationInputBuilder) -> impl Future<Output = Result<StartContentModerationOutput, SdkError<StartContentModerationError>>>;
    fn start_face_detection(&self, builder: StartFaceDetectionInputBuilder) -> impl Future<Output = Result<StartFaceDetectionOutput, SdkError<StartFaceDetectionError>>>;
    fn start_face_search(&self, builder: StartFaceSearchInputBuilder) -> impl Future<Output = Result<StartFaceSearchOutput, SdkError<StartFaceSearchError>>>;
    fn start_label_detection(&self, builder: StartLabelDetectionInputBuilder) -> impl Future<Output = Result<StartLabelDetectionOutput, SdkError<StartLabelDetectionError>>>;
    fn start_media_analysis_job(&self, builder: StartMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<StartMediaAnalysisJobOutput, SdkError<StartMediaAnalysisJobError>>>;
    fn start_person_tracking(&self, builder: StartPersonTrackingInputBuilder) -> impl Future<Output = Result<StartPersonTrackingOutput, SdkError<StartPersonTrackingError>>>;
    fn start_project_version(&self, builder: StartProjectVersionInputBuilder) -> impl Future<Output = Result<StartProjectVersionOutput, SdkError<StartProjectVersionError>>>;
    fn start_segment_detection(&self, builder: StartSegmentDetectionInputBuilder) -> impl Future<Output = Result<StartSegmentDetectionOutput, SdkError<StartSegmentDetectionError>>>;
    fn start_stream_processor(&self, builder: StartStreamProcessorInputBuilder) -> impl Future<Output = Result<StartStreamProcessorOutput, SdkError<StartStreamProcessorError>>>;
    fn start_text_detection(&self, builder: StartTextDetectionInputBuilder) -> impl Future<Output = Result<StartTextDetectionOutput, SdkError<StartTextDetectionError>>>;
    fn stop_project_version(&self, builder: StopProjectVersionInputBuilder) -> impl Future<Output = Result<StopProjectVersionOutput, SdkError<StopProjectVersionError>>>;
    fn stop_stream_processor(&self, builder: StopStreamProcessorInputBuilder) -> impl Future<Output = Result<StopStreamProcessorOutput, SdkError<StopStreamProcessorError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_dataset_entries(&self, builder: UpdateDatasetEntriesInputBuilder) -> impl Future<Output = Result<UpdateDatasetEntriesOutput, SdkError<UpdateDatasetEntriesError>>>;
    fn update_stream_processor(&self, builder: UpdateStreamProcessorInputBuilder) -> impl Future<Output = Result<UpdateStreamProcessorOutput, SdkError<UpdateStreamProcessorError>>>;
}
impl RekognitionClient for RekognitionClientImpl {
    fn associate_faces(&self, builder: AssociateFacesInputBuilder) -> impl Future<Output = Result<AssociateFacesOutput, SdkError<AssociateFacesError>>> {
        builder.send_with(&self.0)
    }
    fn compare_faces(&self, builder: CompareFacesInputBuilder) -> impl Future<Output = Result<CompareFacesOutput, SdkError<CompareFacesError>>> {
        builder.send_with(&self.0)
    }
    fn copy_project_version(&self, builder: CopyProjectVersionInputBuilder) -> impl Future<Output = Result<CopyProjectVersionOutput, SdkError<CopyProjectVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_collection(&self, builder: CreateCollectionInputBuilder) -> impl Future<Output = Result<CreateCollectionOutput, SdkError<CreateCollectionError>>> {
        builder.send_with(&self.0)
    }
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn create_face_liveness_session(&self, builder: CreateFaceLivenessSessionInputBuilder) -> impl Future<Output = Result<CreateFaceLivenessSessionOutput, SdkError<CreateFaceLivenessSessionError>>> {
        builder.send_with(&self.0)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        builder.send_with(&self.0)
    }
    fn create_project_version(&self, builder: CreateProjectVersionInputBuilder) -> impl Future<Output = Result<CreateProjectVersionOutput, SdkError<CreateProjectVersionError>>> {
        builder.send_with(&self.0)
    }
    fn create_stream_processor(&self, builder: CreateStreamProcessorInputBuilder) -> impl Future<Output = Result<CreateStreamProcessorOutput, SdkError<CreateStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        builder.send_with(&self.0)
    }
    fn delete_collection(&self, builder: DeleteCollectionInputBuilder) -> impl Future<Output = Result<DeleteCollectionOutput, SdkError<DeleteCollectionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn delete_faces(&self, builder: DeleteFacesInputBuilder) -> impl Future<Output = Result<DeleteFacesOutput, SdkError<DeleteFacesError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project_policy(&self, builder: DeleteProjectPolicyInputBuilder) -> impl Future<Output = Result<DeleteProjectPolicyOutput, SdkError<DeleteProjectPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn delete_project_version(&self, builder: DeleteProjectVersionInputBuilder) -> impl Future<Output = Result<DeleteProjectVersionOutput, SdkError<DeleteProjectVersionError>>> {
        builder.send_with(&self.0)
    }
    fn delete_stream_processor(&self, builder: DeleteStreamProcessorInputBuilder) -> impl Future<Output = Result<DeleteStreamProcessorOutput, SdkError<DeleteStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        builder.send_with(&self.0)
    }
    fn describe_collection(&self, builder: DescribeCollectionInputBuilder) -> impl Future<Output = Result<DescribeCollectionOutput, SdkError<DescribeCollectionError>>> {
        builder.send_with(&self.0)
    }
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>> {
        builder.send_with(&self.0)
    }
    fn describe_project_versions(&self, builder: DescribeProjectVersionsInputBuilder) -> impl Future<Output = Result<DescribeProjectVersionsOutput, SdkError<DescribeProjectVersionsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_projects(&self, builder: DescribeProjectsInputBuilder) -> impl Future<Output = Result<DescribeProjectsOutput, SdkError<DescribeProjectsError>>> {
        builder.send_with(&self.0)
    }
    fn describe_stream_processor(&self, builder: DescribeStreamProcessorInputBuilder) -> impl Future<Output = Result<DescribeStreamProcessorOutput, SdkError<DescribeStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
    fn detect_custom_labels(&self, builder: DetectCustomLabelsInputBuilder) -> impl Future<Output = Result<DetectCustomLabelsOutput, SdkError<DetectCustomLabelsError>>> {
        builder.send_with(&self.0)
    }
    fn detect_faces(&self, builder: DetectFacesInputBuilder) -> impl Future<Output = Result<DetectFacesOutput, SdkError<DetectFacesError>>> {
        builder.send_with(&self.0)
    }
    fn detect_labels(&self, builder: DetectLabelsInputBuilder) -> impl Future<Output = Result<DetectLabelsOutput, SdkError<DetectLabelsError>>> {
        builder.send_with(&self.0)
    }
    fn detect_moderation_labels(&self, builder: DetectModerationLabelsInputBuilder) -> impl Future<Output = Result<DetectModerationLabelsOutput, SdkError<DetectModerationLabelsError>>> {
        builder.send_with(&self.0)
    }
    fn detect_protective_equipment(&self, builder: DetectProtectiveEquipmentInputBuilder) -> impl Future<Output = Result<DetectProtectiveEquipmentOutput, SdkError<DetectProtectiveEquipmentError>>> {
        builder.send_with(&self.0)
    }
    fn detect_text(&self, builder: DetectTextInputBuilder) -> impl Future<Output = Result<DetectTextOutput, SdkError<DetectTextError>>> {
        builder.send_with(&self.0)
    }
    fn disassociate_faces(&self, builder: DisassociateFacesInputBuilder) -> impl Future<Output = Result<DisassociateFacesOutput, SdkError<DisassociateFacesError>>> {
        builder.send_with(&self.0)
    }
    fn distribute_dataset_entries(&self, builder: DistributeDatasetEntriesInputBuilder) -> impl Future<Output = Result<DistributeDatasetEntriesOutput, SdkError<DistributeDatasetEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn get_celebrity_info(&self, builder: GetCelebrityInfoInputBuilder) -> impl Future<Output = Result<GetCelebrityInfoOutput, SdkError<GetCelebrityInfoError>>> {
        builder.send_with(&self.0)
    }
    fn get_celebrity_recognition(&self, builder: GetCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<GetCelebrityRecognitionOutput, SdkError<GetCelebrityRecognitionError>>> {
        builder.send_with(&self.0)
    }
    fn get_content_moderation(&self, builder: GetContentModerationInputBuilder) -> impl Future<Output = Result<GetContentModerationOutput, SdkError<GetContentModerationError>>> {
        builder.send_with(&self.0)
    }
    fn get_face_detection(&self, builder: GetFaceDetectionInputBuilder) -> impl Future<Output = Result<GetFaceDetectionOutput, SdkError<GetFaceDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_face_liveness_session_results(&self, builder: GetFaceLivenessSessionResultsInputBuilder) -> impl Future<Output = Result<GetFaceLivenessSessionResultsOutput, SdkError<GetFaceLivenessSessionResultsError>>> {
        builder.send_with(&self.0)
    }
    fn get_face_search(&self, builder: GetFaceSearchInputBuilder) -> impl Future<Output = Result<GetFaceSearchOutput, SdkError<GetFaceSearchError>>> {
        builder.send_with(&self.0)
    }
    fn get_label_detection(&self, builder: GetLabelDetectionInputBuilder) -> impl Future<Output = Result<GetLabelDetectionOutput, SdkError<GetLabelDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_media_analysis_job(&self, builder: GetMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<GetMediaAnalysisJobOutput, SdkError<GetMediaAnalysisJobError>>> {
        builder.send_with(&self.0)
    }
    fn get_person_tracking(&self, builder: GetPersonTrackingInputBuilder) -> impl Future<Output = Result<GetPersonTrackingOutput, SdkError<GetPersonTrackingError>>> {
        builder.send_with(&self.0)
    }
    fn get_segment_detection(&self, builder: GetSegmentDetectionInputBuilder) -> impl Future<Output = Result<GetSegmentDetectionOutput, SdkError<GetSegmentDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn get_text_detection(&self, builder: GetTextDetectionInputBuilder) -> impl Future<Output = Result<GetTextDetectionOutput, SdkError<GetTextDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn index_faces(&self, builder: IndexFacesInputBuilder) -> impl Future<Output = Result<IndexFacesOutput, SdkError<IndexFacesError>>> {
        builder.send_with(&self.0)
    }
    fn list_collections(&self, builder: ListCollectionsInputBuilder) -> impl Future<Output = Result<ListCollectionsOutput, SdkError<ListCollectionsError>>> {
        builder.send_with(&self.0)
    }
    fn list_dataset_entries(&self, builder: ListDatasetEntriesInputBuilder) -> impl Future<Output = Result<ListDatasetEntriesOutput, SdkError<ListDatasetEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn list_dataset_labels(&self, builder: ListDatasetLabelsInputBuilder) -> impl Future<Output = Result<ListDatasetLabelsOutput, SdkError<ListDatasetLabelsError>>> {
        builder.send_with(&self.0)
    }
    fn list_faces(&self, builder: ListFacesInputBuilder) -> impl Future<Output = Result<ListFacesOutput, SdkError<ListFacesError>>> {
        builder.send_with(&self.0)
    }
    fn list_media_analysis_jobs(&self, builder: ListMediaAnalysisJobsInputBuilder) -> impl Future<Output = Result<ListMediaAnalysisJobsOutput, SdkError<ListMediaAnalysisJobsError>>> {
        builder.send_with(&self.0)
    }
    fn list_project_policies(&self, builder: ListProjectPoliciesInputBuilder) -> impl Future<Output = Result<ListProjectPoliciesOutput, SdkError<ListProjectPoliciesError>>> {
        builder.send_with(&self.0)
    }
    fn list_stream_processors(&self, builder: ListStreamProcessorsInputBuilder) -> impl Future<Output = Result<ListStreamProcessorsOutput, SdkError<ListStreamProcessorsError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        builder.send_with(&self.0)
    }
    fn put_project_policy(&self, builder: PutProjectPolicyInputBuilder) -> impl Future<Output = Result<PutProjectPolicyOutput, SdkError<PutProjectPolicyError>>> {
        builder.send_with(&self.0)
    }
    fn recognize_celebrities(&self, builder: RecognizeCelebritiesInputBuilder) -> impl Future<Output = Result<RecognizeCelebritiesOutput, SdkError<RecognizeCelebritiesError>>> {
        builder.send_with(&self.0)
    }
    fn search_faces(&self, builder: SearchFacesInputBuilder) -> impl Future<Output = Result<SearchFacesOutput, SdkError<SearchFacesError>>> {
        builder.send_with(&self.0)
    }
    fn search_faces_by_image(&self, builder: SearchFacesByImageInputBuilder) -> impl Future<Output = Result<SearchFacesByImageOutput, SdkError<SearchFacesByImageError>>> {
        builder.send_with(&self.0)
    }
    fn search_users(&self, builder: SearchUsersInputBuilder) -> impl Future<Output = Result<SearchUsersOutput, SdkError<SearchUsersError>>> {
        builder.send_with(&self.0)
    }
    fn search_users_by_image(&self, builder: SearchUsersByImageInputBuilder) -> impl Future<Output = Result<SearchUsersByImageOutput, SdkError<SearchUsersByImageError>>> {
        builder.send_with(&self.0)
    }
    fn start_celebrity_recognition(&self, builder: StartCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<StartCelebrityRecognitionOutput, SdkError<StartCelebrityRecognitionError>>> {
        builder.send_with(&self.0)
    }
    fn start_content_moderation(&self, builder: StartContentModerationInputBuilder) -> impl Future<Output = Result<StartContentModerationOutput, SdkError<StartContentModerationError>>> {
        builder.send_with(&self.0)
    }
    fn start_face_detection(&self, builder: StartFaceDetectionInputBuilder) -> impl Future<Output = Result<StartFaceDetectionOutput, SdkError<StartFaceDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn start_face_search(&self, builder: StartFaceSearchInputBuilder) -> impl Future<Output = Result<StartFaceSearchOutput, SdkError<StartFaceSearchError>>> {
        builder.send_with(&self.0)
    }
    fn start_label_detection(&self, builder: StartLabelDetectionInputBuilder) -> impl Future<Output = Result<StartLabelDetectionOutput, SdkError<StartLabelDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn start_media_analysis_job(&self, builder: StartMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<StartMediaAnalysisJobOutput, SdkError<StartMediaAnalysisJobError>>> {
        builder.send_with(&self.0)
    }
    fn start_person_tracking(&self, builder: StartPersonTrackingInputBuilder) -> impl Future<Output = Result<StartPersonTrackingOutput, SdkError<StartPersonTrackingError>>> {
        builder.send_with(&self.0)
    }
    fn start_project_version(&self, builder: StartProjectVersionInputBuilder) -> impl Future<Output = Result<StartProjectVersionOutput, SdkError<StartProjectVersionError>>> {
        builder.send_with(&self.0)
    }
    fn start_segment_detection(&self, builder: StartSegmentDetectionInputBuilder) -> impl Future<Output = Result<StartSegmentDetectionOutput, SdkError<StartSegmentDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn start_stream_processor(&self, builder: StartStreamProcessorInputBuilder) -> impl Future<Output = Result<StartStreamProcessorOutput, SdkError<StartStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
    fn start_text_detection(&self, builder: StartTextDetectionInputBuilder) -> impl Future<Output = Result<StartTextDetectionOutput, SdkError<StartTextDetectionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_project_version(&self, builder: StopProjectVersionInputBuilder) -> impl Future<Output = Result<StopProjectVersionOutput, SdkError<StopProjectVersionError>>> {
        builder.send_with(&self.0)
    }
    fn stop_stream_processor(&self, builder: StopStreamProcessorInputBuilder) -> impl Future<Output = Result<StopStreamProcessorOutput, SdkError<StopStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_dataset_entries(&self, builder: UpdateDatasetEntriesInputBuilder) -> impl Future<Output = Result<UpdateDatasetEntriesOutput, SdkError<UpdateDatasetEntriesError>>> {
        builder.send_with(&self.0)
    }
    fn update_stream_processor(&self, builder: UpdateStreamProcessorInputBuilder) -> impl Future<Output = Result<UpdateStreamProcessorOutput, SdkError<UpdateStreamProcessorError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: RekognitionClient> RekognitionClient for &T {
    fn associate_faces(&self, builder: AssociateFacesInputBuilder) -> impl Future<Output = Result<AssociateFacesOutput, SdkError<AssociateFacesError>>> {
        (*self).associate_faces(builder)
    }
    fn compare_faces(&self, builder: CompareFacesInputBuilder) -> impl Future<Output = Result<CompareFacesOutput, SdkError<CompareFacesError>>> {
        (*self).compare_faces(builder)
    }
    fn copy_project_version(&self, builder: CopyProjectVersionInputBuilder) -> impl Future<Output = Result<CopyProjectVersionOutput, SdkError<CopyProjectVersionError>>> {
        (*self).copy_project_version(builder)
    }
    fn create_collection(&self, builder: CreateCollectionInputBuilder) -> impl Future<Output = Result<CreateCollectionOutput, SdkError<CreateCollectionError>>> {
        (*self).create_collection(builder)
    }
    fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> impl Future<Output = Result<CreateDatasetOutput, SdkError<CreateDatasetError>>> {
        (*self).create_dataset(builder)
    }
    fn create_face_liveness_session(&self, builder: CreateFaceLivenessSessionInputBuilder) -> impl Future<Output = Result<CreateFaceLivenessSessionOutput, SdkError<CreateFaceLivenessSessionError>>> {
        (*self).create_face_liveness_session(builder)
    }
    fn create_project(&self, builder: CreateProjectInputBuilder) -> impl Future<Output = Result<CreateProjectOutput, SdkError<CreateProjectError>>> {
        (*self).create_project(builder)
    }
    fn create_project_version(&self, builder: CreateProjectVersionInputBuilder) -> impl Future<Output = Result<CreateProjectVersionOutput, SdkError<CreateProjectVersionError>>> {
        (*self).create_project_version(builder)
    }
    fn create_stream_processor(&self, builder: CreateStreamProcessorInputBuilder) -> impl Future<Output = Result<CreateStreamProcessorOutput, SdkError<CreateStreamProcessorError>>> {
        (*self).create_stream_processor(builder)
    }
    fn create_user(&self, builder: CreateUserInputBuilder) -> impl Future<Output = Result<CreateUserOutput, SdkError<CreateUserError>>> {
        (*self).create_user(builder)
    }
    fn delete_collection(&self, builder: DeleteCollectionInputBuilder) -> impl Future<Output = Result<DeleteCollectionOutput, SdkError<DeleteCollectionError>>> {
        (*self).delete_collection(builder)
    }
    fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> impl Future<Output = Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>> {
        (*self).delete_dataset(builder)
    }
    fn delete_faces(&self, builder: DeleteFacesInputBuilder) -> impl Future<Output = Result<DeleteFacesOutput, SdkError<DeleteFacesError>>> {
        (*self).delete_faces(builder)
    }
    fn delete_project(&self, builder: DeleteProjectInputBuilder) -> impl Future<Output = Result<DeleteProjectOutput, SdkError<DeleteProjectError>>> {
        (*self).delete_project(builder)
    }
    fn delete_project_policy(&self, builder: DeleteProjectPolicyInputBuilder) -> impl Future<Output = Result<DeleteProjectPolicyOutput, SdkError<DeleteProjectPolicyError>>> {
        (*self).delete_project_policy(builder)
    }
    fn delete_project_version(&self, builder: DeleteProjectVersionInputBuilder) -> impl Future<Output = Result<DeleteProjectVersionOutput, SdkError<DeleteProjectVersionError>>> {
        (*self).delete_project_version(builder)
    }
    fn delete_stream_processor(&self, builder: DeleteStreamProcessorInputBuilder) -> impl Future<Output = Result<DeleteStreamProcessorOutput, SdkError<DeleteStreamProcessorError>>> {
        (*self).delete_stream_processor(builder)
    }
    fn delete_user(&self, builder: DeleteUserInputBuilder) -> impl Future<Output = Result<DeleteUserOutput, SdkError<DeleteUserError>>> {
        (*self).delete_user(builder)
    }
    fn describe_collection(&self, builder: DescribeCollectionInputBuilder) -> impl Future<Output = Result<DescribeCollectionOutput, SdkError<DescribeCollectionError>>> {
        (*self).describe_collection(builder)
    }
    fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> impl Future<Output = Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>> {
        (*self).describe_dataset(builder)
    }
    fn describe_project_versions(&self, builder: DescribeProjectVersionsInputBuilder) -> impl Future<Output = Result<DescribeProjectVersionsOutput, SdkError<DescribeProjectVersionsError>>> {
        (*self).describe_project_versions(builder)
    }
    fn describe_projects(&self, builder: DescribeProjectsInputBuilder) -> impl Future<Output = Result<DescribeProjectsOutput, SdkError<DescribeProjectsError>>> {
        (*self).describe_projects(builder)
    }
    fn describe_stream_processor(&self, builder: DescribeStreamProcessorInputBuilder) -> impl Future<Output = Result<DescribeStreamProcessorOutput, SdkError<DescribeStreamProcessorError>>> {
        (*self).describe_stream_processor(builder)
    }
    fn detect_custom_labels(&self, builder: DetectCustomLabelsInputBuilder) -> impl Future<Output = Result<DetectCustomLabelsOutput, SdkError<DetectCustomLabelsError>>> {
        (*self).detect_custom_labels(builder)
    }
    fn detect_faces(&self, builder: DetectFacesInputBuilder) -> impl Future<Output = Result<DetectFacesOutput, SdkError<DetectFacesError>>> {
        (*self).detect_faces(builder)
    }
    fn detect_labels(&self, builder: DetectLabelsInputBuilder) -> impl Future<Output = Result<DetectLabelsOutput, SdkError<DetectLabelsError>>> {
        (*self).detect_labels(builder)
    }
    fn detect_moderation_labels(&self, builder: DetectModerationLabelsInputBuilder) -> impl Future<Output = Result<DetectModerationLabelsOutput, SdkError<DetectModerationLabelsError>>> {
        (*self).detect_moderation_labels(builder)
    }
    fn detect_protective_equipment(&self, builder: DetectProtectiveEquipmentInputBuilder) -> impl Future<Output = Result<DetectProtectiveEquipmentOutput, SdkError<DetectProtectiveEquipmentError>>> {
        (*self).detect_protective_equipment(builder)
    }
    fn detect_text(&self, builder: DetectTextInputBuilder) -> impl Future<Output = Result<DetectTextOutput, SdkError<DetectTextError>>> {
        (*self).detect_text(builder)
    }
    fn disassociate_faces(&self, builder: DisassociateFacesInputBuilder) -> impl Future<Output = Result<DisassociateFacesOutput, SdkError<DisassociateFacesError>>> {
        (*self).disassociate_faces(builder)
    }
    fn distribute_dataset_entries(&self, builder: DistributeDatasetEntriesInputBuilder) -> impl Future<Output = Result<DistributeDatasetEntriesOutput, SdkError<DistributeDatasetEntriesError>>> {
        (*self).distribute_dataset_entries(builder)
    }
    fn get_celebrity_info(&self, builder: GetCelebrityInfoInputBuilder) -> impl Future<Output = Result<GetCelebrityInfoOutput, SdkError<GetCelebrityInfoError>>> {
        (*self).get_celebrity_info(builder)
    }
    fn get_celebrity_recognition(&self, builder: GetCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<GetCelebrityRecognitionOutput, SdkError<GetCelebrityRecognitionError>>> {
        (*self).get_celebrity_recognition(builder)
    }
    fn get_content_moderation(&self, builder: GetContentModerationInputBuilder) -> impl Future<Output = Result<GetContentModerationOutput, SdkError<GetContentModerationError>>> {
        (*self).get_content_moderation(builder)
    }
    fn get_face_detection(&self, builder: GetFaceDetectionInputBuilder) -> impl Future<Output = Result<GetFaceDetectionOutput, SdkError<GetFaceDetectionError>>> {
        (*self).get_face_detection(builder)
    }
    fn get_face_liveness_session_results(&self, builder: GetFaceLivenessSessionResultsInputBuilder) -> impl Future<Output = Result<GetFaceLivenessSessionResultsOutput, SdkError<GetFaceLivenessSessionResultsError>>> {
        (*self).get_face_liveness_session_results(builder)
    }
    fn get_face_search(&self, builder: GetFaceSearchInputBuilder) -> impl Future<Output = Result<GetFaceSearchOutput, SdkError<GetFaceSearchError>>> {
        (*self).get_face_search(builder)
    }
    fn get_label_detection(&self, builder: GetLabelDetectionInputBuilder) -> impl Future<Output = Result<GetLabelDetectionOutput, SdkError<GetLabelDetectionError>>> {
        (*self).get_label_detection(builder)
    }
    fn get_media_analysis_job(&self, builder: GetMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<GetMediaAnalysisJobOutput, SdkError<GetMediaAnalysisJobError>>> {
        (*self).get_media_analysis_job(builder)
    }
    fn get_person_tracking(&self, builder: GetPersonTrackingInputBuilder) -> impl Future<Output = Result<GetPersonTrackingOutput, SdkError<GetPersonTrackingError>>> {
        (*self).get_person_tracking(builder)
    }
    fn get_segment_detection(&self, builder: GetSegmentDetectionInputBuilder) -> impl Future<Output = Result<GetSegmentDetectionOutput, SdkError<GetSegmentDetectionError>>> {
        (*self).get_segment_detection(builder)
    }
    fn get_text_detection(&self, builder: GetTextDetectionInputBuilder) -> impl Future<Output = Result<GetTextDetectionOutput, SdkError<GetTextDetectionError>>> {
        (*self).get_text_detection(builder)
    }
    fn index_faces(&self, builder: IndexFacesInputBuilder) -> impl Future<Output = Result<IndexFacesOutput, SdkError<IndexFacesError>>> {
        (*self).index_faces(builder)
    }
    fn list_collections(&self, builder: ListCollectionsInputBuilder) -> impl Future<Output = Result<ListCollectionsOutput, SdkError<ListCollectionsError>>> {
        (*self).list_collections(builder)
    }
    fn list_dataset_entries(&self, builder: ListDatasetEntriesInputBuilder) -> impl Future<Output = Result<ListDatasetEntriesOutput, SdkError<ListDatasetEntriesError>>> {
        (*self).list_dataset_entries(builder)
    }
    fn list_dataset_labels(&self, builder: ListDatasetLabelsInputBuilder) -> impl Future<Output = Result<ListDatasetLabelsOutput, SdkError<ListDatasetLabelsError>>> {
        (*self).list_dataset_labels(builder)
    }
    fn list_faces(&self, builder: ListFacesInputBuilder) -> impl Future<Output = Result<ListFacesOutput, SdkError<ListFacesError>>> {
        (*self).list_faces(builder)
    }
    fn list_media_analysis_jobs(&self, builder: ListMediaAnalysisJobsInputBuilder) -> impl Future<Output = Result<ListMediaAnalysisJobsOutput, SdkError<ListMediaAnalysisJobsError>>> {
        (*self).list_media_analysis_jobs(builder)
    }
    fn list_project_policies(&self, builder: ListProjectPoliciesInputBuilder) -> impl Future<Output = Result<ListProjectPoliciesOutput, SdkError<ListProjectPoliciesError>>> {
        (*self).list_project_policies(builder)
    }
    fn list_stream_processors(&self, builder: ListStreamProcessorsInputBuilder) -> impl Future<Output = Result<ListStreamProcessorsOutput, SdkError<ListStreamProcessorsError>>> {
        (*self).list_stream_processors(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        (*self).list_tags_for_resource(builder)
    }
    fn list_users(&self, builder: ListUsersInputBuilder) -> impl Future<Output = Result<ListUsersOutput, SdkError<ListUsersError>>> {
        (*self).list_users(builder)
    }
    fn put_project_policy(&self, builder: PutProjectPolicyInputBuilder) -> impl Future<Output = Result<PutProjectPolicyOutput, SdkError<PutProjectPolicyError>>> {
        (*self).put_project_policy(builder)
    }
    fn recognize_celebrities(&self, builder: RecognizeCelebritiesInputBuilder) -> impl Future<Output = Result<RecognizeCelebritiesOutput, SdkError<RecognizeCelebritiesError>>> {
        (*self).recognize_celebrities(builder)
    }
    fn search_faces(&self, builder: SearchFacesInputBuilder) -> impl Future<Output = Result<SearchFacesOutput, SdkError<SearchFacesError>>> {
        (*self).search_faces(builder)
    }
    fn search_faces_by_image(&self, builder: SearchFacesByImageInputBuilder) -> impl Future<Output = Result<SearchFacesByImageOutput, SdkError<SearchFacesByImageError>>> {
        (*self).search_faces_by_image(builder)
    }
    fn search_users(&self, builder: SearchUsersInputBuilder) -> impl Future<Output = Result<SearchUsersOutput, SdkError<SearchUsersError>>> {
        (*self).search_users(builder)
    }
    fn search_users_by_image(&self, builder: SearchUsersByImageInputBuilder) -> impl Future<Output = Result<SearchUsersByImageOutput, SdkError<SearchUsersByImageError>>> {
        (*self).search_users_by_image(builder)
    }
    fn start_celebrity_recognition(&self, builder: StartCelebrityRecognitionInputBuilder) -> impl Future<Output = Result<StartCelebrityRecognitionOutput, SdkError<StartCelebrityRecognitionError>>> {
        (*self).start_celebrity_recognition(builder)
    }
    fn start_content_moderation(&self, builder: StartContentModerationInputBuilder) -> impl Future<Output = Result<StartContentModerationOutput, SdkError<StartContentModerationError>>> {
        (*self).start_content_moderation(builder)
    }
    fn start_face_detection(&self, builder: StartFaceDetectionInputBuilder) -> impl Future<Output = Result<StartFaceDetectionOutput, SdkError<StartFaceDetectionError>>> {
        (*self).start_face_detection(builder)
    }
    fn start_face_search(&self, builder: StartFaceSearchInputBuilder) -> impl Future<Output = Result<StartFaceSearchOutput, SdkError<StartFaceSearchError>>> {
        (*self).start_face_search(builder)
    }
    fn start_label_detection(&self, builder: StartLabelDetectionInputBuilder) -> impl Future<Output = Result<StartLabelDetectionOutput, SdkError<StartLabelDetectionError>>> {
        (*self).start_label_detection(builder)
    }
    fn start_media_analysis_job(&self, builder: StartMediaAnalysisJobInputBuilder) -> impl Future<Output = Result<StartMediaAnalysisJobOutput, SdkError<StartMediaAnalysisJobError>>> {
        (*self).start_media_analysis_job(builder)
    }
    fn start_person_tracking(&self, builder: StartPersonTrackingInputBuilder) -> impl Future<Output = Result<StartPersonTrackingOutput, SdkError<StartPersonTrackingError>>> {
        (*self).start_person_tracking(builder)
    }
    fn start_project_version(&self, builder: StartProjectVersionInputBuilder) -> impl Future<Output = Result<StartProjectVersionOutput, SdkError<StartProjectVersionError>>> {
        (*self).start_project_version(builder)
    }
    fn start_segment_detection(&self, builder: StartSegmentDetectionInputBuilder) -> impl Future<Output = Result<StartSegmentDetectionOutput, SdkError<StartSegmentDetectionError>>> {
        (*self).start_segment_detection(builder)
    }
    fn start_stream_processor(&self, builder: StartStreamProcessorInputBuilder) -> impl Future<Output = Result<StartStreamProcessorOutput, SdkError<StartStreamProcessorError>>> {
        (*self).start_stream_processor(builder)
    }
    fn start_text_detection(&self, builder: StartTextDetectionInputBuilder) -> impl Future<Output = Result<StartTextDetectionOutput, SdkError<StartTextDetectionError>>> {
        (*self).start_text_detection(builder)
    }
    fn stop_project_version(&self, builder: StopProjectVersionInputBuilder) -> impl Future<Output = Result<StopProjectVersionOutput, SdkError<StopProjectVersionError>>> {
        (*self).stop_project_version(builder)
    }
    fn stop_stream_processor(&self, builder: StopStreamProcessorInputBuilder) -> impl Future<Output = Result<StopStreamProcessorOutput, SdkError<StopStreamProcessorError>>> {
        (*self).stop_stream_processor(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        (*self).tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        (*self).untag_resource(builder)
    }
    fn update_dataset_entries(&self, builder: UpdateDatasetEntriesInputBuilder) -> impl Future<Output = Result<UpdateDatasetEntriesOutput, SdkError<UpdateDatasetEntriesError>>> {
        (*self).update_dataset_entries(builder)
    }
    fn update_stream_processor(&self, builder: UpdateStreamProcessorInputBuilder) -> impl Future<Output = Result<UpdateStreamProcessorOutput, SdkError<UpdateStreamProcessorError>>> {
        (*self).update_stream_processor(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edRekognitionClient {}
    impl RekognitionClient for edRekognitionClient {
        async fn associate_faces(&self, builder: AssociateFacesInputBuilder) -> Result<AssociateFacesOutput, SdkError<AssociateFacesError>>;
        async fn compare_faces(&self, builder: CompareFacesInputBuilder) -> Result<CompareFacesOutput, SdkError<CompareFacesError>>;
        async fn copy_project_version(&self, builder: CopyProjectVersionInputBuilder) -> Result<CopyProjectVersionOutput, SdkError<CopyProjectVersionError>>;
        async fn create_collection(&self, builder: CreateCollectionInputBuilder) -> Result<CreateCollectionOutput, SdkError<CreateCollectionError>>;
        async fn create_dataset(&self, builder: CreateDatasetInputBuilder) -> Result<CreateDatasetOutput, SdkError<CreateDatasetError>>;
        async fn create_face_liveness_session(&self, builder: CreateFaceLivenessSessionInputBuilder) -> Result<CreateFaceLivenessSessionOutput, SdkError<CreateFaceLivenessSessionError>>;
        async fn create_project(&self, builder: CreateProjectInputBuilder) -> Result<CreateProjectOutput, SdkError<CreateProjectError>>;
        async fn create_project_version(&self, builder: CreateProjectVersionInputBuilder) -> Result<CreateProjectVersionOutput, SdkError<CreateProjectVersionError>>;
        async fn create_stream_processor(&self, builder: CreateStreamProcessorInputBuilder) -> Result<CreateStreamProcessorOutput, SdkError<CreateStreamProcessorError>>;
        async fn create_user(&self, builder: CreateUserInputBuilder) -> Result<CreateUserOutput, SdkError<CreateUserError>>;
        async fn delete_collection(&self, builder: DeleteCollectionInputBuilder) -> Result<DeleteCollectionOutput, SdkError<DeleteCollectionError>>;
        async fn delete_dataset(&self, builder: DeleteDatasetInputBuilder) -> Result<DeleteDatasetOutput, SdkError<DeleteDatasetError>>;
        async fn delete_faces(&self, builder: DeleteFacesInputBuilder) -> Result<DeleteFacesOutput, SdkError<DeleteFacesError>>;
        async fn delete_project(&self, builder: DeleteProjectInputBuilder) -> Result<DeleteProjectOutput, SdkError<DeleteProjectError>>;
        async fn delete_project_policy(&self, builder: DeleteProjectPolicyInputBuilder) -> Result<DeleteProjectPolicyOutput, SdkError<DeleteProjectPolicyError>>;
        async fn delete_project_version(&self, builder: DeleteProjectVersionInputBuilder) -> Result<DeleteProjectVersionOutput, SdkError<DeleteProjectVersionError>>;
        async fn delete_stream_processor(&self, builder: DeleteStreamProcessorInputBuilder) -> Result<DeleteStreamProcessorOutput, SdkError<DeleteStreamProcessorError>>;
        async fn delete_user(&self, builder: DeleteUserInputBuilder) -> Result<DeleteUserOutput, SdkError<DeleteUserError>>;
        async fn describe_collection(&self, builder: DescribeCollectionInputBuilder) -> Result<DescribeCollectionOutput, SdkError<DescribeCollectionError>>;
        async fn describe_dataset(&self, builder: DescribeDatasetInputBuilder) -> Result<DescribeDatasetOutput, SdkError<DescribeDatasetError>>;
        async fn describe_project_versions(&self, builder: DescribeProjectVersionsInputBuilder) -> Result<DescribeProjectVersionsOutput, SdkError<DescribeProjectVersionsError>>;
        async fn describe_projects(&self, builder: DescribeProjectsInputBuilder) -> Result<DescribeProjectsOutput, SdkError<DescribeProjectsError>>;
        async fn describe_stream_processor(&self, builder: DescribeStreamProcessorInputBuilder) -> Result<DescribeStreamProcessorOutput, SdkError<DescribeStreamProcessorError>>;
        async fn detect_custom_labels(&self, builder: DetectCustomLabelsInputBuilder) -> Result<DetectCustomLabelsOutput, SdkError<DetectCustomLabelsError>>;
        async fn detect_faces(&self, builder: DetectFacesInputBuilder) -> Result<DetectFacesOutput, SdkError<DetectFacesError>>;
        async fn detect_labels(&self, builder: DetectLabelsInputBuilder) -> Result<DetectLabelsOutput, SdkError<DetectLabelsError>>;
        async fn detect_moderation_labels(&self, builder: DetectModerationLabelsInputBuilder) -> Result<DetectModerationLabelsOutput, SdkError<DetectModerationLabelsError>>;
        async fn detect_protective_equipment(&self, builder: DetectProtectiveEquipmentInputBuilder) -> Result<DetectProtectiveEquipmentOutput, SdkError<DetectProtectiveEquipmentError>>;
        async fn detect_text(&self, builder: DetectTextInputBuilder) -> Result<DetectTextOutput, SdkError<DetectTextError>>;
        async fn disassociate_faces(&self, builder: DisassociateFacesInputBuilder) -> Result<DisassociateFacesOutput, SdkError<DisassociateFacesError>>;
        async fn distribute_dataset_entries(&self, builder: DistributeDatasetEntriesInputBuilder) -> Result<DistributeDatasetEntriesOutput, SdkError<DistributeDatasetEntriesError>>;
        async fn get_celebrity_info(&self, builder: GetCelebrityInfoInputBuilder) -> Result<GetCelebrityInfoOutput, SdkError<GetCelebrityInfoError>>;
        async fn get_celebrity_recognition(&self, builder: GetCelebrityRecognitionInputBuilder) -> Result<GetCelebrityRecognitionOutput, SdkError<GetCelebrityRecognitionError>>;
        async fn get_content_moderation(&self, builder: GetContentModerationInputBuilder) -> Result<GetContentModerationOutput, SdkError<GetContentModerationError>>;
        async fn get_face_detection(&self, builder: GetFaceDetectionInputBuilder) -> Result<GetFaceDetectionOutput, SdkError<GetFaceDetectionError>>;
        async fn get_face_liveness_session_results(&self, builder: GetFaceLivenessSessionResultsInputBuilder) -> Result<GetFaceLivenessSessionResultsOutput, SdkError<GetFaceLivenessSessionResultsError>>;
        async fn get_face_search(&self, builder: GetFaceSearchInputBuilder) -> Result<GetFaceSearchOutput, SdkError<GetFaceSearchError>>;
        async fn get_label_detection(&self, builder: GetLabelDetectionInputBuilder) -> Result<GetLabelDetectionOutput, SdkError<GetLabelDetectionError>>;
        async fn get_media_analysis_job(&self, builder: GetMediaAnalysisJobInputBuilder) -> Result<GetMediaAnalysisJobOutput, SdkError<GetMediaAnalysisJobError>>;
        async fn get_person_tracking(&self, builder: GetPersonTrackingInputBuilder) -> Result<GetPersonTrackingOutput, SdkError<GetPersonTrackingError>>;
        async fn get_segment_detection(&self, builder: GetSegmentDetectionInputBuilder) -> Result<GetSegmentDetectionOutput, SdkError<GetSegmentDetectionError>>;
        async fn get_text_detection(&self, builder: GetTextDetectionInputBuilder) -> Result<GetTextDetectionOutput, SdkError<GetTextDetectionError>>;
        async fn index_faces(&self, builder: IndexFacesInputBuilder) -> Result<IndexFacesOutput, SdkError<IndexFacesError>>;
        async fn list_collections(&self, builder: ListCollectionsInputBuilder) -> Result<ListCollectionsOutput, SdkError<ListCollectionsError>>;
        async fn list_dataset_entries(&self, builder: ListDatasetEntriesInputBuilder) -> Result<ListDatasetEntriesOutput, SdkError<ListDatasetEntriesError>>;
        async fn list_dataset_labels(&self, builder: ListDatasetLabelsInputBuilder) -> Result<ListDatasetLabelsOutput, SdkError<ListDatasetLabelsError>>;
        async fn list_faces(&self, builder: ListFacesInputBuilder) -> Result<ListFacesOutput, SdkError<ListFacesError>>;
        async fn list_media_analysis_jobs(&self, builder: ListMediaAnalysisJobsInputBuilder) -> Result<ListMediaAnalysisJobsOutput, SdkError<ListMediaAnalysisJobsError>>;
        async fn list_project_policies(&self, builder: ListProjectPoliciesInputBuilder) -> Result<ListProjectPoliciesOutput, SdkError<ListProjectPoliciesError>>;
        async fn list_stream_processors(&self, builder: ListStreamProcessorsInputBuilder) -> Result<ListStreamProcessorsOutput, SdkError<ListStreamProcessorsError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_users(&self, builder: ListUsersInputBuilder) -> Result<ListUsersOutput, SdkError<ListUsersError>>;
        async fn put_project_policy(&self, builder: PutProjectPolicyInputBuilder) -> Result<PutProjectPolicyOutput, SdkError<PutProjectPolicyError>>;
        async fn recognize_celebrities(&self, builder: RecognizeCelebritiesInputBuilder) -> Result<RecognizeCelebritiesOutput, SdkError<RecognizeCelebritiesError>>;
        async fn search_faces(&self, builder: SearchFacesInputBuilder) -> Result<SearchFacesOutput, SdkError<SearchFacesError>>;
        async fn search_faces_by_image(&self, builder: SearchFacesByImageInputBuilder) -> Result<SearchFacesByImageOutput, SdkError<SearchFacesByImageError>>;
        async fn search_users(&self, builder: SearchUsersInputBuilder) -> Result<SearchUsersOutput, SdkError<SearchUsersError>>;
        async fn search_users_by_image(&self, builder: SearchUsersByImageInputBuilder) -> Result<SearchUsersByImageOutput, SdkError<SearchUsersByImageError>>;
        async fn start_celebrity_recognition(&self, builder: StartCelebrityRecognitionInputBuilder) -> Result<StartCelebrityRecognitionOutput, SdkError<StartCelebrityRecognitionError>>;
        async fn start_content_moderation(&self, builder: StartContentModerationInputBuilder) -> Result<StartContentModerationOutput, SdkError<StartContentModerationError>>;
        async fn start_face_detection(&self, builder: StartFaceDetectionInputBuilder) -> Result<StartFaceDetectionOutput, SdkError<StartFaceDetectionError>>;
        async fn start_face_search(&self, builder: StartFaceSearchInputBuilder) -> Result<StartFaceSearchOutput, SdkError<StartFaceSearchError>>;
        async fn start_label_detection(&self, builder: StartLabelDetectionInputBuilder) -> Result<StartLabelDetectionOutput, SdkError<StartLabelDetectionError>>;
        async fn start_media_analysis_job(&self, builder: StartMediaAnalysisJobInputBuilder) -> Result<StartMediaAnalysisJobOutput, SdkError<StartMediaAnalysisJobError>>;
        async fn start_person_tracking(&self, builder: StartPersonTrackingInputBuilder) -> Result<StartPersonTrackingOutput, SdkError<StartPersonTrackingError>>;
        async fn start_project_version(&self, builder: StartProjectVersionInputBuilder) -> Result<StartProjectVersionOutput, SdkError<StartProjectVersionError>>;
        async fn start_segment_detection(&self, builder: StartSegmentDetectionInputBuilder) -> Result<StartSegmentDetectionOutput, SdkError<StartSegmentDetectionError>>;
        async fn start_stream_processor(&self, builder: StartStreamProcessorInputBuilder) -> Result<StartStreamProcessorOutput, SdkError<StartStreamProcessorError>>;
        async fn start_text_detection(&self, builder: StartTextDetectionInputBuilder) -> Result<StartTextDetectionOutput, SdkError<StartTextDetectionError>>;
        async fn stop_project_version(&self, builder: StopProjectVersionInputBuilder) -> Result<StopProjectVersionOutput, SdkError<StopProjectVersionError>>;
        async fn stop_stream_processor(&self, builder: StopStreamProcessorInputBuilder) -> Result<StopStreamProcessorOutput, SdkError<StopStreamProcessorError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_dataset_entries(&self, builder: UpdateDatasetEntriesInputBuilder) -> Result<UpdateDatasetEntriesOutput, SdkError<UpdateDatasetEntriesError>>;
        async fn update_stream_processor(&self, builder: UpdateStreamProcessorInputBuilder) -> Result<UpdateStreamProcessorOutput, SdkError<UpdateStreamProcessorError>>;
    }
}
