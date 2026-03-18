use axum::{
    Json,
    extract::{Path, State},
};
use sea_orm::{DbBackend, FromQueryResult, Statement};

use crate::{
    api_handlers::report::report_dto::ReportListUserActive,
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

/* Get Report de Lista de Usarios  */
pub async fn get_report_list_user_active(
    State(app_ctx): State<AppContext>,
    Path(tipo_user): Path<String>,
) -> Result<Json<ApiResponse<Vec<ReportListUserActive>>>, ApiError> {
    let report: Vec<ReportListUserActive> =
        ReportListUserActive::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "user"."id" , "user"."username", "user"."role" FROM "user"  WHERE "user"."role" = $1 "#,
            [
                    tipo_user.into(),
            ],
        ))
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    //SI esta vacio activar ApiError Data no encontrado

    match report.is_empty() {
        true => {
            return Err(ApiError::NotFound);
        }
        false => (),
    }

    let api_response = ApiResponse::new(
        report,
        1,
        "Report generated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    );

    Ok(Json(api_response))
}
