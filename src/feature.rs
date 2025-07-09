use std::{collections::HashMap, fmt::Display};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    DuplicateFeatureName(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DuplicateFeatureName(name) => {
                write!(f, "Duplicate feature name: {name}")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DuplicateFeatureName(_) => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Feature {}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FeatureBuilder {}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Features {
    features: HashMap<String, Feature>,
}
#[derive(Debug, Clone)]
pub struct FeaturesBuilder {
    features: HashMap<String, Feature>,
}
impl Feature {
    pub fn try_default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> FeatureBuilder { FeatureBuilder {} }
}
impl Features {
    pub fn try_default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> FeaturesBuilder {
        FeaturesBuilder { features: HashMap::new() }
    }
}
impl FeatureBuilder {
    pub fn build(&self) -> Result<Feature> { Ok(Feature {}) }
}
impl FeaturesBuilder {
    pub fn feature(
        &mut self, name: impl Into<String>, feature: Feature,
    ) -> Result<&mut Self> {
        let name = name.into();
        if self.features.contains_key(&name) {
            return Err(Error::DuplicateFeatureName(name.clone()));
        }
        self.features.insert(name, feature);
        Ok(self)
    }

    pub fn with_mut_features(
        &mut self, f: impl FnOnce(&mut HashMap<String, Feature>),
    ) -> Result<&mut Self> {
        f(&mut self.features);
        Ok(self)
    }

    pub fn build(&self) -> Result<Features> {
        Ok(Features { features: self.features.clone() })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(crate) fn features() -> Result<Features> { Features::builder().build() }
    #[test]
    fn test_features() { features().unwrap(); }
}
