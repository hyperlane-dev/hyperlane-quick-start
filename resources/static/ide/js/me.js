console.log(
  '%c 如有疑问请联系邮箱 %c root@ltpp.vip %c ',
  'background:#35495e ; padding: 1px; border-radius: 3px 0 0 3px;  color: #fff',
  'background:#41b883 ; padding: 1px; border-radius: 0 3px 3px 0;  color: #fff',
  'background:transparent',
);
console.log(
  '%c ' +
    '                  _oo0oo_                   \n                  o8888888o				     \n                  88" . "88				     \n                  (| -_- |)				     \n                   O\\ = /O				     \n' +
    "               ____/`---'\\____			     \n             .   ' \\\\| |// `.			     \n" +
    '              / \\\\||| : |||// \\				 \n           / _||||| -卍- |||||- \\		     \n              | | \\\\\\ - /// | |				 \n' +
    "            | \\_| ''\\---/'' | |				 \n" +
    '             \\ .-\\__ `-` ___/-. /			 \n' +
    "          ___`. .' /--.--\\ `. . __		     \n" +
    '       ."" "< `.___\\_<|>_/___. ` >" "".      \n      | | : `- \\`.;`\\ _ /`;.`/ - ` : | |     \n        \\ \\ `-. \\_ __\\ /__ _/ .-` / /        \n' +
    "======`-.____`-.___\\_____/___.-`____.-'======\n                   `=---='                   \n" +
    '.............................................\n      佛祖镇楼                  BUG辟易       \n',
  'background:#35495e ;  color: yellow',
);
document.cookie = 'SameSite=Lax; Secure; Max-Age=3153600000';
window.addEventListener(
  'mousewheel',
  function (e) {
    e = e || window.event;
    if ((e.wheelDelta && event.ctrlKey) || e.detail) {
      event.preventDefault();
    }
  },
  { capture: false, passive: false },
);
window.addEventListener('keydown', function (e) {
  if (e.ctrlKey && e.shiftKey) {
    e.preventDefault();
    return;
  }
  if (e.ctrlKey === true) {
    if (
      e.keyCode == 65 ||
      e.keyCode == 67 ||
      e.keyCode == 86 ||
      e.keyCode == 87 ||
      e.keyCode == 89 ||
      e.keyCode == 90 ||
      e.keyCode == 88
    ) {
      return;
    }
    e.preventDefault();
  }
  if (e.keyCode == 116 || e.keyCode == 123) {
    e.preventDefault();
  }
});
window.addEventListener('contextmenu', function (e) {
  e.preventDefault();
});
