use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;


    Ok(())
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

const HTML_ERROR: &str = r#"<div> Not permitted </div>"#;

const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <script src="https://unpkg.com/htmx.org@2.0.0" integrity="sha384-wS5l5IKJBvK6sPTKa2WZ1js3d947pvWXbPJ1OmWfEuxLgeHcEbjUUA5i9V5ZkpCw" crossorigin="anonymous"></script>
        <script src="https://unpkg.com/htmx-ext-response-targets@2.0.0/response-targets.js"></script>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body hx-ext="response-targets">

<div>
    <div id="response-div"></div>
    <button hx-get="/error"
            hx-target='#response-div'
            hx-target-5*='#serious-errors'
            hx-target-4*='this'
            hx-swap="outerHTML"
            >
        Register!
    </button>
</div>
    
</body>
</html>
"#;

