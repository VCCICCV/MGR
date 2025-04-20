use std::{collections::HashMap, hash::Hash};

use rayon::prelude::*;

/// 通用树构建器
///
/// 提供高性能的树结构构建功能，支持并行处理和自定义排序。
///
/// # 特点
/// - 高性能：支持并行排序和处理
/// - 内存优化：智能预分配内存
/// - 通用性：支持任意数据类型
/// - 灵活性：支持自定义ID类型和排序规则
pub struct TreeBuilder;

impl TreeBuilder {
    /// 构建有序树结构（支持并行）
    ///
    /// # 参数
    /// - `nodes`: 源节点集合
    /// - `id_fn`: 获取节点ID的函数
    /// - `pid_fn`: 获取父节点ID的函数
    /// - `order_fn`: 获取排序键的函数
    /// - `set_children_fn`: 设置子节点的函数
    ///
    /// # 类型参数
    /// - `T`: 节点类型
    /// - `Id`: ID类型，必须可比较和可哈希
    /// - `Order`: 排序键类型，必须可比较
    ///
    /// # 示例
    /// ```rust
    /// let tree = TreeBuilder::build(
    ///     nodes,
    ///     |node| node.id,                            // ID获取器
    ///     |node| node.parent_id,                     // 父ID获取器
    ///     |node| node.sequence,                      // 排序键
    ///     |node, children| node.children = children, // 设置子节点
    /// );
    /// ```
    ///
    /// # 性能注意事项
    /// - 对于大数据集（>1000条），会自动调整内存分配策略
    /// - 使用并行排序提高性能
    /// - 预分配内存减少重新分配
    #[inline]
    pub fn build<T, Id, Order, F1, F2, F3, F4>(
        mut nodes: Vec<T>,
        id_fn: F1,
        pid_fn: F2,
        order_fn: F3,
        mut set_children_fn: F4,
    ) -> Vec<T>
    where
        T: Sized + Send + Sync,
        Id: Eq + Hash + Send + Sync,
        Order: Ord + Send + Sync,
        F1: Fn(&T) -> Id + Send + Sync,
        F2: Fn(&T) -> Option<Id> + Send + Sync,
        F3: Fn(&T) -> Order + Send + Sync,
        F4: FnMut(&mut T, Vec<T>) + Send + Sync,
    {
        if nodes.is_empty() {
            return vec![];
        }

        let len = nodes.len();

        // 并行排序
        nodes.par_sort_unstable_by_key(|node| order_fn(node));

        // 智能容量预分配
        let (root_capacity, child_capacity) = Self::calculate_capacity(len);
        let mut root_nodes = Vec::with_capacity(root_capacity);
        let mut child_map = HashMap::with_capacity(child_capacity);

        // 单次遍历分组
        for node in nodes {
            match pid_fn(&node) {
                Some(pid) => {
                    child_map
                        .entry(pid)
                        .or_insert_with(|| Vec::with_capacity(4))
                        .push(node);
                },
                None => root_nodes.push(node),
            }
        }

        // 递归构建树
        Self::attach_children(
            &mut root_nodes,
            &mut child_map,
            &id_fn,
            &mut set_children_fn,
        );

        root_nodes
    }

    /// 构建无序树结构（最高性能）
    ///
    /// 与 `build` 方法相比，此方法:
    /// - 不进行排序，性能更高
    /// - 不需要并行支持
    /// - 内存占用更少
    ///
    /// # 使用场景
    /// - 数据已经有序
    /// - 不需要排序
    /// - 追求最高性能
    ///
    /// # 示例
    /// ```rust
    /// let tree = TreeBuilder::build_fast(
    ///     nodes,
    ///     |node| node.id,
    ///     |node| node.parent_id,
    ///     |node, children| node.children = children,
    /// );
    /// ```
    #[inline]
    pub fn build_fast<T, Id, F1, F2, F3>(
        nodes: Vec<T>,
        id_fn: F1,
        pid_fn: F2,
        mut set_children_fn: F3,
    ) -> Vec<T>
    where
        T: Sized,
        Id: Eq + Hash,
        F1: Fn(&T) -> Id,
        F2: Fn(&T) -> Option<Id>,
        F3: FnMut(&mut T, Vec<T>),
    {
        if nodes.is_empty() {
            return vec![];
        }

        let len = nodes.len();
        let (root_capacity, child_capacity) = Self::calculate_capacity(len);
        let mut root_nodes = Vec::with_capacity(root_capacity);
        let mut child_map = HashMap::with_capacity(child_capacity);

        for node in nodes {
            match pid_fn(&node) {
                Some(pid) => {
                    child_map
                        .entry(pid)
                        .or_insert_with(|| Vec::with_capacity(4))
                        .push(node);
                },
                None => root_nodes.push(node),
            }
        }

        Self::attach_children(
            &mut root_nodes,
            &mut child_map,
            &id_fn,
            &mut set_children_fn,
        );

        root_nodes
    }

    /// 计算最优容量分配
    #[inline]
    fn calculate_capacity(len: usize) -> (usize, usize) {
        if len < 1000 {
            (len / 3, len * 2 / 3)
        } else {
            (len / 10, len * 9 / 10)
        }
    }

    /// 递归构建树结构
    #[inline]
    fn attach_children<T, Id, F1, F2>(
        nodes: &mut [T],
        child_map: &mut HashMap<Id, Vec<T>>,
        id_fn: &F1,
        set_children_fn: &mut F2,
    ) where
        Id: Eq + Hash,
        F1: Fn(&T) -> Id,
        F2: FnMut(&mut T, Vec<T>),
    {
        for node in nodes.iter_mut() {
            if let Some(mut children) = child_map.remove(&id_fn(node)) {
                Self::attach_children(&mut children, child_map, id_fn, set_children_fn);
                set_children_fn(node, children);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct Node {
        id: i32,
        pid: Option<i32>,
        order: i32,
        children: Vec<Node>,
    }

    #[test]
    fn test_build_tree() {
        let nodes = vec![
            Node {
                id: 1,
                pid: None,
                order: 1,
                children: vec![],
            },
            Node {
                id: 2,
                pid: Some(1),
                order: 2,
                children: vec![],
            },
            Node {
                id: 3,
                pid: Some(1),
                order: 1,
                children: vec![],
            },
        ];

        let tree = TreeBuilder::build(
            nodes,
            |node| node.id,
            |node| node.pid,
            |node| node.order,
            |node, children| node.children = children,
        );

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].children.len(), 2);
        assert_eq!(tree[0].children[0].id, 3);
        assert_eq!(tree[0].children[1].id, 2);
    }

    #[test]
    fn test_performance() {
        let mut nodes = Vec::with_capacity(10000);
        for i in 0..10000 {
            nodes.push(Node {
                id: i,
                pid: if i == 0 { None } else { Some((i - 1) / 10) },
                order: i % 100,
                children: vec![],
            });
        }

        let start = Instant::now();
        let _tree1 = TreeBuilder::build(
            nodes.clone(),
            |node| node.id,
            |node| node.pid,
            |node| node.order,
            |node, children| node.children = children,
        );
        println!("Parallel build: {:?}", start.elapsed());

        let start = Instant::now();
        let _tree2 = TreeBuilder::build_fast(
            nodes,
            |node| node.id,
            |node| node.pid,
            |node, children| node.children = children,
        );
        println!("Fast build: {:?}", start.elapsed());
    }
}
