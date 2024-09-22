import { goto } from '$app/navigation';

/**
 * We intentionally introduce a 1 second delay for aesthetics xD
 * */
const _goto = async (url: string) => {
  return new Promise(async (resolve) => {
    setTimeout(async () => {
      resolve(await goto(url));
    }, 1000);
  });
};
/**
 * We intentionally introduce a 1 second delay for aesthetics xD
 * */
const logout = async (): Promise<void> => {
  return new Promise(async (resolve) => {
    setTimeout(async () => {
      localStorage.removeItem('auth_token');
      resolve();
    }, 1000);
  });
};

const to_timestamp = (date: string): string => {
  // NOTE: we will strip the last 3 characters of the timestamp
  // so it will work with SQLite
  return new Date(date).getTime().toString().substring(0, 10);
};

export { _goto as goto, logout, to_timestamp };
