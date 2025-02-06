import { createSlice } from "@reduxjs/toolkit";
import { userSchema } from "../types/user";
import { handleErrorMessage } from "../handlers/errorHandler";
import { User } from "../types/user";
import { deleteCredentials, saveCredentials } from "../handlers/keychainHandler";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import mmkvStorage from "../store/mmkvStorage";

interface AuthState {
    user: User | null;
}





const authSlice = createSlice({
    name: 'authSlice',
    initialState: {
        user: null,
    } as AuthState,
    reducers: {
        saveUserToState: (state, action) => {
            try {
                const { user, token } = action.payload;
                state.user = user;

                if (user) {
                    saveCredentials(user, token);
                }
                
            } catch (error) {
                handleErrorMessage(error);
            }

            console.log("Usuario en el estado: " + state.user?.username);
        },
        onLogout: (state) => {
            state.user = null;
            deleteCredentials();
        },
    },
});


export const useIsSignedIn = (): boolean => {
    return useSelector((state: RootState) => state.authSlice.user !== null);
}

export const useCurrentUser = () => {
    return useSelector((state: RootState) => state.authSlice.user);
};

export const { saveUserToState, onLogout } = authSlice.actions;
export default authSlice.reducer;