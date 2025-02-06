import * as Keychain from "react-native-keychain";
import { User } from "../types/user";

export const saveCredentials = async (user: User, token: string) => {
  await Keychain.setGenericPassword(user.username, token);
};

const getCredentials = async () => {
  try {
    const credentials = await Keychain.getGenericPassword();
    if (credentials) {
      return { user: credentials.username, token: credentials.password };
    }
  } catch (error) {
    console.error("Error al obtener credenciales:", error);
  }
  return null;
};

export const deleteCredentials = async () => {
  await Keychain.resetGenericPassword();
};