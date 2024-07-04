use crate::js_command_match::constants::{
    DEFAULT_SEED_PHRASE_PATH, LEDGER_PATH_ALIASES, NETWORK_ID_ALIASES, USE_LEDGER_ALIASES,
};

#[derive(Debug, Clone, clap::Parser)]
/// This is a legacy `create-account` command. Once you run it with the specified arguments, new syntax command will be suggested.
pub struct DeleteAccountArgs {
    account_id: String,
    beneficiary_id: String,
    // TODO Rename ledger variables the same way as it is for create_account
    #[clap(long, aliases = USE_LEDGER_ALIASES, default_value_t = false)]
    use_ledger: bool,
    #[clap(long, aliases = LEDGER_PATH_ALIASES, default_missing_value = Some(DEFAULT_SEED_PHRASE_PATH), num_args=0..=1)]
    ledger_path: Option<String>,
    #[clap(long, aliases = NETWORK_ID_ALIASES, default_value=None)]
    network_id: Option<String>,
    #[clap(allow_hyphen_values = true, num_args = 0..)]
    _unknown_args: Vec<String>,
}

impl DeleteAccountArgs {
    pub fn to_cli_args(&self, network_config: String) -> Vec<String> {
        let network_id = self.network_id.clone().unwrap_or(network_config);

        let mut command = vec![
            "account".to_string(),
            "delete-account".to_string(),
            self.account_id.to_owned(),
            "beneficiary".to_string(),
            self.beneficiary_id.to_owned(),
        ];

        command.push("network-config".to_string());
        command.push(network_id);

        if self.use_ledger {
            command.push("sign-with-ledger".to_string());
            command.push("--seed-phrase-hd-path".to_string());
            command.push(self.ledger_path.to_owned().unwrap_or_default());
        } else {
            command.push("sign-with-keychain".to_string());
        }

        command.push("send".to_string());

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn delete_account_using_ledger_testnet() {
        let account_id = "bob.testnet";
        let beneficiary_id = "alice.testnet";

        for i in 0..USE_LEDGER_ALIASES.len() {
            let use_ledger_parameter_alias = &format!("--{}", &USE_LEDGER_ALIASES[i]);
            let delete_args = DeleteAccountArgs::parse_from(&[
                "near",
                account_id,
                beneficiary_id,
                use_ledger_parameter_alias,
            ]);
            let result = DeleteAccountArgs::to_cli_args(&delete_args, "testnet".to_string());
            assert_eq!(
                result.join(" "),
                format!(
                    "account delete-account {} beneficiary {} network-config testnet sign-with-ledger --seed-phrase-hd-path  send",
                    account_id,
                    beneficiary_id
                )
            )
        }
    }

    #[test]
    fn delete_account_using_ledger_and_custom_path_testnet() {
        let account_id = "bob.testnet";
        let beneficiary_id = "alice.testnet";

        for i in 0..LEDGER_PATH_ALIASES.len() {
            let ledger_path_parameter_alias = &format!("--{}", &LEDGER_PATH_ALIASES[i]);
            let use_ledger_parameter_alias = &format!("--{}", &USE_LEDGER_ALIASES[0]);
            let delete_args = DeleteAccountArgs::parse_from(&[
                "near",
                account_id,
                beneficiary_id,
                use_ledger_parameter_alias,
                ledger_path_parameter_alias,
                DEFAULT_SEED_PHRASE_PATH,
            ]);
            let result = DeleteAccountArgs::to_cli_args(&delete_args, "testnet".to_string());
            assert_eq!(
                result.join(" "),
                format!(
                    "account delete-account {} beneficiary {} network-config testnet sign-with-ledger --seed-phrase-hd-path {} send",
                    account_id,
                    beneficiary_id,
                    DEFAULT_SEED_PHRASE_PATH
                )
            )
        }
    }

    #[test]
    fn delete_account_using_ledger_mainnet() {
        let account_id = "bob.testnet";
        let beneficiary_id = "alice.testnet";
        let network_id = "mainnet";

        for i in 0..NETWORK_ID_ALIASES.len() {
            let network_id_parameter_alias = &format!("--{}", &NETWORK_ID_ALIASES[i]);
            let use_ledger_parameter_alias = &format!("--{}", &USE_LEDGER_ALIASES[0]);
            let delete_args = DeleteAccountArgs::parse_from(&[
                "near",
                account_id,
                beneficiary_id,
                use_ledger_parameter_alias,
                network_id_parameter_alias,
                network_id,
            ]);
            let result = DeleteAccountArgs::to_cli_args(&delete_args, "testnet".to_string());
            assert_eq!(
                result.join(" "),
                format!(
                    "account delete-account {} beneficiary {} network-config {} sign-with-ledger --seed-phrase-hd-path  send",
                    account_id,
                    beneficiary_id,
                    network_id
                )
            )
        }
    }

    #[test]
    fn delete_account_using_keychain_testnet() {
        let account_id = "bob.testnet";
        let beneficiary_id = "alice.testnet";
        let delete_args = DeleteAccountArgs::parse_from(&["near", account_id, beneficiary_id]);
        let result = DeleteAccountArgs::to_cli_args(&delete_args, "testnet".to_string());
        assert_eq!(
            result.join(" "),
            format!(
                "account delete-account {} beneficiary {} network-config testnet sign-with-keychain send",
                account_id,
                beneficiary_id
            )
        );
    }
}
