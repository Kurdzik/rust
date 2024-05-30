use super::schema::otomoto_data;
   use diesel::prelude::*;
   use diesel::sql_types::*;
   
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::otomoto_data)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OtomotoData {
    pub price: Nullable<i32>,
    pub vehicle_brand: Option<String>,
    pub vehicle_model: Option<String>,
    pub vehicle_version: Option<String>,
    pub vehicle_generation: Option<String>,
    pub year_of_production: Option<i64>,
    pub mileage: Option<i32>,
    pub engine_capacity: Option<i64>,
    pub fuel_type: Option<String>,
    pub horse_power: Option<i32>,
    pub transmission_type: Option<String>,
    pub drive_type: Option<String>,
    pub gas_usage_per_100km: Option<String>,
    pub car_body_type: Option<String>,
    pub number_of_doors: Option<i32>,
    pub number_of_seats: Option<i32>,
    pub color: Option<String>, 
    pub country_of_origin: Option<String>,
    pub damaged: Option<bool>,
}