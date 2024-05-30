table! {
    use diesel::sql_types::*;
    use super::OtomotoData;

    otomoto_data (index) {
        index -> Int4,
        price -> Nullable<Int4>,
        vehicle_brand -> Nullable<Varchar>,
        vehicle_model -> Nullable<Varchar>,
        vehicle_version -> Nullable<Varchar>,
        vehicle_generation -> Nullable<Varchar>,
        year_of_production -> Nullable<Int8>,
        mileage -> Nullable<Int4>,
        engine_capacity -> Nullable<Int8>,
        fuel_type -> Nullable<Varchar>,
        horse_power -> Nullable<Int4>,
        transmission_type -> Nullable<Varchar>,
        drive_type -> Nullable<Varchar>,
        gas_usage_per_100km -> Nullable<Varchar>,
        car_body_type -> Nullable<Varchar>,
        number_of_doors -> Nullable<Int4>,
        number_of_seats -> Nullable<Int4>,
        color -> Nullable<Varchar>,
        country_of_origin -> Nullable<Varchar>,
        damaged -> Nullable<Bool>,
    }
}