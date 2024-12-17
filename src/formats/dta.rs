use std::fmt::Display;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

#[derive(Default)]
pub struct DTAAssetLoader;

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Debug)]
pub enum DTAAssetLoaderError {
    Io(std::io::Error),
    Parse(nom::Err<String>),
}

#[derive(Debug, Default, Asset, TypePath)]
pub struct DTAFile;

impl std::error::Error for DTAAssetLoaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DTAAssetLoaderError::Io(e) => Some(e),
            DTAAssetLoaderError::Parse(e) => Some(e),
        }
    }
}

impl Display for DTAAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AssetLoader for DTAAssetLoader {
    type Asset = DTAFile;
    type Settings = ();
    type Error = DTAAssetLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Ok(DTAFile)
    }

    fn extensions(&self) -> &[&str] {
        &["dta", "DTA"]
    }
}
