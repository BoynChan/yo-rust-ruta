use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use tonic::{transport::Server, Code, Request, Response, Status};

use greeter_server::{Greeter, GreeterServer};
use sea_orm::ColumnTrait;

use grpc_longHu::name_table;
use grpc_longHu::name_table::Entity as Name;

use grpc_longHu::DB;

tonic::include_proto!("helloworld");

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = HelloReply {
            message: format!("Hello {}", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn insert_name(
        &self,
        request: Request<InsertNameRequest>,
    ) -> Result<Response<InsertNameResponse>, Status> {
        let new_name = name_table::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: sea_orm::ActiveValue::Set(request.get_ref().name.clone()),
        };
        if let Err(r) = new_name.insert(DB.get().await).await {
            return Ok(Response::new(InsertNameResponse { success: false }));
        }
        Ok(Response::new(InsertNameResponse { success: true }))
    }

    async fn get_name(
        &self,
        request: Request<GetNameRequest>,
    ) -> Result<Response<GetNameResponse>, Status> {
        let result = Name::find()
            .filter(name_table::Column::Name.eq(request.get_ref().name.as_str()))
            .one(DB.get().await)
            .await;
        if result.is_err() {
            return Err(Status::new(Code::Unavailable, "db error"));
        }
        let find_name = result.unwrap();
        if find_name.is_none() {
            return Err(Status::new(Code::NotFound, "name not found"));
        }
        let one = find_name.unwrap();
        Ok(Response::new(GetNameResponse {
            id: one.id,
            name: one.name,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
