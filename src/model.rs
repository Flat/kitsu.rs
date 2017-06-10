//! A collection of models for use with - and returned by - the Kitsu API.
// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zey@zey.moe>
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
use serde_json;
use std::collections::HashMap;
use std::mem;
use super::{Result, request};

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

impl Anime {
    /// Gets an anime by its Id.
    #[inline]
    pub fn get(id: u64) -> Result<Anime> {
        Ok(request::get_anime(id)?.data)
    }

    /// The current airing status of the anime.
    #[inline]
    pub fn airing_status(&self) -> AiringStatus {
        self.attributes.airing_status()
    }

    /// Refreshes the instance of the anime, retrieving the anime and replacing
    /// the current instance with it. Returns the old copy.
    #[inline]
    pub fn refresh(&mut self) -> Result<Anime> {
        let id = self.id;

        Ok(mem::replace(self, Anime::get(id)?))
    }

    /// Generates a URL to the Kitsu page for the anime.
    #[inline]
    pub fn url(&self) -> String {
        self.attributes.url()
    }

    /// Generates a formatted URL to the youtube video.
    #[inline]
    pub fn youtube_url(&self) -> Option<String> {
        self.attributes.youtube_url()
    }
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
    pub abbreviated_titles: Vec<String>,
    /// Age rating for the anime.
    ///
    /// # Examples
    ///
    //// [`AgeRating::R`]
    #[serde(rename="ageRating")]
    pub age_rating: Option<AgeRating>,
    /// Description of the age rating.
    ///
    /// # Examples
    ///
    /// `Violence, Profanity`
    #[serde(rename="ageRatingGuide")]
    pub age_rating_guide: Option<String>,
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
    pub episode_count: Option<u32>,
    /// How many minutes long each episode is.
    ///
    /// # Examples
    ///
    /// `24`
    #[serde(rename="episodeLength")]
    pub episode_length: Option<u32>,
    /// How many favourites the anime has.
    ///
    /// # Examples
    ///
    /// `209`
    #[serde(rename="favoritesCount")]
    pub favourites_count: Option<u32>,
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
    /// The rank based on the popularity of the anime.
    ///
    /// # Examples
    ///
    /// `2`
    #[serde(rename="populartiyRank")]
    pub popularity_rank: Option<u32>,
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
    /// The rank of the anime based on its overall rating.
    ///
    /// # Examples
    ///
    /// `5`
    #[serde(rename="ratingRank")]
    pub rating_rank: Option<u32>,
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
    /// The sub type of the anime.
    #[serde(rename="subType")]
    pub sub_type: Option<String>,
    /// Synopsis of the anime.
    ///
    /// # Examples
    ///
    /// `Several hundred years ago, humans were exterminated by titans...`
    pub synopsis: String,
    /// The titles of the anime.
    pub titles: AnimeTitles,
    /// The number of users who have marked the anime.
    ///
    /// # Examples
    ///
    /// `3232532`
    #[serde(rename="userCount")]
    pub user_count: Option<u32>,
    /// YouTube video id for PV.
    ///
    /// # Examples
    ///
    /// `n4Nj6Y_SNYI`
    #[serde(rename="youtubeVideoId")]
    pub youtube_video_id: Option<String>,
}

impl AnimeAttributes {
    /// The current airing status of the anime.
    pub fn airing_status(&self) -> AiringStatus {
        if self.end_date.is_some() {
            AiringStatus::Finished
        } else {
            AiringStatus::Airing
        }
    }

    /// Generates a URL to the Kitsu page for the anime.
    #[inline]
    pub fn url(&self) -> String {
        format!("https://kitsu.io/anime/{}", self.slug)
    }

    /// Generates a formatted URL to the youtube video.
    #[inline]
    pub fn youtube_url(&self) -> Option<String> {
        self.youtube_video_id.as_ref().map(youtube_url)
    }
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

impl CoverImage {
    /// Retrieves the URL to the largest cover image in descending order where
    /// available, if any.
    ///
    /// This places priority on the [`original`] field.
    ///
    /// [`original`]: #structfield.original
    pub fn largest<'a>(&'a self) -> Option<&'a String> {
        self.original.as_ref().or(self.large.as_ref()).or(self.small.as_ref())
    }
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

impl Image {
    /// Retrieves the URL to the largest image in descending order where
    /// available, if any.
    ///
    /// This places priority on the [`original`] field.
    ///
    /// [`original`]: #structfield.original
    pub fn largest<'a>(&'a self) -> Option<&'a String> {
        self.original.as_ref()
            .or(self.large.as_ref())
            .or(self.medium.as_ref())
            .or(self.small.as_ref())
            .or(self.tiny.as_ref())
    }
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

impl Manga {
    /// Gets a manga by its Id.
    #[inline]
    pub fn get(id: u64) -> Result<Manga> {
        Ok(request::get_manga(id)?.data)
    }

    /// The current airing status of the manga.
    #[inline]
    pub fn airing_status(&self) -> AiringStatus {
        self.attributes.airing_status()
    }

    /// Refreshes the instance of the manga, retrieving the manga and replacing
    /// the current instance with it. Returns the old copy.
    #[inline]
    pub fn refresh(&mut self) -> Result<Manga> {
        let id = self.id;

        Ok(mem::replace(self, Manga::get(id)?))
    }

    /// Generates a URL to the Kitsu page for the manga.
    #[inline]
    pub fn url(&self) -> String {
        self.attributes.url()
    }

    /// Generates a formatted URL to the youtube video.
    #[inline]
    pub fn youtube_url(&self) -> Option<String> {
        self.attributes.youtube_url()
    }
}

/// Information about a [`Manga`].
///
/// [`Manga`]: struct.Manga.html
#[derive(Clone, Debug, Deserialize)]
pub struct MangaAttributes {
    /// Shortened nicknames for the manga.
    #[serde(rename="abbreviatedTitles")]
    pub abbreviated_titles: Vec<String>,
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
    pub chapter_count: Option<u64>,
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
    /// The rank based on the popularityof the manga.
    ///
    /// # Examples
    ///
    /// `10`
    #[serde(rename="populartiyRank")]
    pub popularity_rank: Option<u32>,
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
    /// The rank of the manga based on its overall rating.
    ///
    /// # Examples
    ///
    /// `13`
    #[serde(rename="ratingRank")]
    pub rating_rank: Option<u32>,
    /// Name of media of serialization.
    pub serialization: Option<String>,
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
    pub start_date: Option<String>,
    /// Synopsis of the manga.
    ///
    /// # Examples
    ///
    /// `Hori may seem like a normal teenage girl, but she's a completely...`
    pub synopsis: String,
    /// The titles of the manga.
    pub titles: MangaTitles,
    /// The number of volumes released for the manga.
    #[serde(rename="volumeCount")]
    pub volume_count: Option<u64>,
    /// The id of the related YouTube video.
    #[serde(rename="youtubeVideoId")]
    pub youtube_video_id: Option<String>,
}

impl MangaAttributes {
    /// The current airing status of the manga.
    pub fn airing_status(&self) -> AiringStatus {
        if self.end_date.is_some() {
            AiringStatus::Finished
        } else {
            AiringStatus::Airing
        }
    }

    /// Generates a URL to the Kitsu page for the manga.
    #[inline]
    pub fn url(&self) -> String {
        format!("https://kitsu.io/manga/{}", self.slug)
    }

    /// Generates a formatted URL to the youtube video.
    #[inline]
    pub fn youtube_url(&self) -> Option<String> {
        self.youtube_video_id.as_ref().map(youtube_url)
    }
}

/// How many times each rating has been given to the media item.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RatingFrequencies {
    /// Number of 0 stars given.
    #[serde(default, rename="0.0")]
    pub rating_0_0: i64,
    /// Number of 0.5 stars given.
    #[serde(default, rename="0.5")]
    pub rating_0_5: i64,
    /// Number of 1.0 stars given.
    #[serde(default, rename="1.0")]
    pub rating_1_0: i64,
    /// Nubmer of 1.5 stars given.
    #[serde(default, rename="1.5")]
    pub rating_1_5: i64,
    /// Number of 2.0 stars given.
    #[serde(default, rename="2.0")]
    pub rating_2_0: i64,
    /// Number of 2.5 stars given.
    #[serde(default, rename="2.5")]
    pub rating_2_5: i64,
    /// Number of 3.0 stars given.
    #[serde(default, rename="3.0")]
    pub rating_3_0: i64,
    /// Number of 3.5 stars given.
    #[serde(default, rename="3.5")]
    pub rating_3_5: i64,
    /// Number of 4.0 stars given.
    #[serde(default, rename="4.0")]
    pub rating_4_0: i64,
    /// Number of 4.5 stars given.
    #[serde(default, rename="4.5")]
    pub rating_4_5: i64,
    /// Number of 5.0 stars given.
    #[serde(default, rename="5.0")]
    pub rating_5_0: i64,
}

#[derive(Clone, Debug, Deserialize)]
/// The titles of the anime.
pub struct AnimeTitles {
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
    pub ja_jp: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
/// The titles of the manga.
pub struct MangaTitles {
    /// The English title of the manga.
    ///
    /// # Examples
    ///
    /// `Attack on Titan`
    pub en: Option<String>,
    /// The romaji title of the manga.
    ///
    /// # Examples
    ///
    /// `Shingeki no Kyojin`
    pub en_jp: Option<String>,
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
    #[serde(rename="type")]
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
    pub avatar: Option<Image>,
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
    pub cover_image: Option<Image>,
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
    /// `female`
    pub gender: Option<String>,
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

impl User {
    /// Generates a URL to the Kitsu page for the user.
    #[inline]
    pub fn url(&self) -> String {
        self.attributes.url()
    }
}

impl UserAttributes {
    /// Generates a URL to the Kitsu page for the user.
    #[inline]
    pub fn url(&self) -> String {
        format!("https://kitsu.io/users/{}", self.name)
    }
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
    #[serde(rename="profileLinks")]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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
    /// Indicator that the anime is rated R18.
    R18,
    /// Indicator that the anime is rated R-18+.
    #[serde(rename="R18+")]
    R18Plus,
    /// Indicator that the anime is rated TV-Y7.
    #[serde(rename="TV-Y7")]
    TvY7,
}

impl AgeRating {
    /// The name of the age rating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kitsu_io::model::AgeRating;
    ///
    /// assert_eq!(AgeRating::PG.name().unwrap(), "PG");
    /// ```
    pub fn name(&self) -> Result<String> {
        let mut name = serde_json::to_string(self)?;

        // Serde wraps the encoded string in quotation marks, so remove those.
        let _ = name.remove(0);
        let _ = name.pop();

        Ok(name)
    }
}

/// The airing status of an [`Anime`].
///
/// [`Anime`]: struct.Anime.html
pub enum AiringStatus {
    /// Indicator that the anime is currently airing.
    Airing,
    /// Indicator that the anime has finished airing.
    Finished,
}

impl AiringStatus {
    /// The name of the airing status.
    pub fn name(&self) -> &str {
        match *self {
            AiringStatus::Airing => "airing",
            AiringStatus::Finished => "finished",
        }
    }
}

/// The type of [`Anime`].
///
/// [`Anime`]: struct.Anime.html
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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

impl AnimeType {
    /// The name of the [anime][`Anime`] type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kitsu_io::model::AnimeType;
    ///
    /// assert_eq!(AnimeType::Movie.name().unwrap(), "movie");
    /// assert_eq!(AnimeType::TV.name().unwrap(), "TV");
    /// ```
    ///
    /// [`Anime`]: struct.Anime.html
    pub fn name(&self) -> Result<String> {
        let mut name = serde_json::to_string(self)?;

        let _ = name.remove(0);
        let _ = name.pop();

        Ok(name)
    }
}

/// The type of a [`Manga`].
///
/// [`Manga`]: struct.Manga.html
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MangaType {
    /// Indicator that the manga is a doujin.
    #[serde(rename="doujin")]
    Doujin,
    /// Indicator that the manga is a regular manga.
    #[serde(rename="manga")]
    Manga,
    /// Indicator that the manga is a manhua.
    #[serde(rename="manhua")]
    Manhua,
    /// Indicator that the manga is a novel.
    #[serde(rename="novel")]
    Novel,
    /// Indicator that the manga is a oneshot.
    #[serde(rename="oneshot")]
    Oneshot,
}

impl MangaType {
    /// The name of the age rating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kitsu_io::model::MangaType;
    ///
    /// assert_eq!(MangaType::Novel.name().unwrap(), "novel");
    /// ```
    pub fn name(&self) -> Result<String> {
        let mut name = serde_json::to_string(self)?;

        let _ = name.remove(0);
        let _ = name.pop();

        Ok(name)
    }
}

/// The type of result from a search or retrieval.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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

impl Type {
    /// The name of the age rating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kitsu_io::model::Type;
    ///
    /// assert_eq!(Type::Anime.name().unwrap(), "anime");
    /// ```
    pub fn name(&self) -> Result<String> {
        let mut name = serde_json::to_string(self)?;

        let _ = name.remove(0);
        let _ = name.pop();

        Ok(name)
    }
}

/// Indicator of whether a [`User`] has a waifu or husbando.
///
/// [`User`]: struct.User.html
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WaifuOrHusbando {
    /// Indicator that the user has a husbando.
    #[serde(rename="husbando")]
    Husbando,
    /// Indicator that the user has a waifu.
    #[serde(rename="waifu")]
    Waifu,
}

impl WaifuOrHusbando {
    /// The name of the age rating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kitsu_io::model::WaifuOrHusbando;
    ///
    /// assert_eq!(WaifuOrHusbando::Husbando.name().unwrap(), "husbando");
    /// ```
    pub fn name(&self) -> Result<String> {
        let mut name = serde_json::to_string(self)?;

        let _ = name.remove(0);
        let _ = name.pop();

        Ok(name)
    }
}

#[inline]
fn youtube_url(id: &String) -> String {
    format!("https://www.youtube.com/watch?v={}", id)
}
