import { combineReducers, configureStore } from '@reduxjs/toolkit'
import { setupListeners } from '@reduxjs/toolkit/query'
import { productApi } from '../services/productService'
import { ErrorMiddleware } from '../middleware/apiErrorHandling'
import { baseApi } from '../services/baseApi'

const rootReducer = combineReducers({
  [baseApi.reducerPath]: baseApi.reducer,
})

export const store = configureStore({
    reducer: rootReducer,
    middleware: (getDefaultMiddleware) =>
      getDefaultMiddleware().
    concat(baseApi.middleware, ErrorMiddleware),
  })
  
  setupListeners(store.dispatch)

  export type RootState = ReturnType<typeof store.getState>
  export type AppDispatch = typeof store.dispatch
  export type AppStore = typeof store