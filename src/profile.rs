use std::{any::Any, collections::HashMap, fmt::Debug, sync::Arc};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    DuplicateProfileName(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DuplicateProfileName(name) => {
                write!(f, "Profile with name {name} already exists")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DuplicateProfileName(_) => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[allow(private_bounds)]
pub trait Profile: Debug + ProfileEq + Any {}
trait ProfileEq {
    fn eq(&self, other: &dyn Profile) -> bool;
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dev;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Release;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Test;
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
impl Eq for Profiles {}
impl<T: Profile + PartialEq + Eq> ProfileEq for T {
    fn eq(&self, other: &dyn Profile) -> bool {
        match (other as &dyn Any).downcast_ref::<T>() {
            Some(other) => self == other,
            None => false,
        }
    }
}
impl Profile for Dev {}
impl Profile for Release {}
impl Profile for Test {}
impl Profiles {
    pub fn try_default() -> Result<Self> {
        Ok(Profiles::builder()
            .dev("debug")?
            .release("release")?
            .test("test")?
            .build())
    }

    pub fn builder() -> ProfilesBuilder {
        ProfilesBuilder { profiles: HashMap::new() }
    }
}
impl ProfilesBuilder {
    pub fn profile(
        &mut self, name: impl Into<String>, profile: impl Profile,
    ) -> Result<&mut Self> {
        let name = name.into();
        if self.profiles.contains_key(&name) {
            return Err(Error::DuplicateProfileName(format!(
                "Profile with name {name} already exists",
            )));
        }
        self.profiles.insert(name, Arc::new(profile));
        Ok(self)
    }

    pub fn with_mut_profiles(
        &mut self, f: impl FnOnce(&mut HashMap<String, Arc<dyn Profile>>),
    ) -> &mut Self {
        f(&mut self.profiles);
        self
    }

    pub fn dev(&mut self, name: impl Into<String>) -> Result<&mut Self> {
        self.profile(name, Dev)
    }

    pub fn release(&mut self, name: impl Into<String>) -> Result<&mut Self> {
        self.profile(name, Release)
    }

    pub fn test(&mut self, name: impl Into<String>) -> Result<&mut Self> {
        self.profile(name, Test)
    }

    pub fn build(&self) -> Profiles {
        Profiles { profiles: self.profiles.clone() }
    }
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn profiles()
    -> std::result::Result<Profiles, Box<dyn std::error::Error>> {
        Ok(Profiles::builder()
            .profile("debug", Dev)?
            .profile("release", Release)?
            .profile("test", Test)?
            .build())
    }
    #[test]
    fn test_profiles() { profiles().unwrap(); }
}
