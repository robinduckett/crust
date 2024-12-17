use std::fmt::Display;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Default)]
pub struct MNGAssetLoader;

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Debug)]
pub enum MNGAssetLoaderError {
    Io(std::io::Error),
    Parse(nom::Err<String>),
}

#[derive(Debug, Default, Asset, TypePath)]
pub struct MNGFile;

impl std::error::Error for MNGAssetLoaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MNGAssetLoaderError::Io(e) => Some(e),
            MNGAssetLoaderError::Parse(e) => Some(e),
        }
    }
}

impl Display for MNGAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AssetLoader for MNGAssetLoader {
    type Asset = MNGFile;
    type Settings = ();
    type Error = MNGAssetLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Ok(MNGFile)
    }

    fn extensions(&self) -> &[&str] {
        &["mng", "MNG"]
    }
}
