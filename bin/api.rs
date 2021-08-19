use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use serde::Serialize;

use oshkosh_kiwanis_web_crawler::{ContestGoal, NewTopDog};

#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    response_type: String,
    data: T,
}

/// Define HTTP actor
struct MyWs;

type Context = ws::WebsocketContext<MyWs>;

impl Actor for MyWs {
    type Context = Context;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => match text.as_str() {
                "dogs" => ctx.text(
                    self.get_dogs()
                ),
                "contests" => ctx.text(
                    self.get_contests()
                ),
                _ => ()
            },
            Ok(ws::Message::Close(..)) => {
                ctx.close(None);
            }
            _ => (),
        }
    }
}

impl MyWs {
    fn get_contests(&self) -> String {
        let serialized = std::fs::read_to_string("contest-goals.json").unwrap_or("".into());

        // wrap this in an event response
        let data = serde_json::from_str::<Vec<ContestGoal>>(&serialized).unwrap();

        let resp = ApiResponse {
            response_type: "contest-goals".into(),
            data,
        };

        serde_json::to_string(&resp).unwrap()
    }

    fn get_dogs(&self) -> String {
        let serialized = std::fs::read_to_string("top-dogs.json").unwrap_or("".into());

        // wrap this in an event response
        let data = serde_json::from_str::<Vec<NewTopDog>>(&serialized).unwrap();

        let resp = ApiResponse {
            response_type: "top-dogs".into(),
            data,
        };

        serde_json::to_string(&resp).unwrap()
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("0.0.0.0:80")?
        .run()
        .await
}