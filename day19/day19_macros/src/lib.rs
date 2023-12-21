use std::{fs, str::FromStr};

use aoc::ParseLineError;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, BinOp, LitStr, LitInt, token::{Gt, Lt, Ne}};

#[derive(Debug)]
struct WorkflowDefinition {
    name: Ident,
    steps: Vec<WorkflowStep>,
    final_step: Ident,
}

impl FromStr for WorkflowDefinition {

    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 || s.starts_with("{") {
            return Err(ParseLineError::new("Workflow Definition", s));
        }
        let s = s.replace("}", "");
        let mut split = s.split("{");
        let name: Ident = syn::parse_str(split.next().unwrap()).unwrap_or_else(|_| {
            println!("{} could not be parsed!", s);
            syn::parse_str("my_fn").unwrap()
        });
        let mut steps: Vec<WorkflowStep> = Vec::new();
        let mut step_defs = split.next().unwrap().split(",");
        let mut final_step: Option<Ident> = None;
        while let Some(s) = step_defs.next() {
            if s.contains(":") {
                let mut split = s.split(":");
                let condition = split.next().unwrap();
                let dest: Ident = syn::parse_str(split.next().unwrap()).unwrap_or_else(|s| {
                    println!("{} could not be parsed:", s);
                    syn::parse_str("my_fn").unwrap()
                });
                let field: Ident = syn::parse_str(&condition[0..1]).unwrap_or_else(|s| {
                    println!("{} could not be parsed:", s);
                    syn::parse_str("my_fn").unwrap()
                });
                let op: BinOp = syn::parse_str(&condition[1..2]).unwrap();
                let value: LitInt = syn::parse_str(&condition[2..condition.len()]).unwrap();
                let step = WorkflowStep { dest, field, op, value };
                steps.push(step);
            } else {
                final_step = Some(syn::parse_str(s).unwrap());
            }
        }
        Ok(WorkflowDefinition {
            name,
            steps,
            final_step: final_step.unwrap(),
        })
    }

}

#[derive(Debug, Clone)]
struct WorkflowStep {
    dest: Ident,
    field: Ident,
    op: BinOp,
    value: LitInt,
}

impl ToTokens for WorkflowStep {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let field = self.field.clone();
        let op = self.op;
        let dest = self.dest.clone();
        let value = self.value.clone();
        let res = quote! {
            if part.#field #op #value {
                return #dest(part);
            }
        };
        tokens.append_all(res);
    }
}

#[derive(Debug, Clone)]
struct WorkflowStepForRange {
    dest: Ident,
    field: Ident,
    op: BinOp,
    value: LitInt,
}

impl From<&WorkflowStep> for WorkflowStepForRange {
    fn from(value: &WorkflowStep) -> Self {
        WorkflowStepForRange {
            dest: value.dest.clone(),
            field: value.field.clone(),
            op: value.op.clone(),
            value: value.value.clone(),
        }
    }
}

impl ToTokens for WorkflowStepForRange {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let field = self.field.clone();
        let split_fn_op = match self.op {
            BinOp::Lt(_) => "lt",
            BinOp::Gt(_) => "gt",
            _ => panic!("Unknown operator"),
        };
        let split_fn: Ident = syn::parse_str(&format!("split_{}_{}", field, split_fn_op)).unwrap();
        let dest: Ident = syn::parse_str(&format!("range_{}", self.dest)).unwrap();
        let value = self.value.clone();
        let res = quote! {
            //println!(" - {:?} -> {}", range, stringify!(#split_fn));
            let split = #split_fn(range, #value);
            if let Some(mut s) = split {
                res += #dest(&mut s);
            }
            if range.#field.len() == 0 {
                return res;
            }
        };
        tokens.append_all(res);
    }
}

#[proc_macro]
pub fn make_workflows(input: TokenStream) -> TokenStream {
    let (fn_names, steps, final_steps) = read_input(input);
    make_workflow_fns(&fn_names, &steps, &final_steps)
}

#[proc_macro]
pub fn make_ranges(input: TokenStream) -> TokenStream {
    let (fn_names, steps, final_steps) = read_input(input);
    make_range_fns(&fn_names, &steps, &final_steps)
}

fn read_input(input: TokenStream) -> (Vec<Ident>, Vec<Vec<WorkflowStep>>, Vec<Ident>) {
    let literal: LitStr = syn::parse(input).unwrap();
    let path = literal.value();
    println!("{}", path);
    let file = fs::read_to_string(&path).unwrap();
    let file = file.replace("fn", "my_fn")  // stupid input!
        .replace("in", "my_in")
        .replace("A", "fn_true")
        .replace("R", "fn_false")
        .replace("\r", "");
    let workflows: Vec<WorkflowDefinition> = file.split("\n").filter_map(|line| {
        match line.parse() {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }).collect();
    let fn_names: Vec<Ident> = workflows.iter().map(|wf| wf.name.clone()).collect();
    let steps: Vec<Vec<WorkflowStep>> = workflows.iter().map(|wf| wf.steps.clone()).collect();
    let final_steps: Vec<Ident> = workflows.iter().map(|wf| wf.final_step.clone()).collect();
    (fn_names, steps, final_steps)
}

fn make_workflow_fns(fn_names: &Vec<Ident>, steps: &Vec<Vec<WorkflowStep>>, final_steps: &Vec<Ident>) -> TokenStream {
    quote! {
        #(fn #fn_names(part: &Part) -> bool {
            //println!(" -> {}", stringify!(#fn_names));
            #(#steps)*
            #final_steps(part)
        })*
    }.into()
}

fn make_range_fns(fn_names: &Vec<Ident>, steps: &Vec<Vec<WorkflowStep>>, final_steps: &Vec<Ident>) -> TokenStream {
    let range_names: Vec<Ident> = fn_names.iter()
        .map(|nm| syn::parse_str(&format!("range_{}", nm)).unwrap())
        .collect();
    let range_steps: Vec<Vec<WorkflowStepForRange>> = steps.iter().map(|v| {
        v.iter().map(|s| WorkflowStepForRange::from(s)).collect()
    }).collect();
    let final_steps: Vec<Ident> = final_steps.iter()
        .map(|nm| syn::parse_str(&format!("range_{}", nm)).unwrap())
        .collect();
    quote! {
        #(fn #range_names(range: &mut PartRange) -> u64 {
            println!("{:?} -> {}", range, stringify!(#range_names));
            let mut res = 0;
            #(#range_steps)*
            res += #final_steps(range);
            res
        })*
    }.into()
}

#[proc_macro]
pub fn make_splits(_: TokenStream) -> TokenStream {
    make_split_fns(&vec!["x", "m", "a", "s"])
}

fn make_split_fns(fields: &Vec<&str>) -> TokenStream {
    let fn_names_lt: Vec<Ident> = fields.iter().map(|f| syn::parse_str(&format!("split_{}_lt", f)).unwrap()).collect();
    let fn_names_gt: Vec<Ident> = fields.iter().map(|f| syn::parse_str(&format!("split_{}_gt", f)).unwrap()).collect();
    let fields: Vec<Ident> = fields.iter().map(|f| syn::parse_str(f).unwrap()).collect();
    quote! {
        #(fn #fn_names_lt(old: &mut PartRange, split: u64) -> Option<PartRange> {
            let mut old_ranges: Vec<(u64, u64)> = Vec::new();
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            for range in &old.#fields {
                if range.1 < split {
                    new_ranges.push(*range);
                    continue;
                }
                if range.0 >= split {
                    old_ranges.push(*range);
                    continue;
                }
                new_ranges.push((range.0, split - 1));
                old_ranges.push((split, range.1));
            }
            old.#fields = old_ranges;
            if new_ranges.len() > 0 {
                let mut split = old.clone();
                split.#fields = new_ranges;
                Some(split)
            } else {
                None
            }
        })*

        #(fn #fn_names_gt(old: &mut PartRange, split: u64) -> Option<PartRange> {
            let mut old_ranges: Vec<(u64, u64)> = Vec::new();
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            for range in &old.#fields {
                if range.0 > split {
                    new_ranges.push(*range);
                    continue;
                }
                if range.1 <= split {
                    old_ranges.push(*range);
                    continue;
                }
                new_ranges.push((split + 1, range.1));
                old_ranges.push((range.0, split));
            }
            old.#fields = old_ranges;
            if new_ranges.len() > 0 {
                let mut split = old.clone();
                split.#fields = new_ranges;
                Some(split)
            } else {
                None
            }
        })*
    }.into()
}