use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::inventory_locations::inventory_location_service::inventory_location_service::{
    create_inventory_location, delete_inventory_location, get_inventory_location_by_id,
    get_inventory_locations, update_inventory_location,
};
use crate::api_module::inventory_movements::inventory_movements_service::inventory_movements_service::{
    create_inventory_movement, delete_inventory_movement, get_inventory_movement_by_id,
    get_inventory_movements, update_inventory_movement,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const INVENTORY_MOVEMENT: &str = route!("/inventory_movement");
const INVENTORY_MOVEMENT_BY_ID: &str = route!("/inventory_movement/{:id}");
const INVENTORY_MOVEMENTS_LIST: &str = route!("/inventory_movement");
const INVENTORY_MOVEMENT_DELETE: &str = route!("/inventory_movement/{:id}");
const INVENTORY_MOVEMENT_UPDATE: &str = route!("/inventory_movement/{:id}");

const INVENTORY_LOCATIONS: &str = route!("/inventory_locations");
const INVENTORY_LOCATIONS_BY_ID: &str = route!("/inventory_locations/{:id}");
const INVENTORY_LOCATIONS_DELETE: &str = route!("/inventory_locations/{:id}");
const INVENTORY_LOCATIONS_UPDATE: &str = route!("/inventory_locations/{:id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Inventory Movement routes
        .route(INVENTORY_MOVEMENT, post(create_inventory_movement))
        .route(INVENTORY_MOVEMENT_BY_ID, get(get_inventory_movement_by_id))
        .route(INVENTORY_MOVEMENTS_LIST, get(get_inventory_movements))
        .route(INVENTORY_MOVEMENT_DELETE, delete(delete_inventory_movement))
        .route(INVENTORY_MOVEMENT_UPDATE, patch(update_inventory_movement))
        // Inventory Locations routes
        .route(INVENTORY_LOCATIONS, post(create_inventory_location))
        .route(INVENTORY_LOCATIONS_BY_ID, get(get_inventory_location_by_id))
        .route(INVENTORY_LOCATIONS, get(get_inventory_locations))
        .route(INVENTORY_LOCATIONS_DELETE, delete(delete_inventory_location))
        .route(INVENTORY_LOCATIONS_UPDATE, patch(update_inventory_location))
}
