//! Module containing structs to work with languages and translations.
//! To work correctly GraphQL wants to InputObject and OutputObjects to be separate,
//! so TranslatedTextInput and TranslatedText were created.
use std::fmt;
use std::str::FromStr;

use juniper::FieldError;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[graphql(name = "Language", description = "Applicable Languages")]
pub enum Language {
    #[graphql(description = "English")]
    #[serde(rename = "en")]
    English,
    #[graphql(description = "Chinese")]
    #[serde(rename = "ch")]
    Chinese,
    #[graphql(description = "German")]
    #[serde(rename = "ge")]
    German,
    #[graphql(description = "Russian")]
    #[serde(rename = "ru")]
    Russian,
    #[graphql(description = "Spanish")]
    #[serde(rename = "es")]
    Spanish,
    #[graphql(description = "French")]
    #[serde(rename = "fr")]
    French,
    #[graphql(description = "Korean")]
    #[serde(rename = "ko")]
    Korean,
    #[graphql(description = "Portuguese")]
    #[serde(rename = "po")]
    Portuguese,
    #[graphql(description = "Japanese")]
    #[serde(rename = "ja")]
    Japanese,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang = match *self {
            Language::English => "en",
            Language::Chinese => "ch",
            Language::German => "ge",
            Language::Russian => "ru",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Korean => "ko",
            Language::Portuguese => "po",
            Language::Japanese => "ja",
        };
        write!(f, "{}", lang)
    }
}

impl FromStr for Language {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "english" => Language::English,
            "chinese" => Language::Chinese,
            "german" => Language::German,
            "russian" => Language::Russian,
            "spanish" => Language::Spanish,
            "french" => Language::French,
            "korean" => Language::Korean,
            "portuguese" => Language::Portuguese,
            "japanese" => Language::Japanese,
            _ => {
                return Err(FieldError::new(
                    "Unknown service",
                    graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve service name. Unknown service: '{}'", s)
                        }}),
                ))
            }
        })
    }
}

#[derive(GraphQLInputObject, Clone, Debug)]
#[graphql(description = "Text with language")]
pub struct TranslatedTextInput {
    #[graphql(description = "Language")]
    pub lang: Language,
    #[graphql(description = "Text")]
    pub text: String,
}

impl Serialize for TranslatedTextInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lang = self.lang.to_string();
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&lang, &self.text)?;
        map.end()
    }
}

#[derive(GraphQLObject, Clone, Debug)]
#[graphql(description = "Text with language")]
pub struct TranslatedText {
    #[graphql(description = "Language")]
    pub lang: Language,
    #[graphql(description = "Text")]
    pub text: String,
}

impl TranslatedText {
    pub fn new(lang: Language, text: String) -> Self {
        Self { lang, text }
    }
}

impl<'de> Deserialize<'de> for TranslatedText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Lang,
            Text,
        }

        struct TranslatedTextVisitor;

        impl<'de> Visitor<'de> for TranslatedTextVisitor {
            type Value = TranslatedText;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TranslatedText")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TranslatedText, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut lang = None;
                let mut text = None;
                if let Some(key) = map.next_key()? {
                    lang = Some(Language::from_str(key).map_err(|_| de::Error::missing_field("lang"))?);
                    text = Some(map.next_value()?);
                }
                let lang = lang.ok_or_else(|| de::Error::missing_field("lang"))?;
                let text = text.ok_or_else(|| de::Error::missing_field("text"))?;
                Ok(TranslatedText::new(lang, text))
            }
        }

        const FIELDS: &'static [&'static str] = &["lang", "text"];
        deserializer.deserialize_struct("TranslatedText", FIELDS, TranslatedTextVisitor)
    }
}
