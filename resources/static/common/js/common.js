const HyperlaneCommon = {
  ...Toast,
  ...DateUtil,
  ...NumberUtil,
  ...FunctionUtil,
  ...DOMUtil,
  ...URLUtil,
  ...StringUtil,
  ...ValidateUtil,
  ...StorageUtil,
  ...ObjectUtil,
  ...FileUtil,
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = HyperlaneCommon;
}
