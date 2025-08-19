use serde::Deserialize;
use eframe::{egui, run_native, NativeOptions};
use tinyfiledialogs as tfd;
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use chrono::{NaiveDate, Datelike};
use walkdir::WalkDir;

// Define as estruturas de dados para desserializar o XML da nota fiscal.
// Cada struct corresponde a um elemento no XML.

/// Representa a resposta da consulta de NFSe.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "ConsultarNfseResposta")]
struct ConsultarNfseResposta {
    #[serde(rename = "ListaNfse")]
    lista_nfse: ListaNfse,
}

/// Contém a lista de notas fiscais.
#[derive(Debug, Deserialize, Clone)]
struct ListaNfse {
    #[serde(rename = "CompNfse", default)]
    comp_nfse: Vec<CompNfse>,
}

/// Representa um componente da nota fiscal.
#[derive(Debug, Deserialize, Clone)]
struct CompNfse {
    #[serde(rename = "Nfse")]
    nfse: Nfse,
}

/// Contém as informações da nota fiscal.
#[derive(Debug, Deserialize, Clone)]
struct Nfse {
    #[serde(rename = "InfNfse")]
    inf_nfse: InfNfse,
}

/// Detalhes da nota fiscal.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct InfNfse {
    numero: u32,
    data_emissao: String,
    servico: Servico,
    prestador_servico: Prestador,
    tomador_servico: Tomador,
}

/// Informações sobre o serviço prestado.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Servico {
    valores: Valores,
    discriminacao: String,
}

/// Valores relacionados ao serviço.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Valores {
    valor_servicos: f32,
}

/// Dados do prestador de serviço.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Prestador {
    razao_social: String,
    identificacao_prestador: IdentificacaoPrestador,
}

/// Identificação do prestador (CNPJ).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct IdentificacaoPrestador {
    cnpj: String,
}

/// Dados do tomador de serviço.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct Tomador {
    razao_social: String,
    identificacao_tomador: IdentificacaoTomador,
}

/// Identificação do tomador (CPF ou CNPJ).
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct IdentificacaoTomador {
    #[serde(rename = "CpfCnpj")]
    cpf_cnpj: CpfCnpj,
}

/// Estrutura para armazenar CPF ou CNPJ.
#[derive(Debug, Deserialize, Clone)]
struct CpfCnpj {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
    #[serde(rename = "Cpf")]
    cpf: Option<String>,
}

/// Analisa um único arquivo XML e retorna os dados desserializados.
fn parse_xml_from_file(file_path: &PathBuf) -> Result<ConsultarNfseResposta, String> {
    // Abre o arquivo XML.
    let mut file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Erro ao abrir o arquivo \"{:?}\": {}", file_path, e)),
    };

    // Lê o conteúdo do arquivo para uma string.
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(format!("Erro ao ler o arquivo: {}", e));
    }

    // Remove o BOM (Byte Order Mark) do início do arquivo, se existir.
    let contents = contents.trim_start_matches('\u{feff}');

    // Desserializa o conteúdo XML para a estrutura de dados.
    let resposta: Result<ConsultarNfseResposta, _> = quick_xml::de::from_str(contents);

    // Retorna o resultado da desserialização.
    match resposta {
        Ok(r) => Ok(r),
        Err(e) => Err(format!("Erro ao processar o XML em \"{:?}\": {}", file_path, e)),
    }
}

/// Estrutura principal da aplicação de GUI.
struct TemplateApp {
    selected_files: Vec<PathBuf>,
    parsed_invoices: Vec<InfNfse>,
    error_message: Option<String>,
}

impl Default for TemplateApp {
    /// Cria uma nova instância da aplicação.
    fn default() -> Self {
        Self {
            selected_files: Vec::new(),
            parsed_invoices: Vec::new(),
            error_message: None,
        }
    }
}

impl eframe::App for TemplateApp {
    /// Atualiza a interface gráfica a cada frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Processador de Notas Fiscais");

            // Botões para selecionar arquivos ou pastas.
            ui.horizontal(|ui| {
                // Botão para selecionar múltiplos arquivos XML.
                if ui.button("Selecionar Arquivos XML").clicked() {
                    let files = tfd::open_file_dialog_multi("Selecione os arquivos XML", "", Some((&["*.xml"], "Arquivos XML")));
                    if let Some(files) = files {
                        self.selected_files = files.into_iter().map(PathBuf::from).collect();
                        self.process_files();
                    }
                }
                // Botão para selecionar uma pasta.
                if ui.button("Selecionar Pasta").clicked() {
                    let folder = tfd::select_folder_dialog("Selecione uma pasta", "");
                    if let Some(folder) = folder {
                        // Percorre a pasta e subpastas em busca de arquivos XML.
                        self.selected_files = WalkDir::new(folder)
                            .into_iter()
                            .filter_map(|e| e.ok())
                            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("xml"))
                            .map(|e| e.path().to_path_buf())
                            .collect();
                        self.process_files();
                    }
                }
            });

            // Exibe os arquivos selecionados.
            ui.group(|ui| {
                ui.label("Arquivos Selecionados:");
                if self.selected_files.is_empty() {
                    ui.label("Nenhum arquivo selecionado.");
                } else {
                    for path in &self.selected_files {
                        ui.label(format!("{}", path.display()));
                    }
                }
            });

            // Exibe mensagens de erro, se houver.
            if let Some(msg) = &self.error_message {
                ui.colored_label(egui::Color32::RED, msg);
            }

            // Exibe o número de notas fiscais processadas.
            ui.label(format!("Notas Fiscais Processadas: {}", self.parsed_invoices.len()));

            // Exibe os detalhes de cada nota fiscal em uma área de rolagem.
            egui::ScrollArea::vertical().show(ui, |ui| {
                for invoice in &self.parsed_invoices {
                    ui.group(|ui| {
                        ui.label(format!("Número: {}", invoice.numero));
                        ui.label(format!("Data de Emissão: {}", invoice.data_emissao));
                        ui.label(format!("Prestador: {}", invoice.prestador_servico.razao_social));
                        ui.label(format!("CNPJ Prestador: {}", invoice.prestador_servico.identificacao_prestador.cnpj));
                        ui.label(format!("Tomador: {}", invoice.tomador_servico.razao_social));
                        if let Some(cnpj) = &invoice.tomador_servico.identificacao_tomador.cpf_cnpj.cnpj {
                            ui.label(format!("CNPJ Tomador: {}", cnpj));
                        }
                        if let Some(cpf) = &invoice.tomador_servico.identificacao_tomador.cpf_cnpj.cpf {
                            ui.label(format!("CPF Tomador: {}", cpf));
                        }
                        ui.label(format!("Valor: {:.2}", invoice.servico.valores.valor_servicos));
                        ui.label(format!("Descrição: {}", invoice.servico.discriminacao));
                    });
                }
            });
        });
    }
}

impl TemplateApp {
    /// Processa a lista de arquivos XML selecionados.
    fn process_files(&mut self) {
        self.parsed_invoices.clear();
        self.error_message = None;

        for path in &self.selected_files {
            match parse_xml_from_file(path) {
                Ok(resposta) => {
                    for comp_nfse in resposta.lista_nfse.comp_nfse {
                        self.parsed_invoices.push(comp_nfse.nfse.inf_nfse);
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Erro ao processar {}: {}", path.display(), e));
                    break;
                }
            }
        }
    }
}

/// Função principal que inicia a aplicação.
fn main() {
    let options = NativeOptions::default();
    // Executa a aplicação nativa com as opções e a estrutura da aplicação.
    let _ = run_native(
        "Processador de Notas Fiscais",
        options,
        Box::new(|_cc| Ok(Box::new(TemplateApp::default()))),
    );
}
