# Terra CW20 Blocklist

## Deploy 

With [LocalTerra](https://github.com/terra-money/localterra) running and [Terrain](https://docs.terra.money/docs/develop/dapp/quick-start/initial-setup.html) installed, you can deploy with the following command: 

```
terrain deploy cw20-blocklist --signer validator
```

## Interact with deployed CW20

First start the Terrain console: 

```
terrain console
```

Then you can use the built in commands to interact with the deployed contract: 

```sh
terrain > await lib.blocked('terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8');
{ blocked: false }
terrain > await lib.mint('terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8', '100000000');
{
  txhash: '7C7152831C2F856D8F673ADBF0FC798D376E5BD0B278ED9DF3C857B9AA7C1475',
}
terrain > await lib.block('terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8');
{
  txhash: 'E75EA60410F65588AD1ED45B39B7FF0A6F1CBED5A4F99EBE3C6F2'
}
terrain > await lib.blocked('terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8');
{ blocked: true }
terrain > await lib.transfer('terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v', '1000');
Uncaught Error: Request failed with status code 400
    data: {
      code: 3,
      message: 'failed to execute message; message index: 0: Address is on the blocklist: execute wasm contract failed: invalid request',
      details: []
    }
```