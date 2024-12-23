export enum IpcErrorActionType {
  None,
  Redirect,
}

export enum ErrorType {
  Expected,
  User,
  Internal,
}

export type ErrorPayload = {
  error_type: ErrorType;
  message: String;
  error: String;
  details: String;
  action_type: IpcErrorActionType;
};

export type IPCUserType = {};
