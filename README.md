# Fuel Bridge Message Predicates

Most messages sent from the base chain to Fuel will use a predicate as the message recipient. These predicates allow anyone to spend the `InputMessage` but verifies that a specific script is used in the transaction along with conditions of what additional inputs and outputs are acceptable to ensure security and reliability that the message is handled appropriately.

## Message to Contract Predicate

The Message to Contract Predicate is for messages that are trying to send a data payload to a designated Fuel contract. This predicate runs through the following checks:
- Verify Inputs
  - There are no more than 8 total inputs
  - The first input is an `InputContract` with `contractID` that matches the first 32 bytes in the message data field
  - The second input is an `InputMessage` and is the only `InputMessage` (the message with a data payload to send to a contract)
  - There are no other `InputMessage` inputs
  - Only `InputCoin` inputs with `asset_id` set as the base asset are allowed (to cover gas costs)
  - Other `InputContract` inputs are allowed but are not required
- Verify Outputs
  - There are no more than 8 total outputs
  - The first output is an `OutputContract` with `inputIndex` set as `0`
  - The second output is an `OutputChange` with `asset_id` set as the base asset to collect unused gas fees
  - There are no other `OutputChange` inputs
  - Other `OutputContract` outputs are allowed but are not required
  - Any `OutputVariable` outputs are allowed but are not required
  - Any `OutputMessage` outputs are allowed but are not required
  - There are no `OutputCoin` outputs allowed on the transaction
- Verify script bytecode hash for the transaction matches for the designated [Message to Contract Script](#message-to-contract-script)
- Verify that the sum of all `InputCoin` inputs is greater than the gas minimum

If all of these conditions are met, then the predicate evaluates as true.

### Message to Contract Script

The message to contract predicate relies on a script that performs only the following operations:
- Transfer the exact amount of base asset specified in the `InputMessage` `amount` field to the contract with ID that matches the first 32 bytes in the message data field
- Call the function `processMessage` on the contract with ID that matches the first 32 bytes in the message data field

## Building From Source

### Dependencies

| dep     | version                                                  |
| ------- | -------------------------------------------------------- |
| Forc    | [>=v0.19.1](https://fuellabs.github.io/sway/v0.19.1/introduction/installation.html) |

### Building

Build:

```sh
forc build -p contract-message-predicate
forc build -p contract-message-receiver
forc build -p contract-message-script
forc build -p contract-message-test
```

Run tests:

```sh
cd contract-message-test && forc test && cd ..
```

## Contributing

Code must be formatted.

```sh
forc-fmt -p contract-message-predicate
forc-fmt -p contract-message-receiver
forc-fmt -p contract-message-script
forc-fmt -p contract-message-test
cd contract-message-test && cargo fmt && cd ..
```

## License

The primary license for this repo is `Apache 2.0`, see [`LICENSE`](./LICENSE).
