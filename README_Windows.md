# Berry Club Bot Workshop

This is a workshop to cover basics of NEAR Protocol smart-contracts written in Rust.

## Preparation

**Note**: after installing each of these steps, you may need to close the Command Prompt and reopen it for changes to take effect.

### Install required tools

NOTE: This process is Windows. For other OSes please see `README.md`.

#### Install [Rust and rustup](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup):

#### Add wasm target to your toolchain:

```bash
rustup target add wasm32-unknown-unknown
```

You will also need a [Windows-specific download](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16), then open it and install the C++ packages:

![Choose C++](./assets/vsbuild.png)

#### Install `near-cli`

Note: you'll need [to install NodeJS](https://nodejs.org/en/download).

See [`near-cli` installation docs](https://docs.near.org/docs/development/near-cli#installation)

#### Install `git`

See [installation guide](https://github.com/git-guides/install-git#install-git-on-windows) from GitHub.

### Prepare workshop repo and verify tools installation

#### Clone the workshop repository

```bash
git clone https://github.com/evgenykuzyakov/workshop
cd workshop
```

It should clone the repository to a local folder `workshop`.

#### Compile the contract

```bash
win\build.bat
```

If you have successfully installed Rust and `wasm32` target, then the `cargo build…` command should compile the contract into `res/berry_bot.wasm`.
```
   Compiling autocfg v1.0.0
   Compiling proc-macro2 v1.0.9
   Compiling unicode-xid v0.2.0
   ...
   Compiling near-vm-logic v2.0.0
   Compiling near-sdk v2.0.0
   Compiling berry-bot v0.1.0 (workshop)
    Finished release [optimized] target(s) in 43.13s
```

#### You can check that the contract is present in `res/berry_bot.wasm`

```bash
dir res
```

I hope you see:

```shell
…
05/02/2021  01:55 PM           148,112 berry_bot.wasm
…
```

### Setup NEAR account

#### Register a new account on testnet.

You can create a new account using [NEAR Testnet Wallet](https://wallet.testnet.near.org/)
It'll create a new account for you on the NEAR Testnet. The full account ID will be something like `alice.testnet`

As a `Security Method` for this workshop I'd recommend to use either `Recovery Phrase` or `Email Recovery`.

#### Authorize your account with `near-cli`

To allow using your account with `near-cli` you need to login from the terminal.
```bash
near login
```

This should open a new browser tab in the NEAR Testnet web-wallet and ask you give `full access` to your account.
It's fine to do, since we're talking about Testnet account.
You'll need access from command line and `near-cli` for this workshop.

Once you authorized it in the browser, the command line should automatically succeed.

You should see something like this in the console:
```
Logged in as [ alice.testnet ] with public key [ ed25519:HP3oxy... ] successfully
```

#### Store your account ID into local variable

To help with this workshop let's store your account ID into `ACCOUNT_ID` variable in bash.
Replace `<YOUR_ACCOUNT_ID>` with your actual account ID that you created in the wallet, e.g. `alice.testnet`.

```bash
set ACCOUNT_ID=<YOUR_ACCOUNT_ID>
```

#### Verification

Let's verify that you've successfully created the account and added it to `near-cli`.

Run the following:
```bash
near call --accountId=%ACCOUNT_ID% workshop.testnet hello
```

If it succeeded then you've successfully completed your account setup. You should see something like this:
```
Scheduling a call: workshop.testnet.hello()
Receipt: 5rKUqv4t9JVQryvyfrgrFr8R48iV4sFX7nD56KUv6Vhb
	Log [workshop.testnet]: Hello, test-12331.testnet!
Transaction Id 8D2L4AdhbZ3CqWXMpRURsyqUTNJaBJDcFQsN4W8vU4y7
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/8D2L4AdhbZ3CqWXMpRURsyqUTNJaBJDcFQsN4W8vU4y7
'Hello, test-12331.testnet!'
```

**Congrats!**

## Part 1 - Fix my code

Guess what, I made a bunch of errors in the implementation and you need to fix them.
Good thing, that I've added unit tests before I broke the code.

Run unit tests
```bash
win\test.bat
```

You'll see a failed test. E.g.
```
running 1 test
test rect::tests::test_rect_complex ... FAILED

failures:

---- rect::tests::test_rect_complex stdout ----

00 ..................................................
01 ..................................................
02 ..................................................
03 .....XXXX.........................................
04 .....XXXX.........................................
05 .....XXXX.........................................
06 .....XXXX.........................................
07 ..................................................
08 ..................................................
09 ..................................................
10 ..................................................
11 ..................................................
12 ..................................................
13 ..................................................
14 ..................................................
15 ..................................................
16 ..................................................
17 ..................................................
18 ..................................................
19 ..................................................
20 ..................................................
21 ..................................................
22 ..................................................
23 ..................................................
24 ..................................................
25 ..................................................
26 ..................................................
27 ..................................................
28 ..................................................
29 ..................................................
30 ..................................................
31 ..................................................
32 ..................................................
33 ..................................................
34 ..................................................
35 ..................................................
36 ..................................................
37 ..................................................
38 ..................................................
39 ..................................................
40 ..................................................
41 ..................................................
42 ..................................................
43 ..................................................
44 ..................................................
45 ..................................................
46 ..................................................
47 ..................................................
48 ..................................................
49 ..................................................
thread 'rect::tests::test_rect_complex' panicked at 'assertion failed: `(left == right)`
  left: `".....XXXX......"`,
 right: `"..............."`: Line 3', src/rect.rs:90:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

It means the test `test_rect_complex` has failed and the assert at file `src/rect.rs` line `90` failed.
Open the file and try to figure out what went wrong. Once you figured out what is wrong, try rerunning tests.

Repeat the process of fixing the implementation (not tests) to resolve all bugs.

Once all unit tests are fixed, you're ready to move to the part 2.

## Part 2 - Let's draw

In this part you'll need to implement a new method that will draw something complex.
To do this, you can use rectangle and circle primitives and later merge them into one image.

Run
```bash
win\draw_art.bat
```

You should see a preview of the art. Right now it renders a target with three circles.

```
00 ..................................................
01 ..................................................
02 ..................................................
03 ..................................................
04 ..................................................
05 .................XXXXXXX..........................
06 ..............XXX.......XXX.......................
07 ............XX.............XX.....................
08 ...........X.................X....................
09 ..........X...................X...................
10 .........X.......XXXXXXX.......X..................
11 ........X......XXX.....XXX......X.................
12 .......X......X...........X......X................
13 .......X.....X.............X.....X................
14 ......X.....X...............X.....X...............
15 ......X....X......XXXXX......X....X...............
16 ......X....X.....XX...XX.....X....X...............
17 .....X....XX....X.......X....XX....X..............
18 .....X....X....XX.......XX....X....X..............
19 .....X....X....X.........X....X....X..............
20 .....X....X....X.........X....X....X..............
21 .....X....X....X.........X....X....X..............
22 .....X....X....XX.......XX....X....X..............
23 .....X....XX....X.......X....XX....X..............
24 ......X....X.....XX...XX.....X....X...............
25 ......X....X......XXXXX......X....X...............
26 ......X.....X...............X.....X...............
27 .......X.....X.............X.....X................
28 .......X......X...........X......X................
29 ........X......XXX.....XXX......X.................
30 .........X.......XXXXXXX.......X..................
31 ..........X...................X...................
32 ...........X.................X....................
33 ............XX.............XX.....................
34 ..............XXX.......XXX.......................
35 .................XXXXXXX..........................
36 ..................................................
37 ..................................................
38 ..................................................
39 ..................................................
40 ..................................................
41 ..................................................
42 ..................................................
43 ..................................................
44 ..................................................
45 ..................................................
46 ..................................................
47 ..................................................
48 ..................................................
49 ..................................................
test art::tests::draw_art ... ok
```

Now, open file `src/art.rs` and modify the implementation of method `internal_render_art` at line `26`.

You can draw whatever you want and debug it by running `win\draw_art.bat`.
But note, that too many pixels might lead to performance issues.

Once you are satisfied with your art preview, it's time to try it for real.

## Part 3 - Deploy to testnet

### Recompile the contract

You've modified the code to fix tests and implemented your art, so we need to rebuild.
```bash
win\build.bat
```

Every time you modify code of your contract you may want to recompile the contract.
The resulting binary is be located at `res/berry_bot.wasm`

### Deploying the contract

Accounts in NEAR Protocol can also contain one contract. The contract has full access to the account it belongs.
But a new transaction can only be initiated by signing this transaction with an access key (this stops Skynet and singularity).
A transaction may call a method on the contract by name and pass arguments.

To deploy a contract, we need to issue a transaction.
I wrote a convenient script to deploy the contract to your account stored in `%ACCOUNT_ID%`.
(All it does is `near deploy %ACCOUNT_ID% res\berry_bot.wasm`)
```bash
win\deploy.bat
```

You should see something like this:

```
Starting deployment. Account id: test-12331.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/berry_bot.wasm
Transaction Id 7jxk9ANig8wJ1BqFennJN4pSuVaMn9n3HjaeKfDYfsDY
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/7jxk9ANig8wJ1BqFennJN4pSuVaMn9n3HjaeKfDYfsDY
Done deploying to test-12331.testnet
```

NOTE: You can re-deploy your contract multiple times, so long as you have access key on the account.
Every time you've modified the code and want to change the contract on-chain you need to recompile and redeploy.

### Call the contract

To call `<METHOD_NAME>` on your contract with arguments `<ARGS>` issue the following command:
```bash
near call %ACCOUNT_ID% --accountId=%ACCOUNT_ID% --gas=300000000000000 <METHOD_NAME> <ARGS>
```

* `<METHOD_NAME>` is a public method name that you want to call from the contract
* `<ARGS>` are JSON encoded arguments to the method, e.g. `"{\"left\": 10, \"top\": 20, \"width\": 10, \"height\": 5, \"color\": 16711680}"`

For example to draw you art, you need to call the following:
```bash
near call %ACCOUNT_ID% --accountId=%ACCOUNT_ID% --gas=300000000000000 render_art "{}"
```

If everything works well, then you should see your art rendered in logs, e.g.:
```
Scheduling a call: test-12331.testnet.render_art({})
Receipts: B3odJ16wnajwsrUbw2PVZyY2QDCbwFJsTAXvsWTaGGZV, 7FMyKSgszNgGhCBYv7Sg7jVS2b2LzTJ4w8e9T1fTEDWV
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: .................XXXXXXX..........................
	Log [test-12331.testnet]: ..............XXX.......XXX.......................
	Log [test-12331.testnet]: ............XX.............XX.....................
	Log [test-12331.testnet]: ...........X.................X....................
	Log [test-12331.testnet]: ..........X...................X...................
	Log [test-12331.testnet]: .........X.......XXXXXXX.......X..................
	Log [test-12331.testnet]: ........X......XXX.....XXX......X.................
	Log [test-12331.testnet]: .......X......X...........X......X................
	Log [test-12331.testnet]: .......X.....X.............X.....X................
	Log [test-12331.testnet]: ......X.....X...............X.....X...............
	Log [test-12331.testnet]: ......X....X......XXXXX......X....X...............
	Log [test-12331.testnet]: ......X....X.....XX...XX.....X....X...............
	Log [test-12331.testnet]: .....X....XX....X.......X....XX....X..............
	Log [test-12331.testnet]: .....X....X....XX.......XX....X....X..............
	Log [test-12331.testnet]: .....X....X....X.........X....X....X..............
	Log [test-12331.testnet]: .....X....X....X.........X....X....X..............
	Log [test-12331.testnet]: .....X....X....X.........X....X....X..............
	Log [test-12331.testnet]: .....X....X....XX.......XX....X....X..............
	Log [test-12331.testnet]: .....X....XX....X.......X....XX....X..............
	Log [test-12331.testnet]: ......X....X.....XX...XX.....X....X...............
	Log [test-12331.testnet]: ......X....X......XXXXX......X....X...............
	Log [test-12331.testnet]: ......X.....X...............X.....X...............
	Log [test-12331.testnet]: .......X.....X.............X.....X................
	Log [test-12331.testnet]: .......X......X...........X......X................
	Log [test-12331.testnet]: ........X......XXX.....XXX......X.................
	Log [test-12331.testnet]: .........X.......XXXXXXX.......X..................
	Log [test-12331.testnet]: ..........X...................X...................
	Log [test-12331.testnet]: ...........X.................X....................
	Log [test-12331.testnet]: ............XX.............XX.....................
	Log [test-12331.testnet]: ..............XXX.......XXX.......................
	Log [test-12331.testnet]: .................XXXXXXX..........................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
	Log [test-12331.testnet]: ..................................................
Transaction Id ZWv4Ac2Qqvs8cA1CT1AHE6ovNXUGT7SCuLiDQFmSqv3
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/ZWv4Ac2Qqvs8cA1CT1AHE6ovNXUGT7SCuLiDQFmSqv3
''
```

You can also click the explorer link to see this on chain, e.g [Target](https://explorer.testnet.near.org/transactions/ZWv4Ac2Qqvs8cA1CT1AHE6ovNXUGT7SCuLiDQFmSqv3)

There is a helper script that let you call methods on your contract easier `win\call.bat`.
```bash
win\call.bat <METHOD_NAME> <ARGS>
```

An example of a command to render a red rectangle
```bash
win\call.bat render_rect "{\"left\": 10, \"top\": 20, \"width\": 10, \"height\": 5, \"color\": 16711680}"
```

Or to render your art
```bash
win\call.bat render_art "{}"
```

## Part 4 - Test in Prod

If you were able to see your art in the explorer, you've already made the history on Testnet. It's now persistent on chain.
The next step is to use it with the real app.

### Build for real

Your current contract doesn't actually issue cross-contract calls to berry club, so you need to re-compile it with a compilation feature.
```bash
win\build_for_real.bat
```

Now redeploy your contract
```bash
win\deploy.bat
```

Game on.

### Buy some 🥑

The first thing we'll need is to buy some avocados to draw in the berry club.
Your contract has a helper method `buy_avocado` to do this.
```bash
win\call.bat buy_avocado "{}"
```

You should see a log message like this:
```
	Log [test-12331.testnet]: Purchased 15001.500 Avocado tokens for 50.000 NEAR
```

`50 NEAR` is enough to get you `15000` 🥑 and this is enough to draw `15000` pixels.

### Open Berry Club on the Testnet

[test.berryclub.io](https://test.berryclub.io)

You should see a board. It might be messy, because everyone shares the same board.
You can login with your testnet account using web-wallet. This will display your name and your avocado balance.

Keep this tab open.

### Let's render your art

```bash
win\call.bat render_art "{}"
```

You should see it rendered on the board in the browser.
You should also see your account ID in the list on the right.
If you hover over your account ID in the list, it will highlight your pixels.

### Iterate

Now try drawing other primitives by using `win\call.bat`

Remember, if you modify the code, then you need to recompile and redeploy the contract.

Enjoy!