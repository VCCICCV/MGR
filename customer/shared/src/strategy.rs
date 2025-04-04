// use std::any::Any;
// use std::collections::HashMap;
// use std::sync::Arc;

// trait AbstractExecuteStrategy<T, U> {
//     fn execute(&self, request: T) -> U;
//     fn execute_resp(&self, request: T) -> U;
//     fn mark(&self) -> String;
// }

// struct AbstractStrategyChoose {
//     // 执行策略集合
//     abstract_execute_strategy_map: HashMap<
//         String,
//         Arc<dyn AbstractExecuteStrategy<Box<dyn Any>, Box<dyn Any>>>
//     >,
// }

// impl AbstractStrategyChoose {
//     fn new() -> Self {
//         Self {
//             abstract_execute_strategy_map: HashMap::new(),
//         }
//     }
//     // 选择策略
//     fn choose(
//         &self,
//         mark: &str
//     ) -> Option<Arc<dyn AbstractExecuteStrategy<Box<dyn Any>, Box<dyn Any>>>> {
//         self.abstract_execute_strategy_map.get(mark).cloned()
//     }
//     // 选择并执行策略
//     fn choose_and_execute<T: 'static + Send + Sync>(&self, mark: &str, request_param: T)
//         where
//             for<'a> &'a dyn AbstractExecuteStrategy<
//                 Box<dyn Any>,
//                 Box<dyn Any>
//             >: AbstractExecuteStrategy<T, ()>
//     {
//         if let Some(execute_strategy) = self.choose(mark) {
//             execute_strategy.execute(Box::new(request_param));
//         } else {
//             panic!("Strategy not found for mark: {}", mark);
//         }
//     }
//     // 选择并执行策略并返回结果
// }

// struct StrategyImpl1 {}

// impl AbstractExecuteStrategy<String, usize> for StrategyImpl1 {
//     fn execute(&self, request: String) -> usize {
//         request.len()
//     }

//     fn execute_resp(&self, request: String) -> usize {
//         request.len()
//     }

//     fn mark(&self) -> String {
//         "strategy1".to_string()
//     }
// }

// struct StrategyImpl2 {}

// impl AbstractExecuteStrategy<String, usize> for StrategyImpl2 {
//     fn execute(&self, request: String) -> usize {
//         request.len() * 2
//     }

//     fn execute_resp(&self, request: String) -> usize {
//         request.len() * 2
//     }

//     fn mark(&self) -> String {
//         "strategy2".to_string()
//     }
// }
