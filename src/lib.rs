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
#[cfg(feature="kms")]
pub mod kms;
#[cfg(feature="api-gateway-v2")]
pub mod api_gateway_v2;
#[cfg(feature="access-analyzer")]
pub mod access_analyzer;
#[cfg(feature="elasti-cache")]
pub mod elasti_cache;
#[cfg(feature="ec2")]
pub mod ec2;
#[cfg(feature="cognito-identity-provider")]
pub mod cognito_identity_provider;
#[cfg(feature="secrets-manager")]
pub mod secrets_manager;
#[cfg(feature="api-gateway")]
pub mod api_gateway;
#[cfg(feature="s3")]
pub mod s3;
#[cfg(feature="dynamo-db")]
pub mod dynamo_db;
#[cfg(feature="account")]
pub mod account;

