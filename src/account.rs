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
use aws_sdk_account::operation::accept_primary_email_update::{builders::*, *};
use aws_sdk_account::operation::delete_alternate_contact::{builders::*, *};
use aws_sdk_account::operation::disable_region::{builders::*, *};
use aws_sdk_account::operation::enable_region::{builders::*, *};
use aws_sdk_account::operation::get_alternate_contact::{builders::*, *};
use aws_sdk_account::operation::get_contact_information::{builders::*, *};
use aws_sdk_account::operation::get_primary_email::{builders::*, *};
use aws_sdk_account::operation::get_region_opt_status::{builders::*, *};
use aws_sdk_account::operation::list_regions::{builders::*, *};
use aws_sdk_account::operation::put_alternate_contact::{builders::*, *};
use aws_sdk_account::operation::put_contact_information::{builders::*, *};
use aws_sdk_account::operation::start_primary_email_update::{builders::*, *};
use aws_sdk_account::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use aws_sdk_account::Client;

pub use aws_sdk_account::*;

pub struct AccountClientImpl(Client);
impl AccountClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AccountClient {
    fn accept_primary_email_update(&self, builder: AcceptPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<AcceptPrimaryEmailUpdateOutput, SdkError<AcceptPrimaryEmailUpdateError>>>;
    fn delete_alternate_contact(&self, builder: DeleteAlternateContactInputBuilder) -> impl Future<Output = Result<DeleteAlternateContactOutput, SdkError<DeleteAlternateContactError>>>;
    fn disable_region(&self, builder: DisableRegionInputBuilder) -> impl Future<Output = Result<DisableRegionOutput, SdkError<DisableRegionError>>>;
    fn enable_region(&self, builder: EnableRegionInputBuilder) -> impl Future<Output = Result<EnableRegionOutput, SdkError<EnableRegionError>>>;
    fn get_alternate_contact(&self, builder: GetAlternateContactInputBuilder) -> impl Future<Output = Result<GetAlternateContactOutput, SdkError<GetAlternateContactError>>>;
    fn get_contact_information(&self, builder: GetContactInformationInputBuilder) -> impl Future<Output = Result<GetContactInformationOutput, SdkError<GetContactInformationError>>>;
    fn get_primary_email(&self, builder: GetPrimaryEmailInputBuilder) -> impl Future<Output = Result<GetPrimaryEmailOutput, SdkError<GetPrimaryEmailError>>>;
    fn get_region_opt_status(&self, builder: GetRegionOptStatusInputBuilder) -> impl Future<Output = Result<GetRegionOptStatusOutput, SdkError<GetRegionOptStatusError>>>;
    fn list_regions(&self, builder: ListRegionsInputBuilder) -> impl Future<Output = Result<ListRegionsOutput, SdkError<ListRegionsError>>>;
    fn put_alternate_contact(&self, builder: PutAlternateContactInputBuilder) -> impl Future<Output = Result<PutAlternateContactOutput, SdkError<PutAlternateContactError>>>;
    fn put_contact_information(&self, builder: PutContactInformationInputBuilder) -> impl Future<Output = Result<PutContactInformationOutput, SdkError<PutContactInformationError>>>;
    fn start_primary_email_update(&self, builder: StartPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<StartPrimaryEmailUpdateOutput, SdkError<StartPrimaryEmailUpdateError>>>;
}
impl AccountClient for AccountClientImpl {
    fn accept_primary_email_update(&self, builder: AcceptPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<AcceptPrimaryEmailUpdateOutput, SdkError<AcceptPrimaryEmailUpdateError>>> {
        builder.send_with(&self.0)
    }
    fn delete_alternate_contact(&self, builder: DeleteAlternateContactInputBuilder) -> impl Future<Output = Result<DeleteAlternateContactOutput, SdkError<DeleteAlternateContactError>>> {
        builder.send_with(&self.0)
    }
    fn disable_region(&self, builder: DisableRegionInputBuilder) -> impl Future<Output = Result<DisableRegionOutput, SdkError<DisableRegionError>>> {
        builder.send_with(&self.0)
    }
    fn enable_region(&self, builder: EnableRegionInputBuilder) -> impl Future<Output = Result<EnableRegionOutput, SdkError<EnableRegionError>>> {
        builder.send_with(&self.0)
    }
    fn get_alternate_contact(&self, builder: GetAlternateContactInputBuilder) -> impl Future<Output = Result<GetAlternateContactOutput, SdkError<GetAlternateContactError>>> {
        builder.send_with(&self.0)
    }
    fn get_contact_information(&self, builder: GetContactInformationInputBuilder) -> impl Future<Output = Result<GetContactInformationOutput, SdkError<GetContactInformationError>>> {
        builder.send_with(&self.0)
    }
    fn get_primary_email(&self, builder: GetPrimaryEmailInputBuilder) -> impl Future<Output = Result<GetPrimaryEmailOutput, SdkError<GetPrimaryEmailError>>> {
        builder.send_with(&self.0)
    }
    fn get_region_opt_status(&self, builder: GetRegionOptStatusInputBuilder) -> impl Future<Output = Result<GetRegionOptStatusOutput, SdkError<GetRegionOptStatusError>>> {
        builder.send_with(&self.0)
    }
    fn list_regions(&self, builder: ListRegionsInputBuilder) -> impl Future<Output = Result<ListRegionsOutput, SdkError<ListRegionsError>>> {
        builder.send_with(&self.0)
    }
    fn put_alternate_contact(&self, builder: PutAlternateContactInputBuilder) -> impl Future<Output = Result<PutAlternateContactOutput, SdkError<PutAlternateContactError>>> {
        builder.send_with(&self.0)
    }
    fn put_contact_information(&self, builder: PutContactInformationInputBuilder) -> impl Future<Output = Result<PutContactInformationOutput, SdkError<PutContactInformationError>>> {
        builder.send_with(&self.0)
    }
    fn start_primary_email_update(&self, builder: StartPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<StartPrimaryEmailUpdateOutput, SdkError<StartPrimaryEmailUpdateError>>> {
        builder.send_with(&self.0)
    }
}
impl <T: AccountClient> AccountClient for &T {
    fn accept_primary_email_update(&self, builder: AcceptPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<AcceptPrimaryEmailUpdateOutput, SdkError<AcceptPrimaryEmailUpdateError>>> {
        (*self).accept_primary_email_update(builder)
    }
    fn delete_alternate_contact(&self, builder: DeleteAlternateContactInputBuilder) -> impl Future<Output = Result<DeleteAlternateContactOutput, SdkError<DeleteAlternateContactError>>> {
        (*self).delete_alternate_contact(builder)
    }
    fn disable_region(&self, builder: DisableRegionInputBuilder) -> impl Future<Output = Result<DisableRegionOutput, SdkError<DisableRegionError>>> {
        (*self).disable_region(builder)
    }
    fn enable_region(&self, builder: EnableRegionInputBuilder) -> impl Future<Output = Result<EnableRegionOutput, SdkError<EnableRegionError>>> {
        (*self).enable_region(builder)
    }
    fn get_alternate_contact(&self, builder: GetAlternateContactInputBuilder) -> impl Future<Output = Result<GetAlternateContactOutput, SdkError<GetAlternateContactError>>> {
        (*self).get_alternate_contact(builder)
    }
    fn get_contact_information(&self, builder: GetContactInformationInputBuilder) -> impl Future<Output = Result<GetContactInformationOutput, SdkError<GetContactInformationError>>> {
        (*self).get_contact_information(builder)
    }
    fn get_primary_email(&self, builder: GetPrimaryEmailInputBuilder) -> impl Future<Output = Result<GetPrimaryEmailOutput, SdkError<GetPrimaryEmailError>>> {
        (*self).get_primary_email(builder)
    }
    fn get_region_opt_status(&self, builder: GetRegionOptStatusInputBuilder) -> impl Future<Output = Result<GetRegionOptStatusOutput, SdkError<GetRegionOptStatusError>>> {
        (*self).get_region_opt_status(builder)
    }
    fn list_regions(&self, builder: ListRegionsInputBuilder) -> impl Future<Output = Result<ListRegionsOutput, SdkError<ListRegionsError>>> {
        (*self).list_regions(builder)
    }
    fn put_alternate_contact(&self, builder: PutAlternateContactInputBuilder) -> impl Future<Output = Result<PutAlternateContactOutput, SdkError<PutAlternateContactError>>> {
        (*self).put_alternate_contact(builder)
    }
    fn put_contact_information(&self, builder: PutContactInformationInputBuilder) -> impl Future<Output = Result<PutContactInformationOutput, SdkError<PutContactInformationError>>> {
        (*self).put_contact_information(builder)
    }
    fn start_primary_email_update(&self, builder: StartPrimaryEmailUpdateInputBuilder) -> impl Future<Output = Result<StartPrimaryEmailUpdateOutput, SdkError<StartPrimaryEmailUpdateError>>> {
        (*self).start_primary_email_update(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAccountClient {}
    impl AccountClient for edAccountClient {
        async fn accept_primary_email_update(&self, builder: AcceptPrimaryEmailUpdateInputBuilder) -> Result<AcceptPrimaryEmailUpdateOutput, SdkError<AcceptPrimaryEmailUpdateError>>;
        async fn delete_alternate_contact(&self, builder: DeleteAlternateContactInputBuilder) -> Result<DeleteAlternateContactOutput, SdkError<DeleteAlternateContactError>>;
        async fn disable_region(&self, builder: DisableRegionInputBuilder) -> Result<DisableRegionOutput, SdkError<DisableRegionError>>;
        async fn enable_region(&self, builder: EnableRegionInputBuilder) -> Result<EnableRegionOutput, SdkError<EnableRegionError>>;
        async fn get_alternate_contact(&self, builder: GetAlternateContactInputBuilder) -> Result<GetAlternateContactOutput, SdkError<GetAlternateContactError>>;
        async fn get_contact_information(&self, builder: GetContactInformationInputBuilder) -> Result<GetContactInformationOutput, SdkError<GetContactInformationError>>;
        async fn get_primary_email(&self, builder: GetPrimaryEmailInputBuilder) -> Result<GetPrimaryEmailOutput, SdkError<GetPrimaryEmailError>>;
        async fn get_region_opt_status(&self, builder: GetRegionOptStatusInputBuilder) -> Result<GetRegionOptStatusOutput, SdkError<GetRegionOptStatusError>>;
        async fn list_regions(&self, builder: ListRegionsInputBuilder) -> Result<ListRegionsOutput, SdkError<ListRegionsError>>;
        async fn put_alternate_contact(&self, builder: PutAlternateContactInputBuilder) -> Result<PutAlternateContactOutput, SdkError<PutAlternateContactError>>;
        async fn put_contact_information(&self, builder: PutContactInformationInputBuilder) -> Result<PutContactInformationOutput, SdkError<PutContactInformationError>>;
        async fn start_primary_email_update(&self, builder: StartPrimaryEmailUpdateInputBuilder) -> Result<StartPrimaryEmailUpdateOutput, SdkError<StartPrimaryEmailUpdateError>>;
    }
}
