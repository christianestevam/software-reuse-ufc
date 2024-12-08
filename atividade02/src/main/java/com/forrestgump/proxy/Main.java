package com.forrestgump.proxy;

import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;

import java.util.Date;

public class Main {
    public static void main(String[] args) {
        String secret = "senhaSuperSecreta";

        String token = JWT.create()
                .withIssuer("banco-digital")
                .withClaim("usuario", "Joao")
                .withClaim("role", "user")
                .withIssuedAt(new Date())
                .sign(Algorithm.HMAC256(secret));

        System.out.println("Token Gerado: " + token);

        ServicoConfidencial servico = new ServicoConfidencialProxy(secret);

        System.out.println("\nTentativa de acesso com token válido:");
        servico.acessarDocumentos(token);

        System.out.println("\nTentativa de acesso com token inválido:");
        try {
            servico.acessarDocumentos("tokenInvalido");
        } catch (SecurityException e) {
            System.out.println(e.getMessage());
        }
    }
}
