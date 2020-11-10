#![allow(clippy::single_match)]

mod chat;
mod conversations;

use crate::rust::{Member, Method, Module, Response, ResponseType};

pub fn correct(modules: &mut [Module]) {
    for module in modules {
        match module.name.as_str() {
            "chat" => chat::correct(module),
            "conversations" => conversations::correct(module),
            _ => {}
        }
    }
}

fn dedup_vec(r#type: &mut ResponseType) {
    if let ResponseType::Vec(v) = r#type {
        if let ResponseType::Vec(vi) = &v.r#type {
            *v = vi.clone();
        }
    }
}

fn set_parameters_required(method: &mut Method, params: &[&str]) {
    for mut param in &mut method.parameters {
        if params.contains(&param.name.as_str()) {
            param.required = true;
        }
    }
}

struct ResponseTypeModifier<'a>(Option<&'a mut ResponseType>);

impl<'a> ResponseTypeModifier<'a> {
    fn member(self, member: &str) -> ResponseModifier<'a> {
        if let Some(ResponseType::Object(o)) = self.0 {
            o.iter_mut().find(|m| m.name == member).into()
        } else {
            ResponseModifier(None)
        }
    }

    fn member_type(self, member: &str) -> Self {
        self.member(member).r#type()
    }

    fn vec(self) -> ResponseModifier<'a> {
        if let Some(ResponseType::Vec(v)) = self.0 {
            (&mut **v).into()
        } else {
            ResponseModifier(None)
        }
    }

    fn vec_type(self) -> Self {
        self.vec().r#type()
    }

    fn set_to_inner<F>(self, fun: F) -> Self
    where
        for<'b> F: FnOnce(ResponseTypeModifier<'b>) -> ResponseTypeModifier<'b>,
    {
        if let Some(container) = self.0 {
            let replace_opt = fun(container.into()).0.cloned();
            if let Some(replacement) = replace_opt {
                *container = replacement;
            }
            container.into()
        } else {
            Self(None)
        }
    }

    fn split(&mut self) -> ResponseTypeModifier<'_> {
        if let Some(inner) = &mut self.0 {
            ResponseTypeModifier(Some(inner))
        } else {
            ResponseTypeModifier(None)
        }
    }
}

impl<'a> From<&'a mut Method> for ResponseTypeModifier<'a> {
    fn from(m: &'a mut Method) -> Self {
        Self(Some(&mut m.response.r#type))
    }
}

impl<'a> From<&'a mut ResponseType> for ResponseTypeModifier<'a> {
    fn from(t: &'a mut ResponseType) -> Self {
        Self(Some(t))
    }
}

struct ResponseModifier<'a>(Option<&'a mut Response>);

impl<'a> ResponseModifier<'a> {
    pub fn required(mut self, req: bool) -> Self {
        if let Some(ref mut r) = self.0.as_mut() {
            r.required = req;
        }
        self
    }

    pub fn r#type(self) -> ResponseTypeModifier<'a> {
        ResponseTypeModifier(self.0.map(|r| &mut r.r#type))
    }
}

impl<'a> From<&'a mut Response> for ResponseModifier<'a> {
    fn from(r: &'a mut Response) -> Self {
        Self(Some(r))
    }
}

impl<'a> From<Option<&'a mut Member>> for ResponseModifier<'a> {
    fn from(m: Option<&'a mut Member>) -> Self {
        Self(m.map(|m| &mut m.r#type))
    }
}
