use serde::{Deserialize, Serialize};

use crate::helpers::get_environment;
use crate::models::users::preferences::Theme;

const SITE_NAME: &str = "Community";
const DESCRIPTION: &str =
    "Community is a social media platform for communities.";

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct OpenGraphImage<'r> {
    pub url: &'r str,
    pub alt: &'r str,
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct OpenGraphMetadata<'r> {
    pub title: &'r str,
    pub description: &'r str,
    pub image: Option<OpenGraphImage<'r>>,
    pub url: &'r str,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SeoMetadata<'r> {
    pub title: &'r str,
    pub description: &'r str,
    pub open_graph: Option<OpenGraphMetadata<'r>>,
    pub site_name: &'r str,
    pub language: &'r str,
    pub theme: &'r str,
    pub env: String,
}

#[derive(Default, Clone)]
pub struct SeoMetadataBuilder<'r> {
    title: &'r str,
    description: &'r str,
    open_graph: Option<OpenGraphMetadata<'r>>,
    site_name: &'r str,
    language: Option<&'r str>,
    theme: Option<&'r str>,
    env: String,
}

impl<'r> SeoMetadataBuilder<'r> {
    pub fn new() -> Self {
        Self {
            title: SITE_NAME,
            site_name: SITE_NAME,
            description: DESCRIPTION,
            env: get_environment(),
            ..Self::default()
        }
    }

    pub fn title(mut self, title: &'r str) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: &'r str) -> Self {
        self.description = description;
        self
    }

    pub fn open_graph(mut self, open_graph: OpenGraphMetadata<'r>) -> Self {
        self.open_graph = Some(open_graph);
        self
    }

    pub fn language(mut self, language: &'r str) -> Self {
        self.language = Some(language);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme.into());
        self
    }

    pub fn finalize(self) -> SeoMetadata<'r> {
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

impl<'r> SeoMetadata<'r> {
    pub fn build() -> SeoMetadataBuilder<'r> {
        SeoMetadataBuilder::new()
    }
}
