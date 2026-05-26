#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] teeffs_core::Error),

    #[error("operation not supported on 2d grids")]
    Grid2dOnly,

    #[error("operation not supported on 3d grids")]
    Grid3dOnly,

    #[error("operation not supported on infinite grids")]
    GridInfinite,

    #[error("operation not supported by this grid")]
    NotSupported,
}

pub type Result<T> = std::result::Result<T, Error>;
