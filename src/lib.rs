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
#[cfg(feature="access-analyzer")]
pub mod access_analyzer;
#[cfg(feature="account")]
pub mod account;
#[cfg(feature="acm")]
pub mod acm;
#[cfg(feature="acm-pca")]
pub mod acm_pca;
#[cfg(feature="amp")]
pub mod amp;
#[cfg(feature="amplify")]
pub mod amplify;
#[cfg(feature="amplify-backend")]
pub mod amplify_backend;
#[cfg(feature="amplify-ui-builder")]
pub mod amplify_ui_builder;
#[cfg(feature="api-gateway")]
pub mod api_gateway;
#[cfg(feature="api-gateway-management")]
pub mod api_gateway_management;
#[cfg(feature="api-gateway-v2")]
pub mod api_gateway_v2;
#[cfg(feature="app-config")]
pub mod app_config;
#[cfg(feature="app-config-data")]
pub mod app_config_data;
#[cfg(feature="app-fabric")]
pub mod app_fabric;
#[cfg(feature="app-flow")]
pub mod app_flow;
#[cfg(feature="app-integrations")]
pub mod app_integrations;
#[cfg(feature="app-runner")]
pub mod app_runner;
#[cfg(feature="app-sync")]
pub mod app_sync;
#[cfg(feature="app-test")]
pub mod app_test;
#[cfg(feature="athena")]
pub mod athena;
#[cfg(feature="backup")]
pub mod backup;
#[cfg(feature="batch")]
pub mod batch;
#[cfg(feature="cloud-formation")]
pub mod cloud_formation;
#[cfg(feature="cloud-front")]
pub mod cloud_front;
#[cfg(feature="cloud-trail")]
pub mod cloud_trail;
#[cfg(feature="cloud-watch")]
pub mod cloud_watch;
#[cfg(feature="code-build")]
pub mod code_build;
#[cfg(feature="code-commit")]
pub mod code_commit;
#[cfg(feature="code-pipeline")]
pub mod code_pipeline;
#[cfg(feature="cognito-identity-provider")]
pub mod cognito_identity_provider;
#[cfg(feature="data-brew")]
pub mod data_brew;
#[cfg(feature="data-pipeline")]
pub mod data_pipeline;
#[cfg(feature="direct-connect")]
pub mod direct_connect;
#[cfg(feature="dynamo-db")]
pub mod dynamo_db;
#[cfg(feature="ec2")]
pub mod ec2;
#[cfg(feature="efs")]
pub mod efs;
#[cfg(feature="eks")]
pub mod eks;
#[cfg(feature="elasti-cache")]
pub mod elasti_cache;
#[cfg(feature="elastic-beanstalk")]
pub mod elastic_beanstalk;
#[cfg(feature="emr")]
pub mod emr;
#[cfg(feature="global-accelerator")]
pub mod global_accelerator;
#[cfg(feature="glue")]
pub mod glue;
#[cfg(feature="iam")]
pub mod iam;
#[cfg(feature="kinesis")]
pub mod kinesis;
#[cfg(feature="kms")]
pub mod kms;
#[cfg(feature="lambda")]
pub mod lambda;
#[cfg(feature="macie2")]
pub mod macie2;
#[cfg(feature="mq")]
pub mod mq;
#[cfg(feature="open-search")]
pub mod open_search;
#[cfg(feature="polly")]
pub mod polly;
#[cfg(feature="quick-sight")]
pub mod quick_sight;
#[cfg(feature="rds")]
pub mod rds;
#[cfg(feature="redshift")]
pub mod redshift;
#[cfg(feature="rekognition")]
pub mod rekognition;
#[cfg(feature="s3")]
pub mod s3;
#[cfg(feature="sage-maker")]
pub mod sage_maker;
#[cfg(feature="secrets-manager")]
pub mod secrets_manager;
#[cfg(feature="sns")]
pub mod sns;
#[cfg(feature="sqs")]
pub mod sqs;
#[cfg(feature="ssm")]
pub mod ssm;
#[cfg(feature="textract")]
pub mod textract;
#[cfg(feature="transcribe")]
pub mod transcribe;
#[cfg(feature="translate")]
pub mod translate;
#[cfg(feature="x-ray")]
pub mod x_ray;

