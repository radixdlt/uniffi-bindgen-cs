/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod gen_cs;

use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use fs_err::File;
pub use gen_cs::generate_bindings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use uniffi_bindgen::{interface::ComponentInterface, BindingsConfig};

#[derive(Parser)]
#[clap(name = "uniffi-bindgen")]
#[clap(version = clap::crate_version!())]
#[clap(propagate_version = true)]
struct Cli {
    /// Directory in which to write generated files. Default is same folder as .udl file.
    #[clap(long, short)]
    out_dir: Option<Utf8PathBuf>,

    /// Path to the optional uniffi config file. If not provided, uniffi-bindgen will try to guess it from the UDL's file location.
    #[clap(long, short)]
    config: Option<Utf8PathBuf>,

    /// Extract proc-macro metadata from cdylib for this crate.
    #[clap(long)]
    lib_file: Option<Utf8PathBuf>,

    /// Pass in a cdylib path rather than a UDL file
    #[clap(long = "library", conflicts_with_all = &["config", "lib-file"], requires = "out-dir")]
    library_mode: bool,

    /// When `--library` is passed, only generate bindings for one crate
    #[clap(long = "crate", requires = "library-mode")]
    crate_name: Option<String>,

    /// Path to the UDL file, or cdylib if `library-mode` is specified
    source: Utf8PathBuf,

    /// Do not try to format the generated bindings.
    #[clap(long, short)]
    no_format: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigRoot {
    #[serde(default)]
    bindings: ConfigBindings,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigBindings {
    #[serde(default)]
    csharp: gen_cs::Config,
}

impl BindingsConfig for ConfigRoot {
    fn update_from_ci(&mut self, ci: &ComponentInterface) {
        self.bindings.csharp.update_from_ci(ci);
    }

    fn update_from_cdylib_name(&mut self, cdylib_name: &str) {
        self.bindings.csharp.update_from_cdylib_name(cdylib_name);
    }

    fn update_from_dependency_configs(&mut self, config_map: HashMap<&str, &Self>) {
        self.bindings.csharp.update_from_dependency_configs(
            config_map
                .iter()
                .map(|(key, config)| (*key, &config.bindings.csharp))
                .collect(),
        );
    }
}

struct BindingGenerator {
    try_format_code: bool,
}

impl uniffi_bindgen::BindingGenerator for BindingGenerator {
    type Config = ConfigRoot;

    fn write_bindings(
        &self,
        ci: &ComponentInterface,
        config: &Self::Config,
        out_dir: &Utf8Path,
    ) -> anyhow::Result<()> {
        let bindings_file = out_dir.join(format!("{}.cs", ci.namespace()));
        let mut f = File::create(&bindings_file)?;

        let mut bindings = generate_bindings(&config.bindings.csharp, &ci)?;

        if self.try_format_code {
            match gen_cs::formatting::format(bindings.clone()) {
                Ok(formatted) => bindings = formatted,
                Err(e) => {
                    println!(
                        "Warning: Unable to auto-format {} using CSharpier (hint: 'dotnet tool install -g csharpier'): {e:?}",
                        bindings_file.file_name().unwrap(),
                    );
                }
            }
        }

        bindings = gen_cs::formatting::add_header(bindings);
        write!(f, "{}", bindings)?;

        Ok(())
    }

    fn check_library_path(&self, library_path: &Utf8Path, cdylib_name: Option<&str>) -> Result<()> {
        if cdylib_name.is_none() {
            bail!("Generate bindings for C# requires a cdylib, but {library_path} was given");
        }
        Ok(())
    }
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.library_mode {
        let out_dir = cli
            .out_dir
            .expect("--out-dir is required when using --library");
        uniffi_bindgen::library_mode::generate_external_bindings(
            BindingGenerator {
                try_format_code: !cli.no_format,
            },
            &cli.source,
            cli.crate_name,
            &out_dir,
        )
        .map(|_| ())
    } else {
        uniffi_bindgen::generate_external_bindings(
            BindingGenerator {
                try_format_code: !cli.no_format,
            },
            &cli.source,
            cli.config.as_deref(),
            cli.out_dir.as_deref(),
            cli.lib_file.as_deref(),
            cli.crate_name.as_deref(),
        )
    }
}
