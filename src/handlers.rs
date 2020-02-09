use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::{thread, time};

#[cfg(test)]
use fake_clock::FakeClock as Instant;

#[derive(Deserialize, Serialize)]
pub struct Event {
    id: Option<i32>,
    //    timestamp: f64,
    //    kind: String,
    //    tags: Vec<String>,
}

pub async fn event(evt: web::Json<Event>) -> impl Responder {
    //    let new_event = store_in_db(evt.timestamp, &evt.kind, &evt.tags);
    // Event { id: Some(evt.id.unwrap_or(0)) };
    return web::Json(Event {
        id: Some(evt.id.unwrap_or(0)),
    });
}

pub async fn index(req: HttpRequest) -> impl Responder {
    if req.headers().get(header::CONTENT_TYPE) != None {
        HttpResponse::Ok()
    } else {
        #[cfg(not(test))]
        {
            use std::io::Write;
            writeln!(
                std::io::stderr(),
                "CONTENT_TYPE is not defined in the header."
            )
            .unwrap();
        }
        HttpResponse::BadRequest()
    }
}

pub async fn index2(req: HttpRequest) -> HttpResponse {
    if req.headers().get(header::CONTENT_TYPE) != None {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

pub async fn index3(req: HttpRequest) -> HttpResponse {
    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);
    if req.headers().get(header::CONTENT_TYPE) != None {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test;
    use std::time::Duration;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_index2_ok() {
        let req =
            test::TestRequest::with_header("content-type", "application/json").to_http_request();
        let resp = index2(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[async_test]
    async fn test_index2_err() {
        let req =
            test::TestRequest::default().to_http_request();
        let resp = index2(req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_duration() {
        let d = Duration::new(2, 0);
        let now = Instant::now();
        let new_now = Instant::now() + d;
        println!("{:?}", now.elapsed());
        println!("{:?} {:?}", new_now, now);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};
    use futures_await_test::async_test;

    #[async_test]
    async fn test_index_ok() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("Content-Type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[async_test]
    async fn test_index_err() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[async_test]
    async fn test_event_ok() {
        let check = Some(67);
        let mut app = test::init_service(App::new().route("/", web::post().to(event))).await;
        let req = test::TestRequest::post()
            .set_json(&Event { id: check })
            .uri("/")
            .to_request();
        let resp: Event = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.id, check);
    }
}
