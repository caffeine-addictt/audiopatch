use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

const NOT_FOUND: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Audiopatch</title>
    <style>
      body {
        margin: 0;
        padding: 0;
        height: 100vh;
        background-color: #0e0e0e;
        color: #ffffff;
        display: flex;
        justify-content: center;
        align-items: center;
        font-family: system-ui, sans-serif;
        text-align: center;
      }
      a {
        color: #4ea1ff;
        text-decoration: none;
        font-weight: bold;
      }
      a:hover {
        text-decoration: underline;
      }
    </style>
  </head>
  <body>
    <div>
      <h1>Looks like you're lost!</h1>
      <p>This endpoint doesn't exist.</p>
      <p><a href="https://github.com/caffeine-addictt/audiopatch" target="_blank">View Audiopatch documentation →</a></p>
    </div>
  </body>
</html>
"#;

pub async fn handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(NOT_FOUND))
}
