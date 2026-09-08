use anyhow::anyhow;
use anyhow::Result;
use clap::Args;
use clap::Parser;
use clap::ValueEnum;
use figment::providers::Format;
use figment::providers::Serialized;
use figment::providers::Toml;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
pub(crate) struct Cli {
    #[arg(long, value_name = "FILE")]
    pub(crate) config: Vec<PathBuf>,

    #[arg(long, required = false)]
    pub(crate) dump_config: bool,

    #[command(flatten)]
    pub(crate) cli_config: CliConfig,
}

#[derive(Deserialize, Debug, Clone, Serialize, Args)]
pub(crate) struct BlockConfig {
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) block_size: Option<u64>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) completion_nsec: Option<u64>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) irq_mode: Option<u32>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hw_queue_depth: Option<u64>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) memory_backed: Option<u32>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) size: Option<u64>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rotational: Option<u32>,
}

impl Default for BlockConfig {
    fn default() -> Self {
        Self {
            block_size: Some(4096),
            completion_nsec: Some(0),
            irq_mode: Some(0),
            hw_queue_depth: Some(64),
            memory_backed: Some(0),
            size: Some(4096),
            rotational: Some(0),
        }
    }
}

#[derive(Args, Deserialize, Debug, Serialize)]
pub(crate) struct CliConfig {
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) samples: Option<u32>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) runtime: Option<u32>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ramp: Option<u32>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) device: Option<String>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) jobcounts: Option<Vec<u32>>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) workloads: Option<Vec<String>>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) queue_depths: Option<Vec<u32>>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) block_sizes: Option<Vec<String>>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) prep: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fio: Option<PathBuf>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) module: Option<String>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) module_args: Option<Vec<String>>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) modprobe: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) insmod: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) module_reload_policy: Option<ModuleReloadPolicy>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) compress: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verify: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) capture: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cpufreq_governor_performance: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_boost_amd: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_boost_intel: Option<bool>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) amd_pstate_fixed_3ghz: Option<bool>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tag: Option<String>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output_path: Option<PathBuf>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote: Option<Url>,

    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) use_hugepages: Option<bool>,

    #[command(flatten)]
    #[serde(flatten)]
    pub(crate) block_cfg: BlockConfig,
}

#[derive(Serialize, Deserialize, ValueEnum, Copy, Clone, Debug)]
pub(crate) enum ModuleReloadPolicy {
    Always,
    Once,
}

#[derive(Deserialize, Debug, Serialize)]
pub(crate) struct Config {
    pub(crate) samples: u32,
    pub(crate) runtime: u32,
    pub(crate) ramp: u32,
    pub(crate) device: String,
    pub(crate) jobcounts: Vec<u32>,
    pub(crate) workloads: Vec<String>,
    pub(crate) queue_depths: Vec<u32>,
    pub(crate) block_sizes: Vec<String>,
    pub(crate) prep: bool,
    pub(crate) fio: PathBuf,
    pub(crate) configure_c_nullblk: bool,
    pub(crate) configfs_rnull: bool,

    #[serde(default)]
    pub(crate) disable_boost_amd: bool,

    #[serde(default)]
    pub(crate) disable_boost_intel: bool,

    #[serde(default)]
    pub(crate) amd_pstate_fixed_3ghz: bool,

    #[serde(default)]
    pub(crate) cpufreq_governor_performance: bool,

    #[serde(default)]
    pub(crate) hipri: bool,

    #[serde(default)]
    pub(crate) module: Option<String>,

    #[serde(default)]
    pub(crate) module_args: Vec<String>,

    #[serde(default)]
    pub(crate) modprobe: bool,

    #[serde(default)]
    pub(crate) insmod: bool,

    pub(crate) module_reload_policy: ModuleReloadPolicy,

    #[serde(default)]
    pub(crate) compress: bool,

    #[serde(default)]
    pub(crate) verify: bool,
    pub(crate) capture: bool,

    #[serde(default)]
    pub(crate) tag: Option<String>,

    #[serde(default)]
    pub(crate) output_path: Option<PathBuf>,

    #[serde(default)]
    pub(crate) remote: Option<Url>,

    #[serde(default)]
    pub(crate) use_hugepages: bool,

    #[serde(flatten)]
    pub(crate) block_cfg: BlockConfig,
}

impl Config {
    pub(crate) fn verify(&self) -> Result<()> {
        if self.insmod && self.modprobe {
            return Err(anyhow!("Cannot set insmod and probe at the same time"));
        }

        if self.module.is_some() && !(self.insmod || self.modprobe) {
            return Err(anyhow!("Missing insmod or modprobe option"));
        }

        if self.compress && !self.capture {
            return Err(anyhow!("Cannot compress without capture"));
        }

        if self.remote.is_some() && !self.compress {
            return Err(anyhow!("Cannot upload without compress"));
        }

        if self.remote.is_some() && !self.capture {
            return Err(anyhow!("Cannot upload without capture"));
        }

        if self.verify
            && self
                .module_args
                .iter()
                .any(|mod_arg| mod_arg.starts_with("nr_devices"))
        {
            return Err(anyhow!(
                "Cannot verify if nr_devices is set since memory_backed would be off"
            ));
        }

        Ok(())
    }

    pub(crate) fn parse() -> Result<Self> {
        let args = Cli::parse();
        let cli_config = args.cli_config;

        let mut fig = figment::Figment::new();
        for file_config in args
            .config
            .into_iter()
            .map(|path| match path.exists() {
                true => Ok(path),
                false => Err(anyhow!("Could not find config file")),
            })
            .map(|res| res.map(Toml::file))
        {
            fig = fig.merge(file_config?);
        }

        let config: Config = fig
            .merge(Serialized::defaults(cli_config))
            .join(Serialized::defaults(Config::default()))
            .extract()?;

        log::info!("Configuration: {config:#?}");

        config.verify()?;

        if args.dump_config {
            std::process::exit(0);
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            samples: 30,
            runtime: 30,
            ramp: 10,
            device: String::from("/dev/null"),
            jobcounts: vec![1],
            workloads: vec![String::from("read")],
            queue_depths: vec![1],
            block_sizes: vec![String::from("4k")],
            prep: Default::default(),
            fio: PathBuf::from("fio"),
            module: Default::default(),
            module_args: Default::default(),
            modprobe: Default::default(),
            insmod: Default::default(),
            module_reload_policy: ModuleReloadPolicy::Always,
            compress: Default::default(),
            verify: Default::default(),
            capture: Default::default(),
            cpufreq_governor_performance: Default::default(),
            tag: None,
            configure_c_nullblk: false,
            configfs_rnull: false,
            output_path: None,
            remote: None,
            hipri: false,
            disable_boost_amd: false,
            disable_boost_intel: false,
            amd_pstate_fixed_3ghz: false,
            use_hugepages: false,
            block_cfg: BlockConfig::default(),
        }
    }
}
