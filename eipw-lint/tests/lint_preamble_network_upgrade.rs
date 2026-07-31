/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use eipw_lint::reporters::Text;
use eipw_lint::Linter;

fn proposal(proposal_type: &str, category: Option<&str>, network_upgrade: Option<&str>) -> String {
    let category = category
        .map(|category| format!("category: {category}\n"))
        .unwrap_or_default();
    let network_upgrade = network_upgrade
        .map(|network_upgrade| format!("network-upgrade: {network_upgrade}\n"))
        .unwrap_or_default();

    format!(
        r#"---
eip: 1
title: Example proposal
description: Example description
author: John Doe (@johndoe)
discussions-to: https://ethereum-magicians.org/t/hello/1
status: Draft
type: {proposal_type}
{category}{network_upgrade}created: 2020-01-01
---

## Abstract
This is the abstract for the proposal.

## Specification
This is the specification for the proposal.

## Rationale
This is the rationale for the proposal.

## Security Considerations
These are the security considerations for the proposal.

## Copyright
Copyright and related rights waived via [CC0](../LICENSE.md).
"#
    )
}

async fn reports_for(src: &str) -> String {
    Linter::<Text<String>>::default()
        .check_slice(None, src)
        .run()
        .await
        .unwrap()
        .into_inner()
}

#[tokio::test]
async fn network_upgrade_is_accepted_on_meta() {
    let src = proposal("Meta", None, Some("Glamsterdam"));

    assert_eq!(reports_for(&src).await, "");
}

#[tokio::test]
async fn network_upgrade_is_optional_on_meta() {
    let src = proposal("Meta", None, None);

    assert_eq!(reports_for(&src).await, "");
}

#[tokio::test]
async fn network_upgrade_is_rejected_on_standards_track() {
    let src = proposal("Standards Track", Some("Core"), Some("Glamsterdam"));
    let reports = reports_for(&src).await;

    assert!(
        reports.contains("preamble header `network-upgrade` is only allowed when `type` is `Meta`")
    );
    assert!(!reports.contains("preamble has extra header"));
}

#[tokio::test]
async fn network_upgrade_is_rejected_on_informational() {
    let src = proposal("Informational", None, Some("Glamsterdam"));
    let reports = reports_for(&src).await;

    assert!(
        reports.contains("preamble header `network-upgrade` is only allowed when `type` is `Meta`")
    );
    assert!(!reports.contains("preamble has extra header"));
}

#[tokio::test]
async fn empty_network_upgrade_is_rejected() {
    let src = proposal("Meta", None, Some(""));
    let reports = reports_for(&src).await;

    assert!(reports.contains("preamble header `network-upgrade` value is too short (min 1)"));
    assert!(!reports.contains("preamble has extra header"));
}
