import { goto } from "$app/navigation";

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
      localStorage.removeItem("auth_token");
      resolve();
    }, 1000);
  });
}

export { _goto as goto, logout };
