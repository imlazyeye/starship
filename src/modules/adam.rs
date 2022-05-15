use super::{Context, Module, ModuleConfig};
use crate::configs::adam::AdamConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils;

/// Creates a module with the current Adam version, as well as the GM runtime it is using
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("adam");
    let config = AdamConfig::try_load(module.config);

    let is_adam_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extentions)
        .is_match()
        && get_adam_version(context, &config).is_some();

    if !is_adam_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "adam_symbol" => Some(config.adam_symbol),
                _ => None,
            })
            .map_meta(|var, _| match var {
                "gm_symbol" => Some(config.gm_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "adam_style" => Some(Ok(config.adam_style)),
                _ => None,
            })
            .map_style(|variable| match variable {
                "gm_style" => Some(Ok(config.gm_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "adam_version" => get_adam_version(context, &config).map(Ok),
                _ => None,
            })
            .map(|variable| match variable {
                "runtime_version" => get_runtime_version(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `adam`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_adam_version(context: &Context, config: &AdamConfig) -> Option<String> {
    let cmd_output = context.exec_cmd("adam", &["--version"])?;
    let adam_version = cmd_output.stdout.replace("adam ", "").replace('\n', "");
    VersionFormatter::format_version(&adam_version, config.version_format).ok()
}

fn get_runtime_version(context: &Context) -> Option<String> {
    let file_contents = utils::read_file(&context.current_dir.join(".adam.toml")).ok()?;
    let adam_toml: toml::Value = toml::from_str(&file_contents).ok()?;
    adam_toml
        .get("runtime")
        .map(|v| v.to_string().replace('"', ""))
}
