export enum IpcErrorActionType {
  None,
  Redirect,
}

export enum IpcErrorType {
  Expected,
  User,
  Internal,
}

export type IpcErrorPayload = {
  error_type: IpcErrorType;
  message: string;
  error: string;
  details: string;
  action_type: IpcErrorActionType;
};

export type IpcLoginType = {
  email: string;
  password: string;
};

export type IpcSignUpType = {
  fullName: string;
} & IpcLoginType;

export type IpcUserType = {
  full_name: string;
  email: string;
};

export enum InvokableFunctions {
  SignUp = "sign_up",
  Login = "login",
  GetSession = "get_session",
  Logout = "logout",
}
