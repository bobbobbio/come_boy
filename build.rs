// Copyright 2018 Remi Bernotavicius

#![recursion_limit = "128"]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate syn;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Write;
use std::iter;
use std::num::ParseIntError;
use std::process::Command;
use std::str::FromStr;

#[derive(Deserialize)]
struct OpcodeOnDisk {
    instr: String,
    description: String,
    args: Vec<String>,
    size: u8,
    duration: Option<u8>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
enum OpcodeParameterType {
    Register,
    ImmediateOneByteData,
    ImmediateTwoByteData,
    ConstantValue,
    Address,
}

impl OpcodeParameterType {
    fn format_string(&self) -> String {
        match self {
            OpcodeParameterType::Register => "{:?}",
            OpcodeParameterType::ImmediateOneByteData => "#${:02x}",
            OpcodeParameterType::ImmediateTwoByteData => "#${:02x}",
            OpcodeParameterType::ConstantValue => "{}",
            OpcodeParameterType::Address => "${:02x}",
        }
        .into()
    }
}

impl From<OpcodeArgument> for OpcodeParameterType {
    fn from(a: OpcodeArgument) -> Self {
        match a {
            OpcodeArgument::Register(_) => OpcodeParameterType::Register,
            OpcodeArgument::ReadOneByte => OpcodeParameterType::ImmediateOneByteData,
            OpcodeArgument::ReadTwoBytes => OpcodeParameterType::ImmediateTwoByteData,
            OpcodeArgument::ReadAddress => OpcodeParameterType::Address,
            OpcodeArgument::ConstantValue(_) => OpcodeParameterType::ConstantValue,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct OpcodeParameter {
    type_: OpcodeParameterType,
    name: Ident,
}

impl OpcodeParameter {
    fn new(type_: OpcodeParameterType, name: String) -> Self {
        OpcodeParameter {
            type_,
            name: Ident::new(&name, Span::call_site()),
        }
    }

    fn from_index(input: (usize, &OpcodeParameterType)) -> Self {
        let (i, type_) = input;
        Self::new(
            type_.clone(),
            match type_ {
                OpcodeParameterType::Register => format!("register{}", i + 1),
                OpcodeParameterType::ImmediateOneByteData => format!("data{}", i + 1),
                OpcodeParameterType::ImmediateTwoByteData => format!("data{}", i + 1),
                OpcodeParameterType::ConstantValue => format!("data{}", i + 1),
                OpcodeParameterType::Address => format!("address{}", i + 1),
            },
        )
    }
}

impl ToTokens for OpcodeParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        tokens.extend(match self.type_ {
            OpcodeParameterType::Register => quote!(#name: Intel8080Register),
            OpcodeParameterType::ImmediateOneByteData => quote!(#name: u8),
            OpcodeParameterType::ImmediateTwoByteData => quote!(#name: u16),
            OpcodeParameterType::ConstantValue => quote!(#name: u8),
            OpcodeParameterType::Address => quote!(#name: u16),
        });
    }
}

#[derive(Debug, PartialEq, Clone)]
enum OpcodeArgument {
    Register(Ident),
    ReadOneByte,
    ReadTwoBytes,
    ReadAddress,
    ConstantValue(u8),
}

impl ToTokens for OpcodeArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            OpcodeArgument::Register(r) => quote!(Intel8080Register::#r),
            OpcodeArgument::ReadOneByte => quote!(read_u8(&mut stream).unwrap()),
            OpcodeArgument::ReadTwoBytes => quote!(read_u16(&mut stream).unwrap()),
            OpcodeArgument::ReadAddress => quote!(read_u16(&mut stream).unwrap()),
            OpcodeArgument::ConstantValue(v) => quote!(#v),
        });
    }
}

impl FromStr for OpcodeArgument {
    type Err = ParseOpcodeArgumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D8" => Ok(OpcodeArgument::ReadOneByte),
            "D16" => Ok(OpcodeArgument::ReadTwoBytes),
            "adr" => Ok(OpcodeArgument::ReadAddress),
            value => match value.parse() {
                Ok(v) => Ok(OpcodeArgument::ConstantValue(v)),
                Err(_) => Ok(OpcodeArgument::Register(Ident::new(
                    value,
                    Span::call_site(),
                ))),
            },
        }
    }
}

#[derive(Debug)]
struct ParseOpcodeArgumentError;

impl Display for ParseOpcodeArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseOpcodeArgumentError {
    fn description(&self) -> &str {
        ""
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct OpcodeCode {
    code: Vec<u8>,
}

impl OpcodeCode {
    fn new() -> Self {
        OpcodeCode { code: vec![] }
    }

    fn subcode(&self, depth: usize) -> Self {
        OpcodeCode {
            code: vec![self.code[depth]],
        }
    }

    fn split(&self) -> Vec<Self> {
        (0..self.len())
            .into_iter()
            .map(|n| self.subcode(n))
            .collect()
    }

    fn len(&self) -> usize {
        self.code.len()
    }
}

impl ToTokens for OpcodeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut hex_str: String = "0x".into();
        for code in &self.code {
            hex_str += &format!("{:02X}", code);
        }
        tokens.extend(TokenStream::from_str(&hex_str));
    }
}

#[derive(Debug)]
struct OpcodeCodeParseError {
    message: String,
}

impl Display for OpcodeCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for OpcodeCodeParseError {
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl OpcodeCodeParseError {
    fn new(message: String) -> Self {
        OpcodeCodeParseError { message }
    }
}

impl From<ParseIntError> for OpcodeCodeParseError {
    fn from(p: ParseIntError) -> Self {
        OpcodeCodeParseError {
            message: format!("{}", p),
        }
    }
}

impl FromStr for OpcodeCode {
    type Err = OpcodeCodeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = OpcodeCode::new();

        if !s.starts_with("0x") {
            return Err(OpcodeCodeParseError::new(s.into()));
        }

        let mut num = u32::from_str_radix(&s[2..], 16)?;
        while num != 0 {
            output.code.push((num & 0xFF) as u8);
            num = num >> 8;
        }
        output.code.reverse();

        if output.code.len() == 0 {
            output.code.push(0);
        }

        Ok(output)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct OpcodeFunction {
    name: Ident,
    parameters: Vec<OpcodeParameter>,
}

impl OpcodeFunction {
    fn new(name: String, parameter_types: Vec<OpcodeParameterType>) -> Self {
        OpcodeFunction {
            name: Ident::new(&name, Span::call_site()),
            parameters: parameter_types
                .iter()
                .enumerate()
                .map(OpcodeParameter::from_index)
                .collect(),
        }
    }
}

impl ToTokens for OpcodeFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let parameters = &self.parameters;
        tokens.extend(quote!(
            fn #name(&mut self, #(#parameters),*)
        ));
    }
}

#[derive(Debug, PartialEq, Clone)]
struct OpcodeFunctionCall {
    function: OpcodeFunction,
    args: Vec<OpcodeArgument>,
}

impl OpcodeFunctionCall {
    fn new(function: OpcodeFunction, args: Vec<OpcodeArgument>) -> Self {
        OpcodeFunctionCall { function, args }
    }
}

impl ToTokens for OpcodeFunctionCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let function_name = &self.function.name;
        let args = &self.args;
        tokens.extend(quote!(
            #function_name(#(#args),*)
        ));
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Opcode {
    code: OpcodeCode,
    instruction: String,
    description: String,
    function_call: OpcodeFunctionCall,
    size: u8,
    duration: u8,
}

impl Opcode {
    fn from_disk<F>(code: String, on_disk: OpcodeOnDisk, mut function_factory: F) -> Self
    where
        F: FnMut(String, Vec<OpcodeParameterType>) -> OpcodeFunction,
    {
        let description = on_disk.description.replace(" ", "_").to_lowercase();
        let args: Vec<OpcodeArgument> = on_disk.args.iter().map(|e| e.parse().unwrap()).collect();
        let function = function_factory(
            description.clone(),
            args.iter().map(|e| e.clone().into()).collect(),
        );
        Opcode {
            code: code.parse().unwrap(),
            instruction: on_disk.instr,
            description,
            function_call: OpcodeFunctionCall::new(function, args),
            duration: on_disk.duration.unwrap_or(0),
            size: on_disk.size,
        }
    }
}

struct OpcodeDispatchTreeNode {
    code: OpcodeCode,
    opcode: Option<Opcode>,
    children: BTreeMap<OpcodeCode, OpcodeDispatchTreeNode>,
    leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    inner_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
}

impl OpcodeDispatchTreeNode {
    fn new(
        code: OpcodeCode,
        leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
        inner_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    ) -> Self {
        OpcodeDispatchTreeNode {
            code,
            opcode: None,
            children: BTreeMap::new(),
            leaf_tokenize,
            inner_tokenize,
        }
    }

    fn add_opcode(&mut self, depth: usize, opcode: Opcode) -> bool {
        let code = opcode.code.subcode(depth);
        if code != self.code {
            return false;
        }
        if depth == opcode.code.len() - 1 {
            assert!(self.opcode.is_none());
            self.opcode = Some(opcode);
            return true;
        } else {
            let next_code = opcode.code.subcode(depth + 1);
            match self.children.get_mut(&next_code) {
                Some(n) => {
                    return n.add_opcode(depth + 1, opcode);
                }
                _ => {}
            }
            let mut child = OpcodeDispatchTreeNode::new(
                next_code.clone(),
                self.leaf_tokenize,
                self.inner_tokenize,
            );
            child.add_opcode(depth + 1, opcode);
            self.children.insert(next_code, child);
            return true;
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl ToTokens for OpcodeDispatchTreeNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_leaf() {
            (self.leaf_tokenize)(self, tokens);
        } else {
            (self.inner_tokenize)(self, tokens);
        }
    }
}

struct OpcodeDispatchTree {
    root: Option<OpcodeDispatchTreeNode>,
    leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    inner_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
}

impl OpcodeDispatchTree {
    fn new(
        leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
        inner_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    ) -> Self {
        OpcodeDispatchTree {
            root: None,
            leaf_tokenize,
            inner_tokenize,
        }
    }

    fn add_opcode(&mut self, opcode: Opcode) -> bool {
        if self.root.is_none() {
            self.root = Some(OpcodeDispatchTreeNode::new(
                opcode.code.subcode(0),
                self.leaf_tokenize,
                self.inner_tokenize,
            ));
        }
        self.root.as_mut().unwrap().add_opcode(0, opcode)
    }
}

impl ToTokens for OpcodeDispatchTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.root.as_ref().unwrap().to_tokens(tokens);
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct OpcodePrinterFunction {
    function: OpcodeFunction,
    instruction: String,
}

impl OpcodePrinterFunction {
    fn new(function: OpcodeFunction, instruction: String) -> Self {
        OpcodePrinterFunction {
            function,
            instruction,
        }
    }
}

impl ToTokens for OpcodePrinterFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let instruction = &self.instruction;
        let params = self.function.parameters.iter().map(|p| p.name.clone());
        let mut fmt_string = "{:04}".to_string();
        for param in &self.function.parameters {
            fmt_string += " ";
            fmt_string += &param.type_.format_string();
        }
        let function = &self.function;

        tokens.extend(quote!(
            #function {
                self.error = write!(self.stream_out, #fmt_string, #instruction #(, #params)*);
            }
        ));
    }
}

fn build_opcodes<F>(
    opcodes_json: BTreeMap<String, OpcodeOnDisk>,
    mut function_factory: F,
) -> Vec<Opcode>
where
    F: FnMut(String, Vec<OpcodeParameterType>) -> OpcodeFunction,
{
    let mut opcodes = vec![];

    for (code, opcode_on_disk) in opcodes_json {
        opcodes.push(Opcode::from_disk(
            code,
            opcode_on_disk,
            &mut function_factory,
        ));
    }

    opcodes
}

struct OpcodeGenerator {
    instruction_set_name: &'static str,
    trait_name: Ident,
    use_path: Vec<Ident>,
    printer_name: Ident,
    opcodes: Vec<Opcode>,
    functions: BTreeMap<Ident, OpcodeFunction>,
}

impl OpcodeGenerator {
    fn new(
        instruction_set_name: &'static str,
        use_path: Vec<String>,
        opcodes_json: BTreeMap<String, OpcodeOnDisk>,
    ) -> Self {
        let mut functions = BTreeMap::new();
        let opcodes = build_opcodes(opcodes_json, |name, params| {
            let new_function = OpcodeFunction::new(name, params);

            if let Some(existing) = functions.get(&new_function.name) {
                assert_eq!(existing, &new_function);
                return new_function;
            }

            functions.insert(new_function.name.clone(), new_function.clone());

            return new_function;
        });
        let trait_name = Ident::new(
            &format!("{}InstructionSet", instruction_set_name),
            Span::call_site(),
        );
        let printer_name = Ident::new(
            &format!("{}InstructionPrinter", instruction_set_name),
            Span::call_site(),
        );
        let use_path = use_path
            .iter()
            .map(|p| Ident::new(p, Span::call_site()))
            .collect();

        OpcodeGenerator {
            instruction_set_name,
            trait_name,
            use_path,
            printer_name,
            opcodes,
            functions,
        }
    }

    fn generate_preamble(&self, tokens: &mut TokenStream) {
        let use_path = &self.use_path;
        let printer_name = &self.printer_name;
        tokens.extend(quote!(
            use emulator_common::Intel8080Register;
            use #(#use_path)::*::#printer_name;
            use std::io;
            use util::{read_u16, read_u8};
        ));
    }

    fn generate_instructions_trait(&self, tokens: &mut TokenStream) {
        let trait_name = &self.trait_name;
        let functions = self.functions.values();
        tokens.extend(quote!(
            pub trait #trait_name {
                #(#functions; )*
            }
        ));
    }

    fn build_dispatches(
        &self,
        leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
        inner_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    ) -> Vec<OpcodeDispatchTree> {
        let mut dispatches = vec![OpcodeDispatchTree::new(leaf_tokenize, inner_tokenize)];
        for opcode in &self.opcodes {
            while !dispatches.last_mut().unwrap().add_opcode(opcode.clone()) {
                dispatches.push(OpcodeDispatchTree::new(leaf_tokenize, inner_tokenize));
            }
        }
        dispatches
    }

    fn generate_instruction_dispatch(&self, tokens: &mut TokenStream) {
        let trait_name = &self.trait_name;
        let function_name = Ident::new(
            &format!(
                "dispatch_{}_instruction",
                self.instruction_set_name.to_lowercase()
            ),
            Span::call_site(),
        );

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let code = &tree.opcode.as_ref().unwrap().code;
            let function_call = &tree.opcode.as_ref().unwrap().function_call;
            let duration = tree.opcode.as_ref().unwrap().duration;
            tokens.extend(quote!(
                #code => { machine.#function_call; #duration }
            ));
        }

        fn inner_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let code = &tree.code;
            let dispatches = tree.children.values();
            tokens.extend(quote!(
                #code => match (#code as u16) << 8 | read_u8(&mut stream).unwrap() as u16 {
                    #( #dispatches, )*
                    v => panic!("Unknown opcode {}", v),
                }
            ));
        }

        let dispatches = self.build_dispatches(leaf_tokenize, inner_tokenize);

        tokens.extend(quote!(
            pub fn #function_name<I: #trait_name>(mut stream: &[u8], machine: &mut I) -> u8 {
                let opcode = read_u8(&mut stream).unwrap();
                match opcode {
                    #( #dispatches, )*
                    v => panic!("Unknown opcode {}", v),
                }
            }
        ));
    }

    fn generate_get_instruction(&self, tokens: &mut TokenStream) {
        let function_name = Ident::new(
            &format!(
                "get_{}_instruction",
                self.instruction_set_name.to_lowercase()
            ),
            Span::call_site(),
        );

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let code = &tree.opcode.as_ref().unwrap().code;
            let subcodes = code.split();
            let size = tree.opcode.as_ref().unwrap().size;
            tokens.extend(quote!(
                #code => (vec![#(#subcodes),*], #size)
            ));
        }

        fn inner_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let code = &tree.code;
            let dispatches = tree.children.values();
            tokens.extend(quote!(
                #code => match (#code as u16) << 8 | match read_u8(&mut stream) {
                    Ok(x) => x,
                    _ => return None,
                } as u16
                {
                    #( #dispatches, )*
                    _ => return None,
                }
            ));
        }

        let dispatches = self.build_dispatches(leaf_tokenize, inner_tokenize);

        tokens.extend(quote!(
            pub fn #function_name<R: io::Read>(mut stream: R) -> Option<Vec<u8>> {
                let (mut instr, size) = match read_u8(&mut stream).unwrap() {
                    #( #dispatches, )*
                    _ => return None,
                };
                let op_size = instr.len();
                instr.resize(size as usize, 0);
                stream.read(&mut instr[op_size..]).unwrap();
                return Some(instr);
            }
        ));
    }

    fn generate_opcode_printer(&self, tokens: &mut TokenStream) {
        let trait_name = &self.trait_name;
        let printer_name = &self.printer_name;

        let mut functions = BTreeSet::new();
        for opcode in &self.opcodes {
            functions.insert(OpcodePrinterFunction::new(
                opcode.function_call.function.clone(),
                opcode.instruction.clone(),
            ));
        }

        tokens.extend(quote!(
            impl<'a> #trait_name for #printer_name<'a> {
                #(#functions)*
            }
        ));
    }

    fn generate(&self, tokens: &mut TokenStream) {
        self.generate_preamble(tokens);
        self.generate_instructions_trait(tokens);
        self.generate_instruction_dispatch(tokens);
        self.generate_get_instruction(tokens);
        self.generate_opcode_printer(tokens);
    }
}

fn generate_opcode_rs(
    output_file: &str,
    opcodes_path: &str,
    instruction_set_name: &'static str,
    opcodes_json: BTreeMap<String, OpcodeOnDisk>,
) {
    let use_path = opcodes_path.split("/").map(Into::into).collect();
    let mut tokens = TokenStream::new();
    let generator = OpcodeGenerator::new(instruction_set_name, use_path, opcodes_json);
    generator.generate(&mut tokens);

    let mut out = File::create(output_file).unwrap();
    write!(out, "{}", tokens).unwrap();
    out.flush().unwrap();

    assert!(Command::new("rustfmt")
        .arg(output_file)
        .status()
        .unwrap()
        .success());
}

fn generate_opcodes(opcodes_path: &str, name: &'static str) {
    let opcodes_json = format!("src/{}/opcodes.json", opcodes_path);
    println!("cargo:rerun-if-changed={}", opcodes_json);

    let opcodes: BTreeMap<String, OpcodeOnDisk> =
        serde_json::from_reader(File::open(&opcodes_json).unwrap()).unwrap();

    let output_file = format!("src/{}/opcode_gen.rs", opcodes_path);
    generate_opcode_rs(&output_file, opcodes_path, name, opcodes);
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
enum AddressRange {
    Exact(u16),
    Range { start: u16, end: u16 },
}

impl AddressRange {
    fn start(&self) -> u16 {
        match self {
            AddressRange::Exact(v) => *v,
            AddressRange::Range { start, .. } => *start,
        }
    }
}

#[derive(Debug)]
struct ParseAddressRangeError {
    message: String,
}

impl Display for ParseAddressRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseAddressRangeError {
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl ParseAddressRangeError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<ParseIntError> for ParseAddressRangeError {
    fn from(p: ParseIntError) -> Self {
        Self {
            message: format!("{}", p),
        }
    }
}

fn parse_hex(s: &str) -> Result<u16, ParseIntError> {
    if &s[..2] == "0x" {
        u16::from_str_radix(&s[2..], 16)
    } else {
        u16::from_str_radix(s, 16)
    }
}

impl std::str::FromStr for AddressRange {
    type Err = ParseAddressRangeError;

    fn from_str(s: &str) -> Result<Self, ParseAddressRangeError> {
        let mut parts = s.split("..");
        let start = parts
            .next()
            .ok_or(ParseAddressRangeError::new("Missing start of range".into()))?;
        if let Some(end) = parts.next() {
            Ok(AddressRange::Range {
                start: parse_hex(start)?,
                end: parse_hex(end)?,
            })
        } else {
            Ok(AddressRange::Exact(parse_hex(start)?))
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
enum MappingType {
    Read,
    ReadWrite,
    Write,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct MemoryMapping {
    field: String,
    mapping_type: MappingType,
    #[serde(default)]
    full_address: bool,
}

fn filter_read<T>((v, mapping_type): &(T, MappingType)) -> Option<&T> {
    if mapping_type == &MappingType::Read || mapping_type == &MappingType::ReadWrite {
        Some(v)
    } else {
        None
    }
}

fn filter_write<T>((v, mapping_type): &(T, MappingType)) -> Option<&T> {
    if mapping_type == &MappingType::Write || mapping_type == &MappingType::ReadWrite {
        Some(v)
    } else {
        None
    }
}

fn generate_memory_map_from_mapping(
    mapping: &BTreeMap<AddressRange, MemoryMapping>,
    mutable: bool,
) -> TokenStream {
    let values: BTreeSet<String> = mapping.values().map(|v| v.field.clone()).collect();

    let src_path: &Vec<syn::Expr> = &values
        .iter()
        .map(|v| syn::parse_str(v.as_ref()).unwrap())
        .collect();
    let field_name: &Vec<Ident> = &values
        .iter()
        .map(|v| Ident::new(&v.replace(".", "_"), Span::call_site()))
        .collect();

    let name: Ident = if mutable {
        syn::parse_quote!(GameBoyMemoryMapMut)
    } else {
        syn::parse_quote!(GameBoyMemoryMap)
    };

    let macro_name: Ident = if mutable {
        syn::parse_quote!(build_memory_map_mut)
    } else {
        syn::parse_quote!(build_memory_map)
    };

    let maybe_mut_a = iter::once(if mutable {
        Some(syn::token::Mut(Span::call_site()))
    } else {
        None
    })
    .cycle();
    let maybe_mut_b = maybe_mut_a.clone();

    let expr: &Vec<(syn::Expr, MappingType)> = &mapping
        .iter()
        .map(|(k, v)| {
            (
                match k {
                    AddressRange::Range { start, end } if *start > 0 => {
                        syn::parse_quote!(address >= #start && address < #end)
                    }
                    AddressRange::Range { start: _, end } => syn::parse_quote!(address < #end),
                    AddressRange::Exact(address) => syn::parse_quote!(address == #address),
                },
                v.mapping_type,
            )
        })
        .collect();

    let offset: &Vec<(syn::Expr, MappingType)> = &mapping
        .iter()
        .map(|(k, v)| {
            let offset = if v.full_address { 0 } else { k.start() };
            (syn::parse_quote!(#offset), v.mapping_type)
        })
        .collect();

    let selected_field: &Vec<(Ident, MappingType)> = &mapping
        .values()
        .map(|v| {
            (
                Ident::new(&v.field.replace(".", "_"), Span::call_site()),
                v.mapping_type,
            )
        })
        .collect();

    let read_expr = expr.iter().filter_map(filter_read);
    let set_expr = expr.iter().filter_map(filter_write);
    let read_field = selected_field.iter().filter_map(filter_read);
    let set_field = selected_field.iter().filter_map(filter_write);
    let read_offset = offset.iter().filter_map(filter_read);
    let set_offset = offset.iter().filter_map(filter_write);

    let set_memory_body: syn::Expr = if mutable {
        syn::parse_quote!(
            #(if #set_expr {
                self.#set_field.set_value(address - #set_offset, value)
            }) else *
        )
    } else {
        syn::parse_quote!(panic!("Called set_memory on non-mutable MemoryMap"))
    };

    quote!(
        pub struct #name<'a> {
            #(pub #field_name: &'a #maybe_mut_a dyn super::MemoryMappedHardware,)*
        }

        #[macro_export]
        macro_rules! #macro_name {
            ($f:expr) => {
                crate::game_boy_emulator::memory_controller::memory_map::#name {
                    #(#field_name: & #maybe_mut_b $f.#src_path,)*
                }
            };
        }

        impl<'a> super::MemoryAccessor for #name<'a> {
            fn read_memory(&self, address: u16) -> u8 {
                #(if #read_expr {
                    self.#read_field.read_value(address - #read_offset)
                }) else *
                else {
                    0xFF
                }
            }

            #[allow(unused_variables)]
            fn set_memory(&mut self, address: u16, value: u8) {
                #set_memory_body
            }

            fn describe_address(&self, _address: u16) -> super::MemoryDescription {
                super::MemoryDescription::Instruction
            }
        }
    )
    .into()
}

fn generate_memory_map(memory_map_path: &str, _name: &'static str) {
    let memory_map_json = format!("src/{}/memory_map.json", memory_map_path);
    println!("cargo:rerun-if-changed={}", memory_map_json);

    let mapping: BTreeMap<String, MemoryMapping> =
        serde_json::from_reader(File::open(&memory_map_json).unwrap()).unwrap();
    let mapping: BTreeMap<AddressRange, MemoryMapping> = mapping
        .into_iter()
        .map(|(k, v)| (k.parse().unwrap(), v))
        .collect();

    let output_file = &format!("src/{}/memory_map.rs", memory_map_path);
    let mut out = File::create(output_file).unwrap();

    for &mutable in &[true, false] {
        let tokens = generate_memory_map_from_mapping(&mapping, mutable);
        write!(out, "{}", tokens).unwrap();
    }

    out.flush().unwrap();

    Command::new("rustfmt").arg(output_file).status().ok();
}

fn main() {
    generate_opcodes("intel_8080_emulator/opcodes", "Intel8080");
    generate_opcodes("lr35902_emulator/opcodes", "LR35902");
    generate_memory_map("game_boy_emulator/memory_controller", "GameBoy");
}
