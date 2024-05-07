// models.rs

use aws_sdk_s3 as s3;
use std::fmt;
use s3::error::SdkError;
use std::error::Error as StdError;
use s3::operation::create_bucket::CreateBucketError;
use s3::operation::put_bucket_tagging::PutBucketTaggingError;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Note {
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub title: String,
    pub content: String,
    pub nonce: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub timestamp: Option<String>,
}

#[derive(Debug)]
pub enum BucketError {
    BucketAlreadyExists,
    S3Error(Box<dyn StdError>),
    TaggingError,
}

impl From<SdkError<CreateBucketError>> for BucketError {
    fn from(err: SdkError<CreateBucketError>) -> BucketError {
        BucketError::S3Error(Box::new(err))
    }
}

impl From<SdkError<PutBucketTaggingError>> for BucketError {
    fn from(err: SdkError<PutBucketTaggingError>) -> BucketError {
        BucketError::S3Error(Box::new(err))
    }
}

impl fmt::Display for BucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BucketError::BucketAlreadyExists => write!(f, "Bucket already exists"),
            BucketError::S3Error(err) => write!(f, "S3 error: {}", err),
            BucketError::TaggingError => write!(f, "Error creating tag"),
        }
    }
}

impl From<aws_sdk_s3::Error> for BucketError {
    fn from(err: aws_sdk_s3::Error) -> BucketError {
        BucketError::S3Error(Box::new(err))
    }
}