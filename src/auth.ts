import { SvelteKitAuth } from "@auth/sveltekit";
import { ZodError } from "zod";
import Credentials from "@auth/sveltekit/providers/credentials";

export const { handle, signIn, signOut } = SvelteKitAuth({
  providers: [
    Credentials({
      // HACK: we can't call the rust backend here
      // so we will do the actual validation call in the client side
      // we won't do any real authenticatation, we will just trick Auth.js
      // to persist the user data
      credentials: {
        user: {},
      },
      authorize: async (credentials) => {
        console.log("LOGIN ATTEMPT:", JSON.stringify(credentials));
        // TODO: refactor this shit
        try {
          const { user } = credentials;
          if (!user) {
            return null;
          }
          console.log("login success:", JSON.stringify(credentials));
          const user_data = JSON.parse(user as string);
          console.log("[USER_DATA]:", user_data);
          return user_data;
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
  pages: {
    signIn: "/login",
    signOut: "/login",
  },
});
