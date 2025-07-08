use std::{any::Any, collections::HashMap, fmt::Debug, sync::Arc};
pub trait Profile: Debug + ProfileEq + Any {}
pub trait ProfileEq {
    fn eq(&self, other: &dyn Profile) -> bool;
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dev;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Release;
#[derive(Debug, Clone)]
pub struct Profiles {
    profiles: HashMap<String, Arc<dyn Profile>>,
}
#[derive(Debug, Clone)]
pub struct ProfilesBuilder {
    profiles: HashMap<String, Arc<dyn Profile>>,
}
impl PartialEq for Profiles {
    fn eq(&self, other: &Self) -> bool {
        self.profiles.iter().all(|(k1, v1)| match other.profiles.get(k1) {
            Some(v2) => v1.eq(v2.as_ref()),
            None => false,
        })
    }
}
impl<T: Profile + PartialEq> ProfileEq for T {
    fn eq(&self, other: &dyn Profile) -> bool {
        match (other as &dyn Any).downcast_ref::<T>() {
            Some(other) => self == other,
            None => false,
        }
    }
}
impl Profile for Dev {}
impl Profile for Release {}
impl Profiles {
    pub fn builder() -> ProfilesBuilder {
        ProfilesBuilder { profiles: HashMap::new() }
    }
}
impl Default for Profiles {
    fn default() -> Self {
        Profiles::builder().dev("debug").release("release").build()
    }
}
impl ProfilesBuilder {
    pub fn profile(mut self, name: &str, profile: impl Profile) -> Self {
        self.profiles.insert(name.to_string(), Arc::new(profile));
        self
    }

    pub fn dev(mut self, name: &str) -> Self {
        self.profiles.insert(name.to_string(), Arc::new(Dev));
        self
    }

    pub fn release(mut self, name: &str) -> Self {
        self.profiles.insert(name.to_string(), Arc::new(Release));
        self
    }

    pub fn build(self) -> Profiles { Profiles { profiles: self.profiles } }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profiles() {
        assert_eq!(
            Profiles::default(),
            Profiles::builder().dev("debug").release("release").build()
        );
        assert_eq!(
            Profiles::builder()
                .profile("debug", Dev)
                .profile("release", Release)
                .build(),
            Profiles::builder().dev("debug").release("release").build()
        );
    }
}
