mod db;
mod errors;
mod models;

use std::{env, str::FromStr, thread, time};
use std::io::prelude::*;

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use chrono::{Local, Utc};
use cron::Schedule;
use deadpool_postgres::{Client, ManagerConfig, Pool, RecyclingMethod};
use reqwest;
use tokio_postgres::NoTls;

use db::{get_latest_data, get_locations, insert_temperature};
use models::WeatherResult;

fn get_envvar(var_name: &String) -> String {
    match env::var(var_name) {
        Ok(v) => v,
        Err(_) => panic!("Unable to get {}", var_name),
    }
}

struct AppSettings {
    pub openweather_api_key: String,
    pub pool: Pool,
}

fn get_deadpool_config() -> deadpool_postgres::Config {
    let hostname: String = get_envvar(&"POSTGRES_HOSTNAME".to_string());
    let db: String = get_envvar(&"POSTGRES_DB".to_string());
    let user: String = get_envvar(&"POSTGRES_USER".to_string());
    let pwd: String = get_envvar(&"POSTGRES_PASSWORD".to_string());

    let mut cfg = deadpool_postgres::Config::new();
    cfg.host = Some(hostname);
    cfg.port = Some(5432); // TODO: get it from an envvar
    cfg.dbname = Some(db);
    cfg.user = Some(user);
    cfg.password = Some(pwd);
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}

async fn update_weather_data(client: &Client, api_key: String) -> Result<bool, Error> {
    let sleep_time = time::Duration::from_secs(1);

    let locations = get_locations(&client).await?;
    for location in locations.iter() {
        println!("[/update] Getting data for {}", location.city_name);

        let res = reqwest::get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            location.lat, location.long, api_key
        ))
        .await;

        if res.is_err() {
            println!("[/update] FAILED request for {}", location.city_name);
            continue;
        }

        let body = res.unwrap().json::<WeatherResult>().await.unwrap();

        thread::sleep(sleep_time);

        insert_temperature(&client, location.city_id, body.main.temp).await?;
    }

    Ok(true)
}

#[get("/latest")]
async fn latest(app_data: web::Data<AppSettings>) -> Result<HttpResponse, Error> {
    println!("[/latest] Getting the latest data");
    let pool = &app_data.pool.clone();
    let client = pool.get().await.unwrap();
    let latest_data = get_latest_data(&client).await?;

    Ok(HttpResponse::Ok().json(latest_data))
}

#[get("/")]
async fn hello(app_data: web::Data<AppSettings>) -> Result<HttpResponse, Error> {
    let api_key = &app_data.openweather_api_key;
    let body = reqwest::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        "52.48", "-1.89", api_key
    ))
    .await
    .unwrap()
    .json::<WeatherResult>()
    .await
    .unwrap();

    Ok(HttpResponse::Ok().json(body))
}

#[get("/update")]
async fn update(app_data: web::Data<AppSettings>) -> Result<HttpResponse, Error> {
    // TODO: find a way to send and forget
    println!("[/update] Updating the database...");
    let pool = &app_data.pool.clone();
    let api_key = &app_data.openweather_api_key;
    let client = pool.get().await.unwrap();

    // TODO make it an env var
    let sleep_time = time::Duration::from_secs(1);

    let locations = get_locations(&client).await?;
    for location in locations.iter() {
        println!("[/update] Getting data for {}", location.city_name);

        let res = reqwest::get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            location.lat, location.long, api_key
        ))
        .await;

        if res.is_err() {
            println!("[/update] FAILED request for {}", location.city_name);
            continue;
        }

        let body = res.unwrap().json::<WeatherResult>().await.unwrap();

        thread::sleep(sleep_time);

        let _new_temp = insert_temperature(&client, location.city_id, body.main.temp).await?;
    }

    println!("[/update] Update finished!");
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let expression = "1 * * * * *";
    // let schedule = Schedule::from_str(expression).unwrap();

    // loop {
    //     let local = &Local::now();
    //     let mut upcoming = schedule.upcoming(local.timezone()).take(1);
    //     actix_rt::time::sleep(time::Duration::from_millis(1000)).await;

    //     if let Some(datetime) = upcoming.next() {
    //         println!("{} v {}", datetime.to_rfc3339(), local.to_rfc3339());
    //         if datetime.timestamp() <= Local::now().timestamp() {
    //             println!("corn ran");
    //         }
    //     }
    // }

    // Build pool
    let cfg = get_deadpool_config();
    let pool = cfg.create_pool(None, NoTls).unwrap();
    let openweather_api_key = get_envvar(&"OPENWEATHER_API_KEY".to_string());

    let cron_pool = pool.clone();

    actix_rt::spawn(async move {
        let client = cron_pool.get().await.unwrap();
        // TODO: make this an env var
        //                sec  min  hour  day  month  wkday  year
        let expression = "0    0    */2   *    *      *      *";
        let schedule = Schedule::from_str(expression).unwrap();

        let cron_api_key = get_envvar(&"OPENWEATHER_API_KEY".to_string());

        let mut upcoming = schedule.upcoming(Local::now().timezone()).take(1);
        let mut next = upcoming.next().unwrap();
        println!("Next cron is at: {}", next.to_rfc3339());

        loop {
            let api_key = cron_api_key.clone();
            if next.timestamp() <= Local::now().timestamp() {
                upcoming = schedule.upcoming(Local::now().timezone()).take(1);
                next = upcoming.next().unwrap();
                println!("Next cron is at: {}", next.to_rfc3339());

                let update_result = update_weather_data(&client, api_key).await.unwrap();
                if !update_result {
                    panic!("Failed to update the databse");
                }
            }
        }
    });

    let app_settings = AppSettings {
        openweather_api_key,
        pool,
    };
    let app_data = web::Data::new(app_settings);

    println!("Starting server...");
    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            // TODO: REMOVE B4 DEPLOYING
            .wrap(cors)
            .app_data(app_data.clone())
            .service(hello)
            .service(update)
            .service(latest)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
