//! Module containing structs to work with languages and translations.
//! To work correctly GraphQL wants to InputObject and OutputObjects to be separate,
//! so TranslationInput and Translation were created.
use std::fmt;
use std::str::FromStr;

use juniper::FieldError;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[graphql(name = "Language", description = "Applicable Languages")]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[graphql(description = "English")]
    En,
    #[graphql(description = "Chinese")]
    Ch,
    #[graphql(description = "German")]
    De,
    #[graphql(description = "Russian")]
    Ru,
    #[graphql(description = "Spanish")]
    Es,
    #[graphql(description = "French")]
    Fr,
    #[graphql(description = "Korean")]
    Ko,
    #[graphql(description = "Portuguese")]
    Po,
    #[graphql(description = "Japanese")]
    Ja,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang = match *self {
            Language::En => "en",
            Language::Ch => "ch",
            Language::De => "de",
            Language::Ru => "ru",
            Language::Es => "es",
            Language::Fr => "fr",
            Language::Ko => "ko",
            Language::Po => "po",
            Language::Ja => "ja",
        };
        write!(f, "{}", lang)
    }
}

impl FromStr for Language {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "en" => Language::En,
            "ch" => Language::Ch,
            "de" => Language::De,
            "ru" => Language::Ru,
            "es" => Language::Es,
            "fr" => Language::Fr,
            "ko" => Language::Ko,
            "po" => Language::Po,
            "ja" => Language::Ja,
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


impl Language {
    pub fn as_vec() -> Vec<LanguageGraphQl> {
        vec![
            Language::En,
            Language::Ch,
            Language::De,
            Language::Ru,
            Language::Es,
            Language::Fr,
            Language::Ko,
            Language::Po,
            Language::Ja,
        ].into_iter()
            .map(|value| LanguageGraphQl::new(value.to_string()))
            .collect()
    }
    
}

#[derive(GraphQLInputObject, Clone, Debug)]
#[graphql(description = "Text with language")]
pub struct TranslationInput {
    #[graphql(description = "Language")]
    pub lang: Language,
    #[graphql(description = "Text")]
    pub text: String,
}

impl Serialize for TranslationInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lang = self.lang.to_string();
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("lang", &lang)?;
        map.serialize_entry("text", &self.text)?;
        map.end()
    }
}

#[derive(GraphQLObject, Clone, Debug)]
#[graphql(description = "Text with language")]
pub struct Translation {
    #[graphql(description = "Language")]
    pub lang: Language,
    #[graphql(description = "Text")]
    pub text: String,
}

impl Translation {
    pub fn new(lang: Language, text: String) -> Self {
        Self { lang, text }
    }
}

impl<'de> Deserialize<'de> for Translation {
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

        struct TranslationVisitor;

        impl<'de> Visitor<'de> for TranslationVisitor {
            type Value = Translation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Translation")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Translation, V::Error>
                where V: SeqAccess<'de>
            {
                let lang = seq.next_element()?
                              .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let text = seq.next_element()?
                               .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Translation::new(lang, text))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Translation, V::Error>
                where V: MapAccess<'de>
            {
                let mut lang = None;
                let mut text = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Lang => {
                            if lang.is_some() {
                                return Err(de::Error::duplicate_field("lang"));
                            }
                            let val = map.next_value()?;
                            let language = Language::from_str(val).map_err(|_| de::Error::missing_field("lang"))?;
                            lang = Some(language);
                        }
                        Field::Text => {
                            if text.is_some() {
                                return Err(de::Error::duplicate_field("text"));
                            }
                            text = Some(map.next_value()?);
                        }
                    }
                }
                let lang = lang.ok_or_else(|| de::Error::missing_field("lang"))?;
                let text = text.ok_or_else(|| de::Error::missing_field("text"))?;
                Ok(Translation::new(lang, text))
            }
        }

        const FIELDS: &'static [&'static str] = &["lang", "text"];
        deserializer.deserialize_struct("Translation", FIELDS, TranslationVisitor)
    }
}



#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct LanguageGraphQl {
    #[graphql(description="ISO 639-1 code")]
    pub iso_code: String,
}

impl LanguageGraphQl {
    pub fn new(iso_code: String) -> Self {
        Self { iso_code}
    }
}
