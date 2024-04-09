use crate::adsb_data::Root;
use async_std::task;
use sqlx::{Error, MySql, MySqlPool, Pool, Row};

pub async fn insert_adsb(adsb: &Root) {
    let result = task::block_on(connect());
    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }
        Ok(pool) => {
            let archive_result = sqlx::query("CALL insert_archive(?, ?)")
                .bind(&adsb.now)
                .bind(&adsb.messages)
                .fetch_one(&pool)
                .await
                .unwrap();

            let archive_id: u64 = archive_result.get(0);

            for aircraft in adsb.aircraft.iter() {
                let mut barometric_altitude = 0;
                let cloned_aircraft = aircraft.clone().barometric_altitude;
                match cloned_aircraft {
                    None => {}
                    Some(aircraft) => {
                        if aircraft.is_i64() {
                            barometric_altitude = aircraft.as_i64().unwrap();
                        }
                    }
                }

                let aircraft_result = sqlx::query("CALL insert_aircraft(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);")
                    .bind(archive_id)
                    .bind(&aircraft.hex)
                    .bind(&aircraft.type_field)
                    .bind(&aircraft.flight)
                    .bind(&aircraft.registration)
                    .bind(&aircraft.aircraft_type)
                    .bind(barometric_altitude)
                    .bind(&aircraft.geometric_altitude)
                    .bind(&aircraft.ground_speed)
                    .bind(&aircraft.track)
                    .bind(&aircraft.geom_rate)
                    .bind(&aircraft.category)
                    .bind(&aircraft.nav_qnh)
                    .bind(&aircraft.nav_altitude_mcp)
                    .bind(&aircraft.nav_heading)
                    .bind(&aircraft.latitude)
                    .bind(&aircraft.longitude)
                    .bind(&aircraft.nic)
                    .bind(&aircraft.rc)
                    .bind(&aircraft.seen_pos)
                    .bind(&aircraft.version)
                    .bind(&aircraft.nic_baro)
                    .bind(&aircraft.nac_p)
                    .bind(&aircraft.nac_v)
                    .bind(&aircraft.sil)
                    .bind(&aircraft.sil_type)
                    .bind(&aircraft.gva)
                    .bind(&aircraft.sda)
                    .bind(&aircraft.alert)
                    .bind(&aircraft.spi)
                    .bind(&aircraft.messages)
                    .bind(&aircraft.seen)
                    .bind(&aircraft.rssi)
                    .execute(&pool).await;

                match aircraft_result {
                    Ok(_) => {}
                    Err(err_info) => {
                        println!("{:?}", err_info);
                    }
                }
            }
        }
    }
}

async fn connect() -> Result<Pool<MySql>, Error> {
    return MySqlPool::connect("mysql://client:skypark@192.168.232.140:3306/adsb").await;
}
