---
title: Committee Shuffle at Epoch Boundary
description: Periodically create new committees to reach consensus by randomly selecting from a group of eligible validators that have stake.
author: Grant Kee (@grantkee)
discussions-to: https://forum.telcoin.org/t/epoch-boundary-validator-shuffle/343
status: Draft
created: 2024-10-29
---

# Epochs with New Committees

## Abstract

This TNIP specifies a Validator Shuffle Protocol designed to enhance network security and decentralization by introducing a robust, cryptographic mechanism for randomizing validator committee selection at defined intervals, termed “epochs”. Eligible validators stake enough TEL to a contract and participate in consensus. Periodically, the protocol randomly selects a group of validators to form a new committee. The new committee is responsible for verifying, settling, and executing transactions for the next epoch.

## Motivation

The foundational security of blockchain protocols relies heavily on the decentralization and unpredictability of consensus participants. A deterministic or predictable pattern in validator selection leads to vulnerabilities, such as targeted attacks and collusion, ultimately compromising the integrity and security of the network.

In Ethereum, validators are randomly shuffled into different committees responsible for validating specific portions of transactions and proposing new blocks. This randomness is critical as it prevents bad actors from predicting their assignments and colluding to influence the network unfairly. By leveraging a similar random committee selection mechanism, Telcoin Network can enhance its robustness against such attacks. Randomly shuffling validators into consensus committees not only minimizes the risks of collusion, it ensures a fair representation of the validator pool and increases the difficulty for any adversarial entities aiming to control the committee selection process.

Introducing a process to rotate eligible validators provides fresh opportunities for new nodes to participate in consensus as voting committee members. Randomly shuffling validators underpins both network security and community growth, aligning with the broader goals of decentralized security and equitable participation.

## Specification

### Definitions

##### Committee Voting Validators ([CVVs](#committee-voting-validators-cvvs))

Validators currently in the committee, responsible for casting votes, extending the canonical chain, and reaching consensus on transactions and finality.

##### Non-Voting Validators ([NVVs](#non-voting-validators-nvvs))

Validators that track and execute consensus but do not vote for every block in the epoch. They participate by receiving consensus through a gossip network consisting of both [CVVs](#committee-voting-validators-cvvs) and [NVVs](#non-voting-validators-nvvs). [NVVs](#non-voting-validators-nvvs) only vote on the latest execution result to support new committees during epoch transitions.

##### On-chain Committee Information

Committee information is stored on-chain to support client verification. The “Consensus Registry” contract is located at 0x467Bccd9d29f223BcE8043b84E8C8B282827790F and includes the following committee types:

* C: The current committee which votes to reach consensus and extend the canonical tip.
* C<sub>p</sub>: The committee from the previous epoch.
* C<sub>n</sub>: The committee in the next epoch.
* C<sub>n+1</sub>: The committee after the next epoch, which is 2 epochs ahead of the current epoch.

##### Epoch Duration

An epoch is defined as a 24-hour time period. The transition is triggered by the first commit to the consensus Directed Acyclic Graph (DAG) after the 24-hour interval.

##### Epoch Boundary (*e*)

The last round of consensus committed to the DAG within an epoch, serving as the trigger for the next committee to advance. The timestamp for e must be greater-than or equal-to the 24-hour interval, but it cannot exceed 30 seconds past.

* *e* : The epoch boundary.
* *e<sub>t</sub>* : "Epoch boundary time" is the UNIX timestamp used to quantify the epoch boundary (currently expected to be 24-hours after the epoch starts).
* *e<sub>r</sub>* : "Epoch boundary round" is the last round of consensus committed by all [CVVs](#committee-voting-validators-cvvs) in the current epoch.
* *e<sub>n</sub>* : "Epoch boundary number" is the number of epochs, starting at 0 and incrementing +1 with every new epoch.
* *e<sub>m</sub>* : "Epoch boundary max" is the maximum amount of time (in seconds) that the pending committee should wait before falling back to a previous execution result to include in their genesis certificate.

NOTE: The timestamp for *e<sub>r</sub>* must be within [*e<sub>t</sub>* , *e<sub>t</sub>* + *e<sub>m</sub>*).

##### Signed Execution Result

The sealed header of the last block produced while executing *e<sub>r</sub>*. The header's hash is signed by the node’s BLS12-381 private key. The signed execution result includes the sealed header and the signed hash of the sealed header. Peers verify signed execution results by taking the SHA-256 hash of the header and comparing it to the signature.

##### Staking Contract

Validators must stake 1 million TEL to the designated staking contract at `0x467Bccd9d29f223BcE8043b84E8C8B282827790F`. The locking period for stake is 10 epochs, after which a validator may withdraw their funds and exit the protocol entirely.

##### NFT Requirement

Validator wallets require an NFT issued by Telcoin Association for staking. The NFT issuance process involves a human, real-world review by the Telcoin Association and is exclusive to GSMA members.

##### Shuffle Mechanism

The Fisher-Yates shuffle algorithm is used to randomly reorder validators. This shuffle occurs once per epoch, and uses the aggregate BLS signature from the leader certificate from the last committed consensus round mixed with the accumulated randomness during the epoch as the source of entropy.

##### Chain Syncing

New nodes synchronize by downloading all consensus output and executing the data up to the current epoch. The Telcoin Association's TAO manages snapshots of execution and consensus data to facilitate this process.

##### Handshake

The handshake is permissionless. [NVVs](#non-voting-validators-nvvs) initiate contact with known validators to subscribe to the latest consensus. The handshake involves exchanging network information to establish peer connections.

##### Transition from [NVV](#non-voting-validators-nvvs) to [CVV](#committee-voting-validators-cvvs)

[NVVs](#non-voting-validators-nvvs) become eligible to transition to [CVVs](#committee-voting-validators-cvvs) after participating for one full epoch. The epoch boundary marks the transition, with validators signing and broadcasting their final execution results to form a new committee based on a quorum of signatures.

##### Committee Transition

At each epoch boundary, the outgoing committee signs and broadcasts the last sealed execution result. The incoming committee waits for a quorum of signed execution results to establish the genesis certificate for the new epoch.

### Becoming an Eligible Validator

Validators must first obtain an NFT through Telcoin Association’s decentralized governance and have a fully synced node online. The validator NFT allows wallets to deposit TEL to the staking contract located at `0x467Bccd9d29f223BcE8043b84E8C8B282827790F`.

Once a node has completed the [staking process](#staking-contract), the validator's status is updated to "active" after one full epoch. Once the validator status is "active" on-chain, it is eligible to become a [CVV](#committee-voting-validators-cvvs). The newly eligible validator will be included in the next shuffle that determines C<sub>n+1</sub>. The node's effective stake is considered during the shuffle process.

The node initiates the handshake protocol with an existing node. The initial handshake from a new node attempting to join the consensus network includes the following information:

* Primary network address
* Worker network address
* BLS12-381 signature of the ECDSA secp256k1 public key used to stake
* The chain id of the network the node is trying to join

Protocol implementations should support node operators to manually specify an IP address to initiate the handshake. Well-known beacons must be supported by the Telcoin Autonomous Operations (TAO) to facilitate peer discovery.

#### “Friendly” network

Once a validator has verified a new node joining the network, the new peer’s information is forwarded to all peers. Nodes must store this information in a persistent way to ensure all known, eligible nodes have network addresses for closing epochs.

If a node needs to update network information, it must initiate another handshake sequence. Nodes must update the peer’s stored information.

In the early stages, the core protocol team is responsible for assisting node operators joining the network. Eventually, the protocol will be open-source so anyone can run a node and execute consensus output. Only GSMA full members are eligible to become [CVVs](#committee-voting-validators-cvvs).

### Closing Epoch Responsibilities

#### Executing the last output

[CVVs](#committee-voting-validators-cvvs) reach consensus and commit rounds to a local DAG until a certain UNIX timestamp is reached (et). Once e<sub>t</sub> is reached, [CVVs](#committee-voting-validators-cvvs) have up to 60 seconds (or enough time to ensure execution attempt, even if extra rounds needed for commit) to commit a leader and execute the output from consensus. While executing the last output, [CVVs](#committee-voting-validators-cvvs) must update the consensus registry contract with committee’s updates using a system call to `concludeEpoch` on the consensus registry contract at 0x0111  The required committee updates are:

1. The current committee must become the previous committee.

<p style="text-align:center;">C<sub>p</sub> = C</p>

2. The next committee must become the current committee to ensure clients following the canonical tip always read the correct current committee from state.

<p style="text-align:center;">C = C<sub>n</sub></p>

3. The committee after the next committee must become the  next committee.

<p style="text-align:center;">C<sub>n</sub> = C<sub>n+1</sub></p>

4. The committee after the next committee must be deterministically decided using the Fisher-Yates shuffle algorithm for all eligible validators from the staking contract in the current epoch that is closing. The source of randomness for shuffling must be the aggregate BLS12-381 signature from the leader certificate. TODO: specify eligible validator requirements in previous section

<p style="text-align:center;">C<sub>n+1</sub> = new committee from a deterministic shuffle</p>

The final execution result is signed by the validator’s BLS12-381 private key and unreliably broadcast to all known validators ([CVVs](#committee-voting-validators-cvvs) and [NVVs](#non-voting-validators-nvvs)).

> TODO: should the signed result be reliably broadcast to the next committee?

[NVVs](#non-voting-validators-nvvs) must also sign and vote on execution results at *e*. All validators continue to gossip signed execution results at *e* until the committee taking over for the next epoch (updated [C](#on-chain-committee-information)) reaches a quorum of signatures (2f+1) from all eligible validators. These votes comprise a quorum from all staked validator nodes on the network, completing the epoch. Pending [CVVs](#committee-voting-validators-cvvs) reliably forward all signed execution results to other committee members to support a successful transition.

#### Recovering from failed execution at *e*

Byzantine nodes must not prevent the network from closing an epoch or a new committee from taking over consensus. If the (pending) current committee C fails to receive a quorum of signed execution results at *e* to start the epoch, then the certified block in the last DAG commit is used to start the epoch. The new committee C must use this leader certificate’s aggregate signatures to generate C<sub>n+1</sub> since the closing committee failed to propose this committee. Nodes must be penalized in the first round of the new epoch by slashing stake if they failed to attest the epoch boundary. The penalty must be applied in addition to the node’s other opening epoch responsibilities. The amount of stake to deduct from the validator’s staked balance will be determined through social governance.

#### Forwarding transactions that weren’t executed

Transactions are executed if they’re included in the consensus DAG commit, so inevitably there will be transactions that were certified within one epoch but unsettled and executed before *e* is reached. The new committee should not rely on a previous committee’s certificates and must reach consensus again before executing any transactions leftover from a previous epoch. To ensure the best possible user experience, exiting [CVVs](#committee-voting-validators-cvvs) must track and reliably forward any remaining transactions to pending [CVVs](#committee-voting-validators-cvvs). [CVVs](#committee-voting-validators-cvvs) in the new epoch must reverify and prioritize these transactions in the early rounds of the new epoch.

An alternative approach is for [CVVs](#committee-voting-validators-cvvs) to track their certified batches and continue advancing rounds until all their batches are executed. At *e<sub>t</sub>*, validators would stop proposing batches. Primaries would only propose empty headers to ensure additional commits to the DAG. Once all batches for a primary are settled, the primary would include a system message in their proposed headers to indicate the epoch should close. A quorum of these system messages triggers the epoch transaction.

On the happy path, this seems more efficient. The network already spent computation and bandwidth to certify these transactions, and the additional overhead to propose empty blocks to reach consensus seems trivial compared to forwarding certified transactions to the next committee. The next committee would still need to re-verify all transactions, so the computation and bandwidth costs are duplicated for the next committee.

Network latency could cause issues if a certificate is delayed or never committed to the DAG.

### Beginning a new epoch

#### Epoch genesis

Epochs are conceptually new chains that begin with the closing state from the previous epoch. The pending committee C collects signed execution results for the last block of the last executed round of consensus until a quorum of all eligible validator nodes is reached (2f+1) or time expires after e<sub>m</sub>, whichever happens first.

The first round committed to the DAG must apply rewards and penalties for the previous epoch for all validators through a system call to the staking contract at 0x467Bccd9d29f223BcE8043b84E8C8B282827790F.

#### Recovering from different execution results at genesis

Because the asynchronous nature of the protocol could cause some nodes to miss the signed execution result at *e*, [CVVs](#committee-voting-validators-cvvs) must verify signatures and use the latest, signed, canonical tip in the event that the proposed genesis certificates include different execution results. Consider the scenario that some nodes have the correct signed execution result and others used the fallback tip because they did not receive a quorum of votes in time. This could happen if e<sub>m</sub> expires while a node has only broadcast its signed execution results to some of its peers.

During epoch genesis, [CVVs](#committee-voting-validators-cvvs) that receive unexpected execution results must consider the most recent result. If a CVV receives an outdated execution result from a peer’s genesis certificate, then it responds with the correct execution results, including all signatures that formed a quorum. The CVV that included the fallback execution result must then verify all signatures of the more-recently signed execution result and reissue their genesis certificate to start the next epoch.

This ensures that the closing executing result from the previous epoch will start the next epoch as long as at least one committee member receives the signed execution result in time.

#### Preparing for the next epoch

Validators that are in the next committee (C<sub>n</sub>) must update their peer lists to include all future committee members by making a request on the gossipsub peer network for peers matching the peer's BLS12-381 public keys. Nodes should drop peers in preference of their future committee members to ensure they have network information for all committee members by the start of the next epoch.

TODO: would a published information be better? Push vs Pull

### Networking

#### Handshake

Validators must support a trustless exchange of peer information. [CVVs](#committee-voting-validators-cvvs) need to know the ports and IP addresses of all other [CVVs](#committee-voting-validators-cvvs) in order to effectively participate in consensus. However, publicly exposing this information introduces attack vectors for DOS attacks.

The public RPC must add a new endpoint called `tn_validatorHandshake` that verifies and acknowledges new peers that have staked TEL and joined the network.

#### Publicly available epoch genesis

Validators must include a new RPC endpoint called `tn_epochGenesis` that returns the quorum of signatures and the sealed header of the execution result used by the current committee to start the current epoch.

### Future considerations

#### Small pool of validators

The current protocol strategy is for a small network of robust nodes. Once the network has a larger number of staked nodes, there will be more [NVVs](#non-voting-validators-nvvs) available to attest to the current committee’s proposed state changes. At that time, it is beneficial to reconsider how [NVVs](#non-voting-validators-nvvs) participate in securing the network. One idea is having [NVVs](#non-voting-validators-nvvs) take random samples of execution results throughout an epoch to ensure validators aren’t being lazy. Lazy validators might subscribe to a peer’s execution results and including it in their own proposed headers as if they performed the execution result.

#### Maintaining peers

The number of validator peers maintained by a CVV is not a concern at this time because the number of eligible validators is small. However, as the network grows in size, it may be necessary to implement a limit to the number of peers a CVV maintains for consensus gossip. It’s critical that consensus maintains smooth operation with maximum bandwidth prioritized for committee messages. Handshakes and gossipping consensus data may influence performance as the network grows.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Rationale

<!--
  The rationale fleshes out the specification by describing what motivated the design and why particular design decisions were made. It should describe alternate designs that were considered and related work, e.g. how the feature is supported in other protocols. The rationale should discuss important objections or concerns raised during discussion around the TNIP.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

### Permissionless [NVVs](#non-voting-validators-nvvs)

The network's security is reinforced with permissionless clients that independently execute consensus output. The network should further enhance its security by allowing these nodes, once staked, to perform attestations against [CVV](#committee-voting-validators-cvvs) consensus output. However, this is outside the scope of this proposal.

Furthermore, node operators may need time to setup their environment and optimize performance (NAT, firewall, latency, etc). The protocol could be designed so operators submit a 2nd transaction to update their status to "active" after staking, but this is inconvenient. Node operators are encouraged to use state-of-the-art security best practices for managing private keys. A second transaction after staking introduces friction to join the network and requires additional gas. Instead, the system is designed so operators can simply stake after they've ensured their node is ready to participate in consensus.

Permissionless access to consensus output accentuates the need for trustless peer communication.

### Initiating Handshakes

Nodes joining the network must initiate handshakes to exchange network information so existing nodes can find them. Initiating handshakes through public RPC methods allows any node operator to join the network as easily as submitting a transaction and reduces the challenge of discovering peers. This approach also allows nodes to easily update their network information.

### [CVV](#committee-voting-validators-cvvs) Gossip

Committee nodes must limit the number and prioritize the type of peers they maintain. Consensus output must be gossipped outside the committee so non-voting clients can track the canonical tip. However, it's critical that [CVVs](#committee-voting-validators-cvvs) prioritize inner-committee communication above all else to ensure consensus is successful.

### Reliable Broadcast??
### Fallback at *e*

## Backwards Compatibility

No backward compatibility issues.

## Test Cases

N/A

## Reference Implementation

N/A

## Security Considerations

<!--
  All TNIPs must contain a section that discusses the security implications/considerations relevant to the proposed change. Include information that might be important for security discussions, surfaces risks and can be used throughout the life cycle of the proposal. For example, include security-relevant design decisions, concerns, important discussions, implementation-specific guidance and pitfalls, an outline of threats and risks and how they are being addressed. TNIP submissions missing the "Security Considerations" section will be rejected. A TNIP cannot proceed to status "Final" without a Security Considerations discussion deemed sufficient by the reviewers.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

### Limited Decentralization (Early Stages)

GSMA full-members hold the exclusive right to validate transactions and participate in consensus. The members are decentralized within the context of Mobile Network Operators (MNOs), but eligible committee members are more limited than other protocols because of this requirement. It is beneficial for the protocol and the network at large for GSMA members to stake and operate multiple nodes. However, it will take time for validator numbers to reach a critical point of diversity where the protocol can tolerate more byzantine nodes without significant consequences.

### Denial of Service (DoS)

Validators must share network information with peers and maintain constant communication. It's also important that [NVVs are permissionless](#permissionless-nvvs) and trustless. As the network continues to grow, validator numbers are expected to be limited in the early stages which increases the effectiveness of flooding validators with bogus requests. Spamming ports dedicated to consensus messages between [CVVs](#committee-voting-validators-cvvs) is especially detrimental to network progress.

Additional consideration is needed to enhance the network's robustness against these types of attacks. Currently, the network uses virtual private networks for committees messages. Redundant proxies may also be effective in preventing this type of attack. MNOs have significant experience mitigating these types of attacks and should provide direction towards refining network security.

### Lazy Validators

Lazy validators might subscribe to a peer's execution results through a public RPC and use the data as their own, as if they computed the output individually. Right now, there is some level of trust that validators won't be lazy and skip execution. Stake and knowledge is a reasonable deterrent in the early stages, but more work is needed to remove this trust assumption as the network grows larger. One idea is to have staked [NVVs](#non-voting-validators-nvvs) randomly sample [CVV](#committee-voting-validators-cvvs) execution results throughout each epoch, but that is outside the scope of this proposal.

## Copyright

Copyright and related rights waived via [CC0](../LICENSE.md).
