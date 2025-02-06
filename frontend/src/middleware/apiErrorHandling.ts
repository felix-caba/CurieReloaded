import { toast } from '@backpackapp-io/react-native-toast'
import { isRejectedWithValue } from '@reduxjs/toolkit'
import type { MiddlewareAPI, Middleware } from '@reduxjs/toolkit'


export const ErrorMiddleware: Middleware =
  (api: MiddlewareAPI) => (next) => (action) => {
    if (isRejectedWithValue(action)) {
      toast.error("Xd")
      console.log(action.error)
      toast.error(action.error.message || 'Error desconocido')
    }
    return next(action)
  }