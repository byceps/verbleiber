/*
 * Copyright 2022-2026 Jochen Kupperschmidt
 * License: MIT
 */

use std::time::Duration;

use ureq::Agent;
use ureq::tls::TlsConfig;

pub(crate) fn build_agent(timeout: Duration, disable_tls_verification: bool) -> Agent {
    Agent::config_builder()
        .timeout_global(Some(timeout))
        .tls_config(
            TlsConfig::builder()
                .disable_verification(disable_tls_verification)
                .build(),
        )
        .build()
        .into()
}
