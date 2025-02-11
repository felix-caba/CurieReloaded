import { toast } from "@backpackapp-io/react-native-toast";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";
import { z } from "zod";

interface ErrorResponse {
    error: {
        code: number;
        reason: string;
        description: string;
    }
}

function isFetchBaseQueryError(error: any): error is FetchBaseQueryError {
    return 'status' in error; 
 }

export function handleErrorMessage(error: any) {
    if (error instanceof z.ZodError) {
        const errorMessages = error.errors.map((error) => error.message);
        errorMessages.forEach((message) => {
            toast.error(message);
        });
    }
    else if (isFetchBaseQueryError(error)) {
        const errorData = error.data as ErrorResponse;
        if (errorData && errorData.error) {
            toast.error(errorData.error.description);
        } else {
            toast.error(`An unexpected error occurred: ${error.status}`);
        }
    }
    else {
        console.log(error);
    }
}