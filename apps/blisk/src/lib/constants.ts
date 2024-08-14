export const COLOR_SCHEMES = ["dark", "light"] as const;

export const A_MONTH_IN_SECONDS = 60 * 60 * 24 * 30;

export const BREAKPOINTS = {
  md: 768,
  lg: 1280,
};

export const OPTIMISTIC_ID = -9999;

export const VALID_REACTIONS = ["like", "love", "laugh", "wow", "sad", "angry"] as const;

export const VALID_REACTION_FOR = ["post", "comment"] as const;

export const AUTHORIZATION_REQUEST_PARAMS = [
  "scope",
  "response_type",
  "client_id",
  "redirect_uri",
  "state",
  "response_mode",
  "nonce",
  "display",
  "prompt",
  "max_age",
  "ui_locales",
  "id_token_hint",
  "login_hint",
  "acr_values",
] as const;
