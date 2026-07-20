use std::rc::Rc;
use std::cell::RefCell;

use crate::builtin::{
    array,
    dictionary,
    string,
    math,
    io,
    system,
};

use crate::ast::{Program, Stmt, Expr};
use crate::runtime::{Environment, Value};
use crate::context::execution::ExecutionContext;
use crate::module::registry::ModuleRegistry;
use crate::module::loader::ModuleLoader;
use crate::runner;
use super::signal::RuntimeSignal;

pub struct Interpreter {
    env: Environment,
    module_loader: ModuleLoader,
    context: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            module_loader: ModuleLoader::new(),
            context: ExecutionContext::new(),
        }
    }

    pub fn set_current_file(
       &mut self,
       path: std::path::PathBuf,
    ) {
       self.context.set_current_file(path);
    }

    pub fn current_file(
       &self,
    ) -> Option<&std::path::PathBuf> {
       self.context.current_file()
    }

    pub fn current_directory(
       &self,
    ) -> Option<&std::path::PathBuf> {
       self.context.current_directory()
    }

    pub fn run(&mut self, program: &Program) {
       for stmt in &program.statements {
          if let Err(signal) = self.execute(stmt) {
              panic!("Unexpected runtime signal: {:?}", signal);
          }
       }
    }

    fn execute_block(
       &mut self,
       statements: &[Stmt],
       env: Environment,
    ) -> Result<(), RuntimeSignal> {

       // Simpan environment lama
       let previous = self.env.clone();

       // Masuk ke scope baru
       self.env = env;

       // Jalankan semua statement
       for stmt in statements {
           if let Err(signal) = self.execute(stmt) {
              // Kembalikan scope lama
              self.env = previous;
              return Err(signal);
           }
       }

       // Keluar dari scope
       self.env = previous;

       Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeSignal> {
        match stmt {
            Stmt::Let { name, value } => {
                let value = self.evaluate(value);
                self.env.define(name.clone(), value);
                Ok(())
            }
            // Start Stmt::Assign
            Stmt::Assign { name, operator, value } => {
                let value = self.evaluate(value);

                let final_value = match operator.as_str() {
                   "=" => value,

                   "+=" => {
                      let old = self.env.get(name).unwrap_or(Value::Null);
                      self.eval_binary(old, &"+".to_string(), value)
                   }

                   "-=" => {
                      let old = self.env.get(name).unwrap_or(Value::Null);
                      self.eval_binary(old, &"-".to_string(), value)
                   }

                   "*=" => {
                      let old = self.env.get(name).unwrap_or(Value::Null);
                      self.eval_binary(old, &"*".to_string(), value)
                   }

                   "/=" => {
                      let old = self.env.get(name).unwrap_or(Value::Null);
                      self.eval_binary(old, &"/".to_string(), value)
                   }

                   "%=" => {
                      let old = self.env.get(name).unwrap_or(Value::Null);
                      self.eval_binary(old, &"%".to_string(), value)
                   }

                   _ => panic!("Unknown assignment operator {}", operator),
               };

               self.env.assign(name, final_value);

               Ok(())
            }
            // End Stmt::Assign

            Stmt::Print { value } => {
                let value = self.evaluate(value);
                println!("{:?}", value);
                Ok(())
            }
            Stmt::If {
              condition,
              then_branch,
              else_branch,
            } => {
              let cond = self.evaluate(condition);

                let is_true = match cond {
                   Value::Boolean(v) => v,
                   Value::Number(n) => n != 0.0,
                   Value::Null => false,
                   Value::String(ref s) => !s.is_empty(),
                   Value::Array(ref a) => !a.borrow().is_empty(),
                   Value::Dictionary(ref d) => !d.borrow().is_empty(),
                   Value::Function { .. } => true,
                };

                if is_true {
                    for stmt in then_branch {
                       self.execute(stmt)?;
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                       self.execute(stmt)?;
                    }
                }

                Ok(())
             }
             Stmt::While { condition, body } => {
                loop {
                   let cond = self.evaluate(condition);

                      let is_true = match cond {
                         Value::Boolean(v) => v,
                         Value::Number(n) => n != 0.0,
                         Value::Null => false,
                         Value::String(ref s) => !s.is_empty(),
                         Value::Array(ref a) => !a.borrow().is_empty(),
                         Value::Dictionary(ref d) => !d.borrow().is_empty(),
                         Value::Function { .. } => true,
                     };

                    if !is_true {
                        break;
                    }

                    for stmt in body {
                      match self.execute(stmt) {
                         Ok(()) => {}

                         Err(RuntimeSignal::Break) => {
                            return Ok(());
                         }

                         Err(RuntimeSignal::Continue) => {
                            break;
                         }
                         Err(RuntimeSignal::Return(value)) => {
                            return Err(RuntimeSignal::Return(value));
                         }
                    }
                 }
               }

               Ok(())
              }
              
              Stmt::For {
                init,
                condition,
                update,
                body,
              } => {
                // jalankan initializer
                self.execute(init)?;

                loop {
                   let cond = self.evaluate(condition);

                   let is_true = match cond {
                      Value::Boolean(v) => v,
                      Value::Number(n) => n != 0.0,
                      Value::Null => false,
                      Value::String(ref s) => !s.is_empty(),
                      Value::Array(ref a) => !a.borrow().is_empty(),
                      Value::Dictionary(ref d) => !d.borrow().is_empty(),
                      Value::Function { .. } => true,
                   };

                   if !is_true {
                      break;
                   }

                   for stmt in body {
                      match self.execute(stmt) {
                          Ok(()) => {}

                          Err(RuntimeSignal::Break) => {
                            return Ok(());
                          }

                          Err(RuntimeSignal::Continue) => {
                            break;
                          }

                          Err(RuntimeSignal::Return(value)) => {
                            return Err(RuntimeSignal::Return(value));
                          }
                      }
                   }

                   // jalankan update
                   self.execute(update)?;
                 }

                 Ok(())
              }

                  
              Stmt::Break => {
                 Err(RuntimeSignal::Break)
              }

              Stmt::Continue => {
                 Err(RuntimeSignal::Continue)
              }
              
              Stmt::Function {
                 name,
                 params,
                 body,
              } => {
                 self.env.define(
                    name.clone(),
                    Value::Function {
                       params: params.clone(),
                       body: body.clone(),
                       env: self.env.clone(),
                    },
                 );

                 Ok(())
              }

              Stmt::Return { value } => {
                 let value = self.evaluate(value);
                 Err(RuntimeSignal::Return(value))
              }
              Stmt::Expression { expression } => {
                 let _ = self.evaluate(expression);
                 Ok(())
              }
              //Start Import
              Stmt::Import { module } => {
                 if !ModuleRegistry::exists(module)
                     && !self.module_loader.module_exists(module)
                 {
                     panic!("Unknown module '{}'", module);
                 }

                 if self.module_loader.is_loaded(module) {
                     return Ok(());
                 }

                 let source = ModuleLoader::load_source(module);

                 self.module_loader.mark_loaded(module);

                 runner::run_source(source, self);

                 Ok(())
              }
              // End Import
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),

            Expr::String(s) => Value::String(s.clone()),

            Expr::Boolean(b) => Value::Boolean(*b),

            Expr::Null => Value::Null,

            Expr::Identifier(name) => {
                self.env.get(name).unwrap_or(Value::Null)
            }

            // Start Expr::Binary
            Expr::Binary {
               left,
               operator,
               right,
            } => {

               let left_value = self.evaluate(left);

               match operator.as_str() {

                  "&&" => {
                      match left_value {
                         Value::Boolean(false) => {
                             Value::Boolean(false)
                         }

                         Value::Boolean(true) => {
                             let right_value = self.evaluate(right);

                             match right_value {
                                 Value::Boolean(v) => Value::Boolean(v),
                                 _ => panic!("Operator && requires boolean operands"),
                             }
                         }

                         _ => panic!("Operator && requires boolean operands"),
                      }
                   }

                   "||" => {
                      match left_value {
                          Value::Boolean(true) => {
                              Value::Boolean(true)
                          }

                          Value::Boolean(false) => {
                              let right_value = self.evaluate(right);

                              match right_value {
                                 Value::Boolean(v) => Value::Boolean(v),
                                 _ => panic!("Operator || requires boolean operands"),
                              }
                          }

                          _ => panic!("Operator || requires boolean operands"),
                       }
                     }

                     _ => {
                       let right_value = self.evaluate(right);

                       self.eval_binary(left_value, operator, right_value)
                     }
                 }
            }
            // End Expr::Binary

            Expr::Call { callee, arguments } => {

                if let Expr::Identifier(name) = &**callee {
                    if let Some(value) = self.call_builtin(name, arguments) {
                        return value;
                    }
                }

                let function = self.evaluate(callee);

                match function {

                   Value::Function {
                      params,
                      body,
                      env,
                   } => {

                      if params.len() != arguments.len() {
                       panic!(
                         "Expected {} arguments, got {}",
                         params.len(),
                         arguments.len()
                      );
                   }

                   let mut local_env = Environment::with_parent(env);

                   for (param, arg) in params.iter().zip(arguments.iter()) {
                     let value = self.evaluate(arg);
                     local_env.define(param.clone(), value);
                   }

                   match self.execute_block(&body, local_env) {

                     Ok(()) => Value::Null,

                     Err(RuntimeSignal::Return(value)) => value,

                     Err(signal) => {
                        panic!("Unexpected runtime signal: {:?}", signal);
                     }
                   }
                 }

                 _ => {
                    panic!("Can only call functions.");
                 }
              }  
            }
            //
            Expr::Array { elements } => {
                let mut values = Vec::new();

                for element in elements {
                   values.push(self.evaluate(element));
                }

                Value::Array(Rc::new(RefCell::new(values)))
            }

            Expr::Index { object, index } => {
                let object = self.evaluate(object);
                let index = self.evaluate(index);

                 match (object, index) {
                     (Value::Array(values), Value::Number(i)) => {
                       let i = i as usize;

                       values
                         .borrow()
                         .get(i)
                         .cloned()
                         .unwrap_or(Value::Null)
                     }
                     (Value::Dictionary(map), Value::String(key)) => {
                       map.borrow()
                         .get(&key)
                         .cloned()
                         .unwrap_or(Value::Null)
                     }

                     _ => {
                       panic!("Index operator can only be used on arrays or dictionaries.");
                     }
                 }
             }
             // start Expr::Unary
             Expr::Unary { operator, right } => {
                let value = self.evaluate(right);

                   // start match
                   match operator.as_str() {
                      "+" => {
                          match value {
                             Value::Number(n) => Value::Number(n),
                             _ => panic!("Unary '+' requires number"),
                          }
                       }

                       "-" => {
                          match value {
                             Value::Number(n) => Value::Number(-n),
                             _ => panic!("Unary '-' requires number"),
                          }
                       }

                       "!" => {
                         let result = match value {
                             Value::Boolean(v) => !v,
                             Value::Number(n) => n == 0.0,
                             Value::Null => true,
                             Value::String(ref s) => s.is_empty(),
                             Value::Array(ref a) => a.borrow().is_empty(),
                             Value::Dictionary(ref d) => d.borrow().is_empty(),
                             Value::Function { .. } => false,
                         };

                         Value::Boolean(result)
                    }
                    // end match

                    _ => panic!("Unknown unary operator '{}'", operator),
                 }
             }
             // End Expr::Unary

             // Start Expr::Dictionary
             Expr::Dictionary { entries } => {
                use std::collections::HashMap;
                use std::rc::Rc;
                use std::cell::RefCell;

                let mut map = HashMap::new();

                for (key_expr, value_expr) in entries {
                   let key = match self.evaluate(key_expr) {
                      Value::String(s) => s,
                      _ => panic!("Dictionary key must be a string"),
                };

                let value = self.evaluate(value_expr);

                   map.insert(key, value);
                }

                Value::Dictionary(Rc::new(RefCell::new(map)))
             }
             // End Expr::Dictionary   
         }
           
    }
    fn call_builtin(
        &mut self,
        name: &str,
        arguments: &[Expr],
    ) -> Option<Value> {
        match name {
           "len" => {
               if arguments.len() != 1 {
                  panic!("len() expects 1 argument");
               }

               let value = self.evaluate(&arguments[0]);

               Some(array::len(value))    
           }
           "push" => {
               if arguments.len() != 2 {
                  panic!("push() expects 2 argument");
                }

                let array = self.evaluate(&arguments[0]);
                let value = self.evaluate(&arguments[1]);

                Some(array::push(array, value))
           }
           "pop" => {
               if arguments.len() != 1 {
                  panic!("pop() expects 1 argument");
               }

               let array = self.evaluate(&arguments[0]);

               Some(array::pop(array))
           }
           "insert" => {
               if arguments.len() != 3 {
                  panic!("insert() expects 3 argument");
               }

               let array = self.evaluate(&arguments[0]);
               let index = self.evaluate(&arguments[1]);
               let value = self.evaluate(&arguments[2]);

               Some(array::insert(array, index, value))
           }
           "remove" => {
              if arguments.len() != 2 {
                panic!("remove() expects 2 argument");
              }

              let array = self.evaluate(&arguments[0]);
              let index = self.evaluate(&arguments[1]);

              Some(array::remove(array, index))
           }
           "clear" => {
              if arguments.len() != 1 {
                panic!("clear() expects 1 argument");
              }

              let value = self.evaluate(&arguments[0]);

              match &value {
                 Value::Array(_) => Some(array::clear(value)),
                 Value::Dictionary(_) => Some(dictionary::clear(value)),
                 _ => panic!("clear() expects an array or dictionary"),
              }
           }
           "reverse" => {
              if arguments.len() != 1 {
                 panic!("reverse() expects 1 argument");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::reverse(array))
           }
           "first" => {
              if arguments.len() != 1 {
                 panic!("first() expects 1 argument");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::first(array))
           }
           "last" => {
              if arguments.len() != 1 {
                 panic!("last() expects 1 argument");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::last(array))
           }
           "is_empty" => {
              if arguments.len() != 1 {
                 panic!("is_empty() expects 1 argument");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::is_empty(array))
           }
           "trim" => {
              if arguments.len() != 1 {
                 panic!("trim() expects 1 argument");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::trim(value))
           }
           "upper" => {
              if arguments.len() != 1 {
                 panic!("upper() expects 1 argument");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::upper(value))
           }
           "lower" => {
              if arguments.len() != 1 {
                 panic!("lower() expects 1 argument");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::lower(value))
           }
           "contains" => {
             if arguments.len() != 2 {
               panic!("contains() expects 2 argument");
             }

             let target = self.evaluate(&arguments[0]);
             let value = self.evaluate(&arguments[1]);

             match &target {
                Value::Array(_) => Some(array::contains(target, value)),
                Value::String(_) => Some(string::contains(target, value)),
                _ => panic!("contains() only supports arrays or strings"),
             }
           }
           "starts_with" => {
             if arguments.len() != 2 {
                panic!("starts_with() expects 2 argument");
             }

             let text = self.evaluate(&arguments[0]);
             let prefix = self.evaluate(&arguments[1]);

             Some(string::starts_with(text, prefix))
           }
           "ends_with" => {
             if arguments.len() != 2 {
               panic!("ends_with() expects 2 argument");
             }

             let text = self.evaluate(&arguments[0]);
             let suffix = self.evaluate(&arguments[1]);

             Some(string::ends_with(text, suffix))
           }
           "replace" => {
             if arguments.len() != 3 {
                panic!("replace() expects 3 argument");
             }

             let text = self.evaluate(&arguments[0]);
             let from = self.evaluate(&arguments[1]);
             let to = self.evaluate(&arguments[2]);

             Some(string::replace(text, from, to))
           }
           "split" => {
             if arguments.len() != 2 {
                panic!("split() expects 2 argument");
             }

             let text = self.evaluate(&arguments[0]);
             let separator = self.evaluate(&arguments[1]);

             Some(string::split(text, separator))
           }
           "keys" => {
             if arguments.len() != 1 {
                panic!("keys() requires 1 argument");
             }

             let dictionary = self.evaluate(&arguments[0]);

             Some(dictionary::keys(dictionary))
           }
           "values" => {
              if arguments.len() != 1 {
                  panic!("values() expects 1 argument");
              }

              let value = self.evaluate(&arguments[0]);

              Some(dictionary::values(value))
           }
           "has_key" => {
              if arguments.len() != 2 {
                 panic!("has_key() expects 2 arguments");
              }

              let dictionary = self.evaluate(&arguments[0]);
              let key = self.evaluate(&arguments[1]);

              Some(dictionary::has_key(dictionary, key))
           }
           "remove_key" => {
              if arguments.len() != 2 {
                 panic!("remove_key() expects 2 arguments");
              }

              let dictionary = self.evaluate(&arguments[0]);
              let key = self.evaluate(&arguments[1]);

              Some(dictionary::remove(dictionary, key))
           }
           "set" => {
              if arguments.len() != 3 {
                 panic!("set() expects 3 arguments");
              }

              let dictionary = self.evaluate(&arguments[0]);
              let key = self.evaluate(&arguments[1]);
              let value = self.evaluate(&arguments[2]);

              Some(dictionary::set(dictionary, key, value))
           }
           "get" => {
              if arguments.len() != 2 && arguments.len() != 3 {
                  panic!("get() expects 2 or 3 arguments");
              }

              let dictionary = self.evaluate(&arguments[0]);
              let key = self.evaluate(&arguments[1]);

              let default = if arguments.len() == 3 {
                 Some(self.evaluate(&arguments[2]))
              } else {
                 None
              };

              Some(dictionary::get(dictionary, key, default))
           }           
           "merge" => {
              if arguments.len() != 2 {
                 panic!("merge() expects 2 arguments");
              }

              let target = self.evaluate(&arguments[0]);
              let source = self.evaluate(&arguments[1]);

              Some(dictionary::merge(target, source))
           }
           "clone" => {
              if arguments.len() != 1 {
                panic!("clone() expects 1 argument");
              }

              let dictionary = self.evaluate(&arguments[0]);

              Some(dictionary::clone(dictionary))
           }
           //
           _ => None,
        }
    }
    // Start eval_binary(){}
    fn eval_binary(
       &self,
       left: Value,
       operator: &String,
       right: Value,
    ) -> Value {
        // Start match operator.as_str()
        match operator.as_str() {
           "&&" => {
               match (left, right) {
                  (Value::Boolean(a), Value::Boolean(b)) => {
                      Value::Boolean(a && b)
                  }
                  _ => panic!("Operator && requires boolean operands"),
               }
            }

            "||" => {
               match (left, right) {
                  (Value::Boolean(a), Value::Boolean(b)) => {
                      Value::Boolean(a || b)
                  }
                  _ => panic!("Operator || requires boolean operands"),
               }
             }
             
             // Start _=>{
             _ => {
                match (left.clone(), right.clone()) {
                   (Value::Number(a), Value::Number(b)) => {
                       match operator.as_str() {
                          "+" => Value::Number(a + b),
                          "-" => Value::Number(a - b),
                          "*" => Value::Number(a * b),
                          "/" => Value::Number(a / b),
                          "%" => Value::Number(a % b),

                          "==" => Value::Boolean(a == b),
                          "!=" => Value::Boolean(a != b),

                          "<" => Value::Boolean(a < b),
                          "<=" => Value::Boolean(a <= b),

                          ">" => Value::Boolean(a > b),
                          ">=" => Value::Boolean(a >= b),

                          _ => Value::Null,
                       }
                   }

                   // Start _ => match (left, right)
                   _ => match (left, right) {

                      (Value::Boolean(a), Value::Boolean(b)) => {
                         match operator.as_str() {
                           "==" => Value::Boolean(a == b),
                           "!=" => Value::Boolean(a != b),
                           _ => Value::Null,
                         }
                      }

                      (Value::String(a), Value::String(b)) => {
                         match operator.as_str() {
                            "==" => Value::Boolean(a == b),
                            "!=" => Value::Boolean(a != b),
                            _ => Value::Null,
                         }
                      }

                      (Value::Null, Value::Null) => {
                         match operator.as_str() {
                            "==" => Value::Boolean(true),
                            "!=" => Value::Boolean(false),
                            _ => Value::Null,
                         }
                      }

                      _ => Value::Null,
                   } 
                   // End _=> match (left, right)
                }
            }
        }
        // End match operator.as_str()
    }
    // End eval_binary()
                   
}
