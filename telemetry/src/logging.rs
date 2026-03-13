// ── Modules ───────────────────────────────────────────────────────────────────

mod log;
mod recorder;
mod router;

pub use self::{
    log::{LogLevel, LogValue, LogEvent, Log},
    recorder::Recorder,
    router::{Router, RouterHandle, RouterReference},
};

// ── Aliases ───────────────────────────────────────────────────────────────────

const ROUTER_CAP: usize = 1024;

const HOT_VEC: usize = 8; 
const HOT_STR: usize = 32; 
pub type HotRecorder = Recorder<HOT_VEC, HOT_STR>;
pub type HotRouter = Router<ROUTER_CAP, HOT_STR>;

const DEFAULT_VEC: usize = 64;
const DEFAULT_STR: usize = 512;
pub type DefaultRecorder = Recorder<DEFAULT_VEC, DEFAULT_STR>;
pub type DefaultRouter = Router<ROUTER_CAP, DEFAULT_STR>;