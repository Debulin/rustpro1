use List::*;

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let a_int = 1u32;
    let a_bool = true;
    let unit = ();
    // 定义一个外部作用域的常量
    const OUTER_CONSTANT: u32 = 100;

    {
        // 定义一个内部作用域的常量
        const INNER_CONSTANT: u32 = 200;
        
        // 在内部作用域中访问外部作用域的常量
        println!("Accessing OUTER_CONSTANT from inner scope: {}", OUTER_CONSTANT);
        
        // 在内部作用域中访问内部作用域的常量
        println!("Accessing INNER_CONSTANT from inner scope: {}", INNER_CONSTANT);
    }
    
    // 在外部作用域中访问外部作用域的常量
    println!("Accessing OUTER_CONSTANT from outer scope: {}", OUTER_CONSTANT);
    
    // 尝试在外部作用域中访问内部作用域的常量（会导致编译错误）
    // println!("Accessing INNER_CONSTANT from outer scope: {}", INNER_CONSTANT); // 取消注释这行会导致编译错误
}
