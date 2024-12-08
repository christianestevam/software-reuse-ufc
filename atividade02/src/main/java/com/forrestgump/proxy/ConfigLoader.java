package com.forrestgump.proxy;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class ConfigLoader {
    private static Map<String, Object> config;

    static {
        try {
            ObjectMapper mapper = new ObjectMapper(new YAMLFactory());
            config = mapper.readValue(ConfigLoader.class.getResourceAsStream("/application.yml"), Map.class);
        } catch (IOException e) {
            throw new RuntimeException("Erro ao carregar configuração", e);
        }
    }

    public static String getJwtSecret() {
        return ((Map<String, String>) config.get("jwt")).get("secret");
    }

    public static String getJwtIssuer() {
        return ((Map<String, String>) config.get("jwt")).get("issuer");
    }

    public static List<Map<String, Object>> getPermissions() {
        return (List<Map<String, Object>>) config.get("permissions");
    }
}
