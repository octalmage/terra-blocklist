# CW20 Blocklist

This is a sample contract that extends the cw20-base to add blocklist functionality. 

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
