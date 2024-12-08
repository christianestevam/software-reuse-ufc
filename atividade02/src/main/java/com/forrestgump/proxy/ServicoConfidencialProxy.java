package com.forrestgump.proxy;

import com.auth0.jwt.interfaces.DecodedJWT;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;

public class ServicoConfidencialProxy implements ServicoConfidencial {
    private static final Logger logger = LoggerFactory.getLogger(ServicoConfidencialProxy.class);
    private final ServicoConfidencialReal servicoReal;
    private final TokenUtils tokenUtils;


    public ServicoConfidencialProxy(String secret) {
        this.servicoReal = new ServicoConfidencialReal();
        this.tokenUtils = new TokenUtils(secret);
    }

    @Override
    public void acessarDocumentos(String token) {
        try {
            DecodedJWT jwt = tokenUtils.validateToken(token);
            String usuario = jwt.getClaim("usuario").asString();
            String role = jwt.getClaim("role").asString();

            if (!hasPermission(role, "DOCUMENTS")) {
                throw new SecurityException("Acesso negado. Permissão insuficiente.");
            }

            logger.info("Usuário {} autenticado com a role {}", usuario, role);
            servicoReal.acessarDocumentos(token);
        } catch (Exception ex) {
            logger.error("Falha no acesso: {}", ex.getMessage());
            throw new SecurityException("Acesso negado.");
        }
    }


    private boolean hasPermission(String role, String resource) {
        List<Map<String, Object>> permissions = ConfigLoader.getPermissions();
        return permissions.stream()
                .filter(p -> p.get("role").equals(role))
                .anyMatch(p -> {
                    List<String> accessList = (List<String>) p.get("access");
                    return accessList.contains(resource) || accessList.contains("ALL");
                });
    }

}
