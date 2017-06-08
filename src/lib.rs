extern crate futures;
extern crate owning_ref;

use futures::future::{Future, join_all, ok};
use futures::stream::{Stream, iter};
use owning_ref::OwningHandle;

fn main() {
    let vec1: Vec<()> = Vec::new();
    let vec2: Vec<()> = Vec::new();

    let data = Box::new(vec1);
    OwningHandle::new_with_fn(data, |data_ptr| {
        let vec1 = unsafe { &*data_ptr };
        Box::new(join_all(vec2.iter()
                          .map(move |_| {
                              iter::<_, _, ()>(vec1.iter()
                                               .filter_map(move |_| {
                                                   Some(Ok(ok::<_, ()>(5)))
                                               }))
                                  .and_then(|f| f)
                                  .for_each(move |_| {
                                      ok::<_, ()>(2).map(|_| ())
                                  })
                          })))
    });
}

