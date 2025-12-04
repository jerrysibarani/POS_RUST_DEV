use utoipa::OpenApi;

use crate::handlers::unit_handler::{
    get_all_units,
    get_unit_by_id,
    create_unit,
    update_unit,
    delete_unit,
};

use crate::models::unit::{Unit, CreateUnitRequest, UpdateUnitRequest};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_units,
        get_unit_by_id,
        create_unit,
        update_unit,
        delete_unit
    ),
    components(
        schemas(Unit, CreateUnitRequest, UpdateUnitRequest)
    ),
    tags(
        (name = "Unit", description = "Unit CRUD API")
    )
)]
pub struct ApiDoc;
