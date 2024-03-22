use leptos::*;
use serde::{ Deserialize, Serialize };
use server_fn::codec::SerdeLite;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: u16,
    pub title: String,
    pub completed: bool,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::ServerFnError;
    use sqlx::{ Connection, SqliteConnection };

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
    }
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use self::ssr::*;
    use http::request::Parts;

    // this is just an example of how to access server context injected in the handlers
    let req_parts = use_context::<Parts>();

    if let Some(req_parts) = req_parts {
        println!("Uri = {:?}", req_parts.uri);
    }

    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut todos = Vec::new();
    let mut rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        todos.push(row);
    }

    // Lines below show how to set status code and headers on the response
    // let resp = expect_context::<ResponseOptions>();
    // resp.set_status(StatusCode::IM_A_TEAPOT);
    // resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

    Ok(todos)
}

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;

    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1250));

    match
        sqlx
            ::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
            .bind(title)
            .execute(&mut conn).await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(output = SerdeLite)]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;

    Ok(
        sqlx
            ::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&mut conn).await
            .map(|_| ())?
    )
}
