import { SvelteKitAuth } from "@auth/sveltekit";
import { ZodError } from "zod";
import Credentials from "@auth/sveltekit/providers/credentials";
import { signInSchema } from "./lib/zod";
// Your own logic for dealing with plaintext password strings; be careful!
import { hash } from "$lib/hash";
import { get_user_from_db } from "$lib/db";

export const { handle, signIn, signOut } = SvelteKitAuth({
  providers: [
    Credentials({
      // You can specify which fields should be submitted, by adding keys to the `credentials` object.
      // e.g. domain, username, password, 2FA token, etc.
      credentials: {
        email: {},
        password: {},
      },
      authorize: async (credentials) => {
        console.log("LOGIN ATTEMPT:", JSON.stringify(credentials));
        // TODO: refactor this shit
        try {
          let user = null;

          // @ts-ignore
          const { email, password } =
            await signInSchema.parseAsync(credentials);
          const pwHash = await hash(password);
          user = await get_user_from_db(email, pwHash);
          if (!user) {
            return null;
          }
          console.log("login success:", JSON.stringify(credentials));
          return user;
        } catch (error) {
          if (error instanceof ZodError) {
            return null;
          }
          // NOTE: we need to add this as without this we will return undefined
          // an we will violate the return type
          console.log("FUCK:", error);
          return null;
        }
      },
    }),
  ],
});
