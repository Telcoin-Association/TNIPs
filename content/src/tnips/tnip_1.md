---
title: TNIP Purpose and Guidelines
description: Guidelines for TNIP process.
author: grant (@grantkee) <grant@telcoin.org>, et al.
status: Living
created: 2024-07-30
---

# Guidelines for TNIP Process

## What is a TNIP?

TNIP stands for Telcoin Network Improvement Proposal. A TNIP is a design document providing information to the Telcoin Network community, or describing a new feature for Telcoin Network or its processes or environment. The TNIP should provide a concise technical specification of the feature and a rationale for the feature. The TNIP author is responsible for building consensus within the community and documenting dissenting opinions.

## TNIP Rationale
creating change to trigger CI

TNIPs are intended to be the primary mechanism for proposing new features, for collecting community technical input on an issue, and for documenting the design decisions that have gone into Telcoin Network. TNIPs are maintained as text files in a versioned repository, so their revision history is the historical record of the feature proposal.

For Telcoin Network implementers, TNIPs are a convenient way to track the progress of their implementation. Ideally each implementation maintainer would list the TNIPs that they have implemented. This will give end users a convenient way to know the current status of a given implementation or library.

## TNIP Types

There are three types of TNIP:

- A **Standards Track TNIP** describes any change that affects most or all Telcoin Network implementations, such as: a change to the network protocol, a change in block or transaction validity rules, proposed application standards/conventions, or any change or addition that affects the interoperability of applications using Telcoin Network. Furthermore, Standards Track TNIPs are broken down into the following categories:
  - **Core**: improvements requiring a consensus fork, as well as changes that are not necessarily consensus critical but may be relevant to “core dev” discussions.
  - **Networking**: includes improvements around devp2p and Light Telcoin Network Subprotocol, as well as proposed improvements to network protocol specifications.
  - **Interface**: includes improvements around around client API/RPC specifications and standards, as well as language-level standards like method names and contract ABIs.

- A **Meta EIP** describes a process surrounding Telcoin Network or proposes a change to (or an event in) a process. Process TNIPs are like Standards Track TNIPs but apply to areas other than the Telcoin Network protocol itself. They may propose an implementation, but not to Telcoin Network's codebase; they often require community consensus; unlike Informational TNIPs, they are more than recommendations, and users are typically not free to ignore them. Examples include procedures, guidelines, changes to the decision-making process, and changes to the tools or environment used in Telcoin Network development. Any meta-TNIP is also considered a Process TNIP.

- An **Informational TNIP** describes an Telcoin Network design issue, or provides general guidelines or information to the Telcoin Network community, but does not propose a new feature. Informational TNIPs do not necessarily represent Telcoin Network community consensus or a recommendation, so users and implementers are free to ignore Informational TNIPs.

It is highly recommended that a single TNIP contain a single key proposal or new idea. TNIPs that focus on a pariticular issue are more likely to become integrated into the protocol.

A TNIP must meet certain minimum criteria. It must be a clear and complete description of the proposed enhancement. The enhancement must represent a net improvement. The proposed implementation, if applicable, must be solid and must not complicate the protocol unduly.

## TNIP Work Flow

### Creating a TNIP

Before you begin writing a formal TNIP, you should vet your idea. Ask the Telcoin Network community first if an idea is original to avoid wasting time on something that will be rejected based on prior research. It is thus recommended to open a discussion thread on [the Telcoin Network forum](https://forum.telcoin.org/c/council-proposals/tnip/85) to do this.

Once the idea has been vetted, your next responsibility is to present (by means of a TNIP) the idea to the reviewers and all interested parties, invite editors, developers, and the community to give feedback on the aforementioned channels. You should try and gauge whether the interest in your TNIP is commensurate with both the work involved in implementing it and how many parties will have to conform to it. The work required for implementing a core TNIP is significant and the TNIP will need sufficient interest from the Telcoin Network protocol team. Negative community feedback is taken into consideration and may prevent your TNIP from moving past the Draft stage.

### TNIP Process

The following is the standardization process for all TNIPs in all tracks:

**Idea** - An idea that is pre-draft. This is not tracked within the TNIP Repository.

**Draft** - The first formally tracked stage of a TNIP in development. A TNIP is merged by a TNIP Editor into the TNIP repository when properly formatted.

**Review** - A TNIP Author marks a TNIP as ready for and requesting Peer Review.

**Last Call** - This is the final review window for a TNIP before moving to `Final`. A TNIP editor will assign `Last Call` status and set a review end date (`last-call-deadline`), typically 14 days later.

If this period results in necessary normative changes it will revert the TNIP to `Review`.

**Final** - This TNIP represents the final standard. A Final TNIP exists in a state of finality and should only be updated to correct errata and add non-normative clarifications.

A PR moving a TNIP from Last Call to Final SHOULD contain no changes other than the status update. Any content or editorial proposed change SHOULD be separate from this status-updating PR and committed prior to it.

**Stagnant** - Any TNIP in `Draft` or `Review` or `Last Call` if inactive for a period of 6 months or greater is moved to `Stagnant`. A TNIP may be resurrected from this state by Authors or TNIP Editors through moving it back to `Draft` or its earlier status. If not resurrected, a proposal may stay forever in this status.

>*TNIP Authors are notified of any algorithmic change to the status of their TNIP*

**Withdrawn** - The TNIP Author(s) have withdrawn the proposed TNIP. This state has finality and can no longer be resurrected using this TNIP number. If the idea is pursued at later date it is considered a new proposal.

**Living** - A special status for TNIPs that are designed to be continually updated and not reach a state of finality. This includes most notably TNIP-1.

## What belongs in a successful TNIP?

Each TNIP should have the following parts:

- **Preamble** - RFC 822 style headers containing metadata about the TNIP, including the TNIP number, a short descriptive title (limited to a maximum of 44 characters), a description (limited to a maximum of 140 characters), and the author details. Irrespective of the category, the title and description should not include TNIP number.
- **Abstract** - Abstract is a multi-sentence (short paragraph) technical summary. This should be a very terse and human-readable version of the specification section. Someone should be able to read the abstract and get the gist of what this specification does.
- **Motivation** - The motivation section is critical for TNIPs that want to change the Telcoin Network protocol. It should clearly explain why the existing protocol specification is inadequate to address the problem that the TNIP solves. This section may be brief if the motivation is evident.
- **Specification** - The technical specification should describe the syntax and semantics of any new feature. The specification should be detailed.
- **Rationale** - The rationale fleshes out the specification by describing what motivated the design and why particular design decisions were made. It should describe alternate designs that were considered and related work, e.g. how the feature is supported in other protocols. The rationale should discuss important objections or concerns raised during discussion around the TNIP.
- **Backwards Compatibility** *(optional)* - All TNIPs that introduce backwards incompatibilities must include a section describing these incompatibilities and their consequences. The TNIP must explain how the author proposes to deal with these incompatibilities. This section may be omitted if the proposal does not introduce any backwards incompatibilities, but this section must be included if backward incompatibilities exist.
- **Test Cases** - Test cases for an implementation are mandatory for TNIPs. Tests should either be inlined in the TNIP as data (such as input/expected output pairs) or listed as test cases with outcomes.
- **Reference Implementation** *(optional)* - An optional section that contains a reference/example implementation that facilitates deeper understanding. This section may be omitted for all TNIPs.
- **Security Considerations** - All TNIPs must contain a section that discusses the security implications/considerations relevant to the proposed change. Include information that might be important for security discussions, surfaces risks and can be used throughout the life-cycle of the proposal. E.g. include security-relevant design decisions, concerns, important discussions, implementation-specific guidance and pitfalls, an outline of threats and risks and how they are being addressed. TNIP submissions missing the "Security Considerations" section will be rejected. A TNIP cannot proceed to status "Final" without a Security Considerations discussion deemed sufficient by the reviewers.
- **Copyright Waiver** - All TNIPs must be in the public domain. The copyright waiver MUST link to the license file and use the following wording: `Copyright and related rights waived via [CC0](../LICENSE.md).`

## TNIP Formats and Templates

TNIPs should be written in [markdown](https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet) format. There is a [template](https://github.com/Telcoin-Association/TNIPs/blob/main/content/tnip-template.md) to follow.

## TNIP Header Preamble

Each TNIP must begin with an [RFC 822](https://www.ietf.org/rfc/rfc822.txt) style header preamble. The header information must appear in the following order.

`tnip`: *TNIP number*

`title`: *The TNIP title is a few words, not a complete sentence*

`description`: *Description is one full (short) sentence*

`author`: *The list of the author's or authors' name(s) and/or username(s), or name(s) and email(s). Details are below.*

`discussions-to`: *The url pointing to the official discussion thread*

`status`: *Draft, Review, Last Call, Final, Stagnant, Withdrawn, Living*

`last-call-deadline`: *The date last call period ends on* (Optional field, only needed when status is `Last Call`)

`created`: *Date the TNIP was created on*

`requires`: *TNIP number(s)* (Optional field)

`withdrawal-reason`: *A sentence explaining why the TNIP was withdrawn.* (Optional field, only needed when status is `Withdrawn`)

Headers that permit lists must separate elements with commas.

Headers requiring dates will always do so in the format of ISO 8601 (yyyy-mm-dd).

### `author` header

The `author` header lists the names, email addresses and/or usernames of the authors/owners of the TNIP. Those who prefer anonymity may use a username only, or a first name and a username. The format of the `author` header value must be:

name + github username
> Another E. User (@username)

or

name + email
> Random T. User &lt;address@dom.ain&gt;

or

name + username + email
> Some L. User (@username) &lt;address@dom.ain&gt;

or

just name
> Mystery S. User

Note: At least one author must use a GitHub username, in order to get notified on change requests and have the capability to approve or reject them.

### `discussions-to` header

While a TNIP is a draft, a `discussions-to` header will indicate the URL where the TNIP is being discussed.

The preferred discussion URL is a topic on [Telcoin Network Forum](https://forum.telcoin.org/c/council-proposals/tnip/85). The URL cannot point to Github pull requests, any URL which is ephemeral, and any URL which can get locked over time (i.e. Reddit topics).

### `created` header

The `created` header records the date that the TNIP was assigned a number. Both headers should be in yyyy-mm-dd format, e.g. 2001-08-14.

### `requires` header

TNIPs may have a `requires` header, indicating the TNIP numbers that this TNIP depends on. If such a dependency exists, this field is required.

A `requires` dependency is created when the current TNIP cannot be understood or implemented without a concept or technical element from another TNIP. Merely mentioning another TNIP does not necessarily create such a dependency.

## Linking to other TNIPs

References to other TNIPs should follow the format `TNIP-N` where `N` is the TNIP number you are referring to.  Each TNIP that is referenced in a TNIP **MUST** be accompanied by a relative markdown link the first time it is referenced, and **MAY** be accompanied by a link on subsequent references.  The link **MUST** always be done via relative paths so that the links work in this GitHub repository, forks of this repository, the main TNIPs site, mirrors of the main TNIP site, etc.  For example, you would link to this TNIP as `./tnip-1.md`.

## Auxiliary Files

Images, diagrams and auxiliary files should be included in a subdirectory of the `assets` folder for that TNIP as follows: `assets/tnip-N` (where **N** is to be replaced with the TNIP number). When linking to an image in the TNIP, use relative links such as `../assets/tnip-1/image.png`.

## Transferring TNIP Ownership

It occasionally becomes necessary to transfer ownership of TNIPs to a new developer. In general, we'd like to retain the original author as a co-author of the transferred TNIP, but that's really up to the original author. A good reason to transfer ownership is because the original author no longer has the time or interest in updating it or following through with the TNIP process, or has fallen off the face of the 'net (i.e. is unreachable or isn't responding to email). A bad reason to transfer ownership is because you don't agree with the direction of the TNIP. We try to build consensus around a TNIP, but if that's not possible, you can always submit a competing TNIP.

If you are interested in assuming ownership of a TNIP, send a message asking to take over, addressed to both the original author and the TNIP editor.

## TNIP Editors

The current TNIP editors are

- Grant Kee (@grantkee)
- Steven Stanfield (@sstanfield)
- Markus Osterlund (@robriks)

## TNIP Editor Responsibilities

For each new TNIP that comes in, an editor does the following:

- Read the TNIP to check if it is ready: sound and complete. The ideas must make technical sense, even if they don't seem likely to get to final status.
- The title should accurately describe the content.
- Check the TNIP for language (spelling, grammar, sentence structure, etc.), markup (GitHub flavored Markdown), code style

If the TNIP isn't ready, the editor will send it back to the author for revision, with specific instructions.

Once the TNIP is ready for the repository, the TNIP editor will:

- Assign a TNIP number (generally incremental; editors can reassign if number sniping is suspected)
- Merge the corresponding [pull request](https://github.com/telcoin-association/TNIPs/pulls)
- Send a message back to the TNIP author with the next step.

Many TNIPs are written and maintained by developers with write access to the Telcoin Network codebase. The TNIP editors monitor TNIP changes, and correct any structure, grammar, spelling, or markup mistakes we see.

The editors don't pass judgment on TNIPs. We merely do the administrative & editorial part.

## Style Guide

### Titles

The `title` field in the preamble:

- Should not include the word "standard" or any variation thereof; and
- Should not include the TNIP's number.

### Descriptions

The `description` field in the preamble:

- Should not include the word "standard" or any variation thereof; and
- Should not include the TNIP's number.

### TNIP numbers

TNIPs must be written in the hyphenated form `TNIP-X` where `X` is that TNIP's assigned number.

### RFC 2119 and RFC 8174

TNIPs are encouraged to follow [RFC 2119](https://www.ietf.org/rfc/rfc2119.html) and [RFC 8174](https://www.ietf.org/rfc/rfc8174.html) for terminology and to insert the following at the beginning of the Specification section:

> The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## History

This document was derived heavily from [Ethereum's EIP-1](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1.md), which was derived heavily from [Bitcoin's BIP-0001](https://github.com/bitcoin/bips) written by Amir Taaki which in turn was derived from [Python's PEP-0001](https://peps.python.org/). In many places text was simply copied and modified. Although the PEP-0001 text was written by Barry Warsaw, Jeremy Hylton, and David Goodger, they are not responsible for its use in the Telcoin Network Improvement Process, and should not be bothered with technical questions specific to Telcoin Network or the TNIP. Please direct all comments to the TNIP editors.

## Copyright

Copyright and related rights waived via [CC0](../LICENSE.md).
