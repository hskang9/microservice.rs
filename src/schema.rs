table! {
    recipes (id) {
        id -> Int4,
        name -> Text,
        author -> Text,
        description -> Text,
        ingredients -> Array<Text>,
        method -> Array<Text>,
        url -> Text,
    }
}
