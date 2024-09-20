import Fetch from "../fetch";

type ValidateSession = {
  is_valid: boolean;
};

const FetchInstance = Fetch.getInstance();

export const validateSessionToken = async (session_token: string) => {
  const validateResult = await FetchInstance.fetch<ValidateSession>(
    `/api/validate_session?session_token=${session_token}`
  );
  return validateResult;
};
