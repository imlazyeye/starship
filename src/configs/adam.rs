use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct AdamConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub adam_symbol: &'a str,
    pub gm_symbol: &'a str,
    pub adam_style: &'a str,
    pub gm_style: &'a str,
    pub disabled: bool,
    pub detect_extentions: Vec<&'a str>,
}

impl<'a> Default for AdamConfig<'a> {
    fn default() -> Self {
        AdamConfig {
            format: "via $gm_symbol[$runtime_version]($gm_style)( / $adam_symbol[$adam_version]($adam_style)) ",
            version_format: "v${raw}",
            adam_symbol: "🧟 ",
            gm_symbol: "🪀 ",
            adam_style: "bold yellow",
            gm_style: "bold green",
            disabled: false,
            detect_extentions: vec!["yyp"],
        }
    }
}
