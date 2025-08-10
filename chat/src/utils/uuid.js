import { v4 as uuidv4 } from 'uuid';

export const getPersistentUUID = function () {
  const KEY = 'browser_uuid';
  let uuid = localStorage.getItem(KEY);
  if (!uuid) {
    uuid = uuidv4();
    localStorage.setItem(KEY, uuid);
  }
  return uuid;
};
