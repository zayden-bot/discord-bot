// @generated automatically by Diesel CLI.

diesel::table! {
    good_morning_images (id) {
        id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::table! {
    good_night_images (id) {
        id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    good_morning_images,
    good_night_images,
);
