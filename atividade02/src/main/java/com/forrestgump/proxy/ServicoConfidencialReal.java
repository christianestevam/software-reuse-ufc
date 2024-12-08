package com.forrestgump.proxy;

public class ServicoConfidencialReal implements ServicoConfidencial {
    @Override
    public void acessarDocumentos(String token) {
        System.out.println("Acessando documentos confidenciais...");
        System.out.println("Documento: Declaração de Imposto de Renda 2023");
    }
}
