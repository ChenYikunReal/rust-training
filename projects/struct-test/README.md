# Rust结构体的一些说明

结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。
