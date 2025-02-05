// This file has been generated by Specta. DO NOT EDIT.

import { createTauRPCProxy as createProxy, type InferCommandOutput } from 'taurpc'
type TAURI_CHANNEL<T> = (response: T) => void


export type ErrorAction = "None" | "Redirect"

export type ErrorPayload = { error_type: ErrorType; message: string; error: string; details: string; action_type: ErrorAction }

export type ErrorType = "Expected" | "User" | "Internal"

export type IpcUser = { full_name: string; email: string }

const ARGS_MAP = { 'auth':'{"logout":[],"signUp":["full_name","email","password"],"getSession":[],"login":["email","password"]}' }
export type Router = { 'auth': { signUp: (fullName: string, email: string, password: string) => Promise<IpcUser>, 
login: (email: string, password: string) => Promise<IpcUser>, 
getSession: () => Promise<IpcUser>, 
logout: () => Promise<null> } };


export type { InferCommandOutput }
export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)
