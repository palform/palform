use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, to},
    App, HttpServer,
};
use apistos::app::OpenApiWrapper;
use regex::Regex;
use sea_orm::DatabaseConnection;

use crate::{
    actix_util::not_found::not_found_handler,
    captcha::requests::CAPTCHA_HEADER,
    config::Config,
    geo::IPGeolocator,
    mail::client::PalformMailClient,
    memory_db::memory_db::MemoryDB,
    openapi::get_openapi_spec,
    palform_s3::{
        buckets::{S3BucketSubmissionAssets, S3BucketTeamAssets},
        client::PalformS3Client,
    },
    routes::{legacy_compat_routes, main_routes},
};

pub async fn run_api_server(
    config: &Config,
    db: &DatabaseConnection,
    stripe: &stripe::Client,
) -> std::io::Result<()> {
    let bind_addr = config.bind_addr.clone();
    let config = config.clone();
    let db = db.clone();

    let mail = PalformMailClient::new(&config).await;
    let s3_submission_assets = PalformS3Client::<S3BucketSubmissionAssets>::init(&config)
        .expect("init submission assets s3");
    let s3_team_assets =
        PalformS3Client::<S3BucketTeamAssets>::init(&config).expect("init team assets s3");
    let geo = IPGeolocator::new().await.expect("geolocator init");
    let captcha_header_name = CAPTCHA_HEADER.clone();
    let stripe = stripe.clone();
    let memory_db = MemoryDB::new(&config).await;

    HttpServer::new(move || {
        let cors_regex = Regex::new(&config.cors_origin).expect("parse cors regex");
        let cors = Cors::default()
            .allowed_origin_fn(move |header_value, _| {
                cors_regex.is_match(header_value.to_str().expect("string from origin header"))
            })
            .allow_any_method()
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                captcha_header_name.clone(),
            ])
            .supports_credentials()
            .max_age(600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(MultipartFormConfig::default().total_limit(config.file_upload_size_limit))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(mail.clone()))
            .app_data(web::Data::new(s3_submission_assets.clone()))
            .app_data(web::Data::new(s3_team_assets.clone()))
            .app_data(web::Data::new(stripe.clone()))
            .app_data(web::Data::new(geo.clone()))
            .app_data(web::Data::new(memory_db.clone()))
            .document(get_openapi_spec())
            .service(main_routes())
            .service(legacy_compat_routes())
            .build("/openapi.json")
            .default_service(to(not_found_handler))
    })
    .bind(bind_addr)?
    .run()
    .await
}
