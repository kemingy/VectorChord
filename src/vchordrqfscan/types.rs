use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct VchordrqfscanInternalBuildOptions {
    #[serde(default = "VchordrqfscanInternalBuildOptions::default_lists")]
    #[validate(length(min = 1, max = 8), custom(function = VchordrqfscanInternalBuildOptions::validate_lists))]
    pub lists: Vec<u32>,
    #[serde(default = "VchordrqfscanInternalBuildOptions::default_spherical_centroids")]
    pub spherical_centroids: bool,
    #[serde(default = "VchordrqfscanInternalBuildOptions::default_build_threads")]
    #[validate(range(min = 1, max = 255))]
    pub build_threads: u16,
}

impl VchordrqfscanInternalBuildOptions {
    fn default_lists() -> Vec<u32> {
        vec![1000]
    }
    fn validate_lists(lists: &[u32]) -> Result<(), ValidationError> {
        if !lists.is_sorted() {
            return Err(ValidationError::new("`lists` should be in ascending order"));
        }
        if !lists.iter().all(|x| (1..=1 << 24).contains(x)) {
            return Err(ValidationError::new("list is too long or too short"));
        }
        Ok(())
    }
    fn default_spherical_centroids() -> bool {
        false
    }
    fn default_build_threads() -> u16 {
        1
    }
}

impl Default for VchordrqfscanInternalBuildOptions {
    fn default() -> Self {
        Self {
            lists: Self::default_lists(),
            spherical_centroids: Self::default_spherical_centroids(),
            build_threads: Self::default_build_threads(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct VchordrqfscanExternalBuildOptions {
    pub table: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum VchordrqfscanBuildOptions {
    Internal(VchordrqfscanInternalBuildOptions),
    External(VchordrqfscanExternalBuildOptions),
}

impl Default for VchordrqfscanBuildOptions {
    fn default() -> Self {
        Self::Internal(Default::default())
    }
}

impl Validate for VchordrqfscanBuildOptions {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use VchordrqfscanBuildOptions::*;
        match self {
            Internal(internal_build) => internal_build.validate(),
            External(external_build) => external_build.validate(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct VchordrqfscanIndexingOptions {
    #[serde(default = "VchordrqfscanIndexingOptions::default_residual_quantization")]
    pub residual_quantization: bool,
    pub build: VchordrqfscanBuildOptions,
}

impl VchordrqfscanIndexingOptions {
    fn default_residual_quantization() -> bool {
        false
    }
}
