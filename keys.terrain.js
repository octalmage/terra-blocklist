// can use `process.env.SECRET_MNEMONIC` or `process.env.SECRET_PRIV_KEY`
// to populate secret in CI environment instead of hardcoding

module.exports = {
  bombay: {
    mnemonic: process.env.SECRET_MNEMONIC,
  },
};
