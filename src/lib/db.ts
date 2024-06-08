import { invoke } from "@tauri-apps/api/tauri";
type User = {
  id?: string;
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

const get_user_from_db = async (
  email: string,
  password: string,
): Promise<User> => {
  console.log("BEFORE INVOKE");
  const resp = await invoke("authenticate", {
    email: email,
    password: password,
  });
  return {
    id: "1",
    name: "John Doe",
    email: email,
  };
};

export { get_user_from_db };
export type { User };
