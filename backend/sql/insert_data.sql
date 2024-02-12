INSERT INTO weather.data(temperature_c, city_id)
VALUES ($1, $2)
RETURNING $table_fields;
