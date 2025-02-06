import { combineReducers, configureStore } from '@reduxjs/toolkit'
import { setupListeners } from '@reduxjs/toolkit/query'
import { ErrorMiddleware } from '../middleware/apiErrorHandling'
import { baseApi } from '../services/baseApi'
import { persistStore, persistReducer, FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER } from 'redux-persist'
import mmkvStorage from './mmkvStorage'
import authSlice from '../slices/authSlice'


const rootReducer = combineReducers({
  [baseApi.reducerPath]: baseApi.reducer,
  authSlice: authSlice,
})


const persistConfig = {
  key: 'root',
  storage: mmkvStorage,
  whitelist: [authSlice.name],
}

const persistedReducer = persistReducer(persistConfig, rootReducer)

export const store = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) =>
      getDefaultMiddleware({
        serializableCheck: {
          ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
        },
      }).
    concat(baseApi.middleware, ErrorMiddleware),

  })
  
  setupListeners(store.dispatch)

  export type RootState = ReturnType<typeof store.getState>
  export type AppDispatch = typeof store.dispatch
  export type AppStore = typeof store

  export const persistor = persistStore(store)