// @generated automatically by Diesel CLI.

diesel::table! {
    event_planet (event_id, planet_id) {
        event_id -> Int4,
        planet_id -> Int4,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        reason -> Nullable<Varchar>,
        damage -> Nullable<Int4>,
    }
}

diesel::table! {
    hunters (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        power -> Int4,
        money -> Int4,
        ship_id -> Int4,
    }
}

diesel::table! {
    inquisitors (id) {
        id -> Int4,
        name -> Varchar,
        power -> Int4,
        money -> Int4,
    }
}

diesel::table! {
    inquisitors_squads (inquisitor_id, squad_id) {
        inquisitor_id -> Int4,
        squad_id -> Int4,
    }
}

diesel::table! {
    jedi (id) {
        id -> Int4,
        name -> Varchar,
        power -> Int4,
    }
}

diesel::table! {
    jedi_data (id) {
        id -> Int4,
        jedi_id -> Int4,
        planet_id -> Int4,
        normal_id -> Int4,
        team_size -> Nullable<Int4>,
        ship_power -> Nullable<Int4>,
        price -> Nullable<Int4>,
    }
}

diesel::table! {
    jedi_requests (id) {
        id -> Int4,
        price -> Int4,
        jedi_id -> Int4,
    }
}

diesel::table! {
    mission (id) {
        id -> Int4,
        complexity -> Int4,
        jedi_id -> Int4,
    }
}

diesel::table! {
    normals (id) {
        id -> Int4,
        name -> Varchar,
        money -> Int4,
    }
}

diesel::table! {
    planets (id) {
        id -> Int4,
        name -> Varchar,
        color -> Varchar,
        size -> Int4,
        race -> Nullable<Varchar>,
    }
}

diesel::table! {
    ships (id) {
        id -> Int4,
        model -> Varchar,
        color -> Varchar,
        fuel_type -> Nullable<Varchar>,
        power -> Int4,
    }
}

diesel::table! {
    squads (id) {
        id -> Int4,
        name -> Varchar,
        ship_id -> Int4,
    }
}

diesel::table! {
    jedi_data_inquisitors (jedi_data_id, inquisitor_id) {
        jedi_data_id -> Int4,
        inquisitor_id -> Int4,
    }
}

diesel::joinable!(event_planet -> events (event_id));

diesel::joinable!(inquisitors_squads -> inquisitors (inquisitor_id));
diesel::joinable!(inquisitors_squads -> squads (squad_id));

diesel::joinable!(jedi_data_inquisitors -> jedi_data (jedi_data_id));
diesel::joinable!(jedi_data_inquisitors -> inquisitors (inquisitor_id));

diesel::joinable!(jedi_data -> normals (normal_id));
diesel::joinable!(jedi_data -> jedi (jedi_id));
diesel::joinable!(jedi_data -> planets (planet_id));

diesel::joinable!(jedi_requests -> jedi (jedi_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_planet,
    events,
    hunters,
    inquisitors,
    inquisitors_squads,
    jedi,
    jedi_data,
    jedi_requests,
    mission,
    normals,
    planets,
    ships,
    squads,
    jedi_data_inquisitors,
);
