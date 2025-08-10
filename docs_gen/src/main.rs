// === File: docs_gen/src/main.rs ===
/*
    Description: Gerador de documentação a partir dos comentários e código-fonte do projeto.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;

fn main() {
    println!("Gerando documentação para o LogLine Motor...");

    let workspace_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".."));
    let output_dir = workspace_dir.join("docs");
    
    // Criar diretório de saída se necessário
    fs::create_dir_all(&output_dir).expect("Não foi possível criar o diretório de docs");

    // Configurar o arquivo de saída
    let md_output = output_dir.join("reference.md");
    let mut file = File::create(md_output).expect("Falha ao criar arquivo de documentação");

    // Escrever cabeçalho
    writeln!(file, "# LogLine Motor: Documentação de Referência\n").unwrap();
    writeln!(file, "*Gerado automaticamente em {}*\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    
    // Escrever descrição do projeto
    writeln!(file, "## Visão Geral\n").unwrap();
    writeln!(file, "LogLine Motor é um sistema de orquestração e execução para a linguagem LogLine, com suporte a contratos, ideias e simulações.\n").unwrap();
    
    // Gerar índice de módulos
    generate_module_index(&workspace_dir, &mut file);
    
    // Processar cada crate
    process_crates(&workspace_dir, &mut file);
    
    println!("Documentação gerada em: {}", output_dir.join("reference.md").display());
}

fn generate_module_index(workspace_dir: &Path, file: &mut File) {
    writeln!(file, "## Módulos\n").unwrap();
    
    let cargo_toml_path = workspace_dir.join("Cargo.toml");
    let content = fs::read_to_string(cargo_toml_path).expect("Não foi possível ler Cargo.toml");
    
    // Extrair a lista de membros do workspace
    let re = Regex::new(r#"members\s*=\s*\[\s*([^\]]*)\s*\]"#).unwrap();
    if let Some(captures) = re.captures(&content) {
        if let Some(members_str) = captures.get(1) {
            let members_str = members_str.as_str();
            // Processar cada membro
            for member in members_str.split(',') {
                let member = member.trim().trim_matches('"').trim_matches('\'');
                if !member.is_empty() {
                    writeln!(file, "- [{}](#{})  ", member, member).unwrap();
                }
            }
        }
    }
    
    writeln!(file).unwrap();
}

fn process_crates(workspace_dir: &Path, file: &mut File) {
    // Processar cada diretório no workspace que tem um Cargo.toml
    for entry in WalkDir::new(workspace_dir)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            path.is_dir() || path.file_name().unwrap_or_default() == "Cargo.toml"
        })
    {
        let entry = entry.unwrap();
        if entry.file_name() == "Cargo.toml" && entry.depth() == 2 {
            let crate_dir = entry.path().parent().unwrap();
            let crate_name = crate_dir.file_name().unwrap().to_str().unwrap();
            
            writeln!(file, "## {}\n", crate_name).unwrap();
            
            // Extrair descrição do Cargo.toml
            if let Ok(content) = fs::read_to_string(entry.path()) {
                let re = Regex::new(r#"description\s*=\s*"([^"]*)""#).unwrap();
                if let Some(captures) = re.captures(&content) {
                    if let Some(desc) = captures.get(1) {
                        writeln!(file, "{}\n", desc.as_str()).unwrap();
                    }
                }
            }
            
            // Processar arquivos de código-fonte
            process_source_files(crate_dir, file);
            
            writeln!(file).unwrap();
        }
    }
}

fn process_source_files(crate_dir: &Path, file: &mut File) {
    let src_dir = crate_dir.join("src");
    if !src_dir.exists() || !src_dir.is_dir() {
        return;
    }
    
    writeln!(file, "### Arquivos\n").unwrap();
    
    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let rel_path = path.strip_prefix(&crate_dir).unwrap();
        
        writeln!(file, "#### {}\n", rel_path.display()).unwrap();
        
        // Extrair comentários do arquivo
        if let Ok(content) = fs::read_to_string(path) {
            extract_doc_comments(&content, file);
        }
        
        writeln!(file).unwrap();
    }
}

fn extract_doc_comments(content: &str, file: &mut File) {
    // Regex para encontrar comentários de documentação (/// ou //!)
    let re = Regex::new(r"(?m)^(?:///|//!)\s*(.*)$").unwrap();
    
    // Extrair e agrupar comentários de documentação
    let mut doc_block = String::new();
    let mut in_block = false;
    
    for line in content.lines() {
        if let Some(captures) = re.captures(line) {
            if let Some(comment) = captures.get(1) {
                if !in_block {
                    in_block = true;
                }
                doc_block.push_str(comment.as_str());
                doc_block.push('\n');
            }
        } else if in_block {
            // Fim de um bloco de documentação
            if !doc_block.trim().is_empty() {
                writeln!(file, "{}\n", doc_block.trim()).unwrap();
            }
            doc_block.clear();
            in_block = false;
        }
    }
    
    // Caso um bloco termine no final do arquivo
    if in_block && !doc_block.trim().is_empty() {
        writeln!(file, "{}\n", doc_block.trim()).unwrap();
    }
}