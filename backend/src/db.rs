use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::DbError;
use crate::models::{Data, Location, LatestWeatherData};

pub async fn get_locations(client: &Client) -> Result<Vec<Location>, DbError> {
    let _stmt = include_str!("../sql/get_locations.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Location::from_row_ref(row).unwrap())
        .collect::<Vec<Location>>();

    if result.is_empty() {
        return Err(DbError::NotFound);
    }
    Ok(result)
}

pub async fn insert_temperature(
    client: &Client,
    city_id: i32,
    temp_c: f32,
) -> Result<Data, DbError> {
    let _stmt = include_str!("../sql/insert_data.sql");
    let _stmt = _stmt.replace("$table_fields", &Data::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&temp_c, &city_id])
        .await?
        .iter()
        .map(|row| Data::from_row_ref(row).unwrap())
        .collect::<Vec<Data>>()
        .pop()
        .ok_or(DbError::InternalError)
}

pub async fn get_latest_data(
    client: &Client,
) -> Result<Vec<LatestWeatherData>, DbError> {
    let _stmt = include_str!("../sql/select_latest.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| LatestWeatherData::from_row_ref(row).unwrap())
        .collect::<Vec<LatestWeatherData>>();

    if result.is_empty() {
        return Err(DbError::NotFound);
    }
    Ok(result)

}
