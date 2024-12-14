use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext, RenderAssetUsages},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use nom::{
    multi::count,
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};
use std::fmt::Display;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct S16ImageInfo {
    pub offset: u32,
    pub width: u16,
    pub height: u16,
    pub image: Handle<Image>,
}

#[derive(Debug, Default)]
pub enum S16ImageFormat {
    #[default]
    Rgb555 = 0,
    Rgb565 = 1,
}

#[derive(Debug, Asset, TypePath, Default)]
#[allow(dead_code)]
pub struct S16Image {
    pub name: String,
    pub images: Vec<S16ImageInfo>,
    pub format: S16ImageFormat,
    pub image_count: u16,
}

impl From<u32> for S16ImageFormat {
    fn from(value: u32) -> Self {
        match value {
            0 => S16ImageFormat::Rgb555,
            1 => S16ImageFormat::Rgb565,
            _ => panic!("Invalid S16 image format"),
        }
    }
}

#[derive(Default)]
pub struct S16AssetLoader;

#[non_exhaustive]
#[derive(Debug)]
pub enum S16AssetLoaderError {
    Io(std::io::Error),
    Parse(nom::Err<String>),
}

impl std::error::Error for S16AssetLoaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            S16AssetLoaderError::Io(e) => Some(e),
            S16AssetLoaderError::Parse(e) => Some(e),
        }
    }
}

impl Display for S16AssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl S16Image {
    pub fn from_bytes<'a>(
        buffer: &'a [u8],
        load_context: &mut LoadContext<'_>,
    ) -> IResult<&'a [u8], S16Image> {
        let mut images = vec![];

        let name = load_context
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let (bytes, (format, number_of_images)) = tuple((le_u32, le_u16))(buffer)?;
        let (bytes, bitmap_offsets) =
            count(tuple((le_u32, le_u16, le_u16)), number_of_images as usize)(bytes)?;

        for (offset_index, &bitmap) in bitmap_offsets.iter().enumerate() {
            let (offset, width, height) = bitmap;

            let dst_bitmap_size = width as usize * height as usize * 4;
            let src_bitmap =
                &buffer[offset as usize..(width as usize * height as usize * 2 + offset as usize)];
            let mut buffer = vec![0u8; dst_bitmap_size];

            let (bytes, pixels) = count(le_u16, width as usize * height as usize)(src_bitmap)?;
            let mut index = 0;
            for pixel in pixels {
                let (red, green, blue) = match format.into() {
                    S16ImageFormat::Rgb565 => (
                        ((pixel & 0xf800) >> 8) as u8,
                        ((pixel & 0x07e0) >> 3) as u8,
                        ((pixel & 0x001f) << 3) as u8,
                    ),
                    S16ImageFormat::Rgb555 => (
                        ((pixel & 0x7c00) >> 7) as u8,
                        ((pixel & 0x03e0) >> 2) as u8,
                        ((pixel & 0x001f) << 3) as u8,
                    ),
                };

                buffer[index] = red;
                buffer[index + 1] = green;
                buffer[index + 2] = blue;
                buffer[index + 3] = if red as u32 + green as u32 + blue as u32 == 0 {
                    0
                } else {
                    255
                };
                index += 4;
            }

            assert!(bytes.is_empty());

            let image = Image::new_fill(
                Extent3d {
                    width: width as u32,
                    height: height as u32,
                    ..Default::default()
                },
                TextureDimension::D2,
                &buffer,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );

            let image_handle = load_context.add_labeled_asset(format!("{}", offset_index), image);

            images.push(S16ImageInfo {
                offset,
                width,
                height,
                image: image_handle,
            });
        }

        Ok((
            bytes,
            S16Image {
                name,
                images,
                format: format.into(),
                image_count: number_of_images,
            },
        ))
    }
}

impl AssetLoader for S16AssetLoader {
    type Asset = S16Image;
    type Settings = ();
    type Error = S16AssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(S16AssetLoaderError::Io)?;

        match S16Image::from_bytes(&bytes, load_context) {
            Ok((_, image)) => Ok(image),
            Err(e) => Err(S16AssetLoaderError::Parse(
                e.map(|e| e.code.description().to_string()),
            )),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["s16", "S16"]
    }
}
