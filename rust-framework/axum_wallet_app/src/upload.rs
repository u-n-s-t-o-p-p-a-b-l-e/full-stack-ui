use axum::{
    body::Bytes,
    extract::Multipart,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn upload_image(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "file" {
            let filename = field.file_name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            save_file(filename, data).await.unwrap();

            return (StatusCode::OK, Html("<p>File uploaded successfully.</p>"));
        }
    }

    (StatusCode::BAD_REQUEST, Html("<p>File upload failed!</p>"))
}

async fn save_file(filename: String, data: Bytes) -> std::io::Result<()> {
    tokio::fs::create_dir_all("uploads").await?;

    let mut file = File::create(format!("uploads/{}", filename)).await?;
    file.write_all(&data).await?;
    Ok(())
}
