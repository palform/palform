import * as OTPAuth from "otpauth";

export function generateNewTOTP(nickname: string) {
    const secret = new OTPAuth.Secret();
    const totp = new OTPAuth.TOTP({
        issuer: "Palform",
        label: nickname,
        secret,
    });

    return {
        uri: totp.toString(),
        secret: secret.base32,
    };
}

export function verifyTOTP(code: string, secret: string) {
    const totp = new OTPAuth.TOTP({
        secret: OTPAuth.Secret.fromBase32(secret),
    });
    const delta = totp.validate({ token: code });
    return delta !== null;
}
