use crate::js_command_match::constants::{
    DEFAULT_SEED_PHRASE_PATH, INITIAL_BALANCE_ALIASES, LEDGER_PATH_ALIASES, NETWORK_ID_ALIASES,
    PK_LEDGER_PATH_ALIASES, PUBLIC_KEY_ALIASES, SEED_PHRASE_ALIASES, SIGN_WITH_LEDGER_ALIASES,
    USE_ACCOUNT_ALIASES, USE_FAUCET_ALIASES, USE_LEDGER_PK_ALIASES,
};

#[derive(Debug, Clone, clap::Parser)]
#[clap(alias("create"))]
pub struct CreateAccountArgs {
    new_account_id: String,
    #[clap(long, aliases = USE_FAUCET_ALIASES, default_value_t = false)]
    use_faucet: bool,
    #[clap(long, aliases = USE_ACCOUNT_ALIASES, default_value = None, conflicts_with = "use_faucet")]
    use_account: Option<String>,
    #[clap(long, aliases = INITIAL_BALANCE_ALIASES, default_value = Some("1"))]
    initial_balance: Option<String>,
    #[clap(long, aliases = PUBLIC_KEY_ALIASES, default_value = None)]
    public_key: Option<String>,
    #[clap(long, aliases = SEED_PHRASE_ALIASES, default_value = None, conflicts_with = "public_key")]
    seed_phrase: Option<String>,
    #[clap(long, aliases = SIGN_WITH_LEDGER_ALIASES, default_value_t = false, conflicts_with="use_faucet")]
    sign_with_ledger: bool,
    #[clap(long, aliases = LEDGER_PATH_ALIASES, default_value = Some(DEFAULT_SEED_PHRASE_PATH))]
    ledger_path: Option<String>,
    #[clap(long, aliases = USE_LEDGER_PK_ALIASES, default_value_t = false, conflicts_with = "public_key")]
    use_ledger_pk: bool,
    #[clap(long, aliases = PK_LEDGER_PATH_ALIASES, default_value = Some(DEFAULT_SEED_PHRASE_PATH))]
    pk_ledger_path: Option<String>,
    #[clap(long, aliases = NETWORK_ID_ALIASES, default_value=None)]
    network_id: Option<String>,
}

impl CreateAccountArgs {
    pub fn to_cli_args(&self, network_config: String) -> Vec<String> {
        let network_id = self.network_id.clone().unwrap_or(network_config.to_owned());

        let mut command = vec!["account".to_string(), "create-account".to_string()];

        if self.use_faucet {
            command.push("sponsor-by-faucet-service".to_string());
            command.push(self.new_account_id.to_owned());
        } else {
            command.push("fund-myself".to_string());
            command.push(self.new_account_id.to_owned());
            command.push(format!(
                "{} NEAR",
                self.initial_balance.to_owned().unwrap_or_default()
            ));
        }

        if self.use_ledger_pk {
            command.push("use-ledger".to_owned());

            // add after issue with ledger key is resolved
            // command.push(format!("--seed-phrase-hd-path {}", self.pk_ledger_path.clone().unwrap()));
        };

        if self.seed_phrase.is_some() {
            command.push("use-manually-provided-seed-phrase".to_string());
            command.push(self.seed_phrase.clone().unwrap());
        };

        if self.public_key.is_some() {
            command.push("use-manually-provided-public-key".to_string());
            command.push(self.public_key.clone().unwrap());
        };

        if !self.seed_phrase.is_some() && !self.public_key.is_some() && !self.use_ledger_pk {
            command.push("autogenerate-new-keypair".to_string());
            command.push("save-to-keychain".to_string());
        };

        if !self.use_faucet {
            command.push("sign-as".to_string());
            command.push(
                self.use_account
                    .to_owned()
                    .expect("Valid master account must be provided"),
            );
        };

        command.push("network-config".to_string());
        command.push(network_id);

        if self.use_faucet {
            command.push("create".to_string());
        } else {
            if self.sign_with_ledger {
                command.push("sign-with-ledger".to_string());
                command.push("--seed-phrase-hd-path".to_string());
                command.push(self.ledger_path.to_owned().unwrap_or_default());
            } else {
                command.push("sign-with-keychain".to_string());
            }
            command.push("send".to_string());
        }

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn create_account_using_faucet_testnet() {
        for i in 0..USE_FAUCET_ALIASES.len() {
            let use_faucet_parameter_alias = &format!("--{}", &USE_FAUCET_ALIASES[i]);
            let create_account_args =
                CreateAccountArgs::parse_from(&["near", "bob.testnet", use_faucet_parameter_alias]);
            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account sponsor-by-faucet-service bob.testnet autogenerate-new-keypair save-to-keychain network-config testnet create"
          )
        }
    }

    #[test]
    fn create_account_using_master_account_without_initial_balance_testnet() {
        for i in 0..USE_ACCOUNT_ALIASES.len() {
            let master_account_parameter_alias = &format!("--{}", &USE_ACCOUNT_ALIASES[i]);
            let create_account_args = CreateAccountArgs::parse_from(&[
                "near",
                "bob.testnet",
                master_account_parameter_alias,
                "alice.testnet",
            ]);

            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account fund-myself bob.testnet 1 NEAR autogenerate-new-keypair save-to-keychain sign-as alice.testnet network-config testnet sign-with-keychain send"
          )
        }
    }

    #[test]
    fn create_account_using_master_account_with_init_balance_testnet() {
        for i in 0..INITIAL_BALANCE_ALIASES.len() {
            let initial_balance_parameter_alias = &format!("--{}", &INITIAL_BALANCE_ALIASES[i]);
            let master_account_parameter_alias = &format!("--{}", &USE_ACCOUNT_ALIASES[0]);
            let create_account_args = CreateAccountArgs::parse_from(&[
                "near",
                "bob.testnet",
                master_account_parameter_alias,
                "alice.testnet",
                initial_balance_parameter_alias,
                "0.1",
            ]);

            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account fund-myself bob.testnet 0.1 NEAR autogenerate-new-keypair save-to-keychain sign-as alice.testnet network-config testnet sign-with-keychain send"
          )
        }
    }

    #[test]
    fn create_account_using_seed_phrase_and_faucet_testnet() {
        for i in 0..SEED_PHRASE_ALIASES.len() {
            let seed_phrase_parameter_alias = &format!("--{}", &SEED_PHRASE_ALIASES[i]);
            let use_faucet_parameter_alias = &format!("--{}", &USE_FAUCET_ALIASES[0]);
            let create_account_args = CreateAccountArgs::parse_from(&[
                "near",
                "bob.testnet",
                seed_phrase_parameter_alias,
                "crisp clump stay mean dynamic become fashion mail bike disorder chronic sight",
                use_faucet_parameter_alias,
            ]);

            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account sponsor-by-faucet-service bob.testnet use-manually-provided-seed-phrase crisp clump stay mean dynamic become fashion mail bike disorder chronic sight network-config testnet create"
          )
        }
    }

    #[test]
    fn create_account_using_public_key_master_key_and_initial_balance_testnet() {
        for i in 0..PUBLIC_KEY_ALIASES.len() {
            let public_key_parameter_alias = &format!("--{}", &PUBLIC_KEY_ALIASES[i]);
            let master_account_parameter_alias = &format!("--{}", &USE_ACCOUNT_ALIASES[0]);
            let initial_balance_parameter_alias = &format!("--{}", &INITIAL_BALANCE_ALIASES[0]);
            let create_account_args = CreateAccountArgs::parse_from(&[
                "near",
                "bob.testnet",
                master_account_parameter_alias,
                "alice.testnet",
                public_key_parameter_alias,
                "78MziB9aTNsu19MHHVrfWy762S5mAqXgCB6Vgvrv9uGV",
                initial_balance_parameter_alias,
                "0.1",
            ]);

            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account fund-myself bob.testnet 0.1 NEAR use-manually-provided-public-key 78MziB9aTNsu19MHHVrfWy762S5mAqXgCB6Vgvrv9uGV sign-as alice.testnet network-config testnet sign-with-keychain send"
          )
        }
    }

    #[test]
    fn create_account_using_ledger_testnet() {
        for i in 0..USE_LEDGER_PK_ALIASES.len() {
            let use_ledger_parameter_alias = &format!("--{}", &USE_LEDGER_PK_ALIASES[i]);
            let create_account_args = CreateAccountArgs::parse_from(&[
                "near",
                "bob.testnet",
                use_ledger_parameter_alias,
                "--useFaucet",
            ]);

            let result =
                CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
            assert_eq!(
              result.join(" "),
              "account create-account sponsor-by-faucet-service bob.testnet use-ledger network-config testnet create"
          )
        }
    }

    #[test]
    fn create_account_using_master_account_and_ledger_testnet() {
        let create_account_args = CreateAccountArgs::parse_from(&[
            "near",
            "bob.testnet",
            "--useAccount",
            "alice.testnet",
            "--signWithLedger",
            "--networkId",
            "testnet",
        ]);

        let result = CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
        assert_eq!(
            result.join(" "),
            "account create-account fund-myself bob.testnet 1 NEAR autogenerate-new-keypair save-to-keychain sign-as alice.testnet network-config testnet sign-with-ledger --seed-phrase-hd-path 44'/397'/0'/0'/1' send"
        )
    }

    #[test]
    fn create_account_using_master_account_and_ledger_mainnet() {
        let create_account_args = CreateAccountArgs::parse_from(&[
            "near",
            "bob.near",
            "--useAccount",
            "alice.near",
            "--signWithLedger",
            "--ledgerPath",
            "44'/397'/0'/0'/2'",
            "--networkId",
            "mainnet",
        ]);

        let result = CreateAccountArgs::to_cli_args(&create_account_args, "testnet".to_string());
        assert_eq!(
              result.join(" "),
              "account create-account fund-myself bob.near 1 NEAR autogenerate-new-keypair save-to-keychain sign-as alice.near network-config mainnet sign-with-ledger --seed-phrase-hd-path 44'/397'/0'/0'/2' send"
          )
    }
}
