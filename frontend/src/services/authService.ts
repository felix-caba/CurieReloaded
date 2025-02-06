import { baseApi } from './baseApi'
import type { Auxiliar, Material, Reactivo } from '../types/product'
import { LoginRequest, User } from '../types/user';



export type LoginResponse = User & {
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