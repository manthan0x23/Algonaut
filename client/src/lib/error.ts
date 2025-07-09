import type { AxiosError } from "axios";



export const getAxiosResponseError = (err: Error) => {
  return ((err as AxiosError).response?.data as { error?: string })?.error;
};

export type ApiError = AxiosError<{ message: string; error: string[] }>;
