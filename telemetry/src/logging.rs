mod metrics;
mod sinks;
mod log;
mod recorder;
mod router;

pub use metrics::*;
pub use sinks::*;
pub use log::{Log, LogEvent, LogLevel, LogValue};
pub use recorder::Recorder;
pub use router::{Reference, Router};


const ROUTER_CAP: usize = 1024;

const HOT_VEC: usize = 8; 
const DEFAULT_VEC: usize = 64;

const HOT_STR: usize = 32; 
const DEFAULT_STR: usize = 512;


pub type HotRecorder = Recorder<HOT_VEC, HOT_STR>;
pub type DefaultRecorder = Recorder<DEFAULT_VEC, DEFAULT_STR>;

pub type HotRouter = Router<ROUTER_CAP, HOT_STR>;
pub type DefaultRouter = Router<ROUTER_CAP, DEFAULT_STR>;


