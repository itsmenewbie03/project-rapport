import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/tauri";
export const load = (async () => {
  const token = localStorage.getItem("auth_token");
  const session = await invoke("get_session", { token: token });
  console.log(`[RUST RETURNED]: ${JSON.stringify(session)}`);
  return {
    session: session,
  };
}) satisfies PageLoad;
