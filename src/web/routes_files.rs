use std::collections::HashMap;
use axum::body::Bytes;
use axum::extract::{Multipart, Path};
use axum::response::IntoResponse;
use axum_macros::debug_handler;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use shiva::core::{Document, TransformerTrait};
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct DownloadFile {
    file_name: String,
    file_data: (Bytes, HashMap<String, Bytes>),
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Format {
    Markdown,
    Html,
    Text,
    Pdf,
}

#[derive(Debug, Clone, Serialize)]
struct InputFileInfo {
    upload_file_name: String,
    upload_file_extension: String,
    upload_file_data: Bytes,
}


#[debug_handler]
    pub async fn handler_convert_file(
        Path(output_format): Path<String>,
        multipart: Multipart,
    ) -> impl IntoResponse {
        println!("-->> {:<12} - handler_convert_file - output_extension_{output_format}", "HANDLER");

        let data_uploaded_file = upload_file(multipart).await.unwrap();

        let build_response_file = convert_file(
            data_uploaded_file.upload_file_name,
            data_uploaded_file.upload_file_extension,
            data_uploaded_file.upload_file_data,
            output_format,
        ).await.unwrap();

        Ok(build_response_file)
    }


async fn convert_file(
    file_name: String,
    file_extension: String,
    input_file_data_bytes: Bytes,
    output_format: String,
) -> Result<DownloadFile> {
    let document = match file_extension.as_str() {
        "md" => Document::from(
            shiva::markdown::Transformer::parse(&input_file_data_bytes, &HashMap::new())
                .unwrap()),
//we scale the code to other acceptable formats
        _ => return Err(Error::FailParseDocument),
    };

    let output_file_bytes = match output_format.as_str() {
        "md" => shiva::markdown::Transformer::generate(&document).unwrap(),
//we scale the code to other acceptable formats
        _ => return Err(Error::FailConvertFile),
    };

    Ok(DownloadFile {
        file_name,
        file_data: output_file_bytes,
    })
}

async fn upload_file(mut multipart: Multipart) -> Result<InputFileInfo> {
    let mut file_name = None;
    let mut file_extension = None;
    let mut file_data = Bytes::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        let filename = field.file_name().unwrap_or("").to_string();

        if name == "file" {
            file_name = Some(filename.clone());
            file_extension = filename
                .split(".")
                .last()
                .map(|ext| ext.to_lowercase())
                .filter(|ext| !ext.trim().is_empty())
                .map(String::from);

            if let Some(ref ext) = file_extension {
                if supported_format(ext) {
                    file_data = field.bytes().await.unwrap();
                } else {
                    return Err(Error::FailBytes)
                }
            } else {
                return Err(Error::UnsupportedFormat)
            }
        }
    }
    let file_name = file_name.unwrap_or("Shiva_convert".to_string());
    let file_extension = file_extension.ok_or("File extension not found").unwrap();
    let file_data = file_data;

    Ok(InputFileInfo {
        upload_file_name: file_name,
        upload_file_extension: file_extension,
        upload_file_data: file_data,
    })
}

fn supported_format(file_extension: &str) -> bool {
    match file_extension {
        "md" | "html" | "htm" | "txt " | "pdf" | "json" => true,

        _ => false,
    }
}