export const getPersistentUUID = function () {
  const KEY = 'browser_uuid';
  let uuid = localStorage.getItem(KEY);
  if (!uuid) {
    uuid = crypto.randomUUID();
    localStorage.setItem(KEY, uuid);
  }
  return uuid;
};
