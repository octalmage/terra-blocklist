# CW20 Blocklist

This is a sample contract that extends the cw20-base to add blocklist functionality.

Note: This implementation does not implement the `MarketingInfo` or `DownloadLogo` queries since these are [not used on Terra](https://lcd.terra.dev/wasm/contracts/terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76/store?query_msg=%7B%22MarketingInfo%22%3A%20%7B%7D%7D).

## Functionality

In addition to the normal CW20-base methods and queries, the following execute messages are available: 

```rust
AddToBlockedList {
    address: String,
},
RemoveFromBlockedList {
    address: String,
},
DestroyBlockedFunds {
    address: String,
},
UpdateMinter {
    address: String,
},
```

`AddToBlockList` allows the owner to add a user to the internal blocklist. When this happens, the funds are effectively frozen. 

`RemoveFromBlockedList` allows the owner to do undo `AddToBlockList`.

`DestroyBlockedFunds` allows the owner of the contract to burn funds in any wallet currently on the blocklist.

`UpdateMinter` allows the owner to update the address that is allowed to mint. Useful for migrations to a new multisig. 

New query message added: 

```
IsBlocked {
    address: String,
},
```

`IsBlocked` can be used to see if an address is currently blocked. 
