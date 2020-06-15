use std::marker::PhantomData;
use snowcat_derive::{Blueprint, Transferable};
use std::net::TcpStream;
use std::io::Write;

pub trait Blueprint {
    fn get_blueprint() -> String;

    fn get_table() -> String;
}

pub trait Transferable {
    fn serialize(&self) -> String;
}

#[derive(Debug, Clone)]
enum ActionType {
    Filter,
    Insert,
    Order,
    CreateTable,
}

impl ActionType {
    pub fn to_string(&self) -> String {
        match self {
            ActionType::Filter => "filter",
            ActionType::Insert => "insert",
            ActionType::Order => "order",
            ActionType::CreateTable => "create_table",
        }.to_string()
    }
}

#[derive(Debug, Clone)]
enum ActionPayload<T: Transferable> {
    Condition(Condition),
    Object(T),
}

#[derive(Debug, Clone)]
struct Action<T: Transferable> {
    action_type: ActionType,
    payload: ActionPayload<T>,
}

pub struct Connect<T: Blueprint + Transferable> {
    table_name: String,
    actions: Vec<Action<T>>,
    bind: String,
    _marker: PhantomData<T>,
}

impl<T: Blueprint + Transferable> Connect<T> {
    pub fn new(bind: &str) -> Connect<T> {
        Connect {
            table_name: T::get_table(),
            actions: Vec::new(),
            bind: bind.to_string(),
            _marker: PhantomData::<T> {},
        }
    }

    pub fn insert(&mut self, value: T) -> &Connect<T> {
        &self.actions.push(Action {
            action_type: ActionType::Insert,
            payload: ActionPayload::Object(value)
        });

        self
    }

    pub fn filter(&mut self, condition: Condition) -> &Connect<T> {
        if !self.validation_condition(&condition) {
            panic!("Wrong condition for this table")
        };

        &self.actions.push(Action {
            action_type: ActionType::Filter,
            payload: ActionPayload::Condition(condition)
        });

        self
    }

    pub fn execute(&self) {
        println!("{}", self.serialize());
        let string = self.serialize();

        let mut stream = TcpStream::connect(&self.bind).unwrap();
        stream.write(string.as_bytes());
    }

    fn validation_condition(&self, condition: &Condition) -> bool {
        true
    }

    fn serialize(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!(">{}\n", &self.table_name));

        for action in &self.actions {
            let payload = match &action.payload {
                ActionPayload::Condition(e) => e.serialize(),
                ActionPayload::Object(e) => e.serialize(),
            };

            result.push_str(&format!("#{}\n", action.action_type.to_string()));
            result.push_str(&format!("{}\n", payload));
        }

        result
    }
}

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