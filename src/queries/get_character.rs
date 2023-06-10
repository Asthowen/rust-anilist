pub const GET_CHARACTER: &str = r#"
query ($id: Int) {
 Character (id: $id) {
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
   gender
   dateOfBirth {
     year
     month
     day
   }
   age
   bloodType
   isFavourite
   isFavouriteBlocked
   siteUrl
   favourites
   modNotes
 }
}
"#;
