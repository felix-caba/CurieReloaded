import { toast } from '@backpackapp-io/react-native-toast'
import { isRejectedWithValue } from '@reduxjs/toolkit'
import type { MiddlewareAPI, Middleware } from '@reduxjs/toolkit'
import { handleErrorMessage } from '../handlers/errorHandler'


export const ErrorMiddleware: Middleware =
  (api: MiddlewareAPI) => (next) => (action) => {
    if (isRejectedWithValue(action)) {
        handleErrorMessage(action.error)
    }
    return next(action)
  }