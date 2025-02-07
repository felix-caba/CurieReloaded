  import { baseApi } from './baseApi'
  import type { Auxiliar, Material, Reactivo } from '../types/product'

  export const productApi = baseApi.injectEndpoints({
    endpoints: (builder) => ({
      getReactivos: builder.query<Reactivo[], void>({
        query: () => ({
          url: '/reactivo/select',
          method: 'GET',
        }),
       
      

      }),



      
    }),
  })

  export const { useGetReactivosQuery } = productApi