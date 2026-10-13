//! Desktop Pet 使用者設定：五項必填，其他皆為可選覆寫。
//! 生成素材屬於另一個系統管理的 manifest，不會寫回使用者 pet.toml。
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;
use uuid::Uuid;

pub const PET_WIDTH: u32 = 200;
pub const PET_HEIGHT: u32 = 200;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("invalid profile field: {0}")]
    Invalid(&'static str),
    #[error("invalid pet id")]
    InvalidId,
    #[error("failed to parse profile: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AppearanceOverride {
    #[serde(default, rename = "顏色", alias = "color")]
    pub color: Option<String>,
    #[serde(default, rename = "風格", alias = "style")]
    pub style: Option<String>,
    #[serde(default, rename = "品種", alias = "breed")]
    pub breed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AnimationOverride {
    #[serde(default, rename = "動作", alias = "actions")]
    pub actions: Option<Vec<String>>,
    #[serde(default, rename = "速度", alias = "speed")]
    pub speed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PetProfile {
    #[serde(rename = "名字", alias = "name")]
    pub name: String,
    #[serde(rename = "種類", alias = "species")]
    pub species: String,
    #[serde(rename = "喜好", alias = "likes")]
    pub likes: Vec<String>,
    #[serde(rename = "個性", alias = "personality")]
    pub personality: Vec<String>,
    #[serde(rename = "興趣", alias = "interests")]
    pub interests: Vec<String>,
    #[serde(default, rename = "外觀", alias = "appearance")]
    pub appearance: Option<AppearanceOverride>,
    #[serde(default, rename = "動畫", alias = "animation")]
    pub animation: Option<AnimationOverride>,
}

fn valid_item(text: &str) -> bool {
    !text.trim().is_empty() && text.len() <= 256 && !text.contains('\0')
}

fn valid_items(items: &[String]) -> bool {
    items.len() <= 32 && items.iter().all(|x| valid_item(x))
}

impl PetProfile {
    pub fn parse(input: &str) -> Result<Self, ProfileError> {
        let value: Self = toml::from_str(input)?;
        value.validate()?;
        Ok(value)
    }

    /// TOML 1.0-compatible keys, including quoted Chinese table names.
    pub fn user_toml(&self) -> Result<String, serde_json::Error> {
        fn line(
            out: &mut String,
            key: &str,
            value: &impl Serialize,
        ) -> Result<(), serde_json::Error> {
            out.push_str(&format!(
                "{} = {}\n",
                serde_json::to_string(key)?,
                serde_json::to_string(value)?
            ));
            Ok(())
        }
        let mut out = String::new();
        line(&mut out, "名字", &self.name)?;
        line(&mut out, "種類", &self.species)?;
        line(&mut out, "喜好", &self.likes)?;
        line(&mut out, "個性", &self.personality)?;
        line(&mut out, "興趣", &self.interests)?;
        if let Some(appearance) = &self.appearance {
            out.push_str("\n[\"外觀\"]\n");
            for (key, value) in [
                ("顏色", &appearance.color),
                ("風格", &appearance.style),
                ("品種", &appearance.breed),
            ] {
                if let Some(value) = value {
                    line(&mut out, key, value)?;
                }
            }
        }
        if let Some(animation) = &self.animation {
            out.push_str("\n[\"動畫\"]\n");
            if let Some(actions) = &animation.actions {
                line(&mut out, "動作", actions)?;
            }
            if let Some(speed) = &animation.speed {
                line(&mut out, "速度", speed)?;
            }
        }
        Ok(out)
    }
    pub fn validate(&self) -> Result<(), ProfileError> {
        if !valid_item(&self.name) {
            return Err(ProfileError::Invalid("名字"));
        }
        if !valid_item(&self.species) {
            return Err(ProfileError::Invalid("種類"));
        }
        if !valid_items(&self.likes) {
            return Err(ProfileError::Invalid("喜好"));
        }
        if !valid_items(&self.personality) {
            return Err(ProfileError::Invalid("個性"));
        }
        if !valid_items(&self.interests) {
            return Err(ProfileError::Invalid("興趣"));
        }
        if let Some(v) = &self.appearance {
            for item in [&v.color, &v.style, &v.breed] {
                if item.as_ref().is_some_and(|s| !valid_item(s)) {
                    return Err(ProfileError::Invalid("外觀"));
                }
            }
        }
        if let Some(v) = &self.animation
            && (v.actions.as_ref().is_some_and(|items| !valid_items(items))
                || v.speed.as_ref().is_some_and(|s| !valid_item(s)))
        {
            return Err(ProfileError::Invalid("動畫"));
        }
        Ok(())
    }

    pub fn requested_actions(&self) -> BTreeSet<String> {
        self.animation
            .as_ref()
            .and_then(|a| a.actions.as_ref())
            .map(|x| x.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetManifest {
    pub schema_version: u16,
    pub pet_id: Uuid,
    pub original_image_width: u32,
    pub original_image_height: u32,
    pub display_width: u32,
    pub display_height: u32,
    pub asset_version: u32,
    pub generated_frames: Vec<String>,
    pub profile_hash: String,
    pub sha256: String,
    pub file: String,
    pub columns: u8,
    pub rows: Vec<String>,
    pub frames_per_row: u8,
}

impl AssetManifest {
    pub fn generated(mut self, version: u32, profile_hash: String, sha256: String) -> Self {
        self.asset_version = version;
        self.profile_hash = profile_hash;
        self.sha256 = sha256;
        self.file = format!("generated/{version}/sheet.png");
        self
    }
    pub fn new(pet_id: Uuid, image_width: u32, image_height: u32) -> Self {
        Self {
            schema_version: 1,
            pet_id,
            original_image_width: image_width,
            original_image_height: image_height,
            display_width: PET_WIDTH,
            display_height: PET_HEIGHT,
            asset_version: 1,
            generated_frames: ["idle", "moving", "happy", "learning"]
                .into_iter()
                .flat_map(|row| (0..4).map(move |frame| format!("{row}/{frame}")))
                .collect(),
            profile_hash: String::new(),
            sha256: String::new(),
            file: String::new(),
            columns: 4,
            rows: ["idle", "moving", "happy", "learning"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            frames_per_row: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MINIMAL: &str = r#""名字" = "小啾"
"種類" = "鳥"
"喜好" = ["唱歌", "水果"]
"個性" = ["吵雜"]
"興趣" = ["音樂", "探索"]
"#;

    #[test]
    fn five_fields_are_enough_without_advanced_settings() {
        let profile = PetProfile::parse(MINIMAL).unwrap();
        assert_eq!(profile.name, "小啾");
        assert!(profile.appearance.is_none());
        assert!(profile.animation.is_none());
    }

    #[test]
    fn optional_override_has_priority_without_forcing_defaults() {
        let profile = PetProfile::parse(&format!(
            "{MINIMAL}\n[\"外觀\"]\n\"顏色\" = \"綠色\"\n[\"動畫\"]\n\"動作\" = [\"拍翅膀\"]\n"
        ))
        .unwrap();
        assert_eq!(
            profile.appearance.as_ref().unwrap().color.as_deref(),
            Some("綠色")
        );
        assert!(profile.requested_actions().contains("拍翅膀"));
    }

    #[test]
    fn image_size_is_independent_from_display_bounds() {
        let manifest = AssetManifest::new(Uuid::new_v4(), 2048, 1536);
        assert_eq!(
            (manifest.display_width, manifest.display_height),
            (200, 200)
        );
        assert_eq!(manifest.original_image_width, 2048);
    }

    #[test]
    fn missing_required_fields_fail() {
        assert!(PetProfile::parse(r#""名字" = "小啾""#).is_err());
        assert!(PetProfile::parse(&MINIMAL.replace(r#""種類" = "鳥""#, r#""種類" = """#)).is_err());
    }
}
