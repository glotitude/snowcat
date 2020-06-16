use crate::core::traits::Transferable;

use snowcat_derive::{Blueprint, Transferable};

#[derive(Debug, Clone)]
pub enum Operator {
    Eq,
    Nq,
    Gt,
    Ge,
    Lt,
    Le,
}

impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Eq => "eq",
            Operator::Nq => "nq",
            Operator::Gt => "gt",
            Operator::Ge => "ge",
            Operator::Lt => "lt",
            Operator::Le => "le",
        }.to_string()
    }
}

#[derive(Debug, Clone, Transferable)]
pub struct Condition {
    pub field_name: String,
    pub operator: Operator,
    pub value: String,
}

impl Condition {
    pub fn serialize(&self) -> String {
        let mut result = Vec::new();

        result.extend_from_slice(self.field_name.as_bytes());
        result.extend_from_slice(",".as_bytes());

        let op_name = match self.operator {
            Operator::Eq => "Eq",
            Operator::Nq => "Nq",
            Operator::Gt => "Gt",
            Operator::Ge => "Ge",
            Operator::Lt => "Lt",
            Operator::Le => "Le",
        };
        result.extend_from_slice(op_name.as_bytes());
        result.extend_from_slice(",".as_bytes());

        result.extend_from_slice(self.value.as_bytes());

        String::from_utf8(result).unwrap()
    }
}