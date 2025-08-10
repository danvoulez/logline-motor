// === File: parser/src/lib.rs ===
/*
    Description: Parser para a linguagem LogLine com funções públicas para parsing.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

pub mod ast;
mod parser;

use ast::Command;

/// Erros que podem ocorrer durante o parsing.
#[derive(Debug)]
pub enum Error {
    /// Erro de sintaxe com mensagem explicativa.
    Syntax(String),
}

/// Faz o parsing de uma string LogLine em uma estrutura de comando.
///
/// # Argumentos
///
/// * `input` - Uma string contendo o comando LogLine a ser analisado.
///
/// # Retorno
///
/// Um `Result` que contém o comando parseado ou um erro de parsing.
pub fn parse(input: &str) -> Result<Command, Error> {
    parser::parse_command(input).map_err(|e| {
        Error::Syntax(format!("Erro de sintaxe: {}", e))
    })
}