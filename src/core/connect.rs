use crate::core::traits::{Blueprint, Transferable};
use crate::core::conditions::Condition;

use std::marker::PhantomData;
use snowcat_derive::{Blueprint, Transferable};
use std::net::TcpStream;
use std::io::Write;

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

