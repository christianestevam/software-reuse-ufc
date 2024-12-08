### **README: Proxy Seguro com Validação JWT**

---

## **Descrição do Projeto**

Este projeto implementa um sistema que utiliza o padrão de design **Proxy** para proteger o acesso a um serviço confidencial. Ele valida tokens JWT para autenticar e autorizar os usuários antes de permitir o acesso ao serviço real. As permissões são configuradas dinamicamente através de um arquivo de configuração YAML.

---

## **Funcionalidades**

- **Autenticação com JWT:** Validação de tokens JWT para garantir acesso seguro.
- **Autorização Baseada em Permissões:** Controle de acesso baseado em roles e permissões configuradas.
- **Padrão Proxy:** Intermediação entre o cliente e o serviço real, garantindo validação de segurança.
- **Configuração Dinâmica:** Configuração de segredo JWT, emissor e permissões via `application.yml`.
- **Logs Detalhados:** Utilização de SLF4J com Logback para rastreamento de ações.

---

## **Tecnologias Utilizadas**

- **Java 21**
- **Maven**
- **Auth0 Java JWT**: Para geração e validação de tokens JWT.
- **Jackson**: Para manipulação de JSON e YAML.
- **SLF4J + Logback**: Para logging estruturado.

---

## **Estrutura do Projeto**

```plaintext
atividade02/
├── pom.xml
├── src/
    ├── main/
       ├── java/
       │   └── com/
       │       └── forrestgump/
       │           └── proxy/
       │               ├── Main.java
       │               ├── ServicoConfidencial.java
       │               ├── ServicoConfidencialReal.java
       │               ├── ServicoConfidencialProxy.java
       │               ├── TokenUtils.java
       │               └── ConfigLoader.java
       └── resources/
           ├── application.yml
           └── logback.xml

```

---

## **Configuração do Projeto**

### **1. Pré-requisitos**
- Java 17 ou superior instalado.
- Maven instalado.

### **2. Configuração do `application.yml`**

O arquivo `src/main/resources/application.yml` contém as configurações para o segredo JWT, emissor e permissões. Exemplo:

```yaml
jwt:
  secret: "senhaSuperSecreta"
  issuer: "banco-digital"

permissions:
  - role: "admin"
    access: ["ALL"]
  - role: "user"
    access: ["DOCUMENTS"]
```

### **3. Configuração do Logging**

O arquivo `logback.xml` em `src/main/resources` configura o logging. Exemplo básico:

```xml
<configuration>
    <appender name="CONSOLE" class="ch.qos.logback.core.ConsoleAppender">
        <encoder>
            <pattern>%d{yyyy-MM-dd HH:mm:ss} [%thread] %-5level %logger{36} - %msg%n</pattern>
        </encoder>
    </appender>

    <root level="info">
        <appender-ref ref="CONSOLE" />
    </root>
</configuration>
```

---

## **Como Executar**

### **1. Clonar o Repositório**

```bash
git clone 
cd proxy-seguro
```

### **2. Compilar o Projeto**

```bash
mvn clean compile
```

### **3. Executar a Aplicação**

```bash
mvn exec:java -Dexec.mainClass="com.forrestgump.proxy.Main"
```

---

## **Exemplo de Uso**

### **Token Gerado**
Ao executar o código, será gerado um token JWT com as claims `usuario` e `role`. Exemplo:

```plaintext
Token Gerado: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### **Saída Esperada**
Para um token válido, a aplicação exibe:

```plaintext
Tentativa de acesso com token válido:
2024-12-07 23:00:00 [main] INFO  c.f.proxy.ServicoConfidencialProxy - Usuário Joao autenticado com a role user
Acessando documentos confidenciais...
Documento: Declaração de Imposto de Renda 2023

Tentativa de acesso com token inválido:
2024-12-07 23:00:01 [main] ERROR c.f.proxy.ServicoConfidencialProxy - Falha no acesso: The token was expected to have 3 parts but got 0.
Acesso negado.
```

---

## **Principais Classes**

1. **`Main.java`**
    - Classe principal que gera tokens e realiza chamadas ao proxy.

2. **`ServicoConfidencialProxy.java`**
    - Implementa o padrão Proxy, validando os tokens e as permissões antes de delegar ao serviço real.

3. **`TokenUtils.java`**
    - Gera e valida tokens JWT usando a biblioteca Auth0.

4. **`ConfigLoader.java`**
    - Carrega configurações do arquivo `application.yml`.

5. **`ServicoConfidencialReal.java`**
    - Representa o serviço real que fornece acesso a informações confidenciais.

---

## **Personalização**

### **Adicionando Novas Roles e Permissões**
Edite o arquivo `application.yml` para incluir novas roles e permissões:

```yaml
permissions:
  - role: "manager"
    access: ["REPORTS", "DOCUMENTS"]
```

---

## **Dependências**

As dependências são gerenciadas pelo Maven. No `pom.xml`, temos:

- **Auth0 Java JWT**: Manipulação de tokens JWT.
- **SLF4J + Logback**: Logging.
- **Jackson**: Manipulação de JSON e YAML.

---