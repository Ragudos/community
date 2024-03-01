use serde::{Deserialize, Serialize};

use crate::{helpers::get_environment, models::users::preferences::Theme};

const SITE_NAME: &str = "Community";
const DESCRIPTION: &str = "Community is a social media platform for communities.";

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct OpenGraphMetadata {
    pub title: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub url: &'static str,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SeoMetadata {
    pub title: &'static str,
    pub description: &'static str,
    pub open_graph: Option<OpenGraphMetadata>,
    pub site_name: &'static str,
    pub language: &'static str,
    pub theme: &'static str,
    pub env: String
}

#[derive(Default, Clone)]
pub struct SeoMetadataBuilder {
    title: &'static str,
    description: &'static str,
    open_graph: Option<OpenGraphMetadata>,
    site_name: &'static str,
    language: Option<&'static str>,
    theme: Option<&'static str>,
    env: String,
}

impl SeoMetadataBuilder {
    pub fn new() -> Self {
        Self {
            title: SITE_NAME,
            site_name: SITE_NAME,
            description: DESCRIPTION,
            env: get_environment(),
            ..Self::default()
        }
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn open_graph(mut self, open_graph: OpenGraphMetadata) -> Self {
        self.open_graph = Some(open_graph);
        self
    }

    pub fn language(mut self, language: &'static str) -> Self {
        self.language = Some(language);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme.into());
        self
    }

    pub fn finalize(self) -> SeoMetadata {
        SeoMetadata {
            env: self.env,
            title: self.title,
            description: self.description,
            open_graph: self.open_graph,
            site_name: self.site_name,
            language: self.language.unwrap_or("en"),
            theme: self.theme.unwrap_or(Theme::System.into()),
        }
    }
}

impl SeoMetadata {
    pub fn build() -> SeoMetadataBuilder {
        SeoMetadataBuilder::new()
    }
}
