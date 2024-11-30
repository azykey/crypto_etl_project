**Crypto ETL Project** 

de forma lógica e expressiva, imagine um painel dividido em três áreas distintas, mas conectadas, como partes de uma máquina precisa, onde cada engrenagem desempenha um papel único.

---

**Seção 1: O Horizonte da Extração**  
- Esta parte do painel é representada como uma rede de canais, capturando informações fluindo diretamente da fonte — a CoinGecko API.  
- Cada canal carrega dados como um rio levando matéria-prima: preços, capitalização de mercado, volume.  
- A fonte é dinâmica, sempre em movimento, como o mercado de criptomoedas que nunca dorme.  
- No centro, um mecanismo — o cliente HTTP — coleta e traduz esses fluxos em pacotes compreensíveis.

---

**Seção 2: O Núcleo da Transformação**  
- Aqui, os dados brutos passam por um conjunto de filtros representados como prismas que organizam a luz.  
- Esses prismas refinam, limpam e ajustam os dados, transformando números dispersos em informações coerentes.  
- Imagine engrenagens com diferentes texturas, simbolizando critérios como "valores positivos" ou "campos obrigatórios".  
- No final, o prisma emite feixes ordenados — os dados agora têm propósito e clareza, prontos para o próximo estágio.

---

**Seção 3: A Torre de Carregamento**  
- A última seção é alta e imponente, representando o banco de dados PostgreSQL como uma torre sólida.  
- É um armazém digital onde cada peça processada encontra seu lugar.  
- Linhas de transporte — as queries SQL — entram e saem, organizando as informações em estantes bem estruturadas.  
- Há uma porta giratória no topo: os dados podem ser buscados e utilizados novamente, alimentando análises ou insights futuros.

---

**Ligação Entre as Áreas**  
- Um trilho conecta as três partes, simbolizando a comunicação interna do projeto, onde cada etapa depende da anterior.  
- As transições são fluidas, mas robustas, como os processos assíncronos em Rust e as conexões do pool do SQLx.  
- Ao lado do trilho, pequenos monitores indicam os logs e mensagens, sempre prontos para reportar status ou alertar falhas.

---

**Elemento Central: O Propósito**  
- No coração do painel, em letras firmes, está a palavra-chave: **CONFIANÇA**.  
- Este projeto não é apenas um pipeline técnico; é uma ponte que transforma dados em decisões, incertezas em clareza.  
- Ele serve para quem busca não apenas entender o mercado, mas dominá-lo, armado com ferramentas que revelam o que está oculto nos números.

O painel é, acima de tudo, uma manifestação de equilíbrio entre a lógica implacável das máquinas e a necessidade humana de organização e controle em um mundo digital caótico. Ele simboliza a fusão de precisão técnica e propósito visionário.

# Crypto ETL Project

## Descrição
Pipeline de Extração, Transformação e Carregamento (ETL) para dados de criptomoedas usando Rust.

## Tecnologias
- Rust
- PostgreSQL
- Docker
- SQLx
- Tokio

## Requisitos
- Rust
- Docker
- Docker Compose

## Configuração
1. Clone o repositório
2. Configure `.env` com suas credenciais
3. Execute: `docker-compose up --build`

## Funcionalidades
- Extração de dados de criptomoedas via CoinGecko API
- Transformação e limpeza de dados
- Carregamento em banco PostgreSQL
- Suporte a múltiplas criptomoedas
- Logging robusto
- Tratamento de erros com `anyhow`

## Licença
**Licença de Uso Personalizada**  

Copyright (c) 2024 adilson oliveira ky

Por este meio, **não é concedida permissão automática** para o uso, modificação, distribuição ou comercialização deste software e dos arquivos de documentação associados (a "Obra").  

Qualquer pessoa que deseje lidar com a Obra, incluindo, mas não se limitando a, usar, copiar, modificar, mesclar, publicar, distribuir, sublicenciar e/ou vender cópias da Obra, **deverá obter permissão expressa, por escrito**, do proprietário dos direitos autorais.  

**Condições para uso autorizado:**  
1. O aviso de copyright acima deverá ser incluído em todas as cópias ou partes substanciais da Obra.  
2. O uso da Obra deverá seguir estritamente os termos acordados com o proprietário.  

**Isenção de Garantia:**  
O SOFTWARE É FORNECIDO "NO ESTADO EM QUE SE ENCONTRA", SEM GARANTIA DE QUALQUER TIPO, EXPRESSA OU IMPLÍCITA, INCLUINDO, MAS NÃO SE LIMITANDO, ÀS GARANTIAS DE COMERCIALIZAÇÃO, ADEQUAÇÃO A UM FIM ESPECÍFICO E NÃO VIOLAÇÃO. EM NENHUM CASO OS AUTORES OU DETENTORES DOS DIREITOS AUTORAIS SERÃO RESPONSÁVEIS POR QUALQUER REIVINDICAÇÃO, DANO OU OUTRA RESPONSABILIDADE, SEJA EM UMA AÇÃO DE CONTRATO, DELITO OU DE OUTRA FORMA, DECORRENTE DE OU EM CONEXÃO COM O SOFTWARE OU O USO OU OUTRAS NEGOCIAÇÕES NO SOFTWARE.  

**Contato para Permissão:**  
Para solicitar autorização ou obter mais informações sobre o uso da Obra, entre em contato com:  
adilsonoliveira.2788@gmail.com  

---



---

# **Crypto ETL Project**

## **Índice**
1. [Visão Geral](#visão-geral)
2. [Pré-requisitos](#pré-requisitos)
3. [Configuração do Ambiente](#configuração-do-ambiente)
4. [Estrutura do Projeto](#estrutura-do-projeto)
5. [Funcionamento do Pipeline ETL](#funcionamento-do-pipeline-etl)
   - [1. Extração](#1-extração)
   - [2. Transformação](#2-transformação)
   - [3. Carregamento](#3-carregamento)
6. [Como Executar](#como-executar)
7. [Ambiente de Desenvolvimento](#ambiente-de-desenvolvimento)
8. [Deploy e Produção](#deploy-e-produção)
9. [Testes e Monitoramento](#testes-e-monitoramento)
10. [Licença](#licença)

---

## **Visão Geral**
O **Crypto ETL Project** é um pipeline de **Extração, Transformação e Carregamento (ETL)** desenvolvido em **Rust**, projetado para coletar dados de criptomoedas de uma API pública, transformá-los e carregá-los em um banco de dados PostgreSQL.

### **Principais Funcionalidades**
- **Extração de dados:** Obtém preços, market cap, volume, etc., de criptomoedas da API CoinGecko.
- **Transformação de dados:** Limpa, valida e formata os dados para carregamento.
- **Carregamento no banco:** Armazena os dados em uma tabela PostgreSQL com suporte a atualizações incrementais.
- **Portabilidade:** Infraestrutura baseada em Docker e Docker Compose.
- **Configuração dinâmica:** Personalizável através de variáveis de ambiente.

---

## **Pré-requisitos**
Antes de começar, certifique-se de ter instalado:
1. **Rust** (versão 1.67 ou superior) - [Instalar Rust](https://www.rust-lang.org/tools/install)
2. **Docker** e **Docker Compose** - [Instalar Docker](https://docs.docker.com/get-docker/)
3. **PostgreSQL** (opcional para desenvolvimento local) - [Instalar PostgreSQL](https://www.postgresql.org/download/)

---

## **Configuração do Ambiente**
1. Clone este repositório:
   ```bash
   git clone https://github.com/seu-usuario/crypto-etl-project.git
   cd crypto-etl-project
   ```

2. Crie um arquivo `.env`:
   ```bash
   DATABASE_URL=postgresql://user:password@localhost:5432/crypto_data
   API_BASE_URL=https://api.coingecko.com/api/v3
   COINS_TO_TRACK=bitcoin,ethereum,binancecoin
   ```

3. Configure o Docker Compose para subir o banco de dados:
   ```bash
   docker-compose up -d
   ```

---

## **Estrutura do Projeto**
```plaintext
crypto_etl_project/
├── src/
│   ├── main.rs                # Ponto de entrada principal
│   ├── config.rs              # Carregamento das configurações
│   ├── models.rs              # Definição das estruturas de dados
│   ├── extraction.rs          # Extração de dados da API
│   ├── transformation.rs      # Transformação de dados
│   └── loading.rs             # Carregamento de dados no banco
│
├── migrations/
│   └── 20240101_create_crypto_tables.sql  # Script SQL para tabelas
│
├── Cargo.toml                 # Gerenciador de dependências Rust
├── Dockerfile                 # Configuração da imagem Docker
├── docker-compose.yml         # Configuração Docker Compose
├── .env                       # Configurações de ambiente
└── README.md                  # Documentação do projeto
```

---

## **Funcionamento do Pipeline ETL**
### **1. Extração**
A extração é realizada pela função `extract_crypto_data` no módulo `extraction.rs`. Os dados são coletados da [CoinGecko API](https://www.coingecko.com/en/api).

**Fluxo:**
- Envia uma requisição HTTP para buscar informações sobre as criptomoedas configuradas.
- Recebe os dados e os converte em objetos `CryptoCoin`.

**Código-chave:**
```rust
let url = format!(
    "{}/coins/markets?vs_currency=usd&ids={}&order=market_cap_desc&sparkline=false",
    config.api_base_url,
    config.coins_to_track.join(",")
);

let coins: Vec<CryptoCoin> = client.get(&url)
    .send()
    .await?
    .json()
    .await?;
```

---

### **2. Transformação**
A transformação limpa e valida os dados extraídos, descartando valores inconsistentes e arredondando campos numéricos.

**Fluxo:**
- Remove moedas com `market_cap` ou `current_price` iguais a 0.
- Arredonda os valores de preço e market cap.

**Código-chave:**
```rust
pub fn transform_crypto_data(coins: Vec<CryptoCoin>) -> Result<Vec<CryptoCoin>> {
    let transformed_coins: Vec<CryptoCoin> = coins
        .into_iter()
        .filter(|coin| coin.market_cap > 0.0 && coin.current_price > 0.0)
        .map(|mut coin| {
            coin.current_price = (coin.current_price * 100.0).round() / 100.0;
            coin
        })
        .collect();
    Ok(transformed_coins)
}
```

---

### **3. Carregamento**
O carregamento insere os dados no banco PostgreSQL, garantindo que entradas duplicadas sejam atualizadas.

**Fluxo:**
- Conecta ao banco de dados via SQLx.
- Executa um `UPSERT` (inserção ou atualização) para evitar duplicatas.

**Código-chave:**
```rust
sqlx::query!(
    r#"
    INSERT INTO crypto_prices 
    (id, symbol, name, current_price, market_cap, total_volume, price_change_percentage_24h, extraction_timestamp)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT (id, extraction_timestamp) DO UPDATE 
    SET 
        current_price = EXCLUDED.current_price,
        market_cap = EXCLUDED.market_cap
    "#,
    coin.id, coin.symbol, coin.name, coin.current_price,
    coin.market_cap, coin.total_volume, coin.price_change_percentage_24h,
    coin.extraction_timestamp
)
.execute(pool)
.await?;
```

---

## **Como Executar**
1. Suba o ambiente Docker:
   ```bash
   docker-compose up --build
   ```

2. O serviço ETL será executado automaticamente. Para executar localmente:
   ```bash
   cargo run
   ```

---

## **Ambiente de Desenvolvimento**
- **Testes Locais:**
  Use o PostgreSQL local configurado no `.env` para testar a execução do pipeline:
  ```bash
  DATABASE_URL=postgresql://user:password@localhost:5432/crypto_data cargo run
  ```

- **Debug:**
  Habilite logs detalhados:
  ```bash
  RUST_LOG=debug cargo run
  ```

---

## **Deploy e Produção**
1. Compile o binário:
   ```bash
   cargo build --release
   ```

2. Use a imagem Docker gerada no `Dockerfile` para implantar em um ambiente de produção.

3. Escalabilidade:
   - Use Kubernetes para gerenciar múltiplas instâncias do ETL.
   - Configure filas de mensagens para processar grandes volumes de dados.

---

## **Testes e Monitoramento**
- **Testes:**
  Configure testes unitários com `cargo test` para os módulos:
  ```bash
  cargo test
  ```

- **Monitoramento:**
  Adicione integração com Prometheus para métricas, como:
  - Tempo de execução do ETL
  - Taxa de erros de extração/carregamento
  - Dados inseridos por execução

---



---

## **Licença**  

Este projeto está licenciado sob a [Licença MIT](https://mit-license.org/). Consulte o arquivo `LICENSE` incluído no repositório para mais detalhes.  

Para mais informações, visite [https://mit-license.org/](https://mit-license.org/).  

---

---

## **Licença de Uso Exclusiva**  

**Copyright (c) 2024 Adilson Oliveira Ky**  

Esta obra, incluindo o software e todos os arquivos de documentação associados (a "Obra"), é de propriedade exclusiva do autor. **É proibido, em qualquer circunstância, copiar, reproduzir, modificar, distribuir ou utilizar esta Obra sem a permissão expressa e por escrito do proprietário.**  

### **Restrições Gerais:**
1. **Proibição de Uso Não Autorizado:**  
   Qualquer uso, cópia, reprodução, ou redistribuição da Obra, integral ou parcial, sem autorização prévia por escrito, é estritamente proibido.  

2. **Exclusividade do Proprietário:**  
   Nenhum direito, licença ou permissão é concedido automaticamente por meio desta licença. O uso só será permitido mediante acordo explícito com o autor.  

3. **Proteção Contra Cópias:**  
   Ao copiar, distribuir ou reproduzir a Obra sem permissão, o infrator estará sujeito às penalidades legais aplicáveis, incluindo responsabilidade civil e criminal.  

---

### **Condições para Uso Autorizado:**
1. **Autorização Prévia:**  
   O uso da Obra deverá ser negociado diretamente com o proprietário, e as condições acordadas deverão ser formalizadas por escrito.  

2. **Inclusão de Créditos:**  
   Qualquer utilização autorizada deverá manter o aviso de copyright visível e íntegro em todas as cópias ou distribuições da Obra.  

---

### **Isenção de Garantias:**
A Obra é fornecida **"como está"** e **sem garantias de qualquer tipo**, expressas ou implícitas, incluindo, mas não se limitando a garantias de adequação, comercialização ou não violação.  
O proprietário não se responsabiliza por quaisquer danos diretos, indiretos ou consequenciais que possam surgir do uso, mesmo autorizado, da Obra.  

---

### **Violação e Penalidades:**
O uso não autorizado da Obra poderá acarretar:  
- Ações legais por violação de direitos autorais.  
- Indenizações financeiras e demais sanções previstas em lei.  

---

### **Contato para Permissão:**
Para obter autorização ou negociar o uso da Obra, entre em contato pelo e-mail:  
**adilsonoliveira.2788@gmail.com**

--- 



--- 

