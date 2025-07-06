export type UserPayload = {
  name: string;
  email: string;
  avatar_url: string;
};

export type SessionClaim = {
  user: UserPayload;
  iat: bigint;
  uid: string;
  ip: string;
};
