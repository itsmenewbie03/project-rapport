import crypto from 'crypto';

// WARN: most of this is written by gemini
// I will refactor this to my coding style later(or never xD)

const hash = async (password: string) => {
  const salt = await generateSalt(); // Call to generate a secure random salt
  const hashBuffer = await bcryptHash(password, salt);
  return hashBuffer;
};

async function generateSalt(): Promise<string> {
  return new Promise((resolve, reject) => {
    crypto.randomBytes(128, (err, buffer) => {
      if (err) {
        reject(err);
      } else {
        resolve(buffer.toString('base64')); // Convert to base64 for storage
      }
    });
  });
}

/**
 * @returns hash in base64 string format
 * */
async function bcryptHash(password: string, salt: string): Promise<string> {
  const cost = 10; // Adjust the cost factor as needed (higher = slower, more secure)
  return new Promise((resolve, reject) => {
    crypto.pbkdf2(
      password,
      salt,
      cost * 2 ** 16,
      512,
      'sha512',
      (err, derivedKey) => {
        if (err) {
          reject(err);
        } else {
          resolve(
            Buffer.concat([Buffer.from(salt), derivedKey]).toString('base64'),
          ); // Combine salt and hash
        }
      },
    );
  });
}

export { hash };
