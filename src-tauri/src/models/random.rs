// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Random;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Random = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Random {
    pub id: String,
    // pub slug: Option<String>,
    // pub alternative_slugs: Option<RandomAlternativeSlugs>,
    // pub created_at: Option<String>,
    // pub updated_at: Option<String>,
    // pub promoted_at: Option<String>,
    // pub width: Option<i64>,
    // pub height: Option<i64>,
    // pub color: Option<String>,
    // pub blur_hash: Option<String>,
    // pub description: Option<String>,
    // pub alt_description: Option<String>,
    // pub breadcrumbs: Option<Vec<Option<serde_json::Value>>>,
    pub urls: Urls,
    // pub links: Option<RandomLinks>,
    // pub likes: Option<i64>,
    // pub liked_by_user: Option<bool>,
    // pub current_user_collections: Option<Vec<Option<serde_json::Value>>>,
    // pub sponsorship: Option<serde_json::Value>,
    // pub topic_submissions: Option<RandomTopicSubmissions>,
    // pub user: Option<User>,
    // pub exif: Option<Exif>,
    // pub location: Option<Location>,
    // pub meta: Option<Meta>,
    // pub public_domain: Option<bool>,
    // pub tags: Option<Vec<Tag>>,
    // pub tags_preview: Option<Vec<TagsPreview>>,
    // pub views: Option<i64>,
    // pub downloads: Option<i64>,
    // pub topics: Option<Vec<Option<serde_json::Value>>>,
    // pub name: Option<String>,
    // pub founded: Option<i64>,
    // pub members: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomAlternativeSlugs {
    pub en: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exif {
    pub make: Option<serde_json::Value>,
    pub model: Option<serde_json::Value>,
    pub name: Option<serde_json::Value>,
    pub exposure_time: Option<serde_json::Value>,
    pub aperture: Option<serde_json::Value>,
    pub focal_length: Option<serde_json::Value>,
    pub iso: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomLinks {
    #[serde(rename = "self")]
    pub links_self: String,
    pub html: String,
    pub download: String,
    pub download_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: Option<serde_json::Value>,
    pub city: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub latitude: i64,
    pub longitude: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub index: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: TypeEnum,
    pub title: String,
    pub source: Option<TagSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagSource {
    pub ancestry: Ancestry,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub meta_title: String,
    pub meta_description: String,
    pub cover_photo: PurpleCoverPhoto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ancestry {
    #[serde(rename = "type")]
    pub ancestry_type: TypeClass,
    pub category: Option<TypeClass>,
    pub subcategory: Option<TypeClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeClass {
    pub slug: String,
    pub pretty_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleCoverPhoto {
    pub id: String,
    pub slug: String,
    pub alternative_slugs: PurpleAlternativeSlugs,
    pub created_at: String,
    pub updated_at: String,
    pub promoted_at: Option<String>,
    pub width: i64,
    pub height: i64,
    pub color: String,
    pub blur_hash: String,
    pub description: Option<String>,
    pub alt_description: String,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub urls: Urls,
    pub links: RandomLinks,
    pub likes: i64,
    pub liked_by_user: bool,
    pub current_user_collections: Vec<Option<serde_json::Value>>,
    pub sponsorship: Option<serde_json::Value>,
    pub topic_submissions: PurpleTopicSubmissions,
    pub premium: Option<bool>,
    pub plus: Option<bool>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleAlternativeSlugs {
    pub en: String,
    pub es: Option<String>,
    pub ja: Option<String>,
    pub fr: Option<String>,
    pub it: Option<String>,
    pub ko: Option<String>,
    pub de: Option<String>,
    pub pt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub slug: String,
    pub title: String,
    pub index: i64,
    #[serde(rename = "type")]
    pub breadcrumb_type: TypeEnum,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeEnum {
    #[serde(rename = "landing_page")]
    LandingPage,
    Search,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PurpleTopicSubmissions {
    pub nature: Option<Nature>,
    pub architecture_interior: Option<Nature>,
    pub color_of_water: Option<Nature>,
    pub wallpapers: Option<Nature>,
    pub experimental: Option<Nature>,
    pub textures_patterns: Option<Nature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nature {
    pub status: Status,
    pub approved_on: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Approved,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
    pub small_s3: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub updated_at: String,
    pub username: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub twitter_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub bio: String,
    pub location: Option<String>,
    pub links: UserLinks,
    pub profile_image: ProfileImage,
    pub instagram_username: Option<String>,
    pub total_collections: i64,
    pub total_likes: i64,
    pub total_photos: i64,
    pub total_promoted_photos: i64,
    pub accepted_tos: bool,
    pub for_hire: bool,
    pub social: Social,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    #[serde(rename = "self")]
    pub links_self: String,
    pub html: String,
    pub photos: String,
    pub likes: String,
    pub portfolio: String,
    pub following: String,
    pub followers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImage {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    pub instagram_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub twitter_username: Option<String>,
    pub paypal_email: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsPreview {
    #[serde(rename = "type")]
    pub tags_preview_type: TypeEnum,
    pub title: String,
    pub source: Option<TagsPreviewSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsPreviewSource {
    pub ancestry: Ancestry,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub meta_title: String,
    pub meta_description: String,
    pub cover_photo: FluffyCoverPhoto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyCoverPhoto {
    pub id: String,
    pub slug: String,
    pub alternative_slugs: RandomAlternativeSlugs,
    pub created_at: String,
    pub updated_at: String,
    pub promoted_at: String,
    pub width: i64,
    pub height: i64,
    pub color: String,
    pub blur_hash: String,
    pub description: String,
    pub alt_description: String,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub urls: Urls,
    pub links: RandomLinks,
    pub likes: i64,
    pub liked_by_user: bool,
    pub current_user_collections: Vec<Option<serde_json::Value>>,
    pub sponsorship: Option<serde_json::Value>,
    pub topic_submissions: FluffyTopicSubmissions,
    pub premium: bool,
    pub plus: bool,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyTopicSubmissions {
    pub nature: Nature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomTopicSubmissions {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub name: String,
    pub artist: Artist,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub founded: i64,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub duration: i64,
}
