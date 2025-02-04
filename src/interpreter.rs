use std::fmt::Display;
use dashmap::DashMap; // Replaced HashMap with DashMap
use std::sync::{Arc, Mutex};
use crate::{errors::Error, Action, Adapter, Block, Result, Verb};

fn build_node_tree(message: String) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut starts = Vec::new();

    let mut previous: char = ' ';

    for (i, ch) in message.chars().enumerate() {
        if ch == '{' && previous != '\\' {
            starts.push(i);
        } else if ch == '}' && previous != '\\' {
            if let Some(start) = starts.pop() {
                let node = Node {
                    coordinates: (start, i),
                    ..Default::default()
                };
                nodes.push(node);
            }
        }
        previous = ch;
    }
    nodes
}

#[derive(Clone)]
pub struct Interpreter {
    blocks: Arc<Vec<Box<dyn Block>>>,
}

impl Interpreter {
    pub fn new(blocks: Vec<Box<dyn Block>>) -> Self {
        Self {
            blocks: Arc::new(blocks),
        }
    }

    fn _process_blocks(&self, interpreter: Arc<Interpreter>, mut ctx: Context) -> Option<String> {
        interpreter
            .blocks
            .iter()
            .find_map(|b| b.process(&mut ctx).unwrap_or(None))
    }

    fn solve(
        &self,
        message: String,
        mut node_ordered_list: Vec<Node>,
        response: Arc<Mutex<Response>>,
        charlimit: Option<usize>,
    ) -> Result<String> {
        // final renamed to result because of reserved keyword
        let mut result = message.clone();
        let mut total_work = 0;
        let interpreter: Arc<Interpreter> = Arc::new(Interpreter {
            blocks: self.blocks.clone(),
        });

        for i in 0..node_ordered_list.len() {
            let node = &mut node_ordered_list[i];
            node.verb = Some(Verb::new(
                &result[node.coordinates.0..node.coordinates.1 + 1].to_string(),
            ));

            let ctx = Context {
                response: response.clone(),
                verb: node.verb.clone().unwrap(),
                interpreter: interpreter.clone(),
                original_message: message.clone(),
            };

            if let Some(value) = self._process_blocks(interpreter.clone(), ctx) {
                node.output = Some(value)
            } else {
                continue;
            }

            if let Some(limit) = charlimit {
                total_work += node.output.clone().unwrap().len();
                if total_work > limit {
                    return Err(Error::WorkloadExceeded(format!(
                        "{} / {}",
                        total_work, limit
                    )));
                }
            }

            let (start, end) = node.coordinates;
            let message_slice_len = (end + 1) - start;
            let replacement_len = node.output.clone().unwrap().len();
            let differential: isize = replacement_len as isize - message_slice_len as isize;

            let resp = response.lock().unwrap();
            if resp.actions.contains_key("TSE_STOP") {
                return Ok(format!(
                    "{}{}",
                    result[..start].to_string(),
                    node.output.clone().unwrap()
                ));
            }
            drop(resp);

            result = format!(
                "{}{}{}",
                result[..start].to_string(),
                node.output.clone().unwrap(),
                result[end + 1..].to_string()
            );

            for node in node_ordered_list.iter_mut().skip(i + 1) {
                let new_start = if &node.coordinates.0 > &start {
                    node.coordinates.0 as isize + differential
                } else {
                    node.coordinates.0 as isize
                } as usize;

                let new_end = if &node.coordinates.1 > &start {
                    node.coordinates.1 as isize + differential
                } else {
                    node.coordinates.1 as isize
                } as usize;

                node.coordinates = (new_start, new_end);
            }
        }

        Ok(result)
    }

    pub fn process(
        &self,
        message: String,
        seed_variables: Option<DashMap<String, Adapter>>, // Replaced HashMap with DashMap
        charlimit: Option<usize>,
    ) -> Result<Response> {
        let response = Arc::new(Mutex::new(Response::default()));

        if let Some(variables) = seed_variables {
            response.lock().unwrap().variables.extend(variables);
        }

        let node_ordered_list = build_node_tree(message.clone());

        let output = self.solve(message, node_ordered_list, response.clone(), charlimit)?;
        {
            let mut resp = response.lock().unwrap();
            if resp.body.is_none() {
                resp.body = Some(output);
            }
        };
        Ok(Arc::try_unwrap(response).unwrap().into_inner().unwrap())
    }
}

pub struct Context {
    pub verb: Verb,
    pub original_message: String,
    pub interpreter: Arc<Interpreter>,
    pub response: Arc<Mutex<Response>>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub body: Option<String>,
    pub actions: DashMap<String, Action>, // Replaced HashMap with DashMap
    pub variables: DashMap<String, Adapter>, // Replaced HashMap with DashMap
}

impl Default for Response {
    fn default() -> Self {
        Self {
            body: None,
            actions: DashMap::new(), // Replaced HashMap with DashMap
            variables: DashMap::new(), // Replaced HashMap with DashMap
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    output: Option<String>,
    coordinates: (usize, usize),
    verb: Option<Verb>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{:#?} at {:#?}",
            self.verb.as_ref().map(|v| v.to_string()),
            self.coordinates
        );
        write!(f, "{}", str)
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            output: None,
            coordinates: (0, 0),
            verb: None,
        }
    }
}