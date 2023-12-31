use crate::structs::*;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::MySqlPool;

pub async fn items(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
  let query = "Select item_id, item_type, item_name, item_price from items".to_string();
  let items: Vec<Items> = match sqlx::query_as(&query).fetch_all(&pool).await {
    Ok(items) => items,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (StatusCode::OK, Json(items)).into_response()
}

pub async fn add_item(
  Extension(pool): Extension<MySqlPool>,
  Json(data): Json<Items>,
) -> impl IntoResponse {
  println!("{:?}", data);
  let query = format!(
    "Insert into items (item_name, item_price, item_type) values ('{}', {}, '{}')",
    data.item_name, data.item_price, data.item_type
  );
  let rows_affected = match sqlx::query(&query).execute(&pool).await {
    Ok(rows_affected) => rows_affected,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", e),
      )
        .into_response()
    }
  };

  (
    StatusCode::OK,
    format!("Number of rows affected: {:?}", rows_affected),
  )
    .into_response()
}
