// === File: parser/src/parser.rs ===
/*
    Description: Implementação do parser com combinators nom para a linguagem LogLine.
    LogLine Motor
    Author: @danvoulez
    License: Apache-2.0
    Version: v1.0.0
    Date: 2025-08-07
    Repository: https://git.danvoulez/loglinemotor
    Contact: dan@danvoulez.com
*/

use nom::{
    IResult,
    bytes::complete::{tag, take_till, take_while},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, map_res},
    multi::separated_list0,
    // Import the combinators we use. We retain `tuple` here to build
    // compound parsers; although the `Parser` trait is implemented
    // directly for tuples, the helper function is still required to
    // construct the parser from individual combinators.
    sequence::{delimited, tuple},
    branch::alt,
    Parser,
};

use crate::ast::{Command, Imperative, ImperativeKind};

/// Faz o parsing de um comando LogLine completo.
pub fn parse_command(input: &str) -> Result<Command, String> {
    match command(input) {
        Ok((rest, cmd)) => {
            if rest.trim().is_empty() {
                Ok(cmd)
            } else {
                Err(format!("Entrada não foi totalmente consumida. Restante: '{}'", rest))
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// Parser para um comando LogLine.
fn command(input: &str) -> IResult<&str, Command> {
    // Apply the Parser trait's parse method to run the parser
    map(imperative, Command::Imperative).parse(input)
}

/// Parser para um comando imperativo.
fn imperative(input: &str) -> IResult<&str, Imperative> {
    alt((
        define_contract,
        define_idea,
        simulate_entity,
        orchestrate,
        invoke_ruleset,
    ))
    .parse(input)
}

/// Parser para o comando DEFINE CONTRACT.
fn define_contract(input: &str) -> IResult<&str, Imperative> {
    // Parse the DEFINE CONTRACT prefix
    let (input, _) = tuple((tag("DEFINE"), multispace1, tag("CONTRACT"), multispace1)).parse(input)?;
    // Parse the contract identifier
    let (input, id) = identifier(input)?;
    let (input, _) = multispace1(input)?;
    // Parse the list of clauses separated by commas
    let (input, clauses) = separated_list0(
        tuple((multispace0, char(','), multispace0)),
        clause,
    )
    .parse(input)?;
    
    Ok((
        input,
        Imperative {
            kind: ImperativeKind::DefineContract {
                id: id.to_string(),
                clauses: clauses.into_iter().map(String::from).collect(),
            },
        },
    ))
}

/// Parser para o comando DEFINE IDEA.
fn define_idea(input: &str) -> IResult<&str, Imperative> {
    let (input, _) = tuple((tag("DEFINE"), multispace1, tag("IDEA"), multispace1)).parse(input)?;
    let (input, id) = identifier(input)?;
    let (input, _) = multispace1(input)?;
    let (input, text) = quoted_string(input)?;
    
    Ok((
        input,
        Imperative {
            kind: ImperativeKind::DefineIdea {
                id: id.to_string(),
                text: text.to_string(),
            },
        },
    ))
}

/// Parser para o comando SIMULATE ENTITY.
fn simulate_entity(input: &str) -> IResult<&str, Imperative> {
    let (input, _) = tuple((tag("SIMULATE"), multispace1, tag("ENTITY"), multispace1)).parse(input)?;
    let (input, id) = identifier(input)?;
    let (input, _) = multispace1(input)?;
    let (input, rounds) = map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)?;
    
    Ok((
        input,
        Imperative {
            kind: ImperativeKind::SimulateEntity {
                id: id.to_string(),
                rounds,
            },
        },
    ))
}

/// Parser para o comando ORCHESTRATE.
fn orchestrate(input: &str) -> IResult<&str, Imperative> {
    let (input, _) = tuple((tag("ORCHESTRATE"), multispace1)).parse(input)?;
    let (input, mode) = identifier(input)?;
    
    Ok((
        input,
        Imperative {
            kind: ImperativeKind::Orchestrate {
                mode: mode.to_string(),
            },
        },
    ))
}

/// Parser para o comando INVOKE RULESET.
fn invoke_ruleset(input: &str) -> IResult<&str, Imperative> {
    let (input, _) = tuple((tag("INVOKE"), multispace1, tag("RULESET"), multispace1)).parse(input)?;
    let (input, ruleset_id) = identifier(input)?;
    let (input, _) = tuple((multispace1, tag("ON"), multispace1)).parse(input)?;
    let (input, entity_id) = identifier(input)?;
    
    Ok((
        input,
        Imperative {
            kind: ImperativeKind::InvokeRuleset {
                entity_id: entity_id.to_string(),
                ruleset_id: ruleset_id.to_string(),
            },
        },
    ))
}

/// Parser para uma string entre aspas duplas.
fn quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        take_till(|c| c == '"'),
        char('"'),
    )
    .parse(input)
}

/// Parser para uma cláusula de contrato.
fn clause(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ' || c == '.' || c == ':'
        || c == '(' || c == ')' || c == '[' || c == ']' || c == '$' || c == '%' || c == '@')(input)
}

/// Parser para um identificador.
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '@')(input)
}