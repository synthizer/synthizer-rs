//! Bindings to [Synthizer](https://github.com/synthizer/synthizer).  For
//! documentation of the library itself, see [the
//! book](https://synthizer.github.io).  This documentation covers aspects
//! specific to the Rust bindings which map to the C bindings in a relatively
//! straightforward manner
//!
//! # Handles
//!
//!Synthizer [Handle]s are reference-counted pointers which work like `Arc`. All
//! the objects in this library that correspond to Synthizer objects (e.g.
//! [BufferGenerator] but not [CustomStreamDef]) impl [Clone] and internally
//! contain a handle.  So, e.g:
//!
//! ```
//! use synthizer as syz;
//! # fn main() -> syz::Result<()> {
//! #    let _guard = syz::initialize()?;
//! let ctx = syz::Context::new()?;
//! // Refers to the same context.
//! let ctx2 = ctx.clone();
//! #    Ok(())
//! # }
//! ```
//!
//! # Initialization
//!
//! To initialize the library, use either [initialize] or the [LibraryConfig]
//! type.  These will give you a [InitializationGuard] which must be kept alive
//! for the duration of the program.  After the [InitializationGuard] goes out
//! of scope, Synthizer functions all error.
//!
//! # Properties
//!
//! Properties are modeled as `get_property` and `set_property` functions.  For
//! example `get_gain` and `set_gain` for `SYZ_P_GAIN` from the C API.
//!
//! Object properties aren't type checked but error if using an object of the
//! wrong type at runtime.  An internal trait, `ToSyzHandle`, is implemented for
//! all library types.
//!
//! # Common Functionality
//!
//! All Synthizer objects implement a set of common methods on their struct.
//! When the Synthizer manual refers to things like `syz_pause`, this can be
//! found as `.pause()` on all applicable objects.
//!
//! # Userdata
//! Synthizer supports userdata, which can be used to tie application entities
//! to Synthizer objects via `Arc<Any>`.  This is set by e.g.
//! [Handle::set_userdata].  Note that unlike Synthizer itself, the Rust
//! bindings have to put userdata behind a [std::sync::RwLock] to offer thread
//! safety,.
//! # Casting Objects
//!
//! Synthizer itself is modeled as a hierarchy of "classes".  For example
//! [Source] is a "base class" of all sources.  This is handled in Rust via
//! adding a `cast_to` method to all Synthizer types, which can be used to
//! attempt to cast to other object types when possible.  For example [Source3D]
//! to [Source], but also [Handle] to [Source3D].
//!
//! Where this cast is infallible, `From` impls are provided.
//!
//! # Custom Streams
//!
//! Custom streams are possible via [CustomStreamDef] and
//! [register_stream_protocol].  See the documentation on those types for more
//! info.  In general, it is possible to convert anything implementing
//! [std::io::Read] and [std::io::Seek] to a custom stream by implementing
//! [CloseStream] for that type or a wrapper struct thereof.
#[macro_use]
mod common_functionality_macros;
#[macro_use]
mod property_tables;
#[macro_use]
mod handle;

mod angular_panned_source;
mod biquad;
mod buffer;
mod buffer_generator;
mod casting;
mod context;
mod custom_streams;
mod delete_config;
mod direct_source;
mod enums;
mod errors;
mod events;
mod generator;
mod global_echo;
mod global_fdn_reverb;
mod initialization;
mod internal_prelude;
mod noise_generator;
mod routes;
mod scalar_panned_source;
mod source;
mod source_3d;
mod streaming_generator;
mod userdata;
mod version;

pub use angular_panned_source::*;
pub use biquad::*;
pub use buffer::*;
pub use buffer_generator::*;
pub use context::*;
pub use custom_streams::*;
pub use delete_config::*;
pub use direct_source::*;
pub use enums::*;
pub use errors::*;
pub use events::*;
pub use generator::*;
pub use global_echo::*;
pub use global_fdn_reverb::*;
pub use handle::*;
pub use initialization::*;
pub use noise_generator::*;
pub use routes::*;
pub use scalar_panned_source::*;
pub use source::*;
pub use source_3d::*;
pub use streaming_generator::*;
pub use version::*;
