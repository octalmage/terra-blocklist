module.exports = ({ wallets, refs, config, client }) => ({
  blocked: (address) => client.query("cw20-blocklist", { is_blocked: { address } }),
  balance: (address) => client.query("cw20-blocklist", { balance: { address } }),
  mint: (recipient, amount, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { mint: { recipient, amount } }),    
  block: (address, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { add_to_blocked_list: { address } }),    
  redeem: (amount, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { redeem: { amount } }),    
  destroyBlockedFunds: (address, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { destroy_blocked_funds: { address } }),    
  unblock: (address, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { remove_from_blocked_list: { address } }),    
  transfer: (recipient, amount, signer = wallets.validator) => client.execute(signer, "cw20-blocklist", { transfer: { recipient, amount } }),    
});
