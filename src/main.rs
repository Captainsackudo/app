use warp::Filter;
mod translator;
mod ocr;

#[tokio::main]
async fn main() {
    let upload = warp::path("translate")
        .and(warp::post())
        .and(warp::multipart::form().max_length(10_000_000))
        .and_then(translator::handle_image_translation);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST"]);

    let routes = upload.with(cors);

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}