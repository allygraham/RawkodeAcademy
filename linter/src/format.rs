use chrono::{DateTime, Duration, Utc};
use hcl::ser::LabeledBlock;
use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;

const DURATION_REGEX: &str = r"([0-9]{1,2}):([0-9]{1,2}):?([0-9]{1,2})?";

fn chapter_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    let regex = Regex::new(DURATION_REGEX)
        .map_err(|_| serde::de::Error::custom("Cannot create regex for Duration"))?;

    let captures = regex
        .captures(value.as_str())
        .ok_or_else(|| serde::de::Error::custom(format!("Cannot parse Duration from {}", value)))?;

    match (captures.get(1), captures.get(2), captures.get(3)) {
        (Some(hours), Some(minutes), Some(seconds)) => {
            let hours = hours.as_str().parse::<i64>().map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse hours from {:?}", hours))
            })?;

            let minutes = minutes.as_str().parse::<i64>().map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse minutes from {:?}", minutes))
            })?;

            let seconds = seconds.as_str().parse::<i64>().map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse seconds from {:?}", seconds))
            })?;

            Ok(Duration::hours(hours) + Duration::minutes(minutes) + Duration::seconds(seconds))
        }
        (Some(minutes), Some(seconds), None) => {
            let minutes = minutes.as_str().parse::<i64>().map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse minutes from {:?}", minutes))
            })?;

            let seconds = seconds.as_str().parse::<i64>().map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse seconds from {:?}", seconds))
            })?;

            Ok(Duration::minutes(minutes) + Duration::seconds(seconds))
        }
        _ => Err(serde::de::Error::custom(format!(
            "Cannot parse Duration from {}",
            value
        ))),
    }
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    #[serde(deserialize_with = "chapter_duration")]
    pub time: Duration,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub show: String,
    pub published_at: DateTime<Utc>,
    pub youtube_id: String,
    pub youtube_category: u16,
    // FIXME: pub links: Vec<Url>,
    pub links: Vec<String>,
    pub chapters: Vec<Chapter>,
}

#[derive(Deserialize, Debug)]
pub struct Episodes {
    pub episode: LabeledBlock<IndexMap<String, Episode>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize() {
        let content = fs::read_to_string("sample_episode.hcl").unwrap();
        let episodes: Episodes = hcl::from_str(&content).unwrap();

        assert_eq!(episodes.episode.len(), 1);

        let episode = episodes.episode.get("BDD Test Automation").unwrap();

        assert_eq!(episode.show, "rawkode-live");
        assert_eq!(
            episode.published_at,
            DateTime::parse_from_rfc3339("2021-01-19T13:00:00+00:00")
                .unwrap()
                .with_timezone(&Utc)
        );
        assert_eq!(episode.youtube_id, "dc6A930goAU");
        assert_eq!(episode.youtube_category, 24);
        assert_eq!(episode.links.len(), 10);
        assert_eq!(episode.links[9], "github.com/bbqsrc/cucumber-rust");

        assert_eq!(episode.chapters.len(), 5);
        assert_eq!(
            episode.chapters[4].time,
            Duration::minutes(15) + Duration::seconds(30)
        );
        assert_eq!(
            episode.chapters[4].title,
            "Writing Acceptance Tests with Cucumber Rust"
        );
    }
}
