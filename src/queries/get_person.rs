pub const GET_PERSON: &str = r#"
query ($id: Int) {
 Staff (id: $id) {
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
   image {
     large
     medium
   }
   description(asHtml: true)
   primaryOccupations
   gender
   dateOfBirth {
     year
     month
     day
   }
   dateOfDeath {
     year
     month
     day
   }
   age
   yearsActive
   homeTown
   bloodType
   isFavourite
   isFavouriteBlocked
   siteUrl
   staffMedia(sort: POPULARITY) {
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
       }
       id
       staffRole
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
   characterMedia(sort: POPULARITY) {
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
       }
       id
       characters {
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
   }
   favourites
   modNotes
 }
}
"#;
