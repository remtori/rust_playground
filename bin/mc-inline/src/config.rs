use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use serde::Deserialize;

pub struct RunOptions {
    pub is_watch: bool,
    pub base_dir: PathBuf,
    pub paths: BTreeMap<usize, Vec<PathBuf>>,
}

impl RunOptions {
    pub fn new() -> Result<Self, anyhow::Error> {
        let input_args = &[
            clap::Arg::with_name("dir")
                .help("Path to datapack directory")
                .short("d")
                .long("dir")
                .takes_value(true),
            clap::Arg::with_name("include")
                .help("Glob pattern of file to preprocessed")
                .short("i")
                .long("include")
                .multiple(true)
                .takes_value(true),
            clap::Arg::with_name("exclude")
                .help("Pattern of file to exclude/ignore")
                .short("e")
                .long("exclude")
                .multiple(true)
                .takes_value(true),
            clap::Arg::with_name("config")
                .help("Path to .mcinline.json, by default we will try to look for this file in CWD and its parent")
                .long_help(
"Path to .mcinline.json, by default we will try to look for this file in CWD and its parent

This config file can has up to 3 array field: \"order\", \"include\" and \"exclude\"
The include, exclude field and --include, --exclude args is mutually exclusive

\"order\" field contains a list of glob pattern that denote the processing order
File that does not match any pattern in the order will be executed
with any order AFTER all the file that does match"
                )
                .short("c")
                .long("config")
                .takes_value(true)
        ];

        let matches = clap::App::new("MC Inline")
            .version("0.0.1")
            .about("A preprocessor for .mcfunction that help with code generation and readability")
            .settings(&[
                clap::AppSettings::ArgRequiredElseHelp,
                clap::AppSettings::DeriveDisplayOrder,
                clap::AppSettings::SubcommandRequiredElseHelp,
            ])
            .subcommand(
                clap::SubCommand::with_name("compile")
                    .about("Run the preprocessor on all input files")
                    .args(input_args),
            )
            .subcommand(
                clap::SubCommand::with_name("watch")
                    .about("Run the preprocessor on all input files then watch and run on changes")
                    .args(input_args),
            )
            .get_matches();

        let (base_dir, config, is_watch) = {
            let (base_dir, include, exclude, config) = match matches.subcommand() {
                ("compile", Some(matches)) | ("watch", Some(matches)) => (
                    matches.value_of("dir").map(PathBuf::from),
                    matches.values_of("include"),
                    matches.values_of("exclude"),
                    matches.value_of("config"),
                ),
                _ => unreachable!(),
            };

            let mut config = ConfigFile::new(base_dir.clone(), config.map(PathBuf::from))?
                .unwrap_or(ConfigFile::default());

            if config.include.is_some() && include.is_some() {
                println!(
                    "Config file \"include\" field and --include, --dir is mutually exclusive"
                );
                std::process::exit(1);
            }

            if config.exclude.is_some() && exclude.is_some() {
                println!("Config file \"exclude\" field and --exclude is mutually exclusive");
                std::process::exit(1);
            }

            if let Some(include) = include {
                config
                    .include
                    .replace(include.map(ToOwned::to_owned).collect());
            }

            if config.include.is_none() && base_dir.is_some() {
                config.include.replace(vec!["**/*.mcfunction".to_owned()]);
            }

            if let Some(exclude) = exclude {
                config
                    .exclude
                    .replace(exclude.map(ToOwned::to_owned).collect());
            }

            let base_dir =
                base_dir.unwrap_or_else(|| std::env::current_dir().expect("Can not access CWD"));

            let is_watch = matches.subcommand_name().unwrap().eq("watch");

            (base_dir, config, is_watch)
        };

        let paths = {
            let mut unique_paths = HashSet::new();

            let mut patterns = Vec::new();

            if let Some(v) = &config.exclude {
                patterns.extend(v.iter().map(|e| format!("!{}", e)));
            }

            if let Some(v) = config.include {
                patterns.extend(v.into_iter());
            }

            for entry in globwalk::GlobWalkerBuilder::from_patterns(&base_dir, &patterns).build()? {
                let entry = entry?;
                if entry.metadata()?.is_file() {
                    unique_paths.insert(entry.path().to_path_buf());
                }
            }

            let mut order = Vec::new();
            if let Some(patterns) = &config.order {
                for pattern in patterns {
                    let mut builder = ignore::overrides::OverrideBuilder::new(&base_dir);
                    builder.add(pattern)?;
                    order.push(builder.build()?);
                }
            }

            let mut out = BTreeMap::new();
            unique_paths.into_iter().for_each(|path| {
                let priority = order
                    .iter()
                    .enumerate()
                    .find_map(|(idx, pattern)| {
                        if pattern.matched(&path, false).is_whitelist() {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(usize::MAX);

                out.entry(priority).or_insert_with(Vec::new).push(path);
            });

            out
        };

        Ok(Self {
            is_watch,
            base_dir,
            paths,
        })
    }
}

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    order: Option<Vec<String>>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl ConfigFile {
    fn new(
        base_dir: Option<PathBuf>,
        path: Option<PathBuf>,
    ) -> Result<Option<ConfigFile>, anyhow::Error> {
        if let Some(mut path) = path {
            // If path to config file is relative, make its relative to base_dir
            if path.is_relative() {
                if let Some(mut base) = base_dir {
                    base.push(&path);
                    path = base;
                }
            }

            return Ok(Some(serde_json::from_reader(std::fs::File::open(path)?)?));
        }

        let mcinline = PathBuf::from(".mcinline.json");

        let mut path =
            base_dir.unwrap_or_else(|| std::env::current_dir().expect("Can not access CWD"));

        loop {
            path.push(&mcinline);
            if path.exists() {
                return Ok(Some(serde_json::from_reader(std::fs::File::open(path)?)?));
            }

            path.pop();
            if !path.pop() {
                return Ok(None);
            }
        }
    }
}
