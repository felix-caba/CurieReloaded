import { createApi } from "@reduxjs/toolkit/query/react"

import { fetchBaseQuery } from "@reduxjs/toolkit/query/react"
import { RootState } from "../store/store"

export const baseApi = createApi({
    tagTypes: ['Reactivo', 'User'],
    reducerPath: 'api',
    baseQuery: fetchBaseQuery({ 
      baseUrl: 'http://10.0.2.2:8000/api',

    }),
    endpoints: () => ({}),
  })

  