package com.forrestgump.proxy;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.exceptions.JWTVerificationException;
import com.auth0.jwt.interfaces.DecodedJWT;
import com.auth0.jwt.interfaces.JWTVerifier;

import java.util.Date;

public class TokenUtils {
    private final JWTVerifier verifier;

    public TokenUtils(String secret) {
        this.verifier = JWT.require(Algorithm.HMAC256(secret))
                .withIssuer(ConfigLoader.getJwtIssuer())
                .build();
    }

    public DecodedJWT validateToken(String token) throws JWTVerificationException {
        return verifier.verify(token);
    }

    public static String generateToken(String user, String role, String secret) {
        return JWT.create()
                .withIssuer(ConfigLoader.getJwtIssuer())
                .withClaim("user", user)
                .withClaim("role", role)
                .withIssuedAt(new Date())
                .sign(Algorithm.HMAC256(secret));
    }
}
