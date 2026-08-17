pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedbackItem {
    /// Whether the user liked the generated item.
    #[serde(default)]
    pub thumbs_up: bool,
    /// The feedback text provided by the user.
    #[serde(default)]
    pub feedback: String,
    /// Whether the user provided emotions.
    #[serde(default)]
    pub emotions: bool,
    /// Whether the user thinks the clone is inaccurate.
    #[serde(default)]
    pub inaccurate_clone: bool,
    /// Whether the user thinks there are glitches in the audio.
    #[serde(default)]
    pub glitches: bool,
    /// Whether the user thinks the audio quality is good.
    #[serde(default)]
    pub audio_quality: bool,
    /// Whether the user provided other feedback.
    #[serde(default)]
    pub other: bool,
    /// The review status of the item. Defaults to 'not_reviewed'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
}

impl FeedbackItem {
    pub fn builder() -> FeedbackItemBuilder {
        <FeedbackItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedbackItemBuilder {
    thumbs_up: Option<bool>,
    feedback: Option<String>,
    emotions: Option<bool>,
    inaccurate_clone: Option<bool>,
    glitches: Option<bool>,
    audio_quality: Option<bool>,
    other: Option<bool>,
    review_status: Option<String>,
}

impl FeedbackItemBuilder {
    pub fn thumbs_up(mut self, value: bool) -> Self {
        self.thumbs_up = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn emotions(mut self, value: bool) -> Self {
        self.emotions = Some(value);
        self
    }

    pub fn inaccurate_clone(mut self, value: bool) -> Self {
        self.inaccurate_clone = Some(value);
        self
    }

    pub fn glitches(mut self, value: bool) -> Self {
        self.glitches = Some(value);
        self
    }

    pub fn audio_quality(mut self, value: bool) -> Self {
        self.audio_quality = Some(value);
        self
    }

    pub fn other(mut self, value: bool) -> Self {
        self.other = Some(value);
        self
    }

    pub fn review_status(mut self, value: impl Into<String>) -> Self {
        self.review_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedbackItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thumbs_up`](FeedbackItemBuilder::thumbs_up)
    /// - [`feedback`](FeedbackItemBuilder::feedback)
    /// - [`emotions`](FeedbackItemBuilder::emotions)
    /// - [`inaccurate_clone`](FeedbackItemBuilder::inaccurate_clone)
    /// - [`glitches`](FeedbackItemBuilder::glitches)
    /// - [`audio_quality`](FeedbackItemBuilder::audio_quality)
    /// - [`other`](FeedbackItemBuilder::other)
    pub fn build(self) -> Result<FeedbackItem, BuildError> {
        Ok(FeedbackItem {
            thumbs_up: self.thumbs_up.ok_or_else(|| BuildError::missing_field("thumbs_up"))?,
            feedback: self.feedback.ok_or_else(|| BuildError::missing_field("feedback"))?,
            emotions: self.emotions.ok_or_else(|| BuildError::missing_field("emotions"))?,
            inaccurate_clone: self.inaccurate_clone.ok_or_else(|| BuildError::missing_field("inaccurate_clone"))?,
            glitches: self.glitches.ok_or_else(|| BuildError::missing_field("glitches"))?,
            audio_quality: self.audio_quality.ok_or_else(|| BuildError::missing_field("audio_quality"))?,
            other: self.other.ok_or_else(|| BuildError::missing_field("other"))?,
            review_status: self.review_status,
        })
    }
}
