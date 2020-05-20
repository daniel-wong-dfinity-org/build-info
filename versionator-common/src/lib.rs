pub use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BuildInfo {
	pub compiler: CompilerVersion,
	pub version_control: Option<VersionControl>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompilerVersion {
	pub version: Version,
	pub commit_hash: Option<String>,
	pub commit_date: Option<String>,
	pub channel: CompilerChannel,
	pub host_triple: String,
	pub target_triple: String,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompilerChannel {
	Dev,
	Nightly,
	Beta,
	Stable,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionControl {
	Git {
		commit_hash: String,
		dirty: bool,
		name: Option<String>,
	},
}
