// Copyright 2018 Remi Bernotavicius

#![recursion_limit = "128"]

use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use serde_derive::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::num::ParseIntError;
use std::ops::Range;
use std::path::Path;
use std::process::Command;
use std::str::{self, FromStr};

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

impl OpcodeArgument {
    fn dynamic(&self) -> bool {
        match self {
            Self::Register(_) => false,
            Self::ReadOneByte => true,
            Self::ReadTwoBytes => true,
            Self::ReadAddress => true,
            Self::ConstantValue(_) => false,
        }
    }
}

impl ToTokens for OpcodeArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            OpcodeArgument::Register(r) => quote!(Intel8080Register::#r),
            OpcodeArgument::ReadOneByte => quote!(stream.read_u8().unwrap()),
            OpcodeArgument::ReadTwoBytes => quote!(stream.read_u16::<LittleEndian>().unwrap()),
            OpcodeArgument::ReadAddress => quote!(stream.read_u16::<LittleEndian>().unwrap()),
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
    fn cause(&self) -> Option<&dyn Error> {
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
    fn cause(&self) -> Option<&dyn Error> {
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
            num >>= 8;
        }
        output.code.reverse();

        if output.code.is_empty() {
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
    args: Vec<Ident>,
}

impl OpcodeFunctionCall {
    fn new(function: OpcodeFunction, args: Vec<Ident>) -> Self {
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
    camel_name: String,
    instruction: String,
    description: String,
    function_call: OpcodeFunctionCall,
    enum_args: Vec<OpcodeArgument>,
    size: u8,
    duration: u8,
}

impl Opcode {
    fn from_disk<F>(code: String, on_disk: OpcodeOnDisk, mut function_factory: F) -> Self
    where
        F: FnMut(String, Vec<OpcodeParameterType>) -> OpcodeFunction,
    {
        let description = on_disk.description.to_snake_case();
        let camel_name = on_disk.description.to_camel_case();
        let enum_args: Vec<OpcodeArgument> =
            on_disk.args.iter().map(|e| e.parse().unwrap()).collect();
        let function = function_factory(
            description.clone(),
            enum_args.iter().map(|e| e.clone().into()).collect(),
        );
        let args = function.parameters.iter().map(|p| p.name.clone()).collect();
        Opcode {
            code: code.parse().unwrap(),
            instruction: on_disk.instr,
            camel_name,
            description,
            function_call: OpcodeFunctionCall::new(function, args),
            enum_args,
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
            true
        } else {
            let next_code = opcode.code.subcode(depth + 1);
            if let Some(n) = self.children.get_mut(&next_code) {
                return n.add_opcode(depth + 1, opcode);
            }
            let mut child = OpcodeDispatchTreeNode::new(
                next_code.clone(),
                self.leaf_tokenize,
                self.inner_tokenize,
            );
            child.add_opcode(depth + 1, opcode);
            self.children.insert(next_code, child);
            true
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
    enum_name: Ident,
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

            new_function
        });
        let enum_name = Ident::new(
            &format!("{}Instruction", instruction_set_name),
            Span::call_site(),
        );
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
            enum_name,
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
            use crate::bytes::{ReadBytesExt, LittleEndian};
            use crate::emulator_common::Intel8080Register;
            use crate::#(#use_path)::*::#printer_name;
            use crate::io;
            use serde_derive::{Serialize, Deserialize};
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

    fn build_tree_dispatches(
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

    fn build_dispatches_unique_functions(
        &self,
        leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    ) -> Vec<OpcodeDispatchTreeNode> {
        fn do_nothing_tokenize(_: &OpcodeDispatchTreeNode, _: &mut TokenStream) {}

        let mut functions = BTreeSet::new();
        let mut dispatches = vec![];
        for opcode in &self.opcodes {
            if !functions.insert(opcode.function_call.function.clone()) {
                continue;
            }
            let mut leaf = OpcodeDispatchTreeNode::new(
                opcode.code.clone(),
                leaf_tokenize,
                do_nothing_tokenize,
            );
            leaf.opcode = Some(opcode.clone());
            dispatches.push(leaf);
        }
        dispatches
    }

    fn build_dispatches(
        &self,
        leaf_tokenize: fn(&OpcodeDispatchTreeNode, &mut TokenStream),
    ) -> Vec<OpcodeDispatchTreeNode> {
        fn do_nothing_tokenize(_: &OpcodeDispatchTreeNode, _: &mut TokenStream) {}

        let mut dispatches = vec![];
        for opcode in &self.opcodes {
            let mut leaf = OpcodeDispatchTreeNode::new(
                opcode.code.clone(),
                leaf_tokenize,
                do_nothing_tokenize,
            );
            leaf.opcode = Some(opcode.clone());
            dispatches.push(leaf);
        }
        dispatches
    }

    fn generate_instruction_dispatch(&self, tokens: &mut TokenStream) {
        let enum_name = &self.enum_name;
        let trait_name = &self.trait_name;

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let opcode = tree.opcode.as_ref().unwrap();
            let variant_name = Ident::new(&opcode.camel_name, Span::call_site());
            let function_call = &opcode.function_call;
            let field_names = function_call.function.parameters.iter().map(|n| &n.name);
            tokens.extend(quote!(
                Self::#variant_name { #(#field_names, )* } => machine.#function_call
            ));
        }

        let dispatches = self.build_dispatches_unique_functions(leaf_tokenize);

        tokens.extend(quote!(
            impl #enum_name {
                pub fn dispatch<I: #trait_name>(self, machine: &mut I) {
                    match self {
                        #( #dispatches, )*
                    }
                }
            }
        ));
    }

    fn generate_instruction_enum(&self, tokens: &mut TokenStream) {
        let enum_name = &self.enum_name;

        let mut variants: BTreeMap<String, syn::Variant> = BTreeMap::new();
        for opcode in &self.opcodes {
            let variant_name = Ident::new(&opcode.camel_name, Span::call_site());
            let parameters = &opcode.function_call.function.parameters;
            if parameters.is_empty() {
                variants.insert(
                    opcode.camel_name.clone(),
                    syn::parse_quote!(
                        #variant_name
                    ),
                );
            } else {
                variants.insert(
                    opcode.camel_name.clone(),
                    syn::parse_quote!(
                        #variant_name { #(#parameters, )* }
                    ),
                );
            }
        }

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let opcode = tree.opcode.as_ref().unwrap();
            let code = &opcode.code;
            let variant_name = &opcode.camel_name;
            let variant = Ident::new(variant_name, Span::call_site());
            let field_names = opcode
                .function_call
                .function
                .parameters
                .iter()
                .map(|n| &n.name);
            let values = &opcode.enum_args;
            if values.is_empty() {
                tokens.extend(quote!(
                    #code => Some(Self::#variant)
                ));
            } else {
                tokens.extend(quote!(
                    #code => Some(Self::#variant { #(#field_names : #values,)* })
                ));
            }
        }

        fn inner_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let code = &tree.code;
            let dispatches = tree.children.values();
            tokens.extend(quote!(
                #code => match (#code as u16) << 8 | stream.read_u8()? as u16 {
                    #( #dispatches, )*
                    _ => None,
                }
            ));
        }

        let dispatches = self.build_tree_dispatches(leaf_tokenize, inner_tokenize);

        let variants_values = variants.values();

        tokens.extend(quote!(
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
            pub enum #enum_name {
                #( #variants_values, )*
            }

            impl #enum_name {
                #[allow(clippy::unnecessary_cast)]
                pub fn from_reader<R: io::Read>(mut stream: R) -> io::Result<Option<Self>> {
                    let opcode = stream.read_u8()?;
                    Ok(match opcode {
                        #( #dispatches, )*
                        _ => None,
                    })
                }
            }
        ));
    }

    fn generate_size_fn(&self, tokens: &mut TokenStream) {
        let enum_name = &self.enum_name;

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let opcode = tree.opcode.as_ref().unwrap();
            let variant_name = Ident::new(&opcode.camel_name, Span::call_site());
            let size = opcode.size;
            tokens.extend(quote!(
                Self::#variant_name { .. } => { #size }
            ));
        }

        let dispatches = self.build_dispatches_unique_functions(leaf_tokenize);

        tokens.extend(quote!(
            impl #enum_name {
                pub fn size(&self) -> u8 {
                    match self {
                        #( #dispatches, )*
                    }
                }
            }
        ));
    }

    fn generate_duration_fn(&self, tokens: &mut TokenStream) {
        let enum_name = &self.enum_name;

        fn leaf_tokenize(tree: &OpcodeDispatchTreeNode, tokens: &mut TokenStream) {
            let opcode = tree.opcode.as_ref().unwrap();
            let variant_name = Ident::new(&opcode.camel_name, Span::call_site());
            let duration = opcode.duration;

            let field_names = opcode
                .function_call
                .function
                .parameters
                .iter()
                .map(|n| &n.name);
            let values = &opcode.enum_args;

            let matches: Vec<_> = field_names
                .zip(values.iter())
                .filter(|(_, v)| !v.dynamic())
                .collect();

            let field_names = matches.iter().map(|(k, _)| k);
            let values = matches.iter().map(|(_, v)| v);

            tokens.extend(quote!(
                Self::#variant_name { #(#field_names : #values,)* .. } => { #duration }
            ));
        }

        let dispatches = self.build_dispatches(leaf_tokenize);

        tokens.extend(quote!(
            impl #enum_name {
                pub fn duration(&self) -> u8 {
                    match self {
                        #( #dispatches, )*
                        instr => panic!("invalid instruction {:?}", instr)
                    }
                }
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
        self.generate_instruction_enum(tokens);
        self.generate_size_fn(tokens);
        self.generate_duration_fn(tokens);
        self.generate_instructions_trait(tokens);
        self.generate_instruction_dispatch(tokens);
        self.generate_opcode_printer(tokens);
    }
}

fn generate_opcode_rs(
    output_file: &str,
    opcodes_path: &str,
    instruction_set_name: &'static str,
    opcodes_json: BTreeMap<String, OpcodeOnDisk>,
) {
    let use_path = opcodes_path.split('/').map(Into::into).collect();
    let mut tokens = TokenStream::new();
    tokens.extend(quote! {
        #![allow(dead_code)]
    });
    let generator = OpcodeGenerator::new(instruction_set_name, use_path, opcodes_json);
    generator.generate(&mut tokens);

    let mut out = File::create(output_file).unwrap();
    write!(out, "{}", tokens).unwrap();
    out.flush().unwrap();

    // Try to run rustfmt on it, but don't fail if we are unable.
    Command::new("rustfmt").arg(output_file).status().ok();
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
    fn cause(&self) -> Option<&dyn Error> {
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
            .ok_or_else(|| ParseAddressRangeError::new("Missing start of range".into()))?;
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
    type_name: &str,
    generics: syn::Generics,
    where_clause: Option<syn::WhereClause>,
    mapping: &BTreeMap<AddressRange, MemoryMapping>,
    mutable: bool,
) -> TokenStream {
    let name: Ident = syn::Ident::new(type_name, Span::call_site());

    let expr: &Vec<(syn::Expr, MappingType)> = &mapping
        .iter()
        .map(|(k, v)| {
            (
                match k {
                    AddressRange::Range { start, end } if *start > 0 => {
                        syn::parse_quote!((#start..#end).contains(&address))
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

    let selected_field: &Vec<(syn::Expr, MappingType)> = &mapping
        .values()
        .map(|v| (syn::parse_str(&v.field).unwrap(), v.mapping_type))
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
                MemoryMappedHardware::set_value(&mut self.#set_field, address - #set_offset, value)
            }) else *
        )
    } else {
        syn::parse_quote!(panic!("Called set_memory on non-mutable MemoryMap"))
    };

    let memory_controller: syn::Path =
        syn::parse_quote!(crate::game_boy_emulator::memory_controller);

    quote!(
        use super::#name;

        impl #generics #memory_controller::MemoryAccessor for #name #generics #where_clause {
            #[allow(clippy::identity_op, clippy::if_same_then_else)]
            fn read_memory(&self, address: u16) -> u8 {
                #(if #read_expr {
                    MemoryMappedHardware::read_value(&self.#read_field, address - #read_offset)
                }) else *
                else {
                    0xFF
                }
            }

            #[allow(unused_variables, clippy::identity_op, clippy::if_same_then_else)]
            fn set_memory(&mut self, address: u16, value: u8) {
                #set_memory_body
            }

            fn describe_address(&self, _address: u16) -> #memory_controller::MemoryDescription {
                #memory_controller::MemoryDescription::Instruction
            }
        }
    )
}

fn generate_memory_map(
    memory_map_path: &str,
    type_name: &str,
    generics_str: &str,
    where_clause_str: &str,
    mutable: bool,
) {
    let memory_map_json = format!("src/{}/memory_map.json", memory_map_path);
    println!("cargo:rerun-if-changed={}", memory_map_json);

    let mapping: BTreeMap<String, MemoryMapping> =
        serde_json::from_reader(File::open(&memory_map_json).unwrap()).unwrap();
    let mapping: BTreeMap<AddressRange, MemoryMapping> = mapping
        .into_iter()
        .map(|(k, v)| (k.parse().unwrap(), v))
        .collect();

    let mut tokens = TokenStream::new();
    tokens.extend(quote! {
        use crate::game_boy_emulator::memory_controller::MemoryMappedHardware;
    });

    let generics = syn::parse_str(generics_str).unwrap();
    let where_clause = syn::parse_str(where_clause_str).unwrap();

    tokens.extend(generate_memory_map_from_mapping(
        type_name,
        generics,
        where_clause,
        &mapping,
        mutable,
    ));

    let file_name = format!("memory_map{}.rs", if mutable { "_mut" } else { "" });
    let output_file = &format!("src/{}/{}", memory_map_path, file_name);
    let mut out = File::create(output_file).unwrap();
    write!(out, "{}", tokens).unwrap();
    out.flush().unwrap();

    Command::new("rustfmt").arg(output_file).status().ok();
}

fn game_pak_title(path: &Path) -> String {
    let mut rom_file = File::open(path).unwrap();
    let mut rom: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom).unwrap();

    const TITLE: Range<usize> = Range {
        start: 0x0134,
        end: 0x0144,
    };

    let title_slice = &rom[TITLE];
    let title_end = title_slice
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(title_slice.len());
    let title: &str = str::from_utf8(&title_slice[..title_end])
        .unwrap_or_else(|_| panic!("Malformed title {:?}", title_slice));
    title.into()
}

fn generate_rom_test_functions(rom_path: &str, expectations_path: &str, tokens: &mut TokenStream) {
    let roms_path: std::path::PathBuf = rom_path.into();
    println!("Looking in {} for ROMs", roms_path.to_string_lossy());
    println!("cargo:rerun-if-changed={}", roms_path.to_string_lossy());

    if !roms_path.exists() {
        println!("Found no ROMs");
        return;
    }

    let mut entries = std::fs::read_dir(roms_path)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    entries.sort_by(|a, b| a.path().partial_cmp(&b.path()).unwrap());

    for rom_entry in entries {
        let rom_path = rom_entry.path();
        println!("Found ROM {}", rom_path.to_string_lossy());
        println!("cargo:rerun-if-changed={}", rom_path.to_string_lossy());

        let game_pak_title = game_pak_title(&rom_path);
        println!("Identified ROM as \"{}\"", game_pak_title);

        let game_pak_title = game_pak_title.to_lowercase().replace(' ', "_");
        let expectations_path: std::path::PathBuf =
            format!("{}/{}/", expectations_path, game_pak_title).into();
        println!(
            "Looking for expectations in {}",
            expectations_path.to_string_lossy()
        );
        println!(
            "cargo:rerun-if-changed={}",
            expectations_path.to_string_lossy()
        );

        if !expectations_path.exists() {
            println!("Found no expectations");
            continue;
        }

        for expectation_entry in std::fs::read_dir(expectations_path).unwrap() {
            let expectation_entry = expectation_entry.unwrap();
            let expectation_path = expectation_entry.path();

            if expectation_path.extension().unwrap_or_default() != "bmp" {
                continue;
            }

            println!("Found expectation {}", expectation_path.to_string_lossy());

            let stem = expectation_path.file_stem().unwrap().to_str().unwrap();

            let mut stem_parts = stem.split('_');
            let ticks: u64 = stem_parts.next().unwrap().parse().unwrap();
            let replay = stem_parts
                .next()
                .map(|p| expectation_path.with_file_name(p.to_owned() + ".replay"));

            println!("Expectation for clock offset {}", ticks);

            let mut test_name = format!("{}_{}", game_pak_title, ticks);
            if let Some(replay) = &replay {
                test_name += "_";
                test_name += replay.file_stem().unwrap().to_str().unwrap();
            }
            let rom_test_name = Ident::new(&test_name, Span::call_site());
            let save_state_rom_test_name =
                Ident::new(&format!("{}_save_state", test_name), Span::call_site());

            let rom_path = rom_path.to_str().unwrap();
            let expectation_path = expectation_path.to_str().unwrap();
            let replay = if let Some(replay) = replay {
                let replay = replay.to_str().unwrap();
                quote!(Some(#replay))
            } else {
                quote!(None)
            };
            tokens.extend(quote! {
                #[test]
                fn #rom_test_name() -> Result<()> {
                    do_rom_test(#rom_path, #ticks, #expectation_path, #replay)
                }

                #[test]
                fn #save_state_rom_test_name() -> Result<()> {
                    do_save_state_rom_test(#rom_path, #ticks, #expectation_path, #replay)
                }
            });
        }
    }
}

fn generate_rom_tests(rom_dir: &str, expectations_dir: &str, module: &str) {
    let output_file = format!("src/{}/tests/rom_tests/gen.rs", module);
    let mut out = File::create(&output_file).unwrap();
    let mut tokens = TokenStream::new();
    tokens.extend(quote! {
        use crate::game_boy_emulator::Result;
        use super::{do_rom_test, do_save_state_rom_test};
    });
    generate_rom_test_functions(rom_dir, expectations_dir, &mut tokens);

    write!(out, "{}", tokens).unwrap();
    out.flush().unwrap();

    // Try to run rustfmt on it, but don't fail if we are unable.
    Command::new("rustfmt").arg(output_file).status().ok();
}

fn main() {
    generate_opcodes("intel_8080_emulator/opcodes", "Intel8080");
    generate_opcodes("lr35902_emulator/opcodes", "LR35902");
    generate_memory_map(
        "game_boy_emulator/memory_controller",
        "GameBoyMemoryMap",
        "<'a, Storage>",
        "where Storage: crate::storage::PersistentStorage",
        false,
    );
    generate_memory_map(
        "game_boy_emulator/memory_controller",
        "GameBoyMemoryMapMut",
        "<'a, Storage>",
        "where Storage: crate::storage::PersistentStorage",
        true,
    );
    generate_memory_map(
        "game_boy_emulator/sound_controller",
        "SoundController",
        "<>",
        "",
        true,
    );
    generate_memory_map(
        "game_boy_emulator/sound_controller/channel1",
        "Channel1",
        "<>",
        "",
        true,
    );
    generate_memory_map(
        "game_boy_emulator/sound_controller/channel2",
        "Channel2",
        "<>",
        "",
        true,
    );
    generate_memory_map(
        "game_boy_emulator/sound_controller/channel3",
        "Channel3",
        "<>",
        "",
        true,
    );
    generate_rom_tests("test/roms", "test/expectations", "game_boy_emulator");
}
