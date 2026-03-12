const HyperlaneCommon = {
  ...(typeof Toast !== 'undefined' ? Toast : {}),
  ...(typeof DateUtil !== 'undefined' ? DateUtil : {}),
  ...(typeof NumberUtil !== 'undefined' ? NumberUtil : {}),
  ...(typeof FunctionUtil !== 'undefined' ? FunctionUtil : {}),
  ...(typeof DOMUtil !== 'undefined' ? DOMUtil : {}),
  ...(typeof URLUtil !== 'undefined' ? URLUtil : {}),
  ...(typeof StringUtil !== 'undefined' ? StringUtil : {}),
  ...(typeof ValidateUtil !== 'undefined' ? ValidateUtil : {}),
  ...(typeof StorageUtil !== 'undefined' ? StorageUtil : {}),
  ...(typeof ObjectUtil !== 'undefined' ? ObjectUtil : {}),
  ...(typeof FileUtil !== 'undefined' ? FileUtil : {}),
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = HyperlaneCommon;
}
