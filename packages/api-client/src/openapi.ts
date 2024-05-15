// -- file generated by openapi-typescript don't edit --

/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


export interface paths {
  "/auth/login": {
    post: operations["login"];
  };
  "/auth/logout": {
    delete: operations["logout"];
  };
  "/auth/register": {
    post: operations["register"];
  };
  "/auth/verify": {
    get: operations["verify"];
  };
  "/galaxies": {
    get: operations["get_all_galaxies"];
    post: operations["create_galaxy"];
  };
  "/galaxies/{galaxy_id}": {
    get: operations["get_galaxy"];
    put: operations["update_galaxy"];
    delete: operations["delete_galaxy"];
  };
  "/galaxies/{galaxy_id}/planets": {
    get: operations["get_all_planets"];
    post: operations["create_planet"];
  };
  "/galaxies/{galaxy_id}/planets/{planet_id}": {
    get: operations["get_planet"];
    put: operations["update_planet"];
    delete: operations["delete_planet"];
  };
  "/galaxies/{galaxy_id}/stars": {
    get: operations["get_all_stars"];
    post: operations["create_star"];
  };
  "/galaxies/{galaxy_id}/stars/{star_id}": {
    get: operations["get_star"];
    put: operations["update_star"];
    delete: operations["delete_star"];
  };
  "/users/me": {
    get: operations["me"];
  };
}

export type webhooks = Record<string, never>;

export interface components {
  schemas: {
    AuthData: components["schemas"]["Credentials"] & {
      remember: boolean;
    };
    ConnectPlanetToStar: {
      /** Format: uuid */
      id?: string | null;
    };
    CreateGalaxyData: {
      name: string;
    };
    CreatePlanetData: {
      /** Format: int32 */
      capacity: number;
      name: string;
      path: string;
      star: components["schemas"]["ConnectPlanetToStar"];
    };
    CreateStarData: {
      /** Format: uri */
      domain: string;
      name: string;
      /** Format: uri */
      nebula: string;
    };
    Credentials: components["schemas"]["Password"] & {
      username: string;
    };
    ErrorMessage: {
      message: string;
      /** Format: int32 */
      status_code: number;
    };
    Galaxy: {
      /** Format: uuid */
      id: string;
      name: string;
      /** Format: uuid */
      user_id: string;
    };
    Password: {
      password: string;
    };
    Planet: {
      /** Format: int32 */
      capacity: number;
      /** Format: uuid */
      galaxy_id: string;
      /** Format: uuid */
      id: string;
      name: string;
      path: string;
      /** Format: uuid */
      star_id?: string | null;
    };
    Star: {
      domain: string;
      /** Format: uuid */
      galaxy_id: string;
      /** Format: uuid */
      id: string;
      name: string;
      /** Format: uri */
      nebula: string;
    };
    UpdateGalaxyData: {
      name?: string | null;
    };
    UpdatePlanetData: {
      /** Format: int32 */
      capacity?: number | null;
      name?: string | null;
      path?: string | null;
      star?: components["schemas"]["ConnectPlanetToStar"] | null;
    };
    UpdateStarData: {
      /** Format: uri */
      domain?: string | null;
      name?: string | null;
      /** Format: uri */
      nebula?: string | null;
    };
    User: {
      /** Format: uuid */
      id: string;
      name: string;
    };
  };
  responses: {
    /** @description The resource already exists */
    AlreadyExistsResponse: {
      content: {
        "application/json": components["schemas"]["ErrorMessage"];
      };
    };
    /** @description user authorized from session token */
    AuthResponse: {
      headers: {
        /** @description The session ID is returned in a cookie named `session`. You need to include this cookie in subsequent requests. */
        "Set-Cookie"?: string;
      };
      content: {
        "application/json": {
          user: components["schemas"]["User"];
        };
      };
    };
    /** @description all user galaxies */
    GalaxiesList: {
      content: {
        "application/json": components["schemas"]["Galaxy"][];
      };
    };
    /** @description galaxy successfully created */
    GalaxyCreated: {
      content: {
        "application/json": components["schemas"]["Galaxy"];
      };
    };
    /** @description galaxy successfully deleted */
    GalaxyDeleted: {
      content: {
        "application/json": components["schemas"]["Galaxy"];
      };
    };
    /** @description galaxy successfully updated */
    GalaxyUpdated: {
      content: {
        "application/json": components["schemas"]["Galaxy"];
      };
    };
    /** @description An internal error occurred */
    InternalErrorResponse: {
      content: {
        "application/json": components["schemas"]["ErrorMessage"];
      };
    };
    /** @description Requested resources not found */
    NotFoundResponse: {
      content: {
        "application/json": components["schemas"]["ErrorMessage"];
      };
    };
    /** @description planet successfully created */
    PlanetCreated: {
      content: {
        "application/json": components["schemas"]["Planet"];
      };
    };
    /** @description planet successfully deleted */
    PlanetDeleted: {
      content: {
        "application/json": components["schemas"]["Planet"];
      };
    };
    /** @description planet successfully updated */
    PlanetUpdated: {
      content: {
        "application/json": components["schemas"]["Planet"];
      };
    };
    /** @description all planets in the galaxy */
    PlanetsList: {
      content: {
        "application/json": components["schemas"]["Planet"][];
      };
    };
    /** @description specific galaxy with all its stars and planets */
    SpecificGalaxy: {
      content: {
        "application/json": {
          galaxy: components["schemas"]["Galaxy"];
          planets: components["schemas"]["Planet"][];
          stars: components["schemas"]["Star"][];
        };
      };
    };
    /** @description specific planet */
    SpecificPlanet: {
      content: {
        "application/json": components["schemas"]["Planet"];
      };
    };
    /** @description specific star */
    SpecificStar: {
      content: {
        "application/json": components["schemas"]["Star"];
      };
    };
    /** @description star successfully created */
    StarCreated: {
      content: {
        "application/json": components["schemas"]["Star"];
      };
    };
    /** @description star successfully deleted */
    StarDeleted: {
      content: {
        "application/json": components["schemas"]["Star"];
      };
    };
    /** @description star successfully updated */
    StarUpdated: {
      content: {
        "application/json": components["schemas"]["Star"];
      };
    };
    /** @description all stars in the galaxy */
    StarsList: {
      content: {
        "application/json": components["schemas"]["Star"][];
      };
    };
    /** @description User not authorized */
    UnauthorizeResponse: {
      content: {
        "application/json": components["schemas"]["ErrorMessage"];
      };
    };
    UserResponse: {
      content: {
        "application/json": components["schemas"]["User"];
      };
    };
    /** @description The body of the request contains incorrect data */
    ValidationResponse: {
      content: {
        "application/json": components["schemas"]["ErrorMessage"];
      };
    };
  };
  parameters: never;
  requestBodies: never;
  headers: never;
  pathItems: never;
}

export type $defs = Record<string, never>;

export type external = Record<string, never>;

export interface operations {

  login: {
    /** @description login data */
    requestBody: {
      content: {
        "application/json": components["schemas"]["AuthData"];
      };
    };
    responses: {
      200: components["responses"]["AuthResponse"];
      400: components["responses"]["ValidationResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  logout: {
    responses: {
      204: {
        content: never;
      };
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  register: {
    /** @description registration data */
    requestBody: {
      content: {
        "application/json": components["schemas"]["AuthData"];
      };
    };
    responses: {
      200: components["responses"]["AuthResponse"];
      400: components["responses"]["ValidationResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  verify: {
    responses: {
      204: {
        content: never;
      };
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_all_galaxies: {
    responses: {
      200: components["responses"]["GalaxiesList"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  create_galaxy: {
    /** @description data for creating the galaxy */
    requestBody: {
      content: {
        "application/json": components["schemas"]["CreateGalaxyData"];
      };
    };
    responses: {
      200: components["responses"]["GalaxyCreated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_galaxy: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    responses: {
      200: components["responses"]["SpecificGalaxy"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  update_galaxy: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    /** @description data for updating the galaxy */
    requestBody: {
      content: {
        "application/json": components["schemas"]["UpdateGalaxyData"];
      };
    };
    responses: {
      200: components["responses"]["GalaxyUpdated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  delete_galaxy: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    responses: {
      200: components["responses"]["GalaxyDeleted"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_all_planets: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    responses: {
      200: components["responses"]["PlanetsList"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  create_planet: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    /** @description data for creating the planet */
    requestBody: {
      content: {
        "application/json": components["schemas"]["CreatePlanetData"];
      };
    };
    responses: {
      200: components["responses"]["PlanetCreated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_planet: {
    parameters: {
      path: {
        galaxy_id: string;
        planet_id: string;
      };
    };
    responses: {
      200: components["responses"]["SpecificPlanet"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  update_planet: {
    parameters: {
      path: {
        galaxy_id: string;
        planet_id: string;
      };
    };
    /** @description data for updating the planet */
    requestBody: {
      content: {
        "application/json": components["schemas"]["UpdatePlanetData"];
      };
    };
    responses: {
      200: components["responses"]["PlanetUpdated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  delete_planet: {
    parameters: {
      path: {
        galaxy_id: string;
        planet_id: string;
      };
    };
    responses: {
      200: components["responses"]["PlanetDeleted"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_all_stars: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    responses: {
      200: components["responses"]["StarsList"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  create_star: {
    parameters: {
      path: {
        galaxy_id: string;
      };
    };
    /** @description data for creating the star */
    requestBody: {
      content: {
        "application/json": components["schemas"]["CreateStarData"];
      };
    };
    responses: {
      200: components["responses"]["StarCreated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  get_star: {
    parameters: {
      path: {
        galaxy_id: string;
        star_id: string;
      };
    };
    responses: {
      200: components["responses"]["SpecificStar"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  update_star: {
    parameters: {
      path: {
        galaxy_id: string;
        star_id: string;
      };
    };
    /** @description data for updating the star */
    requestBody: {
      content: {
        "application/json": components["schemas"]["UpdateStarData"];
      };
    };
    responses: {
      200: components["responses"]["StarUpdated"];
      400: components["responses"]["ValidationResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      409: components["responses"]["AlreadyExistsResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  delete_star: {
    parameters: {
      path: {
        galaxy_id: string;
        star_id: string;
      };
    };
    responses: {
      200: components["responses"]["StarDeleted"];
      401: components["responses"]["UnauthorizeResponse"];
      404: components["responses"]["NotFoundResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
  me: {
    responses: {
      200: components["responses"]["UserResponse"];
      401: components["responses"]["UnauthorizeResponse"];
      500: components["responses"]["InternalErrorResponse"];
    };
  };
}
