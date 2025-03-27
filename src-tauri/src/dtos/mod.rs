use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub mod doc;
pub mod tag;

/// 进度条数据组件
/// 为甚么要设计成ProgressWrapper包着Mutex<Progress<'x>>呢？
/// 一开始使用单独的Progress<'x>，但是需要更新的update函数需要&mut self，这样就不能多处引用了，
/// 所以用了ProgressWrapper里面用Mutex包裹一下Progress<'x>来提供内部可变性，这样就可以在多个地方引用了。
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// #[tokio::main]
/// async fn main() {
///     use std::sync::Arc;
///     let progress_wrapper = Arc::new(ProgressWrapper::new("正在插入文档", 10));
///     let _ = app_handle.emit("progress_event", progress_wrapper.clone().update("路径不存在", 1), );
/// }
/// ```
#[derive(Debug)]
pub struct ProgressWrapper<'x> {
    progress: Mutex<Progress<'x>>,
}
impl ProgressWrapper<'_> {
    pub fn new(msg: &'static str, max: i32) -> Self {
        let progress = Progress::new(msg, max);
        Self {
            progress: Mutex::new(progress),
        }
    }
    /// 更新进度条，返回更新后的进度条数据
    /// 为什么这里的update函数用的self: &Arc<Self>而不是&self呢？
    /// 因为进度条要到处move，所以要 Arc<Self>
    /// 如果使用&self的话，Arc<Self>.clone().update()会有一个临时变量的问题：temporary value dropped while borrowed
    /// 因为Arc<Self>.clone()是返回一个临时的Arc指针，然后调用update函数时需要引用这个Arc指针指向的ProgressWrapper，结果结束后会Arc指针释放。
    /// 除非Arc<Self>.clone()绑定到一个临时变量，然后再用这个临时变量来调用update函数，这样才不会出现这个错误。
    /// 但是上面那样就得写两行了，所以就直接用了self: &Arc<Self>。这样调用update函数时直接引用用的是这个Arc指针而不是内部指向的数据，后面再释放也没关系。
    pub fn update(self: &Arc<Self>, msg: &'static str, add: i32) -> Progress {
        {
            //更新完把锁放掉，不然死锁会卡在这，必须用大括号划定作用域！
            let mut progress = self.progress.lock().unwrap();
            *progress = progress.update(msg, add);
        }
        self.progress.lock().unwrap().clone()
    }
    pub fn get_progress(&self) -> Progress {
        self.progress.lock().unwrap().clone()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress<'x> {
    pub id: i32, //为了以后多个进度条做准备
    pub msg: &'x str,
    // pub now: f64,
    // pub now: AtomicI32,
    pub now: i32,
    pub total: i32,
}
impl<'x> Progress<'x> {
    pub fn new(msg: &'static str, total: i32) -> Self {
        let progress_id = rng().random_range(1000..=9999);
        Self {
            id: progress_id,
            msg,
            now: 0,
            total,
        }
    }
    pub fn update(&mut self, msg: &'static str, add: i32) -> Self {
        self.now = self.now + add;
        Self {
            id: self.id,
            msg,
            // now: AtomicI32::new(now as i32),
            now: self.now,
            total: self.total,
        }
    }
}
