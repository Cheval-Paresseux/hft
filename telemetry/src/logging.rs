mod metrics;
mod log;
mod recorder;
mod router;

pub use metrics::*;
pub use log::{Log, LogEvent, LogLevel, LogValue};
pub use recorder::Recorder;
pub use router::{Reference, Router};



const HOT_VEC_CAP: usize = 8; 
const HOT_STR_CAP: usize = 32; 
pub type HotRecorder = Recorder<HOT_VEC_CAP, HOT_STR_CAP>;
pub type HotRouter = Router<HOT_STR_CAP>;

const DEFAULT_VEC_CAP: usize = 64;
const DEFAULT_STR_CAP: usize = 512;
pub type DefaultRecorder = Recorder<DEFAULT_VEC_CAP, DEFAULT_STR_CAP>;
pub type DefaultRouter = Router<DEFAULT_STR_CAP>;