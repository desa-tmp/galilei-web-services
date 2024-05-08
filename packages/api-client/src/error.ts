import { components } from "./openapi";

export enum ApiErrorType {
  Unauthorized = 401,
  NotFound = 404,
  AlreadyExists = 409,
  BadRequest = 400,
  InternalServerError = 500,
}

export class ApiError extends Error {
  constructor(private error: components["schemas"]["ErrorMessage"]) {
    super(error.message);
  }

  get type(): ApiErrorType {
    switch (this.error.status_code) {
      case ApiErrorType.Unauthorized:
        return ApiErrorType.Unauthorized;
      case ApiErrorType.NotFound:
        return ApiErrorType.NotFound;
      case ApiErrorType.AlreadyExists:
        return ApiErrorType.AlreadyExists;
      case ApiErrorType.BadRequest:
        return ApiErrorType.BadRequest;
      case ApiErrorType.InternalServerError:
      default:
        return ApiErrorType.InternalServerError;
    }
  }
}
