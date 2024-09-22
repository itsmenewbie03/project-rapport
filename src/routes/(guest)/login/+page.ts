import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';
export const load = (async () => {
  const session = await invoke('get_session');
  return {
    session,
  };
}) satisfies PageLoad;
