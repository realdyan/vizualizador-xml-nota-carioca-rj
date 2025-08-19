Visualizador de XML de Nota Fiscal de Serviço (NFS-e)

Este projeto é uma aplicação de desktop desenvolvida em Rust para visualizar de forma clara e organizada as informações contidas em arquivos XML de Notas Fiscais de Serviço Eletrônicas (NFS-e).

A ferramenta permite que o usuário selecione múltiplos arquivos XML ou um diretório inteiro, e exibe os dados mais importantes de cada nota fiscal em uma interface gráfica simples e intuitiva.

Funcionalidades

    Interface Gráfica Simples: Construído com egui, o aplicativo é fácil de usar.

    Seleção Múltipla de Arquivos: Abra vários arquivos XML de uma só vez através de uma janela de diálogo nativa.

    Seleção de Diretório: Escolha uma pasta e o programa irá buscar e processar recursivamente todos os arquivos .xml que encontrar dentro dela.

    Visualização Detalhada: Exibe as informações essenciais de cada nota fiscal, incluindo:

        Número da Nota

        Data de Emissão

        Prestador do Serviço (Razão Social e CNPJ)

        Tomador do Serviço (Razão Social e CPF/CNPJ)

        Valor Total dos Serviços

        Discriminação do Serviço

    Tratamento de Erros: Exibe mensagens de erro caso um arquivo não possa ser lido ou processado.

<img width="1024" height="600" alt="image" src="https://github.com/user-attachments/assets/e76f58a6-e48e-4570-8e6a-8f46ce60684d" />


    Linguagem: Rust

    Interface Gráfica: eframe / egui

    Processamento de XML: quick-xml

    Desserialização de Dados: Serde

    Janelas de Diálogo Nativas: tinyfiledialogs

    Navegação em Diretórios: walkdir

Como Compilar e Executar

Para compilar e executar este projeto localmente, você precisará ter o ambiente de desenvolvimento Rust instalado.

1. Pré-requisitos:

    Instale o Rust e o Cargo seguindo as instruções em rustup.rs.

2. Clone o Repositório:
Bash

git clone https://github.com/realdyan/vizualizador-xml-nota-carioca-rj.git
cd vizualizador-xml-nota-carioca-rj

3. Compile e Execute o Projeto:

Você pode executar o projeto em modo de desenvolvimento:
Bash

cargo run

Para uma melhor performance, compile em modo de release:
Bash

# Compila o projeto
cargo build --release

# O executável estará em /target/release/
./target/release/vizualizador-xml-nota-carioca-rj

Como Usar

    Inicie a aplicação.

    Clique em "Selecionar Arquivos XML" para escolher um ou mais arquivos de nota fiscal no seu computador.

    Ou clique em "Selecionar Pasta" para que o programa encontre e leia todos os arquivos XML dentro do diretório selecionado e de seus subdiretórios.

    As informações das notas fiscais processadas aparecerão na janela principal, em uma área rolável.

Contribuição

Contribuições são muito bem-vindas! Se você tiver sugestões, melhorias ou correções de bugs, por favor, abra uma issue ou envie um pull request.
