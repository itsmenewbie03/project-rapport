import { goto } from "$app/navigation";

/**
 * Work around for goto() not working in tauri
 * */
const _goto = async (url: string) => {
  setTimeout(async () => {
    await goto(url);
  }, 0);
};

export { _goto as goto };
