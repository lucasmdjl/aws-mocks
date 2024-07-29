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
use aws_sdk_appmesh::operation::create_gateway_route::{builders::*, *};
use aws_sdk_appmesh::operation::create_mesh::{builders::*, *};
use aws_sdk_appmesh::operation::create_route::{builders::*, *};
use aws_sdk_appmesh::operation::create_virtual_gateway::{builders::*, *};
use aws_sdk_appmesh::operation::create_virtual_node::{builders::*, *};
use aws_sdk_appmesh::operation::create_virtual_router::{builders::*, *};
use aws_sdk_appmesh::operation::create_virtual_service::{builders::*, *};
use aws_sdk_appmesh::operation::delete_gateway_route::{builders::*, *};
use aws_sdk_appmesh::operation::delete_mesh::{builders::*, *};
use aws_sdk_appmesh::operation::delete_route::{builders::*, *};
use aws_sdk_appmesh::operation::delete_virtual_gateway::{builders::*, *};
use aws_sdk_appmesh::operation::delete_virtual_node::{builders::*, *};
use aws_sdk_appmesh::operation::delete_virtual_router::{builders::*, *};
use aws_sdk_appmesh::operation::delete_virtual_service::{builders::*, *};
use aws_sdk_appmesh::operation::describe_gateway_route::{builders::*, *};
use aws_sdk_appmesh::operation::describe_mesh::{builders::*, *};
use aws_sdk_appmesh::operation::describe_route::{builders::*, *};
use aws_sdk_appmesh::operation::describe_virtual_gateway::{builders::*, *};
use aws_sdk_appmesh::operation::describe_virtual_node::{builders::*, *};
use aws_sdk_appmesh::operation::describe_virtual_router::{builders::*, *};
use aws_sdk_appmesh::operation::describe_virtual_service::{builders::*, *};
use aws_sdk_appmesh::operation::list_gateway_routes::{builders::*, *};
use aws_sdk_appmesh::operation::list_meshes::{builders::*, *};
use aws_sdk_appmesh::operation::list_routes::{builders::*, *};
use aws_sdk_appmesh::operation::list_tags_for_resource::{builders::*, *};
use aws_sdk_appmesh::operation::list_virtual_gateways::{builders::*, *};
use aws_sdk_appmesh::operation::list_virtual_nodes::{builders::*, *};
use aws_sdk_appmesh::operation::list_virtual_routers::{builders::*, *};
use aws_sdk_appmesh::operation::list_virtual_services::{builders::*, *};
use aws_sdk_appmesh::operation::tag_resource::{builders::*, *};
use aws_sdk_appmesh::operation::untag_resource::{builders::*, *};
use aws_sdk_appmesh::operation::update_gateway_route::{builders::*, *};
use aws_sdk_appmesh::operation::update_mesh::{builders::*, *};
use aws_sdk_appmesh::operation::update_route::{builders::*, *};
use aws_sdk_appmesh::operation::update_virtual_gateway::{builders::*, *};
use aws_sdk_appmesh::operation::update_virtual_node::{builders::*, *};
use aws_sdk_appmesh::operation::update_virtual_router::{builders::*, *};
use aws_sdk_appmesh::operation::update_virtual_service::{builders::*, *};
use aws_sdk_appmesh::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
#[allow(hidden_glob_reexports)]
use aws_sdk_appmesh::Client;
use std::ops::Deref;

pub use aws_sdk_appmesh::*;

pub struct AppMeshClientImpl(Client);
impl AppMeshClientImpl {
    pub fn new(config: &SdkConfig) -> Self { Self(Client::new(config)) }
}
pub trait AppMeshClient {
    fn create_gateway_route(&self, builder: CreateGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateGatewayRouteOutput, SdkError<CreateGatewayRouteError>>>;
    fn create_mesh(&self, builder: CreateMeshInputBuilder) -> impl Future<Output = Result<CreateMeshOutput, SdkError<CreateMeshError>>>;
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>>;
    fn create_virtual_gateway(&self, builder: CreateVirtualGatewayInputBuilder) -> impl Future<Output = Result<CreateVirtualGatewayOutput, SdkError<CreateVirtualGatewayError>>>;
    fn create_virtual_node(&self, builder: CreateVirtualNodeInputBuilder) -> impl Future<Output = Result<CreateVirtualNodeOutput, SdkError<CreateVirtualNodeError>>>;
    fn create_virtual_router(&self, builder: CreateVirtualRouterInputBuilder) -> impl Future<Output = Result<CreateVirtualRouterOutput, SdkError<CreateVirtualRouterError>>>;
    fn create_virtual_service(&self, builder: CreateVirtualServiceInputBuilder) -> impl Future<Output = Result<CreateVirtualServiceOutput, SdkError<CreateVirtualServiceError>>>;
    fn delete_gateway_route(&self, builder: DeleteGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteGatewayRouteOutput, SdkError<DeleteGatewayRouteError>>>;
    fn delete_mesh(&self, builder: DeleteMeshInputBuilder) -> impl Future<Output = Result<DeleteMeshOutput, SdkError<DeleteMeshError>>>;
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>>;
    fn delete_virtual_gateway(&self, builder: DeleteVirtualGatewayInputBuilder) -> impl Future<Output = Result<DeleteVirtualGatewayOutput, SdkError<DeleteVirtualGatewayError>>>;
    fn delete_virtual_node(&self, builder: DeleteVirtualNodeInputBuilder) -> impl Future<Output = Result<DeleteVirtualNodeOutput, SdkError<DeleteVirtualNodeError>>>;
    fn delete_virtual_router(&self, builder: DeleteVirtualRouterInputBuilder) -> impl Future<Output = Result<DeleteVirtualRouterOutput, SdkError<DeleteVirtualRouterError>>>;
    fn delete_virtual_service(&self, builder: DeleteVirtualServiceInputBuilder) -> impl Future<Output = Result<DeleteVirtualServiceOutput, SdkError<DeleteVirtualServiceError>>>;
    fn describe_gateway_route(&self, builder: DescribeGatewayRouteInputBuilder) -> impl Future<Output = Result<DescribeGatewayRouteOutput, SdkError<DescribeGatewayRouteError>>>;
    fn describe_mesh(&self, builder: DescribeMeshInputBuilder) -> impl Future<Output = Result<DescribeMeshOutput, SdkError<DescribeMeshError>>>;
    fn describe_route(&self, builder: DescribeRouteInputBuilder) -> impl Future<Output = Result<DescribeRouteOutput, SdkError<DescribeRouteError>>>;
    fn describe_virtual_gateway(&self, builder: DescribeVirtualGatewayInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewayOutput, SdkError<DescribeVirtualGatewayError>>>;
    fn describe_virtual_node(&self, builder: DescribeVirtualNodeInputBuilder) -> impl Future<Output = Result<DescribeVirtualNodeOutput, SdkError<DescribeVirtualNodeError>>>;
    fn describe_virtual_router(&self, builder: DescribeVirtualRouterInputBuilder) -> impl Future<Output = Result<DescribeVirtualRouterOutput, SdkError<DescribeVirtualRouterError>>>;
    fn describe_virtual_service(&self, builder: DescribeVirtualServiceInputBuilder) -> impl Future<Output = Result<DescribeVirtualServiceOutput, SdkError<DescribeVirtualServiceError>>>;
    fn list_gateway_routes(&self, builder: ListGatewayRoutesInputBuilder) -> impl Future<Output = Result<ListGatewayRoutesOutput, SdkError<ListGatewayRoutesError>>>;
    fn list_meshes(&self, builder: ListMeshesInputBuilder) -> impl Future<Output = Result<ListMeshesOutput, SdkError<ListMeshesError>>>;
    fn list_routes(&self, builder: ListRoutesInputBuilder) -> impl Future<Output = Result<ListRoutesOutput, SdkError<ListRoutesError>>>;
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>>;
    fn list_virtual_gateways(&self, builder: ListVirtualGatewaysInputBuilder) -> impl Future<Output = Result<ListVirtualGatewaysOutput, SdkError<ListVirtualGatewaysError>>>;
    fn list_virtual_nodes(&self, builder: ListVirtualNodesInputBuilder) -> impl Future<Output = Result<ListVirtualNodesOutput, SdkError<ListVirtualNodesError>>>;
    fn list_virtual_routers(&self, builder: ListVirtualRoutersInputBuilder) -> impl Future<Output = Result<ListVirtualRoutersOutput, SdkError<ListVirtualRoutersError>>>;
    fn list_virtual_services(&self, builder: ListVirtualServicesInputBuilder) -> impl Future<Output = Result<ListVirtualServicesOutput, SdkError<ListVirtualServicesError>>>;
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>>;
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>>;
    fn update_gateway_route(&self, builder: UpdateGatewayRouteInputBuilder) -> impl Future<Output = Result<UpdateGatewayRouteOutput, SdkError<UpdateGatewayRouteError>>>;
    fn update_mesh(&self, builder: UpdateMeshInputBuilder) -> impl Future<Output = Result<UpdateMeshOutput, SdkError<UpdateMeshError>>>;
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>>;
    fn update_virtual_gateway(&self, builder: UpdateVirtualGatewayInputBuilder) -> impl Future<Output = Result<UpdateVirtualGatewayOutput, SdkError<UpdateVirtualGatewayError>>>;
    fn update_virtual_node(&self, builder: UpdateVirtualNodeInputBuilder) -> impl Future<Output = Result<UpdateVirtualNodeOutput, SdkError<UpdateVirtualNodeError>>>;
    fn update_virtual_router(&self, builder: UpdateVirtualRouterInputBuilder) -> impl Future<Output = Result<UpdateVirtualRouterOutput, SdkError<UpdateVirtualRouterError>>>;
    fn update_virtual_service(&self, builder: UpdateVirtualServiceInputBuilder) -> impl Future<Output = Result<UpdateVirtualServiceOutput, SdkError<UpdateVirtualServiceError>>>;
}
impl AppMeshClient for AppMeshClientImpl {
    fn create_gateway_route(&self, builder: CreateGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateGatewayRouteOutput, SdkError<CreateGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_mesh(&self, builder: CreateMeshInputBuilder) -> impl Future<Output = Result<CreateMeshOutput, SdkError<CreateMeshError>>> {
        builder.send_with(&self.0)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        builder.send_with(&self.0)
    }
    fn create_virtual_gateway(&self, builder: CreateVirtualGatewayInputBuilder) -> impl Future<Output = Result<CreateVirtualGatewayOutput, SdkError<CreateVirtualGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn create_virtual_node(&self, builder: CreateVirtualNodeInputBuilder) -> impl Future<Output = Result<CreateVirtualNodeOutput, SdkError<CreateVirtualNodeError>>> {
        builder.send_with(&self.0)
    }
    fn create_virtual_router(&self, builder: CreateVirtualRouterInputBuilder) -> impl Future<Output = Result<CreateVirtualRouterOutput, SdkError<CreateVirtualRouterError>>> {
        builder.send_with(&self.0)
    }
    fn create_virtual_service(&self, builder: CreateVirtualServiceInputBuilder) -> impl Future<Output = Result<CreateVirtualServiceOutput, SdkError<CreateVirtualServiceError>>> {
        builder.send_with(&self.0)
    }
    fn delete_gateway_route(&self, builder: DeleteGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteGatewayRouteOutput, SdkError<DeleteGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_mesh(&self, builder: DeleteMeshInputBuilder) -> impl Future<Output = Result<DeleteMeshOutput, SdkError<DeleteMeshError>>> {
        builder.send_with(&self.0)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_gateway(&self, builder: DeleteVirtualGatewayInputBuilder) -> impl Future<Output = Result<DeleteVirtualGatewayOutput, SdkError<DeleteVirtualGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_node(&self, builder: DeleteVirtualNodeInputBuilder) -> impl Future<Output = Result<DeleteVirtualNodeOutput, SdkError<DeleteVirtualNodeError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_router(&self, builder: DeleteVirtualRouterInputBuilder) -> impl Future<Output = Result<DeleteVirtualRouterOutput, SdkError<DeleteVirtualRouterError>>> {
        builder.send_with(&self.0)
    }
    fn delete_virtual_service(&self, builder: DeleteVirtualServiceInputBuilder) -> impl Future<Output = Result<DeleteVirtualServiceOutput, SdkError<DeleteVirtualServiceError>>> {
        builder.send_with(&self.0)
    }
    fn describe_gateway_route(&self, builder: DescribeGatewayRouteInputBuilder) -> impl Future<Output = Result<DescribeGatewayRouteOutput, SdkError<DescribeGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn describe_mesh(&self, builder: DescribeMeshInputBuilder) -> impl Future<Output = Result<DescribeMeshOutput, SdkError<DescribeMeshError>>> {
        builder.send_with(&self.0)
    }
    fn describe_route(&self, builder: DescribeRouteInputBuilder) -> impl Future<Output = Result<DescribeRouteOutput, SdkError<DescribeRouteError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_gateway(&self, builder: DescribeVirtualGatewayInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewayOutput, SdkError<DescribeVirtualGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_node(&self, builder: DescribeVirtualNodeInputBuilder) -> impl Future<Output = Result<DescribeVirtualNodeOutput, SdkError<DescribeVirtualNodeError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_router(&self, builder: DescribeVirtualRouterInputBuilder) -> impl Future<Output = Result<DescribeVirtualRouterOutput, SdkError<DescribeVirtualRouterError>>> {
        builder.send_with(&self.0)
    }
    fn describe_virtual_service(&self, builder: DescribeVirtualServiceInputBuilder) -> impl Future<Output = Result<DescribeVirtualServiceOutput, SdkError<DescribeVirtualServiceError>>> {
        builder.send_with(&self.0)
    }
    fn list_gateway_routes(&self, builder: ListGatewayRoutesInputBuilder) -> impl Future<Output = Result<ListGatewayRoutesOutput, SdkError<ListGatewayRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn list_meshes(&self, builder: ListMeshesInputBuilder) -> impl Future<Output = Result<ListMeshesOutput, SdkError<ListMeshesError>>> {
        builder.send_with(&self.0)
    }
    fn list_routes(&self, builder: ListRoutesInputBuilder) -> impl Future<Output = Result<ListRoutesOutput, SdkError<ListRoutesError>>> {
        builder.send_with(&self.0)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_gateways(&self, builder: ListVirtualGatewaysInputBuilder) -> impl Future<Output = Result<ListVirtualGatewaysOutput, SdkError<ListVirtualGatewaysError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_nodes(&self, builder: ListVirtualNodesInputBuilder) -> impl Future<Output = Result<ListVirtualNodesOutput, SdkError<ListVirtualNodesError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_routers(&self, builder: ListVirtualRoutersInputBuilder) -> impl Future<Output = Result<ListVirtualRoutersOutput, SdkError<ListVirtualRoutersError>>> {
        builder.send_with(&self.0)
    }
    fn list_virtual_services(&self, builder: ListVirtualServicesInputBuilder) -> impl Future<Output = Result<ListVirtualServicesOutput, SdkError<ListVirtualServicesError>>> {
        builder.send_with(&self.0)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        builder.send_with(&self.0)
    }
    fn update_gateway_route(&self, builder: UpdateGatewayRouteInputBuilder) -> impl Future<Output = Result<UpdateGatewayRouteOutput, SdkError<UpdateGatewayRouteError>>> {
        builder.send_with(&self.0)
    }
    fn update_mesh(&self, builder: UpdateMeshInputBuilder) -> impl Future<Output = Result<UpdateMeshOutput, SdkError<UpdateMeshError>>> {
        builder.send_with(&self.0)
    }
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> {
        builder.send_with(&self.0)
    }
    fn update_virtual_gateway(&self, builder: UpdateVirtualGatewayInputBuilder) -> impl Future<Output = Result<UpdateVirtualGatewayOutput, SdkError<UpdateVirtualGatewayError>>> {
        builder.send_with(&self.0)
    }
    fn update_virtual_node(&self, builder: UpdateVirtualNodeInputBuilder) -> impl Future<Output = Result<UpdateVirtualNodeOutput, SdkError<UpdateVirtualNodeError>>> {
        builder.send_with(&self.0)
    }
    fn update_virtual_router(&self, builder: UpdateVirtualRouterInputBuilder) -> impl Future<Output = Result<UpdateVirtualRouterOutput, SdkError<UpdateVirtualRouterError>>> {
        builder.send_with(&self.0)
    }
    fn update_virtual_service(&self, builder: UpdateVirtualServiceInputBuilder) -> impl Future<Output = Result<UpdateVirtualServiceOutput, SdkError<UpdateVirtualServiceError>>> {
        builder.send_with(&self.0)
    }
}
impl <T> AppMeshClient for T
where T: Deref,
      T::Target: AppMeshClient {
    fn create_gateway_route(&self, builder: CreateGatewayRouteInputBuilder) -> impl Future<Output = Result<CreateGatewayRouteOutput, SdkError<CreateGatewayRouteError>>> {
        self.deref().create_gateway_route(builder)
    }
    fn create_mesh(&self, builder: CreateMeshInputBuilder) -> impl Future<Output = Result<CreateMeshOutput, SdkError<CreateMeshError>>> {
        self.deref().create_mesh(builder)
    }
    fn create_route(&self, builder: CreateRouteInputBuilder) -> impl Future<Output = Result<CreateRouteOutput, SdkError<CreateRouteError>>> {
        self.deref().create_route(builder)
    }
    fn create_virtual_gateway(&self, builder: CreateVirtualGatewayInputBuilder) -> impl Future<Output = Result<CreateVirtualGatewayOutput, SdkError<CreateVirtualGatewayError>>> {
        self.deref().create_virtual_gateway(builder)
    }
    fn create_virtual_node(&self, builder: CreateVirtualNodeInputBuilder) -> impl Future<Output = Result<CreateVirtualNodeOutput, SdkError<CreateVirtualNodeError>>> {
        self.deref().create_virtual_node(builder)
    }
    fn create_virtual_router(&self, builder: CreateVirtualRouterInputBuilder) -> impl Future<Output = Result<CreateVirtualRouterOutput, SdkError<CreateVirtualRouterError>>> {
        self.deref().create_virtual_router(builder)
    }
    fn create_virtual_service(&self, builder: CreateVirtualServiceInputBuilder) -> impl Future<Output = Result<CreateVirtualServiceOutput, SdkError<CreateVirtualServiceError>>> {
        self.deref().create_virtual_service(builder)
    }
    fn delete_gateway_route(&self, builder: DeleteGatewayRouteInputBuilder) -> impl Future<Output = Result<DeleteGatewayRouteOutput, SdkError<DeleteGatewayRouteError>>> {
        self.deref().delete_gateway_route(builder)
    }
    fn delete_mesh(&self, builder: DeleteMeshInputBuilder) -> impl Future<Output = Result<DeleteMeshOutput, SdkError<DeleteMeshError>>> {
        self.deref().delete_mesh(builder)
    }
    fn delete_route(&self, builder: DeleteRouteInputBuilder) -> impl Future<Output = Result<DeleteRouteOutput, SdkError<DeleteRouteError>>> {
        self.deref().delete_route(builder)
    }
    fn delete_virtual_gateway(&self, builder: DeleteVirtualGatewayInputBuilder) -> impl Future<Output = Result<DeleteVirtualGatewayOutput, SdkError<DeleteVirtualGatewayError>>> {
        self.deref().delete_virtual_gateway(builder)
    }
    fn delete_virtual_node(&self, builder: DeleteVirtualNodeInputBuilder) -> impl Future<Output = Result<DeleteVirtualNodeOutput, SdkError<DeleteVirtualNodeError>>> {
        self.deref().delete_virtual_node(builder)
    }
    fn delete_virtual_router(&self, builder: DeleteVirtualRouterInputBuilder) -> impl Future<Output = Result<DeleteVirtualRouterOutput, SdkError<DeleteVirtualRouterError>>> {
        self.deref().delete_virtual_router(builder)
    }
    fn delete_virtual_service(&self, builder: DeleteVirtualServiceInputBuilder) -> impl Future<Output = Result<DeleteVirtualServiceOutput, SdkError<DeleteVirtualServiceError>>> {
        self.deref().delete_virtual_service(builder)
    }
    fn describe_gateway_route(&self, builder: DescribeGatewayRouteInputBuilder) -> impl Future<Output = Result<DescribeGatewayRouteOutput, SdkError<DescribeGatewayRouteError>>> {
        self.deref().describe_gateway_route(builder)
    }
    fn describe_mesh(&self, builder: DescribeMeshInputBuilder) -> impl Future<Output = Result<DescribeMeshOutput, SdkError<DescribeMeshError>>> {
        self.deref().describe_mesh(builder)
    }
    fn describe_route(&self, builder: DescribeRouteInputBuilder) -> impl Future<Output = Result<DescribeRouteOutput, SdkError<DescribeRouteError>>> {
        self.deref().describe_route(builder)
    }
    fn describe_virtual_gateway(&self, builder: DescribeVirtualGatewayInputBuilder) -> impl Future<Output = Result<DescribeVirtualGatewayOutput, SdkError<DescribeVirtualGatewayError>>> {
        self.deref().describe_virtual_gateway(builder)
    }
    fn describe_virtual_node(&self, builder: DescribeVirtualNodeInputBuilder) -> impl Future<Output = Result<DescribeVirtualNodeOutput, SdkError<DescribeVirtualNodeError>>> {
        self.deref().describe_virtual_node(builder)
    }
    fn describe_virtual_router(&self, builder: DescribeVirtualRouterInputBuilder) -> impl Future<Output = Result<DescribeVirtualRouterOutput, SdkError<DescribeVirtualRouterError>>> {
        self.deref().describe_virtual_router(builder)
    }
    fn describe_virtual_service(&self, builder: DescribeVirtualServiceInputBuilder) -> impl Future<Output = Result<DescribeVirtualServiceOutput, SdkError<DescribeVirtualServiceError>>> {
        self.deref().describe_virtual_service(builder)
    }
    fn list_gateway_routes(&self, builder: ListGatewayRoutesInputBuilder) -> impl Future<Output = Result<ListGatewayRoutesOutput, SdkError<ListGatewayRoutesError>>> {
        self.deref().list_gateway_routes(builder)
    }
    fn list_meshes(&self, builder: ListMeshesInputBuilder) -> impl Future<Output = Result<ListMeshesOutput, SdkError<ListMeshesError>>> {
        self.deref().list_meshes(builder)
    }
    fn list_routes(&self, builder: ListRoutesInputBuilder) -> impl Future<Output = Result<ListRoutesOutput, SdkError<ListRoutesError>>> {
        self.deref().list_routes(builder)
    }
    fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> impl Future<Output = Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>> {
        self.deref().list_tags_for_resource(builder)
    }
    fn list_virtual_gateways(&self, builder: ListVirtualGatewaysInputBuilder) -> impl Future<Output = Result<ListVirtualGatewaysOutput, SdkError<ListVirtualGatewaysError>>> {
        self.deref().list_virtual_gateways(builder)
    }
    fn list_virtual_nodes(&self, builder: ListVirtualNodesInputBuilder) -> impl Future<Output = Result<ListVirtualNodesOutput, SdkError<ListVirtualNodesError>>> {
        self.deref().list_virtual_nodes(builder)
    }
    fn list_virtual_routers(&self, builder: ListVirtualRoutersInputBuilder) -> impl Future<Output = Result<ListVirtualRoutersOutput, SdkError<ListVirtualRoutersError>>> {
        self.deref().list_virtual_routers(builder)
    }
    fn list_virtual_services(&self, builder: ListVirtualServicesInputBuilder) -> impl Future<Output = Result<ListVirtualServicesOutput, SdkError<ListVirtualServicesError>>> {
        self.deref().list_virtual_services(builder)
    }
    fn tag_resource(&self, builder: TagResourceInputBuilder) -> impl Future<Output = Result<TagResourceOutput, SdkError<TagResourceError>>> {
        self.deref().tag_resource(builder)
    }
    fn untag_resource(&self, builder: UntagResourceInputBuilder) -> impl Future<Output = Result<UntagResourceOutput, SdkError<UntagResourceError>>> {
        self.deref().untag_resource(builder)
    }
    fn update_gateway_route(&self, builder: UpdateGatewayRouteInputBuilder) -> impl Future<Output = Result<UpdateGatewayRouteOutput, SdkError<UpdateGatewayRouteError>>> {
        self.deref().update_gateway_route(builder)
    }
    fn update_mesh(&self, builder: UpdateMeshInputBuilder) -> impl Future<Output = Result<UpdateMeshOutput, SdkError<UpdateMeshError>>> {
        self.deref().update_mesh(builder)
    }
    fn update_route(&self, builder: UpdateRouteInputBuilder) -> impl Future<Output = Result<UpdateRouteOutput, SdkError<UpdateRouteError>>> {
        self.deref().update_route(builder)
    }
    fn update_virtual_gateway(&self, builder: UpdateVirtualGatewayInputBuilder) -> impl Future<Output = Result<UpdateVirtualGatewayOutput, SdkError<UpdateVirtualGatewayError>>> {
        self.deref().update_virtual_gateway(builder)
    }
    fn update_virtual_node(&self, builder: UpdateVirtualNodeInputBuilder) -> impl Future<Output = Result<UpdateVirtualNodeOutput, SdkError<UpdateVirtualNodeError>>> {
        self.deref().update_virtual_node(builder)
    }
    fn update_virtual_router(&self, builder: UpdateVirtualRouterInputBuilder) -> impl Future<Output = Result<UpdateVirtualRouterOutput, SdkError<UpdateVirtualRouterError>>> {
        self.deref().update_virtual_router(builder)
    }
    fn update_virtual_service(&self, builder: UpdateVirtualServiceInputBuilder) -> impl Future<Output = Result<UpdateVirtualServiceOutput, SdkError<UpdateVirtualServiceError>>> {
        self.deref().update_virtual_service(builder)
    }
}
#[cfg(feature = "mockall")]
mockall::mock! {
    pub edAppMeshClient {}
    impl AppMeshClient for edAppMeshClient {
        async fn create_gateway_route(&self, builder: CreateGatewayRouteInputBuilder) -> Result<CreateGatewayRouteOutput, SdkError<CreateGatewayRouteError>>;
        async fn create_mesh(&self, builder: CreateMeshInputBuilder) -> Result<CreateMeshOutput, SdkError<CreateMeshError>>;
        async fn create_route(&self, builder: CreateRouteInputBuilder) -> Result<CreateRouteOutput, SdkError<CreateRouteError>>;
        async fn create_virtual_gateway(&self, builder: CreateVirtualGatewayInputBuilder) -> Result<CreateVirtualGatewayOutput, SdkError<CreateVirtualGatewayError>>;
        async fn create_virtual_node(&self, builder: CreateVirtualNodeInputBuilder) -> Result<CreateVirtualNodeOutput, SdkError<CreateVirtualNodeError>>;
        async fn create_virtual_router(&self, builder: CreateVirtualRouterInputBuilder) -> Result<CreateVirtualRouterOutput, SdkError<CreateVirtualRouterError>>;
        async fn create_virtual_service(&self, builder: CreateVirtualServiceInputBuilder) -> Result<CreateVirtualServiceOutput, SdkError<CreateVirtualServiceError>>;
        async fn delete_gateway_route(&self, builder: DeleteGatewayRouteInputBuilder) -> Result<DeleteGatewayRouteOutput, SdkError<DeleteGatewayRouteError>>;
        async fn delete_mesh(&self, builder: DeleteMeshInputBuilder) -> Result<DeleteMeshOutput, SdkError<DeleteMeshError>>;
        async fn delete_route(&self, builder: DeleteRouteInputBuilder) -> Result<DeleteRouteOutput, SdkError<DeleteRouteError>>;
        async fn delete_virtual_gateway(&self, builder: DeleteVirtualGatewayInputBuilder) -> Result<DeleteVirtualGatewayOutput, SdkError<DeleteVirtualGatewayError>>;
        async fn delete_virtual_node(&self, builder: DeleteVirtualNodeInputBuilder) -> Result<DeleteVirtualNodeOutput, SdkError<DeleteVirtualNodeError>>;
        async fn delete_virtual_router(&self, builder: DeleteVirtualRouterInputBuilder) -> Result<DeleteVirtualRouterOutput, SdkError<DeleteVirtualRouterError>>;
        async fn delete_virtual_service(&self, builder: DeleteVirtualServiceInputBuilder) -> Result<DeleteVirtualServiceOutput, SdkError<DeleteVirtualServiceError>>;
        async fn describe_gateway_route(&self, builder: DescribeGatewayRouteInputBuilder) -> Result<DescribeGatewayRouteOutput, SdkError<DescribeGatewayRouteError>>;
        async fn describe_mesh(&self, builder: DescribeMeshInputBuilder) -> Result<DescribeMeshOutput, SdkError<DescribeMeshError>>;
        async fn describe_route(&self, builder: DescribeRouteInputBuilder) -> Result<DescribeRouteOutput, SdkError<DescribeRouteError>>;
        async fn describe_virtual_gateway(&self, builder: DescribeVirtualGatewayInputBuilder) -> Result<DescribeVirtualGatewayOutput, SdkError<DescribeVirtualGatewayError>>;
        async fn describe_virtual_node(&self, builder: DescribeVirtualNodeInputBuilder) -> Result<DescribeVirtualNodeOutput, SdkError<DescribeVirtualNodeError>>;
        async fn describe_virtual_router(&self, builder: DescribeVirtualRouterInputBuilder) -> Result<DescribeVirtualRouterOutput, SdkError<DescribeVirtualRouterError>>;
        async fn describe_virtual_service(&self, builder: DescribeVirtualServiceInputBuilder) -> Result<DescribeVirtualServiceOutput, SdkError<DescribeVirtualServiceError>>;
        async fn list_gateway_routes(&self, builder: ListGatewayRoutesInputBuilder) -> Result<ListGatewayRoutesOutput, SdkError<ListGatewayRoutesError>>;
        async fn list_meshes(&self, builder: ListMeshesInputBuilder) -> Result<ListMeshesOutput, SdkError<ListMeshesError>>;
        async fn list_routes(&self, builder: ListRoutesInputBuilder) -> Result<ListRoutesOutput, SdkError<ListRoutesError>>;
        async fn list_tags_for_resource(&self, builder: ListTagsForResourceInputBuilder) -> Result<ListTagsForResourceOutput, SdkError<ListTagsForResourceError>>;
        async fn list_virtual_gateways(&self, builder: ListVirtualGatewaysInputBuilder) -> Result<ListVirtualGatewaysOutput, SdkError<ListVirtualGatewaysError>>;
        async fn list_virtual_nodes(&self, builder: ListVirtualNodesInputBuilder) -> Result<ListVirtualNodesOutput, SdkError<ListVirtualNodesError>>;
        async fn list_virtual_routers(&self, builder: ListVirtualRoutersInputBuilder) -> Result<ListVirtualRoutersOutput, SdkError<ListVirtualRoutersError>>;
        async fn list_virtual_services(&self, builder: ListVirtualServicesInputBuilder) -> Result<ListVirtualServicesOutput, SdkError<ListVirtualServicesError>>;
        async fn tag_resource(&self, builder: TagResourceInputBuilder) -> Result<TagResourceOutput, SdkError<TagResourceError>>;
        async fn untag_resource(&self, builder: UntagResourceInputBuilder) -> Result<UntagResourceOutput, SdkError<UntagResourceError>>;
        async fn update_gateway_route(&self, builder: UpdateGatewayRouteInputBuilder) -> Result<UpdateGatewayRouteOutput, SdkError<UpdateGatewayRouteError>>;
        async fn update_mesh(&self, builder: UpdateMeshInputBuilder) -> Result<UpdateMeshOutput, SdkError<UpdateMeshError>>;
        async fn update_route(&self, builder: UpdateRouteInputBuilder) -> Result<UpdateRouteOutput, SdkError<UpdateRouteError>>;
        async fn update_virtual_gateway(&self, builder: UpdateVirtualGatewayInputBuilder) -> Result<UpdateVirtualGatewayOutput, SdkError<UpdateVirtualGatewayError>>;
        async fn update_virtual_node(&self, builder: UpdateVirtualNodeInputBuilder) -> Result<UpdateVirtualNodeOutput, SdkError<UpdateVirtualNodeError>>;
        async fn update_virtual_router(&self, builder: UpdateVirtualRouterInputBuilder) -> Result<UpdateVirtualRouterOutput, SdkError<UpdateVirtualRouterError>>;
        async fn update_virtual_service(&self, builder: UpdateVirtualServiceInputBuilder) -> Result<UpdateVirtualServiceOutput, SdkError<UpdateVirtualServiceError>>;
    }
}
