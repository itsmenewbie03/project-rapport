import * as EmailValidator from 'email-validator';

const validate_email = (email: string): boolean => {
  return EmailValidator.validate(email);
};

export { validate_email };
