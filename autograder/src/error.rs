use crate::cargo_test_output::TestOutput;
use crate::report::{Report, TestReport};
use clap;
use lcov;
use serde_yaml;
use snafu::Snafu;
use std::{io, path::PathBuf};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MyError {
    #[snafu(display("Failed to read from {}: {}", path.display(), source))]
    ReadError { source: io::Error, path: PathBuf },

    #[snafu(display("Failed to write to {}: {}", path.display(), source))]
    WriteError { source: io::Error, path: PathBuf },

    #[snafu(display("Failed to create file {}: {}", path.display(), source))]
    FileCreationError { source: io::Error, path: PathBuf },

    #[snafu(display("Unable to parse yaml to ScoreMap:\n{}\n{}", yaml, source))]
    ScoreMapParseError {
        source: serde_yaml::Error,
        yaml: String,
    },

    #[snafu(display("Unable to serialize struct to json:\n{:?}\n{}", output, source))]
    TestOutputSerializationError {
        source: serde_json::Error,
        output: TestOutput,
    },

    #[snafu(display("Unable to serialize struct to json:\n{:?}\n{}", report, source))]
    TestReportSerializationError {
        source: serde_json::Error,
        report: TestReport,
    },

    #[snafu(display("Unable to serialize struct to json:\n{:?}\n{}", report, source))]
    ReportSerializationError {
        source: serde_json::Error,
        report: Report,
    },

    #[snafu(display("Key {} not found in ScoreMap: ", key))]
    ScoreMapKeyError { key: String },

    #[snafu(display("Bad arg {}: {}", arg, source))]
    Argument { source: clap::Error, arg: String },

    #[snafu(display("Unable to parse lcov string:\n{}", string))]
    LcovReadError { string: String },

    #[snafu(display("{}", msg))]
    AssertionError { msg: String },
}
