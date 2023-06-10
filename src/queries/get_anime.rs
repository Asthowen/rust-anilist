pub const GET_ANIME: &str = r#"
query ($id: Int, $id_mal: Int) {
  Media (id: $id, idMal: $id_mal, type: ANIME) {
    id
    idMal
    title {
      romaji
      english
      native
      userPreferred
    }
    format
    status(version: 2)
    description(asHtml: true)
    startDate {
      year
      month
      day
    }
    endDate {
      year
      month
      day
    }
    season
    seasonYear
    seasonInt
    episodes
    duration
    countryOfOrigin
    isLicensed
    source(version: 3)
    hashtag
    updatedAt
    coverImage {
      extraLarge
      large
      medium
      color
    }
    bannerImage
    genres
    synonyms
    averageScore
    meanScore
    popularity
    isLocked
    trending
    favourites
    tags {
      id
      name
      description
      category
      rank
      isGeneralSpoiler
      isMediaSpoiler
      isAdult
      userId
    }
    relations {
      edges {
        node {
          id
          idMal
          title {
            romaji
            english
            native
            userPreferred
          }
          type
          format
          status(version: 2)
          description(asHtml: true)
          coverImage {
            extraLarge
            large
            medium
            color
          }
          bannerImage
          averageScore
          meanScore
          siteUrl
        }
        id
        relationType(version: 2)
        isMainStudio
      }
    }
    characters(sort: RELEVANCE) {
      nodes {
        id
        name {
          first
          middle
          last
          full
          native
          alternative
          alternativeSpoiler
          userPreferred
        }
        image {
          large
          medium
        }
        description(asHtml: true)
        siteUrl
      }
    }
    staff(sort: RELEVANCE) {
      nodes {
        id
        name {
          first
          middle
          last
          full
          native
          alternative
          userPreferred
        }
        languageV2
        gender
        siteUrl
        favourites
      }
    }
    studios(sort: FAVOURITES) {
      nodes {
        id
        name
        isAnimationStudio
        media(sort: POPULARITY) {
          nodes {
            id
            idMal
            title {
              romaji
              english
              native
              userPreferred
            }
            type
            format
            status(version: 2)
            description(asHtml: true)
            coverImage {
              extraLarge
              large
              medium
              color
            }
            bannerImage
            averageScore
            meanScore
          }
        }
        siteUrl
        favourites
      }
    }
    isFavourite
    isFavouriteBlocked
    isAdult
    nextAiringEpisode {
      id
      airingAt
      timeUntilAiring
      episode
      mediaId
    }
    externalLinks {
      id
      url
      site
      siteId
      type
      language
      color
      icon
    }
    streamingEpisodes {
      title
      thumbnail
      url
      site
    }
    siteUrl
  }
}
"#;
