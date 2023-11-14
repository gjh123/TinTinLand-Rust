# 第四课附加题实践

## 第7题

- 定义了一个 Animal trait 和三个不同的类型 Dog、Cat和 Bird，

- 并为它们实现了 Animal trait。然后使用枚举 Animals 将这三个类型包裹起来，并将它们放入一个 Vec 中。

- 最后对 Vec 进行遍历，并根据具体的类型调用各自的方法。

两种不同实现方法的区别： 

1. **静态分发**（Static Dispatch）：在 Rust 中，通过泛型参数来实现静态分发。这种方法在编译时就确定了具体的类型和方法调用，因此具有更高的性能，编译阶段泛型代码展开。
2. **动态分发**（Dynamic Dispatch）：通过 trait object 来实现动态分发。这种方法在运行时根据具体的类型来确定方法调用，因此具有更大的灵活性。但是，由于需要在运行时进行动态的方法查找（**C++的虚表**），因此会带来一些性能开销。

## 第8题
- 实现 Add trait
- 实现一个dynamic_call（动态分发）函数，
- 接受Trait Object作为参数调用对应的实现

## 运行

```
cd lesson_four
cargo run
```
- trait_object::run();  对应 第7题Code
- adder::run(); 对应 第8题Code

## 结果

```
TypeA: Called
TypeB: Called
TypeC: Called
======================
Value: 15
```




