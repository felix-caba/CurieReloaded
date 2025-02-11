  import { baseApi } from './baseApi'
  import type { Auxiliar, Material, Reactivo } from '../types/product'
import { keyChainToken } from './authService'

  export const productApi = baseApi.injectEndpoints({
    endpoints: (builder) => ({
      getReactivos: builder.query<Reactivo[], void>({
        query: () => ({
          url: '/reactivo/select',
          method: 'GET',
          headers: {
            'Authorization': `Bearer ${keyChainToken()}`
          },
        }),
        providesTags: ['Reactivo'],
      }),

      createReactivo: builder.mutation<Reactivo, Reactivo>({
        query: (reactivo) => ({
          url: '/reactivo/insert',
          method: 'POST',
          body: reactivo,
          headers: {
            'Authorization': `Bearer ${keyChainToken()}`
          },
        }),
        // invalida los tags dentro del cache, para que se actualice la lista de reactivos y no tire del cache del rtk
        invalidatesTags: ['Reactivo'],
      }),



      
    }),
  })

  export const { useGetReactivosQuery } = productApi