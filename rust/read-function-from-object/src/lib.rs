use ducktor::FromJsValue as DuckType;
use js_sys::{Function as JsFunction, Object as JsObject, Reflect as JsReflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsError, JsValue};

// Note: TypeScript definitions are up to us to manually define.
pub struct NamedFnRef {
  fn1: JsFunction,
  fn2: JsFunction,
}

/// Read the `fn1` and `fn2` functions from the `raw_driver` object.
fn reify(raw_driver: JsObject) -> NamedFnRef {
  let fn1 = JsReflect::get(&raw_driver, &"fn1".into())
    .expect(r#"Property "fn1" does not exist"#)
    .dyn_into::<JsFunction>()
    .expect(r#"Property "fn1" is not a function"#);

  let fn2 = JsReflect::get(&raw_driver, &"fn2".into())
    .expect(r#"Property "fn2" does not exist"#)
    .dyn_into::<JsFunction>()
    .expect(r#"Property "fn2" is not a function"#);

  NamedFnRef { fn1, fn2 }
}

#[wasm_bindgen]
pub struct JsExecutor {
  named_fn_ref: NamedFnRef,
}

#[wasm_bindgen]
impl JsExecutor {
  #[wasm_bindgen(constructor)]
  pub fn new(object_of_fns: JsObject) -> Self {
    let named_fn_ref = reify(object_of_fns);
    JsExecutor { named_fn_ref }
  }

  #[wasm_bindgen]
  pub fn call_fn1(&self) -> Result<JsValue, JsError> {
    // free functions don't have any `this` value
    let this = JsValue::NULL;

    self
      .named_fn_ref
      .fn1
      .call0(&this) // call the `fn1` function with no arguments
      .map_err(to_js_error) // convert the string error message into a `JsError`
  }

  #[wasm_bindgen]
  pub fn call_fn2(&self) -> Result<JsValue, JsError> {
    // free functions don't have any `this` value
    let this = JsValue::NULL;

    self
      .named_fn_ref
      .fn2
      .call0(&this) // call the `fn1` function with no arguments
      .map_err(to_js_error) // convert the string error message into a `JsError`
  }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(DuckType, Default)]
pub struct Fn3Container {
  pub fn3: JsFunction,
}

#[wasm_bindgen]
pub fn call_fn3(js_object: JsObject) -> Result<JsValue, JsError> {
  let fn3_container: Fn3Container = Fn3Container::from(&js_object.into());

  // free functions don't have any `this` value
  let this = JsValue::NULL;

  fn3_container
    .fn3
    .call0(&this) // call the `fn3` function with no arguments
    .map_err(to_js_error) // convert the string error message into a `JsError`
}

fn to_js_error(err: JsValue) -> JsError {
  let err_string = err.as_string().unwrap_or_else(|| "Unknown error".to_string());
  JsError::new(err_string.as_str())
}
