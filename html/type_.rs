// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;
use lrs::io::{Write};

use html::{path, Formatter};
use html::markup::{self};
use markup::{Document};
use tree::*;

impl Formatter {
    pub fn type_(&mut self, item: &ItemData, docs: &Document) -> Result {
        match item.inner {
            Item::Struct(ref  s) => self.struct_(item, s, docs),
            Item::Enum(ref    e) => self.enum_(e,   docs),
            Item::Typedef(ref t) => self.typedef(t, docs),
            Item::Trait(ref   t) => self.trait_(t,  docs),
            _ => abort!(),
        }
    }

    pub fn type_static_methods<W: Write>(&mut self, mut file: &mut W,
                                         item: &ItemData) -> Result {
        let impls = item.impls.borrow();

        let mut methods: Vec<_> = Vec::new();

        for impl_ in &*impls {
            if impl_.trait_.is_none() {
                for item in &impl_.items {
                    if let Item::Method(ref method) = item.inner {
                        if let SelfTy::Static = method.self_ {
                            try!(methods.reserve(1));
                            methods.push((impl_, item, method));
                        }
                    }
                }
            }
        }

        if methods.len() == 0 {
            return Ok(());
        }

        methods.sort_by(|&(_, i1, _), &(_, i2, _)| i1.name.as_ref().unwrap().as_ref()
                                              .cmp(i2.name.as_ref().unwrap().as_ref()));

        try!(file.write_all(b"\
            <h2>Static methods</h2>\
            <table>\
                <thead>\
                    <tr>\
                        <th>Name</th>\
                        <th>Description</th>\
                    </tr>\
                </thead>\
                <tbody>\
                    "));

        for &(impl_, item, method) in &methods {
            try!(self.path.reserve(1));
            self.path.push(try!(item.name.as_ref().unwrap().clone()));
            try!(self.method(impl_, item, method));

            try!(file.write_all(b"\
                <tr>\
                    <td>\
                        <a href=\"./\
                    "));
            try!(file.write_all(try!(path::path(&self.path)).as_ref()));
            try!(file.write_all(b"\">"));
            try!(file.write_all(item.name.as_ref().unwrap().as_ref()));
            try!(file.write_all(b"\
                        </a>\
                    </td>\
                    <td>\
                    "));
            try!(markup::short(file, &item.docs.parts));
            try!(file.write_all(b"\
                    </td>\
                </tr>\
                "));

            self.path.pop();
        }

        try!(file.write_all(b"\
                </tbody>\
            </table>\
            "));

        Ok(())
    }
}
