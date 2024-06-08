import { goto } from "$app/navigation";

/**
 * Work around for goto() not working in tauri
 * */
const _goto = async (url: string) => {
  return new Promise(async (resolve) => {
    setTimeout(async () => {
      resolve(await goto(url));
    }, 500);
  });
};

export { _goto as goto };
