import { z } from "zod";
import { signUpSchema } from "./signup";

const loginSchema = signUpSchema.omit({
  firstName: true,
  lastName: true,
});

export type SignInType = z.infer<typeof loginSchema>;
