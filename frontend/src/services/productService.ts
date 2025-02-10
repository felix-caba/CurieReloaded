  import { baseApi } from './baseApi'
  import type { Auxiliar, Material, Reactivo } from '../types/product'

  export const productApi = baseApi.injectEndpoints({
    endpoints: (builder) => ({
      getReactivos: builder.query<Reactivo[], void>({
        query: () => ({
          url: '/reactivo/select',
          method: 'GET',
        }),
        providesTags: ['Reactivo'],
      }),

      createReactivo: builder.mutation<Reactivo, Reactivo>({
        query: (reactivo) => ({
          url: '/reactivo/insert',
          method: 'POST',
          body: reactivo,
        }),
        // invalida los tags dentro del cache, para que se actualice la lista de reactivos y no tire del cache del rtk
        invalidatesTags: ['Reactivo'],
      }),



      
    }),
  })

  export const { useGetReactivosQuery } = productApi