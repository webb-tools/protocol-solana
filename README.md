# protocol-solana

## Build, Deploy and Test

Let's run the test once to see what happens.

First, install dependencies:

```
$ npm install

$ npm install -g mocha
```

Make sure you have your local solana validator running if you want to deploy the program locally:

```
$ solana-test-validator
```

> If you are on Apple Silicon M1 chip, you will have to build Solana from the source. See [this document](https://docs.solana.com/cli/install-solana-cli-tools#build-from-source) for more details

Next, we will build and deploy the program via Anchor.

First, let's build the program:

```
$ anchor build
```

Deploy the program:

```
$ anchor deploy
```

Finally, run the test:

```
$ anchor test
```

> Make sure to terminate the `solana-test-validator` before you run the `test` command
