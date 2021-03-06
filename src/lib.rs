//! The Test Anything Protocol (TAP) is a plaintext format for expressing test results. It has been around since 1987 when it was invented to help test Perl. With this crate, this wonderfully-useful tool has been brought to Rust!
//!
//! This crate provides the machinery needed for producing and emitting TAP streams. Since the TAP specification requires that TAP be emitted to STDOUT, we don't get fancy with stream interfaces.
//!
//! For working, executable examples, please see the `examples` directory.
//!
//! # Examples
//!
//! The first method for producing a TAP stream is the `TapSuite` mechanism. This will come in handy when you're iterating over a collection and want to `map` it to a collection of `TapTest` results. We supply a `TapSuiteBuilder` and a `TapTestBuilder` to make this as smooth as possible.
//!
//! Behold! The `TapSuite`
//!
//! ```
//! use tap_rust::tap_test_builder::TapTestBuilder;
//! use tap_rust::tap_suite_builder::TapSuiteBuilder;
//!
//! // Cons up a failing test
//! let failing_tap_test = TapTestBuilder::new()
//!     .name("Example TAP test")
//!     .passed(false)
//!     .diagnostics(vec!["This test failed because of X"])
//!     .finalize();
//!
//! // 
//! let tap_suite = TapSuiteBuilder::new()
//!     .name("Example TAP suite")
//!     .tests(vec![failing_tap_test])
//!     .finalize();
//!
//! tap_suite.print();
//! ```
//!
//! The second method uses the `TapWriter` facility and may be thought of as the direct approach. This mechanism allows you to write a semi-customizable TAP stream to STDOUT from anywhere in your program.
//!
//! Behold, the `TapWriter`!
//!
//! ```
//! use tap_rust::tap_writer::TapWriter;
//!
//! let writer = TapWriter::new("Example TAP stream");
//!
//! // Write the plan out. This can come before or after the test results themselves.
//! writer.plan(1, 6);
//!
//! // Give me the name as a diagnostic line
//! writer.name();
//!
//! // Print out some test results
//! writer.ok(1, "Panda");
//! writer.ok(2, "Bamboo");
//! writer.ok(3, "Curry");
//! // This one failed, so explain why with a diagnostic line
//! writer.not_ok(4, "Noodle");
//! writer.diagnostic("The above test failed because of XYZ reason");
//! writer.ok(5, "Tree");
//!
//! // uh oh! something went horribly wrong and we need to stop before
//! // we print out the results from test 6!
//! writer.bail_out_with_message("Destabilized warp core! Can't continue!");
//! ```
//!
//!

/// Global constant for the "ok"
const OK_SYMBOL: &'static str = "ok";
/// Global constant for the "not ok"
const NOT_OK_SYMBOL: &'static str = "not ok";

/// `TapTest` -- The core, representing an individual TAP test.
pub mod tap_test;
/// `TapTestBuilder` -- Helper for creating a `TapTest` using the builder pattern.
pub mod tap_test_builder;
/// `TapSuite` -- A collection of `TapTest` objects renderable into a TAP text stream
pub mod tap_suite;
/// `TapTestBuilder` -- Helper for creating a `TapTestSuite` using the builder pattern
pub mod tap_suite_builder;
/// `TapWriter` -- For writing TAP streams incrementally
pub mod tap_writer;
