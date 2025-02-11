import { baseApi } from './baseApi'
import type { Auxiliar, Material, Reactivo } from '../types/product'
import { LoginRequest, User } from '../types/user';
import { getCredentials } from '../handlers/keychainHandler';

export const keyChainToken = async () => {
  const credentials = await getCredentials();
  return credentials?.token;
}

export type LoginResponse = {
    user: User;
    token: string;
}

export const authApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    login: builder.mutation<LoginResponse, LoginRequest>({
      query: (loginRequest) => ({
        url: '/auth/login',
        method: 'POST',
        body: loginRequest, 
      }),
      
    }),
  }),
})



export const { useLoginMutation } = authApi