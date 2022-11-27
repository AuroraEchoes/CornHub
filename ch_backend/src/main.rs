use std::{io::Result, time::Instant, sync::RwLock};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Path};

static mut SHARDS: Vec<Farm> = Vec::new();

#[get("/farm/{farm_id}")]
async fn root(path: Path<u64>) -> impl Responder {
    let id = path.into_inner();
    unsafe {
        let farm = SHARDS.get(0).unwrap();
        if id == farm.owner {
            return HttpResponse::Ok().body(format!("{:?}", SHARDS.get(0).unwrap()));
        } 
        else {
            return HttpResponse::BadRequest().body("Farm not found");
        }
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    unsafe {
        SHARDS.push(Farm {
            owner: 1,
            name: String::from("Aurora's farm"),
            last_interaction: Instant::now()
        });
    }

    HttpServer::new(|| {
        App::new()
            .service(root)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;



    Ok(())
}

struct BackendShard {
    cached_farms: Box<Vec<Farm>>
}

#[derive(Debug)]
struct Farm {
    owner: u64,
    name: String,
    last_interaction: Instant
}