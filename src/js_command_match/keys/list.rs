use crate::js_command_match::constants::NETWORK_ID_ALIASES;

#[derive(Debug, Clone, clap::Parser)]
/// This is a legacy `keys` command. Once you run it with the specified arguments, new syntax command will be suggested.
pub struct KeysArgs {
    account_id: String,
    #[clap(long, aliases = NETWORK_ID_ALIASES)]
    network_id: Option<String>,
}

impl KeysArgs {
    pub fn to_cli_args(&self, network_config: String) -> Vec<String> {
        let network_id = self.network_id.clone().unwrap_or(network_config);

        let command = vec![
            "account".to_string(),
            "list-keys".to_string(),
            self.account_id.to_owned(),
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
    fn list_keys_testnet() {
        let account_id = "bob.testnet";
        let network_id = "testnet";

        for network_id_parameter_alias in NETWORK_ID_ALIASES {
            let keys_args = KeysArgs::parse_from(&[
                "near",
                account_id,
                &format!("--{network_id_parameter_alias}"),
                network_id,
            ]);
            let result = KeysArgs::to_cli_args(&keys_args, "testnet".to_string());
            assert_eq!(
                result.join(" "),
                format!("account list-keys {account_id} network-config {network_id} now",)
            );
        }
    }

    #[test]
    fn list_keys_mainnet() {
        let account_id = "bob.testnet";
        let network_id = "mainnet";

        let network_id_parameter_alias = &format!("--{}", &NETWORK_ID_ALIASES[0]);
        let keys_args =
            KeysArgs::parse_from(&["near", account_id, network_id_parameter_alias, network_id]);
        let result = KeysArgs::to_cli_args(&keys_args, "testnet".to_string());
        assert_eq!(
            result.join(" "),
            format!("account list-keys {account_id} network-config {network_id} now",)
        );
    }
}
