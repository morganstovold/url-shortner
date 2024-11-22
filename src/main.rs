use actix_web::{get, post, web::{self, Data, Redirect, Json}, HttpRequest, Error};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
use sqlx::Row;

#[derive(Deserialize)]
struct CreateUrl {
    url: String,
}

#[derive(Serialize)]
struct UrlResponse {
    short_url: String,
    long_url: String,
}

#[post("/shorten")]
async fn create_short_url(
    req: HttpRequest,
    payload: Json<CreateUrl>,
    pool: Data<PgPool>,
) -> Result<Json<UrlResponse>, Error> {
    let long_url = payload.url.clone();
    let short_url = nanoid::nanoid!(6); // Generate a 6-character unique ID

    // Insert the URL into the database
    sqlx::query(
        "INSERT INTO urls (short_url, long_url) VALUES ($1, $2)"
    )
    .bind(&short_url)
    .bind(&long_url)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Construct the full URL
    let domain = req.headers().get("host").unwrap().to_str().unwrap();
    let full_short_url = format!("http://{}/{}", domain, short_url);

    Ok(Json(UrlResponse {
        short_url: full_short_url,
        long_url,
    }))
}

#[get("/{short_url}")]
async fn redirect(
    path: web::Path<String>,
    pool: Data<PgPool>,
) -> Result<Redirect, Error> {
    let short_url = path.into_inner();
    
    // Query the database to find the long URL
    let record = sqlx::query(
        "SELECT long_url FROM urls WHERE short_url = $1"
    )
    .bind(&short_url)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    match record {
        Some(record) => {
            let long_url: String = record.get("long_url");
            Ok(Redirect::to(long_url).permanent())
        },
        None => Err(actix_web::error::ErrorNotFound("Short URL not found")),
    }
}

#[get("/all")]
async fn get_all_links(req: HttpRequest, pool: Data<PgPool>) -> Result<Json<Vec<UrlResponse>>, Error> {
    // Query the database to get all URLs
    let records = sqlx::query("SELECT short_url, long_url FROM urls")
        .fetch_all(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Construct the domain from the request
    let domain = req.headers().get("host").unwrap().to_str().unwrap();
    
    // Map the records to UrlResponse with full short URLs
    let urls: Vec<UrlResponse> = records
        .iter()
        .map(|record| {
            let short_url: String = record.get("short_url");
            UrlResponse {
                short_url: format!("http://{}/{}", domain, short_url),
                long_url: record.get("long_url"),
            }
        })
        .collect();

    Ok(Json(urls))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Run migrations if needed
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let pool = Data::new(pool);
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(pool)
            .service(create_short_url)
            .service(get_all_links)
            .service(redirect);
    };

    Ok(config.into())
}
