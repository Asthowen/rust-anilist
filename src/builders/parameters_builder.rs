use crate::builders::BuiltQuery;
use serde_json::{Map, Value};

pub(crate) struct ParametersBuilder {
    variables: Map<String, Value>,
    parameters: Vec<String>,
    parameters_definitions: Vec<String>,
    index: usize,
}

impl ParametersBuilder {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            variables: Default::default(),
            parameters: Default::default(),
            parameters_definitions: Default::default(),
            index,
        }
    }

    pub(crate) fn insert(
        &mut self,
        name: &str,
        parameter_type: &str,
        parameter_value: Option<Value>,
    ) {
        if let Some(parameter_value) = parameter_value {
            let var_name = format!("{name}{}", self.index);
            self.parameters.push(format!("{name}: ${var_name}"));
            self.parameters_definitions
                .push(format!("${var_name}: {parameter_type}"));
            self.variables.insert(var_name, parameter_value);
        }
    }

    pub(crate) fn insert_local_parameter(&mut self, name: &str, parameter_value: Option<Value>) {
        if let Some(parameter_value) = parameter_value {
            self.parameters.push(format!("{name}: {parameter_value}"));
        }
    }

    pub(crate) fn insert_local_variable(
        &mut self,
        name: &str,
        parameter_type: &str,
        parameter_value: Option<Value>,
    ) {
        if let Some(parameter_value) = parameter_value {
            let var_name = format!("{name}{}", self.index);
            self.parameters_definitions
                .push(format!("${var_name}: {parameter_type}"));
            self.variables.insert(var_name, parameter_value);
        }
    }

    pub(crate) fn build(self, name: &str, parts: Vec<String>) -> BuiltQuery {
        BuiltQuery {
            field_fragment: format!(
                "{name} ({}) {{ {} }}",
                self.parameters.join(", "),
                parts.join(" ")
            ),
            variables: self.variables,
            parameters_definitions: self.parameters_definitions,
        }
    }
}
