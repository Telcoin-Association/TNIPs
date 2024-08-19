---
title: Execution Nodes for Trustless Public Validation
description: A separate process for independently executing the output from consensus.
author: grant (@grantkee)
discussions-to: https://forum.telcoin.org/t/light-clients-execution-nodes/296
status: Draft
created: 2024-08-06
---

## Abstract

<!--
  The  Abstract is a multi-sentence (short paragraph) technical summary. This should be a very terse and human-readable version of the specification section. Someone should be able to read the abstract and get the gist of what this specification does.

  TODO: Remove this comment before submitting
-->

NOTE: this is not a pure abstract and includes elements that belong in motivation section

This TNIP introduces a separate process, aka an "execution node", that retrieves the latest validated data from the consensus layer and independently executes it. This mechanism is conceptually similar to Ethereum's light clients, which do not download the entire blockchain but instead rely on headers to ensure data integrity and state correctness.

By enabling lightweight and efficient validation of the blockchain state, these execution nodes enhance accessibility and reliability for public users and applications requiring trusted blockchain data without the overhead of full node operation. This architecture promotes decentralization by allowing more participants to engage in verifying transaction states in a trustless manner.

## Motivation
NOTE: what about Axelar just querying block number / hash to validate that way?

<!--
  The Motivation section is critical for TNIPs that want to change the Telcoin Network protocol. It should clearly explain why the existing protocol specification is inadequate to address the problem that the TNIP solves. This section may be brief if the motivation is evident.

  With a few exceptions, external links are not allowed. If you feel that a particular resource would demonstrate a compelling case for your TNIP, then save it as a printer-friendly PDF, put it in the assets folder, and link to that copy.

  TODO: Remove this comment before submitting
-->
The current architecture of Telcoin Network relies heavily on full, validator nodes to execute and verify consensus data. This reliance presents challenges to scalability, accessibility, and security. Telcoin Network's proposal should address these challenges by allowing anyone to execute and attest the latest consensus data independently, without relying on the potentially biased or erroneous execution data from validators. Implementing execution nodes that operate independently to execute raw data from the consensus layer ensures the integrity of consensus remains trustless and verifiable.

Such a system supports not only the infrastructure's robustness and the development of the network but also significantly enhances user accessibility and experience. Light nodes, requiring far fewer resources than full nodes, make it possible for a broader range of participants to engage with Telcoin Network. This inclusivity is vital for maintaining the network's decentralization, as it allows individuals with limited hardware capabilities or those unwilling to allocate substantial storage space to still participate actively.

Furthermore, the user experience is notably improved through light nodes, which enable more efficient operation of lightweight applications, such as mobile wallets and bridging attestation services. Light execution nodes allow users to manage their accounts and interact with decentralized applications directly from their devices, without the overhead associated with running a full node (such as storage space). As the blockchain grows, scalability becomes an increasing concern. Light execution nodes offer a solution by offloading the majority of data handling and computation to full nodes, thus alleviating the burden on individual users and enhancing overall network performance.

## Specification

<!--
  The Specification section should describe the syntax and semantics of any new feature. The specification should be detailed.

  It is recommended to follow RFC 2119 and RFC 8170. Do not remove the key word definitions if RFC 2119 and RFC 8170 are followed.

  TODO: Remove this comment before submitting
-->

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Rationale

<!--
  The rationale fleshes out the specification by describing what motivated the design and why particular design decisions were made. It should describe alternate designs that were considered and related work, e.g. how the feature is supported in other protocols. The rationale should discuss important objections or concerns raised during discussion around the TNIP.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

TBD

## Backwards Compatibility

<!--

  This section is optional.

  All TNIPs that introduce backwards incompatibilities must include a section describing these incompatibilities and their consequences. The TNIP must explain how the author proposes to deal with these incompatibilities. This section may be omitted if the proposal does not introduce any backwards incompatibilities, but this section must be included if backward incompatibilities exist.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

No backward compatibility issues found.

## Test Cases

<!--
  The Test Cases section is mandatory for TNIPs. Tests should either be inlined in the TNIP as data (such as input/expected output pairs) or listed as test cases with outcomes.
  If the test suite is too large to reasonably be included inline, then consider adding it as one or more files in `../assets/tnip-####/`. External links are not allowed.

  TODO: Remove this comment before submitting
-->

## Reference Implementation

<!--
  This section is optional.

  The Reference Implementation section should include a minimal implementation that assists in understanding or implementing this specification. It should not include project build files. The reference implementation is not a replacement for the Specification section, and the proposal should still be understandable without it.
  If the reference implementation is too large to reasonably be included inline, then consider adding it as one or more files in `../assets/tnip-####/`. External links are not allowed.

  TODO: Remove this comment before submitting
-->

## Security Considerations

<!--
  All TNIPs must contain a section that discusses the security implications/considerations relevant to the proposed change. Include information that might be important for security discussions, surfaces risks and can be used throughout the life cycle of the proposal. For example, include security-relevant design decisions, concerns, important discussions, implementation-specific guidance and pitfalls, an outline of threats and risks and how they are being addressed. TNIP submissions missing the "Security Considerations" section will be rejected. A TNIP cannot proceed to status "Final" without a Security Considerations discussion deemed sufficient by the reviewers.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

Needs discussion.

## Copyright

Copyright and related rights waived via [CC0](../LICENSE.md).
