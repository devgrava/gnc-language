use std::rc::Rc;
use std::cell::RefCell;
use crate::builtin::{array, string};

use crate::ast::{Program, Stmt, Expr};
use crate::runtime::{Environment, Value};
use super::signal::RuntimeSignal;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
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

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                self.eval_binary(left, operator, right)
            }
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

                     _ => {
                       panic!("Index hanya bisa digunakan pada array.");
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
                             Value::Function { .. } => false,
                         };

                         Value::Boolean(result)
                    }
                    // end match

                    _ => panic!("Unknown unary operator '{}'", operator),
                 }
             }
             // End Expr::Unary   
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
                  panic!("len() membutuhkan 1 argumen");
               }

               let value = self.evaluate(&arguments[0]);

               Some(array::len(value))    
           }
           "push" => {
               if arguments.len() != 2 {
                  panic!("push() membutuhkan 2 argumen");
                }

                let array = self.evaluate(&arguments[0]);
                let value = self.evaluate(&arguments[1]);

                Some(array::push(array, value))
           }
           "pop" => {
               if arguments.len() != 1 {
                  panic!("pop() membutuhkan 1 argumen");
               }

               let array = self.evaluate(&arguments[0]);

               Some(array::pop(array))
           }
           "insert" => {
               if arguments.len() != 3 {
                  panic!("insert() membutuhkan 3 argumen");
               }

               let array = self.evaluate(&arguments[0]);
               let index = self.evaluate(&arguments[1]);
               let value = self.evaluate(&arguments[2]);

               Some(array::insert(array, index, value))
           }
           "remove" => {
              if arguments.len() != 2 {
                panic!("remove() membutuhkan 2 argumen");
              }

              let array = self.evaluate(&arguments[0]);
              let index = self.evaluate(&arguments[1]);

              Some(array::remove(array, index))
           }
           "clear" => {
              if arguments.len() != 1 {
                 panic!("clear() membutuhkan 1 argumen");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::clear(array))
           }
           "reverse" => {
              if arguments.len() != 1 {
                 panic!("reverse() membutuhkan 1 argumen");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::reverse(array))
           }
           "first" => {
              if arguments.len() != 1 {
                 panic!("first() membutuhkan 1 argumen");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::first(array))
           }
           "last" => {
              if arguments.len() != 1 {
                 panic!("last() membutuhkan 1 argumen");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::last(array))
           }
           "is_empty" => {
              if arguments.len() != 1 {
                 panic!("is_empty() membutuhkan 1 argumen");
              }

              let array = self.evaluate(&arguments[0]);

              Some(array::is_empty(array))
           }
           "trim" => {
              if arguments.len() != 1 {
                 panic!("trim() membutuhkan 1 argumen");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::trim(value))
           }
           "upper" => {
              if arguments.len() != 1 {
                 panic!("upper() membutuhkan 1 argumen");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::upper(value))
           }
           "lower" => {
              if arguments.len() != 1 {
                 panic!("lower() membutuhkan 1 argumen");
              }

              let value = self.evaluate(&arguments[0]);

              Some(string::lower(value))
           }
           "contains" => {
             if arguments.len() != 2 {
               panic!("contains() membutuhkan 2 argumen");
             }

             let target = self.evaluate(&arguments[0]);
             let value = self.evaluate(&arguments[1]);

             match &target {
                Value::Array(_) => Some(array::contains(target, value)),
                Value::String(_) => Some(string::contains(target, value)),
                _ => panic!("contains() hanya mendukung array atau string"),
             }
           }
           "starts_with" => {
             if arguments.len() != 2 {
                panic!("starts_with() membutuhkan 2 argumen");
             }

             let text = self.evaluate(&arguments[0]);
             let prefix = self.evaluate(&arguments[1]);

             Some(string::starts_with(text, prefix))
           }
           "ends_with" => {
             if arguments.len() != 2 {
               panic!("ends_with() membutuhkan 2 argumen");
             }

             let text = self.evaluate(&arguments[0]);
             let suffix = self.evaluate(&arguments[1]);

             Some(string::ends_with(text, suffix))
           }
           "replace" => {
             if arguments.len() != 3 {
                panic!("replace() membutuhkan 3 argumen");
             }

             let text = self.evaluate(&arguments[0]);
             let from = self.evaluate(&arguments[1]);
             let to = self.evaluate(&arguments[2]);

             Some(string::replace(text, from, to))
           }
           "split" => {
             if arguments.len() != 2 {
                panic!("split() membutuhkan 2 argumen");
             }

             let text = self.evaluate(&arguments[0]);
             let separator = self.evaluate(&arguments[1]);

             Some(string::split(text, separator))
           }
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
                match (left, right) {
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

                   _ => Value::Null,
                }
            }
        }
        // End match operator.as_str()
    }
    // End eval_binary()
                   
}
