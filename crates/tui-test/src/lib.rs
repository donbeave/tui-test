pub mod api;
pub mod assert;
pub mod config;
pub mod diagnostics;
pub mod engine;
pub mod input;
pub mod launch;
pub mod locator_query;
pub mod logger;
pub mod pixel;
pub mod profile;
pub mod record;
pub mod render;
pub mod runtime;
pub mod shell;
pub mod snapshot;
pub mod terminal;
pub mod trace;
pub mod wait;

mod event;
mod session;

pub use api::*;
pub use diagnostics::*;
pub use engine::Engine;
pub use launch::{LaunchEnvMode, LaunchOptions, ResolvedLaunch};
#[cfg(feature = "recording-raster")]
pub use render::offline::{render_snapshot_png, write_snapshot_png, RenderedPng};
pub use render::offline::{
    render_snapshot_svg, write_snapshot_svg, RenderError, RenderEvidence, RenderedSvg,
    SnapshotRenderOptions,
};
pub use render::pinned::{
    ContractFont, CursorPhase, PinnedCacheKey, PinnedCachedFace, PinnedFace, PinnedFontError,
    PinnedFontSet, RenderContract, RASTER_POLICY_PINNED_V1, RASTER_POLICY_SYSTEM_V1,
    RENDER_CONTRACT_VERSION,
};
pub use runtime::{
    global_registry, Locator, LocatorClickOptions, LocatorExpectOptions, LocatorFilterOptions,
    Session, SessionHandle, SessionRegistry,
};
pub use snapshot::{
    SnapshotCell, SnapshotColor, SnapshotCursor, SnapshotCursorShape, SnapshotError,
    SnapshotMetadata, SnapshotOptions, SnapshotParts, SnapshotScope, SnapshotSource,
    SnapshotUnderline, TerminalSnapshot, MAX_CELL_TEXT_BYTES, MAX_LINK_BYTES, MAX_SNAPSHOT_CELLS,
    MAX_SNAPSHOT_DIM, MAX_TITLE_BYTES, SNAPSHOT_SCHEMA_VERSION,
};
pub use terminal::backend::Backend;
pub use wait::{WaitQuietMode, WaitQuietOptions, DEFAULT_QUIET_INTERVAL, DEFAULT_QUIET_TIMEOUT};
