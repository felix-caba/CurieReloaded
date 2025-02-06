import { z } from "zod";

const usernameschema = z.string().min(5, "El nombre de usuario debe tener al menos 5 caracteres");
const passwordschema = z.string().min(5, "La contraseña debe tener al menos 5 caracteres");


export const userSchema = z.object({
    idUsuario: z.number().optional(),
    username: usernameschema,
    email: z.string().email(),
    password: passwordschema,
});

export const loginSchema = z.object({
    username: usernameschema,
    password: passwordschema,
});


export type User = z.infer<typeof userSchema>;
export type LoginRequest = z.infer<typeof loginSchema>;