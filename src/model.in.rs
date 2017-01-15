// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Information about an anime, retrieved via [`get_anime`] or [`search_anime`].
///
/// [`get_anime`]: ../fn.get_anime.html
/// [`search_anime`]: ../fn.search_anime.html
#[derive(Clone, Debug, Deserialize)]
pub struct Anime {
    /// Information about the anime.
    pub attributes: AnimeAttributes,
    /// The id of the anime.
    pub id: u64,
    /// The type of item this is. Should always be [`Type::Anime`].
    ///
    /// [`Type::Anime`]: enum.Type.html#variant.Anime
    #[serde(rename="type")]
    pub kind: Type,
    /// Links related to the anime.
    pub links: HashMap<String, String>,
    /// List of the anime's relationships.
    pub relationships: AnimeRelationships,
}

/// Information about an [`Anime`].
///
/// [`Anime`]: struct.Anime.html
#[derive(Clone, Debug, Deserialize)]
pub struct AnimeAttributes {
    /// Shortened nicknames for the [anime][`Anime`].
    ///
    /// # Examples
    ///
    /// `Attack on Titan`
    ///
    /// [`Anime`]: struct.Anime.html
    #[serde(rename="abbreviatedTitles")]
    pub abbreviated_titles: Option<Value>,
    /// Age rating for the anime.
    ///
    /// # Examples
    ///
    //// [`AgeRating::R`]
    #[serde(rename="ageRating")]
    pub age_rating: AgeRating,
    /// Description of the age rating.
    ///
    /// # Examples
    ///
    /// `Violence, Profanity`
    #[serde(rename="ageRatingGuide")]
    pub age_rating_guide: String,
    /// The average of all user ratings for the anime.
    ///
    /// # Examples
    ///
    /// `4.26984658306698`
    #[serde(rename="averageRating")]
    pub average_rating: Option<f64>,
    /// Canonical title for the anime.
    ///
    /// # Examples
    ///
    /// `Attack on Titan`
    #[serde(rename="canonicalTitle")]
    pub canonical_title: String,
    /// The URL template for the cover.
    ///
    /// # Examples
    ///
    /// `https://static.hummingbird.me/anime/7442/cover/$1.png`
    #[serde(rename="coverImage")]
    pub cover_image: Option<CoverImage>,
    /// The cover's offset from the top.
    ///
    /// # Examples
    ///
    /// `263`
    #[serde(rename="coverImageTopOffset")]
    pub cover_image_top_offset: u16,
    /// Date the anime finished airing.
    ///
    /// # Examples
    ///
    /// `2013-09-28`
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// How many episodes the anime has.
    ///
    /// # Examples
    ///
    /// `25`
    #[serde(rename="episodeCount")]
    pub episode_count: u8,
    /// How many minutes long each episode is.
    ///
    /// # Examples
    ///
    /// `24`
    #[serde(rename="episodeLength")]
    pub episode_length: u8,
    /// Show format of the anime.
    ///
    /// # Examples
    ///
    /// [`AnimeType::TV`], [`AnimeType::Special`].
    ///
    /// [`AnimeType::Special`]: enum.AnimeType.html#variant.Special
    /// [`AnimeType::TV`]: enum.AnimeType.html#variant.TV
    #[serde(rename="showType")]
    pub kind: AnimeType,
    /// Whether the anime is Not Safe For Work.
    pub nsfw: bool,
    /// The URL template for the poster.
    ///
    /// # Examples
    ///
    /// `https://static.hummingbird.me/anime/7442/poster/$1.png`
    #[serde(rename="posterImage")]
    pub poster_image: Image,
    /// How many times each rating has been given to the anime.
    #[serde(rename="ratingFrequencies")]
    pub rating_frequencies: RatingFrequencies,
    /// Unique slug used for page URLs.
    ///
    /// # Examples
    ///
    /// `attack-on-titan`
    pub slug: String,
    /// Date the anime started airing/was released.
    ///
    /// # Examples
    ///
    /// `2013-04-07`
    #[serde(rename="startDate")]
    pub start_date: String,
    /// Synopsis of the anime.
    ///
    /// # Examples
    ///
    /// `Several hundred years ago, humans were exterminated by titans...`
    pub synopsis: String,
    /// The titles of the anime.
    pub titles: Titles,
    /// YouTube video id for PV.
    ///
    /// # Examples
    ///
    /// `n4Nj6Y_SNYI`
    #[serde(rename="youtubeVideoId")]
    pub youtube_video_id: Option<String>,
}

/// Links related to the media item.
#[derive(Clone, Debug, Deserialize)]
pub struct Links {
    /// Link to a related media item.
    pub related: String,
    /// Direct link to the media item.
    #[serde(rename="self")]
    pub own: String,
}

/// A relationship for a media item.
#[derive(Clone, Debug, Deserialize)]
pub struct Relationship {
    /// Links for one set of the media item's related links.
    pub links: Links,
}

/// Relationships for an [`Anime`].
///
/// [`Anime`]: struct.Anime.html
#[derive(Clone, Debug, Deserialize)]
pub struct AnimeRelationships {
    /// Castings for the anime.
    pub castings: Relationship,
    /// The anime's episodes.
    pub episodes: Relationship,
    /// The anime's genres.
    pub genres: Relationship,
    /// The anime's installments.
    pub installments: Relationship,
    /// The anime's mappings.
    pub mappings: Relationship,
    /// The anime's reviews.
    pub reviews: Relationship,
    /// The anime's streaming links.
    #[serde(rename="streamingLinks")]
    pub streaming_links: Relationship,
}

/// Information about the cover image for a media item.
#[derive(Clone, Debug, Deserialize)]
pub struct CoverImage {
    /// Link to the large copy.
    pub large: Option<String>,
    /// Link to the original copy.
    pub original: Option<String>,
    /// Link to the small copy.
    pub small: Option<String>,
}

/// A list of links to the media's relevant images.
#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    /// Link to a large size of the image.
    pub large: Option<String>,
    /// Link to a medium size of the image.
    pub medium: Option<String>,
    /// Link to an original size of the image.
    pub original: Option<String>,
    /// Link to a small size of the image.
    pub small: Option<String>,
    /// Link to a tiny size of the image.
    pub tiny: Option<String>,
}

/// Information about a manga, retrived via [`get_manga`] or [`search_manga`].
///
/// [`get_manga`]: ../fn.get_manga.html
/// [`search_manga`]: ../fn.search_manga.html
#[derive(Clone, Debug, Deserialize)]
pub struct Manga {
    /// Information about the manga.
    pub attributes: MangaAttributes,
    /// The id of the manga.
    pub id: u64,
    /// The type of item this is. Should always be [`Type::Manga`].
    ///
    /// [`Type::Manga`]: enum.Type.html#variant.Manga
    #[serde(rename="type")]
    pub kind: Type,
    /// Links related to the manga.
    pub links: HashMap<String, String>,
}

/// Information about a [`Manga`].
///
/// [`Manga`]: struct.Manga.html
#[derive(Clone, Debug, Deserialize)]
pub struct MangaAttributes {
    /// Shortened nicknames for the manga.
    #[serde(rename="abbreviatedTitles")]
    pub abbreviated_titles: Option<Value>,
    /// The average of all user ratings for the manga.
    ///
    /// # Examples
    ///
    /// `4.34926964198231`
    #[serde(rename="averageRating")]
    pub average_rating: Option<f64>,
    /// Canonical title for the manga.
    ///
    /// # Examples
    ///
    /// `Horimiya`
    #[serde(rename="canonicalTitle")]
    pub canonical_title: String,
    /// The number of chapters released.
    #[serde(rename="chapterCount")]
    pub chapter_count: u64,
    /// The URL template for the cover.
    ///
    /// # Examples
    ///
    /// `https://static.hummingbird.me/manga/22352/cover/$1.png`
    #[serde(rename="coverImage")]
    pub cover_image: Option<CoverImage>,
    /// The cover's offset from the top.
    ///
    /// # Examples
    ///
    /// `60`
    #[serde(rename="coverImageTopOffset")]
    pub cover_image_top_offset: u16,
    /// Date the manga finished.
    ///
    /// # Examples
    ///
    /// `2013-09-28`
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// Show format of the manga.
    ///
    /// # Examples
    ///
    /// [`MangaType::Novel`]
    ///
    /// [`MangaType::Novel`]: enum.MangaType.html#variant.Novel
    #[serde(rename="mangaType")]
    pub kind: MangaType,
    /// Whether the manga is Not Safe For Work.
    pub nsfw: bool,
    /// The URL template for the poster.
    ///
    /// # Examples
    ///
    /// `https://static.hummingbird.me/manga/22352/poster/$1.png`
    #[serde(rename="posterImage")]
    pub poster_image: Image,
    /// How many times each rating has been given to the manga.
    #[serde(rename="ratingFrequencies")]
    pub rating_frequencies: RatingFrequencies,
    /// Name of media of serialization.
    pub serialization: String,
    /// Unique slug used for page URLs.
    ///
    /// # Examples
    ///
    /// `horimiya`
    pub slug: String,
    /// Date the manga was serialized.
    ///
    /// # Examples
    ///
    /// `2013-04-07`
    #[serde(rename="startDate")]
    pub start_date: String,
    /// Synopsis of the manga.
    ///
    /// # Examples
    ///
    /// `Hori may seem like a normal teenage girl, but she's a completely...`
    pub synopsis: String,
    /// The titles of the manga.
    pub titles: Titles,
    /// The number of volumes released for the manga.
    #[serde(rename="volumeCount")]
    pub volume_count: u64,
    /// The id of the related YouTube video.
    #[serde(rename="youtubeVideoId")]
    pub youtube_video_id: Option<String>,
}

/// How many times each rating has been given to the media item.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RatingFrequencies {
    /// Number of 0 stars given.
    #[serde(default, rename="0.0")]
    pub rating_0_0: u64,
    /// Number of 0.5 stars given.
    #[serde(default, rename="0.5")]
    pub rating_0_5: u64,
    /// Number of 1.0 stars given.
    #[serde(default, rename="1.0")]
    pub rating_1_0: u64,
    /// Nubmer of 1.5 stars given.
    #[serde(default, rename="1.5")]
    pub rating_1_5: u64,
    /// Number of 2.0 stars given.
    #[serde(default, rename="2.0")]
    pub rating_2_0: u64,
    /// Number of 2.5 stars given.
    #[serde(default, rename="2.5")]
    pub rating_2_5: u64,
    /// Number of 3.0 stars given.
    #[serde(default, rename="3.0")]
    pub rating_3_0: u64,
    /// Number of 3.5 stars given.
    #[serde(default, rename="3.5")]
    pub rating_3_5: u64,
    /// Number of 4.0 stars given.
    #[serde(default, rename="4.0")]
    pub rating_4_0: u64,
    /// Number of 4.5 stars given.
    #[serde(default, rename="4.5")]
    pub rating_4_5: u64,
    /// Number of 5.0 stars given.
    #[serde(default, rename="5.0")]
    pub rating_5_0: u64,
}

#[derive(Clone, Debug, Deserialize)]
/// The titles of the anime.
pub struct Titles {
    /// The English title of the anime.
    ///
    /// # Examples
    ///
    /// `Attack on Titan`
    pub en: Option<String>,
    /// The romaji title of the anime.
    ///
    /// # Examples
    ///
    /// `Shingeki no Kyojin`
    pub en_jp: Option<String>,
    /// The Japanese title of the anime.
    ///
    /// # Examples
    ///
    /// `進撃の巨人`
    pub ja_jp: String,
}

/// Data from a response.
#[derive(Clone, Debug, Deserialize)]
pub struct Response<T: Deserialize> {
    /// The full data from a response.
    pub data: T,
    /// Links relevant to the search.
    #[serde(default)]
    pub links: HashMap<String, String>,
}

/// Information about a user, retrieved via [`get_user`] or [`search_user`].
///
/// [`get_user`]: ../fn.get_user.html
/// [`search_user`]: ../fn.search_user.html
#[derive(Clone, Debug, Deserialize)]
pub struct User {
    /// Information about the user.
    pub attributes: UserAttributes,
    /// The id of the user.
    pub id: u64,
    /// The type of item this is. Should always be [`Type::User`].
    ///
    /// [`Type::User`]: enum.Type.html#variant.User
    pub kind: Type,
    /// Links related to the user.
    pub links: HashMap<String, String>,
    /// List of the user's relationships.
    pub relationships: UserRelationships,
}

/// Information about a [`User`].
///
/// [`User`]: struct.User.html
#[derive(Clone, Debug, Deserialize)]
pub struct UserAttributes {
    /// The raw markdown for the user's long-form about text.
    ///
    /// # Examples
    ///
    /// `I'm curious about https://kitsu.io/anime/nichijou!`
    pub about: String,
    /// The processed and sanitized HTML for the user's long form about text.
    ///
    /// # Examples
    ///
    /// `I'm curious about <a href="https://kitsu.io/anime/nichijou">Nichijou</a>!`
    #[serde(rename="aboutFormatted")]
    pub about_formatted: Option<String>,
    /// Links to the user's avatar.
    pub avatar: Image,
    /// A short (140 character) biographical blurb about the user.
    ///
    /// # Examples
    ///
    /// `私、気になります!`
    pub bio: String,
    /// The user's birthday.
    ///
    /// # Examples
    ///
    /// `1985-07-26`
    pub birthday: Option<String>,
    /// Number of comments user has submitted.
    ///
    /// # Examples
    ///
    /// `15`
    #[serde(rename="commentsCount")]
    pub comments_count: u64,
    /// Links to the user's cover image.
    #[serde(rename="coverImage")]
    pub cover_image: Image,
    /// When the user signed up.
    ///
    /// # Examples
    ///
    /// `1985-07-26T22:13:20.223Z`
    #[serde(rename="createdAt")]
    pub created_at: String,
    /// The user's Facebook id if they have signed in with Facebook.
    ///
    /// # Examples
    ///
    /// `1234567890`
    #[serde(rename="facebookId")]
    pub facebook_id: Option<u64>,
    /// The number of media items the user has favorited.
    #[serde(rename="favoritesCount")]
    pub favorites_count: u64,
    /// Whether the user's feed is completed.
    #[serde(rename="feedCompleted")]
    pub feed_completed: bool,
    /// Number of people following this user.
    ///
    /// # Examples
    ///
    /// `12`
    #[serde(rename="followersCount")]
    pub followers_count: u64,
    /// Number of people this user is following.
    ///
    /// # Examples
    ///
    /// `300`
    #[serde(rename="followingCount")]
    pub following_count: u64,
    /// The user's gender, if provided.
    ///
    /// # Examples
    ///
    /// [`Gender::female`]
    ///
    /// [`Gender::female`]: enum.Gender.html#variant.female
    pub gender: Option<Gender>,
    /// Number of minutes of anime watched.
    ///
    /// # Examples
    ///
    /// `550`
    #[serde(rename="lifeSpentOnAnime")]
    pub life_spent_on_anime: u64,
    /// Number of posts user has liked.
    ///
    /// # Examples
    ///
    /// `12`
    #[serde(rename="likesGivenCount")]
    pub likes_given_count: u64,
    /// Number of likes the user's post has received.
    ///
    /// # Examples
    ///
    /// `45`
    #[serde(rename="likesReceivedCount")]
    pub likes_received_count: u64,
    /// A user-provided location.
    ///
    /// # Examples
    ///
    /// `The Internet`
    pub location: Option<String>,
    /// The user's current username.
    ///
    /// # Examples
    ///
    /// `chitanda`
    pub name: String,
    /// An array of previous names the user has gone by, in
    /// reverse-chronological order.
    ///
    /// # Examples
    ///
    /// ```rs
    /// vec![
    ///     "oldn ame".to_owned(),
    ///     "older name".to_owned()
    /// ]
    /// ```
    #[serde(rename="pastNames")]
    pub past_names: Vec<String>,
    /// Number of posts user has submitted.
    ///
    /// # Examples
    ///
    /// `3`
    #[serde(rename="postsCount")]
    pub posts_count: u64,
    /// Whether the user has finished completing their profile.
    #[serde(rename="profileCompleted")]
    pub profile_completed: bool,
    /// When the user's pro subscripten expires.
    #[serde(rename="proExpiresAt")]
    pub pro_expires_at: Option<String>,
    /// Number of media user has rated.
    ///
    /// # Examples
    ///
    /// `1`
    #[serde(rename="ratingsCount")]
    pub ratings_count: u64,
    /// The number of reviews the user has posted.
    #[serde(rename="reviewsCount")]
    pub reviews_count: u64,
    /// The user's title.
    pub title: Option<String>,
    /// When the user last updated their profile.
    ///
    /// **Note**: This _can_ be the same as the [`created_at`] field, which
    /// indicates that the profile has not been "updated" since creation.
    ///
    /// # Examples
    ///
    /// `1985-07-26T22:13:20.223Z`
    ///
    /// [`created_at`]: #structfield.created_at
    #[serde(rename="updatedAt")]
    pub updated_at: String,
    /// Whether the user has a waifu or husbando.
    ///
    /// # Examples
    ///
    /// `None`
    pub waifu_or_husbando: Option<WaifuOrHusbando>,
    /// The user's website.
    ///
    /// # Examples
    ///
    /// `https://en.wikipedia.org/wiki/Nichijou`
    pub website: Option<String>,
}

/// Relationships for a [`User`].
///
/// [`User`]: struct.User.html
#[derive(Clone, Debug, Deserialize)]
pub struct UserRelationships {
    /// Links to users the user blocks.
    pub blocks: Relationship,
    /// Links to the user's favorite media entries.
    pub favorites: Relationship,
    /// Links to users following the user.
    pub followers: Relationship,
    /// Links to users the user follows.
    pub following: Relationship,
    /// Links to the user's library entries.
    #[serde(rename="libraryEntries")]
    pub library_entries: Relationship,
    /// Links to profiles linked to the user.
    #[serde(rename="linkedProfiles")]
    pub linked_profiles: Relationship,
    /// Links to the user's media.
    #[serde(rename="mediaFollows")]
    pub media_follows: Relationship,
    /// Links to the user's pinned post on their profile.
    #[serde(rename="pinnedPost")]
    pub pinned_post: Relationship,
    /// Links to the user's reviews.
    pub reviews: Relationship,
    /// Links to the user's roles.
    #[serde(rename="userRoles")]
    pub user_roles: Relationship,
    /// Links to the user's waifu or husbando.
    pub waifu: Relationship,
}

/// The age rating of the [`Anime`].
///
/// [`Anime`]: struct.Anime.html
#[derive(Clone, Debug, Deserialize)]
pub enum AgeRating {
    /// Indicator that the anime is rated G.
    G,
    /// Indicator that the anime is rated PG.
    PG,
    /// Indicator that the anime is rated PG-13.
    #[serde(rename="PG-13")]
    PG13,
    /// Indicator that the anime is rated R.
    R,
    /// Indicator that the anime is rated R-17.
    R17,
    /// Indicator that the anime is rated R-17+.
    #[serde(rename="R17+")]
    R17Plus,
    /// Indicator that the anime is rated R-18+.
    #[serde(rename="R18+")]
    R18Plus,
    /// Indicator that the anime is rated TV-Y7.
    #[serde(rename="TV-Y7")]
    TvY7,
}

/// The type of [`Anime`].
///
/// [`Anime`]: struct.Anime.html
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AnimeType {
    /// Indicator that the anime is a movie.
    #[serde(rename="movie")]
    Movie,
    /// Indicator that the anime is music.
    #[serde(rename="music")]
    Music,
    /// Indicator that the anime is an Original Net Animation.
    ONA,
    /// Indicator that the anime is an Original Video Animation.
    OVA,
    /// Indicator that the anime is a special.
    #[serde(rename="special")]
    Special,
    /// Indicator that the anime is a TV show.
    TV,
}

/// The [`User`]'s gender.
///
/// [`User`]: struct.User.html
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Gender {
    /// Indicator that the user is female.
    Female,
    /// Indicator that the user is male.
    Male,
}

/// The type of a [`Manga`].
///
/// [`Manga`]: struct.Manga.html
#[derive(Clone, Debug, Deserialize)]
pub enum MangaType {
    /// Indicator that the manga is a doujin.
    Doujin,
    /// Indicator that the manga is a regular manga.
    Manga,
    /// Indicator that the manga is a manhua.
    Manhua,
    /// Indicator that the manga is a novel.
    Novel,
    /// Indicator that the manga is a oneshot.
    Oneshot,
}

/// The type of result from a search or retrieval.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Type {
    /// Indicator that the result is an [`Anime`].
    ///
    /// [`Anime`]: struct.Anime.html
    #[serde(rename="anime")]
    Anime,
    /// Indicator that the result is a drama.
    #[serde(rename="drama")]
    Drama,
    /// Indicator that the result is a [`Manga`].
    ///
    /// [`Manga`]: struct.Manga.html
    #[serde(rename="manga")]
    Manga,
    /// Indicator that the result is a [`User`].
    ///
    /// [`User`]: struct.User.html
    #[serde(rename="users")]
    User,
}

/// Indicator of whether a [`User`] has a waifu or husbando.
///
/// [`User`]: struct.User.html
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum WaifuOrHusbando {
    /// Indicator that the user has a husbando.
    Husbando,
    /// Indicator that the user has a waifu.
    Waifu,
}
