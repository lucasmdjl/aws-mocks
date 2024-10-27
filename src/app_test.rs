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
use aws_sdk_apptest::operation::create_test_case::{builders::*, *};
use aws_sdk_apptest::operation::create_test_configuration::{builders::*, *};
use aws_sdk_apptest::operation::create_test_suite::{builders::*, *};
use aws_sdk_apptest::operation::delete_test_case::{builders::*, *};
use aws_sdk_apptest::operation::delete_test_configuration::{builders::*, *};
use aws_sdk_apptest::operation::delete_test_run::{builders::*, *};
use aws_sdk_apptest::operation::delete_test_suite::{builders::*, *};
use aws_sdk_apptest::operation::get_test_case::{builders::*, *};
use aws_sdk_apptest::operation::get_test_configuration::{builders::*, *};
use aws_sdk_apptest::operation::get_test_run_step::{builders::*, *};
use aws_sdk_apptest::operation::get_test_suite::{builders::*, *};
use aws_sdk_apptest::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_apptest::operation::list_test_cases::{builders::*, *};
use aws_sdk_apptest::operation::list_test_configurations::{builders::*, *};
use aws_sdk_apptest::operation::list_test_run_steps::{builders::*, *};
use aws_sdk_apptest::operation::list_test_run_test_cases::{builders::*, *};
use aws_sdk_apptest::operation::list_test_runs::{builders::*, *};
use aws_sdk_apptest::operation::list_test_suites::{builders::*, *};
use aws_sdk_apptest::operation::start_test_run::{builders::*, *};
use aws_sdk_apptest::operation::tag_resource::{builders::*, *};
use aws_sdk_apptest::operation::untag_resource::{builders::*, *};
use aws_sdk_apptest::operation::update_test_case::{builders::*, *};
use aws_sdk_apptest::operation::update_test_configuration::{builders::*, *};
use aws_sdk_apptest::operation::update_test_suite::{builders::*, *};
use aws_sdk_apptest::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_apptest::Client;
use std::ops::Deref;

pub use aws_sdk_apptest::*;

pub struct AppTestClientImpl(Client);
impl AppTestClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppTestClient {
    fn create_test_case(&self, builder: CreateTestCaseInputBuilder) -> impl Future<Output = Result<CreateTestCaseOutput, SdkError<CreateTestCaseError>>> + Send;
    fn create_test_configuration(&self, builder: CreateTestConfigurationInputBuilder) -> impl Future<Output = Result<CreateTestConfigurationOutput, SdkError<CreateTestConfigurationError>>> + Send;
    fn create_test_suite(&self, builder: CreateTestSuiteInputBuilder) -> impl Future<Output = Result<CreateTestSuiteOutput, SdkError<CreateTestSuiteError>>> + Send;
    fn delete_test_case(&self, builder: DeleteTestCaseInputBuilder) -> impl Future<Output = Result<DeleteTestCaseOutput, SdkError<DeleteTestCaseError>>> + Send;
    fn delete_test_configuration(&self, builder: DeleteTestConfigurationInputBuilder) -> impl Future<Output = Result<DeleteTestConfigurationOutput, SdkError<DeleteTestConfigurationError>>> + Send;
    fn delete_test_run(&self, builder: DeleteTestRunInputBuilder) -> impl Future<Output = Result<DeleteTestRunOutput, SdkError<DeleteTestRunError>>> + Send;
    fn delete_test_suite(&self, builder: DeleteTestSuiteInputBuilder) -> impl Future<Output = Result<DeleteTestSuiteOutput, SdkError<DeleteTestSuiteError>>> + Send;
    fn get_test_case(&self, builder: GetTestCaseInputBuilder) -> impl Future<Output = Result<GetTestCaseOutput, SdkError<GetTestCaseError>>> + Send;
    fn get_test_configuration(&self, builder: GetTestConfigurationInputBuilder) -> impl Future<Output = Result<GetTestConfigurationOutput, SdkError<GetTestConfigurationError>>> + Send;
    fn get_test_run_step(&self, builder: GetTestRunStepInputBuilder) -> impl Future<Output = Result<GetTestRunStepOutput, SdkError<GetTestRunStepError>>> + Send;
    fn get_test_suite(&self, builder: GetTestSuiteInputBuilder) -> impl Future<Output = Result<GetTestSuiteOutput, SdkError<GetTestSuiteError>>> + Send;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> + Send;
    fn list_test_cases(&self, builder: ListTestCasesInputBuilder) -> impl Future<Output = Result<ListTestCasesOutput, SdkError<ListTestCasesError>>> + Send;
    fn list_test_configurations(&self, builder: ListTestConfigurationsInputBuilder) -> impl Future<Output = Result<ListTestConfigurationsOutput, SdkError<ListTestConfigurationsError>>> + Send;
    fn list_test_run_steps(&self, builder: ListTestRunStepsInputBuilder) -> impl Future<Output = Result<ListTestRunStepsOutput, SdkError<ListTestRunStepsError>>> + Send;
    fn list_test_run_test_cases(&self, builder: ListTestRunTestCasesInputBuilder) -> impl Future<Output = Result<ListTestRunTestCasesOutput, SdkError<ListTestRunTestCasesError>>> + Send;
    fn list_test_runs(&self, builder: ListTestRunsInputBuilder) -> impl Future<Output = Result<ListTestRunsOutput, SdkError<ListTestRunsError>>> + Send;
    fn list_test_suites(&self, builder: ListTestSuitesInputBuilder) -> impl Future<Output = Result<ListTestSuitesOutput, SdkError<ListTestSuitesError>>> + Send;
    fn start_test_run(&self, builder: StartTestRunInputBuilder) -> impl Future<Output = Result<StartTestRunOutput, SdkError<StartTestRunError>>> + Send;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> + Send;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> + Send;
    fn update_test_case(&self, builder: UpdateTestCaseInputBuilder) -> impl Future<Output = Result<UpdateTestCaseOutput, SdkError<UpdateTestCaseError>>> + Send;
    fn update_test_configuration(&self, builder: UpdateTestConfigurationInputBuilder) -> impl Future<Output = Result<UpdateTestConfigurationOutput, SdkError<UpdateTestConfigurationError>>> + Send;
    fn update_test_suite(&self, builder: UpdateTestSuiteInputBuilder) -> impl Future<Output = Result<UpdateTestSuiteOutput, SdkError<UpdateTestSuiteError>>> + Send;
}
impl AppTestClient for AppTestClientImpl {
    fn create_test_case(&self, builder: CreateTestCaseInputBuilder) -> impl Future<Output = Result<CreateTestCaseOutput, SdkError<CreateTestCaseError>>> {
        builder.send_with(&self.0)
    }
    fn create_test_configuration(&self, builder: CreateTestConfigurationInputBuilder) -> impl Future<Output = Result<CreateTestConfigurationOutput, SdkError<CreateTestConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn create_test_suite(&self, builder: CreateTestSuiteInputBuilder) -> impl Future<Output = Result<CreateTestSuiteOutput, SdkError<CreateTestSuiteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_test_case(&self, builder: DeleteTestCaseInputBuilder) -> impl Future<Output = Result<DeleteTestCaseOutput, SdkError<DeleteTestCaseError>>> {
        builder.send_with(&self.0)
    }
    fn delete_test_configuration(&self, builder: DeleteTestConfigurationInputBuilder) -> impl Future<Output = Result<DeleteTestConfigurationOutput, SdkError<DeleteTestConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn delete_test_run(&self, builder: DeleteTestRunInputBuilder) -> impl Future<Output = Result<DeleteTestRunOutput, SdkError<DeleteTestRunError>>> {
        builder.send_with(&self.0)
    }
    fn delete_test_suite(&self, builder: DeleteTestSuiteInputBuilder) -> impl Future<Output = Result<DeleteTestSuiteOutput, SdkError<DeleteTestSuiteError>>> {
        builder.send_with(&self.0)
    }
    fn get_test_case(&self, builder: GetTestCaseInputBuilder) -> impl Future<Output = Result<GetTestCaseOutput, SdkError<GetTestCaseError>>> {
        builder.send_with(&self.0)
    }
    fn get_test_configuration(&self, builder: GetTestConfigurationInputBuilder) -> impl Future<Output = Result<GetTestConfigurationOutput, SdkError<GetTestConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn get_test_run_step(&self, builder: GetTestRunStepInputBuilder) -> impl Future<Output = Result<GetTestRunStepOutput, SdkError<GetTestRunStepError>>> {
        builder.send_with(&self.0)
    }
    fn get_test_suite(&self, builder: GetTestSuiteInputBuilder) -> impl Future<Output = Result<GetTestSuiteOutput, SdkError<GetTestSuiteError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_cases(&self, builder: ListTestCasesInputBuilder) -> impl Future<Output = Result<ListTestCasesOutput, SdkError<ListTestCasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_configurations(&self, builder: ListTestConfigurationsInputBuilder) -> impl Future<Output = Result<ListTestConfigurationsOutput, SdkError<ListTestConfigurationsError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_run_steps(&self, builder: ListTestRunStepsInputBuilder) -> impl Future<Output = Result<ListTestRunStepsOutput, SdkError<ListTestRunStepsError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_run_test_cases(&self, builder: ListTestRunTestCasesInputBuilder) -> impl Future<Output = Result<ListTestRunTestCasesOutput, SdkError<ListTestRunTestCasesError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_runs(&self, builder: ListTestRunsInputBuilder) -> impl Future<Output = Result<ListTestRunsOutput, SdkError<ListTestRunsError>>> {
        builder.send_with(&self.0)
    }
    fn list_test_suites(&self, builder: ListTestSuitesInputBuilder) -> impl Future<Output = Result<ListTestSuitesOutput, SdkError<ListTestSuitesError>>> {
        builder.send_with(&self.0)
    }
    fn start_test_run(&self, builder: StartTestRunInputBuilder) -> impl Future<Output = Result<StartTestRunOutput, SdkError<StartTestRunError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_test_case(&self, builder: UpdateTestCaseInputBuilder) -> impl Future<Output = Result<UpdateTestCaseOutput, SdkError<UpdateTestCaseError>>> {
        builder.send_with(&self.0)
    }
    fn update_test_configuration(&self, builder: UpdateTestConfigurationInputBuilder) -> impl Future<Output = Result<UpdateTestConfigurationOutput, SdkError<UpdateTestConfigurationError>>> {
        builder.send_with(&self.0)
    }
    fn update_test_suite(&self, builder: UpdateTestSuiteInputBuilder) -> impl Future<Output = Result<UpdateTestSuiteOutput, SdkError<UpdateTestSuiteError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppTestClient for T
where T: Deref,
      T::Target: AppTestClient {
    fn create_test_case(&self, builder: CreateTestCaseInputBuilder) -> impl Future<Output = Result<CreateTestCaseOutput, SdkError<CreateTestCaseError>>> {
        self.deref().create_test_case(builder)
    }
    fn create_test_configuration(&self, builder: CreateTestConfigurationInputBuilder) -> impl Future<Output = Result<CreateTestConfigurationOutput, SdkError<CreateTestConfigurationError>>> {
        self.deref().create_test_configuration(builder)
    }
    fn create_test_suite(&self, builder: CreateTestSuiteInputBuilder) -> impl Future<Output = Result<CreateTestSuiteOutput, SdkError<CreateTestSuiteError>>> {
        self.deref().create_test_suite(builder)
    }
    fn delete_test_case(&self, builder: DeleteTestCaseInputBuilder) -> impl Future<Output = Result<DeleteTestCaseOutput, SdkError<DeleteTestCaseError>>> {
        self.deref().delete_test_case(builder)
    }
    fn delete_test_configuration(&self, builder: DeleteTestConfigurationInputBuilder) -> impl Future<Output = Result<DeleteTestConfigurationOutput, SdkError<DeleteTestConfigurationError>>> {
        self.deref().delete_test_configuration(builder)
    }
    fn delete_test_run(&self, builder: DeleteTestRunInputBuilder) -> impl Future<Output = Result<DeleteTestRunOutput, SdkError<DeleteTestRunError>>> {
        self.deref().delete_test_run(builder)
    }
    fn delete_test_suite(&self, builder: DeleteTestSuiteInputBuilder) -> impl Future<Output = Result<DeleteTestSuiteOutput, SdkError<DeleteTestSuiteError>>> {
        self.deref().delete_test_suite(builder)
    }
    fn get_test_case(&self, builder: GetTestCaseInputBuilder) -> impl Future<Output = Result<GetTestCaseOutput, SdkError<GetTestCaseError>>> {
        self.deref().get_test_case(builder)
    }
    fn get_test_configuration(&self, builder: GetTestConfigurationInputBuilder) -> impl Future<Output = Result<GetTestConfigurationOutput, SdkError<GetTestConfigurationError>>> {
        self.deref().get_test_configuration(builder)
    }
    fn get_test_run_step(&self, builder: GetTestRunStepInputBuilder) -> impl Future<Output = Result<GetTestRunStepOutput, SdkError<GetTestRunStepError>>> {
        self.deref().get_test_run_step(builder)
    }
    fn get_test_suite(&self, builder: GetTestSuiteInputBuilder) -> impl Future<Output = Result<GetTestSuiteOutput, SdkError<GetTestSuiteError>>> {
        self.deref().get_test_suite(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_test_cases(&self, builder: ListTestCasesInputBuilder) -> impl Future<Output = Result<ListTestCasesOutput, SdkError<ListTestCasesError>>> {
        self.deref().list_test_cases(builder)
    }
    fn list_test_configurations(&self, builder: ListTestConfigurationsInputBuilder) -> impl Future<Output = Result<ListTestConfigurationsOutput, SdkError<ListTestConfigurationsError>>> {
        self.deref().list_test_configurations(builder)
    }
    fn list_test_run_steps(&self, builder: ListTestRunStepsInputBuilder) -> impl Future<Output = Result<ListTestRunStepsOutput, SdkError<ListTestRunStepsError>>> {
        self.deref().list_test_run_steps(builder)
    }
    fn list_test_run_test_cases(&self, builder: ListTestRunTestCasesInputBuilder) -> impl Future<Output = Result<ListTestRunTestCasesOutput, SdkError<ListTestRunTestCasesError>>> {
        self.deref().list_test_run_test_cases(builder)
    }
    fn list_test_runs(&self, builder: ListTestRunsInputBuilder) -> impl Future<Output = Result<ListTestRunsOutput, SdkError<ListTestRunsError>>> {
        self.deref().list_test_runs(builder)
    }
    fn list_test_suites(&self, builder: ListTestSuitesInputBuilder) -> impl Future<Output = Result<ListTestSuitesOutput, SdkError<ListTestSuitesError>>> {
        self.deref().list_test_suites(builder)
    }
    fn start_test_run(&self, builder: StartTestRunInputBuilder) -> impl Future<Output = Result<StartTestRunOutput, SdkError<StartTestRunError>>> {
        self.deref().start_test_run(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_test_case(&self, builder: UpdateTestCaseInputBuilder) -> impl Future<Output = Result<UpdateTestCaseOutput, SdkError<UpdateTestCaseError>>> {
        self.deref().update_test_case(builder)
    }
    fn update_test_configuration(&self, builder: UpdateTestConfigurationInputBuilder) -> impl Future<Output = Result<UpdateTestConfigurationOutput, SdkError<UpdateTestConfigurationError>>> {
        self.deref().update_test_configuration(builder)
    }
    fn update_test_suite(&self, builder: UpdateTestSuiteInputBuilder) -> impl Future<Output = Result<UpdateTestSuiteOutput, SdkError<UpdateTestSuiteError>>> {
        self.deref().update_test_suite(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppTestClient {}
    impl AppTestClient for edAppTestClient {
        async fn create_test_case(&self, builder: CreateTestCaseInputBuilder) -> Result<CreateTestCaseOutput, SdkError<CreateTestCaseError>>;
        async fn create_test_configuration(&self, builder: CreateTestConfigurationInputBuilder) -> Result<CreateTestConfigurationOutput, SdkError<CreateTestConfigurationError>>;
        async fn create_test_suite(&self, builder: CreateTestSuiteInputBuilder) -> Result<CreateTestSuiteOutput, SdkError<CreateTestSuiteError>>;
        async fn delete_test_case(&self, builder: DeleteTestCaseInputBuilder) -> Result<DeleteTestCaseOutput, SdkError<DeleteTestCaseError>>;
        async fn delete_test_configuration(&self, builder: DeleteTestConfigurationInputBuilder) -> Result<DeleteTestConfigurationOutput, SdkError<DeleteTestConfigurationError>>;
        async fn delete_test_run(&self, builder: DeleteTestRunInputBuilder) -> Result<DeleteTestRunOutput, SdkError<DeleteTestRunError>>;
        async fn delete_test_suite(&self, builder: DeleteTestSuiteInputBuilder) -> Result<DeleteTestSuiteOutput, SdkError<DeleteTestSuiteError>>;
        async fn get_test_case(&self, builder: GetTestCaseInputBuilder) -> Result<GetTestCaseOutput, SdkError<GetTestCaseError>>;
        async fn get_test_configuration(&self, builder: GetTestConfigurationInputBuilder) -> Result<GetTestConfigurationOutput, SdkError<GetTestConfigurationError>>;
        async fn get_test_run_step(&self, builder: GetTestRunStepInputBuilder) -> Result<GetTestRunStepOutput, SdkError<GetTestRunStepError>>;
        async fn get_test_suite(&self, builder: GetTestSuiteInputBuilder) -> Result<GetTestSuiteOutput, SdkError<GetTestSuiteError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_test_cases(&self, builder: ListTestCasesInputBuilder) -> Result<ListTestCasesOutput, SdkError<ListTestCasesError>>;
        async fn list_test_configurations(&self, builder: ListTestConfigurationsInputBuilder) -> Result<ListTestConfigurationsOutput, SdkError<ListTestConfigurationsError>>;
        async fn list_test_run_steps(&self, builder: ListTestRunStepsInputBuilder) -> Result<ListTestRunStepsOutput, SdkError<ListTestRunStepsError>>;
        async fn list_test_run_test_cases(&self, builder: ListTestRunTestCasesInputBuilder) -> Result<ListTestRunTestCasesOutput, SdkError<ListTestRunTestCasesError>>;
        async fn list_test_runs(&self, builder: ListTestRunsInputBuilder) -> Result<ListTestRunsOutput, SdkError<ListTestRunsError>>;
        async fn list_test_suites(&self, builder: ListTestSuitesInputBuilder) -> Result<ListTestSuitesOutput, SdkError<ListTestSuitesError>>;
        async fn start_test_run(&self, builder: StartTestRunInputBuilder) -> Result<StartTestRunOutput, SdkError<StartTestRunError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_test_case(&self, builder: UpdateTestCaseInputBuilder) -> Result<UpdateTestCaseOutput, SdkError<UpdateTestCaseError>>;
        async fn update_test_configuration(&self, builder: UpdateTestConfigurationInputBuilder) -> Result<UpdateTestConfigurationOutput, SdkError<UpdateTestConfigurationError>>;
        async fn update_test_suite(&self, builder: UpdateTestSuiteInputBuilder) -> Result<UpdateTestSuiteOutput, SdkError<UpdateTestSuiteError>>;
    }
}
