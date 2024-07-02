use crate::js_command_match::constants::NETWORK_ID_ALIASES;

#[derive(Debug, Clone, clap::Parser)]
/// This is a legacy `view` command. Once you run it with the specified arguments, new syntax command will be suggested.
pub struct ViewArgs {
    contract_account_id: String,
    method_name: String,
    #[clap(default_value = "")]
    args: String,
    #[clap(long, aliases = NETWORK_ID_ALIASES, default_value=None)]
    network_id: Option<String>,
    #[clap(allow_hyphen_values = true, num_args = 0..)]
    _unknown_args: Vec<String>,
}

impl ViewArgs {
    pub fn to_cli_args(&self, network_config: String) -> Vec<String> {
        let network_id = self.network_id.clone().unwrap_or(network_config);

        let command = vec![
            "contract".to_string(),
            "call-function".to_string(),
            "as-read-only".to_string(),
            self.contract_account_id.to_owned(),
            self.method_name.to_owned(),
            "text-args".to_string(),
            self.args.to_owned(),
            "network-config".to_string(),
            network_id,
            "now".to_string(),
        ];

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn view_testnet() {
        let contract_account_id = "counter.near-examples.testnet";
        let method_name = "get";
        let args = "{\"account_id\": \"bob.testnet\"}";
        let view_args = ViewArgs::parse_from(&[
            "near",
            contract_account_id,
            method_name,
            args
        ]);
        let result = ViewArgs::to_cli_args(&view_args, "testnet".to_string());
        assert_eq!(
            result.join(" "),
            format!(
                "contract call-function as-read-only {} {} text-args {} network-config testnet now",
                contract_account_id,
                method_name,
                args
            )
        )
    }

    #[test]
    fn view_mainnet() {
        let contract_account_id = "counter.near-examples.testnet";
        let method_name = "get";
        let network_id = "mainnet";
        
        for i in 0..NETWORK_ID_ALIASES.len() {
            let network_id_parameter_alias = &format!("--{}", &NETWORK_ID_ALIASES[i]);
            let view_args = ViewArgs::parse_from(&[
                "near",
                contract_account_id,
                method_name,
                network_id_parameter_alias,
                network_id,
            ]);
            let result = ViewArgs::to_cli_args(&view_args, "testnet".to_string());
            assert_eq!(
                result.join(" "),
                format!(
                    "contract call-function as-read-only {} {} text-args  network-config {} now",
                    contract_account_id,
                    method_name,
                    network_id
                )
            )
        }
    }
}
