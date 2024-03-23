pub(crate) use clap::{Parser, Subcommand, ArgAction};

pub(crate) fn non_empty_string(s: &str) -> Result<String, &'static str> {
	if s.is_empty() {
		Err("Value cannot be empty")
	} else {
		Ok(s.to_string())
	}
}

/// Tool to manage systemd-boot in a btrfs based, snapper managed system
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	/// Manually specify snapshot
	#[arg(short, long)]
	pub snapshot: Option<u64>,

	/// Manually specify path to ESP
	#[arg(short = 'p', long = "esp-path", value_parser = non_empty_string)]
	pub esp_path: Option<String>,

	/// Manually set architecture
	#[arg(short, long, value_parser = non_empty_string)]
	pub arch: Option<String>,
	
	/// Override entry token
	#[arg(short = 't', long = "entry-token", value_parser = non_empty_string)]
	pub entry_token: Option<String>,
	
	/// Specify Linux kernel file name
	#[arg(short, long, value_parser = non_empty_string)]
	pub image: Option<String>,
	
	/// Do not update UEFI variables
	#[arg(short = 'n', long = "no-variables")]
	pub no_variables: bool,
	
	/// Always regenerate initrd
	#[arg(short = 'r', long = "regenerate-initrd")]
	pub regenerate_initrd: bool,
	
	/// More verbose output
	#[arg(short, long, action = ArgAction::Count)]
	pub verbosity: u8,

	#[command(subcommand)]
	pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
	/// [UI] Open kernel menu
	Kernels {},

	/// [UI] Open snapshots menu
	Snapshots {},
		
	/// [UI] Open entry menu
	Entries {},

	/// Print the detected bootloader
	Bootloader {},

	/// Create boot entry for specified kernel
	#[command(name = "add-kernel")]
	AddKernel {
		kernel_version: String,
	},

	/// Create boot entries for all kernels in SNAPSHOT
	#[command(name = "add-all-kernels")]
	AddAllKernels {},

	/// Create boot entries for all kernels  in SNAPSHOT,
	/// sets --regenerate-initrd
	Mkinitrd {},

	/// Remove boot entry for specified kernel in SNAPSHOT
	#[command(name = "remove-kernel")]
	RemoveKernel {
		kernel_version: String,
	},

	/// Remove boot entries for all kernels in SNAPSHOT
	#[command(name = "remove-all-kernels")]
	RemoveAllKernels {},

	/// List all kernels related to SNAPSHOT
	#[command(name = "list-kernels")]
	ListKernels {},

	/// List all entries related to SNAPSHOT
	#[command(name = "list-entries")]
	ListEntries {},

	/// List all snapshots
	#[command(name = "list-snapshots")]
	ListSnapshots {},

	/// Make SNAPSHOT the default for next boot and install all kernels if needed
	#[command(name = "set-default-snapshot")]
	SetDefaultSnapshot {},

	/// Check whether SNAPSHOT has any kernels registered, ie is potentially bootable
	#[command(name = "is-bootable")]
	IsBootable {},

	/// Install systemd-boot and shim into ESP
	Install {},

	/// Check whether the bootloader in ESP needs updating
	#[command(name = "needs-update")]
	NeedsUpdate {},

	/// Update the bootloader if it's old
	Update {},

	/// Update the bootloader in any case
	#[command(name = "force-update")]
	ForceUpdate {},

	/// Update TPM2 predictions
	#[command(name = "update-predictions")]
	UpdatePredictions {},
}

pub fn parse_args() -> Args {
    Args::parse()
}