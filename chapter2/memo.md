# # chaper 2 memo

if とか for とか式とか基本的なところ  
Rust はけつに ; がないと戻り値として扱われるらしい。  
以下の2つは同じ  

```rust
if a == 20 {
    return 20;
}

if a == 20 {
    20
}
```